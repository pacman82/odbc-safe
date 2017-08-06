use super::*;
use odbc_sys::*;

/// A `Statement` is most easily thought of as an SQL statement, such as `SELECT * FROM Employee`.
///
/// * The statement's state
/// * The current statement-level diagnostics
/// * The addresses of the application variables bound to the statement's parameters and result set
///   columns
/// * The current settings of each statement attribute
///
/// See [Statement Handles][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/statement-handles
#[derive(Debug)]
pub struct Statement<'con> {
    handle: HStmt<'con>,
}

impl<'con> Statement<'con> {
    /// Provides access to the raw ODBC Statement Handle
    pub unsafe fn as_raw(&self) -> SQLHSTMT {
        self.handle.as_raw()
    }

    /// Allocates a new `Statement`
    pub fn with_parent(parent: &'con Connection<Connected>) -> Return<Self> {
        HStmt::allocate(parent.as_hdbc()).map(|handle| Statement { handle })
    }
}

impl<'con> Diagnostics for Statement<'con> {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn {
        self.handle.diagnostics(rec_number, message_text)
    }
}