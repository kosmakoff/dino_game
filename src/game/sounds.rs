use std::io::Cursor;

macro_rules! load_sound {
    ($path:literal) => {{
        let data = include_bytes!($path);
        Cursor::new(data.as_ref())
    }};
}

pub enum Sound {
    ButtonPress,
    Hit,
    ScoreReached,
}

pub struct Sounds {
    pub button_press: Cursor<&'static [u8]>,
    pub hit: Cursor<&'static [u8]>,
    pub score_reached: Cursor<&'static [u8]>,
}

impl Sounds {
    pub fn new() -> Self {
        Sounds {
            button_press: load_sound!("../../assets/sounds/button-press.ogg"),
            hit: load_sound!("../../assets/sounds/hit.ogg"),
            score_reached: load_sound!("../../assets/sounds/score-reached.ogg"),
        }
    }
}
