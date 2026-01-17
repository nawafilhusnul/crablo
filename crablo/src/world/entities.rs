use crate::core::constants::MONSTER_HP;

#[derive(Clone, Copy, PartialEq)]
pub enum MonsterType {
    Normal, // Standard enemy
    Fast,   // Moves quickly, low HP
    Tank,   // Slow but high HP and damage
    Boss,   // Very strong, appears every 5 floors
}

pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub cd: f32,
    pub monster_type: MonsterType,
    pub damage: i32,
    pub move_cd: f32,
}

impl Monster {
    pub fn new(x: usize, y: usize) -> Self {
        Monster {
            x,
            y,
            hp: MONSTER_HP,
            cd: 0.,
            monster_type: MonsterType::Normal,
            damage: 5,
            move_cd: 1.0,
        }
    }

    pub fn new_fast(x: usize, y: usize) -> Self {
        Monster {
            x,
            y,
            hp: MONSTER_HP / 2, // 15 HP
            cd: 0.,
            monster_type: MonsterType::Fast,
            damage: 3,
            move_cd: 0.5, // Moves twice as fast
        }
    }

    pub fn new_tank(x: usize, y: usize) -> Self {
        Monster {
            x,
            y,
            hp: MONSTER_HP * 2, // 60 HP
            cd: 0.,
            monster_type: MonsterType::Tank,
            damage: 10,
            move_cd: 1.5, // Slower
        }
    }

    pub fn new_boss(x: usize, y: usize) -> Self {
        Monster {
            x,
            y,
            hp: MONSTER_HP * 5, // 150 HP
            cd: 0.,
            monster_type: MonsterType::Boss,
            damage: 20,
            move_cd: 2.0, // Very slow but deadly
        }
    }
}

pub struct DmgText {
    pub x: f32,
    pub y: f32,
    pub dmg: i32,
    pub life: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EquipmentType {
    Sword,  // +5 damage
    Shield, // +5 armor
    Ring,   // +20 max HP
}

#[derive(Clone)]
pub struct ShopItem {
    pub name: String,
    pub cost: i32,
    pub item_type: ShopItemType,
    pub purchased: bool,
}

#[derive(Clone, PartialEq)]
pub enum ShopItemType {
    Heal,   // Full heal
    MaxHp,  // +25 max HP
    Damage, // +3 damage
    Armor,  // +2 armor
}
