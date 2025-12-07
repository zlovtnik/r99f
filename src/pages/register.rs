use leptos::*;
use leptos_router::*;

use crate::services::auth::{register as do_register, use_auth};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (confirm_password, set_confirm_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);

        let username = username.get();
        let username_trimmed = username.trim();

        // Username validation: 3-20 chars, only letters, numbers, underscore, hyphen
        if username_trimmed.len() < 3 {
            set_error.set(Some("Username must be at least 3 characters".to_string()));
            return;
        }
        if username_trimmed.len() > 20 {
            set_error.set(Some("Username must be at most 20 characters".to_string()));
            return;
        }
        if !username_trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            set_error.set(Some("Username can only contain letters, numbers, underscore, and hyphen".to_string()));
            return;
        }

        let password = password.get();
        let confirm = confirm_password.get();

        if password != confirm {
            set_error.set(Some("Passwords do not match".to_string()));
            return;
        }

        if password.len() < 8 {
            set_error.set(Some("Password must be at least 8 characters".to_string()));
            return;
        }

        set_loading.set(true);

        let email = email.get();
        let username = username_trimmed.to_string();
        let navigate = navigate.clone();

        spawn_local(async move {
            match do_register(email, username, password).await {
                Ok(response) => {
                    auth.set_token.set(Some(response.token));
                    auth.set_user.set(Some(response.user));
                    navigate("/dashboard", Default::default());
                    // Don't reset loading - component will unmount on navigation
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="auth-section">
            <div class="box auth-box">
                <h2 class="title is-4 has-text-centered">"Create Account"</h2>

                <Show when=move || error.get().is_some()>
                    <div class="notification is-danger">
                        {move || error.get().unwrap()}
                    </div>
                </Show>

                <form on:submit=on_submit>
                    <div class="field">
                        <label class="label" for="username">"Username"</label>
                        <div class="control">
                            <input
                                class="input"
                                type="text"
                                id="username"
                                placeholder="johndoe"
                                required
                                minlength="3"
                                maxlength="20"
                                pattern="[a-zA-Z0-9_-]+"
                                title="3-20 characters: letters, numbers, underscore, hyphen"
                                disabled=loading
                                prop:value=username
                                on:input=move |ev| set_username.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label" for="email">"Email"</label>
                        <div class="control">
                            <input
                                class="input"
                                type="email"
                                id="email"
                                placeholder="your@email.com"
                                required
                                disabled=loading
                                prop:value=email
                                on:input=move |ev| set_email.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label" for="password">"Password"</label>
                        <div class="control">
                            <input
                                class="input"
                                type="password"
                                id="password"
                                placeholder="••••••••"
                                required
                                minlength="8"
                                disabled=loading
                                prop:value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label" for="confirm_password">"Confirm Password"</label>
                        <div class="control">
                            <input
                                class="input"
                                type="password"
                                id="confirm_password"
                                placeholder="••••••••"
                                required
                                minlength="8"
                                disabled=loading
                                prop:value=confirm_password
                                on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                            />
                        </div>
                    </div>

                    <div class="field">
                        <div class="control">
                            <button
                                type="submit"
                                class="button is-primary is-fullwidth"
                                class:is-loading=loading
                                disabled=loading
                            >
                                "Register"
                            </button>
                        </div>
                    </div>
                </form>

                <p class="has-text-centered mt-4">
                    "Already have an account? "
                    <A href="/login">"Login"</A>
                </p>
            </div>
        </div>
    }
}
