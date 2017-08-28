use odbc_sys::*;

/// Describes a column or parameter type.
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    /// Fixed sized single byte character data
    Char(SQLULEN),
    /// Exact numerical, with (Precision, Scale)
    Numeric(SQLULEN, SQLSMALLINT),
    /// Exact numerical, with (Precision, Scale)
    Decimal(SQLULEN, SQLSMALLINT),
    /// Integer numerical with precision 10
    Integer,
    /// Small integer numerical with precision 5
    SmallInt,
    /// Approximate numerical with precision 15
    Float,
    /// Approximate numerical with precison 7
    Real,
    /// Approximate numerical with precision 15
    Double,
    /// Variadic sized single byte character data
    Varchar(SQLULEN),
}

/// Determines the type stored at the data source
///
/// See [Data Types][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/appendixes/appendix-d-data-types
impl DataType {
    /// See [SQL Data Types][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/appendixes/sql-data-types
    pub fn sql_data_type(&self) -> SqlDataType {
        use DataType::*;
        match *self {
            Char(_) => SQL_CHAR,
            Numeric(_, _) => SQL_NUMERIC,
            Decimal(_, _) => SQL_DECIMAL,
            Integer => SQL_INTEGER,
            SmallInt => SQL_SMALLINT,
            Float => SQL_FLOAT,
            Real => SQL_REAL,
            Double => SQL_DOUBLE,
            Varchar(_) => SQL_VARCHAR,
        }
    }

    /// See [Column Size][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/appendixes/column-size
    pub fn column_size(&self) -> SQLULEN {
        use DataType::*;
        match *self {
            Numeric(precision, _) |
            Decimal(precision, _) => precision,
            Integer => 10,
            SmallInt => 5,
            Float | Double => 15,
            Real => 7,
            Char(len) | Varchar(len) => len,
        }
    }

    /// See [Decimal Digits][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/appendixes/decimal-digits
    pub fn decimal_digits(&self) -> SQLSMALLINT {
        use DataType::*;
        match *self {
            Char(_) | Integer | Float | Real | Double | Varchar(_) => 0,
            Numeric(_, scale) |
            Decimal(_, scale) => scale,
            SmallInt => 5,
        }
    }
}
