mod app;
mod data;

use app::*;
use data::*;
use leptos::view;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
