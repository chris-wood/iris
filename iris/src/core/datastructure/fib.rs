use std::vec;
use std::fmt;
use common::name::Name as Name;
use core::link::Link as Link;
use core::packet::message::Message as Message;

pub struct FIBEntry {
    name: Name,
    pub faces: Vec<usize>
}

impl fmt::Display for FIBEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
//        for face in self.faces {
//            write!(f, "{}", face);
//        }
//        write!(f, "{}", 0)
    }
}

// impl FIBEntry {
//     fn display(&self) {
//         // self.name.display();
//     }
// }

#[derive(Debug)]
pub enum FIBActionResult {
    InsertAdded,
    InsertMerged
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

    pub fn lookup(&self, target: &Message) -> Option<&FIBEntry> {
        let mut name = target.get_name();
        return self.lookup_by_name(&mut name);
    }

    pub fn lookup_by_name(&self, name: &mut Name) -> Option<&FIBEntry> {
        println!("Lookup ...");
        name.display();
        for entry in self.entries.iter() {
            println!("Against {}", entry.name);
            if entry.name.is_prefix_of(&name) {
                return Some(entry);
            }
        }
        return None;
    }

    // TODO: replace with FIBActionResult
    pub fn insert(&mut self, target: &Name, newFace: usize) -> (bool) {
        //println!("ADDING NEW ENTRY WITH FACE ID {}", newFace);

        for entry in self.entries.iter_mut() {
            if entry.name.equals(target) {
                entry.faces.push(newFace);
                return true; // FIBActionResult::InsertMerged
            }
        }

        let new_name = target.clone();
        let mut entry = FIBEntry {
            name: new_name,
            faces: Vec::new()
        };
        entry.faces.push(newFace);
        self.entries.push(entry);

        return true; // FIBActionResult::InsertAdded
    }
}

#[test]
fn test_fib_insert_new() {
    let mut fib = FIB::new();

    let n1 = Name::create_from_string("/hello/world".to_owned()).unwrap();
    let face = 1;
    let insert_result = fib.insert(&n1, 1);
    assert!(insert_result);
    assert!(fib.entries.len() == 1);
}

#[test]
fn test_fib_insert_merge() {
    let mut fib = FIB::new();

    let n1 = Name::create_from_string("/hello/world".to_owned()).unwrap();
    let face = 1;
    let insert_result = fib.insert(&n1, 1);
    assert!(insert_result);
    assert!(fib.entries.len() == 1);

    let merge_face = 2;
    let merge_result = fib.insert(&n1, merge_face);
    assert!(merge_result);
    assert!(fib.entries.len() == 1);
}

#[test]
fn test_fib_lookup() {
    let mut fib = FIB::new();

    let mut n1 = Name::create_from_string("/hello/world".to_owned()).unwrap();
    let face = 1;
    let insert_result = fib.insert(&n1, 1);
    assert!(insert_result);
    assert!(fib.entries.len() == 1);

    let match_same = fib.lookup_by_name(&n1);
    match match_same {
        Some(entry) => {
            assert!(entry.faces[0] == face);
        }, None => assert!(false)
    };
}
