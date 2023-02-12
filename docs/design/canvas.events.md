# HTML Canvas 中的事件

本文档详细说明了 <Canvas/> 元素支持的浏览器事件。

## canvas 元素的典型事件
参考 https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement#events

- [webglcontextcreationerror](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/webglcontextcreationerror_event)
- [webglcontextlost](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/webglcontextlost_event)
- [webglcontextrestored](https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/webglcontextrestored_event)

## HTMLElement 元素的典型事件

https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement#event_handlers

HTMLElement 元素的事件主要继承自 Element 元素、或实现了 DocumentAndElementEventHandlers、GlobalEventHandlers、TouchEventHandlers 接口。

- [HTMLElement.oncopy](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy)
- [HTMLElement.oncut](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut)
- [HTMLElement.onpaste](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste)
- TouchEventHandlers...

- [invalid](https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/invalid_event)

### Animation events
- [animationcancel](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/animationcancel_event)
- [animationend](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/animationend_event)
- [animationiteration](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/animationiteration_event)
- [animationstart](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/animationstart_event)

### Input events
- [beforeinput](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
    - 支持的元素 `<input>, <select>, or <textarea>`
- [input](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)
- [change](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)

### Pointer events
- [gotpointercapture](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/gotpointercapture_event)
- [lostpointercapture](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/lostpointercapture_event)
- [pointercancel](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointercancel_event)
- [pointerdown](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerdown_event)
- [pointerenter](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerenter_event)
- [pointerleave](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerleave_event)
- [pointermove](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointermove_event)
- [pointerout](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerout_event)
- [pointerover](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerover_event)
- [pointerup](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/pointerup_event)

### Transition events
- [transitioncancel](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/transitioncancel_event)
- [transitionend](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/transitionend_event)
- [transitionrun](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/transitionrun_event)
- [transitionstart](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/transitionstart_event)

## GlobalEventHandlers 接口定义的事件

https://developer.mozilla.org/en-US/docs/Web/API/GlobalEventHandlers


## Web Worker 中的事件处理：
Input For Workers and Worklets，https://github.com/WICG/input-for-workers
Events in Workers and Worklets，https://docs.google.com/document/d/1byDy6IZqvaci-FQoiRzkeAmTSVCyMF5UuaSeGJRHpJk/edit#heading=h.yzqtozqzq3vs

参考资料
- https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement#events

