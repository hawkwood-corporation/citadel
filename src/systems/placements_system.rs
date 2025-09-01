use crate::prelude::*;

/// Sovereign placement system for injecting content at specific template positions
pub struct Placements {
    /// Font loading and preconnect links
    pub fonts_position: String,
    
    /// Schema markup and structured data
    pub schema_position: String,
    
    /// Content at the top of head (after meta tags)
    pub head_top_position: String,
    
    /// Additional head content at bottom (before closing head tag)
    pub head_bottom_position: String,
    
    /// Content that needs to be at the very top of <body>
    pub body_top_position: String,
    
    /// Analytics and tracking scripts
    pub analytics_position: String,
    
    /// Additional scripts before closing </body> tag
    pub scripts_position: String,
    
    /// Content before closing </body> tag
    pub body_bottom_position: String,
}

impl Default for Placements {
    fn default() -> Self {
        Self {
            fonts_position: String::new(),
            schema_position: String::new(),
            head_top_position: String::new(),
            head_bottom_position: String::new(),
            body_top_position: String::new(),
            analytics_position: String::new(),
            scripts_position: String::new(),
            body_bottom_position: String::new(),
        }
    }
}

impl Placements {
    /// Create new empty placements
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add fonts to the fonts position
    pub fn add_fonts(&mut self, fonts: &str) {
        if !self.fonts_position.is_empty() {
            self.fonts_position.push('\n');
        }
        self.fonts_position.push_str(fonts);
    }
    
    /// Add schema markup content
    pub fn add_schema(&mut self, content: &str) {
        if !self.schema_position.is_empty() {
            self.schema_position.push('\n');
        }
        self.schema_position.push_str(content);
    }
    
    /// Add content to head top position
    pub fn add_head_top(&mut self, content: &str) {
        if !self.head_top_position.is_empty() {
            self.head_top_position.push('\n');
        }
        self.head_top_position.push_str(content);
    }
    
    /// Add content to head bottom position
    pub fn add_head_bottom(&mut self, content: &str) {
        if !self.head_bottom_position.is_empty() {
            self.head_bottom_position.push('\n');
        }
        self.head_bottom_position.push_str(content);
    }
    
    /// Add content to body top position  
    pub fn add_body_top(&mut self, content: &str) {
        if !self.body_top_position.is_empty() {
            self.body_top_position.push('\n');
        }
        self.body_top_position.push_str(content);
    }
    
    /// Add analytics content
    pub fn add_analytics(&mut self, content: &str) {
        if !self.analytics_position.is_empty() {
            self.analytics_position.push('\n');
        }
        self.analytics_position.push_str(content);
    }
    
    pub fn add_scripts(&mut self, content: &str) {
        if !self.scripts_position.is_empty() {
            self.scripts_position.push('\n');
        }
        self.scripts_position.push_str(content);
    }
    
    /// Add content to body bottom position
    pub fn add_body_bottom(&mut self, content: &str) {
        if !self.body_bottom_position.is_empty() {
            self.body_bottom_position.push('\n');
        }
        self.body_bottom_position.push_str(content);
    }
}

impl<T, I> Site<T, I> {
    /// Declare content for a specific placement position
    pub fn declare_placement(&mut self, position: PlacementPosition, content: &str) {
        match position {
            PlacementPosition::Fonts => self.placements.add_fonts(content),
            PlacementPosition::Schema => self.placements.add_schema(content),
            PlacementPosition::HeadTop => self.placements.add_head_top(content),
            PlacementPosition::HeadBottom => self.placements.add_head_bottom(content),
            PlacementPosition::BodyTop => self.placements.add_body_top(content),
            PlacementPosition::Analytics => self.placements.add_analytics(content),
            PlacementPosition::Scripts => self.placements.add_scripts(content),
            PlacementPosition::BodyBottom => self.placements.add_body_bottom(content),
        }
    }
    
    pub fn combine_placements(&self, site_content: &str, page_content: &str) -> String {
        match (site_content.is_empty(), page_content.is_empty()) {
            (true, true) => String::new(),
            (false, true) => site_content.to_owned(),
            (true, false) => page_content.to_owned(),
            (false, false) => format!("{}\n{}", site_content, page_content),
        }
    }

}

impl<T> Page<T> {
    /// Declare placement content for this specific page
    pub fn declare_placement(&mut self, position: PlacementPosition, content: &str) {
        match position {
            PlacementPosition::Fonts => self.foundation.placements.add_fonts(content),
            PlacementPosition::Schema => self.foundation.placements.add_schema(content),
            PlacementPosition::HeadTop => self.foundation.placements.add_head_top(content),
            PlacementPosition::HeadBottom => self.foundation.placements.add_head_bottom(content),
            PlacementPosition::BodyTop => self.foundation.placements.add_body_top(content),
            PlacementPosition::Analytics => self.foundation.placements.add_analytics(content),
            PlacementPosition::Scripts => self.foundation.placements.add_scripts(content),
            PlacementPosition::BodyBottom => self.foundation.placements.add_body_bottom(content),
        }
    }
}

/// Enum for specifying placement positions
#[derive(Debug, Clone, Copy)]
pub enum PlacementPosition {
    Fonts,
    Schema,
    HeadTop,
    HeadBottom,
    BodyTop,
    Analytics,
    Scripts,
    BodyBottom,
}

