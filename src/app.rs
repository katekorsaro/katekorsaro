mod components;
mod pages;

use crate::state::*;
use leptos::*;
use leptos_router::{Route, Router, Routes};
use pages::*;

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

pub fn use_state () -> (ReadSignal<State>, WriteSignal<State>) {
    use_context().unwrap()
}
