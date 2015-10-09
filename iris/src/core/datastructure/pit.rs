use std::vec;
// use std::io::Timer;
// use std::time::Duration;
use common::name::Name as Name;
use core::link::Link as Link;

struct PITntry {
    name: Name,
    keyIdRestriction: Vec<u8>,
    hashRestriction: Vec<u8>,
    arrivalFaces: Vec<Box<Link>>,
    lifetime: u32, // number of epochs
}

pub struct PIT {
    entries: Vec<PITntry>
}

impl PIT {
    pub fn new() -> PIT {
        PIT {
            entries: Vec::new()
        }
    }

    pub fn lookup(target: Name) -> (bool) { // TODO: should add PIT-related info to the list, and should return the PIT entry
        return false;
    }

    pub fn insert(target: Name) -> (bool) { // TODO: should add PIT-related info to the list
        return false;
    }
}
