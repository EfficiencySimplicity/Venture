// mod map_print;
// use map_print::print_map;

mod locations;
use locations::*;

fn setup_locations() {
    let house = Location::new("House".to_string());
}

fn main() {
    setup_locations();
    // print_map();
}