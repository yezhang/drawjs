#!/usr/bin/env bash
set -euo pipefail

cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
