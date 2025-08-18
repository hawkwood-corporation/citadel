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

pub trait PageConstructor<T> {
    fn construct_page(&mut self, page: &mut Page<T>);
}


impl<T> Site<T> where Self: PageConstructor<T> {
    pub fn construct(&mut self, page: &mut Page<T>) {
        self.construct_page(page);
        clean_up_metadata(&mut page.foundation);
    }
    
    pub fn create_pages(&mut self) {
        let mut pages = std::mem::take(&mut self.pages);
        
        for page in &mut pages {
            self.construct(page);
        }
        
        self.pages = pages;
    }
}