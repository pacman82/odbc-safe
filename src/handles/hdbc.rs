use super::*;
use odbc_sys::*;
use std::marker::PhantomData;
use std::mem::forget;
use std::ptr::null_mut;
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
                other => if !panicking() {
                    panic!("Unexepected return value of SQLFreeHandle: {:?}.", other)
                },
            }
        }
    }
}

unsafe impl<'env> Handle for HDbc<'env> {
    const HANDLE_TYPE: HandleType = SQL_HANDLE_DBC;

    fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
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
            let result: Return<()> =
                SQLAllocHandle(Self::HANDLE_TYPE, parent.handle(), &mut out).into();
            result.map(|()| {
                HDbc {
                    parent: PhantomData,
                    handle: out as SQLHDBC,
                }
            })
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
                data_source_name.as_text_ptr(),
                data_source_name.text_length(),
                user.as_text_ptr(),
                user.text_length(),
                pwd.as_text_ptr(),
                pwd.text_length(),
            ).into()
        }
    }

    pub fn driver_connect<I>(
        &mut self,
        in_connection_string: &I,
        out_connection_string: &mut [u8],
        driver_completion: SqlDriverConnectOption,
    ) -> Return<SQLSMALLINT>
    where
        I: SqlStr + ?Sized,
    {
        unsafe {
            let window_handle = null_mut();
            let mut out_connection_string_len = 0;
            let ret: Return<()> = SQLDriverConnect(
                self.handle,
                window_handle,
                in_connection_string.as_text_ptr(),
                in_connection_string.text_length(),
                out_connection_string.mut_buf_ptr(),
                out_connection_string.buf_len(),
                &mut out_connection_string_len,
                driver_completion,
            ).into();
            ret.map(|()| out_connection_string_len)
        }
    }

    pub fn disconnect(&mut self) -> Return<()> {
        unsafe { SQLDisconnect(self.handle).into() }
    }

    /// Returns wether the data source is read only
    pub fn is_read_only(&mut self) -> Return<bool> {
        unsafe {
            let mut buffer = [0; 2];
            let ret: Return<()> = SQLGetInfo(
                self.handle,
                SQL_DATA_SOURCE_READ_ONLY,
                buffer.as_mut_ptr() as SQLPOINTER,
                buffer.buf_len(),
                null_mut(),
            ).into();
            ret.map(|()| match buffer[0] as char {
                'N' => false,
                'Y' => true,
                _ => panic!(r#"Briver may only return "N" or "Y""#),
            })
        }
    }
}
