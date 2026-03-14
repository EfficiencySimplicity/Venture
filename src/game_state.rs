use crate::house::*;

// https://doc.rust-lang.org/book/ch10-02-traits.html
// Possibly pass observation string to this thing. Would be
// helpful for creating long commands and clean code
pub trait Observable {
    fn summarize(&self, game_state: &GameState) -> String;
}

pub enum GameStateName {
    House,
}

pub struct GameState {
    pub house: House,
    pub current_location: GameStateName
}

impl GameState {
    pub fn new() -> Self {
        GameState { 
            house: House::new(),
            current_location: GameStateName::House}
    }

    pub fn summarize(&self) -> String {
        self.get_current_location().summarize(&self)
    }

    pub fn get_current_location(&self) -> &impl Observable {
        match self.current_location {
            GameStateName::House => &self.house
        }
    }
}