use leptos::prelude::*;
use leptos_icons::Icon as LeptosIcon;
use leptos_router::components::A;
use phosphor_leptos::{Icon, ENVELOPE_SIMPLE, MAP_PIN_AREA, PHONE};

#[component]
pub fn ContactComponent() -> AnyView {
	view! {
		<section id="contact" class="container py-20 px-4 mx-auto">
			<div class="mx-auto max-w-3xl">
				<h2 class="mb-2 text-3xl font-bold">"Get in Touch"</h2>
				<div class="mb-8 w-20 h-1 bg-primary"></div>

				<div class="p-3 w-full rounded-lg card bg-base-100">
					<div class="flex overflow-y-auto relative flex-col flex-auto p-3 w-full h-auto subpixel-antialiased text-left break-words place-content-inherit align-items-inherit">
						<div class="grid gap-8 md:grid-cols-2">
							<div>
								<h3 class="mb-4 text-xl font-semibold">"Contact Information"</h3>
								<p class="mb-6">
									"I'm interested in freelance opportunities, especially ambitious or large projects. However, if you have other requests or questions, don't hesitate to contact me."
								</p>
								<div class="space-y-4">
									<div class="flex gap-3 items-center text-base-content">
										<div class="p-2 rounded-full bg-primary text-primary-content">
											<Icon icon=ENVELOPE_SIMPLE attr:class="h-6 w-6" />
										</div>
										<div>
											<p class="text-sm text-base-content/70">"Mail"</p>
											<p>"ruthakaza@gmail.com"</p>
										</div>
									</div>
									<div class="flex gap-3 items-center">
										<div class="p-2 rounded-full bg-primary text-primary-content">
											<Icon icon=PHONE attr:class="h-6 w-6" />
										</div>
										<div>
											<p class="text-sm text-base-content/70">"Phone"</p>
											<p>"(+63)993 671 0293"</p>
										</div>
									</div>
									<div class="flex gap-3 items-center">
										<div class="p-2 rounded-full bg-primary text-primary-content">
											<Icon icon=MAP_PIN_AREA attr:class="h-6 w-6" />
										</div>
										<div>
											<p class="text-sm text-base-content/70">"Address"</p>
											<p>"Lapu-lapu City, Cebu City, Philippines"</p>
										</div>
									</div>
									<div class="flex gap-4 mt-8">
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
									</div>
								</div>
							</div>
							<form class="space-y-4">
								<div class="">
									<label
										class="block mb-1 text-sm font-medium text-base-content/70"
										for="name"
									>
										"Name"
									</label>
									<input
										id="name"
										name="name"
										type="text"
										required
										class="py-2 px-4 w-full rounded-md border focus:ring-2 focus:outline-none border-base-content/70 bg-background focus:ring-primary"
									/>
								</div>
								<div class="">
									<label
										class="block mb-1 text-sm font-medium text-base-content/70"
										for="email"
									>
										"Email"
									</label>
									<input
										id="email"
										name="email"
										type="email"
										required
										class="py-2 px-4 w-full rounded-md border focus:ring-2 focus:outline-none border-base-content/70 bg-background focus:ring-primary"
									/>
								</div>
								<div class="">
									<label
										class="block mb-1 text-sm font-medium text-base-content/70"
										for="subject"
									>
										"Subject"
									</label>
									<input
										id="subject"
										name="subject"
										type="text"
										required
										class="py-2 px-4 w-full rounded-md border focus:ring-2 focus:outline-none border-base-content/70 bg-background focus:ring-primary"
									/>
								</div>
								<div class="">
									<label
										class="block mb-1 text-sm font-medium text-base-content/70"
										for="message"
									>
										"Message"
									</label>
									<textarea
										id="message"
										name="message"
										required
										row="4"
										class="py-2 px-4 w-full rounded-md border resize-none focus:ring-2 focus:outline-none border-base-content/70 bg-background focus:ring-primary"
									></textarea>
								</div>
								<button type="submit" class="w-full rounded-md btn btn-primary">
									"Send Message"
								</button>
							</form>
						</div>
					</div>
				</div>
			</div>
		</section>
	}.into_any()
}
