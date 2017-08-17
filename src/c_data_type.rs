use super::*;
use odbc_sys::*;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};

/// See [C Data Types in ODBC][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/c-data-types-in-odbc
pub unsafe trait CDataType {
    /// C Data type of the buffer returned by `mut_sql_ptr()`.
    fn c_data_type() -> SqlCDataType;
    /// Const sql pointer
    fn sql_ptr(&self) -> *const c_void;
    /// Pointer to the buffer in which should be filled with data.
    fn mut_sql_ptr(&mut self) -> SQLPOINTER;
    /// Length of the buffer returned by `mut_sql_ptr()` in bytes.
    fn buffer_len(&self) -> SQLLEN;
}

unsafe impl CDataType for [SQLCHAR] {
    fn c_data_type() -> SqlCDataType {
        SQL_C_BINARY
    }

    fn sql_ptr(&self) -> *const c_void {
        if self.len() == 0 {
            null()
        } else {
            self.as_ptr() as SQLPOINTER
        }
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        if self.len() == 0 {
            null_mut()
        } else {
            self.as_mut_ptr() as SQLPOINTER
        }
    }

    fn buffer_len(&self) -> SQLLEN {
        self.buf_len()
    }
}

unsafe impl CDataType for SQLSMALLINT {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SSHORT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for SQLUSMALLINT {
    fn c_data_type() -> SqlCDataType {
        SQL_C_USHORT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for SQLINTEGER {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SLONG
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for SQLUINTEGER {
    fn c_data_type() -> SqlCDataType {
        SQL_C_ULONG
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for f32 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_FLOAT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for f64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_DOUBLE
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for i8 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_STINYINT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for SQLCHAR {
    fn c_data_type() -> SqlCDataType {
        SQL_C_UTINYINT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for i64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SBIGINT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl CDataType for u64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_UBIGINT
    }

    fn sql_ptr(&self) -> *const c_void {
        let ptr: *const Self = self;
        ptr as *const c_void
    }

    fn mut_sql_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}
