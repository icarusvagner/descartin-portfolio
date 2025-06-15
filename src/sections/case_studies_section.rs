use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn CaseStudiesSection() -> impl IntoView {
    view! {
        <section id="case_studies" class="py-5 pt-12 min-h-screen relative overflow-hidden">
            <h1 class="text-center text-2xl text-neutral-500 font-black uppercase">
                "case studies"
            </h1>
            <h3 class="text-4xl capitalize font-black tracking-wide text-neutral-800 text-center mt-4">
                "latest works"
            </h3>

            <div class="mt-10 flex w-full flex-col items-center gap-24">
                <CaseComponent
                    title="Cebu Tours & Adventures"
                    bg_img="/public/assets/cebu-tours-adventures.png"
                    count="01"
                    link="/case-studies/cebu-tours-adventures"
                    direction=Direction::ToRight
                />
                <CaseComponent
                    title="Pixl8Multimedia"
                    bg_img="/public/assets/pixl8media.jpeg"
                    count="02"
                    link="/case-studies/pixl8multimedia"
                    direction=Direction::ToLeft
                />
                <CaseComponent
                    title="GoldenYears Carehome"
                    bg_img="/public/assets/goldenyears.png"
                    count="03"
                    link="/case-studies/goldenyears"
                    direction=Direction::ToRight
                />
            </div>
        </section>
    }
}

#[derive(Clone, Default)]
enum Direction {
    ToRight,
    ToLeft,
    #[default]
    Center,
}

impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::ToRight => "rtl".to_string(),
            Direction::ToLeft => "ltr".to_string(),
            Direction::Center => "center".to_string(),
        }
    }
}

#[component]
fn CaseComponent(
    #[prop(into)] bg_img: Signal<String>,
    #[prop(into)] title: Signal<String>,
    #[prop(into)] count: Signal<String>,
    #[prop(into)] link: Signal<String>,
    #[prop(into)] direction: Signal<Direction>,
) -> impl IntoView {
    let over_lay = RwSignal::new(false);
    let show_btn = RwSignal::new(false);
    let number_show = RwSignal::new(false);

    view! {
        <a
            href=move || link.get()
            on:mouseover=move |_| {
                set_timeout(move || over_lay.set(true), std::time::Duration::from_millis(500));
                show_btn.set(true);
                set_timeout(move || number_show.set(true), std::time::Duration::from_millis(800));
            }
            on:mouseleave=move |_| {
                set_timeout(move || over_lay.set(false), std::time::Duration::from_millis(500));
                show_btn.set(false);
                set_timeout(move || number_show.set(false), std::time::Duration::from_millis(800));
            }
            class=move || {
                format!(
                    "relative flex p-24 h-[460px] w-full max-w-4xl items-center bg-cover bg-top shadow-[5px_5px_15px_5px_rgba(18,0,0,0.6)] {}",
                    if direction.get().to_string().eq("rtl") {
                        "ml-12"
                    } else if direction.get().to_string().eq("ltr") {
                        "mr-12"
                    } else {
                        "mx-auto"
                    },
                )
            }
            style=move || {
                format!(
                    "background-image: linear-gradient(to right, rgba(0,0,0,0.5), rgba(0,0,0,0.5)), url('{}')",
                    bg_img.get(),
                )
            }
        >
            <div class=move || {
                format!(
                    "flex flex-col w-full gap-2 z-10 text-neutral-300 tracking-wide {}",
                    if show_btn.get() {
                        if direction.get().to_string().eq("rtl") {
                            "scale-105 duration-300 delay-150 transition-all ease-initial"
                        } else {
                            "items-end scale-105 duration-300 delay-150 transition-all ease-initial"
                        }
                    } else {
                        if direction.get().to_string().eq("rtl") {
                            "scale-100 duration-300 delay-150 transition-all ease-initial"
                        } else {
                            "items-end scale-100 duration-300 delay-150 transition-all ease-initial"
                        }
                    },
                )
            }>
                <h1 class="text-3xl font-black">{move || title.get()}</h1>

                <button class="group cursor-pointer relative inline-flex h-12 w-40 items-center justify-center bg-pumpkin">
                    <span class="absolute top-0 right-0 h-full w-0 bg-medium-blue transition-all duration-500 ease-in-out group-hover:left-0 group-hover:right-auto group-hover:w-full"></span>

                    <span class="z-10 text-xl font-medium tracking-wide text-slate-100">
                        "Case Study"
                    </span>
                    <Icon
                        icon=if direction.get().to_string().eq("rtl") {
                            icondata::CgArrowLongRight
                        } else {
                            icondata::CgArrowLongLeft
                        }
                        attr:class=move || {
                            format!(
                                "absolute top-2 h-8 w-12 text-slate-100 transition-all duration-200 ease-initial {}",
                                if direction.get().to_string().eq("rtl") {
                                    "-right-7 group-hover:translate-x-1"
                                } else if direction.get().to_string().eq("ltr") {
                                    "-left-7 group-hover:-translate-x-1"
                                } else {
                                    "mx-auto"
                                },
                            )
                        }
                    />
                </button>
            </div>

            <div class=move || {
                format!(
                    "absolute inset-0 h-full bg-cyan-950/70 {}",
                    if over_lay.get() {
                        "w-full duration-300 delay-150 transition-all ease-in-out"
                    } else {
                        "w-0 duration-300 delay-150 transition-all ease-in-out"
                    },
                )
            }></div>

            <div class=move || {
                format!(
                    "absolute -top-8 -right-3 inline-flex {}",
                    if number_show.get() {
                        if direction.get().to_string().eq("rtl") {
                            "-right-3 scale-in-tr"
                        } else if direction.get().to_string().eq("ltr") {
                            "-left-3 scale-in-tl"
                        } else {
                            "mx-auto scale-in-tr"
                        }
                    } else {
                        if direction.get().to_string().eq("rtl") {
                            "-right-3 scale-out-tr"
                        } else if direction.get().to_string().eq("ltr") {
                            "-left-3 scale-out-tl"
                        } else {
                            "mx-auto scale-out-tr"
                        }
                    },
                )
            }>
                <h1 class="font-oswald text-[11.5625em] leading-none font-black text-neutral-400 p-0 m-0 -translate-y-2 relative">
                    {move || count.get()}
                </h1>
            </div>
        </a>
    }
}
