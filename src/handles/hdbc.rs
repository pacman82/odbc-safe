use super::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::thread::panicking;

#[derive(Debug)]
pub struct HDbc<'env> {
    /// Connection may not outlive the environment used to allocate it
    parent: PhantomData<&'env HEnv>,
    /// Invariant: Connection handle is always valid
    handle: SQLHDBC,
}

impl<'env> Drop for HDbc<'env> {
    fn drop(&mut self) {
        unsafe {
            match SQLFreeHandle(SQL_HANDLE_DBC, self.handle as SQLHANDLE) {
                SQL_SUCCESS => (),
                other => {
                    if !panicking() {
                        panic!("Unexepected return value of SQLFreeHandle: {:?}", other)
                    }
                }
            }
        }
    }
}

unsafe impl<'env> Handle for HDbc<'env> {
    unsafe fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }

    fn handle_type() -> HandleType {
        SQL_HANDLE_DBC
    }
}

impl<'env> HDbc<'env> {
    /// Allocates a new Connection Handle
    pub fn allocate(parent: &HEnv) -> Return<Self> {

        let mut out = null_mut();
        unsafe {
            let result: Return<()> =
                SQLAllocHandle(SQL_HANDLE_DBC, parent.handle() as SQLHANDLE, &mut out).into();
            result.map(|()| HDbc { parent: PhantomData, handle: out as SQLHDBC })
        }
    }

    pub fn connect<DSN, U, P>(&mut self, data_source_name: &DSN, user: &U, pwd: &P) -> Return<()>
    where
        DSN: SqlStr + ?Sized,
        U: SqlStr + ?Sized,
        P: SqlStr + ?Sized,
    {
        unsafe {
            SQLConnect(
                self.handle,
                data_source_name.as_ansi_ptr(),
                data_source_name.len(),
                user.as_ansi_ptr(),
                user.len(),
                pwd.as_ansi_ptr(),
                pwd.len(),
            ).into()
        }
    }
}
