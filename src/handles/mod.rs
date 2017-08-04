//! This module is the first layer of abstraction around the raw handles returned by the ODBC C API
//! It contains wrapper a type for each handle. It ensures that each handle is always valid. Yet it
//! does nothing to ensure that the methods invoked are valid for the state. This means that ODBC
//! function sequence errors may occur if these abstractions are used incorrectly.
//!
//! #Design
//!
//! Why having these `inner` wrapper types instead of doing everything within the outer wrapper
//! types? Besides the added clarity a major rational for these are the `Drop` implementations.
//! Since the outer types (i.e. `Environment`, `Connection`) model their state within the type
//! system they get destroyed and created a lot during the lifetime of one actual ODBC Environment
//! or Connection. It therefore more sensible to mangage allocating and freeing handles within
//! those instances will live just as long as the actual datastructures managed by ODBC.

pub use self::henv::HEnv;
pub use self::hdbc::HDbc;
use super::{Return, SqlStr};
use odbc_sys::*;

mod henv;
mod hdbc;

pub unsafe trait Handle {
    unsafe fn handle(&self) -> SQLHANDLE;
    fn handle_type() -> HandleType;
}
