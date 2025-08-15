use crate::prelude::*;

#[allow(dead_code)]
#[allow(unused_variables)]

impl Site {
    
    pub fn construct_blog_post(&self,  page: &mut PageFoundation) {
        page.content = Some("Hehe".to_owned());
    }
}
