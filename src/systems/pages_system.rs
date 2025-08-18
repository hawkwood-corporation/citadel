use crate::prelude::*;

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

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Pillar {
    Homepage,
    About,
    Services,
    Contact,
    Intelligence,
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum PageSpecification {
    PillarPage(Pillar),
    BlogPost {
        date: Option<String>,
        author: Option<String>,
    },
}

pub struct Page {
    pub foundation: PageFoundation,
    pub specification: PageSpecification,
}

impl Page {
    pub fn as_foundation(&self) -> &PageFoundation {
        &self.foundation
    }
    
    pub fn as_foundation_mut(&mut self) -> &mut PageFoundation {
        &mut self.foundation
    }
}

impl Site {
    pub fn construct(&mut self, page: &mut Page) {
        if let Some(constructor_fn) = self.page_constructors.get(&page.specification) {
            constructor_fn(self, page);
        }
        clean_up_metadata(&mut page.foundation);
    }
    
    pub fn create_pages(&mut self) {
        let constructors = self.page_constructors.clone();
        
        for (page_spec, constructor_fn) in constructors {
            let mut page = Page {
                foundation: PageFoundation::default(),
                specification: page_spec,
            };
            constructor_fn(self, &mut page);
            self.pages.push(page);
        }
    }
}