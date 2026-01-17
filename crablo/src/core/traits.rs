//! Common traits for game components.
//!
//! These traits define common behaviors for game entities.

/// Trait for entities that can take damage.
pub trait Damageable {
    /// Get current health points.
    fn hp(&self) -> i32;

    /// Get maximum health points.
    fn max_hp(&self) -> i32;

    /// Take damage and return actual damage taken.
    fn take_damage(&mut self, amount: i32) -> i32;

    /// Check if the entity is dead.
    fn is_dead(&self) -> bool {
        self.hp() <= 0
    }

    /// Heal the entity.
    fn heal(&mut self, amount: i32);
}

/// Trait for entities that can deal damage.
pub trait DamageDealer {
    /// Get the base damage this entity deals.
    fn damage(&self) -> i32;
}
