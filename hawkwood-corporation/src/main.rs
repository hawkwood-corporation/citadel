#![allow(unused_imports)]
#[macro_use] extern crate slugify;

use citadel::prelude::*;



fn main() {
    
    let mut site = Site::new();
    
    site.create_pages();
    
    site.decree_across_pages();
    
    site.write_files();
    
  
    
}

