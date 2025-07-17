use crate::prelude::*;

#[allow(unused_variables)]

impl Site {
    
    pub fn construct_homepage_css(&self) {
        
        let css = "/* Hero Section */
        .hero {
            padding: 120px 0 100px;
            text-align: center;
            background: linear-gradient(135deg, #fefefe 0%, #f9f8f6 100%);
        }

        .hero h1 {
            font-size: 3.8rem;
            font-weight: 700;
            margin-bottom: 25px;
            color: #B7472A;
            letter-spacing: -1px;
            line-height: 1.2;
        }

        .hero .subtitle {
            font-size: 1.4rem;
            color: #6B7280;
            margin-bottom: 50px;
            font-weight: 400;
            max-width: 700px;
            margin-left: auto;
            margin-right: auto;
        }

        .cta-button {
            background: #B7472A;
            color: #ffffff;
            padding: 20px 45px;
            font-size: 18px;
            font-weight: 600;
            text-decoration: none;
            border-radius: 6px;
            transition: all 0.3s;
            display: inline-block;
            box-shadow: 0 4px 12px rgba(183, 71, 42, 0.3);
        }

        .cta-button:hover {
            background: #9A3B23;
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(183, 71, 42, 0.4);
        }".to_owned();
        
    }
    
    pub fn construct_homepage(&mut self, page: &mut PageData) {
        
        
        
        page.slug = Some("".to_owned());
        
        
        
        
        let title = &page.title;
        page.metadescription = Some("Strategic revenue engineering for serious businesses. We deliver 10:1+ ROI through sophisticated Google Ads systems that perform under pressure.".to_owned());
        
        let head = self.construct_head(page);
        let header = self.construct_header(page);
        
        
        let mut content = format!(
            r####";
<!DOCTYPE html>
    <html lang="en-US">
        {head}
        
        {header}

        <section class="hero">
            <div class="container">
                <h1>Strategic Revenue Engineering</h1>
                <p class="subtitle">We deliver the 10:1+ ROI performance that serious businesses demand. Whether you're managing billions in assets or scaling a high-growth enterprise, we architect revenue systems that perform under pressure.</p>
                <a href="#deployment" class="cta-button">Request Strategic Briefing</a>
            </div>
        </section>

        <section class="section" id="operations">
            <div class="container">
                <div class="badge">Precision Operations</div>
                <h2>Revenue Engineering</h2>
                <p class="section-intro">We don't just run campaigns—we architect revenue systems. Every strategy is designed for sophisticated businesses with serious growth objectives and performance requirements.</p>
                
                <div class="services-grid">
                    <div class="service-card">
                        <h3>Strategic Search Operations</h3>
                        <p>Google Ads campaigns engineered for competitive markets and premium customer acquisition. We navigate complex landscapes to deliver measurable client growth at scale.</p>
                    </div>
                    <div class="service-card">
                        <h3>Market Intelligence Systems</h3>
                        <p>Comprehensive competitive analysis, target market profiling, and performance attribution that serious decision-makers depend on for strategic planning and growth optimization.</p>
                    </div>
                    <div class="service-card">
                        <h3>Revenue Attribution & Analytics</h3>
                        <p>Multi-touch attribution across complex sales cycles. Complete ROI transparency with quarterly strategic reviews and competitive positioning analysis.</p>
                    </div>
                </div>
            </div>
        </section>

        <section class="section section-sage" id="intelligence">
            <div class="container">
                <div class="badge">Combat Record</div>
                <h2>Enterprise Performance Standards</h2>
                <p class="section-intro">Our track record speaks to the sophisticated expectations of serious businesses. These are the metrics that matter when performance is everything.</p>
                
                <div class="metrics">
                    <div class="metric">
                        <span class="metric-number">16.7:1</span>
                        <span class="metric-label">Average ROI</span>
                        <span class="metric-description">Well above institutional 10:1 minimum</span>
                    </div>
                    <div class="metric">
                        <span class="metric-number">£8.4M</span>
                        <span class="metric-label">Revenue Generated</span>
                        <span class="metric-description">Attributable to our campaigns</span>
                    </div>
                    <div class="metric">
                        <span class="metric-number">127</span>
                        <span class="metric-label">Premium Clients</span>
                        <span class="metric-description">Successfully acquired</span>
                    </div>
                    <div class="metric">
                        <span class="metric-number">£540K</span>
                        <span class="metric-label">Average Deal Size</span>
                        <span class="metric-description">Enterprise client value</span>
                    </div>
                </div>
            </div>
        </section>

        <section class="section" id="leadership">
            <div class="container">
                <div class="badge">Strategic Leadership</div>
                <h2>Principal & Founder</h2>
                <div class="about-content">
                    <div>
                        <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='300' height='400' viewBox='0 0 300 400'%3E%3Crect width='300' height='400' fill='%23f0f0f0'/%3E%3Ctext x='150' y='200' text-anchor='middle' dy='.3em' font-family='Georgia, serif' font-size='16' fill='%23666'%3EYour Professional Photo%3C/text%3E%3C/svg%3E" alt="Jake - Principal, Hawkwood Corporation" class="profile-image">
                    </div>
                    <div class="about-text">
                        <h3>Jake, Principal & Strategic Director</h3>
                        <p>Former systems architect with deep experience in autonomous market engineering and distributed systems. I founded Hawkwood Corporation on the principle that serious businesses deserve marketing operations as sophisticated as their core strategies.</p>
                        
                        <p>My approach combines <span class="highlight">data-driven systems thinking</span> with the performance standards and strategic oversight that growth-focused organizations require. Every campaign is architected for long-term relationship building and exceptional ROI that defines successful partnerships.</p>
                        
                        <p>I personally oversee every client engagement, ensuring the strategic oversight and attention to detail that premium partnerships demand.</p>
                    </div>
                </div>
            </div>
        </section>

        <section class="section contact" id="deployment">
            <div class="container">
                <h2>Request Strategic Deployment</h2>
                <p class="section-intro" style="color: rgba(255,255,255,0.9);">Ready to engineer the revenue systems your business deserves? Let's discuss your strategic objectives and performance requirements.</p>
                
                <form class="contact-form">
                    <div class="form-row">
                        <div class="form-group">
                            <label for="firm-name">Company / Organization</label>
                            <input type="text" id="firm-name" name="firm-name" required>
                        </div>
                        <div class="form-group">
                            <label for="contact-name">Your Name</label>
                            <input type="text" id="contact-name" name="contact-name" required>
                        </div>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="email">Contact Email</label>
                            <input type="email" id="email" name="email" required>
                        </div>
                        <div class="form-group">
                            <label for="institution-type">Business Type</label>
                            <select id="institution-type" name="institution-type" required>
                                <option value="">Select Business Type</option>
                                <option value="financial-services">Financial Services</option>
                                <option value="technology">Technology / SaaS</option>
                                <option value="professional-services">Professional Services</option>
                                <option value="healthcare">Healthcare / Medical</option>
                                <option value="manufacturing">Manufacturing / Industrial</option>
                                <option value="real-estate">Real Estate / Property</option>
                                <option value="retail-ecommerce">Retail / E-commerce</option>
                                <option value="other-enterprise">Other Enterprise</option>
                            </select>
                        </div>
                    </div>
                    
                    <div class="form-row">
                        <div class="form-group">
                            <label for="aum">Annual Revenue</label>
                            <select id="aum" name="aum" required>
                                <option value="">Select Revenue Range</option>
                                <option value="10m-50m">£10M - £50M</option>
                                <option value="50m-250m">£50M - £250M</option>
                                <option value="250m-1b">£250M - £1B</option>
                                <option value="1b-plus">£1B+</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="budget">Monthly Marketing Investment</label>
                            <select id="budget" name="budget" required>
                                <option value="">Select Investment Range</option>
                                <option value="15k-30k">£15,000 - £30,000</option>
                                <option value="30k-50k">£30,000 - £50,000</option>
                                <option value="50k-100k">£50,000 - £100,000</option>
                                <option value="100k-plus">£100,000+</option>
                            </select>
                        </div>
                    </div>
                    
                    <div class="form-group full-width">
                        <label for="strategic-objectives">Strategic Objectives & Performance Requirements</label>
                        <textarea id="strategic-objectives" name="strategic-objectives" rows="5" placeholder="Describe your current growth challenges, target market, and specific ROI expectations..." required></textarea>
                    </div>
                    
                    <button type="submit" class="cta-button" style="margin-top: 20px;">Submit Strategic Brief</button>
                </form>
            </div>
        </section>

        <footer>
            <div class="container">
                <p>&copy; 2025 Hawkwood Corporation. Strategic revenue engineering for serious businesses.</p>
            </div>
        </footer>
    </body>
</html>
            "####).to_string();
        
        //wrap_html_body(&mut content);
    
        page.content = Some(content);  
        
    }
}
