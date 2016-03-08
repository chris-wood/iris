use std::vec;
use common::name::Name as Name;
use core::link::Link as Link;
use core::packet::message::Message as Message;

pub struct FIBEntry {
    name: Name,
    pub faces: Vec<usize>
}

// impl FIBEntry {
//     fn display(&self) {
//         // self.name.display();
//     }
// }

pub struct FIB {
    entries: Vec<FIBEntry>
}

impl FIB {
    pub fn new() -> FIB {
        FIB {
            entries: Vec::new()
        }
    }

    // pub fn lookup(&self, target: &Name) -> Option<&FIBEntry> {
    pub fn lookup(&self, target: &Message) -> Option<&FIBEntry> {
        let mut name = target.get_name();
        println!("Lookup {}", name);
        for entry in self.entries.iter() {
            println!("Against {}", entry.name);
            if entry.name.is_prefix_of(&name) {
                return Some(entry);
            }
        }
        return None;
    }

    pub fn insert(&mut self, target: &Name, newFace: usize) -> (bool) {
        println!("ADDING NEW ENTRY WITH FACE ID {}", newFace);

        for entry in self.entries.iter_mut() {
            if entry.name.equals(target) {
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

#[test]
fn test_fib_lookup() {
    let mut fib = FIB::new();
    // let n1 = name::create_from_string("/hello/world");
}
