use std::vec;
use common::name as name;

struct PITntry {
    name: name::Name,
    // TODO
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

    pub fn lookup(target: name::Name) -> (bool) { // TODO: should add PIT-related info to the list, and should return the PIT entry
        return false;
    }

    pub fn insert(target: name::Name) -> (bool) { // TODO: should add PIT-related info to the list
        return false;
    }
}
