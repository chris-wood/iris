use std::vec;
use common::name::Name as Name;
use core::packet::message::Message as Message;

#[derive(Debug)]
pub struct CacheEntry {
    name: Name,
    keyIdRestriction: Vec<u8>,
    hashRestriction: Vec<u8>,
    data: Vec<u8>
}

#[derive(Debug)]
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

#[test]
fn test_compare_vectors() {
    let mut vec1: Vec<u8> = Vec::new();
    let mut vec2: Vec<u8> = Vec::new();

    for x in 0..128 {
        vec1.push(x);
        vec2.push(x);
    }

    assert!(compare_vectors(&vec1, &vec2));
}

impl Cache {
    pub fn new(new_size: usize) -> Cache {
        Cache {
            size: new_size,
            entries: Vec::new()
        }
    }

    pub fn dump_contents(&self) {
        println!("dump_contents() start.");
        for entry in self.entries.iter() {
            println!("entry {:?}", entry);
        }
        println!("dump_contents() done.");
    }

    // pub fn lookup(&self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>) -> Option<&CacheEntry> {
    pub fn lookup(&self, target: &Message) -> Option<&CacheEntry> {
        // for entry in self.entries.iter() {
        //     if entry.name.equals(&target) {
        //         if compare_vectors(&entry.keyIdRestriction, key_id_restr) {
        //             if compare_vectors(&entry.hashRestriction, hash_restr) {
        //                 return Some(entry);
        //             }
        //         }
        //     }
        // }

        for entry in self.entries.iter() {
            
        }

        return None;
    }

    fn evict(&mut self, length: usize) -> (bool) {
        return true;
    }

    // pub fn insert(&mut self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>, data: &Vec<u8>) -> (bool) {
    pub fn insert(&mut self, target: &Message) -> (bool) {
        // let length = key_id_restr.len() + hash_restr.len() + data.len();
        // if length >= self.size {
        //     self.evict(length);
        // }
        //
        // let new_name = target.clone();
        // let mut entry = CacheEntry {
        //     name: new_name,
        //     keyIdRestriction: key_id_restr.clone(),
        //     hashRestriction: hash_restr.clone(),
        //     data: data.clone()
        // };
        // self.entries.push(entry);

        return true;
    }
}

#[test]
fn test_cache_new() {
    let cs = Cache::new(1);

}

#[test]
fn test_cache_lookup() {

}

#[test]
fn test_cache_evict() {

}

#[test]
fn test_cache_insert() {

}

