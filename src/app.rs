mod about_me;
mod hero;
mod nav;
mod projects;
mod home;

use about_me::*;
use hero::*;
use leptos::{component, view, IntoView};
use leptos_router::{Router, Routes, Route};
use nav::*;
use projects::*;
use home::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
