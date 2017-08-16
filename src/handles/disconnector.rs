use super::*;
use std::thread::panicking;
use std::mem::forget;
use std::ptr;

/// A wrapper around a Connection Handle, which calls `disconnect` on Drop
#[derive(Debug)]
pub struct Disconnector<'env>(pub HDbc<'env>);

impl<'env> Drop for Disconnector<'env> {
    fn drop(&mut self) {
        match self.0.disconnect() {
            Success(()) | Info(()) => (),
            Error(()) => if !panicking() {
                panic!("SQLDisconnect returned error")
            },
        }
    }
}

impl<'env> Disconnector<'env> {
    /// Releases inner Connection Handle without calling disconnect.
    pub fn into_hdbc(self) -> HDbc<'env> {
        unsafe {
            let hdbc = ptr::read(&self.0);
            forget(self); // do not call drop
            hdbc
        }
    }
}
