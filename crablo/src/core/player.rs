//! Player state and abilities.

use crate::core::constants::{Difficulty, PLAYER_DAMAGE, PLAYER_START_HP};
use crate::core::traits::{DamageDealer, Damageable};

/// Represents the player character with all stats and abilities.
#[derive(Clone)]
pub struct Player {
    /// X position on the map.
    pub x: usize,
    /// Y position on the map.
    pub y: usize,
    /// Current health points.
    pub hp: i32,
    /// Maximum health points.
    pub max_hp: i32,
    /// Movement cooldown timer.
    pub move_cd: f32,
    /// Current movement path.
    pub path: Vec<(usize, usize)>,
    /// Dash ability cooldown.
    pub dash_cd: f32,
    /// Area attack ability cooldown.
    pub area_cd: f32,
    /// Heal ability cooldown.
    pub heal_cd: f32,
    /// Ranged attack ability cooldown.
    pub ranged_cd: f32,
    /// Bonus damage from equipment.
    pub weapon_damage: i32,
    /// Armor value reducing incoming damage.
    pub armor: i32,
    /// Current experience points.
    pub xp: i32,
    /// Current level.
    pub level: i32,
    /// XP required for next level.
    pub xp_to_next: i32,
    /// Remaining poison duration.
    pub poisoned: f32,
}

impl Damageable for Player {
    fn hp(&self) -> i32 {
        self.hp
    }

    fn max_hp(&self) -> i32 {
        self.max_hp
    }

    fn take_damage(&mut self, amount: i32) -> i32 {
        let reduced = (amount - self.armor.min(amount - 1)).max(1);
        self.hp -= reduced;
        reduced
    }

    fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

impl DamageDealer for Player {
    fn damage(&self) -> i32 {
        PLAYER_DAMAGE + self.weapon_damage
    }
}

impl Player {
    /// Create a new player at the given position.
    pub fn new(x: usize, y: usize, difficulty: Difficulty) -> Self {
        let hp = (PLAYER_START_HP as f32 * difficulty.player_hp_mult()) as i32;
        Player {
            x,
            y,
            hp,
            max_hp: hp,
            move_cd: 0.,
            path: vec![],
            dash_cd: 0.,
            area_cd: 0.,
            heal_cd: 0.,
            ranged_cd: 0.,
            weapon_damage: 0,
            armor: 0,
            xp: 0,
            level: 1,
            xp_to_next: 100,
            poisoned: 0.,
        }
    }
}
