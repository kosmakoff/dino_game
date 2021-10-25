use ggez::{audio, Context, GameResult};

pub enum Sound {
    ButtonPress,
    Hit,
    ScoreReached,
}

pub struct Sounds {
    pub button_press: audio::Source,
    pub hit: audio::Source,
    pub score_reached: audio::Source,
}

impl Sounds {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Sounds {
            button_press: audio::Source::new(ctx, "/sounds/button-press.ogg")?,
            hit: audio::Source::new(ctx, "/sounds/hit.ogg")?,
            score_reached: audio::Source::new(ctx, "/sounds/score-reached.ogg")?,
        })
    }
}
