use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Imaginary {
    number: f64,
}

impl Imaginary {
    pub fn new(number: f64) -> Imaginary {
        Imaginary { number }
    }
}

impl Add<Imaginary> for Imaginary {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Imaginary {
            number: self.number + other.number,
        }
    }
}

impl Sub<Imaginary> for Imaginary {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Imaginary {
            number: self.number - other.number,
        }
    }
}

impl Mul<Imaginary> for Imaginary {
    type Output = f64;

    fn mul(self, other: Self) -> f64 {
        self.number * other.number * -1.0
    }
}

impl Mul<f64> for Imaginary {
    type Output = Imaginary;

    fn mul(self, other: f64) -> Self {
        Imaginary {
            number: self.number * other,
        }
    }
}
