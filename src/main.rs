mod app;
mod data;
mod state;

pub use app::*;
pub use data::*;
pub use state::*;

pub use leptos::*;
pub use leptos_router::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
