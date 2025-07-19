use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_homepage_hero(&self) -> String {
        format!(
            r####"
<section class="hero">
    <div class="container">
        <h1>Strategic Revenue Engineering</h1>
        <p class="subtitle">We deliver the 10:1+ ROI performance that serious businesses demand. Whether you're managing billions in assets or scaling a high-growth enterprise, we architect revenue systems that perform under pressure.</p>
        <a href="#deployment" class="cta-button">Request Strategic Briefing</a>
    </div>
</section>
            "####
        )
    }
}