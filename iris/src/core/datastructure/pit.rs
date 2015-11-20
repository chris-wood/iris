use std::vec;
// use std::io::Timer;
// use std::time::Duration;
use common::name::Name as Name;
use core::link::Link as Link;

pub struct PITEntry {
    name: Name,
    keyIdRestriction: Vec<u8>,
    hashRestriction: Vec<u8>,
    arrivalFaces: Vec<Box<Link>>,
    lifetime: u32, // number of epochs
}

pub struct PIT {
    entries: Vec<PITEntry>
}

fn compare_vectors(x: &Vec<u8>, y: &Vec<u8>) -> (bool) {
    if x.len() == y.len() {
        let mut index = 0;
        while index < x.len() {
            if x[index] != y[index] {
                return false;
            }
            index = index + 1;
        }
        return true;
    }
    return false;
}

impl PIT {
    pub fn new() -> PIT {
        PIT {
            entries: Vec::new()
        }
    }

    // TODO: make keyId and hashRest Optional

    pub fn lookup(&self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>) -> Option<&PITEntry> {
        for entry in self.entries.iter() {
            if entry.name.equals(&target) {
                if compare_vectors(&entry.keyIdRestriction, key_id_restr) {
                    if compare_vectors(&entry.hashRestriction, hash_restr) {
                        return Some(entry);
                    }
                }
            }
        }

        return None;
    }

    pub fn insert(&mut self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>, newFace: Box<Link>) -> (bool) {
        // TODO: replace this with a call to lookup, fixing the borrowed lifetime issue
        for entry in self.entries.iter_mut() {
            if entry.name.equals(&target) {
                if compare_vectors(&entry.keyIdRestriction, key_id_restr) {
                    if compare_vectors(&entry.hashRestriction, hash_restr) {
                        entry.arrivalFaces.push(newFace);
                        return true;
                    }
                }
            }
        }

        let new_name = target.clone();
        let mut entry = PITEntry {
            name: new_name,
            keyIdRestriction: key_id_restr.clone(),
            hashRestriction: hash_restr.clone(),
            arrivalFaces: Vec::new(),
            lifetime: 10 // TODO: completely arbitrary... make this a parameter
        };
        entry.arrivalFaces.push(newFace);
        self.entries.push(entry);

        return true;
    }
}
