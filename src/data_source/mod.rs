pub use self::connected::Connected;
pub use self::hdbc_wrapper::HDbcWrapper;
pub use self::unconnected::Unconnected;
use super::*;
use odbc_sys::*;
use std::ops::DerefMut;

mod connected;
mod unconnected;
mod hdbc_wrapper;

/// A `DataSource` is used to query and manipulate a data source.
///
/// * The state of the connection
/// * The current connection-level diagnostics
/// * The handles of statements and descriptors currently allocated on the connection
/// * The current settings of each connection attribute
///
/// # States
///
/// A `DataSource` is in one of two states `Connected` or `Unconnected`. These are modeled in the
/// type at compile time. Every new `DataSource` starts out as `Unconnected`. To execute a query it
/// needs to be `Connected`. You can achieve this by calling e.g. `connect` and capture the result
/// in a new binding which will be of type `DataSource::<'env, Connected<'env>>`.
///
/// See [Connection Handles in the ODBC Reference][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/connection-handles
#[derive(Debug)]
pub struct DataSource<'env, S: HDbcWrapper<'env> = Unconnected<'env>> {
    /// Connection handle. Either `HDbc` for `Unconnected` or `Connected` for `Connected`.
    handle: S::Handle,
}

impl<'env, Any> DataSource<'env, Any>
where
    Any: HDbcWrapper<'env>,
{
    /// Consumes the `DataSource`, returning the wrapped raw `SQLHDBC`
    ///
    /// Leaks the Connection Handle. This is usually done in order to pass ownership from Rust to
    /// another language. After calling this method, the caller is responsible for invoking
    /// `SQLFreeHandle`.
    pub fn into_raw(self) -> SQLHDBC {
        self.handle.into_hdbc().into_raw()
    }

    /// Provides access to the raw ODBC Connection Handle
    pub fn as_raw(&self) -> SQLHDBC {
        self.handle.as_raw()
    }

    /// May only be invoked with a valid Statement Handle which has been allocated using
    /// `SQLAllocHandle`. Special care must be taken that the Connection Handle passed is in a
    /// State which matches the type.
    pub unsafe fn from_raw(raw: SQLHDBC) -> Self {
        DataSource { handle: Any::from_hdbc(HDbc::from_raw(raw)) }
    }

    /// Express state transiton
    fn transit<Other: HDbcWrapper<'env>>(self) -> DataSource<'env, Other> {
        DataSource { handle: Other::from_hdbc(self.handle.into_hdbc()) }
    }
}

impl<'env> DataSource<'env, Unconnected<'env>> {
    /// Allocates a new `DataSource`. A `DataSource` may not outlive its parent `Environment`.
    ///
    /// See [Allocating a Connection Handle ODBC][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/allocating-a-connection-handle-odbc
    pub fn with_parent<V>(parent: &'env Environment<V>) -> Return<Self>
    where
        V: Version,
    {
        HDbc::allocate(parent.as_henv()).map(|handle| {
            DataSource { handle: Unconnected::from_hdbc(handle) }
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
    ) -> Return<Connection<'env>, DataSource<'env, Unconnected<'env>>>
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

    /// Connects to a data source using a connection string.
    ///
    /// For the syntax regarding the connections string see [SQLDriverConnect][1]. This method is
    /// equivalent of calling `odbc_sys::SQLDriverConnect` with the `SQL_DRIVER_NOPROMPT` parameter.
    ///
    /// See [Choosing a Data Source or Driver][2]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldriverconnect-function
    /// [2]: https://docs.microsoft.com/sql/odbc/reference/develop-app/choosing-a-data-source-or-driver
    pub fn connect_with_connection_string<C>(
        mut self,
        connection_string: &C,
    ) -> Return<Connection<'env>, Self>
    where
        C: SqlStr + ?Sized,
    {
        // We do not care for now.
        let mut out_connection_string = [];
        match self.handle.driver_connect(
            connection_string,
            &mut out_connection_string,
            SQL_DRIVER_NOPROMPT,
        ) {
            Success(_) => Success(self.transit()),
            Info(_) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }
}

impl<'env> Connection<'env> {
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
    pub fn disconnect(mut self) -> Return<DataSource<'env, Unconnected<'env>>, Connection<'env>> {
        match self.handle.disconnect() {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }

    /// `true` if the data source is set to READ ONLY mode, `false` otherwise.
    pub fn is_read_only(&mut self) -> Return<bool>{
        self.handle.is_read_only()
    }
}

impl<'env, S> Diagnostics for DataSource<'env, S>
where
    S: HDbcWrapper<'env>,
{
    fn diagnostics(
        &self,
        rec_number: SQLSMALLINT,
        message_text: &mut [SQLCHAR],
    ) -> ReturnOption<DiagResult> {
        self.handle.diagnostics(rec_number, message_text)
    }
}
