use crate::game::context::Context;
use crate::game::event::EventHandler;

pub struct WalkingState {
    since: u128,
    is_left_leg: bool,
}

pub struct CrouchedState {}

pub enum DinoState {
    Idle { since: u128 },
    Walking(WalkingState),
    Jumping,
    Crouched(CrouchedState),
    Hit,
}

impl DinoState {
    pub fn new() -> Self {
        DinoState::Idle { since: 0 }
    }
}

pub struct Dino {
    state: DinoState,
}

impl EventHandler for Dino {
    fn update(&mut self, context: &Context) {
        match &self.state {
            DinoState::Walking(walking) => {
                println!("Update walking");
                let dt = context.dt;
                println!("More update walking");
            }
            _ => (),
        }
    }
    fn draw(&mut self) {
        todo!()
    }
}

impl Dino {
    pub fn new() -> Self {
        Dino {
            state: DinoState::new(),
        }
    }
}
