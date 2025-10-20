use citadel::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
enum HawkwoodExamplePages {
    Homepage,
    Intelligence, 
    About,
    BlogPost(PostData<ExampleBlogFrontmatter>),
}




// Simple test constructors
fn construct_homepage(site: &mut Site<HawkwoodExamplePages, ()>, page: &mut Page<HawkwoodExamplePages>) {
    page.foundation.slug = Some("".to_owned());
    
    let head = site.construct_head(page);
    
    let html = format!(r##"
        <!DOCTYPE html>
        <html lang="en-US">
        {head}
        <body class="homepage">
            <h1>Hawkwood Corporation</h1>
            <p>Strategic revenue engineering test page.</p>
        </body>
        </html>
    "##);
    
    site.declare_placement(PlacementPosition::BodyTop, r##"
    
    <!-- Google Analytics Body Top Position --> 
    <script>
           
    </script>
    
    "##);
    
    page.foundation.content = Some(html);
}

fn construct_intelligence(site: &mut Site<HawkwoodExamplePages, ()>, page: &mut Page<HawkwoodExamplePages>) {
    site.declare_css("intelligence", r##"
    
        .intelligence {
            background: #B7472A;
        }
        
    "##);
    page.foundation.slug = Some("intelligence".to_owned());
    page.foundation.content = Some("<h1>Market Intelligence Division</h1><p>Test intelligence page.</p>".to_owned());
}

fn construct_about(site: &mut Site<HawkwoodExamplePages, ()>, page: &mut Page<HawkwoodExamplePages>) {
    site.declare_css("about", r##"
    
        .about {
            background: #B7472A;
        }
    
    "##);
    page.foundation.slug = Some("about".to_owned());
    page.foundation.content = Some("<h1>About Hawkwood</h1><p>Test about page.</p>".to_owned());
}

fn construct_blog_post(site: &mut Site<HawkwoodExamplePages, ()>, page: &mut Page<HawkwoodExamplePages>) {
    site.declare_css("blog-post", r##"
    
        .blog-post {
            background: #B7472A;
        }
    
    "##);
   
        page.foundation.content = Some(format!(
            "<h1>{}</h1><p>Blog post content coming soon...</p>", 
            page.foundation.title
        ));
    
}


fn main() {
    let mut pages = vec![
        Page {
            foundation: PageFoundation { 
                title: "Hawkwood Corporation".to_owned(),
                ..default() 
            },
            specification: HawkwoodExamplePages::Homepage,
        },
        Page {
            foundation: PageFoundation { 
                title: "Market Intelligence".to_owned(),
                ..default()
            },
            specification: HawkwoodExamplePages::Intelligence,
        },
        Page {
            foundation: PageFoundation { 
                title: "About Us".to_owned(),
                ..default()
            },
            specification: HawkwoodExamplePages::About,
        }];
        
        for post in citadel_get_all_posts() {  
            println!("Loaded blog post: {} at blog/{}", post.frontmatter.title, post.slug);
            pages.push(Page {
                foundation: PageFoundation {
                    title: post.frontmatter.title.clone(),
                    slug: Some(post.slug.clone()),
                    metadescription: Some(post.frontmatter.description.clone()),
                    ..default()
                },
                specification: HawkwoodExamplePages::BlogPost(post),
            });
        };
    
    use HawkwoodExamplePages::*;
        
    Site::example(())
        .add_constructor(Homepage, construct_homepage)
        .add_constructor(Intelligence, construct_intelligence)
        .add_constructor(About, construct_about)
        .add_constructor(BlogPost(default()), construct_blog_post)
        .add_head_constructor()
        //.add_head_constructor_with(custom_code)
        .add_pages(pages)
        .commence();

}





// -- Example: Manual trait-based constructor routing --
// This shows how to bypass Citadel's HashMap registration system
// and implement your own constructor dispatch if needed.
// Most users should prefer the .add_constructor() approach above.

/*
trait PageConstructor<T, I> {
    fn construct_matcher(&self, site: &mut Site<T, I>, page: &mut Page<T>);
}

use HawkwoodExamplePages::*;

impl PageConstructor<HawkwoodExamplePages, ()> for HawkwoodExamplePages {
    fn construct_matcher(&self, site: &mut Site<HawkwoodExamplePages, ()>, page: &mut Page<HawkwoodExamplePages>) {
        match self {
            Homepage => construct_homepage(site, page),
            Intelligence => construct_intelligence(site, page),
            About => construct_about(site, page),
            BlogPost { .. } => construct_blog_post(site, page),
        }
    }
}

*/
// -- End example --