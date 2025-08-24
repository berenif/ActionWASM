use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_enemy_ai,
            enemy_movement,
            enemy_attack_system,
            update_telegraphs,
            spawn_enemy_projectiles,
            apply_elite_modifiers,
        ).chain().run_if(in_combat));
    }
}

fn in_combat(game_state: Res<GameState>) -> bool {
    matches!(game_state.current_state, CurrentGameState::InRun | CurrentGameState::BossFight)
        && !game_state.paused
}

fn update_enemy_ai(
    mut enemy_query: Query<(
        &mut EnemyAI,
        &mut Enemy,
        &Transform,
        &Health,
        Entity,
    ), Without<LocalPlayer>>,
    player_query: Query<(Entity, &Transform), With<LocalPlayer>>,
    time: Res<Time>,
) {
    let player = player_query.get_single();
    if player.is_err() {
        return; // No player to target
    }
    let (player_entity, player_transform) = player.unwrap();
    let player_pos = player_transform.translation.truncate();

    for (mut ai, enemy, transform, health, entity) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let distance_to_player = enemy_pos.distance(player_pos);

        // Update state timer
        ai.state_timer.tick(time.delta());

        // State machine logic
        match ai.state {
            AIState::Idle => {
                // Check if player is in aggro range
                if distance_to_player < enemy.aggro_range {
                    ai.state = AIState::Chasing;
                    ai.target = Some(player_entity);
                    ai.state_timer = Timer::from_seconds(0.5, TimerMode::Once);
                }
            }
            
            AIState::Patrolling => {
                // Patrol logic (simplified for now)
                if distance_to_player < enemy.aggro_range {
                    ai.state = AIState::Chasing;
                    ai.target = Some(player_entity);
                }
            }
            
            AIState::Chasing => {
                // Update last known position
                ai.last_known_position = Some(player_pos);
                
                // Check if in attack range
                if distance_to_player < enemy.attack_range {
                    // Start telegraph before attack
                    ai.state = AIState::Telegraphing;
                    ai.state_timer = Timer::from_seconds(
                        get_telegraph_duration(enemy.enemy_type),
                        TimerMode::Once
                    );
                } else if distance_to_player > enemy.aggro_range * 1.5 {
                    // Lost player, return to idle
                    ai.state = AIState::Idle;
                    ai.target = None;
                }
            }
            
            AIState::Telegraphing => {
                // Telegraph attack
                if ai.state_timer.finished() {
                    ai.state = AIState::Attacking;
                    ai.state_timer = Timer::from_seconds(0.3, TimerMode::Once);
                }
            }
            
            AIState::Attacking => {
                // Execute attack
                if ai.state_timer.finished() {
                    ai.state = AIState::Recovering;
                    ai.state_timer = Timer::from_seconds(
                        get_recovery_duration(enemy.enemy_type),
                        TimerMode::Once
                    );
                }
            }
            
            AIState::Recovering => {
                // Recovery after attack
                if ai.state_timer.finished() {
                    // Decide next action based on enemy behavior
                    match enemy.behavior {
                        EnemyBehavior::Aggressive => {
                            ai.state = AIState::Chasing;
                        }
                        EnemyBehavior::Defensive => {
                            if distance_to_player < enemy.attack_range * 0.7 {
                                ai.state = AIState::Fleeing;
                                ai.state_timer = Timer::from_seconds(1.0, TimerMode::Once);
                            } else {
                                ai.state = AIState::Chasing;
                            }
                        }
                        _ => {
                            ai.state = AIState::Chasing;
                        }
                    }
                }
            }
            
            AIState::Fleeing => {
                // Move away from player
                if ai.state_timer.finished() || distance_to_player > enemy.attack_range * 1.5 {
                    ai.state = AIState::Chasing;
                }
            }
        }

        // Check for low health behavior changes
        if health.percentage() < 0.3 && enemy.behavior != EnemyBehavior::Support {
            // Enemies become more aggressive when low on health
            if ai.state == AIState::Chasing {
                ai.state_timer = Timer::from_seconds(
                    ai.state_timer.duration().as_secs_f32() * 0.7,
                    TimerMode::Once
                );
            }
        }
    }
}

fn get_telegraph_duration(enemy_type: EnemyType) -> f32 {
    // From Tuning Tables
    match enemy_type {
        EnemyType::CommonMelee => 0.5,
        EnemyType::CommonRanged => 0.7,
        EnemyType::EliteMelee => 0.8,
        EnemyType::EliteRanged => 1.0,
        EnemyType::MiniBoss => 1.5,
        EnemyType::Boss => 2.0,
    }
}

fn get_recovery_duration(enemy_type: EnemyType) -> f32 {
    // From Tuning Tables
    match enemy_type {
        EnemyType::CommonMelee => 0.3,
        EnemyType::CommonRanged => 0.5,
        EnemyType::EliteMelee => 0.4,
        EnemyType::EliteRanged => 0.6,
        EnemyType::MiniBoss => 0.5,
        EnemyType::Boss => 1.0,
    }
}

fn enemy_movement(
    mut enemy_query: Query<(
        &mut Transform,
        &mut Velocity,
        &Enemy,
        &EnemyAI,
    ), Without<LocalPlayer>>,
    player_query: Query<&Transform, With<LocalPlayer>>,
    time: Res<Time>,
) {
    let player_transform = player_query.get_single();
    if player_transform.is_err() {
        return;
    }
    let player_pos = player_transform.unwrap().translation.truncate();

    for (mut transform, mut velocity, enemy, ai) in enemy_query.iter_mut() {
        let enemy_pos = transform.translation.truncate();
        let to_player = player_pos - enemy_pos;
        let distance = to_player.length();

        // Movement based on AI state
        let movement_speed = match enemy.enemy_type {
            EnemyType::CommonMelee => 150.0,
            EnemyType::CommonRanged => 100.0,
            EnemyType::EliteMelee => 200.0,
            EnemyType::EliteRanged => 120.0,
            EnemyType::MiniBoss => 180.0,
            EnemyType::Boss => 100.0,
        };

        match ai.state {
            AIState::Chasing => {
                // Move toward player
                if distance > enemy.attack_range * 0.9 {
                    let direction = to_player.normalize();
                    velocity.linear = direction * movement_speed;
                } else {
                    // Stop when in range
                    velocity.linear = Vec2::ZERO;
                }
            }
            
            AIState::Fleeing => {
                // Move away from player
                if distance < enemy.attack_range * 1.5 {
                    let direction = -to_player.normalize();
                    velocity.linear = direction * movement_speed * 1.2; // Flee faster
                } else {
                    velocity.linear = Vec2::ZERO;
                }
            }
            
            AIState::Patrolling => {
                // Simple patrol pattern
                let time_factor = time.elapsed_seconds();
                velocity.linear = Vec2::new(
                    (time_factor * 0.5).sin() * 50.0,
                    (time_factor * 0.3).cos() * 50.0,
                );
            }
            
            AIState::Telegraphing | AIState::Attacking | AIState::Recovering => {
                // Don't move during these states (or move slowly)
                velocity.linear *= 0.2;
            }
            
            _ => {
                velocity.linear = Vec2::ZERO;
            }
        }

        // Apply velocity
        transform.translation.x += velocity.linear.x * time.delta_seconds();
        transform.translation.y += velocity.linear.y * time.delta_seconds();
    }
}

fn enemy_attack_system(
    mut commands: Commands,
    enemy_query: Query<(
        Entity,
        &Transform,
        &Enemy,
        &EnemyAI,
        &CombatStats,
    )>,
    player_query: Query<&Transform, With<LocalPlayer>>,
) {
    let player_transform = player_query.get_single();
    if player_transform.is_err() {
        return;
    }
    let player_pos = player_transform.unwrap().translation.truncate();

    for (entity, transform, enemy, ai, stats) in enemy_query.iter() {
        if ai.state != AIState::Attacking {
            continue;
        }

        let enemy_pos = transform.translation.truncate();
        let to_player = player_pos - enemy_pos;

        match enemy.enemy_type {
            EnemyType::CommonMelee | EnemyType::EliteMelee => {
                // Spawn melee hitbox
                commands.spawn((
                    Hitbox {
                        size: Vec2::new(40.0, 40.0),
                        offset: to_player.normalize() * 30.0,
                        damage: stats.damage,
                        knockback: 30.0,
                        active: true,
                        hit_entities: Vec::new(),
                    },
                    TransformBundle::from_transform(
                        Transform::from_translation(transform.translation)
                    ),
                    EnemyAttack { owner: entity },
                ));
            }
            
            EnemyType::CommonRanged | EnemyType::EliteRanged => {
                // Spawn projectile
                let direction = to_player.normalize();
                commands.spawn((
                    Projectile {
                        damage: stats.damage,
                        speed: 300.0,
                        lifetime: Timer::from_seconds(3.0, TimerMode::Once),
                        piercing: 0,
                        owner: entity,
                    },
                    Velocity {
                        linear: direction * 300.0,
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.5, 0.0),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                ));
            }
            
            _ => {}
        }
    }
}

#[derive(Component)]
struct EnemyAttack {
    owner: Entity,
}

fn update_telegraphs(
    mut commands: Commands,
    mut telegraph_query: Query<(Entity, &mut Telegraph, &mut Sprite)>,
    enemy_query: Query<(&Transform, &Enemy, &EnemyAI), Changed<EnemyAI>>,
    time: Res<Time>,
) {
    // Update existing telegraphs
    for (entity, mut telegraph, mut sprite) in telegraph_query.iter_mut() {
        telegraph.duration.tick(time.delta());
        
        // Fade in/out effect
        let progress = telegraph.duration.fraction();
        let alpha = if progress < 0.2 {
            // Fade in
            progress * 5.0
        } else if progress > 0.8 {
            // Flash warning
            if (progress * 10.0) as i32 % 2 == 0 {
                1.0
            } else {
                0.5
            }
        } else {
            // Full visibility
            1.0
        };
        
        sprite.color.set_a(alpha);
        
        if telegraph.duration.finished() {
            commands.entity(entity).despawn();
        }
    }

    // Spawn new telegraphs
    for (transform, enemy, ai) in enemy_query.iter() {
        if ai.state == AIState::Telegraphing {
            let (size, color, telegraph_type) = match enemy.enemy_type {
                EnemyType::CommonMelee => (Vec2::new(60.0, 60.0), Color::rgba(1.0, 0.0, 0.0, 0.5), TelegraphType::MeleeSwing),
                EnemyType::CommonRanged => (Vec2::new(20.0, 200.0), Color::rgba(1.0, 1.0, 0.0, 0.5), TelegraphType::RangedShot),
                EnemyType::EliteMelee => (Vec2::new(80.0, 80.0), Color::rgba(1.0, 0.0, 0.0, 0.5), TelegraphType::MeleeSwing),
                EnemyType::EliteRanged => (Vec2::new(30.0, 300.0), Color::rgba(1.0, 0.5, 0.0, 0.5), TelegraphType::RangedShot),
                _ => continue,
            };

            commands.spawn((
                Telegraph {
                    duration: Timer::from_seconds(get_telegraph_duration(enemy.enemy_type), TimerMode::Once),
                    telegraph_type,
                    damage_area: size,
                    color,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(size),
                        ..default()
                    },
                    transform: Transform::from_translation(transform.translation),
                    ..default()
                },
            ));
        }
    }
}

fn spawn_enemy_projectiles(
    mut projectile_query: Query<(&mut Transform, &mut Velocity, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, velocity, projectile) in projectile_query.iter_mut() {
        // Move projectile
        transform.translation.x += velocity.linear.x * time.delta_seconds();
        transform.translation.y += velocity.linear.y * time.delta_seconds();
    }
}

fn apply_elite_modifiers(
    mut enemy_query: Query<(&mut Health, &mut CombatStats, &Enemy)>,
    time: Res<Time>,
) {
    // This would apply elite modifier effects
    // For now, just a placeholder
    for (mut health, mut stats, enemy) in enemy_query.iter_mut() {
        if matches!(enemy.enemy_type, EnemyType::EliteMelee | EnemyType::EliteRanged) {
            // Elite enemies could have special properties
            // This would be expanded based on the EliteModifier enum
        }
    }
}

// Spawn functions for different enemy types
pub fn spawn_melee_enemy(
    commands: &mut Commands,
    position: Vec2,
    enemy_type: EnemyType,
    room_number: u32,
) -> Entity {
    // Scale health based on room number (from Tuning Tables)
    let base_health = match enemy_type {
        EnemyType::CommonMelee => 200.0 + (room_number as f32 * 20.0),
        EnemyType::EliteMelee => 1000.0 + (room_number as f32 * 200.0),
        _ => 200.0,
    };

    let damage = match enemy_type {
        EnemyType::CommonMelee => 20.0 + (room_number as f32 * 2.0),
        EnemyType::EliteMelee => 40.0 + (room_number as f32 * 4.0),
        _ => 20.0,
    };

    commands.spawn((
        Enemy {
            enemy_type,
            behavior: EnemyBehavior::Aggressive,
            aggro_range: 300.0,
            attack_range: 50.0,
        },
        EnemyAI {
            state: AIState::Idle,
            target: None,
            last_known_position: None,
            state_timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
        Health::new(base_health),
        CombatStats {
            damage,
            crit_chance: 0.05,
            crit_damage: 1.5,
            attack_speed: 1.0,
            armor: 0.0,
            damage_reduction: 0.0,
        },
        Velocity::default(),
        Hurtbox {
            size: Vec2::new(30.0, 30.0),
            invulnerable: false,
        },
        SpriteBundle {
            sprite: Sprite {
                color: if matches!(enemy_type, EnemyType::EliteMelee) {
                    Color::rgb(0.8, 0.2, 0.2)
                } else {
                    Color::rgb(0.6, 0.2, 0.2)
                },
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
    )).id()
}

pub fn spawn_ranged_enemy(
    commands: &mut Commands,
    position: Vec2,
    enemy_type: EnemyType,
    room_number: u32,
) -> Entity {
    let base_health = match enemy_type {
        EnemyType::CommonRanged => 150.0 + (room_number as f32 * 15.0),
        EnemyType::EliteRanged => 800.0 + (room_number as f32 * 150.0),
        _ => 150.0,
    };

    let damage = match enemy_type {
        EnemyType::CommonRanged => 15.0 + (room_number as f32 * 1.5),
        EnemyType::EliteRanged => 30.0 + (room_number as f32 * 3.0),
        _ => 15.0,
    };

    commands.spawn((
        Enemy {
            enemy_type,
            behavior: EnemyBehavior::Defensive,
            aggro_range: 400.0,
            attack_range: 200.0,
        },
        EnemyAI {
            state: AIState::Idle,
            target: None,
            last_known_position: None,
            state_timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
        Health::new(base_health),
        CombatStats {
            damage,
            crit_chance: 0.1,
            crit_damage: 2.0,
            attack_speed: 0.8,
            armor: 0.0,
            damage_reduction: 0.0,
        },
        Velocity::default(),
        Hurtbox {
            size: Vec2::new(25.0, 25.0),
            invulnerable: false,
        },
        SpriteBundle {
            sprite: Sprite {
                color: if matches!(enemy_type, EnemyType::EliteRanged) {
                    Color::rgb(0.8, 0.8, 0.2)
                } else {
                    Color::rgb(0.6, 0.6, 0.2)
                },
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
    )).id()
}