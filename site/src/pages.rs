use crate::sections::*;
use crate::data::*;

struct PageData {
    title: String,
    slug: String,
    metadescription: Option<String>,
    content: Option<String>,
}


pub fn create_pages(data: &SiteData) {
    
    let homepage = create_homepage(&data);
    
    
}


pub fn create_homepage(site_data: &SiteData) -> PageData {
    
    PageData {
        
        
        
    }
    
}