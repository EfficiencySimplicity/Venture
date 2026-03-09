// Locations, Locations, Locations...
// These should be hierarchial; sub-locations within locations
// ad infinitum

use std::vec::Vec;

pub struct Location {
    name: String,
    sub_locations: Vec<Location>,
}

impl Location {
    pub fn new(name: String) -> Location {
        Location {
            name,
            sub_locations: Vec::new()
        }
    }
}