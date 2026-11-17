//! Cryptographically-secure random number generation.
//!
//! Default entropy comes from the OS via `getrandom`/`OsRng`:
//!   - Linux/Android: `getrandom(2)` (or `/dev/urandom` fallback)
//!   - macOS/iOS:     `getentropy(2)` / `SecRandomCopyBytes`
//!   - Windows:       `BCryptGenRandom(BCRYPT_USE_SYSTEM_PREFERRED_RNG)`
//!   - WASM/browser: `crypto.getRandomValues` when configured via getrandom's
//!     target support (not enabled by this crate's default features)
//!
//! `OsRng` is a stateless, blocking CSPRNG and is the convention for
//! generating long-term secret material in Rust cryptographic code.  We
//! deliberately avoid `thread_rng` for secret-material paths: while it is
//! cryptographic, having a single, well-known source makes audits simpler
//! and removes any doubt about reseeding semantics. Fallible `*_with_rng`
//! functions allow a caller-owned provider with the `CryptoRng` contract.

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::OsRng;
use rand::{CryptoRng, RngCore};
use zeroize::{Zeroize, Zeroizing};

/// Entropy acquisition and invalid scalar bounds are separate failures.
#[derive(Debug)]
pub enum RandomError {
    InvalidScalarBound,
    Entropy(rand::Error),
}

impl std::fmt::Display for RandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidScalarBound => f.write_str("random scalar requires max > 1"),
            Self::Entropy(error) => write!(f, "entropy source failed: {error}"),
        }
    }
}

impl std::error::Error for RandomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Entropy(error) => Some(error),
            Self::InvalidScalarBound => None,
        }
    }
}

/// Fallible entropy from a caller-selected CSPRNG. On failure, scrub the
/// entire output (including bytes the source may have partially written).
/// `CryptoRng` is a provider contract, not a verification of its implementation.
///
/// ```compile_fail
/// use crypto_lib::utils::random::try_random_bytes_with_rng;
/// use rand::{rngs::SmallRng, SeedableRng};
/// let mut insecure = SmallRng::seed_from_u64(7);
/// try_random_bytes_with_rng(&mut insecure, &mut [0; 32]).unwrap();
/// ```
pub fn try_random_bytes_with_rng<R: RngCore + CryptoRng + ?Sized>(
    rng: &mut R,
    buf: &mut [u8],
) -> Result<(), RandomError> {
    if let Err(error) = rng.try_fill_bytes(buf) {
        buf.zeroize();
        return Err(RandomError::Entropy(error));
    }
    Ok(())
}

/// Fallible OS entropy for applications that must propagate failure.
pub fn try_random_bytes(buf: &mut [u8]) -> Result<(), RandomError> {
    try_random_bytes_with_rng(&mut OsRng, buf)
}

/// Uniform rejection sampling in `[1, max)` using a caller-owned CSPRNG.
/// Invalid bounds are rejected before consuming entropy; source failures
/// propagate without retrying or substituting a deterministic fallback.
///
/// The bound is public. Rejection count and `BigUint` conversions are
/// variable-time; this is not a constant-time scalar representation. The
/// sampling buffer is scrubbed on drop, but `BigUint` allocations are not.
pub fn try_random_scalar_with_rng<R: RngCore + CryptoRng + ?Sized>(
    rng: &mut R,
    max: &BigUint,
) -> Result<BigUint, RandomError> {
    if max <= &BigUint::one() {
        return Err(RandomError::InvalidScalarBound);
    }
    let bits = (max - BigUint::one()).bits();
    let mut bytes = Zeroizing::new(vec![0u8; bits.div_ceil(8) as usize]);
    let excess = (8 - bits % 8) % 8;
    loop {
        try_random_bytes_with_rng(rng, &mut bytes)?;
        let last = bytes.len() - 1;
        bytes[last] &= 0xff >> excess;
        let candidate = BigUint::from_bytes_le(&bytes);
        if !candidate.is_zero() && candidate < *max {
            return Ok(candidate);
        }
    }
}

pub fn try_random_scalar(max: &BigUint) -> Result<BigUint, RandomError> {
    try_random_scalar_with_rng(&mut OsRng, max)
}

/// Fill `buf` with cryptographically-random bytes from the OS CSPRNG.
///
/// Panics if the OS RNG is unavailable (e.g., no `/dev/urandom` and
/// `getrandom` syscall fails). Use [`try_random_bytes`] to propagate failure.
pub fn random_bytes(buf: &mut [u8]) {
    try_random_bytes(buf).expect("OS CSPRNG unavailable");
}

/// Generate a random `BigUint` in the range `[1, max)`.
///
/// Uses rejection sampling on top of `OsRng` so the distribution is uniform
/// — no modular bias.  The output is guaranteed non-zero.
pub fn random_scalar_checked(max: &BigUint) -> Option<BigUint> {
    if max <= &BigUint::one() {
        return None;
    }
    Some(try_random_scalar(max).expect("OS CSPRNG unavailable"))
}

/// Generate a random `BigUint` in the range `[1, max)`.
///
/// Panics for `max <= 1`; use [`random_scalar_checked`] when invalid bounds are
/// possible at runtime.
pub fn random_scalar(max: &BigUint) -> BigUint {
    random_scalar_checked(max).expect("random_scalar requires max > 1")
}

/// Generate `len` cryptographically-random bytes as a `Vec<u8>`.
pub fn random_bytes_vec(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    random_bytes(&mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    // Scripted public test data. This provider is deliberately not exported.
    struct Scripted {
        draws: std::collections::VecDeque<Result<u8, ()>>,
        calls: usize,
    }
    impl CryptoRng for Scripted {}
    impl RngCore for Scripted {
        fn next_u32(&mut self) -> u32 {
            panic!("must use fallible entropy")
        }
        fn next_u64(&mut self) -> u64 {
            panic!("must use fallible entropy")
        }
        fn fill_bytes(&mut self, _: &mut [u8]) {
            panic!("must use fallible entropy")
        }
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.calls += 1;
            match self.draws.pop_front().expect("test entropy exhausted") {
                Ok(byte) => {
                    dest.fill(byte);
                    Ok(())
                }
                Err(()) => {
                    dest.fill(0x5a);
                    Err(rand::Error::new(std::io::Error::other("test failure")))
                }
            }
        }
    }

    #[test]
    fn scalar_rejects_zero_and_out_of_range_without_modulo_bias() {
        let mut rng = Scripted {
            draws: [Ok(0), Ok(31), Ok(7)].into(),
            calls: 0,
        };
        let scalar = try_random_scalar_with_rng(&mut rng, &BigUint::from(17u8)).unwrap();
        assert_eq!(scalar, BigUint::from(7u8));
        assert_eq!(rng.calls, 3);
    }

    #[test]
    fn invalid_bounds_consume_no_entropy_and_failures_scrub_output() {
        let mut rng = Scripted {
            draws: [Err(()), Err(())].into(),
            calls: 0,
        };
        for bound in [0u8, 1] {
            assert!(matches!(
                try_random_scalar_with_rng(&mut rng, &BigUint::from(bound)),
                Err(RandomError::InvalidScalarBound)
            ));
        }
        assert_eq!(rng.calls, 0);
        let mut bytes = [0xab; 32];
        assert!(matches!(
            try_random_bytes_with_rng(&mut rng, &mut bytes),
            Err(RandomError::Entropy(_))
        ));
        assert_eq!(bytes, [0; 32]);
        assert!(matches!(
            try_random_scalar_with_rng(&mut rng, &BigUint::from(17u8)),
            Err(RandomError::Entropy(_))
        ));
        assert_eq!(rng.calls, 2);
    }

    #[test]
    fn scalar_sampling_handles_multi_byte_and_power_of_two_bounds() {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(11);
        for bound in [2u64, 256, 257, 65536, u64::MAX] {
            let bound = BigUint::from(bound);
            for _ in 0..64 {
                let scalar = try_random_scalar_with_rng(&mut rng, &bound).unwrap();
                assert!(!scalar.is_zero() && scalar < bound);
            }
        }
    }

    #[test]
    fn random_scalar_checked_rejects_empty_ranges() {
        assert!(random_scalar_checked(&BigUint::zero()).is_none());
        assert!(random_scalar_checked(&BigUint::one()).is_none());
    }
}
