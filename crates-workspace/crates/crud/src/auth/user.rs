use base64::encode;
use constant::*;
use error::{RtcError, RtcResult};
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use model::user::{ActiveModel, Column, Entity, Model as User};
use schema::auth::user::{AccessToken, AuthUser};
use sea_orm::ColumnTrait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter};
use sha2::{Digest, Sha384, Sha512};
use std::collections::BTreeMap;
use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait HasId: ActiveModelTrait {
    fn set_id(&mut self, v: i32);
}

#[allow(async_fn_in_trait)]
pub trait RegisterAndChangePassword {
    async fn create(self, db: &DatabaseConnection) -> RtcResult<User>;

    async fn change_password(self, db: &DatabaseConnection) -> RtcResult<()>;

    async fn get_user_by_email(&self, db: &DatabaseConnection, email: &str) -> RtcResult<User>;
}

#[allow(async_fn_in_trait)]
// #[async_trait]
pub trait Authentication {
    async fn validate_jwt(&self, db: &DatabaseConnection, token: &str) -> RtcResult<()>;

    async fn login(&self, db: &DatabaseConnection) -> RtcResult<AccessToken>;

    async fn get_user_by_name<'a>(
        &'a self,
        db: &'a DatabaseConnection,
        username: &'a str,
    ) -> RtcResult<User>;
}

impl Authentication for AuthUser {
    async fn validate_jwt(&self, db: &DatabaseConnection, token: &str) -> RtcResult<()> {
        let id = validate_jwt(token)?;

        Entity::find_by_id(id as i32).one(db).await?;
        Ok(())
    }
    async fn login(&self, db: &DatabaseConnection) -> RtcResult<AccessToken> {
        let username = self
            .username
            .as_ref()
            .ok_or(RtcError::MissingParamsError("Empty username"))?;
        let password = self
            .username
            .as_ref()
            .ok_or(RtcError::MissingParamsError("Empty password"))?;
        let user = self.get_user_by_name(db, username).await?;

        if sha256_hash(password).ne(&user.password) {
            Err(RtcError::InvalidError("Incorrect username or password."))
        } else {
            let token = jwt_encrypt(user.id as usize)?;
            Ok(token)
        }
    }

    async fn get_user_by_name<'a>(
        &'a self,
        db: &'a DatabaseConnection,
        username: &'a str,
    ) -> RtcResult<User> {
        let user = Entity::find()
            .filter(Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or(RtcError::NoFoundError("Incorrect Password."))?;
        Ok(user)
    }
}

impl RegisterAndChangePassword for User {
    async fn create(self, db: &DatabaseConnection) -> RtcResult<User> {
        let user = Entity::find()
            .filter(Column::Username.eq(&self.username))
            .one(db)
            .await?;
        if user.is_some() {
            return Err(RtcError::AlreadyExistsError("This user is already exists."));
        }
        let active_model: ActiveModel = self.clone().into();
        active_model.insert(db).await?;
        Ok(self)
    }

    async fn change_password(self, db: &DatabaseConnection) -> RtcResult<()> {
        let active_model: ActiveModel = self.into();
        active_model.save(db).await?;
        Ok(())
    }

    async fn get_user_by_email(&self, db: &DatabaseConnection, email: &str) -> RtcResult<User> {
        let user = Entity::find()
            .filter(Column::Email.eq(email))
            .one(db)
            .await?
            .ok_or(RtcError::NoFoundError("Incorrect email."))?;
        Ok(user)
    }
}

pub fn validate_jwt(token: &str) -> RtcResult<usize> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(SECRET_KEY.as_bytes())?;
    let token: Token<Header, BTreeMap<String, String>, _> = token.verify_with_key(&key)?;
    let claims = token.claims();
    let time: u64 = claims
        .get("sub")
        .ok_or(RtcError::NoFoundError("Get jwt inner sub failed."))?
        .parse()?;

    let current_time = SystemTime::from(UNIX_EPOCH).elapsed()?.as_secs();
    if time.lt(&current_time) {
        return Err(RtcError::NoFoundError("Expired signature."));
    }
    let id: usize = claims
        .get("iat")
        .ok_or(RtcError::NoFoundError("Get jwt inner sub failed."))?
        .parse()?;
    Ok(id)
}


pub fn sha256_hash(str: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(str);
    hasher.update(SECRET_KEY);
    let encrypted_password = hasher.finalize();
    encode(encrypted_password)
}

pub fn jwt_encrypt(id: usize) -> RtcResult<AccessToken> {
    let key: Hmac<Sha384> = Hmac::new_from_slice(SECRET_KEY.as_bytes())?;
    let mut access_claims = BTreeMap::new();
    let mut refresh_claims = BTreeMap::new();
    let time = SystemTime::from(UNIX_EPOCH).elapsed()?;
    let access_sub_time = time.as_secs().add(ACCESS_TOKEN_TIME_DELTA);
    let refresh_sub_time = time.as_secs().add(REFRESH_TOKEN_TIME_DELTA);
    access_claims.insert("sub", access_sub_time.to_string());
    access_claims.insert("iat", id.to_string());

    refresh_claims.insert("sub", refresh_sub_time.to_string());
    refresh_claims.insert("iat", id.to_string());
    Ok(AccessToken::new(
        format!("Bearer {}", access_claims.sign_with_key(&key)?),
        format!("Bearer {}", refresh_claims.sign_with_key(&key)?),
    ))
}
