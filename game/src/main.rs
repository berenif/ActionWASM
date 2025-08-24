use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod game;
mod network;
mod systems;

use game::GamePlugin;
use network::NetworkPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        wasm_bindgen_console_logger::init();
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Multiplayer WASM Game".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: true,
                canvas: Some("#game-canvas".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(NetworkPlugin)
        .run();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // This is required for WASM builds but won't be called
}