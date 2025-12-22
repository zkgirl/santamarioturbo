use turbo::*;
use turbo::input::{gamepad, keyboard};
use turbo::audio;

// Module declarations
mod constants;
mod levels;
mod boss;
mod level_select;
mod settings;
mod player;
mod enemies;
mod projectiles;
mod collision;
mod render;

// Import all constants
pub use constants::*;

#[turbo::game]
struct GameState {
    // Player
    player_x: f32,
    player_y: f32,
    player_vx: f32,
    player_vy: f32,
    player_state: u8,
    player_facing_right: bool,
    player_on_ground: bool,
    player_on_ice: bool,  // New: track if on ice
    coyote_timer: u8,
    jump_buffer_timer: u8,
    anim_frame: u32,
    death_timer: u8,
    falling_sound_played: bool,  // Flag to play falling sound only once per death
    death_sound_played: bool,    // Flag to play death sound only once per death

    // Game state
    lives: u8,
    total_score: u32,        // Total score across all levels
    level_score: u32,        // Score for current level only
    level_time: u32,         // Time spent in current level (frames)
    level_deaths: u8,        // Deaths in current level
    game_time: u32,
    camera_x: f32,
    current_level: u8,
    parallax_offset: f32,
    level_complete: bool,
    game_complete: bool,  // True when all 10 levels completed
    arcade_mode: bool,     // True when in endless arcade mode

    // Level stats and records
    level_best_time: [u32; 10],   // Best time for each level (frames)
    level_best_score: [u32; 10],  // Best score for each level
    level_rank: [u8; 10],         // Rank achieved (0=none, 1=D, 2=C, 3=B, 4=A, 5=S)

    // Level select system
    current_screen: u8,  // SCREEN_LEVEL_SELECT, SCREEN_PLAYING, etc.
    level_unlocked: [bool; 10],  // Track which levels are unlocked
    level_gifts_collected: [u8; 10],  // Track gifts collected per level (0-5)
    selected_level: u8,  // Currently selected level in level select (1-10)
    level_complete_timer: u8,  // Timer for level complete screen before showing level select
    boss_transition_timer: u8,  // Timer for boss transition screen
    input_cooldown: u8,  // Prevent rapid input in level select

    // Settings system
    music_enabled: bool,       // Background music on/off
    music_volume: f32,         // Music volume (0.0 - 1.0)
    sfx_enabled: bool,         // Sound effects on/off
    sfx_volume: f32,           // SFX volume (0.0 - 1.0)
    selected_setting: u8,      // Currently selected option in settings menu
    previous_screen: u8,       // Screen to return to after closing settings

    // Developer mode
    developer_mode: bool,      // Developer mode enabled (unlocks all levels)
    password_input: String,    // Current password input
    entering_password: bool,   // Whether player is currently entering password

    // Chimney (goal) position
    chimney_x: f32,
    chimney_y: f32,

    // Level platforms (x1, x2, y) - stored as flat array
    platform_count: usize,
    plat1_x1: f32, plat1_x2: f32, plat1_y: f32,
    plat2_x1: f32, plat2_x2: f32, plat2_y: f32,
    plat3_x1: f32, plat3_x2: f32, plat3_y: f32,
    plat4_x1: f32, plat4_x2: f32, plat4_y: f32,
    plat5_x1: f32, plat5_x2: f32, plat5_y: f32,
    plat6_x1: f32, plat6_x2: f32, plat6_y: f32,
    plat7_x1: f32, plat7_x2: f32, plat7_y: f32,
    plat8_x1: f32, plat8_x2: f32, plat8_y: f32,
    plat9_x1: f32, plat9_x2: f32, plat9_y: f32,
    plat10_x1: f32, plat10_x2: f32, plat10_y: f32,

    // Enemies - now with type field [x, y, vx, type, facing_right as 0/1, active as 0/1]
    enemy1_x: f32,
    enemy1_y: f32,
    enemy1_vx: f32,
    enemy1_vy: f32,  // Vertical velocity for gravity
    enemy1_type: u8,  // ENEMY_GINGERBREAD, ENEMY_GRINCH, or ENEMY_KRAMPUS
    enemy1_facing_right: bool,
    enemy1_active: bool,
    enemy1_min_x: f32,  // Patrol boundaries
    enemy1_max_x: f32,
    enemy1_state: u8,  // ENEMY_STATE_ALIVE, DYING, or ATTACKING
    enemy1_death_timer: u8,
    enemy1_attack_timer: u8,

    enemy2_x: f32,
    enemy2_y: f32,
    enemy2_vx: f32,
    enemy2_vy: f32,
    enemy2_type: u8,
    enemy2_facing_right: bool,
    enemy2_active: bool,
    enemy2_min_x: f32,
    enemy2_max_x: f32,
    enemy2_state: u8,
    enemy2_death_timer: u8,
    enemy2_attack_timer: u8,

    enemy3_x: f32,
    enemy3_y: f32,
    enemy3_vx: f32,
    enemy3_vy: f32,
    enemy3_type: u8,
    enemy3_facing_right: bool,
    enemy3_active: bool,
    enemy3_min_x: f32,
    enemy3_max_x: f32,
    enemy3_state: u8,
    enemy3_death_timer: u8,
    enemy3_attack_timer: u8,

    enemy4_x: f32,
    enemy4_y: f32,
    enemy4_vx: f32,
    enemy4_vy: f32,
    enemy4_type: u8,
    enemy4_facing_right: bool,
    enemy4_active: bool,
    enemy4_min_x: f32,
    enemy4_max_x: f32,
    enemy4_state: u8,
    enemy4_death_timer: u8,
    enemy4_attack_timer: u8,

    enemy5_x: f32,
    enemy5_y: f32,
    enemy5_vx: f32,
    enemy5_vy: f32,
    enemy5_type: u8,
    enemy5_facing_right: bool,
    enemy5_active: bool,
    enemy5_min_x: f32,
    enemy5_max_x: f32,
    enemy5_state: u8,
    enemy5_death_timer: u8,
    enemy5_attack_timer: u8,

    enemy6_x: f32,
    enemy6_y: f32,
    enemy6_vx: f32,
    enemy6_vy: f32,
    enemy6_type: u8,
    enemy6_facing_right: bool,
    enemy6_active: bool,
    enemy6_min_x: f32,
    enemy6_max_x: f32,
    enemy6_state: u8,
    enemy6_death_timer: u8,
    enemy6_attack_timer: u8,

    enemy7_x: f32,
    enemy7_y: f32,
    enemy7_vx: f32,
    enemy7_vy: f32,
    enemy7_type: u8,
    enemy7_facing_right: bool,
    enemy7_active: bool,
    enemy7_min_x: f32,
    enemy7_max_x: f32,
    enemy7_state: u8,
    enemy7_death_timer: u8,
    enemy7_attack_timer: u8,

    enemy8_x: f32,
    enemy8_y: f32,
    enemy8_vx: f32,
    enemy8_vy: f32,
    enemy8_type: u8,
    enemy8_facing_right: bool,
    enemy8_active: bool,
    enemy8_min_x: f32,
    enemy8_max_x: f32,
    enemy8_state: u8,
    enemy8_death_timer: u8,
    enemy8_attack_timer: u8,

    enemy9_x: f32,
    enemy9_y: f32,
    enemy9_vx: f32,
    enemy9_vy: f32,
    enemy9_type: u8,
    enemy9_facing_right: bool,
    enemy9_active: bool,
    enemy9_min_x: f32,
    enemy9_max_x: f32,
    enemy9_state: u8,
    enemy9_death_timer: u8,
    enemy9_attack_timer: u8,

    enemy10_x: f32,
    enemy10_y: f32,
    enemy10_vx: f32,
    enemy10_vy: f32,
    enemy10_type: u8,
    enemy10_facing_right: bool,
    enemy10_active: bool,
    enemy10_min_x: f32,
    enemy10_max_x: f32,
    enemy10_state: u8,
    enemy10_death_timer: u8,
    enemy10_attack_timer: u8,

    // Gifts
    gift1_x: f32,
    gift1_y: f32,
    gift1_collected: bool,

    gift2_x: f32,
    gift2_y: f32,
    gift2_collected: bool,

    gift3_x: f32,
    gift3_y: f32,
    gift3_collected: bool,

    gift4_x: f32,
    gift4_y: f32,
    gift4_collected: bool,

    gift5_x: f32,
    gift5_y: f32,
    gift5_collected: bool,

    all_gifts_sound_played: bool,  // Flag to play "all collected" sound only once

    // Snowball projectiles (from Grinch attacks)
    snowball1_x: f32,
    snowball1_y: f32,
    snowball1_vx: f32,
    snowball1_start_x: f32,  // Track spawn position for range limit
    snowball1_active: bool,

    snowball2_x: f32,
    snowball2_y: f32,
    snowball2_vx: f32,
    snowball2_start_x: f32,
    snowball2_active: bool,

    snowball3_x: f32,
    snowball3_y: f32,
    snowball3_vx: f32,
    snowball3_start_x: f32,
    snowball3_active: bool,

    snowball4_x: f32,
    snowball4_y: f32,
    snowball4_vx: f32,
    snowball4_start_x: f32,
    snowball4_active: bool,

    // Boss Fight State
    in_boss_fight: bool,
    boss_hp: u16,
    boss_x: f32,
    boss_y: f32,
    boss_vx: f32,
    boss_vy: f32,
    boss_facing_right: bool,
    boss_attack_timer: u8,
    boss_attack_type: u8,  // 0=none, 1=jump, 2=ground_pound, 3=chain
    boss_invincible_timer: u8,  // Invincibility frames after taking damage
    boss_state: u8,  // FSM state (IDLE, WALK, ATTACK, etc.)
    boss_state_timer: u8,  // Timer for current state duration
    boss_anim_timer: u8,  // Timer for animation frames
    boss_phase: u8,  // 1, 2, or 3 based on HP
    boss_phase_transition: u8,  // Timer for phase transition animation
    boss_attack_pattern: u8,  // Phase 3 pattern cycle (0=high jump, 1=ground pound, 2=dash)
    boss_hitbox_active: bool,  // True when melee hitbox is active
    boss_warning_active: bool,  // True when showing telegraph warning
    boss_on_ground: bool,  // True when boss is on a platform

    // Platform crumbling (Phase 3)
    platform_used_left: bool,  // Left platform landed on once
    platform_used_right: bool,  // Right platform landed on once
    platform_crumbled_left: bool,  // Left platform destroyed
    platform_crumbled_right: bool,  // Right platform destroyed

    // Player stun (from boss roar)
    player_stunned: bool,
    player_stun_timer: u8,

    // Player stomp cooldown (5 seconds = 300 frames at 60fps)
    player_stomp_cooldown: u16,

    // Player weapons in boss fight
    selected_weapon: u8,  // WEAPON_GIFT_BOMB or WEAPON_GIFT_GUN
    weapon_cooldown: u8,

    // Player projectiles (gift bombs and gift gun bullets)
    proj1_x: f32, proj1_y: f32, proj1_vx: f32, proj1_vy: f32, proj1_active: bool, proj1_type: u8,
    proj2_x: f32, proj2_y: f32, proj2_vx: f32, proj2_vy: f32, proj2_active: bool, proj2_type: u8,
    proj3_x: f32, proj3_y: f32, proj3_vx: f32, proj3_vy: f32, proj3_active: bool, proj3_type: u8,
    proj4_x: f32, proj4_y: f32, proj4_vx: f32, proj4_vy: f32, proj4_active: bool, proj4_type: u8,
    proj5_x: f32, proj5_y: f32, proj5_vx: f32, proj5_vy: f32, proj5_active: bool, proj5_type: u8,
    proj6_x: f32, proj6_y: f32, proj6_vx: f32, proj6_vy: f32, proj6_active: bool, proj6_type: u8,
    proj7_x: f32, proj7_y: f32, proj7_vx: f32, proj7_vy: f32, proj7_active: bool, proj7_type: u8,
    proj8_x: f32, proj8_y: f32, proj8_vx: f32, proj8_vy: f32, proj8_active: bool, proj8_type: u8,
    proj9_x: f32, proj9_y: f32, proj9_vx: f32, proj9_vy: f32, proj9_active: bool, proj9_type: u8,
    proj10_x: f32, proj10_y: f32, proj10_vx: f32, proj10_vy: f32, proj10_active: bool, proj10_type: u8,

    boss_defeated: bool,  // True when boss is defeated

    // Phase animation tracking (visual only, play once)
    phase2_shown: bool,
    phase3_shown: bool,
}


impl GameState {
    pub fn new() -> Self {
        let state = Self {
            player_x: 50.0,
            player_y: 100.0,
            player_vx: 0.0,
            player_vy: 0.0,
            player_state: STATE_IDLE,
            player_facing_right: true,
            player_on_ground: false,
            player_on_ice: false,
            coyote_timer: 0,
            jump_buffer_timer: 0,
            anim_frame: 0,
            death_timer: 0,
            falling_sound_played: false,
            death_sound_played: false,

            lives: 3,
            total_score: 0,
            level_score: 0,
            level_time: 0,
            level_deaths: 0,
            game_time: 0,
            camera_x: 0.0,
            current_level: 1,
            parallax_offset: 0.0,
            level_complete: false,
            game_complete: false,  // Game not completed yet
            arcade_mode: false,    // Not in arcade mode initially

            // Initialize level records
            level_best_time: [999999; 10],  // Start with max time
            level_best_score: [0; 10],      // No scores yet
            level_rank: [0; 10],            // No ranks yet

            // Level select system - start at level select screen
            current_screen: SCREEN_LEVEL_SELECT,
            level_unlocked: [true, false, false, false, false, false, false, false, false, false],  // Only level 1 unlocked
            level_gifts_collected: [0; 10],  // No gifts collected yet
            selected_level: 1,  // Start with level 1 selected
            level_complete_timer: 0,
            boss_transition_timer: 0,
            input_cooldown: 0,

            // Settings system - default to music and sfx enabled
            music_enabled: true,
            music_volume: 0.2,    // 20% volume by default
            sfx_enabled: true,
            sfx_volume: 1.0,      // 100% volume by default
            selected_setting: SETTING_MUSIC,
            previous_screen: SCREEN_LEVEL_SELECT,

            // Developer mode
            developer_mode: false,
            password_input: String::new(),
            entering_password: false,

            chimney_x: 0.0,
            chimney_y: 0.0,

            // Initialize platforms
            platform_count: 0,
            plat1_x1: 0.0, plat1_x2: 0.0, plat1_y: 0.0,
            plat2_x1: 0.0, plat2_x2: 0.0, plat2_y: 0.0,
            plat3_x1: 0.0, plat3_x2: 0.0, plat3_y: 0.0,
            plat4_x1: 0.0, plat4_x2: 0.0, plat4_y: 0.0,
            plat5_x1: 0.0, plat5_x2: 0.0, plat5_y: 0.0,
            plat6_x1: 0.0, plat6_x2: 0.0, plat6_y: 0.0,
            plat7_x1: 0.0, plat7_x2: 0.0, plat7_y: 0.0,
            plat8_x1: 0.0, plat8_x2: 0.0, plat8_y: 0.0,
            plat9_x1: 0.0, plat9_x2: 0.0, plat9_y: 0.0,
            plat10_x1: 0.0, plat10_x2: 0.0, plat10_y: 0.0,

            // Initialize all enemies as inactive
            enemy1_x: 0.0, enemy1_y: 0.0, enemy1_vx: 0.0, enemy1_vy: 0.0, enemy1_type: ENEMY_NONE,
            enemy1_facing_right: false, enemy1_active: false, enemy1_min_x: 0.0, enemy1_max_x: 0.0,
            enemy1_state: ENEMY_STATE_ALIVE, enemy1_death_timer: 0, enemy1_attack_timer: 0,

            enemy2_x: 0.0, enemy2_y: 0.0, enemy2_vx: 0.0, enemy2_vy: 0.0, enemy2_type: ENEMY_NONE,
            enemy2_facing_right: false, enemy2_active: false, enemy2_min_x: 0.0, enemy2_max_x: 0.0,
            enemy2_state: ENEMY_STATE_ALIVE, enemy2_death_timer: 0, enemy2_attack_timer: 0,

            enemy3_x: 0.0, enemy3_y: 0.0, enemy3_vx: 0.0, enemy3_vy: 0.0, enemy3_type: ENEMY_NONE,
            enemy3_facing_right: false, enemy3_active: false, enemy3_min_x: 0.0, enemy3_max_x: 0.0,
            enemy3_state: ENEMY_STATE_ALIVE, enemy3_death_timer: 0, enemy3_attack_timer: 0,

            enemy4_x: 0.0, enemy4_y: 0.0, enemy4_vx: 0.0, enemy4_vy: 0.0, enemy4_type: ENEMY_NONE,
            enemy4_facing_right: false, enemy4_active: false, enemy4_min_x: 0.0, enemy4_max_x: 0.0,
            enemy4_state: ENEMY_STATE_ALIVE, enemy4_death_timer: 0, enemy4_attack_timer: 0,

            enemy5_x: 0.0, enemy5_y: 0.0, enemy5_vx: 0.0, enemy5_vy: 0.0, enemy5_type: ENEMY_NONE,
            enemy5_facing_right: false, enemy5_active: false, enemy5_min_x: 0.0, enemy5_max_x: 0.0,
            enemy5_state: ENEMY_STATE_ALIVE, enemy5_death_timer: 0, enemy5_attack_timer: 0,

            enemy6_x: 0.0, enemy6_y: 0.0, enemy6_vx: 0.0, enemy6_vy: 0.0, enemy6_type: ENEMY_NONE,
            enemy6_facing_right: false, enemy6_active: false, enemy6_min_x: 0.0, enemy6_max_x: 0.0,
            enemy6_state: ENEMY_STATE_ALIVE, enemy6_death_timer: 0, enemy6_attack_timer: 0,

            enemy7_x: 0.0, enemy7_y: 0.0, enemy7_vx: 0.0, enemy7_vy: 0.0, enemy7_type: ENEMY_NONE,
            enemy7_facing_right: false, enemy7_active: false, enemy7_min_x: 0.0, enemy7_max_x: 0.0,
            enemy7_state: ENEMY_STATE_ALIVE, enemy7_death_timer: 0, enemy7_attack_timer: 0,

            enemy8_x: 0.0, enemy8_y: 0.0, enemy8_vx: 0.0, enemy8_vy: 0.0, enemy8_type: ENEMY_NONE,
            enemy8_facing_right: false, enemy8_active: false, enemy8_min_x: 0.0, enemy8_max_x: 0.0,
            enemy8_state: ENEMY_STATE_ALIVE, enemy8_death_timer: 0, enemy8_attack_timer: 0,

            enemy9_x: 0.0, enemy9_y: 0.0, enemy9_vx: 0.0, enemy9_vy: 0.0, enemy9_type: ENEMY_NONE,
            enemy9_facing_right: false, enemy9_active: false, enemy9_min_x: 0.0, enemy9_max_x: 0.0,
            enemy9_state: ENEMY_STATE_ALIVE, enemy9_death_timer: 0, enemy9_attack_timer: 0,

            enemy10_x: 0.0, enemy10_y: 0.0, enemy10_vx: 0.0, enemy10_vy: 0.0, enemy10_type: ENEMY_NONE,
            enemy10_facing_right: false, enemy10_active: false, enemy10_min_x: 0.0, enemy10_max_x: 0.0,
            enemy10_state: ENEMY_STATE_ALIVE, enemy10_death_timer: 0, enemy10_attack_timer: 0,

            // Initialize all gifts as uncollected
            gift1_x: 0.0, gift1_y: 0.0, gift1_collected: true,  // Start as collected (invisible)
            gift2_x: 0.0, gift2_y: 0.0, gift2_collected: true,
            gift3_x: 0.0, gift3_y: 0.0, gift3_collected: true,
            gift4_x: 0.0, gift4_y: 0.0, gift4_collected: true,
            gift5_x: 0.0, gift5_y: 0.0, gift5_collected: true,

            all_gifts_sound_played: false,

            // Initialize snowball projectiles as inactive
            snowball1_x: 0.0, snowball1_y: 0.0, snowball1_vx: 0.0, snowball1_start_x: 0.0, snowball1_active: false,
            snowball2_x: 0.0, snowball2_y: 0.0, snowball2_vx: 0.0, snowball2_start_x: 0.0, snowball2_active: false,
            snowball3_x: 0.0, snowball3_y: 0.0, snowball3_vx: 0.0, snowball3_start_x: 0.0, snowball3_active: false,
            snowball4_x: 0.0, snowball4_y: 0.0, snowball4_vx: 0.0, snowball4_start_x: 0.0, snowball4_active: false,

            // Initialize boss fight state
            in_boss_fight: false,
            boss_hp: BOSS_HP_MAX,
            boss_x: 0.0,
            boss_y: 0.0,
            boss_vx: 0.0,
            boss_vy: 0.0,
            boss_facing_right: false,
            boss_attack_timer: 0,
            boss_attack_type: 0,
            boss_invincible_timer: 0,
            boss_state: BOSS_STATE_IDLE,
            boss_state_timer: 0,
            boss_anim_timer: 0,
            boss_phase: 1,
            boss_phase_transition: 0,
            boss_attack_pattern: 0,
            boss_hitbox_active: false,
            boss_warning_active: false,
            boss_on_ground: false,

            // Platform crumbling
            platform_used_left: false,
            platform_used_right: false,
            platform_crumbled_left: false,
            platform_crumbled_right: false,

            // Player stun
            player_stunned: false,
            player_stun_timer: 0,

            // Player stomp cooldown (starts ready to stomp)
            player_stomp_cooldown: 300,  // 5 seconds (300 frames)

            // Initialize player weapons
            selected_weapon: WEAPON_GIFT_BOMB,  // Start with gift bombs
            weapon_cooldown: 0,

            // Initialize player projectiles as inactive
            proj1_x: 0.0, proj1_y: 0.0, proj1_vx: 0.0, proj1_vy: 0.0, proj1_active: false, proj1_type: 0,
            proj2_x: 0.0, proj2_y: 0.0, proj2_vx: 0.0, proj2_vy: 0.0, proj2_active: false, proj2_type: 0,
            proj3_x: 0.0, proj3_y: 0.0, proj3_vx: 0.0, proj3_vy: 0.0, proj3_active: false, proj3_type: 0,
            proj4_x: 0.0, proj4_y: 0.0, proj4_vx: 0.0, proj4_vy: 0.0, proj4_active: false, proj4_type: 0,
            proj5_x: 0.0, proj5_y: 0.0, proj5_vx: 0.0, proj5_vy: 0.0, proj5_active: false, proj5_type: 0,
            proj6_x: 0.0, proj6_y: 0.0, proj6_vx: 0.0, proj6_vy: 0.0, proj6_active: false, proj6_type: 0,
            proj7_x: 0.0, proj7_y: 0.0, proj7_vx: 0.0, proj7_vy: 0.0, proj7_active: false, proj7_type: 0,
            proj8_x: 0.0, proj8_y: 0.0, proj8_vx: 0.0, proj8_vy: 0.0, proj8_active: false, proj8_type: 0,
            proj9_x: 0.0, proj9_y: 0.0, proj9_vx: 0.0, proj9_vy: 0.0, proj9_active: false, proj9_type: 0,
            proj10_x: 0.0, proj10_y: 0.0, proj10_vx: 0.0, proj10_vy: 0.0, proj10_active: false, proj10_type: 0,

            boss_defeated: false,

            // Phase animation tracking
            phase2_shown: false,
            phase3_shown: false,
        };

        state
    }

    pub fn update_bgm(&mut self) {
        // Manage background music playback
        if self.music_enabled {
            // Start playing if not already playing
            if !audio::is_playing("sleighride") {
                audio::play("sleighride");
            }

            // Apply exponential volume curve for natural perception
            // Human hearing is logarithmic, so we use volume^3 for better control at lower volumes
            let actual_volume = self.music_volume * self.music_volume * self.music_volume;

            // Update volume if needed
            let current_vol = audio::get_volume("sleighride");
            if (current_vol - actual_volume).abs() > 0.01 {
                audio::set_volume("sleighride", actual_volume);
            }
        } else {
            // Music disabled - stop playback
            if audio::is_playing("sleighride") {
                audio::stop("sleighride");
            }
        }
    }

    pub fn play_sfx(&self, sound_name: &str) {
        // Play sound effect if SFX is enabled
        if self.sfx_enabled {
            audio::play(sound_name);
            // Apply volume with cubic curve (same as BGM)
            let actual_volume = self.sfx_volume * self.sfx_volume * self.sfx_volume;
            audio::set_volume(sound_name, actual_volume);
        }
    }

    /// Play boss-related SFX with priority handling
    /// Priority: krampus_hurt (1) > krampus_chase_03 (2) > krampus_death_01 (3) > krampus_onkill_01 (4)
    pub fn play_boss_sfx(&self, sound_name: &str) {
        if !self.sfx_enabled {
            return;
        }

        // Check priority - higher priority sounds block lower priority
        let priority = match sound_name {
            "krampus_hurt" => 1,        // Highest priority
            "krampus_chase_03" => 2,    // Roar/attack
            "krampus_death_01" => 3,    // Death
            "krampus_onkill_01" => 4,   // Killed Santa
            _ => 5,                      // Lowest
        };

        // Don't play if higher priority sound is playing
        if priority > 1 && audio::is_playing("krampus_hurt") {
            return;
        }
        if priority > 2 && audio::is_playing("krampus_chase_03") {
            return;
        }
        if priority > 3 && audio::is_playing("krampus_death_01") {
            return;
        }

        // Play the sound
        audio::play(sound_name);
        let actual_volume = self.sfx_volume * self.sfx_volume * self.sfx_volume;
        audio::set_volume(sound_name, actual_volume);
    }

    pub fn update(&mut self) {
        self.anim_frame += 1;

        // Update background music
        self.update_bgm();

        // Level select screen
        if self.current_screen == SCREEN_LEVEL_SELECT {
            self.update_level_select();
            self.render_level_select();
            return;
        }

        // Settings screen
        if self.current_screen == SCREEN_SETTINGS {
            self.update_settings();
            self.render_settings();
            return;
        }

        // Level complete screen - show stats screen
        if self.current_screen == SCREEN_LEVEL_COMPLETE {
            let kb = keyboard::get();
            let gp = gamepad::get(0);

            // Allow player to continue with SPACE after seeing stats
            if kb.space().just_pressed() || gp.a.just_pressed() {
                self.current_screen = SCREEN_LEVEL_SELECT;
                self.level_complete_timer = 0;
            }

            self.render();
            return;
        }

        // Boss transition screen - dramatic reveal before boss fight
        if self.current_screen == SCREEN_BOSS_TRANSITION {
            self.boss_transition_timer += 1;

            let kb = keyboard::get();
            let gp = gamepad::get(0);

            // After 1 second, allow player to continue with SPACE
            if self.boss_transition_timer > 60 && (kb.space().just_pressed() || gp.a.just_pressed()) {
                self.load_boss_fight();
            }

            self.render();
            return;
        }

        // Game complete screen - show options for arcade mode or restart
        if self.game_complete {
            let kb = keyboard::get();
            let gp = gamepad::get(0);

            // Press SPACE or START to restart from Level 1
            if kb.space().just_pressed() || gp.start.just_pressed() {
                self.game_complete = false;
                self.arcade_mode = false;
                self.total_score = 0;  // Reset total score for new game
                self.load_level(1);
            }
            // Press DOWN arrow or SELECT to start Arcade Mode
            else if kb.arrow_down().just_pressed() || gp.select.just_pressed() {
                self.game_complete = false;
                self.arcade_mode = true;
                self.total_score = 0;  // Reset for arcade mode
                self.load_level(1);  // Start arcade mode from level 1
            }

            self.render();
            return;
        }

        // Level start screen - wait for space key press
        if self.player_state == STATE_LEVEL_START {
            let kb = keyboard::get();
            let gp = gamepad::get(0);

            // Check keys
            let esc_pressed = kb.escape().just_pressed();
            let space_pressed = kb.space().just_pressed();

            // Start level with SPACE or A button (takes priority over settings)
            if space_pressed || gp.a.just_pressed() {
                self.player_state = STATE_IDLE;
            }
            // Allow opening settings with ESC only if SPACE is NOT pressed
            else if esc_pressed {
                self.previous_screen = SCREEN_PLAYING;
                self.current_screen = SCREEN_SETTINGS;
                self.render();
                return;
            }
            // Gamepad START can also open settings
            else if gp.start.just_pressed() {
                self.previous_screen = SCREEN_PLAYING;
                self.current_screen = SCREEN_SETTINGS;
                self.render();
                return;
            }
            self.render();
            return;
        }

        if self.player_state == STATE_DEAD {
            self.handle_death();
            self.render();
            return;
        }

        // Check for ESC key to open settings during gameplay
        let kb = keyboard::get();
        let gp = gamepad::get(0);
        let esc_pressed = kb.escape().just_pressed();
        let space_pressed = kb.space().just_pressed();

        // If SPACE is pressed, do NOT open settings under any circumstances
        if space_pressed {
            // SPACE should never open settings - continue to normal gameplay
        } else if esc_pressed {
            // Only ESC can open settings
            self.previous_screen = SCREEN_PLAYING;
            self.current_screen = SCREEN_SETTINGS;
            self.render();
            return;
        } else if gp.start.just_pressed() {
            // Gamepad START can also open settings
            self.previous_screen = SCREEN_PLAYING;
            self.current_screen = SCREEN_SETTINGS;
            self.render();
            return;
        }

        self.handle_input();
        self.update_player();

        // Update enemies OR boss (mutually exclusive)
        if self.in_boss_fight {
            self.update_boss();  // Boss FSM update only
        } else {
            self.update_enemies();  // Normal level enemies
        }

        self.update_projectiles();
        self.update_camera();
        self.game_time += 1;
        self.level_time += 1;  // Track time spent in current level
        self.render();
    }
}
