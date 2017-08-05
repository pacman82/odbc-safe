use super::*;
use odbc_sys::*;

/// A `Statement` is most easily thought of as an SQL statement, such as `SELECT * FROM Employee`.
///
/// However, a `Statement` is more than just an SQL statement — it consists of all of the
/// information associated with that SQL statement, such as any result sets created by the
/// `Statement` and parameters used in the execution of the statement. A `Statement` does not even
/// need to have an application-defined SQL statement. For example, when a catalog function such as
/// SQLTables is executed on a `Statement`, it executes a predefined SQL statement that returns a
/// list of table names. Each `Statement` is identified by a statement handle. A `Statement` is
/// associated with a single `Connection`, and there can be multiple `Statements` on that
/// `Connection`. Some drivers limit the number of active `Statement`s they support;
/// Within a piece of code that implements ODBC (the Driver Manager or a driver), the statement
/// handle identifies a structure that contains statement information, such as:
/// * The statement's state
/// * The current statement-level diagnostics
/// * The addresses of the application variables bound to the statement's parameters and result set
///   columns
/// * The current settings of each statement attribute
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