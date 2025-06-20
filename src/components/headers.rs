use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::svgs::InitialSVGLogoTransparent;

#[component]
pub fn FirstHeader() -> impl IntoView {
    view! {
        <nav class="flex items-center px-3 sm:px-8 md:px-12 lg:px-24 absolute top-0 w-full py-2.5 z-[999]">
            <A href="/" attr:class="group">
                <InitialSVGLogoTransparent class="h-12 w-24 text-neutral-500 group-hover:text-neutral-100 duration-400 ease-initial" />
            </A>

            <div class="mx-auto"></div>

            <div class="flex items-center gap-4">
                <LinkTag title="case studies" link="#" />
                <LinkTag title="experience" link="#" />
                <LinkTag title="contact" link="#" />
            </div>
        </nav>
    }
}

#[component]
pub fn LinkTag(
    #[prop(into)] link: Signal<String>,
    #[prop(into)] title: Signal<String>,
) -> impl IntoView {
    view! {
        <A href=move || link.get() attr:class="relative overflow-hidden inline-flex group py-0.5">
            <span class="absolute bottom-0 left-0 w-0 h-1 group-hover:w-full duration-400 transition-all ease-initial bg-gradient-to-r from-blue-500 via-purple-500 to-pink-500"></span>
            <span class="z-10 text-lg capitalize font-black text-stone-500 group-hover:text-stone-50 duration-200 ease-initial">
                {move || title.get()}
            </span>
        </A>
    }
}
