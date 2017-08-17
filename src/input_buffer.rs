use odbc_sys::*;
use std::cmp::min;
use std::ptr::null;

/// This trait is intended to extend `usize` with safe downsize casts, so we can pass it as buffer
/// length into odbc functions.
pub trait InputBuffer {
    fn buf_len<T>(&self) -> T
    where
        T: BufferLength;
    fn buf_ptr(&self) -> *const u8;
}

impl InputBuffer for [u8] {
    fn buf_len<T>(&self) -> T
    where
        T: BufferLength,
    {
        T::from_usize(min(self.len(), T::max_value()))
    }

    fn buf_ptr(&self) -> *const u8 {
        if self.len() == 0 {
            null()
        } else {
            self.as_ptr()
        }
    }
}

pub trait BufferLength {
    fn max_value() -> usize;
    fn from_usize(len: usize) -> Self;
}

impl BufferLength for SQLSMALLINT {
    fn max_value() -> usize {
        Self::max_value() as usize
    }

    fn from_usize(len: usize) -> Self {
        len as Self
    }
}

impl BufferLength for SQLLEN {
    fn max_value() -> usize {
        Self::max_value() as usize
    }

    fn from_usize(len: usize) -> Self {
        len as Self
    }
}
