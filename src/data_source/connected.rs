use super::*;
use std::mem::forget;
use std::ops::Deref;
use std::ptr;
use std::thread::panicking;

/// An `HDbc` with the additional invariant of being 'connected'.
#[derive(Debug)]
pub struct Connected<'env>(HDbc<'env>);

impl<'env> Drop for Connected<'env> {
    fn drop(&mut self) {
        match self.0.rollback() {
            Success(()) | Info(()) => {},
            Error(()) => if !panicking() {
                panic!("SQLEndTran returned error")
            },
        };

        match self.0.disconnect() {
            Success(()) | Info(()) => (),
            Error(()) => if !panicking() {
                panic!("SQLDisconnect returned error")
            },
        }
    }
}

impl<'env> Connected<'env> {
    /// Releases inner Connection Handle without calling disconnect.
    pub fn into_hdbc(self) -> HDbc<'env> {
        unsafe {
            let hdbc = ptr::read(&self.0);
            forget(self); // do not call drop
            hdbc
        }
    }
}

impl<'env> Deref for Connected<'env> {
    type Target = HDbc<'env>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'env> DerefMut for Connected<'env> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'env> HDbcWrapper<'env> for Connected<'env> {
    type Handle = Connected<'env>;

    fn into_hdbc(self) -> HDbc<'env> {
        self.into_hdbc()
    }

    fn from_hdbc(hdbc: HDbc<'env>) -> Self::Handle {
        Connected(hdbc)
    }
}
