import { memory } from "../../mandelbrot-wasm/pkg/mandelbrot_wasm_bg.wasm";
import { Canvas } from "../../mandelbrot-wasm/pkg/mandelbrot_wasm";

const canvas = document.getElementById("plot-canvas");
const width: number = 600;
const height: number = 600;
canvas.width = width;
canvas.height = height;
const renderer = Canvas.new(width, height, "mandelbrot");
const canvas_address = renderer.get_canvas();
const array = new Uint8ClampedArray(memory.buffer, canvas_address, width * height * 4);
const image = new ImageData(
  array, width
)
const ctx = canvas.getContext("2d");

const renderLoop = () => {
  renderer.tick();
  renderer.render();
  ctx.putImageData(image, 0, 0);
  requestAnimationFrame(renderLoop)
}

renderer.render();
ctx.putImageData(image, 0, 0);

renderLoop();

