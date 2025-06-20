use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn CaseStudiesView() -> impl IntoView {
    let for_show = RwSignal::new(true);

    //let for_view = use_context::<WriteSignal<bool>>().expect("To have found setter provider context");

    view! {
        <section class="z-[999] absolute min-h-screen inset-0 bg-neutral-900/70 pb-12">
            <button class="z-[999] -tranxlate-x-1/2 radient-pumpkin-bg fixed top-0 left-1/2 flex cursor-pointer items-center gap-1 px-7 py-5 text-xl font-extrabold tracking-wide text-neutral-50 capitalize">
                <Icon icon=icondata::CgClose attr:class="h-6 w-6" />
                "close"
            </button>

            <div class="mx-auto mt-14 flex h-full max-w-6xl shadow-2xl">
                <div
                    class="relative col-span-2 inline-flex h-full min-w-lg bg-cover bg-center bg-no-repeat shadow-[0px_0px_12px_4px_rgba(23,0,0,0.44)]"
                    style="background-image: linear-gradient(to right, rgba(0,0,0,0.5), rgba(0,0,0,0.5)), url('https://picsum.photos/1200/1200');"
                >
                    <div class="absolute right-0 bottom-0 p-3">
                        <h1 class="text-[12.5em] font-black text-neutral-100">01</h1>
                    </div>
                </div>
                <div
                    class="relative col-span-3 flex h-full w-full flex-col items-center justify-center gap-4 p-5 py-18 text-center"
                    style="background: radial-gradient(ellipse farthest-side at bottom center, #BFB9A1 0%, #F0E8C9 50%, #FFF8D6 100%);"
                >
                    <h1 class="text-[3.75em] font-extrabold text-neutral-800">
                        Castlebyte Techsolutions
                    </h1>
                    <h3 class="font-medium text-neutral-500 uppercase">
                        Lead Developer - Team Project - Early 2025
                    </h3>
                    <h2 class="line-clamp-3 text-base text-neutral-700">
                        "CastleByte delivers full-cycle services—from intuitive UI/UX design and CRM/ERP systems to e-commerce development, mobile apps, SEO, and ongoing IT support—empowering businesses to scale in the digital age."
                    </h2>

                    <a
                        href="/#"
                        aria-current="page"
                        class="group radient-pumpkin-bg relative inline-flex h-12 w-40 items-center justify-center"
                    >
                        <span class="radient-medium-blue-bg absolute top-0 right-0 h-full w-0 transition-all duration-500 ease-in-out group-hover:right-auto group-hover:left-0 group-hover:w-full"></span>
                        <span class="z-10 text-xl font-medium tracking-wide text-slate-100">
                            About me
                        </span>
                    </a>

                    <div class="absolute bottom-3 left-1/2 flex w-full -translate-x-1/2 items-center justify-center text-neutral-400">
                        <img
                            src="https://castlebyte.pixl8media.com/images/logos/cts-logo-castle.png"
                            alt=""
                            class="h-12 grayscale-100"
                        />
                        <h1 class="text-2xl font-bold uppercase">castlebyte techsolutions</h1>
                    </div>
                </div>
            </div>
        </section>
    }
}
