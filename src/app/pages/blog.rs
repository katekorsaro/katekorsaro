use leptos::{component,view,IntoView,ReadSignal, WriteSignal, use_context, SignalUpdate};
use crate::app::components::{Nav};
use crate::state::*;

#[component]
pub fn Blog () -> impl IntoView {
    let (_state, set_state): (ReadSignal<State>, WriteSignal<State>) = use_context().unwrap();
    set_state.update(|x| x.navigation.selected_tab = SelectedTab::Blog);
    view! {
        <Nav/>
        <div class="container mt-16 mx-auto text-center">
            <h1 class="text-3xl text-pink-500">Coming soon...</h1>
        </div>
    }
}
