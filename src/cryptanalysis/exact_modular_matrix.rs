//! Bounded, exact modular matrix products on integer and floating-point units.
//!
//! The public boundary contains canonical `u32` residues. Floating point is an
//! implementation of integer arithmetic here: neither rank nor a modular zero
//! is decided by a numerical tolerance. The matrix methods validate their full
//! inputs before changing an output. Scalar methods have an admitted-input
//! contract (`a, b < modulus`), like the existing [`PrimeField`] methods.
//!
//! Two established constructions are provided beside the integer reference:
//! * `Fma` retains the rounding error of a product with a fused multiply-add,
//!   estimates its quotient, and corrects the exact remainder.
//! * `Split16` splits both operands into 16-bit limbs. Each of four ordinary
//!   binary64 dot products contains at most `2^21` terms, so its absolute value
//!   is at most `2^21 * 65535^2 < 2^53`. Every integer product and partial sum
//!   is therefore exact. Limb results are reduced before recombination.
//!
//! The split implementation is a blocked native reference, not a BLAS/GPU
//! implementation or a claimed performance improvement. Prior art includes
//! Dumas--Giorgi--Pernet, *Dense Linear Algebra over Finite Fields* (2008),
//! van der Hoeven--Lecerf--Quintin, *Modular SIMD arithmetic in Mathemagix*
//! (2016), and this repository's
//! `research/ecgfp5_quotient_bw_finish_20261102/src/bwfinish.cpp`.
//! The latter's one-sided limb bounds are not reused for 31-bit primes.
//!
//! These routines do not change the default F4 engine. The caller chooses the
//! modulus: coordinate elimination uses `p`, relation linear algebra uses the
//! subgroup order, which need not be `p` or have Mersenne form.
//! The reused `Fp32` type is an integer finite-field backend; the name does not
//! mean IEEE binary32. Both floating-point candidates here use binary64.

use std::fmt;
use std::ops::AddAssign;

use super::challenge_ladder::is_prime_u64;
use super::fp_backend::{Fp32, PrimeField};

pub const MERSENNE31: u32 = (1u32 << 31) - 1;
pub const MAX_SPLIT_CHUNK: usize = 1 << 21;
pub const MAX_DIMENSION: usize = 8192;
pub const MAX_MATRIX_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_MATRIX_MULTIPLICATIONS: u64 = 64 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum Backend {
    Integer,
    Fma,
    Split16,
}

impl Backend {
    pub fn name(self) -> &'static str {
        match self {
            Self::Integer => "integer",
            Self::Fma => "fma",
            Self::Split16 => "split16",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct DenseShape {
    pub rows: usize,
    pub inner: usize,
    pub columns: usize,
}

/// Admission bounds for one call. These account for the canonical input and
/// output payloads, not allocator overhead, RSS, or storage owned by the caller
/// outside those slices. The kernels allocate no floating-point matrix copy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct MatrixLimits {
    pub max_dimension: usize,
    pub max_bytes: usize,
    pub max_multiplications: u64,
    pub split_chunk: usize,
}

impl Default for MatrixLimits {
    fn default() -> Self {
        Self {
            max_dimension: MAX_DIMENSION,
            max_bytes: MAX_MATRIX_BYTES,
            max_multiplications: MAX_MATRIX_MULTIPLICATIONS,
            split_chunk: 256,
        }
    }
}

/// Named component counts, not retired instructions or a complete cost model.
/// Logical field multiplications/additions describe the matrix expression;
/// the float counters additionally describe its main products, sums and casts.
/// Conditional remainder corrections and quotient floors are not counted.
/// Canonical add/sub corrections are excluded from `modular_reductions`, which
/// counts wide accumulator/product reductions. Inversion and field setup are
/// outside these counts. Scalar helpers do not change global counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct KernelCounts {
    pub field_multiplications: u64,
    pub field_additions: u64,
    pub float_products: u64,
    pub float_additions: u64,
    pub fused_operations: u64,
    pub float_conversions: u64,
    pub modular_reductions: u64,
    pub matrix_products: u64,
    pub entries_checked: u64,
}

impl KernelCounts {
    /// Accumulate component counts, failing rather than silently wrapping.
    pub fn merge(&mut self, other: Self) {
        macro_rules! add {
            ($field:ident) => {
                self.$field = self
                    .$field
                    .checked_add(other.$field)
                    .expect("kernel counter overflow");
            };
        }
        add!(field_multiplications);
        add!(field_additions);
        add!(float_products);
        add!(float_additions);
        add!(fused_operations);
        add!(float_conversions);
        add!(modular_reductions);
        add!(matrix_products);
        add!(entries_checked);
    }
}

impl AddAssign for KernelCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.merge(rhs);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixProduct {
    pub values: Vec<u32>,
    pub counts: KernelCounts,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatrixError {
    InvalidModulus(u64),
    InvalidLimits,
    DimensionOverflow,
    DimensionLimit,
    StorageLimit,
    WorkLimit,
    Length {
        name: &'static str,
        expected: usize,
        actual: usize,
    },
    NonCanonical {
        name: &'static str,
        index: usize,
        value: u32,
    },
    Allocation,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModulus(p) => write!(f, "modulus {p} must be an odd prime below 2^32"),
            Self::InvalidLimits => {
                write!(f, "invalid matrix limits or split chunk (allowed 1..=2^21)")
            }
            Self::DimensionOverflow => write!(f, "matrix dimension or work calculation overflowed"),
            Self::DimensionLimit => write!(f, "matrix dimension exceeds the admitted limit"),
            Self::StorageLimit => write!(
                f,
                "combined canonical matrix payload exceeds the byte limit"
            ),
            Self::WorkLimit => write!(f, "matrix product exceeds the multiplication limit"),
            Self::Length {
                name,
                expected,
                actual,
            } => write!(f, "{name} has {actual} entries; expected {expected}"),
            Self::NonCanonical { name, index, value } => {
                write!(f, "{name}[{index}]={value} is not a canonical residue")
            }
            Self::Allocation => write!(f, "unable to allocate the admitted matrix output"),
        }
    }
}

impl std::error::Error for MatrixError {}

#[derive(Clone, Debug)]
pub struct ModMatrix32 {
    field: Fp32,
    p: u32,
    pd: f64,
    reciprocal: f64,
    limits: MatrixLimits,
}

impl ModMatrix32 {
    pub fn new(p: u64) -> Result<Self, MatrixError> {
        Self::with_limits(p, MatrixLimits::default())
    }

    pub fn with_limits(p: u64, limits: MatrixLimits) -> Result<Self, MatrixError> {
        if limits.max_dimension == 0
            || limits.max_dimension > MAX_DIMENSION
            || limits.max_bytes == 0
            || limits.max_bytes > MAX_MATRIX_BYTES
            || limits.max_multiplications == 0
            || limits.max_multiplications > MAX_MATRIX_MULTIPLICATIONS
            || limits.split_chunk == 0
            || limits.split_chunk > MAX_SPLIT_CHUNK
        {
            return Err(MatrixError::InvalidLimits);
        }
        if !(3..1u64 << 32).contains(&p) || p.is_multiple_of(2) || !is_prime_u64(p) {
            return Err(MatrixError::InvalidModulus(p));
        }
        Ok(Self {
            field: Fp32::new(p),
            p: p as u32,
            pd: p as f64,
            reciprocal: 1.0 / p as f64,
            limits,
        })
    }

    pub fn modulus(&self) -> u32 {
        self.p
    }

    pub fn limits(&self) -> MatrixLimits {
        self.limits
    }

    /// Canonical inputs only. The sum is computed in `u64` before correction.
    #[inline]
    pub fn add(&self, a: u32, b: u32) -> u32 {
        debug_assert!(a < self.p && b < self.p);
        let s = u64::from(a) + u64::from(b);
        if s >= u64::from(self.p) {
            (s - u64::from(self.p)) as u32
        } else {
            s as u32
        }
    }

    /// Canonical inputs only.
    #[inline]
    pub fn sub(&self, a: u32, b: u32) -> u32 {
        debug_assert!(a < self.p && b < self.p);
        if a >= b {
            a - b
        } else {
            (u64::from(a) + u64::from(self.p) - u64::from(b)) as u32
        }
    }

    /// Canonical input only; zero has no inverse. Reuses existing arithmetic.
    #[inline]
    pub fn inv(&self, a: u32) -> Option<u32> {
        debug_assert!(a < self.p);
        self.field.inv(a)
    }

    /// Canonical inputs only. No input validation or counters in this helper.
    #[inline]
    pub fn mul(&self, a: u32, b: u32, backend: Backend) -> u32 {
        debug_assert!(a < self.p && b < self.p);
        match backend {
            Backend::Integer => self.mul_integer(a, b),
            Backend::Fma => self.mul_fma(a, b),
            Backend::Split16 => {
                let al = f64::from(a & 65535);
                let ah = f64::from(a >> 16);
                let bl = f64::from(b & 65535);
                let bh = f64::from(b >> 16);
                self.recombine([al * bl, al * bh, ah * bl, ah * bh])
            }
        }
    }

    #[inline]
    fn mul_integer(&self, a: u32, b: u32) -> u32 {
        let x = u64::from(a) * u64::from(b);
        if self.p == MERSENNE31 {
            // x < p^2 < 2^62; one fold is below 2p.
            let r = (x & u64::from(MERSENNE31)) + (x >> 31);
            if r >= u64::from(MERSENNE31) {
                (r - u64::from(MERSENNE31)) as u32
            } else {
                r as u32
            }
        } else {
            self.field.reduce(x)
        }
    }

    #[inline]
    fn reduce_u64(&self, x: u64) -> u32 {
        if self.p == MERSENNE31 {
            // This helper also admits wide limb sums, so fold twice.
            let r = (x & u64::from(MERSENNE31)) + (x >> 31);
            let r = (r & u64::from(MERSENNE31)) + (r >> 31);
            if r >= u64::from(MERSENNE31) {
                (r - u64::from(MERSENNE31)) as u32
            } else {
                r as u32
            }
        } else {
            self.field.reduce(x)
        }
    }

    #[inline]
    fn mul_fma(&self, a: u32, b: u32) -> u32 {
        let a = f64::from(a);
        let b = f64::from(b);
        let h = a * b;
        let lo = a.mul_add(b, -h);
        let q = (h * self.reciprocal).floor();
        // With u=2^-53 and p < 2^32, quotient-estimation error is bounded
        // by p*(3u + 3u^2 + u^3) < 1. The quotient
        // differs by at most one. Both cancellation residuals are integers
        // below 2^34 in magnitude and their sum is exact in binary64.
        let mut r = (-q).mul_add(self.pd, h) + lo;
        if r < 0.0 {
            r += self.pd;
        }
        if r >= self.pd {
            r -= self.pd;
        }
        debug_assert!(r >= 0.0 && r < self.pd && r.fract() == 0.0);
        r as u32
    }

    #[inline]
    fn recombine(&self, sums: [f64; 4]) -> u32 {
        debug_assert!(sums
            .iter()
            .all(|&s| s >= 0.0 && s < (1u64 << 53) as f64 && s.fract() == 0.0));
        let r = sums.map(|s| u64::from(self.reduce_u64(s as u64)));
        if self.p == MERSENNE31 {
            // 2^32 == 2 (mod p). Reduced limbs keep this below 2^49.
            self.reduce_u64(r[0] + 65536 * (r[1] + r[2]) + 2 * r[3])
        } else {
            // Horner recombination keeps each integer below 2^49 even
            // at the largest admitted 32-bit prime.
            let high = u64::from(self.reduce_u64(65536 * r[3] + r[1] + r[2]));
            self.reduce_u64(65536 * high + r[0])
        }
    }

    fn checked_shape(&self, shape: DenseShape) -> Result<(usize, usize, usize, u64), MatrixError> {
        let DenseShape {
            rows,
            inner,
            columns,
        } = shape;
        let a = rows
            .checked_mul(inner)
            .ok_or(MatrixError::DimensionOverflow)?;
        let b = inner
            .checked_mul(columns)
            .ok_or(MatrixError::DimensionOverflow)?;
        let c = rows
            .checked_mul(columns)
            .ok_or(MatrixError::DimensionOverflow)?;
        let entries = a
            .checked_add(b)
            .and_then(|x| x.checked_add(c))
            .ok_or(MatrixError::DimensionOverflow)?;
        let bytes = entries
            .checked_mul(std::mem::size_of::<u32>())
            .ok_or(MatrixError::DimensionOverflow)?;
        let work = u64::try_from(c)
            .ok()
            .and_then(|x| x.checked_mul(u64::try_from(inner).ok()?))
            .ok_or(MatrixError::DimensionOverflow)?;
        if [rows, inner, columns]
            .into_iter()
            .any(|d| d > self.limits.max_dimension)
        {
            return Err(MatrixError::DimensionLimit);
        }
        if bytes > self.limits.max_bytes {
            return Err(MatrixError::StorageLimit);
        }
        if work > self.limits.max_multiplications {
            return Err(MatrixError::WorkLimit);
        }
        Ok((a, b, c, work))
    }

    fn validate(
        &self,
        name: &'static str,
        values: &[u32],
        expected: usize,
    ) -> Result<(), MatrixError> {
        if values.len() != expected {
            return Err(MatrixError::Length {
                name,
                expected,
                actual: values.len(),
            });
        }
        if let Some((index, &value)) = values.iter().enumerate().find(|(_, v)| **v >= self.p) {
            return Err(MatrixError::NonCanonical { name, index, value });
        }
        Ok(())
    }

    pub fn mul_dense(
        &self,
        a: &[u32],
        b: &[u32],
        shape: DenseShape,
        backend: Backend,
    ) -> Result<MatrixProduct, MatrixError> {
        let (na, nb, nc, work) = self.checked_shape(shape)?;
        self.validate("A", a, na)?;
        self.validate("B", b, nb)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(nc)
            .map_err(|_| MatrixError::Allocation)?;
        values.resize(nc, 0);
        let mut counts = self.apply_product(&mut values, a, b, shape, backend, false, work);
        counts.entries_checked = (na + nb) as u64;
        Ok(MatrixProduct { values, counts })
    }

    /// `C <- C - A*B`. All dimensions, bounds, and residues are validated
    /// before the first output write. No temporary dense product is allocated.
    pub fn sub_mul_dense(
        &self,
        c: &mut [u32],
        a: &[u32],
        b: &[u32],
        shape: DenseShape,
        backend: Backend,
    ) -> Result<KernelCounts, MatrixError> {
        let (na, nb, nc, work) = self.checked_shape(shape)?;
        self.validate("A", a, na)?;
        self.validate("B", b, nb)?;
        self.validate("C", c, nc)?;
        let mut counts = self.apply_product(c, a, b, shape, backend, true, work);
        counts.entries_checked = (na + nb + nc) as u64;
        Ok(counts)
    }

    fn apply_product(
        &self,
        c: &mut [u32],
        a: &[u32],
        b: &[u32],
        shape: DenseShape,
        backend: Backend,
        subtract: bool,
        work: u64,
    ) -> KernelCounts {
        let DenseShape {
            rows,
            inner,
            columns,
        } = shape;
        let mut counts = KernelCounts {
            field_multiplications: work,
            field_additions: work,
            matrix_products: 1,
            ..KernelCounts::default()
        };
        if backend == Backend::Split16 {
            let chunks = inner.div_ceil(self.limits.split_chunk);
            let completed_chunks = (rows as u64) * (columns as u64) * (chunks as u64);
            counts.float_products = 4 * work;
            counts.float_additions = 4 * work;
            counts.float_conversions = 4 * work + 4 * completed_chunks;
            counts.modular_reductions = completed_chunks * if self.p == MERSENNE31 { 5 } else { 6 };
            for i in 0..rows {
                for j in 0..columns {
                    for start in (0..inner).step_by(self.limits.split_chunk) {
                        let end = inner.min(start + self.limits.split_chunk);
                        let mut sums = [0.0; 4];
                        for t in start..end {
                            let x = a[i * inner + t];
                            let y = b[t * columns + j];
                            let xl = f64::from(x & 65535);
                            let xh = f64::from(x >> 16);
                            let yl = f64::from(y & 65535);
                            let yh = f64::from(y >> 16);
                            sums[0] += xl * yl;
                            sums[1] += xl * yh;
                            sums[2] += xh * yl;
                            sums[3] += xh * yh;
                        }
                        let product = self.recombine(sums);
                        let dest = &mut c[i * columns + j];
                        *dest = if subtract {
                            self.sub(*dest, product)
                        } else {
                            self.add(*dest, product)
                        };
                    }
                }
            }
        } else {
            counts.modular_reductions = work;
            if backend == Backend::Fma {
                counts.float_products = 2 * work;
                counts.float_additions = work;
                counts.fused_operations = 2 * work;
                counts.float_conversions = 3 * work;
            }
            // The same loop ordering is used for the integer and FMA arms.
            for i in 0..rows {
                for t in 0..inner {
                    let x = a[i * inner + t];
                    for j in 0..columns {
                        let product = self.mul(x, b[t * columns + j], backend);
                        let dest = &mut c[i * columns + j];
                        *dest = if subtract {
                            self.sub(*dest, product)
                        } else {
                            self.add(*dest, product)
                        };
                    }
                }
            }
        }
        counts
    }

    /// `dst <- dst - factor*pivot`, with canonical inputs and equal lengths.
    /// Scalar helpers admit canonical values without checking; this row-level
    /// boundary validates everything before mutating `dst`.
    pub fn row_sub_assign(
        &self,
        dst: &mut [u32],
        pivot: &[u32],
        factor: u32,
        backend: Backend,
    ) -> Result<KernelCounts, MatrixError> {
        let shape = DenseShape {
            rows: 1,
            inner: 1,
            columns: dst.len(),
        };
        self.sub_mul_dense(dst, &[factor], pivot, shape, backend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BACKENDS: [Backend; 3] = [Backend::Integer, Backend::Fma, Backend::Split16];

    fn oracle(a: u32, b: u32, p: u32) -> u32 {
        ((u128::from(a) * u128::from(b)) % u128::from(p)) as u32
    }

    fn random(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    fn matrix_oracle(a: &[u32], b: &[u32], s: DenseShape, p: u32) -> Vec<u32> {
        let mut c = vec![0; s.rows * s.columns];
        for i in 0..s.rows {
            for j in 0..s.columns {
                let sum: u128 = (0..s.inner)
                    .map(|t| u128::from(a[i * s.inner + t]) * u128::from(b[t * s.columns + j]))
                    .sum();
                c[i * s.columns + j] = (sum % u128::from(p)) as u32;
            }
        }
        c
    }

    #[test]
    fn exhaustive_small_prime_products() {
        for p in [3, 5, 7, 17, 31, 127, 257] {
            let field = ModMatrix32::new(p).unwrap();
            for a in 0..p as u32 {
                for b in 0..p as u32 {
                    for backend in BACKENDS {
                        assert_eq!(
                            field.mul(a, b, backend),
                            oracle(a, b, p as u32),
                            "p={p}, a={a}, b={b}, {backend:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn wide_prime_boundaries_and_seeded_products() {
        for p in [65521, 131071, u64::from(MERSENNE31), 4_294_967_291] {
            let field = ModMatrix32::new(p).unwrap();
            let values: Vec<_> = [
                0,
                1,
                2,
                65535,
                65536,
                65537,
                (1 << 26) - 1,
                (1 << 26) + 1,
                p / 2 - 1,
                p / 2,
                p / 2 + 1,
                p - 3,
                p - 2,
                p - 1,
            ]
            .into_iter()
            .filter(|&x| x < p)
            .map(|x| x as u32)
            .collect();
            for &a in &values {
                for &b in &values {
                    for backend in BACKENDS {
                        assert_eq!(field.mul(a, b, backend), oracle(a, b, p as u32));
                    }
                }
            }
            let mut seed = 0x6a09_e667_f3bc_c909;
            for _ in 0..20000 {
                let a = (random(&mut seed) % p) as u32;
                let b = (random(&mut seed) % p) as u32;
                for backend in BACKENDS {
                    assert_eq!(field.mul(a, b, backend), oracle(a, b, p as u32));
                }
            }
        }
    }

    #[test]
    fn naive_double_product_regression() {
        let p = MERSENNE31;
        let a = p - 1;
        let rounded = (f64::from(a) * f64::from(a)) as u64;
        assert_eq!(rounded % u64::from(p), u64::from(p - 3));
        let field = ModMatrix32::new(u64::from(p)).unwrap();
        for backend in BACKENDS {
            assert_eq!(field.mul(a, a, backend), 1);
        }
        // The reciprocal estimate rounds to one quotient too high for this
        // pair, giving pre-correction r=-1. The add-p branch must restore p-1.
        let a = p - 2;
        let b = (p + 1) / 2;
        let h = f64::from(a) * f64::from(b);
        let q = (h * field.reciprocal).floor();
        let before_correction = (-q).mul_add(field.pd, h) + f64::from(a).mul_add(f64::from(b), -h);
        assert_eq!(before_correction, -1.0);
        assert_eq!(field.mul(a, b, Backend::Fma), p - 1);
    }

    #[test]
    fn rectangular_products_subtraction_and_chunk_boundaries() {
        for p in [31, u64::from(MERSENNE31), 4_294_967_291] {
            for chunk in [1, 7, 16, 65, 256, MAX_SPLIT_CHUNK] {
                let field = ModMatrix32::with_limits(
                    p,
                    MatrixLimits {
                        split_chunk: chunk,
                        ..MatrixLimits::default()
                    },
                )
                .unwrap();
                for inner in [0, 1, 7, 16, 17, 64, 65, 257] {
                    let s = DenseShape {
                        rows: 3,
                        inner,
                        columns: 5,
                    };
                    let mut seed = 0xbb67_ae85_84ca_a73b;
                    let a: Vec<_> = (0..s.rows * inner)
                        .map(|i| {
                            if i % 7 == 0 {
                                0
                            } else {
                                (random(&mut seed) % p) as u32
                            }
                        })
                        .collect();
                    let b: Vec<_> = (0..inner * s.columns)
                        .map(|i| {
                            if i % 11 == 0 {
                                p as u32 - 1
                            } else {
                                (random(&mut seed) % p) as u32
                            }
                        })
                        .collect();
                    let want = matrix_oracle(&a, &b, s, p as u32);
                    for backend in BACKENDS {
                        let product = field.mul_dense(&a, &b, s, backend).unwrap();
                        assert_eq!(product.values, want);
                        assert_eq!(
                            product.counts.field_multiplications,
                            (s.rows * inner * s.columns) as u64
                        );
                        let mut c = vec![p as u32 - 1; s.rows * s.columns];
                        field.sub_mul_dense(&mut c, &a, &b, s, backend).unwrap();
                        let expected: Vec<_> = want
                            .iter()
                            .map(|&v| {
                                ((u128::from(p - 1) + u128::from(p) - u128::from(v))
                                    % u128::from(p)) as u32
                            })
                            .collect();
                        assert_eq!(c, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn empty_dimensions_and_dependent_rows() {
        let field = ModMatrix32::new(u64::from(MERSENNE31)).unwrap();
        for backend in BACKENDS {
            for shape in [
                DenseShape {
                    rows: 0,
                    inner: 3,
                    columns: 2,
                },
                DenseShape {
                    rows: 3,
                    inner: 2,
                    columns: 0,
                },
            ] {
                let a = vec![1; shape.rows * shape.inner];
                let b = vec![1; shape.inner * shape.columns];
                assert!(field
                    .mul_dense(&a, &b, shape, backend)
                    .unwrap()
                    .values
                    .is_empty());
            }
            let pivot = [1, 7, MERSENNE31 - 1, 0];
            let mut duplicate: Vec<_> = pivot.iter().map(|&x| oracle(x, 13, MERSENNE31)).collect();
            field
                .row_sub_assign(&mut duplicate, &pivot, 13, backend)
                .unwrap();
            assert_eq!(duplicate, [0; 4]);
            let a = [1, 2, 1, 2, 0, 0];
            let b = [MERSENNE31 - 1, 17, 9, 3, 5, 2];
            let s = DenseShape {
                rows: 3,
                inner: 2,
                columns: 3,
            };
            let c = field.mul_dense(&a, &b, s, backend).unwrap().values;
            assert_eq!(&c[0..3], &c[3..6]);
            assert_eq!(&c[6..9], &[0, 0, 0]);
        }
    }

    #[test]
    fn rejects_invalid_moduli_limits_dimensions_and_values_before_write() {
        for p in [0, 1, 2, 4, 9, 65535, 1u64 << 32, u64::MAX] {
            assert!(matches!(
                ModMatrix32::new(p),
                Err(MatrixError::InvalidModulus(_))
            ));
        }
        for chunk in [0, MAX_SPLIT_CHUNK + 1] {
            assert!(matches!(
                ModMatrix32::with_limits(
                    31,
                    MatrixLimits {
                        split_chunk: chunk,
                        ..MatrixLimits::default()
                    }
                ),
                Err(MatrixError::InvalidLimits)
            ));
        }
        let field = ModMatrix32::new(31).unwrap();
        assert_eq!(
            field
                .mul_dense(
                    &[],
                    &[],
                    DenseShape {
                        rows: usize::MAX,
                        inner: 2,
                        columns: 2
                    },
                    Backend::Integer
                )
                .unwrap_err(),
            MatrixError::DimensionOverflow
        );
        assert_eq!(
            field
                .mul_dense(
                    &[],
                    &[],
                    DenseShape {
                        rows: MAX_DIMENSION + 1,
                        inner: 0,
                        columns: 0
                    },
                    Backend::Integer
                )
                .unwrap_err(),
            MatrixError::DimensionLimit
        );
        assert_eq!(
            field
                .mul_dense(
                    &[],
                    &[],
                    DenseShape {
                        rows: 8192,
                        inner: 8192,
                        columns: 1
                    },
                    Backend::Integer
                )
                .unwrap_err(),
            MatrixError::StorageLimit
        );
        let limited = ModMatrix32::with_limits(
            31,
            MatrixLimits {
                max_multiplications: 7,
                ..MatrixLimits::default()
            },
        )
        .unwrap();
        assert_eq!(
            limited
                .mul_dense(
                    &[1; 4],
                    &[1; 4],
                    DenseShape {
                        rows: 2,
                        inner: 2,
                        columns: 2
                    },
                    Backend::Integer
                )
                .unwrap_err(),
            MatrixError::WorkLimit
        );
        let s = DenseShape {
            rows: 1,
            inner: 1,
            columns: 2,
        };
        for backend in BACKENDS {
            let mut c = [4, 5];
            assert!(matches!(
                field.sub_mul_dense(&mut c, &[1], &[2, 31], s, backend),
                Err(MatrixError::NonCanonical {
                    name: "B",
                    index: 1,
                    value: 31
                })
            ));
            assert_eq!(c, [4, 5]);
            assert!(matches!(
                field.sub_mul_dense(&mut c, &[1], &[2], s, backend),
                Err(MatrixError::Length { name: "B", .. })
            ));
            assert_eq!(c, [4, 5]);
            assert!(matches!(
                field.row_sub_assign(&mut c, &[1, 2], 31, backend),
                Err(MatrixError::NonCanonical { name: "A", .. })
            ));
            assert_eq!(c, [4, 5]);
            let mut invalid_c = [4, 31];
            assert!(matches!(
                field.sub_mul_dense(&mut invalid_c, &[1], &[2, 3], s, backend),
                Err(MatrixError::NonCanonical {
                    name: "C",
                    index: 1,
                    value: 31
                })
            ));
            assert_eq!(invalid_c, [4, 31]);
        }
    }

    #[test]
    fn maximal_limb_sum_bound_and_count_aggregation() {
        // Exercise a sum close to 2^53 without allocating a giant matrix.
        let maximum = (MAX_SPLIT_CHUNK as u64) * 65535u64 * 65535u64;
        assert!(maximum < 1u64 << 53);
        let mut sum = 0.0;
        for _ in 0..MAX_SPLIT_CHUNK {
            sum += 65535.0 * 65535.0;
        }
        assert_eq!(sum as u64, maximum);
        for p in [u64::from(MERSENNE31), 4_294_967_291] {
            let field = ModMatrix32::new(p).unwrap();
            let got = field.recombine([sum; 4]);
            let limb = u128::from(maximum);
            let want = ((limb + (1u128 << 16) * (limb + limb) + (1u128 << 32) * limb)
                % u128::from(p)) as u32;
            assert_eq!(got, want);
            let mut counts = KernelCounts::default();
            for backend in BACKENDS {
                let product = field
                    .mul_dense(
                        &[1, 2],
                        &[3, 4],
                        DenseShape {
                            rows: 1,
                            inner: 2,
                            columns: 1,
                        },
                        backend,
                    )
                    .unwrap();
                counts += product.counts;
            }
            assert_eq!(counts.field_multiplications, 6);
            assert_eq!(counts.matrix_products, 3);
        }
    }
}
