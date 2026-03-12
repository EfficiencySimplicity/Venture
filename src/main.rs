mod game_state;
mod house;
use game_state::*;

fn main() {
    let game = GameState::new();
    println!("{}", game.summarize());
}