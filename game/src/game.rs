use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::*;
use crate::resources::*;
use crate::movement::MovementPlugin;
use crate::combat::{CombatPlugin, cleanup_hitboxes};
use crate::enemy::EnemyPlugin;
use crate::room::RoomPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources
        app.init_resource::<GameState>()
            .init_resource::<PlayerInventory>()
            .init_resource::<RunStats>()
            .init_resource::<CombatLog>()
            .init_resource::<InputSettings>()
            .init_resource::<AudioSettings>()
            .init_resource::<PerformanceStats>();
        
        // Add sub-plugins for different systems
        app.add_plugins((
            MovementPlugin,
            CombatPlugin,
            EnemyPlugin,
            RoomPlugin,
        ));
        
        // Add core game systems
        app.add_systems(Startup, setup_game)
            .add_systems(Update, (
                handle_pickups,
                update_ui,
                cleanup_hitboxes,
                update_run_timer,
            ).chain());
    }
}

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub round: u32,
    pub is_host: bool,
    pub game_started: bool,
    pub players: Vec<PlayerData>,
}

#[derive(Resource, Default)]
pub struct PlayerList {
    pub local_player_id: Option<String>,
    pub players: Vec<Entity>,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: Color,
    pub score: u32,
    pub is_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: [f32; 4],
    pub score: u32,
}

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct RemotePlayer {
    pub id: String,
}

// New systems for the roguelike ARPG

fn handle_pickups(
    mut commands: Commands,
    pickup_query: Query<(Entity, &Pickup, &Transform)>,
    mut player_query: Query<(&Transform, &mut Health), With<LocalPlayer>>,
    mut inventory: ResMut<PlayerInventory>,
    mut run_stats: ResMut<RunStats>,
) {
    for (player_transform, mut health) in player_query.iter_mut() {
        let player_pos = player_transform.translation.truncate();
        
        for (pickup_entity, pickup, pickup_transform) in pickup_query.iter() {
            let pickup_pos = pickup_transform.translation.truncate();
            let distance = player_pos.distance(pickup_pos);
            
            // Check if player is close enough to collect
            if distance < 30.0 && pickup.auto_collect {
                match pickup.pickup_type {
                    PickupType::Gold => {
                        inventory.gold += pickup.value as u32;
                        run_stats.gold_collected += pickup.value as u32;
                    }
                    PickupType::Health => {
                        health.heal(health.max * pickup.value);
                    }
                    PickupType::Soul => {
                        inventory.souls += pickup.value as u32;
                    }
                    _ => {}
                }
                
                // Remove pickup
                commands.entity(pickup_entity).despawn();
            }
        }
    }
}

fn update_ui(
    mut text_query: Query<&mut Text, With<GameInfoText>>,
    game_state: Res<GameState>,
    inventory: Res<PlayerInventory>,
    run_stats: Res<RunStats>,
    current_room: Option<Res<CurrentRoom>>,
) {
    for mut text in text_query.iter_mut() {
        let room_info = if let Some(room) = current_room {
            format!("Enemies: {} | ", room.enemies_remaining)
        } else {
            String::new()
        };
        
        text.sections[0].value = format!(
            "Room {} | {}Gold: {} | Souls: {} | Time: {:.0}s",
            game_state.room_number,
            room_info,
            inventory.gold,
            inventory.souls,
            run_stats.run_time,
        );
    }
}

fn update_run_timer(
    mut run_stats: ResMut<RunStats>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if matches!(game_state.current_state, CurrentGameState::InRun) {
        run_stats.run_time += time.delta_seconds();
    }
}

fn setup_game(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // UI Text for game info
    commands.spawn(
        TextBundle::from_section(
            "Room 1 | Gold: 0 | Souls: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    )
    .insert(GameInfoText);
    
    // Set initial game state
    game_state.current_state = CurrentGameState::InRun;
}

#[derive(Component)]
struct GameInfoText;

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Player, With<LocalPlayer>>,
) {
    for mut player in query.iter_mut() {
        let mut direction = Vec2::ZERO;
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            player.velocity = direction * 300.0; // Speed in pixels per second
        } else {
            player.velocity = Vec2::ZERO;
        }
    }
}

fn update_player_positions(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        if player.velocity != Vec2::ZERO {
            let new_position = transform.translation.xy() + player.velocity * time.delta_seconds();
            
            // Clamp to game boundaries
            let clamped_x = new_position.x.clamp(-580.0, 580.0);
            let clamped_y = new_position.y.clamp(-380.0, 380.0);
            
            transform.translation.x = clamped_x;
            transform.translation.y = clamped_y;
        }
    }
}

fn sync_game_state(
    game_state: Res<GameState>,
    mut text_query: Query<&mut Text, With<GameInfoText>>,
) {
    for mut text in text_query.iter_mut() {
        if game_state.game_started {
            text.sections[0].value = format!(
                "Round: {} | Players: {} | Host: {}",
                game_state.round,
                game_state.players.len(),
                if game_state.is_host { "Yes" } else { "No" }
            );
        } else {
            text.sections[0].value = format!(
                "Waiting for players... ({} connected)",
                game_state.players.len()
            );
        }
    }
}

fn render_players(
    mut commands: Commands,
    game_state: Res<GameState>,
    player_list: Res<PlayerList>,
    mut existing_players: Query<(Entity, &mut Transform, &mut Sprite, &Player)>,
) {
    // Update existing players
    for (entity, mut transform, mut sprite, player) in existing_players.iter_mut() {
        if let Some(player_data) = game_state.players.iter().find(|p| p.id == player.id) {
            transform.translation.x = player_data.position.x;
            transform.translation.y = player_data.position.y;
            sprite.color = Color::srgba(
                player_data.color[0],
                player_data.color[1],
                player_data.color[2],
                player_data.color[3],
            );
        }
    }
}

pub fn spawn_local_player(
    commands: &mut Commands,
    player_id: String,
    position: Vec2,
    color: Color,
) -> Entity {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Player {
            id: player_id.clone(),
            position,
            velocity: Vec2::ZERO,
            color,
            score: 0,
            is_local: true,
        },
        LocalPlayer,
    )).id()
}

pub fn spawn_remote_player(
    commands: &mut Commands,
    player_id: String,
    position: Vec2,
    color: Color,
) -> Entity {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Player {
            id: player_id.clone(),
            position,
            velocity: Vec2::ZERO,
            color,
            score: 0,
            is_local: false,
        },
        RemotePlayer { id: player_id },
    )).id()
}