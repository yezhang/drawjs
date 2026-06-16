# Visual Verification Workflow

本文档定义 Novadraw 工作流中的视觉验证门禁。目标是把 app 截图能力纳入可恢复、可审计的验证链路，用于验证渲染管线、Figure 协议、裁剪、坐标、Viewport、Connection 和文本控件的端到端正确性。

## 六阶段

| 阶段 | 名称 | 职责 | 自动化来源 |
|---|---|---|---|
| 1 | Unit Contract Tests | 验证几何、坐标、布局、事件、UpdateManager 等协议行为 | `cargo test` |
| 2 | RenderCommand Snapshot | 验证同输入下渲染命令序列稳定 | `agent/visual-regression.yaml` 的 `render_command_snapshots` |
| 3 | Screenshot Capture | 运行 app 截图模式并归档 PNG | `agent/visual-regression.yaml` 的 `suites[].command` |
| 4 | Pixel / Semantic Check | 执行 baseline hash、语义 checklist、规则检查 | `agent/visual-verify.rb` |
| 5 | AI Visual Review | 由具备图像分析能力的 Agent 读取截图并给出视觉结论 | `target/visual-verification/ai-review-request.md` |
| 6 | Visual Report | 输出 JSON / Markdown / 人工校对视图 / AI 审查请求 | `target/visual-verification/report.*` |

## 命令

快速验证不运行视觉门禁：

```bash
./agent/workflow-verify.sh
```

运行标准验证后追加视觉门禁：

```bash
./agent/workflow-verify.sh --visual
```

仅运行视觉门禁：

```bash
./agent/workflow-visual-verify.sh
```

指定 manifest 和输出目录：

```bash
./agent/workflow-visual-verify.sh agent/visual-regression.yaml target/visual-verification
```

## Manifest

视觉验证清单位于：

```text
agent/visual-regression.yaml
```

每个 suite 至少包含：

```yaml
- id: m8-viewport-clip
  milestone_id: M8
  app: viewport-app
  enabled: true
  command: cargo run -p viewport-app -- --screenshot-clip
  screenshot_glob: apps/viewport-app/screenshot/*.png
  checks:
    - kind: pixel
      baseline: agent/visual-baselines/viewport-app/clip-to-viewport.png
    - kind: semantic
      rule: content_must_not_bleed_outside_viewport_border
    - kind: ai_review
      checklist:
        - 黑色 viewport 边框外不应出现黄色原点块。
  review_panel:
    coordinate_system: "image pixel coordinates, origin at top-left"
    expected_layout:
      - viewport 边框应固定不动，content 应在边框内部被裁剪。
    expected_colors:
      - 黑色 viewport 边框应清晰。
    expected_regions:
      - id: viewport_border
        bounds: [120, 80, 520, 420]
        expectation: "content 不得越过该区域"
    questions:
      - content 是否 bleed 到 viewport 边框外？
```

## 判定规则

- `Unit Contract Tests` 是硬门禁，由 `workflow-verify.sh` 中的 `cargo test` 覆盖。
- `RenderCommand Snapshot` 优先级高于截图，因为它更容易定位到渲染管线的具体阶段。
- `Screenshot Capture` 失败时，相关 suite 失败。
- `Pixel Check` 当前使用 baseline PNG 的 SHA-256 精确比对；遇到抗锯齿或字体波动时，应改用语义检查或后续引入感知 diff。
- `Semantic Check` 默认进入 `pending_ai_review`，由 Agent 图像分析补结论。
- `AI Visual Review` 默认是 advisory，不阻塞脚本退出；需要强制阻塞时使用 `ruby agent/visual-verify.rb --strict-ai`。
- `Human Review Panel` 会把截图、尺寸、布局期望、颜色期望、坐标区域和 AI 结果模板集中到 `human-review.md`，方便人工校对。

## 人工校对视图

每次视觉验证都会生成：

```text
target/visual-verification/human-review.md
```

该文件用于人工校对图片内容，重点呈现：

- 截图预览
- 图片尺寸
- 坐标系统说明
- 关键布局观察项
- 关键颜色观察项
- 关键坐标/区域观察项
- AI 分析结果模板

推荐校对顺序：

1. 先看 `report.md` 确认哪些 suite 生成了截图。
2. 打开 `human-review.md`，对照图片预览检查布局、颜色、坐标。
3. 若发现问题，把 AI Result Template 中的 `verdict / confidence / findings` 填完整。
4. 将最终结论回写到当前 delta 的 worklog 或视觉报告归档。

## Agent 图像分析

当报告中存在 `pending_ai_review`，工作流会生成：

```text
target/visual-verification/ai-review-request.md
```

Agent 应按该文件列出的截图路径读取图片，并输出结构化结论：

```json
{
  "verdict": "fail",
  "confidence": 0.92,
  "findings": [
    {
      "title": "content bleed outside viewport",
      "evidence": "黄色原点块出现在黑色 viewport 边框外",
      "suspected_layer": "clip transform mapping",
      "next_debug_entry": "NdCanvas::clip_rect -> VelloRenderer::push_clip_layer"
    }
  ]
}
```

结论应回写到 `agent/inner-loop-worklog.md` 或当前 delta 的验证记录中。若要将 AI 结论纳入硬门禁，必须先有稳定的结构化回写文件，再启用 `--strict-ai`。

## 截图能力约束

为了让视觉验证稳定，app 截图入口应逐步统一支持：

```text
--screenshot=<scene_id>
--screenshot-all
--screenshot-output=<path>
--window-size=800x600
--scale-factor=1.0
--background=#eeeeee
```

当前已有 app 仍可使用现有 timestamp 文件名，`visual-verify.rb` 会通过 `screenshot_glob` 选择命令执行后的最新 PNG。后续建议加入固定输出路径，减少归档和 baseline 更新成本。

## 适用里程碑

| Milestone | 视觉验证重点 |
|---|---|
| M2 | 基础图元、paint 三段式、bounds/clientArea 可视化 |
| M3 | 嵌套 clip、递归/迭代渲染等价 |
| M4 | 深层坐标、坐标根移动、事件点降域 |
| M5 | 布局截图、dirty repair 后画面稳定 |
| M8 | Viewport clip、scroll origin、zoom、nested viewport |
| M9 | Anchor、router、decorations |
| M10 | 文本、图像、边框、tooltip |

## 更新 baseline

baseline 更新必须人工确认，禁止脚本自动覆盖：

1. 运行 `./agent/workflow-verify.sh --visual`。
2. 读取 `target/visual-verification/report.md`。
3. 对失败截图执行 AI / 人工视觉审查。
4. 确认是预期变化后，手动复制 PNG 到 `agent/visual-baselines/<app>/...`。
5. 在 worklog 记录 baseline 更新原因。
