use crate::prelude::{css_system::Breakpoints, *};
use std::{collections::HashMap, path::PathBuf, hash::Hash};

pub struct Site<T> {
    pub title: String,
    pub base_url: Url,
    pub pages: Vec<Page<T>>,
    pub sections: Sections,
    pub css: HashMap<String, String>,
    pub breakpoints: Breakpoints,
    pub settings: Settings,
    pub page_constructors: HashMap<T, fn(&mut Site<T>, &mut Page<T>)>,
}

impl<T: Hash + Eq + Clone> Site<T> {
    pub fn example() -> Self {
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
            page_constructors: HashMap::new(),
        }
    }
}

pub struct Settings {
    pub output_folder: PathBuf,
}