use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Renderer {
    width: usize,
    height: usize,
    canvas: Vec<u32>,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(width: usize, height: usize) -> Renderer {
        let length = width * height;
        let canvas: Vec<u32> = vec![0xFF_FF_00_FF; length];
        Renderer {
            canvas,
            width,
            height,
        }
    }

    pub fn get_canvas(&self) -> *const u32 {
        self.canvas.as_ptr()
    }

    pub fn render(&mut self) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                self.canvas[y * self.height + x] = 0xFF_FF_FF_FF;
            }
        }
    }
}
