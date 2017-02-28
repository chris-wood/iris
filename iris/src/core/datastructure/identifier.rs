use std::vec;
use common::name::Name as Name;

#[derive(Clone, Debug)]
pub struct Identifier {
    name: Name,
    key_id: Vec<u8>,
    content_id: Vec<u8>,
}

impl Identifier {
    pub fn empty() -> Identifier {
        Identifier {
            name: Name::empty(),
            key_id: Vec::new(),
            content_id: Vec::new(),
        }
    }

    pub fn new(name: Name, key_id: &[u8], content_id: &[u8]) -> Identifier {
        Identifier {
            name: name,
            key_id: key_id.to_vec(),
            content_id: content_id.to_vec(),
        }
    }

    pub fn equals(&self, other: &Identifier) -> bool {
        if self.name.equals(&other.name) {
            // XXX: compare the key ID and content ID
            return true
        }
        return false;
    }
}
