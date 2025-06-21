use std::fmt::format;

use crate::sections::*;
use crate::data::*;

pub struct PageData {
    pub title: String,
    pub slug: Option<String>,
    pub metadescription: Option<String>,
    pub content: Option<String>,
}


pub fn create_pages(data: &SiteData) -> Vec<PageData> {
    
    let mut pages = Vec::new();
    
    // Homepage
    pages.push(create_homepage(&data));

    pages
    
}


pub fn create_homepage(site_data: &SiteData) -> PageData {
    
    let title = site_data.title.clone();
    let metadescription = "Strategic revenue engineering for serious businesses. We deliver 10:1+ ROI through sophisticated Google Ads systems that perform under pressure.".to_owned();
    
    let head_data = HeadData {
        title: title.clone(),
        metadescription: metadescription.clone(),
    };
    
    let head = create_head(&head_data);

    let content = format!(
        r#"
        <html>
            {head}
            <body>
                <h1>{title}</h1>
            </body>
        </html>
        "#
    );

    let page = PageData {
        title,
        slug: None,
        metadescription: Some(metadescription),
        content: Some(content),
    };
    
    
    page
    
}