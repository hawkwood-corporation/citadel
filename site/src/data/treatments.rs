#[derive(Debug)]
pub struct NavItem {
    pub name: String,
    pub url: String,
}

pub fn create_nav_items() -> Vec<NavItem> {
    vec![
        NavItem { name: "Treatments".to_string(), url: "/treatments/".to_string() },
        // ...
    ]
}