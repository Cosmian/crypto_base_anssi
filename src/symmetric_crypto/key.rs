//! Defines a symmetric key object of variable size.

use core::{hash::Hash, ops::Deref};
use std::ops::DerefMut;

use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use crate::{
    reexport::rand_core::CryptoRngCore, CBytes, CryptoCoreError, FixedSizeCBytes,
    RandomFixedSizeCBytes, Secret, SecretCBytes,
};

/// A type that holds symmetric key of a fixed  size.
///
/// It is internally built using an array of bytes of the given length.
#[derive(Debug, Hash, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SymmetricKey<const LENGTH: usize>(Secret<LENGTH>);

impl<const LENGTH: usize> CBytes for SymmetricKey<LENGTH> {}

impl<const LENGTH: usize> FixedSizeCBytes<LENGTH> for SymmetricKey<LENGTH> {
    fn to_bytes(&self) -> [u8; LENGTH] {
        let mut dest = [0; LENGTH];
        self.0.to_unprotected_bytes(&mut dest);
        dest
    }

    fn try_from_bytes(mut bytes: [u8; LENGTH]) -> Result<Self, CryptoCoreError> {
        Ok(Self(Secret::from_unprotected_bytes(&mut bytes)))
    }
}

impl<const LENGTH: usize> RandomFixedSizeCBytes<LENGTH> for SymmetricKey<LENGTH> {
    #[inline(always)]
    fn new<R: CryptoRngCore>(rng: &mut R) -> Self {
        Self(Secret::random(rng))
    }

    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

/// The symmetric key is a secret and must be zeroized.
impl<const LENGTH: usize> SecretCBytes<LENGTH> for SymmetricKey<LENGTH> {}

impl<const LENGTH: usize> Deref for SymmetricKey<LENGTH> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const LENGTH: usize> DerefMut for SymmetricKey<LENGTH> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const LENGTH: usize> Default for SymmetricKey<LENGTH> {
    fn default() -> Self {
        Self(Secret::new())
    }
}

impl<const LENGTH: usize> From<SymmetricKey<LENGTH>> for Zeroizing<Vec<u8>> {
    fn from(value: SymmetricKey<LENGTH>) -> Self {
        Zeroizing::new(value.0.to_vec())
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        reexport::rand_core::SeedableRng, symmetric_crypto::key::SymmetricKey, CsRng,
        RandomFixedSizeCBytes,
    };

    const KEY_LENGTH: usize = 32;

    #[test]
    fn test_key() {
        let mut cs_rng = CsRng::from_entropy();
        let key_1 = SymmetricKey::<KEY_LENGTH>::new(&mut cs_rng);
        assert_eq!(KEY_LENGTH, key_1.len());
        let key_2 = SymmetricKey::new(&mut cs_rng);
        assert_eq!(KEY_LENGTH, key_2.len());
        assert_ne!(key_1, key_2);
    }
}
