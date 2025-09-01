use crate::prelude::*;
use std::hash::Hash;

pub struct PageFoundation {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub placements: Placements,
}

impl Default for PageFoundation {
    fn default() -> Self {
        Self {
            title: String::new(),
            slug: None,
            metadescription: None,
            content: None,
            image: None,
            placements: Placements::new(),
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

impl<T: Hash + Eq + Clone, I> Site<T, I> {
    /// Data-oriented constructor registration - now with smart enum handling!
    pub fn add_constructor(mut self, page_type: T, constructor: fn(&mut Site<T, I>, &mut Page<T>)) -> Self {
        self.page_constructors.insert(page_type, constructor);
        self
    }
    
    /// Data-oriented page registration
    pub fn add_pages(mut self, pages: Vec<Page<T>>) -> Self {
        self.pages = pages;
        self
    }
    
    /// Smart constructor dispatch - handles both specific variants and enum patterns
    pub fn construct(&mut self, page: &mut Page<T>) {
        // First: Try exact match (specific variant)
        if let Some(&constructor) = self.page_constructors.get(&page.specification) {
            constructor(self, page);
            clean_up_metadata(&mut page.foundation);
            return;
        }
        
        // Second: Try discriminant match (enum parent pattern)
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