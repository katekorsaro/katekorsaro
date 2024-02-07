use crate::app::components::{AboutMe, Hero, Nav, Projects};
use crate::state::*;
use leptos::{component, use_context, view, IntoView, ReadSignal, SignalUpdate, WriteSignal};

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
