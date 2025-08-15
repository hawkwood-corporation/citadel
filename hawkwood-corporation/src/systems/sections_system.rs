#[allow(unused_imports)]
use crate::prelude::*;

pub struct Section {
    pub content: String,
}

/// Static, reusable sections
pub struct Sections {
    pub main_wrap_open: String,
    pub main_wrap_close: String,
}

impl Sections {
    pub fn new() -> Self {
        Self {
            main_wrap_open: String::new(),
            main_wrap_close: String::new(),
        }
    }
}

impl Site {
    pub fn construct_sections(&mut self) {
        // Clean, focused section creation
        self.sections.main_wrap_open = r####"
<div class="site-container">
    <main>
    "####
            .to_owned();

        self.sections.main_wrap_close = r####"
    </main>
</div>
    "####
            .to_owned();
    }
}
