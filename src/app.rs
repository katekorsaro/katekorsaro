mod hero;
mod about_me;

use leptos::{component,view,IntoView};
use hero::*;
use about_me::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Hero/>
        <AboutMe/>
    }
}
