use odbc_sys::*;
use std::cmp::min;
use std::mem::size_of;

/// Indicates a type which can be used as a target in `Statement::get_data()`.
pub unsafe trait Target {
    /// C Data type of the buffer returned by `value_ptr()`.
    fn c_data_type() -> SqlCDataType;
    /// Pointer to the buffer in which should be filled with data.
    fn value_ptr(&mut self) -> SQLPOINTER;
    /// Length of the buffer returned by `value_ptr()` in bytes.
    fn buffer_len(&self) -> SQLLEN;
}

unsafe impl Target for [SQLCHAR] {
    fn c_data_type() -> SqlCDataType {
        SQL_C_BINARY
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        self.as_mut_ptr() as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        min(self.len() as SQLLEN, SQLLEN::max_value())
    }
}

unsafe impl Target for SQLSMALLINT {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SSHORT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for SQLUSMALLINT {
    fn c_data_type() -> SqlCDataType {
        SQL_C_USHORT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for SQLINTEGER {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SLONG
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for SQLUINTEGER {
    fn c_data_type() -> SqlCDataType {
        SQL_C_ULONG
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for f32 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_FLOAT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for f64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_DOUBLE
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for i8 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_STINYINT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for SQLCHAR {
    fn c_data_type() -> SqlCDataType {
        SQL_C_UTINYINT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for i64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_SBIGINT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}

unsafe impl Target for u64 {
    fn c_data_type() -> SqlCDataType {
        SQL_C_UBIGINT
    }

    fn value_ptr(&mut self) -> SQLPOINTER {
        let ptr: *mut Self = self;
        ptr as SQLPOINTER
    }

    fn buffer_len(&self) -> SQLLEN {
        size_of::<Self>() as SQLLEN
    }
}