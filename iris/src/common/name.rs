use std::vec::Vec as Vec;
use std::string::String as String;

pub struct Name {
    components: Vec<String>
}

impl Name {
    pub fn new(components: Vec<String>) -> Name {
        Name {
            components: components
        }
    }
}
