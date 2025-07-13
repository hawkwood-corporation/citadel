use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_header(&self, page: &PageData) -> String {
        let nav_items = self.create_nav_items();
        let nav = self.construct_nav(&nav_items);

        format!(
            r####"
            
            <header>
                <nav class="container">
                    <a href="#" class="logo">HAWKWOOD CORPORATION</a>
                    {nav}
                </nav>
            </header>
            
            "####
        )
    }

    pub fn construct_nav(&self, nav_items: &[NavItem]) -> String {
        let nav_links: Vec<String> = nav_items
            .iter()
            .map(|item| format!(r#"<li><a href="{}">{}</a></li>"#, item.url, item.name))
            .collect();

        format!(
            r####"
            <ul class="nav-links">
                {}
            </ul>"####,
            nav_links.join("\n                ")
        )
    }
}
