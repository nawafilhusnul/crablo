// Map constants
pub const MAP_SIZE: usize = 20;
pub const TILE_WIDTH: f32 = 32.;
pub const TILE_HEIGHT: f32 = 16.;

// Player constants
pub const PLAYER_START_HP: i32 = 100;
pub const PLAYER_DAMAGE: i32 = 10;
pub const PLAYER_MOVE_CD: f32 = 0.15;

// Monster constants (base values, individual monsters may vary)
pub const MONSTER_HP: i32 = 30;

// Scoring
pub const GOLD_VALUE: i32 = 100;
pub const KILL_BONUS: i32 = 50;

// Animation
pub const DMG_TEXT_DURATION: f32 = 1.0;
pub const DMG_TEXT_SPEED: f32 = 20.;

// Difficulty settings
#[derive(Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn monster_damage_mult(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.5,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.5,
        }
    }

    pub fn monster_hp_mult(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.7,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.5,
        }
    }

    pub fn player_hp_mult(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.5,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 0.75,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }
}
