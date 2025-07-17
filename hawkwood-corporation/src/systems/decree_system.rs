use crate::prelude::*;

impl Site {
    
    pub fn decree_across_pages(&mut self) {
        
        
        let final_css = self.construct_css();
        
        self.decree_css(final_css);
        
        
        
    }
    
}