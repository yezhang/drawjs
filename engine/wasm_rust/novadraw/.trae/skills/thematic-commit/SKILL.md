---
name: "thematic-commit"
description: "Splits local changes into atomic commits by theme. Invoke when the user asks to commit current work by topic, batch staged changes, or turn a dirty worktree into reviewable commits."
---

# Thematic Commit

按主题拆分当前工作区改动，并生成中文、原子化的提交。

## When To Use

- 用户要求“按主题提交”“拆成几次 commit”“把当前改动整理后提交”
- 工作区存在多类改动，直接一次提交会混淆责任边界
- 需要先 dry-run 预览，再决定逐组提交

## Required Context

1. 读取 `AGENTS.md` 与 `CLAUDE.md`
2. 查看 `git status --short`
3. 必要时查看 `git diff --stat` 与局部 diff，确认主题边界

## Decision Rule

先判断当前改动是否落在仓库里稳定的几类主题：

- `engine-contracts`
  - 引擎 API、FigureGraph 契约、render/geometry 兼容接口
  - `apps/*` 示例代码
  - `novadraw-scene/tests/*`
- `visual-verification`
  - 视觉验收脚本、截图回归、product existence checks
  - demo matrix / 验收入口文档
- `workflow-context`
  - backlog、roadmap、checkpoint、workflow doctor/history
  - 恢复热路径治理
- `tooling`
  - skill/helper 自身

如果当前改动基本符合这些边界，优先使用 helper：

```bash
.trae/skills/thematic-commit/scripts/commit_by_theme.sh --dry-run
```

如果存在跨边界、命名异常或临时文件，不要硬套 helper。此时应：

1. 先人工查看 `git diff --name-only`
2. 明确每一组的责任边界
3. 用 `git add -A -- <paths...>` 手工分组
4. 用中文提交信息逐组提交

## Procedure

1. 检查工作区状态，判断是否需要先询问用户提交粒度
2. 先执行 helper 的 `--dry-run`
3. 复核分组是否合理，确认没有把无关文件混入同一组
4. 如果分组合理，执行：

```bash
.trae/skills/thematic-commit/scripts/commit_by_theme.sh
```

5. 如果只需提交某一组，执行：

```bash
.trae/skills/thematic-commit/scripts/commit_by_theme.sh --only <group>
```

6. 提交后再次运行 `git status --short`
7. 向用户汇报：
   - 本次生成了哪些 commit
   - 每个 commit 覆盖哪些主题
   - 剩余未提交内容是什么

## Commit Message Rule

- 摘要必须使用中文
- 一次 commit 只表达一个主题
- 不允许把代码实现、工作流治理、视觉验收混成一个提交

## Guardrails

- 若 helper 分组无法准确表达改动边界，立即转为手工 staging
- 不得为了凑整把 unrelated changes 混到一个 commit
- 不得修改或回滚用户未要求处理的改动
- 如果存在明显风险分组，应先告诉用户再提交

## Helper

- 脚本路径：`.trae/skills/thematic-commit/scripts/commit_by_theme.sh`
- 支持：
  - `--dry-run`
  - `--only <group>`
  - `--list-groups`
