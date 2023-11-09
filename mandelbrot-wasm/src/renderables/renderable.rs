pub trait Renderable {
    fn render(&self, x: usize, y: usize) -> u32;
    fn tick(&mut self);
}
