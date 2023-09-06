mod public_key;

pub use public_key::{X25519PublicKey, X25519_PUBLIC_KEY_LENGTH};
use sha2::{Digest, Sha512};

use crate::{Ed25519PrivateKey, RandomFixedSizeCBytes, CURVE_25519_PRIVATE_KEY_LENGTH};

use super::private_key::Curve25519Secret;

pub type X25519PrivateKey = Curve25519Secret;

impl X25519PrivateKey {
    /// Convert the ED25519 private key to an X25519 private key
    pub fn from_ed25519_private_key(sk: &Ed25519PrivateKey) -> Self {
        // see ed25519_dalek::ExpandedSecretKey::to_curve25519_private_key
        // The spec-compliant way to define an expanded secret key. This computes `SHA512(sk)`, clamps the
        // first 32 bytes and uses it as a scalar, and uses the second 32 bytes as a domain separator for
        // hashing.
        // We recover the same first 32 bytes to generate the scalar for X25519
        let hash = Sha512::default().chain_update(sk.as_bytes()).finalize();
        let mut seed = [0_u8; CURVE_25519_PRIVATE_KEY_LENGTH];
        seed.copy_from_slice(&hash[0..CURVE_25519_PRIVATE_KEY_LENGTH]);
        Self(seed)
    }
}
