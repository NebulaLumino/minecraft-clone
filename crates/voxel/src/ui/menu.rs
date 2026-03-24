//! Menu module
//!
//! Main menu, pause menu, world select.

/// Game menu state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    Main,
    Playing,
    Paused,
    WorldSelect,
}

impl MenuState {
    /// Check if menu is active (non-playing state)
    pub fn is_menu_active(&self) -> bool {
        matches!(self, MenuState::Main | MenuState::Paused | MenuState::WorldSelect)
    }

    /// Check if game should render world
    pub fn should_render_world(&self) -> bool {
        matches!(self, MenuState::Playing | MenuState::Paused)
    }
}

/// Menu manager
#[derive(Debug, Clone)]
pub struct Menu {
    state: MenuState,
}

impl Menu {
    /// Create new menu (starts at main menu)
    pub fn new() -> Self {
        Self {
            state: MenuState::Main,
        }
    }

    /// Start playing (close menu)
    pub fn play(&mut self) {
        self.state = MenuState::Playing;
    }

    /// Return to main menu
    pub fn main_menu(&mut self) {
        self.state = MenuState::Main;
    }

    /// Pause game
    pub fn pause(&mut self) {
        if self.state == MenuState::Playing {
            self.state = MenuState::Paused;
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.state == MenuState::Paused {
            self.state = MenuState::Playing;
        }
    }

    /// Toggle pause
    pub fn toggle_pause(&mut self) {
        match self.state {
            MenuState::Playing => self.pause(),
            MenuState::Paused => self.resume(),
            _ => {}
        }
    }

    /// Open world select
    pub fn world_select(&mut self) {
        self.state = MenuState::WorldSelect;
    }

    /// Get current state
    pub fn state(&self) -> MenuState {
        self.state
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.state == MenuState::Playing
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}
