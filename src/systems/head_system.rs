use crate::prelude::*;

impl<T: Hash + Eq + Clone> Site<T> {
    pub fn add_head_constructor(mut self, head_fn: fn(&mut Site<T>, &Page<T>) -> String) -> Self {
        self.head_constructor = Some(head_fn);
        self
    }
    
    pub fn construct_head(&mut self, page: &Page<T>) -> String {
        if let Some(head_fn) = self.head_constructor {
            head_fn(self, page)
        } else {
            self.seo_basics_head(page)  // Default fallback
        }
    }
    
    

    pub fn seo_basics_head(&mut self, page: &Page<T>) -> String {
        let title = &page.foundation.title;
        let metadescription = page.foundation.metadescription.as_deref().unwrap_or("");
        
        // Rest of your existing logic stays the same!
        let content = format!(r####"
            <head>
                <title>{title}</title>
                <meta name="description" content="{metadescription}">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                
                <link rel="preconnect" href="https://fonts.googleapis.com">
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                <link href="https://fonts.googleapis.com/css2?family=Hanken+Grotesk:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet">
                <style>
                    [CSS_POSITION]
                </style>
                <!-- Google Analytics -->
                <script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
                <script>
                    window.dataLayer = window.dataLayer || [];
                    function gtag(){{dataLayer.push(arguments);}}
                    gtag('js', new Date());
                    gtag('config', 'GA_TRACKING_ID');
                </script>
                <!-- Quicklink Page Preloading -->
                <script defer src="https://cdn.jsdelivr.net/npm/quicklink@3.0.1/dist/quicklink.umd.js"></script>
                <script>
                    window.addEventListener('load', () => {{
                        quicklink.listen();
                        }});
                </script>
            </head>
        "####);
        
        // Keep all your CSS declarations
        self.declare_css("foundation", "
        
        /* Foundation */
        
        
        ");
        // etc.
        
        content
    }
}