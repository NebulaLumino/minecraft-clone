//! Time module
//!
//! Game time, day/night cycle, and time-of-day calculations.


/// Game time manager
///
/// Minecraft time: 0-24000 ticks = 1 full day (20 minutes real time at 20 TPS)
/// - 0: Dawn/sunrise
/// - 6000: Midday (noon)
/// - 12000: Dusk/sunset
/// - 18000: Midnight
/// - 24000: Next dawn
#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    /// Total game time in ticks
    ticks: i64,
    /// Time scale (1.0 = normal, 0.0 = paused)
    time_scale: f32,
}

impl GameTime {
    /// Create new game time starting at dawn
    pub fn new() -> Self {
        Self {
            ticks: 0,
            time_scale: 1.0,
        }
    }

    /// Create game time starting at a specific time of day
    /// `time_of_day` is 0-24000
    pub fn with_time_of_day(time_of_day: i64) -> Self {
        Self {
            ticks: time_of_day,
            time_scale: 1.0,
        }
    }

    /// Update game time
    pub fn update(&mut self, delta: f32) {
        if self.time_scale > 0.0 {
            // 20 TPS = 1 tick per 50ms
            // delta is in seconds, so multiply by 20 to get ticks
            let ticks_delta = (delta * 20.0 * self.time_scale as f32) as i64;
            self.ticks = (self.ticks + ticks_delta) % 24000;
        }
    }

    /// Get current time of day (0-24000)
    pub fn time_of_day(&self) -> i64 {
        self.ticks
    }

    /// Get time of day as fraction (0.0 - 1.0)
    pub fn time_of_day_fraction(&self) -> f32 {
        self.ticks as f32 / 24000.0
    }

    /// Check if it's daytime (0 - 12000)
    pub fn is_daytime(&self) -> bool {
        self.ticks < 12000
    }

    /// Get sun angle (0.0 at dawn, PI at dusk)
    pub fn sun_angle(&self) -> f32 {
        // Dawn at 0, noon at PI/2, dusk at PI, midnight at 3*PI/2
        // Using cosine: cos(0) = 1 (noon), cos(PI) = -1 (midnight)
        let fraction = self.ticks as f32 / 12000.0 * std::f32::consts::PI;
        fraction
    }

    /// Get ambient light level (0.0 - 1.0)
    /// Returns brightness for block lighting
    pub fn ambient_light(&self) -> f32 {
        if self.ticks < 12000 {
            // Daytime: 1.0 at noon, 0.0 at night
            // Smooth transition at dawn/dusk
            let day_fraction = self.ticks as f32 / 12000.0;
            if day_fraction < 0.1 {
                // Dawn transition
                day_fraction * 10.0
            } else if day_fraction > 0.9 {
                // Dusk transition
                (1.0 - day_fraction) * 10.0
            } else {
                1.0
            }
        } else {
            // Nighttime: always dim
            0.0
        }
    }

    /// Get sky light color multiplier
    pub fn sky_color(&self) -> (f32, f32, f32) {
        if self.ticks < 12000 {
            // Day sky - blue tint
            (0.4, 0.7, 1.0)
        } else if self.ticks < 13000 {
            // Sunset - orange/red
            (1.0, 0.5, 0.3)
        } else if self.ticks < 18000 {
            // Night sky - dark blue
            (0.1, 0.1, 0.2)
        } else {
            // Pre-dawn - purple/dark
            (0.3, 0.2, 0.4)
        }
    }

    /// Set time scale (0.0 = paused, 1.0 = normal)
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.clamp(0.0, 10.0);
    }

    /// Get time scale
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Set time of day directly
    pub fn set_time_of_day(&mut self, time: i64) {
        self.ticks = time.clamp(0, 23999);
    }

    /// Advance by one tick (for server-side updates)
    pub fn tick(&mut self) {
        if self.time_scale > 0.0 {
            self.ticks = (self.ticks + 1) % 24000;
        }
    }

    /// Get formatted time string (Minecraft style)
    pub fn formatted_time(&self) -> String {
        let hours = (self.ticks / 1000) as i32;
        let minutes = ((self.ticks % 1000) * 60 / 1000) as i32;

        // Convert to 24-hour format starting at 6 AM
        let hours_24 = (6 + hours) % 24;

        format!("{:02}:{:02}", hours_24, minutes)
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_night_cycle() {
        let mut time = GameTime::new();
        assert!(time.is_daytime());

        time.set_time_of_day(12000);
        assert!(!time.is_daytime());

        time.set_time_of_day(0);
        assert!(time.is_daytime());
    }

    #[test]
    fn test_time_wrapping() {
        let mut time = GameTime::with_time_of_day(23999);
        time.tick();
        assert_eq!(time.time_of_day(), 0);
    }
}
