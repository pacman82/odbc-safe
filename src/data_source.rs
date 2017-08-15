use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

/// A `DataSource` is used to query and manipulate a data source.
///
/// * The state of the connection
/// * The current connection-level diagnostics
/// * The handles of statements and descriptors currently allocated on the connection
/// * The current settings of each connection attribute
///
/// See [Connection Handles in the ODBC Reference][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/connection-handles
#[derive(Debug)]
pub struct DataSource<'env, S = Unconnected> {
    state: PhantomData<S>,
    handle: HDbc<'env>,
}

/// Indicates that a `DataSource` is allocated, but not connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Unconnected {}
/// Indicates that a `DataSource` is connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Connected {}

impl<'env, Any> DataSource<'env, Any> {
    /// Consumes the `DataSource`, returning the wrapped raw `SQLHDBC`
    ///
    /// Leaks the Connection Handle. This is usually done in order to pass ownership from Rust to
    /// another language. After calling this method, the caller is responsible for invoking
    /// `SQLFreeHandle`.
    pub fn into_raw(self) -> SQLHDBC {
        self.handle.into_raw()
    }

    /// Provides access to the raw ODBC Connection Handle
    pub fn as_raw(&self) -> SQLHDBC {
        self.handle.as_raw()
    }

    /// May only be invoked with a valid Statement Handle which has been allocated using
    /// `SQLAllocHandle`. Special care must be taken that the Connection Handle passed is in a
    /// State which matches the type.
    pub unsafe fn from_raw(raw: SQLHDBC) -> Self {
        DataSource {
            handle: HDbc::from_raw(raw),
            state: PhantomData,
        }
    }

    /// Express state transiton
    fn transit<Other>(self) -> DataSource<'env, Other> {
        DataSource {
            state: PhantomData,
            handle: self.handle,
        }
    }
}

impl<'env> DataSource<'env, Unconnected> {
    /// Allocates a new `DataSource`. A `DataSource` may not outlive its parent `Environment`.
    ///
    /// See [Allocating a Connection Handle ODBC][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/allocating-a-connection-handle-odbc
    pub fn with_parent<V>(parent: &'env Environment<V>) -> Return<Self>
    where
        V: Version,
    {
        HDbc::allocate(parent.as_henv()).map(|handle| {
            DataSource {
                state: PhantomData,
                handle: handle,
            }
        })
    }

    /// Establishes connections to a driver and a data source. The connection handle references
    /// storage of all information about the connection to the data source, including status,
    /// transaction state, and error information.
    ///
    /// * See [Connecting with SQLConnect][1]
    /// * See [SQLConnectFunction][2]
    ///
    /// # State transition
    /// On success this method changes the Connection handles state from `Allocated` to `Connected`
    /// . Since this state change is expressed in the type system, the method consumes self. And
    /// returns a new instance in the result type.
    ///
    /// # Arguments
    ///
    /// * `data_source_name` - Data source name. The data might be located on the same computer as
    ///                        the program, or on another computer somewhere on a network.
    /// * `user` - User identifier.
    /// * `pwd` - Authenticatien string (typically the password).
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlconnect-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqlconnect-function
    pub fn connect<DSN, U, P>(
        mut self,
        data_source_name: &DSN,
        user: &U,
        pwd: &P,
    ) -> Return<DataSource<'env, Connected>, DataSource<'env, Unconnected>>
    where
        DSN: SqlStr + ?Sized,
        U: SqlStr + ?Sized,
        P: SqlStr + ?Sized,
    {
        match self.handle.connect(data_source_name, user, pwd) {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }
}

impl<'env> DataSource<'env, Connected> {
    /// Used by `Statement`s constructor
    pub(crate) fn as_hdbc(&self) -> &HDbc {
        &self.handle
    }

    /// When an application has finished using a data source, it calls `disconnect`. `disconnect`
    /// disconnects the driver from the data source.
    ///
    /// * See [Disconnecting from a Data Source or Driver][1]
    /// * See [SQLDisconnect Function][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/disconnecting-from-a-data-source-or-driver
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldisconnect-function
    pub fn disconnect(
        mut self,
    ) -> Return<DataSource<'env, Unconnected>, DataSource<'env, Connected>> {
        match self.handle.disconnect() {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }
}

impl<'env, S> Diagnostics for DataSource<'env, S> {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> ReturnOption<DiagResult> {
        self.handle.diagnostics(rec_number, message_text)
    }
}
