pub mod footers;
pub mod headers;
pub mod svgs;

use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

use crate::mcp::ModalContextProvider;

#[component]
pub fn NextWorkButton(
    #[prop(into)] title: Signal<String>,
    #[prop(into)] link: Signal<String>,
    #[prop(into, optional)] color1: MaybeProp<String>,
    #[prop(into, optional)] color2: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <A
            href=move || link.get()
            attr:class="group relative inline-flex w-full overflow-hidden py-12 px-10 md:py-24"
        >
            <span class=move || {
                format!(
                    "absolute inset-0 z-0 h-0 w-full transition-all duration-300 ease-initial group-hover:h-full {}",
                    color1.get().unwrap_or("bg-slate-900".to_string()),
                )
            }></span>
            <span class=move || {
                format!(
                    "absolute inset-0 z-0 h-0 w-full transition-all delay-200 duration-300 ease-initial group-hover:h-full {}",
                    color2.get().unwrap_or("bg-sky-600".to_string()),
                )
            }></span>

            <div class="z-10 mx-auto grid min-w-xl md:grid-cols-2 items-center">
                <div class="flex flex-col justify-center text-left gap-5">
                    <h3 class="text-lg md:text-xl tracking-tight text-amber-600 uppercase delay-150 duration-300 ease-in-out group-hover:text-stone-100">
                        "next work"
                    </h3>
                    <h1 class="text-2xl md:text-5xl font-black tracking-wider uppercase delay-150 duration-300 ease-in-out group-hover:text-stone-100">
                        {move || title.get()}
                    </h1>
                </div>
                <div class="hidden md:flex items-center justify-end">
                    <svg
                        width="150"
                        height="50"
                        class="transition-all delay-100 duration-300 ease-in-out group-hover:translate-x-4 group-hover:text-stone-100"
                        viewBox="0 0 150 50"
                        fill="none"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <rect y="23.2143" width="147" height="3.57143" fill="currentColor" />
                        <rect
                            width="39.0497"
                            height="3.80921"
                            transform="matrix(0.808138 0.588993 -0.41592 0.909401 115.462 0)"
                            fill="currentColor"
                        />
                        <rect
                            width="147"
                            height="3.57143"
                            transform="matrix(1 0 0 -1 0 26.7857)"
                            fill="currentColor"
                        />
                        <rect
                            width="39.0497"
                            height="3.80921"
                            transform="matrix(0.808138 -0.588993 -0.41592 -0.909401 115.462 50)"
                            fill="currentColor"
                        />
                        <path d="M150 25L146.831 22.8L146.832 27.2L150 25Z" fill="currentColor" />
                    </svg>
                </div>
            </div>
        </A>
    }
}

#[component]
pub fn MobileMenu(children: Children) -> impl IntoView {
    let context = ModalContextProvider::expect_context();
    let body_menu = RwSignal::new(false);
    let menu_1 = RwSignal::new(false);
    let menu_2 = RwSignal::new(false);
    let menu_3 = RwSignal::new(false);

    let on_click = move |_| {
        set_timeout(
            move || context.show_menu.update(|val| *val = false),
            std::time::Duration::from_millis(500),
        );
    };

    set_timeout(
        move || body_menu.set(true),
        std::time::Duration::from_millis(100),
    );

    set_timeout(
        move || menu_1.set(true),
        std::time::Duration::from_millis(100),
    );

    set_timeout(
        move || menu_2.set(true),
        std::time::Duration::from_millis(150),
    );

    set_timeout(
        move || menu_3.set(true),
        std::time::Duration::from_millis(200),
    );

    view! {
        <section class=move || {
            format!(
                "fixed z-[9999] overflow-hidden h-dvh w-0 {}",
                if body_menu.get() { "w-full" } else { "" },
            )
        }>
            <div class="relative h-full w-full pb-24">
                <span class=move || {
                    format!(
                        "absolute inset-0 h-full w-0 bg-amber-500 transition-all duration-200 ease-in-out {}",
                        if menu_1.get() { "w-full" } else { "" },
                    )
                }></span>
                <span class=move || {
                    format!(
                        "absolute inset-0 z-10 h-full w-0 bg-gradient-to-l from-blue-950  to-blue-600 transition-all delay-100 duration-200 ease-in-out {}",
                        if menu_2.get() { "w-full" } else { "" },
                    )
                }></span>

                <div class=move || {
                    format!(
                        "z-20 relative flex h-full w-0 flex-col px-10 py-2 transition-all delay-150 ease-in-out {}",
                        if menu_3.get() { "w-full" } else { "" },
                    )
                }>
                    <button on:click=on_click class="relative flex self-end cursor-pointer">
                        <div class="inset-0 absolute h-8 w-1 rotate-45 bg-stone-200"></div>
                        <div class="inset-0 absolute h-8 w-1 -rotate-45 bg-stone-200"></div>
                    </button>

                    <div class="flex-1 flex flex-col justify-center items-start gap-10">
                        {children()}
                    </div>
                    <div class="flex items-center gap-5">
                        <A href="#">
                            <Icon
                                icon=icondata::BiTwitter
                                attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                            />
                        </A>
                        <A href="#">
                            <Icon
                                icon=icondata::BiGithub
                                attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                            />
                        </A>

                        <A href="#">
                            <Icon
                                icon=icondata::BiLinkedin
                                attr:class="h-8 w-8 text-slate-400 hover:text-slate-100 duration-200 ease-in-out"
                            />
                        </A>
                    </div>
                </div>
            </div>
        </section>
    }
}
