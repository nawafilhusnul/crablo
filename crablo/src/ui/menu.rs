use crate::core::constants::Difficulty;
use crate::core::database::ScoreEntry;
use macroquad::prelude::*;

pub fn draw_menu(selected_difficulty: Difficulty, has_save: bool) {
    draw_text("CRABLO", screen_width() / 2. - 80., 100., 60., DARKPURPLE);

    draw_text(
        "Select Difficulty:",
        screen_width() / 2. - 100.,
        180.,
        28.,
        BLACK,
    );

    let difficulties = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
    for (i, diff) in difficulties.iter().enumerate() {
        let y = 220. + (i as f32 * 40.);
        let color = if *diff == selected_difficulty {
            GOLD
        } else {
            DARKGRAY
        };
        let prefix = if *diff == selected_difficulty {
            "> "
        } else {
            "  "
        };
        draw_text(
            &format!("{}{}", prefix, diff.name()),
            screen_width() / 2. - 60.,
            y,
            30.,
            color,
        );
    }

    draw_text(
        "Use UP/DOWN to select, ENTER to start",
        screen_width() / 2. - 180.,
        380.,
        20.,
        GRAY,
    );

    draw_text(
        "Press H for Hall of Fame",
        screen_width() / 2. - 110.,
        410.,
        20.,
        GRAY,
    );

    if has_save {
        draw_text(
            "Press C to CONTINUE saved game",
            screen_width() / 2. - 140.,
            440.,
            20.,
            GREEN,
        );
    }
}

pub fn draw_pause() {
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(0., 0., 0., 0.7),
    );

    draw_text(
        "PAUSED",
        screen_width() / 2. - 80.,
        screen_height() / 2. - 50.,
        60.,
        WHITE,
    );

    draw_text(
        "Press ESC or P to resume",
        screen_width() / 2. - 120.,
        screen_height() / 2. + 20.,
        24.,
        LIGHTGRAY,
    );

    draw_text(
        "Press Q to quit to menu",
        screen_width() / 2. - 110.,
        screen_height() / 2. + 50.,
        24.,
        LIGHTGRAY,
    );

    draw_text(
        "Press S to SAVE and quit",
        screen_width() / 2. - 115.,
        screen_height() / 2. + 80.,
        24.,
        GREEN,
    );
}

pub fn draw_game_over(score: i32, floor: i32) {
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(1., 1., 1., 0.7),
    );

    draw_text(
        "GAME OVER",
        screen_width() / 2. - 120.,
        screen_height() / 2. - 50.,
        50.,
        RED,
    );

    draw_text(
        &format!("Score: {} | Floor: {}", score, floor),
        screen_width() / 2. - 100.,
        screen_height() / 2.,
        24.,
        BLACK,
    );

    draw_text(
        "Press ENTER to save score",
        screen_width() / 2. - 110.,
        screen_height() / 2. + 40.,
        20.,
        DARKGRAY,
    );

    draw_text(
        "Press H for Hall of Fame",
        screen_width() / 2. - 100.,
        screen_height() / 2. + 70.,
        20.,
        DARKGRAY,
    );
}

pub fn draw_enter_name(player_name: &str) {
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(0.9, 0.9, 0.9, 1.),
    );

    draw_text(
        "Enter your name:",
        screen_width() / 2. - 100.,
        screen_height() / 2. - 50.,
        30.,
        BLACK,
    );

    draw_text(
        &format!("{}_", player_name),
        screen_width() / 2. - 80.,
        screen_height() / 2.,
        40.,
        DARKBLUE,
    );

    draw_text(
        "Press ENTER to confirm",
        screen_width() / 2. - 100.,
        screen_height() / 2. + 50.,
        20.,
        GRAY,
    );
}

pub fn draw_hall_of_fame(scores: &[ScoreEntry]) {
    draw_rectangle(
        0.,
        0.,
        screen_width(),
        screen_height(),
        Color::new(0.1, 0.1, 0.2, 1.),
    );

    draw_text("HALL OF FAME", screen_width() / 2. - 120., 80., 50., GOLD);

    for (i, entry) in scores.iter().enumerate() {
        let y = 140. + (i as f32 * 35.);
        let color = match i {
            0 => GOLD,
            1 => LIGHTGRAY,
            2 => ORANGE,
            _ => WHITE,
        };

        draw_text(
            &format!("{}. {} - {}", entry.rank, entry.name, entry.score),
            screen_width() / 2. - 150.,
            y,
            28.,
            color,
        );
    }

    draw_text(
        "Press ENTER to return to menu",
        screen_width() / 2. - 140.,
        screen_height() - 70.,
        20.,
        GRAY,
    );

    draw_text(
        "Press R to RESET all scores",
        screen_width() / 2. - 130.,
        screen_height() - 40.,
        20.,
        RED,
    );
}
