use crate::*;

impl GameState {
    pub fn update_camera(&mut self) {
        let target_x = self.player_x - 160.0;
        self.camera_x += (target_x - self.camera_x) * 0.1;
        if self.camera_x < 0.0 {self.camera_x = 0.0;}

        // Parallax background scrolling (much slower than camera for better effect)
        self.parallax_offset = self.camera_x * 0.2;
    }

    pub fn check_enemy_collisions(&mut self) {
        let px = self.player_x;
        let py = self.player_y;
        let pvy = self.player_vy;

        // Helper function to check collision with an enemy
        let check_collision = |ex: f32, ey: f32, active: bool| -> (bool, bool) {
            if !active {return (false, false);}
            let overlaps = (px - ex).abs() < 24.0 && (py - ey).abs() < 24.0;
            if overlaps {
                let stomping = py < ey - 10.0 && pvy > 1.0;
                (true, stomping)
            } else {
                (false, false)
            }
        };

        // Enemy 1
        let (hit, stomping) = check_collision(self.enemy1_x, self.enemy1_y, self.enemy1_active && self.enemy1_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                // Start death animation instead of instant disappear
                self.enemy1_state = ENEMY_STATE_DYING;
                self.enemy1_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 2
        let (hit, stomping) = check_collision(self.enemy2_x, self.enemy2_y, self.enemy2_active && self.enemy2_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy2_state = ENEMY_STATE_DYING;
                self.enemy2_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 3
        let (hit, stomping) = check_collision(self.enemy3_x, self.enemy3_y, self.enemy3_active && self.enemy3_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy3_state = ENEMY_STATE_DYING;
                self.enemy3_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 4
        let (hit, stomping) = check_collision(self.enemy4_x, self.enemy4_y, self.enemy4_active && self.enemy4_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy4_state = ENEMY_STATE_DYING;
                self.enemy4_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 5
        let (hit, stomping) = check_collision(self.enemy5_x, self.enemy5_y, self.enemy5_active && self.enemy5_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy5_state = ENEMY_STATE_DYING;
                self.enemy5_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 6
        let (hit, stomping) = check_collision(self.enemy6_x, self.enemy6_y, self.enemy6_active && self.enemy6_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy6_state = ENEMY_STATE_DYING;
                self.enemy6_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 7
        let (hit, stomping) = check_collision(self.enemy7_x, self.enemy7_y, self.enemy7_active && self.enemy7_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy7_state = ENEMY_STATE_DYING;
                self.enemy7_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 8
        let (hit, stomping) = check_collision(self.enemy8_x, self.enemy8_y, self.enemy8_active && self.enemy8_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy8_state = ENEMY_STATE_DYING;
                self.enemy8_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 9
        let (hit, stomping) = check_collision(self.enemy9_x, self.enemy9_y, self.enemy9_active && self.enemy9_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy9_state = ENEMY_STATE_DYING;
                self.enemy9_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }

        // Enemy 10
        let (hit, stomping) = check_collision(self.enemy10_x, self.enemy10_y, self.enemy10_active && self.enemy10_state == ENEMY_STATE_ALIVE);
        if hit {
            if stomping {
                self.enemy10_state = ENEMY_STATE_DYING;
                self.enemy10_death_timer = ENEMY_DEATH_TIMER;
                self.player_vy = STOMP_BOUNCE;
                self.level_score += SCORE_ENEMY;
                self.play_sfx("stompenemy");
            } else {
                self.die();
                return;
            }
        }
    }

    pub fn check_gift_collisions(&mut self) {
        // Check if player ACTUALLY touches the gift (strict collision - must overlap, not just pass nearby)
        let check_gift_touch = |x: f32, y: f32, collected: bool| -> bool {
            if collected {return false;}  // Already collected
            let dx = (self.player_x - x).abs();
            let dy = (self.player_y - y).abs();
            // Strict collision: player sprite is 32x32, gift is 32x32
            // They must overlap significantly - reduced from 28 to 18 pixels
            dx < 18.0 && dy < 18.0
        };

        if check_gift_touch(self.gift1_x, self.gift1_y, self.gift1_collected) {
            self.gift1_collected = true;
            self.level_score += SCORE_GIFT;
            self.play_sfx("gift_collect");
        }
        if check_gift_touch(self.gift2_x, self.gift2_y, self.gift2_collected) {
            self.gift2_collected = true;
            self.level_score += SCORE_GIFT;
            self.play_sfx("gift_collect");
        }
        if check_gift_touch(self.gift3_x, self.gift3_y, self.gift3_collected) {
            self.gift3_collected = true;
            self.level_score += SCORE_GIFT;
            self.play_sfx("gift_collect");
        }
        if check_gift_touch(self.gift4_x, self.gift4_y, self.gift4_collected) {
            self.gift4_collected = true;
            self.level_score += SCORE_GIFT;
            self.play_sfx("gift_collect");
        }
        if check_gift_touch(self.gift5_x, self.gift5_y, self.gift5_collected) {
            self.gift5_collected = true;
            self.level_score += SCORE_GIFT;
            self.play_sfx("gift_collect");
        }

        // Check if all gifts are collected and play special sound
        if self.gift1_collected && self.gift2_collected && self.gift3_collected &&
           self.gift4_collected && self.gift5_collected {
            // Only play the "all collected" sound once
            // We'll use a flag to prevent it from playing every frame
            if !self.all_gifts_sound_played {
                self.play_sfx("all_gift_collected");
                self.all_gifts_sound_played = true;
            }
        }
    }

    pub fn check_chimney_collision(&mut self) {
        // Check if player reaches the chimney (goal)
        let dx = (self.player_x - self.chimney_x).abs();
        let dy = (self.player_y - self.chimney_y).abs();

        if dx < 32.0 && dy < 32.0 && !self.level_complete {
            self.level_complete = true;
            self.play_sfx("chimney_goal");

            // === ARCADE MODE: Skip everything, just load next level ===
            if self.arcade_mode {
                self.lives = 3;  // Always reset to 3 hearts
                if self.current_level < 10 {
                    self.load_level(self.current_level + 1);
                } else {
                    // Loop back to level 1
                    self.load_level(1);
                }
                return;  // Skip all scoring/screens
            }

            // === NORMAL MODE: Calculate scores and show screens ===
            // Count gifts collected in this level
            let gifts_collected = [
                self.gift1_collected,
                self.gift2_collected,
                self.gift3_collected,
                self.gift4_collected,
                self.gift5_collected,
            ].iter().filter(|&&g| g).count() as u8;

            self.level_gifts_collected[(self.current_level - 1) as usize] = gifts_collected;

            // === CALCULATE BONUSES ===
            // Base completion bonus
            self.level_score += SCORE_LEVEL_COMPLETE;

            // Perfect gifts bonus (all 5 gifts collected)
            if gifts_collected == 5 {
                self.level_score += SCORE_PERFECT_GIFTS;
            }

            // No-death bonus
            if self.level_deaths == 0 {
                self.level_score += SCORE_NO_DEATH;
            }

            // Time bonus (like Super Mario Bros: bonus for completing under par time)
            let level_idx = (self.current_level - 1) as usize;
            let par_time_frames = LEVEL_PAR_TIMES[level_idx] * 60; // Convert seconds to frames
            let time_seconds = self.level_time / 60;

            if self.level_time < par_time_frames {
                let time_remaining = (par_time_frames - self.level_time) / 60;
                self.level_score += time_remaining * SCORE_TIME_BONUS;
            }

            // Calculate rank for this level
            let rank = self.calculate_rank(gifts_collected, self.level_deaths, time_seconds, par_time_frames / 60);
            self.level_rank[level_idx] = rank;

            // Update personal bests
            if self.level_score > self.level_best_score[level_idx] {
                self.level_best_score[level_idx] = self.level_score;
            }
            if self.level_time < self.level_best_time[level_idx] {
                self.level_best_time[level_idx] = self.level_time;
            }

            // Add level score to total score
            self.total_score += self.level_score;

            // Unlock next level
            if self.current_level < 10 {
                self.level_unlocked[self.current_level as usize] = true;
            }

            // Advance to next level with 3 fresh lives
            self.lives = 3;

            if self.current_level < 10 {
                // Show level complete screen, then go to level select
                self.current_screen = SCREEN_LEVEL_COMPLETE;
                self.level_complete_timer = 0;
            } else {
                // Completed level 10! Show boss transition screen!
                self.current_screen = SCREEN_BOSS_TRANSITION;
                self.boss_transition_timer = 0;
            }
        }
    }

    pub fn calculate_rank(&self, gifts: u8, deaths: u8, time: u32, par_time: u32) -> u8 {
        // S Rank: Perfect performance (all gifts, no deaths, beat par time)
        if gifts == 5 && deaths == 0 && time <= par_time {
            return RANK_S;
        }

        // A Rank: Excellent (all gifts OR (no deaths AND good time))
        if gifts == 5 || (deaths == 0 && time <= par_time + 20) {
            return RANK_A;
        }

        // B Rank: Good (4+ gifts OR (1-2 deaths AND decent time))
        if gifts >= 4 || (deaths <= 2 && time <= par_time + 40) {
            return RANK_B;
        }

        // C Rank: Average (completed with 2+ gifts OR 3-5 deaths)
        if gifts >= 2 || deaths <= 5 {
            return RANK_C;
        }

        // D Rank: Completed but poor performance
        RANK_D
    }

    pub fn die(&mut self) {
        if !self.death_sound_played {
            self.play_sfx("dead");
            self.death_sound_played = true;
        }
        self.player_state = STATE_DEAD;
        self.death_timer = 45;  // 0.75 seconds - faster respawn!
        self.level_deaths += 1;  // Track deaths for scoring
    }

    pub fn handle_death(&mut self) {
        self.death_timer -= 1;

        // Auto-respawn after death animation
        if self.death_timer == 0 {
            self.lives -= 1;

            if self.lives == 0 {
                // All lives lost - return to level select screen
                self.lives = 3;  // Reset to 3 lives for next attempt
                self.in_boss_fight = false;  // Exit boss fight
                self.current_screen = SCREEN_LEVEL_SELECT;
            } else if self.in_boss_fight {
                // BOSS FIGHT: Respawn on OPPOSITE side of arena from Krampus
                // Arena is 320px wide, so spawn on left if boss is right, and vice versa
                if self.boss_x > 160.0 {
                    // Boss is on right side, spawn Santa on left
                    self.player_x = 30.0;
                } else {
                    // Boss is on left side, spawn Santa on right
                    self.player_x = 260.0;
                }
                self.player_y = 80.0;  // Above ground platform
                self.player_vx = 0.0;
                self.player_vy = 0.0;
                self.player_state = STATE_IDLE;
                self.player_on_ground = false;
                self.death_sound_played = false;
                self.falling_sound_played = false;
                
                // Reset stomp cooldown so Santa can stomp immediately after respawn
                self.player_stomp_cooldown = PLAYER_STOMP_COOLDOWN_FRAMES;
                
                // Clear boss snowballs to give player a chance
                self.snowball1_active = false;
                self.snowball2_active = false;
                self.snowball3_active = false;
                self.snowball4_active = false;
            } else {
                // Normal level: respawn at start of current level
                let current = self.current_level;
                self.load_level(current);
            }
        }
    }

}
