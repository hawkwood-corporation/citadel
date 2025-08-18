use crate::prelude::*;
use std::hash::Hash;

pub struct PageFoundation {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl Default for PageFoundation {
    fn default() -> Self {
        Self {
            title: String::new(),
            slug: None,
            metadescription: None,
            content: None,
            image: None,
        }
    }
}

pub struct Page<T> {
    pub foundation: PageFoundation,
    pub specification: T,
}

impl<T> Page<T> {
    pub fn as_foundation(&self) -> &PageFoundation {
        &self.foundation
    }
    
    pub fn as_foundation_mut(&mut self) -> &mut PageFoundation {
        &mut self.foundation
    }
}

impl<T: Hash + Eq + Clone> Site<T> {
    /// Data-oriented constructor registration
    pub fn add_constructor(mut self, page_type: T, constructor: fn(&mut Site<T>, &mut Page<T>)) -> Self {
        self.page_constructors.insert(page_type, constructor);
        self
    }
    
    /// Data-oriented page registration
    pub fn add_pages(mut self, pages: Vec<Page<T>>) -> Self {
        self.pages = pages;
        self
    }
    
    /// Internal constructor dispatch - match by discriminant for enum variants with data
    pub fn construct(&mut self, page: &mut Page<T>) {
        // Find constructor by matching discriminant (enum variant type, ignoring data)
        let found_constructor = self.page_constructors.iter()
            .find_map(|(key, constructor)| {
                if std::mem::discriminant(key) == std::mem::discriminant(&page.specification) {
                    Some(*constructor)
                } else {
                    None
                }
            });
        
        if let Some(constructor) = found_constructor {
            constructor(self, page);
        } else {
            eprintln!("Warning: No constructor found for page type");
        }
        clean_up_metadata(&mut page.foundation);
    }
    
    /// Data transformation through the registered constructors
    pub fn create_pages(&mut self) {
        let mut pages = std::mem::take(&mut self.pages);
        
        for page in &mut pages {
            self.construct(page);
        }
        
        self.pages = pages;
    }
}