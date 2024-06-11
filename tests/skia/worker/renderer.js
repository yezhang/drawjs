importScripts('../../../node_modules/canvaskit-wasm/bin/canvaskit.js');


const ckLoaded = CanvasKitInit({
  locateFile: (file) => '../../../node_modules/canvaskit-wasm/bin/'+file});

class InputEvent extends Event {

}

console.log(InputEvent);
console.log(Event);

const cmdMap = {
  'init': function(payload) {
    const  {canvas } = payload;
    ckLoaded.then((CanvasKit) => {
      const surface = CanvasKit.MakeWebGLCanvasSurface(canvas);
  
      const paint = new CanvasKit.Paint();
      paint.setColor(CanvasKit.Color4f(0.9, 0, 0, 1.0));
      paint.setStyle(CanvasKit.PaintStyle.Stroke);
      paint.setAntiAlias(true);
      const rr = CanvasKit.RRectXY(CanvasKit.LTRBRect(10, 60, 210, 260), 25, 15);
  
      function draw(canvas) {
        canvas.clear(CanvasKit.WHITE);
        canvas.drawRRect(rr, paint);
      }
      surface.drawOnce(draw);
    });
  },
  'click': function(payload) {

  }
};

self.onmessage = (e) => {
  const data = e.data;
  console.log(data);

  const {cmd, payload} = data;
  console.log(cmd);

  if(cmd in cmdMap) {
    cmdMap[cmd](payload);
  }
}