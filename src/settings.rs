use crate::*;

impl GameState {
    pub fn update_settings(&mut self) {
        let kb = keyboard::get();
        let gp = gamepad::get(0);

        // Update input cooldown
        if self.input_cooldown > 0 {
            self.input_cooldown -= 1;
        }

        // Handle text input for developer password
        if self.entering_password {
            // Get text input from keyboard (check each key individually)
            if self.password_input.len() < 20 {  // Limit password length
                if kb.key_a().just_pressed() { self.password_input.push('a'); }
                if kb.key_b().just_pressed() { self.password_input.push('b'); }
                if kb.key_c().just_pressed() { self.password_input.push('c'); }
                if kb.key_d().just_pressed() { self.password_input.push('d'); }
                if kb.key_e().just_pressed() { self.password_input.push('e'); }
                if kb.key_f().just_pressed() { self.password_input.push('f'); }
                if kb.key_g().just_pressed() { self.password_input.push('g'); }
                if kb.key_h().just_pressed() { self.password_input.push('h'); }
                if kb.key_i().just_pressed() { self.password_input.push('i'); }
                if kb.key_j().just_pressed() { self.password_input.push('j'); }
                if kb.key_k().just_pressed() { self.password_input.push('k'); }
                if kb.key_l().just_pressed() { self.password_input.push('l'); }
                if kb.key_m().just_pressed() { self.password_input.push('m'); }
                if kb.key_n().just_pressed() { self.password_input.push('n'); }
                if kb.key_o().just_pressed() { self.password_input.push('o'); }
                if kb.key_p().just_pressed() { self.password_input.push('p'); }
                if kb.key_q().just_pressed() { self.password_input.push('q'); }
                if kb.key_r().just_pressed() { self.password_input.push('r'); }
                if kb.key_s().just_pressed() { self.password_input.push('s'); }
                if kb.key_t().just_pressed() { self.password_input.push('t'); }
                if kb.key_u().just_pressed() { self.password_input.push('u'); }
                if kb.key_v().just_pressed() { self.password_input.push('v'); }
                if kb.key_w().just_pressed() { self.password_input.push('w'); }
                if kb.key_x().just_pressed() { self.password_input.push('x'); }
                if kb.key_y().just_pressed() { self.password_input.push('y'); }
                if kb.key_z().just_pressed() { self.password_input.push('z'); }
            }

            // Backspace to delete characters
            if kb.backspace().just_pressed() {
                self.password_input.pop();
            }

            // Enter to submit password
            if kb.enter().just_pressed() || gp.a.just_pressed() {
                if self.password_input == "santagiftsme" {
                    // Correct password! Enable developer mode and unlock all levels
                    self.developer_mode = true;
                    for i in 0..10 {
                        self.level_unlocked[i] = true;
                    }
                    self.play_sfx("all_gifts_collected");  // Success sound
                }
                // Clear password and exit password entry mode
                self.password_input.clear();
                self.entering_password = false;
            }

            // ESC or B to cancel password entry
            if kb.escape().just_pressed() || gp.b.just_pressed() {
                self.password_input.clear();
                self.entering_password = false;
            }

            return;  // Don't process other inputs while entering password
        }

        // Navigate with arrow keys (only when cooldown is 0)
        if self.input_cooldown == 0 {
            let mut moved = false;

            // Up/Down navigation through settings options
            if kb.arrow_up().just_pressed() || gp.up.just_pressed() {
                if self.selected_setting > 0 {
                    self.selected_setting -= 1;
                    moved = true;
                    self.play_sfx("menu_button");
                }
            } else if kb.arrow_down().just_pressed() || gp.down.just_pressed() {
                if self.selected_setting < SETTING_BACK {
                    self.selected_setting += 1;
                    moved = true;
                    self.play_sfx("menu_button");
                }
            }

            // Left/Right to adjust volume
            if kb.arrow_left().just_pressed() || gp.left.just_pressed() {
                match self.selected_setting {
                    SETTING_MUSIC => {
                        // Decrease music volume by 10%
                        self.music_volume = (self.music_volume - 0.1).max(0.0);
                        if self.music_volume == 0.0 {
                            self.music_enabled = false;
                        }
                        self.play_sfx("menu_button");
                    }
                    SETTING_SFX => {
                        // Decrease SFX volume by 10%
                        self.sfx_volume = (self.sfx_volume - 0.1).max(0.0);
                        if self.sfx_volume == 0.0 {
                            self.sfx_enabled = false;
                        }
                        self.play_sfx("menu_button");
                    }
                    _ => {}
                }
                moved = true;
            } else if kb.arrow_right().just_pressed() || gp.right.just_pressed() {
                match self.selected_setting {
                    SETTING_MUSIC => {
                        // Increase music volume by 10%
                        self.music_volume = (self.music_volume + 0.1).min(1.0);
                        if self.music_volume > 0.0 {
                            self.music_enabled = true;
                        }
                        self.play_sfx("menu_button");
                    }
                    SETTING_SFX => {
                        // Increase SFX volume by 10%
                        self.sfx_volume = (self.sfx_volume + 0.1).min(1.0);
                        if self.sfx_volume > 0.0 {
                            self.sfx_enabled = true;
                        }
                        self.play_sfx("menu_button");
                    }
                    _ => {}
                }
                moved = true;
            }

            if moved {
                self.input_cooldown = 8;  // Add cooldown to prevent rapid movement
            }
        }

        // Select option with ENTER or A button (NOT SPACE!)
        if kb.enter().just_pressed() || gp.a.just_pressed() {
            match self.selected_setting {
                SETTING_MUSIC => {
                    // Toggle music on/off
                    self.music_enabled = !self.music_enabled;
                    self.play_sfx("menu_button");
                }
                SETTING_SFX => {
                    // Toggle SFX on/off
                    self.sfx_enabled = !self.sfx_enabled;
                    self.play_sfx("menu_button");
                }
                SETTING_ARCADE => {
                    // Toggle arcade mode on/off
                    self.arcade_mode = !self.arcade_mode;
                    self.play_sfx("menu_button");
                }
                SETTING_DEVELOPER => {
                    // Enter password mode
                    self.entering_password = true;
                    self.password_input.clear();
                    self.play_sfx("menu_button");
                }
                SETTING_BACK => {
                    // Always return to level select (main menu)
                    self.play_sfx("menu_button");
                    self.current_screen = SCREEN_LEVEL_SELECT;
                    self.selected_setting = SETTING_MUSIC;  // Reset to first option
                }
                _ => {}
            }
        }

        // ESC or B to go back to previous screen
        if kb.escape().just_pressed() || gp.b.just_pressed() {
            self.play_sfx("menu_button");
            self.current_screen = self.previous_screen;
            self.selected_setting = SETTING_MUSIC;  // Reset to first option
        }
    }

    pub fn render_settings(&mut self) {
        // Draw scorepage background
        sprite!("scorepage", x = 0, y = 0);

        // Stats box positioning
        let box_x = 40;
        let box_y = 50;
        let padding = 15;
        let line_height = 12;  // Compact line height
        let text_x = box_x + padding;
        let mut current_y = box_y + padding;

        // Title
        text!("SETTINGS", x = text_x + 40, y = current_y, color = 0xFFFFFFFF);
        current_y += line_height + 2;

        // Music option
        let music_color = if self.selected_setting == SETTING_MUSIC { 0xFFFF00FF } else { 0xFFFFFFFF };
        text!("MUSIC:", x = text_x, y = current_y, color = music_color);

        // Music volume bar
        let bar_x = text_x + 90;
        let bar_y = current_y;
        let bar_width = 100;
        let bar_height = 8;

        // Background bar (dark)
        rect!(x = bar_x, y = bar_y, w = bar_width, h = bar_height, color = 0x333333FF);

        // Filled bar based on volume
        if self.music_enabled {
            let fill_width = (bar_width as f32 * self.music_volume) as u32;
            rect!(x = bar_x, y = bar_y, w = fill_width, h = bar_height, color = 0x00FF00FF);
        }

        // Volume percentage
        let volume_percent = (self.music_volume * 100.0) as u32;
        let status = if self.music_enabled {
            format!("{}%", volume_percent)
        } else {
            "OFF".to_string()
        };
        text!(&status, x = bar_x + bar_width + 5, y = current_y, color = music_color);

        current_y += line_height;

        // SFX option
        let sfx_color = if self.selected_setting == SETTING_SFX { 0xFFFF00FF } else { 0xFFFFFFFF };
        text!("SFX:", x = text_x, y = current_y, color = sfx_color);

        // SFX volume bar
        let bar_y = current_y;

        // Background bar (dark)
        rect!(x = bar_x, y = bar_y, w = bar_width, h = bar_height, color = 0x333333FF);

        // Filled bar based on volume
        if self.sfx_enabled {
            let fill_width = (bar_width as f32 * self.sfx_volume) as u32;
            rect!(x = bar_x, y = bar_y, w = fill_width, h = bar_height, color = 0x00FF00FF);
        }

        // Volume percentage
        let volume_percent = (self.sfx_volume * 100.0) as u32;
        let status = if self.sfx_enabled {
            format!("{}%", volume_percent)
        } else {
            "OFF".to_string()
        };
        text!(&status, x = bar_x + bar_width + 5, y = current_y, color = sfx_color);

        current_y += line_height;

        // Arcade mode option
        let arcade_color = if self.selected_setting == SETTING_ARCADE { 0xFFFF00FF } else { 0xFFFFFFFF };
        text!("ARCADE:", x = text_x, y = current_y, color = arcade_color);
        let arcade_status = if self.arcade_mode { "ON" } else { "OFF" };
        text!(arcade_status, x = text_x + 90, y = current_y, color = arcade_color);

        current_y += line_height;

        // Developer option
        let dev_color = if self.selected_setting == SETTING_DEVELOPER { 0xFFFF00FF } else { 0xFFFFFFFF };

        if self.entering_password {
            // Show password input field
            text!("DEVELOPER:", x = text_x, y = current_y, color = dev_color);

            // Show password input with asterisks for privacy
            let password_display = "*".repeat(self.password_input.len());
            text!(&password_display, x = text_x + 90, y = current_y, color = 0xFFFFFFFF);

            // Show cursor blinking
            if (self.anim_frame / 30) % 2 == 0 {
                text!("_", x = text_x + 90 + (self.password_input.len() * 8) as i32, y = current_y, color = 0xFFFFFFFF);
            }
        } else {
            text!("DEVELOPER:", x = text_x, y = current_y, color = dev_color);

            let dev_status = if self.developer_mode {
                "ENABLED"
            } else {
                "LOCKED"
            };
            text!(dev_status, x = text_x + 90, y = current_y, color = dev_color);
        }

        current_y += line_height;

        // Back to main menu
        let back_color = if self.selected_setting == SETTING_BACK { 0xFFFF00FF } else { 0xFFFFFFFF };
        text!("BACK TO MENU", x = text_x + 30, y = current_y, color = back_color);

        // Instructions at bottom
        let instructions = if self.entering_password {
            "Type Password   ENTER: Submit   ESC: Cancel"
        } else {
            "Arrows: Adjust   ENTER: Select   ESC: Back"
        };
        text!(instructions, x = 25, y = 165, color = 0xFFFFFFFF);
    }
}
