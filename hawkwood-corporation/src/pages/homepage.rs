use crate::citadel::*;

impl Site {
    pub fn construct_homepage(&self, page: &mut PageData) -> PageData {
        
        let title = page.title.clone();
        let metadescription = "Strategic revenue engineering for serious businesses. We deliver 10:1+ ROI through sophisticated Google Ads systems that perform under pressure.".to_owned();
        
        

        let content = format!(
            r#"
            <html>
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
        
        let head = self.construct_head(&page);
        
        
        page
        
    }

}