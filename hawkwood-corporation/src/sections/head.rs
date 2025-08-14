use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    
    pub fn construct_head(&mut self, data: &mut PageData) -> String {
        
        let title = &data.title;
        let metadescription = data.metadescription.as_deref().unwrap_or("");
        
        let content = format!(
            r####"
            
            <head>
                <title>{title}</title>
                <meta name="description" content="{metadescription}">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                
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
        
        {}

        
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
        {}
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
    


}