use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};

use crate::components::{headers::FirstHeader, svgs::InitialSVGLogo};

#[component]
pub fn MainLayout() -> impl IntoView {
    let show_loader = RwSignal::new(false);

    Effect::new(move || {
        let _ = use_location().pathname.get();
        show_loader.set(true);
        set_timeout(
            move || show_loader.set(false),
            std::time::Duration::from_millis(1200),
        );
    });

    view! {
        <Show when=show_loader fallback=|| view! { <OutletComponent /> }>
            <LoadingScreen />
        </Show>
    }
}

#[component]
fn OutletComponent() -> impl IntoView {
    view! {
        <FirstHeader />
        <Outlet />
    }
}

#[component]
pub fn LoadingScreen() -> impl IntoView {
    let overlay = RwSignal::new(false);

    set_timeout(
        move || overlay.set(true),
        std::time::Duration::from_millis(1000),
    );

    view! {
        <section class="min-h-screen relative w-full bg-slate-100 dark:bg-slate-900 inline-flex items-center justify-center">
            <div class="z-0 w-48 h-80 flicker-in-1">
                <InitialSVGLogo class="h-full w-full" />
            </div>

            <div class=move || {
                format!(
                    "absolute inset-0 h-full bg-cyan-950 z-[999] {}",
                    if overlay.get() {
                        "w-full duration-300 transition-all ease-in-out"
                    } else {
                        "w-0"
                    },
                )
            }></div>

        </section>
    }
}
