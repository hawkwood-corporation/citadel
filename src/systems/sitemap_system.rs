use crate::prelude::*;
use std::fs;

impl<T, I> Site<T, I> {
    /// Generate and write sitemap.xml - sovereign and simple
    /// Generate and write sitemap.xml - sovereign and simple
    pub fn generate_sitemap(&self) {
        // Ensure output directory exists
        fs::create_dir_all(&self.settings.output_folder)
            .expect("Failed to create output directory for sitemap");
            
        let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
        
        for page in &self.pages {
            // Construct the full URL matching the file structure
            let url = if let Some(slug) = &page.foundation.slug {
                if slug.is_empty() || slug == "/" {
                    self.base_url.to_string().trim_end_matches('/').to_owned()
                } else if self.settings.use_trailing_slashes {
                    // With trailing slashes: /about/ (implied index.html)
                    let url_with_slash = format!("{}/", slug);
                    self.base_url.join(&url_with_slash).unwrap().to_string()
                } else {
                    // Without trailing slashes: /about (as .html file)
                    self.base_url.join(slug).unwrap().to_string()
                }
            } else {
                self.base_url.to_string().trim_end_matches('/').to_owned()
            };
            
            sitemap.push_str(&format!(r#"
    <url>
        <loc>{}</loc>
    </url>"#, url));
        }
        
        sitemap.push_str(r#"
</urlset>"#);
        
        // Write to output folder
        let output_path = self.settings.output_folder.join("sitemap.xml");
        fs::write(&output_path, sitemap)
            .expect("Failed to write sitemap.xml");
        
        println!("Generated sitemap.xml with {} URLs", self.pages.len());
    }
    
    /// Generate robots.txt with sitemap reference
    pub fn generate_robots_txt(&self) {
        // Ensure output directory exists
        fs::create_dir_all(&self.settings.output_folder)
            .expect("Failed to create output directory for robots.txt");
            
        let sitemap_url = self.base_url.join("sitemap.xml").unwrap();
        
        let robots = format!(r#"User-agent: *
Allow: /

Sitemap: {}"#, sitemap_url);
        
        let output_path = self.settings.output_folder.join("robots.txt");
        fs::write(&output_path, robots)
            .expect("Failed to write robots.txt");
        
        println!("Generated robots.txt");
    }
}