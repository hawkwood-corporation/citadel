use crate::prelude::*;

impl<T, I> Site<T, I> {
    pub fn declare_decree(&mut self, from: &str, to: &str) -> &mut Self {
        self.decrees.push((from.to_owned(), to.to_owned()));
        self
    }
}


impl<T: Hash + Eq + Clone, I> Site<T, I> {
    pub fn commence(&mut self) {
        println!("Creating Pages");
        self.create_pages();
        println!("Decreeing Across Pages");
        self.decree_across_pages();
        println!("Writing Files");
        self.write_files();
        println!("Copying Assets");
        self.copy_assets();
    }
    
    pub fn decree_across_pages(&mut self) {
        let final_css = self.construct_css();
        
        self.decrees.push(("[CSS_POSITION]".to_owned(), final_css));
        
        for page in &mut self.pages {
            if let Some(content) = &mut page.foundation.content {
                
                // Apply all decrees
                for (from, to) in &self.decrees {
                    *content = content.replace(from, to);
                }
                
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