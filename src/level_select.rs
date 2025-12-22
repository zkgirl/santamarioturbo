use crate::*;

impl GameState {
    pub fn update_level_select(&mut self) {
        let kb = keyboard::get();
        let gp = gamepad::get(0);

        // Update input cooldown
        if self.input_cooldown > 0 {
            self.input_cooldown -= 1;
        }

        // Navigate with arrow keys in 2x5 grid (only when cooldown is 0)
        if self.input_cooldown == 0 {
            let mut moved = false;

            // Up/Down navigation (levels 1-5 on top row, 6-10 on bottom row)
            if kb.arrow_up().just_pressed() || gp.up.just_pressed() {
                if self.selected_level > 5 {
                    self.selected_level -= 5;  // Move to top row
                    moved = true;
                    self.play_sfx("menu_button");
                }
            } else if kb.arrow_down().just_pressed() || gp.down.just_pressed() {
                if self.selected_level <= 5 {
                    self.selected_level += 5;  // Move to bottom row
                    if self.selected_level > 10 {
                        self.selected_level = 10;
                    }
                    moved = true;
                    self.play_sfx("menu_button");
                }
            }

            // Left/Right navigation within rows
            if kb.arrow_left().just_pressed() || gp.left.just_pressed() {
                if self.selected_level > 1 && self.selected_level <= 10 {
                    // Check if we're at the start of a row
                    if self.selected_level == 6 {
                        self.selected_level = 5;  // Jump from level 6 to 5
                    } else {
                        self.selected_level -= 1;
                    }
                    moved = true;
                    self.play_sfx("menu_button");
                }
            } else if kb.arrow_right().just_pressed() || gp.right.just_pressed() {
                if self.selected_level < 10 {
                    // Check if we're at the end of a row
                    if self.selected_level == 5 {
                        self.selected_level = 6;  // Jump from level 5 to 6
                    } else {
                        self.selected_level += 1;
                    }
                    moved = true;
                    self.play_sfx("menu_button");
                }
            }

            if moved {
                self.input_cooldown = 8;  // Add cooldown to prevent rapid movement
            }
        }

        // Check keys
        let space_pressed = kb.space().just_pressed();
        let esc_pressed = kb.escape().just_pressed();

        // Select level with SPACE or A button (takes priority)
        if space_pressed || gp.a.just_pressed() {
            // Can select unlocked levels OR any level if developer mode is enabled
            if self.level_unlocked[(self.selected_level - 1) as usize] || self.developer_mode {
                self.play_sfx("menu_button");
                self.current_screen = SCREEN_PLAYING;
                self.load_level(self.selected_level);
            }
        }
        // Open settings with ESC or SELECT button (only if SPACE is NOT pressed)
        else if esc_pressed || gp.select.just_pressed() {
            self.play_sfx("menu_button");
            self.previous_screen = SCREEN_LEVEL_SELECT;
            self.current_screen = SCREEN_SETTINGS;
        }
    }

    pub fn render_level_select(&mut self) {
        // Draw background sprite
        sprite!("levelpage/background", x = 0, y = 0);

        // Center the wooden frame perfectly (360px frame on 320px screen)
        sprite!("levelpage/wooden frame", x = -20, y = 0);

        // 2x5 Grid layout - 10 levels total
        // Each sprite is 48x48, levelselect highlight is also 48x48
        let positions = [
            // Top row - 5 levels evenly spaced
            (36, 52),    // Level 1
            (88, 52),    // Level 2
            (140, 52),   // Level 3
            (192, 52),   // Level 4
            (244, 52),   // Level 5
            // Bottom row - 5 levels evenly spaced
            (36, 112),   // Level 6
            (88, 112),   // Level 7
            (140, 112),  // Level 8
            (192, 112),  // Level 9
            (244, 112),  // Level 10
        ];

        for i in 0..10 {
            let (x, y) = positions[i];
            let level_num = (i + 1) as u8;
            let is_selected = level_num == self.selected_level;

            // Draw selection highlight (golden frame) - same position as level for perfect fit
            if is_selected {
                sprite!("levelpage/levelselect", x = x, y = y);
            }

            // Draw level content directly at position
            // Show unlocked if level is unlocked OR developer mode is enabled
            if self.level_unlocked[i] || self.developer_mode {
                // Show level number sprite
                let level_sprite = match level_num {
                    1 => "levelpage/level one",
                    2 => "levelpage/level 2",
                    3 => "levelpage/level 3",
                    4 => "levelpage/level 4",
                    5 => "levelpage/level 5",
                    6 => "levelpage/level 6",
                    7 => "levelpage/level 7",
                    8 => "levelpage/level 8",
                    9 => "levelpage/level 9",
                    10 => "levelpage/level 10",
                    _ => "levelpage/locked level",
                };
                sprite!(level_sprite, x = x, y = y);

                // Show gift stars overlaid at bottom of level tile
                let gifts = self.level_gifts_collected[i];
                if gifts > 0 {
                    let gift_sprite = match gifts {
                        1 => "levelpage/1 gift",
                        2 => "levelpage/2 gifts",
                        3 => "levelpage/3 gifts",
                        4 => "levelpage/4 gifts",
                        5 => "levelpage/5 gifts",
                        _ => "levelpage/1 gift",
                    };
                    sprite!(gift_sprite, x = x, y = y);
                }
            } else {
                // Show lock icon for locked levels
                sprite!("levelpage/locked level", x = x, y = y);
            }
        }

        // Navigation instructions at bottom
        text!("Arrow Keys: Navigate   SPACE: Select   ESC: Settings", x = 15, y = 168, color = 0xFFFFFFFF);
    }

}
