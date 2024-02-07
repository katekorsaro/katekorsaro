use leptos::{component, view, IntoView, use_context, ReadSignal, WriteSignal, SignalGet};
use crate::state::*;

#[component]
pub fn Nav() -> impl IntoView {
    let (state, _set_state): (ReadSignal<State>, WriteSignal<State>) = use_context().unwrap();
    view! {
        <div class="container mx-auto text-right mt-4">
            <ul>
                <li class="inline-block underline p-2 rounded rounded-4"
                    class=("bg-slate-700", move || state.get().navigation.selected_tab == SelectedTab::Home)
                >
                    <a href="/">"Home"</a>
                </li>
                <li class="inline-block underline p-2 rounded rounded-4"
                    class=("bg-slate-700", move || state.get().navigation.selected_tab == SelectedTab::Blog)
                >
                    <a href="/blog">"Blog"</a>
                </li>
            </ul>
        </div>
    }
}
