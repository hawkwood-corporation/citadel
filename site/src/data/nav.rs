pub struct NavItem {
    pub name: String,
    pub url: String,
}

pub fn create_nav_items() -> Vec<NavItem> {
    vec![
        NavItem { name: "Operations".to_string(), url: "/operations/".to_string() },
        // ...
    ]
}