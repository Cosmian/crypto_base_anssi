use crate::KeyTrait;
use core::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};
use rand_core::{CryptoRng, RngCore};
use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod curve25519;

pub trait DhKeyPair<const PK_LENGTH: usize, const SK_LENGTH: usize>:
    Debug + PartialEq + Eq + Send + Sync + Sized + Clone + Zeroize + ZeroizeOnDrop
where
    Self::PublicKey: From<Self::PrivateKey> + Add + Mul<Self::PrivateKey, Output = Self::PublicKey>,
    Self::PrivateKey: Add + Sub + Mul + Div,
{
    /// This is needed to be able to use `{ MyKeyPair::PK_LENGTH }`
    /// as associated constant
    const PK_LENGTH: usize = PK_LENGTH;

    /// This is needed to be able to use `{ MyKeyPair::SK_LENGTH }`
    /// as associated constant
    const SK_LENGTH: usize = SK_LENGTH;

    /// Public key
    type PublicKey: KeyTrait<PK_LENGTH>;

    /// Secret key
    type PrivateKey: KeyTrait<SK_LENGTH>;

    /// Create a new key pair
    #[must_use]
    fn new<R: RngCore + CryptoRng>(rng: &mut R) -> Self;

    /// Return a reference to the public key.
    fn public_key(&self) -> &Self::PublicKey;

    /// Return a reference to the secret key.
    fn private_key(&self) -> &Self::PrivateKey;
}
