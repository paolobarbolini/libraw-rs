use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

use libraw_rs_sys as sys;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error {
    code: i32,
}

impl Error {
    pub(crate) fn check(code: i32) -> Result<()> {
        if code == sys::LibRaw_errors_LIBRAW_SUCCESS {
            Ok(())
        } else {
            Err(Error { code })
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "libraw error: {}", self.code)
    }
}

impl StdError for Error {}
