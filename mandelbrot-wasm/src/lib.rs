use wasm_bindgen::prelude::*;

mod complex;
mod imaginary;

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    num + 1
}
