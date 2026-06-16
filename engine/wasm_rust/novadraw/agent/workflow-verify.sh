#!/usr/bin/env bash
set -euo pipefail

MODE="${1:---fast}"

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
