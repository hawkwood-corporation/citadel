use crate::citadel::*;

pub struct Site {
    pub title: String,
    pub pages: Vec<PageData>,
}

impl Site {
    pub fn new() -> Self {
        Self {
            title: "Hawkwood Corporation".to_owned(),
            pages: Vec::new(),
        }
    }
}