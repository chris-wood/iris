use std::vec;
use common::name::Name as Name;
use core::packet as Packet;
use core::packet::message::Message as Message;

#[derive(Debug)]
pub struct CacheEntry {
    name: Name,
    key_id_restriction: Vec<u8>,
    content_id_restriction: Vec<u8>,
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

    pub fn lookup(&self, target: &Message) -> Option<&CacheEntry> {
        // Extract the request key parameters
        let name = target.get_name();
        let key_id = target.get_key_id_overlay();
        let content_id = target.get_key_id_overlay();

        for entry in self.entries.iter() {
            let mut is_match = true;
            if !entry.name.equals(&name) {
                is_match = false;
            }

            // TODO: move to funcion.
            if is_match {
                match key_id {
                    Some ((o, l)) => {
                        let length = entry.key_id_restriction.len();
                        if l == length {
                            let mut index = 0;
                            while (index < l) {
                                if (entry.key_id_restriction[index] != target.byte_at(o + index)) {
                                    is_match = false;
                                    break;
                                }
                                index = index + 1;
                            }
                        } else {
                            is_match = false;
                        }
                    }, None => {}
                }
            }

            if is_match {
                match content_id {
                    Some ((o, l)) => {
                        let length = entry.content_id_restriction.len();
                        if l == length {
                            let mut index = 0;
                            while (index < l) {
                                if (entry.content_id_restriction[index] != target.byte_at(o + index)) {
                                    is_match = false;
                                    break;
                                }
                                index = index + 1;
                            }
                        } else {
                            is_match = false;
                        }
                    }, None => {}
                }
            }

            if is_match {
                return Some(entry);
            }
        }

        return None;
    }

    fn evict(&mut self, length: usize) -> (bool) {
        let length = self.entries.len();
        if (length > 1) {
            self.entries.swap_remove(0);
        }
        return true;
    }

    pub fn insert(&mut self, target: &Message) -> (bool) {
        let length = target.size();
        if length >= self.size {
            self.evict(length);
        }

        let bytes = target.bytes();
        let new_name = target.get_name();

        let mut key_id = Vec::new();
        match target.get_key_id_overlay() {
            Some ((o, l)) => {
                let mut index = o;
                while (index < l) {
                    key_id.push(bytes[index]);
                    index = index + 1;
                }
            }, None => {}
        }

        let mut content_id = Vec::new();
        match target.get_key_id_overlay() {
            Some ((o, l)) => {
                let mut index = o;
                while (index < l) {
                    content_id.push(bytes[index]);
                    index = index + 1;
                }
            }, None => {}
        }

        let mut entry = CacheEntry {
            name: new_name,
            key_id_restriction: key_id,
            content_id_restriction: content_id,
            data: bytes
        };
        self.entries.push(entry);
        self.size = self.size + length;

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

    let msg = Packet::decode_packet(buffer);

    let mut cache = Cache::new(1);
    let result = cache.insert(&msg);
    assert!(result);

    // TODO: assert that the size increased by the length of the message and keyId/ContentId
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

    let msg = Packet::decode_packet(buffer);

    let mut cache = Cache::new(1);
    let result = cache.insert(&msg);
    assert!(result);

    let lookup_result = cache.lookup(&msg);
    match lookup_result {
        Some(entry) => {},
        None => assert!(false)
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

    let msg = Packet::decode_packet(buffer);

    let mut cache = Cache::new(1);
    let result = cache.insert(&msg);
    assert!(result);

    let evict_result = cache.evict(100);
    match evict_result {
        true => {},
        false => assert!(false)
    }
}
