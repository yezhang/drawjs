import { IFigure } from 'org/paintbox/IFigure';
import { EventDispatcher } from 'org/paintbox/EventDispatcher';
import { UpdateManager } from 'org/paintbox/UpdateManager';

/**
 * LightweightSystem 是 DOM 和 Paintbox 的桥梁。
 * 用于将 {@link Figure Figures} 寄宿在 &lt;canvas/&gt; 元素中。
 * <p>
 * 使用 LightweightSystem 的一般流程：
 * <ol>
 * <li>创建一个 &lt;canvas/&gt;。
 * <li>创建一个 LightweightSystem，并将 canvas 元素传递进来。
 * <li>创建一个 Paintbox Figure，调用 setContents(IFigure)。
 * 这个 Figure 作为 Paintbox 应用的顶层 Figure。
 * </ol>
 */
export class LightweightSystem {
  private canvas: OffscreenCanvas | HTMLCanvasElement;
  contents: IFigure; //TODO: 放置到 webworker 中
  private root: IFigure;
  private dispatcher: EventDispatcher;
  private manager: UpdateManager = new DeferredUpdateManager();
  private ignoreResize: number;
}