use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

/// A `Connection` is used to query and manipulate a data source.
///
/// * The state of the connection
/// * The current connection-level diagnostics
/// * The handles of statements and descriptors currently allocated on the connection
/// * The current settings of each connection attribute
///
/// See [Connection Handles in the ODBC Reference][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/connection-handles
#[derive(Debug)]
pub struct Connection<'env, S = Unconnected> {
    state: PhantomData<S>,
    handle: HDbc<'env>,
}

/// Indicates that a `Connection` is allocated, but not connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Unconnected {}
/// Indicates that a `Connection` is connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Connected {}

impl<'env, Any> Connection<'env, Any> {
    /// Provides access to the raw ODBC Connection Handle
    pub unsafe fn as_raw(&self) -> SQLHDBC {
        self.handle.as_raw()
    }

    /// Express state transiton
    fn transit<Other>(self) -> Connection<'env, Other> {
        Connection {
            state: PhantomData,
            handle: self.handle,
        }
    }
}

impl<'env> Connection<'env, Unconnected> {
    /// Allocates a new `Connection`. A `Connection` may not outlive its parent `Environment`.
    ///
    /// See [Allocating a Connection Handle ODBC][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/allocating-a-connection-handle-odbc
    pub fn with_parent<V>(parent: &'env Environment<V>) -> Return<Self>
    where
        V: Version,
    {
        HDbc::allocate(parent.as_henv()).map(|handle| {
            Connection {
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
    ) -> Return<Connection<'env, Connected>, Connection<'env, Unconnected>>
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

impl<'env> Connection<'env, Connected> {
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
    ) -> Return<Connection<'env, Unconnected>, Connection<'env, Connected>> {
        match self.handle.disconnect() {
            Success(()) => Success(self.transit()),
            Info(()) => Info(self.transit()),
            Error(()) => Error(self.transit()),
        }
    }
}

impl<'env, S> Diagnostics for Connection<'env, S> {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn {
        self.handle.diagnostics(rec_number, message_text)
    }
}
