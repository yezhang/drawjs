const render_worker = new Worker('/test/skia/worker/renderer.js');
const canvas = document.getElementById('app');
const offscreen = canvas.transferControlToOffscreen();

render_worker.postMessage({cmd: 'init', payload:{
  canvas: offscreen
}}, [offscreen]);

canvas.addEventListener('click', (e) =>{
  this.focus();
  render_worker.postMessage({cmd: 'event', payload: {
    e: {
      x: e.clientX,
      y: e.clientY,
      button: e.button
    }
  }});
});

canvas.addEventListener('keypress', (e) =>{
  console.log('keypress');
})