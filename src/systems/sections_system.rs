use crate::prelude::*;

impl<T, I> Site<T, I> {
    pub fn declare_section(&mut self, key: &str, section: &str) {
        if !self.sections.contains_key(key) {
            println!("Declared section: {}", key);
            self.sections.insert(key.to_string(), section.to_string());
        }
    }
}