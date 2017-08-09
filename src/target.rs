use odbc_sys::*;
use std::cmp::min;

/// Indicates a type which can be used as a target in `Statement::get_data()`.
pub unsafe trait Target {
    /// C Data type of the buffer returned by `value_ptr()`.
    fn c_data_type() -> SqlCDataType;
    /// Pointer to the buffer in which should be filled with data.
    fn value_ptr(&mut self) -> SQLPOINTER;
    /// Length of the buffer returned by `value_ptr()` in bytes.
    fn buffer_len(&self) -> SQLLEN;
}

unsafe impl Target for [u8] {
    fn c_data_type() -> SqlCDataType {
        SQL_C_CHAR
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        self.as_mut_ptr() as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        min(self.len() as SQLLEN, SQLLEN::max_value())
    }
}
