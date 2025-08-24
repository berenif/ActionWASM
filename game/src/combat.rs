use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;
use crate::movement::buffer_input;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            buffer_input,
            handle_attack_input,
            update_attack_state,
            spawn_hitboxes,
            check_hit_detection,
            apply_damage,
            spawn_damage_numbers,
            update_damage_numbers,
            handle_death,
            apply_knockback,
        ).chain().run_if(in_combat));
    }
}

fn in_combat(game_state: Res<GameState>) -> bool {
    matches!(game_state.current_state, CurrentGameState::InRun | CurrentGameState::BossFight)
        && !game_state.paused
}

fn handle_attack_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<ButtonInput<GamepadButton>>,
    time: Res<Time>,
    mut query: Query<(
        &mut AttackState,
        &mut InputBuffer,
        &CombatStats,
        &DashState,
    ), With<LocalPlayer>>,
) {
    for (mut attack_state, mut input_buffer, stats, dash_state) in query.iter_mut() {
        // Can't attack while dashing (unless it's a dash attack)
        if dash_state.is_dashing && attack_state.attack_type != AttackType::DashAttack {
            continue;
        }

        // Check if we can start a new attack
        if attack_state.is_attacking && !attack_state.can_cancel {
            continue;
        }

        let mut new_attack = None;

        // Check keyboard input
        if keyboard.just_pressed(KeyCode::KeyJ) {
            new_attack = Some(AttackType::LightAttack);
        } else if keyboard.just_pressed(KeyCode::KeyK) {
            new_attack = Some(AttackType::HeavyAttack);
        }

        // Check gamepad input
        for gamepad in gamepads.iter() {
            if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West)) {
                new_attack = Some(AttackType::LightAttack);
            } else if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                new_attack = Some(AttackType::HeavyAttack);
            }
        }

        // Check input buffer
        let current_time = time.elapsed_seconds();
        input_buffer.buffer.retain(|buffered| {
            if new_attack.is_none() && current_time - buffered.timestamp < input_buffer.max_buffer_time {
                match buffered.action {
                    InputAction::LightAttack => {
                        new_attack = Some(AttackType::LightAttack);
                        false
                    }
                    InputAction::HeavyAttack => {
                        new_attack = Some(AttackType::HeavyAttack);
                        false
                    }
                    _ => true,
                }
            } else {
                true
            }
        });

        if let Some(attack_type) = new_attack {
            start_attack(&mut attack_state, attack_type, stats);
        }
    }
}

fn start_attack(attack_state: &mut AttackState, attack_type: AttackType, stats: &CombatStats) {
    attack_state.is_attacking = true;
    attack_state.attack_type = attack_type;

    // Set timers based on attack type and attack speed stat
    // Using frame data from Tuning Tables
    let speed_multiplier = 1.0 / stats.attack_speed.max(0.1);
    
    match attack_type {
        AttackType::LightAttack => {
            // Light attack: 8f startup, 4f active, 6f recovery (0.3s total)
            attack_state.startup_timer = Timer::from_seconds(0.133 * speed_multiplier, TimerMode::Once);
            attack_state.active_timer = Timer::from_seconds(0.067 * speed_multiplier, TimerMode::Once);
            attack_state.recovery_timer = Timer::from_seconds(0.1 * speed_multiplier, TimerMode::Once);
            attack_state.can_cancel = false; // Can cancel after frame 12
            
            // Combo system
            if attack_state.combo_count < 3 {
                attack_state.combo_count += 1;
            } else {
                attack_state.combo_count = 1;
            }
        }
        AttackType::HeavyAttack => {
            // Heavy attack: 20f startup, 8f active, 20f recovery (0.8s total)
            attack_state.startup_timer = Timer::from_seconds(0.333 * speed_multiplier, TimerMode::Once);
            attack_state.active_timer = Timer::from_seconds(0.133 * speed_multiplier, TimerMode::Once);
            attack_state.recovery_timer = Timer::from_seconds(0.333 * speed_multiplier, TimerMode::Once);
            attack_state.can_cancel = false; // Can cancel after frame 28
            attack_state.combo_count = 0; // Heavy attack resets combo
        }
        AttackType::DashAttack => {
            // Dash attack: Quick strike during dash
            attack_state.startup_timer = Timer::from_seconds(0.05 * speed_multiplier, TimerMode::Once);
            attack_state.active_timer = Timer::from_seconds(0.1 * speed_multiplier, TimerMode::Once);
            attack_state.recovery_timer = Timer::from_seconds(0.05 * speed_multiplier, TimerMode::Once);
            attack_state.can_cancel = true;
            attack_state.combo_count = 0;
        }
        AttackType::Special => {
            // Special attack: Skill cast timing
            attack_state.startup_timer = Timer::from_seconds(0.167 * speed_multiplier, TimerMode::Once);
            attack_state.active_timer = Timer::from_seconds(0.083 * speed_multiplier, TimerMode::Once);
            attack_state.recovery_timer = Timer::from_seconds(0.25 * speed_multiplier, TimerMode::Once);
            attack_state.can_cancel = false;
            attack_state.combo_count = 0;
        }
    }

    // Reset all timers
    attack_state.startup_timer.reset();
    attack_state.active_timer.reset();
    attack_state.recovery_timer.reset();
}

fn update_attack_state(
    time: Res<Time>,
    mut query: Query<&mut AttackState>,
) {
    for mut attack_state in query.iter_mut() {
        if !attack_state.is_attacking {
            continue;
        }

        // Progress through attack phases
        if !attack_state.startup_timer.finished() {
            attack_state.startup_timer.tick(time.delta());
            
            // Check for cancel window (80% of animations can be cancelled)
            if attack_state.startup_timer.fraction() > 0.8 {
                attack_state.can_cancel = true;
            }
        } else if !attack_state.active_timer.finished() {
            attack_state.active_timer.tick(time.delta());
        } else if !attack_state.recovery_timer.finished() {
            attack_state.recovery_timer.tick(time.delta());
            attack_state.can_cancel = true; // Can always cancel during recovery
        } else {
            // Attack complete
            attack_state.is_attacking = false;
            attack_state.can_cancel = false;
        }
    }
}

fn spawn_hitboxes(
    mut commands: Commands,
    query: Query<(Entity, &AttackState, &Transform, &CombatStats), Changed<AttackState>>,
) {
    for (entity, attack_state, transform, stats) in query.iter() {
        if attack_state.is_attacking && attack_state.startup_timer.finished() && !attack_state.active_timer.finished() {
            // Spawn hitbox based on attack type
            let (size, offset, damage_mult, knockback) = match attack_state.attack_type {
                AttackType::LightAttack => {
                    let combo_mult = 1.0 + (attack_state.combo_count as f32 * 0.2);
                    (Vec2::new(60.0, 40.0), Vec2::new(40.0, 0.0), combo_mult, 50.0)
                }
                AttackType::HeavyAttack => {
                    (Vec2::new(80.0, 60.0), Vec2::new(50.0, 0.0), 2.0, 150.0)
                }
                AttackType::DashAttack => {
                    (Vec2::new(50.0, 30.0), Vec2::new(30.0, 0.0), 1.5, 100.0)
                }
                AttackType::Special => {
                    (Vec2::new(100.0, 100.0), Vec2::new(0.0, 0.0), 3.0, 200.0)
                }
            };

            commands.spawn((
                Hitbox {
                    size,
                    offset,
                    damage: stats.damage * damage_mult,
                    knockback,
                    active: true,
                    hit_entities: Vec::new(),
                },
                TransformBundle::from_transform(
                    Transform::from_translation(transform.translation + offset.extend(0.0))
                ),
                AttackHitbox { owner: entity },
            ));
        }
    }
}

#[derive(Component)]
struct AttackHitbox {
    owner: Entity,
}

fn check_hit_detection(
    mut hitbox_query: Query<(&mut Hitbox, &Transform, &AttackHitbox)>,
    hurtbox_query: Query<(Entity, &Hurtbox, &Transform, Option<&Enemy>), Without<AttackHitbox>>,
    player_query: Query<Entity, With<LocalPlayer>>,
) {
    for (mut hitbox, hitbox_transform, attack_hitbox) in hitbox_query.iter_mut() {
        if !hitbox.active {
            continue;
        }

        let hitbox_pos = hitbox_transform.translation.truncate();

        for (target_entity, hurtbox, hurtbox_transform, maybe_enemy) in hurtbox_query.iter() {
            // Don't hit yourself
            if target_entity == attack_hitbox.owner {
                continue;
            }

            // Don't hit invulnerable targets
            if hurtbox.invulnerable {
                continue;
            }

            // Don't hit the same entity twice with the same hitbox
            if hitbox.hit_entities.contains(&target_entity) {
                continue;
            }

            let hurtbox_pos = hurtbox_transform.translation.truncate();

            // Simple AABB collision detection
            if check_aabb_collision(
                hitbox_pos,
                hitbox.size,
                hurtbox_pos,
                hurtbox.size,
            ) {
                // Register hit
                hitbox.hit_entities.push(target_entity);
            }
        }
    }
}

fn check_aabb_collision(pos1: Vec2, size1: Vec2, pos2: Vec2, size2: Vec2) -> bool {
    let half_size1 = size1 / 2.0;
    let half_size2 = size2 / 2.0;

    (pos1.x - half_size1.x < pos2.x + half_size2.x)
        && (pos1.x + half_size1.x > pos2.x - half_size2.x)
        && (pos1.y - half_size1.y < pos2.y + half_size2.y)
        && (pos1.y + half_size1.y > pos2.y - half_size2.y)
}

fn apply_damage(
    mut commands: Commands,
    hitbox_query: Query<(&Hitbox, &AttackHitbox)>,
    mut target_query: Query<(Entity, &mut Health, &Transform, &CombatStats, Option<&Enemy>)>,
    attacker_query: Query<&CombatStats>,
    mut combat_log: ResMut<CombatLog>,
    mut run_stats: ResMut<RunStats>,
    time: Res<Time>,
) {
    for (hitbox, attack_hitbox) in hitbox_query.iter() {
        let attacker_stats = attacker_query.get(attack_hitbox.owner).unwrap();

        for &hit_entity in hitbox.hit_entities.iter() {
            if let Ok((entity, mut health, transform, defender_stats, maybe_enemy)) = target_query.get_mut(hit_entity) {
                // Calculate damage
                let mut damage = hitbox.damage;

                // Critical hit calculation
                let mut rng = rand::thread_rng();
                let is_crit = rng.gen::<f32>() < attacker_stats.crit_chance;
                if is_crit {
                    damage *= attacker_stats.crit_damage;
                }

                // Apply armor and damage reduction
                damage = (damage - defender_stats.armor).max(1.0);
                damage *= 1.0 - defender_stats.damage_reduction;

                // Apply damage
                let died = health.take_damage(damage);

                // Update stats
                run_stats.damage_dealt += damage;

                // Log damage event
                combat_log.recent_damage.push(DamageEvent {
                    source: "Player".to_string(),
                    target: if maybe_enemy.is_some() { "Enemy".to_string() } else { "Unknown".to_string() },
                    amount: damage,
                    is_crit,
                    damage_type: DamageType::Physical,
                    timestamp: time.elapsed_seconds(),
                });

                // Spawn damage number
                commands.spawn((
                    DamageNumber {
                        amount: damage,
                        is_crit,
                        lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                        velocity: Vec2::new(rng.gen_range(-50.0..50.0), 100.0),
                    },
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{}", damage as i32),
                            TextStyle {
                                font_size: if is_crit { 32.0 } else { 24.0 },
                                color: if is_crit { Color::YELLOW } else { Color::WHITE },
                                ..default()
                            },
                        ),
                        transform: Transform::from_translation(transform.translation + Vec3::new(0.0, 20.0, 10.0)),
                        ..default()
                    },
                ));

                if died {
                    // Handle death in separate system
                    commands.entity(entity).insert(Dead);
                }
            }
        }
    }
}

#[derive(Component)]
struct Dead;

fn spawn_damage_numbers(
    // Implementation handled in apply_damage for now
) {
    // This could be expanded to handle different types of damage numbers
}

fn update_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Transform, &mut Text)>,
) {
    for (entity, mut damage_num, mut transform, mut text) in query.iter_mut() {
        damage_num.lifetime.tick(time.delta());

        // Float upward and fade
        transform.translation.y += damage_num.velocity.y * time.delta_seconds();
        transform.translation.x += damage_num.velocity.x * time.delta_seconds();
        
        // Slow down over time
        damage_num.velocity *= 0.95;

        // Fade out
        let alpha = 1.0 - damage_num.lifetime.fraction();
        if let Some(section) = text.sections.first_mut() {
            section.style.color.set_a(alpha);
        }

        // Remove when expired
        if damage_num.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Enemy>, Option<&LocalPlayer>), With<Dead>>,
    mut game_state: ResMut<GameState>,
    mut run_stats: ResMut<RunStats>,
) {
    for (entity, health, maybe_enemy, maybe_player) in query.iter() {
        if let Some(_enemy) = maybe_enemy {
            // Enemy died
            run_stats.enemies_killed += 1;
            
            // Spawn death effect
            // TODO: Add particle effects, loot drops
            
            // Despawn enemy
            commands.entity(entity).despawn_recursive();
        } else if let Some(_player) = maybe_player {
            // Player died
            game_state.current_state = CurrentGameState::Death;
            run_stats.deaths += 1;
            
            // Don't despawn player, show death screen instead
        }
    }
}

fn apply_knockback(
    hitbox_query: Query<(&Hitbox, &Transform, &AttackHitbox)>,
    mut target_query: Query<(&mut Velocity, &Transform), Without<AttackHitbox>>,
) {
    for (hitbox, hitbox_transform, _) in hitbox_query.iter() {
        for &hit_entity in hitbox.hit_entities.iter() {
            if let Ok((mut velocity, target_transform)) = target_query.get_mut(hit_entity) {
                // Calculate knockback direction
                let direction = (target_transform.translation.truncate() - hitbox_transform.translation.truncate()).normalize();
                
                // Apply knockback
                velocity.linear += direction * hitbox.knockback;
            }
        }
    }
}

// Cleanup system for hitboxes
pub fn cleanup_hitboxes(
    mut commands: Commands,
    query: Query<(Entity, &AttackHitbox)>,
    attack_query: Query<&AttackState>,
) {
    for (entity, attack_hitbox) in query.iter() {
        if let Ok(attack_state) = attack_query.get(attack_hitbox.owner) {
            if !attack_state.is_attacking || attack_state.active_timer.finished() {
                commands.entity(entity).despawn();
            }
        } else {
            // Owner doesn't exist anymore
            commands.entity(entity).despawn();
        }
    }
}