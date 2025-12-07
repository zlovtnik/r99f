use leptos::*;

use crate::models::user::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse};
use crate::services::api;

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub token: ReadSignal<Option<String>>,
    pub set_token: WriteSignal<Option<String>>,
    pub user: ReadSignal<Option<UserResponse>>,
    pub set_user: WriteSignal<Option<UserResponse>>,
}

pub fn use_auth() -> AuthContext {
    expect_context::<AuthContext>()
}

pub async fn login(email: String, password: String) -> Result<AuthResponse, String> {
    let request = LoginRequest { email, password };
    let response: AuthResponse = api::post("/auth/login", &request, None).await?;
    store_token(&response.token);
    Ok(response)
}

pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<AuthResponse, String> {
    let request = CreateUserRequest {
        email,
        username,
        password,
    };
    let response: AuthResponse = api::post("/auth/register", &request, None).await?;
    store_token(&response.token);
    Ok(response)
}

pub fn logout() {
    clear_token();
    // Reload the page to clear state
    if let Some(window) = web_sys::window() {
        let _ = window.location().set_href("/");
    }
}

pub async fn get_current_user(token: &str) -> Result<UserResponse, String> {
    api::get("/users/me", Some(token)).await
}

pub async fn get_all_users(token: &str) -> Result<Vec<UserResponse>, String> {
    api::get("/users", Some(token)).await
}

fn store_token(token: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or("No window available")?;
    let storage = window
        .local_storage()
        .map_err(|_| "Failed to access localStorage")?
        .ok_or("localStorage not available")?;
    storage
        .set_item("auth_token", token)
        .map_err(|_| "Failed to store token".to_string())
}

fn clear_token() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("auth_token");
        }
    }
}
