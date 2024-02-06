mod about_me;
mod hero;
mod nav;
mod projects;

use about_me::*;
use hero::*;
use leptos::{component, view, IntoView};
use nav::*;
use projects::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Nav/>
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
