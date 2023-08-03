pub type Result<T> = core::result::Result<T, Error>;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    General(anyhow::Error),
    TemplateNotFound,
    UnsupportedKind,
    ResourceNotFound,
    ResourceAlreadyExists(anyhow::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::General(e) => write!(f, "{}", e),
            Self::ResourceAlreadyExists(e) => write!(f, "{}", e),
            Self::TemplateNotFound => write!(f, "Template not found"),
            Self::UnsupportedKind => write!(f, "UnsupportedKind"),
            _ => write!(f, "InternalServerError"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = serde_json::json!({
            "error": self.to_string()
        })
        .to_string();
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}


impl From<kube::Error> for Error {
    fn from(err: kube::Error) -> Self {
        Error::General(err.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::General(err.into())
    }
}