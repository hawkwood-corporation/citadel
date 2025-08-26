use crate::prelude::*;

/// Sovereign placement system for injecting content at specific template positions
pub struct Placements {
    /// Font loading and preconnect links
    pub fonts_position: String,
    
    /// Additional head content (schema markup, meta tags, etc.)
    pub head_bottom_position: String,
    
    /// Content that needs to be at the very top of <body>
    pub body_top_position: String,
    
    /// Analytics and tracking scripts
    pub analytics_position: String,
    
    /// Content before closing </body> tag
    pub body_bottom_position: String,
}

impl Default for Placements {
    fn default() -> Self {
        Self {
            fonts_position: String::new(),
            head_bottom_position: String::new(),
            body_top_position: String::new(),
            analytics_position: String::new(),
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
    
    /// Add content to head top position
    pub fn add_head_top(&mut self, content: &str) {
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
            PlacementPosition::HeadTop => self.placements.add_head_top(content),
            PlacementPosition::BodyTop => self.placements.add_body_top(content),
            PlacementPosition::Analytics => self.placements.add_analytics(content),
            PlacementPosition::BodyBottom => self.placements.add_body_bottom(content),
        }
    }
}

/// Enum for specifying placement positions
#[derive(Debug, Clone, Copy)]
pub enum PlacementPosition {
    Fonts,
    HeadTop,
    BodyTop,
    Analytics,
    BodyBottom,
}