use crate::*;

#[component]
pub fn ProjectCard(project: ReadSignal<Project>) -> impl IntoView {
    view! {
        <div
            class=("border-rose-500", move || project.get().status_tag == ProjectStatus::Red)
            class=("border-orange-500", move || project.get().status_tag == ProjectStatus::Amber)
            class=("border-lime-500", move || project.get().status_tag == ProjectStatus::Emerald)
            class=("border-cyan-500", move || project.get().status_tag == ProjectStatus::Cyan)
            class="bg-slate-900 rounded rounded-4 p-2 border-s-4 shadow"
        >
            <h2
                class="text-cyan-500 text-lg font-semibold"
                class=("underline", move || project.get().project_url.is_some())
            >
                <a href=move || {
                    project.get().project_url.clone()
                }>{move || project.get().title.clone()}</a>
            </h2>
            <p class="text-slate-400 text-xs">"Status: " {move || project.get().status.clone()}</p>
            <p class="mt-2">{move || project.get().description.clone()}</p>
            <p class="text-slate-500 mt-2 text-xs">

                {move || {
                    project
                        .get()
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
