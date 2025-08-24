use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod game;
mod network;
mod systems;
mod components;
mod resources;
mod combat;
mod movement;
mod enemy;
mod room;

use game::GamePlugin;
use network::NetworkPlugin;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        // Simple console logging for WASM
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roguelike ARPG".to_string(),
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