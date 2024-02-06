use crate::Project;
use leptos::{component, view, IntoView, ReadSignal, SignalGet};

#[component]
pub fn ProjectCard(project: ReadSignal<Project>) -> impl IntoView {
    view! {
        <div class="bg-slate-800 rounded rounded-4 p-2 border-s-4 border-amber-500">
            <h1 class="text-cyan-500">{project.get().title}</h1>
            <p class="text-xs">"Status: " {project.get().status}</p>
            <p class="mt-2">{project.get().description}</p>
        </div>
    }
}
