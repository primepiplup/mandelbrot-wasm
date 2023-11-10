use crate::renderables::renderable::Renderable;

pub struct Default {
    base_color: u32,
    counter: u32,
}

impl Renderable for Default {
    fn render(&self, x: usize, y: usize) -> u32 {
        u32::wrapping_add(self.counter, (x ^ y) as u32 | self.base_color)
    }

    fn tick(&mut self) {
        self.counter = u32::wrapping_add(self.counter, 1);
    }
}

impl Default {
    pub fn new(_width: usize, _height: usize) -> Default {
        Default {
            base_color: 0xFF_00_00_00,
            counter: 0,
        }
    }
}
