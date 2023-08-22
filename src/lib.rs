#[macro_use]
mod browser;
mod engine;
mod game;
mod utils;

use engine::GameLoop;
use game::WalkTheDog;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game llop");
    });

    Ok(())
}
