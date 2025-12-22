use crate::*;

impl GameState {
    pub fn render_boss_fight(&mut self) {
        // === PHASE-BASED ARENA BACKGROUND ===
        match self.boss_phase {
            3 => sprite!("backgrounds/boss_arena_bg_phase3", x = 0, y = 0),
            2 => sprite!("backgrounds/boss_arena_bg_phase2", x = 0, y = 0),
            _ => sprite!("backgrounds/boss_arena_bg_phase1", x = 0, y = 0),
        }

        // === PHASE INDICATOR ===
        if self.boss_phase == 2 && self.boss_phase_transition > 0 {
            if self.anim_frame % 20 < 10 {
                text!("!! PHASE 2 !!", x = 115, y = 40, color = 0xFF8800FF);
            }
        } else if self.boss_phase == 3 && self.boss_phase_transition > 0 {
            if self.anim_frame % 15 < 8 {
                text!("!! FINAL PHASE !!", x = 100, y = 40, color = 0xFF0000FF);
            }
        }

        // === ARENA PLATFORMS (using snowtile sprites) ===
        // Main ground platform - full width with snow tiles
        let mut x = (self.plat1_x1 as i32 / 32) * 32;
        while x < self.plat1_x2 as i32 {
            let tile_idx = ((x / 32) % 2) as i32;
            if tile_idx == 0 {
                sprite!("snowtile0", x = x, y = self.plat1_y as i32);
            } else {
                sprite!("snowtile1", x = x, y = self.plat1_y as i32);
            }
            x += 32;
        }

        // Left dodge platform (can crumble in Phase 3)
        if !self.platform_crumbled_left {
            let mut x = (self.plat2_x1 as i32 / 32) * 32;
            while x < self.plat2_x2 as i32 {
                let tile_idx = ((x / 32) % 2) as i32;
                // Show cracked texture if used once
                if self.platform_used_left && self.boss_phase == 3 {
                    // Flash red to show danger
                    if self.anim_frame % 20 < 10 {
                        sprite!("snowtile1", x = x, y = self.plat2_y as i32);
                        rect!(x = x, y = self.plat2_y as i32, w = 32, h = 32, color = 0xFF000044);
                    } else {
                        sprite!("snowtile0", x = x, y = self.plat2_y as i32);
                    }
                } else {
                    if tile_idx == 0 {
                        sprite!("snowtile0", x = x, y = self.plat2_y as i32);
                    } else {
                        sprite!("snowtile1", x = x, y = self.plat2_y as i32);
                    }
                }
                x += 32;
            }
        }

        // Right dodge platform (can crumble in Phase 3)
        if !self.platform_crumbled_right {
            let mut x = (self.plat3_x1 as i32 / 32) * 32;
            while x < self.plat3_x2 as i32 {
                let tile_idx = ((x / 32) % 2) as i32;
                // Show cracked texture if used once
                if self.platform_used_right && self.boss_phase == 3 {
                    // Flash red to show danger
                    if self.anim_frame % 20 < 10 {
                        sprite!("snowtile1", x = x, y = self.plat3_y as i32);
                        rect!(x = x, y = self.plat3_y as i32, w = 32, h = 32, color = 0xFF000044);
                    } else {
                        sprite!("snowtile0", x = x, y = self.plat3_y as i32);
                    }
                } else {
                    if tile_idx == 0 {
                        sprite!("snowtile0", x = x, y = self.plat3_y as i32);
                    } else {
                        sprite!("snowtile1", x = x, y = self.plat3_y as i32);
                    }
                }
                x += 32;
            }
        }

        // === GIANT KRAMPUS BOSS (64x64) ===
        if self.boss_defeated {
            // Show death animation when boss is defeated (3 frames)
            let bx = self.boss_x as i32;
            let by = self.boss_y as i32;
            let flip = !self.boss_facing_right;

            let death_frame = match (self.anim_frame / 10) % 3 {
                0 => "boss/krampus_boss_death_0",
                1 => "boss/krampus_boss_death_1",
                _ => "boss/krampus_boss_death_2",
            };
            sprite!(death_frame, x = bx, y = by, flip_x = flip);
        } else {
            let bx = self.boss_x as i32;
            let by = self.boss_y as i32;
            let flip = !self.boss_facing_right;

            // Select sprite based on boss state and animation
            let boss_sprite = match self.boss_state {
                BOSS_STATE_IDLE => {
                    // Idle animation: 2 frames
                    if (self.boss_anim_timer / 20) % 2 == 0 {
                        "boss/krampus_boss_idle0"
                    } else {
                        "boss/krampus_boss_idle1"
                    }
                },
                BOSS_STATE_WALK => {
                    // Walk animation: 3 frames
                    match (self.boss_anim_timer / 12) % 3 {
                        0 => "boss/boss_walk1",
                        1 => "boss/boss_walk2",
                        _ => "boss/boss_walk3",
                    }
                },
                BOSS_STATE_ATTACK => {
                    // Attack animation: 3 frames
                    match (self.boss_anim_timer / 10) % 3 {
                        0 => "boss/krampus_boss_attack01",
                        1 => "boss/krampus_boss_attack1",
                        _ => "boss/krampus_boss_attack2",
                    }
                },
                BOSS_STATE_JUMP => {
                    "boss/krampus_boss_jump"
                },
                BOSS_STATE_ROAR => {
                    // Roar animation: 2 frames
                    if (self.boss_anim_timer / 15) % 2 == 0 {
                        "boss/boss_roar_0"
                    } else {
                        "boss/boss_roar_1"
                    }
                },
                BOSS_STATE_HURT => {
                    // Hurt animation: 2 frames
                    if (self.boss_anim_timer / 8) % 2 == 0 {
                        "boss/krampus_boss_hurt0"
                    } else {
                        "boss/krampus_boss_hurt1"
                    }
                },
                _ => "boss/krampus_boss_idle0",
            };

            // Phase transitions show transformation sprites
            let final_sprite = if self.boss_phase_transition > 0 {
                match self.boss_phase {
                    2 => {
                        // Phase 2 transformation: 3 frames
                        match (60 - self.boss_phase_transition) / 20 {
                            0 => "boss/krampus_boss_phase2_0",
                            1 => "boss/krampus_boss_phase2_1",
                            _ => "boss/krampus_boss_phase2_2",
                        }
                    },
                    3 => {
                        // Phase 3 transformation: 3 frames
                        match (60 - self.boss_phase_transition) / 20 {
                            0 => "boss/krampus_boss_phase3_0",
                            1 => "boss/krampus_boss_phase3_1",
                            _ => "boss/krampus_boss_phase3_2",
                        }
                    },
                    _ => boss_sprite,
                }
            } else if self.boss_phase == 3 {
                // Phase 3: Use phase3 sprites for idle/walk, faster animations
                match self.boss_state {
                    BOSS_STATE_IDLE | BOSS_STATE_WALK => {
                        match (self.boss_anim_timer / 10) % 3 {
                            0 => "boss/krampus_boss_phase3_0",
                            1 => "boss/krampus_boss_phase3_1",
                            _ => "boss/krampus_boss_phase3_2",
                        }
                    },
                    _ => boss_sprite,
                }
            } else if self.boss_phase == 2 {
                // Phase 2: Use phase2 sprites for idle/walk
                match self.boss_state {
                    BOSS_STATE_IDLE | BOSS_STATE_WALK => {
                        match (self.boss_anim_timer / 12) % 3 {
                            0 => "boss/krampus_boss_phase2_0",
                            1 => "boss/krampus_boss_phase2_1",
                            _ => "boss/krampus_boss_phase2_2",
                        }
                    },
                    _ => boss_sprite,
                }
            } else {
                boss_sprite
            };

            // No damage overlay - just show hurt animation
            // (damage feedback is through HURT state animation only)
            sprite!(final_sprite, x = bx, y = by, flip_x = flip);

            // Show roar indicator when boss is telegraphing
            if self.boss_state == BOSS_STATE_ROAR {
                if self.anim_frame % 10 < 5 {
                    text!("!!", x = bx + 24, y = by - 16, color = 0xFF0000FF);
                }
            }
        }

        // (Removed stun effect - boss roar is telegraph only)
        if false {
            let px = self.player_x as i32;
            let py = self.player_y as i32;
            // Draw spinning stars above player's head
            if self.anim_frame % 15 < 8 {
                text!("*", x = px + 8, y = py - 12, color = 0xFFFF00FF);
                text!("*", x = px + 20, y = py - 8, color = 0xFFFF00FF);
            } else {
                text!("*", x = px + 20, y = py - 12, color = 0xFFFF00FF);
                text!("*", x = px + 8, y = py - 8, color = 0xFFFF00FF);
            }
        }

        // Draw Santa (player) with standard animations (stomp-only combat)
        if self.player_state != STATE_DEAD {
            let px = self.player_x as i32;
            let py = self.player_y as i32;
            let flip = !self.player_facing_right;

            // Standard Santa animations (no weapons)
            match self.player_state {
                STATE_IDLE => {
                    let frame = ((self.anim_frame / 20) % 2) as i32;
                    match frame {
                        0 => sprite!("santa bossfight/santaidle0", x = px, y = py, flip_x = flip),
                        _ => sprite!("santa bossfight/santaidle1", x = px, y = py, flip_x = flip),
                    }
                }
                STATE_RUNNING => {
                    let frame = ((self.anim_frame / 8) % 4) as i32;
                    match frame {
                        0 => sprite!("santa bossfight/santarun0", x = px, y = py, flip_x = flip),
                        1 => sprite!("santa bossfight/santarun1", x = px, y = py, flip_x = flip),
                        2 => sprite!("santa bossfight/santarun2", x = px, y = py, flip_x = flip),
                        _ => sprite!("santa bossfight/santarun3", x = px, y = py, flip_x = flip),
                    }
                }
                STATE_JUMPING => sprite!("santa bossfight/santajump32x32", x = px, y = py, flip_x = flip),
                STATE_FALLING => sprite!("santa bossfight/santafall32x32", x = px, y = py, flip_x = flip),
                _ => sprite!("santa bossfight/santaidle0", x = px, y = py, flip_x = flip),
            }
        } else {
            // Death sprite
            let px = self.player_x as i32;
            let py = self.player_y as i32;
            let flip = !self.player_facing_right;
            sprite!("santa bossfight/santadeath", x = px, y = py, flip_x = flip);
        }

        // Draw BOSS snowball projectiles (not player projectiles - stomp only combat)
        if self.snowball1_active {
            sprite!("snawball", x = self.snowball1_x as i32, y = self.snowball1_y as i32);
        }
        if self.snowball2_active {
            sprite!("snawball", x = self.snowball2_x as i32, y = self.snowball2_y as i32);
        }
        if self.snowball3_active {
            sprite!("snawball", x = self.snowball3_x as i32, y = self.snowball3_y as i32);
        }
        if self.snowball4_active {
            sprite!("snawball", x = self.snowball4_x as i32, y = self.snowball4_y as i32);
        }

        // === BOSS HP BAR (top of screen) ===
        text!("BOSS:", x = 5, y = 5);
        text!("KRAMPUS", x = 40, y = 5);

        // HP bar border
        rect!(x = 78, y = 3, w = 204, h = 16, color = 0x000000ff);

        // HP bar background
        rect!(x = 80, y = 5, w = 200, h = 12, color = 0x333333ff);

        // HP bar color based on health
        let hp_percent = self.boss_hp as f32 / BOSS_HP_MAX as f32;
        let hp_width = (hp_percent * 200.0) as u32;
        let hp_color = if hp_percent > 0.6 {
            0xFF0000ff  // Red (healthy)
        } else if hp_percent > 0.3 {
            0xFF8800ff  // Orange (wounded)
        } else {
            0xFFFF00ff  // Yellow (critical)
        };
        rect!(x = 80, y = 5, w = hp_width, h = 12, color = hp_color);

        // HP text with percentage
        let hp_text = format!("{}/{}",  self.boss_hp, BOSS_HP_MAX);
        text!(&hp_text, x = 285, y = 5);
        let percent_text = format!("{}%", (hp_percent * 100.0) as u32);
        text!(&percent_text, x = 148, y = 5);

        // === STOMP COOLDOWN TIMER (left side) ===
        rect!(x = 5, y = 25, w = 130, h = 20, color = 0x00000088);
        
        if self.player_stomp_cooldown >= PLAYER_STOMP_COOLDOWN_FRAMES {
            // Ready to stomp!
            text!("STOMP: READY!", x = 10, y = 30, color = 0x00FF00FF);
        } else {
            // Show countdown (convert frames to seconds with 1 decimal place)
            let remaining_frames = PLAYER_STOMP_COOLDOWN_FRAMES - self.player_stomp_cooldown;
            let remaining_seconds = remaining_frames as f32 / 60.0;
            let stomp_text = format!("STOMP: {:.1}s", remaining_seconds);
            text!(&stomp_text, x = 10, y = 30, color = 0xFF6666FF);
        }

        // === SCORE AND LIVES (bottom) ===
        rect!(x = 5, y = 160, w = 310, h = 15, color = 0x00000088);
        let score_text = format!("SCORE: {}", self.total_score);
        text!(&score_text, x = 10, y = 163);
        let lives_text = format!("LIVES: {}", self.lives);
        text!(&lives_text, x = 200, y = 163);

        // Lives hearts visual
        for i in 0..self.lives {
            sprite!("lifeheart", x = 255 + (i as i32 * 16), y = 161, w = 12, h = 12);
        }

        // === CONTROLS TUTORIAL (fades out) ===
        if self.game_time < 240 {  // Show for 4 seconds
            rect!(x = 60, y = 115, w = 200, h = 25, color = 0x00000088);
            text!("ARROWS: Move/Jump", x = 70, y = 118);
            text!("JUMP ON HEAD TO STOMP!", x = 70, y = 130, color = 0x00FF00FF);
        }

        // === BOSS DEFEATED VICTORY SCREEN ===
        if self.boss_defeated {
            // Draw boss arena background instead of solid overlay
            sprite!("backgrounds/boss_arena_bg_phase1", x = 0, y = 0);
            // Semi-transparent overlay for text readability
            rect!(x = 0, y = 0, w = 320, h = 180, color = 0x00000088);

            // Victory box
            rect!(x = 30, y = 40, w = 260, h = 100, color = 0x00000000);
            rect!(x = 32, y = 42, w = 256, h = 96, color = 0x1a1a1aff);

            // Animated victory text
            if self.anim_frame % 20 < 10 {
                text!("=== BOSS DEFEATED ===", x = 55, y = 55);
            } else {
                text!("*** BOSS DEFEATED ***", x = 55, y = 55);
            }

            // Stats
            let final_score = format!("Total Score: {}", self.total_score);
            text!(&final_score, x = 75, y = 75);
            text!("You saved Christmas!", x = 65, y = 90);

            // Continue prompt (blinking)
            if self.anim_frame % 30 < 20 {
                text!("Press SPACE to Continue", x = 55, y = 115);
            }
        }
    }

    pub fn render_boss_transition(&mut self) {
        // Clean, centered boss transition screen
        // Dark dramatic background
        clear(0x0a0a0aff);  // Almost black for dramatic effect

        // Display the boss warning image centered (160x50px sprite)
        sprite!("levelpage/krampusbosslevel", x = 0, y = 0);

        // Dramatic text overlay with pulsing effect at the top
        let pulse = (self.boss_transition_timer as f32 * 0.1).sin().abs();
        let alpha = (128.0 + pulse * 127.0) as u32;
        let color = (alpha << 24) | 0xFFFFFF;

        text!("FINAL BOSS APPROACHING...", x = 60, y = 30, color = color);

        // Instructions at bottom
        if self.boss_transition_timer > 60 {
            text!("Press SPACE to continue", x = 75, y = 165, color = 0xFFFFFFFF);
        }
    }

    pub fn render(&mut self) {
        // BOSS FIGHT RENDERING
        if self.in_boss_fight {
            self.render_boss_fight();
            return;
        }

        // BOSS TRANSITION SCREEN
        if self.current_screen == SCREEN_BOSS_TRANSITION {
            self.render_boss_transition();
            return;
        }

        // LEVEL COMPLETE STATS SCREEN
        if self.current_screen == SCREEN_LEVEL_COMPLETE {
            self.render_level_complete_stats();
            return;
        }

        // Game Complete Victory Screen
        if self.game_complete {
            // Draw boss arena background instead of solid color
            sprite!("backgrounds/boss_arena_bg_phase1", x = 0, y = 0);
            // Semi-transparent overlay for text readability
            rect!(x = 0, y = 0, w = 320, h = 180, color = 0x00000088);

            // Victory message
            text!("=== CONGRATULATIONS! ===", x = 50, y = 30, color = 0xffff00ff);
            text!("You completed all 10 levels!", x = 50, y = 55, color = 0xffffffff);

            // Score display
            text!("Total Score: {}", self.total_score; x = 80, y = 75, color = 0x00ff00ff);

            // Options
            text!("Choose your adventure:", x = 60, y = 100, color = 0xffffffaa);
            text!("Press SPACE: Restart from Level 1", x = 20, y = 120, color = 0xffaa00ff);
            text!("Press DOWN: Start ARCADE MODE", x = 25, y = 140, color = 0xff00ffff);
            text!("(Endless loop through all levels!)", x = 30, y = 155, color = 0xaaaaaaff);

            return;  // Don't draw game elements
        }

        // Parallax background - scrolls slower than camera and loops seamlessly
        let bg_x = -(self.parallax_offset as i32) % 320;
        sprite!("background320x180", x = bg_x, y = 0);
        sprite!("background320x180", x = bg_x + 320, y = 0);
        sprite!("background320x180", x = bg_x - 320, y = 0);

        let cam_x = self.camera_x as i32;

        // Draw platforms based on level data
        if self.platform_count >= 1 { self.draw_platform(self.plat1_x1 as i32, self.plat1_x2 as i32, self.plat1_y as i32, cam_x); }
        if self.platform_count >= 2 { self.draw_platform(self.plat2_x1 as i32, self.plat2_x2 as i32, self.plat2_y as i32, cam_x); }
        if self.platform_count >= 3 { self.draw_platform(self.plat3_x1 as i32, self.plat3_x2 as i32, self.plat3_y as i32, cam_x); }
        if self.platform_count >= 4 { self.draw_platform(self.plat4_x1 as i32, self.plat4_x2 as i32, self.plat4_y as i32, cam_x); }
        if self.platform_count >= 5 { self.draw_platform(self.plat5_x1 as i32, self.plat5_x2 as i32, self.plat5_y as i32, cam_x); }
        if self.platform_count >= 6 { self.draw_platform(self.plat6_x1 as i32, self.plat6_x2 as i32, self.plat6_y as i32, cam_x); }
        if self.platform_count >= 7 { self.draw_platform(self.plat7_x1 as i32, self.plat7_x2 as i32, self.plat7_y as i32, cam_x); }
        if self.platform_count >= 8 { self.draw_platform(self.plat8_x1 as i32, self.plat8_x2 as i32, self.plat8_y as i32, cam_x); }
        if self.platform_count >= 9 { self.draw_platform(self.plat9_x1 as i32, self.plat9_x2 as i32, self.plat9_y as i32, cam_x); }
        if self.platform_count >= 10 { self.draw_platform(self.plat10_x1 as i32, self.plat10_x2 as i32, self.plat10_y as i32, cam_x); }

        // Chimney (goal)
        self.draw_chimney(cam_x);

        // Gifts
        self.draw_gifts(cam_x);

        // Enemies
        self.draw_enemies(cam_x);

        // Player
        self.draw_player(cam_x);

        // HUD
        self.draw_hud();
    }

    pub fn draw_platform(&self, x1: i32, x2: i32, y: i32, cam_x: i32) {
        let mut x = (x1 / 32) * 32;
        while x < x2 {
            let screen_x = x - cam_x;
            let tile_idx = ((x / 32) % 2) as i32;
            if tile_idx == 0 {
                sprite!("snowtile0", x = screen_x, y = y);
            } else {
                sprite!("snowtile1", x = screen_x, y = y);
            }
            x += 32;
        }
    }

    pub fn draw_chimney(&self, cam_x: i32) {
        let cx = self.chimney_x as i32 - cam_x;
        let cy = self.chimney_y as i32;

        // Draw goalhouse sprite
        sprite!("goalhouse", x = cx, y = cy);
    }

    pub fn draw_player(&self, cam_x: i32) {
        let x = self.player_x as i32 - cam_x;
        let y = self.player_y as i32;
        let flip = !self.player_facing_right;  // Flip when facing left

        if self.player_state == STATE_DEAD {
            sprite!("santadeath", x = x, y = y, flip_x = flip);
            return;
        }

        match self.player_state {
            STATE_IDLE => {
                let frame = ((self.anim_frame / 20) % 2) as i32;
                match frame {
                    0 => sprite!("santaidle0", x = x, y = y, flip_x = flip),
                    _ => sprite!("santaidle1", x = x, y = y, flip_x = flip),
                }
            }
            STATE_RUNNING => {
                let frame = ((self.anim_frame / 8) % 4) as i32;
                match frame {
                    0 => sprite!("santarun0", x = x, y = y, flip_x = flip),
                    1 => sprite!("santarun1", x = x, y = y, flip_x = flip),
                    2 => sprite!("santarun2", x = x, y = y, flip_x = flip),
                    _ => sprite!("santarun3", x = x, y = y, flip_x = flip),
                }
            }
            STATE_JUMPING => sprite!("santajump32x32", x = x, y = y, flip_x = flip),
            STATE_FALLING => sprite!("santafall32x32", x = x, y = y, flip_x = flip),
            _ => {}
        }
    }

    pub fn draw_enemies(&self, cam_x: i32) {
        // Draw enemy based on type and state
        let draw_enemy = |x: f32, y: f32, enemy_type: u8, facing_right: bool, active: bool,
                         state: u8, attack_timer: u8, anim_frame: u32| {
            if !active {return;}

            let ex = x as i32 - cam_x;
            let ey = y as i32;
            let flip = !facing_right;

            // Show death sprite if dying
            if state == ENEMY_STATE_DYING {
                match enemy_type {
                    ENEMY_GINGERBREAD => sprite!("gingerdead32x32", x = ex, y = ey, flip_x = flip),
                    ENEMY_GRINCH => sprite!("grinchdead1frame32x32", x = ex, y = ey, flip_x = flip),
                    ENEMY_KRAMPUS => sprite!("krampusdeath1fram32x32", x = ex, y = ey, flip_x = flip),
                    _ => {}
                }
                return;
            }

            // Check if attacking (cooldown just started)
            let is_attacking = attack_timer > 0;

            match enemy_type {
                ENEMY_GINGERBREAD => {
                    // Only 4 sprites available (0-3)
                    let frame = ((anim_frame / 10) % 4) as i32;
                    match frame {
                        0 => sprite!("gingerbread0", x = ex, y = ey, flip_x = flip),
                        1 => sprite!("gingerbread1", x = ex, y = ey, flip_x = flip),
                        2 => sprite!("gingerbread2", x = ex, y = ey, flip_x = flip),
                        _ => sprite!("gingerbread3", x = ex, y = ey, flip_x = flip),
                    }
                }
                ENEMY_GRINCH => {
                    // Show attack animation if recently attacked
                    if is_attacking && attack_timer > GRINCH_ATTACK_COOLDOWN - 20 {
                        let attack_frame = ((attack_timer / 5) % 2) as i32;
                        match attack_frame {
                            0 => sprite!("grinchattack0", x = ex, y = ey, flip_x = flip),
                            _ => sprite!("grinchattack1", x = ex, y = ey, flip_x = flip),
                        }
                    } else {
                        // Only 3 walk sprites available (0, 1, 3 - missing 2)
                        let walk_frame = ((anim_frame / 10) % 3) as i32;
                        match walk_frame {
                            0 => sprite!("grinchwalk0", x = ex, y = ey, flip_x = flip),
                            1 => sprite!("grinchwalk1", x = ex, y = ey, flip_x = flip),
                            _ => sprite!("grinchwalk3", x = ex, y = ey, flip_x = flip),
                        }
                    }
                }
                ENEMY_KRAMPUS => {
                    // Show chain attack animation if recently attacked
                    if is_attacking && attack_timer > KRAMPUS_ATTACK_COOLDOWN - 20 {
                        let attack_frame = ((attack_timer / 5) % 2) as i32;
                        match attack_frame {
                            0 => sprite!("krampusattack0", x = ex, y = ey, flip_x = flip),
                            _ => sprite!("krampusattack1", x = ex, y = ey, flip_x = flip),
                        }
                    } else {
                        let walk_frame = ((anim_frame / 10) % 4) as i32;
                        match walk_frame {
                            0 => sprite!("krampuswalk0", x = ex, y = ey, flip_x = flip),
                            1 => sprite!("krampuswalk1", x = ex, y = ey, flip_x = flip),
                            2 => sprite!("krampuswalk2", x = ex, y = ey, flip_x = flip),
                            _ => sprite!("krampuswalk3", x = ex, y = ey, flip_x = flip),
                        }
                    }
                }
                _ => {}
            }
        };

        draw_enemy(self.enemy1_x, self.enemy1_y, self.enemy1_type, self.enemy1_facing_right,
                  self.enemy1_active, self.enemy1_state, self.enemy1_attack_timer, self.anim_frame);
        draw_enemy(self.enemy2_x, self.enemy2_y, self.enemy2_type, self.enemy2_facing_right,
                  self.enemy2_active, self.enemy2_state, self.enemy2_attack_timer, self.anim_frame);
        draw_enemy(self.enemy3_x, self.enemy3_y, self.enemy3_type, self.enemy3_facing_right,
                  self.enemy3_active, self.enemy3_state, self.enemy3_attack_timer, self.anim_frame);
        draw_enemy(self.enemy4_x, self.enemy4_y, self.enemy4_type, self.enemy4_facing_right,
                  self.enemy4_active, self.enemy4_state, self.enemy4_attack_timer, self.anim_frame);
        draw_enemy(self.enemy5_x, self.enemy5_y, self.enemy5_type, self.enemy5_facing_right,
                  self.enemy5_active, self.enemy5_state, self.enemy5_attack_timer, self.anim_frame);
        draw_enemy(self.enemy6_x, self.enemy6_y, self.enemy6_type, self.enemy6_facing_right,
                  self.enemy6_active, self.enemy6_state, self.enemy6_attack_timer, self.anim_frame);
        draw_enemy(self.enemy7_x, self.enemy7_y, self.enemy7_type, self.enemy7_facing_right,
                  self.enemy7_active, self.enemy7_state, self.enemy7_attack_timer, self.anim_frame);
        draw_enemy(self.enemy8_x, self.enemy8_y, self.enemy8_type, self.enemy8_facing_right,
                  self.enemy8_active, self.enemy8_state, self.enemy8_attack_timer, self.anim_frame);
        draw_enemy(self.enemy9_x, self.enemy9_y, self.enemy9_type, self.enemy9_facing_right,
                  self.enemy9_active, self.enemy9_state, self.enemy9_attack_timer, self.anim_frame);
        draw_enemy(self.enemy10_x, self.enemy10_y, self.enemy10_type, self.enemy10_facing_right,
                  self.enemy10_active, self.enemy10_state, self.enemy10_attack_timer, self.anim_frame);

        // Draw snowball projectiles
        self.draw_snowballs(cam_x);
    }

    pub fn draw_snowballs(&self, cam_x: i32) {
        // Draw snowball projectiles using the snawball sprite
        let draw_snowball = |x: f32, y: f32, active: bool| {
            if !active {return;}

            let sx = x as i32 - cam_x;
            let sy = y as i32;

            // Draw snowball sprite (16x16)
            sprite!("snawball", x = sx, y = sy);
        };

        draw_snowball(self.snowball1_x, self.snowball1_y, self.snowball1_active);
        draw_snowball(self.snowball2_x, self.snowball2_y, self.snowball2_active);
        draw_snowball(self.snowball3_x, self.snowball3_y, self.snowball3_active);
        draw_snowball(self.snowball4_x, self.snowball4_y, self.snowball4_active);
    }

    pub fn draw_gifts(&self, cam_x: i32) {
        // Draw floating gifts with bobbing animation - only draw if not collected
        let anim_frame = self.anim_frame;

        let draw_gift = |x: f32, y: f32, collected: bool, cam_x: i32, offset: u32| {
            if collected {return;}  // Don't draw collected gifts

            let gx = x as i32 - cam_x;

            // Floating bobbing animation - each gift has different phase
            let time = (anim_frame + offset) as f32 * 0.1;
            let bob_offset = (time.sin() * 6.0) as i32;  // Float up and down 6 pixels
            let gy = y as i32 + bob_offset;

            // Draw gift icon with floating effect
            sprite!("gift32x32", x = gx, y = gy);
        };

        // Each gift has different animation offset for variety
        draw_gift(self.gift1_x, self.gift1_y, self.gift1_collected, cam_x, 0);
        draw_gift(self.gift2_x, self.gift2_y, self.gift2_collected, cam_x, 20);
        draw_gift(self.gift3_x, self.gift3_y, self.gift3_collected, cam_x, 40);
        draw_gift(self.gift4_x, self.gift4_y, self.gift4_collected, cam_x, 60);
        draw_gift(self.gift5_x, self.gift5_y, self.gift5_collected, cam_x, 80);
    }

    pub fn draw_hud(&self) {
        // Level start screen
        if self.player_state == STATE_LEVEL_START {
            text!("PRESS SPACE TO START", x = 70, y = 80, color = 0xffffffff);
            text!("Level {}", self.current_level; x = 130, y = 100, color = 0xffff00ff);
            return;
        }

        // === TOP HUD BAR - Single row with all stats ===
        rect!(x = 0, y = 0, w = 320, h = 18, color = 0x000000AA);

        // Lives - Heart icons
        for i in 0..self.lives {
            let x = 5 + (i as i32 * 16);
            sprite!("lifeheart", x = x, y = 3, w = 12, h = 12);
        }

        // Level number
        text!("Lv {}", self.current_level; x = 60, y = 4, color = 0xFFFF00FF);

        // Score
        text!("Score {}", self.level_score; x = 95, y = 4, color = 0x00FF00FF);

        // Gifts collected
        let gifts_collected = [self.gift1_collected, self.gift2_collected, self.gift3_collected,
                               self.gift4_collected, self.gift5_collected]
            .iter().filter(|&&x| x).count();
        text!("Gifts {}/5", gifts_collected; x = 175, y = 4, color = 0xFFD700FF);

        // Time
        let minutes = self.level_time / 3600;
        let seconds = (self.level_time / 60) % 60;
        text!("{}:{:02}", minutes, seconds; x = 240, y = 4);

        // Deaths - Only show if deaths > 0
        if self.level_deaths > 0 {
            text!("x{}", self.level_deaths; x = 285, y = 4, color = 0xFF6666FF);
        }

        // Arcade mode indicator (overlays on right if active)
        if self.arcade_mode {
            rect!(x = 260, y = 2, w = 50, h = 14, color = 0xFF00FFAA);
            text!("ARCADE", x = 263, y = 4, color = 0x000000FF);
        }

        // Death messages (centered in play area)
        if self.player_state == STATE_DEAD {
            if self.lives == 0 {
                text!("Level Failed!", x = 90, y = 85, color = 0xff0000ff);
            } else {
                text!("Respawning...", x = 95, y = 90, color = 0xffaa00ff);
            }
        }
    }

    pub fn render_level_complete_stats(&self) {
        // Festive Christmas-themed level completion screen
        // Display the optimized festive background sprite (128 colors)
        sprite!("scorepage", x = -20, y = 0);  // Center the 360px sprite on 320px screen

        // Stats breakdown
        let gifts_collected = [self.gift1_collected, self.gift2_collected, self.gift3_collected,
                               self.gift4_collected, self.gift5_collected]
            .iter().filter(|&&x| x).count() as u8;

        let level_idx = (self.current_level - 1) as usize;
        let par_time = LEVEL_PAR_TIMES[level_idx];
        let time_seconds = self.level_time / 60;
        let rank = self.level_rank[level_idx];

        // Define the stats box (dark window area in the sprite)
        let box_x = 40;       // Box top-left X (screen coords)
        let box_y = 60;       // Box top-left Y (screen coords)
        let box_height = 80; // Box height
        let padding = 20;     // Box padding
        let line_height = 10; // Spacing between lines

        // Calculate left-aligned X position with padding (box-relative)
        let text_x = box_x + padding;

        // Starting Y position with top padding (box-relative)
        let mut current_y = box_y + padding;

        // TIME
        text!("TIME: {}:{:02}", time_seconds / 60, time_seconds % 60; x = text_x, y = current_y, color = 0xFFFFFFFF);
        current_y += line_height;

        // PAR
        text!("PAR_TIME: {}s", par_time; x = text_x, y = current_y, color = if time_seconds <= par_time { 0x00FF00FF } else { 0xFFAA66FF });
        current_y += line_height;

        // GIFTS
        text!("GIFTS: {}/5", gifts_collected; x = text_x, y = current_y, color = 0xFFD700FF);
        current_y += line_height;

        // DEATHS
        text!("DEATHS: {}", self.level_deaths; x = text_x, y = current_y, color = if self.level_deaths == 0 { 0x00FF00FF } else { 0xFF6666FF });
        current_y += line_height;

        // SCORE
        text!("SCORE: {}", self.level_score; x = text_x, y = current_y, color = 0x00FF88FF);
        current_y += line_height;

        // Display rank
        let rank_text = match rank {
            RANK_S => "S",
            RANK_A => "A",
            RANK_B => "B",
            RANK_C => "C",
            RANK_D => "D",
            _ => "?",
        };
        let rank_color = match rank {
            RANK_S => 0xFFD700FF,  // Gold
            RANK_A => 0x00FF00FF,  // Green
            RANK_B => 0x4499FFFF,  // Blue
            RANK_C => 0xFF8800FF,  // Orange
            _ => 0x888888FF,       // Gray
        };

        // RANK
        text!("RANK: {}", rank_text; x = text_x, y = current_y, color = rank_color);

        // NEW BEST indicator (on same line, to the right)
        if self.level_score >= self.level_best_score[level_idx] && self.level_score > 0 {
            if self.anim_frame % 20 < 10 {
                text!("NEW BEST!", x = text_x + 85, y = current_y, color = 0xFFFF00FF);
            }
        }

        // Bottom instruction (blinking) - positioned at bottom of box
        if self.anim_frame % 40 < 30 {
            let bottom_y = box_y + box_height + 5;  // Just below the box
            text!("PRESS SPACE TO CONTINUE", x = box_x + 15, y = bottom_y, color = 0xFFFF00FF);
        }
    }
}
