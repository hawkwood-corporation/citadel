use crate::citadel::*;

impl Site {
    pub fn construct_homepage(&self,  page: &mut PageData) {
        
        let head = self.construct_head(page);
        
        
        let title = &page.title;
        let metadescription = "Strategic revenue engineering for serious businesses. We deliver 10:1+ ROI through sophisticated Google Ads systems that perform under pressure.".to_owned();

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
    }
}
