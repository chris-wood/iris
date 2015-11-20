use std::vec::Vec as Vec;
use std::string::String as String;

#[derive(Debug)]
pub struct Name {
    components: Vec<String>
}

impl Name {
    pub fn new(components: Vec<String>) -> (Name) {
        Name {
            components: components
        }
    }

    pub fn len(&self) -> usize{
        return self.components.len();
    }

    pub fn at(&self, index: usize) -> String {
        return self.components[index].clone();
    }

    pub fn clone(&self) -> Name {
        let mut components:Vec<String> = Vec::new();
        let mut index: usize = 0;
        let self_size: usize = self.len();
        while index < self_size {
            components.push(self.at(index));
            index = index + 1;
        };

        let name = Name {
            components: components
        };
        return name;
    }

    pub fn equals(&self, target: &Name) -> (bool) {
        let self_size: usize = self.len();
        let target_size: usize = target.len();

        if self_size != target_size {
            return false;
        }

        let mut index: usize = 0;
        while index < self_size {
            if self.at(index) != target.at(index) {
                return false;
            }
            index = index + 1;
        }

        return true;
    }
}
