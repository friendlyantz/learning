extern crate wasm_bindgen;

use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use std::str;

#[wasm_bindgen]
extern "C" {
    //fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // #[wasm_bindgen(method, structural, js_name = rustMine)]
    // fn rust_mine(text: &str, a: i32);
}

// #[wasm_bindgen(method, js_name = rustMine)]
#[wasm_bindgen]
pub fn wasm_mine(text: &str) {
    log(&rust_mine(text));
}

pub fn rust_mine(text: &str) -> String {
    let mut hasher = Sha256::new();
    (0..std::usize::MAX)
        .find_map(|nonce| {
            hasher.input(&format!("{}{}", text, nonce));
            let result = hasher.result_reset();
            // Ensure the first two hex digets match, then we only want to check the part of the
            // 3rd hex digit.
            //
            // 11 >> 4 # => 0
            // 12 >> 4 # => 0
            // 22 >> 4 # => 1
            if &result[0..2] == &[0, 0] && result[2] >> 4 == 0 {
                Some(format!("{:x}", result))
            } else {
                None
            }
        })
        .expect("unable to find answer")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            rust_mine("hello"),
            "0000037660ee0e22df67a053537e000325bbfad2cce9b8b7c795f6aa961d5cb7".to_string()
        );
    }
}
