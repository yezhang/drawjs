#!/usr/bin/env bash
set -euo pipefail

ruby agent/workflow-doctor.rb
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
