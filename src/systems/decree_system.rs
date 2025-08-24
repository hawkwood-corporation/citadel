use crate::prelude::*;

impl<T: Hash + Eq + Clone, I> Site<T, I> {
    pub fn commence(&mut self) {
        self.construct_sections();
        self.create_pages();
        self.decree_across_pages();
        self.write_files();
        self.copy_assets();
    }

    pub fn decree_across_pages(&mut self) {
        let final_css = self.construct_css();
        
        for page in &mut self.pages {
            let page_data = &mut page.foundation;
            
            if let Some(content) = &mut page_data.content {
                *content = content.replace("[CSS_POSITION]", &final_css);
                format_html(content);
            }
        }
    }
}