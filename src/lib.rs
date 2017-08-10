//! A crate for writing odbc clients in safe Rust
//!
//! This crate is only concerned about providing safe bindings to ODBC. It does not try to provide
//! convinience or idiomatic usage on top. This is left to higher level crates.
//!
//! # Desgin decisions:
//! * Constructing a child is not considered to mutate the parent. This is necessare so a parent
//!   handle can be referenced by several child handles.
//! * Any transition in the ODBC State machine is modelled in the type system. This prevents
//!   Function Sequence errors. See [ODBC State Transition Tables][1]
//! [1]: https://docs.microsoft.com/sql/odbc/reference/appendixes/appendix-b-odbc-state-transition-tables
#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unused_import_braces, unused_qualifications
)]

extern crate odbc_sys;

pub use return_::{Return, Success, Info, Error};
pub use return_nodata::ReturnNoData;
pub use environment::Environment;
pub use connection::{Connection, Connected, Unconnected};
pub use statement::{Statement, HasResult};
pub use version::{NoVersion, Odbc3, Odbc3m8};
pub use sql_str::SqlStr;
pub use diagnostics::{Diagnostics, DiagResult, DiagReturn};
pub use target::Target;
pub use indicator::Indicator;
pub use version::Version;
use handles::{Handle, HEnv, HDbc, HStmt};

mod version;
mod return_;
mod return_nodata;
mod sql_str;
mod handles;
mod diagnostics;
mod environment;
mod connection;
mod statement;
mod target;
mod indicator;
