#[macro_use]
mod browser;
mod game;
mod engine;
mod segments;
mod sound;


use wasm_bindgen::prelude::*;

use engine::GameLoop;
use game::WalkTheDog;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    // Your code goes here!
    let context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });


    Ok(())
}
