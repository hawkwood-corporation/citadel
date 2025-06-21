pub struct Site {
    pub title: String,
}

impl Site {
    pub fn new() -> Self {
        Self {
            title: "Hawkwood Corporation".to_owned(),
        }
    }
}