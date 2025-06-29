use crate::prelude::*;

pub struct NavItem {
    pub name: String,
    pub url: String,
}

impl Site {
    pub fn create_nav_items(&self) -> Vec<NavItem> {
        vec![
            NavItem { 
                name: "Operations".to_owned(), 
                url: "/operations".to_owned() 
            },
            NavItem { 
                name: "Intelligence".to_owned(), 
                url: "/intelligence".to_owned() 
            },
            NavItem { 
                name: "Leadership".to_owned(), 
                url: "/leadership".to_owned() 
            },
            NavItem { 
                name: "Deployment".to_owned(), 
                url: "/deployment".to_owned() 
            },
        ]
    }
}