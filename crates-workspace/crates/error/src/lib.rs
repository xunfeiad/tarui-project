pub mod crud_error;

use actix_web::{error, http::header::ContentType};
use actix_web::{http::StatusCode, HttpResponse, Responder, ResponseError};
use std::time::SystemTimeError;
use std::{fmt::Debug, num::ParseIntError};
use thiserror::Error;
use tokio::sync::oneshot::error::RecvError;

pub type RtcResult<T, E = RtcError> = Result<T, E>;

#[derive(Error, Debug)]
pub enum RtcError {
    #[error("{0}")]
    JwtInvalidLength(#[from] sha2::digest::InvalidLength),
    #[error("{0}")]
    JwtVerifyFailed(#[from] jwt::Error),
    #[error(transparent)]
    AnyHowError(#[from] anyhow::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    SystemTimeError(#[from] SystemTimeError),
    #[error(transparent)]
    ActixWeb(#[from] actix_web::Error),
    #[error(transparent)]
    RecvError(#[from] RecvError),
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),

    #[error("{0}")]
    NoFoundError(&'static str),
    #[error("{0}")]
    MissingParamsError(&'static str),
    #[error("{0}")]
    AlreadyExistsError(&'static str),
    #[error("{0}")]
    InvalidError(&'static str),

    #[error("{0}")]
    InternalServerError(&'static str),
}

// use tokio::sync::{oneshot::error::RecvError};

// impl From<RecvError> for RtcError {
//     fn from(err: RecvError) -> RtcError {
//         RtcError::RecvError(err)
//     }
// }

// impl<T> From<SendError<T>> for RtcError
// {
//     fn from(value: SendError<T>) -> Self {
//         Self::SendError(value)
//     }
// }

impl error::ResponseError for RtcError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            RtcError::AnyHowError(_)
            | RtcError::JwtInvalidLength(_)
            | RtcError::JwtVerifyFailed(_)
            | RtcError::NoFoundError(_)
            | RtcError::ParseIntError(_)
            | RtcError::MissingParamsError(_) => StatusCode::BAD_REQUEST,
            RtcError::ActixWeb(_)
            | RtcError::DbErr(_)
            | RtcError::SystemTimeError(_)
            | RtcError::RecvError(_)
            | RtcError::InvalidError(_)
            | RtcError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RtcError::AlreadyExistsError(_) => StatusCode::CONFLICT,
        }
    }
}

// impl Responder for RtcError {
//     type Body = ();
//     fn with_status(self, status: StatusCode) -> actix_web::CustomizeResponder<Self>
//         where
//             Self: Sized, {
//                 match self {
//                     RtcError::AnyHowError(_)  | RtcError::JwtInvalidLength(_) | RtcError::JwtVerifyFailed(_) | RtcError::NoFoundError(_) | RtcError::ParseIntError(_) | RtcError::MissingParamsError(_)=> StatusCode::BAD_REQUEST,
//                     RtcError::ActixWeb(_) | RtcError::DbErr(_) | RtcError::SystemTimeError(_)  | RtcError::RecvError(_) | RtcError::InvalidError(_) | RtcError::InternalServerError(_)=> StatusCode::INTERNAL_SERVER_ERROR,
//                     RtcError::AlreadyExistsError(_) => StatusCode::CONFLICT
//                 }
//     }
//     // fn with_header(self, header: impl actix_web::http::header::TryIntoHeaderPair) -> actix_web::CustomizeResponder<Self>
//     //     where
//     //         Self: Sized, {
//     //     todo!()
//     // }
//     fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
//         todo!()
//     }
//     fn customize(self) -> actix_web::CustomizeResponder<Self>
//         where
//             Self: Sized, {
//         todo!()
//     }
// }
