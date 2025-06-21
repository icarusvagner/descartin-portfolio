mod callback;
mod routes;

pub use callback::*;

pub mod components;
pub mod layouts;
pub mod sections;
pub mod views;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Body, Html, Title};
use routes::AppRoutes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let for_show = RwSignal::new(false);
    provide_context(for_show);

    view! {
        <Title text="Lance Phillip Descartin - Interactive Frontend" />
        <Html {..} class="scroll-smooth" />
        <Body class:overflow-hidden=move || for_show.get() />

        <main class="min-h-screen">
            <AppRoutes />
        </main>
    }
}
