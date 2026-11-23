//! Bounded, exact synthetic-root transforms over an odd-characteristic field.
//!
//! This is a reference implementation of the algebra in section 6 of the
//! September 23, 2026 manuscript, `06-transforms.tex`: normalized `H_0`
//! butterflies, signed monomial twiddles in `F[Y]/(Y^r + 1)`, retained
//! bit-reversed frequency order, and the normalized opposite transform.
//! Unlike the manuscript's complex coefficient implementation, this module
//! works exactly over `F`, with no rounding or floating-point arithmetic.
//!
//! The reference uses one axis and ordinary quadratic negacyclic products of
//! the polynomial records. It does **not** implement simultaneous tensor
//! layers, tape layouts, Gaussian resampling, or the complete claimed
//! sub-n-log-n integer multiplier. With `t` transform points and record length
//! `r`, its field-operation count is `O(t*r*r + t*r*log(t))` and its storage
//! is `O(t*r)`. This embedding chooses `r = max(t/2, 2)`, giving a cubic
//! field-operation upper bound in `t`. The explicit resource caps are intentional.
//!
//! Source: `06-transforms.tex`, Git blob
//! `de073e9292d58573f7065a3cd4f214e0d05f3444`:
//! <https://github.com/openai/math/blob/4eaee87548250b2682affda7d05d947c05554bae/preprints/Integer-multiplication-below-n-log-n-September-23-2026/build/sections/06-transforms.tex>.

use crate::field::Field;
use std::fmt;

/// Maximum conservatively counted field-element slots in owned vectors,
/// regardless of the caller's transform-point limit. This bounds the
/// quadratic embedding; caller-owned inputs and allocator metadata are excluded.
pub const MAX_WORKING_SLOTS: usize = 1 << 18;

/// Additional vector-storage byte guard for unusually large element types.
pub const MAX_WORKING_BYTES: usize = 64 << 20;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    CharacteristicTwo,
    NonInvertibleTwo,
    SizeOverflow,
    PointLimit { required: usize, limit: usize },
    WorkingSlotLimit { required: usize, limit: usize },
    WorkingByteLimit { required: usize, limit: usize },
    AllocationFailed,
    InconsistentOutput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacteristicTwo => {
                write!(f, "synthetic H0 requires characteristic other than two")
            }
            Self::NonInvertibleTwo => write!(f, "field did not supply a valid inverse of two"),
            Self::SizeOverflow => write!(f, "synthetic transform size overflow"),
            Self::PointLimit { required, limit } => {
                write!(
                    f,
                    "synthetic transform needs {required} points, limit is {limit}"
                )
            }
            Self::WorkingSlotLimit { required, limit } => {
                write!(
                    f,
                    "synthetic transform needs {required} scalar slots, limit is {limit}"
                )
            }
            Self::WorkingByteLimit { required, limit } => {
                write!(
                    f,
                    "synthetic transform needs {required} working bytes, limit is {limit}"
                )
            }
            Self::AllocationFailed => write!(f, "synthetic transform allocation failed"),
            Self::InconsistentOutput => write!(
                f,
                "synthetic transform produced a non-scalar or wrapped result"
            ),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug)]
struct Shape {
    output: usize,
    points: usize,
    record: usize,
    cells: usize,
}

fn shape<E>(a: usize, b: usize, max_points: usize) -> Result<Shape, Error> {
    // Called only for nonempty inputs. Keep checks here so enormous lengths
    // can be tested without constructing enormous slices.
    let output = a
        .checked_add(b)
        .and_then(|v| v.checked_sub(1))
        .ok_or(Error::SizeOverflow)?;
    let points = output
        .checked_next_power_of_two()
        .ok_or(Error::SizeOverflow)?;
    if points > max_points {
        return Err(Error::PointLimit {
            required: points,
            limit: max_points,
        });
    }
    // The synthetic root Y^(2r/t) exists because t divides 2r. Keep r >= 2
    // as in the manuscript, including the length-one and length-two cases.
    let record = (points / 2).max(2);
    record.checked_mul(2).ok_or(Error::SizeOverflow)?;
    let cells = points.checked_mul(record).ok_or(Error::SizeOverflow)?;
    // Two transform arrays, one record scratch buffer, and the output. This
    // is conservative: the second array is freed before allocating output.
    let slots = cells
        .checked_mul(2)
        .and_then(|v| v.checked_add(record))
        .and_then(|v| v.checked_add(output))
        .ok_or(Error::SizeOverflow)?;
    if slots > MAX_WORKING_SLOTS {
        return Err(Error::WorkingSlotLimit {
            required: slots,
            limit: MAX_WORKING_SLOTS,
        });
    }
    let bytes = slots
        .checked_mul(std::mem::size_of::<E>().max(1))
        .ok_or(Error::SizeOverflow)?;
    if bytes > MAX_WORKING_BYTES {
        return Err(Error::WorkingByteLimit {
            required: bytes,
            limit: MAX_WORKING_BYTES,
        });
    }
    Ok(Shape {
        output,
        points,
        record,
        cells,
    })
}

fn zeros<E: Copy>(len: usize, zero: E) -> Result<Vec<E>, Error> {
    let mut v = Vec::new();
    v.try_reserve_exact(len)
        .map_err(|_| Error::AllocationFailed)?;
    v.resize(len, zero);
    Ok(v)
}

/// Multiply a polynomial record by Y^exponent, modulo Y^r + 1.
/// Exponents are reduced modulo 2r, so negative twiddles need no signed cast.
fn monomial<F: Field>(f: &F, record: &mut [F::E], exponent: usize, scratch: &mut [F::E]) {
    let r = record.len();
    let exponent = exponent % (2 * r);
    let shift = exponent % r;
    let common_negative = exponent >= r;
    for (i, &x) in record.iter().enumerate() {
        let j = i + shift;
        let negative = common_negative ^ (j >= r);
        scratch[j % r] = if negative { f.neg(x) } else { x };
    }
    record.copy_from_slice(scratch);
}

fn level<F: Field>(
    f: &F,
    data: &mut [F::E],
    points: usize,
    r: usize,
    width: usize,
    inv_two: F::E,
    opposite: bool,
    scratch: &mut [F::E],
) {
    let half = width / 2;
    let stride = 2 * r / width;
    for block in (0..points).step_by(width) {
        for k in 0..half {
            let lo = (block + k) * r;
            let hi = (block + k + half) * r;
            let exponent = stride * k;
            if opposite {
                // Reverse the twiddle first, then apply H0, not its inverse.
                monomial(f, &mut data[hi..hi + r], exponent, scratch);
            }
            for j in 0..r {
                let x = data[lo + j];
                let y = data[hi + j];
                data[lo + j] = f.mul(f.add(x, y), inv_two);
                data[hi + j] = f.mul(f.sub(x, y), inv_two);
            }
            if !opposite {
                let inverse_exponent = (2 * r - exponent) % (2 * r);
                monomial(f, &mut data[hi..hi + r], inverse_exponent, scratch);
            }
        }
    }
}

/// Implements B_t F_t^- when opposite=false, and F_t^+ B_t^-1 otherwise.
/// Their composition is I/t, since both transforms are normalized.
fn transform<F: Field>(
    f: &F,
    data: &mut [F::E],
    points: usize,
    r: usize,
    inv_two: F::E,
    opposite: bool,
    scratch: &mut [F::E],
) {
    if opposite {
        let mut width = 2;
        while width <= points {
            level(f, data, points, r, width, inv_two, true, scratch);
            if width == points {
                break;
            }
            width *= 2;
        }
    } else {
        let mut width = points;
        while width >= 2 {
            level(f, data, points, r, width, inv_two, false, scratch);
            width /= 2;
        }
    }
}

/// Ordinary quadratic multiplication in F[Y]/(Y^r + 1), normalized by 1/r.
fn normalized_ring_product<F: Field>(f: &F, a: &[F::E], b: &[F::E], inv_r: F::E, out: &mut [F::E]) {
    out.fill(f.zero());
    let r = a.len();
    for (i, &x) in a.iter().enumerate() {
        if f.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if f.is_zero(y) {
                continue;
            }
            let product = f.mul(x, y);
            if i + j < r {
                out[i + j] = f.add(out[i + j], product);
            } else {
                out[i + j - r] = f.sub(out[i + j - r], product);
            }
        }
    }
    for x in out {
        *x = f.mul(*x, inv_r);
    }
}

/// Exact ordinary coefficient convolution, using synthetic roots of unity.
///
/// Coefficients are low-to-high, and a nonempty result has exactly
/// `a.len() + b.len() - 1` coefficients (including any trailing zeros).
/// Empty inputs produce an empty result. The field must have an invertible
/// two; this is checked using field arithmetic, not the possibly truncated
/// `Field::char()` accessor. As with other `Field` clients, the caller must
/// supply a valid implementation of that trait.
///
/// Inputs are embedded as constant records in F[Y]/(Y^r + 1), zero-padded
/// to `t >= a.len() + b.len() - 1` points. No t-th root in F is needed.
/// Both forward transforms retain their bit-reversed frequency order. The
/// pointwise product is divided by r as in the manuscript. The normalized
/// opposite returns convolution/(r*t^2), so the final scale is r*t^2.
/// All divisions are by powers of two and are exact field operations.
///
/// `max_points` bounds t. Independent scalar-slot and byte caps prevent a
/// very large caller limit from requesting unbounded working storage. A cap
/// violation returns an error; it never silently substitutes another method.
pub fn convolve<F: Field>(
    f: &F,
    a: &[F::E],
    b: &[F::E],
    max_points: usize,
) -> Result<Vec<F::E>, Error> {
    let two = f.add(f.one(), f.one());
    if f.is_zero(two) {
        return Err(Error::CharacteristicTwo);
    }
    let inv_two = f.inv(two);
    if f.mul(two, inv_two) != f.one() {
        return Err(Error::NonInvertibleTwo);
    }
    if a.is_empty() || b.is_empty() {
        return Ok(Vec::new());
    }
    let s = shape::<F::E>(a.len(), b.len(), max_points)?;
    let mut fa = zeros(s.cells, f.zero())?;
    let mut fb = zeros(s.cells, f.zero())?;
    let mut scratch = zeros(s.record, f.zero())?;
    for (i, &x) in a.iter().enumerate() {
        fa[i * s.record] = x;
    }
    for (i, &x) in b.iter().enumerate() {
        fb[i * s.record] = x;
    }
    transform(f, &mut fa, s.points, s.record, inv_two, false, &mut scratch);
    transform(f, &mut fb, s.points, s.record, inv_two, false, &mut scratch);
    let mut inv_r = f.one();
    for _ in 0..s.record.trailing_zeros() {
        inv_r = f.mul(inv_r, inv_two);
    }
    for i in 0..s.points {
        let start = i * s.record;
        normalized_ring_product(
            f,
            &fa[start..start + s.record],
            &fb[start..start + s.record],
            inv_r,
            &mut scratch,
        );
        fa[start..start + s.record].copy_from_slice(&scratch);
    }
    drop(fb);
    transform(f, &mut fa, s.points, s.record, inv_two, true, &mut scratch);
    let mut scale = f.one();
    for _ in 0..(s.record.trailing_zeros() + 2 * s.points.trailing_zeros()) {
        scale = f.add(scale, scale);
    }
    let mut result = zeros(s.output, f.zero())?;
    for (i, record) in fa.chunks_exact(s.record).enumerate() {
        if record[1..].iter().any(|&x| !f.is_zero(x)) || (i >= s.output && !f.is_zero(record[0])) {
            return Err(Error::InconsistentOutput);
        }
        if i < s.output {
            result[i] = f.mul(record[0], scale);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{Rng, Zp};

    // Deliberately independent arithmetic, also covering characteristics two
    // and three, which the production Zp constructor does not accept.
    #[derive(Clone)]
    struct TinyPrime(u64);
    impl Field for TinyPrime {
        type E = u64;
        fn zero(&self) -> u64 {
            0
        }
        fn one(&self) -> u64 {
            1
        }
        fn add(&self, a: u64, b: u64) -> u64 {
            (a + b) % self.0
        }
        fn sub(&self, a: u64, b: u64) -> u64 {
            (a + self.0 - b) % self.0
        }
        fn neg(&self, a: u64) -> u64 {
            (self.0 - a) % self.0
        }
        fn mul(&self, a: u64, b: u64) -> u64 {
            (a * b) % self.0
        }
        fn inv(&self, a: u64) -> u64 {
            assert_ne!(a, 0);
            self.pow(a, (self.0 - 2) as u128)
        }
        fn from_u64(&self, n: u64) -> u64 {
            n % self.0
        }
        fn char(&self) -> u64 {
            self.0
        }
        fn size(&self) -> u128 {
            self.0 as u128
        }
        fn random(&self, rng: &mut Rng) -> u64 {
            rng.below(self.0)
        }
    }

    fn schoolbook(p: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] = ((out[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64;
            }
        }
        out
    }

    #[test]
    fn ordinary_convolutions_match_independent_schoolbook() {
        for p in [3, 31, 65537, (1u64 << 31) - 1] {
            let f = TinyPrime(p);
            let mut state = 0x736e6c6f676eu64;
            for (la, lb) in [
                (1, 1),
                (1, 2),
                (2, 1),
                (1, 9),
                (2, 3),
                (7, 10),
                (16, 17),
                (33, 7),
                (64, 65),
            ] {
                let mut sample = || {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    state % p
                };
                let a: Vec<_> = (0..la).map(|_| sample()).collect();
                let b: Vec<_> = (0..lb).map(|_| sample()).collect();
                assert_eq!(convolve(&f, &a, &b, 128).unwrap(), schoolbook(p, &a, &b));
            }
            let a = vec![p - 1; 9];
            let b = vec![p - 1; 16];
            assert_eq!(convolve(&f, &a, &b, 32).unwrap(), schoolbook(p, &a, &b));
            assert_eq!(convolve(&f, &[0, 0], &[1, 0, 0], 4).unwrap(), vec![0; 4]);
            assert_eq!(convolve(&f, &[], &[1], 0).unwrap(), Vec::<u64>::new());
            assert_eq!(convolve(&f, &[1], &[], 0).unwrap(), Vec::<u64>::new());
        }
    }

    #[test]
    fn production_field_without_long_base_field_roots_works() {
        let p = (1u64 << 31) - 1; // v_2(p - 1) = 1, but our t is 32.
        let f = Zp::new(p);
        let a: Vec<_> = (0..17).map(|i| (i * i + 3) as u64).collect();
        let b = vec![p - 1; 9];
        assert_eq!(convolve(&f, &a, &b, 32).unwrap(), schoolbook(p, &a, &b));
    }

    #[test]
    fn signed_monomial_permutation_and_negacyclic_product() {
        let f = TinyPrime(31);
        let mut scratch = vec![0; 4];
        let original = vec![1, 2, 3, 4];
        let mut a = original.clone();
        monomial(&f, &mut a, 1, &mut scratch);
        assert_eq!(a, vec![27, 1, 2, 3]);
        monomial(&f, &mut a, 7, &mut scratch); // Y^-1.
        assert_eq!(a, original);
        monomial(&f, &mut a, 4, &mut scratch);
        assert_eq!(a, vec![30, 29, 28, 27]);
        normalized_ring_product(&f, &[0, 0, 0, 1], &[0, 1, 0, 0], 8, &mut scratch);
        assert_eq!(scratch, vec![23, 0, 0, 0]); // -1/4 mod 31.
    }

    #[test]
    fn stored_forward_matches_direct_dft_and_opposite_scales_by_inverse_t() {
        let f = TinyPrime(31);
        for t in [1usize, 2, 4, 8, 16] {
            let r = (t / 2).max(2);
            let original: Vec<_> = (0..t * r)
                .map(|i| (i * i + 2 * i + 7) as u64 % 31)
                .collect();
            let mut data = original.clone();
            let mut scratch = vec![0; r];
            transform(&f, &mut data, t, r, 16, false, &mut scratch);
            let inv_t = f.inv(t as u64 % 31);
            let bits = t.trailing_zeros();
            // Direct definition of normalized DFT, with monomial products
            // computed independently by exponent reduction of each term.
            for frequency in 0..t {
                let slot = if bits == 0 {
                    0
                } else {
                    frequency.reverse_bits() >> (usize::BITS - bits)
                };
                let mut expected = vec![0; r];
                for k in 0..t {
                    let exponent = (2 * r - ((2 * r / t) * frequency * k) % (2 * r)) % (2 * r);
                    for j in 0..r {
                        let degree = (j + exponent) % (2 * r);
                        let x = original[k * r + j];
                        let index = degree % r;
                        expected[index] = if degree >= r {
                            f.sub(expected[index], x)
                        } else {
                            f.add(expected[index], x)
                        };
                    }
                }
                for x in &mut expected {
                    *x = f.mul(*x, inv_t);
                }
                assert_eq!(&data[slot * r..(slot + 1) * r], expected);
            }
            transform(&f, &mut data, t, r, 16, true, &mut scratch);
            let expected: Vec<_> = original.into_iter().map(|x| f.mul(x, inv_t)).collect();
            assert_eq!(data, expected);
        }
    }

    #[test]
    fn characteristic_and_resource_limits_are_explicit() {
        assert_eq!(
            convolve(&TinyPrime(2), &[1], &[1], 1),
            Err(Error::CharacteristicTwo)
        );
        // Deliberately invalid prime-field fixture: Fermat inversion for 9
        // does not return 1/2. A nonzero two alone is not our admission test.
        assert_eq!(
            convolve(&TinyPrime(9), &[1], &[1], 1),
            Err(Error::NonInvertibleTwo)
        );
        assert_eq!(
            convolve(&TinyPrime(3), &[1, 2], &[1, 2], 2),
            Err(Error::PointLimit {
                required: 4,
                limit: 2
            })
        );
        assert_eq!(
            shape::<u64>(usize::MAX, 2, usize::MAX).unwrap_err(),
            Error::SizeOverflow
        );
        assert!(matches!(
            shape::<u64>(129, 129, usize::MAX),
            Err(Error::WorkingSlotLimit { .. })
        ));
        assert!(matches!(
            shape::<[u64; 2048]>(33, 32, 64),
            Err(Error::WorkingByteLimit { .. })
        ));
    }
}
