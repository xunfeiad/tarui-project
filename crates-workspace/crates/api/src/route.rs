use crate::auth::user::{change_password, login, register, send_email, varify_code};
use actix_web::{web, HttpResponse};
/// use actix_web::{web, App, HttpResponse};
///
/// // this function could be located in different module
/// fn config(cfg: &mut web::ServiceConfig) {
///     cfg.service(web::resource("/test")
///         .route(web::get().to(|| HttpResponse::Ok()))
///         .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
///     );
/// }
///
/// App::new()
///     .configure(config)  // <- register resources
///     .route("/index.html", web::get().to(|| HttpResponse::Ok()));

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::scope("auth")
            .service(login)
            .service(register)
            .service(change_password)
            .service(send_email)
            .service(varify_code)
    );
    // cfg.service(web::resource("/ws").route(web::get().to(echo)));
}
