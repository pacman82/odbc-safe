use odbc_sys::*;

/// Used to indicate the required target buffer length.
#[derive(Debug, Clone, Copy)]
pub enum Indicator {
    /// The length required to hold all the data.
    Length(SQLLEN),
    /// Driver does not know how much data is available.
    NoTotal,
    /// The value to be retrieved is NULL.
    Null,
}

impl From<SQLLEN> for Indicator {
    fn from(source: SQLLEN) -> Indicator {
        match source {
            SQL_NO_TOTAL => Indicator::NoTotal,
            SQL_NULL_DATA => Indicator::Null,
            other => Indicator::Length(other),
        }
    }
}
