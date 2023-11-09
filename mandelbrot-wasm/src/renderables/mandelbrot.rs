use crate::complex::Complex;
use crate::renderables::renderable::Renderable;

pub struct Mandelbrot {
    width: f64,
    height: f64,
    x_center: f64,
    y_center: f64,
    x_span: f64,
    y_span: f64,
    max_iter: usize,
}

impl Renderable for Mandelbrot {
    fn render(&self, x: usize, y: usize) -> u32 {
        let c = self.translate(x, y);
        let mut z = c;
        let mut counter = 0;
        while z.within(100000.0) && counter < self.max_iter {
            z = z * z + c;
            counter += 1;
        }
        if z.within(2.0) {
            return 0xFF_00_00_00;
        } else {
            return self.get_color_for_number(counter);
        }
    }

    fn tick(&mut self) {
        self.x_span = self.x_span - (self.x_span / 25.0);
        self.y_span = self.y_span - (self.y_span / 25.0);
    }
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize, x_center: f64, y_center: f64) -> Mandelbrot {
        Mandelbrot {
            width: width as f64,
            height: height as f64,
            x_center,
            y_center,
            x_span: 4.0,
            y_span: 4.0,
            max_iter: 400,
        }
    }

    fn get_color_for_number(&self, number: usize) -> u32 {
        let value: u8 = ((256.0 / self.max_iter as f64) * (number as f64)) as u8;
        let begin_color = 0xFF_00_FF_00;
        let red_hue = (value as u32) << 0;
        begin_color | red_hue
    }

    fn translate(&self, x: usize, y: usize) -> Complex {
        let half_width: f64 = self.width / 2.0;
        let half_height: f64 = self.height / 2.0;
        let x_offset_pixels = x as f64 - half_width;
        let y_offset_pixels = -1.0 * (y as f64 - half_height);
        let x_half_span = self.x_span / 2.0;
        let y_half_span = self.y_span / 2.0;
        let x_normalized = (x_offset_pixels) / (half_width);
        let y_normalized = (y_offset_pixels) / (half_height);
        let x_from_origin = x_normalized * x_half_span;
        let y_from_origin = y_normalized * y_half_span;
        let x_translated = x_from_origin + self.x_center;
        let y_translated = y_from_origin + self.y_center;
        Complex::new(x_translated, y_translated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_is_applied_as_expected() {
        let mandelbrotter = Mandelbrot::new(600, 600, 0.0, 0.0);

        let point = mandelbrotter.translate(450, 150);

        assert_eq!(point, Complex::new(1.0, 1.0));
    }

    #[test]
    fn translation_is_applied_as_expected_with_different_origin() {
        let mandelbrotter = Mandelbrot::new(600, 600, 1.0, 1.0);

        let point = mandelbrotter.translate(450, 450);

        assert_eq!(point, Complex::new(2.0, 0.0));
    }
}
