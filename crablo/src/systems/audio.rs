use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

pub struct GameAudio {
    pub hit_sound: Option<Sound>,
    pub gold_sound: Option<Sound>,
    pub death_sound: Option<Sound>,
}

impl GameAudio {
    pub async fn new() -> Self {
        // Try to load sounds, but don't fail if they don't exist
        let hit_sound = load_sound("assets/hit.wav").await.ok();
        let gold_sound = load_sound("assets/gold.wav").await.ok();
        let death_sound = load_sound("assets/death.wav").await.ok();

        GameAudio {
            hit_sound,
            gold_sound,
            death_sound,
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
}
