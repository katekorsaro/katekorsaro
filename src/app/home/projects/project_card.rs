use crate::{Project, ProjectStatus};
use leptos::{component, view, IntoView};

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let url = project.project_url.clone();

    view! {
        <div
            class=("border-red-500", move || project.status_tag == ProjectStatus::Red)
            class=("border-amber-500", move || project.status_tag == ProjectStatus::Amber)
            class=("border-emerald-500", move || project.status_tag == ProjectStatus::Emerald)
            class=("border-cyan-500", move || project.status_tag == ProjectStatus::Cyan)
            class="bg-slate-800 rounded rounded-4 p-2 border-s-4 shadow"
        >
            <h2 class="text-cyan-500 text-lg font-semibold" class=("underline", move || url.is_some())>
                <a href=move || project.project_url.clone()>{move || project.title.clone()}</a>
            </h2>
            <p class="text-slate-400 text-xs">"Status: " {move || project.status.clone()}</p>
            <p class="mt-2">{move || project.description.clone()}</p>
            <p class="text-slate-500 mt-2 text-xs">

                {move || {
                    project
                        .tags
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|x| format!("#{}", x))
                        .reduce(|acc, e| format!("{acc} {e}"))
                }}

            </p>
        </div>
    }
}
