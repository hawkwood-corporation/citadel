pub struct NavItem {
    pub name: String,
    pub url: String,
}

pub fn create_nav_items() -> Vec<NavItem> {
    vec![
        NavItem { name: "Operations".to_owned(), url: "/operations/".to_owned() },
        // ...
    ]
}