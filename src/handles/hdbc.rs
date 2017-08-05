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
                        panic!("Unexepected return value of SQLFreeHandle: {:?}.", other)
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
    pub unsafe fn as_raw(&self) -> SQLHDBC {
        self.handle
    }

    /// Allocates a new Connection Handle
    pub fn allocate(parent: &HEnv) -> Return<Self> {

        let mut out = null_mut();
        unsafe {
            let result: Return<()> = SQLAllocHandle(Self::handle_type(), parent.handle(), &mut out)
                .into();
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
                data_source_name.text_length(),
                user.as_ansi_ptr(),
                user.text_length(),
                pwd.as_ansi_ptr(),
                pwd.text_length(),
            ).into()
        }
    }

    pub fn disconnect(&mut self) -> Return<()> {
        unsafe { SQLDisconnect(self.handle).into() }
    }
}
