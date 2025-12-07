use leptos::*;
use leptos_router::*;

use crate::services::auth::use_auth;

#[component]
pub fn HomePage() -> impl IntoView {
    let auth = use_auth();
    let is_logged_in = move || auth.token.get().is_some();

    view! {
        <div class="home-page">
            // Hero Section with full-width background
            <section class="hero-banner">
                <div class="hero-overlay"></div>
                <div class="hero-content">
                    <p class="hero-tagline">"RESIDENTIAL & COMMERCIAL ROOFING"</p>
                    <h1 class="hero-headline">"We've Got You Covered!"</h1>
                    <p class="hero-subheadline">"Your Trusted Roofing Specialists"</p>
                    <div class="hero-cta">
                        <Show
                            when=is_logged_in
                            fallback=move || view! {
                                <A href="/register" class="btn-book">"BOOK APPOINTMENT"</A>
                                <A href="#services" class="btn-services">"OUR SERVICES"</A>
                            }
                        >
                            <A href="/dashboard" class="btn-book">"GO TO DASHBOARD"</A>
                        </Show>
                    </div>
                </div>
            </section>

            // Protection & Peace of Mind Section
            <section class="info-section">
                <div class="info-container">
                    <div class="info-card">
                        <h2 class="info-title">"Protection & Peace of Mind"</h2>
                        <p class="info-text">
                            "We understand that your roof protects not just your house, but your loved ones – now and for years to come. Hiring a qualified roofing contractor is one of the most critically important decisions you'll make for your home."
                        </p>
                        <A href="#contact" class="info-link">"We Can Help →"</A>
                    </div>
                    <div class="info-card">
                        <h2 class="info-title">"Residential & Commercial"</h2>
                        <p class="info-text">
                            "Residential and commercial roofs have their own set of unique challenges. Regardless of the type or size of your property, Ruben Lama Roofing understands your needs and is ready to meet those unique challenges!"
                        </p>
                        <A href="#services" class="info-link">"Learn More →"</A>
                    </div>
                </div>
            </section>

            // Free Estimate CTA
            <section class="estimate-banner">
                <div class="estimate-content">
                    <h2 class="estimate-title">"Schedule Us For A Free Estimate"</h2>
                    <p class="estimate-text">"Book an instant appointment now for your free residential estimate!"</p>
                    <A href="/register" class="btn-estimate">"BOOK NOW"</A>
                </div>
            </section>

            // Services Section
            <section id="services" class="services-section">
                <h2 class="section-heading">"Our Services"</h2>
                <div class="services-grid">
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🏠"</span>
                        </div>
                        <h3 class="service-name">"Roof Installation"</h3>
                        <p class="service-info">"Complete new roof installations for residential and commercial properties with premium materials."</p>
                    </div>
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🔧"</span>
                        </div>
                        <h3 class="service-name">"Roof Repair"</h3>
                        <p class="service-info">"Fast, reliable repairs for leaks, storm damage, missing shingles, and general wear."</p>
                    </div>
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🔍"</span>
                        </div>
                        <h3 class="service-name">"Roof Inspection"</h3>
                        <p class="service-info">"Comprehensive roof inspections with detailed reports and maintenance recommendations."</p>
                    </div>
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🏢"</span>
                        </div>
                        <h3 class="service-name">"Commercial Roofing"</h3>
                        <p class="service-info">"Specialized solutions for flat roofs, metal roofing, and large-scale commercial projects."</p>
                    </div>
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🚨"</span>
                        </div>
                        <h3 class="service-name">"Emergency Service"</h3>
                        <p class="service-info">"24/7 emergency roofing services when disaster strikes. We're here when you need us most."</p>
                    </div>
                    <div class="service-item">
                        <div class="service-icon-wrap">
                            <span class="service-icon">"🛡️"</span>
                        </div>
                        <h3 class="service-name">"Maintenance"</h3>
                        <p class="service-info">"Regular maintenance programs to extend your roof's lifespan and prevent costly repairs."</p>
                    </div>
                </div>
            </section>

            // We're Ready Section
            <section class="ready-section">
                <div class="ready-content">
                    <h2 class="ready-title">"We're Ready"</h2>
                    <p class="ready-text">
                        "Whether you have questions about your roof, need a small repair, or a complete roofing job, our knowledgeable and friendly staff is ready to help address any of your roofing concerns."
                    </p>
                    <A href="#contact" class="btn-contact">"GET IN TOUCH"</A>
                </div>
            </section>

            // Bottom Info Grid
            <section id="contact" class="bottom-grid">
                <div class="grid-item about-item">
                    <h3 class="grid-title">"WHO WE ARE"</h3>
                    <p class="grid-text">
                        "We are Ruben Lama Roofing, a dedicated roofing contractor committed to providing our customers the highest degree of craftsmanship, quality, and integrity at the most competitive rates."
                    </p>
                    <A href="#" class="grid-link">"More about us →"</A>
                </div>
                <div class="grid-item contact-item">
                    <h3 class="grid-title">"GET IN TOUCH"</h3>
                    <p class="grid-text">
                        "Our regular business hours are Monday-Friday, 8:00 am - 4:30 pm"
                    </p>
                    <div class="contact-details">
                        <p>"📍 Your Local Area"</p>
                        <p>"📞 (555) 123-4567"</p>
                        <p>"✉️ info@rubenlamaroofing.com"</p>
                    </div>
                </div>
                <div class="grid-item book-item">
                    <h3 class="grid-title">"BOOK AN APPOINTMENT"</h3>
                    <p class="grid-text">
                        "Book an appointment now for a free estimate. Pick a date and time that best suits you."
                    </p>
                    <A href="/register" class="btn-book-small">"Book Now!"</A>
                </div>
            </section>
        </div>
    }
}
