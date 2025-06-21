pub struct SiteData {
    pub title: String,
}


pub fn create_site_data() -> SiteData {
    SiteData {
        title: "Hawkwood Corporation".to_owned(),
    }   
}