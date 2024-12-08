use ggez::audio;
use ggez::audio::SoundSource;
use ggez::{Context, GameResult};

pub struct SoundTracks {
    pub tracks: [GameResult<audio::Source>; 1],
}

#[derive(Copy, Clone, Debug)]
pub enum SoundTypes {
    LRClick,
}

impl SoundTracks {
    pub fn new(ctx: &mut Context) -> Self {
        let idx_0 = audio::Source::new(ctx, "/left_right_movement_click.wav");
        println!("{:?}", idx_0);
        Self {
            tracks: [
                idx_0,
            ],
        }
    }
}

pub fn play_sound_track(ctx: &mut Context, sound_track: &mut GameResult<audio::Source>) -> bool {
    match sound_track {
        Ok(audio_src) => {
            match audio_src.play(ctx) {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}

