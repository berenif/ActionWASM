use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::*;

// ============= Core Game State =============

#[derive(Resource, Debug, Clone)]
pub struct GameState {
    pub current_state: CurrentGameState,
    pub run_stats: RunStats,
    pub room_number: u32,
    pub biome: BiomeType,
    pub difficulty: DifficultyLevel,
    pub paused: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_state: CurrentGameState::MainMenu,
            run_stats: RunStats::default(),
            room_number: 1,
            biome: BiomeType::Tutorial,
            difficulty: DifficultyLevel::Normal,
            paused: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentGameState {
    MainMenu,
    InRun,
    RoomTransition,
    BoonSelection,
    Shop,
    BossFight,
    Death,
    Victory,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BiomeType {
    Tutorial,
    Biome1,  // Basic enemies, no hazards
    Biome2,  // Projectiles, poison pools
    Biome3,  // Shields, teleporters
    Biome4,  // All mechanics, time pressure
    Biome5,  // Victory lap
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DifficultyLevel {
    Easy,
    Normal,
    Hard,
    Nightmare,
    Ascension(u8),  // 1-20 ascension levels
}

// ============= Run Statistics =============

#[derive(Resource, Debug, Clone, Default)]
pub struct RunStats {
    pub enemies_killed: u32,
    pub damage_dealt: f32,
    pub damage_taken: f32,
    pub rooms_cleared: u32,
    pub gold_collected: u32,
    pub boons_collected: u32,
    pub run_time: f32,
    pub deaths: u32,
}

// ============= Economy Resources =============

#[derive(Resource, Debug, Clone)]
pub struct PlayerInventory {
    pub gold: u32,
    pub souls: u32,
    pub keys: u32,
    pub active_boons: Vec<Boon>,
    pub passive_boons: Vec<Boon>,
    pub curses: Vec<Curse>,
    pub max_active_boons: usize,
    pub max_passive_boons: usize,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            gold: 0,
            souls: 0,
            keys: 0,
            active_boons: Vec::new(),
            passive_boons: Vec::new(),
            curses: Vec::new(),
            max_active_boons: 3,
            max_passive_boons: 6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Curse {
    pub name: String,
    pub description: String,
    pub effect: CurseEffect,
    pub duration: Option<u32>,  // Some curses last X rooms
}

#[derive(Debug, Clone)]
pub enum CurseEffect {
    ReducedMaxHealth(f32),
    ReducedSpeed(f32),
    ReducedVision(f32),
    HealthDrain(f32),
    IncreasedEnemyDamage(f32),
    OneHitDeath,
}

// ============= Room Generation Resources =============

#[derive(Resource)]
pub struct CurrentRoom {
    pub entity: Entity,
    pub room_type: RoomType,
    pub enemies_remaining: u32,
    pub doors_locked: bool,
    pub spawn_points: Vec<Vec2>,
    pub hazards: Vec<Entity>,
}

#[derive(Resource)]
pub struct RoomGenerator {
    pub seed: u64,
    pub rooms_generated: u32,
}

// ============= Combat Resources =============

#[derive(Resource, Default)]
pub struct CombatLog {
    pub recent_damage: Vec<DamageEvent>,
    pub last_death_cause: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub source: String,
    pub target: String,
    pub amount: f32,
    pub is_crit: bool,
    pub damage_type: DamageType,
    pub timestamp: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Bleed,
    True,  // Ignores armor
}

// ============= Input Resources =============

#[derive(Resource, Default)]
pub struct InputSettings {
    pub input_buffer_time: f32,
    pub dash_double_tap: bool,
    pub auto_aim: bool,
    pub controller_enabled: bool,
}

impl InputSettings {
    pub fn new() -> Self {
        Self {
            input_buffer_time: 0.1,  // 100ms buffer as per Design Bible
            dash_double_tap: false,
            auto_aim: false,
            controller_enabled: true,
        }
    }
}

// ============= Wave/Spawn Management =============

#[derive(Resource)]
pub struct WaveManager {
    pub current_wave: u32,
    pub enemies_per_wave: Vec<u32>,
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub spawn_queue: Vec<EnemySpawnData>,
}

#[derive(Debug, Clone)]
pub struct EnemySpawnData {
    pub enemy_type: EnemyType,
    pub position: Vec2,
    pub elite_modifier: Option<EliteModifier>,
}

#[derive(Debug, Clone, Copy)]
pub enum EliteModifier {
    Armored,      // 1.5x HP, 50% damage reduction
    Berserker,    // 2x damage, +50% attack speed
    Regenerating, // 2x HP, 5% HP/sec regen
    Shielded,     // Immune until shield breaks
    Vampiric,     // Heals 20% damage dealt
    Explosive,    // Explodes on death
}

// ============= Boon Selection Resources =============

#[derive(Resource)]
pub struct BoonSelectionState {
    pub available_boons: Vec<BoonChoice>,
    pub reroll_cost: u32,
    pub rerolls_used: u32,
}

#[derive(Debug, Clone)]
pub struct BoonChoice {
    pub boon: Boon,
    pub synergies: Vec<String>,
    pub preview_stats: BoonPreview,
}

#[derive(Debug, Clone)]
pub struct BoonPreview {
    pub damage_change: Option<f32>,
    pub health_change: Option<f32>,
    pub speed_change: Option<f32>,
    pub special_effect: Option<String>,
}

// ============= Meta Progression Resources =============

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgression {
    pub account_level: u32,
    pub total_runs: u32,
    pub successful_runs: u32,
    pub unlocked_weapons: Vec<String>,
    pub unlocked_boons: Vec<String>,
    pub unlocked_characters: Vec<String>,
    pub permanent_upgrades: Vec<PermanentUpgrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentUpgrade {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub effect: String,
}

// ============= Performance Monitoring =============

#[derive(Resource, Default)]
pub struct PerformanceStats {
    pub fps: f32,
    pub frame_time: f32,
    pub entity_count: usize,
    pub draw_calls: u32,
}

// ============= Audio Resources =============

#[derive(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            sfx_volume: 0.8,
            music_volume: 0.6,
            muted: false,
        }
    }
}