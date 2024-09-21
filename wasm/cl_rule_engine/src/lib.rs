use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(num: i32) -> i32 {
    return num + 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        assert_eq!(add_one(3), 4);
    }
}
