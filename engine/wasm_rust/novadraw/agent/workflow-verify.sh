#!/usr/bin/env bash
set -euo pipefail

MODE="--fast"
GATE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --fast|--visual|--full)
      MODE="$1"
      shift
      ;;
    --gate=*)
      GATE="${1#--gate=}"
      shift
      ;;
    --gate)
      GATE="${2:-}"
      shift 2
      ;;
    *)
      echo "Usage: $0 [--fast|--visual|--full] [--gate=ready]" >&2
      exit 2
      ;;
  esac
done

ruby agent/workflow-doctor.rb
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test --workspace --lib --bins --tests

case "$MODE" in
  --fast)
    ;;
  --visual|--full)
    ./agent/workflow-visual-verify.sh
    ;;
  *)
    echo "Usage: $0 [--fast|--visual|--full]" >&2
    exit 2
    ;;
esac

case "$GATE" in
  "")
    ;;
  ready)
    ruby - <<'RUBY'
# encoding: UTF-8
# frozen_string_literal: true

Encoding.default_external = Encoding::UTF_8
Encoding.default_internal = Encoding::UTF_8

require "yaml"

errors = []

active = YAML.load_file("agent/backlog/active.yaml") || {}
active_items = Array(active["items"])
oversized_active = active_items.select { |item| item["status"].to_s == "pending" && item["title"].to_s.length > 120 }
errors << "active backlog still contains oversized pending delta candidates" unless oversized_active.empty?

debts = YAML.load_file("agent/backlog/baseline-debts.yaml") || {}
blocking_debts = Array(debts["baseline_debts"]).select do |debt|
  debt["status"] == "open" && %w[high medium].include?(debt["impact"].to_s)
end
unless blocking_debts.empty?
  ids = blocking_debts.map { |debt| debt["id"] || "<missing id>" }.join(", ")
  errors << "blocking baseline debts remain open: #{ids}"
end

worklog = File.file?("agent/inner-loop-worklog.md") ? File.read("agent/inner-loop-worklog.md") : ""
smoke_window = worklog.lines.last(400).join
unless smoke_window.match?(/smoke/i) && smoke_window.match?(/✅|passed|通过|重新发现/)
  errors << "recent discover smoke evidence not found in inner-loop-worklog.md"
end

if errors.empty?
  puts "ready gate ok"
else
  warn "ready gate failed:"
  errors.each { |error| warn "- #{error}" }
  exit 1
end
RUBY
    ;;
  *)
    echo "Unknown gate: $GATE" >&2
    exit 2
    ;;
esac
