use leptos::*;
use leptos_router::*;

use crate::models::user::UserResponse;
use crate::services::auth::{get_all_users, get_current_user, use_auth};

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    let (error, set_error) = create_signal(None::<String>);

    // Redirect if not logged in
    create_effect(move |_| {
        if auth.token.get().is_none() {
            navigate("/login", Default::default());
        }
    });

    // Guard: only render dashboard content when authenticated
    view! {
        <Show
            when=move || auth.token.get().is_some()
            fallback=|| view! {
                <div class="has-text-centered">
                    <p>"Redirecting to login..."</p>
                </div>
            }
        >
            <DashboardContent auth=auth error=error set_error=set_error />
        </Show>
    }
}

/// Inner component that only renders when authenticated.
/// This ensures resources are only created when a token exists.
#[component]
fn DashboardContent(
    auth: crate::services::auth::AuthContext,
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    // Fetch current user data - always fetch fresh to ensure data matches current token
    let user_resource = create_resource(
        move || auth.token.get(),
        move |token| async move {
            let Some(token) = token else {
                return Err("Not authenticated".to_string());
            };
            // Always fetch fresh user data to ensure it matches the current token
            get_current_user(&token).await
        },
    );

    // Handle user fetch error
    create_effect(move |_| {
        if let Some(Err(e)) = user_resource.get() {
            log::error!("Failed to fetch current user: {}", e);
            set_error.set(Some(format!("Failed to load user: {}", e)));
        }
    });

    // Fetch all users - only runs when token exists
    let users_resource = create_resource(
        move || auth.token.get(),
        move |token| async move {
            let token = token.expect("Token must exist in DashboardContent");
            get_all_users(&token).await
        },
    );

    // Handle users fetch error
    create_effect(move |_| {
        if let Some(Err(e)) = users_resource.get() {
            log::error!("Failed to fetch users list: {}", e);
            set_error.set(Some(format!("Failed to load users: {}", e)));
        }
    });

    let users = move || -> Vec<UserResponse> {
        users_resource
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
    };

    let loading = move || user_resource.loading().get() || users_resource.loading().get();

    view! {
        <div>
            <h1 class="title is-2">"Dashboard"</h1>

            <Show when=move || error.get().is_some()>
                <div class="notification is-danger">
                    {move || error.get().unwrap()}
                </div>
            </Show>

            <Show
                when=move || !loading()
                fallback=|| view! {
                    <div class="loader-wrapper">
                        <div class="loader"></div>
                    </div>
                }
            >
                <div class="columns">
                    <div class="column is-one-third">
                        <div class="box stat-card">
                            <div class="stat-value">{move || users().len()}</div>
                            <div class="stat-label">"Total Users"</div>
                        </div>
                    </div>

                    <div class="column is-two-thirds">
                        <div class="box">
                            <h3 class="title is-5">"Welcome back!"</h3>
                            <p>
                                {move || {
                                    user_resource.get()
                                        .and_then(|r| r.ok())
                                        .map(|u| format!("Logged in as {}", u.username))
                                        .unwrap_or_else(|| "Loading...".to_string())
                                }}
                            </p>
                        </div>
                    </div>
                </div>

                <div class="box mt-5">
                    <h3 class="title is-5">"All Users"</h3>
                    <For
                        each=move || users()
                        key=|user| user.id
                        children=move |user| view! {
                            <div class="user-item">
                                <p><strong>{user.username.clone()}</strong></p>
                                <p class="is-size-7 has-text-grey-light">{user.email.clone()}</p>
                            </div>
                        }
                    />
                </div>
            </Show>
        </div>
    }
}
