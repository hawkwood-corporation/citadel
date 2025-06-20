
pub struct GoogleReview {
    pub reviewer_name: String,
    pub review_text: String,
    pub rating: u8, // 1-5 stars
    pub date: String, // "2 months ago" or whatever format you want
    pub reviewer_avatar: Option<String>, // URL to avatar image
}



pub fn create_reviews_data() -> Vec<GoogleReview> {
    vec![
        GoogleReview {
            reviewer_name: "Jessica Joslin-Smiley".to_string(),
            review_text: "I came here for Invisalign...".to_string(),
            rating: 5,
            date: "1 year ago".to_string(),
            reviewer_avatar: None,
        },
        // ...
    ]
}