#!/usr/bin/env ruby
# encoding: UTF-8
# frozen_string_literal: true

Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

require 'digest'
require 'fileutils'
require 'json'
require 'open3'
require 'optparse'
require 'timeout'
require 'yaml'

options = {
  manifest: 'agent/visual-regression.yaml',
  output: nil,
  strict_ai: false
}

OptionParser.new do |opts|
  opts.banner = 'Usage: ruby agent/visual-verify.rb [options]'
  opts.on('--manifest PATH', 'Visual verification manifest') { |value| options[:manifest] = value }
  opts.on('--output PATH', 'Report output directory') { |value| options[:output] = value }
  opts.on('--strict-ai', 'Fail when AI review is still pending') { options[:strict_ai] = true }
end.parse!

def now_utc
  Time.now.utc.strftime('%Y-%m-%dT%H:%M:%SZ')
end

def command_result(id:, command:, timeout_seconds:, required:)
  started_at = now_utc
  stdout = +''
  stderr = +''
  status = nil
  timed_out = false

  begin
    Timeout.timeout(timeout_seconds) do
      stdout, stderr, status = Open3.capture3(command)
    end
  rescue Timeout::Error
    timed_out = true
  end

  passed = !timed_out && status&.success?
  {
    id: id,
    command: command,
    required: required,
    timeout_seconds: timeout_seconds,
    status: passed ? 'passed' : (required ? 'failed' : 'warning'),
    exit_code: status&.exitstatus,
    timed_out: timed_out,
    started_at: started_at,
    finished_at: now_utc,
    stdout_tail: stdout.lines.last(40).join,
    stderr_tail: stderr.lines.last(40).join
  }
end

def newest_file(glob, after_time)
  Dir.glob(glob)
     .select { |path| File.file?(path) && File.mtime(path) >= after_time }
     .max_by { |path| File.mtime(path) }
end

def sha256(path)
  Digest::SHA256.file(path).hexdigest
end

def png_dimensions(path)
  return nil unless path && File.file?(path)

  File.open(path, 'rb') do |file|
    signature = file.read(8)
    return nil unless signature == "\x89PNG\r\n\x1A\n".b

    file.read(4)
    chunk_type = file.read(4)
    return nil unless chunk_type == 'IHDR'

    width, height = file.read(8).unpack('N2')
    { width: width, height: height }
  end
rescue StandardError
  nil
end

def run_pixel_check(check, actual)
  baseline = check['baseline']
  return { kind: 'pixel', status: 'pending', reason: 'actual screenshot missing' } unless actual && File.file?(actual)
  return { kind: 'pixel', status: 'pending', reason: "baseline missing: #{baseline}" } unless baseline && File.file?(baseline)

  actual_hash = sha256(actual)
  baseline_hash = sha256(baseline)
  status = actual_hash == baseline_hash ? 'passed' : 'failed'
  {
    kind: 'pixel',
    status: status,
    actual: actual,
    baseline: baseline,
    actual_sha256: actual_hash,
    baseline_sha256: baseline_hash,
    note: 'Exact hash check. Use semantic or perceptual checks when antialiasing is expected.'
  }
end

def run_semantic_check(check, actual)
  rule = check['rule'] || 'manual_semantic_review'
  status = actual && File.file?(actual) ? 'pending_ai_review' : 'pending'
  {
    kind: 'semantic',
    status: status,
    rule: rule,
    actual: actual,
    checklist: Array(check['checklist'])
  }
end

def phase_status(items)
  return 'skipped' if items.empty?
  return 'failed' if items.any? { |item| item[:status] == 'failed' || item['status'] == 'failed' }
  return 'pending' if items.any? { |item| item[:status].to_s.start_with?('pending') || item['status'].to_s.start_with?('pending') }

  'passed'
end

def build_human_review_panel(suite, screenshot)
  review_panel = suite['review_panel'] || {}
  dimensions = png_dimensions(screenshot)
  {
    status: screenshot ? 'ready' : 'skipped',
    image: screenshot,
    absolute_image: screenshot ? File.expand_path(screenshot) : nil,
    image_dimensions: dimensions,
    coordinate_system: review_panel['coordinate_system'] || 'image pixel coordinates, origin at top-left',
    expected_layout: Array(review_panel['expected_layout']),
    expected_colors: Array(review_panel['expected_colors']),
    expected_regions: Array(review_panel['expected_regions']),
    questions: Array(review_panel['questions']),
    ai_result_template: {
      verdict: 'pending',
      confidence: nil,
      layout_observations: [],
      color_observations: [],
      coordinate_observations: [],
      findings: []
    }
  }
end

manifest_path = options[:manifest]
abort("manifest not found: #{manifest_path}") unless File.file?(manifest_path)

manifest = YAML.load_file(manifest_path)
output_dir = options[:output] || manifest.dig('report', 'output_dir') || manifest['output_dir'] || 'target/visual-verification'
FileUtils.mkdir_p(output_dir)

report = {
  version: 1,
  generated_at: now_utc,
  manifest: manifest_path,
  output_dir: output_dir,
  phases: {},
  suites: []
}

main_verify = manifest['covered_by_main_verify'] || {}
report[:phases][:unit_contract_tests] = {
  status: main_verify['unit_contract_tests'] ? 'covered_by_main_verify' : 'not_configured',
  command: main_verify['unit_contract_tests']
}

snapshot_results = []
Array(manifest.dig('render_command_snapshots', 'commands')).each do |entry|
  next unless entry.fetch('enabled', true)

  snapshot_results << command_result(
    id: entry['id'],
    command: entry['run'],
    timeout_seconds: entry.fetch('timeout_seconds', 120),
    required: entry.fetch('required', true)
  )
end
report[:phases][:render_command_snapshot] = {
  status: phase_status(snapshot_results),
  results: snapshot_results
}

Array(manifest['suites']).each do |suite|
  suite_report = {
    id: suite['id'],
    milestone_id: suite['milestone_id'],
    app: suite['app'],
    enabled: suite.fetch('enabled', true),
    screenshot_capture: nil,
    human_review_panel: nil,
    checks: []
  }

  unless suite_report[:enabled]
    suite_report[:screenshot_capture] = { status: 'skipped', reason: 'suite disabled in manifest' }
    report[:suites] << suite_report
    next
  end

  before = Time.now
  capture = command_result(
    id: "#{suite['id']}:screenshot",
    command: suite['command'],
    timeout_seconds: suite.fetch('timeout_seconds', 180),
    required: suite.fetch('required', true)
  )

  screenshot = nil
  screenshot_glob = suite['screenshot_glob']
  screenshot = newest_file(screenshot_glob, before) if screenshot_glob
  capture[:screenshot] = screenshot
  suite_report[:screenshot_capture] = capture
  suite_report[:human_review_panel] = build_human_review_panel(suite, screenshot)

  Array(suite['checks']).each do |check|
    suite_report[:checks] << case check['kind']
                             when 'pixel'
                               run_pixel_check(check, screenshot)
                             when 'semantic'
                               run_semantic_check(check, screenshot)
                             when 'ai_review'
                               {
                                 kind: 'ai_review',
                                 status: screenshot ? 'pending_ai_review' : 'pending',
                                 actual: screenshot,
                                 checklist: Array(check['checklist'])
                               }
                             else
                               { kind: check['kind'] || 'unknown', status: 'warning', reason: 'unknown check kind' }
                             end
  end

  report[:suites] << suite_report
end

capture_results = report[:suites].map { |suite| suite[:screenshot_capture] }.compact
all_checks = report[:suites].flat_map { |suite| suite[:checks] }
report[:phases][:screenshot_capture] = { status: phase_status(capture_results), results: capture_results }
report[:phases][:pixel_semantic_check] = { status: phase_status(all_checks), results: all_checks }
report[:phases][:ai_visual_review] = {
  status: all_checks.any? { |check| check[:status] == 'pending_ai_review' } ? 'pending_ai_review' : 'not_required',
  strict: options[:strict_ai]
}

json_path = File.join(output_dir, 'report.json')
md_path = File.join(output_dir, 'report.md')
ai_path = File.join(output_dir, 'ai-review-request.md')
human_review_path = File.join(output_dir, 'human-review.md')

File.write(json_path, JSON.pretty_generate(report))

markdown = +"# Visual Verification Report\n\n"
markdown << "- Generated at: #{report[:generated_at]}\n"
markdown << "- Manifest: `#{manifest_path}`\n"
markdown << "- JSON report: `#{json_path}`\n\n"
markdown << "## Phases\n\n"
report[:phases].each do |name, phase|
  markdown << "- `#{name}`: `#{phase[:status] || phase['status']}`\n"
end
markdown << "\n## Suites\n\n"
report[:suites].each do |suite|
  markdown << "### #{suite[:id]}\n\n"
  markdown << "- Milestone: `#{suite[:milestone_id]}`\n"
  markdown << "- App: `#{suite[:app]}`\n"
  markdown << "- Enabled: `#{suite[:enabled]}`\n"
  markdown << "- Screenshot: `#{suite.dig(:screenshot_capture, :screenshot) || 'n/a'}`\n"
  markdown << "- Capture status: `#{suite.dig(:screenshot_capture, :status)}`\n"
  if suite[:human_review_panel]
    panel = suite[:human_review_panel]
    dimensions = panel[:image_dimensions]
    markdown << "- Human review panel: `#{human_review_path}`\n"
    markdown << "- Image size: `#{dimensions ? "#{dimensions[:width]}x#{dimensions[:height]}" : 'unknown'}`\n"
  end
  suite[:checks].each do |check|
    markdown << "- Check `#{check[:kind]}`: `#{check[:status]}` #{check[:rule] || check[:reason]}\n"
  end
  markdown << "\n"
end
File.write(md_path, markdown)

human_markdown = +"# Human Visual Review\n\n"
human_markdown << "This file presents screenshot content analysis inputs for manual calibration.\n\n"
report[:suites].each do |suite|
  panel = suite[:human_review_panel]
  next unless panel && panel[:status] == 'ready'

  dimensions = panel[:image_dimensions]
  human_markdown << "## #{suite[:id]}\n\n"
  human_markdown << "- Milestone: `#{suite[:milestone_id]}`\n"
  human_markdown << "- App: `#{suite[:app]}`\n"
  human_markdown << "- Image: `#{panel[:image]}`\n"
  human_markdown << "- Absolute image: `#{panel[:absolute_image]}`\n"
  human_markdown << "- Image size: `#{dimensions ? "#{dimensions[:width]}x#{dimensions[:height]}" : 'unknown'}`\n"
  human_markdown << "- Coordinate system: #{panel[:coordinate_system]}\n\n"
  human_markdown << "![#{suite[:id]}](#{panel[:absolute_image]})\n\n"

  human_markdown << "### Layout Observations\n\n"
  if panel[:expected_layout].empty?
    human_markdown << "- No expected layout observations configured.\n"
  else
    panel[:expected_layout].each { |item| human_markdown << "- [ ] #{item}\n" }
  end

  human_markdown << "\n### Color Observations\n\n"
  if panel[:expected_colors].empty?
    human_markdown << "- No expected color observations configured.\n"
  else
    panel[:expected_colors].each { |item| human_markdown << "- [ ] #{item}\n" }
  end

  human_markdown << "\n### Coordinate / Region Observations\n\n"
  if panel[:expected_regions].empty?
    human_markdown << "- No expected coordinate regions configured.\n"
  else
    human_markdown << "| Region | Expected Bounds | Expected Content |\n"
    human_markdown << "|---|---|---|\n"
    panel[:expected_regions].each do |region|
      bounds = Array(region['bounds']).join(', ')
      human_markdown << "| #{region['id'] || 'region'} | `#{bounds}` | #{region['expectation'] || ''} |\n"
    end
  end

  human_markdown << "\n### Calibration Questions\n\n"
  if panel[:questions].empty?
    human_markdown << "- No calibration questions configured.\n"
  else
    panel[:questions].each { |item| human_markdown << "- [ ] #{item}\n" }
  end

  human_markdown << "\n### AI Result Template\n\n"
  human_markdown << "```json\n"
  human_markdown << JSON.pretty_generate(panel[:ai_result_template])
  human_markdown << "\n```\n\n"
end
File.write(human_review_path, human_markdown)

ai_markdown = +"# AI Visual Review Request\n\n"
ai_markdown << "Use this file as the handoff for an agent with image analysis capability.\n\n"
report[:suites].each do |suite|
  screenshot = suite.dig(:screenshot_capture, :screenshot)
  next unless screenshot

  ai_checks = suite[:checks].select { |check| check[:status] == 'pending_ai_review' }
  next if ai_checks.empty?

  ai_markdown << "## #{suite[:id]}\n\n"
  ai_markdown << "- Image: `#{screenshot}`\n"
  ai_markdown << "- Milestone: `#{suite[:milestone_id]}`\n"
  ai_markdown << "- Human review panel: `#{human_review_path}`\n"
  panel = suite[:human_review_panel]
  if panel
    dimensions = panel[:image_dimensions]
    ai_markdown << "- Image size: `#{dimensions ? "#{dimensions[:width]}x#{dimensions[:height]}" : 'unknown'}`\n"
    ai_markdown << "- Coordinate system: #{panel[:coordinate_system]}\n"
  end
  ai_checks.each do |check|
    Array(check[:checklist]).each { |item| ai_markdown << "- #{item}\n" }
  end
  if panel
    Array(panel[:expected_layout]).each { |item| ai_markdown << "- Layout: #{item}\n" }
    Array(panel[:expected_colors]).each { |item| ai_markdown << "- Color: #{item}\n" }
    Array(panel[:expected_regions]).each do |region|
      ai_markdown << "- Region #{region['id']}: bounds=#{Array(region['bounds']).join(', ')}; expectation=#{region['expectation']}\n"
    end
  end
  ai_markdown << "\n"
end
File.write(ai_path, ai_markdown)

report[:phases][:visual_report] = {
  status: 'passed',
  json: json_path,
  markdown: md_path,
  human_review: human_review_path,
  ai_review_request: ai_path
}
File.write(json_path, JSON.pretty_generate(report))

failed = report[:phases].any? { |_name, phase| phase[:status] == 'failed' || phase['status'] == 'failed' }
failed ||= options[:strict_ai] && report[:phases][:ai_visual_review][:status] == 'pending_ai_review'

puts "Visual verification report: #{md_path}"
puts "AI review request: #{ai_path}"
exit(failed ? 1 : 0)
