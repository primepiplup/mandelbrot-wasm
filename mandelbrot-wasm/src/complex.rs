use crate::imaginary::Imaginary;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
struct Complex {
    real: f64,
    imaginary: Imaginary,
}

impl Complex {
    pub fn new(real: f64, imaginary: f64) -> Complex {
        Complex {
            real,
            imaginary: Imaginary::new(imaginary),
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let real = self.real + other.real;
        let imaginary = self.imaginary + other.imaginary;
        Complex { real, imaginary }
    }
}

impl Sub<Complex> for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let real = self.real - other.real;
        let imaginary = self.imaginary - other.imaginary;
        Complex { real, imaginary }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let first = self.real * other.real;
        let inner = other.imaginary * self.real;
        let outer = self.imaginary * other.real;
        let last = self.imaginary * other.imaginary;

        Complex {
            real: first + last,
            imaginary: inner + outer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_numbers_are_multiplied_correctly() {
        let cn_1 = Complex::new(6.0, 3.0);
        let cn_2 = Complex::new(2.0, 5.0);

        let multiplied = cn_1 * cn_2;

        assert_eq!(multiplied, Complex::new(-3.0, 36.0));
    }

    #[test]
    fn complex_numbers_are_added_properly() {
        let cn_1 = Complex::new(8.0, 7.0);
        let cn_2 = Complex::new(4.0, 3.0);

        let added = cn_1 + cn_2;

        assert_eq!(added, Complex::new(12.0, 10.0));
    }

    #[test]
    fn complex_numbers_are_subtracted_properly() {
        let cn_1 = Complex::new(7.0, 9.0);
        let cn_2 = Complex::new(3.0, 4.0);

        let added = cn_1 - cn_2;

        assert_eq!(added, Complex::new(4.0, 5.0));
    }
}
