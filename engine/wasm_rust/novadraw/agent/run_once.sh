#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-}"

if [[ -z "${MODE}" ]]; then
  cat <<'EOF'
Usage:
  ./agent/run_once.sh discover
  ./agent/run_once.sh review
  ./agent/run_once.sh execute
  ./agent/run_once.sh resume

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
  - ${ROOT_DIR}/agent/architecture_contracts.md
  - ${ROOT_DIR}/agent/delta_backlog.yaml
  - ${ROOT_DIR}/agent/session_checkpoint.md
  - ${ROOT_DIR}/agent/inbox.md
  - ${ROOT_DIR}/agent/worklog.md
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
