use citadel::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
enum TestPageSpec {
    Homepage,
    BlogPost {
        date: Option<String>,
        author: Option<String>,
    },
}

impl PageConstructor<TestPageSpec> for Site<TestPageSpec> {
    fn construct_page(&mut self, page: &mut Page<TestPageSpec>) {
        match &page.specification {
            TestPageSpec::Homepage => {
                self.construct_homepage(page);
            },
            TestPageSpec::BlogPost { .. } => {
                self.construct_blog_post(page);
            },
        }
    }
}

fn main() {
    let mut site: Site<TestPageSpec> = Site::example();
    
    let pages = vec![
        Page {
            foundation: PageFoundation { 
                title: "Homepage".to_owned(),
                ..Default::default() 
            },
            specification: TestPageSpec::Homepage,
        },
        Page {
            foundation: PageFoundation { 
                title: "My First Post".to_owned(),
                ..Default::default()
            },
            specification: TestPageSpec::BlogPost {
                date: Some("2025-01-15".to_owned()),
                author: Some("Jake".to_owned()),
            },
        },
    ];
    
    site.pages = pages;
    site.commence();
}