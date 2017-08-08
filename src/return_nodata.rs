use odbc_sys::*;

/// Holds result and indicates the overall success or failure of a function.
#[derive(Debug)]
#[must_use]
pub enum ReturnNoData<T, E = ()> {
    /// The function has been executed successfully. Holds result.
    Success(T),
    /// The function has been executed successfully. There have been warnings. Holds result.
    Info(T),
    /// No more data was available
    NoData(E),
    /// An error occured.
    Error(E),
}

impl<T, E> ReturnNoData<T, E> {
    /// Maps a `ReturnNoData<T,E>` to `ReturnNoData<U,E>` by applying a function to a contained
    /// `Success` or `Info` value, leaving an `Error` or `NoData` value untouched.
    pub fn map<F, U>(self, f: F) -> ReturnNoData<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ReturnNoData::Success(t) => ReturnNoData::Success(f(t)),
            ReturnNoData::Info(t) => ReturnNoData::Info(f(t)),
            ReturnNoData::NoData(e) => ReturnNoData::NoData(e),
            ReturnNoData::Error(e) => ReturnNoData::Error(e),
        }
    }
}

impl From<SQLRETURN> for ReturnNoData<()> {
    fn from(source: SQLRETURN) -> ReturnNoData<()> {
        match source {
            SQL_SUCCESS => ReturnNoData::Success(()),
            SQL_SUCCESS_WITH_INFO => ReturnNoData::Info(()),
            SQL_ERROR => ReturnNoData::Error(()),
            SQL_NO_DATA => ReturnNoData::NoData(()),
            other => panic!("Unexpected SQLRETURN value: {:?}", other),
        }
    }
}
