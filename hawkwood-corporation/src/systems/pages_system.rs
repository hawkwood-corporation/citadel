use crate::citadel::*;

pub struct PageData {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
}

// definitely change to flat

pub enum Page {
    Homepage { page: PageData },
    BlogPost { page: PageData },
    //ProductPage { data: PageData },
}

impl Site {
    pub fn construct(&self, page_type: &mut Page) {
        let page = match page_type {
            Page::Homepage { page } => { self.construct_homepage( page ); page },
            Page::BlogPost { page } => { self.construct_blog_post( page ); page },
            //Page::ProductPage { data } => self.construct_product_page(),
        };
        
        clean_up_metadata(page);
        
    }

/*************  ✨ Windsurf Command ⭐  *************/
/*******  063b67c7-1103-4397-9a87-b1e6b86005c7  *******/
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
