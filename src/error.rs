use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    Sqlx(sqlx::Error),
    StatusCode(StatusCode),
    Other(Response),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<Response> for AppError {
    fn from(value: Response) -> Self {
        Self::Other(value)
    }
}
impl From<StatusCode> for AppError {
    fn from(value: StatusCode) -> Self {
        Self::StatusCode(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(error) => {
                tracing::error!("Db Error: {error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::StatusCode(status_code) => status_code.into_response(),
            Self::Other(response) => response,
        }
    }
}
