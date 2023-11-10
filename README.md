# Mandelbrot WASM

My implementation of a visualization of the mandelbrot set written in Rust and compiled to WebAssembly. A HTML canvas is used in typescript, and a WebAssembly memory buffer is filled with images which are then mapped onto a Uint8ClampedArray in JavaScript to have efficient rendering of the generated images without copying large memory elements around between the languages. 

## Architecture
The Rust code exposes a struct called Canvas, which contains an array of u32 numbers which each represent a colour. Each colour is built up of four bytes, each byte representing one element of the RGBA value of that colour. The bytes are set throughout the code by using `0x00_00_00_00` notation where each '_' separated element is one byte. This is in reverse order, so: `0xA_B_G_R`.
The Canvas has a property which is a Box containing a struct that implements the `Renderable` trait, meaning that it has `render()` and `tick()` functions. These allow a `Renderable` object to be used for drawing onto the Canvas.
The `render()` method has the following signature: `render(&self, x: usize, y: usize) -> u32` meaning that it takes a reference to itself and a x and y coordinate on the Canvas, and returns a colour represented in u32 format. The logic of how a x and y position are converted into a colour are dependent on the `Renderable` object, meaning that different interesting visualizations can be easily implemented.
The `tick()` method has the following signature: `tick(&mut self) -> ()` meaning that it takes a mutable reference to itself, and returns nothing. This function is called after every frame of animation, and allows the object to change its internal state such that the next render call will produce a different image. For example, the `Mandelbrot` struct implements the `tick()` method such that it alters the span in both real and imaginary space in which it renders, resulting in a zoom-in effect.

## Running the project

This current version uses the Vite development server to host the files. However, you could bundle and host the files statically if you so desired with some work. To use the development server do the following things:

To build the rust into a WebAssembly package you need wasm-pack installed on your machine. You can do this with `cargo install wasm-pack`. Then, inside of the mandelbrot-wasm directory run:
```wasm-pack build```
A new `/pkg` folder should have appeared. This is directly referenced inside of the typescript file for the website.

For running the server you need to have npm installed. Run: 
```npm install``` to install the necessary dependencies for running the Vite server.
You should now be able to run:
```npm run dev``` to get the Vite server running on your localhost.
Navigate your webbrowser to `localhost:5173` and you should see the mandelbrot in action.
