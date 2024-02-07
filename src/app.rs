mod components;
mod pages;

use crate::*;
pub use components::*;
pub use pages::*;

#[component]
pub fn App() -> impl IntoView {
    let (state, set_state) = create_signal(crate::state::State::new());
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

pub fn use_state() -> (ReadSignal<crate::state::State>, WriteSignal<crate::state::State>) {
    use_context().unwrap()
}
