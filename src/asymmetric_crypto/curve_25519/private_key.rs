use std::ops::{Add, Div, Mul, Sub};

use curve25519_dalek::Scalar;
use rand_chacha::rand_core::CryptoRngCore;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[cfg(feature = "ser")]
use crate::bytes_ser_de::{Deserializer, Serializable, Serializer};
use crate::{CBytes, CryptoCoreError, FixedSizeCBytes, RandomFixedSizeCBytes, SecretCBytes};

/// Asymmetric private key based on Curve25519.
///
/// This type wraps a scalar which is clamped to the curve.
/// `Curve25519PrivateKey` should not be used directly
/// but rather re-used as a base type for other final types on the curve
/// such as `X22519PrivateKey`.
#[derive(Hash, Clone, Debug, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct Curve25519PrivateKey(pub(crate) Scalar);

impl CBytes for Curve25519PrivateKey {}

impl FixedSizeCBytes<{ crypto_box::KEY_SIZE }> for Curve25519PrivateKey {
    fn to_bytes(&self) -> [u8; Self::LENGTH] {
        self.0.to_bytes()
    }

    fn try_from_bytes(slice: [u8; Self::LENGTH]) -> Result<Self, CryptoCoreError> {
        Ok(Self(Scalar::from_bits_clamped(slice)))
    }
}

impl RandomFixedSizeCBytes<{ crypto_box::KEY_SIZE }> for Curve25519PrivateKey {
    fn new<R: CryptoRngCore>(rng: &mut R) -> Self {
        let mut bytes = [0; Self::LENGTH];
        rng.fill_bytes(&mut bytes);
        Self(Scalar::from_bits_clamped(bytes))
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl SecretCBytes<{ crypto_box::KEY_SIZE }> for Curve25519PrivateKey {}

/// Key Serialization framework
#[cfg(feature = "ser")]
impl Serializable for Curve25519PrivateKey {
    type Error = CryptoCoreError;

    fn length(&self) -> usize {
        Self::LENGTH
    }

    fn write(&self, ser: &mut Serializer) -> Result<usize, Self::Error> {
        ser.write_array(self.as_bytes())
    }

    fn read(de: &mut Deserializer) -> Result<Self, Self::Error> {
        let bytes = de.read_array::<{ Self::LENGTH }>()?;
        Self::try_from_bytes(bytes)
    }
}

// Curve arithmetic

impl<'a> Add<&'a Curve25519PrivateKey> for &Curve25519PrivateKey {
    type Output = Curve25519PrivateKey;

    fn add(self, rhs: &Curve25519PrivateKey) -> Self::Output {
        Curve25519PrivateKey(self.0 + rhs.0)
    }
}

impl<'a> Sub<&'a Curve25519PrivateKey> for &Curve25519PrivateKey {
    type Output = Curve25519PrivateKey;

    fn sub(self, rhs: &Curve25519PrivateKey) -> Self::Output {
        Curve25519PrivateKey(self.0 - rhs.0)
    }
}

impl<'a> Mul<&'a Curve25519PrivateKey> for &Curve25519PrivateKey {
    type Output = Curve25519PrivateKey;

    fn mul(self, rhs: &Curve25519PrivateKey) -> Self::Output {
        Curve25519PrivateKey(self.0 * rhs.0)
    }
}

impl<'a> Div<&'a Curve25519PrivateKey> for &Curve25519PrivateKey {
    type Output = Curve25519PrivateKey;

    fn div(self, rhs: &Curve25519PrivateKey) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Curve25519PrivateKey(self.0 * rhs.0.invert())
    }
}
