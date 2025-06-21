use crate::citadel::*;

pub struct PageData {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
}

pub enum Page {
    Homepage { data: PageData },
    //BlogPost { data: PageData },
    //ProductPage { data: PageData },
}

impl Site {
    pub fn construct(&self, page_type: Page) -> PageData {
        match page_type {
            Page::Homepage { mut data } => self.construct_homepage( &mut data ),
            //Page::BlogPost { data } => self.construct_blog_post(),
            //Page::ProductPage { data } => self.construct_product_page(),
        }
    }

    pub fn create_pages(&mut self) -> Vec<PageData> {
        let mut pages = Vec::new();

    

        pages
    }
}
