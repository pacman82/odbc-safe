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

pub extern crate odbc_sys as sys;

pub use c_data_type::CDataType;
pub use data_source::{HDbcWrapper, DataSource, Unconnected, Connected, AutocommitOff, AutocommitOn, AutocommitMode};
pub use data_type::DataType;
pub use diagnostics::{Diagnostics, DiagResult};
pub use environment::Environment;

pub use handles::Handle;
pub use indicator::Indicator;
pub use return_::{Return, Success, Info, Error};
pub use return_option::ReturnOption;
pub use sql_str::SqlStr;
pub use statement::{Statement, NoCursor, Open, Positioned, Unprepared, Prepared, CursorState};
pub use version::{NoVersion, Odbc3, Odbc3m8};
pub use version::Version;

use output_buffer::OutputBuffer;
use handles::{HEnv, HDbc, HStmt};

mod version;
mod return_;
mod return_option;
mod sql_str;
mod handles;
mod diagnostics;
mod environment;
mod data_source;
mod statement;
mod c_data_type;
mod indicator;
mod data_type;
mod output_buffer;

/// `Connection` can be used as a shorthand for a `DataSource` in `Connected` state.
pub type Connection<'env, AC> = DataSource<'env, Connected<'env, AC>>;

/// Shorthand for `Statements` in `Open` state.
pub type ResultSet<'con, 'param, 'col, P> = Statement<'con, 'param, 'col, Open, P>;
