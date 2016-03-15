use std::vec;

use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use common::name::Name as Name;
use core::packet::message::Message as Message;
use core::packet as Packet;

pub struct PITEntry {
    name: Name,
    keyIdRestriction: Vec<u8>,
    hashRestriction: Vec<u8>,
    arrival_faces: Vec<usize>, // make this mutable for its lifetime
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

    pub fn lookup(&self, target: &Message) -> Option<&PITEntry> {
        for entry in self.entries.iter() {
            let target_name = target.get_name();
            if entry.name.equals(&target_name) {
                // TODO: need to add missing checks for the key_id and content_id
                return Some(entry);
            }
        }

        return None;
    }

    fn lookup_mut(&mut self, target: &Message) -> Option<(&mut PITEntry, usize)> {
        let mut index: usize = 0;
        for entry in self.entries.iter_mut() {
            let target_name = target.get_name();
            if entry.name.equals(&target_name) {
                // TODO: need to add missing checks for the key_id and content_id
                return Some((entry, index));
            }
            index = index + 1;
        }

        return None;
    }

    // Can only be called by the owner! Oof!
    // pub fn insert(&mut self, target: &Name, key_id_restr: &Vec<u8>, hash_restr: &Vec<u8>, new_face: usize) -> (bool) {
    pub fn insert(&mut self, target: &Message, new_face: usize) -> (bool) {
        let mut new_entry: Option<PITEntry> = None;
        match self.lookup_mut(target) {
            Some((entry, index)) => {
                entry.arrival_faces.push(new_face);
                return true;
            },
            None => {
                let clone = target.clone();
                let entry = PITEntry {
                    name: clone.get_name(),
                    keyIdRestriction: vec![],
                    hashRestriction: vec![],
                    arrival_faces: vec![new_face],
                    lifetime: 10
                };
                new_entry = Some(entry);
            }
        }

        match new_entry {
            Some(entry) => {
                self.entries.push(entry);
                return true;
            },
            None => { }
        }
        return false;
    }

    pub fn flush(&mut self, target: &Message) -> (bool) {
        let mut target_index = 0;
        match self.lookup_mut(target) {
            Some((entry, index)) => {
                target_index = index;
            },
            None => {
                return false;
            }
        }

        self.entries.swap_remove(target_index);
        return true;
    }
}


#[test]
fn test_pit_insert() {
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

    // 0. Create the PIT
    let mut pit = PIT::new();

    // 1. decode the packet
    let msg = Packet::decode_packet(buffer);

    // 2. insert the interest
    let mut face = 5;
    let mut result = pit.insert(&msg, face);
    assert!(result == true);

    // 3. try to insert yet another interest from a different face
    face = 10;
    result = pit.insert(&msg, face);
    assert!(result == true);
}

#[test]
fn test_pit_lookup() {
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

    // 0. Create the PIT
    let mut pit = PIT::new();

    // 1. decode the packet
    let msg = Packet::decode_packet(buffer);

    // 2. insert the interest
    let mut face = 5;
    let mut result = pit.insert(&msg, face);
    assert!(result == true);

    let data_path = Path::new("../data/packet1_interest.bin");
    let data_display = path.display();

    let mut data_file = match File::open(&data_path) {
        Err(why) => panic!("couldn't open {}: {}", data_display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut data_file_contents = Vec::new();
    match data_file.read_to_end(&mut data_file_contents) {
        Err(why) => panic!("couldn't read {}: {}", data_display, Error::description(&why)),
        Ok(_) => {}
    }
    let data_buffer = &data_file_contents[..]; // take reference to the entire thing (i.e., a slice)nt flags = fcntl(fwd_state->fd, F_GETFL, NULL);

    // 3. decode the content object
    let content = Packet::decode_packet(data_buffer);

    // 4. lookup the conten object
    let result = pit.lookup(&content);
    match result {
        Some(entry) => {
            println!("Matched correctly!");
        },
        None => assert!(false)
    };
}

#[test]
fn test_pit_flush() {
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

    let mut pit = PIT::new();

    let msg = Packet::decode_packet(buffer);

    // Test flush first
    let pre_insert_result = pit.flush(&msg);
    assert!(pre_insert_result == false);

    // Acquire the pre-insert count
    let expected_count = pit.entries.len();

    let mut face = 5;
    let mut result = pit.insert(&msg, face);
    assert!(result == true);

    // Test post-insert flush
    let post_insert_result = pit.flush(&msg);
    assert!(post_insert_result == true);

    let actual_count = pit.entries.len();
    assert!(expected_count == actual_count);
}
