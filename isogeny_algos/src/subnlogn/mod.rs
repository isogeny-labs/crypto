//! Experimental reconstruction of "Integer multiplication below n log n" (2026-09-23).
//!
//! The literal parameter selector, rank-labelled recursive tensor and interchange
//! schedules, certified Gaussian resampling, complex transforms, CRT transport,
//! and whole-integer recovery are executable components. `multiply_published`
//! reports the actual selected route; every native-sized input selects the
//! prescribed small-input base case. `assembly::Plan` exercises the numerical
//! construction at explicit admitted parameters without changing the selector.
//!
//! This is not a verified sub-nlogn native runtime: the full fixed network has not
//! been executed, and the segmented filesystem interpreter's host overhead is
//! outside the paper's fixed-tape recurrence. A base-case or diagnostic product
//! does not establish that bound. This public-data code is not constant time.
//!
//! The bounded finite-field reference and its opt-in `ExperimentalField` adapter
//! remain separate. See `research/subnlogn_rust_20261122/README.md` for source pins,
//! stage mapping, cost obligations, and evidence scope.

pub mod assembly;
pub mod certified;
pub mod crt;
pub mod fastdiv;
pub mod fastint;
pub mod motif;
pub mod parameters;
pub mod resampling;
pub mod ring;
pub mod swap;
pub mod synthetic;
pub mod tape;
pub mod tensor;
pub mod transforms;

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
    /// Literal September 2026 parameter prescription and experimental reconstruction.
    /// This request is not a claim that a rank-saving branch ran or a bound was verified.
    PublishedSubNlogN,
}

#[derive(Debug)]
pub enum Error {
    SizeOverflow,
    InputLimit { supplied: usize, limit: usize },
    CoefficientBound,
    Synthetic(synthetic::Error),
    Parameters(parameters::Error),
    Assembly(assembly::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SizeOverflow => write!(f, "integer length exceeds native addressability"),
            Self::InputLimit { supplied, limit } => {
                write!(f, "input has {supplied} limbs; hard limit is {limit}")
            }
            Self::CoefficientBound => write!(
                f,
                "integer convolution cannot be reconstructed within its coefficient bound"
            ),
            Self::Synthetic(e) => write!(f, "synthetic transform: {e:?}"),
            Self::Parameters(e) => write!(f, "{e}"),
            Self::Assembly(e) => write!(f, "{e}"),
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

/// Synthetic-reference limit; checked before trimming or allocating on that route.
pub const MAX_INPUT_LIMBS: usize = 256;
const INTEGER_PRIME: u64 = (1u64 << 61) - 1;

/// Actual route, separate from the requested algorithm's name.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerRoute {
    SyntheticReference,
    PublishedBaseCase { reason: parameters::BaseCaseReason },
    PublishedTransform,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntegerProduct {
    /// Canonical little-endian 64-bit limbs; zero is the empty vector.
    pub limbs: Vec<u64>,
    /// Requested construction. Consult `route` for the actual executed path.
    pub backend: Algorithm,
    pub route: IntegerRoute,
    /// Synthetic reference length or published target volume; zero for a base case.
    pub transform_points: usize,
}

/// Execution details for the literal published parameter selection.
#[derive(Debug)]
pub enum PublishedExecution {
    BaseCase {
        reason: parameters::BaseCaseReason,
        prerequisite: fastint::Work,
    },
    Transform {
        parameters: assembly::Parameters,
        execution: Box<assembly::Execution>,
        precision: Box<assembly::PrecisionCertificate>,
    },
}

#[derive(Debug)]
pub struct PublishedProduct {
    pub limbs: Vec<u64>,
    pub execution: PublishedExecution,
}

/// Execute the literal parameter prescription and report the actual route.
///
/// The exponent epsilon=2^-75 is not enlarged for practical inputs. Consequently
/// every input length representable by `usize` selects a base case. That case
/// uses the concrete recursive Fermat multiplier and is explicitly identified.
/// An admitted transform uses `assembly::Plan`; any error is returned, with no
/// error-triggered fallback. For explicit diagnostic geometry use that plan
/// directly. Neither this API nor its receipts verify a native asymptotic bound.
pub fn multiply_published(a: &[u64], b: &[u64]) -> Result<PublishedProduct, Error> {
    for input in [a, b] {
        input.len().checked_mul(64).ok_or(Error::SizeOverflow)?;
    }
    let a = Big::from_limbs(a);
    let b = Big::from_limbs(b);
    let n = a.bits().max(b.bits());
    match parameters::select(n).map_err(Error::Parameters)? {
        parameters::Selection::BaseCase { reason } => {
            a.bits()
                .checked_add(b.bits())
                .and_then(|bits| bits.checked_next_power_of_two())
                .ok_or(Error::SizeOverflow)?;
            let (value, prerequisite) = fastint::multiply_with_work(&a, &b);
            Ok(PublishedProduct {
                limbs: value.0,
                execution: PublishedExecution::BaseCase {
                    reason,
                    prerequisite,
                },
            })
        }
        parameters::Selection::Transform(parameters) => {
            let mut plan = assembly::Plan::new(parameters.clone()).map_err(Error::Assembly)?;
            let product = plan.multiply(&a, &b).map_err(Error::Assembly)?;
            Ok(PublishedProduct {
                limbs: product.value.0,
                execution: PublishedExecution::Transform {
                    parameters,
                    execution: Box::new(product.execution),
                    precision: Box::new(product.precision),
                },
            })
        }
    }
}

/// Multiply unsigned integers through the explicitly requested construction.
///
/// Inputs and output use little-endian u64 limbs. `max_points` limits only the
/// synthetic-reference route; the literal prescription chooses its own geometry.
/// `multiply_published` exposes the more detailed construction receipts.
///
/// On the synthetic-reference route, every byte-convolution coefficient is at
/// most min(byte_lengths)*255^2. The explicit bound below ensures recovery from
/// the prime-field result is unique before carrying. That reference is bounded
/// independently by `MAX_INPUT_LIMBS`.
pub fn multiply_exact(
    a: &[u64],
    b: &[u64],
    algorithm: Algorithm,
    max_points: usize,
) -> Result<IntegerProduct, Error> {
    if algorithm == Algorithm::PublishedSubNlogN {
        let product = multiply_published(a, b)?;
        let (route, transform_points) = match product.execution {
            PublishedExecution::BaseCase { reason, .. } => {
                (IntegerRoute::PublishedBaseCase { reason }, 0)
            }
            PublishedExecution::Transform { parameters, .. } => (
                IntegerRoute::PublishedTransform,
                parameters.target_lengths.iter().product(),
            ),
        };
        return Ok(IntegerProduct {
            limbs: product.limbs,
            backend: algorithm,
            route,
            transform_points,
        });
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
            route: IntegerRoute::SyntheticReference,
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
        route: IntegerRoute::SyntheticReference,
        transform_points: points,
    })
}
