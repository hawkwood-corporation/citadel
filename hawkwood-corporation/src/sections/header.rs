use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_header(&mut self, page: &PageData) -> String {
        
        
        self.declare_css("header", r##"

/* ===== HEADER ===== */
{}
header {
    
    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 40px;
        padding: 15px 60px;
        max-width: 1200px;
        margin: 0 auto;   

        ul {
            display: flex;
            list-style: none;
            gap: 30px;
            margin: 0;
            padding: 0;
            
            li {
                list-style: none;
            
                a {
                    padding: 14px 10px;
                }
            }
        }
        
        a {
            text-decoration: none;
            color: var(--smoky-black);
        }
    }

}



"##);
        
        let home_nav_item = NavWrappedItem { 
            name: "Home".to_owned(), 
            path: "/".to_owned(),
            content: r#"<img src="hawkwood-corporation-logo.png" width="40px" height="40px" class="logo" alt="Hawkwood Corporation Logo">"#.to_owned(),
            class: None,
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
            content: "Call Us".to_owned(),
            class: Some("call-us-link".to_owned()),
        };
        
        let home_link = self.construct_nav_wrapped_link(&home_nav_item, &page);
        
        let nav_links = self.construct_nav_links(&nav_items, &page);
        
        let call_us_link = self.construct_nav_wrapped_link(&call_us_item, &page);
        
        let nav_toggle = self.construct_nav_toggle();

        format!(
            r####"
            
            <header>
            <nav aria-label="Main navigation">
                {home_link}                
                <ul>
                    {nav_links}
                </ul>
                {call_us_link}
                {nav_toggle}
            </nav>
            </header>
            
            "####
        )
    }

}
