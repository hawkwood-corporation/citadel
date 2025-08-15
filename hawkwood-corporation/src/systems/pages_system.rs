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

pub enum Pillar {
    Homepage,
    About,
    Services,
    Contact,
    Intelligence,
}

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
        match &page.specification {
            PageSpecification::PillarPage(pillar_type) => {
                match pillar_type {
                    Pillar::Homepage => self.construct_homepage(&mut page.foundation),
                    Pillar::About => {
                        // TODO: implement construct_about
                    },
                    Pillar::Services => {
                        // TODO: implement construct_services
                    },
                    Pillar::Contact => {
                        // TODO: implement construct_contact
                    },
                    Pillar::Intelligence => {
                        // TODO: implement construct_intelligence
                    },
                }
            },
            PageSpecification::BlogPost { .. } => {
                self.construct_blog_post(&mut page.foundation);
            },
        }
        
        clean_up_metadata(&mut page.foundation);
    }
    

    pub fn create_pages(&mut self) {
        
        /*let mut pages = Vec::new();
        
        let mut page = Page::PillarPage {    
            page: PageData {
                title: "Homepage".to_owned(),
                slug: None,
                metadescription: None,
                content: None,
                image: None,
            }
        };
        self.construct(&mut page);
        pages.push(page);*/
        
        
        let mut pages = vec![
            Page {
                foundation: PageFoundation { 
                    title: "Homepage".to_owned(),
                    ..Default::default() 
                },
                specification: PageSpecification::PillarPage(Pillar::Homepage),
            },
            Page {
                foundation: PageFoundation { 
                    title: "My First Post".to_owned(),
                    ..Default::default()
                },
                specification: PageSpecification::BlogPost {
                    date: Some("2025-01-15".to_owned()),
                    author: Some("Jake".to_owned()),
                },
            },
        ];
        
        for page in &mut pages {
            self.construct(page);
        }
        
        self.pages = pages;
        
    }

    
}
