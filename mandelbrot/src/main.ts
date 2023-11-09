import { memory } from "../../mandelbrot-wasm/pkg/mandelbrot_wasm_bg.wasm";
import { Renderer } from "../../mandelbrot-wasm/pkg/mandelbrot_wasm";

const canvas = document.getElementById("plot-canvas");
const width: number = 600;
const height: number = 600;
canvas.width = width;
canvas.height = height;
const renderer = Renderer.new(width, height);
const canvas_address = renderer.get_canvas();
const array = new Uint8ClampedArray(memory.buffer, canvas_address, width * height * 4);
const image = new ImageData(
  array, width
)
const ctx = canvas.getContext("2d");

ctx.putImageData(image, 0, 0);
