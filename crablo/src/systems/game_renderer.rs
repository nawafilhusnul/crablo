use macroquad::prelude::*;

use crate::core::constants::{Difficulty, MAP_SIZE, MONSTER_HP};
use crate::core::player::Player;
use crate::systems::rendering::{draw_stickman, draw_stickman_typed, draw_wall, to_screen};
use crate::world::entities::{DmgText, EquipmentType, Monster, MonsterType, ShopItem};
use crate::world::map::Tile;

pub struct GameRenderData<'a> {
    pub map: &'a [[Tile; MAP_SIZE]; MAP_SIZE],
    pub explored: &'a [[bool; MAP_SIZE]; MAP_SIZE],
    pub cam: (f32, f32),
    pub shake: f32,
    pub player: &'a Player,
    pub monsters: &'a [Monster],
    pub texts: &'a [DmgText],
    pub gold: &'a [(usize, usize)],
    pub potions: &'a [(usize, usize)],
    pub spikes: &'a [(usize, usize)],
    pub poison: &'a [(usize, usize)],
    pub equipment_drops: &'a [(usize, usize, EquipmentType)],
    pub score: i32,
    pub floor: i32,
    pub difficulty: Difficulty,
    pub in_shop: bool,
    pub shop_items: &'a [ShopItem],
}

pub fn render_game(data: &GameRenderData) {
    let cam_with_shake = apply_screen_shake(data.cam, data.shake);

    render_map(data, cam_with_shake);
    render_equipment_drops(data, cam_with_shake);
    render_path(data, cam_with_shake);
    render_player(data, cam_with_shake);
    render_monsters(data, cam_with_shake);
    render_floating_texts(data.texts);
    render_hud(data);
    render_minimap(data);

    if data.in_shop {
        render_shop(data.shop_items, data.score);
    }
}

fn apply_screen_shake(cam: (f32, f32), shake: f32) -> (f32, f32) {
    if shake > 0. {
        (
            cam.0 + shake * 10.0 * (get_time() * 50.0).sin() as f32,
            cam.1 + shake * 10.0 * (get_time() * 60.0).cos() as f32,
        )
    } else {
        cam
    }
}

fn render_map(data: &GameRenderData, cam: (f32, f32)) {
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            if !data.explored[y][x] {
                continue;
            }

            if data.map[y][x] == Tile::Wall {
                draw_wall(x, y, cam);
            } else {
                let (sx, sy) = to_screen(x, y, cam);
                render_tile_content(data, x, y, sx, sy);
            }
        }
    }
}

fn render_tile_content(data: &GameRenderData, x: usize, y: usize, sx: f32, sy: f32) {
    if data.gold.contains(&(x, y)) {
        draw_circle(sx, sy + 16., 6., GOLD);
    } else if data.potions.contains(&(x, y)) {
        draw_circle(sx, sy + 12., 8., RED);
        draw_line(sx - 4., sy + 12., sx + 4., sy + 12., 2., WHITE);
        draw_line(sx, sy + 8., sx, sy + 16., 2., WHITE);
    } else if data.spikes.contains(&(x, y)) {
        draw_triangle(
            Vec2::new(sx - 5., sy + 18.),
            Vec2::new(sx, sy + 8.),
            Vec2::new(sx + 5., sy + 18.),
            DARKGRAY,
        );
    } else if data.poison.contains(&(x, y)) {
        draw_ellipse(sx, sy + 16., 8., 4., 0., Color::new(0.2, 0.8, 0.2, 0.7));
    } else {
        draw_circle(sx, sy + 16., 2., LIGHTGRAY);
    }
}

fn render_equipment_drops(data: &GameRenderData, cam: (f32, f32)) {
    for (ex, ey, eq_type) in data.equipment_drops {
        if data.explored[*ey][*ex] {
            let (sx, sy) = to_screen(*ex, *ey, cam);
            let color = match eq_type {
                EquipmentType::Sword => ORANGE,
                EquipmentType::Shield => SKYBLUE,
                EquipmentType::Ring => PINK,
            };
            draw_circle(sx, sy + 12., 8., color);
            draw_circle(sx, sy + 12., 5., WHITE);
        }
    }
}

fn render_path(data: &GameRenderData, cam: (f32, f32)) {
    for (px, py) in &data.player.path {
        if data.explored[*py][*px] {
            let (sx, sy) = to_screen(*px, *py, cam);
            draw_circle(sx, sy + 16., 4., GOLD);
        }
    }
}

fn render_player(data: &GameRenderData, cam: (f32, f32)) {
    draw_stickman(data.player.x, data.player.y, cam, false);
}

fn render_monsters(data: &GameRenderData, cam: (f32, f32)) {
    for m in data.monsters {
        if !data.explored[m.y][m.x] {
            continue;
        }
        draw_stickman_typed(m.x, m.y, cam, true, Some(m.monster_type));
        render_monster_health_bar(m, cam, data.difficulty);
    }
}

fn render_monster_health_bar(m: &Monster, cam: (f32, f32), difficulty: Difficulty) {
    let (sx, sy) = to_screen(m.x, m.y, cam);
    let bar_width = 24.;
    let bar_height = 4.;
    let max_hp = get_monster_max_hp(m.monster_type) * difficulty.monster_hp_mult();
    let hp_ratio = m.hp as f32 / max_hp;

    draw_rectangle(
        sx - bar_width / 2.,
        sy - 45.,
        bar_width,
        bar_height,
        DARKGRAY,
    );
    draw_rectangle(
        sx - bar_width / 2.,
        sy - 45.,
        bar_width * hp_ratio,
        bar_height,
        GREEN,
    );
}

fn get_monster_max_hp(monster_type: MonsterType) -> f32 {
    match monster_type {
        MonsterType::Fast => (MONSTER_HP / 2) as f32,
        MonsterType::Tank => (MONSTER_HP * 2) as f32,
        MonsterType::Normal => MONSTER_HP as f32,
        MonsterType::Boss => (MONSTER_HP * 5) as f32,
    }
}

fn render_floating_texts(texts: &[DmgText]) {
    for t in texts {
        let (text, color) = if t.dmg < 0 {
            (format!("+{}", -t.dmg), GREEN)
        } else {
            (format!("-{}", t.dmg), RED)
        };
        draw_text(&text, t.x, t.y, 20., color);
    }
}

fn render_hud(data: &GameRenderData) {
    render_hp_bar(data.player);
    render_score_floor(data.score, data.floor);
    render_difficulty(data.difficulty);
    render_ability_cooldowns(data.player);
    render_level_xp(data.player);
    render_equipment_stats(data.player);
    render_poison_indicator(data.player);
}

fn render_hp_bar(player: &Player) {
    let hp_bar_width = 200.;
    let hp_bar_height = 20.;
    let hp_ratio = player.hp as f32 / player.max_hp as f32;

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
        &format!("{}/{}", player.hp, player.max_hp),
        25.,
        screen_height() - 35.,
        18.,
        WHITE,
    );
}

fn render_score_floor(score: i32, floor: i32) {
    draw_text(
        &format!("SCORE: {}  FLOOR: {}", score, floor),
        20.,
        screen_height() - 70.,
        24.,
        BLACK,
    );
}

fn render_difficulty(difficulty: Difficulty) {
    draw_text(
        &format!("[{}]", difficulty.name()),
        screen_width() - 100.,
        30.,
        20.,
        DARKGRAY,
    );
}

fn render_ability_cooldowns(player: &Player) {
    let abilities = [
        ("DASH [SPACE]", player.dash_cd),
        ("AREA [Q]", player.area_cd),
        ("HEAL [E]", player.heal_cd),
        ("RANGED [R]", player.ranged_cd),
    ];

    for (i, (name, cd)) in abilities.iter().enumerate() {
        let y = screen_height() - 90. - (i as f32 * 18.);
        let (text, color) = if *cd <= 0. {
            (format!("{} READY", name), GREEN)
        } else {
            (format!("{} [{:.1}s]", name, cd), GRAY)
        };
        draw_text(&text, 20., y, 16., color);
    }
}

fn render_level_xp(player: &Player) {
    draw_text(
        &format!("LVL {}", player.level),
        230.,
        screen_height() - 45.,
        20.,
        DARKBLUE,
    );

    let xp_bar_width = 80.;
    let xp_ratio = player.xp as f32 / player.xp_to_next as f32;
    draw_rectangle(230., screen_height() - 38., xp_bar_width, 8., DARKGRAY);
    draw_rectangle(
        230.,
        screen_height() - 38.,
        xp_bar_width * xp_ratio,
        8.,
        BLUE,
    );
}

fn render_equipment_stats(player: &Player) {
    if player.weapon_damage > 0 || player.armor > 0 {
        draw_text(
            &format!("DMG+{} ARM+{}", player.weapon_damage, player.armor),
            230.,
            screen_height() - 60.,
            16.,
            ORANGE,
        );
    }
}

fn render_poison_indicator(player: &Player) {
    if player.poisoned > 0. {
        draw_text(
            &format!("POISONED {:.1}s", player.poisoned),
            230.,
            screen_height() - 75.,
            16.,
            GREEN,
        );
    }
}

fn render_minimap(data: &GameRenderData) {
    let minimap_size = 100.;
    let minimap_x = screen_width() - minimap_size - 10.;
    let minimap_y = screen_height() - minimap_size - 10.;
    let tile_size = minimap_size / MAP_SIZE as f32;

    draw_rectangle(
        minimap_x,
        minimap_y,
        minimap_size,
        minimap_size,
        Color::new(0., 0., 0., 0.7),
    );

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

    for m in data.monsters {
        if data.explored[m.y][m.x] {
            let mx = minimap_x + m.x as f32 * tile_size;
            let my = minimap_y + m.y as f32 * tile_size;
            draw_rectangle(mx, my, tile_size, tile_size, PURPLE);
        }
    }

    let px = minimap_x + data.player.x as f32 * tile_size;
    let py = minimap_y + data.player.y as f32 * tile_size;
    draw_rectangle(px, py, tile_size * 1.5, tile_size * 1.5, BLUE);

    draw_rectangle_lines(minimap_x, minimap_y, minimap_size, minimap_size, 2., WHITE);
}

fn render_shop(items: &[ShopItem], gold: i32) {
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
