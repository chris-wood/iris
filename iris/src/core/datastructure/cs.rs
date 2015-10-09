use std::vec;
use common::name::Name as Name;

struct CacheEntry {
    name: Name,
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

    pub fn lookup(target: Name) -> (bool) { // TODO: should return data element and bool
        return false;
    }

    pub fn insert(target: Name) -> (bool) { // TODO: should add data element to be inserted
        return false;
    }
}
