//! Exact recursive Fermat-ring multiplication used by the analytic primitives.
//!
//! This is the classical prerequisite, **not** the new sub-nlogn algorithm.
//! It follows the negacyclic construction of Brent and Zimmermann, *Modern
//! Computer Arithmetic*, §2.3.3, Algorithm 2.4. No external arithmetic callback,
//! floating-point FFT, or unbounded schoolbook leaf is used.
//!
//! At an N-bit node choose a power of two L dividing N, with
//! L <= sqrt(N)/ceil(log2(N)). Split into L radix-2^w chunks, w=N/L, and
//! convolve in Z/(2^K+1), K=L*ceil((2w+log2(L)+2)/L). The extra two bits allow
//! symmetric recovery of every signed negacyclic coefficient. Multiplication
//! by roots of unity is a signed binary shift. Forward DIF and inverse DIT
//! retain the same bit-reversed ordering; there is no global bit-reversal pass.
//!
//! The recurrence has L*K = 2N+O(N/log^2 N) and
//! log K = (log N)/2+O(log log N). Its bit cost is
//! O(N log N log log N); the final carry pass writes disjoint bit ranges in
//! O(N) work. These are mathematical operation bounds, not a runtime benchmark
//! or a proof that Rust's allocator is a multitape machine.

use crate::bigint::Big;

/// A fixed cutoff; its value never grows with input length.
const BASE_BITS: usize = 4096;

/// Counts actual recursive operations, useful for distinguishing the FFT path
/// from a small-input base case. Saturating diagnostic counters do not dispatch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub transform_nodes: u64,
    pub schoolbook_leaves: u64,
    pub butterflies: u64,
    pub depth: usize,
}

/// Multiply arbitrary unsigned integers exactly.
pub fn multiply(a: &Big, b: &Big) -> Big {
    multiply_with_work(a, b).0
}

/// The same implementation, exposing which branches actually ran.
pub fn multiply_with_work(a: &Big, b: &Big) -> (Big, Work) {
    let mut work = Work::default();
    if a.is_zero() || b.is_zero() {
        return (Big::zero(), work);
    }
    let bits = a
        .bits()
        .checked_add(b.bits())
        .expect("product length overflow");
    if bits <= BASE_BITS {
        work.schoolbook_leaves = 1;
        return (a.mul(b), work);
    }
    let n = bits
        .checked_next_power_of_two()
        .expect("product length overflow");
    // a*b < 2^n, so the modular result is the ordinary integer product.
    let value = multiply_mod(a, b, n, &mut work, 0);
    (value, work)
}

fn pow2(n: usize) -> Big {
    Big::from_u64(1).shl(n)
}

/// Copy only the requested bit interval: shifting the entire source for every
/// chunk would accidentally make splitting quadratic in the number of chunks.
pub(crate) fn bit_slice(x: &Big, start: usize, len: usize) -> Big {
    if len == 0 || start >= x.bits() {
        return Big::zero();
    }
    let actual = len.min(x.bits() - start);
    let count = actual.div_ceil(64);
    let first = start / 64;
    let shift = start % 64;
    let mut out = Vec::with_capacity(count);
    for j in 0..count {
        let mut limb = x.0.get(first + j).copied().unwrap_or(0) >> shift;
        if shift != 0 {
            limb |= x.0.get(first + j + 1).copied().unwrap_or(0) << (64 - shift);
        }
        out.push(limb);
    }
    if !actual.is_multiple_of(64) {
        *out.last_mut().expect("nonempty slice") &= (1u64 << (actual % 64)) - 1;
    }
    Big::from_limbs(&out)
}

fn write_bits(out: &mut [u64], offset: usize, value: &Big) {
    let limb = offset / 64;
    let shift = offset % 64;
    for (j, &x) in value.0.iter().enumerate() {
        out[limb + j] |= x << shift;
        if shift != 0 && limb + j + 1 < out.len() {
            out[limb + j + 1] |= x >> (64 - shift);
        }
    }
}

struct Fermat {
    n: usize,
    modulus: Big,
    half: Big,
}

impl Fermat {
    fn new(n: usize) -> Self {
        Self {
            n,
            modulus: pow2(n).add_small(1),
            half: pow2(n - 1),
        }
    }

    fn neg(&self, a: &Big) -> Big {
        if a.is_zero() {
            Big::zero()
        } else {
            self.modulus.sub(a)
        }
    }

    fn add(&self, a: &Big, b: &Big) -> Big {
        let sum = a.add(b);
        if sum >= self.modulus {
            sum.sub(&self.modulus)
        } else {
            sum
        }
    }

    fn sub(&self, a: &Big, b: &Big) -> Big {
        if a >= b {
            a.sub(b)
        } else {
            self.modulus.sub(&b.sub(a))
        }
    }

    /// Reduction for a nonnegative integer with at most 2N+1 bits.
    fn reduce(&self, value: &Big) -> Big {
        debug_assert!(value.bits() <= 2 * self.n + 1);
        let low = bit_slice(value, 0, self.n);
        let high = value.shr(self.n);
        self.sub(&low, &high)
    }

    fn shift(&self, value: &Big, exponent: usize) -> Big {
        let exponent = exponent % (2 * self.n);
        let result = self.reduce(&value.shl(exponent % self.n));
        if exponent >= self.n {
            self.neg(&result)
        } else {
            result
        }
    }

    fn inverse_shift(&self, value: &Big, exponent: usize) -> Big {
        let exponent = exponent % (2 * self.n);
        self.shift(
            value,
            if exponent == 0 {
                0
            } else {
                2 * self.n - exponent
            },
        )
    }
}

fn transform(values: &mut [Big], ring: &Fermat, inverse: bool, work: &mut Work) {
    let len = values.len();
    assert!(len.is_power_of_two());
    assert_eq!((2 * ring.n) % len, 0);
    if inverse {
        let mut block = 2;
        while block <= len {
            let half = block / 2;
            let step = 2 * ring.n / block;
            for chunk in values.chunks_exact_mut(block) {
                let (low, high) = chunk.split_at_mut(half);
                for (j, (a, b)) in low.iter_mut().zip(high.iter_mut()).enumerate() {
                    let t = ring.inverse_shift(b, j * step);
                    let sum = ring.add(a, &t);
                    let difference = ring.sub(a, &t);
                    *a = sum;
                    *b = difference;
                    work.butterflies = work.butterflies.saturating_add(1);
                }
            }
            if block == len {
                break;
            }
            block *= 2;
        }
        let log_len = len.trailing_zeros() as usize;
        for value in values {
            *value = ring.inverse_shift(value, log_len);
        }
    } else {
        let mut block = len;
        while block >= 2 {
            let half = block / 2;
            let step = 2 * ring.n / block;
            for chunk in values.chunks_exact_mut(block) {
                let (low, high) = chunk.split_at_mut(half);
                for (j, (a, b)) in low.iter_mut().zip(high.iter_mut()).enumerate() {
                    let sum = ring.add(a, b);
                    let difference = ring.sub(a, b);
                    *a = sum;
                    *b = ring.shift(&difference, j * step);
                    work.butterflies = work.butterflies.saturating_add(1);
                }
            }
            block /= 2;
        }
    }
}

/// Sign-magnitude carry, kept local so the low-level prerequisite does not
/// depend on the certified arithmetic that itself depends on multiplication.
struct Carry {
    negative: bool,
    magnitude: Big,
}

impl Carry {
    fn zero() -> Self {
        Self {
            negative: false,
            magnitude: Big::zero(),
        }
    }

    fn add(&mut self, negative: bool, magnitude: Big) {
        if self.negative == negative {
            self.magnitude = self.magnitude.add(&magnitude);
        } else if self.magnitude >= magnitude {
            self.magnitude = self.magnitude.sub(&magnitude);
        } else {
            self.magnitude = magnitude.sub(&self.magnitude);
            self.negative = negative;
        }
        if self.magnitude.is_zero() {
            self.negative = false;
        }
    }

    /// Euclidean radix division, including a negative carry. The returned
    /// digit is always nonnegative, while self becomes floor(self/2^w).
    fn take_digit(&mut self, w: usize, radix: &Big) -> Big {
        let low = bit_slice(&self.magnitude, 0, w);
        self.magnitude = self.magnitude.shr(w);
        let digit = if self.negative && !low.is_zero() {
            self.magnitude = self.magnitude.add_small(1);
            radix.sub(&low)
        } else {
            low
        };
        if self.magnitude.is_zero() {
            self.negative = false;
        }
        digit
    }
}

fn dimensions(n: usize) -> (usize, usize, usize) {
    let log_n = usize::BITS as usize - (n - 1).leading_zeros() as usize;
    let mut len = 1usize;
    while let Some(next) = len.checked_mul(2) {
        if !n.is_multiple_of(next) {
            break;
        }
        // Division avoids overflowing when checking next^2 * log_n^2 <= n.
        let budget = n / log_n / log_n;
        if next > budget / next {
            break;
        }
        len = next;
    }
    // Every generated N above the cutoff is divisible by at least four.
    assert!(len >= 4, "invalid recursive Fermat dimensions");
    let width = n / len;
    let guard = len.trailing_zeros() as usize + 2;
    let child = (2 * width + guard).div_ceil(len) * len;
    assert!(child < n, "Fermat recursion must decrease");
    (len, width, child)
}

fn multiply_mod(a: &Big, b: &Big, n: usize, work: &mut Work, depth: usize) -> Big {
    work.depth = work.depth.max(depth);
    let ring = Fermat::new(n);
    debug_assert!(a < &ring.modulus && b < &ring.modulus);
    if a.is_zero() || b.is_zero() {
        return Big::zero();
    }
    // Canonical endpoint 2^N represents -1 and is not an ordinary N-bit chunk.
    if a.bit(n) {
        return ring.neg(b);
    }
    if b.bit(n) {
        return ring.neg(a);
    }
    if n <= BASE_BITS {
        work.schoolbook_leaves = work.schoolbook_leaves.saturating_add(1);
        return ring.reduce(&a.mul(b));
    }
    work.transform_nodes = work.transform_nodes.saturating_add(1);
    let (len, width, child) = dimensions(n);
    let coefficient_ring = Fermat::new(child);
    let twist_step = child / len;
    let mut left = Vec::with_capacity(len);
    let mut right = Vec::with_capacity(len);
    for j in 0..len {
        left.push(coefficient_ring.shift(&bit_slice(a, j * width, width), j * twist_step));
        right.push(coefficient_ring.shift(&bit_slice(b, j * width, width), j * twist_step));
    }
    transform(&mut left, &coefficient_ring, false, work);
    transform(&mut right, &coefficient_ring, false, work);
    for (a_j, b_j) in left.iter_mut().zip(right.iter()) {
        *a_j = multiply_mod(a_j, b_j, child, work, depth + 1);
    }
    drop(right);
    transform(&mut left, &coefficient_ring, true, work);
    let mut carry = Carry::zero();
    let mut out = vec![0; n.div_ceil(64)];
    let radix = pow2(width);
    for (j, coefficient) in left.into_iter().enumerate() {
        let coefficient = coefficient_ring.inverse_shift(&coefficient, j * twist_step);
        if coefficient > coefficient_ring.half {
            carry.add(true, coefficient_ring.modulus.sub(&coefficient));
        } else {
            carry.add(false, coefficient);
        }
        let digit = carry.take_digit(width, &radix);
        write_bits(&mut out, j * width, &digit);
    }
    // Sum c_j * 2^(jw) = out + carry*2^N = out - carry (mod 2^N+1).
    let out = Big::from_limbs(&out);
    debug_assert!(carry.magnitude < ring.modulus);
    if carry.negative {
        ring.add(&out, &carry.magnitude)
    } else {
        ring.sub(&out, &carry.magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(limbs: usize, seed: u64) -> Big {
        let mut state = seed;
        let values = (0..limbs)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            })
            .collect::<Vec<_>>();
        Big::from_limbs(&values)
    }

    #[test]
    fn small_native_oracle_and_zero() {
        for a in [0, 1, 7, u32::MAX as u64, u64::MAX] {
            for b in [0, 1, 13, u64::MAX] {
                assert_eq!(
                    multiply(&Big::from_u64(a), &Big::from_u64(b)),
                    Big::from_u128(a as u128 * b as u128)
                );
            }
        }
    }

    #[test]
    fn recursive_products_against_independent_schoolbook() {
        for (a_len, b_len) in [(33, 33), (65, 1), (67, 70), (180, 193), (511, 257)] {
            let a = fixture(a_len, 0x9e3779b97f4a7c15);
            let b = fixture(b_len, 0x2545f4914f6cdd1d);
            let (got, work) = multiply_with_work(&a, &b);
            assert_eq!(got, a.mul(&b), "lengths {a_len},{b_len}");
            assert!(work.transform_nodes > 0);
            assert!(work.butterflies > 0);
            assert!(work.schoolbook_leaves > 0);
        }
    }

    #[test]
    fn negative_coefficients_endpoint_and_long_carries() {
        for n in [8192, 16384, 32768] {
            let ring = Fermat::new(n);
            let near = pow2(n).sub_small(1);
            let mut work = Work::default();
            assert_eq!(
                multiply_mod(&near, &near, n, &mut work, 0),
                Big::from_u64(4)
            );
            assert_eq!(
                multiply_mod(&pow2(n), &near, n, &mut work, 0),
                Big::from_u64(2)
            );
            assert_eq!(
                multiply_mod(&pow2(n), &pow2(n), n, &mut work, 0),
                Big::from_u64(1)
            );
            let a = fixture(n / 64, 123);
            let b = fixture(n / 64, 456);
            let expected = a.mul(&b).divrem(&ring.modulus).1;
            assert_eq!(multiply_mod(&a, &b, n, &mut work, 0), expected);
        }
    }

    #[test]
    fn shifts_and_retained_transform_layout() {
        let ring = Fermat::new(96);
        for input in [Big::zero(), Big::from_u64(1), pow2(96), fixture(1, 99)] {
            for exponent in [0, 1, 63, 95, 96, 97, 191, 192, 377] {
                assert_eq!(
                    ring.shift(&input, exponent),
                    input.shl(exponent).divrem(&ring.modulus).1
                );
            }
        }
        let mut values = (0..16)
            .map(|j| Big::from_u64(j * j + 3))
            .collect::<Vec<_>>();
        let original = values.clone();
        let mut work = Work::default();
        transform(&mut values, &ring, false, &mut work);
        transform(&mut values, &ring, true, &mut work);
        assert_eq!(values, original);
    }

    #[test]
    fn non_aligned_bit_chunks_and_euclidean_carry() {
        let x = fixture(8, 777);
        for offset in [0, 1, 63, 64, 65, 127, 333, 512] {
            for width in [0, 1, 33, 64, 65, 135] {
                let expected = if width == 0 {
                    Big::zero()
                } else {
                    x.shr(offset).divrem(&pow2(width)).1
                };
                assert_eq!(bit_slice(&x, offset, width), expected);
            }
        }
        let mut c = Carry {
            negative: true,
            magnitude: Big::from_u64(257),
        };
        assert_eq!(c.take_digit(8, &pow2(8)), Big::from_u64(255));
        assert!(c.negative);
        assert_eq!(c.magnitude, Big::from_u64(2));
    }
}
