use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
pub use error_code_derive::ToErrorInfo;
use std::{
    fmt::{Debug, Display},
    hash::{DefaultHasher, Hash, Hasher},
    str::FromStr,
};
pub trait ToErrorInfo {
    type T: FromStr;
    fn to_error_info(&self) -> ErrorInfo<Self::T>;
}
pub struct ErrorInfo<T> {
    pub app_code: T,
    pub code: &'static str,
    pub hash: String,
    pub client_msg: &'static str,
    pub server_msg: String,
}

impl<T> ErrorInfo<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn new(
        app_code: &str,
        code: &'static str,
        client_msg: &'static str,
        server_msg: impl Display,
    ) -> Self {
        let server_msg = server_msg.to_string();
        let mut hasher = DefaultHasher::new();
        server_msg.hash(&mut hasher);
        let hash = hasher.finish();
        let hash = URL_SAFE_NO_PAD.encode(hash.to_be_bytes());

        Self {
            app_code: T::from_str(app_code).expect("can't convert to app code"),
            code,
            hash,
            client_msg,
            server_msg: server_msg.to_string(),
        }
    }
}

impl<T> ErrorInfo<T> {
    pub fn client_msg(&self) -> &str {
        if self.client_msg.is_empty() {
            &self.server_msg
        } else {
            self.client_msg
        }
    }
}
// Display: for client facing error message
impl<T> Display for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.client_msg())
    }
}

impl<T> Debug for ErrorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}] {}", self.code, self.hash, self.server_msg)
    }
}
