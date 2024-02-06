use crate::{Project, ProjectStatus};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div
            class=("border-red-500", project.status_tag == ProjectStatus::Red)
            class=("border-amber-500", project.status_tag == ProjectStatus::Amber)
            class=("border-emerald-500", project.status_tag == ProjectStatus::Emerald)
            class=("border-cyan-500", project.status_tag == ProjectStatus::Cyan)
            class="bg-slate-800 rounded rounded-4 p-2 border-s-4"
        >
            <h1 class="text-cyan-500">{project.title}</h1>
            <p class="text-xs">"Status: " {project.status}</p>
            <p class="mt-2">{project.description}</p>
        </div>
    }
}
