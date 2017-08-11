use odbc_sys::*;

/// Holds result and indicates the overall success or failure of a function.
#[derive(Debug)]
#[must_use]
pub enum ReturnOption<T, E = ()> {
    /// The function has been executed successfully. Holds result.
    Success(T),
    /// The function has been executed successfully. There have been warnings. Holds result.
    Info(T),
    /// No more data was available
    NoData(E),
    /// An error occured.
    Error(E),
}

impl<T, E> ReturnOption<T, E> {
    /// Maps a `ReturnOption<T,E>` to `ReturnOption<U,E>` by applying a function to a contained
    /// `Success` or `Info` value, leaving an `Error` or `NoData` value untouched.
    pub fn map<F, U>(self, f: F) -> ReturnOption<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ReturnOption::Success(t) => ReturnOption::Success(f(t)),
            ReturnOption::Info(t) => ReturnOption::Info(f(t)),
            ReturnOption::NoData(e) => ReturnOption::NoData(e),
            ReturnOption::Error(e) => ReturnOption::Error(e),
        }
    }
}

impl From<SQLRETURN> for ReturnOption<()> {
    fn from(source: SQLRETURN) -> ReturnOption<()> {
        match source {
            SQL_SUCCESS => ReturnOption::Success(()),
            SQL_SUCCESS_WITH_INFO => ReturnOption::Info(()),
            SQL_ERROR => ReturnOption::Error(()),
            SQL_NO_DATA => ReturnOption::NoData(()),
            other => panic!("Unexpected SQLRETURN value: {:?}", other),
        }
    }
}
