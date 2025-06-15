use leptos::prelude::*;

use crate::sections::{case_studies_section::CaseStudiesSection, hero_section::HeroSection};

#[component]
pub fn IndexView() -> impl IntoView {
    view! {
        <HeroSection />
        <CaseStudiesSection />
    }
}
