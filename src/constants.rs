// PRD-Accurate Physics Constants
pub const GRAVITY: f32 = 0.22;
pub const MAX_FALL_SPEED: f32 = 5.5;
pub const JUMP_VELOCITY: f32 = -4.8;
pub const WALK_SPEED: f32 = 1.8;
pub const GROUND_ACCEL: f32 = 0.15;
pub const GROUND_DECEL: f32 = 0.12;
pub const AIR_CONTROL: f32 = 0.6;
pub const COYOTE_TIME: u8 = 7;
pub const JUMP_BUFFER: u8 = 7;
pub const STOMP_BOUNCE: f32 = -3.5;

// Sprite dimensions
pub const PLAYER_HEIGHT: f32 = 32.0;
pub const ENEMY_HEIGHT: f32 = 32.0;

// Ice physics
pub const ICE_FRICTION: f32 = 0.02;  // Much lower than normal ground decel
pub const ICE_ACCEL: f32 = 0.08;     // Lower than normal ground accel

// Player states
pub const STATE_IDLE: u8 = 0;
pub const STATE_RUNNING: u8 = 1;
pub const STATE_JUMPING: u8 = 2;
pub const STATE_FALLING: u8 = 3;
pub const STATE_DEAD: u8 = 4;
pub const STATE_LEVEL_START: u8 = 5;

// Game screen states
pub const SCREEN_LEVEL_SELECT: u8 = 0;
pub const SCREEN_PLAYING: u8 = 1;
pub const SCREEN_LEVEL_COMPLETE: u8 = 2;
pub const SCREEN_GAME_COMPLETE: u8 = 3;
pub const SCREEN_BOSS_TRANSITION: u8 = 4;
pub const SCREEN_SETTINGS: u8 = 5;

// Settings menu options
pub const SETTING_MUSIC: u8 = 0;
pub const SETTING_SFX: u8 = 1;
pub const SETTING_ARCADE: u8 = 2;
pub const SETTING_DEVELOPER: u8 = 3;
pub const SETTING_BACK: u8 = 4;

// Tile types
pub const TILE_AIR: u8 = 0;
pub const TILE_SNOW: u8 = 1;
pub const TILE_WOOD: u8 = 2;
pub const TILE_ICE: u8 = 3;

// Enemy types
pub const ENEMY_NONE: u8 = 0;
pub const ENEMY_GINGERBREAD: u8 = 1;
pub const ENEMY_GRINCH: u8 = 2;
pub const ENEMY_KRAMPUS: u8 = 3;

// Enemy states
pub const ENEMY_STATE_ALIVE: u8 = 0;
pub const ENEMY_STATE_DYING: u8 = 1;
pub const ENEMY_STATE_ATTACKING: u8 = 2;

// Enemy behavior constants
pub const GRINCH_ATTACK_RANGE: f32 = 80.0;  // Range to detect player and throw snowball (ONLY when visible on screen)
pub const GRINCH_ATTACK_COOLDOWN: u8 = 120;  // Frames between attacks (2 seconds)
pub const KRAMPUS_ATTACK_RANGE: f32 = 50.0;  // Range to attack with chain (reduced for better gameplay)
pub const KRAMPUS_ATTACK_COOLDOWN: u8 = 90;  // Frames between attacks
pub const ENEMY_DEATH_TIMER: u8 = 30;  // How long death animation plays (0.5 seconds)

// Level dimensions
pub const LEVEL_WIDTH: i32 = 1200;   // Much wider levels for proper gameplay
pub const TILE_SIZE: i32 = 16;
pub const SCREEN_WIDTH: i32 = 320;
pub const SCREEN_HEIGHT: i32 = 180;

// Maximum entities per level
pub const MAX_ENEMIES: usize = 10;
pub const MAX_GIFTS: usize = 5;
pub const MAX_PLATFORMS: usize = 15;
pub const MAX_PROJECTILES: usize = 8;  // Maximum snowballs on screen

// Projectile constants
pub const SNOWBALL_SPEED: f32 = 1.2;  // Slower for early levels so player can react and dodge
pub const SNOWBALL_SIZE: f32 = 16.0;

// Boss Fight constants
pub const BOSS_HP_MAX: u16 = 150;  // Boss has 150 HP (tripled for difficulty)
pub const BOSS_SIZE: f32 = 64.0;  // Giant Krampus boss is 64x64 (special boss sprites)
pub const GIFT_BOMB_SPEED: f32 = 3.0;  // Gift bombs fly fast
pub const GIFT_BOMB_SIZE: f32 = 16.0;
pub const GIFT_GUN_SPEED: f32 = 4.5;  // Gift gun projectiles are faster
pub const GIFT_GUN_SIZE: f32 = 12.0;
pub const GIFT_BOMB_COOLDOWN: u8 = 60;  // 1 second between bombs
pub const GIFT_GUN_COOLDOWN: u8 = 15;  // 0.25 seconds between gun shots
pub const MAX_PLAYER_PROJECTILES: usize = 10;  // Max gift bombs/bullets on screen

// Boss attack patterns
pub const BOSS_JUMP_VELOCITY: f32 = -6.0;  // Boss jumps high
pub const BOSS_GROUND_POUND_DAMAGE_RANGE: f32 = 100.0;  // Range of ground pound shockwave
pub const BOSS_CHAIN_RANGE: f32 = 120.0;  // Range of chain attack
pub const BOSS_ATTACK_COOLDOWN: u8 = 120;  // 2 seconds between boss attacks

// Weapon types
pub const WEAPON_GIFT_BOMB: u8 = 0;
pub const WEAPON_GIFT_GUN: u8 = 1;

// Boss FSM states (Mario-style pattern-based)
pub const BOSS_STATE_IDLE: u8 = 0;
pub const BOSS_STATE_WALK: u8 = 1;
pub const BOSS_STATE_ATTACK: u8 = 2;  // BASH attack
pub const BOSS_STATE_JUMP: u8 = 3;
pub const BOSS_STATE_ROAR: u8 = 4;    // Telegraph before attacks
pub const BOSS_STATE_HURT: u8 = 5;
pub const BOSS_STATE_PHASE_TRANSITION: u8 = 6;
pub const BOSS_STATE_DEATH: u8 = 7;
pub const BOSS_STATE_ATTACK_SNOWBALL: u8 = 8;  // Snowball projectile attack

// Stomp combat constants
pub const BOSS_STOMP_DAMAGE: u16 = 10;        // 15 stomps to kill (150 HP)
pub const BOSS_STOMP_COOLDOWN: u8 = 24;       // ~400ms invincibility after stomp
pub const BOSS_STOMP_HEAD_ZONE: f32 = 20.0;   // Top 20px of 64px boss = stomp zone
pub const BOSS_SNOWBALL_SPEED: f32 = 2.5;     // Snowball projectile speed
pub const BOSS_ROAR_TELEGRAPH: u8 = 32;       // ~530ms telegraph before attack

// Frame timings (60 FPS)
pub const BOSS_IDLE_DURATION: u8 = 30;
pub const BOSS_ATTACK_TOTAL_FRAMES: u8 = 30;
pub const BOSS_ATTACK_WINDUP_FRAMES: u8 = 9;
pub const BOSS_ATTACK_ACTIVE_START: u8 = 10;
pub const BOSS_ATTACK_ACTIVE_END: u8 = 15;
pub const BOSS_HURT_DURATION: u8 = 12;
pub const BOSS_ROAR_DURATION: u8 = 45;
pub const BOSS_ROAR_STUN_FRAME: u8 = 20;
pub const BOSS_ROAR_STUN_DURATION: u8 = 30;
pub const BOSS_WARNING_DURATION: u8 = 25;
pub const BOSS_PHASE_TRANSITION_DURATION: u8 = 45;
pub const BOSS_DEATH_SILENCE_FRAMES: u8 = 10;
pub const BOSS_INVINCIBILITY_FRAMES: u8 = 10;
pub const PLAYER_INVINCIBILITY_FRAMES: u8 = 45;
pub const PLAYER_STOMP_COOLDOWN_FRAMES: u16 = 300;  // 5 seconds at 60fps

// Phase-based speeds
pub const BOSS_WALK_SPEED_PHASE1: f32 = 0.8;
pub const BOSS_WALK_SPEED_PHASE2: f32 = 1.2;
pub const BOSS_WALK_SPEED_PHASE3: f32 = 1.6;

// Phase-based jump velocities
pub const BOSS_JUMP_VELOCITY_PHASE1: f32 = -6.5;
pub const BOSS_JUMP_VELOCITY_PHASE2: f32 = -7.5;
pub const BOSS_JUMP_VELOCITY_PHASE3: f32 = -8.5;

// Phase-based attack cooldowns
pub const BOSS_ATTACK_COOLDOWN_PHASE1: u8 = 120;  // 2 seconds
pub const BOSS_ATTACK_COOLDOWN_PHASE2: u8 = 90;   // 1.5 seconds
pub const BOSS_ATTACK_COOLDOWN_PHASE3: u8 = 70;   // 1.17 seconds

// Attack ranges
pub const BOSS_MELEE_RANGE: f32 = 48.0;
pub const BOSS_JUMP_RANGE: f32 = 96.0;
pub const BOSS_ROAR_RADIUS: f32 = 40.0;
pub const BOSS_DASH_SPEED: f32 = 2.5;

// Rank system
pub const RANK_NONE: u8 = 0;
pub const RANK_D: u8 = 1;
pub const RANK_C: u8 = 2;
pub const RANK_B: u8 = 3;
pub const RANK_A: u8 = 4;
pub const RANK_S: u8 = 5;

// Scoring constants
pub const SCORE_GIFT: u32 = 100;           // Points per gift collected
pub const SCORE_ENEMY: u32 = 200;          // Points per enemy defeated
pub const SCORE_LEVEL_COMPLETE: u32 = 500; // Base completion bonus
pub const SCORE_PERFECT_GIFTS: u32 = 1000; // Bonus for collecting all 5 gifts
pub const SCORE_NO_DEATH: u32 = 500;       // Bonus for no deaths
pub const SCORE_TIME_BONUS: u32 = 10;      // Points per second remaining under par

// Par times for each level (in seconds) - Professional speedrun targets
pub const LEVEL_PAR_TIMES: [u32; 10] = [
    60,   // Level 1: Tutorial - 60 seconds
    90,   // Level 2: Gingerbread Alley - 90 seconds
    75,   // Level 3: Long Jump - 75 seconds
    80,   // Level 4: Highrise - 80 seconds
    85,   // Level 5: Ice Bridge - 85 seconds
    70,   // Level 6: Grinch Rooftops - 70 seconds
    120,  // Level 7: Rooftop Maze - 120 seconds (long level)
    130,  // Level 8: Grinch Gauntlet - 130 seconds
    110,  // Level 9: Krampus Pursuit - 110 seconds
    100,  // Level 10: North Pole Showdown - 100 seconds
];

// Platform structure - defined as rectangles
pub struct Platform {
    pub x1: f32,
    pub x2: f32,
    pub y: f32,
}
