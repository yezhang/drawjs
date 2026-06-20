---
name: "capture-interruption"
description: "Captures the current architecture task before interruption and writes a restart point. Invoke when urgent work appears or the current session must stop."
---

# Capture Interruption

在主线工作被打断时冻结现场，保证下一次恢复时不需要重新梳理所有上下文。

## When To Use

- 有突发 Bug、Review、临时任务插入时
- 当前会话必须结束时
- 已做完部分分析，但还没完成本轮改动时

## Required Inputs

- `agent/inner-loop-checkpoint.md`
- `agent/inner-loop-worklog.md`
- 当前正在处理的 delta

## Procedure

1. 记录当前正在处理的 delta
2. 用简洁语言写清当前已完成内容
3. 写出下一步最小动作，保证可直接恢复
4. 将突发任务记录到 `inner-loop-checkpoint.md` 的 `Interruptions` 小节
5. 如果当前 delta 需要暂停，将状态改为 `blocked` 或保持 `in_progress`

## Output Format

- Interrupted Delta
- What Was Finished
- Exact Restart Point
- Interruption Item
- Whether Mainline Is Blocked

## Guardrails

- 不要把突发任务直接塞进主线 backlog
- 不要只写笼统描述，必须写到“下一步最小动作”
- 如果当前分析推翻了原假设，必须在 checkpoint 中明确写出
