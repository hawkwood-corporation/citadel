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
    pub decrees: Vec<(String, String)>,
}

impl<T: Hash + Eq + Clone, I: Default> Default for Site<T, I> {
    fn default() -> Self {
        Self {
            title: "Citadel Site".to_owned(),
            base_url: Url::parse("https://example.com/").expect("Invalid base URL"),
            pages: Vec::new(),
            sections: Sections::new(),
            css: HashMap::new(),
            placements: Placements::new(),
            breakpoints: Breakpoints {
                mobile: "1000px".to_owned(), 
            },
            settings: Settings::default(),
            imperium: I::default(),
            page_constructors: HashMap::new(),
            head_constructor: None,
            decrees: Vec::new(),
        }
    }
}

impl<T: Hash + Eq + Clone, I> Site<T, I> {
    pub fn example(imperium: I) -> Self 
    where 
        I: Default 
    {
        Self {
            title: "Hawkwood Corporation".to_owned(),
            base_url: Url::parse("https://hawkwoodcorporation.com/").expect("Invalid base URL"),
            imperium,
            ..Self::default()
        }
    }
    
    pub fn new(title: String, base_url: Url, imperium: I) -> Self 
    where 
        I: Default 
    {
        Self {
            title,
            base_url,
            imperium,
            ..Self::default()
        }
    }
}


pub struct Settings {
    pub output_folder: PathBuf,
    pub verbose_assets_copying: bool,
    pub title_append: Option<String>,
    pub use_trailing_slashes: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            output_folder: PathBuf::from("public"),
            verbose_assets_copying: false,
            //title_append: Some(" - {site_title}".to_owned()),
            title_append: None,
            use_trailing_slashes: true,
        }
    }
}

/// Convenience function for Default::default()
pub fn default<T: Default>() -> T {
    Default::default()
}