use std::string::FromUtf8Error;

use error_code::ToErrorInfo;
use thiserror::Error;
#[derive(Error, Debug, ToErrorInfo)]
#[error_info(app_type = "http::StatusCode", prefix = "01")]
pub enum MyError {
    #[error("Invalid command: {0}")]
    #[error_info(code = "IC", app_code = "400")]
    InvalidCommand(String),

    #[error("Invalid argument: {0}")]
    #[error_info(code = "IA", app_code = "400", client_msg = "friendly msg")]
    InvalidArgument(String),
    #[error("{0}")]
    #[error_info(code = "RE", app_code = "500")]
    RespError(#[from] FromUtf8Error),
}

fn main() {
    let err = MyError::InvalidArgument("argument".to_string());
    let info = err.to_error_info();
    println!("{:?}", info);
}
