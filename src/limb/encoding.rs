//! Limb encoding

use super::{Limb, LimbUInt};
use crate::Encoding;

impl Encoding for Limb {
    const BIT_SIZE: usize = Self::BIT_SIZE;
    const BYTE_SIZE: usize = Self::BYTE_SIZE;

    #[cfg(target_pointer_width = "32")]
    type Repr = [u8; 4];
    #[cfg(target_pointer_width = "64")]
    type Repr = [u8; 8];

    #[inline]
    fn from_be_bytes(bytes: Self::Repr) -> Self {
        Limb(LimbUInt::from_be_bytes(bytes))
    }

    #[inline]
    fn from_le_bytes(bytes: Self::Repr) -> Self {
        Limb(LimbUInt::from_le_bytes(bytes))
    }

    #[inline]
    fn to_be_bytes(&self) -> Self::Repr {
        self.0.to_be_bytes()
    }

    #[inline]
    fn to_le_bytes(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn limb()(inner in any::<LimbUInt>()) -> Limb {
            Limb(inner)
        }
    }

    proptest! {
        #[test]
        fn roundtrip(a in limb()) {
            assert_eq!(a, Limb::from_be_bytes(a.to_be_bytes()));
            assert_eq!(a, Limb::from_le_bytes(a.to_le_bytes()));
        }
    }
}
