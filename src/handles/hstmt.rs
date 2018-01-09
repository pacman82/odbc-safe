use super::*;
use sys::*;
use std::marker::PhantomData;
use std::ptr::{null, null_mut};
use std::thread::panicking;

#[derive(Debug)]
pub struct HStmt<'con> {
    /// Statement may not outlive the connection used to allocate it.
    parent: PhantomData<&'con HDbc<'con>>,
    /// Invariant: Connection handle is always valid.
    handle: SQLHSTMT,
}

impl<'con, 'param> Drop for HStmt<'con> {
    fn drop(&mut self) {
        unsafe {
            match SQLFreeHandle(SQL_HANDLE_STMT, self.handle as SQLHANDLE) {
                SQL_SUCCESS => (),
                other => {
                    if !panicking() {
                        panic!("Unexepected return value of SQLFreeHandle: {:?}.", other)
                    }
                }
            }
        }
    }
}

unsafe impl<'env, 'param> Handle for HStmt<'env> {
    const HANDLE_TYPE: HandleType = SQL_HANDLE_STMT;

    fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }
}

impl<'env, 'param> HStmt<'env> {
    pub fn as_raw(&self) -> SQLHSTMT {
        self.handle
    }

    /// Allocates a new Statement Handle
    pub fn allocate(parent: &HDbc) -> Return<Self> {
        let mut out = null_mut();
        unsafe {
            let result: Return<()> = SQLAllocHandle(SQL_HANDLE_STMT, parent.handle(), &mut out)
                .into();
            result.map(|()| {
                HStmt {
                    parent: PhantomData,
                    handle: out as SQLHSTMT,
                }
            })
        }
    }

    pub fn exec_direct<T>(&mut self, statement_text: &T) -> ReturnOption<()>
    where
        T: SqlStr + ?Sized,
    {
        unsafe {
            SQLExecDirect(
                self.handle,
                statement_text.as_text_ptr(),
                statement_text.text_length_int(),
            ).into()
        }
    }

    pub fn num_result_cols(&self) -> Return<SQLSMALLINT> {
        let mut out: SQLSMALLINT = 0;
        let ret = unsafe { SQLNumResultCols(self.handle, &mut out) };
        let ret: Return<()> = ret.into();
        ret.map(|()| out)
    }

    pub fn fetch(&mut self) -> ReturnOption<()> {
        unsafe { SQLFetch(self.handle).into() }
    }

    pub fn get_data<T>(
        &mut self,
        col_or_param_num: SQLUSMALLINT,
        target: &mut T,
    ) -> ReturnOption<Indicator>
    where
        T: CDataType + ?Sized,
    {
        let mut str_len_or_ind = 0;
        let ret: ReturnOption<()> = unsafe {
            SQLGetData(
                self.handle,
                col_or_param_num,
                T::c_data_type(),
                target.mut_sql_ptr(),
                target.buffer_len(),
                &mut str_len_or_ind,
            ).into()
        };
        ret.map(|()| str_len_or_ind.into())
    }

    pub fn close_cursor(&mut self) -> Return<()> {
        unsafe { SQLCloseCursor(self.handle).into() }
    }

    /// Binds a parameter to a parameter marker in an SQL Statement
    ///
    /// It is the callers responsibility to make sure the bound parameters live long enough.
    pub unsafe fn bind_input_parameter<T>(
        &mut self,
        parameter_number: SQLUSMALLINT,
        parameter_type: DataType,
        value: &T,
        indicator: Option<&SQLLEN>
    ) -> Return<()>
    where
        T: CDataType + ?Sized,
    {
        let indicator: *const SQLLEN = match indicator {
            Some(indicator) => indicator,
            None => null(),
        };
        SQLBindParameter(
            self.handle,
            parameter_number,
            SQL_PARAM_INPUT,
            T::c_data_type(),
            parameter_type.sql_data_type(),
            parameter_type.column_size(),
            parameter_type.decimal_digits(),
            value.sql_ptr() as SQLPOINTER,
            0,
            indicator as *mut SQLLEN,
        ).into()
    }

    pub unsafe fn bind_parameter_set(
        &mut self,
        parameter_size: usize,
        array_size: usize
    ) -> Return<()>
    {
        let res: Return<()> = SQLSetStmtAttr(
            self.handle,
            SQL_ATTR_PARAM_BIND_TYPE,
            parameter_size as SQLPOINTER,
            0
        ).into();

        if res.is_err() {
            return res;
        }

        SQLSetStmtAttr(
            self.handle,
            SQL_ATTR_PARAMSET_SIZE,
            array_size as SQLPOINTER,
            0
        ).into()
    }

    pub fn prepare<T>(&mut self, statement_text: &T) -> Return<()>
    where
        T: SqlStr + ?Sized,
    {
        unsafe {
            SQLPrepare(
                self.handle,
                statement_text.as_text_ptr(),
                statement_text.text_length_int(),
            ).into()
        }
    }

    pub fn reset_parameters(&mut self) -> Return<()> {
        unsafe { SQLFreeStmt(self.handle, SQL_RESET_PARAMS).into() }
    }

    pub fn execute(&mut self) -> ReturnOption<()> {
        unsafe { SQLExecute(self.handle).into() }
    }

    /// Release all columen buffers bound by `bind_col`. Except bookmark column.
    pub fn reset_columns(&mut self) -> Return<()> {
        unsafe { SQLFreeStmt(self.handle, SQL_UNBIND).into() }
    }

    /// Binds application data buffers to columns in the result set
    ///
    /// It is the callers responsibility to make sure the bound columns live long enough.
    pub unsafe fn bind_col<T>(
        &mut self,
        column_number: SQLUSMALLINT,
        value: &mut T,
        indicator: Option<&mut SQLLEN>,
    ) -> Return<()>
    where
        T: CDataType + ?Sized,
    {
        let indicator: *mut SQLLEN = match indicator {
            Some(indicator) => indicator,
            None => null_mut(),
        };
        SQLBindCol(
            self.handle,
            column_number,
            T::c_data_type(),
            value.mut_sql_ptr(),
            value.buffer_len(),
            indicator,
        ).into()
    }

    pub fn describe_col<T>(
        &mut self,
        column_number: SQLUSMALLINT,
        column_name: &mut T,
        column_name_indicator: &mut SQLSMALLINT,
        data_type: &mut SqlDataType,
        column_size: &mut SQLULEN,
        decimal_digits: &mut SQLSMALLINT,
        nullable: &mut Nullable,
    ) -> Return<()>
    where
        T: OutputBuffer + ?Sized,
    {
        unsafe {
            SQLDescribeCol(
                self.handle,
                column_number,
                column_name.mut_buf_ptr(),
                column_name.buf_len(),
                column_name_indicator,
                data_type,
                column_size,
                decimal_digits,
                nullable,
            ).into()
        }
    }
}
