use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::resources::*;
use crate::enemy::{spawn_melee_enemy, spawn_ranged_enemy};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RoomGenerator>()
            .add_systems(Startup, setup_first_room)
            .add_systems(Update, (
                check_room_clear,
                handle_door_interaction,
                room_transition,
                spawn_room_rewards,
                update_room_hazards,
            ).chain());
    }
}

impl Default for RoomGenerator {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            rooms_generated: 0,
        }
    }
}

fn setup_first_room(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut room_gen: ResMut<RoomGenerator>,
) {
    // Generate the first room (tutorial room)
    let room_entity = generate_room(
        &mut commands,
        RoomType::Safe,
        1,
        BiomeType::Tutorial,
        &mut room_gen,
    );
    
    // Set up initial room state
    commands.insert_resource(CurrentRoom {
        entity: room_entity,
        room_type: RoomType::Safe,
        enemies_remaining: 0,
        doors_locked: false,
        spawn_points: vec![
            Vec2::new(-200.0, 0.0),
            Vec2::new(200.0, 0.0),
            Vec2::new(0.0, 150.0),
            Vec2::new(0.0, -150.0),
        ],
        hazards: Vec::new(),
    });
    
    // Spawn player in first room
    spawn_player(&mut commands, Vec2::ZERO);
}

fn generate_room(
    commands: &mut Commands,
    room_type: RoomType,
    room_number: u32,
    biome: BiomeType,
    room_gen: &mut RoomGenerator,
) -> Entity {
    room_gen.rooms_generated += 1;
    
    // Determine room properties based on type and progression
    let (enemy_count, exits, rewards) = match room_type {
        RoomType::Combat => {
            // Scale enemies based on room number (from Tuning Tables)
            let count = match room_number {
                1..=3 => rand::thread_rng().gen_range(2..=4),
                4..=6 => rand::thread_rng().gen_range(3..=6),
                7..=10 => rand::thread_rng().gen_range(4..=8),
                11..=15 => rand::thread_rng().gen_range(5..=10),
                16..=20 => rand::thread_rng().gen_range(6..=12),
                _ => rand::thread_rng().gen_range(8..=15),
            };
            
            let exits = generate_exits();
            let rewards = vec![
                RewardType::Gold(20 + room_number * 5),
                RewardType::Boon(BoonRarity::Common),
            ];
            
            (count, exits, rewards)
        }
        
        RoomType::Elite => {
            let count = (room_number / 5 + 1).min(3);
            let exits = generate_exits();
            let rewards = vec![
                RewardType::Gold(60 + room_number * 10),
                RewardType::Boon(BoonRarity::Rare),
            ];
            
            (count, exits, rewards)
        }
        
        RoomType::Shop => {
            let exits = generate_exits();
            (0, exits, vec![])
        }
        
        RoomType::Treasure => {
            let exits = generate_exits();
            let rewards = vec![
                RewardType::Gold(100 + room_number * 15),
                RewardType::Boon(BoonRarity::Epic),
            ];
            
            (0, exits, rewards)
        }
        
        RoomType::Boss => {
            let exits = vec![Direction::North]; // Single exit after boss
            let rewards = vec![
                RewardType::Gold(300 + room_number * 20),
                RewardType::Boon(BoonRarity::Legendary),
                RewardType::Heal(1.0), // Full heal
            ];
            
            (1, exits, rewards) // 1 boss enemy
        }
        
        RoomType::Secret => {
            let exits = generate_exits();
            let rewards = vec![
                RewardType::Gold(150 + room_number * 20),
                RewardType::Boon(BoonRarity::Epic),
            ];
            
            (0, exits, rewards)
        }
        
        RoomType::Safe => {
            let exits = generate_exits();
            (0, exits, vec![RewardType::Heal(0.25)])
        }
    };
    
    // Create room entity
    let room_entity = commands.spawn((
        Room {
            room_type,
            cleared: false,
            enemy_count,
            exits: exits.clone(),
            rewards,
        },
        TransformBundle::default(),
        VisibilityBundle::default(),
    )).id();
    
    // Spawn room geometry (walls, floor, etc.)
    spawn_room_geometry(commands, room_entity, &biome);
    
    // Spawn doors
    for direction in exits {
        spawn_door(commands, room_entity, direction, room_type);
    }
    
    // Spawn enemies if combat room
    if enemy_count > 0 && room_type != RoomType::Boss {
        spawn_room_enemies(commands, room_entity, enemy_count, room_number, room_type);
    }
    
    // Spawn hazards based on biome
    if should_spawn_hazards(biome, room_number) {
        spawn_room_hazards(commands, room_entity, biome);
    }
    
    room_entity
}

fn generate_exits() -> Vec<Direction> {
    let mut rng = rand::thread_rng();
    let mut exits = vec![Direction::North]; // Always have at least one exit
    
    // Randomly add more exits (2-3 total exits common)
    if rng.gen_bool(0.7) {
        exits.push(Direction::East);
    }
    if rng.gen_bool(0.5) {
        exits.push(Direction::West);
    }
    if rng.gen_bool(0.3) {
        exits.push(Direction::South);
    }
    
    exits
}

fn spawn_room_geometry(
    commands: &mut Commands,
    room_entity: Entity,
    biome: &BiomeType,
) {
    // Room dimensions
    const ROOM_WIDTH: f32 = 1200.0;
    const ROOM_HEIGHT: f32 = 800.0;
    const WALL_THICKNESS: f32 = 20.0;
    
    let wall_color = match biome {
        BiomeType::Tutorial => Color::rgb(0.4, 0.4, 0.4),
        BiomeType::Biome1 => Color::rgb(0.5, 0.3, 0.2),
        BiomeType::Biome2 => Color::rgb(0.3, 0.5, 0.3),
        BiomeType::Biome3 => Color::rgb(0.3, 0.3, 0.5),
        BiomeType::Biome4 => Color::rgb(0.5, 0.2, 0.5),
        BiomeType::Biome5 => Color::rgb(0.6, 0.5, 0.3),
    };
    
    // Top wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(ROOM_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, ROOM_HEIGHT / 2.0, 0.0)),
            ..default()
        },
        Wall,
    )).set_parent(room_entity);
    
    // Bottom wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(ROOM_WIDTH, WALL_THICKNESS)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -ROOM_HEIGHT / 2.0, 0.0)),
            ..default()
        },
        Wall,
    )).set_parent(room_entity);
    
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(WALL_THICKNESS, ROOM_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-ROOM_WIDTH / 2.0, 0.0, 0.0)),
            ..default()
        },
        Wall,
    )).set_parent(room_entity);
    
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: wall_color,
                custom_size: Some(Vec2::new(WALL_THICKNESS, ROOM_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(ROOM_WIDTH / 2.0, 0.0, 0.0)),
            ..default()
        },
        Wall,
    )).set_parent(room_entity);
    
    // Floor (visual only)
    let floor_color = match biome {
        BiomeType::Tutorial => Color::rgb(0.2, 0.2, 0.2),
        BiomeType::Biome1 => Color::rgb(0.3, 0.2, 0.1),
        BiomeType::Biome2 => Color::rgb(0.2, 0.3, 0.2),
        BiomeType::Biome3 => Color::rgb(0.2, 0.2, 0.3),
        BiomeType::Biome4 => Color::rgb(0.3, 0.1, 0.3),
        BiomeType::Biome5 => Color::rgb(0.4, 0.3, 0.2),
    };
    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: floor_color,
                custom_size: Some(Vec2::new(ROOM_WIDTH - WALL_THICKNESS * 2.0, ROOM_HEIGHT - WALL_THICKNESS * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ..default()
        },
    )).set_parent(room_entity);
}

#[derive(Component)]
struct Wall;

fn spawn_door(
    commands: &mut Commands,
    room_entity: Entity,
    direction: Direction,
    room_type: RoomType,
) {
    const DOOR_SIZE: Vec2 = Vec2::new(80.0, 20.0);
    const ROOM_WIDTH: f32 = 1200.0;
    const ROOM_HEIGHT: f32 = 800.0;
    
    let (position, rotation, size) = match direction {
        Direction::North => (Vec3::new(0.0, ROOM_HEIGHT / 2.0, 1.0), 0.0, DOOR_SIZE),
        Direction::South => (Vec3::new(0.0, -ROOM_HEIGHT / 2.0, 1.0), 0.0, DOOR_SIZE),
        Direction::East => (Vec3::new(ROOM_WIDTH / 2.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2, Vec2::new(DOOR_SIZE.y, DOOR_SIZE.x)),
        Direction::West => (Vec3::new(-ROOM_WIDTH / 2.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2, Vec2::new(DOOR_SIZE.y, DOOR_SIZE.x)),
    };
    
    let door_color = if room_type == RoomType::Boss {
        Color::rgb(0.8, 0.2, 0.2) // Red for boss doors
    } else {
        Color::rgb(0.4, 0.3, 0.2) // Brown for normal doors
    };
    
    commands.spawn((
        Door {
            direction,
            locked: room_type != RoomType::Safe,
            leads_to: None,
        },
        SpriteBundle {
            sprite: Sprite {
                color: door_color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_z(rotation)),
            ..default()
        },
    )).set_parent(room_entity);
}

fn spawn_room_enemies(
    commands: &mut Commands,
    room_entity: Entity,
    enemy_count: u32,
    room_number: u32,
    room_type: RoomType,
) {
    let mut rng = rand::thread_rng();
    
    // Spawn points for enemies
    let spawn_points = vec![
        Vec2::new(-200.0, 100.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(-200.0, -100.0),
        Vec2::new(200.0, -100.0),
        Vec2::new(0.0, 200.0),
        Vec2::new(0.0, -200.0),
        Vec2::new(-300.0, 0.0),
        Vec2::new(300.0, 0.0),
    ];
    
    for i in 0..enemy_count {
        let spawn_pos = spawn_points[i as usize % spawn_points.len()];
        
        // Determine enemy type based on room type and progression
        let enemy_type = if room_type == RoomType::Elite {
            // Elite rooms have elite enemies
            if rng.gen_bool(0.5) {
                EnemyType::EliteMelee
            } else {
                EnemyType::EliteRanged
            }
        } else {
            // Normal rooms have mix of common enemies
            let elite_chance = (room_number as f32 / 20.0).min(0.4);
            
            if rng.gen_bool(elite_chance as f64) {
                if rng.gen_bool(0.5) {
                    EnemyType::EliteMelee
                } else {
                    EnemyType::EliteRanged
                }
            } else {
                // 60% melee, 40% ranged for common enemies
                if rng.gen_bool(0.6) {
                    EnemyType::CommonMelee
                } else {
                    EnemyType::CommonRanged
                }
            }
        };
        
        // Spawn the enemy
        let enemy_entity = match enemy_type {
            EnemyType::CommonMelee | EnemyType::EliteMelee => {
                spawn_melee_enemy(commands, spawn_pos, enemy_type, room_number)
            }
            EnemyType::CommonRanged | EnemyType::EliteRanged => {
                spawn_ranged_enemy(commands, spawn_pos, enemy_type, room_number)
            }
            _ => continue,
        };
        
        // Parent enemy to room
        commands.entity(enemy_entity).set_parent(room_entity);
    }
}

fn spawn_room_hazards(
    commands: &mut Commands,
    room_entity: Entity,
    biome: BiomeType,
) {
    // Spawn hazards based on biome type
    match biome {
        BiomeType::Biome2 => {
            // Poison pools
            spawn_poison_pool(commands, room_entity, Vec2::new(100.0, 100.0));
            spawn_poison_pool(commands, room_entity, Vec2::new(-100.0, -100.0));
        }
        BiomeType::Biome3 | BiomeType::Biome4 => {
            // Spike traps or other hazards
            spawn_spike_trap(commands, room_entity, Vec2::new(0.0, 0.0));
        }
        _ => {}
    }
}

fn spawn_poison_pool(commands: &mut Commands, room_entity: Entity, position: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.8, 0.2, 0.6),
                custom_size: Some(Vec2::new(80.0, 80.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(-0.5)),
            ..default()
        },
        Hazard {
            damage_per_second: 10.0,
            hazard_type: HazardType::Poison,
        },
    )).set_parent(room_entity);
}

fn spawn_spike_trap(commands: &mut Commands, room_entity: Entity, position: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(-0.5)),
            ..default()
        },
        Hazard {
            damage_per_second: 20.0,
            hazard_type: HazardType::Spikes,
        },
    )).set_parent(room_entity);
}

#[derive(Component)]
struct Hazard {
    damage_per_second: f32,
    hazard_type: HazardType,
}

#[derive(Debug, Clone, Copy)]
enum HazardType {
    Poison,
    Spikes,
    Fire,
    Ice,
}

fn should_spawn_hazards(biome: BiomeType, room_number: u32) -> bool {
    match biome {
        BiomeType::Tutorial | BiomeType::Biome1 => false,
        BiomeType::Biome2 => room_number > 5,
        BiomeType::Biome3 | BiomeType::Biome4 | BiomeType::Biome5 => true,
    }
}

fn spawn_player(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        Player {
            id: "player".to_string(),
            is_local: true,
        },
        LocalPlayer,
        Health::new(100.0), // Player starts with 100 HP (survives 5 common hits per Design Bible)
        CombatStats::default(),
        MovementStats::default(),
        DashState::default(),
        AttackState::default(),
        Velocity::default(),
        InputBuffer::default(),
        Hurtbox {
            size: Vec2::new(32.0, 32.0),
            invulnerable: false,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.6, 1.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(1.0)),
            ..default()
        },
    ));
}

fn check_room_clear(
    mut room_query: Query<(&mut Room, &Children)>,
    enemy_query: Query<&Enemy>,
    mut current_room: ResMut<CurrentRoom>,
    mut door_query: Query<&mut Door>,
) {
    if let Ok((mut room, children)) = room_query.get_mut(current_room.entity) {
        if room.cleared {
            return;
        }
        
        // Count remaining enemies
        let mut enemies_remaining = 0;
        for child in children.iter() {
            if enemy_query.get(*child).is_ok() {
                enemies_remaining += 1;
            }
        }
        
        current_room.enemies_remaining = enemies_remaining;
        
        // Check if room is cleared
        if enemies_remaining == 0 && room.enemy_count > 0 {
            room.cleared = true;
            current_room.doors_locked = false;
            
            // Unlock all doors
            for child in children.iter() {
                if let Ok(mut door) = door_query.get_mut(*child) {
                    door.locked = false;
                }
            }
            
            // Room cleared! Spawn rewards will be handled by another system
        }
    }
}

fn handle_door_interaction(
    keyboard: Res<ButtonInput<KeyCode>>,
    door_query: Query<(&Door, &Transform)>,
    player_query: Query<&Transform, With<LocalPlayer>>,
    current_room: Res<CurrentRoom>,
    mut game_state: ResMut<GameState>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }
    
    let player_transform = player_query.get_single();
    if player_transform.is_err() {
        return;
    }
    let player_pos = player_transform.translation.truncate();
    
    for (door, door_transform) in door_query.iter() {
        let door_pos = door_transform.translation.truncate();
        let distance = player_pos.distance(door_pos);
        
        // Check if player is near door
        if distance < 50.0 && !door.locked {
            // Initiate room transition
            game_state.current_state = CurrentGameState::RoomTransition;
            game_state.room_number += 1;
        }
    }
}

fn room_transition(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut room_gen: ResMut<RoomGenerator>,
    current_room: Res<CurrentRoom>,
    room_query: Query<Entity, With<Room>>,
) {
    if game_state.current_state != CurrentGameState::RoomTransition {
        return;
    }
    
    // Despawn old room
    for entity in room_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Determine next room type
    let room_type = determine_next_room_type(game_state.room_number);
    
    // Generate new room
    let new_room = generate_room(
        &mut commands,
        room_type,
        game_state.room_number,
        game_state.biome,
        &mut room_gen,
    );
    
    // Update current room resource
    commands.insert_resource(CurrentRoom {
        entity: new_room,
        room_type,
        enemies_remaining: 0,
        doors_locked: room_type != RoomType::Safe,
        spawn_points: vec![
            Vec2::new(-200.0, 0.0),
            Vec2::new(200.0, 0.0),
            Vec2::new(0.0, 150.0),
            Vec2::new(0.0, -150.0),
        ],
        hazards: Vec::new(),
    });
    
    // Return to gameplay
    game_state.current_state = CurrentGameState::InRun;
}

fn determine_next_room_type(room_number: u32) -> RoomType {
    let mut rng = rand::thread_rng();
    
    // Boss rooms every 10 rooms
    if room_number % 10 == 0 {
        return RoomType::Boss;
    }
    
    // Shop rooms every 5 rooms
    if room_number % 5 == 0 {
        return RoomType::Shop;
    }
    
    // Random room type with weights
    let roll = rng.gen::<f32>();
    if roll < 0.6 {
        RoomType::Combat
    } else if roll < 0.8 {
        RoomType::Elite
    } else if roll < 0.9 {
        RoomType::Treasure
    } else {
        RoomType::Secret
    }
}

fn spawn_room_rewards(
    mut commands: Commands,
    room_query: Query<(&Room, &Transform), Changed<Room>>,
) {
    for (room, transform) in room_query.iter() {
        if !room.cleared {
            continue;
        }
        
        // Spawn reward pickups
        for (i, reward) in room.rewards.iter().enumerate() {
            let offset = Vec2::new(i as f32 * 50.0 - 25.0, 0.0);
            
            match reward {
                RewardType::Gold(amount) => {
                    spawn_gold_pickup(&mut commands, transform.translation.truncate() + offset, *amount);
                }
                RewardType::Heal(percentage) => {
                    spawn_health_pickup(&mut commands, transform.translation.truncate() + offset, *percentage);
                }
                _ => {
                    // Boons and items would trigger selection UI
                }
            }
        }
    }
}

fn spawn_gold_pickup(commands: &mut Commands, position: Vec2, amount: u32) {
    commands.spawn((
        Pickup {
            pickup_type: PickupType::Gold,
            auto_collect: true,
            value: amount as f32,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.5)),
            ..default()
        },
    ));
}

fn spawn_health_pickup(commands: &mut Commands, position: Vec2, percentage: f32) {
    commands.spawn((
        Pickup {
            pickup_type: PickupType::Health,
            auto_collect: true,
            value: percentage,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.5)),
            ..default()
        },
    ));
}

fn update_room_hazards(
    hazard_query: Query<(&Hazard, &Transform)>,
    mut player_query: Query<(&mut Health, &Transform), With<LocalPlayer>>,
    time: Res<Time>,
) {
    for (mut health, player_transform) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        
        for (hazard, hazard_transform) in hazard_query.iter() {
            let hazard_pos = hazard_transform.translation.truncate();
            
            // Simple overlap check
            if player_pos.distance(hazard_pos) < 30.0 {
                // Apply hazard damage
                health.take_damage(hazard.damage_per_second * time.delta_seconds());
            }
        }
    }
}