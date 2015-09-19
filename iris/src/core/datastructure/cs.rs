use std::vec;
use common::name as name;

struct CacheEntry {
    name: name::Name,
    // TODO: data
}

pub struct Cache {
    entries: Vec<CacheEntry>
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            entries: Vec::new()
        }
    }

    pub fn lookup(target: name::Name) -> (bool) { // TODO: should return data element and bool
        return false;
    }

    pub fn insert(target: name::Name) -> (bool) { // TODO: should add data element to be inserted
        return false;
    }
}
