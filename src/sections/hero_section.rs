use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;
use leptos_use::use_debounce_fn;

use crate::{components::svgs::InitialSVGLogo, mcp::ModalContextProvider};

#[component]
pub fn HeroSection() -> impl IntoView {
    let contact_context = ModalContextProvider::expect_context().contact_state;

    let on_click = use_debounce_fn(move || contact_context.update(|val| *val = true), 500.0);

    view! {
        <section
            id="top"
            class="relative flex min-h-screen sm:min-h-[580px] xl:min-h-[53rem] overflow-hidden items-center bg-slate-100 dark:bg-slate-900"
        >
            <div class="absolute top-1/2 sm:top-24 xl:top-1/2 -translate-y-1/2 -translate-x-1/2 left-1/2 z-0 w-48 h-80">
                <InitialSVGLogo class="h-full w-full dark:opacity-100 opacity-75" />
            </div>

            <div class="z-10 flex flex-col gap-10 px-10 sm:px-18 md:px-36 lg:px-44">
                <h1 class="main-hero-title text-left font-black tracking-wide text-stone-800 dark:text-slate-300 relative">
                    "Lance Phillip Descartin"
                    <span class="absolute right-0 top-0 w-full h-full bg-neutral-300 z-10 scale-out-right"></span>
                </h1>
                <h3 class="text-left font-lora font-extrabold tracking-wider text-slate-400 italic relative xl:text-xl">
                    "Full-stack Software Developer"
                    <span class="absolute left-0 top-0 w-full h-full bg-neutral-300 z-10 scale-out-left"></span>
                </h3>

                <button
                    on:click=move |_| {
                        on_click();
                    }
                    class="slide-in-elliptic-left-fwd cursor-pointer group relative inline-flex h-12 w-40 items-center justify-center radient-pumpkin-bg"
                >
                    <span class="absolute top-0 right-0 h-full w-0 radient-medium-blue-bg transition-all duration-500 ease-in-out group-hover:left-0 group-hover:right-auto group-hover:w-full"></span>

                    <span class="z-10 text-xl font-medium tracking-wide text-slate-100">
                        "About me"
                    </span>
                    <Icon
                        icon=icondata::CgArrowLongRight
                        attr:class="absolute top-2 -right-7 h-8 w-12 text-slate-400 dark:text-slate-100 transition-all duration-200 ease-initial group-hover:translate-x-1"
                    />
                </button>
            </div>

            <div class="absolute bottom-0 left-1/2 -translate-x-1/2 group">
                <A href="#case_studies" attr:class="relative flex flex-col items-center">
                    <h1 class="text-sm font-medium text-neutral-600 dark:text-slate-50">"Works"</h1>
                    <div class="mt-2 h-10 w-0.5 bg-slate-300 dark:bg-slate-500 group-hover:dark:bg-slate-200 group-hover:bg-slate-500 duration-300 ease-initial"></div>
                    <Icon
                        icon=icondata::BsChevronCompactDown
                        attr:class="h-6 w-6 text-slate-500 dark:text-slate-200 absolute group-hover:-bottom-2 -bottom-5 duration-300 ease-initial transition-all left-1/2 -translate-x-1/2"
                    />
                </A>
            </div>
            <IconLinks />
        </section>
    }
}

#[component]
fn IconLinks() -> impl IntoView {
    view! {
        <div class="hidden sm:absolute top-1/2 right-4 -translate-y-1/2 scale-in-right">
            <div class="flex flex-col items-center justify-center gap-5">
                <a href="#">
                    <Icon
                        icon=icondata::BiTwitter
                        attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                    />
                </a>
                <a href="#">
                    <Icon
                        icon=icondata::BiGithub
                        attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                    />
                </a>

                <a href="#">
                    <Icon
                        icon=icondata::BiLinkedin
                        attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                    />
                </a>
            </div>
        </div>
    }
}
