pub mod site;
pub mod nav;
pub mod head;

// Re-export everything so you can use data::SiteData instead of data::types::SiteData
pub use site::*;
pub use nav::*;
pub use head::*;