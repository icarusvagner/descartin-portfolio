use leptos::prelude::*;

use crate::{
    components::skills_component::skills::CardComponent,
    utils::svgs::toolsothers::{
        Agile, CiCd, FigmaDesign, Git, GitHub, GithubActions, Jest, LinuxTux, Nginx, Python,
    },
};

#[component]
pub fn ToolsOthersTab() -> AnyView {
    view! {
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <LinuxTux />
                </div>
                <p class="text-sm">"Linux"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <Git />
                </div>
                <p class="text-sm">"Git"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <GitHub />
                </div>
                <p class="text-sm">"GitHub"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <CiCd />
                </div>
                <p class="text-sm">"CI / CD"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <Agile />
                </div>
                <p class="text-sm">"Agile"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <GithubActions />
                </div>
                <p class="text-sm">"GitHub Actions"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <Nginx />
                </div>
                <p class="text-sm">"Nginx"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <Python />
                </div>
                <p class="text-sm">"Python"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <FigmaDesign />
                </div>
                <p class="text-sm">"Figma"</p>
            </div>
        </CardComponent>
        <CardComponent>
            <div>
                <div class="flex justify-center mb-2">
                    <Jest />
                </div>
                <p class="text-sm">"Jest"</p>
            </div>
        </CardComponent>
    }
    .into_any()
}
