use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_nav_toggle(&mut self) -> String {
        
        self.declare_css("nav_toggle", r##"
        
/* ===== NAV TOGGLE BUTTON ===== */

/* Hide checkbox */
#nav-toggle-button {
    display: none;
}

/* Nav toggle button (hidden by default) */
#nav-label {
    display: none;
    cursor: pointer;
    z-index: 2000;
}

/* Nav toggle symbol lines */
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

/* ===== NAV TOGGLE BUTTON MOBILE ===== */
@media screen and (max-width: 600px) {
    
    /* Show nav toggle on mobile */
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

        format!(
            r####"
            
            <input id="nav-toggle-button" type="checkbox">
            <label id="nav-label" for="nav-toggle-button"><span class="nav-toggle-symbol"></span></label>
            
            "####
        )
    }

}
