use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello(hello_str: String) {
    gloo_console::info!(hello_str);
}
