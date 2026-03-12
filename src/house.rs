use crate::game_state::*;

pub struct House {
    // The fields here are anything within the house which has a state
    lamp: Lamp,
}

impl House {
    pub fn new() -> Self {
        House { lamp: Lamp::Intact }
    }
}

impl Observable for House {
    fn summarize(&self, game_state: &GameState) -> String {
        let mut observation = String::new();
        observation.push_str("You are in a log cabin.\n");
        observation.push_str("In the cabin is a bed, at the foot of which is a dresser, at the foot of which is a broken lamp.\n");
        observation.push_str("The cabin has not much else in the way of furniture,\n");
        observation.push_str("but there is a door and a window illuminating your pillow with the morning light");
        observation
    }
}

pub enum Lamp {
    Intact,// If you never see the lamp while in bed, is this needed?
    Broken {note_taken: bool}
}