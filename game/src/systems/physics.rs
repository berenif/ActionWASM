use bevy::prelude::*;

pub struct PhysicsSystem;

impl Plugin for PhysicsSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_velocity, check_collisions).chain());
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &crate::game::Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        if player.velocity.length() > 0.0 {
            let delta = player.velocity * time.delta_seconds();
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
            
            // Keep within bounds
            transform.translation.x = transform.translation.x.clamp(-580.0, 580.0);
            transform.translation.y = transform.translation.y.clamp(-380.0, 380.0);
        }
    }
}

fn check_collisions(
    mut players: Query<(Entity, &Transform, &mut crate::game::Player)>,
) {
    let mut combinations = players.iter_combinations_mut();
    while let Some([(entity1, transform1, mut player1), (entity2, transform2, mut player2)]) = combinations.fetch_next() {
        let distance = transform1.translation.distance(transform2.translation);
        
        // Simple collision detection (30 pixel radius per player)
        if distance < 60.0 {
            // Bounce players apart
            let direction = (transform1.translation - transform2.translation).normalize();
            // This would need proper physics handling in a real game
        }
    }
}