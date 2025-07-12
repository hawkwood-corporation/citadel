pub mod prelude;
mod sections {
    pub mod head;
    pub mod header;
    pub mod css;
}
mod site;
mod systems {
    pub mod nav_system;
    pub mod pages_system;
    pub mod sections_system;
    pub mod write_files_system;
    pub mod cleanup_system;
}

mod page_types {
    pub mod homepage;
    pub mod blog_post;
}