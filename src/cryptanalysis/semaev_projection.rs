//! Exact projection joins for prime-field Semaev decomposition systems.
//!
//! This module replaces a Gröbner-basis decision for the three-summand
//! chain by a join on the roots of its two quadratic `S_3` links.  A root in
//! `F_p` is keyed by its value.  The conjugate roots of an irreducible
//! quadratic are keyed by the coefficients of their monic minimal
//! polynomial, so the join never has to construct `F_{p^2}`.  For four
//! summands, the same pair projection gives the balanced tree
//!
//! ```text
//! S_3(x1,x2,u) = S_3(x3,x4,v) = S_3(u,v,target) = 0.
//! ```
//!
//! ### Rational-discriminant batch inversion (Lead P0-1a)
//!
//! For any short-Weierstrass curve `E : y^2 = f(x) = x^3 + a*x + b`, the
//! non-monic discriminant of `S_3(x1, x2, X) = c2*X^2 + c1*X + c0` satisfies
//! the polynomial identity
//!
//! ```text
//! c1^2 - 4*c2*c0 == 16 * f(x1) * f(x2).
//! ```
//!
//! Whenever `P_1 = (x1, y1)` and `P_2 = (x2, y2)` are `F_p`-rational points on
//! `E` with `x1 != x2`, the discriminant is `(4*y1*y2)^2 (mod p)` — already a
//! square in `F_p`.  Caching canonical `y_i` once per factor-base abscissa
//! (and `y_u` via the group law for every finite pair root `u`) eliminates all
//! per-pair and per-query Tonelli–Shanks square roots, while Montgomery
//! simultaneous inversion (`batch_inv`) over `2*(x1 - x2)^2` replaces the two
//! scalar inversions per pair/query by a single shared inversion per batch.
//! Off-curve inputs fall back safely without sacrificing exact root parity.
//!
//! The construction is exact, but it is still a table algorithm: it stores
//! at most `min(p, O(B^2))` finite pair roots for a factor base of size `B`.
//! It is therefore an oracle-stage alternative, not by itself an asymptotic
//! ECDLP claim.

use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use super::ec_index_calculus::sqrt_mod_p;
use super::pkm_smooth_tower::{addm, invm, mulm, sqrtm, subm, Curve};
use super::prime_fast::{Fp64, MAX_BITS};
use crate::ecc::field::FieldElement;

const BUILD_BATCH: usize = 2048;
const QUERY_BATCH: usize = 64;

/// A root represented without choosing an extension-field model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum QuadraticRootKey {
    /// A root in the base field.
    Base(u64),
    /// Either conjugate root of `X^2 + linear*X + constant`.
    Irreducible { linear: u64, constant: u64 },
}

/// Multi-limb counterpart of [`QuadraticRootKey`] for `BigUint` prime fields.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum BigQuadraticRootKey {
    Base(BigUint),
    Irreducible { linear: BigUint, constant: BigUint },
}

/// The roots of a polynomial of degree at most two.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum QuadraticRoots {
    None,
    Finite(Vec<QuadraticRootKey>),
    All,
}

/// Multi-limb counterpart of [`QuadraticRoots`] for `BigUint` prime fields.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum BigQuadraticRoots {
    None,
    Finite(Vec<BigQuadraticRootKey>),
    All,
}

/// Exact counted `F_p` primitive operations for before/after cost comparison.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct FpOpCounts {
    pub adds: u64,
    pub subs: u64,
    pub muls: u64,
    pub sqrs: u64,
    pub invs: u64,
    pub sqrts: u64,
}

impl FpOpCounts {
    pub fn total_mul_equiv_no_inv_sqrt(&self) -> u64 {
        self.muls + self.sqrs
    }

    pub fn add_assign(&mut self, rhs: Self) {
        self.adds += rhs.adds;
        self.subs += rhs.subs;
        self.muls += rhs.muls;
        self.sqrs += rhs.sqrs;
        self.invs += rhs.invs;
        self.sqrts += rhs.sqrts;
    }
}

/// Exact, structural work done while building the pair projection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct BuildStats {
    pub unordered_pairs: u64,
    pub root_keys: u64,
    pub base_field_root_keys: u64,
    pub irreducible_root_keys: u64,
    pub universal_pairs: u64,
    pub distinct_keys: usize,
    pub distinct_base_roots: usize,
}

/// Exact work done by one online query.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct QueryStats {
    pub quadratics: u64,
    pub root_keys: u64,
    pub lookups: u64,
    pub compatible_keys: u64,
    pub cancellation_checks: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct ThreeSumWitness {
    pub x: [u64; 3],
    /// `None` denotes the point at infinity in the cancellation branch.
    pub auxiliary: Option<QuadraticRootKey>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FourSumWitness {
    pub x: [u64; 4],
    /// `None` in one slot denotes a cancelling pair and the point at infinity.
    pub auxiliaries: [Option<u64>; 2],
}

/// `S_3(x1,x2,X)`, low coefficient first.
pub fn s3_quadratic(curve: &Curve, x1: u64, x2: u64) -> [u64; 3] {
    let p = curve.p;
    let x1 = x1 % p;
    let x2 = x2 % p;
    let d = subm(x1, x2, p);
    let s = addm(x1, x2, p);
    let product = mulm(x1, x2, p);
    let c2 = mulm(d, d, p);
    let c1 = subm(
        0,
        mulm(
            2 % p,
            addm(
                mulm(s, addm(product, curve.a % p, p), p),
                mulm(2 % p, curve.b % p, p),
                p,
            ),
            p,
        ),
        p,
    );
    let shifted = subm(product, curve.a % p, p);
    let c0 = subm(
        mulm(shifted, shifted, p),
        mulm(4 % p, mulm(curve.b % p, s, p), p),
        p,
    );
    [c0, c1, c2]
}

pub fn eval_quadratic(q: [u64; 3], x: u64, p: u64) -> u64 {
    let x = x % p;
    addm(
        q[0] % p,
        mulm(x, addm(q[1] % p, mulm(q[2] % p, x, p), p), p),
        p,
    )
}

/// Classify the roots of `c0 + c1 X + c2 X^2` over the algebraic closure.
pub fn quadratic_root_keys(q: [u64; 3], p: u64) -> QuadraticRoots {
    let mut dummy = FpOpCounts::default();
    quadratic_root_keys_counted(q, p, &mut dummy)
}

/// Reference `quadratic_root_keys` with exact `F_p` operation accounting.
pub fn quadratic_root_keys_counted(
    q: [u64; 3],
    p: u64,
    ops: &mut FpOpCounts,
) -> QuadraticRoots {
    let [c0, c1, c2] = q.map(|x| x % p);
    if c2 == 0 {
        if c1 == 0 {
            return if c0 == 0 {
                QuadraticRoots::All
            } else {
                QuadraticRoots::None
            };
        }
        ops.subs += 1;
        ops.invs += 1;
        ops.muls += 1;
        return QuadraticRoots::Finite(vec![QuadraticRootKey::Base(mulm(
            subm(0, c0, p),
            invm(c1, p),
            p,
        ))]);
    }

    ops.invs += 1;
    let inv_lead = invm(c2, p);
    ops.muls += 2;
    let linear = mulm(c1, inv_lead, p);
    let constant = mulm(c0, inv_lead, p);
    ops.sqrs += 1;
    ops.muls += 1;
    ops.subs += 1;
    let discriminant = subm(mulm(linear, linear, p), mulm(4 % p, constant, p), p);
    ops.sqrts += 1;
    if let Some(root_disc) = sqrtm(discriminant, p) {
        ops.invs += 1;
        let inv_two = invm(2 % p, p);
        ops.subs += 1;
        let minus_linear = subm(0, linear, p);
        ops.adds += 1;
        ops.subs += 1;
        ops.muls += 2;
        let r0 = mulm(addm(minus_linear, root_disc, p), inv_two, p);
        let r1 = mulm(subm(minus_linear, root_disc, p), inv_two, p);
        let mut roots = vec![QuadraticRootKey::Base(r0)];
        if r1 != r0 {
            roots.push(QuadraticRootKey::Base(r1));
        }
        QuadraticRoots::Finite(roots)
    } else {
        QuadraticRoots::Finite(vec![QuadraticRootKey::Irreducible { linear, constant }])
    }
}

fn s3_quadratic_counted(curve: &Curve, x1: u64, x2: u64, ops: &mut FpOpCounts) -> [u64; 3] {
    ops.subs += 4;
    ops.adds += 3;
    ops.muls += 5;
    ops.sqrs += 2;
    s3_quadratic(curve, x1, x2)
}

#[inline]
fn canonical_y(y: u64, p: u64) -> u64 {
    let neg = subm(0, y % p, p);
    y.min(neg)
}

fn lift_canonical_counted(curve: &Curve, x: u64, ops: &mut FpOpCounts) -> Option<u64> {
    let p = curve.p;
    let x = x % p;
    ops.sqrs += 1;
    ops.muls += 2;
    ops.adds += 2;
    let rhs = curve.rhs(x);
    ops.sqrts += 1;
    sqrtm(rhs, p).map(|y| canonical_y(y, p))
}

/// Simultaneous modular inversion (Montgomery's trick) over arbitrary odd `p > 2`
/// fitting in `u64`, using `Fp64` Montgomery arithmetic when `p < 2^62` and exact
/// `u128` reductions otherwise. Zero entries are left unchanged.
pub fn batch_inv_u64_counted(xs: &mut [u64], p: u64, scratch: &mut Vec<u64>, ops: &mut FpOpCounts) {
    let k = xs.len();
    if k == 0 {
        return;
    }
    if p > 3 && p < (1u64 << MAX_BITS) && xs.iter().all(|&v| v % p != 0) {
        let f = Fp64::new(p);
        for v in xs.iter_mut() {
            *v = f.to_mont(*v);
        }
        let muls_each = if k == 1 { 0 } else { 3 * (k as u64 - 1) };
        ops.muls += muls_each;
        ops.invs += 1;
        f.batch_inv(xs, scratch);
        for v in xs.iter_mut() {
            *v = f.from_mont(*v);
        }
        return;
    }
    scratch.clear();
    scratch.reserve(k);
    let mut acc = 1u64 % p;
    let mut nonzero = 0usize;
    for v in xs.iter_mut() {
        *v %= p;
        scratch.push(acc);
        if *v != 0 {
            if nonzero > 0 {
                ops.muls += 1;
            }
            acc = mulm(acc, *v, p);
            nonzero += 1;
        }
    }
    if nonzero == 0 {
        return;
    }
    ops.invs += 1;
    let mut inv_acc = invm(acc, p);
    let mut seen_nonzero = 0usize;
    for i in (0..k).rev() {
        let xi = xs[i];
        if xi == 0 {
            continue;
        }
        seen_nonzero += 1;
        if nonzero == 1 {
            xs[i] = inv_acc;
        } else if seen_nonzero == nonzero {
            xs[i] = inv_acc;
        } else {
            ops.muls += 1;
            xs[i] = mulm(inv_acc, scratch[i], p);
            ops.muls += 1;
            inv_acc = mulm(inv_acc, xi, p);
        }
    }
}

/// Rational-discriminant batched `S_3(x1, x2, X)` root classifier over `(x1, y1_opt, x2, y2_opt)`
/// tuples.
///
/// - When both `y1_opt` and `y2_opt` are `Some` (curve-rational points), uses
///   `\Delta = (4*y1*y2)^2` and a single shared Montgomery batch inversion over
///   `2*(x1 - x2)^2` (plus `c1` on non-2-torsion diagonals `x1 == x2`), paying
///   **zero** Tonelli–Shanks square roots.
/// - When one point is curve-rational (`Some(y)`) and the other is off-curve (`None`),
///   uses multiplicativity of the Legendre symbol (`\Delta = (4*y)^2 * f(x_other)` is
///   `0` iff `y == 0` and a non-residue otherwise) to emit the exact `Base` or
///   `Irreducible` key from the same shared batch inversion, also with **zero** `sqrtm` calls.
/// - When both points are off-curve (`None, None`), safely falls back to
///   [`quadratic_root_keys_counted`].
pub fn s3_pair_roots_rational_batched(
    curve: &Curve,
    pairs: &[(u64, Option<u64>, u64, Option<u64>)],
    ops: &mut FpOpCounts,
) -> Vec<QuadraticRoots> {
    let p = curve.p;
    let a = curve.a % p;
    let b = curve.b % p;
    let n = pairs.len();
    let mut out = vec![QuadraticRoots::None; n];
    if n == 0 {
        return out;
    }

    #[derive(Clone, Copy)]
    enum Slot {
        ConstAll,
        ConstNone,
        Fallback([u64; 3]),
        DiagLinear { c0: u64, denom_idx: usize },
        RationalOffDiag {
            minus_c1: u64,
            four_y1y2: u64,
            denom_idx: usize,
        },
        HalfOffCurve {
            c0: u64,
            c1: u64,
            y_zero: bool,
            denom_idx: usize,
        },
    }

    let mut slots = Vec::with_capacity(n);
    let mut denoms = Vec::with_capacity(n);

    for &(x1_raw, y1_opt, x2_raw, y2_opt) in pairs {
        let x1 = x1_raw % p;
        let x2 = x2_raw % p;
        match (y1_opt.map(|y| y % p), y2_opt.map(|y| y % p)) {
            (Some(y1), Some(y2)) => {
                if x1 == x2 {
                    // Diagonal: c2 = 0, c1 = -4*f(x1) = -4*y1^2, c0 = (x1^2 - a)^2 - 8*b*x1.
                    ops.sqrs += 2;
                    ops.subs += 3;
                    ops.muls += 4;
                    let y1_sq = mulm(y1, y1, p);
                    let c1 = subm(0, mulm(4 % p, y1_sq, p), p);
                    let x1_sq = mulm(x1, x1, p);
                    let shifted = subm(x1_sq, a, p);
                    let c0 = subm(
                        mulm(shifted, shifted, p),
                        mulm(8 % p, mulm(b, x1, p), p),
                        p,
                    );
                    if c1 == 0 {
                        slots.push(if c0 == 0 {
                            Slot::ConstAll
                        } else {
                            Slot::ConstNone
                        });
                    } else {
                        let denom_idx = denoms.len();
                        denoms.push(c1);
                        slots.push(Slot::DiagLinear { c0, denom_idx });
                    }
                } else {
                    // Off-diagonal rational: denom = 2*(x1 - x2)^2, -c1 = 2*((x1+x2)*(x1*x2+a) + 2*b).
                    ops.subs += 1;
                    ops.adds += 4;
                    ops.sqrs += 1;
                    ops.muls += 4;
                    let d = subm(x1, x2, p);
                    let c2 = mulm(d, d, p);
                    let two_c2 = addm(c2, c2, p);
                    let s = addm(x1, x2, p);
                    let prod = mulm(x1, x2, p);
                    let two_b = addm(b, b, p);
                    let inner = addm(mulm(s, addm(prod, a, p), p), two_b, p);
                    let minus_c1 = addm(inner, inner, p);
                    let y1y2 = mulm(y1, y2, p);
                    let two_y1y2 = addm(y1y2, y1y2, p);
                    let four_y1y2 = addm(two_y1y2, two_y1y2, p);
                    let denom_idx = denoms.len();
                    denoms.push(two_c2);
                    slots.push(Slot::RationalOffDiag {
                        minus_c1,
                        four_y1y2,
                        denom_idx,
                    });
                }
            }
            (Some(y_on), None) | (None, Some(y_on)) => {
                let q = s3_quadratic_counted(curve, x1, x2, ops);
                let [c0, c1, c2] = q;
                if c2 == 0 {
                    slots.push(Slot::Fallback(q));
                } else {
                    ops.adds += 1;
                    let two_c2 = addm(c2, c2, p);
                    let denom_idx = denoms.len();
                    denoms.push(two_c2);
                    slots.push(Slot::HalfOffCurve {
                        c0,
                        c1,
                        y_zero: y_on == 0,
                        denom_idx,
                    });
                }
            }
            (None, None) => {
                let q = s3_quadratic_counted(curve, x1, x2, ops);
                slots.push(Slot::Fallback(q));
            }
        }
    }

    let mut scratch = Vec::with_capacity(denoms.len());
    batch_inv_u64_counted(&mut denoms, p, &mut scratch, ops);

    for (i, slot) in slots.into_iter().enumerate() {
        out[i] = match slot {
            Slot::ConstAll => QuadraticRoots::All,
            Slot::ConstNone => QuadraticRoots::None,
            Slot::Fallback(q) => quadratic_root_keys_counted(q, p, ops),
            Slot::DiagLinear { c0, denom_idx } => {
                ops.subs += 1;
                ops.muls += 1;
                let root = mulm(subm(0, c0, p), denoms[denom_idx], p);
                QuadraticRoots::Finite(vec![QuadraticRootKey::Base(root)])
            }
            Slot::RationalOffDiag {
                minus_c1,
                four_y1y2,
                denom_idx,
            } => {
                let inv_2c2 = denoms[denom_idx];
                if four_y1y2 == 0 {
                    ops.muls += 1;
                    let r0 = mulm(minus_c1, inv_2c2, p);
                    QuadraticRoots::Finite(vec![QuadraticRootKey::Base(r0)])
                } else {
                    ops.adds += 1;
                    ops.subs += 1;
                    ops.muls += 2;
                    let r0 = mulm(addm(minus_c1, four_y1y2, p), inv_2c2, p);
                    let r1 = mulm(subm(minus_c1, four_y1y2, p), inv_2c2, p);
                    let mut roots = vec![QuadraticRootKey::Base(r0)];
                    if r1 != r0 {
                        roots.push(QuadraticRootKey::Base(r1));
                    }
                    QuadraticRoots::Finite(roots)
                }
            }
            Slot::HalfOffCurve {
                c0,
                c1,
                y_zero,
                denom_idx,
            } => {
                let inv_2c2 = denoms[denom_idx];
                if y_zero {
                    ops.subs += 1;
                    ops.muls += 1;
                    let r0 = mulm(subm(0, c1, p), inv_2c2, p);
                    QuadraticRoots::Finite(vec![QuadraticRootKey::Base(r0)])
                } else {
                    ops.adds += 1;
                    ops.muls += 2;
                    let inv_c2 = addm(inv_2c2, inv_2c2, p);
                    let linear = mulm(c1, inv_c2, p);
                    let constant = mulm(c0, inv_c2, p);
                    QuadraticRoots::Finite(vec![QuadraticRootKey::Irreducible { linear, constant }])
                }
            }
        };
    }

    out
}

/// Multi-limb (`BigUint` / `FieldElement`) reference `S_3(x1, x2, X)` root classifier
/// using per-pair modular inversions and Tonelli–Shanks `sqrt_mod_p`.
pub fn s3_pair_roots_biguint_reference(
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
    x1: &BigUint,
    x2: &BigUint,
) -> BigQuadraticRoots {
    let fe = |v: BigUint| FieldElement::new(v, p.clone());
    let x1 = fe(x1 % p);
    let x2 = fe(x2 % p);
    let a = fe(a % p);
    let b = fe(b % p);
    let two = fe(BigUint::from(2u32));
    let four = fe(BigUint::from(4u32));
    let d = x1.sub(&x2);
    let s = x1.add(&x2);
    let prod = x1.mul(&x2);
    let c2 = d.mul(&d);
    let c1 = two.mul(&s.mul(&prod.add(&a)).add(&two.mul(&b))).neg();
    let shifted = prod.sub(&a);
    let c0 = shifted.mul(&shifted).sub(&four.mul(&b.mul(&s)));

    if c2.is_zero() {
        if c1.is_zero() {
            return if c0.is_zero() {
                BigQuadraticRoots::All
            } else {
                BigQuadraticRoots::None
            };
        }
        let inv_c1 = c1.inv().expect("c1 != 0");
        return BigQuadraticRoots::Finite(vec![BigQuadraticRootKey::Base(
            c0.neg().mul(&inv_c1).value,
        )]);
    }

    let inv_c2 = c2.inv().expect("c2 != 0");
    let linear = c1.mul(&inv_c2);
    let constant = c0.mul(&inv_c2);
    let disc = linear.mul(&linear).sub(&four.mul(&constant));
    if let Some(root_disc_val) = sqrt_mod_p(&disc.value, p) {
        let root_disc = fe(root_disc_val);
        let inv_two = two.inv().expect("p odd");
        let minus_lin = linear.neg();
        let r0 = minus_lin.add(&root_disc).mul(&inv_two).value;
        let r1 = minus_lin.sub(&root_disc).mul(&inv_two).value;
        let mut roots = vec![BigQuadraticRootKey::Base(r0.clone())];
        if r1 != r0 {
            roots.push(BigQuadraticRootKey::Base(r1));
        }
        BigQuadraticRoots::Finite(roots)
    } else {
        BigQuadraticRoots::Finite(vec![BigQuadraticRootKey::Irreducible {
            linear: linear.value,
            constant: constant.value,
        }])
    }
}

/// Multi-limb (`BigUint` / `FieldElement`) rational-discriminant batched `S_3` root
/// classifier over arbitrary-size prime fields (`p > 2^64` included), sharing a single
/// Montgomery simultaneous inversion over `2*(x1 - x2)^2` across the batch.
pub fn s3_pair_roots_biguint_rational_batched(
    p: &BigUint,
    a: &BigUint,
    b: &BigUint,
    pairs: &[(BigUint, Option<BigUint>, BigUint, Option<BigUint>)],
) -> Vec<BigQuadraticRoots> {
    let fe = |v: BigUint| FieldElement::new(v, p.clone());
    let a_fe = fe(a % p);
    let b_fe = fe(b % p);
    let two_b = b_fe.add(&b_fe);
    let four = fe(BigUint::from(4u32));
    let eight = fe(BigUint::from(8u32));

    enum BigSlot {
        ConstAll,
        ConstNone,
        Fallback(BigUint, BigUint),
        DiagLinear { c0: FieldElement, idx: usize },
        Rational {
            minus_c1: FieldElement,
            four_y1y2: FieldElement,
            idx: usize,
        },
        HalfOffCurve {
            c0: FieldElement,
            c1: FieldElement,
            y_zero: bool,
            idx: usize,
        },
    }

    let mut slots = Vec::with_capacity(pairs.len());
    let mut denoms: Vec<FieldElement> = Vec::with_capacity(pairs.len());

    for (x1_raw, y1_opt, x2_raw, y2_opt) in pairs {
        let x1 = fe(x1_raw % p);
        let x2 = fe(x2_raw % p);
        match (y1_opt, y2_opt) {
            (Some(y1_raw), Some(y2_raw)) => {
                let y1 = fe(y1_raw % p);
                let y2 = fe(y2_raw % p);
                if x1 == x2 {
                    let c1 = four.mul(&y1.mul(&y1)).neg();
                    let shifted = x1.mul(&x1).sub(&a_fe);
                    let c0 = shifted.mul(&shifted).sub(&eight.mul(&b_fe.mul(&x1)));
                    if c1.is_zero() {
                        slots.push(if c0.is_zero() {
                            BigSlot::ConstAll
                        } else {
                            BigSlot::ConstNone
                        });
                    } else {
                        let idx = denoms.len();
                        denoms.push(c1);
                        slots.push(BigSlot::DiagLinear { c0, idx });
                    }
                } else {
                    let d = x1.sub(&x2);
                    let c2 = d.mul(&d);
                    let two_c2 = c2.add(&c2);
                    let s = x1.add(&x2);
                    let prod = x1.mul(&x2);
                    let inner = s.mul(&prod.add(&a_fe)).add(&two_b);
                    let minus_c1 = inner.add(&inner);
                    let y1y2 = y1.mul(&y2);
                    let two_y1y2 = y1y2.add(&y1y2);
                    let four_y1y2 = two_y1y2.add(&two_y1y2);
                    let idx = denoms.len();
                    denoms.push(two_c2);
                    slots.push(BigSlot::Rational {
                        minus_c1,
                        four_y1y2,
                        idx,
                    });
                }
            }
            (Some(y_on), None) | (None, Some(y_on)) => {
                let d = x1.sub(&x2);
                let c2 = d.mul(&d);
                if c2.is_zero() {
                    slots.push(BigSlot::Fallback(x1.value, x2.value));
                } else {
                    let s = x1.add(&x2);
                    let prod = x1.mul(&x2);
                    let inner = s.mul(&prod.add(&a_fe)).add(&two_b);
                    let c1 = inner.add(&inner).neg();
                    let shifted = prod.sub(&a_fe);
                    let c0 = shifted.mul(&shifted).sub(&four.mul(&b_fe.mul(&s)));
                    let idx = denoms.len();
                    denoms.push(c2.add(&c2));
                    slots.push(BigSlot::HalfOffCurve {
                        c0,
                        c1,
                        y_zero: (y_on % p).is_zero(),
                        idx,
                    });
                }
            }
            (None, None) => {
                slots.push(BigSlot::Fallback(x1.value, x2.value));
            }
        }
    }

    // Montgomery simultaneous inversion over `FieldElement`s.
    if !denoms.is_empty() {
        let mut prefix = Vec::with_capacity(denoms.len());
        let mut acc = FieldElement::new(BigUint::one(), p.clone());
        for d in &denoms {
            prefix.push(acc.clone());
            acc = acc.mul(d);
        }
        let mut inv_acc = acc.inv().expect("nonzero denominators");
        for i in (0..denoms.len()).rev() {
            let di = denoms[i].clone();
            denoms[i] = inv_acc.mul(&prefix[i]);
            inv_acc = inv_acc.mul(&di);
        }
    }

    slots
        .into_iter()
        .map(|slot| match slot {
            BigSlot::ConstAll => BigQuadraticRoots::All,
            BigSlot::ConstNone => BigQuadraticRoots::None,
            BigSlot::Fallback(x1, x2) => s3_pair_roots_biguint_reference(p, a, b, &x1, &x2),
            BigSlot::DiagLinear { c0, idx } => BigQuadraticRoots::Finite(vec![
                BigQuadraticRootKey::Base(c0.neg().mul(&denoms[idx]).value),
            ]),
            BigSlot::Rational {
                minus_c1,
                four_y1y2,
                idx,
            } => {
                let inv_2c2 = &denoms[idx];
                if four_y1y2.is_zero() {
                    BigQuadraticRoots::Finite(vec![BigQuadraticRootKey::Base(
                        minus_c1.mul(inv_2c2).value,
                    )])
                } else {
                    let r0 = minus_c1.add(&four_y1y2).mul(inv_2c2).value;
                    let r1 = minus_c1.sub(&four_y1y2).mul(inv_2c2).value;
                    let mut roots = vec![BigQuadraticRootKey::Base(r0.clone())];
                    if r1 != r0 {
                        roots.push(BigQuadraticRootKey::Base(r1));
                    }
                    BigQuadraticRoots::Finite(roots)
                }
            }
            BigSlot::HalfOffCurve {
                c0,
                c1,
                y_zero,
                idx,
            } => {
                let inv_2c2 = &denoms[idx];
                if y_zero {
                    BigQuadraticRoots::Finite(vec![BigQuadraticRootKey::Base(
                        c1.neg().mul(inv_2c2).value,
                    )])
                } else {
                    let inv_c2 = inv_2c2.add(inv_2c2);
                    BigQuadraticRoots::Finite(vec![BigQuadraticRootKey::Irreducible {
                        linear: c1.mul(&inv_c2).value,
                        constant: c0.mul(&inv_c2).value,
                    }])
                }
            }
        })
        .collect()
}

/// A target-independent projection of all unordered factor-base pairs.
#[derive(Clone, Debug)]
pub struct PairProjection {
    curve: Curve,
    singular: bool,
    base: Vec<u64>,
    /// Canonical `y_i = min(y, p - y)` for each `base[i]`.
    base_y: Vec<u64>,
    witnesses: HashMap<QuadraticRootKey, (u64, u64)>,
    finite_witnesses: HashMap<u64, (u64, u64)>,
    /// Deterministic contiguous entries `(u, y_u, (x1, x2))` for `finite_witnesses`,
    /// enabling cache-local block-batched inversions with early witness exit.
    finite_list: Vec<(u64, u64, (u64, u64))>,
    universal_pair: Option<(u64, u64)>,
    stats: BuildStats,
}

impl PairProjection {
    /// Build the exact pair projection using canonical `y_i` caching and
    /// Montgomery simultaneous batch inversion over `2*(x1 - x2)^2`.
    pub fn build(curve: Curve, base: &[u64]) -> Result<Self, String> {
        let mut dummy = FpOpCounts::default();
        Self::build_counted(curve, base, &mut dummy)
    }

    /// Same as [`PairProjection::build`], recording exact counted `F_p` operations.
    pub fn build_counted(
        curve: Curve,
        base: &[u64],
        ops: &mut FpOpCounts,
    ) -> Result<Self, String> {
        if curve.p <= 2 {
            return Err("projection join requires an odd prime field".into());
        }
        let p = curve.p;
        let a = curve.a % p;
        let b = curve.b % p;
        let norm_curve = Curve { p, a, b };
        let singular = norm_curve.is_singular();

        let mut clean: Vec<u64> = base.iter().map(|&x| x % p).collect();
        clean.sort_unstable();
        clean.dedup();

        let mut base_y = Vec::with_capacity(clean.len());
        for &x in &clean {
            match lift_canonical_counted(&norm_curve, x, ops) {
                Some(y) => base_y.push(y),
                None => return Err(format!("factor-base x={bad} is not an F_p point", bad = x)),
            }
        }

        let m = clean.len();
        let total_pairs = m * (m + 1) / 2;
        let mut witnesses = HashMap::with_capacity(total_pairs.min(p as usize));
        let mut finite_witnesses = HashMap::with_capacity(total_pairs.min(p as usize));
        let mut finite_list = Vec::with_capacity(total_pairs.min(p as usize));
        let mut universal_pair = None;
        let mut stats = BuildStats::default();

        // Fast native Montgomery path when p < 2^62 and p > 3.
        if p > 3 && p < (1u64 << MAX_BITS) {
            let f = Fp64::new(p);
            let am = f.to_mont(a);
            let three_m = f.to_mont(3 % p);
            let xm: Vec<u64> = clean.iter().map(|&x| f.to_mont(x)).collect();
            let ym: Vec<u64> = base_y.iter().map(|&y| f.to_mont(y)).collect();

            #[derive(Clone, Copy)]
            struct FastPair {
                x1: u64,
                x2: u64,
                i: usize,
                j: usize,
                denom_idx: usize,
            }

            let mut chunk_pairs: Vec<FastPair> = Vec::with_capacity(BUILD_BATCH);
            let mut denoms: Vec<u64> = Vec::with_capacity(BUILD_BATCH);
            let mut scratch: Vec<u64> = Vec::with_capacity(BUILD_BATCH);

            let mut flush = |chunk_pairs: &mut Vec<FastPair>,
                             denoms: &mut Vec<u64>,
                             stats: &mut BuildStats,
                             ops: &mut FpOpCounts| {
                if chunk_pairs.is_empty() {
                    return;
                }
                if !denoms.is_empty() {
                    let k = denoms.len();
                    ops.muls += if k == 1 { 0 } else { 3 * (k as u64 - 1) };
                    ops.invs += 1;
                    f.batch_inv(denoms, &mut scratch);
                }
                for fp in chunk_pairs.drain(..) {
                    if fp.i == fp.j {
                        // Diagonal pair (x1 == x1).
                        if fp.denom_idx == usize::MAX {
                            // y1 == 0: singular if 3*x1^2 + a == 0, else None.
                            ops.sqrs += 1;
                            ops.muls += 1;
                            ops.adds += 1;
                            let num = f.add(f.mul(three_m, f.sqr(xm[fp.i])), am);
                            if num == 0 {
                                stats.universal_pairs += 1;
                                universal_pair.get_or_insert((fp.x1, fp.x2));
                            }
                        } else {
                            // inv_2y = (2*y1)^(-1); lam = (3*x1^2 + a)*inv_2y; root = lam^2 - 2*x1.
                            let inv_2y = denoms[fp.denom_idx];
                            let x1m = xm[fp.i];
                            let y1m = ym[fp.i];
                            ops.sqrs += 2;
                            ops.muls += 2;
                            ops.adds += 2;
                            ops.subs += 1;
                            let num = f.add(f.mul(three_m, f.sqr(x1m)), am);
                            let lam = f.mul(num, inv_2y);
                            let rm = f.sub(f.sqr(lam), f.add(x1m, x1m));
                            let root = f.from_mont(rm);
                            let key = QuadraticRootKey::Base(root);
                            stats.root_keys += 1;
                            stats.base_field_root_keys += 1;
                            witnesses.entry(key).or_insert((fp.x1, fp.x2));
                            if let std::collections::hash_map::Entry::Vacant(e) =
                                finite_witnesses.entry(root)
                            {
                                e.insert((fp.x1, fp.x2));
                                ops.subs += 2;
                                ops.muls += 1;
                                let yum = f.sub(f.mul(lam, f.sub(x1m, rm)), y1m);
                                let yu = canonical_y(f.from_mont(yum), p);
                                finite_list.push((root, yu, (fp.x1, fp.x2)));
                            }
                        }
                    } else {
                        // Off-diagonal pair (x1 != x2): inv_2d2 = (2*(x1 - x2)^2)^(-1).
                        // Notice inv_dx = 2*(x1 - x2) * inv_2d2 = (x1 - x2)^(-1), giving both
                        // chord slopes lam_minus = (y1 + y2)*inv_dx and lam_plus = (y1 - y2)*inv_dx,
                        // whose roots r0, r1 match (-c1 +/- 4*y1*y2)/(2*(x1-x2)^2) identically and
                        // yield y(u) = lam*(x1 - r) - y1 without Tonelli-Shanks.
                        let inv_2d2 = denoms[fp.denom_idx];
                        let x1m = xm[fp.i];
                        let x2m = xm[fp.j];
                        let y1m = ym[fp.i];
                        let y2m = ym[fp.j];
                        ops.subs += 1;
                        ops.adds += 2;
                        ops.muls += 1;
                        let dx = f.sub(x1m, x2m);
                        let inv_dx = f.mul(f.add(dx, dx), inv_2d2);
                        let sum_x = f.add(x1m, x2m);

                        ops.adds += 1;
                        ops.muls += 1;
                        ops.sqrs += 1;
                        ops.subs += 1;
                        let lam0 = f.mul(f.add(y1m, y2m), inv_dx);
                        let r0m = f.sub(f.sqr(lam0), sum_x);
                        let r0 = f.from_mont(r0m);

                        let record_root =
                            |root: u64,
                             rm: u64,
                             lam: u64,
                             stats: &mut BuildStats,
                             witnesses: &mut HashMap<QuadraticRootKey, (u64, u64)>,
                             finite_witnesses: &mut HashMap<u64, (u64, u64)>,
                             finite_list: &mut Vec<(u64, u64, (u64, u64))>,
                             ops: &mut FpOpCounts| {
                                let key = QuadraticRootKey::Base(root);
                                stats.root_keys += 1;
                                stats.base_field_root_keys += 1;
                                witnesses.entry(key).or_insert((fp.x1, fp.x2));
                                if let std::collections::hash_map::Entry::Vacant(e) =
                                    finite_witnesses.entry(root)
                                {
                                    e.insert((fp.x1, fp.x2));
                                    ops.subs += 2;
                                    ops.muls += 1;
                                    let yum = f.sub(f.mul(lam, f.sub(x1m, rm)), y1m);
                                    let yu = canonical_y(f.from_mont(yum), p);
                                    finite_list.push((root, yu, (fp.x1, fp.x2)));
                                }
                            };

                        record_root(
                            r0,
                            r0m,
                            lam0,
                            stats,
                            &mut witnesses,
                            &mut finite_witnesses,
                            &mut finite_list,
                            ops,
                        );

                        if y1m != 0 && y2m != 0 {
                            ops.subs += 2;
                            ops.muls += 1;
                            ops.sqrs += 1;
                            let lam1 = f.mul(f.sub(y1m, y2m), inv_dx);
                            let r1m = f.sub(f.sqr(lam1), sum_x);
                            let r1 = f.from_mont(r1m);
                            if r1 != r0 {
                                record_root(
                                    r1,
                                    r1m,
                                    lam1,
                                    stats,
                                    &mut witnesses,
                                    &mut finite_witnesses,
                                    &mut finite_list,
                                    ops,
                                );
                            }
                        }
                    }
                }
                denoms.clear();
            };

            for i in 0..m {
                for j in i..m {
                    stats.unordered_pairs += 1;
                    let denom_idx = if i == j {
                        if ym[i] == 0 {
                            usize::MAX
                        } else {
                            ops.adds += 1;
                            let idx = denoms.len();
                            denoms.push(f.add(ym[i], ym[i]));
                            idx
                        }
                    } else {
                        ops.subs += 1;
                        ops.sqrs += 1;
                        ops.adds += 1;
                        let dx = f.sub(xm[i], xm[j]);
                        let d2 = f.sqr(dx);
                        let idx = denoms.len();
                        denoms.push(f.add(d2, d2));
                        idx
                    };
                    chunk_pairs.push(FastPair {
                        x1: clean[i],
                        x2: clean[j],
                        i,
                        j,
                        denom_idx,
                    });
                    if chunk_pairs.len() == BUILD_BATCH {
                        flush(&mut chunk_pairs, &mut denoms, &mut stats, ops);
                    }
                }
            }
            flush(&mut chunk_pairs, &mut denoms, &mut stats, ops);
        } else {
            // General odd p fallback (e.g. p == 3 or 2^62 <= p < 2^63) using
            // s3_pair_roots_rational_batched.
            let mut batch = Vec::with_capacity(BUILD_BATCH);
            let mut coords = Vec::with_capacity(BUILD_BATCH);
            let mut flush_gen =
                |batch: &mut Vec<(u64, Option<u64>, u64, Option<u64>)>,
                 coords: &mut Vec<(u64, u64)>,
                 stats: &mut BuildStats,
                 ops: &mut FpOpCounts| {
                    if batch.is_empty() {
                        return;
                    }
                    let roots_vec = s3_pair_roots_rational_batched(&norm_curve, batch, ops);
                    for ((x1, x2), roots) in coords.drain(..).zip(roots_vec) {
                        match roots {
                            QuadraticRoots::None => {}
                            QuadraticRoots::All => {
                                stats.universal_pairs += 1;
                                universal_pair.get_or_insert((x1, x2));
                            }
                            QuadraticRoots::Finite(keys) => {
                                for key in keys {
                                    stats.root_keys += 1;
                                    match key {
                                        QuadraticRootKey::Base(root) => {
                                            stats.base_field_root_keys += 1;
                                            if let std::collections::hash_map::Entry::Vacant(e) =
                                                finite_witnesses.entry(root)
                                            {
                                                e.insert((x1, x2));
                                                let yu = lift_canonical_counted(
                                                    &norm_curve, root, ops,
                                                )
                                                .unwrap_or(0);
                                                finite_list.push((root, yu, (x1, x2)));
                                            }
                                        }
                                        QuadraticRootKey::Irreducible { .. } => {
                                            stats.irreducible_root_keys += 1;
                                        }
                                    }
                                    witnesses.entry(key).or_insert((x1, x2));
                                }
                            }
                        }
                    }
                    batch.clear();
                };

            for i in 0..m {
                for j in i..m {
                    stats.unordered_pairs += 1;
                    batch.push((clean[i], Some(base_y[i]), clean[j], Some(base_y[j])));
                    coords.push((clean[i], clean[j]));
                    if batch.len() == BUILD_BATCH {
                        flush_gen(&mut batch, &mut coords, &mut stats, ops);
                    }
                }
            }
            flush_gen(&mut batch, &mut coords, &mut stats, ops);
        }

        stats.distinct_keys = witnesses.len();
        stats.distinct_base_roots = finite_witnesses.len();
        Ok(Self {
            curve: norm_curve,
            singular,
            base: clean,
            base_y,
            witnesses,
            finite_witnesses,
            finite_list,
            universal_pair,
            stats,
        })
    }

    /// Reference unbatched build (two `invm` + one Tonelli–Shanks `sqrtm` per pair)
    /// retained for exact equivalence verification and counted-op benchmarking.
    pub fn build_reference_counted(
        curve: Curve,
        base: &[u64],
        ops: &mut FpOpCounts,
    ) -> Result<Self, String> {
        if curve.p <= 2 {
            return Err("projection join requires an odd prime field".into());
        }
        let p = curve.p;
        let norm_curve = Curve {
            p,
            a: curve.a % p,
            b: curve.b % p,
        };
        let mut clean: Vec<u64> = base.iter().map(|&x| x % p).collect();
        clean.sort_unstable();
        clean.dedup();
        let mut base_y = Vec::with_capacity(clean.len());
        for &x in &clean {
            let Some(y) = lift_canonical_counted(&norm_curve, x, ops) else {
                return Err(format!("factor-base x={x} is not an F_p point"));
            };
            base_y.push(y);
        }

        let mut witnesses = HashMap::new();
        let mut finite_witnesses = HashMap::new();
        let mut finite_list = Vec::new();
        let mut universal_pair = None;
        let mut stats = BuildStats::default();
        for (i, &x1) in clean.iter().enumerate() {
            for &x2 in &clean[i..] {
                stats.unordered_pairs += 1;
                let q = s3_quadratic_counted(&norm_curve, x1, x2, ops);
                match quadratic_root_keys_counted(q, p, ops) {
                    QuadraticRoots::None => {}
                    QuadraticRoots::All => {
                        stats.universal_pairs += 1;
                        universal_pair.get_or_insert((x1, x2));
                    }
                    QuadraticRoots::Finite(keys) => {
                        for key in keys {
                            stats.root_keys += 1;
                            match key {
                                QuadraticRootKey::Base(root) => {
                                    stats.base_field_root_keys += 1;
                                    if let std::collections::hash_map::Entry::Vacant(e) =
                                        finite_witnesses.entry(root)
                                    {
                                        e.insert((x1, x2));
                                        let yu = sqrtm(norm_curve.rhs(root), p)
                                            .map(|y| canonical_y(y, p))
                                            .unwrap_or(0);
                                        finite_list.push((root, yu, (x1, x2)));
                                    }
                                }
                                QuadraticRootKey::Irreducible { .. } => {
                                    stats.irreducible_root_keys += 1;
                                }
                            }
                            witnesses.entry(key).or_insert((x1, x2));
                        }
                    }
                }
            }
        }
        stats.distinct_keys = witnesses.len();
        stats.distinct_base_roots = finite_witnesses.len();
        Ok(Self {
            curve: norm_curve,
            singular: norm_curve.is_singular(),
            base: clean,
            base_y,
            witnesses,
            finite_witnesses,
            finite_list,
            universal_pair,
            stats,
        })
    }

    pub fn curve(&self) -> Curve {
        self.curve
    }

    pub fn factor_base(&self) -> &[u64] {
        &self.base
    }

    pub fn cached_base_y(&self) -> &[u64] {
        &self.base_y
    }

    pub fn build_stats(&self) -> BuildStats {
        self.stats
    }

    pub fn finite_support(&self) -> usize {
        self.finite_witnesses.len()
    }

    /// Exact three-summand query, including the pair-cancellation component
    /// omitted by a finite affine auxiliary variable.
    pub fn decompose_three(&self, target: u64) -> (Option<ThreeSumWitness>, QueryStats) {
        let mut dummy = FpOpCounts::default();
        self.decompose_three_counted(target, &mut dummy)
    }

    /// Same as [`PairProjection::decompose_three`], recording exact counted `F_p` ops.
    pub fn decompose_three_counted(
        &self,
        target: u64,
        ops: &mut FpOpCounts,
    ) -> (Option<ThreeSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();

        stats.cancellation_checks += 1;
        if !self.base.is_empty() && self.base.binary_search(&target).is_ok() {
            let x = self.base[0];
            return (
                Some(ThreeSumWitness {
                    x: [x, x, target],
                    auxiliary: None,
                }),
                stats,
            );
        }

        if self.base.is_empty() {
            return (None, stats);
        }

        let y_target = lift_canonical_counted(&self.curve, target, ops);
        // Subgroup closure: if the curve is nonsingular and `target` is not an F_p point,
        // no three F_p points can sum to `target`.
        if !self.singular && self.universal_pair.is_none() && y_target.is_none() {
            return (None, stats);
        }

        let mut batch = Vec::with_capacity(QUERY_BATCH);
        for chunk_start in (0..self.base.len()).step_by(QUERY_BATCH) {
            let chunk_end = (chunk_start + QUERY_BATCH).min(self.base.len());
            batch.clear();
            for idx in chunk_start..chunk_end {
                batch.push((self.base[idx], Some(self.base_y[idx]), target, y_target));
            }
            let roots_block = s3_pair_roots_rational_batched(&self.curve, &batch, ops);
            for (offset, roots) in roots_block.into_iter().enumerate() {
                let x3 = self.base[chunk_start + offset];
                stats.quadratics += 1;
                match roots {
                    QuadraticRoots::None => {}
                    QuadraticRoots::All => {
                        if let Some((&key, &(x1, x2))) = self.witnesses.iter().next() {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                        if let Some((x1, x2)) = self.universal_pair {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: None,
                                }),
                                stats,
                            );
                        }
                    }
                    QuadraticRoots::Finite(keys) => {
                        stats.root_keys += keys.len() as u64;
                        for key in keys {
                            stats.lookups += 1;
                            if let Some(&(x1, x2)) = self.witnesses.get(&key) {
                                stats.compatible_keys += 1;
                                return (
                                    Some(ThreeSumWitness {
                                        x: [x1, x2, x3],
                                        auxiliary: Some(key),
                                    }),
                                    stats,
                                );
                            }
                            if let Some((x1, x2)) = self.universal_pair {
                                stats.compatible_keys += 1;
                                return (
                                    Some(ThreeSumWitness {
                                        x: [x1, x2, x3],
                                        auxiliary: Some(key),
                                    }),
                                    stats,
                                );
                            }
                        }
                    }
                }
            }
        }
        (None, stats)
    }

    /// Reference unbatched `decompose_three` for counted-op and witness parity tests.
    pub fn decompose_three_reference_counted(
        &self,
        target: u64,
        ops: &mut FpOpCounts,
    ) -> (Option<ThreeSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();

        stats.cancellation_checks += 1;
        if !self.base.is_empty() && self.base.binary_search(&target).is_ok() {
            let x = self.base[0];
            return (
                Some(ThreeSumWitness {
                    x: [x, x, target],
                    auxiliary: None,
                }),
                stats,
            );
        }

        for &x3 in &self.base {
            stats.quadratics += 1;
            let q = s3_quadratic_counted(&self.curve, x3, target, ops);
            match quadratic_root_keys_counted(q, p, ops) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some((&key, &(x1, x2))) = self.witnesses.iter().next() {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: Some(key),
                            }),
                            stats,
                        );
                    }
                    if let Some((x1, x2)) = self.universal_pair {
                        stats.compatible_keys += 1;
                        return (
                            Some(ThreeSumWitness {
                                x: [x1, x2, x3],
                                auxiliary: None,
                            }),
                            stats,
                        );
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        stats.lookups += 1;
                        if let Some(&(x1, x2)) = self.witnesses.get(&key) {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                        if let Some((x1, x2)) = self.universal_pair {
                            stats.compatible_keys += 1;
                            return (
                                Some(ThreeSumWitness {
                                    x: [x1, x2, x3],
                                    auxiliary: Some(key),
                                }),
                                stats,
                            );
                        }
                    }
                }
            }
        }
        (None, stats)
    }

    /// Exact balanced four-summand query with batched rational-discriminant
    /// query denominators and early witness exit.
    ///
    /// For exhaustive compatibility counting without early exit, see
    /// [`PairProjection::decompose_four_exhaustive`].
    pub fn decompose_four(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        let mut dummy = FpOpCounts::default();
        self.decompose_four_counted(target, &mut dummy)
    }

    /// Same as [`PairProjection::decompose_four`], recording exact counted `F_p` ops.
    pub fn decompose_four_counted(
        &self,
        target: u64,
        ops: &mut FpOpCounts,
    ) -> (Option<FourSumWitness>, QueryStats) {
        self.decompose_four_inner(target, true, ops)
    }

    /// Balanced four-summand query scanning all finite pair roots without early exit,
    /// so `stats.compatible_keys` records the full compatibility count.
    pub fn decompose_four_exhaustive(&self, target: u64) -> (Option<FourSumWitness>, QueryStats) {
        let mut dummy = FpOpCounts::default();
        self.decompose_four_inner(target, false, &mut dummy)
    }

    fn decompose_four_inner(
        &self,
        target: u64,
        early_exit: bool,
        ops: &mut FpOpCounts,
    ) -> (Option<FourSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();
        let mut witness = None;

        // If either pair cancels to infinity, the other pair must have target
        // as a finite root. Repetition of a factor-base x is allowed.
        stats.cancellation_checks += 1;
        if let (Some(&cancel), Some(&(x3, x4))) =
            (self.base.first(), self.finite_witnesses.get(&target))
        {
            let w = FourSumWitness {
                x: [cancel, cancel, x3, x4],
                auxiliaries: [None, Some(target)],
            };
            if early_exit {
                return (Some(w), stats);
            }
            witness = Some(w);
        }

        if self.finite_list.is_empty() {
            return (witness, stats);
        }

        let y_target = lift_canonical_counted(&self.curve, target, ops);
        if !self.singular && y_target.is_none() {
            if early_exit {
                return (None, stats);
            }
        }

        let mut batch = Vec::with_capacity(QUERY_BATCH);
        for chunk_start in (0..self.finite_list.len()).step_by(QUERY_BATCH) {
            let chunk_end = (chunk_start + QUERY_BATCH).min(self.finite_list.len());
            batch.clear();
            for &(u, yu, _) in &self.finite_list[chunk_start..chunk_end] {
                batch.push((u, Some(yu), target, y_target));
            }
            let roots_block = s3_pair_roots_rational_batched(&self.curve, &batch, ops);
            for (offset, roots) in roots_block.into_iter().enumerate() {
                let (u, _, (x1, x2)) = self.finite_list[chunk_start + offset];
                stats.quadratics += 1;
                match roots {
                    QuadraticRoots::None => {}
                    QuadraticRoots::All => {
                        if let Some(&(v, _, (x3, x4))) = self.finite_list.first() {
                            stats.compatible_keys += 1;
                            let w = FourSumWitness {
                                x: [x1, x2, x3, x4],
                                auxiliaries: [Some(u), Some(v)],
                            };
                            if early_exit {
                                return (Some(w), stats);
                            }
                            witness.get_or_insert(w);
                        }
                    }
                    QuadraticRoots::Finite(keys) => {
                        stats.root_keys += keys.len() as u64;
                        for key in keys {
                            let QuadraticRootKey::Base(v) = key else {
                                continue;
                            };
                            stats.lookups += 1;
                            if let Some(&(x3, x4)) = self.finite_witnesses.get(&v) {
                                stats.compatible_keys += 1;
                                let w = FourSumWitness {
                                    x: [x1, x2, x3, x4],
                                    auxiliaries: [Some(u), Some(v)],
                                };
                                if early_exit {
                                    return (Some(w), stats);
                                }
                                witness.get_or_insert(w);
                            }
                        }
                    }
                }
            }
        }
        (witness, stats)
    }

    /// Reference unbatched `decompose_four` (no early exit, per-pair `invm` + `sqrtm`)
    /// retained for exact parity and speedup benchmarking.
    pub fn decompose_four_reference_counted(
        &self,
        target: u64,
        ops: &mut FpOpCounts,
    ) -> (Option<FourSumWitness>, QueryStats) {
        let p = self.curve.p;
        let target = target % p;
        let mut stats = QueryStats::default();
        let mut witness = None;

        stats.cancellation_checks += 1;
        if let (Some(&cancel), Some(&(x3, x4))) =
            (self.base.first(), self.finite_witnesses.get(&target))
        {
            witness = Some(FourSumWitness {
                x: [cancel, cancel, x3, x4],
                auxiliaries: [None, Some(target)],
            });
        }

        for &(u, _, (x1, x2)) in &self.finite_list {
            stats.quadratics += 1;
            let q = s3_quadratic_counted(&self.curve, u, target, ops);
            match quadratic_root_keys_counted(q, p, ops) {
                QuadraticRoots::None => {}
                QuadraticRoots::All => {
                    if let Some(&(v, _, (x3, x4))) = self.finite_list.first() {
                        stats.compatible_keys += 1;
                        witness.get_or_insert(FourSumWitness {
                            x: [x1, x2, x3, x4],
                            auxiliaries: [Some(u), Some(v)],
                        });
                    }
                }
                QuadraticRoots::Finite(keys) => {
                    stats.root_keys += keys.len() as u64;
                    for key in keys {
                        let QuadraticRootKey::Base(v) = key else {
                            continue;
                        };
                        stats.lookups += 1;
                        if let Some(&(x3, x4)) = self.finite_witnesses.get(&v) {
                            stats.compatible_keys += 1;
                            witness.get_or_insert(FourSumWitness {
                                x: [x1, x2, x3, x4],
                                auxiliaries: [Some(u), Some(v)],
                            });
                        }
                    }
                }
            }
        }
        (witness, stats)
    }
}

/// Interval factor base, restricted to x-coordinates of `F_p` points.
pub fn interval_factor_base(curve: &Curve, bound: u64) -> Vec<u64> {
    (0..bound.min(curve.p))
        .filter(|&x| sqrtm(curve.rhs(x), curve.p).is_some())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pkm_smooth_tower::{s3, Pt};

    fn curve() -> Curve {
        Curve { p: 101, a: 2, b: 3 }
    }

    fn canonical_point(curve: &Curve, x: u64) -> Pt {
        Some((x, sqrtm(curve.rhs(x), curve.p).unwrap()))
    }

    fn signed_points(curve: &Curve, x: u64) -> Vec<Pt> {
        let point = canonical_point(curve, x);
        let neg = curve.neg(point);
        if neg == point {
            vec![point]
        } else {
            vec![point, neg]
        }
    }

    fn direct(curve: &Curve, base: &[u64], target: u64, arity: usize) -> bool {
        fn tuples(
            curve: &Curve,
            base: &[u64],
            target: u64,
            arity: usize,
            depth: usize,
            sum: Pt,
        ) -> bool {
            if depth == arity {
                return sum.is_some_and(|(x, _)| x == target);
            }
            base.iter().any(|&x| {
                signed_points(curve, x).into_iter().any(|point| {
                    tuples(curve, base, target, arity, depth + 1, curve.add(sum, point))
                })
            })
        }
        tuples(curve, base, target, arity, 0, None)
    }

    fn witness_reaches(curve: &Curve, xs: &[u64], target: u64) -> bool {
        fn visit(curve: &Curve, xs: &[u64], target: u64, depth: usize, sum: Pt) -> bool {
            if depth == xs.len() {
                return sum.is_some_and(|(x, _)| x == target);
            }
            signed_points(curve, xs[depth])
                .into_iter()
                .any(|point| visit(curve, xs, target, depth + 1, curve.add(sum, point)))
        }
        visit(curve, xs, target, 0, None)
    }

    fn canonicalize_roots(r: QuadraticRoots) -> QuadraticRoots {
        match r {
            QuadraticRoots::Finite(mut v) => {
                v.sort_unstable();
                QuadraticRoots::Finite(v)
            }
            other => other,
        }
    }

    fn canonicalize_big_roots(r: BigQuadraticRoots) -> BigQuadraticRoots {
        match r {
            BigQuadraticRoots::Finite(mut v) => {
                v.sort_unstable();
                BigQuadraticRoots::Finite(v)
            }
            other => other,
        }
    }

    #[test]
    fn quadratic_coefficients_match_s3() {
        let e = curve();
        for x1 in 0..e.p {
            for x2 in 0..e.p {
                let q = s3_quadratic(&e, x1, x2);
                for x3 in 0..e.p {
                    assert_eq!(eval_quadratic(q, x3, e.p), s3(&e, x1, x2, x3));
                }
            }
        }
    }

    #[test]
    fn root_keys_are_exact() {
        let p = 101;
        for c0 in 0..17 {
            for c1 in 0..17 {
                for c2 in 0..17 {
                    let q = [c0, c1, c2];
                    let actual: Vec<_> = (0..p).filter(|&x| eval_quadratic(q, x, p) == 0).collect();
                    match quadratic_root_keys(q, p) {
                        QuadraticRoots::None => assert!(actual.is_empty()),
                        QuadraticRoots::All => assert_eq!(actual.len(), p as usize),
                        QuadraticRoots::Finite(keys) => {
                            let mut got: Vec<_> = keys
                                .into_iter()
                                .filter_map(|k| match k {
                                    QuadraticRootKey::Base(x) => Some(x),
                                    QuadraticRootKey::Irreducible { .. } => None,
                                })
                                .collect();
                            got.sort_unstable();
                            assert_eq!(got, actual);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn projection_matches_signed_group_control() {
        let e = curve();
        let base = interval_factor_base(&e, 12);
        let projection = PairProjection::build(e, &base).unwrap();
        for target in 0..e.p {
            if sqrtm(e.rhs(target), e.p).is_none() {
                assert!(projection.decompose_three(target).0.is_none());
                assert!(projection.decompose_four(target).0.is_none());
                continue;
            }
            let three = projection.decompose_three(target).0;
            assert_eq!(
                three.is_some(),
                direct(&e, &base, target, 3),
                "three-sum target {target}"
            );
            if let Some(witness) = three {
                assert!(witness_reaches(&e, &witness.x, target));
            }
            let four = projection.decompose_four(target).0;
            assert_eq!(
                four.is_some(),
                direct(&e, &base, target, 4),
                "four-sum target {target}"
            );
            if let Some(witness) = four {
                assert!(witness_reaches(&e, &witness.x, target));
            }
        }
    }

    #[test]
    fn rejects_non_point_factor_base_entries() {
        let e = curve();
        let bad = (0..e.p).find(|&x| sqrtm(e.rhs(x), e.p).is_none()).unwrap();
        assert!(PairProjection::build(e, &[bad]).is_err());
    }

    #[test]
    fn rational_batched_zero_mismatches_across_primes_and_edge_cases() {
        // Test small primes (p = 3, 5, 101, 1009 — exhaustive over all (x1, x2) in F_p^2,
        // including 2-torsion y_i = 0, diagonals x1 = x2, off-curve twist points, and singular curves),
        // Prime31 (p = 2^31 - 1), and 61-bit / 62-bit primes.
        let exhaustive_curves = [
            Curve { p: 3, a: 1, b: 0 },      // p = 3, 2-torsion at x = 0
            Curve { p: 5, a: 1, b: 0 },      // 2-torsion at x = 0, 2, 3
            Curve { p: 101, a: 2, b: 3 },    // standard nonsingular
            Curve { p: 101, a: 100, b: 0 },  // y^2 = x^3 - x, three 2-torsion points (0, 1, 100)
            Curve { p: 101, a: 0, b: 0 },    // singular cusp (universal_pair at x = 0)
            Curve { p: 257, a: 3, b: 0 },    // 2-torsion + Fermat prime
        ];
        for c in exhaustive_curves {
            let ys: Vec<Option<u64>> = (0..c.p)
                .map(|x| sqrtm(c.rhs(x), c.p).map(|y| canonical_y(y, c.p)))
                .collect();
            let mut pairs = Vec::with_capacity((c.p * c.p) as usize);
            for x1 in 0..c.p {
                for x2 in 0..c.p {
                    pairs.push((x1, ys[x1 as usize], x2, ys[x2 as usize]));
                }
            }
            let mut fast_ops = FpOpCounts::default();
            let got = s3_pair_roots_rational_batched(&c, &pairs, &mut fast_ops);
            for (idx, &(x1, _, x2, _)) in pairs.iter().enumerate() {
                let want = quadratic_root_keys(s3_quadratic(&c, x1, x2), c.p);
                assert_eq!(
                    canonicalize_roots(got[idx].clone()),
                    canonicalize_roots(want),
                    "mismatch at p={} a={} b={} x1={x1} x2={x2}",
                    c.p,
                    c.a,
                    c.b
                );
            }
        }

        // Prime31 (2^31 - 1), 44-bit, 61-bit (2^61 - 1), and >2^62 63-bit prime (exercises general batch_inv fallback).
        let large_curves = [
            Curve {
                p: 2_147_483_647, // Prime31 = 2^31 - 1
                a: 2_147_483_644, // a = -3
                b: 0,             // includes 2-torsion at x = 0
            },
            Curve {
                p: 2_147_483_647,
                a: 2,
                b: 3,
            },
            Curve {
                p: (1u64 << 61) - 1,
                a: (1u64 << 61) - 4,
                b: 5,
            },
            Curve {
                p: (1u64 << 62) + 135, // 63-bit prime > 2^62 (general u128 batch_inv path)
                a: 3,
                b: 0, // 2-torsion at x = 0
            },
        ];
        for c in large_curves {
            assert!(crate::cryptanalysis::pkm_smooth_tower::is_prime(c.p));
            let mut xs = vec![0u64, 1, 2, 3, c.p - 1, c.p - 2];
            for k in 0..80u64 {
                xs.push((k.wrapping_mul(0x9E37_79B9_7F4A_7C15)) % c.p);
            }
            let ys: Vec<Option<u64>> = xs
                .iter()
                .map(|&x| sqrtm(c.rhs(x), c.p).map(|y| canonical_y(y, c.p)))
                .collect();
            let mut pairs = Vec::new();
            for i in 0..xs.len() {
                for j in 0..xs.len() {
                    pairs.push((xs[i], ys[i], xs[j], ys[j]));
                }
            }
            let mut fast_ops = FpOpCounts::default();
            let got = s3_pair_roots_rational_batched(&c, &pairs, &mut fast_ops);
            for (idx, &(x1, _, x2, _)) in pairs.iter().enumerate() {
                let want = quadratic_root_keys(s3_quadratic(&c, x1, x2), c.p);
                assert_eq!(
                    canonicalize_roots(got[idx].clone()),
                    canonicalize_roots(want),
                    "large-prime mismatch at p={} x1={x1} x2={x2}",
                    c.p
                );
            }
        }
    }

    #[test]
    fn pair_projection_build_and_queries_match_reference_and_cut_fp_ops() {
        let curves = [
            (Curve { p: 1009, a: 1006, b: 0 }, 48u64), // includes 2-torsion
            (Curve { p: 2_147_483_647, a: 2_147_483_644, b: 7 }, 160u64), // Prime31
            (Curve { p: (1u64 << 61) - 1, a: 3, b: 5 }, 120u64), // 61-bit Mersenne
        ];
        for (c, bound) in curves {
            let base = interval_factor_base(&c, bound);
            let mut fast_build_ops = FpOpCounts::default();
            let mut ref_build_ops = FpOpCounts::default();
            let fast = PairProjection::build_counted(c, &base, &mut fast_build_ops).unwrap();
            let reference =
                PairProjection::build_reference_counted(c, &base, &mut ref_build_ops).unwrap();

            assert_eq!(fast.build_stats(), reference.build_stats());
            assert_eq!(fast.witnesses, reference.witnesses);
            assert_eq!(fast.finite_witnesses, reference.finite_witnesses);
            assert_eq!(fast.universal_pair, reference.universal_pair);
            // Fast build performs only |base| sqrts (for base lifting), never O(|base|^2) sqrts!
            assert_eq!(fast_build_ops.sqrts, base.len() as u64);
            assert!(fast_build_ops.invs * 20 < ref_build_ops.invs);

            // Check online queries on both on-curve, off-curve, and factor-base targets.
            let mut targets: Vec<u64> = (0..60u64).map(|t| t % c.p).collect();
            if let Some(&(u, _, _)) = fast.finite_list.last() {
                targets.push(u);
            }
            let mut fast_q_ops = FpOpCounts::default();
            let mut ref_q_ops = FpOpCounts::default();
            for t in targets {
                let (w3_fast, _) = fast.decompose_three_counted(t, &mut fast_q_ops);
                let (w3_ref, _) = reference.decompose_three_reference_counted(t, &mut ref_q_ops);
                assert_eq!(w3_fast.is_some(), w3_ref.is_some(), "3-sum p={} t={t}", c.p);
                if let Some(w) = w3_fast {
                    assert!(witness_reaches(&c, &w.x, t));
                }

                let (w4_fast, _) = fast.decompose_four_counted(t, &mut fast_q_ops);
                let (w4_exh, st_exh) = fast.decompose_four_exhaustive(t);
                let (w4_ref, st_ref) =
                    reference.decompose_four_reference_counted(t, &mut ref_q_ops);
                assert_eq!(w4_fast.is_some(), w4_ref.is_some(), "4-sum p={} t={t}", c.p);
                assert_eq!(w4_exh.is_some(), w4_ref.is_some());
                assert_eq!(st_exh.compatible_keys, st_ref.compatible_keys);
                if let Some(w) = w4_fast {
                    assert!(witness_reaches(&c, &w.x, t));
                }
            }
            assert!(fast_q_ops.invs < ref_q_ops.invs);
            assert!(fast_q_ops.sqrts <= ref_q_ops.sqrts);
        }
    }

    #[test]
    fn multi_limb_biguint_rational_batched_zero_mismatches() {
        // Multi-limb primes: 128-bit prime (2^127 - 1) and 256-bit secp256k1 prime.
        let p127 = (BigUint::one() << 127) - BigUint::one();
        let p256 = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap();

        for (p, a, b) in [
            (p127, BigUint::from(3u32), BigUint::zero()), // includes 2-torsion at x = 0
            (p256, BigUint::zero(), BigUint::from(7u32)), // secp256k1
        ] {
            let mut pts: Vec<(BigUint, Option<BigUint>)> = Vec::new();
            for x_u32 in 0..24u32 {
                let x = BigUint::from(x_u32);
                let rhs = (&x * &x * &x + &a * &x + &b) % &p;
                let y = sqrt_mod_p(&rhs, &p);
                pts.push((x, y));
            }
            let mut pairs = Vec::new();
            for (x1, y1) in &pts {
                for (x2, y2) in &pts {
                    pairs.push((x1.clone(), y1.clone(), x2.clone(), y2.clone()));
                }
            }
            let got = s3_pair_roots_biguint_rational_batched(&p, &a, &b, &pairs);
            for (idx, (x1, _, x2, _)) in pairs.iter().enumerate() {
                let want = s3_pair_roots_biguint_reference(&p, &a, &b, x1, x2);
                assert_eq!(
                    canonicalize_big_roots(got[idx].clone()),
                    canonicalize_big_roots(want),
                    "multi-limb mismatch idx={idx} x1={x1} x2={x2}"
                );
            }
        }
    }
}
