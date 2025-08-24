use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_movement_input,
            handle_dash_input,
            update_dash_state,
            apply_velocity,
            clamp_to_arena,
        ).chain().run_if(in_game));
    }
}

fn in_game(game_state: Res<GameState>) -> bool {
    matches!(game_state.current_state, CurrentGameState::InRun)
        && !game_state.paused
}

fn handle_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<(
        &mut Velocity,
        &MovementStats,
        &DashState,
    ), With<LocalPlayer>>,
) {
    for (mut velocity, stats, dash_state) in query.iter_mut() {
        // Skip movement input while dashing
        if dash_state.is_dashing {
            continue;
        }

        let mut movement = Vec2::ZERO;

        // Keyboard input (8-directional)
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            movement.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            movement.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            movement.x += 1.0;
        }

        // Gamepad input
        for gamepad in gamepads.iter() {
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap_or(0.0);
            let left_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
                .unwrap_or(0.0);

            // Apply deadzone
            if left_stick_x.abs() > 0.15 {
                movement.x += left_stick_x;
            }
            if left_stick_y.abs() > 0.15 {
                movement.y += left_stick_y;
            }
        }

        // Normalize diagonal movement to maintain consistent speed
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        velocity.linear = movement * stats.current_speed;
    }
}

fn handle_dash_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    time: Res<Time>,
    mut query: Query<(
        &mut DashState,
        &mut Velocity,
        &MovementStats,
        &mut Hurtbox,
        &mut InputBuffer,
    ), With<LocalPlayer>>,
) {
    for (mut dash_state, mut velocity, stats, mut hurtbox, mut input_buffer) in query.iter_mut() {
        // Check if dash is on cooldown
        if !dash_state.cooldown_timer.finished() {
            continue;
        }

        let mut should_dash = false;

        // Keyboard dash input (Space or Shift)
        if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::ShiftLeft) {
            should_dash = true;
        }

        // Gamepad dash input (A button on Xbox, X on PlayStation)
        for gamepad in gamepads.iter() {
            if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                should_dash = true;
            }
        }

        // Check input buffer for dash
        input_buffer.buffer.retain(|buffered| {
            if buffered.action == InputAction::Dash 
                && time.elapsed_seconds() - buffered.timestamp < input_buffer.max_buffer_time {
                should_dash = true;
                false // Remove from buffer
            } else {
                true // Keep in buffer
            }
        });

        if should_dash && !dash_state.is_dashing {
            // Start dash
            dash_state.is_dashing = true;
            dash_state.dash_timer.reset();
            
            // Dash in current movement direction, or last facing direction
            let dash_dir = if velocity.linear.length() > 0.0 {
                velocity.linear.normalize()
            } else {
                Vec2::new(1.0, 0.0) // Default to right if standing still
            };
            
            dash_state.dash_direction = dash_dir;
            velocity.linear = dash_dir * stats.dash_speed;
            
            // Enable i-frames during dash
            if dash_state.has_iframes {
                hurtbox.invulnerable = true;
            }
        }
    }
}

fn update_dash_state(
    time: Res<Time>,
    mut query: Query<(
        &mut DashState,
        &mut Velocity,
        &MovementStats,
        &mut Hurtbox,
    ), With<LocalPlayer>>,
) {
    for (mut dash_state, mut velocity, stats, mut hurtbox) in query.iter_mut() {
        // Update dash timer
        if dash_state.is_dashing {
            dash_state.dash_timer.tick(time.delta());
            
            if dash_state.dash_timer.finished() {
                // End dash
                dash_state.is_dashing = false;
                dash_state.cooldown_timer.reset();
                
                // Remove i-frames
                hurtbox.invulnerable = false;
                
                // Return to normal movement
                velocity.linear = Vec2::ZERO;
            } else {
                // Maintain dash velocity
                velocity.linear = dash_state.dash_direction * stats.dash_speed;
            }
        }
        
        // Update cooldown timer
        if !dash_state.cooldown_timer.finished() {
            dash_state.cooldown_timer.tick(time.delta());
        }
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.linear.x * time.delta_seconds();
        transform.translation.y += velocity.linear.y * time.delta_seconds();
    }
}

fn clamp_to_arena(
    mut query: Query<&mut Transform, With<Player>>,
) {
    // Arena boundaries (adjust based on your arena size)
    const ARENA_HALF_WIDTH: f32 = 600.0;
    const ARENA_HALF_HEIGHT: f32 = 400.0;
    const PLAYER_RADIUS: f32 = 16.0;

    for mut transform in query.iter_mut() {
        // Clamp X position
        transform.translation.x = transform.translation.x.clamp(
            -ARENA_HALF_WIDTH + PLAYER_RADIUS,
            ARENA_HALF_WIDTH - PLAYER_RADIUS,
        );
        
        // Clamp Y position
        transform.translation.y = transform.translation.y.clamp(
            -ARENA_HALF_HEIGHT + PLAYER_RADIUS,
            ARENA_HALF_HEIGHT - PLAYER_RADIUS,
        );
    }
}

// System to handle input buffering
pub fn buffer_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut InputBuffer, With<LocalPlayer>>,
) {
    for mut input_buffer in query.iter_mut() {
        // Buffer attack inputs
        if keyboard.just_pressed(KeyCode::KeyJ) {
            input_buffer.buffer.push(BufferedInput {
                action: InputAction::LightAttack,
                timestamp: time.elapsed_seconds(),
            });
        }
        
        if keyboard.just_pressed(KeyCode::KeyK) {
            input_buffer.buffer.push(BufferedInput {
                action: InputAction::HeavyAttack,
                timestamp: time.elapsed_seconds(),
            });
        }
        
        if keyboard.just_pressed(KeyCode::Space) {
            input_buffer.buffer.push(BufferedInput {
                action: InputAction::Dash,
                timestamp: time.elapsed_seconds(),
            });
        }
        
        // Clean up old buffered inputs
        let current_time = time.elapsed_seconds();
        input_buffer.buffer.retain(|buffered| {
            current_time - buffered.timestamp < input_buffer.max_buffer_time
        });
    }
}