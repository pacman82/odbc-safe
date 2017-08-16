use super::*;
use std::ops::Deref;

/// An `HDbc` with the additional invariant of being 'allocated', but not 'connected'.
#[derive(Debug)]
pub struct Unconnected<'env>(HDbc<'env>);

impl<'env> Deref for Unconnected<'env> {
    type Target = HDbc<'env>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'env> DerefMut for Unconnected<'env> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'env> HDbcWrapper<'env> for Unconnected<'env> {
    type Handle = Unconnected<'env>;

    fn into_hdbc(self) -> HDbc<'env> {
        self.0
    }

    fn from_hdbc(hdbc: HDbc<'env>) -> Self::Handle {
        Unconnected(hdbc)
    }
}
