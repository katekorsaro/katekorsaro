mod components;
mod pages;

use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};
use pages::Home;

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
