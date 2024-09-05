/*******************************************************************************
 * Copyright (c) 2024 Cénotélie Opérations SAS (cenotelie.fr)
 ******************************************************************************/

//! Objects related to authentication

use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};

use crate::utils::apierror::{error_invalid_request, specialize, ApiError};

/// Represents a data about a successful authentication
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authentication {
    /// The principal (email of the user)
    pub principal: AuthenticationPrincipal,
    /// Whether a crate can be uploaded
    #[serde(rename = "canWrite")]
    pub can_write: bool,
    /// Whether administration can be done
    #[serde(rename = "canAdmin")]
    pub can_admin: bool,
}

impl Authentication {
    /// Creates a new authentication for a self connection
    #[must_use]
    pub fn new_self() -> Self {
        Self {
            principal: AuthenticationPrincipal::SelfAuth,
            can_write: false,
            can_admin: false,
        }
    }

    // Creates a new authentication for a service using a global token
    #[must_use]
    pub fn new_service(token_id: String) -> Self {
        Self {
            principal: AuthenticationPrincipal::Service { token_id },
            can_write: false,
            can_admin: false,
        }
    }

    // Creates a new user authentication that can do everything
    #[must_use]
    pub fn new_user(uid: i64, email: String) -> Self {
        Self {
            principal: AuthenticationPrincipal::User { uid, email },
            can_write: true,
            can_admin: true,
        }
    }

    /// Gets the uid of the associated user
    pub fn uid(&self) -> Result<i64, ApiError> {
        if let AuthenticationPrincipal::User { uid, email: _ } = &self.principal {
            Ok(*uid)
        } else {
            Err(specialize(
                error_invalid_request(),
                String::from("Expected a user to be authenticated"),
            ))
        }
    }

    /// Gets the email of the associated user
    pub fn email(&self) -> Result<&str, ApiError> {
        if let AuthenticationPrincipal::User { uid: _, email } = &self.principal {
            Ok(email)
        } else {
            Err(specialize(
                error_invalid_request(),
                String::from("Expected a user to be authenticated"),
            ))
        }
    }
}

/// The principal associated to an authentication
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthenticationPrincipal {
    /// A user is authenticated
    User { uid: i64, email: String },
    /// A service through a global token
    Service { token_id: String },
    /// The registry itself when connecting to itself
    SelfAuth,
}

/// A token for a registry user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryUserToken {
    /// The unique identifier
    pub id: i64,
    /// The token name
    pub name: String,
    /// The last time the token was used
    #[serde(rename = "lastUsed")]
    pub last_used: NaiveDateTime,
    /// Whether a crate can be uploaded using this token
    #[serde(rename = "canWrite")]
    pub can_write: bool,
    /// Whether administration can be done using this token through the API
    #[serde(rename = "canAdmin")]
    pub can_admin: bool,
}

/// A token for a registry user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistryUserTokenWithSecret {
    /// The unique identifier
    pub id: i64,
    /// The token name
    pub name: String,
    /// The value for the token
    pub secret: String,
    /// The last time the token was used
    #[serde(rename = "lastUsed")]
    pub last_used: NaiveDateTime,
    /// Whether a crate can be uploaded using this token
    #[serde(rename = "canWrite")]
    pub can_write: bool,
    /// Whether administration can be done using this token through the API
    #[serde(rename = "canAdmin")]
    pub can_admin: bool,
}

/// An OAuth access token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthToken {
    /// The access token
    pub access_token: String,
    /// The type of token
    pub token_type: String,
    /// The expiration time
    pub expires_in: Option<i64>,
    /// The refresh token
    pub refresh_token: Option<String>,
    /// The grant scope
    pub scope: Option<String>,
}

/// Finds a field in a JSON blob
pub fn find_field_in_blob<'v>(blob: &'v serde_json::Value, path: &str) -> Option<&'v str> {
    let mut last = blob;
    for item in path.split('.') {
        last = last.as_object()?.get(item)?;
    }
    last.as_str()
}

/// Event when a token was used
#[derive(Debug, Clone)]
pub struct TokenUsageEvent {
    /// Whether this was a user token (or a global one)
    pub is_user_token: bool,
    /// The unique identifier for the token
    pub token_id: i64,
    /// The timestamp when the token was used
    pub timestamp: NaiveDateTime,
}
