use crate::*;

impl GameState {
    pub fn apply_enemy_physics_and_collisions(&mut self, enemy_idx: usize) {
        // Get enemy data based on index
        let (x, y, _vx, vy, active) = match enemy_idx {
            0 => (&mut self.enemy1_x, &mut self.enemy1_y, &mut self.enemy1_vx, &mut self.enemy1_vy, &mut self.enemy1_active),
            1 => (&mut self.enemy2_x, &mut self.enemy2_y, &mut self.enemy2_vx, &mut self.enemy2_vy, &mut self.enemy2_active),
            2 => (&mut self.enemy3_x, &mut self.enemy3_y, &mut self.enemy3_vx, &mut self.enemy3_vy, &mut self.enemy3_active),
            3 => (&mut self.enemy4_x, &mut self.enemy4_y, &mut self.enemy4_vx, &mut self.enemy4_vy, &mut self.enemy4_active),
            4 => (&mut self.enemy5_x, &mut self.enemy5_y, &mut self.enemy5_vx, &mut self.enemy5_vy, &mut self.enemy5_active),
            5 => (&mut self.enemy6_x, &mut self.enemy6_y, &mut self.enemy6_vx, &mut self.enemy6_vy, &mut self.enemy6_active),
            6 => (&mut self.enemy7_x, &mut self.enemy7_y, &mut self.enemy7_vx, &mut self.enemy7_vy, &mut self.enemy7_active),
            7 => (&mut self.enemy8_x, &mut self.enemy8_y, &mut self.enemy8_vx, &mut self.enemy8_vy, &mut self.enemy8_active),
            8 => (&mut self.enemy9_x, &mut self.enemy9_y, &mut self.enemy9_vx, &mut self.enemy9_vy, &mut self.enemy9_active),
            9 => (&mut self.enemy10_x, &mut self.enemy10_y, &mut self.enemy10_vx, &mut self.enemy10_vy, &mut self.enemy10_active),
            _ => return,
        };

        if !*active { return; }

        // Apply gravity
        *vy += GRAVITY;
        if *vy > MAX_FALL_SPEED {
            *vy = MAX_FALL_SPEED;
        }

        // Store previous Y for collision detection
        let prev_y = *y;

        // Update vertical position
        *y += *vy;

        // Platform collision helper (inlined to avoid borrow issues)
        let check_platform = |x: &mut f32, y: &mut f32, vy: &mut f32, x1: f32, x2: f32, platform_y: f32, prev_y: f32| {
            let enemy_feet_y = *y + ENEMY_HEIGHT;
            let prev_feet_y = prev_y + ENEMY_HEIGHT;

            let visual_x1 = ((x1 as i32 / 32) * 32) as f32;
            let last_tile_start = (((x2 - 1.0) as i32 / 32) * 32) as f32;
            let visual_x2 = last_tile_start + 32.0;

            let enemy_left = *x;
            let enemy_right = *x + ENEMY_HEIGHT;

            if enemy_right > visual_x1 - 5.0 && enemy_left < visual_x2 + 5.0 {
                if prev_feet_y <= platform_y && enemy_feet_y >= platform_y && *vy >= 0.0 {
                    *y = platform_y - ENEMY_HEIGHT;
                    *vy = 0.0;
                    true
                } else if enemy_feet_y >= platform_y - 3.0 && enemy_feet_y <= platform_y + 3.0 && *vy >= 0.0 {
                    *y = platform_y - ENEMY_HEIGHT;
                    *vy = 0.0;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        // Check each platform
        if self.platform_count >= 1 { check_platform(x, y, vy, self.plat1_x1, self.plat1_x2, self.plat1_y, prev_y); }
        if self.platform_count >= 2 { check_platform(x, y, vy, self.plat2_x1, self.plat2_x2, self.plat2_y, prev_y); }
        if self.platform_count >= 3 { check_platform(x, y, vy, self.plat3_x1, self.plat3_x2, self.plat3_y, prev_y); }
        if self.platform_count >= 4 { check_platform(x, y, vy, self.plat4_x1, self.plat4_x2, self.plat4_y, prev_y); }
        if self.platform_count >= 5 { check_platform(x, y, vy, self.plat5_x1, self.plat5_x2, self.plat5_y, prev_y); }
        if self.platform_count >= 6 { check_platform(x, y, vy, self.plat6_x1, self.plat6_x2, self.plat6_y, prev_y); }
        if self.platform_count >= 7 { check_platform(x, y, vy, self.plat7_x1, self.plat7_x2, self.plat7_y, prev_y); }
        if self.platform_count >= 8 { check_platform(x, y, vy, self.plat8_x1, self.plat8_x2, self.plat8_y, prev_y); }
        if self.platform_count >= 9 { check_platform(x, y, vy, self.plat9_x1, self.plat9_x2, self.plat9_y, prev_y); }
        if self.platform_count >= 10 { check_platform(x, y, vy, self.plat10_x1, self.plat10_x2, self.plat10_y, prev_y); }

        // Deactivate enemy if it falls off screen
        if *y > 200.0 {
            *active = false;
        }
    }

    pub fn update_enemies(&mut self) {
        // Helper function to update a single enemy
        let update_enemy = |x: &mut f32, vx: &mut f32, enemy_type: u8, facing_right: &mut bool,
                            active: &mut bool, min_x: f32, max_x: f32, state: &mut u8,
                            death_timer: &mut u8, attack_timer: &mut u8, player_x: f32, player_y: f32, enemy_y: f32, camera_x: f32| {
            if !*active {return;}

            // Handle death state
            if *state == ENEMY_STATE_DYING {
                *death_timer -= 1;
                if *death_timer == 0 {
                    *active = false;  // Remove enemy completely after death animation
                }
                return;  // Don't move or attack while dying
            }

            // Decrement attack cooldown
            if *attack_timer > 0 {
                *attack_timer -= 1;
            }

            // Normal movement (only if alive)
            if *state == ENEMY_STATE_ALIVE {
                *x += *vx;
                if *x < min_x || *x > max_x {
                    *vx = -*vx;
                    *facing_right = *vx > 0.0;
                }

                // Attack logic for Grinch - ONLY if visible on screen!
                if enemy_type == ENEMY_GRINCH && *attack_timer == 0 {
                    // Check if Grinch is visible on screen (320px wide screen)
                    let is_on_screen = *x >= camera_x - 32.0 && *x <= camera_x + 320.0 + 32.0;

                    if is_on_screen {
                        let dx = (*x - player_x).abs();
                        let dy = (enemy_y - player_y).abs();

                        // Attack only if player is in range AND Grinch is visible
                        if dx < GRINCH_ATTACK_RANGE && dy < 50.0 {
                            // Face Santa when attacking!
                            *facing_right = player_x > *x;

                            // Player is in range! Trigger attack
                            *attack_timer = GRINCH_ATTACK_COOLDOWN;
                            return;  // Signal to spawn snowball (handled outside)
                        }
                    }
                }

                // Attack logic for Krampus - ONLY if visible on screen!
                if enemy_type == ENEMY_KRAMPUS && *attack_timer == 0 {
                    // Check if Krampus is visible on screen (320px wide screen)
                    let is_on_screen = *x >= camera_x - 32.0 && *x <= camera_x + 320.0 + 32.0;

                    if is_on_screen {
                        let dx = (*x - player_x).abs();
                        let dy = (enemy_y - player_y).abs();

                        // Attack only if player is in range AND Krampus is visible
                        if dx < KRAMPUS_ATTACK_RANGE && dy < 50.0 {
                            // Face Santa when attacking!
                            *facing_right = player_x > *x;

                            // Player is in range! Trigger chain attack
                            *attack_timer = KRAMPUS_ATTACK_COOLDOWN;
                            // Chain attack is visual-only (no projectile), handled in rendering
                        }
                    }
                }
            }
        };

        // Store player position and camera for attack detection
        let px = self.player_x;
        let py = self.player_y;
        let cam_x = self.camera_x;

        // Apply physics first to all enemies
        self.apply_enemy_physics_and_collisions(0);
        self.apply_enemy_physics_and_collisions(1);
        self.apply_enemy_physics_and_collisions(2);
        self.apply_enemy_physics_and_collisions(3);
        self.apply_enemy_physics_and_collisions(4);
        self.apply_enemy_physics_and_collisions(5);
        self.apply_enemy_physics_and_collisions(6);
        self.apply_enemy_physics_and_collisions(7);
        self.apply_enemy_physics_and_collisions(8);
        self.apply_enemy_physics_and_collisions(9);

        // Update enemy 1 AI
        update_enemy(&mut self.enemy1_x, &mut self.enemy1_vx, self.enemy1_type,
                    &mut self.enemy1_facing_right, &mut self.enemy1_active,
                    self.enemy1_min_x, self.enemy1_max_x, &mut self.enemy1_state,
                    &mut self.enemy1_death_timer, &mut self.enemy1_attack_timer, px, py, self.enemy1_y, cam_x);

        // Spawn snowball if Grinch just attacked
        if self.enemy1_type == ENEMY_GRINCH && self.enemy1_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy1_x, self.enemy1_y, px);
        }

        // Update enemy 2
        update_enemy(&mut self.enemy2_x, &mut self.enemy2_vx, self.enemy2_type,
                    &mut self.enemy2_facing_right, &mut self.enemy2_active,
                    self.enemy2_min_x, self.enemy2_max_x, &mut self.enemy2_state,
                    &mut self.enemy2_death_timer, &mut self.enemy2_attack_timer, px, py, self.enemy2_y, cam_x);

        if self.enemy2_type == ENEMY_GRINCH && self.enemy2_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy2_x, self.enemy2_y, px);
        }

        // Update enemy 3
        update_enemy(&mut self.enemy3_x, &mut self.enemy3_vx, self.enemy3_type,
                    &mut self.enemy3_facing_right, &mut self.enemy3_active,
                    self.enemy3_min_x, self.enemy3_max_x, &mut self.enemy3_state,
                    &mut self.enemy3_death_timer, &mut self.enemy3_attack_timer, px, py, self.enemy3_y, cam_x);

        if self.enemy3_type == ENEMY_GRINCH && self.enemy3_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy3_x, self.enemy3_y, px);
        }

        // Update enemy 4
        update_enemy(&mut self.enemy4_x, &mut self.enemy4_vx, self.enemy4_type,
                    &mut self.enemy4_facing_right, &mut self.enemy4_active,
                    self.enemy4_min_x, self.enemy4_max_x, &mut self.enemy4_state,
                    &mut self.enemy4_death_timer, &mut self.enemy4_attack_timer, px, py, self.enemy4_y, cam_x);

        if self.enemy4_type == ENEMY_GRINCH && self.enemy4_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy4_x, self.enemy4_y, px);
        }

        // Update enemy 5
        update_enemy(&mut self.enemy5_x, &mut self.enemy5_vx, self.enemy5_type,
                    &mut self.enemy5_facing_right, &mut self.enemy5_active,
                    self.enemy5_min_x, self.enemy5_max_x, &mut self.enemy5_state,
                    &mut self.enemy5_death_timer, &mut self.enemy5_attack_timer, px, py, self.enemy5_y, cam_x);

        if self.enemy5_type == ENEMY_GRINCH && self.enemy5_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy5_x, self.enemy5_y, px);
        }

        // Update enemy 6
        update_enemy(&mut self.enemy6_x, &mut self.enemy6_vx, self.enemy6_type,
                    &mut self.enemy6_facing_right, &mut self.enemy6_active,
                    self.enemy6_min_x, self.enemy6_max_x, &mut self.enemy6_state,
                    &mut self.enemy6_death_timer, &mut self.enemy6_attack_timer, px, py, self.enemy6_y, cam_x);

        if self.enemy6_type == ENEMY_GRINCH && self.enemy6_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy6_x, self.enemy6_y, px);
        }

        // Update enemy 7
        update_enemy(&mut self.enemy7_x, &mut self.enemy7_vx, self.enemy7_type,
                    &mut self.enemy7_facing_right, &mut self.enemy7_active,
                    self.enemy7_min_x, self.enemy7_max_x, &mut self.enemy7_state,
                    &mut self.enemy7_death_timer, &mut self.enemy7_attack_timer, px, py, self.enemy7_y, cam_x);

        if self.enemy7_type == ENEMY_GRINCH && self.enemy7_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy7_x, self.enemy7_y, px);
        }

        // Update enemy 8
        update_enemy(&mut self.enemy8_x, &mut self.enemy8_vx, self.enemy8_type,
                    &mut self.enemy8_facing_right, &mut self.enemy8_active,
                    self.enemy8_min_x, self.enemy8_max_x, &mut self.enemy8_state,
                    &mut self.enemy8_death_timer, &mut self.enemy8_attack_timer, px, py, self.enemy8_y, cam_x);

        if self.enemy8_type == ENEMY_GRINCH && self.enemy8_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy8_x, self.enemy8_y, px);
        }

        // Update enemy 9
        update_enemy(&mut self.enemy9_x, &mut self.enemy9_vx, self.enemy9_type,
                    &mut self.enemy9_facing_right, &mut self.enemy9_active,
                    self.enemy9_min_x, self.enemy9_max_x, &mut self.enemy9_state,
                    &mut self.enemy9_death_timer, &mut self.enemy9_attack_timer, px, py, self.enemy9_y, cam_x);

        if self.enemy9_type == ENEMY_GRINCH && self.enemy9_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy9_x, self.enemy9_y, px);
        }

        // Update enemy 10
        update_enemy(&mut self.enemy10_x, &mut self.enemy10_vx, self.enemy10_type,
                    &mut self.enemy10_facing_right, &mut self.enemy10_active,
                    self.enemy10_min_x, self.enemy10_max_x, &mut self.enemy10_state,
                    &mut self.enemy10_death_timer, &mut self.enemy10_attack_timer, px, py, self.enemy10_y, cam_x);

        if self.enemy10_type == ENEMY_GRINCH && self.enemy10_attack_timer == GRINCH_ATTACK_COOLDOWN - 1 {
            self.spawn_snowball(self.enemy10_x, self.enemy10_y, px);
        }
    }

    pub fn check_krampus_chain_attacks(&mut self) {
        // Helper to check if a Krampus enemy hits the player with chain during attack
        let check_chain_hit = |enemy_x: f32, enemy_y: f32, enemy_type: u8, enemy_active: bool,
                               enemy_state: u8, attack_timer: u8, facing_right: bool, player_x: f32, player_y: f32| -> bool {
            if !enemy_active || enemy_type != ENEMY_KRAMPUS || enemy_state != ENEMY_STATE_ALIVE {
                return false;
            }

            // Only damage player during active attack frames
            if attack_timer > KRAMPUS_ATTACK_COOLDOWN - 15 && attack_timer < KRAMPUS_ATTACK_COOLDOWN {
                let dx = enemy_x - player_x;
                let dy = (enemy_y - player_y).abs();

                // Chain extends in front of Krampus
                let in_front = if facing_right { dx < 0.0 && dx > -96.0 } else { dx > 0.0 && dx < 96.0 };

                if in_front && dy < 40.0 {
                    return true;  // Hit by chain!
                }
            }

            false
        };

        let px = self.player_x;
        let py = self.player_y;

        // Check all enemies for Krampus chain attacks
        if check_chain_hit(self.enemy1_x, self.enemy1_y, self.enemy1_type, self.enemy1_active,
                          self.enemy1_state, self.enemy1_attack_timer, self.enemy1_facing_right, px, py) {
            self.die();
            return;
        }

        if check_chain_hit(self.enemy2_x, self.enemy2_y, self.enemy2_type, self.enemy2_active,
                          self.enemy2_state, self.enemy2_attack_timer, self.enemy2_facing_right, px, py) {
            self.die();
            return;
        }

        if check_chain_hit(self.enemy3_x, self.enemy3_y, self.enemy3_type, self.enemy3_active,
                          self.enemy3_state, self.enemy3_attack_timer, self.enemy3_facing_right, px, py) {
            self.die();
            return;
        }

        if check_chain_hit(self.enemy4_x, self.enemy4_y, self.enemy4_type, self.enemy4_active,
                          self.enemy4_state, self.enemy4_attack_timer, self.enemy4_facing_right, px, py) {
            self.die();
            return;
        }

        if check_chain_hit(self.enemy5_x, self.enemy5_y, self.enemy5_type, self.enemy5_active,
                          self.enemy5_state, self.enemy5_attack_timer, self.enemy5_facing_right, px, py) {
            self.die();
        }
    }

}
