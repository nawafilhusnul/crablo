use crate::core::constants::{Difficulty, MAP_SIZE};
use crate::world::map::Tile;
use macroquad::prelude::*;

pub struct HudData<'a> {
    pub hp: i32,
    pub max_hp: i32,
    pub score: i32,
    pub floor: i32,
    pub level: i32,
    pub xp: i32,
    pub xp_to_next: i32,
    pub weapon_damage: i32,
    pub armor: i32,
    pub poisoned: f32,
    pub dash_cd: f32,
    pub area_cd: f32,
    pub heal_cd: f32,
    pub ranged_cd: f32,
    pub difficulty: Difficulty,
    // Minimap data
    pub player_pos: (usize, usize),
    pub map: &'a [[Tile; MAP_SIZE]; MAP_SIZE],
    pub explored: &'a [[bool; MAP_SIZE]; MAP_SIZE],
    pub gold: &'a [(usize, usize)],
    pub potions: &'a [(usize, usize)],
    pub monster_positions: &'a [(usize, usize)],
}

pub fn draw_hud(data: &HudData) {
    // HP Bar
    let hp_bar_width = 200.;
    let hp_bar_height = 20.;
    let hp_ratio = data.hp as f32 / data.max_hp as f32;
    draw_rectangle(
        20.,
        screen_height() - 50.,
        hp_bar_width,
        hp_bar_height,
        DARKGRAY,
    );
    draw_rectangle(
        20.,
        screen_height() - 50.,
        hp_bar_width * hp_ratio,
        hp_bar_height,
        RED,
    );
    draw_rectangle_lines(
        20.,
        screen_height() - 50.,
        hp_bar_width,
        hp_bar_height,
        2.,
        BLACK,
    );
    draw_text(
        &format!("{}/{}", data.hp, data.max_hp),
        25.,
        screen_height() - 35.,
        18.,
        WHITE,
    );

    // Score and Floor
    draw_text(
        &format!("SCORE: {}  FLOOR: {}", data.score, data.floor),
        20.,
        screen_height() - 70.,
        24.,
        BLACK,
    );

    // Difficulty indicator
    draw_text(
        &format!("[{}]", data.difficulty.name()),
        screen_width() - 100.,
        30.,
        20.,
        DARKGRAY,
    );

    // Ability cooldowns
    let abilities = [
        ("DASH [SPACE]", data.dash_cd),
        ("AREA [Q]", data.area_cd),
        ("HEAL [E]", data.heal_cd),
        ("RANGED [R]", data.ranged_cd),
    ];
    for (i, (name, cd)) in abilities.iter().enumerate() {
        let y = screen_height() - 90. - (i as f32 * 18.);
        let text = if *cd <= 0. {
            format!("{} READY", name)
        } else {
            format!("{} [{:.1}s]", name, cd)
        };
        let color = if *cd <= 0. { GREEN } else { GRAY };
        draw_text(&text, 20., y, 16., color);
    }

    // Level and XP bar
    draw_text(
        &format!("LVL {}", data.level),
        230.,
        screen_height() - 45.,
        20.,
        DARKBLUE,
    );
    let xp_bar_width = 80.;
    let xp_ratio = data.xp as f32 / data.xp_to_next as f32;
    draw_rectangle(230., screen_height() - 38., xp_bar_width, 8., DARKGRAY);
    draw_rectangle(
        230.,
        screen_height() - 38.,
        xp_bar_width * xp_ratio,
        8.,
        BLUE,
    );

    // Equipment stats
    if data.weapon_damage > 0 || data.armor > 0 {
        draw_text(
            &format!("DMG+{} ARM+{}", data.weapon_damage, data.armor),
            230.,
            screen_height() - 60.,
            16.,
            ORANGE,
        );
    }

    // Poison indicator
    if data.poisoned > 0. {
        draw_text(
            &format!("POISONED {:.1}s", data.poisoned),
            230.,
            screen_height() - 75.,
            16.,
            GREEN,
        );
    }

    // Minimap
    draw_minimap(data);
}

fn draw_minimap(data: &HudData) {
    let minimap_size = 100.;
    let minimap_x = screen_width() - minimap_size - 10.;
    let minimap_y = screen_height() - minimap_size - 10.;
    let tile_size = minimap_size / MAP_SIZE as f32;

    // Background
    draw_rectangle(
        minimap_x,
        minimap_y,
        minimap_size,
        minimap_size,
        Color::new(0., 0., 0., 0.7),
    );

    // Tiles
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            if !data.explored[y][x] {
                continue;
            }
            let mx = minimap_x + x as f32 * tile_size;
            let my = minimap_y + y as f32 * tile_size;

            let color = if data.map[y][x] == Tile::Wall {
                DARKGRAY
            } else if data.gold.contains(&(x, y)) {
                GOLD
            } else if data.potions.contains(&(x, y)) {
                RED
            } else {
                LIGHTGRAY
            };
            draw_rectangle(mx, my, tile_size, tile_size, color);
        }
    }

    // Monsters
    for &(mx, my) in data.monster_positions {
        if data.explored[my][mx] {
            let px = minimap_x + mx as f32 * tile_size;
            let py = minimap_y + my as f32 * tile_size;
            draw_rectangle(px, py, tile_size, tile_size, PURPLE);
        }
    }

    // Player
    let px = minimap_x + data.player_pos.0 as f32 * tile_size;
    let py = minimap_y + data.player_pos.1 as f32 * tile_size;
    draw_rectangle(px, py, tile_size * 1.5, tile_size * 1.5, BLUE);

    // Border
    draw_rectangle_lines(minimap_x, minimap_y, minimap_size, minimap_size, 2., WHITE);
}
