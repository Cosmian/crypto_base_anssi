pub use ed25519_dalek::{SecretKey as EdSecretKey, VerifyingKey as EdPublicKey};

use crate::asymmetric_crypto::curve_25519::curve_secret::Curve25519Secret;

pub type Ed25519PrivateKey = Curve25519Secret;
