mod about_me;
mod hero;
mod projects;

use about_me::*;
use hero::*;
use leptos::{component, view, IntoView};
use projects::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
