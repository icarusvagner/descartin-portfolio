mod callback;
mod mcp;
mod routes;
mod throttle;

pub use callback::*;

pub mod components;
pub mod layouts;
pub mod sections;
pub mod views;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Title};
use routes::AppRoutes;

use crate::{mcp::ContextProvider, views::case_studies::ShowModalContext};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Title text="Lance Phillip Descartin - Interactive Frontend" />

        <ContextProvider>
            <main class="min-h-screen dark:bg-slate-800 bg-slate-50 text-neutral-900 dark:text-neutral-50 overflow-x-hidden">
                <AppRoutes />
            </main>

            <ShowModalContext />
        </ContextProvider>
    }
}
