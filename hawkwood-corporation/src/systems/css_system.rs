use crate::prelude::*;

impl Site {
    
    pub fn declare_css(&mut self, key: &str, css: &str) {
        if !self.css.contains_key(key) {
            self.css.insert(key.to_string(), css.to_string());
        }
    }
    
    pub fn construct_css(&self) -> String {
        let priority_order = ["foundation", "layout", "typography"]; // Foundation stuff
        let mut result = Vec::new();
        
        // Priority CSS first (the foundational stuff)
        for &key in &priority_order {
            if let Some(css) = self.css.get(key) {
                result.push(css.clone());
            }
        }
        
        // Everything else alphabetically (well-scoped components)
        let mut remaining: Vec<_> = self.css.iter()
            .filter(|(key, _)| !priority_order.contains(&key.as_str()))
            .collect();
        remaining.sort_by_key(|(key, _)| *key);
        
        for (_, css) in remaining {
            result.push(css.clone());
        }
        
        result.join("\n")
    }
    
    pub fn decree_css(&mut self, final_css: String) {
        
        for page in &mut self.pages {
            match page {
                Page::Homepage { page } => {
                    if let Some(content) = &mut page.content {
                        *content = content.replace("[CSS_POSITION]", &final_css);
                    }
                },
                Page::BlogPost { page } => {
                    if let Some(content) = &mut page.content {
                        *content = content.replace("[CSS_POSITION]", &final_css);
                    }
                }
            }
        }
    }
    
}