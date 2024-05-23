use anyhow::Result;
use axum::{response::IntoResponse, routing::get, Router};
use backtrace::Backtrace;
use error_code::ToErrorInfo;
use http::{Response, StatusCode};
use thiserror::Error;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Debug, Error, ToErrorInfo)]
#[error_info(app_type = "StatusCode", prefix = "01")]
pub enum AppError {
    #[error("Invalid param: {0}")]
    #[error_info(code = "IP", app_code = "400")]
    InvalidParam(String),

    #[error("Item {0} not found")]
    #[error_info(code = "NF", app_code = "404")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    #[error_info(
        code = "SE",
        app_code = "500",
        client_msg = "we had a server problem, please try again later!"
    )]
    ServeError(String),

    #[error("Unknown error")]
    #[error_info(code = "UK", app_code = "500")]
    Unknown,
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index_handler));
    let addr = "0.0.0.0:8888";
    info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn index_handler() -> Result<&'static str, AppError> {
    let bt = Backtrace::new();
    Err(AppError::ServeError(format!("{:?}", bt)))
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let info = self.to_error_info();
        let status = info.app_code;

        if status.is_server_error() {
            warn!("{:?}", info);
        } else {
            info!("{:?}", info);
        }

        Response::builder()
            .status(status)
            .body(info.to_string().into())
            .unwrap()
    }
}
