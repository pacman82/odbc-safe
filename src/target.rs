use odbc_sys::*;

/// Indicates a type which can be used as a target in `Statement::get_data()`.
pub unsafe trait Target {
    /// C Data type of the buffer returned by `value_ptr()`.
    fn c_data_type() -> SqlCDataType;
    /// Pointer to the buffer in which should be filled with data.
    fn value_ptr(&mut self) -> SQLPOINTER;
    /// Length of the buffer returned by `value_ptr()` in bytes.
    fn buffer_len(&self) -> SQLLEN;
}
