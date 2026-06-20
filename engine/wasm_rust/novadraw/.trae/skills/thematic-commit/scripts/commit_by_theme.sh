#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

DRY_RUN=0
ONLY_GROUP=""

usage() {
  cat <<'EOF'
Usage: .trae/skills/thematic-commit/scripts/commit_by_theme.sh [--dry-run] [--only <group>] [--list-groups]

Groups:
  engine-contracts     图元契约、渲染兼容接口、scene tests、示例代码
  visual-verification  视觉验收流程、截图校验、product existence checks
  workflow-context     backlog、roadmap、checkpoint 与热路径治理
  tooling              skill 自身与 helper
EOF
}

list_groups() {
  printf '%s\n' \
    "engine-contracts" \
    "visual-verification" \
    "workflow-context" \
    "tooling"
}

group_message() {
  case "$1" in
    engine-contracts) printf '%s\n' "补齐图元契约与渲染兼容接口" ;;
    visual-verification) printf '%s\n' "新增视觉验收工作流与产品存在性检查" ;;
    workflow-context) printf '%s\n' "清理工作流热路径与 backlog 恢复上下文" ;;
    tooling) printf '%s\n' "新增按主题提交 skill" ;;
    *)
      echo "Unknown group: $1" >&2
      exit 2
      ;;
  esac
}

group_paths() {
  case "$1" in
    engine-contracts)
      cat <<'EOF'
apps/
novadraw-apps/src/app.rs
novadraw-geometry/src/
novadraw-geometry/tests/
novadraw-render/src/
novadraw-render/tests/
novadraw-scene/src/
novadraw-scene/tests/
EOF
      ;;
    visual-verification)
      cat <<'EOF'
agent/m1-product-existence-checks.md
agent/m2-contract-alignment-summary.md
agent/m2-product-existence-checks.md
agent/visual-regression.yaml
agent/visual-verification.md
agent/visual-verify.rb
agent/workflow-verify.sh
agent/workflow-visual-verify.sh
doc/06-roadmap/demo-matrix.md
EOF
      ;;
    workflow-context)
      cat <<'EOF'
agent/backlog/
agent/draw2d-core-milestones.yaml
agent/goal-roadmap.md
agent/governance-contract-coverage.md
agent/inner-loop-checkpoint.md
agent/inner-loop-worklog.md
agent/interruptions-inbox.md
agent/quality-integration-test-plan.md
agent/workflow-doctor.rb
agent/workflow-history.md
doc/06-roadmap/product-deliverables.md
EOF
      ;;
    tooling)
      cat <<'EOF'
.trae/skills/thematic-commit/
EOF
      ;;
    *)
      echo "Unknown group: $1" >&2
      exit 2
      ;;
  esac
}

collect_paths() {
  local group="$1"
  local line
  local -a paths=()
  while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    paths+=("$line")
  done < <(group_paths "$group")
  printf '%s\0' "${paths[@]}"
}

group_has_changes() {
  local group="$1"
  local -a paths=()
  while IFS= read -r -d '' path; do
    paths+=("$path")
  done < <(collect_paths "$group")
  [[ ${#paths[@]} -gt 0 ]] || return 1
  [[ -n "$(git status --short -- "${paths[@]}")" ]]
}

commit_group() {
  local group="$1"
  local -a paths=()
  local message

  while IFS= read -r -d '' path; do
    paths+=("$path")
  done < <(collect_paths "$group")

  if ! group_has_changes "$group"; then
    return 0
  fi

  message="$(group_message "$group")"

  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "[$group] $message"
    git status --short -- "${paths[@]}"
    echo
    return 0
  fi

  git add -A -- "${paths[@]}"
  if git diff --cached --quiet; then
    return 0
  fi
  git commit -m "$message"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      DRY_RUN=1
      shift
      ;;
    --only)
      [[ $# -ge 2 ]] || { usage >&2; exit 2; }
      ONLY_GROUP="$2"
      shift 2
      ;;
    --list-groups)
      list_groups
      exit 0
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "$ONLY_GROUP" ]]; then
  commit_group "$ONLY_GROUP"
  exit 0
fi

for group in engine-contracts visual-verification workflow-context tooling; do
  commit_group "$group"
done
