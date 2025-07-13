use crate::prelude::*;

#[allow(dead_code)]
#[allow(unused_variables)]

impl Site {
    
    pub fn construct_css_stylesheet(&self) -> String {
        
        let mut css = 
            r####"
            
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }
            
            /* 1rem = 1px Trick */

            /*html {
                font-size: 6.25%;
            }

            body {
                font-size: 16em;
            }*/
            
            

            body {
                font-family: 'Georgia', 'Times New Roman', serif;
                background: #fefefe;
                color: #2d2d2d;
                line-height: 1.7;
            }

            .container {
                max-width: 1200px;
                margin: 0 auto;
                padding: 0 30px;
            }

            /* Header */
            header {
                background: #ffffff;
                padding: 25px 0;
                border-bottom: 1px solid #e8e5e2;
                box-shadow: 0 2px 8px rgba(0,0,0,0.04);
            }

            nav {
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .logo {
                font-size: 28px;
                font-weight: 700;
                color: #B7472A;
                text-decoration: none;
                letter-spacing: 1px;
            }

            .nav-links {
                display: flex;
                list-style: none;
                gap: 50px;
            }

            .nav-links a {
                color: #4a4a4a;
                text-decoration: none;
                font-weight: 500;
                font-size: 16px;
                transition: color 0.3s;
            }

            .nav-links a:hover {
                color: #B7472A;
            }

            /* Hero Section */
            .hero {
                padding: 120px 0 100px;
                text-align: center;
                background: linear-gradient(135deg, #fefefe 0%, #f9f8f6 100%);
            }

            .hero h1 {
                font-size: 3.8rem;
                font-weight: 700;
                margin-bottom: 25px;
                color: #B7472A;
                letter-spacing: -1px;
                line-height: 1.2;
            }

            .hero .subtitle {
                font-size: 1.4rem;
                color: #6B7280;
                margin-bottom: 50px;
                font-weight: 400;
                max-width: 700px;
                margin-left: auto;
                margin-right: auto;
            }

            .cta-button {
                background: #B7472A;
                color: #ffffff;
                padding: 20px 45px;
                font-size: 18px;
                font-weight: 600;
                text-decoration: none;
                border-radius: 6px;
                transition: all 0.3s;
                display: inline-block;
                box-shadow: 0 4px 12px rgba(183, 71, 42, 0.3);
            }

            .cta-button:hover {
                background: #9A3B23;
                transform: translateY(-2px);
                box-shadow: 0 6px 20px rgba(183, 71, 42, 0.4);
            }

            /* Section Styling */
            .section {
                padding: 100px 0;
            }

            .section-sage {
                background: #F6F8F5;
            }

            .section h2 {
                font-size: 2.8rem;
                font-weight: 700;
                margin-bottom: 25px;
                color: #B7472A;
                text-align: center;
            }

            .section-intro {
                font-size: 1.3rem;
                color: #5B6B73;
                text-align: center;
                max-width: 900px;
                margin: 0 auto 80px;
                line-height: 1.6;
            }

            /* Services Grid */
            .services-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
                gap: 50px;
                margin-top: 80px;
            }

            .service-card {
                background: #ffffff;
                padding: 50px 40px;
                border-radius: 8px;
                border: 1px solid #e8e5e2;
                transition: all 0.3s;
                box-shadow: 0 4px 15px rgba(0,0,0,0.05);
            }

            .service-card:hover {
                border-color: #87A96B;
                transform: translateY(-5px);
                box-shadow: 0 8px 25px rgba(0,0,0,0.1);
            }

            .service-card h3 {
                font-size: 1.6rem;
                color: #87A96B;
                margin-bottom: 20px;
                font-weight: 600;
            }

            .service-card p {
                color: #5B6B73;
                font-size: 1.1rem;
                text-align: left;
                margin: 0;
                line-height: 1.6;
            }

            /* Performance Metrics */
            .metrics {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                gap: 60px;
                margin-top: 80px;
            }

            .metric {
                text-align: center;
                padding: 40px 20px;
                background: #ffffff;
                border-radius: 8px;
                box-shadow: 0 4px 15px rgba(0,0,0,0.05);
            }

            .metric-number {
                font-size: 3.5rem;
                font-weight: 800;
                color: #B7472A;
                display: block;
                margin-bottom: 10px;
            }

            .metric-label {
                font-size: 1rem;
                color: #6B7280;
                font-weight: 500;
                text-transform: uppercase;
                letter-spacing: 1px;
            }

            .metric-description {
                font-size: 0.95rem;
                color: #87A96B;
                margin-top: 8px;
                font-style: italic;
            }

            /* About Section */
            .about-content {
                display: grid;
                grid-template-columns: 1fr 2fr;
                gap: 80px;
                align-items: center;
                margin-top: 60px;
            }

            .profile-image {
                width: 100%;
                max-width: 300px;
                border-radius: 8px;
                box-shadow: 0 8px 25px rgba(0,0,0,0.15);
            }

            .about-text {
                font-size: 1.2rem;
                color: #4a4a4a;
                line-height: 1.7;
            }

            .about-text h3 {
                color: #B7472A;
                margin-bottom: 20px;
                font-size: 1.8rem;
            }

            /* Contact Section */
            .contact {
                background: #87A96B;
                color: #ffffff;
                text-align: center;
            }

            .contact h2 {
                color: #ffffff;
            }

            .contact-form {
                max-width: 600px;
                margin: 0 auto;
                margin-top: 60px;
            }

            .form-row {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 30px;
                margin-bottom: 30px;
            }

            .form-group {
                text-align: left;
            }

            .form-group.full-width {
                grid-column: 1 / -1;
            }

            .form-group label {
                display: block;
                margin-bottom: 8px;
                color: #ffffff;
                font-weight: 500;
                font-size: 15px;
            }

            .form-group input,
            .form-group select,
            .form-group textarea {
                width: 100%;
                padding: 18px;
                background: rgba(255,255,255,0.1);
                border: 1px solid rgba(255,255,255,0.3);
                border-radius: 6px;
                color: #ffffff;
                font-size: 16px;
                transition: all 0.3s;
            }

            .form-group input::placeholder,
            .form-group textarea::placeholder {
                color: rgba(255,255,255,0.7);
            }

            .form-group input:focus,
            .form-group select:focus,
            .form-group textarea:focus {
                outline: none;
                border-color: #B7472A;
                background: rgba(255,255,255,0.15);
            }

            .form-group select option {
                background: #87A96B;
                color: #ffffff;
            }

            /* Footer */
            footer {
                background: #ffffff;
                padding: 60px 0;
                text-align: center;
                border-top: 1px solid #e8e5e2;
            }

            footer p {
                color: #6B7280;
                margin: 0;
                font-size: 15px;
            }

            /* Responsive Design */
            @media (max-width: 768px) {
                .hero h1 {
                    font-size: 2.8rem;
                }
                
                .hero .subtitle {
                    font-size: 1.2rem;
                }
                
                .nav-links {
                    display: none;
                }

                .about-content {
                    grid-template-columns: 1fr;
                    text-align: center;
                }

                .form-row {
                    grid-template-columns: 1fr;
                }

                .services-grid {
                    grid-template-columns: 1fr;
                }

                .metrics {
                    grid-template-columns: repeat(2, 1fr);
                }
            }

            /* Additional Premium Touches */
            .section-divider {
                height: 1px;
                background: linear-gradient(90deg, transparent, #e8e5e2, transparent);
                margin: 60px 0;
            }

            .highlight {
                color: #B7472A;
                font-weight: 600;
            }

            .badge {
                background: #87A96B;
                color: #ffffff;
                padding: 4px 12px;
                border-radius: 20px;
                font-size: 0.85rem;
                font-weight: 500;
                display: inline-block;
                margin-bottom: 20px;
            }
            
            
            "####;
            
            let css = self.convert_rem_to_tem(css);
            
            css
    
    }
    
    
    
    pub fn convert_tem_to_rem(&self, css: &str) -> String {
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
                }
                
                result.push_str(part);
            }
        }
        
        result
    }
    
    pub fn convert_rem_to_tem(&self, css: &str) -> String {
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
                }
                
                result.push_str(part);
            }
        }
        
        result
    }
}


       