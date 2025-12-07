use leptos::*;
use leptos_router::*;

use crate::services::auth::{logout, use_auth};

#[component]
pub fn Header() -> impl IntoView {
    let auth = use_auth();
    let is_logged_in = move || auth.token.get().is_some();

    // State for mobile menu toggle
    let (menu_open, set_menu_open) = create_signal(false);

    view! {
        <nav class="navbar is-dark is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <A href="/" class="navbar-item has-text-weight-bold">
                    "R99F"
                </A>
                <a
                    role="button"
                    class="navbar-burger"
                    class:is-active=menu_open
                    aria-label="menu"
                    aria-expanded=move || menu_open.get().to_string()
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div id="mainNavbar" class="navbar-menu" class:is-active=menu_open>
                <div class="navbar-start">
                    <A href="/" class="navbar-item">"Home"</A>
                </div>

                <div class="navbar-end">
                    <Show
                        when=is_logged_in
                        fallback=|| view! {
                            <A href="/login" class="navbar-item">"Login"</A>
                            <A href="/register" class="navbar-item">"Register"</A>
                        }
                    >
                        <A href="/dashboard" class="navbar-item">"Dashboard"</A>
                        <A href="/profile" class="navbar-item">"Profile"</A>
                        <div class="navbar-item">
                            <button class="button is-dark" on:click=move |_| logout()>
                                "Logout"
                            </button>
                        </div>
                    </Show>
                </div>
            </div>
        </nav>
    }
}
