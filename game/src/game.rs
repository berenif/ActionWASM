use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_resource::<PlayerList>()
            .add_systems(Startup, setup_game)
            .add_systems(Update, (
                handle_player_input,
                update_player_positions,
                sync_game_state,
                render_players,
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

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // UI Text for game info
    commands.spawn(
        TextBundle::from_section(
            "Waiting for players...",
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

    // Game boundaries
    let wall_thickness = 10.0;
    let wall_color = Color::srgb(0.5, 0.5, 0.5);
    
    // Top wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(1200.0, wall_thickness)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 400.0, 0.0)),
        ..default()
    });

    // Bottom wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(1200.0, wall_thickness)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, -400.0, 0.0)),
        ..default()
    });

    // Left wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, 800.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-600.0, 0.0, 0.0)),
        ..default()
    });

    // Right wall
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(wall_thickness, 800.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(600.0, 0.0, 0.0)),
        ..default()
    });
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