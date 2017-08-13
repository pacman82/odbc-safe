use super::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::thread::panicking;

#[derive(Debug)]
pub struct HStmt<'con, 'param> {
    /// Statement may not outlive the connection used to allocate it.
    parent: PhantomData<&'con HDbc<'con>>,
    /// Statement may not outlive parameters bound to it.
    parameters: PhantomData<&'param [u8]>,
    /// Invariant: Connection handle is always valid.
    handle: SQLHSTMT,
}

impl<'con, 'param> Drop for HStmt<'con, 'param> {
    fn drop(&mut self) {
        unsafe {
            match SQLFreeHandle(SQL_HANDLE_STMT, self.handle as SQLHANDLE) {
                SQL_SUCCESS => (),
                other => if !panicking() {
                    panic!("Unexepected return value of SQLFreeHandle: {:?}.", other)
                },
            }
        }
    }
}

unsafe impl<'env, 'param> Handle for HStmt<'env, 'param> {
    fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }

    fn handle_type() -> HandleType {
        SQL_HANDLE_STMT
    }
}

impl<'env, 'param> HStmt<'env, 'param> {
    pub fn as_raw(&self) -> SQLHSTMT {
        self.handle
    }

    /// Allocates a new Statement Handle
    pub fn allocate(parent: &HDbc) -> Return<Self> {

        let mut out = null_mut();
        unsafe {
            let result: Return<()> =
                SQLAllocHandle(SQL_HANDLE_STMT, parent.handle(), &mut out).into();
            result.map(|()| HStmt { parent: PhantomData, parameters: PhantomData, handle: out as SQLHSTMT })
        }
    }

    pub fn exec_direct<T>(&mut self, statement_text: &T) -> ReturnOption<()>
    where
        T: SqlStr + ?Sized,
    {
        unsafe {
            SQLExecDirect(
                self.handle,
                statement_text.as_ansi_ptr(),
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

    pub fn bind_input_parameter<T>(
        &mut self,
        parameter_number: SQLUSMALLINT,
        parameter_type: DataType,
        value: Option<&'param T>,
    ) -> Return<()>
    where
        T: CDataType + ?Sized,
    {
        let mut indicator = if value.is_some() { 0 } else { SQL_NULL_DATA };
        unsafe {
            SQLBindParameter(
                self.handle,
                parameter_number,
                SQL_PARAM_INPUT,
                T::c_data_type(),
                parameter_type.sql_data_type(),
                parameter_type.column_size(),
                parameter_type.decimal_digits(),
                value.map_or(null_mut(), |v| v.sql_ptr() as SQLPOINTER),
                0,
                &mut indicator,
            ).into()
        }
    }
}
