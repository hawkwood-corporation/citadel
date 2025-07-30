use crate::prelude::{css_system::Breakpoints, *};
use std::{collections::HashMap, path::PathBuf/*, sync::Mutex*/};

pub struct Site {
    pub title: String,
    pub base_url: Url,
    pub pages: Vec<Page>,
    pub sections: Sections,
    pub css: HashMap<String, String>,
    pub breakpoints: Breakpoints,
    pub settings: Settings,
}

impl Site {
    pub fn new() -> Self {
        Self {
            title: "Hawkwood Corporation".to_owned(),
            base_url: Url::parse("https://hawkwoodcorporation.com/").expect("Invalid base URL"),
            pages: Vec::new(),
            sections: Sections::new(),
            css: HashMap::new(),
            breakpoints: Breakpoints {
                mobile: "1000px".to_owned(), 
            },
            settings: Settings {
                output_folder: PathBuf::from("public"),
            },
        }
    }
}

pub struct Settings {
    pub output_folder: PathBuf,
}
