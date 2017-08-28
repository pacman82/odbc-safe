use odbc_sys::*;
use std::cmp::min;
use std::ptr::null_mut;

/// This trait is intended to extend `usize` with safe downsize casts, so we can pass it as buffer
/// length into odbc functions.
pub trait OutputBuffer {
    fn buf_len<T>(&self) -> T
    where
        T: BufferLength;
    fn mut_buf_ptr(&mut self) -> *mut u8;
}

impl OutputBuffer for [u8] {
    fn buf_len<T>(&self) -> T
    where
        T: BufferLength,
    {
        T::from_usize(min(self.len(), T::max_value()))
    }

    fn mut_buf_ptr(&mut self) -> *mut u8 {
        if self.is_empty() {
            null_mut()
        } else {
            self.as_mut_ptr()
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
