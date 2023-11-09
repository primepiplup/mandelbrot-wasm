use wasm_bindgen::prelude::*;

use crate::renderables::default::Default;
use crate::renderables::mandelbrot::Mandelbrot;
use crate::renderables::renderable::Renderable;

#[wasm_bindgen]
pub struct Canvas {
    width: usize,
    height: usize,
    canvas: Vec<u32>,
    renderer: Box<dyn Renderable>,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(width: usize, height: usize, plot_set: &str) -> Canvas {
        let length = width * height;
        let canvas: Vec<u32> = vec![0xFF_FF_00_FF; length];
        let renderer: Box<dyn Renderable> = match plot_set {
            "mandelbrot" => Box::new(Mandelbrot::new(width, height, -0.75, 0.031)),
            _ => Box::new(Default::new(width, height)),
        };
        Canvas {
            canvas,
            width,
            height,
            renderer,
        }
    }

    pub fn get_canvas(&self) -> *const u32 {
        self.canvas.as_ptr()
    }

    pub fn render(&mut self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                self.canvas[y * self.height + x] = self.renderer.render(x, y);
            }
        }
    }

    pub fn tick(&mut self) -> () {
        self.renderer.tick();
    }
}
