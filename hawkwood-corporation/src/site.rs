use crate::prelude::*;
use std::{collections::HashMap, path::PathBuf/*, sync::Mutex*/};

pub struct Site {
    pub title: String,
    pub pages: Vec<Page>,
    pub sections: Sections,
    pub css: HashMap<String, String>,
    pub settings: Settings,
}

impl Site {
    pub fn new() -> Self {
        Self {
            title: "Hawkwood Corporation".to_owned(),
            pages: Vec::new(),
            sections: Sections::new(),
            css: HashMap::new(),
            settings: Settings {
                output_folder: PathBuf::from("public"),
            },
        }
    }
}

pub struct Settings {
    pub output_folder: PathBuf,
}
