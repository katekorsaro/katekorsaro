use crate::{Project, ProjectStatus};
use leptos::{component, view, IntoView};

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div
            class=("border-red-500", move || project.status_tag == ProjectStatus::Red)
            class=("border-amber-500", move || project.status_tag == ProjectStatus::Amber)
            class=("border-emerald-500", move || project.status_tag == ProjectStatus::Emerald)
            class=("border-cyan-500", move || project.status_tag == ProjectStatus::Cyan)
            class="bg-slate-800 rounded rounded-4 p-2 border-s-4"
        >
            <h1 class="text-cyan-500">{move || project.title.clone()}</h1>
            <p class="text-xs text-slate-400">"Status: " {move || project.status.clone()}</p>
            <p class="mt-2">{move || project.description.clone()}</p>
            <p class="text-xs text-slate-500 mt-2">

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
