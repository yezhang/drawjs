#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-}"

if [[ -z "${MODE}" ]]; then
  cat <<'EOF'
Usage:
  ./agent/workflow-run-once.sh discover
  ./agent/workflow-run-once.sh review
  ./agent/workflow-run-once.sh execute
  ./agent/workflow-run-once.sh resume
  ./agent/workflow-run-once.sh smoke
  ./agent/workflow-run-once.sh stabilize

Notes:
  - This script does not call an AI model directly.
  - It prints the exact prompt and required files for the next manual agent round.
  - Use it as a stable launcher for one bounded workflow iteration.
EOF
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

print_common_context() {
  cat <<EOF
Project root: ${ROOT_DIR}

Required files:
  - ${ROOT_DIR}/CLAUDE.md
  - ${ROOT_DIR}/AGENTS.md
  - ${ROOT_DIR}/doc/理想架构设计.md
  - ${ROOT_DIR}/agent/governance-architecture-contracts.md
  - ${ROOT_DIR}/agent/outer-loop-delta-backlog.yaml
  - ${ROOT_DIR}/agent/inner-loop-checkpoint.md
  - ${ROOT_DIR}/agent/interruptions-inbox.md
  - ${ROOT_DIR}/agent/inner-loop-worklog.md
EOF
}

case "${MODE}" in
  discover)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 discover-architecture-deltas，对照理想架构找出当前最值得进入 backlog 的候选问题。
要求：
- 先总结关键契约
- 输出 candidate delta
- 对每个候选项给出根因摘要
- 说明 promote/reject 建议
- 给出建议优先级和 done_when
EOF
    ;;
  review)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 review-delta-backlog，整理当前 backlog。
要求：
- 检查重复项、过大项、过期项
- 判断 candidate 是否应提升或拒绝
- 重排优先级
- 明确当前最值得执行的一个 delta
EOF
    ;;
  smoke)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 discover-architecture-deltas 的 smoke test。不要直接沿用 backlog 结论，而是按审计清单重新检查代码，并告诉我这次 discover 是否能重新发现 agent/quality-discover-smoke-test.md 中列出的已知问题样本。
要求：
- 列出本轮审计了哪些契约
- 列出本轮检查了哪些代码入口
- 说明重新发现了哪些已知偏差
- 若输出 0 个 candidate，必须解释覆盖范围与遗漏范围
EOF
    ;;
  stabilize)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 resume-architecture-work 和 review-delta-backlog 的稳定性检查。先验证 inner-loop-checkpoint.md 是否满足 quality-checkpoint-schema.md，再结合 quality-workflow-readiness.md 判断当前工作流是否已经达到可用于真实架构推进的等级。
要求：
- 输出 Schema Health
- 输出 Gate Violations
- 输出 Current Readiness Level
- 输出 Go / No-Go 建议
EOF
    ;;
  execute)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 execute-architecture-delta，本轮只处理一个 delta，不要跨层级大改。
要求：
- 先解释根因
- 给最小修改方案
- 修改代码后运行验证
- 更新 backlog、checkpoint、worklog
- 如果发现残余问题，输出新的 candidate delta
EOF
    ;;
  resume)
    print_common_context
    cat <<'EOF'

Prompt:
请执行 resume-architecture-work，读取 agent 工作流文件，并告诉我当前主线、当前假设、下一步最小动作；如果 backlog 可能失真，请先建议运行 discover-architecture-deltas。
EOF
    ;;
  *)
    echo "Unknown mode: ${MODE}" >&2
    exit 1
    ;;
esac
