use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============= Player Components =============

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub id: String,
    pub is_local: bool,
}

#[derive(Component)]
pub struct LocalPlayer;

// ============= Movement Components =============

#[derive(Component, Debug, Clone, Default)]
pub struct Velocity {
    pub linear: Vec2,
}

#[derive(Component, Debug, Clone)]
pub struct MovementStats {
    pub base_speed: f32,
    pub current_speed: f32,
    pub dash_speed: f32,
    pub dash_duration: f32,
    pub dash_cooldown: f32,
}

impl Default for MovementStats {
    fn default() -> Self {
        Self {
            base_speed: 300.0,
            current_speed: 300.0,
            dash_speed: 800.0,
            dash_duration: 0.2,  // 0.2s dash duration per Design Bible
            dash_cooldown: 0.4,  // 0.4s cooldown per Design Bible
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct DashState {
    pub is_dashing: bool,
    pub dash_timer: Timer,
    pub cooldown_timer: Timer,
    pub dash_direction: Vec2,
    pub has_iframes: bool,
}

impl Default for DashState {
    fn default() -> Self {
        Self {
            is_dashing: false,
            dash_timer: Timer::from_seconds(0.2, TimerMode::Once),
            cooldown_timer: Timer::from_seconds(0.4, TimerMode::Once),
            dash_direction: Vec2::ZERO,
            has_iframes: true,
        }
    }
}

// ============= Combat Components =============

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        self.current = (self.current - amount).max(0.0);
        self.current <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub attack_speed: f32,
    pub armor: f32,
    pub damage_reduction: f32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            damage: 100.0,
            crit_chance: 0.1,      // 10% base crit
            crit_damage: 2.0,      // 2x damage on crit
            attack_speed: 1.0,     // 1.0 = normal speed
            armor: 0.0,
            damage_reduction: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct AttackState {
    pub is_attacking: bool,
    pub attack_type: AttackType,
    pub startup_timer: Timer,
    pub active_timer: Timer,
    pub recovery_timer: Timer,
    pub can_cancel: bool,
    pub combo_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackType {
    LightAttack,
    HeavyAttack,
    DashAttack,
    Special,
}

impl Default for AttackState {
    fn default() -> Self {
        // Using frame data from Tuning Tables (60 FPS)
        Self {
            is_attacking: false,
            attack_type: AttackType::LightAttack,
            startup_timer: Timer::from_seconds(0.133, TimerMode::Once),  // 8 frames
            active_timer: Timer::from_seconds(0.067, TimerMode::Once),   // 4 frames
            recovery_timer: Timer::from_seconds(0.1, TimerMode::Once),   // 6 frames
            can_cancel: false,
            combo_count: 0,
        }
    }
}

#[derive(Component)]
pub struct Hitbox {
    pub size: Vec2,
    pub offset: Vec2,
    pub damage: f32,
    pub knockback: f32,
    pub active: bool,
    pub hit_entities: Vec<Entity>,
}

#[derive(Component)]
pub struct Hurtbox {
    pub size: Vec2,
    pub invulnerable: bool,
}

// ============= Enemy Components =============

#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub behavior: EnemyBehavior,
    pub aggro_range: f32,
    pub attack_range: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyType {
    CommonMelee,
    CommonRanged,
    EliteMelee,
    EliteRanged,
    MiniBoss,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyBehavior {
    Aggressive,  // Always moves toward player
    Defensive,   // Maintains distance
    Support,     // Buffs other enemies
    Patrol,      // Follows waypoints
}

#[derive(Component, Debug)]
pub struct EnemyAI {
    pub state: AIState,
    pub target: Option<Entity>,
    pub last_known_position: Option<Vec2>,
    pub state_timer: Timer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIState {
    Idle,
    Patrolling,
    Chasing,
    Attacking,
    Telegraphing,
    Recovering,
    Fleeing,
}

#[derive(Component)]
pub struct Telegraph {
    pub duration: Timer,
    pub telegraph_type: TelegraphType,
    pub damage_area: Vec2,
    pub color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum TelegraphType {
    MeleeSwing,
    RangedShot,
    AreaOfEffect,
    ChargeAttack,
}

// ============= Room Components =============

#[derive(Component)]
pub struct Room {
    pub room_type: RoomType,
    pub cleared: bool,
    pub enemy_count: u32,
    pub exits: Vec<Direction>,
    pub rewards: Vec<RewardType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomType {
    Combat,
    Elite,
    Shop,
    Treasure,
    Boss,
    Secret,
    Safe,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub enum RewardType {
    Gold(u32),
    Boon(BoonRarity),
    Heal(f32),
    Item(String),
}

#[derive(Debug, Clone, Copy)]
pub enum BoonRarity {
    Common,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Component)]
pub struct Door {
    pub direction: Direction,
    pub locked: bool,
    pub leads_to: Option<Entity>,
}

// ============= Boon/Power-up Components =============

#[derive(Component, Debug, Clone)]
pub struct Boon {
    pub name: String,
    pub description: String,
    pub rarity: BoonRarity,
    pub boon_type: BoonType,
    pub stacks: u32,
    pub max_stacks: u32,
}

#[derive(Debug, Clone)]
pub enum BoonType {
    DamageBoost(f32),
    SpeedBoost(f32),
    CritBoost(f32),
    HealthBoost(f32),
    Lifesteal(f32),
    ChainLightning(u32),
    Thorns(f32),
    DashReset,
    DoubleJump,
}

// ============= UI Components =============

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct DamageNumber {
    pub amount: f32,
    pub is_crit: bool,
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct GameInfoText;

// ============= Input Components =============

#[derive(Component, Default)]
pub struct InputBuffer {
    pub buffer: Vec<BufferedInput>,
    pub max_buffer_time: f32,
}

#[derive(Clone, Debug)]
pub struct BufferedInput {
    pub action: InputAction,
    pub timestamp: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputAction {
    Move(Vec2),
    LightAttack,
    HeavyAttack,
    Dash,
    Interact,
    Pause,
}

// ============= Status Effect Components =============

#[derive(Component)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: Timer,
    pub stacks: u32,
    pub value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusEffectType {
    Burn,
    Poison,
    Freeze,
    Stun,
    Slow,
    Bleed,
    Shield,
    Regen,
}

// ============= Projectile Components =============

#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: Timer,
    pub piercing: u32,
    pub owner: Entity,
}

// ============= Pickup Components =============

#[derive(Component)]
pub struct Pickup {
    pub pickup_type: PickupType,
    pub auto_collect: bool,
    pub value: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum PickupType {
    Gold,
    Health,
    Soul,
    PowerUp,
}