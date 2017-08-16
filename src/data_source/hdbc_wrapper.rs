use super::*;

/// Implemented by `Connected` and `Unconnected`.
///
/// There are two implementations of this trait. These two implementations only decide wether or
/// not a `disconnect` should be executed on drop. This trait allows to handle them both in generic
/// code and makes them syntactically very similar to a direct use of `HDbc`.
pub trait HDbcWrapper<'env>: DerefMut<Target = HDbc<'env>> {
    type Handle: HDbcWrapper<'env>;
    fn into_hdbc(self) -> HDbc<'env>;
    fn from_hdbc(handle: HDbc<'env>) -> Self::Handle;
}
