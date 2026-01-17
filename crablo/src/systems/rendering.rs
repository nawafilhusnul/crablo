use macroquad::prelude::*;

use crate::core::constants::{MAP_SIZE, TILE_HEIGHT, TILE_WIDTH};

pub fn to_screen(x: usize, y: usize, cam: (f32, f32)) -> (f32, f32) {
    (
        (x as f32 - y as f32) * TILE_WIDTH + cam.0,
        (x as f32 + y as f32) * TILE_HEIGHT + cam.1,
    )
}

pub fn to_tile(sx: f32, sy: f32, cam: (f32, f32)) -> Option<(usize, usize)> {
    let (ax, ay) = (sx - cam.0, sy - cam.1);

    let tx = (ax / TILE_WIDTH + ay / TILE_HEIGHT) / 2.;
    let ty = (ay / TILE_HEIGHT - ax / TILE_WIDTH) / 2.;

    if tx >= 0. && ty >= 0. && (tx as usize) < MAP_SIZE && (ty as usize) < MAP_SIZE {
        Some((tx as usize, ty as usize))
    } else {
        None
    }
}

use crate::world::entities::MonsterType;

pub fn draw_stickman(x: usize, y: usize, cam: (f32, f32), enemy: bool) {
    draw_stickman_typed(x, y, cam, enemy, None);
}

pub fn draw_stickman_typed(
    x: usize,
    y: usize,
    cam: (f32, f32),
    enemy: bool,
    monster_type: Option<MonsterType>,
) {
    let (sx, mut sy) = to_screen(x, y, cam);

    sy += 16.;

    // Determine color based on monster type
    let color = if !enemy {
        BLACK
    } else {
        match monster_type {
            Some(MonsterType::Fast) => BLUE,
            Some(MonsterType::Tank) => DARKPURPLE,
            Some(MonsterType::Boss) => MAROON,
            _ => BLACK,
        }
    };

    // Scale for tank and boss monsters
    let scale = match monster_type {
        Some(MonsterType::Tank) => 1.3,
        Some(MonsterType::Boss) => 1.8,
        _ => 1.0,
    };

    // shadow
    draw_ellipse(
        sx,
        sy + 3.,
        10. * scale,
        5. * scale,
        0.,
        Color::new(0., 0., 0., 0.2),
    );

    // head
    if enemy {
        draw_line(
            sx - 5. * scale,
            sy - 32. * scale,
            sx,
            sy - 30. * scale,
            2.,
            color,
        );
        draw_line(
            sx + 5. * scale,
            sy - 32. * scale,
            sx,
            sy - 30. * scale,
            2.,
            color,
        );
    } else {
        draw_circle_lines(sx, sy - 32., 7., 2., color);
    }

    // body and limbs
    for l in [
        [0., -25., 0., -8.],
        [0., -20., -8., -15.],
        [0., -20., 8., -15.],
        [0., -8., 6., 0.],
        [0., -8., 6., 0.],
    ] {
        draw_line(
            sx + l[0] * scale,
            sy + l[1] * scale,
            sx + l[2] * scale,
            sy + l[3] * scale,
            2.,
            color,
        );
    }
}

pub fn draw_wall(x: usize, y: usize, cam: (f32, f32)) {
    let (sx, sy) = to_screen(x, y, cam);

    let v = [
        vec2(sx, sy - 40.),
        vec2(sx + 32., sy - 24.),
        vec2(sx, sy - 8.),
        vec2(sx - 32., sy - 24.),
        vec2(sx + 32., sy),
        vec2(sx, sy + 16.),
        vec2(sx - 32., sy),
    ];

    let colors = [
        Color::new(0.8, 0.8, 0.8, 1.),
        Color::new(0.5, 0.5, 0.5, 1.),
        Color::new(0.6, 0.6, 0.6, 1.),
    ];

    // draw faces
    draw_triangle(v[0], v[1], v[2], colors[0]);
    draw_triangle(v[0], v[2], v[3], colors[0]);
    draw_triangle(v[1], v[4], v[5], colors[1]);
    draw_triangle(v[1], v[5], v[2], colors[1]);
    draw_triangle(v[3], v[2], v[5], colors[2]);
    draw_triangle(v[3], v[5], v[6], colors[2]);

    // draw outline
    for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0), (1, 4), (2, 5), (3, 6)] {
        draw_line(v[a].x, v[a].y, v[b].x, v[b].y, 1., BLACK);
    }
}
