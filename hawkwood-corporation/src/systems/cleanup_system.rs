use crate::prelude::*;
use slugify::slugify;


pub fn clean_up_metadata(page: &mut PageFoundation) {
    if page.slug.is_none() {
        page.slug = Some(slugify!(&page.title))
    };
}

pub fn format_html(content: &mut String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut indent: usize = 0;
    let mut in_style_tag = false;
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() { 
            continue;
        }
        
        // Handle CSS blocks (only within style tags)
        if in_style_tag {
            if trimmed.contains('}') {
                indent = indent.saturating_sub(1);
            }
        }
        
        // Handle ALL closing tags the same way
        if trimmed.starts_with("</") {
            indent = indent.saturating_sub(1);
        }
        
        // Add the line with current indentation
        result.push_str(&"  ".repeat(indent));
        result.push_str(trimmed);
        result.push('\n');
        
        // Handle ALL opening tags the same way
        if let Some(tag_name) = extract_opening_tag(trimmed) {
            if has_closing_tag(&lines[i..], &tag_name) {
                indent += 1;
            }
        }
        
        // CSS block indentation (only in style tags)
        if in_style_tag && trimmed.contains('{') {
            indent += 1;
        }
        
        // Track style tag state AFTER processing
        if trimmed.contains("<style") {
            in_style_tag = true;
        }
        if trimmed.contains("</style>") {
            in_style_tag = false;
        }
    }
    
    *content = result;
}

fn extract_opening_tag(line: &str) -> Option<String> {
    if line.starts_with('<') 
       && !line.starts_with("</") 
       && !line.ends_with("/>")
       && !line.contains("<!") 
       && line.contains('>') {
        
        // Extract tag name (e.g., "div" from "<div class='foo'>")
        let start = line.find('<')? + 1;
        let end = line[start..].find(|c: char| c.is_whitespace() || c == '>')? + start;
        let tag_name = line[start..end].to_string();
        
        // Skip indenting for html tag
        if tag_name == "html" {
            return None;
        }
        
        // Check if closing tag is on the same line
        let closing_tag = format!("</{}>", tag_name);
        if line.contains(&closing_tag) {
            None  // Don't indent if opening and closing are on same line
        } else {
            Some(tag_name)
        }
    } else {
        None
    }
}

fn has_closing_tag(remaining_lines: &[&str], tag_name: &str) -> bool {
    let closing_tag = format!("</{}>", tag_name);
    remaining_lines.iter().any(|line| line.contains(&closing_tag))
}