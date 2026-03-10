mod locations;
use locations::*;

fn main() {
    let house = House::new();
    println!("{}", house.summarize());
}