use crate::*;

impl GameState {
    pub fn load_level(&mut self, level: u8) {
        self.current_level = level;
        self.level_complete = false;
        self.current_screen = SCREEN_PLAYING;  // Set to playing screen

        // CRITICAL: Clear all boss fight state when loading regular levels!
        self.in_boss_fight = false;
        self.boss_defeated = false;
        self.boss_transition_timer = 0;

        self.player_x = 50.0;
        self.player_y = 118.0;  // Will be adjusted by each level's platform
        self.player_vx = 0.0;
        self.player_vy = 0.0;
        self.camera_x = 0.0;

        // Start level immediately (no press to start screen)
        self.player_state = STATE_IDLE;
        self.death_timer = 0;

        // Reset level-specific stats for fresh attempt
        self.level_score = 0;
        self.level_time = 0;
        self.level_deaths = 0;
        self.all_gifts_sound_played = false;  // Reset gift sound flag
        self.falling_sound_played = false;  // Reset falling sound flag
        self.death_sound_played = false;  // Reset death sound flag

        // NOTE: Lives are NOT reset here! Lives only reset when:
        // 1. Starting new game (in new())
        // 2. Advancing to next level (in check_chimney_collision())
        // 3. Game over - returning to level select (in handle_death())

        // Reset ALL enemies and projectiles to default state
        self.enemy1_active = false; self.enemy1_state = ENEMY_STATE_ALIVE; self.enemy1_death_timer = 0; self.enemy1_attack_timer = 0;
        self.enemy2_active = false; self.enemy2_state = ENEMY_STATE_ALIVE; self.enemy2_death_timer = 0; self.enemy2_attack_timer = 0;
        self.enemy3_active = false; self.enemy3_state = ENEMY_STATE_ALIVE; self.enemy3_death_timer = 0; self.enemy3_attack_timer = 0;
        self.enemy4_active = false; self.enemy4_state = ENEMY_STATE_ALIVE; self.enemy4_death_timer = 0; self.enemy4_attack_timer = 0;
        self.enemy5_active = false; self.enemy5_state = ENEMY_STATE_ALIVE; self.enemy5_death_timer = 0; self.enemy5_attack_timer = 0;
        self.enemy6_active = false; self.enemy6_state = ENEMY_STATE_ALIVE; self.enemy6_death_timer = 0; self.enemy6_attack_timer = 0;
        self.enemy7_active = false; self.enemy7_state = ENEMY_STATE_ALIVE; self.enemy7_death_timer = 0; self.enemy7_attack_timer = 0;
        self.enemy8_active = false; self.enemy8_state = ENEMY_STATE_ALIVE; self.enemy8_death_timer = 0; self.enemy8_attack_timer = 0;
        self.enemy9_active = false; self.enemy9_state = ENEMY_STATE_ALIVE; self.enemy9_death_timer = 0; self.enemy9_attack_timer = 0;
        self.enemy10_active = false; self.enemy10_state = ENEMY_STATE_ALIVE; self.enemy10_death_timer = 0; self.enemy10_attack_timer = 0;

        // Clear all snowballs
        self.snowball1_active = false;
        self.snowball2_active = false;
        self.snowball3_active = false;
        self.snowball4_active = false;

        match level {
            1 => self.load_level_1(),
            2 => self.load_level_2(),
            3 => self.load_level_3(),
            4 => self.load_level_4(),
            5 => self.load_level_5(),
            6 => self.load_level_6(),
            7 => self.load_level_7(),
            8 => self.load_level_8(),
            9 => self.load_level_9(),
            10 => self.load_level_10(),
            _ => self.load_level_1(),  // Default to level 1
        }
    }

    // Level 1: "Rooftop Warm-Up" - Easy tutorial (~2000px)
    pub fn load_level_1(&mut self) {
        self.platform_count = 7;

        // Starting platform
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 350.0;
        self.plat1_y = 150.0;

        // 70px gap
        self.plat2_x1 = 420.0;
        self.plat2_x2 = 700.0;
        self.plat2_y = 145.0;

        // 80px gap
        self.plat3_x1 = 780.0;
        self.plat3_x2 = 1050.0;
        self.plat3_y = 150.0;

        // 70px gap
        self.plat4_x1 = 1120.0;
        self.plat4_x2 = 1400.0;
        self.plat4_y = 140.0;

        // 80px gap
        self.plat5_x1 = 1480.0;
        self.plat5_x2 = 1750.0;
        self.plat5_y = 150.0;

        // 70px gap - near goal
        self.plat6_x1 = 1820.0;
        self.plat6_x2 = 2050.0;
        self.plat6_y = 145.0;

        // FINAL GOAL PLATFORM
        self.plat7_x1 = 1850.0;
        self.plat7_x2 = 2100.0;
        self.plat7_y = 150.0;

        self.plat8_x1 = 0.0; self.plat8_x2 = 0.0; self.plat8_y = 0.0;
        self.plat9_x1 = 0.0; self.plat9_x2 = 0.0; self.plat9_y = 0.0;
        self.plat10_x1 = 0.0; self.plat10_x2 = 0.0; self.plat10_y = 0.0;

        // GIFTS
        self.gift1_x = 175.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 560.0;
        self.gift2_y = 95.0;
        self.gift2_collected = false;

        self.gift3_x = 915.0;
        self.gift3_y = 100.0;
        self.gift3_collected = false;

        self.gift4_x = 1260.0;
        self.gift4_y = 90.0;
        self.gift4_collected = false;

        self.gift5_x = 1615.0;
        self.gift5_y = 100.0;
        self.gift5_collected = false;

        // Enemies - just 1 for tutorial
        self.enemy1_x = 1260.0;
        self.enemy1_y = self.plat4_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.4;
        self.enemy1_type = ENEMY_GINGERBREAD;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat4_x1 + 20.0;
        self.enemy1_max_x = self.plat4_x2 - 20.0;

        self.enemy2_active = false;
        self.enemy3_active = false;
        self.enemy4_active = false;
        self.enemy5_active = false;

        // Chimney (goal)
        self.chimney_x = 1975.0;
        self.chimney_y = self.plat7_y - 48.0;
    }

    // Level 2: "Gingerbread Alley" - Easy (~2200px)
    pub fn load_level_2(&mut self) {
        self.platform_count = 7;

        // Starting platform
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 350.0;
        self.plat1_y = 150.0;

        // 70px gap
        self.plat2_x1 = 420.0;
        self.plat2_x2 = 750.0;
        self.plat2_y = 145.0;

        // 80px gap
        self.plat3_x1 = 830.0;
        self.plat3_x2 = 1150.0;
        self.plat3_y = 140.0;

        // 70px gap
        self.plat4_x1 = 1220.0;
        self.plat4_x2 = 1500.0;
        self.plat4_y = 150.0;

        // 80px gap
        self.plat5_x1 = 1580.0;
        self.plat5_x2 = 1850.0;
        self.plat5_y = 145.0;

        // 70px gap
        self.plat6_x1 = 1920.0;
        self.plat6_x2 = 2150.0;
        self.plat6_y = 150.0;

        // FINAL GOAL PLATFORM - 80px gap
        self.plat7_x1 = 2000.0;
        self.plat7_x2 = 2300.0;
        self.plat7_y = 150.0;

        self.plat8_x1 = 0.0; self.plat8_x2 = 0.0; self.plat8_y = 0.0;
        self.plat9_x1 = 0.0; self.plat9_x2 = 0.0; self.plat9_y = 0.0;
        self.plat10_x1 = 0.0; self.plat10_x2 = 0.0; self.plat10_y = 0.0;

        // GIFTS
        self.gift1_x = 175.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 585.0;
        self.gift2_y = 95.0;
        self.gift2_collected = false;

        self.gift3_x = 990.0;
        self.gift3_y = 90.0;
        self.gift3_collected = false;

        self.gift4_x = 1360.0;
        self.gift4_y = 100.0;
        self.gift4_collected = false;

        self.gift5_x = 1715.0;
        self.gift5_y = 95.0;
        self.gift5_collected = false;

        // ENEMIES - 3 for this level
        self.enemy1_x = 585.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.5;
        self.enemy1_type = ENEMY_GINGERBREAD;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 990.0;
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.5;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1360.0;
        self.enemy3_y = self.plat4_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.5;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat4_x1 + 20.0;
        self.enemy3_max_x = self.plat4_x2 - 20.0;

        self.enemy4_active = false;
        self.enemy5_active = false;

        // Chimney
        self.chimney_x = 2150.0;
        self.chimney_y = self.plat7_y - 48.0;
    }

    // Level 3: "Long Jump District" - Early Medium (~2200px)
    pub fn load_level_3(&mut self) {
        self.platform_count = 7;

        // Start platform
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // 80px gap
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 650.0;
        self.plat2_y = 145.0;

        // 90px gap
        self.plat3_x1 = 740.0;
        self.plat3_x2 = 1000.0;
        self.plat3_y = 140.0;

        // 80px gap
        self.plat4_x1 = 1080.0;
        self.plat4_x2 = 1350.0;
        self.plat4_y = 150.0;

        // 90px gap
        self.plat5_x1 = 1440.0;
        self.plat5_x2 = 1700.0;
        self.plat5_y = 140.0;

        // 80px gap
        self.plat6_x1 = 1780.0;
        self.plat6_x2 = 2000.0;
        self.plat6_y = 150.0;

        // FINAL GOAL PLATFORM
        self.plat7_x1 = 1950.0;
        self.plat7_x2 = 2250.0;
        self.plat7_y = 150.0;

        self.plat8_x1 = 0.0; self.plat8_x2 = 0.0; self.plat8_y = 0.0;
        self.plat9_x1 = 0.0; self.plat9_x2 = 0.0; self.plat9_y = 0.0;
        self.plat10_x1 = 0.0; self.plat10_x2 = 0.0; self.plat10_y = 0.0;

        // GIFTS
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 515.0;
        self.gift2_y = 95.0;
        self.gift2_collected = false;

        self.gift3_x = 870.0;
        self.gift3_y = 90.0;
        self.gift3_collected = false;

        self.gift4_x = 1215.0;
        self.gift4_y = 100.0;
        self.gift4_collected = false;

        self.gift5_x = 1570.0;
        self.gift5_y = 90.0;
        self.gift5_collected = false;

        // ENEMIES - 4 for this level
        self.enemy1_x = 515.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.6;
        self.enemy1_type = ENEMY_GINGERBREAD;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 870.0;
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.6;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1215.0;
        self.enemy3_y = self.plat4_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.6;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat4_x1 + 20.0;
        self.enemy3_max_x = self.plat4_x2 - 20.0;

        self.enemy4_x = 1570.0;
        self.enemy4_y = self.plat5_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.6;
        self.enemy4_type = ENEMY_GINGERBREAD;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat5_x1 + 20.0;
        self.enemy4_max_x = self.plat5_x2 - 20.0;

        self.enemy5_active = false;

        // Chimney
        self.chimney_x = 2100.0;
        self.chimney_y = self.plat7_y - 48.0;
    }

    // Level 4: "Highrise Gifts" - Mid Level (~2400px)
    pub fn load_level_4(&mut self) {
        self.platform_count = 8;

        // Starting platform
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // 80px gap
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 650.0;
        self.plat2_y = 140.0;

        // 90px gap
        self.plat3_x1 = 740.0;
        self.plat3_x2 = 1000.0;
        self.plat3_y = 135.0;

        // 80px gap
        self.plat4_x1 = 1080.0;
        self.plat4_x2 = 1350.0;
        self.plat4_y = 150.0;

        // 90px gap
        self.plat5_x1 = 1440.0;
        self.plat5_x2 = 1700.0;
        self.plat5_y = 140.0;

        // 100px gap - harder
        self.plat6_x1 = 1800.0;
        self.plat6_x2 = 2050.0;
        self.plat6_y = 150.0;

        // 90px gap
        self.plat7_x1 = 2140.0;
        self.plat7_x2 = 2350.0;
        self.plat7_y = 145.0;

        // FINAL GOAL PLATFORM
        self.plat8_x1 = 2200.0;
        self.plat8_x2 = 2500.0;
        self.plat8_y = 150.0;

        self.plat9_x1 = 0.0; self.plat9_x2 = 0.0; self.plat9_y = 0.0;
        self.plat10_x1 = 0.0; self.plat10_x2 = 0.0; self.plat10_y = 0.0;

        // GIFTS
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 515.0;
        self.gift2_y = 90.0;
        self.gift2_collected = false;

        self.gift3_x = 870.0;
        self.gift3_y = 85.0;
        self.gift3_collected = false;

        self.gift4_x = 1215.0;
        self.gift4_y = 100.0;
        self.gift4_collected = false;

        self.gift5_x = 1570.0;
        self.gift5_y = 90.0;
        self.gift5_collected = false;

        // ENEMIES - 5 with Grinches
        self.enemy1_x = 515.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.7;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 870.0;
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.6;
        self.enemy2_type = ENEMY_GRINCH;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1215.0;
        self.enemy3_y = self.plat4_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.7;
        self.enemy3_type = ENEMY_GINGERBREAD;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat4_x1 + 20.0;
        self.enemy3_max_x = self.plat4_x2 - 20.0;

        self.enemy4_x = 1570.0;
        self.enemy4_y = self.plat5_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.6;
        self.enemy4_type = ENEMY_GRINCH;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat5_x1 + 20.0;
        self.enemy4_max_x = self.plat5_x2 - 20.0;

        self.enemy5_x = 1925.0;
        self.enemy5_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 0.8;
        self.enemy5_type = ENEMY_GINGERBREAD;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat6_x1 + 20.0;
        self.enemy5_max_x = self.plat6_x2 - 20.0;

        // Chimney
        self.chimney_x = 2350.0;
        self.chimney_y = self.plat8_y - 48.0;
    }

    // Level 5: "Ice Bridge City" - Mid Level (~2500px)
    pub fn load_level_5(&mut self) {
        self.platform_count = 8;

        // Starting platform
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // 90px gap
        self.plat2_x1 = 390.0;
        self.plat2_x2 = 650.0;
        self.plat2_y = 145.0;

        // 100px gap
        self.plat3_x1 = 750.0;
        self.plat3_x2 = 1000.0;
        self.plat3_y = 140.0;

        // 90px gap
        self.plat4_x1 = 1090.0;
        self.plat4_x2 = 1350.0;
        self.plat4_y = 150.0;

        // 100px gap
        self.plat5_x1 = 1450.0;
        self.plat5_x2 = 1700.0;
        self.plat5_y = 140.0;

        // 90px gap
        self.plat6_x1 = 1790.0;
        self.plat6_x2 = 2050.0;
        self.plat6_y = 150.0;

        // 100px gap
        self.plat7_x1 = 2150.0;
        self.plat7_x2 = 2400.0;
        self.plat7_y = 145.0;

        // FINAL GOAL PLATFORM
        self.plat8_x1 = 2300.0;
        self.plat8_x2 = 2600.0;
        self.plat8_y = 150.0;

        self.plat9_x1 = 0.0; self.plat9_x2 = 0.0; self.plat9_y = 0.0;
        self.plat10_x1 = 0.0; self.plat10_x2 = 0.0; self.plat10_y = 0.0;

        // GIFTS
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 520.0;
        self.gift2_y = 95.0;
        self.gift2_collected = false;

        self.gift3_x = 875.0;
        self.gift3_y = 90.0;
        self.gift3_collected = false;

        self.gift4_x = 1220.0;
        self.gift4_y = 100.0;
        self.gift4_collected = false;

        self.gift5_x = 1575.0;
        self.gift5_y = 90.0;
        self.gift5_collected = false;

        // ENEMIES - 5 fast
        self.enemy1_x = 520.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.7;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 875.0;
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.7;
        self.enemy2_type = ENEMY_GRINCH;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1220.0;
        self.enemy3_y = self.plat4_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.8;
        self.enemy3_type = ENEMY_GINGERBREAD;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat4_x1 + 20.0;
        self.enemy3_max_x = self.plat4_x2 - 20.0;

        self.enemy4_x = 1575.0;
        self.enemy4_y = self.plat5_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.7;
        self.enemy4_type = ENEMY_GRINCH;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat5_x1 + 20.0;
        self.enemy4_max_x = self.plat5_x2 - 20.0;

        self.enemy5_x = 1920.0;
        self.enemy5_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 0.8;
        self.enemy5_type = ENEMY_GINGERBREAD;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat6_x1 + 20.0;
        self.enemy5_max_x = self.plat6_x2 - 20.0;

        // Chimney
        self.chimney_x = 2450.0;
        self.chimney_y = self.plat8_y - 48.0;
    }

    // Level 6: "Grinch Rooftops" - Mid-Late (~2600px) with 2nd floor
    pub fn load_level_6(&mut self) {
        self.platform_count = 10;

        // GROUND FLOOR (y=150)
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // Ground platform 2
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 650.0;
        self.plat2_y = 150.0;

        // 2ND FLOOR - elevated section (y=105)
        self.plat3_x1 = 500.0;
        self.plat3_x2 = 750.0;
        self.plat3_y = 105.0;

        // Ground platform 3
        self.plat4_x1 = 730.0;
        self.plat4_x2 = 1000.0;
        self.plat4_y = 150.0;

        // 2ND FLOOR - middle elevated
        self.plat5_x1 = 880.0;
        self.plat5_x2 = 1100.0;
        self.plat5_y = 105.0;

        // Ground platform 4
        self.plat6_x1 = 1080.0;
        self.plat6_x2 = 1400.0;
        self.plat6_y = 150.0;

        // 2ND FLOOR - high section
        self.plat7_x1 = 1280.0;
        self.plat7_x2 = 1500.0;
        self.plat7_y = 105.0;

        // Ground platform 5
        self.plat8_x1 = 1480.0;
        self.plat8_x2 = 1800.0;
        self.plat8_y = 150.0;

        // 2ND FLOOR - near goal
        self.plat9_x1 = 1700.0;
        self.plat9_x2 = 1950.0;
        self.plat9_y = 105.0;

        // FINAL GOAL PLATFORM
        self.plat10_x1 = 2040.0;
        self.plat10_x2 = 2400.0;
        self.plat10_y = 150.0;

        // GIFTS - some on 2nd floor!
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 625.0;  // On 2nd floor plat3
        self.gift2_y = 50.0;
        self.gift2_collected = false;

        self.gift3_x = 990.0;  // On 2nd floor plat5
        self.gift3_y = 45.0;
        self.gift3_collected = false;

        self.gift4_x = 1390.0;  // On 2nd floor plat7
        self.gift4_y = 40.0;
        self.gift4_collected = false;

        self.gift5_x = 1825.0;  // On 2nd floor plat9
        self.gift5_y = 50.0;
        self.gift5_collected = false;

        // ENEMIES - on both floors
        self.enemy1_x = 515.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.7;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 625.0;  // 2nd floor
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.6;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1240.0;
        self.enemy3_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.8;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat6_x1 + 20.0;
        self.enemy3_max_x = self.plat6_x2 - 20.0;

        self.enemy4_x = 1390.0;  // 2nd floor
        self.enemy4_y = self.plat7_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.7;
        self.enemy4_type = ENEMY_GINGERBREAD;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat7_x1 + 20.0;
        self.enemy4_max_x = self.plat7_x2 - 20.0;

        self.enemy5_x = 1640.0;
        self.enemy5_y = self.plat8_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 0.8;
        self.enemy5_type = ENEMY_GRINCH;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat8_x1 + 20.0;
        self.enemy5_max_x = self.plat8_x2 - 20.0;

        // Chimney
        self.chimney_x = 2250.0;
        self.chimney_y = self.plat10_y - 48.0;
    }

    // Level 7: "Rooftop Maze" - Late Level (~2800px) with 2nd floor
    pub fn load_level_7(&mut self) {
        self.platform_count = 10;

        // GROUND FLOOR
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // Ground platform 2
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 600.0;
        self.plat2_y = 150.0;

        // 2ND FLOOR
        self.plat3_x1 = 480.0;
        self.plat3_x2 = 750.0;
        self.plat3_y = 108.0;

        // Ground platform 3
        self.plat4_x1 = 730.0;
        self.plat4_x2 = 1050.0;
        self.plat4_y = 150.0;

        // 2ND FLOOR - elevated
        self.plat5_x1 = 950.0;
        self.plat5_x2 = 1200.0;
        self.plat5_y = 106.0;

        // Ground platform 4
        self.plat6_x1 = 1180.0;
        self.plat6_x2 = 1550.0;
        self.plat6_y = 150.0;

        // 2ND FLOOR - high
        self.plat7_x1 = 1450.0;
        self.plat7_x2 = 1700.0;
        self.plat7_y = 105.0;

        // Ground platform 5
        self.plat8_x1 = 1680.0;
        self.plat8_x2 = 2100.0;
        self.plat8_y = 150.0;

        // 2ND FLOOR - near goal
        self.plat9_x1 = 2000.0;
        self.plat9_x2 = 2300.0;
        self.plat9_y = 108.0;

        // FINAL GOAL PLATFORM
        self.plat10_x1 = 2400.0;
        self.plat10_x2 = 2700.0;
        self.plat10_y = 150.0;

        // GIFTS - some on 2nd floor
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 615.0;  // 2nd floor
        self.gift2_y = 45.0;
        self.gift2_collected = false;

        self.gift3_x = 1075.0;  // 2nd floor
        self.gift3_y = 40.0;
        self.gift3_collected = false;

        self.gift4_x = 1575.0;  // 2nd floor
        self.gift4_y = 35.0;
        self.gift4_collected = false;

        self.gift5_x = 2150.0;  // 2nd floor
        self.gift5_y = 45.0;
        self.gift5_collected = false;

        // ENEMIES - fast on both floors
        self.enemy1_x = 490.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.8;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 615.0;  // 2nd floor
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.7;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1365.0;
        self.enemy3_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.9;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat6_x1 + 20.0;
        self.enemy3_max_x = self.plat6_x2 - 20.0;

        self.enemy4_x = 1575.0;  // 2nd floor
        self.enemy4_y = self.plat7_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.8;
        self.enemy4_type = ENEMY_GINGERBREAD;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat7_x1 + 20.0;
        self.enemy4_max_x = self.plat7_x2 - 20.0;

        self.enemy5_x = 1890.0;
        self.enemy5_y = self.plat8_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 1.0;
        self.enemy5_type = ENEMY_GRINCH;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat8_x1 + 20.0;
        self.enemy5_max_x = self.plat8_x2 - 20.0;

        self.enemy6_active = false;
        self.enemy7_active = false;
        self.enemy8_active = false;
        self.enemy9_active = false;
        self.enemy10_active = false;

        // Chimney
        self.chimney_x = 2550.0;
        self.chimney_y = self.plat10_y - 48.0;
    }

    // Level 8: "Grinch Gauntlet" - Late Level (~2900px) with 2nd floor
    pub fn load_level_8(&mut self) {
        self.platform_count = 10;

        // GROUND FLOOR
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // Ground platform 2
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 650.0;
        self.plat2_y = 150.0;

        // 2ND FLOOR
        self.plat3_x1 = 530.0;
        self.plat3_x2 = 800.0;
        self.plat3_y = 106.0;

        // Ground platform 3
        self.plat4_x1 = 780.0;
        self.plat4_x2 = 1100.0;
        self.plat4_y = 150.0;

        // 2ND FLOOR - elevated
        self.plat5_x1 = 1000.0;
        self.plat5_x2 = 1300.0;
        self.plat5_y = 105.0;

        // Ground platform 4
        self.plat6_x1 = 1280.0;
        self.plat6_x2 = 1650.0;
        self.plat6_y = 150.0;

        // 2ND FLOOR - high
        self.plat7_x1 = 1550.0;
        self.plat7_x2 = 1850.0;
        self.plat7_y = 104.0;

        // Ground platform 5
        self.plat8_x1 = 1830.0;
        self.plat8_x2 = 2200.0;
        self.plat8_y = 150.0;

        // 2ND FLOOR - near goal
        self.plat9_x1 = 2100.0;
        self.plat9_x2 = 2400.0;
        self.plat9_y = 106.0;

        // FINAL GOAL PLATFORM
        self.plat10_x1 = 2500.0;
        self.plat10_x2 = 2800.0;
        self.plat10_y = 150.0;

        // GIFTS - some on 2nd floor
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 665.0;  // 2nd floor
        self.gift2_y = 40.0;
        self.gift2_collected = false;

        self.gift3_x = 1150.0;  // 2nd floor
        self.gift3_y = 35.0;
        self.gift3_collected = false;

        self.gift4_x = 1700.0;  // 2nd floor
        self.gift4_y = 30.0;
        self.gift4_collected = false;

        self.gift5_x = 2250.0;  // 2nd floor
        self.gift5_y = 40.0;
        self.gift5_collected = false;

        // ENEMIES - tough on both floors
        self.enemy1_x = 515.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.8;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 665.0;  // 2nd floor
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.7;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1465.0;
        self.enemy3_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 0.9;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat6_x1 + 20.0;
        self.enemy3_max_x = self.plat6_x2 - 20.0;

        self.enemy4_x = 1700.0;  // 2nd floor
        self.enemy4_y = self.plat7_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.8;
        self.enemy4_type = ENEMY_GINGERBREAD;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat7_x1 + 20.0;
        self.enemy4_max_x = self.plat7_x2 - 20.0;

        self.enemy5_x = 2015.0;
        self.enemy5_y = self.plat8_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 1.0;
        self.enemy5_type = ENEMY_GRINCH;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat8_x1 + 20.0;
        self.enemy5_max_x = self.plat8_x2 - 20.0;

        self.enemy6_active = false;
        self.enemy7_active = false;
        self.enemy8_active = false;
        self.enemy9_active = false;
        self.enemy10_active = false;

        // Chimney
        self.chimney_x = 2650.0;
        self.chimney_y = self.plat10_y - 48.0;
    }

    // Level 9: "Krampus Pursuit" - Late Level (~3000px) with 2nd floor - hardest!
    pub fn load_level_9(&mut self) {
        self.platform_count = 10;

        // GROUND FLOOR
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 300.0;
        self.plat1_y = 150.0;

        // Ground platform 2
        self.plat2_x1 = 380.0;
        self.plat2_x2 = 700.0;
        self.plat2_y = 150.0;

        // 2ND FLOOR - high
        self.plat3_x1 = 580.0;
        self.plat3_x2 = 900.0;
        self.plat3_y = 105.0;

        // Ground platform 3
        self.plat4_x1 = 880.0;
        self.plat4_x2 = 1250.0;
        self.plat4_y = 150.0;

        // 2ND FLOOR - very high
        self.plat5_x1 = 1150.0;
        self.plat5_x2 = 1450.0;
        self.plat5_y = 103.0;

        // Ground platform 4
        self.plat6_x1 = 1430.0;
        self.plat6_x2 = 1850.0;
        self.plat6_y = 150.0;

        // 2ND FLOOR - highest
        self.plat7_x1 = 1750.0;
        self.plat7_x2 = 2100.0;
        self.plat7_y = 102.0;

        // Ground platform 5
        self.plat8_x1 = 2080.0;
        self.plat8_x2 = 2500.0;
        self.plat8_y = 150.0;

        // 2ND FLOOR - near goal
        self.plat9_x1 = 2400.0;
        self.plat9_x2 = 2700.0;
        self.plat9_y = 105.0;

        // FINAL GOAL PLATFORM
        self.plat10_x1 = 2800.0;
        self.plat10_x2 = 3100.0;
        self.plat10_y = 150.0;

        // GIFTS - on 2nd floor for challenge!
        self.gift1_x = 150.0;
        self.gift1_y = 100.0;
        self.gift1_collected = false;

        self.gift2_x = 740.0;  // 2nd floor
        self.gift2_y = 55.0;
        self.gift2_collected = false;

        self.gift3_x = 1300.0;  // 2nd floor
        self.gift3_y = 53.0;
        self.gift3_collected = false;

        self.gift4_x = 1925.0;  // 2nd floor - highest
        self.gift4_y = 52.0;
        self.gift4_collected = false;

        self.gift5_x = 2550.0;  // 2nd floor
        self.gift5_y = 55.0;
        self.gift5_collected = false;

        // ENEMIES - hardest level
        self.enemy1_x = 540.0;
        self.enemy1_y = self.plat2_y - 32.0 - ENEMY_HEIGHT;
        self.enemy1_vx = 0.9;
        self.enemy1_type = ENEMY_GRINCH;
        self.enemy1_facing_right = true;
        self.enemy1_active = true;
        self.enemy1_state = ENEMY_STATE_ALIVE;
        self.enemy1_death_timer = 0;
        self.enemy1_attack_timer = 0;
        self.enemy1_min_x = self.plat2_x1 + 20.0;
        self.enemy1_max_x = self.plat2_x2 - 20.0;

        self.enemy2_x = 740.0;  // 2nd floor
        self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
        self.enemy2_vx = 0.8;
        self.enemy2_type = ENEMY_GINGERBREAD;
        self.enemy2_facing_right = true;
        self.enemy2_active = true;
        self.enemy2_state = ENEMY_STATE_ALIVE;
        self.enemy2_death_timer = 0;
        self.enemy2_attack_timer = 0;
        self.enemy2_min_x = self.plat3_x1 + 20.0;
        self.enemy2_max_x = self.plat3_x2 - 20.0;

        self.enemy3_x = 1640.0;
        self.enemy3_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
        self.enemy3_vx = 1.0;
        self.enemy3_type = ENEMY_GRINCH;
        self.enemy3_facing_right = true;
        self.enemy3_active = true;
        self.enemy3_state = ENEMY_STATE_ALIVE;
        self.enemy3_death_timer = 0;
        self.enemy3_attack_timer = 0;
        self.enemy3_min_x = self.plat6_x1 + 20.0;
        self.enemy3_max_x = self.plat6_x2 - 20.0;

        self.enemy4_x = 1925.0;  // 2nd floor - highest
        self.enemy4_y = self.plat7_y - 32.0 - ENEMY_HEIGHT;
        self.enemy4_vx = 0.9;
        self.enemy4_type = ENEMY_GINGERBREAD;
        self.enemy4_facing_right = true;
        self.enemy4_active = true;
        self.enemy4_state = ENEMY_STATE_ALIVE;
        self.enemy4_death_timer = 0;
        self.enemy4_attack_timer = 0;
        self.enemy4_min_x = self.plat7_x1 + 20.0;
        self.enemy4_max_x = self.plat7_x2 - 20.0;

        self.enemy5_x = 2290.0;
        self.enemy5_y = self.plat8_y - 32.0 - ENEMY_HEIGHT;
        self.enemy5_vx = 1.0;
        self.enemy5_type = ENEMY_GRINCH;
        self.enemy5_facing_right = true;
        self.enemy5_active = true;
        self.enemy5_state = ENEMY_STATE_ALIVE;
        self.enemy5_death_timer = 0;
        self.enemy5_attack_timer = 0;
        self.enemy5_min_x = self.plat8_x1 + 20.0;
        self.enemy5_max_x = self.plat8_x2 - 20.0;

        self.enemy6_active = false;
        self.enemy7_active = false;
        self.enemy8_active = false;
        self.enemy9_active = false;
        self.enemy10_active = false;

        // Chimney
        self.chimney_x = 2950.0;
        self.chimney_y = self.plat10_y - 48.0;
    }

    // Level 10: "North Pole Showdown" - Final boss
    pub fn load_level_10(&mut self){ // Level 10: "North Pole Showdown" — Final Boss Arena
    self.platform_count = 10;

    // ───────────────── MAIN GROUND FLOW ─────────────────

    // Start arena
    self.plat1_x1 = 0.0;
    self.plat1_x2 = 380.0;
    self.plat1_y  = 150.0;

    // Bridge
    self.plat2_x1 = 450.0;   // 70px gap
    self.plat2_x2 = 520.0;
    self.plat2_y  = 145.0;

    // Grinch arena
    self.plat3_x1 = 580.0;
    self.plat3_x2 = 920.0;
    self.plat3_y  = 140.0;

    // Bridge
    self.plat4_x1 = 990.0;   // 70px gap
    self.plat4_x2 = 1060.0;
    self.plat4_y  = 145.0;

    // Krampus arena
    self.plat5_x1 = 1120.0;
    self.plat5_x2 = 1480.0;
    self.plat5_y  = 150.0;

    // ───────────────── UPPER DODGE PLATFORMS ─────────────────

    self.plat6_x1 = 120.0;
    self.plat6_x2 = 300.0;
    self.plat6_y  = 110.0;

    self.plat7_x1 = 650.0;
    self.plat7_x2 = 850.0;
    self.plat7_y  = 115.0;

    self.plat8_x1 = 1200.0;
    self.plat8_x2 = 1400.0;
    self.plat8_y  = 115.0;

    // ───────────────── FINAL APPROACH ─────────────────

    self.plat9_x1 = 1540.0;
    self.plat9_x2 = 1700.0;
    self.plat9_y  = 150.0;

    // GOAL PLATFORM (near & rewarding)
    self.plat10_x1 = 1760.0;
    self.plat10_x2 = 1960.0;
    self.plat10_y  = 150.0;

    // ───────────────── GIFTS ─────────────────

    self.gift1_x = 180.0;
    self.gift1_y = 90.0;
    self.gift1_collected = false;

    self.gift2_x = 740.0;
    self.gift2_y = 95.0;
    self.gift2_collected = false;

    self.gift3_x = 820.0;
    self.gift3_y = 65.0;
    self.gift3_collected = false;

    self.gift4_x = 1280.0;
    self.gift4_y = 90.0;
    self.gift4_collected = false;

    self.gift5_x = 1860.0;
    self.gift5_y = 95.0;
    self.gift5_collected = false;

    // ───────────────── ENEMIES — FINAL BOSS GAUNTLET ─────────────────

// Enemy 1 — Grinch minion (Start Arena)
self.enemy1_x = 200.0;
self.enemy1_y = self.plat1_y - 32.0 - ENEMY_HEIGHT;
self.enemy1_vx = 0.4;
self.enemy1_type = ENEMY_GRINCH;
self.enemy1_facing_right = true;
self.enemy1_active = true;
self.enemy1_min_x = self.plat1_x1 + 20.0;
self.enemy1_max_x = self.plat1_x2 - 20.0;

// Enemy 2 — Grinch mid-boss (Grinch Arena)
self.enemy2_x = 740.0;
self.enemy2_y = self.plat3_y - 32.0 - ENEMY_HEIGHT;
self.enemy2_vx = 0.5;
self.enemy2_type = ENEMY_GRINCH;
self.enemy2_facing_right = true;
self.enemy2_active = true;
self.enemy2_min_x = self.plat3_x1 + 20.0;
self.enemy2_max_x = self.plat3_x2 - 20.0;

// Enemy 3 — Krampus final boss (Krampus Arena)
self.enemy3_x = 1280.0;
self.enemy3_y = self.plat5_y - 32.0 - ENEMY_HEIGHT;
self.enemy3_vx = 0.4;
self.enemy3_type = ENEMY_GRINCH; // swap to KRAMPUS when available
self.enemy3_facing_right = true;
self.enemy3_active = true;
self.enemy3_min_x = self.plat5_x1 + 20.0;
self.enemy3_max_x = self.plat5_x2 - 20.0;

// Enemy 4 — Upper dodge Grinch (plat6)
self.enemy4_x = 200.0;
self.enemy4_y = self.plat6_y - 32.0 - ENEMY_HEIGHT;
self.enemy4_vx = 0.3;
self.enemy4_type = ENEMY_GRINCH;
self.enemy4_facing_right = true;
self.enemy4_active = true;
self.enemy4_min_x = self.plat6_x1 + 20.0;
self.enemy4_max_x = self.plat6_x2 - 20.0;

// Enemy 5 — Upper dodge Gingerbread (plat7)
self.enemy5_x = 750.0;
self.enemy5_y = self.plat7_y - 32.0 - ENEMY_HEIGHT;
self.enemy5_vx = 0.3;
self.enemy5_type = ENEMY_GINGERBREAD;
self.enemy5_facing_right = true;
self.enemy5_active = true;
self.enemy5_min_x = self.plat7_x1 + 20.0;
self.enemy5_max_x = self.plat7_x2 - 20.0;

// Enemy 6 — Upper dodge Grinch (plat8)
self.enemy6_x = 1300.0;
self.enemy6_y = self.plat8_y - 32.0 - ENEMY_HEIGHT;
self.enemy6_vx = 0.4;
self.enemy6_type = ENEMY_GRINCH;
self.enemy6_facing_right = true;
self.enemy6_active = true;
self.enemy6_min_x = self.plat8_x1 + 20.0;
self.enemy6_max_x = self.plat8_x2 - 20.0;

// Enemy 7 — Path pressure (plat9)
self.enemy7_x = 1620.0;
self.enemy7_y = self.plat9_y - 32.0 - ENEMY_HEIGHT;
self.enemy7_vx = 0.4;
self.enemy7_type = ENEMY_GINGERBREAD;
self.enemy7_facing_right = true;
self.enemy7_active = true;
self.enemy7_min_x = self.plat9_x1 + 20.0;
self.enemy7_max_x = self.plat9_x2 - 20.0;

// Enemy 8 — Final platform Grinch (plat10)
self.enemy8_x = 1860.0;
self.enemy8_y = self.plat10_y - 32.0 - ENEMY_HEIGHT;
self.enemy8_vx = 0.5;
self.enemy8_type = ENEMY_GRINCH;
self.enemy8_facing_right = true;
self.enemy8_active = true;
self.enemy8_min_x = self.plat10_x1 + 20.0;
self.enemy8_max_x = self.plat10_x2 - 20.0;

// Disable unused slots
self.enemy9_active  = false;
self.enemy10_active = false;

    // ───────────────── GOAL HOUSE ─────────────────

    self.chimney_x = 1860.0;               // Center of plat10
    self.chimney_y = self.plat10_y - 48.0; // Correct vertical alignment
}

    // BOSS FIGHT: Santa vs Giant Krampus!
    pub fn load_boss_fight(&mut self) {
        self.in_boss_fight = true;
        self.level_complete = false;
        self.boss_defeated = false;
        self.current_screen = SCREEN_PLAYING;  // CRITICAL: Must set screen or update never runs!

        // Boss arena - Simple flat platform for epic showdown
        self.platform_count = 3;

        // Main ground platform - wide arena
        self.plat1_x1 = 0.0;
        self.plat1_x2 = 320.0;  // Full screen width
        self.plat1_y = 150.0;

        // Small platform on left for dodging (reduced 32px from left edge)
        self.plat2_x1 = 52.0;   // Was 20.0, moved 32px right
        self.plat2_x2 = 100.0;
        self.plat2_y = 100.0;

        // Small platform on right for dodging (reduced 32px from right edge)
        self.plat3_x1 = 220.0;
        self.plat3_x2 = 268.0;  // Was 300.0, moved 32px left
        self.plat3_y = 100.0;

        // Reset player to left side of arena - CRITICAL INITIALIZATION!
        self.player_x = 50.0;
        self.player_y = 80.0;
        self.player_vx = 0.0;
        self.player_vy = 0.0;
        self.player_state = STATE_IDLE;  // Must be IDLE not DEAD!
        self.player_on_ground = false;  // Will be set by collision detection
        self.player_on_ice = false;
        self.player_facing_right = true;
        self.camera_x = 0.0;  // Fixed camera for boss fight

        // CRITICAL: Reset timers that can block input!
        self.death_timer = 0;  // MUST BE 0 or game freezes in death handler!
        self.coyote_timer = 0;
        self.jump_buffer_timer = 0;
        self.anim_frame = 0;

        // Reset death sound flags
        self.falling_sound_played = false;
        self.death_sound_played = false;

        // Initialize GIANT KRAMPUS BOSS on right side
        self.boss_hp = BOSS_HP_MAX;
        self.boss_x = 200.0;  // Right side of arena (center-right)
        self.boss_y = self.plat1_y - BOSS_SIZE;  // On ground platform (64x64 sprite)
        self.boss_vx = 0.0;
        self.boss_vy = 0.0;
        self.boss_facing_right = false;  // Facing player
        self.boss_attack_timer = BOSS_ATTACK_COOLDOWN;
        self.boss_attack_type = 0;
        self.boss_invincible_timer = 0;
        self.boss_state = BOSS_STATE_IDLE;
        self.boss_state_timer = 0;
        self.boss_anim_timer = 0;
        self.boss_phase = 1;
        self.boss_phase_transition = 0;
        self.boss_attack_pattern = 0;
        self.boss_hitbox_active = false;
        self.boss_warning_active = false;
        self.boss_on_ground = true;

        // Reset platform crumbling state
        self.platform_used_left = false;
        self.platform_used_right = false;
        self.platform_crumbled_left = false;
        self.platform_crumbled_right = false;

        // Reset player stun
        self.player_stunned = false;
        self.player_stun_timer = 0;

        // Reset stomp cooldown (ready to stomp immediately)
        self.player_stomp_cooldown = PLAYER_STOMP_COOLDOWN_FRAMES;

        // Setup player weapons
        self.selected_weapon = WEAPON_GIFT_BOMB;
        self.weapon_cooldown = 0;

        // Clear all projectiles
        self.proj1_active = false;
        self.proj2_active = false;
        self.proj3_active = false;
        self.proj4_active = false;
        self.proj5_active = false;
        self.proj6_active = false;
        self.proj7_active = false;
        self.proj8_active = false;
        self.proj9_active = false;
        self.proj10_active = false;

        // No enemies or gifts in boss fight - deactivate ALL 10 enemies!
        self.enemy1_active = false;
        self.enemy2_active = false;
        self.enemy3_active = false;
        self.enemy4_active = false;
        self.enemy5_active = false;
        self.enemy6_active = false;
        self.enemy7_active = false;
        self.enemy8_active = false;
        self.enemy9_active = false;
        self.enemy10_active = false;

        self.gift1_collected = true;
        self.gift2_collected = true;
        self.gift3_collected = true;
        self.gift4_collected = true;
        self.gift5_collected = true;

        // Clear enemy snowballs
        self.snowball1_active = false;
        self.snowball2_active = false;
        self.snowball3_active = false;
        self.snowball4_active = false;

        // Give Santa 6 lives for the boss fight (regular levels have 3)
        self.lives = 8;

        // Reset game time for boss fight
        self.game_time = 0;
        self.level_time = 0;
        self.level_score = 0;
        self.level_deaths = 0;
    }

}
