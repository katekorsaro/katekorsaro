use crate::app::components::{AboutMe, Hero, Nav, Projects};
use leptos::{component, view, IntoView, ReadSignal, WriteSignal, use_context, SignalUpdate};
use crate::state::*;

#[component]
pub fn Home() -> impl IntoView {
    let (_state, set_state): (ReadSignal<State>, WriteSignal<State>) = use_context().unwrap();
    set_state.update(|x| x.navigation.selected_tab = SelectedTab::Home);
    view! {
        <Nav/>
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
