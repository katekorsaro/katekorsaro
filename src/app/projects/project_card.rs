use leptos::{component, view, IntoView, ReadSignal};

#[component]
pub fn ProjectCard(
    title: ReadSignal<String>,
    status: ReadSignal<String>,
    description: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-800 rounded rounded-4 p-2 border-s-4 border-amber-500">
            <h1 class="text-cyan-500">{title}</h1>
            <p class="text-xs">"Status: " {status}</p>
            <p class="mt-2">{description}</p>
        </div>
    }
}
