use crate::prelude::*;
use slugify::slugify;


pub fn clean_up_metadata(page: &mut PageData) {
    if page.slug.is_none() {
        page.slug = Some(slugify!(&page.title))
    };
}

pub fn wrap_html_body(content: &mut String) {
    *content = format!("<!DOCTYPE html><html><body>{}</body></html>", content);
}

