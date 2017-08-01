use super::*;
use odbc_sys::*;
use std::cmp::min;
/// A buffer large enough to hold an SOLState for diagnostics and a terminating zero.
pub type State = [SQLCHAR; SQL_SQLSTATE_SIZE + 1];

/// Result of `Diagnostics::diagnostics`
#[derive(Debug, Clone, Copy)]
pub struct DiagResult {
    /// A five-character SQLSTATE code (and terminating NULL) for the diagnostic record
    /// `rec_number`. The first two characters indicate the class; the next three indicate the
    /// subclass. For more information, see [SQLSTATE][1]s.
    /// [1]: https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/sqlstates
    state: State,
    /// Native error code specific to the data source.
    native_error: SQLINTEGER,
    /// The total number of characters (excluding the terminating NULL) available to return in
    /// `message_text`.
    text_length: SQLSMALLINT,
}

/// Returned by `Diagnostics::diagnostics`
#[derive(Debug, Clone, Copy)]
pub enum DiagReturn {
    /// The function successfully returned diagnostic information.
    Success(DiagResult),
    /// The `message_text` buffer was too small to hold the requested diagnostic message. No
    /// diagnostic records were generated. To determine that a truncation occurred, the application
    /// must compare the buffer length to the actual number of bytes available, which is found in
    /// `DiagResult::text_length`
    Info(DiagResult),
    /// `rec_number` was negative or `0`.
    Error,
    /// `rec_number` was greater than the number of diagnostic records that existed for the handle
    /// specified in Handle. The function also returns `NoData` for any positive `rec_number` if
    /// there are no diagnostic records available.
    NoData,
}

/// A type implementing this trait is able to provide diagnostic information regarding the last
/// method call.
pub trait Diagnostics {
    /// Returns the current values of multiple fields of a diagnostic record that contains error,
    /// warning, and status information.
    ///
    /// # Arguments
    ///
    /// * `rec_number` - Indicates the status record from which the application seeks information.
    ///                  Status records are numbered from 1.
    /// * `message_text` - Buffer in which to return the diagnostic message text string. If the
    ///                    number of characters to return is greater than the buffer length, the
    ///                    diagnostic message is truncated to `max(message_text.len() - 1, 0)`. For
    ///                    the format of the string, see [Diagnostic Messages][1]
    /// [1]: https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/diagnostic-messages
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn;
}

impl<H: Handle> Diagnostics for H {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn {
        unsafe {
            let mut text_length = 0;
            let mut state = [0; 6];
            let mut native_error = 0;
            let ret = SQLGetDiagRec(H::handle_type(),
                                    self.handle(),
                                    rec_number,
                                    state.as_mut_ptr(),
                                    &mut native_error,
                                    message_text.as_mut_ptr(),
                                    min(message_text.len() as SQLSMALLINT,
                                        SQLSMALLINT::max_value()),
                                    &mut text_length);
            let result = DiagResult {
                text_length: text_length,
                state: state,
                native_error: native_error,
            };
            match ret {
                SQL_SUCCESS => DiagReturn::Success(result),
                SQL_SUCCESS_WITH_INFO => DiagReturn::Info(result),
                SQL_ERROR => DiagReturn::Error,
                SQL_NO_DATA => DiagReturn::NoData,
                unexpected => panic!("SQLGetDiagRec returned: {:?}", unexpected),
            }
        }
    }
}