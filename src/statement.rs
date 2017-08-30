use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

/// A `Statement` is most easily thought of as an SQL statement, such as
/// `SELECT * FROM Employee`.
///
/// * The statement's state
/// * The current statement-level diagnostics
/// * The addresses of the application variables bound to the statement's
/// parameters and result set
///   columns
/// * The current settings of each statement attribute
///
/// See [Statement Handles][1]
///
/// Specific to the rust wrapper of an ODBC Statement is, that we do keep track
/// of the lifetimes of
/// the parent `Connection`, parameters as well as `columns` bound to the
/// `Statement`. Since it is
/// possible to unbind the parameters and columns we have to keep track of
/// their lifetimes
/// seperatly.
///
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/statement-handles
#[derive(Debug)]
pub struct Statement<'con, 'param, 'col, C = NoCursor, A = Unprepared> {
    cursor: PhantomData<C>,
    access_plan: PhantomData<A>,
    /// Statement may not outlive parameters bound to it.
    parameters: PhantomData<&'param [u8]>,
    columns: PhantomData<&'col [u8]>,
    handle: HStmt<'con>,
}

/// Cursor state of `Statement`. A statement is likely to enter this state
/// after executing e.g a
/// `SELECT` query.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Open {}
/// State used by `Statement`. A statement is likely to enter this state after
/// executing e.g. a
/// `CREATE TABLE` statement.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum NoCursor {}
/// Cursor state of `Statement`. A statement will enter this state after a
/// successful call to
/// `fetch()`
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Positioned {}
/// State used by `Statement`. A statement will enter this state after a
/// successful call to
/// `prepare()`.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Prepared {}
/// State used by `Statement`. Indicates that no Access Plan has been created,
/// yet.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Unprepared {}

/// Implemented by the `Open` and `Positioned` states for `Statement`.
pub trait CursorState {}
impl CursorState for Open {}
impl CursorState for Positioned {}

impl<'con, 'param, 'col, S, A> Statement<'con, 'param, 'col, S, A> {
    /// Provides access to the raw ODBC Statement Handle
    pub fn as_raw(&self) -> SQLHSTMT {
        self.handle.as_raw()
    }

    /// Binds a parameter to a parameter marker in an SQL Statement
    ///
    /// # Result
    /// This method will destroy the statement and create a new one which may not outlive the bound
    /// parameter. This is to ensure that the statement will not derefernce an invalid pointer
    /// during execution. Use `reset_parameters` to reset the bound parameters and increase the
    /// `'param` lifetime back to `'static`.
    ///
    /// # Arguments
    /// * `parameter_number` - Index of the marker to bind to the parameter. Starting at `1`
    /// * `parameter_type` - SQL Type of the parameter
    /// * `value` - Reference to bind to the marker
    ///
    /// See [SQLBindParameter Function][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlbindparameter-function#columnsize-argument
    pub fn bind_input_parameter<'p, T>(
        mut self,
        parameter_number: SQLUSMALLINT,
        parameter_type: DataType,
        value: Option<&'p T>,
    ) -> Return<Statement<'con, 'p, 'col, S, A>, Self>
    where
        T: CDataType + ?Sized,
        'param: 'p,
    {
        unsafe {
            match self.handle.bind_input_parameter(
                parameter_number,
                parameter_type,
                value,
            ) {
                Success(()) => Success(self.transit()),
                Info(()) => Info(self.transit()),
                Error(()) => Error(self.transit()),
            }
        }
    }

    /// Binds a buffer and an indicator to a column.
    ///
    /// See [SQLBindCol][1]:
    /// [1]: [https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindcol-function]
    pub fn bind_col<'col_new, T>(
        mut self,
        column_number: SQLUSMALLINT,
        value: &'col_new mut T,
        indicator: &'col_new mut SQLLEN,
    ) -> Return<Statement<'con, 'param, 'col_new, S, A>, Self>
    where
        T: CDataType + ?Sized,
        'col: 'col_new,
    {
        unsafe {
            match self.handle.bind_col(column_number, value, indicator) {
                Success(()) => Success(self.transit()),
                Info(()) => Info(self.transit()),
                Error(()) => Error(self.transit()),
            }
        }
    }

    /// Unbinds the parameters from the parameter markers
    pub fn reset_parameters(mut self) -> Statement<'con, 'static, 'col, S, A> {
        self.handle.reset_parameters().unwrap();
        self.transit()
    }

    /// Unbinds column buffers from result set.
    pub fn reset_columns(mut self) -> Statement<'con, 'param, 'static, S, A> {
        self.handle.reset_columns().unwrap();
        self.transit()
    }

    fn transit<'p, 'c, S2, A2>(self) -> Statement<'con, 'p, 'c, S2, A2> {
        Statement {
            handle: self.handle,
            parameters: PhantomData,
            columns: PhantomData,
            cursor: PhantomData,
            access_plan: PhantomData,
        }
    }
}

impl<'con, 'param, 'col, C, A> Statement<'con, 'param, 'col, C, A>
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
    pub fn fetch(
        mut self,
    ) -> ReturnOption<
        Statement<'con, 'param, 'col, Positioned, A>,
        Statement<'con, 'param, 'col, NoCursor, A>,
    > {
        match self.handle.fetch() {
            ReturnOption::Success(()) => ReturnOption::Success(self.transit()),
            ReturnOption::Info(()) => ReturnOption::Info(self.transit()),
            ReturnOption::NoData(()) => ReturnOption::NoData(self.transit()),
            ReturnOption::Error(()) => ReturnOption::Error(self.transit()),
        }
    }

    /// Closes the cursor. Cursors only need to be closed explicitly if the
    /// Statement handle is
    /// intended to be reused, but a result set is not consumed.
    ///
    /// See [SQLCloseCursor][1]
    /// See [Closing the Cursor][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlclosecursor-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/closing-the-cursor
    pub fn close_cursor(
        mut self,
    ) -> Return<Statement<'con, 'param, 'col, NoCursor>, Statement<'con, 'param, 'col, C, A>> {
        match self.handle.close_cursor() {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }

    /// Return information about result set column
    ///
    /// See [SQLDescribeCol Function][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldescribecol-function
    pub fn describe_col<T>(
        &mut self,
        column_number: SQLUSMALLINT,
        column_name: &mut T,
        column_name_indicator: &mut SQLSMALLINT,
        nullable: &mut Nullable,
    ) -> Return<Option<DataType>>
    where
        T: OutputBuffer + ?Sized,
    {
        let mut data_type = SQL_UNKNOWN_TYPE;
        let mut column_size = 0;
        let mut decimal_digits = 0;
        self.handle.describe_col(
            column_number,
            column_name,
            column_name_indicator,
            &mut data_type,
            &mut column_size,
            &mut decimal_digits,
            nullable
        ).map(|()| DataType::new(data_type, column_size, decimal_digits))
    }
}

impl<'con, 'param, 'col> Statement<'con, 'param, 'col, NoCursor, Unprepared> {
    /// Allocates a new `Statement`
    pub fn with_parent(parent: &'con Connection) -> Return<Self> {
        HStmt::allocate(parent.as_hdbc()).map(|handle| {
            Statement {
                handle,
                parameters: PhantomData,
                cursor: PhantomData,
                columns: PhantomData,
                access_plan: PhantomData,
            }
        })
    }

    /// Prepares a `Statement` for execution by creating an Access Plan.
    ///
    /// See [SQLPrepare Function][1]
    /// See [Prepare and Execute a Statement (ODBC)][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlprepare-function
    /// [2]: https://docs.microsoft.com/sql/relational-databases/native-client-odbc-how-to/execute-queries/prepare-and-execute-a-statement-odbc
    pub fn prepare<T>(
        mut self,
        statement_text: &T,
    ) -> Return<
        Statement<'con, 'param, 'col, NoCursor, Prepared>,
        Statement<'con, 'param, 'col, NoCursor>,
    >
    where
        T: SqlStr + ?Sized,
    {
        // According to the state transition table preparing statements which are
        // already prepared
        // is possible. However we would need to check the status code in order to
        // decide which
        // state the `Statement` is in the case of an error. So for now we only support
        // preparing
        // freshly allocated statements until someone has a use case for 'repreparing'
        // a statement.
        match self.handle.prepare(statement_text) {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }

    /// Executes a preparable statement, using the current values of the
    /// parametr marker variables.
    ///
    /// * See [SQLExecDirect][1]
    /// * See [Direct Execution][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlexecdirect-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/direct-execution-odbc
    pub fn exec_direct<T>(
        mut self,
        statement_text: &T,
    ) -> ReturnOption<
        ResultSet<'con, 'param, 'col, Unprepared>,
        Statement<'con, 'param, 'col, NoCursor>,
    >
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

impl<'con, 'param, 'col> Statement<'con, 'param, 'col, NoCursor, Prepared> {

    /// Return information about result set column
    ///
    /// See [SQLDescribeCol Function][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldescribecol-function
    pub fn describe_col<T>(
        &mut self,
        column_number: SQLUSMALLINT,
        column_name: &mut T,
        column_name_indicator: &mut SQLSMALLINT,
        nullable: &mut Nullable,
    ) -> Return<Option<DataType>>
    where
        T: OutputBuffer + ?Sized,
    {
        let mut data_type = SQL_UNKNOWN_TYPE;
        let mut column_size = 0;
        let mut decimal_digits = 0;
        self.handle.describe_col(
            column_number,
            column_name,
            column_name_indicator,
            &mut data_type,
            &mut column_size,
            &mut decimal_digits,
            nullable
        ).map(|()| DataType::new(data_type, column_size, decimal_digits))
    }

    /// Executes a prepared statement, using the current values fo the
    /// parameter marker variables
    /// if any parameter markers exist in the statement.
    ///
    /// See [SQLExecute Function][1]
    /// See [Prepared Execution][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlexecute-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/prepared-execution-odbc
    pub fn execute(mut self) -> ReturnOption<ResultSet<'con, 'param, 'col, Prepared>, Self> {
        match self.handle.execute() {
            ReturnOption::Success(()) => ReturnOption::Success(self.transit()),
            ReturnOption::Info(()) => ReturnOption::Info(self.transit()),
            ReturnOption::Error(()) => ReturnOption::Error(self.transit()),
            ReturnOption::NoData(()) => ReturnOption::NoData(self.transit()),
        }
    }
}

impl<'con, 'param, 'col, A> Statement<'con, 'param, 'col, Positioned, A> {
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
        T: CDataType + ?Sized,
    {
        self.handle.get_data(col_or_param_num, target)
    }
}

impl<'con, 'param, 'col, C> Diagnostics for Statement<'con, 'param, 'col, C> {
    fn diagnostics(
        &self,
        rec_number: SQLSMALLINT,
        message_text: &mut [SQLCHAR],
    ) -> ReturnOption<DiagResult> {
        self.handle.diagnostics(rec_number, message_text)
    }
}