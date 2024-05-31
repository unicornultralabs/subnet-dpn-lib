// use crate::{accounting::AccountingService, connection::ConnectionService};
use actix_web::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorWrapper {
    #[serde(skip_serializing)]
    status_code: u16,
    err_code: Option<usize>,
    err_msg: String,
}

impl ErrorWrapper {
    pub fn builder(status_code: StatusCode, err_msg: &str) -> Self {
        Self {
            status_code: status_code.as_u16(),
            err_code: None,
            err_msg: err_msg.to_string(),
        }
    }

    pub fn err_code(&mut self, err_code: usize) -> &mut Self {
        self.err_code = Some(err_code);
        self
    }

    pub fn err_msg(&self) -> String {
        self.err_msg.clone()
    }

    pub fn build(&mut self) -> HttpResponse {
        match StatusCode::from_u16(self.status_code) {
            Ok(code) => match code {
                StatusCode::OK => return HttpResponse::Ok().json(self),
                StatusCode::CREATED => return HttpResponse::Created().json(self),
                StatusCode::BAD_REQUEST => return HttpResponse::BadRequest().json(self),
                StatusCode::UNAUTHORIZED => return HttpResponse::Unauthorized().json(self),
                StatusCode::NOT_FOUND => return HttpResponse::NotFound().json(self),
                StatusCode::INTERNAL_SERVER_ERROR => {
                    return HttpResponse::InternalServerError().json(self)
                }
                _ => {
                    return HttpResponse::InternalServerError().json(Self {
                        status_code: self.status_code,
                        err_code: None,
                        err_msg: "status code not implemented".to_owned(),
                    })
                }
            },
            Err(_) => {
                return HttpResponse::InternalServerError().json(Self {
                    status_code: self.status_code,
                    err_code: None,
                    err_msg: "invalid status code".to_owned(),
                })
            }
        }
    }
}
