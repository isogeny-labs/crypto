//! Shared GCD, extended GCD and modular inverse for the whole library, with
//! half-GCD (subquadratic, recursive) above measured crossovers and the
//! classical algorithms below them.
//!
//! * [`poly`] — polynomials over any field ([`poly::PolyField`]): monic
//!   [`poly::gcd`], [`poly::xgcd`], [`poly::inv_mod`].  [`poly::Fp64`] is the
//!   prime field `F_p`, `p < 2^63`, with polynomials as `Vec<u64>` (constant
//!   term first).  Callers with their own coefficient type implement
//!   [`poly::PolyField`] for it.
//! * [`gf2`] — `GF(2)[x]` bit-packed in `u64` words (bit `i` of word `w` is
//!   the coefficient of `x^(64w+i)`): [`gf2::gcd`], [`gf2::xgcd`],
//!   [`gf2::inv_mod`], with carry-less multiplication.
//! * [`int`] — integers: binary GCD and extended Euclid on machine words
//!   ([`int::gcd_u64`], [`int::inv_mod_u64`], …) and, for `BigUint`, Lehmer
//!   and a recursive half-GCD ([`int::gcd_big`], [`int::xgcd_big`],
//!   [`int::modinv_big`]).
//!
//! Every public function returns the canonical answer (monic polynomial
//! gcd; non-negative integer gcd; inverse in `[0, m)`), whichever algorithm
//! runs, so callers can switch algorithms without changing results.  Bézout
//! coefficients are canonical too: the ones the classical extended
//! Euclidean algorithm returns (see each function), so half-GCD and Euclid
//! agree bit for bit.
//!
//! Crossovers ([`poly::HGCD_THRESHOLD`], [`gf2::HGCD_THRESHOLD_BITS`],
//! [`int::HGCD_THRESHOLD_BITS`], [`int::GCD_HGCD_THRESHOLD_BITS`]) come
//! from `src/bin/hgcd_bench.rs`; below them the entry points run the
//! classical algorithm (Euclid, or Lehmer for integers), so they are never
//! slower than it by more than a branch.  Half-GCD pays only for large
//! operands (polynomial degree in the thousands, integers of thousands of
//! bits); for the small operands most callers pass, the gain comes from
//! the faster classical kernels (binary GCD, one-divide inverse, Lehmer).

pub mod gf2;
pub mod hgcd;
pub mod int;
pub mod poly;
