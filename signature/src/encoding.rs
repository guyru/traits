//! Encoding support.

use crate::Error;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Support for decoding/encoding signatures as bytes.
pub trait SignatureEncoding:
    Clone + Sized + for<'a> TryFrom<&'a [u8], Error = Error> + Into<Self::Repr>
{
    /// Byte representation of a signature.
    type Repr: 'static + AsRef<[u8]> + Clone + Send + Sync;

    /// Encode signature as its byte representation.
    fn to_bytes(&self) -> Self::Repr {
        self.clone().into()
    }

    /// Encode signature as a byte vector.
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn to_vec(&self) -> Vec<u8> {
        self.to_bytes().as_ref().to_vec()
    }
}
