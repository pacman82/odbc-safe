use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

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
pub struct Statement<'con, S = NoResult> {
    state: PhantomData<S>,
    handle: HStmt<'con>,
}

/// Cursor state of `Statement`. A statement is likely to enter this state after executing e.g a
/// `SELECT` query.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum HasResult {}
/// State used by `Statement`. A statement is likely to enter this state after executing e.g. a
/// `CREATE TABLE` statement.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum NoResult {}
/// Cursor state of `Statement`. A statement will enter this state after a successful call to
/// `fetch()`
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Positioned {}

pub trait CursorState {}
impl CursorState for HasResult {}
impl CursorState for Positioned {}

impl<'con, S> Statement<'con, S> {
    /// Provides access to the raw ODBC Statement Handle
    pub fn as_raw(&self) -> SQLHSTMT {
        self.handle.as_raw()
    }

    fn transit<S2>(self) -> Statement<'con, S2> {
        Statement {
            handle: self.handle,
            state: PhantomData,
        }
    }
}

impl<'con, C> Statement<'con, C>
where
    C: CursorState,
{
    /// Returns the number of columns of the result set
    ///
    /// See [SQLNumResultCols][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlnumresultcols-function
    pub fn num_result_cols(&self) -> Return<SQLSMALLINT> {
        self.handle.num_result_cols()
    }

    /// Advances Cursor to next row
    ///
    /// See [SQLFetch][1]
    /// See [Fetching a Row of Data][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlfetch-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/fetching-a-row-of-data
    pub fn fetch(mut self) -> ReturnOption<Statement<'con, Positioned>, Statement<'con, NoResult>> {
        match self.handle.fetch() {
            ReturnOption::Success(()) => ReturnOption::Success(self.transit()),
            ReturnOption::Info(()) => ReturnOption::Info(self.transit()),
            ReturnOption::NoData(()) => ReturnOption::NoData(self.transit()),
            ReturnOption::Error(()) => ReturnOption::Error(self.transit()),
        }
    }

    /// Closes the cursor. Cursors only need to be closed explicitly if the Statement handle is
    /// intended to be reused, but a result set is not consumed.
    ///
    /// See [SQLCloseCursor][1]
    /// See [Closing the Cursor][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlclosecursor-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/closing-the-cursor
    pub fn close_cursor(mut self) -> Return<Statement<'con, NoResult>, Statement<'con, C>> {
        match self.handle.close_cursor() {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }
}

impl<'con> Statement<'con, NoResult> {
    /// Allocates a new `Statement`
    pub fn with_parent(parent: &'con Connection<Connected>) -> Return<Self> {
        HStmt::allocate(parent.as_hdbc()).map(|handle| {
            Statement {
                handle,
                state: PhantomData,
            }
        })
    }

    /// Executes a preparable statement, using the current values of the parametr marker variables.
    ///
    /// * See [SQLExecDirect][1]
    /// * See [Direct Execution][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlexecdirect-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/direct-execution-odbc
    pub fn exec_direct<T>(
        mut self,
        statement_text: &T,
    ) -> ReturnOption<Statement<'con, HasResult>, Statement<'con, NoResult>>
    where
        T: SqlStr + ?Sized,
    {
        match self.handle.exec_direct(statement_text) {
            ReturnOption::Success(()) => ReturnOption::Success(self.transit()),
            ReturnOption::Info(()) => ReturnOption::Info(self.transit()),
            ReturnOption::NoData(()) => ReturnOption::NoData(self.transit()),
            ReturnOption::Error(()) => ReturnOption::Error(self.transit()),
        }
    }
}

impl<'con> Statement<'con, Positioned> {
    /// Retrieves data for a single column or output parameter.
    ///
    /// See [SQLGetData][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlgetdata-function
    pub fn get_data<T>(
        &mut self,
        col_or_param_num: SQLUSMALLINT,
        target: &mut T,
    ) -> ReturnOption<Indicator>
    where
        T: Target + ?Sized,
    {
        self.handle.get_data(col_or_param_num, target)
    }
}

impl<'con, C> Diagnostics for Statement<'con, C> {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn {
        self.handle.diagnostics(rec_number, message_text)
    }
}
