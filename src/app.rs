use leptos::*;
use leptos_router::*;

use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::pages::home::HomePage;
use crate::pages::login::LoginPage;
use crate::pages::register::RegisterPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::profile::ProfilePage;
use crate::services::auth::AuthContext;

#[component]
pub fn App() -> impl IntoView {
    // Create auth context
    let (auth_token, set_auth_token) = create_signal(get_stored_token());
    let (current_user, set_current_user) = create_signal(None::<crate::models::user::UserResponse>);

    // Provide auth context to all children
    provide_context(AuthContext {
        token: auth_token,
        set_token: set_auth_token,
        user: current_user,
        set_user: set_current_user,
    });

    view! {
        <Router>
            <div class="app-container">
                <Header/>
                <main class="section main-content">
                    <div class="container">
                        <Routes>
                            <Route path="/" view=HomePage/>
                            <Route path="/login" view=LoginPage/>
                            <Route path="/register" view=RegisterPage/>
                            <Route path="/dashboard" view=DashboardPage/>
                            <Route path="/profile" view=ProfilePage/>
                        </Routes>
                    </div>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

fn get_stored_token() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    storage.get_item("auth_token").ok()?
}
