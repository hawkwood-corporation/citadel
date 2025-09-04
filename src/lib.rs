/*
 * Citadel - Hawkwood Corporation's Sovereign Static Site Platform
 * Copyright (C) 2025 Jake Arthur / Hawkwood Corporation
 *
 * Built with Rust for perfect SEO and limitless control without frameworks.
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the The Citadel License. See the license file for details.
 */
 
pub mod prelude;
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
    pub mod head_system;
    pub mod placements_system;
    pub mod sitemap_system;
}
