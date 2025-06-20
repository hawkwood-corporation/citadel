pub mod site;
pub mod nav;

// Re-export everything so you can use data::SiteData instead of data::types::SiteData
pub use site::*;
pub use nav::*;