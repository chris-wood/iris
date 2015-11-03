use std::vec;
use common::name::Name as Name;

struct CacheEntry {
    name: Name,
    keyIdRestriction: Vec<u8>,
    hashRestriction: Vec<u8>,
    data: Vec<u8>
}

pub struct Cache {
    size: usize,
    entries: Vec<CacheEntry>
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

impl Cache {
    pub fn new(new_size: usize) -> Cache {
        Cache {
            size: new_size,
            entries: Vec::new()
        }
    }

    pub fn lookup(&mut self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>) -> Option<&CacheEntry> {
        for entry in self.entries.iter_mut() {
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

    fn evict(&mut self, length: usize) -> (bool) {
        return true;
    }

    pub fn insert(&mut self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>, data: &Vec<u8>) -> (bool) {
        let length = key_id_restr.len() + hash_restr.len() + data.len();
        if length >= self.size {
            self.evict(length);
        }

        let new_name = target.clone();
        let mut entry = CacheEntry {
            name: new_name,
            keyIdRestriction: key_id_restr.clone(),
            hashRestriction: hash_restr.clone(),
            data: data.clone()
        };
        self.entries.push(entry);

        return true;
    }
}
