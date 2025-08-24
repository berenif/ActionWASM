use bevy::prelude::*;

pub struct RenderingSystem;

impl Plugin for RenderingSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_player_colors, animate_players));
    }
}

fn update_player_colors(
    mut query: Query<(&mut Sprite, &crate::game::Player), Changed<crate::game::Player>>,
) {
    for (mut sprite, player) in query.iter_mut() {
        sprite.color = player.color;
    }
}

fn animate_players(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &crate::game::Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        // Simple bobbing animation when moving
        if player.velocity.length() > 0.0 {
            let bobbing = (time.elapsed_seconds() * 10.0).sin() * 2.0;
            transform.scale.x = 1.0 + bobbing * 0.02;
            transform.scale.y = 1.0 - bobbing * 0.02;
        } else {
            // Reset scale when not moving
            transform.scale = Vec3::ONE;
        }
    }
}