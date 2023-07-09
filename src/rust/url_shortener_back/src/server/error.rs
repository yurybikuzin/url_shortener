use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServerError {
    #[display(fmt = "$0")]
    Anyhow(anyhow::Error),
    #[display(fmt = "not found")]
    NotFound,
    #[display(fmt = "failed to recieve response for $0")]
    FailedToRecieveResponse(&'static str),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::FailedToRecieveResponse(_) | Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
