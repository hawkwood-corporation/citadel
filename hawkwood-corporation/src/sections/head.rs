use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    
    pub fn construct_head(&self, data: &mut PageData) -> String {
        
        let title = &data.title;
        let metadescription = data.metadescription.as_deref().unwrap_or("");
        let css = self.construct_css();
        
        format!(
            r####"
            
            <head>
                <title>{title}</title>
                <meta name="description" content="{metadescription}">
                <style>
                    {css}
                </style>
                <!-- Google Analytics -->
                <script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
                <script>
                    window.dataLayer = window.dataLayer || [];
                    function gtag(){{dataLayer.push(arguments);}}
                    gtag('js', new Date());
                    gtag('config', 'GA_TRACKING_ID');
                </script>
                <!-- Page-specific tracking -->

                <!-- Quicklink Page Preloading -->
                <script defer src="https://cdn.jsdelivr.net/npm/quicklink@3.0.1/dist/quicklink.umd.js"></script>
                <script>
                    window.addEventListener('load', () => {{
                        quicklink.listen();
                        }});
                </script>
            </head>
            
            "####
            
        ).to_owned()
        
    }
    


}