# Architecture Inbox

用于收纳中途插入的突发任务，避免主线架构改进上下文丢失。

## 使用规则

- 每条突发任务单独记录
- 必须注明来源、影响范围、是否阻塞主线
- 如果任务与当前 delta 无关，不要直接插入主线 backlog

## Template

```text
- id: INBOX-YYYYMMDD-01
  source: user | bug | review | experiment
  summary: 一句话描述突发任务
  impact: low | medium | high
  blocks_current_delta: yes | no
  suggested_action: defer | split | switch
```

## Items

- 暂无
