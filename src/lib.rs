#[cfg(feature = "error_stack_dyn_ext")]
mod err_stack_ext;
#[cfg(feature = "log_err")]
mod log_err;

#[cfg(feature = "error_stack_dyn_ext")]
pub use err_stack_ext::*;
#[cfg(feature = "log_err")]
pub use log_err::*;

type StdResult<T, E> = Result<T, E>;

/**
When `Ok()` and `Err` variants of `Result` are the same
type, it returns this type whether it's an error, or not.


# Examples

Basic usage:

```
use error_traits::MergeOkErr;

let res: Result<String, String> = Ok("foo".to_owned());
let merged_res: String = res.merge_ok_err();
```
*/
pub trait MergeOkErr<T> {
    fn merge_ok_err(self) -> T;
}

impl<T> MergeOkErr<T> for StdResult<T, T> {
    fn merge_ok_err(self) -> T {
        self.unwrap_or_else(|err| err)
    }
}

/**
This trait provides little helper method that replaces error completely ignoring it.
Required in order to get rid of ugly `.map_err(|_| bar())` calls.


# Examples

Basic usage:

```
use error_traits::MapErrBy;

struct MyError;

let error = || MyError;
let number: Result<u32, MyError> = "42".parse::<u32>().map_err_by(error);
```
*/
pub trait MapErrBy<T, N> {
    fn map_err_by(self, f: impl Fn() -> N) -> StdResult<T, N>;
}

impl<T, E, N> MapErrBy<T, N> for StdResult<T, E> {
    fn map_err_by(self, f: impl Fn() -> N) -> StdResult<T, N> {
        self.map_err(|_| f())
    }
}

/**
Turns error into a string.


# Examples

Basic usage:

```
use error_traits::{MapErrBy, MapErrToString};

let number: Result<u32, String> = "42".parse::<u32>().map_err_to_str();
```
*/
pub trait MapErrToString<T> {
    fn map_err_to_str(self) -> Result<T, String>;
}

impl<T, E> MapErrToString<T> for StdResult<T, E>
where
    E: ToString,
{
    fn map_err_to_str(self) -> StdResult<T, String> {
        self.map_err(|e| e.to_string())
    }
}

/**
Applies `f` to `Result` type if it is an error an returns it back.
Meant to be used for logging, or something alike.


# Examples

```
use error_traits::PassErrWith;
let result = "foo".parse::<u16>().pass_err_with(|e| println!("[:: LOG ::] {e}"));
```
*/
pub trait PassErrWith {
    type Error;

    fn pass_err_with(self, f: impl Fn(&Self::Error)) -> Self;
}

impl<T, E> PassErrWith for Result<T, E> {
    type Error = E;

    fn pass_err_with(self, f: impl Fn(&Self::Error)) -> Self {
        if let Err(e) = &self {
            f(e)
        }
        self
    }
}
