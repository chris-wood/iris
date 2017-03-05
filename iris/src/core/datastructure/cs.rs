use std::vec;
use common::name::Name as Name;
use common::identifier::Identifier as Identifier;
use core::packet::Packet as Packet;

#[derive(Debug)]
pub struct CacheEntry {
    identifier: Identifier,
    packet: Packet,
}

impl CacheEntry {
    pub fn build_Packet(&self) -> Packet {
        return self.packet.clone();
    }
}

#[derive(Debug)]
pub struct Cache {
    size: usize,
    entries: Vec<CacheEntry>
}

impl Cache {
    pub fn new(new_size: usize) -> Cache {
        Cache {
            size: new_size,
            entries: Vec::new()
        }
    }

    pub fn lookup(&self, target: &Packet) -> Option<&CacheEntry> {
        for entry in self.entries.iter() {
            if entry.identifier.equals(&target.identifier) {
                return Some(entry);
            }
        }
        return None;
    }

    fn evict(&mut self, length: usize) -> (bool) {
        let length = self.entries.len();
        if length > 1 {
            self.entries.swap_remove(0);
        }
        return true;
    }

    pub fn insert(&mut self, target: &Packet) -> (bool) {
        let length = target.size();
        if length >= self.size {
            self.evict(length);
        }

        let mut entry = CacheEntry {
            identifier: target.identifier.clone(),
            packet: target.clone()
        };

        self.entries.push(entry);
        self.size = self.size + target.size();

        return true;
    }
}

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[test]
fn test_cache_insert() {
    let path = Path::new("../data/packet1_interest.bin");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_contents = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }
    let buffer = &file_contents[..];

    match Packet::decode(buffer, 0) {
        Err(e) => assert!(false),
        Ok(msg) => {
            let mut cache = Cache::new(1);
            let result = cache.insert(&msg);
            assert!(result);
        }
    }
    // TODO: assert that the size increased by the length of the Packet and keyId/ContentId
}

#[test]
fn test_cache_lookup() {
    let cache = Cache::new(1);

    let path = Path::new("../data/packet1_interest.bin");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_contents = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }
    let buffer = &file_contents[..];

    match Packet::decode(buffer, 0) {
        Err(e) => assert!(false),
        Ok(msg) => {
            let mut cache = Cache::new(1);
            let result = cache.insert(&msg);
            assert!(result);

            let lookup_result = cache.lookup(&msg);
            match lookup_result {
                Some(entry) => {},
                None => assert!(false)
            }
        }
    }

}

#[test]
fn test_cache_evict() {
    let cache = Cache::new(1);

    let path = Path::new("../data/packet1_interest.bin");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut file_contents = Vec::new();
    match file.read_to_end(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => {}
    }
    let buffer = &file_contents[..];

    match Packet::decode(buffer, 0) {
        Err(e) => assert!(false),
        Ok(msg) => {
            let mut cache = Cache::new(1);
            let result = cache.insert(&msg);
            assert!(result);

            let evict_result = cache.evict(100);
            match evict_result {
                true => {},
                false => assert!(false)
            }
        }
    }
}
