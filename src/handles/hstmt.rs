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
}
