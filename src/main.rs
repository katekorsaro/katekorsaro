mod app;
mod data;
mod state;

use app::*;
use data::*;
use state::*;

use leptos::*;
use leptos_router::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
