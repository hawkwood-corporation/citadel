pub struct SiteData {
    title: String,
}


pub fn create_site_data() -> SiteData {
    SiteData {
        title: String::from("Hawkwood Corporation"),
    }   
}