//! Experimental components of "Integer multiplication below n log n" (2026-09-23).
//!
//! This module provides an exact finite-field reference for the synthetic-transform
//! algebra, a bounded scalar XOR motif, and an opt-in adapter for existing polynomial
//! callers. It does NOT implement the rank-saving recursive tensor schedule,
//! precision-controlled Gaussian resampling, or the complete asymptotic multiplier.
//! `Algorithm::PublishedSubNlogN` is explicitly unavailable: no conventional fallback
//! can silently impersonate that algorithm. This public-data research code is not
//! constant time and makes no performance or complexity claim.
//!
//! Primary source: <https://github.com/openai/math/tree/main/preprints/Integer-multiplication-below-n-log-n-September-23-2026>.
//! See `research/subnlogn_rust_20261122/README.md` for the component map and limits.

pub mod motif;
pub mod synthetic;

use crate::bigint::Big;
use crate::field::{Field, Prepared, Rng, Zp};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// A requested algorithm, never an inferred label for conventional fallback work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Algorithm {
    /// Bounded exact finite-field reference for the paper's synthetic-transform algebra.
    SyntheticReference,
    /// Full September 2026 construction; not implemented in this module.
    PublishedSubNlogN,
}

#[derive(Debug)]
pub enum Error {
    FullAlgorithmUnavailable,
    InputLimit { supplied: usize, limit: usize },
    CoefficientBound,
    Synthetic(synthetic::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FullAlgorithmUnavailable => write!(f, "full sub-nlogn multiplier unavailable: recursive rank-saving schedule, certified resampling and complete assembly remain unimplemented"),
            Self::InputLimit { supplied, limit } => write!(f, "input has {supplied} limbs; hard limit is {limit}"),
            Self::CoefficientBound => write!(f, "integer convolution cannot be reconstructed within its coefficient bound"),
            Self::Synthetic(e) => write!(f, "synthetic transform: {e:?}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<synthetic::Error> for Error {
    fn from(value: synthetic::Error) -> Self {
        Self::Synthetic(value)
    }
}

/// Dispatch policy for the explicit experimental field wrapper.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Further bounded by the synthetic module's hard allocation limits.
    pub max_points: usize,
    /// Shorter operand length below which `conv_long` retains the existing route.
    pub min_len: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            max_points: 128,
            min_len: 32,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatchStats {
    /// Calls entering the strict synthetic operation (including rejected calls).
    pub attempts: u64,
    /// Successful synthetic operations; an empty product does no transform work.
    pub synthetic: u64,
    /// `conv_long` calls delegated to the original backend, including its `None` result.
    pub fallback: u64,
}
#[derive(Default)]
struct Counters {
    attempts: AtomicU64,
    synthetic: AtomicU64,
    fallback: AtomicU64,
}

/// Explicitly wraps a field to use the reference convolution on admitted products.
/// All other field operations, including prepared-product representations, retain
/// the inner field's implementations. Clones share diagnostic dispatch counters.
/// `conv_long` delegates to the inner field after any synthetic error; callers
/// requiring error diagnostics or candidate-only work should use `try_convolve`.
#[derive(Clone)]
pub struct ExperimentalField<F> {
    inner: F,
    config: Config,
    counters: Arc<Counters>,
}
impl<F: Field> ExperimentalField<F> {
    pub fn new(inner: F, config: Config) -> Self {
        Self {
            inner,
            config,
            counters: Arc::default(),
        }
    }
    pub fn inner(&self) -> &F {
        &self.inner
    }
    pub fn stats(&self) -> DispatchStats {
        DispatchStats {
            attempts: self.counters.attempts.load(Ordering::Relaxed),
            synthetic: self.counters.synthetic.load(Ordering::Relaxed),
            fallback: self.counters.fallback.load(Ordering::Relaxed),
        }
    }
    /// Strict operation: report all synthetic errors; never fall back.
    /// Unlike `conv_long`, this method does not apply the minimum-length policy.
    pub fn try_convolve(&self, a: &[F::E], b: &[F::E]) -> Result<Vec<F::E>, Error> {
        self.counters.attempts.fetch_add(1, Ordering::Relaxed);
        let product = synthetic::convolve(&self.inner, a, b, self.config.max_points)?;
        self.counters.synthetic.fetch_add(1, Ordering::Relaxed);
        Ok(product)
    }
}

impl<F: Field> Field for ExperimentalField<F> {
    type E = F::E;
    fn zero(&self) -> Self::E {
        self.inner.zero()
    }
    fn one(&self) -> Self::E {
        self.inner.one()
    }
    fn add(&self, a: Self::E, b: Self::E) -> Self::E {
        self.inner.add(a, b)
    }
    fn sub(&self, a: Self::E, b: Self::E) -> Self::E {
        self.inner.sub(a, b)
    }
    fn neg(&self, a: Self::E) -> Self::E {
        self.inner.neg(a)
    }
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E {
        self.inner.mul(a, b)
    }
    fn inv(&self, a: Self::E) -> Self::E {
        self.inner.inv(a)
    }
    fn from_u64(&self, n: u64) -> Self::E {
        self.inner.from_u64(n)
    }
    fn from_i64(&self, n: i64) -> Self::E {
        self.inner.from_i64(n)
    }
    fn char(&self) -> u64 {
        self.inner.char()
    }
    fn characteristic_exceeds(&self, n: u64) -> bool {
        self.inner.characteristic_exceeds(n)
    }
    fn size(&self) -> u128 {
        self.inner.size()
    }
    fn q(&self) -> Big {
        self.inner.q()
    }
    fn random(&self, rng: &mut Rng) -> Self::E {
        self.inner.random(rng)
    }
    fn is_zero(&self, a: Self::E) -> bool {
        self.inner.is_zero(a)
    }
    fn sq(&self, a: Self::E) -> Self::E {
        self.inner.sq(a)
    }
    fn pow(&self, a: Self::E, e: u128) -> Self::E {
        self.inner.pow(a, e)
    }
    fn pow_big(&self, a: Self::E, e: &Big) -> Self::E {
        self.inner.pow_big(a, e)
    }
    fn sqrt(&self, a: Self::E) -> Option<Self::E> {
        self.inner.sqrt(a)
    }
    fn div(&self, a: Self::E, b: Self::E) -> Self::E {
        self.inner.div(a, b)
    }
    fn karatsuba_threshold(&self) -> usize {
        self.inner.karatsuba_threshold()
    }
    fn sqr_threshold(&self) -> usize {
        self.inner.sqr_threshold()
    }
    fn powmod_table_max(&self) -> usize {
        self.inner.powmod_table_max()
    }
    fn conv(&self, a: &[Self::E], b: &[Self::E]) -> Vec<Self::E> {
        self.inner.conv(a, b)
    }
    fn conv_sq(&self, a: &[Self::E]) -> Vec<Self::E> {
        self.inner.conv_sq(a)
    }
    fn conv_trunc(&self, a: &[Self::E], b: &[Self::E], n: usize) -> Vec<Self::E> {
        self.inner.conv_trunc(a, b, n)
    }
    fn prepare(&self, b: &[Self::E], max_len: usize, cyc: usize) -> Option<Prepared> {
        self.inner.prepare(b, max_len, cyc)
    }
    fn conv_prepared(&self, a: &[Self::E], b: &Prepared) -> Option<Vec<Self::E>> {
        self.inner.conv_prepared(a, b)
    }
    fn conv_cyclic(&self, a: &[Self::E], b: &[Self::E], n: usize) -> Option<Vec<Self::E>> {
        self.inner.conv_cyclic(a, b, n)
    }
    fn dot_rev(&self, a: &[Self::E], b: &[Self::E]) -> Self::E {
        self.inner.dot_rev(a, b)
    }
    fn dot_rev_rows(&self, a: &[Self::E], rows: &[&[Self::E]]) -> Vec<Self::E> {
        self.inner.dot_rev_rows(a, rows)
    }
    fn batch_inv(&self, xs: &[Self::E]) -> Vec<Self::E> {
        self.inner.batch_inv(xs)
    }
    fn conv_long(&self, a: &[Self::E], b: &[Self::E]) -> Option<Vec<Self::E>> {
        if a.len().min(b.len()) >= self.config.min_len {
            if let Ok(product) = self.try_convolve(a, b) {
                return Some(product);
            }
        }
        self.counters.fallback.fetch_add(1, Ordering::Relaxed);
        self.inner.conv_long(a, b)
    }
}

/// Independent of requested transform limits; checked before trimming or allocating.
pub const MAX_INPUT_LIMBS: usize = 256;
const INTEGER_PRIME: u64 = (1u64 << 61) - 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerProduct {
    /// Canonical little-endian 64-bit limbs; zero is the empty vector.
    pub limbs: Vec<u64>,
    pub backend: Algorithm,
    /// Zero for a zero product, otherwise the synthetic transform length.
    pub transform_points: usize,
}

/// Multiply bounded unsigned integers through exact byte convolution over a prime.
///
/// Inputs and output use little-endian u64 limbs. With byte coefficients, every
/// convolution coefficient is at most min(byte_lengths)*255^2. The explicit bound
/// below ensures recovery from the prime-field result is unique before carrying.
/// This executes the synthetic reference only and has no sub-nlogn complexity claim.
pub fn multiply_exact(
    a: &[u64],
    b: &[u64],
    algorithm: Algorithm,
    max_points: usize,
) -> Result<IntegerProduct, Error> {
    if algorithm == Algorithm::PublishedSubNlogN {
        return Err(Error::FullAlgorithmUnavailable);
    }
    for input in [a, b] {
        if input.len() > MAX_INPUT_LIMBS {
            return Err(Error::InputLimit {
                supplied: input.len(),
                limit: MAX_INPUT_LIMBS,
            });
        }
    }
    let nonzero = |xs: &[u64]| xs.iter().rposition(|&x| x != 0).map_or(0, |i| i + 1);
    let (an, bn) = (nonzero(a), nonzero(b));
    if an == 0 || bn == 0 {
        return Ok(IntegerProduct {
            limbs: vec![],
            backend: algorithm,
            transform_points: 0,
        });
    }
    let digits = |xs: &[u64]| {
        let mut out: Vec<u64> = xs
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .map(u64::from)
            .collect();
        while out.last() == Some(&0) {
            out.pop();
        }
        out
    };
    let (a8, b8) = (digits(&a[..an]), digits(&b[..bn]));
    let bound = (a8.len().min(b8.len()) as u128) * 255 * 255;
    if bound >= INTEGER_PRIME as u128 {
        return Err(Error::CoefficientBound);
    }
    let points = (a8.len() + b8.len() - 1)
        .checked_next_power_of_two()
        .ok_or(Error::CoefficientBound)?;
    let coefficients = synthetic::convolve(&Zp::new(INTEGER_PRIME), &a8, &b8, max_points)?;
    let mut bytes = Vec::with_capacity(8 * (an + bn));
    let mut carry = 0u128;
    for c in coefficients {
        if c as u128 > bound {
            return Err(Error::CoefficientBound);
        }
        carry += c as u128;
        bytes.push((carry & 255) as u8);
        carry >>= 8;
    }
    while carry != 0 {
        bytes.push((carry & 255) as u8);
        carry >>= 8;
    }
    let mut limbs: Vec<u64> = bytes
        .chunks(8)
        .map(|chunk| {
            let mut word = [0u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            u64::from_le_bytes(word)
        })
        .collect();
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
    Ok(IntegerProduct {
        limbs,
        backend: algorithm,
        transform_points: points,
    })
}
