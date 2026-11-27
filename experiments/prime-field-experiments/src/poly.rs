use std::mem::MaybeUninit;

use anyhow::{ensure, Result};
use flint_sys::nmod_poly::*;
use flint_sys::nmod_poly_factor::{nmod_poly_factor_clear, nmod_poly_factor_init, nmod_poly_roots};
use flint_sys::nmod_types::{nmod_poly_factor_struct, nmod_poly_struct};
use flint_sys::ulong_extras::{n_invmod, n_is_prime, n_sqrtmod};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimeField(u64);

impl PrimeField {
    pub fn new(p: u64) -> Result<Self> {
        ensure!(
            p >= 2 && unsafe { n_is_prime(p) } != 0,
            "expected a prime modulus"
        );
        Ok(Self(p))
    }

    pub fn modulus(self) -> u64 {
        self.0
    }

    pub(crate) fn reduce(self, coefficient: i128) -> u64 {
        coefficient.rem_euclid(i128::from(self.0)) as u64
    }

    pub(crate) fn add(self, a: u64, b: u64) -> u64 {
        ((u128::from(a) + u128::from(b)) % u128::from(self.0)) as u64
    }

    pub(crate) fn neg(self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.0 - a
        }
    }

    pub(crate) fn mul(self, a: u64, b: u64) -> u64 {
        ((u128::from(a) * u128::from(b)) % u128::from(self.0)) as u64
    }

    pub(crate) fn inv(self, a: u64) -> u64 {
        assert!(a != 0 && a < self.0);
        // SAFETY: the nonzero residue is invertible in this prime field.
        unsafe { n_invmod(a, self.0) }
    }

    pub(crate) fn pow(self, mut base: u64, mut exponent: usize) -> u64 {
        let mut value = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                value = self.mul(value, base);
            }
            base = self.mul(base, base);
            exponent >>= 1;
        }
        value
    }

    pub(crate) fn sqrt(self, a: u64) -> Option<u64> {
        debug_assert!(self.0 > 2 && a < self.0);
        // SAFETY: FLINT takes an odd prime and a reduced residue.
        let root = unsafe { n_sqrtmod(a, self.0) };
        (root != 0 || a == 0).then_some(root)
    }
}

/// An owned FLINT polynomial. The field proof and native allocation travel
/// together. Each Rayon job creates its own polynomials.
pub(crate) struct Poly {
    raw: nmod_poly_struct,
    pub(crate) field: PrimeField,
}

type Unary = unsafe extern "C" fn(*mut nmod_poly_struct, *const nmod_poly_struct);
type Binary =
    unsafe extern "C" fn(*mut nmod_poly_struct, *const nmod_poly_struct, *const nmod_poly_struct);

impl Poly {
    fn with_capacity(field: PrimeField, capacity: usize) -> Self {
        let mut raw = MaybeUninit::uninit();
        // SAFETY: FLINT initializes the struct and owns the coefficient allocation.
        unsafe {
            nmod_poly_init2(
                raw.as_mut_ptr(),
                field.modulus(),
                capacity
                    .try_into()
                    .expect("polynomial capacity fits in slong"),
            );
            Self {
                raw: raw.assume_init(),
                field,
            }
        }
    }

    pub(crate) fn new(field: PrimeField, coefficients: &[u64]) -> Self {
        let mut poly = Self::with_capacity(field, coefficients.len());
        for (i, &coefficient) in coefficients.iter().enumerate() {
            // SAFETY: poly is initialized and set_coeff_ui reduces the coefficient.
            unsafe { nmod_poly_set_coeff_ui(&mut poly.raw, i as i64, coefficient) };
        }
        poly
    }

    pub(crate) fn from_signed(field: PrimeField, coefficients: &[i128]) -> Self {
        let reduced: Vec<_> = coefficients.iter().map(|&c| field.reduce(c)).collect();
        Self::new(field, &reduced)
    }

    pub(crate) fn degree(&self) -> i64 {
        self.raw.length - 1
    }

    pub(crate) fn coeff(&self, i: usize) -> u64 {
        if i >= self.raw.length as usize {
            return 0;
        }
        // SAFETY: this index is within FLINT's initialized coefficient range.
        unsafe { *self.raw.coeffs.add(i) }
    }

    pub(crate) fn eval(&self, x: u64) -> u64 {
        // SAFETY: the polynomial is initialized and the evaluation point is reduced.
        unsafe { nmod_poly_evaluate_nmod(&self.raw, x % self.field.modulus()) }
    }

    fn unary(&self, op: Unary) -> Self {
        let mut out = Self::with_capacity(self.field, 0);
        // SAFETY: distinct initialized input and output over the same field.
        unsafe { op(&mut out.raw, &self.raw) };
        out
    }

    fn binary(&self, other: &Self, op: Binary) -> Self {
        assert_eq!(self.field, other.field);
        let mut out = Self::with_capacity(self.field, 0);
        // SAFETY: the destination is distinct and all operands have the same field.
        unsafe { op(&mut out.raw, &self.raw, &other.raw) };
        out
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        self.binary(other, nmod_poly_add)
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        self.binary(other, nmod_poly_sub)
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        self.binary(other, nmod_poly_mul)
    }

    pub(crate) fn gcd(&self, other: &Self) -> Self {
        self.binary(other, nmod_poly_gcd)
    }

    pub(crate) fn quotient(&self, divisor: &Self) -> Self {
        assert!(divisor.degree() >= 0);
        self.binary(divisor, nmod_poly_div)
    }

    pub(crate) fn derivative(&self) -> Self {
        self.unary(nmod_poly_derivative)
    }

    pub(crate) fn scale(&self, scalar: u64) -> Self {
        let mut out = Self::with_capacity(self.field, 0);
        // SAFETY: both polynomials are initialized. Pass a reduced scalar.
        unsafe {
            nmod_poly_scalar_mul_nmod(&mut out.raw, &self.raw, scalar % self.field.modulus());
        }
        out
    }

    pub(crate) fn shift(&self, center: u64) -> Self {
        let mut out = Self::with_capacity(self.field, 0);
        // SAFETY: FLINT computes f(X + center) over the common prime field.
        unsafe {
            nmod_poly_taylor_shift(&mut out.raw, &self.raw, center % self.field.modulus());
        }
        out
    }

    /// Compute (X + shift)^exponent modulo this polynomial.
    pub(crate) fn linear_power_mod(&self, shift: u64, exponent: u64) -> Self {
        assert!(self.degree() > 0);
        let mut out = Self::with_capacity(self.field, 0);
        if self.degree() < 32 {
            let base = Self::new(self.field, &[shift, 1]);
            // SAFETY: the modulus has positive degree and all operands share a field.
            unsafe {
                nmod_poly_powmod_ui_binexp(&mut out.raw, &base.raw, exponent, &self.raw);
            }
            return out;
        }

        let translated;
        let modulus = if shift == 0 {
            self
        } else {
            translated = self.shift(self.field.neg(shift));
            &translated
        };
        let length = modulus.raw.length;
        let mut reversed = Self::with_capacity(self.field, 0);
        let mut inverse = Self::with_capacity(self.field, 0);
        // SAFETY: reversal makes the nonzero leading coefficient the constant
        // coefficient. Its truncated series inverse exists in the prime field.
        unsafe {
            nmod_poly_reverse(&mut reversed.raw, &modulus.raw, length);
            nmod_poly_inv_series(&mut inverse.raw, &reversed.raw, length);
            nmod_poly_powmod_x_ui_preinv(&mut out.raw, exponent, &modulus.raw, &inverse.raw);
        }
        if shift == 0 {
            out
        } else {
            out.shift(shift)
        }
    }

    pub(crate) fn native_roots(&self) -> Vec<u64> {
        if self.degree() <= 0 {
            return Vec::new();
        }
        let mut factors = MaybeUninit::uninit();
        // SAFETY: the nonzero polynomial is over a validated prime field. FLINT
        // initializes the factor list and returns monic linear factors.
        unsafe {
            nmod_poly_factor_init(factors.as_mut_ptr());
            let mut factors = Factors(factors.assume_init());
            nmod_poly_roots(&mut factors.0, &self.raw, 0);
            let mut roots = Vec::with_capacity(factors.0.num as usize);
            for index in 0..factors.0.num as usize {
                let factor = factors.0.p.add(index);
                roots.push(self.field.neg(nmod_poly_get_coeff_ui(factor, 0)));
            }
            roots.sort_unstable();
            roots
        }
    }
}

struct Factors(nmod_poly_factor_struct);

impl Drop for Factors {
    fn drop(&mut self) {
        // SAFETY: this factor list was initialized by FLINT and is owned here.
        unsafe { nmod_poly_factor_clear(&mut self.0) };
    }
}

impl PartialEq for Poly {
    fn eq(&self, other: &Self) -> bool {
        // SAFETY: both polynomials are initialized and their moduli agree.
        self.field == other.field && unsafe { nmod_poly_equal(&self.raw, &other.raw) != 0 }
    }
}

impl Drop for Poly {
    fn drop(&mut self) {
        // SAFETY: this is the unique owner of the initialized FLINT allocation.
        unsafe { nmod_poly_clear(&mut self.raw) };
    }
}
