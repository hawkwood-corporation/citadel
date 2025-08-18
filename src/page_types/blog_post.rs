use crate::prelude::*;

#[allow(dead_code)]
#[allow(unused_variables)]

impl<T> Site<T> {
    
    pub fn construct_blog_post(&self,  page: &mut Page<T>) {
        page.foundation.content = Some("Hehe".to_owned());
    }
}
