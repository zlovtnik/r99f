use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 3, max = 20, message = "Username must be 3-20 characters"))]
    #[validate(custom(function = "validate_username_chars"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    pub password: String,
}

impl CreateUserRequest {
    /// Validates the request and returns a user-friendly error message if invalid.
    pub fn validate_fields(&self) -> Result<(), String> {
        self.validate().map_err(|e| {
            e.field_errors()
                .values()
                .flat_map(|errors| errors.iter())
                .filter_map(|e| e.message.as_ref())
                .next()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "Validation failed".to_string())
        })
    }
}

impl std::fmt::Debug for CreateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserRequest")
            .field("email", &self.email)
            .field("username", &self.username)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(min = 3, max = 20, message = "Username must be 3-20 characters"))]
    #[validate(custom(function = "validate_username_chars"))]
    pub username: Option<String>,
}

impl UpdateUserRequest {
    /// Validates the request and returns a user-friendly error message if invalid.
    pub fn validate_fields(&self) -> Result<(), String> {
        self.validate().map_err(|e| {
            // Extract the first validation error message
            e.field_errors()
                .values()
                .flat_map(|errors| errors.iter())
                .filter_map(|e| e.message.as_ref())
                .next()
                .map(|m| m.to_string())
                .unwrap_or_else(|| "Validation failed".to_string())
        })
    }
}

/// Custom validator for username characters: alphanumeric, underscore, hyphen only
fn validate_username_chars(username: &str) -> Result<(), validator::ValidationError> {
    if username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
        Ok(())
    } else {
        let mut err = validator::ValidationError::new("invalid_chars");
        err.message = Some("Username can only contain letters, numbers, underscore, and hyphen".into());
        Err(err)
    }
}

/// Custom validator for password strength: requires uppercase, lowercase, and digit
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if has_uppercase && has_lowercase && has_digit {
        Ok(())
    } else {
        let mut err = validator::ValidationError::new("weak_password");
        err.message = Some(
            "Password must contain at least one uppercase letter, one lowercase letter, and one digit".into()
        );
        Err(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl std::fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginRequest")
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub error: String,
}
