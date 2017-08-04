use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

/// A `Connection` is used to query and manipulate a data source.
///
/// A connection consists of a driver and a data source. A connection handle identifies each
/// connection. The connection handle defines not only which driver to use but which data source to
/// use with that driver. Within a segment of code that implements ODBC (the Driver Manager or a
/// driver), the connection handle identifies a structure that contains connection information,
/// such as the following:
///
/// * The state of the connection
/// * The current connection-level diagnostics
/// * The handles of statements and descriptors currently allocated on the connection
/// * The current settings of each connection attribute
///
/// ODBC does not prevent multiple simultaneous `Connection`s, if the driver supports them.
/// Therefore, in a particular ODBC environment, multiple `Connection`s might point to a variety of
/// drivers and data sources, to the same driver and a variety of data sources, or even to multiple
/// `Connection`s to the same driver and data source. Some drivers limit the number of active
/// `Connection`s they support; `Connection`s are primarily used when connecting to the data source
/// , disconnecting from the data source, getting information about the driver and data source,
/// retrieving diagnostics, and performing transactions. They are also used when setting and
/// getting connection attributes and when getting the native format of an SQL statement.
#[derive(Debug)]
pub struct Connection<'env, S = Allocated> {
    state: PhantomData<S>,
    handle: HDbc<'env>,
}

/// Indicates that a `Connection` is allocated, but not yet connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Allocated {}
/// Indicates that a `Connection` is connected to a Data Source.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Connected {}

impl<'env, Any> Connection<'env, Any> {
    /// Express state transiton
    fn transit<Other>(self) -> Connection<'env, Other> {
        Connection {
            state: PhantomData,
            handle: self.handle,
        }
    }
}

impl<'env> Connection<'env, Allocated> {
    /// Allocates a new `Connection`. A `Connection` may not outlive its parent `Environment`.
    ///
    /// The Driver Manager allocates a structure in which to store information about the statement.
    /// The Driver Manager does not call `SQLAllocHandle` in the driver at this time because it
    /// does not know which driver to call. It delays calling `SQLAllocHandle` in the driver until
    /// the application calls a function to connect to a data source.
    ///
    /// It is important to note that allocating a `Connection`is not the same as loading a
    /// driver. The driver is not loaded until a connection function is called. Thus, after
    /// allocating a `Connection` and before connecting to the driver or data source, most methods
    /// can not be called by the application. An attempt to do so will result in a compiler error.
    pub fn with_parent<V>(parent: &'env Environment<V>) -> Return<Self>
    where
        V: Version,
    {
        HDbc::allocate(parent.henv()).map(|handle| {
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
    pub fn connect<DSN, U, P>(
        mut self,
        data_source_name: &DSN,
        user: &U,
        pwd: &P,
    ) -> Return<Connection<'env, Connected>, Connection<'env, Allocated>>
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

    /// When an application has finished using a data source, it calls `disconnect`. `disconnect`
    /// disconnects the driver from the data source.
    ///
    /// The application also can reuse the connection, either to connect to a different data source
    /// or reconnect to the same data source. The decision to remain connected, as opposed to
    /// disconnecting and reconnecting later, requires that the application writer consider the
    /// relative costs of each option; both connecting to a data source and remaining connected can
    /// be relatively costly depending on the connection medium. In making a correct tradeoff, the
    /// application must also make assumptions about the likelihood and timing of further
    /// operations on the same data source.
    pub fn disconnect(
        mut self,
    ) -> Return<Connection<'env, Allocated>, Connection<'env, Connected>> {
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
