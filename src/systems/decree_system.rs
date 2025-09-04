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
            if let Some(content) = &mut page.foundation.content {
                // Replace CSS
                *content = content.replace("[CSS_POSITION]", &final_css);
                
                // Simple body injection
                if let Some(body_start) = content.find("<body") {
                    if let Some(close_pos) = content[body_start..].find('>') {
                        let insert_pos = body_start + close_pos + 1;
                        content.insert_str(insert_pos, &format!("\n{}", &self.placements.body_top_position));
                    }
                }
                
                if let Some(body_end) = content.rfind("</body>") {
                    content.insert_str(body_end, &format!("{}\n", &self.placements.body_bottom_position));
                }
                
                format_html(content);
            }
        }
        
        self.generate_sitemap();
        self.generate_robots_txt();
        
    }
}