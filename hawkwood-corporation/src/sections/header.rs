use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_header(&mut self, page: &PageData) -> String {
        
        
        self.declare_css("header", r##"

/* ===== HEADER ===== */
{}
header {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    z-index: 10;
    padding: 18px var(--site-padding-x);
    margin: 0 auto;
    max-width: 100%;
    
    nav {
        --transform: scale(0.97);
        --nav-link-padding-x: 14px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        max-width: 1440px;
        gap: 40px;
        margin: 0 auto;

        ul {
            display: flex;
            list-style: none;
            gap: 30px;
            margin: 0;
            padding: 0;
            
            li {
                list-style: none;
                transition: transform 0.3s ease;
                width: fit-content;
                
                &:hover {
                    transform: var(--transform);
                }
            
                a {
                    padding: var(--nav-link-padding-x) 14px;
                }
            }
        }
         
        a {
            text-decoration: none;
            color: var(--smoky-black);
            font-size: 18tem;
            cursor: pointer;
        }
        
        .company-title {
            font-size: 24tem;
            color: var(--smoky-black);
            transition: transform 0.3s ease;
            
            &:hover {
                transform: scale(0.97);
            }
        }
        
        .directive {
            padding: 10px 50px;
        }
        
        
    }

}

@media screen and (max-width: [mobile]) {
    
    
    header {

        
        &:has(#nav-toggle-checkbox:checked){
            position: relative;
            padding-bottom: 30px;
        }
        
        nav {
            
            --transform: translateX(6px) scale(0.98);
            
            ul {
                display: none;
                
                li {
                    a {
                        margin-left: calc(-1 * var(--nav-link-padding-x));
                    }
                }
            }
            
            .directive {
                display: none; 
            }
            
            &:has(#nav-toggle-checkbox:checked){
                display: grid;
                grid-template-columns: 1fr;
                grid-template-rows: auto 1fr;
                row-gap: 30px;
            }
            
            #nav-toggle-checkbox:checked {
                
                & ~ ul {
                    display: flex;
                    flex-direction: column;
                    grid-area: 2 / 1 / 2 / 3;
                    padding-top: 10px;
                }
                
                & ~ .directive {
                    display: block;
                    grid-area: 3 / 1 / 3 / 3;
                    position: relative;
                }
                
                
                & ~ #nav-toggle-button {
                    grid-area: 1 / 2;
                }
            }
        }

    }
}



"##);
        
        /*let home_nav_item = NavWrappedItem { 
            name: "Home".to_owned(), 
            path: "/".to_owned(),
            content: r#"<img src="hawkwood-corporation-logo.png" width="40px" height="40px" class="logo" alt="Hawkwood Corporation Logo">"#.to_owned(),
            class: None,
        };*/
        
        let home_nav_item = NavWrappedItem { 
            name: "Home".to_owned(), 
            path: "/".to_owned(),
            content: "Hawkwood Corporation".to_owned(),
            class: Some("company-title".to_owned()),
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
            content: r##"Call Us <svg class="symbol call-symbol" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path d="M144.27,45.93a8,8,0,0,1,9.8-5.66,86.22,86.22,0,0,1,61.66,61.66,8,8,0,0,1-5.66,9.8A8.23,8.23,0,0,1,208,112a8,8,0,0,1-7.73-5.94,70.35,70.35,0,0,0-50.33-50.33A8,8,0,0,1,144.27,45.93Zm-2.33,41.8c13.79,3.68,22.65,12.54,26.33,26.33A8,8,0,0,0,176,120a8.23,8.23,0,0,0,2.07-.27,8,8,0,0,0,5.66-9.8c-5.12-19.16-18.5-32.54-37.66-37.66a8,8,0,1,0-4.13,15.46Zm81.94,95.35A56.26,56.26,0,0,1,168,232C88.6,232,24,167.4,24,88A56.26,56.26,0,0,1,72.92,32.12a16,16,0,0,1,16.62,9.52l21.12,47.15,0,.12A16,16,0,0,1,109.39,104c-.18.27-.37.52-.57.77L88,129.45c7.49,15.22,23.41,31,38.83,38.51l24.34-20.71a8.12,8.12,0,0,1,.75-.56,16,16,0,0,1,15.17-1.4l.13.06,47.11,21.11A16,16,0,0,1,223.88,183.08Zm-15.88-2s-.07,0-.11,0h0l-47-21.05-24.35,20.71a8.44,8.44,0,0,1-.74.56,16,16,0,0,1-15.75,1.14c-18.73-9.05-37.4-27.58-46.46-46.11a16,16,0,0,1,1-15.7,6.13,6.13,0,0,1,.57-.77L96,95.15l-21-47a.61.61,0,0,1,0-.12A40.2,40.2,0,0,0,40,88,128.14,128.14,0,0,0,168,216,40.21,40.21,0,0,0,208,181.07Z"></path></svg>"##.to_owned(),
            class: Some("directive".to_owned()),
        };
        
        let home_link = self.construct_nav_wrapped_link(&home_nav_item, &page);
        
        let nav_links = self.construct_nav_links(&nav_items, &page);
        
        let call_us_link = self.construct_nav_wrapped_link(&call_us_item, &page);
        
        let (nav_checkbox, nav_button) = self.construct_nav_toggle();

        format!(
            r####"
            
            <header>
            <nav aria-label="Main navigation">
                {nav_checkbox}
                {home_link}                
                <ul>
                    {nav_links}
                </ul>
                {call_us_link}
                {nav_button}
            </nav>
            </header>
            
            "####
        )
    }

}
