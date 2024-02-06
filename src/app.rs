mod home;
mod nav;

use home::*;
use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes};
use nav::*;

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
