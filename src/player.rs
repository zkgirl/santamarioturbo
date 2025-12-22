use crate::*;

impl GameState {
    pub fn handle_input(&mut self) {
        // Get keyboard and gamepad input
        let kb = keyboard::get();
        let gp = gamepad::get(0);

        // Check for left/right input (keyboard arrow keys or gamepad d-pad)
        let left_pressed = kb.arrow_left().pressed() || gp.left.pressed();
        let right_pressed = kb.arrow_right().pressed() || gp.right.pressed();

        // Check for jump input (space bar or gamepad A button)
        let jump_just_pressed = kb.space().just_pressed() || gp.a.just_pressed();
        let jump_held = kb.space().pressed() || gp.a.pressed();

        // Update facing direction
        if left_pressed {
            self.player_facing_right = false;
        } else if right_pressed {
            self.player_facing_right = true;
        }

        // Calculate target velocity based on input
        let target_vx = if left_pressed {
            -WALK_SPEED
        } else if right_pressed {
            WALK_SPEED
        } else {
            0.0
        };

        // Apply smooth acceleration/deceleration
        let accel_rate = if self.player_on_ground {
            if target_vx == 0.0 {GROUND_DECEL} else {GROUND_ACCEL}
        } else {
            GROUND_ACCEL * AIR_CONTROL
        };

        if (target_vx - self.player_vx).abs() < accel_rate {
            self.player_vx = target_vx;
        } else if self.player_vx < target_vx {
            self.player_vx += accel_rate;
        } else {
            self.player_vx -= accel_rate;
        }

        // BOSS FIGHT: Stomp-only combat - no weapon switching/firing
        // (All weapon code removed - Santa damages Krampus by stomping only)

        // Jump buffer - store jump input for a few frames
        if jump_just_pressed {
            self.jump_buffer_timer = JUMP_BUFFER;
        }
        if self.jump_buffer_timer > 0 {
            self.jump_buffer_timer -= 1;
        }

        // Execute jump when both buffer and coyote time are active
        if self.jump_buffer_timer > 0 && self.coyote_timer > 0 {
            self.player_vy = JUMP_VELOCITY;
            self.jump_buffer_timer = 0;
            self.coyote_timer = 0;
            self.play_sfx("jump");  // Play jump sound
        }

        // Short hop - release jump early for shorter jump
        if !jump_held && self.player_vy < 0.0 && self.player_vy > JUMP_VELOCITY / 2.0 {
            self.player_vy *= 0.68;
        }
    }

    pub fn update_player(&mut self) {
        // Gravity
        self.player_vy += GRAVITY;
        if self.player_vy > MAX_FALL_SPEED {
            self.player_vy = MAX_FALL_SPEED;
        }

        // Store previous position for collision detection
        let prev_y = self.player_y;

        // Update position
        self.player_x += self.player_vx;
        self.player_y += self.player_vy;

        // Fall death - if player falls below screen
        if self.player_y > 200.0 {
            if !self.falling_sound_played {
                self.play_sfx("falling_down");
                self.falling_sound_played = true;
            }
            self.die();
            return;
        }

        // Platform collision - check all active platforms
        self.player_on_ground = false;

        // Check each platform with previous position
        if self.platform_count >= 1 { self.check_platform_collision(self.plat1_x1, self.plat1_x2, self.plat1_y, prev_y); }
        if self.platform_count >= 2 { self.check_platform_collision(self.plat2_x1, self.plat2_x2, self.plat2_y, prev_y); }
        if self.platform_count >= 3 { self.check_platform_collision(self.plat3_x1, self.plat3_x2, self.plat3_y, prev_y); }
        if self.platform_count >= 4 { self.check_platform_collision(self.plat4_x1, self.plat4_x2, self.plat4_y, prev_y); }
        if self.platform_count >= 5 { self.check_platform_collision(self.plat5_x1, self.plat5_x2, self.plat5_y, prev_y); }
        if self.platform_count >= 6 { self.check_platform_collision(self.plat6_x1, self.plat6_x2, self.plat6_y, prev_y); }
        if self.platform_count >= 7 { self.check_platform_collision(self.plat7_x1, self.plat7_x2, self.plat7_y, prev_y); }
        if self.platform_count >= 8 { self.check_platform_collision(self.plat8_x1, self.plat8_x2, self.plat8_y, prev_y); }
        if self.platform_count >= 9 { self.check_platform_collision(self.plat9_x1, self.plat9_x2, self.plat9_y, prev_y); }
        if self.platform_count >= 10 { self.check_platform_collision(self.plat10_x1, self.plat10_x2, self.plat10_y, prev_y); }

        // Coyote time
        if self.player_on_ground {
            self.coyote_timer = COYOTE_TIME;
        } else if self.coyote_timer > 0 {
            self.coyote_timer -= 1;
        }

        // Animation state
        if self.player_on_ground {
            self.player_state = if self.player_vx.abs() > 0.3 {STATE_RUNNING} else {STATE_IDLE};
        } else {
            self.player_state = if self.player_vy < 0.0 {STATE_JUMPING} else {STATE_FALLING};
        }

        // Bounds - prevent Santa from leaving the visible frame
        if self.player_x < 0.0 { self.player_x = 0.0; }
        
        // Right boundary - for boss fight keep Santa in arena (320px frame, 32px player width)
        if self.in_boss_fight {
            if self.player_x > 288.0 { self.player_x = 288.0; }  // 320 - 32 = 288
        }

        // Check collisions
        self.check_enemy_collisions();
        self.check_gift_collisions();
        self.check_chimney_collision();

        // Fall death
        if self.player_y > 180.0 {
            if !self.falling_sound_played {
                self.play_sfx("falling_down");
                self.falling_sound_played = true;
            }
            self.die();
        }
    }

    pub fn check_platform_collision(&mut self, x1: f32, x2: f32, platform_y: f32, prev_y: f32) {
        // Calculate player's feet position (player_y is the TOP of the sprite)
        let player_feet_y = self.player_y + PLAYER_HEIGHT;
        let prev_feet_y = prev_y + PLAYER_HEIGHT;

        // Use ACTUAL platform bounds
        let platform_left = x1;
        let platform_right = x2;

        // Player sprite is 32 pixels wide
        let player_left = self.player_x;
        let player_right = self.player_x + PLAYER_HEIGHT;

        // Overlap check - ANY part of player must overlap the platform
        if player_right > platform_left && player_left < platform_right {
            // Only check collision when falling (player_vy >= 0)
            if self.player_vy >= 0.0 {
                // Method 1: Swept collision - player crossed the platform from above
                let crossed_platform = prev_feet_y <= platform_y && player_feet_y >= platform_y;
                
                // Method 2: Close proximity - player feet are within landing range of platform
                // This catches cases where the player is at the right height but didn't "cross" in one frame
                let close_to_platform = player_feet_y >= platform_y && player_feet_y <= platform_y + 8.0;
                let was_above = prev_feet_y <= platform_y + 4.0;
                
                if crossed_platform || (close_to_platform && was_above) {
                    // Land on the platform
                    self.player_y = platform_y - PLAYER_HEIGHT;
                    self.player_vy = 0.0;
                    self.player_on_ground = true;
                    self.play_sfx("land");
                }
            }
        }
    }

}
