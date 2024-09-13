use actix_web::{
    get, post, web::{Data, Json, Query}, Responder
};
use anyhow::Context;
use crud::auth::{email::SendEmail, user::{Authentication, RegisterAndChangePassword}};
use error::{RtcError, RtcResult};
use model::user::Model as User;
use schema::{
    auth::user::{AuthUser, ChangePasswordSchema},
    state::AppState,
    CusResponse
};
use deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime};


#[post("/login")]
async fn login(auth_user: Json<AuthUser>, state: Data<AppState>) -> RtcResult<impl Responder> {
    let access_token = auth_user.0.login(&state.db).await?;
    Ok(Json(CusResponse::new(200, "ok".into(), access_token)))
}

#[post("/register")]
async fn register(register_user: Json<User>, state: Data<AppState>) -> RtcResult<impl Responder> {
    let user = register_user.0.create(&state.db).await?;
    Ok(Json(CusResponse::new(200, "ok".into(), user)))
}

#[post("/change_password")]
async fn change_password(
    change_password_schema: Query<ChangePasswordSchema>,
    state: Data<AppState>,
) -> RtcResult<impl Responder> {
    let user = User::default();
    let mut user = user
        .get_user_by_email(&state.db, &change_password_schema.0.email)
        .await?;
    user.email = change_password_schema.email.clone();
    user.password = change_password_schema.password.clone();
    let user = user.create(&state.db).await?;
    Ok(Json(CusResponse::new(200, "ok".into(), user)))
}

#[get("/send_email")]
async fn send_email(user: Query<AuthUser>, state: Data<AppState>) -> RtcResult<impl Responder> {
    let random_str = utils::get_random_str();
    let email = user
        .email
        .as_ref()
        .ok_or(RtcError::NoFoundError("Email can'b be empty."))?;
    
    let mut redis = state.redis.lock().await;
    cmd("SET").arg(&[email.as_str(), random_str.as_str()]).query_async::<()>(&mut *redis).await.context("Put code into redis failed.")?;
    let mut context = tera::Context::new();
    context.insert("code", &random_str);
    let body = state.tera.render("auth/email.html.tera", &context).context("Render auth/email.html.tera failed.")?;
    user.send_email("Varift Email", body)?;
    Ok(CusResponse::<u32>::default())
}


#[post("/varify_code")]
async fn varify_code(user: Json<AuthUser>, state: Data<AppState>) -> RtcResult<impl Responder>{
    let email = user.email.as_ref().context("Empty email.")?;
    let code = user.code.as_ref().context("invalid code")?;
    let mut redis = state.redis.lock().await;
    let random_str = utils::get_random_str();
    let real_code: String = cmd("GET").arg(&[email.as_str()]).query_async(&mut *redis).await.context("Get code from redis failed.")?;
    cmd("SET").arg(&[email.as_str(), &random_str]).query_async::<()>(&mut *redis).await.context("Put code into redis failed.")?;

    if code == &real_code{
        Ok(CusResponse::<u32>::default())
    }else{
        Err(RtcError::InternalServerError("Invalid code"))
    }
}
