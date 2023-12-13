//! Module for the application

use cenotelie_lib_apierror::{
    error_conflict, error_forbidden, error_invalid_request, error_not_found, error_unauthorized, specialize, ApiError,
};
use chrono::Local;
use data_encoding::HEXLOWER;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use ring::digest::{Context, SHA256};

use crate::objects::{Configuration, OAuthToken, RegistryUser, RegistryUserToken, RegistryUserTokenWithSecret};

use super::namegen::generate_name;
use super::Application;

/// Generates a token
fn generate_token(length: usize) -> String {
    let rng = thread_rng();
    String::from_utf8(rng.sample_iter(&Alphanumeric).take(length).collect()).unwrap()
}

/// Computes the SHA256 digest of bytes
fn sha256(buffer: &[u8]) -> String {
    let mut context = Context::new(&SHA256);
    context.update(buffer);
    let digest = context.finish();
    HEXLOWER.encode(digest.as_ref())
}

/// Hashes a token secret
fn hash_token(input: &str) -> String {
    sha256(input.as_bytes())
}

/// Checks a token hash
pub fn check_hash(token: &str, hashed: &str) -> Result<(), ApiError> {
    let matches = hashed == sha256(token.as_bytes());
    if matches {
        Ok(())
    } else {
        Err(error_unauthorized())
    }
}

impl<'c> Application<'c> {
    /// Gets the data about the current user
    pub async fn get_current_user(&self, principal: &str) -> Result<RegistryUser, ApiError> {
        let uid = self.check_is_user(principal).await?;
        self.get_user_profile(uid).await
    }

    /// Retrieves a user profile
    async fn get_user_profile(&self, uid: i64) -> Result<RegistryUser, ApiError> {
        let maybe_row = sqlx::query_as!(
            RegistryUser,
            "SELECT id, isActive AS is_active, email, login, name, roles FROM RegistryUser WHERE id = $1",
            uid
        )
        .fetch_optional(&mut *self.transaction.borrow().await)
        .await?;
        maybe_row.ok_or_else(error_not_found)
    }

    /// Attemps to login using an OAuth code
    pub async fn login_with_oauth_code(&self, configuration: &Configuration, code: &str) -> Result<RegistryUser, ApiError> {
        let client = reqwest::Client::new();
        // retrieve the token
        let response = client
            .post(&configuration.oauth_token_uri)
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", &configuration.oauth_callback_uri),
                ("client_id", &configuration.oauth_client_id),
                ("client_secret", &configuration.oauth_client_secret),
            ])
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(specialize(error_unauthorized(), String::from("authentication failed")));
        }
        let body = response.bytes().await?;
        let token = serde_json::from_slice::<OAuthToken>(&body)?;

        // retrieve the user profile
        let response = client
            .get(&configuration.oauth_userinfo_uri)
            .header("authorization", format!("Bearer {}", token.access_token))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(specialize(error_unauthorized(), String::from("authentication failed")));
        }
        let body = response.bytes().await?;
        let user_info = serde_json::from_slice::<serde_json::Value>(&body)?;
        let email = user_info
            .as_object()
            .ok_or_else(error_unauthorized)?
            .get("email")
            .ok_or_else(error_unauthorized)?
            .as_str()
            .ok_or_else(error_unauthorized)?;

        // resolve the user
        let row = sqlx::query!(
            "SELECT id, isActive AS is_active, login, name, roles FROM RegistryUser WHERE email = $1 LIMIT 1",
            email
        )
        .fetch_optional(&mut *self.transaction.borrow().await)
        .await?;
        if let Some(row) = row {
            if !row.is_active {
                return Err(specialize(error_unauthorized(), String::from("inactive user")));
            }
            // already exists
            return Ok(RegistryUser {
                id: row.id,
                is_active: true,
                email: email.to_string(),
                login: row.login,
                name: row.name,
                roles: row.roles,
            });
        }
        // create the user
        let count = sqlx::query!("SELECT COUNT(id) AS count FROM RegistryUser")
            .fetch_one(&mut *self.transaction.borrow().await)
            .await?
            .count;
        let mut login = email[..email.find('@').unwrap()].to_string();
        while sqlx::query!("SELECT COUNT(id) AS count FROM RegistryUser WHERE login = $1", login)
            .fetch_one(&mut *self.transaction.borrow().await)
            .await?
            .count
            != 0
        {
            login = generate_name();
        }
        let roles = if count == 0 { "admin" } else { "" };
        let id = sqlx::query!(
            "INSERT INTO RegistryUser (isActive, login, name, roles) VALUES (TRUE, $1, $1, $2) RETURNING id",
            login,
            roles
        )
        .fetch_one(&mut *self.transaction.borrow().await)
        .await?
        .id;
        Ok(RegistryUser {
            id,
            is_active: true,
            email: email.to_string(),
            name: login.to_string(),
            login,
            roles: roles.to_string(),
        })
    }

    /// Gets the known users
    pub async fn get_users(&self, principal: &str) -> Result<Vec<RegistryUser>, ApiError> {
        let uid = self.check_is_user(principal).await?;
        self.check_is_admin(uid).await?;
        let rows = sqlx::query_as!(
            RegistryUser,
            "SELECT id, isActive AS is_active, email, login, name, roles FROM RegistryUser ORDER BY login",
        )
        .fetch_all(&mut *self.transaction.borrow().await)
        .await?;
        Ok(rows)
    }

    /// Updates the information of a user
    pub async fn update_user(&self, principal: &str, target: &RegistryUser) -> Result<RegistryUser, ApiError> {
        let uid = self.check_is_user(principal).await?;
        let is_admin = if target.id == uid {
            self.get_is_admin(uid).await?
        } else {
            self.check_is_admin(uid).await?;
            true
        };
        let row = sqlx::query!("SELECT login, roles FROM RegistryUser WHERE id = $1 LIMIT 1", target.id)
            .fetch_optional(&mut *self.transaction.borrow().await)
            .await?
            .ok_or_else(error_not_found)?;
        let old_roles = row.roles;
        if !is_admin && target.roles != old_roles {
            // not admin and changing roles
            return Err(specialize(error_forbidden(), String::from("only admins can change roles")));
        }
        if is_admin && target.id == uid && target.roles.split(',').all(|role| role.trim() != "admin") {
            // admin and removing admin role from self
            return Err(specialize(error_forbidden(), String::from("admins cannot remove themselves")));
        }
        if target.login.is_empty() {
            return Err(specialize(error_invalid_request(), String::from("login cannot be empty")));
        }
        if row.login != target.login {
            // check that the new login is available
            if sqlx::query!("SELECT COUNT(id) AS count FROM RegistryUser WHERE login = $1", target.login)
                .fetch_one(&mut *self.transaction.borrow().await)
                .await?
                .count
                != 0
            {
                return Err(specialize(
                    error_conflict(),
                    String::from("the specified login is not available"),
                ));
            }
        }
        sqlx::query!(
            "UPDATE RegistryUser SET login = $2, name = $3, roles = $4 WHERE id = $1",
            target.id,
            target.login,
            target.name,
            target.roles
        )
        .execute(&mut *self.transaction.borrow().await)
        .await?;
        Ok(target.clone())
    }

    /// Attempts to deactivate a user
    pub async fn deactivate_user(&self, principal: &str, target: &str) -> Result<(), ApiError> {
        let uid = self.check_is_user(principal).await?;
        let target_uid = self.check_is_user(target).await?;
        self.check_is_admin(uid).await?;
        if uid == target_uid {
            // cannot deactivate self
            return Err(specialize(error_forbidden(), String::from("cannot self deactivate")));
        }
        sqlx::query!("UPDATE RegistryUser SET isActive = FALSE WHERE id = $1", target_uid)
            .execute(&mut *self.transaction.borrow().await)
            .await?;
        Ok(())
    }

    /// Attempts to re-activate a user
    pub async fn reactivate_user(&self, principal: &str, target: &str) -> Result<(), ApiError> {
        let uid = self.check_is_user(principal).await?;
        self.check_is_admin(uid).await?;
        sqlx::query!("UPDATE RegistryUser SET isActive = TRUE WHERE email = $1", target)
            .execute(&mut *self.transaction.borrow().await)
            .await?;
        Ok(())
    }

    /// Gets the tokens for a user
    pub async fn get_tokens(&self, principal: &str) -> Result<Vec<RegistryUserToken>, ApiError> {
        let uid = self.check_is_user(principal).await?;
        let rows = sqlx::query!(
            "SELECT id, name, lastUsed AS last_used FROM RegistryUserToken WHERE user = $1 ORDER BY id",
            uid
        )
        .fetch_all(&mut *self.transaction.borrow().await)
        .await?;
        Ok(rows
            .into_iter()
            .map(|row| RegistryUserToken {
                id: row.id,
                name: row.name,
                last_used: row.last_used,
            })
            .collect())
    }

    /// Creates a token for the current user
    pub async fn create_token(&self, principal: &str, name: &str) -> Result<RegistryUserTokenWithSecret, ApiError> {
        let uid = self.check_is_user(principal).await?;
        let token_secret = generate_token(64);
        let token_hash = hash_token(&token_secret);
        let now = Local::now().naive_local();
        let id = sqlx::query!(
            "INSERT INTO RegistryUserToken (user, name, token, lastUsed) VALUES ($1, $2, $3, $4) RETURNING id",
            uid,
            name,
            token_hash,
            now
        )
        .fetch_one(&mut *self.transaction.borrow().await)
        .await?
        .id;
        Ok(RegistryUserTokenWithSecret {
            id,
            name: name.to_string(),
            secret: token_secret,
            last_used: now,
        })
    }

    /// Revoke a previous token
    pub async fn revoke_token(&self, principal: &str, token_id: i64) -> Result<(), ApiError> {
        let uid = self.check_is_user(principal).await?;
        sqlx::query!("DELETE FROM RegistryUserToken WHERE user = $1 AND id = $2", uid, token_id)
            .execute(&mut *self.transaction.borrow().await)
            .await?;
        Ok(())
    }

    /// Checks an authentication request with a token
    pub async fn check_token(&self, login: &str, token_secret: &str) -> Result<(), ApiError> {
        let rows = sqlx::query!(
            "SELECT RegistryUserToken.id, token
            FROM RegistryUser INNER JOIN RegistryUserToken ON RegistryUser.id = RegistryUserToken.user
            WHERE isActive = TRUE AND login = $1",
            login
        )
        .fetch_all(&mut *self.transaction.borrow().await)
        .await?;
        for row in rows {
            if check_hash(token_secret, &row.token).is_ok() {
                let now = Local::now().naive_local();
                sqlx::query!("UPDATE RegistryUserToken SET lastUsed = $2 WHERE id = $1", row.id, now)
                    .execute(&mut *self.transaction.borrow().await)
                    .await?;
                return Ok(());
            }
        }
        Err(error_unauthorized())
    }
}
