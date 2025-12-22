use crate::*;

impl GameState {
    /// Boss-only update - integrates with normal level pipeline
    pub fn update_boss(&mut self) {
        // ========================================
        // BOSS DEFEATED CHECK
        // ========================================
        if self.boss_defeated {
            let kb = keyboard::get();
            let gp = gamepad::get(0);
            if kb.space().just_pressed() || gp.start.just_pressed() {
                // Victory! Show game complete screen
                self.game_complete = true;
                self.in_boss_fight = false;
            }
            return;
        }

        // ========================================
        // DEATH STATE CHECK
        // ========================================
        if self.boss_state == BOSS_STATE_DEATH {
            self.update_boss_death();
            return;
        }

        // ========================================
        // BOSS FSM UPDATE
        // ========================================

        // Decrement invincibility timer
        if self.boss_invincible_timer > 0 {
            self.boss_invincible_timer -= 1;
        }

        // Increment player stomp cooldown (counts up to max)
        if self.player_stomp_cooldown < PLAYER_STOMP_COOLDOWN_FRAMES {
            self.player_stomp_cooldown += 1;
        }

        // Check for boss death (HP reaches 0)
        if self.boss_hp == 0 {
            self.enter_boss_death();
            return;
        }

        // Check for phase transitions (visual only)
        if self.boss_hp <= 15 && self.boss_phase == 2 && self.boss_state != BOSS_STATE_PHASE_TRANSITION && !self.phase3_shown {
            self.enter_phase_transition(3);
            return;
        }
        if self.boss_hp <= 35 && self.boss_phase == 1 && self.boss_state != BOSS_STATE_PHASE_TRANSITION && !self.phase2_shown {
            self.enter_phase_transition(2);
            return;
        }

        // Update FSM
        self.update_boss_fsm();

        // Boss physics (gravity, collision)
        self.apply_boss_gravity();
        self.resolve_boss_collisions();

        // Check player stomp on boss (STOMP-ONLY COMBAT)
        self.check_boss_stomp_collision();

        // Check boss melee hit on player (bash attack)
        if self.boss_hitbox_active {
            self.check_boss_melee_hit();
        }

        // Check boss snowball hits on player
        self.check_boss_snowball_hits();

        // Boss animation timer
        self.boss_anim_timer += 1;
    }

    // ========================================
    // BOSS FSM DISPATCHER (SIMPLIFIED)
    // ========================================

    fn update_boss_fsm(&mut self) {
        match self.boss_state {
            BOSS_STATE_IDLE => self.update_boss_idle(),
            BOSS_STATE_WALK => self.update_boss_walk(),
            BOSS_STATE_ATTACK => self.update_boss_attack(),
            BOSS_STATE_ATTACK_SNOWBALL => self.update_boss_snowball_attack(),
            BOSS_STATE_JUMP => self.update_boss_jump(),
            BOSS_STATE_ROAR => self.update_boss_roar(),
            BOSS_STATE_HURT => self.update_boss_hurt(),
            BOSS_STATE_PHASE_TRANSITION => self.update_boss_phase_transition(),
            _ => {},
        }
    }

    // ========================================
    // FSM STATE HANDLERS
    // ========================================

    fn update_boss_idle(&mut self) {
        self.boss_state_timer += 1;

        if self.boss_state_timer >= BOSS_IDLE_DURATION {
            self.transition_to_boss_state(BOSS_STATE_WALK);
        }
    }

    fn update_boss_walk(&mut self) {
        // Face player
        if self.player_x < self.boss_x {
            self.boss_facing_right = false;
        } else {
            self.boss_facing_right = true;
        }

        // Move toward player (constant speed - no phase changes)
        let walk_speed = BOSS_WALK_SPEED_PHASE1;

        if self.boss_facing_right {
            self.boss_vx = walk_speed;
        } else {
            self.boss_vx = -walk_speed;
        }

        // Attack cooldown
        if self.boss_attack_timer > 0 {
            self.boss_attack_timer -= 1;
        }

        // Decide next action based on distance
        if self.boss_attack_timer == 0 {
            let distance = (self.player_x - self.boss_x).abs();

            if distance < BOSS_MELEE_RANGE {
                // Close range: ROAR then BASH
                self.boss_attack_type = 0; // Will do bash after roar
                self.transition_to_boss_state(BOSS_STATE_ROAR);
                self.play_boss_sfx("krampus_chase_03");  // Roar sound
                self.boss_attack_pattern += 1;  // Count this attack
            } else if distance > BOSS_JUMP_RANGE {
                // Far away: ROAR then SNOWBALL
                self.boss_attack_type = 1; // Will do snowball after roar
                self.transition_to_boss_state(BOSS_STATE_ROAR);
                self.play_boss_sfx("krampus_chase_03");  // Roar sound
                self.boss_attack_pattern += 1;  // Count this attack
            } else if self.boss_attack_pattern >= 4 && self.boss_on_ground {
                // After 4 attacks, jump toward player
                self.transition_to_boss_state(BOSS_STATE_JUMP);
                self.boss_attack_pattern = 0;  // Reset counter after jump
            } else if self.player_y < self.boss_y - 30.0 && self.boss_on_ground && self.boss_attack_pattern >= 4 {
                // Player is on upper platform AND 4 attacks done - JUMP toward them!
                self.transition_to_boss_state(BOSS_STATE_JUMP);
                self.boss_attack_pattern = 0;  // Reset counter after jump
            }
        }
    }

    fn update_boss_roar(&mut self) {
        self.boss_state_timer += 1;
        self.boss_vx = 0.0;  // Stop moving during roar (telegraph)

        // End of roar - transition to attack
        if self.boss_state_timer >= BOSS_ROAR_TELEGRAPH {
            self.reset_boss_attack_cooldown();
            
            // Transition to the queued attack type
            if self.boss_attack_type == 0 {
                self.transition_to_boss_state(BOSS_STATE_ATTACK);  // Bash
            } else {
                self.transition_to_boss_state(BOSS_STATE_ATTACK_SNOWBALL);  // Snowball
            }
        }
    }

    fn update_boss_attack(&mut self) {
        self.boss_state_timer += 1;
        self.boss_vx = 0.0;  // Stop moving during attack

        // Step forward slightly during bash (frames 5-15)
        if self.boss_state_timer >= 5 && self.boss_state_timer <= 15 {
            let step_speed = if self.boss_facing_right { 1.5 } else { -1.5 };
            self.boss_vx = step_speed;
        }

        // Hitbox activation frames (10-15)
        if self.boss_state_timer == BOSS_ATTACK_ACTIVE_START {
            self.boss_hitbox_active = true;
        }
        if self.boss_state_timer == BOSS_ATTACK_ACTIVE_END {
            self.boss_hitbox_active = false;
        }

        // End of attack
        if self.boss_state_timer >= BOSS_ATTACK_TOTAL_FRAMES {
            self.reset_boss_attack_cooldown();
            self.transition_to_boss_state(BOSS_STATE_IDLE);
        }
    }

    fn update_boss_snowball_attack(&mut self) {
        self.boss_state_timer += 1;
        self.boss_vx = 0.0;  // Stop moving during attack

        // Throw snowball at frame 15
        if self.boss_state_timer == 15 {
            self.spawn_boss_snowball();
        }

        // End of attack
        if self.boss_state_timer >= BOSS_ATTACK_TOTAL_FRAMES {
            self.reset_boss_attack_cooldown();
            self.transition_to_boss_state(BOSS_STATE_IDLE);
        }
    }

    fn update_boss_jump(&mut self) {
        self.boss_state_timer += 1;

        // Landing check
        if self.boss_on_ground && self.boss_vy == 0.0 && self.boss_state_timer > 10 {
            self.transition_to_boss_state(BOSS_STATE_IDLE);
        }
    }

    fn update_boss_hurt(&mut self) {
        self.boss_state_timer += 1;
        self.boss_vx = 0.0;  // Stop moving when hurt

        if self.boss_state_timer >= BOSS_HURT_DURATION {
            self.transition_to_boss_state(BOSS_STATE_IDLE);
        }
    }

    fn update_boss_phase_transition(&mut self) {
        self.boss_phase_transition += 1;
        self.boss_vx = 0.0;  // Stop moving during transition

        if self.boss_phase_transition >= BOSS_PHASE_TRANSITION_DURATION {
            self.boss_phase_transition = 0;
            
            // Mark phase animation as shown
            if self.boss_phase == 2 {
                self.phase2_shown = true;
            } else if self.boss_phase == 3 {
                self.phase3_shown = true;
            }
            
            self.transition_to_boss_state(BOSS_STATE_IDLE);
        }
    }

    fn update_boss_death(&mut self) {
        self.boss_state_timer += 1;
        self.boss_vx = 0.0;
        self.boss_vy = 0.0;

        // Death animation plays for ~96 frames (~1.6 seconds)
        if self.boss_state_timer >= 96 {
            self.boss_defeated = true;
        }
    }

    // ========================================
    // FSM HELPER FUNCTIONS
    // ========================================

    fn transition_to_boss_state(&mut self, new_state: u8) {
        self.boss_state = new_state;
        self.boss_state_timer = 0;
        self.boss_hitbox_active = false;

        // State entry logic
        match new_state {
            BOSS_STATE_JUMP => {
                self.boss_vy = BOSS_JUMP_VELOCITY_PHASE1;
                self.boss_on_ground = false;
            },
            _ => {},
        }
    }

    fn enter_phase_transition(&mut self, new_phase: u8) {
        self.boss_phase = new_phase;
        self.boss_state = BOSS_STATE_PHASE_TRANSITION;
        self.boss_phase_transition = 1;
        self.boss_state_timer = 0;
        self.boss_vx = 0.0;
    }

    fn enter_boss_death(&mut self) {
        self.boss_state = BOSS_STATE_DEATH;
        self.boss_state_timer = 0;
        self.boss_vx = 0.0;
        self.boss_vy = 0.0;
        self.play_boss_sfx("krampus_death_01");  // Boss death sound
    }

    fn reset_boss_attack_cooldown(&mut self) {
        // Constant cooldown (no phase changes)
        self.boss_attack_timer = BOSS_ATTACK_COOLDOWN_PHASE1;
    }

    // ========================================
    // BOSS PHYSICS
    // ========================================

    fn apply_boss_gravity(&mut self) {
        if !self.boss_on_ground {
            self.boss_vy += GRAVITY;
            if self.boss_vy > MAX_FALL_SPEED {
                self.boss_vy = MAX_FALL_SPEED;
            }
        }

        // Update position
        self.boss_x += self.boss_vx;
        self.boss_y += self.boss_vy;

        // Screen boundaries
        if self.boss_x < 0.0 {
            self.boss_x = 0.0;
            self.boss_vx = 0.0;
        }
        if self.boss_x > 320.0 - BOSS_SIZE {
            self.boss_x = 320.0 - BOSS_SIZE;
            self.boss_vx = 0.0;
        }
    }

    fn resolve_boss_collisions(&mut self) {
        self.boss_on_ground = false;

        // Main ground platform
        if self.boss_y + BOSS_SIZE >= self.plat1_y && self.boss_y + BOSS_SIZE <= self.plat1_y + 10.0 {
            if self.boss_x + BOSS_SIZE > self.plat1_x1 && self.boss_x < self.plat1_x2 {
                if self.boss_vy >= 0.0 {
                    self.boss_y = self.plat1_y - BOSS_SIZE;
                    self.boss_vy = 0.0;
                    self.boss_on_ground = true;
                }
            }
        }

        // Left platform
        if self.boss_y + BOSS_SIZE >= self.plat2_y && self.boss_y + BOSS_SIZE <= self.plat2_y + 10.0 {
            if self.boss_x + BOSS_SIZE > self.plat2_x1 && self.boss_x < self.plat2_x2 {
                if self.boss_vy >= 0.0 {
                    self.boss_y = self.plat2_y - BOSS_SIZE;
                    self.boss_vy = 0.0;
                    self.boss_on_ground = true;
                }
            }
        }

        // Right platform
        if self.boss_y + BOSS_SIZE >= self.plat3_y && self.boss_y + BOSS_SIZE <= self.plat3_y + 10.0 {
            if self.boss_x + BOSS_SIZE > self.plat3_x1 && self.boss_x < self.plat3_x2 {
                if self.boss_vy >= 0.0 {
                    self.boss_y = self.plat3_y - BOSS_SIZE;
                    self.boss_vy = 0.0;
                    self.boss_on_ground = true;
                }
            }
        }

        // INVISIBLE WALLS: Prevent Krampus from walking under upper platforms
        // When on the GROUND platform (boss_y is about 86 when standing on ground at y=150)
        // Upper platforms are at y=100, so boss on ground has y = 150 - 64 = 86
        if self.boss_on_ground && self.boss_y > 80.0 {
            // Left platform ends at x=100, so boss left edge can't go past 100
            if self.boss_x < 100.0 {
                self.boss_x = 100.0;
                self.boss_vx = 0.0;
            }
            // Right platform starts at x=220, so boss right edge (x+64) can't go past 220
            if self.boss_x + BOSS_SIZE > 220.0 {
                self.boss_x = 220.0 - BOSS_SIZE;  // 220 - 64 = 156
                self.boss_vx = 0.0;
            }
        }
    }

    // ========================================
    // STOMP COMBAT (MARIO-STYLE)
    // ========================================

    fn check_boss_stomp_collision(&mut self) {
        // Boss cannot be stomped during these states
        if self.boss_state == BOSS_STATE_ROAR ||
           self.boss_state == BOSS_STATE_PHASE_TRANSITION ||
           self.boss_state == BOSS_STATE_DEATH ||
           self.boss_state == BOSS_STATE_HURT {
            return;
        }

        // Check if boss is invincible (stomp cooldown)
        if self.boss_invincible_timer > 0 {
            return;
        }

        // Player hitbox
        let player_left = self.player_x;
        let player_right = self.player_x + PLAYER_HEIGHT;
        let player_top = self.player_y;
        let player_bottom = self.player_y + PLAYER_HEIGHT;

        // Boss hitbox
        let boss_left = self.boss_x;
        let boss_right = self.boss_x + BOSS_SIZE;
        let boss_top = self.boss_y;
        let boss_head_bottom = self.boss_y + BOSS_STOMP_HEAD_ZONE;  // Top ~20px is head

        // Check X overlap (player center is within boss width)
        let x_overlap = player_right > boss_left && player_left < boss_right;

        if !x_overlap {
            return;
        }

        // Check for STOMP: Player falling and landing on boss head
        // Must be falling, player feet touching boss head, and player center above boss
        let is_falling = self.player_vy > 0.0;
        let player_center_x = self.player_x + 16.0;  // 32px player width / 2
        let boss_center_x = self.boss_x + 32.0;      // 64px boss width / 2
        let player_above_boss = player_center_x > boss_left + 10.0 && player_center_x < boss_right - 10.0;
        
        let stomp_hit = is_falling && 
                        player_above_boss &&
                        player_bottom >= boss_top && 
                        player_bottom <= boss_head_bottom + 5.0 &&   // Tighter tolerance
                        player_top < boss_top + 5.0;  // Player must be mostly above boss

        if stomp_hit {
            // Check if stomp cooldown is ready (300 frames = 5 seconds)
            if self.player_stomp_cooldown >= PLAYER_STOMP_COOLDOWN_FRAMES {
                // STOMP SUCCESS! Cooldown is ready
                if self.boss_hp > BOSS_STOMP_DAMAGE {
                    self.boss_hp -= BOSS_STOMP_DAMAGE;
                } else {
                    self.boss_hp = 0;
                }

                // Reset stomp cooldown (starts counting again)
                self.player_stomp_cooldown = 0;

                // Invincibility after stomp (prevents double-stomp)
                self.boss_invincible_timer = BOSS_STOMP_COOLDOWN;
                
                // Bounce player
                self.player_vy = STOMP_BOUNCE;
                
                // Knockback boss away from player (impact effect)
                let knockback_speed = 4.0;
                if self.player_x < self.boss_x {
                    // Player is on the left, push boss right
                    self.boss_vx = knockback_speed;
                } else {
                    // Player is on the right, push boss left
                    self.boss_vx = -knockback_speed;
                }
                
                // Boss enters hurt state
                self.transition_to_boss_state(BOSS_STATE_HURT);
                
                // Play hurt sound
                self.play_boss_sfx("krampus_hurt");
            } else {
                // STOMP FAILED! Cooldown not ready - Santa dies!
                if self.player_state != STATE_DEAD {
                    self.player_state = STATE_DEAD;
                    self.death_timer = 45;
                    self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
                }
            }
            
            return;
        }

        // Check for SIDE COLLISION: Player takes damage
        // Must have BOTH X and Y overlap to be a valid hit (tighter hitboxes)
        let x_close = (self.player_x - self.boss_x).abs() < 24.0;  // Reduced from 40px to 24px
        let side_hit = x_close && 
                       player_bottom > boss_top + BOSS_STOMP_HEAD_ZONE + 5.0 &&  // More tolerance for stomp zone
                       player_top < boss_top + BOSS_SIZE - 10.0;  // Not touching ground

        if side_hit && self.player_state != STATE_DEAD {
            // Side collision = Santa dies
            self.player_state = STATE_DEAD;
            self.death_timer = 45;
            self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
        }
    }

    fn check_boss_melee_hit(&mut self) {
        // Rectangle collision for bash attack - tighter hitbox
        let dx = (self.player_x - self.boss_x).abs();
        let dy = (self.player_y - self.boss_y).abs();

        // Reduced from 48px to 32px - must be very close to get hit
        if dx < 32.0 && dy < 32.0 {
            if self.player_state != STATE_DEAD {
                self.player_state = STATE_DEAD;
                self.death_timer = 45;
                self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
            }
        }
    }

    // ========================================
    // SNOWBALL PROJECTILE SYSTEM
    // ========================================

    fn spawn_boss_snowball(&mut self) {
        // Find inactive snowball slot
        let speed = if self.boss_facing_right { BOSS_SNOWBALL_SPEED } else { -BOSS_SNOWBALL_SPEED };
        let spawn_x = if self.boss_facing_right { self.boss_x + BOSS_SIZE } else { self.boss_x - 16.0 };
        let spawn_y = self.boss_y + 20.0;  // Mid-height of boss

        if !self.snowball1_active {
            self.snowball1_active = true;
            self.snowball1_x = spawn_x;
            self.snowball1_y = spawn_y;
            self.snowball1_vx = speed;
            self.snowball1_start_x = spawn_x;
        } else if !self.snowball2_active {
            self.snowball2_active = true;
            self.snowball2_x = spawn_x;
            self.snowball2_y = spawn_y;
            self.snowball2_vx = speed;
            self.snowball2_start_x = spawn_x;
        } else if !self.snowball3_active {
            self.snowball3_active = true;
            self.snowball3_x = spawn_x;
            self.snowball3_y = spawn_y;
            self.snowball3_vx = speed;
            self.snowball3_start_x = spawn_x;
        } else if !self.snowball4_active {
            self.snowball4_active = true;
            self.snowball4_x = spawn_x;
            self.snowball4_y = spawn_y;
            self.snowball4_vx = speed;
            self.snowball4_start_x = spawn_x;
        }
    }

    fn check_boss_snowball_hits(&mut self) {
        // Check collision of snowballs with player (tighter hitbox)
        let check_hit = |x: f32, y: f32, active: bool| -> bool {
            if !active { return false; }
            let dx = (self.player_x + 16.0 - x).abs();
            let dy = (self.player_y + 16.0 - y).abs();
            dx < 16.0 && dy < 16.0  // Tighter hitbox - must actually touch
        };

        if check_hit(self.snowball1_x, self.snowball1_y, self.snowball1_active) {
            self.snowball1_active = false;
            if self.player_state != STATE_DEAD {
                self.player_state = STATE_DEAD;
                self.death_timer = 45;
                self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
            }
        }
        if check_hit(self.snowball2_x, self.snowball2_y, self.snowball2_active) {
            self.snowball2_active = false;
            if self.player_state != STATE_DEAD {
                self.player_state = STATE_DEAD;
                self.death_timer = 45;
                self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
            }
        }
        if check_hit(self.snowball3_x, self.snowball3_y, self.snowball3_active) {
            self.snowball3_active = false;
            if self.player_state != STATE_DEAD {
                self.player_state = STATE_DEAD;
                self.death_timer = 45;
                self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
            }
        }
        if check_hit(self.snowball4_x, self.snowball4_y, self.snowball4_active) {
            self.snowball4_active = false;
            if self.player_state != STATE_DEAD {
                self.player_state = STATE_DEAD;
                self.death_timer = 45;
                self.play_boss_sfx("krampus_onkill_01");  // Krampus killed Santa
            }
        }
    }
}
