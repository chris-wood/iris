use std::fmt;
use std::vec::Vec as Vec;

#[derive(PartialEq, Clone, Debug)]
pub enum HashType {
    SHA256,
    SHA512
}

#[derive(Debug, Clone)]
pub struct Hash {
    hash_type: HashType,
    hash_bytes: Vec<u8>
}

impl Hash {
    pub fn create(hash_type: HashType, bytes: &[u8]) -> Hash {
        Hash {
            hash_type: hash_type,
            hash_bytes: bytes.to_vec()
        }
    }

    pub fn equals(&self, target: &Hash) -> (bool) {
        assert!(false);
        return false
    }
}

#[test]
fn test_hash_constructor() {
    // TODO(caw): create an interest with ccn-lite and then use that output to test here
    assert!(true);
}
