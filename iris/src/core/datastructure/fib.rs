use std::vec;
use common::name::Name as Name;
use core::link::Link as Link;

pub struct FIBEntry {
    name: Name,
    faces: Vec<Box<Link>>
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

    pub fn lookup(target: Name) -> (bool) { // TODO: should return FIBEntry and bool
        return false;
    }

    pub fn insert(&mut self, target: Name, newFace: Box<Link>) -> (bool) {
        for entry in self.entries.iter_mut() {
            if entry.name.equals(&target) {
                entry.faces.push(newFace);
                return true;
            }
        }

        let new_name = target.clone();
        let mut entry = FIBEntry {
            name: new_name,
            faces: Vec::new()
        };
        entry.faces.push(newFace);

        return true;
    }
}
