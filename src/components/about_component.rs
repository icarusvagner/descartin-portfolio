use leptos::prelude::*;
use leptos_icons::Icon as LeptosIcon;
use leptos_router::components::A;
use phosphor_leptos::{
	Icon, CALENDAR_DOTS, DOWNLOAD_SIMPLE, ENVELOPE_SIMPLE, GRADUATION_CAP, MAP_PIN_AREA, PHONE,
};

#[component]
pub fn AboutComponent() -> impl IntoView {
	view! {
		<section id="about" class="container py-20 px-4 mx-auto">
			<div class="mx-auto max-w-3xl">
				<h2 class="mb-2 text-3xl font-bold">"About Me"</h2>
				<div class="mb-8 w-20 h-1 bg-primary"></div>
				<div class="grid gap-8 md:grid-cols-2">
					<div>
						<p class="mb-4">
							"I'm a passionate software engineer with 3+ years of experience building web applications and digital products. I specialize in Rust, Leptos, Axum, and modern web technologies."
						</p>
						<p class="mb-4">
							"My approach combines technical expertise with a strong focus on user experience and business goals. I enjoy solving complex problems and turning ideas into reality through clean, efficient code."
						</p>
						<p>
							"When I'm not coding, you'll find me reading manga and tech blogs, or experimenting with new technologies. I'm always eager to learn and grow as a developer."
						</p>
					</div>
					<div>
						<h3 class="mb-4 text-xl font-semibold">
							"Personal Information"
						</h3>
						<ul class="space-y-3">
							<li class="flex gap-3 items-center">
								<Icon
									icon=ENVELOPE_SIMPLE
									attr:class="text-primary h-8 w-8"
								/>
								<span>"cnale1211435@gmail.com"</span>
							</li>
							<li class="flex gap-3 items-center">
								<Icon icon=PHONE attr:class="text-primary h-8 w-8" />
								<span>"(+63)993 671 0293"</span>
							</li>
							<li class="flex gap-3 items-center">
								<Icon icon=MAP_PIN_AREA attr:class="text-primary h-8 w-8" />
								<span>"Lapu-lapu City, Cebu, Philippines"</span>
							</li>
							<li class="flex gap-3 items-center">
								<Icon
									icon=CALENDAR_DOTS
									attr:class="text-primary h-8 w-8"
								/>
								<span>"3+ Years Experience"</span>
							</li>
							<li class="flex gap-3 items-center">
								<Icon
									icon=GRADUATION_CAP
									attr:class="text-primary h-8 w-8"
								/>
								<span>
									"B.S Information Technology, Cordova Public College"
								</span>
							</li>
						</ul>
						<div class="flex gap-4 mt-6">
							<A
								href="https://github.com/icarusvagner"
								attr:class="cursor-pointer"
							>
								<LeptosIcon
									icon=icondata::FaSquareGithubBrands
									attr:class="h-10 w-10"
								/>
							</A>
							<A
								href="https://www.linkedin.com/in/lance-phillip-descartin-189139241/"
								attr:class="cursor-pointer"
							>
								<LeptosIcon
									icon=icondata::FaLinkedinBrands
									attr:class="h-10 w-10"
								/>
							</A>
							<A
								href="https://youtube.com/@cnalecoding"
								attr:class="cursor-pointer"
							>
								<LeptosIcon
									icon=icondata::FaSquareYoutubeBrands
									attr:class="h-10 w-10"
								/>
							</A>
							<A href="/lance-phillip-resume.docx" attr:download>
								<Icon icon=DOWNLOAD_SIMPLE attr:class="h-10 w-10" />
							</A>
						</div>
					</div>
				</div>
			</div>
		</section>
	}.into_any()
}
