use crate::prelude::*;

pub struct Breakpoints {
    pub mobile: String,
}

impl<T> Site<T> {
    pub fn declare_css(&mut self, key: &str, css: &str) {
        let css = css.replace("{}", "");
        if !self.css.contains_key(key) {
            println!("Declared CSS: {}", key);
            self.css.insert(key.to_string(), css.to_string());
        }
    }

    pub fn construct_css(&self) -> String {
        let priority_order = [
            "foundation",
            "sovereign_colors",
            "layout",
            "typography",
            "header",
            "nav_toggle",
        ]; // Foundation stuff
        let mut result = Vec::new();

        // Priority CSS first (the foundational stuff)
        for &key in &priority_order {
            if let Some(css) = self.css.get(key) {
                result.push(css.clone());
            }
        }

        // Everything else alphabetically (well-scoped components)
        let mut remaining: Vec<_> = self
            .css
            .iter()
            .filter(|(key, _)| !priority_order.contains(&key.as_str()))
            .collect();
        remaining.sort_by_key(|(key, _)| *key);

        for (_, css) in remaining {
            result.push(css.clone());
        }

        let full_css = result.join("\n");

        let full_css = self.convert_tem_to_rem(full_css);

        let full_css = self.css_value_replacer(full_css);

        full_css
    }

    pub fn convert_tem_to_rem(&self, css: String) -> String {
        let parts: Vec<&str> = css.split("tem").collect();
        let mut result = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i == 0 {
                result.push_str(part);
            } else {
                // Get the previous part to find the number at its end
                let prev_part = &parts[i - 1];

                // Find the number at the end of previous part
                let mut num_start = prev_part.len();
                for ch in prev_part.chars().rev() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_start -= ch.len_utf8();
                    } else {
                        break;
                    }
                }

                if let Ok(pixels) = prev_part[num_start..].parse::<f32>() {
                    // Remove the number from result and add converted value
                    result.truncate(result.len() - (prev_part.len() - num_start));
                    result.push_str(&format!("{}rem", pixels / 16.0));
                } else {
                    // No number found, put "tem" back where it was
                    result.push_str("tem");
                }

                result.push_str(part);
            }
        }

        result
    }

    // True em system. 18tem = 18px but in rem
    pub fn convert_rem_to_tem(&self, css: String) -> String {
        let parts: Vec<&str> = css.split("rem").collect();
        let mut result = String::new();

        for (i, part) in parts.iter().enumerate() {
            if i == 0 {
                result.push_str(part);
            } else {
                // Get the previous part to find the number at its end
                let prev_part = &parts[i - 1];

                // Find the number at the end of previous part
                let mut num_start = prev_part.len();
                for ch in prev_part.chars().rev() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_start -= ch.len_utf8();
                    } else {
                        break;
                    }
                }

                if let Ok(pixels) = prev_part[num_start..].parse::<f32>() {
                    // Remove the number from result and add converted value
                    result.truncate(result.len() - (prev_part.len() - num_start));
                    result.push_str(&format!("{}tem", pixels * 16.0));
                } else {
                    // No number found, put "rem" back where it was
                    result.push_str("rem");
                }

                result.push_str(part);
            }
        }

        result
    }

    fn css_value_replacer(&self, css: String) -> String {
        css.replace("[mobile]", &self.breakpoints.mobile)
    }
}
