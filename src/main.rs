use citadel::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
enum HawkwoodPages {
    Homepage,
    Intelligence, 
    About,
    BlogPost {
        date: Option<String>,
        author: Option<String>,
    },
}

// Simple test constructors
fn construct_homepage(site: &mut Site<HawkwoodPages>, page: &mut Page<HawkwoodPages>) {
    page.foundation.slug = Some("".to_owned());
    
    let head = site.construct_head(page);  // ← This calls your fallback!
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body>
            <h1>Hawkwood Corporation</h1>
            <p>Strategic revenue engineering test page.</p>
        </body>
        </html>
    "##);
    
    page.foundation.content = Some(html);
}

fn construct_intelligence(site: &mut Site<HawkwoodPages>, page: &mut Page<HawkwoodPages>) {
    site.declare_css("intelligence", r##"
    
        .intelligence {
            background: #B7472A;
        }
        
    "##);
    page.foundation.slug = Some("intelligence".to_owned());
    page.foundation.content = Some("<h1>Market Intelligence Division</h1><p>Test intelligence page.</p>".to_owned());
}

fn construct_about(site: &mut Site<HawkwoodPages>, page: &mut Page<HawkwoodPages>) {
    site.declare_css("about", r##"
    
        .about {
            background: #B7472A;
        }
    
    "##);
    page.foundation.slug = Some("about".to_owned());
    page.foundation.content = Some("<h1>About Hawkwood</h1><p>Test about page.</p>".to_owned());
}

fn construct_blog_post(site: &mut Site<HawkwoodPages>, page: &mut Page<HawkwoodPages>) {
    site.declare_css("blog-post", r##"
    
        .blog-post {
            background: #B7472A;
        }
    
    "##);
    // Access the blog post data from the specification
    if let HawkwoodPages::BlogPost { date, author } = &page.specification {
        let date_str = date.as_deref().unwrap_or("Unknown date");
        let author_str = author.as_deref().unwrap_or("Unknown author");
        
        page.foundation.content = Some(format!(
            "<h1>{}</h1><p>By {} on {}</p><p>Blog post content coming soon...</p>", 
            page.foundation.title, author_str, date_str
        ));
    }
}

fn main() {
    let pages = vec![
        Page {
            foundation: PageFoundation { 
                title: "Hawkwood Corporation".to_owned(),
                ..Default::default() 
            },
            specification: HawkwoodPages::Homepage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Market Intelligence".to_owned(),
                ..Default::default()
            },
            specification: HawkwoodPages::Intelligence,
        },
        Page {
            foundation: PageFoundation { 
                title: "About Us".to_owned(),
                ..Default::default()
            },
            specification: HawkwoodPages::About,
        },
        Page {
            foundation: PageFoundation { 
                title: "My First Blog Post".to_owned(),
                ..Default::default()
            },
            specification: HawkwoodPages::BlogPost {
                date: Some("2025-01-15".to_owned()),
                author: Some("Jake".to_owned()),
            },
        },
        Page {
            foundation: PageFoundation { 
                title: "Another Blog Post".to_owned(),
                ..Default::default()
            },
            specification: HawkwoodPages::BlogPost {
                date: Some("2025-01-20".to_owned()),
                author: Some("Claude".to_owned()),
            },
        },
    ];
    
    Site::example()
        .add_constructor(HawkwoodPages::Homepage, construct_homepage)
        .add_constructor(HawkwoodPages::Intelligence, construct_intelligence)
        .add_constructor(HawkwoodPages::About, construct_about)
        .add_constructor(HawkwoodPages::BlogPost { date: None, author: None }, construct_blog_post)
        .add_pages(pages)
        .commence();
}
