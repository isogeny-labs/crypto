//! **E34 of `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (§9.38): a cheaper
//! class representative for the order-24 walk on E18's `j = 0` curves, and
//! every per-step operation of the walk counted and emitted.**
//!
//! The walk loop here is a copy of E29's ([`super::quartic_folds::rho_twist24_with`],
//! itself E18's loop [`super::quartic_folds::rho4_folded_with`]) with a
//! pluggable class representative ([`RepArm`]) and five native-operation
//! counters ([`NativeOps`]).  E29's functions are not touched; the arms
//! [`RepArm::Order6Built`] and [`RepArm::Order24Built`] reproduce them field
//! for field (unit tests below), and the cheap arms reproduce their walks.
//!
//! **The cheap representative.**  On the canonical model, candidate
//! `(j, k)` of a class is `ι^k ∘ Φ^j (P)`; on abscissae both maps are
//! coordinate-wise, `x_i ↦ c_{ijk}·x_i` with `c_{ijk} = h^{2j} z^{ij} ω^k ∈ F_p`.
//! So the minimum abscissa (lexicographic in `E4`'s coordinates) is found on
//! coordinate `0` alone (the twelve `c_{0jk}` are distinct), the winner's
//! abscissa is then completed, only its ordinate is computed (`Φ^j` on `y` is
//! a monomial map), and the sign is fixed by a half-`p` test on the first
//! non-zero ordinate coordinate.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::gaudry_quartic::{Curve4, Fp4, Pt4, E4};
use super::quartic_folds::{RhoFold, RhoRun4F, Twist24Instance};
use super::residual_walk::{inv_mod, mix64, pow_mod};

fn mulm(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

/// The native operations a walk performs outside its counted `F_p`
/// multiplications (§9.38's counting rule).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct NativeOps {
    /// Comparisons of two `F_p` residues in the class representative (an
    /// `E4` comparison counts one per coordinate examined; a test against
    /// `0` or `(p − 1)/2` counts one).
    pub cmp: u64,
    /// `F_p` negations or additions in the class representative.
    pub neg: u64,
    /// Multiplications, additions, subtractions and negations mod `n` in the
    /// logarithm bookkeeping (an inversion mod `n` counts one).
    pub modn: u64,
    /// Partition-hash evaluations.
    pub hash: u64,
    /// Hash-table lookups and inserts.
    pub table: u64,
}

impl NativeOps {
    pub fn total(&self) -> u64 {
        self.cmp + self.neg + self.modn + self.hash + self.table
    }
}

/// The class representative a walk uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum RepArm {
    /// E18's / E29's order-6 `⟨±ι⟩` representative ([`RhoFold::Psi`]), counted.
    Order6Built,
    /// The order-6 class by coordinate-0 abscissae first.
    Order6Cheap,
    /// E29's order-24 representative (three full Frobenius images, twelve
    /// candidates), counted.
    Order24Built,
    /// The cheap order-24 representative (§9.38).
    Order24Cheap,
    /// The same, with coordinate 0 by sign classes (`5` products, `6`
    /// negations).
    Order24CheapMulLean,
    /// Negative control: omits candidate `(j, k) = (3, 2)`.
    Order24OmitCandidate,
    /// Negative control: the ordinate by `Φ^{j−1}` instead of `Φ^j`.
    Order24WrongOrdinate,
}

impl RepArm {
    pub fn is_order24(self) -> bool {
        !matches!(self, RepArm::Order6Built | RepArm::Order6Cheap)
    }
}

/// Per-walk constants of the cheap representatives.
#[derive(Clone, Debug)]
pub struct RepTables {
    p: u64,
    n: u64,
    half: u64,
    /// `c[i][j][k] = h^{2j} z^{ij} ω^k`.
    c: [[[u64; 3]; 4]; 4],
    /// `Φ^j` on ordinates: output coordinate `o` is `scalar · y[src]`.
    ymap: [[(usize, u64); 4]; 4],
    /// `λ_Φ^j λ_ω^k mod n`.
    mult: [[u64; 3]; 4],
    /// Whether the twelve `c[0][j][k]` are distinct (no tie on coordinate 0
    /// unless `x₀ = 0`).
    distinct0: bool,
    /// `c[0][j + 2][k] = −c[0][j][k]` (the mul-lean variant's sign classes).
    sign_classes0: bool,
    /// `F_p` multiplications and mod-`n` operations spent building them.
    pub setup_fp_muls: u64,
    pub setup_modn: u64,
}

impl RepTables {
    /// Built from the instance with every product counted.
    pub fn new(inst: &Twist24Instance, f: &Fp4) -> RepTables {
        let p = inst.core.p;
        let n = inst.core.n;
        let mut muls = 0u64;
        let mut modn = 0u64;
        let mut mp = |a: u64, b: u64| -> u64 {
            if a == 1 {
                return b;
            }
            if b == 1 {
                return a;
            }
            muls += 1;
            mulm(a, b, p)
        };
        let w = [1, inst.core.omega % p, 0];
        let w = [w[0], w[1], mp(w[1], w[1])];
        let zp = {
            let z2 = mp(inst.z, inst.z);
            [1, inst.z, z2, mp(z2, inst.z)]
        };
        let h2 = {
            let h4 = mp(inst.h2, inst.h2);
            [1, inst.h2, h4, mp(h4, inst.h2)]
        };
        let mut c = [[[0u64; 3]; 4]; 4];
        for (i, ci) in c.iter_mut().enumerate() {
            for (j, cij) in ci.iter_mut().enumerate() {
                let a = mp(h2[j], zp[(i * j) % 4]);
                for (k, v) in cij.iter_mut().enumerate() {
                    *v = mp(a, w[k]);
                }
            }
        }
        // Φ on ordinates: frob_p (coordinate i by zⁱ), then ·h3·t^deg, a
        // coordinate shift with the reduction t⁴ = c on wrap.
        let step: [(usize, u64); 4] = core::array::from_fn(|i| {
            let o = i + inst.h3_degree;
            let s = mp(zp[i], inst.h3_coeff);
            (o % 4, if o >= 4 { mp(s, f.c) } else { s })
        });
        // ymap[j][o] = (src, scalar): compose `j` steps.
        let mut ymap = [[(0usize, 1u64); 4]; 4];
        let mut cur: [(usize, u64); 4] = core::array::from_fn(|i| (i, 1u64)); // src i -> (dst, scalar)
        for (j, ym) in ymap.iter_mut().enumerate() {
            if j > 0 {
                cur = core::array::from_fn(|i| {
                    let (d, s) = cur[i];
                    let (d2, s2) = step[d];
                    (d2, mp(s, s2))
                });
            }
            for (i, &(d, s)) in cur.iter().enumerate() {
                ym[d] = (i, s);
            }
        }
        let mut mult = [[0u64; 3]; 4];
        let lw = [1, inst.core.lambda % n, {
            modn += 1;
            mulm(inst.core.lambda, inst.core.lambda, n)
        }];
        let mut lf = 1u64;
        for (j, mj) in mult.iter_mut().enumerate() {
            if j > 0 {
                modn += 1;
                lf = mulm(lf, inst.lambda_frob, n);
            }
            for (k, v) in mj.iter_mut().enumerate() {
                if k > 0 {
                    modn += 1;
                }
                *v = mulm(lf, lw[k], n);
            }
        }
        let mut seen: Vec<u64> = c[0].iter().flatten().copied().collect();
        seen.sort_unstable();
        seen.dedup();
        let distinct0 = seen.len() == 12;
        let sign_classes0 = (0..2).all(|j| (0..3).all(|k| c[0][j + 2][k] == (p - c[0][j][k]) % p));
        RepTables {
            p,
            n,
            half: (p - 1) / 2,
            c,
            ymap,
            mult,
            distinct0,
            sign_classes0,
            setup_fp_muls: muls,
            setup_modn: modn,
        }
    }

    /// `c · v` for a constant `c`: a copy (`c = 1`), a negation (`c = −1`) or
    /// one counted product.
    fn by_const(&self, f: &Fp4, ops: &mut NativeOps, c: u64, v: u64) -> u64 {
        if c == 1 {
            v
        } else if c == self.p - 1 {
            ops.neg += 1;
            (self.p - v) % self.p
        } else {
            f.charge(1);
            mulm(c, v, self.p)
        }
    }

    /// The sign: E29's `−y < y` (lexicographic), decided by a range test on
    /// the first non-zero coordinate; `y` negated only when needed.
    fn sign(&self, ops: &mut NativeOps, y: E4, m: u64) -> (E4, u64) {
        let p = self.p;
        for i in 0..4 {
            let v = y.0[i];
            ops.cmp += 1;
            if v.wrapping_sub(1) < self.half {
                return (y, m); // v ∈ [1, (p−1)/2]: y < −y.
            }
            ops.cmp += 1;
            if v > self.half {
                // −y < y: negate the non-zero coordinates.
                let mut out = y;
                for c in out.0.iter_mut() {
                    if *c != 0 {
                        ops.neg += 1;
                        *c = p - *c;
                    }
                }
                ops.modn += 1;
                return (out, (self.n - m) % self.n);
            }
        }
        (y, m) // y = 0: E29 keeps y.
    }

    /// The cheap representatives (order 6 when `order24 = false`).
    fn cheap(&self, f: &Fp4, ops: &mut NativeOps, pt: Pt4, arm: RepArm) -> (Pt4, u64) {
        if pt.inf {
            return (pt, 1);
        }
        let p = self.p;
        let js = if arm.is_order24() { 4 } else { 1 };
        let mut tied: Vec<usize> = (0..js * 3)
            .filter(|&idx| !(arm == RepArm::Order24OmitCandidate && idx == 11))
            .collect();
        // Known coordinates of each candidate's abscissa.
        let mut xs = [[None::<u64>; 4]; 12];
        for i in 0..4 {
            if tied.len() == 1 {
                break;
            }
            let xi = pt.x.0[i];
            ops.cmp += 1;
            if xi == 0 {
                for &idx in &tied {
                    xs[idx][i] = Some(0);
                }
                continue; // every candidate's coordinate i is 0: all stay tied.
            }
            let lean = i == 0 && arm == RepArm::Order24CheapMulLean && self.sign_classes0;
            for &idx in &tied {
                let (j, k) = (idx / 3, idx % 3);
                if lean && j >= 2 {
                    continue;
                }
                xs[idx][i] = Some(self.by_const(f, ops, self.c[i][j][k], xi));
            }
            if lean {
                for &idx in &tied {
                    let (j, k) = (idx / 3, idx % 3);
                    if j >= 2 {
                        let v = xs[(j - 2) * 3 + k][i].expect("sign-class partner");
                        ops.neg += 1;
                        xs[idx][i] = Some((p - v) % p);
                    }
                }
            }
            let mut best = tied[0];
            for &idx in &tied[1..] {
                ops.cmp += 1;
                if xs[idx][i] < xs[best][i] {
                    best = idx;
                }
            }
            if i == 0 && self.distinct0 && tied.len() == js * 3 {
                tied = vec![best];
            } else {
                let mv = xs[best][i];
                let mut keep = Vec::new();
                for &idx in &tied {
                    if idx == best {
                        keep.push(idx);
                        continue;
                    }
                    ops.cmp += 1;
                    if xs[idx][i] == mv {
                        keep.push(idx);
                    }
                }
                tied = keep;
            }
        }
        let idx = tied[0];
        let (j, k) = (idx / 3, idx % 3);
        let x = E4(core::array::from_fn(|i| match xs[idx][i] {
            Some(v) => v,
            None => self.by_const(f, ops, self.c[i][j][k], pt.x.0[i]),
        }));
        let jy = if arm == RepArm::Order24WrongOrdinate {
            (j + 3) % 4
        } else {
            j
        };
        let y = E4(core::array::from_fn(|o| {
            let (src, s) = self.ymap[jy][o];
            self.by_const(f, ops, s, pt.y.0[src])
        }));
        let (y, m) = self.sign(ops, y, self.mult[j][k]);
        (Pt4 { x, y, inf: false }, m)
    }
}

/// `a < b` in `E4`'s order, counting one comparison per coordinate examined.
fn lex_lt(ops: &mut NativeOps, a: &E4, b: &E4) -> bool {
    for i in 0..4 {
        ops.cmp += 1;
        if a.0[i] != b.0[i] {
            return a.0[i] < b.0[i];
        }
    }
    false
}

/// `pow_mod(base, k)` for `k < 3`, counting its products as performed.
fn pow_small(ops: &mut NativeOps, base: u64, k: u64, n: u64) -> u64 {
    ops.modn += [0, 2, 3][k as usize];
    pow_mod(base, k, n)
}

/// E18's / E29's order-6 representative ([`RhoFold::Psi`] in `rho_canon`), counted.
fn order6_built(
    inst: &Twist24Instance,
    curve: &Curve4,
    ops: &mut NativeOps,
    pt: Pt4,
) -> (Pt4, u64) {
    let f = &curve.f;
    let n = inst.core.n;
    ops.modn += 1;
    let lam = [
        1,
        inst.core.lambda % n,
        mulm(inst.core.lambda, inst.core.lambda, n),
    ];
    let mut best = (pt, 1u64);
    for (k, &l) in lam.iter().enumerate().skip(1) {
        let img = inst.core.psi(curve, &pt, k as u32);
        if lex_lt(ops, &img.x, &best.0.x) {
            best = (img, l);
        }
    }
    ops.neg += 4;
    let ny = f.neg(&best.0.y);
    if lex_lt(ops, &ny, &best.0.y) {
        ops.modn += 1;
        (Pt4 { y: ny, ..best.0 }, (n - best.1) % n)
    } else {
        best
    }
}

/// E29's order-24 representative (`rho_twist24_with`'s `canon`), counted.
fn order24_built(
    inst: &Twist24Instance,
    curve: &Curve4,
    ops: &mut NativeOps,
    pt: Pt4,
) -> (Pt4, u64) {
    let f = &curve.f;
    let n = inst.core.n;
    if pt.inf {
        return (pt, 1);
    }
    let mut best = (pt, 1u64);
    let mut image = pt;
    let mut multiplier = 1u64;
    for j in 0..4 {
        if j > 0 {
            image = inst.frob(curve, &image);
            ops.modn += 1;
            multiplier = mulm(multiplier, inst.lambda_frob, n);
        }
        for k in 0..3u32 {
            let candidate = inst.core.psi(curve, &image, k);
            let pk = pow_small(ops, inst.core.lambda, k as u64, n);
            ops.modn += 1;
            let m = mulm(multiplier, pk, n);
            if lex_lt(ops, &candidate.x, &best.0.x) {
                best = (candidate, m);
            }
        }
    }
    ops.neg += 4;
    let ny = f.neg(&best.0.y);
    if lex_lt(ops, &ny, &best.0.y) {
        ops.modn += 1;
        (Pt4 { y: ny, ..best.0 }, (n - best.1) % n)
    } else {
        best
    }
}

/// The representative of `pt` under `arm` and `m` with `rep = [m]·pt`.
pub fn rep(
    inst: &Twist24Instance,
    curve: &Curve4,
    tables: &RepTables,
    ops: &mut NativeOps,
    arm: RepArm,
    pt: Pt4,
) -> (Pt4, u64) {
    match arm {
        RepArm::Order6Built => order6_built(inst, curve, ops, pt),
        RepArm::Order24Built => order24_built(inst, curve, ops, pt),
        _ => tables.cheap(&curve.f, ops, pt, arm),
    }
}

/// E18's partition (`rho_part`, `r` classes).
fn part(pt: &Pt4, r: usize) -> usize {
    let h = pt.x.0[0] ^ pt.x.0[1].wrapping_mul(0x9E37_79B9) ^ pt.x.0[2].rotate_left(17);
    (mix64(h ^ pt.x.0[3].rotate_left(29)) % r as u64) as usize
}

/// E27's visit digest (an instrument; not counted).
fn visit_digest(acc: u64, pt: &Pt4) -> u64 {
    let mut h = acc;
    for v in pt.x.0.iter().chain(pt.y.0.iter()) {
        h = mix64(h ^ v.wrapping_mul(0x9E37_79B9_7F4A_7C15));
    }
    h
}

/// One counted walk (§9.38).
#[derive(Clone, Debug, Serialize)]
pub struct CountedWalk {
    pub arm: RepArm,
    pub karatsuba: bool,
    /// E29's record fields (`visit_digest` computed for every arm).
    pub run: RhoRun4F,
    pub ops: NativeOps,
    /// Representative calls, and the `F_p` multiplications and native
    /// operations they spent (all included in `run` and `ops`).
    pub rep_calls: u64,
    pub rep_fp_muls: u64,
    pub rep_ops: NativeOps,
    /// The cheap arms' per-walk constants (included in `run.fp_muls`).
    pub table_fp_muls: u64,
}

/// E29's walk ([`super::quartic_folds::rho_twist24_with`], and for the
/// order-6 arms [`super::quartic_folds::rho4_folded_with`] with
/// [`RhoFold::Psi`]) with the representative `arm` and every native
/// operation counted.
pub fn rho_j0_counted(
    inst: &Twist24Instance,
    arm: RepArm,
    seed: u64,
    karatsuba: bool,
) -> CountedWalk {
    let curve = Curve4::with_params_arith(inst.core.p, inst.core.a, inst.core.b, karatsuba);
    let f = &curve.f;
    let n = inst.core.n;
    let mut rng = StdRng::seed_from_u64(seed ^ 0x8D04_F01D);
    let jumps = 32usize;
    let mut ops = NativeOps::default();
    let mut rep_ops = NativeOps::default();
    let (mut rep_calls, mut rep_fp_muls) = (0u64, 0u64);
    let mults: Vec<(u64, u64, Pt4)> = (0..jumps)
        .map(|_| {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(1..n);
            (
                a,
                b,
                curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b)),
            )
        })
        .collect();
    let mut a = rng.gen_range(0..n);
    let mut b = rng.gen_range(1..n);
    let start = curve.add(&curve.mul(&inst.core.g, a), &curve.mul(&inst.core.q, b));
    // The cheap arms' constants, charged to the walk.
    let tables = RepTables::new(inst, f);
    let cheap = !matches!(arm, RepArm::Order6Built | RepArm::Order24Built);
    let table_fp_muls = if cheap { tables.setup_fp_muls } else { 0 };
    f.charge(table_fp_muls);
    ops.modn += if cheap { tables.setup_modn } else { 0 };
    let mut canon = |pt: Pt4, ops: &mut NativeOps| -> (Pt4, u64) {
        let (m0, c0, n0, d0) = (f.muls(), ops.cmp, ops.neg, ops.modn);
        let out = rep(inst, &curve, &tables, ops, arm, pt);
        rep_calls += 1;
        rep_fp_muls += f.muls() - m0;
        rep_ops.cmp += ops.cmp - c0;
        rep_ops.neg += ops.neg - n0;
        rep_ops.modn += ops.modn - d0;
        out
    };
    let mut table: HashMap<Pt4, (u64, u64)> = HashMap::new();
    let (mut steps, mut cycle_escapes) = (0u64, 0u64);
    let mut digest = 0u64;
    let mut found = None;
    let cap = 64 * (n as f64).sqrt() as u64 + 1_000_000;
    let (mut pt, mult) = canon(start, &mut ops);
    ops.modn += 2;
    a = mulm(a, mult, n);
    b = mulm(b, mult, n);
    while steps <= cap {
        steps += 1;
        digest = visit_digest(digest, &pt);
        ops.table += 1;
        if let Some(&(a2, b2)) = table.get(&pt) {
            ops.modn += 1;
            let db = (b + n - b2) % n;
            if db != 0 {
                ops.modn += 3;
                found = Some(mulm((a2 + n - a) % n, inv_mod(db, n), n));
                break;
            }
            cycle_escapes += 1;
            let (doubled, m) = canon(curve.add(&pt, &pt), &mut ops);
            pt = doubled;
            ops.modn += 4;
            a = mulm(mulm(2, a, n), m, n);
            b = mulm(mulm(2, b, n), m, n);
            continue;
        }
        ops.table += 1;
        table.insert(pt, (a, b));
        ops.hash += 1;
        let mut j = part(&pt, jumps);
        let (mut next, mut m) = canon(curve.add(&pt, &mults[j].2), &mut ops);
        ops.hash += 1;
        if part(&next, jumps) == j {
            j = (j + 1) % jumps;
            (next, m) = canon(curve.add(&pt, &mults[j].2), &mut ops);
        }
        ops.modn += 4;
        a = mulm((a + mults[j].0) % n, m, n);
        b = mulm((b + mults[j].1) % n, m, n);
        pt = next;
    }
    let (walk_muls, walk_ops) = (f.muls(), curve.ops());
    let correct = found == Some(inst.core.d)
        && found.is_some_and(|d| curve.mul(&inst.core.g, d) == inst.core.q);
    CountedWalk {
        arm,
        karatsuba,
        run: RhoRun4F {
            fold: if arm.is_order24() {
                RhoFold::Twist24
            } else {
                RhoFold::Psi
            },
            seed,
            steps,
            group_ops: curve.ops(),
            fp_muls: f.muls(),
            cycle_escapes,
            correct,
            recovered: found,
            visit_digest: digest,
            verify_fp_muls: f.muls() - walk_muls,
            verify_group_ops: curve.ops() - walk_ops,
        },
        ops,
        rep_calls,
        rep_fp_muls,
        rep_ops,
        table_fp_muls,
    }
}

/// One identity check (§9.38 (b)): for `points` random points (and forced
/// `x₀ = 0` and `x = 0` points), every class member's representative under
/// `arm` against E29's (`Order24Built` or `Order6Built`).
#[derive(Clone, Debug, Default, Serialize)]
pub struct IdentityCheck {
    pub arm: Option<RepArm>,
    pub points: u64,
    pub forced_x0_zero: u64,
    pub x_zero_point: bool,
    pub members: u64,
    pub mismatches: u64,
    /// `rep = [m]·member` checked by scalar multiplication.
    pub scalar_checks: u64,
    pub scalar_failures: u64,
}

pub fn identity_check(
    inst: &Twist24Instance,
    arm: RepArm,
    points: usize,
    seed: u64,
) -> IdentityCheck {
    let curve = inst.core.curve();
    let f = &curve.f;
    let n = inst.core.n;
    let tables = RepTables::new(inst, f);
    let reference = if arm.is_order24() {
        RepArm::Order24Built
    } else {
        RepArm::Order6Built
    };
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE34_1D);
    let mut pts: Vec<Pt4> = Vec::new();
    let g = inst.core.g;
    // Points of ⟨G⟩ (so `[m]` is defined): [r]G.
    while pts.len() < points {
        pts.push(curve.mul(&g, rng.gen_range(1..n)));
    }
    let mut forced = 0u64;
    // Forced x₀ = 0 (and x₀ = x₁ = 0): on a prime-order curve every affine
    // point is in ⟨G⟩.
    let mut tries = 0;
    while forced < 8 && tries < 100_000 {
        tries += 1;
        let x = E4([
            0,
            if forced % 2 == 0 {
                rng.gen_range(0..inst.core.p)
            } else {
                0
            },
            rng.gen_range(0..inst.core.p),
            rng.gen_range(0..inst.core.p),
        ]);
        if let Some(pt) = curve.lift_x(&x, &mut rng) {
            pts.push(pt);
            forced += 1;
        }
    }
    let x_zero = curve.lift_x(&E4::ZERO, &mut rng);
    if let Some(pt) = x_zero {
        pts.push(pt);
    }
    let mut out = IdentityCheck {
        arm: Some(arm),
        points: points as u64,
        forced_x0_zero: forced,
        x_zero_point: x_zero.is_some(),
        ..Default::default()
    };
    let mut ops = NativeOps::default();
    for (pi, &pt) in pts.iter().enumerate() {
        let mut members = Vec::new();
        let mut image = pt;
        for j in 0..(if arm.is_order24() { 4 } else { 1 }) {
            if j > 0 {
                image = inst.frob(&curve, &image);
            }
            for k in 0..3u32 {
                let c = inst.core.psi(&curve, &image, k);
                members.push(c);
                members.push(curve.neg(&c));
            }
        }
        for (mi, &mem) in members.iter().enumerate() {
            let want = rep(inst, &curve, &tables, &mut ops, reference, mem);
            let got = rep(inst, &curve, &tables, &mut ops, arm, mem);
            out.members += 1;
            if want != got {
                out.mismatches += 1;
            }
            if pi % 16 == 0 && mi % 5 == 0 {
                out.scalar_checks += 1;
                if curve.mul(&mem, got.1) != got.0 {
                    out.scalar_failures += 1;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::quartic_folds::{
        construct_twist24, rho4_folded_with, rho_twist24_with,
    };

    fn small_instance() -> Twist24Instance {
        construct_twist24(241, 3, 1).expect("E29's p = 241 model")
    }

    fn without_digest(mut r: RhoRun4F) -> String {
        r.visit_digest = 0;
        format!("{r:?}")
    }

    #[test]
    fn e34_built_arms_reproduce_e29_walks() {
        let inst = small_instance();
        for karatsuba in [false, true] {
            for seed in [1000u64, 1001] {
                let o6 = rho_j0_counted(&inst, RepArm::Order6Built, seed, karatsuba);
                let e6 = rho4_folded_with(&inst.core, RhoFold::Psi, seed, karatsuba);
                assert_eq!(format!("{:?}", o6.run), format!("{e6:?}"));
                let o24 = rho_j0_counted(&inst, RepArm::Order24Built, seed, karatsuba);
                let e24 = rho_twist24_with(&inst, seed, karatsuba);
                assert_eq!(without_digest(o24.run.clone()), without_digest(e24));
                assert!(o6.run.correct && o24.run.correct);
                // Positive control of the counters: 8 products per E29 order-6 representative.
                assert_eq!(o6.rep_fp_muls, 8 * o6.rep_calls);
            }
        }
    }

    #[test]
    fn e34_cheap_arms_give_e29s_walks() {
        let inst = small_instance();
        for karatsuba in [false, true] {
            let seed = 1002;
            let b24 = rho_j0_counted(&inst, RepArm::Order24Built, seed, karatsuba);
            for arm in [RepArm::Order24Cheap, RepArm::Order24CheapMulLean] {
                let c = rho_j0_counted(&inst, arm, seed, karatsuba);
                assert_eq!(c.run.visit_digest, b24.run.visit_digest);
                assert_eq!(
                    (
                        c.run.steps,
                        c.run.group_ops,
                        c.run.cycle_escapes,
                        c.run.recovered,
                        c.run.correct
                    ),
                    (
                        b24.run.steps,
                        b24.run.group_ops,
                        b24.run.cycle_escapes,
                        b24.run.recovered,
                        b24.run.correct
                    )
                );
                assert!(c.run.fp_muls < b24.run.fp_muls);
            }
            let b6 = rho_j0_counted(&inst, RepArm::Order6Built, seed, karatsuba);
            let c6 = rho_j0_counted(&inst, RepArm::Order6Cheap, seed, karatsuba);
            assert_eq!(c6.run.visit_digest, b6.run.visit_digest);
            assert_eq!(c6.run.steps, b6.run.steps);
            assert!(c6.run.fp_muls < b6.run.fp_muls);
        }
    }

    #[test]
    fn e34_cheap_representatives_equal_e29s_and_controls_are_caught() {
        let inst = small_instance();
        for arm in [
            RepArm::Order24Cheap,
            RepArm::Order24CheapMulLean,
            RepArm::Order6Cheap,
        ] {
            let c = identity_check(&inst, arm, 64, 7);
            assert!(c.forced_x0_zero > 0);
            assert_eq!(c.mismatches, 0, "{arm:?}");
            assert_eq!(c.scalar_failures, 0, "{arm:?}");
        }
        for arm in [RepArm::Order24OmitCandidate, RepArm::Order24WrongOrdinate] {
            let c = identity_check(&inst, arm, 64, 7);
            assert!(c.mismatches > 0, "negative control {arm:?} not caught");
        }
        // The omitted-candidate walk visits other representatives.
        let b = rho_j0_counted(&inst, RepArm::Order24Built, 1000, false);
        let x = rho_j0_counted(&inst, RepArm::Order24OmitCandidate, 1000, false);
        assert_ne!(b.run.visit_digest, x.run.visit_digest);
    }
}
