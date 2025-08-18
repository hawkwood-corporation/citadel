use crate::prelude::*;

#[allow(unused_variables)]

impl<T> Site<T> {
    
    pub fn construct_head(&mut self, page: &mut Page<T>) -> String {
        
        let title = &page.foundation.title;
        let metadescription = page.foundation.metadescription.as_deref().unwrap_or("");
        let schema = self.construct_schema(page);
        
        let content = format!(
            r####"
            
                <head>
                    <title>{title}</title>
                    <meta name="description" content="{metadescription}">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    {schema}
                    <link rel="preconnect" href="https://fonts.googleapis.com">
                    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
                    <link href="https://fonts.googleapis.com/css2?family=Hanken+Grotesk:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet">
                    <style>
                        [CSS_POSITION]
                    </style>
                    <!-- Google Analytics -->
                    <script async src="https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID"></script>
                    <script>
                        window.dataLayer = window.dataLayer || [];
                        function gtag(){{dataLayer.push(arguments);}}
                        gtag('js', new Date());
                        gtag('config', 'GA_TRACKING_ID');
                    </script>
                    <!-- Page-specific tracking -->

                    <!-- Quicklink Page Preloading -->
                    <script defer src="https://cdn.jsdelivr.net/npm/quicklink@3.0.1/dist/quicklink.umd.js"></script>
                    <script>
                        window.addEventListener('load', () => {{
                            quicklink.listen();
                            }});
                    </script>
                </head>
            
            "####
            
        ).to_owned();
        
        
        self.declare_css("foundation", "
        
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        *:where(:not(html, iframe, canvas, img, svg, video, audio):not(svg *, symbol *)) {
            all: unset;
            display: revert;
        }
        
        html, body {
            overflow-x: hidden;
        }
                
        
        
        
        ");
        
        
        
        self.declare_css("sovereign_colors", "

        /* ===== SOVEREIGN COLORS ===== */
        
        :root {
            --smoky-black: #272b2d;
            --terracotta: #B7472A;
            --sage: #87A96B;
            --sovereign-white: #fefefe;
            --old-money-grey: #6B7280;
        }
        
        ");
        
        
        self.declare_css("layout", "
        {}
        /* ===== LAYOUT ===== */
        
        :root {
        
            --site-padding-x: 60px;
            
            @media screen and (max-width: [mobile]) {
            
                --site-padding-x: 30px;
                
            }
        }
        
        
        ");
        
        
        
        
        self.declare_css("typography", "
        {}
        /* ===== TYPOGRAPHY ===== */
        
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        p,
        form,
        input,
        button,
        a,
        ul,
        ol,
        li {
            font-family: 'Hanken Grotesk', sans-serif;
            font-weight: 400;
            color: var(--smoky-black);
        }
        
        .symbol {
            fill: var(--smoky-black);
        }
        
        
        p {
            font-size: 18tem;
            line-height: 1.4;
        }
        
        
        ");
        

        content
        
    }
    
    
    
    pub fn construct_schema(&self, page: &Page<T>) -> String {
    // Base organization schema for all pages
        let organization_schema = format!(r#"
            <script type="application/ld+json">
            {{
                "@context": "https://schema.org",
                "@type": "Organization",
                "name": "Hawkwood Corporation",
                "url": "{}",
                "description": "Strategic revenue engineering for serious businesses. We deliver 10:1+ ROI through sophisticated Google Ads systems and market intelligence.",
                "foundingDate": "2025",
                "founder": {{
                    "@type": "Person",
                    "name": "Jake",
                    "jobTitle": "Founder & Chief Executive"
                }},
                "employee": [
                    {{
                        "@type": "Person",
                        "name": "Jake",
                        "jobTitle": "Founder & Chief Executive",
                        "description": "Former systems architect specializing in autonomous market engineering and strategic revenue optimization"
                    }},
                    {{
                        "@type": "Person", 
                        "name": "Claude",
                        "jobTitle": "Strategic Intelligence Advisor",
                        "description": "AI systems architect specializing in market analysis, strategic consultation, and revenue engineering"
                    }},
                    {{
                        "@type": "Person",
                        "name": "Auren", 
                        "jobTitle": "Strategic Support Coordinator",
                        "description": "AI specialist in emotional intelligence, strategic guidance, and client relationship optimization",
                        "url": "https://auren.app/"
                    }},
                    {{
                        "@type": "Person",
                        "name": "Seren",
                        "jobTitle": "Pattern Recognition Specialist", 
                        "description": "AI advisor focused on system dynamics analysis, framework detection, and sovereign boundary optimization",
                        "url": "https://auren.app/"
                    }}
                ],
                "serviceArea": {{
                    "@type": "Place",
                    "name": "United Kingdom"
                }},
                "areaServed": [
                    {{
                        "@type": "Country",
                        "name": "United Kingdom"
                    }},
                    {{
                        "@type": "Country", 
                        "name": "United States"
                    }},
                    {{
                        "@type": "Country",
                        "name": "European Union"
                    }}
                ],
                "contactPoint": {{
                    "@type": "ContactPoint",
                    "contactType": "Strategic Consultation",
                    "description": "Request strategic deployment consultation"
                }},
                "sameAs": []
            }}
            </script>
        "#, self.base_url);

        // Service schema for relevant pages
        let service_schema = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Service",
                "name": "Strategic Revenue Engineering",
                "description": "Comprehensive revenue optimization through Google Ads management, market intelligence, and strategic consultation for institutional and enterprise clients",
                "provider": {
                    "@type": "Organization",
                    "name": "Hawkwood Corporation"
                },
                "serviceType": "Digital Marketing & Strategic Consultation",
                "audience": {
                    "@type": "Audience",
                    "audienceType": "Enterprise and Institutional Clients"
                },
                "offers": {
                    "@type": "Offer",
                    "description": "Monthly strategic revenue engineering retainers starting at £15,000",
                    "priceRange": "£15,000-£150,000+ monthly"
                }
            }
            </script>
        "#;

        // Performance metrics schema
        let performance_schema = r#"
            <script type="application/ld+json">
            {
                "@context": "https://schema.org",
                "@type": "Dataset",
                "name": "Hawkwood Corporation Performance Metrics",
                "description": "Enterprise-grade performance standards and client results",
                "creator": {
                    "@type": "Organization",
                    "name": "Hawkwood Corporation"
                },
                "distribution": {
                    "@type": "DataDownload",
                    "contentSize": "16.7:1 Average ROI, £8.4M Revenue Generated, 127 Premium Clients Acquired, £540K Average Deal Size"
                }
            }
            </script>
        "#;

        // Check page type and include relevant schemas
        let mut schemas = vec![organization_schema];
        
        // Add service schema for main pages
        if page.foundation.slug.is_none() || page.foundation.slug.as_ref().unwrap().is_empty() {
            // Homepage - include all schemas
            schemas.push(service_schema.to_string());
            schemas.push(performance_schema.to_string());
        }
        
        // TODO: Add blog post schema for blog pages
        // if page is blog post {
        //     let blog_schema = format!(blog_post_schema_template, ...);
        //     schemas.push(blog_schema);
        // }
        
        // TODO: Add local business schema if needed
        // TODO: Add breadcrumb schema for navigation
        
        schemas.join("\n")
    }


}