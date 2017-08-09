use super::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::thread::panicking;

#[derive(Debug)]
pub struct HStmt<'con> {
    /// Connection may not outlive the environment used to allocate it
    parent: PhantomData<&'con HDbc<'con>>,
    /// Invariant: Connection handle is always valid
    handle: SQLHSTMT,
}

impl<'con> Drop for HStmt<'con> {
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

unsafe impl<'env> Handle for HStmt<'env> {
    unsafe fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }

    fn handle_type() -> HandleType {
        SQL_HANDLE_STMT
    }
}

impl<'env> HStmt<'env> {
    pub unsafe fn as_raw(&self) -> SQLHSTMT {
        self.handle
    }

    /// Allocates a new Statement Handle
    pub fn allocate(parent: &HDbc) -> Return<Self> {

        let mut out = null_mut();
        unsafe {
            let result: Return<()> =
                SQLAllocHandle(SQL_HANDLE_STMT, parent.handle(), &mut out).into();
            result.map(|()| HStmt { parent: PhantomData, handle: out as SQLHSTMT })
        }
    }

    pub fn exec_direct<T>(&mut self, statement_text: &T) -> ReturnNoData<()>
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

    pub fn fetch(&mut self) -> ReturnNoData<()> {
        unsafe { SQLFetch(self.handle).into() }
    }

    pub fn get_data<T>(
        &mut self,
        col_or_param_num: SQLUSMALLINT,
        target: &mut T,
    ) -> ReturnNoData<Indicator>
    where
        T: Target + ?Sized,
    {
        let mut str_len_or_ind = 0;
        let ret: ReturnNoData<()> = unsafe {
            SQLGetData(
                self.handle,
                col_or_param_num,
                T::c_data_type(),
                target.value_ptr(),
                target.buffer_len(),
                &mut str_len_or_ind,
            ).into()
        };
        ret.map(|()| str_len_or_ind.into())
    }
}
