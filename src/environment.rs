use super::*;
use sys::*;
use std::marker::PhantomData;

/// An `Environment` is a global context, in which to access data.
///
/// Associated with an `Environment` is any information that is global in nature, such as:
///
/// * The `Environment`'s state
/// * The current environment-level diagnostics
/// * The handles of connections currently allocated on the environment
/// * The current stetting of each environment attribute
///
/// See: [Environment Handles in the ODBC Reference][1]
/// [1]: https://docs.microsoft.com/sql/odbc/reference/develop-app/environment-handles
#[derive(Debug)]
pub struct Environment<V> {
    version: PhantomData<V>,
    /// Invariant: Should always point to a valid ODBC Environment with Version declared as V or
    /// `NoVersion`
    handle: HEnv,
}

impl<V> Environment<V> {
    /// Provides access to the raw ODBC environment handle.
    pub fn as_raw(&self) -> SQLHENV {
        self.handle.as_raw()
    }

    /// Express state transiton
    fn transit<Other>(self) -> Environment<Other> {
        Environment {
            version: PhantomData,
            handle: self.handle,
        }
    }
}

impl<V: Version> Environment<V> {
    /// Used by `Connection`s constructor
    pub(crate) fn as_henv(&self) -> &HEnv {
        &self.handle
    }

    /// Fills buffers with information about the available datasources
    ///
    /// A 32 / 64 Bit Application will only return information about either 32 or 64 Bit
    /// DataSources.
    ///
    /// # Returns
    ///
    /// (server_name_length, description_length)
    ///
    /// See [SQLDataSources][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldatasources-function
    pub fn data_sources(
        &mut self,
        direction: FetchOrientation,
        server_name: &mut [u8],
        description: &mut [u8],
    ) -> ReturnOption<(SQLSMALLINT, SQLSMALLINT)> {
        self.handle
            .data_sources(direction, server_name, description)
    }

    /// Fills buffers with information about the available datasources
    ///
    /// A 32 / 64 Bit Application will only return information about either 32 or 64 Bit
    /// DataSources.
    ///
    /// # Returns
    ///
    /// (description_length, attributes_length)
    ///
    /// See [SQLDrivers][1]
    /// [1]: https://docs.microsoft.com/sql/odbc/reference/syntax/sqldrivers-function
    pub fn drivers(
        &mut self,
        direction: FetchOrientation,
        description: &mut [u8],
        attributes: &mut [u8],
    ) -> ReturnOption<(SQLSMALLINT, SQLSMALLINT)> {
        self.handle.drivers(direction, description, attributes)
    }
}

impl Environment<NoVersion> {
    /// Allocates a new `Environment`
    pub fn new() -> Return<Self> {
        HEnv::allocate().map(|handle| {
            Environment {
                version: PhantomData,
                handle: handle,
            }
        })
    }

    /// Before an application allocates a connection which specification it follows. Currently
    /// these bindings only support ODBC 3.x.
    ///
    /// It is valid to specify ODBC 3.x even then connecting to an ODBC 2.x driver. Applications
    /// must however avoid calling 3.x functionality on 2.x drivers. Since drivers are connected at
    /// runtime, these kind of errors can not be catched by the type system.
    pub fn declare_version<V: Version>(mut self) -> Return<Environment<V>, Environment<NoVersion>> {
        let result = self.handle.declare_version(V::constant());
        match result {
            Success(()) => Success(self.transit()),
            Info(()) => Success(self.transit()),
            Error(()) => Success(self.transit()),
        }
    }

    /// Before an application allocates a connection which specification it follows. Currently
    /// these bindings only support ODBC 3.x.
    ///
    /// It is valid to specify ODBC 3.x even then connecting to an ODBC 2.x driver. Applications
    /// must however avoid calling 3.x functionality on 2.x drivers. Since drivers are connected at
    /// runtime, these kind of errors can not be catched by the type system.
    ///
    /// This method is a shorthand for `declare_version::<Odbc3m8>`.
    pub fn declare_version_3_8(self) -> Return<Environment<Odbc3m8>, Environment<NoVersion>> {
        self.declare_version()
    }

    /// Before an application allocates a connection which specification it follows. Currently
    /// these bindings only support ODBC 3.x.
    ///
    /// It is valid to specify ODBC 3.x even then connecting to an ODBC 2.x driver. Applications
    /// must however avoid calling 3.x functionality on 2.x drivers. Since drivers are connected at
    /// runtime, these kind of errors can not be catched by the type system.
    ///
    /// This method is a shorthand for `declare_version::<Odbc3>`.
    pub fn declare_version_3(self) -> Return<Environment<Odbc3>, Environment<NoVersion>> {
        self.declare_version()
    }
}

impl<V> Diagnostics for Environment<V> {
    fn diagnostics(
        &self,
        rec_number: SQLSMALLINT,
        message_text: &mut [SQLCHAR],
    ) -> ReturnOption<DiagResult> {
        self.handle.diagnostics(rec_number, message_text)
    }
}
