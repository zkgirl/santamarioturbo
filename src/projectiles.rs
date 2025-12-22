use crate::*;

impl GameState {
    pub fn spawn_snowball(&mut self, enemy_x: f32, enemy_y: f32, player_x: f32) {
        // Find an inactive snowball slot
        // Always aim at Santa! Calculate direction based on player position
        let snowball_vx = if player_x > enemy_x { SNOWBALL_SPEED } else { -SNOWBALL_SPEED };

        if !self.snowball1_active {
            self.snowball1_x = enemy_x;
            self.snowball1_y = enemy_y + 8.0;  // Center of enemy sprite
            self.snowball1_vx = snowball_vx;
            self.snowball1_start_x = enemy_x;  // Store spawn position
            self.snowball1_active = true;
        } else if !self.snowball2_active {
            self.snowball2_x = enemy_x;
            self.snowball2_y = enemy_y + 8.0;
            self.snowball2_vx = snowball_vx;
            self.snowball2_start_x = enemy_x;
            self.snowball2_active = true;
        } else if !self.snowball3_active {
            self.snowball3_x = enemy_x;
            self.snowball3_y = enemy_y + 8.0;
            self.snowball3_vx = snowball_vx;
            self.snowball3_start_x = enemy_x;
            self.snowball3_active = true;
        } else if !self.snowball4_active {
            self.snowball4_x = enemy_x;
            self.snowball4_y = enemy_y + 8.0;
            self.snowball4_vx = snowball_vx;
            self.snowball4_start_x = enemy_x;
            self.snowball4_active = true;
        }
    }

    pub fn update_projectiles(&mut self) {
        // Helper to update a single snowball with 80px range limit
        let update_snowball = |x: &mut f32, y: f32, vx: f32, start_x: f32, active: &mut bool, player_x: f32, player_y: f32| {
            if !*active {return false;}

            // Move snowball
            *x += vx;

            // Check if traveled more than 80 pixels from spawn point
            let distance_traveled = (*x - start_x).abs();
            if distance_traveled > 80.0 {
                *active = false;
                return false;
            }

            // Check collision with player
            let dx = (*x - player_x).abs();
            let dy = (y - player_y).abs();
            if dx < 24.0 && dy < 24.0 {
                *active = false;
                return true;  // Hit player!
            }

            false
        };

        let px = self.player_x;
        let py = self.player_y;

        // Update all snowballs with range limit
        if update_snowball(&mut self.snowball1_x, self.snowball1_y, self.snowball1_vx, self.snowball1_start_x, &mut self.snowball1_active, px, py) {
            self.die();
        }
        if update_snowball(&mut self.snowball2_x, self.snowball2_y, self.snowball2_vx, self.snowball2_start_x, &mut self.snowball2_active, px, py) {
            self.die();
        }
        if update_snowball(&mut self.snowball3_x, self.snowball3_y, self.snowball3_vx, self.snowball3_start_x, &mut self.snowball3_active, px, py) {
            self.die();
        }
        if update_snowball(&mut self.snowball4_x, self.snowball4_y, self.snowball4_vx, self.snowball4_start_x, &mut self.snowball4_active, px, py) {
            self.die();
        }

        // Check Krampus chain attacks (melee range attacks during attack animation)
        self.check_krampus_chain_attacks();
    }

}
