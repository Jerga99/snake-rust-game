
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
   alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// wasm-pack build --target web
