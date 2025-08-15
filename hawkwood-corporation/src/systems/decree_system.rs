use crate::prelude::*;

impl Site {
    
    pub fn decree_across_pages(&mut self) {
        
        let final_css = self.construct_css();
        
        //println!("{}",final_css);
        
        for page in &mut self.pages {
            let page_data = &mut page.foundation;
            
            if let Some(content) = &mut page_data.content {
                // Replace CSS placeholder
                *content = content.replace("[CSS_POSITION]", &final_css);
                // Format HTML
                format_html(content);
            }
        }
    }
    
}