use super::*;
use odbc_sys::*;
use std::marker::PhantomData;

/// An `Environment` is a global context, in which to access data.
///
/// Associated with an `Environment` is any information that is global in nature, such as:
///
/// * The `Environment`'s state
/// * The current environment-level diagnostics
/// * The handles of connections currently allocated on the environment
/// * The current stetting of each environment attribute
#[derive(Debug)]
pub struct Environment<V> {
    version: PhantomData<V>,
    /// Invariant: Should always point to a valid ODBC Environment with Version declared as V or
    /// `NoVersion`
    handle: HEnv,
}

impl Environment<NoVersion> {
    /// Allocates a new `Environment`
    pub fn allocate() -> Return<Self> {
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
    pub fn declare_version<V: Version>(mut self) -> Return<Environment<V>> {
        let result = self.handle.declare_version(V::constant());
        result.map(move |()| Environment{version: PhantomData, handle: self.handle})
    }

    /// Before an application allocates a connection which specification it follows. Currently
    /// these bindings only support ODBC 3.x.
    ///
    /// It is valid to specify ODBC 3.x even then connecting to an ODBC 2.x driver. Applications
    /// must however avoid calling 3.x functionality on 2.x drivers. Since drivers are connected at
    /// runtime, these kind of errors can not be catched by the type system.
    ///
    /// This method is a shorthand for `declare_version::<Odbc3m8>`.
    pub fn declare_version_3_8(self) -> Return<Environment<Odbc3m8>> {
        self.declare_version::<Odbc3m8>()
    }
}

impl<V: Version> Environment<V> {
    /// Used by `Connection`s constructor
    pub(crate) fn henv(&self) -> &HEnv {
        &self.handle
    }
}

impl<V> Environment<V> {
    /// Provides access to the raw ODBC environment handle.
    pub unsafe fn handle(&self) -> SQLHENV {
        self.handle.handle()
    }
}

impl<V> Diagnostics for Environment<V> {
    fn diagnostics(&self, rec_number: SQLSMALLINT, message_text: &mut [SQLCHAR]) -> DiagReturn {
        self.handle.diagnostics(rec_number, message_text)
    }
}
