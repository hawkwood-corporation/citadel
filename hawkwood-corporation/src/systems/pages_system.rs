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
    //BlogPost { data: PageData },
    //ProductPage { data: PageData },
}

impl Site {
    pub fn construct(&self, page_type: &mut Page, pages: &mut Vec<Page>) {
        match page_type {
            Page::Homepage { page } => self.construct_homepage( page ),
            //Page::BlogPost { data } => self.construct_blog_post(),
            //Page::ProductPage { data } => self.construct_product_page(),
        };
        
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
        self.construct(&mut page, &mut pages);
        pages.push(page);
        
    
    }
}
