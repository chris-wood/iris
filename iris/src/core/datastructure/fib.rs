use std::vec;
use common::name::Name as Name;
use core::link::Link as Link;

pub struct FIBEntry {
    name: Name,
    // faces: Vec<Box<Link>>
    pub faces: Vec<u16>
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

    pub fn lookup(&self, target: &Name) -> Option<&FIBEntry> {
        for entry in self.entries.iter() {
            if entry.name.equals(&target) {
                return Some(entry);
            }
        }
        return None;
    }

    pub fn insert(&mut self, target: Name, newFace: u16) -> (bool) {
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
        self.entries.push(entry);

        return true;
    }
}
