use std::vec;
use common::name as name;
use core::face as face;

pub struct FIBEntry {
    name: name::Name,
    faces: Vec<face::Face>
}

pub struct FIB {
    entries: Vec<FIBEntry>
}

impl FIB {
    pub fn new() -> FIB {
        FIB {
            entries: Vec::new()
        }
    }

    pub fn lookup(target: name::Name) -> (bool) { // TODO: should return FIBEntry and bool
        return false;
    }

    pub fn insert(target: name::Name, newFace: face::Face) -> (bool) {
        return false;
    }
}
