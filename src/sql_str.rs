use odbc_sys::*;
use std::ffi::CStr;

/// A type implementing this trait can be passed as a string argument in API calls
pub unsafe trait SqlStr {
    /// Returns a pointer to the start of the string
    fn as_ansi_ptr(&self) -> *const SQLCHAR;
    /// Returns buffer length or SQL_NTS
    fn len(&self) -> SQLSMALLINT;
}

unsafe impl SqlStr for CStr {
    fn as_ansi_ptr(&self) -> *const SQLCHAR {
        self.as_ptr() as *const SQLCHAR
    }

    fn len(&self) -> SQLSMALLINT {
        SQL_NTS
    }
}

unsafe impl SqlStr for [u8] {
    fn as_ansi_ptr(&self) -> *const SQLCHAR {
        self.as_ptr()
    }

    fn len(&self) -> SQLSMALLINT {
        if self.len() > SQLSMALLINT::max_value() as usize {
            panic!(
                "Buffer length of {} is greater than SQLSMALLINT::MAX: {}",
                self.len(),
                SQLSMALLINT::max_value()
            );
        }
        self.len() as SQLSMALLINT
    }
}
