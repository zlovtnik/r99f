use gloo_net::http::{Request, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::models::user::ApiError;

/// API base URL, configured at compile time via `API_BASE_URL` env var.
/// Defaults to "http://localhost:3000/api" for local development.
/// 
/// Set during build: `API_BASE_URL=https://api.example.com/api trunk build`
const API_BASE_URL: &str = match option_env!("API_BASE_URL") {
    Some(url) => url,
    None => "http://localhost:3000/api",
};

/// Returns the configured API base URL (useful for debugging/display)
pub fn api_base_url() -> &'static str {
    API_BASE_URL
}

// ─────────────────────────────────────────────────────────────────────────────
// Helper functions to reduce duplication
// ─────────────────────────────────────────────────────────────────────────────

/// Constructs full URL from endpoint, handling leading slash
fn build_url(endpoint: &str) -> String {
    if endpoint.starts_with('/') {
        format!("{}{}", API_BASE_URL, endpoint)
    } else {
        format!("{}/{}", API_BASE_URL, endpoint)
    }
}

/// Attaches Authorization header if token is provided
fn attach_auth(request: RequestBuilder, token: Option<&str>) -> RequestBuilder {
    match token {
        Some(t) => request.header("Authorization", &format!("Bearer {}", t)),
        None => request,
    }
}

/// Handles response parsing: returns parsed T on success, formatted error on failure
async fn handle_response<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    let status = response.status();
    if response.ok() {
        response
            .json::<T>()
            .await
            .map_err(|e| format!("Parse error (status {}): {}", status, e))
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or(ApiError { error: "Unknown error".to_string() });
        Err(format!("API error (status {}): {}", status, error.error))
    }
}

/// Handles response for requests with no response body (e.g., DELETE)
async fn handle_empty_response(response: Response) -> Result<(), String> {
    let status = response.status();
    if response.ok() {
        Ok(())
    } else {
        let error: ApiError = response
            .json()
            .await
            .unwrap_or(ApiError { error: "Unknown error".to_string() });
        Err(format!("API error (status {}): {}", status, error.error))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API functions
// ─────────────────────────────────────────────────────────────────────────────

pub async fn get<T: DeserializeOwned>(endpoint: &str, token: Option<&str>) -> Result<T, String> {
    let request = attach_auth(Request::get(&build_url(endpoint)), token);
    
    let response = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    handle_response(response).await
}

pub async fn post<T: DeserializeOwned, B: Serialize>(
    endpoint: &str,
    body: &B,
    token: Option<&str>,
) -> Result<T, String> {
    let request = attach_auth(
        Request::post(&build_url(endpoint)).header("Content-Type", "application/json"),
        token,
    );

    let response = request
        .json(body)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    handle_response(response).await
}

pub async fn put<T: DeserializeOwned, B: Serialize>(
    endpoint: &str,
    body: &B,
    token: Option<&str>,
) -> Result<T, String> {
    let request = attach_auth(
        Request::put(&build_url(endpoint)).header("Content-Type", "application/json"),
        token,
    );

    let response = request
        .json(body)
        .map_err(|e| format!("Serialization error: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    handle_response(response).await
}

pub async fn delete(endpoint: &str, token: Option<&str>) -> Result<(), String> {
    let request = attach_auth(Request::delete(&build_url(endpoint)), token);

    let response = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    handle_empty_response(response).await
}
