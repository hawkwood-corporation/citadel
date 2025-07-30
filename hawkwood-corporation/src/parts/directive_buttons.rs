use crate::prelude::*;

#[allow(unused_variables)]

pub struct Directive {
    pub href: String,
    pub text: String,
    pub symbol: String,
    pub class: Option<String>,
}

impl Site {
    pub fn construct_directive_button(&mut self, directive: &Directive) -> String {
        
        self.declare_css("directive_button", r##"
        
/* ===== DIRECTIVE BUTTONS ===== */

.directive {
    border: 1px solid var(--smoky-black);
    padding: 12px 30px;
    display: flex;
    gap: 10tem;
    color: var(--smoky-black);
    transition: all 0.3s ease; /* Smooth color/background fade */
    width: fit-content;
    font-size: 18tem;
    text-decoration: none;
    text-wrap: nowrap;
    
    &:hover {
        background-color: var(--smoky-black);
        color: white;
        
            .symbol {
                transform: rotate(30deg); /* Clockwise rotation */
            }
            .upper-right-arrow {
                transform: rotate(45deg);
            }
        }
        
    }
    
    .symbol {
        width: 20tem;
        fill: currentColor;
        stroke: currentColor;
        transition: all 0.3s ease; /* Smooth fill AND transform */
        transform-origin: center; /* Rotate from center */
    }
    
    
}


}"##);
        
        let href = &directive.href;
        let text = &directive.text;
        //let symbol = format!(" {}", &directive.symbol);
        let symbol = &directive.symbol;
        let class = directive.class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default();

        format!(
            r####"
            
            <a href="{href}" class="directive{class}">{text}{symbol}</a>
            
            
            "####
        )
    }

}
