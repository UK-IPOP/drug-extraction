mod utils;

use drug_extraction;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, drug-extract-wasm!");
}

#[wasm_bindgen]
pub fn namer() -> String {
    let character = drug_extraction::saiyan();
    return format!("My favorite saiyan is: {}!", character);
}
