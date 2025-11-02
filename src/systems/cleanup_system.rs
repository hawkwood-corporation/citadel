use crate::prelude::*;
use slugify::slugify;


pub fn clean_up_metadata(page: &mut PageFoundation) {
    if page.slug.is_none() {
        page.slug = Some(slugify!(&page.title))
    };
}

pub fn format_html(content: &mut String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::with_capacity(content.len() + lines.len() * 2); // Preallocate
    let mut indent: usize = 0;
    let mut in_style_tag = false;
    
    // Pre-scan to build a map of opening tags to their closing positions
    let mut tag_stack: Vec<(String, usize)> = Vec::new();
    let mut closing_positions: std::collections::HashMap<usize, String> = std::collections::HashMap::new();
    
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        
        // Track opening tags
        if let Some(tag_name) = extract_opening_tag(trimmed) {
            tag_stack.push((tag_name.clone(), i));
        }
        
        // Match closing tags
        if trimmed.starts_with("</") {
            if let Some(start_pos) = trimmed.find("</") {
                if let Some(end_pos) = trimmed[start_pos+2..].find('>') {
                    let tag_name = trimmed[start_pos+2..start_pos+2+end_pos].to_owned();
                    
                    // Pop matching opening tag
                    if let Some((open_tag, open_line)) = tag_stack.pop() {
                        if open_tag == tag_name {
                            closing_positions.insert(open_line, tag_name);
                        }
                    }
                }
            }
        }
    }
    
    // Now format with O(1) lookups
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() { 
            continue;
        }
        
        // Handle CSS blocks
        if in_style_tag && trimmed.contains('}') {
            indent = indent.saturating_sub(1);
        }
        
        // Handle closing tags
        if trimmed.starts_with("</") {
            indent = indent.saturating_sub(1);
        }
        
        // Add the line
        result.push_str(&"  ".repeat(indent));
        result.push_str(trimmed);
        result.push('\n');
        
        // Handle opening tags - O(1) lookup now!
        /*if let Some(tag_name) = extract_opening_tag(trimmed) {
            if closing_positions.contains_key(&i) {
                indent += 1;
            }
        }*/
        
        // Handle opening tags - O(1) lookup now!
        if extract_opening_tag(trimmed).is_some() && closing_positions.contains_key(&i) {
            indent += 1;
        }
        
        // CSS blocks
        if in_style_tag && trimmed.contains('{') {
            indent += 1;
        }
        
        // Track style tags
        if trimmed.contains("<style") {
            in_style_tag = true;
        }
        if trimmed.contains("</style>") {
            in_style_tag = false;
        }
    }
    
    *content = result;
}

pub fn old_format_html(content: &mut String) {
    println!("Formatting HTML");
    let lines: Vec<&str> = content.lines().collect();
    let mut result = String::new();
    let mut indent: usize = 0;
    let mut in_style_tag = false;
    
    for (i, line) in lines.iter().enumerate() {
        println!("Processing line: {}", i);
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