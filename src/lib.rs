mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

use utils::set_panic_hook;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
