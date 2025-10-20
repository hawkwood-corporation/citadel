//use crate::prelude::*;
use std::fs;
//use std::path::Path;

/// Generic content data that flows through Citadel
#[derive(Debug, Clone, PartialEq, Hash, Eq, Default)]
pub struct PostData<F> {
    pub slug: String,
    pub frontmatter: F,
    pub content: String,
    pub raw_content: String,
}

/// Load content from directory with your parser
pub fn load_content<F>(
    dir: &str, 
    parser: fn(&str) -> Result<(F, String), Box<dyn std::error::Error>>
) -> Vec<PostData<F>> {
    let mut items = Vec::new();
    scan_directory(dir, &mut items, parser);
    items
}

/// Scan directory recursively for markdown files
fn scan_directory<F>(
    dir_path: &str, 
    items: &mut Vec<PostData<F>>,
    parser: fn(&str) -> Result<(F, String), Box<dyn std::error::Error>>
) {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    scan_directory(path_str, items, parser);
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                // Create slug from relative path
                let relative_path = path.strip_prefix("content/").unwrap_or(&path);
                let mut slug = relative_path
                    .with_extension("")
                    .to_string_lossy()
                    .replace('\\', "/");
                
                // Handle index.md files - use parent folder as slug
                if slug.ends_with("/index") {
                    slug = slug.trim_end_matches("/index").to_owned();
                }
                
                if let Ok(raw) = fs::read_to_string(&path) {
                    if let Ok((frontmatter, markdown)) = parser(&raw) {
                        items.push(PostData {
                            slug,
                            frontmatter,
                            content: render_markdown(&markdown),
                            raw_content: markdown,
                        });
                    }
                }
            }
        }
    }
}

/// Simple markdown to HTML renderer
pub fn render_markdown(markdown: &str) -> String {
    #[cfg(feature = "markdown")]
    {
        use pulldown_cmark::{Parser, Options, html};
        
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        
        let parser = Parser::new_ext(markdown, options);
        let mut html_content = String::new();
        html::push_html(&mut html_content, parser);
        html_content
    }
    
    #[cfg(not(feature = "markdown"))]
    markdown.to_owned()
}

// ===== Example Blog Implementation =====

/// Example blog post frontmatter
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ExampleBlogFrontmatter {
    pub title: String,
    pub description: String,
    pub author: String,
    pub date_published: String,
}

/// Example TOML parser for blog posts
pub fn citadel_parse_example_frontmatter(content: &str) -> Result<(ExampleBlogFrontmatter, String), Box<dyn std::error::Error>> {
    if !content.starts_with("+++") {
        return Err("No TOML frontmatter found".into());
    }
    
    let end_marker = content[3..].find("+++").ok_or("No closing frontmatter marker")?;
    let frontmatter_content = &content[3..end_marker + 3];
    let markdown_content = content[end_marker + 6..].trim_start();
    
    let mut frontmatter = ExampleBlogFrontmatter::default();
    
    for line in frontmatter_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"').trim_matches('\'');
            
            match key {
                "title" => frontmatter.title = value.to_owned(),
                "description" => frontmatter.description = value.to_owned(),
                "author" => frontmatter.author = value.to_owned(),
                "date_published" => frontmatter.date_published = value.to_owned(),
                _ => {}
            }
        }
    }
    
    Ok((frontmatter, markdown_content.to_owned()))
}

/// Convenience function for loading blog posts specifically
pub fn citadel_get_all_posts() -> Vec<PostData<ExampleBlogFrontmatter>> {
    let mut posts = load_content("content", citadel_parse_example_frontmatter);
    
    // Sort by date (newest first)
    posts.sort_by(|a, b| {
        b.frontmatter.date_published.cmp(&a.frontmatter.date_published)
    });
    
    posts
}