use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

// Embed audio files directly into the binary
const HIT_WAV: &[u8] = include_bytes!("../../assets/hit.wav");
const GOLD_WAV: &[u8] = include_bytes!("../../assets/gold.wav");
const DEATH_WAV: &[u8] = include_bytes!("../../assets/death.wav");
const YEAY_WAV: &[u8] = include_bytes!("../../assets/yeay.wav");
const GAME_OVER_WAV: &[u8] = include_bytes!("../../assets/game-over.wav");

pub struct GameAudio {
    pub hit_sound: Option<Sound>,
    pub gold_sound: Option<Sound>,
    pub death_sound: Option<Sound>,
    pub level_sound: Option<Sound>,
    pub game_over_sound: Option<Sound>,
}

impl GameAudio {
    pub async fn new() -> Self {
        // Load sounds from embedded bytes
        let hit_sound = load_sound_from_bytes(HIT_WAV).await.ok();
        let gold_sound = load_sound_from_bytes(GOLD_WAV).await.ok();
        let death_sound = load_sound_from_bytes(DEATH_WAV).await.ok();
        let level_sound = load_sound_from_bytes(YEAY_WAV).await.ok();
        let game_over_sound = load_sound_from_bytes(GAME_OVER_WAV).await.ok();

        GameAudio {
            hit_sound,
            gold_sound,
            death_sound,
            level_sound,
            game_over_sound,
        }
    }

    pub fn play_hit(&self) {
        if let Some(sound) = &self.hit_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.5,
                },
            );
        }
    }

    pub fn play_gold(&self) {
        if let Some(sound) = &self.gold_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.6,
                },
            );
        }
    }

    pub fn play_death(&self) {
        if let Some(sound) = &self.death_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.7,
                },
            );
        }
    }

    pub fn play_level_complete(&self) {
        if let Some(sound) = &self.level_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.8,
                },
            );
        }
    }

    pub fn play_game_over(&self) {
        if let Some(sound) = &self.game_over_sound {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.8,
                },
            );
        }
    }
}
