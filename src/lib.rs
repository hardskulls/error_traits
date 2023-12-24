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

let res : Result<String, String> = Ok("foo".to_owned());
let merged_res : String = res.merge_ok_err();
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
let number : Result<u32, MyError> = "42".parse::<u32>().map_err_by(error);
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

let number : Result<u32, String> = "42".parse::<u32>().map_err_to_str();
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
Wraps any type in `Ok` or `Err` variant of standard `Result` type.
Helps to avoid parenthesis apocalypse.


# Examples

```
use error_traits::WrapInRes;

enum MyEnum
{
    Foo(u8),
    Bar(u8)
}

// Instead of writing
let e : Result<String, Option<MyEnum>> = Err(Some(MyEnum::Foo(0)));

// you can write:
let r : Result<String, Option<MyEnum>> = MyEnum::Foo(0).into().in_err();
// or
let r : Result<Option<MyEnum>, _> =
    MyEnum::Foo(0)
        .into()
        .in_ok::<String>();
```
*/
pub trait WrapInRes<T> {
    fn in_ok<ERR>(self) -> Result<T, ERR>;
    fn in_err<OK>(self) -> Result<OK, T>;
}

impl<T> WrapInRes<T> for T {
    fn in_ok<ERR>(self) -> Result<T, ERR> {
        Ok(self)
    }
    fn in_err<OK>(self) -> Result<OK, T> {
        Err(self)
    }
}

pub trait ToEmpty {
    fn to_empty(self);
}

impl<T> ToEmpty for T {
    fn to_empty(self) {}
}

/**
Transforms one type into another.


# Examples

```
use std::time::Duration;
use error_traits::MapType;

let minutes = 5;

let duration : Duration = minutes.map_type(|m| Duration::from_secs(m * 60));
```
*/
pub trait MapType<M, N> {
    fn map_type(self, f: impl FnOnce(M) -> N) -> N;
}

impl<M, N> MapType<M, N> for M {
    fn map_type(self, f: impl FnOnce(M) -> N) -> N {
        f(self)
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

/// Wraps type in `Option` and returns `None` if condition is true.
pub trait ToNoneIf: Sized {
    fn to_none_if(self, cond: impl Fn(&Self) -> bool) -> Option<Self> {
        match cond(&self) {
            true => None,
            _ => Some(self)
        }
    }
}

impl<T> ToNoneIf for T {}

/// Wraps type in `Result` and returns `Err` if condition is true.
pub trait ToErrIf: Sized {
    fn to_err_if<T>(self, cond: impl Fn(&Self) -> bool, err: T) -> Result<Self, T> {
        match cond(&self) {
            true => Err(err),
            _ => Ok(self)
        }
    }
}

impl<T> ToErrIf for T {}
