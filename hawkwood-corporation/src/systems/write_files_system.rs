use crate::prelude::*;
use std::{fs, path::PathBuf};

impl Site {
    
    pub fn write_files(&self) {
        
        // Create public folder
        
        let mut output_path = PathBuf::from("..");
        output_path.push(&self.settings.output_folder);
        
        println!("{}", output_path.display());
        fs::create_dir_all(&output_path)
            .expect("Failed to create output folder");
    
        println!("Pages to generate: {}", &self.pages.len());
        
        for page_item in &self.pages {
            let (page, filename) = match page_item {
                
                Page::Homepage { page } => {
                    let filename = "index.html".to_owned();
                    (page, filename)
                },
                Page::BlogPost { page } /*| Page::ProductPage { page }*/ =>
                {
                    let filename = format!("{}.html", page.slug.as_ref().unwrap());
                    (page, filename)
                },
            };
            println!("Writing page to file: {}", filename);
            fs::write(output_path.join(filename), page.content.as_ref().unwrap())
            .expect("Failed to write page to file");
        }
        
        
        println!("Site generated!");
        
    }
    
}
