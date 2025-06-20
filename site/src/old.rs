#[derive(Debug)]
struct SiteData {
    title: String,
    nav_items: Vec<NavItem>,
    hero: Hero,
    treatments: Vec<Treatment>,
    testimonials: Vec<Testimonial>,
}

#[derive(Debug)]
struct NavItem {
    name: String,
    url: String,
}

#[derive(Debug)]
struct Hero {
    subtitle: String,
    title: String,
    title_span: String,
    description: String,
    cutout_images: Vec<String>,
    background_image: String,
}

#[derive(Debug)]
struct Treatment {
    title: String,
    description: String,
    image: String,
    url: String,
}

#[derive(Debug)]
struct Testimonial {
    name: String,
    text: String,
    date: String,
    rating: u8,
}


fn create_site_data() -> SiteData {
    
    SiteData {
        title: "Dentozen Dental & Skin Spa".to_string(),
        nav_items: vec![
            NavItem { name: "Treatments".to_string(), url: "/treatments/".to_string() },
            NavItem { name: "New Patients".to_string(), url: "/new-patients/".to_string() },
            NavItem { name: "Results".to_string(), url: "/results/".to_string() },
            NavItem { name: "Testimonials".to_string(), url: "/testimonials/".to_string() },
            NavItem { name: "Pricing".to_string(), url: "/pricing/".to_string() },
            NavItem { name: "Our Team".to_string(), url: "/our-team/".to_string() },
            NavItem { name: "Contact".to_string(), url: "/contact/".to_string() },
        ],
        treatments: vec![
            Treatment {
                title: "Clear Aligners".to_string(),
                description: "Professional teeth straightening using invisible aligners. Effective treatment for crowded or spaced teeth.".to_string(),
                image: "clear-aligners.webp".to_string(),
                url: "/clear-aligners/".to_string(),
            },
            Treatment {
                title: "Composite Bonding".to_string(),
                description: "Fast, effective smile enhancement using tooth-colored materials that match your natural teeth.".to_string(),
                image: "composite-bonding.webp".to_string(),
                url: "/composite-bonding/".to_string(),
            },
            // Add more treatments...
        ],
        testimonials: vec![
            Testimonial {
                name: "Jessica Joslin-Smiley".to_string(),
                text: "I came here for Invisalign, whitening and composite bonding and I couldn't be happier with the whole experience!!".to_string(),
                date: "1 year ago".to_string(),
                rating: 5,
            },
            // Add more testimonials...
        ],
    }
    
}

fn generate_page2(data: &SiteData) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en-GB">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{}</title>
    <link rel="stylesheet" href="/css/style.css">
    <link rel="stylesheet" href="/css/custom.css">
</head>
<body class="home">
    {}
    {}
    {}
    {}
    {}
    {}
    {}
</body>
</html>"#, 
        data.title,
        render_header(&data.nav_items),
        render_hero(&data.hero),
        render_our_practice(),
        render_reviews(&data.testimonials),
        render_treatments(&data.treatments),
        render_how_to_find_us(),
        render_footer()
    )
}

fn generate_page(data: &SiteData) -> String {
    
    let hero = Hero {
        subtitle: "A Retreat in Enfield, London".to_string(),
        title: "Welcome to Dentozen".to_string(),
        title_span: "Dental & Skin Spa.".to_string(),
        description: "Our modern clinic combines advanced dentistry with exceptional comfort. We provide comprehensive care in a carefully designed, relaxing environment.".to_string(),
        cutout_images: vec![
            "homepage-hero-image-old.jpg".to_string(),
            "IMG_4069.jpg".to_string(), 
            "IMG_3913.jpg".to_string()
        ],
        background_image: "homepage-hero-image.webp".to_string(),
    };
    
    let title = &data.title;
    let header = render_header(&data.nav_items);
    let hero = render_hero(&hero);
    let practice = render_our_practice();
    let reviews = render_reviews(&data.testimonials);
    let treatments = render_treatments(&data.treatments);
    let find_us = render_how_to_find_us();
    let footer = render_footer();
    
    
    
    format!(r#"<!DOCTYPE html>
<html lang="en-GB">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title}</title>
    <link rel="stylesheet" href="/css/style.css">
    <link rel="stylesheet" href="/css/custom.css">
</head>
<body class="home">
    {header}
    {hero}
    {practice}
    {reviews}
    {treatments}
    {find_us}
    {footer}
</body>
</html>"#).trim().to_string()
}


fn render_header(nav_items: &[NavItem]) -> String {
    let nav_html = nav_items.iter()
        .map(|item| format!(r#"<li class="menu-item"><a href="{}">{}</a></li>"#, item.url, item.name))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"<header class="site-header">
    <div class="site-header-content">
        <a href="/" class="site-logo">
            <img width="50" height="50" src="/site-logo.svg" alt="Site Logo">
        </a>
        <nav class="nav-primary">
            <ul class="menu main-menu">
                {}
            </ul>
        </nav>
        <div class="button-group header-cta-buttons">
            <a href="https://dentozen.setmore.com" class="cta-button book-appointment-cta-button">Book Appointment</a>
        </div>
    </div>
</header>"#, nav_html)
}

fn render_hero(hero: &Hero) -> String {
    let images_html = hero.cutout_images.iter()
        .map(|img| format!(r#"<img class="hero-cutout-image" src="{}" />"#, img))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"<div class="hero-section hero-left-aligned tall-hero" style="background-image: url('{}'); background-size: cover; background-position: center;">
    <div class="hero-inner">
        <div class="hero-content">
            <div class="hero-text">
                <hgroup class="title-wrap">
                    <p>{}</p>
                    <h1>{}<span>{}</span></h1>
                </hgroup>
            </div>
            <div class="hero-images">
                {}
            </div>
            <div class="hero-extra-text">
                <p>{}</p>
            </div>
            <div class="button-group hero-cta-buttons">
                <a href="https://dentozen.setmore.com" class="cta-button book-appointment-cta-button">Book Appointment</a>
                <a href="tel:02036952400" class="cta-button">Call Our Reception</a>
            </div>
        </div>
    </div>
</div>"#, hero.background_image, hero.subtitle, hero.title, hero.title_span, images_html, hero.description)
}

fn render_our_practice() -> String {
    r#"<div class="our-practice">
    <div class="our-practice-inner">
        <hgroup>
            <p>A Closer Look at Dentozen</p>
            <h2>Modern Dentistry & Aesthetic Treatments</h2>
        </hgroup>
        <p>Our modern Enfield clinic combines advanced dental care with professional aesthetic treatments.</p>
    </div>
</div>"#.to_string()
}

fn render_reviews(testimonials: &[Testimonial]) -> String {
    let testimonials_html = testimonials.iter()
        .map(|t| format!(r#"<div class="google-testimonial">
            <p class="google-testimonial-name">{}</p>
            <div class="google-testimonial-text"><p>{}</p></div>
        </div>"#, t.name, t.text))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"<div class="reviews">
    <div class="reviews-inner">
        <hgroup>
            <p>From Our Patients</p>
            <h2>Reviews</h2>
        </hgroup>
        {}
    </div>
</div>"#, testimonials_html)
}

fn render_treatments(treatments: &[Treatment]) -> String {
    let treatments_html = treatments.iter()
        .map(|t| format!(r#"<div class="card treatment-card">
            <div class="card-image">
                <a href="{}"><img src="{}" /></a>
            </div>
            <div class="card-text">
                <h3 class="card-title"><a href="{}">{}</a></h3>
                <p>{}</p>
                <a class="info-button" href="{}">Learn More</a>
            </div>
        </div>"#, t.url, t.image, t.url, t.title, t.description, t.url))
        .collect::<Vec<_>>()
        .join("\n");

    format!(r#"<div class="treatments-section">
    <div class="treatments-section-inner">
        <h2>Our Services</h2>
        <div class="treatments-section-grid">
            {}
        </div>
    </div>
</div>"#, treatments_html)
}

fn render_how_to_find_us() -> String {
    r#"<div class="how-to-find-us-section">
    <div class="how-to-find-us-section-inner">
        <h2>How to Find Us</h2>
        <p>Located minutes from Bush Hill Park overground station in Enfield.</p>
    </div>
</div>"#.to_string()
}

fn render_footer() -> String {
    r#"<footer class="site-footer">
    <div class="site-footer-content">
        <p>Copyright © 2025 · Dentozen Dental & Skin Spa</p>
    </div>
</footer>"#.to_string()
}