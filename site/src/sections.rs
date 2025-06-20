use crate::data::*;

pub struct Section {
    content: String,
}

fn google_reviews(reviews: &[GoogleReview]) -> String {
    let reviews_html = reviews.iter()
        .map(|review| {
            let stars = "★".repeat(review.rating as usize);
            let empty_stars = "☆".repeat(5 - review.rating as usize);
            
            format!(r#"
            <div class="google-review">
                <div class="review-header">
                    <span class="reviewer-name">{}</span>
                    <div class="rating">{}{}</div>
                    <span class="review-date">{}</span>
                </div>
                <div class="review-text">
                    <p>{}</p>
                </div>
            </div>"#, 
                review.reviewer_name, 
                stars, 
                empty_stars, 
                review.date, 
                review.review_text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"
    <section class="google-reviews">
        <h2>What Our Patients Say</h2>
        <div class="reviews-container">
            {}
        </div>
    </section>"#, reviews_html)
}