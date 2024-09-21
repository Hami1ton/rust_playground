use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    return num + 1;
}