use odbc_sys::*;

pub trait Version {
    fn constant() -> OdbcVersion;
}

/// Used to indicate that the ODBC environments version is not yet declared
#[derive(Debug,Clone,Copy)]
pub struct NoVersion;

/// Used to declare ODBC 3 specifications.
#[derive(Debug,Clone,Copy)]
pub struct Odbc3;

/// Used to declare ODBC 3.8 specifications.
#[derive(Debug,Clone,Copy)]
pub struct Odbc3m8;

impl Version for Odbc3 {
    fn constant() -> OdbcVersion {
        SQL_OV_ODBC3
    }
}

impl Version for Odbc3m8 {
    fn constant() -> OdbcVersion {
        SQL_OV_ODBC3_80
    }
}
