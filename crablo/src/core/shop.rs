use crate::world::entities::{ShopItem, ShopItemType};

pub fn create_shop_items() -> Vec<ShopItem> {
    vec![
        ShopItem {
            name: "Full Heal".to_string(),
            cost: 200,
            item_type: ShopItemType::Heal,
            purchased: false,
        },
        ShopItem {
            name: "+25 Max HP".to_string(),
            cost: 300,
            item_type: ShopItemType::MaxHp,
            purchased: false,
        },
        ShopItem {
            name: "+3 Damage".to_string(),
            cost: 400,
            item_type: ShopItemType::Damage,
            purchased: false,
        },
        ShopItem {
            name: "+2 Armor".to_string(),
            cost: 350,
            item_type: ShopItemType::Armor,
            purchased: false,
        },
    ]
}

pub struct ShopPurchaseResult {
    pub heal_full: bool,
    pub max_hp_bonus: i32,
    pub damage_bonus: i32,
    pub armor_bonus: i32,
}

pub fn try_purchase(
    items: &mut [ShopItem],
    index: usize,
    gold: i32,
) -> Option<(i32, ShopPurchaseResult)> {
    if index >= items.len() {
        return None;
    }

    let item = &items[index];
    if item.purchased || gold < item.cost {
        return None;
    }

    let cost = item.cost;
    let result = match item.item_type {
        ShopItemType::Heal => ShopPurchaseResult {
            heal_full: true,
            max_hp_bonus: 0,
            damage_bonus: 0,
            armor_bonus: 0,
        },
        ShopItemType::MaxHp => ShopPurchaseResult {
            heal_full: false,
            max_hp_bonus: 25,
            damage_bonus: 0,
            armor_bonus: 0,
        },
        ShopItemType::Damage => ShopPurchaseResult {
            heal_full: false,
            max_hp_bonus: 0,
            damage_bonus: 3,
            armor_bonus: 0,
        },
        ShopItemType::Armor => ShopPurchaseResult {
            heal_full: false,
            max_hp_bonus: 0,
            damage_bonus: 0,
            armor_bonus: 2,
        },
    };

    items[index].purchased = true;
    Some((cost, result))
}
