use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_header(&self, page: &PageData) -> String {
        
        let home_nav_item = NavWrappedItem { 
            name: "Home".to_owned(), 
            path: "/".to_owned(),
            content: r#"<img src="hawkwood-corporation-logo.svg" alt="Hawkwood Corporation Logo">"#.to_owned()
        };
        
        let nav_items = vec![
            NavItem { 
                name: "About".to_owned(), 
                path: "/about".to_owned()
            },
            NavItem { 
                name: "Projects".to_owned(), 
                path: "/projects".to_owned() 
            },
            NavItem { 
                name: "Intelligence".to_owned(), 
                path: "/intelligence".to_owned() 
            },
        ];
        
        let call_us_item = NavWrappedItem { 
            name: "Call Us".to_owned(), 
            path: "/call-us".to_owned(),
            content: "Call Us".to_owned()
        };
        
        let home_link = self.construct_nav_wrapped_link(&home_nav_item, &page);
        
        let nav_links = self.construct_nav_links(&nav_items, &page);
        
        let call_us_link = self.construct_nav_wrapped_link(&call_us_item, &page);
        

        format!(
            r####"
            
            <header>
                <div class="logo">
                    {home_link}
                </div>
                <nav aria-label="Main navigation">
                    <ul>
                        {nav_links}
                    </ul>
                </nav>
                {call_us_link}
            </header>
            
            "####
        )
    }

}
