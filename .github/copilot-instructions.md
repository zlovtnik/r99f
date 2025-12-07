# Copilot Instructions for r99f

## Project Overview

**Rust WASM SPA** using **Leptos 0.6** with client-side rendering (CSR). Pure frontend app compiled to WebAssembly.

## Architecture

- **Framework**: Leptos 0.6 CSR compiled to WASM via Trunk
- **HTTP Client**: `gloo-net` (browser-native, not `reqwest`)
- **State**: Leptos signals + context for auth
- **Build**: Trunk (serves on port 8080, builds to `dist/`)

## Key Patterns

### Component Structure
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    view! { <div>"Content"</div> }
}
```

### Auth Context
Provided in `app.rs`, consumed via `use_auth()`:
```rust
let auth = use_auth();
let token = auth.token.get();
auth.set_token.set(Some(new_token));
```

### API Calls
All HTTP goes through `src/services/api.rs`:
```rust
api::get::<ResponseType>("/endpoint", Some(&token)).await
api::post::<ResponseType, RequestType>("/endpoint", &body, token).await
```

### Async in Components
Use `create_resource` for data fetching (prevents race conditions):
```rust
let data_resource = create_resource(
    move || auth.token.get(),  // source signal - refetches when this changes
    move |token| async move {
        if let Some(token) = token {
            api::get("/data", Some(&token)).await.ok()
        } else {
            None
        }
    }
);

// Check loading state
let loading = move || data_resource.loading().get();

// Access data
let data = move || data_resource.get().flatten();
```

Use `spawn_local` only for one-off async operations (form submissions, button clicks):
```rust
let on_submit = move |ev: web_sys::SubmitEvent| {
    ev.prevent_default();
    spawn_local(async move {
        match api::post("/endpoint", &body, token).await {
            Ok(response) => { /* success */ }
            Err(e) => set_error.set(Some(e)),
        }
    });
};
```

### Navigation
```rust
let navigate = use_navigate();
navigate("/dashboard", Default::default());
```

## Error Handling Pattern

API errors use a structured `ApiResult<T>` type to distinguish auth errors from other failures:

```rust
// In services/api.rs
pub enum ApiError {
    /// 401 Unauthorized - token invalid/expired, requires re-login
    Unauthorized,
    /// 403 Forbidden - authenticated but lacks permission
    Forbidden,
    /// Network failure (offline, DNS, timeout)
    Network(String),
    /// JSON serialization/deserialization failed
    Serialization(String),
    /// Backend returned error response with message
    Api { status: u16, message: String },
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    pub fn is_auth_error(&self) -> bool {
        matches!(self, ApiError::Unauthorized | ApiError::Forbidden)
    }
    
    pub fn to_display_string(&self) -> String {
        match self {
            ApiError::Unauthorized => "Session expired. Please log in again.".to_string(),
            ApiError::Forbidden => "You don't have permission to access this resource.".to_string(),
            ApiError::Network(e) => format!("Network error: {}", e),
            ApiError::Serialization(e) => format!("Data error: {}", e),
            ApiError::Api { message, .. } => message.clone(),
        }
    }
}
```

### Authentication Error Handling in API Layer

The `api.rs` module must detect 401/403 responses and handle them specially:

```rust
// In handle_response() or equivalent
async fn handle_response<T: DeserializeOwned>(response: Response) -> ApiResult<T> {
    let status = response.status();
    
    match status {
        401 => {
            // Clear stored auth state immediately
            clear_auth_state();
            Err(ApiError::Unauthorized)
        }
        403 => Err(ApiError::Forbidden),
        200..=299 => {
            // Success - parse response
            response.json().await
                .map_err(|e| ApiError::Serialization(e.to_string()))
        }
        _ => {
            // Other errors - try to parse ApiError from body
            let message = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(ApiError::Api { status, message })
        }
    }
}

fn clear_auth_state() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("auth_token");
        }
    }
}
```

### Centralized Reauth Handling in Components

Components should check for auth errors and trigger re-authentication:

```rust
// Pattern for handling API responses with auth error detection
spawn_local(async move {
    match api::get::<Data>("/protected", Some(&token)).await {
        Ok(data) => { /* handle success */ }
        Err(ApiError::Unauthorized) | Err(ApiError::Forbidden) => {
            // Clear auth context and redirect to login
            auth.set_token.set(None);
            auth.set_user.set(None);
            navigate("/login", Default::default());
        }
        Err(e) => {
            set_error.set(Some(e.to_display_string()));
        }
    }
});
```

### Centralized Auth Error Handler (Recommended)

Create a helper function in `services/auth.rs` for consistent handling:

```rust
/// Handle API result with automatic auth error recovery
pub fn handle_api_result<T>(
    result: ApiResult<T>,
    auth: &AuthContext,
    navigate: &impl Fn(&str, NavigateOptions),
) -> Result<T, String> {
    match result {
        Ok(value) => Ok(value),
        Err(ApiError::Unauthorized) | Err(ApiError::Forbidden) => {
            // Clear auth state
            auth.set_token.set(None);
            auth.set_user.set(None);
            // Redirect to login
            navigate("/login", Default::default());
            Err("Session expired".to_string())
        }
        Err(e) => Err(e.to_display_string()),
    }
}
```

### Error Display in UI

```rust
// Display errors with Show (unwrap is safe - Show guard ensures Some):
<Show when=move || error.get().is_some()>
    <div class="notification is-danger">
        {move || error.get().unwrap()}
    </div>
</Show>
```

### Error Categories Summary

| Error Type | HTTP Status | Action | User Message |
|------------|-------------|--------|--------------|
| `Unauthorized` | 401 | Clear tokens, redirect to login | "Session expired. Please log in again." |
| `Forbidden` | 403 | Redirect to login or show error | "You don't have permission..." |
| `Network` | N/A | Retry option, check connection | "Network error: {details}" |
| `Serialization` | N/A | Log for debugging | "Data error: {details}" |
| `Api` | 4xx/5xx | Display message | Backend error message |

## File Organization

| Directory | Purpose |
|-----------|---------|
| `src/components/` | Reusable UI (Header, Footer) |
| `src/pages/` | Route-level page components |
| `src/services/` | API client, auth logic |
| `src/models/` | Data structures with Serde derives |

## Styling - Bulma CSS

This project uses **Bulma CSS** (v1.0.2) with a custom dark theme. Bulma is a CSS-only framework; interactive behavior (e.g., navbar burger toggle) is handled natively in Leptos components using signals.

### CSS Stack
- **Bulma CSS**: Loaded from CDN in `index.html`
- **styles.css**: Dark theme overrides using CSS variables
- **No BulmaJS**: Interactive components use Leptos signals + `class:is-active` bindings

### Interactive Component Pattern

Bulma requires `is-active` class toggling for interactivity. Handle in Leptos:

```rust
// Navbar burger toggle example
let (menu_open, set_menu_open) = create_signal(false);

view! {
    <a 
        class="navbar-burger"
        class:is-active=menu_open
        on:click=move |_| set_menu_open.update(|v| *v = !*v)
    >...</a>
    
    <div class="navbar-menu" class:is-active=menu_open>...</div>
}
```

### Key Bulma Classes

| Purpose | Classes |
|---------|---------|
| Layout | `section`, `container`, `columns`, `column` |
| Navbar | `navbar`, `navbar-brand`, `navbar-menu`, `navbar-item`, `navbar-burger` |
| Forms | `field`, `control`, `label`, `input`, `button` |
| Cards | `box`, `card`, `card-header`, `card-content` |
| Buttons | `button is-primary`, `button is-dark`, `is-loading`, `is-fullwidth` |
| Notifications | `notification is-danger`, `notification is-success` |
| Text | `title is-1..6`, `subtitle`, `content`, `has-text-centered` |
| Spacing | `mt-1..6`, `mb-1..6`, `mx-auto` etc. |

### Component Patterns

```rust
// Form field
<div class="field">
    <label class="label">"Email"</label>
    <div class="control">
        <input class="input" type="email" />
    </div>
</div>

// Loading button
<button
    class="button is-primary is-fullwidth"
    class:is-loading=loading
    disabled=loading
>
    "Submit"
</button>

// Error notification
<Show when=move || error.get().is_some()>
    <div class="notification is-danger">
        {move || error.get().unwrap()}
    </div>
</Show>

// Responsive columns
<div class="columns">
    <div class="column is-one-third">...</div>
    <div class="column is-two-thirds">...</div>
</div>
```

### Dark Theme Variables (styles.css)
```css
:root {
    --bulma-scheme-main: #0f172a;      /* Primary background */
    --bulma-scheme-main-bis: #1e293b;  /* Secondary (cards, navbar) */
    --bulma-scheme-main-ter: #334155;  /* Tertiary (inputs, hover) */
    --bulma-text: #f1f5f9;             /* Primary text */
    --bulma-text-light: #94a3b8;       /* Muted text */
    --bulma-border: #475569;           /* Borders */
    --bulma-primary: #3b82f6;          /* Primary accent */
}
```

## Development

```bash
trunk serve          # Dev server with hot reload
trunk build --release  # Production build
```

## Critical Details

- **API Base URL**: Compile-time config via `API_BASE_URL` env var, defaults to `http://localhost:3000/api`
  ```bash
  # Production build with custom API URL
  API_BASE_URL=https://api.example.com/api trunk build --release
  ```
- **WASM Target**: Requires `wasm32-unknown-unknown` (`rustup target add wasm32-unknown-unknown`)
- **Leptos CSR**: Uses `features = ["csr"]` - no SSR

## Token Storage & Security

### Current Implementation
Tokens are stored in `localStorage` under key `auth_token`. This is simple but has security implications.

### XSS Risk & Mitigations

**Problem**: `localStorage` is accessible to any JavaScript on the page. XSS attacks can steal tokens.

**Production Recommendations** (in order of preference):

1. **HttpOnly Secure Cookies** (Preferred):
   - Backend sets `Set-Cookie: token=...; HttpOnly; Secure; SameSite=Strict; Path=/api`
   - Token is never accessible to JavaScript
   - Requires backend changes and CORS configuration (see below)

2. **If localStorage must be used**, implement these mitigations:
   - **Strict CSP**: `Content-Security-Policy: default-src 'self'; script-src 'self'`
   - **Input sanitization**: Never render untrusted HTML with `inner_html!`
   - **Short-lived access tokens**: 15-minute expiry, use refresh tokens
   - **Token binding**: Include device/session fingerprint in token claims

### Token Lifecycle

```rust
// Token structure (decode JWT claims client-side for expiry)
pub struct TokenClaims {
    pub exp: i64,      // Expiry timestamp (Unix)
    pub sub: String,   // User ID
    pub iat: i64,      // Issued at
}

// Check if token is expired (with 30-second buffer)
pub fn is_token_expired(token: &str) -> bool {
    // Decode JWT (base64 only, no signature verification on client)
    // Compare exp claim to current time - 30 seconds
    let now = chrono::Utc::now().timestamp();
    claims.exp <= now - 30
}
```

### Refresh Token Flow

```
┌─────────────┐      Access Token (short-lived, 15min)      ┌─────────────┐
│   Client    │ ─────────────────────────────────────────► │   Backend   │
│             │                                             │             │
│             │ ◄───────────── 401 Unauthorized ─────────── │             │
│             │                                             │             │
│             │      Refresh Token (long-lived, 7d)         │             │
│             │ ────────────► POST /auth/refresh ─────────► │             │
│             │                                             │             │
│             │ ◄─────── New Access + Refresh Tokens ────── │             │
└─────────────┘                                             └─────────────┘
```

**Implementation**:
```rust
// In services/auth.rs
pub async fn refresh_token() -> Result<AuthResponse, ApiError> {
    let refresh = get_stored_refresh_token()
        .ok_or(ApiError::Unauthorized)?;
    
    api::post::<AuthResponse, _>("/auth/refresh", &RefreshRequest { token: refresh }, None).await
}

// Silent refresh: call before token expires
pub fn schedule_token_refresh(expires_in_secs: i64) {
    let refresh_at = (expires_in_secs - 60) * 1000; // 1 min before expiry
    set_timeout(move || {
        spawn_local(async move {
            if let Ok(response) = refresh_token().await {
                // Update stored tokens
                store_tokens(&response.access_token, &response.refresh_token);
            }
        });
    }, refresh_at as u32);
}
```

### Token Rotation & Revocation

- **Rotation**: Issue new refresh token on each refresh; invalidate old one server-side
- **Revocation**: On logout, call `POST /auth/logout` to invalidate refresh token server-side
- **Token family tracking**: Server tracks refresh token lineage; revoke entire family on reuse detection

### Initial Auth State (App Startup)

```rust
// In app.rs or main.rs
fn initialize_auth() -> Option<String> {
    let token = get_stored_token()?;
    
    // Client-side expiry check (quick, no network)
    if is_token_expired(&token) {
        // Try refresh
        // Note: This is sync context, so schedule async refresh
        clear_stored_tokens();
        return None;
    }
    
    Some(token)
}

// For full validation, verify with backend on first protected request
// or call /auth/verify endpoint at startup
```

### CORS & Cookie Configuration (CSR + Backend)

**Backend CORS Headers** (required for cross-origin API calls):
```
Access-Control-Allow-Origin: https://your-frontend.com  # NOT * with credentials
Access-Control-Allow-Credentials: true
Access-Control-Allow-Headers: Content-Type, Authorization
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Expose-Headers: Set-Cookie
```

**Cookie Settings** (if using httpOnly cookies):
```
Set-Cookie: access_token=...; HttpOnly; Secure; SameSite=Strict; Path=/api; Max-Age=900
Set-Cookie: refresh_token=...; HttpOnly; Secure; SameSite=Strict; Path=/api/auth/refresh; Max-Age=604800
```

- `HttpOnly`: Prevents JavaScript access
- `Secure`: HTTPS only
- `SameSite=Strict`: Prevents CSRF (use `Lax` if cross-site GET needed)
- `Path=/api/auth/refresh`: Refresh token only sent to refresh endpoint

**Client-Side Credentialed Requests**:
```rust
// In api.rs - enable credentials for cookie-based auth
Request::get(&url)
    .credentials(web_sys::RequestCredentials::Include)  // Send cookies
    .send()
```

### Protecting Refresh Endpoints

1. **Rate limiting**: Max 10 refresh attempts per minute per user
2. **Refresh token binding**: Tie to device fingerprint or IP range
3. **Absolute expiry**: Refresh tokens expire after 7-30 days regardless of activity
4. **Secure storage on server**: Hash refresh tokens before storing (like passwords)

## Model Conventions

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyType { /* fields */ }
```

Use `Option<T>` for optional fields in update requests.

## Testing

<!-- TODO: Testing conventions not yet established -->
