// For drawing we can use canvas:
//   - https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API

var wasm = null;
var memory = null;
var game = null;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

console.log(canvas);

function ext_draw_rectangle(x, y, w, h, color) {
  // color is #RRGGBBAA so we need to shift it
  const r = (color >> 24) & 0xFF;
  const g = (color >> 16) & 0xFF;
  const b = (color >> 8) & 0xFF;
  const a = color  & 0xFF;

  ctx.fillStyle = "rgba(" + r + "," + g + "," + b + "," + a + ")";
  ctx.fillRect(x, y, w, h);
}

function ext_log(sptr, slen) {
  const buf = new Uint8Array(memory.buffer, sptr, slen);
  const str = new TextDecoder("utf8").decode(buf);
  console.log(str);
}

// We import a function for drawing a rectangle so we can call it from our wasm module:
// https://developer.mozilla.org/en-US/docs/WebAssembly/JavaScript_interface/instantiateStreaming_static
const importObject = {
    env: {
      ext_log,
      ext_draw_rectangle,
    }
};

// We are managing key pressed with:
// https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
window.addEventListener("keydown", logKey);

function logKey(e) {
  console.log(e);

  // UP    -> 0
  // RIGHT -> 1
  // DOWN  -> 2
  // LEFT  -> 3
  switch (e.keyCode) {
    case 37: // Left
      wasm.instance.exports.key(game, 3);
      break;
    case 38: // Up
      wasm.instance.exports.key(game, 0);
      break;
    case 39: // Right
      wasm.instance.exports.key(game, 1);
      break;
    case 40: // Down
      wasm.instance.exports.key(game, 2);
      break;
  }
}

// We are doing animation using requestAnimationFrame:
// https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame
// We are animating the canvas...

function step(timeStamp) {
  wasm.instance.exports.update(game);
  wasm.instance.exports.render(game);
  window.requestAnimationFrame(step);
}

WebAssembly
    .instantiateStreaming(fetch("./target/wasm32-unknown-unknown/debug/wasm_module.wasm"), importObject)
    .then((w) => {
        wasm = w;
        memory = w.instance.exports["memory"];
        game = w.instance.exports.init(canvas.width, canvas.height);
        console.log(game);
        window.requestAnimationFrame(step)});

