use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_header(&mut self, page: &PageData) -> String {
        
        
        self.declare_css("header", r##"/* ===== HAMBURGER MENU ===== */

/* Hide checkbox */
#nav-toggle-button {
    display: none;
}

/* Hamburger button (hidden by default) */
#nav-label {
    display: none;
    cursor: pointer;
    z-index: 2000;
}

/* Hamburger icon lines */
.nav-toggle-symbol {
    position: relative;
    width: 35px;
    height: 3px;
    background: #B7472A;
    display: block;
    transition: all 0.2s ease;
}

.nav-toggle-symbol::before,
.nav-toggle-symbol::after {
    content: "";
    position: absolute;
    left: 0;
    width: 35px;
    height: 3px;
    background: #B7472A;
    transition: all 0.2s ease;
}

.nav-toggle-symbol::before {
    top: -10px;
}

.nav-toggle-symbol::after {
    top: 10px;
}

/* Transform to X when checked */
#nav-toggle-button:checked + #nav-label .nav-toggle-symbol {
    background: transparent;
}

#nav-toggle-button:checked + #nav-label .nav-toggle-symbol::before {
    top: 0;
    transform: rotate(135deg);
}

#nav-toggle-button:checked + #nav-label .nav-toggle-symbol::after {
    top: 0;
    transform: rotate(-135deg);
}

/* ===== MOBILE STYLES ===== */
@media screen and (max-width: 1000px) {
    
    /* Show hamburger on mobile */
    #nav-label {
        display: block;
    }
    
    /* Hide desktop nav by default */
    nav ul, nav ul ~ a {
        display: none;
    }
    
    
    
    /* Show nav when checkbox is checked */
    #nav-toggle-button:checked ~ ul,
    #nav-toggle-button:checked ~ .cta {
        display: block;
    }
    
    /* Full screen overlay when menu is open */
    header:has(#nav-toggle-button:checked) {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: var(--background-color);
        z-index: 1000;
    }
    
    /* Lock body scroll when menu is open */
    body:has(#nav-toggle-button:checked) {
        overflow: hidden;
    }
    
    /* Mobile menu styles */
    nav ul {
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 2rem;
        font-size: 2rem;
        font-weight: 600;
        height: 100vh;
        padding: 0;
        margin: 0;
    }
    
    nav ul li {
        list-style: none;
    }
    
    nav ul li a {
        display: block;
        padding: 0.6em 1.2em;
        border-radius: 100px;
        transition: all 0.2s ease;
    }
    
    /* Style the CTA button in mobile menu */
    nav .cta {
        margin-top: 1rem;
        padding: 1rem 2rem;
        border: 2px solid #B7472A;
        border-radius: 50px;
        text-align: center;
    }
    
    /* Smaller text on very small screens */
    @media screen and (max-width: 400px) {
        nav ul {
            font-size: 1.5rem;
        }
    }
}"##);
        
        let home_nav_item = NavWrappedItem { 
            name: "Home".to_owned(), 
            path: "/".to_owned(),
            content: r#"<img src="hawkwood-corporation-logo.svg" class="logo" alt="Hawkwood Corporation Logo">"#.to_owned(),
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
        

        format!(
            r####"
            
            <header>
            <nav aria-label="Main navigation">
                {home_link}                
                <ul>
                    {nav_links}
                </ul>
                {call_us_link}
                <input id="nav-toggle-button" type="checkbox">
                <label id="nav-label" for="nav-toggle-button"><span class="nav-toggle-symbol"></span></label>
            </nav>
            </header>
            
            "####
        )
    }

}
