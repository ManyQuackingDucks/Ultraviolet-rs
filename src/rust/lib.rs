extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rustsub(a: i32, b: i32) -> i32 {
    a - b
}