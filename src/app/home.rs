mod hero;
mod projects;

use crate::app::components::{Nav,AboutMe};
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
