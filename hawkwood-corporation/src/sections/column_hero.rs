use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    pub fn construct_homepage_hero(&mut self) -> String {
        
        self.declare_css("column_hero", r##"
        {}
        
        section.column-hero {
    display: grid;
    grid-template-columns: 1fr minmax(auto, 1440px) 1fr;
    align-items: center;
    height: 100vh;
    padding: 0 40px;
    background-image: url("/low-angle-shot-church-with-staple-bright-sky.jpg");
    background-size: cover;
    background-position: center;
    
    h1 {
        font-size: 80tem;
        font-weight: 600;
    }
    
    .area {
        grid-column: 2;
        max-width: 720px;
        justify-self: start; /* Keeps it left-aligned within the center column */
        display: grid;
        row-gap: 20px;
    }
}
                
        
        
        
        
        
        
        
        
        
        
        
        
        
        "##);
        
        
        let directive = Directive {
            text: "View All Projects".to_string(),
            href: "/projects".to_string(),
            symbol: r##"<svg class="symbol upper-right-arrow" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><path d="M200,64V168a8,8,0,0,1-16,0V83.31L69.66,197.66a8,8,0,0,1-11.32-11.32L172.69,72H88a8,8,0,0,1,0-16H192A8,8,0,0,1,200,64Z"/></path></svg>"##.to_string(),
            class: Some("arrow-right".to_string()),
        };
        
        let directive = self.construct_directive_button(&directive);
        
        format!(
            r####"
            
<section class="column-hero">
    <div class="area">
        <h1>Strategic Revenue Engineering</h1>
        <p class="subtitle">We deliver the 10:1+ ROI performance that serious businesses demand. Whether you're managing billions in assets or scaling a high-growth enterprise, we architect revenue systems that perform under pressure.</p>
        {directive}
    </div>
</section>

            "####
        )
    }
}