use crate::prelude::*;

pub struct NavItem {
    pub name: String,
    pub path: String,
}

pub struct NavWrappedItem {
    pub name: String,
    pub path: String,
    pub content: String,
    pub class: Option<String>,
}


impl<T> Site<T> {
    
    pub fn construct_nav_url(&self, item: &NavItem) -> Url {
        self.base_url.join(&item.path).unwrap()
    }
    
    pub fn construct_nav_links(&self, nav_items: &[NavItem], page: &PageFoundation) -> String {

        let current_slug = page.slug.as_deref().unwrap_or("/");
        
        let nav_links: Vec<String> = nav_items
            .iter()
            .map(|item| {
                let is_current = self.is_current_page(&item.path, current_slug);
                let aria_current = if is_current { r#" aria-current="page""# } else { "" };
                let current_class = if is_current { " current" } else { "" };
                
                format!(
                    r#"<li><a href="{}" class="nav-link{}"{}>{}</a></li>"#,
                    item.path, current_class, aria_current, item.name
                )
            })
            .collect();


            nav_links.join("\n")

    }
    
    pub fn construct_nav_wrapped_link(&self, nav_item: &NavWrappedItem, page: &PageFoundation) -> String {
        
        let current_slug = page.slug.as_deref().unwrap_or("/");
        let is_current = self.is_current_page(&nav_item.path, current_slug);
        let aria_current = if is_current { r#" aria-current="page""# } else { "" };
        let current_class = if is_current { " current" } else { "" };
        let class = nav_item.class.as_ref().map(|c| format!(" {}", c)).unwrap_or_default();
        let title_attr = if nav_item.path == "" || nav_item.path == "/" {
            format!(r##" title="{}""##, self.title)
        } else {
            String::new()
        };
        
        format!(
            r#"<a href="{path}" class="nav-link{current}{class}"{aria}{title}>{content}</a>"#,
            path = nav_item.path,
            current = current_class, 
            class = class,
            aria = aria_current,
            title = title_attr,
            content = nav_item.content
        )
        
    }

    fn is_current_page(&self, nav_url: &str, current_slug: &str) -> bool {
        match nav_url {
            "/" => current_slug.is_empty() || current_slug == "/",
            url => {
                let nav_slug = url.trim_start_matches('/');
                current_slug == nav_slug || current_slug == url
            }
        }
    }
    
}