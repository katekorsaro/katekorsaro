use crate::{Project, ProjectStatus};
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ProjectCard(project: ReadSignal<Project>) -> impl IntoView {
    view! {
        <div
            class=("border-red-500", move || project.get().status_tag == ProjectStatus::Red)
            class=("border-amber-500", move || project.get().status_tag == ProjectStatus::Amber)
            class=("border-emerald-500", move || project.get().status_tag == ProjectStatus::Emerald)
            class=("border-cyan-500", move || project.get().status_tag == ProjectStatus::Cyan)
            class="bg-slate-800 rounded rounded-4 p-2 border-s-4"
        >
            <h1 class="text-cyan-500">{project.get().title}</h1>
            <p class="text-xs">"Status: " {project.get().status}</p>
            <p class="mt-2">{project.get().description}</p>
        </div>
    }
}
