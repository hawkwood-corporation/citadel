use crate::prelude::*;

pub struct PageData {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

impl Default for PageData {
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

pub enum Page {
    PillarPage { page: PageData, pillar: Pillar },
    BlogPost { page: PageData },
}

pub enum Pillar {
    Homepage,
    //About,
    //Services,
    //Contact,
    //Intelligence,
}

pub trait AsPageData {
    fn as_page_data(&self) -> &PageData;
    fn as_page_data_mut(&mut self) -> &mut PageData;
}

impl AsPageData for Page {
    fn as_page_data(&self) -> &PageData {
        match self {
            Page::PillarPage { page, ..  } => page,
            Page::BlogPost { page, .. } => page,
            // Page::ProductPage { page, .. } => page,
        }
    }
    fn as_page_data_mut(&mut self) -> &mut PageData {
        match self {
            Page::PillarPage { page, ..  } => page,
            Page::BlogPost { page, .. } => page,
            // Page::ProductPage { page, .. } => page,
        }
    }
}

impl Site {
    pub fn construct(&mut self, page_type: &mut Page) {
        let page = match page_type {
            Page::PillarPage { page, pillar } => { 
                self.construct_pillar_page(page, pillar); 
                page 
            },
            Page::BlogPost { page, .. } => { 
                self.construct_blog_post(page); 
                page 
            },
        };
        
        clean_up_metadata(page);
    }
    
    pub fn construct_pillar_page(&mut self, page: &mut PageData, pillar: &Pillar) {
        match pillar {
            Pillar::Homepage => self.construct_homepage(page),
            //Pillar::About => self.construct_about_content(page),
            //Pillar::Services => self.construct_services_content(page),
            //Pillar::Contact => self.construct_contact_content(page),
            //Pillar::Intelligence => self.construct_intelligence_content(page),
        }
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
            Page::PillarPage {
                pillar: Pillar::Homepage,
                page: PageData { 
                    title: "Homepage".to_owned(),
                    ..Default::default() 
                } 
            },
            Page::BlogPost { 
                page: PageData { 
                    title: "My First Post".to_owned(),
                    ..Default::default()
                } 
            },
        ];
        
        for page in &mut pages {
            self.construct(page);
        }
        
        self.pages = pages;
        
    }
    
}
