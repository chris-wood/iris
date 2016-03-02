use std::fmt;
use std::vec::Vec as Vec;
use std::string::String as String;

#[derive(Debug)]
pub struct Name {
    // components: Vec<String>
    name_bytes: Vec<u8>,
    name_segment_offsets: Vec<(usize, usize)>
}

impl fmt::Display for Name {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let len = self.name_bytes.len();
        let mut i = 0;
        while i < len {
            write!(f, "{}", self.name_bytes[i] as char);
            i = i + 1;
        }
        write!(f, "{}", 0)
    }
}

impl Name {
    pub fn create_from_bytes(bytes: &Vec<u8>, name_segment_offsets: &Vec<(usize, usize)>) -> Option<Name> {
        // let mut components = Vec::new();
        let mut copy_bytes = Vec::new();
        let mut copy_offsets = Vec::new();

        for b in bytes {
            copy_bytes.push(*b);
        }
        let mut i = 0;
        while i < name_segment_offsets.len() {
            let (o, l) = name_segment_offsets[i];
            copy_offsets.push((o,l));
            i = i + 1;
        }

        let name = Name {
            name_bytes: copy_bytes,
            name_segment_offsets: copy_offsets
        };

        return Some(name);
    }

    pub fn create_from_string(string_form: String) -> Option<Name> {
        let splits: Vec<&str> = string_form.split("/").collect();
        let length = splits.len();
        let mut offset = 0;
        match length {
            0 => return None,
            _ => { // ok
                let mut bytes = Vec::new();
                let mut offsets = Vec::new();

                let mut index = 0;
                if splits[0].len() == 0 {
                    index += 1;
                }
                while index < length - 1 {
                    offsets.push((offset, splits[index].len()));
                    for b in splits[index].as_bytes() {
                        bytes.push(*b);
                    }
                    offset += splits[index].len();
                    index += 1;
                }

                let suffix = splits[index];
                if index == (length - 1) && suffix.len() > 0 {
                    offsets.push((offset, splits[index].len()));
                    for b in splits[index].as_bytes() {
                        bytes.push(*b);
                    }
                }

                let name = Name {
                    name_bytes: bytes,
                    name_segment_offsets: offsets
                };

                return Some(name);
            }
        };
        return None;
    }

    pub fn display(&mut self) {
        let copy_bytes = &self.name_bytes;
        let self_size: usize = self.number_of_components();
        let mut i = 0;
        while i < self_size {
            print!("/");
            let (o, l) = self.name_segment_overlay(i);
            let mut index = 0;
            while index < l {
                print!("{}", copy_bytes[o + index] as char);
                index = index + 1;
            }

            i = i + 1;
        }
    }

    pub fn len(&self) -> usize {
        return self.name_bytes.len();
    }

    pub fn number_of_components(&self) -> usize {
        return self.name_segment_offsets.len();
    }

    pub fn name_segment_overlay(&self, index: usize) -> (usize, usize) {
        return self.name_segment_offsets[index];
    }

    pub fn at(&self, index: usize) -> u8 {
        return self.name_bytes[index];
    }

    pub fn clone(&self) -> Name {
        return Name::create_from_bytes(&self.name_bytes, &self.name_segment_offsets).unwrap();
    }

    pub fn is_prefix_of(&self, target: &Name) -> (bool) {
        let self_size: usize = self.number_of_components();
        let target_size: usize = target.number_of_components();

        println!("{} {}", self, target);

        if self_size > target_size {
            return false; // impossible for self to be a prefix
        }

        let mut index: usize = 0;
        while index < self_size {
            let (offsetA, lengthA) = self.name_segment_offsets[index];
            let (offsetB, lengthB) = target.name_segment_overlay(index);

            println!("{}:{}    {}:{}", offsetA, lengthA, offsetB, lengthB);

            if lengthA != lengthB {
                return false;
            } else {
                let mut i = 0;
                while i < lengthA {
                    if self.at(offsetA + i) != target.at(offsetB + i) {
                        return false;
                    }
                    i = i + 1;
                }
            }

            index = index + 1;
        }

        return true;
    }

    pub fn equals(&self, target: &Name) -> (bool) {
        let self_size: usize = self.number_of_components();
        let target_size: usize = target.number_of_components();

        if self.is_prefix_of(target) && self_size == target_size {
            return true;
        } else {
            return false;
        }
    }
}

#[test]
fn test_name_create_from_bytes() {
    // TODO: create an interest with ccn-lite and then use that output to test here
    assert!(true);
}

#[test]
fn test_name_create_from_string() {
    let mut n1 = Name::create_from_string("/hello".to_owned()).unwrap();
    print!("1: ");
    n1.display();
    println!("");

    let mut n2 = Name::create_from_string("/hello/".to_owned()).unwrap();
    print!("2: ");
    n2.display();
    println!("");

    let mut n3 = Name::create_from_string("hello/".to_owned()).unwrap();
    print!("3: ");
    n3.display();
    println!("");

    assert!(n1.equals(&n2));
    assert!(n1.equals(&n3));
    assert!(n2.equals(&n3));
}

#[test]
fn test_name_len() {
    let n1 = Name::create_from_string("/hello".to_owned()).unwrap();
    assert!(n1.len() == 5);

    let n2 = Name::create_from_string("/hello/world/man".to_owned()).unwrap();
    assert!(n2.len() == 13);
}

