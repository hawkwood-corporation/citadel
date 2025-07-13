use crate::prelude::*;
use slugify::slugify;


pub fn clean_up_metadata(page: &mut PageData) {
    if page.slug.is_none() {
        page.slug = Some(slugify!(&page.title))
    };
}
pub fn wrap_html_body(content: &mut String) {
    if let Some(head_end) = content.find("</head>") {
        let head_end_pos = head_end + "</head>".len();
        let (before_body, after_head) = content.split_at(head_end_pos);
        
        *content = format!(r####"
        
            <!DOCTYPE html>
            <html>
                {before_body}
                <body>
                    {after_head}
                </body>
            </html>
            
            "####,
            before_body = before_body,
            after_head = after_head
        );
    } else {
        // Fallback if no </head> found
        *content = format!(r####"
        
            <!DOCTYPE html>
            <html>
                <body>
                {}
                </body>
            </html>
            
            "####, content);
    }
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
        result.push_str(&"    ".repeat(indent));
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


/*pub fn format_html(content: &mut String) {
    let mut result = String::new();
    let mut indent: usize = 0;
    let mut in_tag = false;
    let mut in_style_tag = false;
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { 
            continue;  // Skip completely empty lines
        }
        
        // Track if we're inside a <style> tag
        if trimmed.contains("<style") {
            in_style_tag = true;
        }
        if trimmed.contains("</style>") {
            in_style_tag = false;
        }
        
        // Handle CSS blocks (only within style tags)
        if in_style_tag {
            if trimmed.contains('}') {
                indent = indent.saturating_sub(1);
            }
        }
        
        // Handle HTML tags (when not in style)
        if !in_style_tag {
            // Decrease indent for closing tags
            if trimmed.starts_with("</") {
                indent = indent.saturating_sub(1);
            }
            
            // Track multi-line HTML tags
            if trimmed.contains('<') && !trimmed.contains('>') {
                in_tag = true;
            }
            if in_tag && trimmed.contains('>') {
                in_tag = false;
            }
        }
        
        // Add the line with current indentation
        result.push_str(&"    ".repeat(indent));
        result.push_str(trimmed);
        result.push('\n');
        
        // Increase indent for opening elements
        if !in_style_tag && !in_tag {
            if trimmed.starts_with('<') 
               && !trimmed.starts_with("</") 
               && !trimmed.ends_with("/>")
               && !trimmed.contains("<!") 
               && trimmed.contains('>') {
                indent += 1;
            }
        }
        
        // CSS block indentation (only in style tags)
        if in_style_tag && trimmed.contains('{') {
            indent += 1;
        }
    }
    
    *content = result;
}*/