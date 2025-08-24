use bevy::prelude::*;

pub struct InputSystem;

impl Plugin for InputSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_touch_input);
    }
}

fn handle_touch_input(
    touches: Res<Touches>,
    mut query: Query<&mut Transform, With<crate::game::LocalPlayer>>,
) {
    #[cfg(target_arch = "wasm32")]
    {
        for touch in touches.iter() {
            if let Some(mut transform) = query.iter_mut().next() {
                // Convert touch position to world coordinates
                let touch_pos = touch.position();
                // Simple touch-to-move logic
                transform.translation.x = touch_pos.x - 400.0; // Adjust for canvas center
                transform.translation.y = -(touch_pos.y - 300.0); // Invert Y and adjust
            }
        }
    }
}