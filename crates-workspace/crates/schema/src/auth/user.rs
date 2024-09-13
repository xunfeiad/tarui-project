use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChangePasswordSchema {
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessToken {
    access_token: String,
    refresh_token: String,
}

impl AccessToken {
    pub fn new(access_token: String, refresh_token: String) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}
