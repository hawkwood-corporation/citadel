use crate::prelude::*;

pub struct PageData {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

pub enum Page {
    Homepage { page: PageData },
    BlogPost { page: PageData },
    //ProductPage { data: PageData },
}

pub trait AsPageData {
    fn as_page_data(&self) -> &PageData;
    fn as_page_data_mut(&mut self) -> &mut PageData;
}

impl AsPageData for Page {
    fn as_page_data(&self) -> &PageData {
        match self {
            Page::Homepage { page } => page,
            Page::BlogPost { page, .. } => page,
            // Page::ProductPage { page, .. } => page,
        }
    }
    fn as_page_data_mut(&mut self) -> &mut PageData {
        match self {
            Page::Homepage { page } => page,
            Page::BlogPost { page, .. } => page,
            // Page::ProductPage { page, .. } => page,
        }
    }
}

impl Site {
    pub fn construct(&mut self, page_type: &mut Page) {
        let page = match page_type {
            Page::Homepage { page } => { self.construct_homepage( page ); page },
            Page::BlogPost { page } => { self.construct_blog_post( page ); page },
            //Page::ProductPage { data } => self.construct_product_page(),
        };
        
        clean_up_metadata(page);
        
    }

    pub fn create_pages(&mut self) {
        
        let mut pages = Vec::new();
        
        let mut page = Page::Homepage {    
            page: PageData {
                title: "Homepage".to_owned(),
                slug: None,
                metadescription: None,
                content: None,
                image: None,
            }
        };
        self.construct(&mut page);
        pages.push(page);
        
        self.pages = pages;
    }
}
