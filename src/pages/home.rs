use leptos::*;
use leptos_router::*;

use crate::services::auth::use_auth;

#[component]
pub fn HomePage() -> impl IntoView {
    let auth = use_auth();
    let is_logged_in = move || auth.token.get().is_some();

    view! {
        <div class="content">
            <h1 class="title is-1">"Welcome to r99f"</h1>
            <p class="subtitle">"A modern single-page application built with Rust, WebAssembly, and PostgreSQL."</p>

            <div class="box">
                <h2 class="title is-4">"Tech Stack"</h2>
                <ul>
                    <li>"🦀 Frontend: Rust + Leptos (WASM)"</li>
                    <li>"⚡ Backend: Rust + Axum"</li>
                    <li>"🐘 Database: PostgreSQL + SQLx"</li>
                    <li>"🔐 Auth: JWT tokens"</li>
                </ul>
            </div>

            <div class="buttons mt-5">
                <Show
                    when=is_logged_in
                    fallback=move || view! {
                        <A href="/register" class="button is-primary is-medium">"Get Started"</A>
                        <A href="/login" class="button is-dark is-medium">"Login"</A>
                    }
                >
                    <A href="/dashboard" class="button is-primary is-medium">"Go to Dashboard"</A>
                </Show>
            </div>
        </div>
    }
}
