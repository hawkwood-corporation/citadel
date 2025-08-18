use crate::prelude::*;
use std::{fs, path::PathBuf};

impl<T> Site<T> {
    pub fn write_files(&self) {
        let mut output_path = PathBuf::from(".");
        output_path.push(&self.settings.output_folder);
        
        println!("{}", output_path.display());
        fs::create_dir_all(&output_path)
            .expect("Failed to create output folder");
    
        println!("Pages to generate: {}", &self.pages.len());
        
        for page_item in &self.pages {
            let filename = get_filename(&page_item.foundation);
            
            println!("Writing page to file: {}", filename);
            fs::write(
                output_path.join(filename), 
                page_item.foundation.content.as_ref().unwrap()
            ).expect("Failed to write page to file");
        }
        
        println!("Site generated!");
    }
}

fn get_filename(foundation: &PageFoundation) -> String {
    if let Some(slug) = &foundation.slug {
        if slug == "/" || slug.is_empty() {
            return "index.html".to_owned();
        }
    }
    
    let title_lower = foundation.title.to_lowercase();
    if title_lower == "homepage" || title_lower == "home" || title_lower == "home page" {
        return "index.html".to_owned();
    }
    
    format!("{}.html", foundation.slug.as_ref().unwrap_or(&foundation.title))
}