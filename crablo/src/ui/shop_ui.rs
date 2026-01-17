use crate::world::entities::ShopItem;
use macroquad::prelude::*;

pub fn draw_shop(items: &[ShopItem], gold: i32) {
    // Dark overlay
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(0., 0., 0., 0.8),
    );

    draw_text("SHOP", screen_width() / 2. - 50., 80., 50., GOLD);
    draw_text(
        &format!("Gold: {}", gold),
        screen_width() / 2. - 60.,
        120.,
        24.,
        GOLD,
    );

    for (i, item) in items.iter().enumerate() {
        let y = 170. + (i as f32 * 50.);
        let color = if item.purchased {
            DARKGRAY
        } else if gold >= item.cost {
            WHITE
        } else {
            RED
        };

        let status = if item.purchased { " [SOLD]" } else { "" };
        draw_text(
            &format!("{}. {} - {} gold{}", i + 1, item.name, item.cost, status),
            screen_width() / 2. - 150.,
            y,
            24.,
            color,
        );
    }

    draw_text(
        "Press 1-4 to buy, ENTER to continue",
        screen_width() / 2. - 160.,
        screen_height() - 50.,
        20.,
        GRAY,
    );
}
