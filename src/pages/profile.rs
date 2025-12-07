use leptos::*;
use leptos_router::*;

use crate::services::auth::{get_current_user, use_auth};

/// Maximum number of retry attempts for fetching user data
const MAX_RETRIES: u32 = 3;

#[component]
pub fn ProfilePage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    // Signal to trigger manual refetch (incremented to force resource re-evaluation)
    let (refetch_trigger, set_refetch_trigger) = create_signal(0u32);

    // Redirect if not logged in
    create_effect(move |_| {
        if auth.token.get().is_none() {
            navigate("/login", Default::default());
        }
    });

    // Fetch user data with retry logic and manual refetch capability
    let user_resource = create_resource(
        move || (auth.token.get(), refetch_trigger.get()),
        move |(token, trigger)| async move {
            let Some(token) = token else {
                return Err("Not authenticated".to_string());
            };

            // Check if we already have user data cached (skip on manual refetch)
            if trigger == 0 {
                if let Some(user) = auth.user.get_untracked() {
                    return Ok(user);
                }
            }

            // Retry loop (immediate retries for transient failures)
            let mut last_error = String::new();
            for attempt in 0..MAX_RETRIES {
                match get_current_user(&token).await {
                    Ok(user) => return Ok(user),
                    Err(e) => {
                        log::warn!("Fetch user attempt {}/{} failed: {}", attempt + 1, MAX_RETRIES, e);
                        last_error = e;
                    }
                }
            }
            Err(format!("Failed after {} attempts: {}", MAX_RETRIES, last_error))
        },
    );

    // Handler for retry button
    let on_retry = move |_| {
        set_refetch_trigger.update(|n| *n += 1);
    };

    view! {
        <div>
            <h1 class="title is-2">"Profile"</h1>

            <Show
                when=move || !user_resource.loading().get()
                fallback=|| view! {
                    <div class="loader-wrapper">
                        <div class="loader"></div>
                    </div>
                }
            >
                {move || {
                    match user_resource.get() {
                        Some(Ok(user)) => view! {
                            <div class="box">
                                <h3 class="title is-5">"Account Information"</h3>

                                <div class="field">
                                    <label class="label">"Username"</label>
                                    <div class="control">
                                        <p class="is-size-5">{user.username.clone()}</p>
                                    </div>
                                </div>

                                <div class="field">
                                    <label class="label">"Email"</label>
                                    <div class="control">
                                        <p class="is-size-5">{user.email.clone()}</p>
                                    </div>
                                </div>

                                <div class="field">
                                    <label class="label">"Member Since"</label>
                                    <div class="control">
                                        <p class="is-size-5">{user.created_at.format("%B %d, %Y").to_string()}</p>
                                    </div>
                                </div>
                            </div>
                        }.into_view(),
                        Some(Err(e)) => view! {
                            <div class="notification is-warning">
                                <p><strong>"Failed to load profile data"</strong></p>
                                <p class="mt-2">{e}</p>
                                <button
                                    class="button is-primary mt-3"
                                    on:click=on_retry
                                >
                                    "Retry"
                                </button>
                            </div>
                        }.into_view(),
                        None => view! {
                            <div class="notification is-warning">
                                <p>"Unable to load profile. Please try again."</p>
                                <button
                                    class="button is-primary mt-3"
                                    on:click=on_retry
                                >
                                    "Retry"
                                </button>
                            </div>
                        }.into_view(),
                    }
                }}
            </Show>
        </div>
    }
}
