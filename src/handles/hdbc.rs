use super::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::thread::panicking;
use std::mem::forget;

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
    fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }

    fn handle_type() -> HandleType {
        SQL_HANDLE_DBC
    }
}

impl<'env> HDbc<'env> {
    /// Consumes the `Connection`, returning the wrapped raw `SQLHDBC`
    ///
    /// Leaks the Connection Handle. This is usually done in order to pass ownership from Rust to
    /// another language. After calling this method, the caller is responsible for invoking
    /// `SQLFreeHandle`.
    pub fn into_raw(self) -> SQLHDBC {
        let raw = self.handle;
        forget(self);
        raw
    }

    /// Provides access to the raw ODBC Connection Handle
    pub fn as_raw(&self) -> SQLHDBC {
        self.handle
    }

    /// May only be invoked with a valid Statement Handle which has been allocated using
    /// `SQLAllocHandle`.
    pub unsafe fn from_raw(raw: SQLHDBC) -> Self {
        HDbc {
            handle: raw,
            parent: PhantomData,
        }
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
