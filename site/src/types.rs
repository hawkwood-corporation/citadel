pub struct SiteData {
    // ... other fields
    google_reviews: Vec<GoogleReview>,
}

pub struct GoogleReview {
    reviewer_name: String,
    review_text: String,
    rating: u8, // 1-5 stars
    date: String, // "2 months ago" or whatever format you want
    reviewer_avatar: Option<String>, // URL to avatar image
}