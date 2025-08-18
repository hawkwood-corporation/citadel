/*
 * Citadel - Hawkwood Corporation's Sovereign Static Site Platform
 * Copyright (C) 2025 Jake Arthur / Hawkwood Corporation
 *
 * Built with Rust for perfect SEO and limitless control without frameworks.
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the The Citadel License. See the license file for details.
 */

#![allow(unused_imports)]
#[macro_use] extern crate slugify;

use citadel::prelude::*;



fn main() {
    
    let mut site = Site::example();
    
    let mut pages = vec![
            Page {
                foundation: PageFoundation { 
                    title: "Homepage".to_owned(),
                    ..Default::default() 
                },
                specification: PageSpecification::PillarPage(Pillar::Homepage),
            },
            Page {
                foundation: PageFoundation { 
                    title: "My First Post".to_owned(),
                    ..Default::default()
                },
                specification: PageSpecification::BlogPost {
                    date: Some("2025-01-15".to_owned()),
                    author: Some("Jake".to_owned()),
                },
            },
        ];
    
        site.commence();
    
}

