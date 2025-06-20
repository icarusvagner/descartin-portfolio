use chrono::{Datelike, Local};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::components::{headers::LinkTag, svgs::InitialSVGLogo};

#[component]
pub fn LeadFooter() -> impl IntoView {
    let year_now = Local::now().year();

    view! {
        <section class="flex flex-col items-center gap-8 bg-neutral-900 mt-10 py-32">
            <A href="#top" attr:class="group relative inline-flex">
                <InitialSVGLogo class="h-32 w-32 group-hover:scale-105 duration-200 ease-in-out transition-all" />
                <p class="absolute top-1/2 right-0 -translate-x-1/2 text-2xl font-black tracking-tight text-neutral-200 group-hover:-right-12 duration-300 transition-all ease-initial group-hover:scale-100 scale-0 font-oswald">
                    "Top"
                </p>
            </A>
            <div class="flex items-center gap-x-14">
                <LinkTag link="#" title="Twitter" />
                <LinkTag link="#" title="Github" />
                <LinkTag link="#" title="Linkedin" />
            </div>

            <div
                class="text-neutral-200 gap-0.5 flex items-center text-[0.75em] tracking-wide"
                style="unicode-bidi: isolate;"
            >
                <Icon icon=icondata::AiCopyrightOutlined attr:class="h-3 w-3" />
                {year_now}
                " Lance Phillip Descartin"
                <div class="mx-1 h-0.5 w-2 bg-neutral-200"></div>
                <A href="mailto:info@descartin.com">"Contact"</A>
            </div>
        </section>
    }
}
