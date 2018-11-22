extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern { pub fn alert(s: &str); }

#[wasm_bindgen(js_namespace = console)]
extern { pub fn log(s: &str); }

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(&format!("Hello, {}!", name));
}

