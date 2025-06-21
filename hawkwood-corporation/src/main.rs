mod data;
mod sections;
mod pages;

use std::fs;

fn main() {
    
    let site_data = data::create_site_data();
    
    let pages = pages::create_pages(&site_data);
    
    let _ = fs::create_dir_all("../public");
    let _ = fs::write("../public/index.html", "test");
    
    println!("Site generated!");
   
    
}

