mod about_me;
mod hero;
mod projects;

use crate::app::Nav;
use about_me::*;
use hero::*;
use leptos::{component, view, IntoView};
use projects::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Nav/>
        <Hero/>
        <AboutMe/>
        <Projects/>
    }
}
