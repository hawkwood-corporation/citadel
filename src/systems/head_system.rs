use crate::prelude::*;

impl<T: Hash + Eq + Clone, I> Site<T, I> {
    
    // Default head constructor - just registers for all pages to get SEO basics
    pub fn add_head_constructor(mut self) -> Self {
        self.head_constructor = Some(|site, page| site.construct_citadel_head(page));
        self
    }
    
    // Custom head constructor - registers custom function for all pages
    pub fn add_head_constructor_with(mut self, head_fn: fn(&mut Site<T, I>, &Page<T>) -> String) -> Self {
        self.head_constructor = Some(head_fn);
        self
    }
    
    pub fn construct_citadel_head(&mut self, page: &Page<T>) -> String {
        let mut title = page.foundation.title.clone();
        
        // Apply title append if configured (but skip for homepage)
        if let Some(append_pattern) = &self.settings.title_append {
            let is_homepage = page.foundation.slug.as_deref().unwrap_or("/") == "/" 
                || page.foundation.slug.as_deref().unwrap_or("") == ""
                || page.foundation.title.to_lowercase() == "homepage"
                || page.foundation.title.to_lowercase() == "home"
                || page.foundation.title == self.title;
            
            if !is_homepage {
                let append = append_pattern.replace("{site_title}", &self.title);
                title.push_str(&append);
            }
        }
        
        let metadescription = page.foundation.metadescription.as_deref().unwrap_or("");
        
        let (metadescription_tag, og_description, twitter_description) = if metadescription.is_empty() {
            (String::new(), String::new(), String::new())
        } else {
            (format!(r#"<meta name="description" content="{metadescription}">"#),
            format!(r#"<meta property="og:description" content="{metadescription}">"#),
            format!(r#"<meta name="twitter:description" content="{metadescription}">"#))
        };
        
        let page_url = if let Some(slug) = &page.foundation.slug {
            if slug.is_empty() || slug == "/" {
                self.base_url.to_string().trim_end_matches('/').to_string()
            } else {
                self.base_url.join(slug).unwrap().to_string()
            }
        } else {
            self.base_url.to_string().trim_end_matches('/').to_string()
        };
        
        let fonts_position = self.combine_placements(
            &self.placements.fonts_position,
            &page.foundation.placements.fonts_position
        );
        
        let head_top_position = self.combine_placements(
            &self.placements.head_top_position,
            &page.foundation.placements.head_top_position
        );
        
        let schema_position = self.combine_placements(
            &self.placements.schema_position,
            &page.foundation.placements.schema_position
        );
        
        let analytics_position = self.combine_placements(
            &self.placements.analytics_position,
            &page.foundation.placements.analytics_position
        );
        
        let head_bottom_position = self.combine_placements(
            &self.placements.head_bottom_position,
            &page.foundation.placements.head_bottom_position
        );
        
        let scripts_position = self.combine_placements(
            &self.placements.scripts_position,
            &page.foundation.placements.scripts_position
        );
        
        format!(r##"
            <head>
                <meta charset="UTF-8">
                <title>{title}</title>
                {metadescription_tag}
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link rel="canonical" href="{page_url}">
                
                <meta property="og:title" content="{title}">
                {og_description}
                <meta property="og:url" content="{page_url}">
                <meta property="og:type" content="website">
                
                <meta name="twitter:title" content="{title}">
                {twitter_description}
                
                {fonts_position}
                
                {head_top_position}
                
                {schema_position}
                
                <style>
                    [CSS_POSITION]
                </style>
                
                {analytics_position}
                
                {scripts_position}
                
                {head_bottom_position}
            </head>
        "##)
    }
    
    // The sovereign method - SEO basics + optional custom additions
    pub fn construct_defunct_citadel_head(&mut self, page: &Page<T>, additional_head_code: Option<&str>) -> String {
        let title = &page.foundation.title;
        let metadescription = page.foundation.metadescription.as_deref().unwrap_or("");
        let additional_head_code = additional_head_code.unwrap_or("");
        
        // Rest of your existing logic stays the same!
        format!(r##"
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
                {additional_head_code}
            </head>
        "##)
    }
    
    // For registered head constructors (site-wide consistency)
    pub fn construct_head(&mut self, page: &Page<T>) -> String {
        if let Some(head_fn) = self.head_constructor {
            head_fn(self, page)  // ← Calls whatever was registered
        } else {
            // Default fallback - just Citadel basics with no additions
            self.construct_citadel_head(page)
        }
    }
    
    // Registered head strategy PLUS additional custom code for this specific page
    pub fn construct_head_with(&mut self, page: &Page<T>, additional_head_code: &str) -> String {
        if let Some(head_fn) = self.head_constructor {
            // Get the registered head and inject the additional code
            let registered_head = head_fn(self, page);
            // Insert the additional code before the closing </head> tag
            registered_head.replace("</head>", &format!("{}\n            </head>", additional_head_code))
        } else {
            // No registered strategy, so use Citadel head with the additional code
            self.construct_citadel_head(page)
        }
    }
}