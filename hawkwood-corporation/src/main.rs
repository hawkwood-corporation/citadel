use std::fs;

mod systems;
mod page_types;
mod sections;
pub mod citadel;
mod site;


use citadel::*;


fn main() {
    
    let mut site = Site::new();
    
    site.create_pages();
    
    let _ = fs::create_dir_all("../public");
    let _ = fs::write("../public/index.html", "test");
    
    println!("Site generated!");
    
}

