use leptos::prelude::*;

use crate::components::NextWorkButton;

#[component]
pub fn Pixl8MultimediaView() -> impl IntoView {
    view! {
        <section
            class="min-h-[580px] xl:min-h-[53rem] bg-center bg-cover bg-no-repeat flex flex-col items-center justify-center gap-5"
            style="background-image: linear-gradient(to top, rgba(0,0,0,0.8) 0%, rgba(0,0,0,0.7) 100%), url('/public/assets/pixl8media.jpeg')"
        >
            <div class="px-3 flex flex-col px-3 gap-1 flex-1 items-center justify-center">
                <h1 class="text-[3.9975843648em] font-extrabold text-neutral-100 capitalize text-center">
                    "Pixl8Multimedia"
                </h1>
                <h3 class="text-[1.125em] italic text-neutral-100 font-normal max-w-xl text-center">
                    "Discover The Captivating World Of Our Book Trailer Collection, Where Imagination Comes To Life On The Screen."
                </h3>
            </div>

            <div class="w-full px-4 py-8">
                <div class="flex flex-wrap justify-center items-center gap-x-10 gap-y-4 font-bold tracking-[2px] uppercase text-white text-center">
                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"role"</h1>
                        <h3 class="text-neutral-100">"front-end developer"</h3>
                    </div>
                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"context"</h1>
                        <h3 class="text-neutral-100">"website redesign"</h3>
                    </div>
                    <div class="flex text-[.75em] tracking-[2px] font-bold items-center gap-2.5 uppercase">
                        <h1 class="text-sky-400">"period"</h1>
                        <h3 class="text-neutral-100">"middle 2024"</h3>
                    </div>
                </div>
            </div>
        </section>

        <section class="mb-24 min-h-screen">
            <section class="relative flex py-24 w-full flex-col items-center justify-center bg-[#011741] px-3">
                <div class="flex max-w-xl flex-col items-center justify-center gap-8">
                    <h1 class="text-7xl font-black text-stone-100 capitalize">"the project"</h1>
                    <p class="text-center text-stone-200">
                        "Pixl8Media functions as a platform which turns stories into genuine visual representations that outstrip their textual manifestation. Authors who work for film production must develop their unique voice which requires worldwide distribution. Build a place within our team to assist development of narratives beyond mainstream standards and create meaningful connections with audience."
                    </p>
                    <a
                        href="https://pixl8media.com"
                        class="text-stone-100 border py-2 px-5 hover:bg-stone-100 hover:text-stone-800 duration-300 transition-all ease-in-out hover:border-stone-100 text-xl font-medium"
                    >
                        "Visit Website"
                    </a>
                </div>
                <div class="absolute bottom-0 left-1/2 -translate-x-1/2 z-0">
                    <h1 class="text-4xl lg:text-7xl font-playfair font-black tracking-widest text-stone-100/30">
                        "Pixl8Multimedia"
                    </h1>
                </div>
            </section>

            <section class="relative overflow-hidden px-3">
                <div class="mx-auto grid h-full grid-cols-1 gap-12 px-3 py-12 lg:max-w-7xl lg:grid-cols-2 lg:px-0 xl:max-w-6xl">
                    <div class="flex flex-col gap-2">
                        <h1 class="case-project-title">"pixl8multimedia"</h1>
                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "Pixl8Multimedia is a dynamic team of creatives, marketers, and storytellers unified by a mission to deliver impactful digital content across film, publishing, branding, and design."
                        </p>
                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "From concept to completion, we bring bold visions to life helping brands connect with their audiences through cinematic visuals, strategic campaigns, and innovative digital experiences."
                        </p>
                        <p class="mt-10 text-left text-lg font-normal tracking-wide text-neutral-600 dark:text-neutral-200">
                            "One exciting aspect of this project is that it's built using "
                            <strong class="text
                            -[#013299]
                            ">"Rust with Leptos (WASM)"</strong>
                            ", ensuring both performance and safety at the core of the frontend experience."
                        </p>
                    </div>

                    <div class="relative">
                        <div class="h-[480px] w-full lg:w-[490px] bg-[url('/public/assets/works/pixl8/img-01.png')] bg-cover bg-center bg-no-repeat"></div>
                        <div class="absolute top-28 right-5 lg:-right-10 h-[480px] w-md lg:w-[400px] bg-[url('/public/assets/works/pixl8/img-02.png')] bg-cover bg-center bg-no-repeat shadow-[6px_6px_25px_-7px_rgba(13,0,0,0.73)]"></div>
                    </div>
                </div>
            </section>

            <div class="relative mx-auto my-12 grid grid-cols-2 lg:grid-cols-5 gap-10 lg:max-w-7xl xl:max-w-6xl justify-center">
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#011741] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$oxford blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#013299] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$egyptian blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#2F71FF] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$brandies blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center group">
                    <div class="h-40 w-40 rounded-full bg-[#8DB0FA] shadow-[0px_0px_15px_5px_rgba(8,0,0,0.41)]"></div>
                    <h1 class="mt-7 text-neutral-700 dark:text-neutral-100 duration-300 ease-in-out group-hover:text-pumpkin">
                        "$jordy blue"
                    </h1>
                </div>
                <div class="flex flex-col items-center justify-center"></div>
            </div>

            <div class="relative mx-auto my-12 grid lg:grid-cols-2 gap-18 lg:gap-10 lg:max-w-7xl xl:max-w-6xl text-stone-900 dark:text-stone-100">
                <div class="space-y-8 text-center font-roboto">
                    <h1 class="text-4xl">"Roboto"</h1>

                    <div class="space-y-4 text-5xl">
                        <p>"a   b   c   d   e   f   g   h   i   j   k   l"</p>
                        <p>"m   n   o   p   q   r   s   t   u   v   w   x"</p>
                        <p>"y   z   1   2   3   4   5   6   7   9   0"</p>
                    </div>
                </div>

                <div class="space-y-8 text-center font-playfair">
                    <h1 class="text-4xl">"Playfair Display"</h1>

                    <div class="space-y-4 text-5xl">
                        <p>"a   b   c   d   e   f   g   h   i   j   k   l"</p>
                        <p>"m   n   o   p   q   r   s   t   u   v   w   x"</p>
                        <p>"y   z   1   2   3   4   5   6   7   9   0"</p>
                    </div>
                </div>
            </div>

            <div class="relative mx-auto mt-16 flex min-h-screen flex-col gap-1 lg:max-w-7xl xl:max-w-6xl">
                <h3 class="text-center font-medium tracking-wide text-sky-600 dark:text-sky-400">
                    "UI & COMPONENTS"
                </h3>
                <h1 class="text-center text-3xl font-bold tracking-widest text-stone-600 dark:text-stone-300 capitalize">
                    "design"
                </h1>
                <div class="relative mt-10 w-full max-w-6xl mt-20">
                    <div class="absolute top-0 right-0 z-10 h-[500px] w-[60%] overflow-hidden rounded-lg bg-white shadow-[6px_6px_25px_-7px_rgba(13,0,0,0.73)]">
                        <img
                            src="/public/assets/works/pixl8/img-03.png"
                            class="h-full w-full object-cover"
                        />
                    </div>

                    <div class="absolute top-20 left-12 lg:left-40 z-30 h-[500px] w-[70%] overflow-hidden rounded-lg shadow-[6px_6px_25px_-7px_rgba(13,0,0,0.73)]">
                        <img
                            src="/public/assets/works/pixl8/img-05.png"
                            class="h-full w-full object-cover"
                        />
                    </div>

                    <div class="absolute top-40 left-0 z-20 h-[400px] w-[300px] overflow-hidden rounded-md bg-white shadow-[6px_6px_25px_-7px_rgba(13,0,0,0.73)]">
                        <img
                            src="/public/assets/works/pixl8/img-04.png"
                            class="h-full w-full object-cover"
                        />
                    </div>
                </div>
            </div>
        </section>

        <NextWorkButton
            title="goldenyears carehome"
            link="/case-studies/goldenyears"
            color1="bg-[#6B5B2D]"
            color2="bg-[#005D99]"
        />
    }
}
