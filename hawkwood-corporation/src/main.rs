#![allow(unused_imports)]
#[macro_use] extern crate slugify;
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
    
    site.write_files();
    
  
    
}

