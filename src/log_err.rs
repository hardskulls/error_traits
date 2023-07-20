
use std::fmt::Display;
use crate::StdResult;

/// If error is present, this trait logs it and returns back.
/// Requires an initialized logger.
///
/// # Examples
///
/// ```
/// use std::net::SocketAddr;
/// use error_traits::LogErr;
///
/// let error = "foo".parse::<SocketAddr>().log_err("some_log_prefix: error");
/// ```

pub trait LogErr
{
    fn log_err(self, log_msg : &str) -> Self;
}

impl<T, E> LogErr for StdResult<T, E>
    where
        E : Display
{
    fn log_err(self, log_prefix : &str) -> Self
    {
        if let Err(e) = &self
        { log::error!("{log_prefix}{e}") }
        self
    }
}


