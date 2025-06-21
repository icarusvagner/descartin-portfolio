mod castlebyte_view;
mod cta_view;
mod goldenyears;
mod pixl_view;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::views::case_studies::{
    castlebyte_view::CastlebyteTechsolutionsView, cta_view::CebuToursAdventuresView,
    goldenyears::GoldenYearsView, pixl_view::Pixl8MultimediaView,
};

#[component]
pub fn CaseStudiesView() -> impl IntoView {
    let mod_param = use_params_map();

    let case = Memo::new(move |_| mod_param.read().get("case").unwrap_or_default());

    match case.get().as_ref() {
        "cebu-tours-adventures" => view! { <CebuToursAdventuresView /> }.into_any(),
        "pixl8multimedia" => view! { <Pixl8MultimediaView /> }.into_any(),
        "goldenyears" => view! { <GoldenYearsView /> }.into_any(),
        "castlebyte-techsolutions" => view! { <CastlebyteTechsolutionsView /> }.into_any(),
        _ => view! { <NotFoundView /> }.into_any(),
    }
}

#[component]
fn NotFoundView() -> impl IntoView {
    view! {
        <section class="min-h-screen flex flex-col gap-3 items-center justify-center bg-neutral-900">
            <h1 class="text-6xl font-extrabold tracking-wider text-neutral-100 lg:text-9xl">
                "404"
            </h1>
            <h3 class="text-xl font-medium tracking-tight text-neutral-100">
                "The page you are looking for is not found"
            </h3>
        </section>
    }
}
