use crate::prelude::{css_system::Breakpoints, *};
use std::{collections::HashMap, path::PathBuf, hash::Hash};

pub struct Site<T, I = ()> {
    pub title: String,
    pub base_url: Url,
    pub pages: Vec<Page<T>>,
    pub sections: Sections,
    pub css: HashMap<String, String>,
    pub placements: Placements,
    pub breakpoints: Breakpoints,
    pub settings: Settings,
    pub imperium: I,
    pub page_constructors: HashMap<T, fn(&mut Site<T, I>, &mut Page<T>)>,
    pub head_constructor: Option<fn(&mut Site<T, I>, &Page<T>) -> String>,
}

impl<T: Hash + Eq + Clone, I> Site<T, I> {
    pub fn example(imperium: I) -> Self {
        Self {
            title: "Hawkwood Corporation".to_owned(),
            base_url: Url::parse("https://hawkwoodcorporation.com/").expect("Invalid base URL"),
            pages: Vec::new(),
            sections: Sections::new(),
            css: HashMap::new(),
            placements: Placements::new(),
            breakpoints: Breakpoints {
                mobile: "1000px".to_owned(), 
            },
            settings: Settings {
                output_folder: PathBuf::from("public"),
            },
            imperium,
            page_constructors: HashMap::new(),
            head_constructor: None,
        }
    }
    
    pub fn new(title: String, base_url: Url, imperium: I) -> Self {
        Self {
            title,
            base_url,
            pages: Vec::new(),
            sections: Sections::new(),
            css: HashMap::new(),
            breakpoints: Breakpoints {
                mobile: "1000px".to_owned(), 
            },
            placements: Placements::new(),
            settings: Settings {
                output_folder: PathBuf::from("public"),
            },
            imperium,
            page_constructors: HashMap::new(),
            head_constructor: None,
        }
    }
}

pub struct Settings {
    pub output_folder: PathBuf,
}