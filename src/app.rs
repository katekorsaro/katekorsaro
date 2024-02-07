mod components;
mod pages;

use leptos::{component, view, IntoView, create_signal,provide_context};
use leptos_router::{Route, Router, Routes};
use pages::*;
use crate::state::*;

#[component]
pub fn App() -> impl IntoView {
    let (state, set_state) = create_signal(State::new());
    provide_context((state, set_state));
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/blog" view=Blog/>
                </Routes>
            </main>
        </Router>
    }
}
