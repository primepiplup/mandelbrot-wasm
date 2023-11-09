use crate::renderables::renderable::Renderable;

pub struct Default {}

impl Renderable for Default {
    fn render(&self, x: usize, y: usize) -> u32 {
        (x ^ y) as u32 | 0xFF_00_00_00
    }

    fn tick(&mut self) {}
}

impl Default {
    pub fn new(_width: usize, _height: usize) -> Default {
        Default {}
    }
}
