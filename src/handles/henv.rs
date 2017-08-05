use super::*;
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
                other => {
                    if !panicking() {
                        panic!("Unexepected return value of SQLFreeHandle: {:?}", other)
                    }
                }
            }
        }
    }
}

unsafe impl Handle for HEnv {
    unsafe fn handle(&self) -> SQLHANDLE {
        self.handle as SQLHANDLE
    }

    fn handle_type() -> HandleType {
        SQL_HANDLE_ENV
    }
}

impl HEnv {
    /// Allocates a new Environment Handle
    pub fn allocate() -> Return<HEnv> {

        let mut out = null_mut();
        unsafe {
            let result: Return<()> = SQLAllocHandle(SQL_HANDLE_ENV, null_mut(), &mut out).into();
            result.map(|()| HEnv { handle: out as SQLHENV })
        }
    }

    pub fn declare_version(&mut self, version: OdbcVersion) -> Return<()> {
        unsafe { SQLSetEnvAttr(self.handle, SQL_ATTR_ODBC_VERSION, version.into(), 0).into() }
    }

    /// Provides access to the raw ODBC environment handle.
    pub unsafe fn as_raw(&self) -> SQLHENV {
        self.handle
    }
}
