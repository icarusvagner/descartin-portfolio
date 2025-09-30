use leptos::prelude::*;
use leptos_router::components::A;
use phosphor_leptos::{Icon, DOWNLOAD_SIMPLE};

#[component]
pub fn HeroComponent() -> impl IntoView {
    view! {
        <section class="container flex flex-col gap-8 items-center py-20 px-4 mx-auto md:flex-row md:py-32 font-poppins">
            <div class="flex-1">
                <div class="inline-flex relative shrink-0">"Available for hire"</div>
                <h1 class="mb-4 text-4xl font-bold md:text-5xl font-oswald">
                    "Lance Phillip Descartin"
                </h1>
                <h2 class="mb-5 text-2xl md:text-3xl text-base-content">
                    "Software Engineer & Full-Stack Developer"
                </h2>
                <p class="mb-8 max-w-lg text-secondary">
                    "I build exceptional and accessible digital experiences for the web. Focused on creating intuitive, high-performance applications that solve real problems."
                </p>
                <div class="flex flex-wrap gap-4">
                    <A href="#projects" attr:class="btn btn-primary">
                        "View my work"
                    </A>
                    <A href="#contact" attr:class="btn btn-outline">
                        "Contact me"
                    </A>
                    <A href="/lance-phillip-resume.docx" attr:class="btn btn-ghost" attr:download>
                        "Download Resume"
                        <Icon icon=DOWNLOAD_SIMPLE attr:class="h-6 w-6" />
                    </A>
                </div>
            </div>
            <div class="w-full md:w-1/2">
                <div class="relative mx-auto w-full max-w-md aspect-square">
                    <div class="absolute inset-0 justify-center bg-gradient-to-b rounded-full opacity-70 from-[#06b6d4] via-[#2563eb] to-[#6366f1]"></div>
                    <span class="flex object-cover overflow-hidden relative z-0 justify-center items-center w-full h-full align-middle rounded-full ring-2 ring-offset-2 shadow-lg box-border outline-solid outline-transparent data-[focus-visible=true]:z-10 data-[focus-visible=true]:outline-2 data-[focus-visible=true]:outline-focus data-[focus-visible=true]:outline-offset-2 text-tiny bg-primary text-primary-foreground ring-offset-background ring-primary dark:ring-offset-background-dark">
                        <img
                            src="https://picsum.photos/1200/1200"
                            class="flex object-cover w-full h-full"
                        />
                    </span>
                </div>
            </div>
        </section>
    }
}
