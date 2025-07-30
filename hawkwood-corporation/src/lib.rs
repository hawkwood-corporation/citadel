pub mod prelude;
mod sections {
    pub mod head;
    pub mod header;
    pub mod old_css;
    pub mod column_hero;
}
mod site;
mod systems {
    pub mod nav_system;
    pub mod pages_system;
    pub mod sections_system;
    pub mod write_files_system;
    pub mod cleanup_system;
    pub mod css_system;
    pub mod decree_system;
    pub mod assets_system;
}

mod page_types {
    pub mod homepage;
    pub mod blog_post;
}

mod parts {
    pub mod nav_toggle;
    pub mod directive_buttons;
}