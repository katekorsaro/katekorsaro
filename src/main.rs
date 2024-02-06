mod app;

use app::*;
use leptos::view;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
