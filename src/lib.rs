mod routes;

pub mod components;
pub mod layouts;
pub mod sections;
pub mod views;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Title};
use routes::AppRoutes;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Lance Phillip Descartin - Interactive Frontend" />

        <main class="min-h-screen">
            <AppRoutes />
        </main>
    }
}
