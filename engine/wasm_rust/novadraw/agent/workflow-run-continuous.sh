#!/usr/bin/env bash
set -euo pipefail

MAX_CYCLES="${1:-3}"
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

cat <<EOF
Project root: ${ROOT_DIR}

Required files:
  - ${ROOT_DIR}/AGENTS.md
  - ${ROOT_DIR}/CLAUDE.md
  - ${ROOT_DIR}/doc/理想架构设计.md
  - ${ROOT_DIR}/agent/README.md
  - ${ROOT_DIR}/agent/workflow-continuous.md
  - ${ROOT_DIR}/agent/governance-architecture-contracts.md
  - ${ROOT_DIR}/agent/governance-contract-coverage.md
  - ${ROOT_DIR}/agent/outer-loop-delta-backlog.yaml
  - ${ROOT_DIR}/agent/backlog/index.yaml
  - ${ROOT_DIR}/agent/backlog/active.yaml
  - ${ROOT_DIR}/agent/backlog/recent.yaml
  - ${ROOT_DIR}/agent/backlog/candidates.yaml
  - ${ROOT_DIR}/agent/backlog/baseline-debts.yaml
  - ${ROOT_DIR}/agent/inner-loop-checkpoint.md
  - ${ROOT_DIR}/agent/inner-loop-worklog.md
  - ${ROOT_DIR}/agent/quality-checkpoint-schema.md
  - ${ROOT_DIR}/agent/quality-testing-strategy.md
  - ${ROOT_DIR}/agent/quality-discover-smoke-test.md
  - ${ROOT_DIR}/agent/workflow-doctor.rb
  - ${ROOT_DIR}/agent/workflow-verify.sh

Prompt:
请按 agent/workflow-continuous.md 运行持续架构闭环。

预算：
- max_cycles: ${MAX_CYCLES}
- max_delta_per_run: 1
- max_consecutive_execute_without_review: 2
- max_failed_verification_retry: 1

要求：
1. 先执行 BOOTSTRAP 和 ASSESS，不要直接改代码。
2. 根据 gate 自动选择 discover / review / resume / execute。
3. 每轮只执行一个最小 delta，不允许跨层级大改。
4. 每轮结束必须更新 backlog、checkpoint、worklog、contract coverage。
5. 如果达到停止条件，立即停止并输出 Current State、Stop Reason、Next Restart Prompt。
6. 如果满足 Ideal Completion Definition，执行 Completion Audit 并输出最终完成报告。
EOF
