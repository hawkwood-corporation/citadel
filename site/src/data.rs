pub mod site;
pub mod nav;
pub mod reviews;
pub mod treatments;

// Re-export everything so you can use data::SiteData instead of data::types::SiteData
pub use site::*;
pub use nav::*;
pub use reviews::*;
pub use treatments::*;