pub use error_code_derive::ToErrorInfo;
use std::str::FromStr;
pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> Result<ErrorInfo<Self::T>, <Self::T as FromStr>::Err>;
}
#[derive(Debug)]
pub struct ErrorInfo<T> {
    pub app_code: T,
    pub code: &'static str,
    pub client_msg: &'static str,
    pub server_msg: String,
}

impl<T: FromStr> ErrorInfo<T> {
    pub fn try_new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl Into<String>,
    ) -> Result<Self, T::Err> {
        Ok(Self {
            app_code: T::from_str(app_code)?,
            code,
            client_msg,
            server_msg: server_msg.into(),
        })
    }
}
