//! Key management: unified key types and serialization helpers.
//!
//! This module provides `KeyBundle` — a single owner's set of keys across
//! all algorithms supported by this library.

use crate::asymmetric::rsa::RsaKeyPair;
use crate::ecc::curve::CurveParams;
use crate::ecc::keys::{EccKeyPair, EccPrivateKey, EccPublicKey};
use crate::utils::encoding::{bigint_to_bytes_be, to_hex};

/// A complete bundle of keys for one identity: ECC + RSA.
pub struct KeyBundle {
    pub ecc: EccKeyPair,
    pub rsa: RsaKeyPair,
}

impl KeyBundle {
    /// Generate a new key bundle (secp256k1 ECC + 2048-bit RSA).
    ///
    /// Note: RSA key generation is slow (~seconds for 2048 bits).
    pub fn generate() -> Self {
        let curve = CurveParams::secp256k1();
        KeyBundle {
            ecc: EccKeyPair::generate(&curve),
            rsa: RsaKeyPair::generate(2048),
        }
    }

    /// Print public key metadata. Private material requires an explicit
    /// export API such as [`ecc_private_to_hex`].
    pub fn summary(&self) {
        self.write_summary(&mut std::io::stdout().lock())
            .expect("failed to write key summary");
    }

    /// Write the public summary to the caller's sink, propagating I/O errors.
    /// This does not format or serialize either private key.
    pub fn write_summary(&self, out: &mut (impl std::io::Write + ?Sized)) -> std::io::Result<()> {
        writeln!(out, "=== ECC ({}) ===", self.ecc.public.curve_name)?;
        if let Some(pub_bytes) = self.ecc.public.to_sec1_uncompressed() {
            writeln!(out, "Public:  {}", to_hex(&pub_bytes))?;
        }
        writeln!(out)?;
        writeln!(out, "=== RSA-{} ===", self.rsa.public.bits)?;
        let n_hex = to_hex(&bigint_to_bytes_be(
            &self.rsa.public.n,
            (self.rsa.public.bits as usize).div_ceil(8),
        ));
        writeln!(out, "n (first 32 bytes): {}", &n_hex[..n_hex.len().min(64)])?;
        writeln!(out, "e: {}", self.rsa.public.e)
    }
}

/// Serialize an ECC private key as a 32-byte hex string.
pub fn ecc_private_to_hex(k: &EccPrivateKey) -> String {
    to_hex(&bigint_to_bytes_be(&k.scalar, 32))
}

/// Serialize an ECC public key as uncompressed SEC1 hex.
pub fn ecc_public_to_hex(k: &EccPublicKey) -> Option<String> {
    k.to_sec1_uncompressed().map(|b| to_hex(&b))
}
