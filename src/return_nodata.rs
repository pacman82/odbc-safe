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

impl<T, E> ReturnNoData<T, E> {}

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
