use super::*;
use std::mem::forget;
use std::ops::Deref;
use std::ptr;
use std::thread::panicking;
use std::marker::PhantomData;

/// State used by `Connected`. Means that autocommit is enabled
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum AutocommitOn {}

/// State used by `Connected`. Means that autocommit is disabled
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum AutocommitOff {}

/// Marker trait for autocommit mode state types
pub trait AutocommitMode {}

impl AutocommitMode for AutocommitOn {}
impl AutocommitMode for AutocommitOff {}

/// An `HDbc` with the additional invariant of being 'connected'.
#[derive(Debug)]
pub struct Connected<'env, AC: AutocommitMode>(HDbc<'env>, PhantomData<AC>);

impl<'env, AC: AutocommitMode> Drop for Connected<'env, AC> {
    fn drop(&mut self) {
        match self.0.rollback() {
            Success(()) | Info(()) => (),
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

impl<'env, AC: AutocommitMode> Connected<'env, AC> {
    /// Releases inner Connection Handle without calling disconnect.
    pub fn into_hdbc(self) -> HDbc<'env> {
        unsafe {
            let hdbc = ptr::read(&self.0);
            forget(self); // do not call drop
            hdbc
        }
    }
}

impl<'env, AC: AutocommitMode> Deref for Connected<'env, AC> {
    type Target = HDbc<'env>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'env, AC: AutocommitMode> DerefMut for Connected<'env, AC> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'env, AC: AutocommitMode> HDbcWrapper<'env> for Connected<'env, AC> {
    type Handle = Connected<'env, AC>;

    fn into_hdbc(self) -> HDbc<'env> {
        self.into_hdbc()
    }

    fn from_hdbc(hdbc: HDbc<'env>) -> Self::Handle {
        Connected(hdbc, PhantomData)
    }
}
