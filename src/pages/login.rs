use leptos::*;
use leptos_router::*;

use crate::services::auth::{login as do_login, use_auth};

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (error, set_error) = create_signal(None::<String>);
    let (loading, set_loading) = create_signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_error.set(None);
        set_loading.set(true);

        let email = email.get();
        let password = password.get();
        let navigate = navigate.clone();

        spawn_local(async move {
            match do_login(email, password).await {
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
                <h2 class="title is-4 has-text-centered">"Login"</h2>

                <Show when=move || error.get().is_some()>
                    <div class="notification is-danger" role="alert" aria-live="assertive">
                        {move || error.get().unwrap()}
                    </div>
                </Show>

                <form on:submit=on_submit>
                    <div class="field">
                        <label class="label" for="email">"Email"</label>
                        <div class="control">
                            <input
                                class="input"
                                type="email"
                                id="email"
                                placeholder="your@email.com"
                                autocomplete="email"
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
                                autocomplete="current-password"
                                required
                                disabled=loading
                                prop:value=password
                                on:input=move |ev| set_password.set(event_target_value(&ev))
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
                                "Login"
                            </button>
                        </div>
                    </div>
                </form>

                <p class="has-text-centered mt-4">
                    "Don't have an account? "
                    <A href="/register">"Register"</A>
                </p>
            </div>
        </div>
    }
}
