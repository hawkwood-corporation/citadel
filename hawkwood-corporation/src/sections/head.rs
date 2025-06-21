use crate::citadel::*;

impl Site {
    
    pub fn construct_head(&self, data: &PageData) -> String {
        
        
        format!(
            r#"<head>
                <title>{}</title>
                <meta name="description" content="{}">
                <!-- Google Analytics -->
                <script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
                <script>
                window.dataLayer = window.dataLayer || [];
                function gtag(){{dataLayer.push(arguments);}}
                gtag('js', new Date());
                gtag('config', 'GA_TRACKING_ID');
                </script>
                <!-- Page-specific tracking -->
            </head>"#,
            data.title,
            data.metadescription.as_ref().unwrap_or(&String::new())
        ).to_owned()
    }

}