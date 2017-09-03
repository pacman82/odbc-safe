use super::{Handle, OutputBuffer, Return, ReturnOption};
use sys::*;
use std::ptr::null_mut;
use std::thread::panicking;

/// An `Environment` is a global context, in which to access data.
///
/// Associated with an `Environment` is any information that is global in nature, such as:
///
/// * The `Environment`'s state
/// * The current environment-level diagnostics
/// * The handles of connections currently allocated on the environment
/// * The current stetting of each environment attribute
#[derive(Debug)]
pub struct HEnv {
    /// Invariant: Should always point to a valid ODBC Environment
    handle: SQLHENV,
}

impl Drop for HEnv {
    fn drop(&mut self) {
        unsafe {
            match SQLFreeHandle(SQL_HANDLE_ENV, self.handle as SQLHANDLE) {
                SQL_SUCCESS => (),
                other => if !panicking() {
                    panic!("Unexepected return value of SQLFreeHandle: {:?}", other)
                },
            }
        }
    }
}

unsafe impl Handle for HEnv {
    const HANDLE_TYPE: HandleType = SQL_HANDLE_ENV;

    fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }
}

impl HEnv {
    /// Allocates a new Environment Handle
    pub fn allocate() -> Return<HEnv> {
        let mut out = null_mut();
        unsafe {
            let result: Return<()> = SQLAllocHandle(SQL_HANDLE_ENV, null_mut(), &mut out).into();
            result.map(|()| {
                HEnv {
                    handle: out as SQLHENV,
                }
            })
        }
    }

    pub fn declare_version(&mut self, version: OdbcVersion) -> Return<()> {
        unsafe { SQLSetEnvAttr(self.handle, SQL_ATTR_ODBC_VERSION, version.into(), 0).into() }
    }

    /// Fills buffers and returns `(name_length, description_length)`
    pub fn data_sources(
        &mut self,
        direction: FetchOrientation,
        server_name: &mut [u8],
        description: &mut [u8],
    ) -> ReturnOption<(SQLSMALLINT, SQLSMALLINT)> {
        unsafe {
            let mut name_length = 0;
            let mut description_length = 0;
            let ret: ReturnOption<()> = SQLDataSources(
                self.handle,
                direction,
                server_name.mut_buf_ptr(),
                server_name.buf_len(),
                &mut name_length,
                description.mut_buf_ptr(),
                description.buf_len(),
                &mut description_length,
            ).into();
            ret.map(|()| (name_length, description_length))
        }
    }

    /// Fills buffers and returns `(description_length, attributes_length)`
    pub fn drivers(
        &mut self,
        direction: FetchOrientation,
        description: &mut [u8],
        attributes: &mut [u8],
    ) -> ReturnOption<(SQLSMALLINT, SQLSMALLINT)> {
        unsafe {
            let mut description_length = 0;
            let mut attributes_length = 0;
            let ret: ReturnOption<()> = SQLDrivers(
                self.handle,
                direction,
                description.mut_buf_ptr(),
                description.buf_len(),
                &mut description_length,
                attributes.mut_buf_ptr(),
                attributes.buf_len(),
                &mut attributes_length,
            ).into();
            ret.map(|()| (description_length, attributes_length))
        }
    }

    /// Provides access to the raw ODBC environment handle.
    pub fn as_raw(&self) -> SQLHENV {
        self.handle
    }
}
