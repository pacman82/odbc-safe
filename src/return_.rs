use odbc_sys::*;

/// Holds result and indicates the overall success or failure of a function.
#[derive(Debug)]
#[must_use]
pub enum Return<T, E = ()> {
    /// The function has been executed successfully. Holds result.
    Success(T),
    /// The function has been executed successfully. There have been warnings. Holds result.
    Info(T),
    /// An error occured.
    Error(E),
}
pub use Return::{Success, Info, Error};

impl<T, E> Return<T, E> {
    /// Maps a `Return<T,E>` to `Return<U,E>` by applying a function to a contained `Success` or
    /// `Info` value, leaving an `Error` value untouched.
    pub fn map<F, U>(self, f: F) -> Return<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Success(v) => Success(f(v)),
            Info(v) => Info(f(v)),
            Error(e) => Error(e),
        }
    }

    /// Maps a `Return<T,E>` to `Result<T,U>` by applying a function to a contained `Error value,
    /// leaving a `Success` or an `Info` value untouched.
    pub fn map_error<F, U>(self, f: F) -> Return<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Success(v) => Success(v),
            Info(v) => Info(v),
            Error(e) => Error(f(e)),
        }
    }

    /// Unwraps the result, yielding the content of `Success` or `Info`
    pub fn unwrap(self) -> T {
        match self {
            Success(v) | Info(v) => v,
            Error(_) => {
                panic!("Unwrapping `Return` failed. Use `Diagnostics` to obtain more information.")
            }
        }
    }
}

impl From<SQLRETURN> for Return<()> {
    fn from(source: SQLRETURN) -> Return<()> {
        match source {
            SQL_SUCCESS => Success(()),
            SQL_SUCCESS_WITH_INFO => Info(()),
            SQL_ERROR => Error(()),
            other => panic!("Unexpected SQLRETURN value: {:?}", other),
        }
    }
}
