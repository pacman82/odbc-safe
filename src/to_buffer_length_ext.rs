use odbc_sys::*;
use std::cmp::min;

/// This trait is intended to extend `usize` with safe downsize casts, so we can pass it as buffer
/// length into odbc functions.
pub trait ToBufferLengthExt<T> {
    fn to_buf_len(self) -> T;
}

impl ToBufferLengthExt<SQLSMALLINT> for usize {
    fn to_buf_len(self) -> SQLSMALLINT {
        min(self, SQLSMALLINT::max_value() as usize) as SQLSMALLINT
    }
}

impl ToBufferLengthExt<SQLLEN> for usize {
    fn to_buf_len(self) -> SQLLEN {
        min(self, SQLSMALLINT::max_value() as usize) as SQLLEN
    }
}
