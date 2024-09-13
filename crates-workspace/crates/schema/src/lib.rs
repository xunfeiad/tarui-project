pub mod auth;
pub mod state;
use std::fmt::Debug;
use actix_web::{http::{header::ContentType, StatusCode}, Responder};
use serde::Serialize;
use actix_web::{body::BoxBody, HttpRequest, HttpResponse};


#[derive(Serialize, Debug)]
pub struct CusResponse<T>
where
    T: Serialize + Debug,
{
    code: i32,
    msg: String, 
    data: Option<T>,   
}

impl<T> CusResponse<T> 
where
    T: Serialize + Debug,
{
    pub fn new(code: i32, msg: String,data: T) -> Self{
        Self{
            code,
            msg,
            data: Some(data)
        }
    }
}

impl<T> Default for CusResponse<T> 
where T: Serialize + Debug,
{
    fn default() -> Self {
        Self{
            code: 200,
            msg: "ok".into(),
            data: None 
        }
    }
}

impl<T> Responder for CusResponse<T> 
where T: Serialize + Debug,
{
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .status(StatusCode::OK)
            .body(body)
    }
}