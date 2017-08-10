use odbc_sys::*;

/// Type indicates an ODBC Version
pub unsafe trait Version {
    /// The `SQL_ATTR_ODBC_VERSION` used with `SQLSetEnvAttr`
    fn constant() -> OdbcVersion;
}

/// Used to indicate that the ODBC environments version is not yet declared
#[derive(Debug, Clone, Copy)]
pub struct NoVersion;

/// Used to declare ODBC 3 specifications.
#[derive(Debug, Clone, Copy)]
pub struct Odbc3;

/// Used to declare ODBC 3.8 specifications.
#[derive(Debug, Clone, Copy)]
pub struct Odbc3m8;

unsafe impl Version for Odbc3 {
    fn constant() -> OdbcVersion {
        SQL_OV_ODBC3
    }
}

unsafe impl Version for Odbc3m8 {
    fn constant() -> OdbcVersion {
        SQL_OV_ODBC3_80
    }
}
