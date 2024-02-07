mod projects;

use crate::app::components::{Nav,AboutMe,Hero};
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
