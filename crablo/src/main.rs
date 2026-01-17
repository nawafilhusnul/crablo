mod core;
mod systems;
mod world;

use macroquad::prelude::*;

use core::constants::Difficulty;
use core::{Database, Game};
use systems::GameAudio;

enum AppState {
    Menu,
    Playing,
    Paused,
    GameOver,
    EnterName,
    HallOfFame,
}

#[macroquad::main("Crablo")]
async fn main() {
    let audio = GameAudio::new().await;
    let db = Database::new().expect("Failed to initialize database");
    let mut game = Game::new();
    let mut player_name = String::new();
    let mut selected_difficulty = Difficulty::Normal;
    let mut state = AppState::Menu;

    loop {
        clear_background(WHITE);

        match state {
            AppState::Menu => {
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

                // Show continue option if save exists
                if db.has_save().unwrap_or(false) {
                    draw_text(
                        "Press C to CONTINUE saved game",
                        screen_width() / 2. - 140.,
                        440.,
                        20.,
                        GREEN,
                    );
                }

                // Handle difficulty selection
                if is_key_pressed(KeyCode::Up) {
                    selected_difficulty = match selected_difficulty {
                        Difficulty::Normal => Difficulty::Easy,
                        Difficulty::Hard => Difficulty::Normal,
                        Difficulty::Easy => Difficulty::Easy,
                    };
                }
                if is_key_pressed(KeyCode::Down) {
                    selected_difficulty = match selected_difficulty {
                        Difficulty::Easy => Difficulty::Normal,
                        Difficulty::Normal => Difficulty::Hard,
                        Difficulty::Hard => Difficulty::Hard,
                    };
                }

                if is_key_pressed(KeyCode::Enter) {
                    let _ = db.delete_save(); // Delete old save when starting new game
                    game = Game::with_difficulty(selected_difficulty);
                    state = AppState::Playing;
                }
                if is_key_pressed(KeyCode::H) {
                    state = AppState::HallOfFame;
                }
                if is_key_pressed(KeyCode::C) {
                    if let Ok(Some(save)) = db.load_game() {
                        game = Game::from_save(&save);
                        state = AppState::Playing;
                    }
                }
            }
            AppState::Playing => {
                // Handle shop input if in shop
                game.handle_shop_input();

                // Check for pause (not while in shop)
                if !game.in_shop && (is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P))
                {
                    state = AppState::Paused;
                } else {
                    let events = game.update(get_frame_time());

                    // Play sounds based on events
                    if events.player_hit {
                        audio.play_hit();
                    }
                    if events.gold_collected {
                        audio.play_gold();
                    }
                    if events.monster_killed {
                        audio.play_death();
                    }

                    if events.game_over {
                        state = AppState::GameOver;
                    }
                }

                game.draw();
            }
            AppState::Paused => {
                game.draw();
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

                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::P) {
                    state = AppState::Playing;
                }
                if is_key_pressed(KeyCode::Q) {
                    state = AppState::Menu;
                }
                if is_key_pressed(KeyCode::S) {
                    let _ = db.save_game(
                        game.floor,
                        game.player.hp,
                        game.player.max_hp,
                        game.score,
                        game.get_difficulty_id(),
                    );
                    state = AppState::Menu;
                }
            }
            AppState::GameOver => {
                game.draw();
                draw_rectangle(
                    0.,
                    0.,
                    screen_width(),
                    screen_height(),
                    Color::new(1., 1., 1., 0.7),
                );

                let (msg, col) = if game.player.hp > 0 {
                    ("VICTORY", GOLD)
                } else {
                    ("GAME OVER", RED)
                };

                draw_text(
                    msg,
                    screen_width() / 2. - 100.,
                    screen_height() / 2.,
                    60.,
                    col,
                );

                draw_text(
                    &format!("Final Score: {}", game.score),
                    screen_width() / 2. - 80.,
                    screen_height() / 2. + 50.,
                    30.,
                    BLACK,
                );

                draw_text(
                    "Press ENTER to save score",
                    screen_width() / 2. - 100.,
                    screen_height() / 2. + 90.,
                    20.,
                    GRAY,
                );

                draw_text(
                    "Press H for Hall of Fame",
                    screen_width() / 2. - 100.,
                    screen_height() / 2. + 115.,
                    20.,
                    GRAY,
                );

                if is_key_pressed(KeyCode::Enter) {
                    player_name.clear();
                    state = AppState::EnterName;
                }
                if is_key_pressed(KeyCode::H) {
                    state = AppState::HallOfFame;
                }
            }
            AppState::EnterName => {
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

                // Handle text input
                if let Some(c) = get_char_pressed() {
                    if c.is_alphanumeric() && player_name.len() < 12 {
                        player_name.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) && !player_name.is_empty() {
                    player_name.pop();
                }

                if is_key_pressed(KeyCode::Enter) && !player_name.is_empty() {
                    let _ = db.save_score(&player_name, game.score);
                    state = AppState::HallOfFame;
                }
            }
            AppState::HallOfFame => {
                draw_rectangle(
                    0.,
                    0.,
                    screen_width(),
                    screen_height(),
                    Color::new(0.1, 0.1, 0.2, 1.),
                );

                draw_text("HALL OF FAME", screen_width() / 2. - 120., 80., 50., GOLD);

                if let Ok(scores) = db.get_top_scores(10) {
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

                if is_key_pressed(KeyCode::Enter) {
                    state = AppState::Menu;
                }
                if is_key_pressed(KeyCode::R) {
                    let _ = db.reset_scores();
                }
            }
        }

        next_frame().await;
    }
}
