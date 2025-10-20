use crate::prelude::*;
use std::{fs, path::PathBuf};

impl<T, I> Site<T, I> {
    pub fn write_files(&self) {
        let mut output_path = PathBuf::from(".");
        output_path.push(&self.settings.output_folder);
        
        println!("{}", output_path.display());
        fs::create_dir_all(&output_path)
            .expect("Failed to create output folder");
    
        println!("Pages to generate: {}", &self.pages.len());
        
        for page_item in &self.pages {
            let filename = self.get_filename(&page_item.foundation);
            let file_path = output_path.join(&filename);
            
            // Create parent directory if it doesn't exist
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)
                    .expect("Failed to create page directory");
            }
            
            println!("Writing page to file: {}", filename);
            fs::write(
                file_path, 
                page_item.foundation.content.as_ref().unwrap()
            ).expect("Failed to write page to file");
        }
        
        println!("Site generated!");
    }
    
    fn get_filename(&self, foundation: &PageFoundation) -> String {
        if let Some(slug) = &foundation.slug {
            if slug == "/" || slug.is_empty() {
                return "index.html".to_owned();
            }
            
            self.set_filename(&slug)
            
        } else {
            let title_lower = foundation.title.to_lowercase();
            if title_lower == "homepage" || title_lower == "home" || title_lower == "home page" {
                return "index.html".to_owned();
            }
            
            let slug = foundation.slug.as_ref().unwrap_or(&foundation.title);
            
            self.set_filename(&slug)
        }
    }
    
    fn set_filename(&self, slug: &str) -> String {
        if self.settings.use_trailing_slashes {
            format!("{}/index.html", slug)
        } else {
            format!("{}.html", slug)
        }
    }
}