#!/usr/bin/env bash
set -euo pipefail

MANIFEST="${1:-agent/visual-regression.yaml}"
OUTPUT_DIR="${2:-target/visual-verification}"

ruby agent/visual-verify.rb \
  --manifest "$MANIFEST" \
  --output "$OUTPUT_DIR"
