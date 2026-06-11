#!/usr/bin/env ruby
# frozen_string_literal: true

require "set"
require "yaml"

ROOT = File.expand_path("..", __dir__)

VALID_MILESTONE_STATUSES = %w[
  not_started
  in_progress
  contract_aligned
  behavior_verified
  complete
].freeze

VALID_DELTA_STATUSES = %w[
  candidate
  promoted
  rejected
  pending
  proposed
  in_progress
  split
  blocked
  verified
  done
].freeze

VALID_DEBT_STATUSES = %w[open accepted resolved].freeze
REQUIRED_CHECKPOINT_SECTIONS = [
  "# Session Checkpoint",
  "## Metadata",
  "## Current Delta",
  "## Current Status",
  "## What Was Done",
  "## Current Hypothesis",
  "## Next Small Step",
  "## Blockers",
  "## Verification State",
  "## Resume Prompt"
].freeze

@errors = []
@warnings = []

def path(relative)
  File.join(ROOT, relative)
end

def fail_check(message)
  @errors << message
end

def warn_check(message)
  @warnings << message
end

def require_file(relative)
  absolute = path(relative)
  fail_check("missing file: #{relative}") unless File.file?(absolute)
  absolute
end

def load_yaml(relative)
  absolute = require_file(relative)
  return {} unless File.file?(absolute)

  YAML.load_file(absolute) || {}
rescue Psych::SyntaxError => e
  fail_check("invalid yaml: #{relative}: #{e.message}")
  {}
end

def future_delta?(id)
  return false unless id

  case id
  when /\AAD-(\d+)/
    Regexp.last_match(1).to_i >= 20
  when /\ACAD-(\d+)/
    Regexp.last_match(1).to_i >= 11
  else
    true
  end
end

def check_milestones
  data = load_yaml("agent/draw2d-core-milestones.yaml")
  bootstrap = data["bootstrap"]
  milestones = data["milestones"] || []

  fail_check("bootstrap M0 missing") unless bootstrap.is_a?(Hash) && bootstrap["id"] == "M0"
  if bootstrap
    status = bootstrap["status"]
    fail_check("M0 has invalid status: #{status}") unless VALID_MILESTONE_STATUSES.include?(status)
    %w[goal scope done_when].each do |field|
      value = bootstrap[field]
      fail_check("M0 missing #{field}") if value.nil? || (value.respond_to?(:empty?) && value.empty?)
    end
  end

  ids = milestones.map { |milestone| milestone["id"] }
  expected = (1..10).map { |index| "M#{index}" }
  fail_check("milestone ids must be exactly #{expected.join(", ")}") unless ids == expected

  milestones.each do |milestone|
    id = milestone["id"]
    status = milestone["status"]
    fail_check("#{id} has invalid status: #{status}") unless VALID_MILESTONE_STATUSES.include?(status)

    %w[title goal scope non_scope contracts probes dependencies enables].each do |field|
      fail_check("#{id} missing #{field}") unless milestone.key?(field)
    end

    %w[scope non_scope contracts probes enables].each do |field|
      value = milestone[field]
      fail_check("#{id}.#{field} must be an array") unless value.is_a?(Array)
      fail_check("#{id}.#{field} must not be empty") if value.is_a?(Array) && value.empty? && field != "enables"
    end
  end

  statuses = milestones.to_h { |milestone| [milestone["id"], milestone["status"]] }
  [data.dig("companion_views", "product_view"),
   data.dig("companion_views", "demo_view"),
   data.dig("companion_views", "status_snapshot"),
   data.dig("companion_views", "view_index")].compact.each do |relative|
    require_file(relative)
  end
  statuses
end

def check_goal_roadmap(milestone_statuses)
  text = File.read(require_file("agent/goal-roadmap.md"))
  roadmap_statuses = {}

  text.each_line do |line|
    next unless line =~ /^\|\s*(M\d+)\s*\|[^|]*\|\s*`([^`]+)`\s*\|/

    roadmap_statuses[Regexp.last_match(1)] = Regexp.last_match(2)
  end

  milestone_statuses.each do |id, status|
    roadmap_status = roadmap_statuses[id]
    fail_check("goal-roadmap missing #{id}") unless roadmap_status
    fail_check("goal-roadmap #{id} status #{roadmap_status} != yaml #{status}") if roadmap_status && roadmap_status != status
  end

  complete_count = milestone_statuses.values.count("complete")
  if text =~ /\|\s*完成 milestone 数\s*\|\s*(\d+)\s*\/\s*10\s*\|/
    displayed = Regexp.last_match(1).to_i
    fail_check("goal-roadmap complete count #{displayed} != yaml #{complete_count}") unless displayed == complete_count
  else
    fail_check("goal-roadmap missing complete count summary")
  end
end

def check_demo_matrix(milestone_ids)
  text = File.read(require_file("doc/06-roadmap/demo-matrix.md"))
  milestone_ids.each do |id|
    fail_check("demo-matrix missing #{id}") unless text.include?("| #{id} |") || text.include?("- [ ] #{id} ") || text.include?("- [x] #{id} ")
  end
end

def check_backlog
  data = load_yaml("agent/outer-loop-delta-backlog.yaml")
  all_items = Array(data["candidate_items"]) + Array(data["items"])
  ids = Set.new

  all_items.each do |item|
    id = item["id"]
    fail_check("backlog item missing id") unless id
    fail_check("duplicated backlog id: #{id}") if id && ids.include?(id)
    ids << id if id

    status = item["status"]
    fail_check("#{id} has invalid status: #{status}") if status && !VALID_DELTA_STATUSES.include?(status)

    kind = item["evolution_kind"]
    if future_delta?(id) && %w[architecture parity].include?(kind)
      fail_check("#{id} must declare milestone_id") unless item["milestone_id"]
    end

    next unless item["milestone_id"]

    fail_check("#{id} milestone_id must be M1-M10") unless item["milestone_id"] =~ /\AM(?:[1-9]|10)\z/
  end

  Array(data["baseline_debts"]).each do |debt|
    id = debt["id"]
    status = debt["status"]
    fail_check("baseline debt missing id") unless id
    fail_check("#{id} has invalid debt status: #{status}") unless VALID_DEBT_STATUSES.include?(status)
  end

  ids
end

def check_checkpoint(backlog_ids)
  text = File.read(require_file("agent/inner-loop-checkpoint.md"))
  positions = REQUIRED_CHECKPOINT_SECTIONS.map { |section| [section, text.index(section)] }
  missing = positions.select { |_section, index| index.nil? }.map(&:first)
  missing.each { |section| fail_check("checkpoint missing section: #{section}") }

  ordered = positions.reject { |_section, index| index.nil? }.map(&:last)
  fail_check("checkpoint sections are out of order") unless ordered == ordered.sort

  if text =~ /## Current Delta\s*\n\s*\n-\s+([A-Z]+-\d+[A-Z]?)/m
    current_delta = Regexp.last_match(1)
    fail_check("checkpoint current delta #{current_delta} not found in backlog") unless backlog_ids.include?(current_delta)
  else
    fail_check("checkpoint current delta is not parseable")
  end
end

milestone_statuses = check_milestones
check_goal_roadmap(milestone_statuses)
check_demo_matrix(milestone_statuses.keys)
backlog_ids = check_backlog
check_checkpoint(backlog_ids)

@warnings.each { |message| warn "warning: #{message}" }

if @errors.empty?
  puts "workflow doctor ok"
else
  warn "workflow doctor failed:"
  @errors.each { |message| warn "- #{message}" }
  exit 1
end
