use chrono::{Datelike, Local};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::components::{headers::LinkTag, svgs::InitialSVGLogo};

#[component]
pub fn LeadFooter() -> impl IntoView {
    let year_now = Local::now().year();
    let on_click = move |_| {
        document().body().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
        document()
            .document_element()
            .unwrap()
            .scroll_to_with_x_and_y(0.0, 0.0);
    };

    view! {
        <section class="flex flex-col items-center gap-8 bg-neutral-100 dark:bg-neutral-900 py-32">
            <button on:click=on_click class="cursor-pointer group relative inline-flex">
                <InitialSVGLogo class="h-32 w-32 group-hover:scale-105 duration-200 ease-in-out transition-all" />
                <p class="absolute top-1/2 right-0 -translate-x-1/2 text-2xl font-black tracking-tight text-neutral-400 group-hover:-right-12 duration-300 transition-all ease-initial dark:text-neutral-200 group-hover:scale-100 scale-0 font-oswald">
                    "Top"
                </p>
            </button>
            <div class="flex items-center gap-x-14">
                <LinkTag link="#" title="Twitter" />
                <LinkTag link="https://github.com/icarusvagner" title="Github" />
                <LinkTag link="#" title="Linkedin" />
            </div>

            <div
                class="text-neutral-800 dark:text-neutral-200 gap-0.5 flex items-center text-[0.75em] tracking-wide"
                style="unicode-bidi: isolate;"
            >
                <Icon icon=icondata::AiCopyrightOutlined attr:class="h-3 w-3" />
                {year_now}
                " Lance Phillip Descartin"
                <div class="mx-1 h-0.5 w-2 bg-neutral-800 dark:bg-neutral-200"></div>
                <A href="mailto:info@descartin.com">"Contact"</A>
            </div>
        </section>
    }
}
