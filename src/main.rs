mod app;
mod components;
mod models;
mod pages;
mod services;

use app::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    log::info!("Starting r99f frontend...");
    mount_to_body(|| view! { <App/> });
}
