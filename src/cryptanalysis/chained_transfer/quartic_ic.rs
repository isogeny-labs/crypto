//! # Open case O4: the line-relation index calculus on a bielliptic plane quartic, counted.
//!
//! Protocol (frozen by hash): `research/chained_transfer_next_20261115/PROTOCOL.md`;
//! its "Amendment 1" governs where it differs from the original sections.
//!
//! `C: v^4 = x^3 + a x + b` covers `E: y^2 = x^3 + a x + b` by
//! `(x, v) -> (x, v^2)`.  A line through two rational points of `C` meets it in
//! a section of degree 4; when the residual quadratic splits over `F_p`, the
//! norm of the section is `0` on `E` and gives the row `sum_i c(R_i) = 0` on the
//! columns of the factor base (the distinct images `c(R)` folded by `+-`).
//!
//! This module is a counted copy of `src/cryptanalysis/bielliptic_quartic.rs`
//! (that file is not edited): the same `rational_points`, `secant`,
//! `line_relation` and `verify_relation` semantics, without the `p <= 257` gate,
//! for `p <= 2^20` (A8), every product reduced mod `p` before the next one, and
//! every `F_p` and `F_r` operation counted by class (A1, A13):
//! `MulP`, `AddP`, `InvP` (one per Fermat inversion), `MulR`, `AddR`, `InvR`,
//! `Canon`, `Lookup`, `Insert`, `Draw`.  Elliptic additions and doublings are
//! counted as group operations and priced by `full_jacobian_ic::ec_add_tally`.
//!
//! Arms: [`solve_quartic_ic`] (arm Q, rows from line sections) and
//! [`solve_direct_ic`] (arm D, rows from elliptic additions of three factor-base
//! points).  Both use one linear algebra (A5): a counted copy of
//! `ic_control::peel` without a target column, a counted copy of
//! `ic_control::wiedemann` that records `InvR` instead of
//! `pivot_inversion_steps(r)` multiplications, and counted back-substitution of
//! every peeled column.  Every run is cold (A6): it is built from
//! `(p, a, b, r, G, T)` alone.
//!
//! Every figure produced with this module is `[pcc:unverified]`.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::ec::{Curve, Pt};
use super::field::{Fp, F2};
use super::full_jacobian_ic::{ec_add_tally, fit_loglog};
use super::ic_control::{addm, mulmod, subm, Row};
use super::negation_remeasure::best_of;
use crate::cryptanalysis::ic_boundary::{invmod, FieldOps};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};

// ── frozen constants (Amendment 1) ───────────────────────────────────────

/// Stated arithmetic domain (A8).
pub const MAX_P: u64 = 1 << 20;
/// Selection scan length (`u = 0..3999`).
pub const SCAN_LEN: u64 = 4000;
/// Target shifts `T + [h]G`, `h = 0..511`.
pub const MAX_SHIFTS: u64 = 512;
/// Pair-trial (Q) or triple (D) budget per folded column (A7).
pub const BUDGET_PER_COLUMN: u64 = 64;
/// Rows beyond `n` before the first solve (A7).
pub const ROW_MARGIN: usize = 20;
/// `seed0` of A10.
pub const SEED0: u64 = 0x5EED_0415;
/// Multiplier of the instance scalar `x_k` (A10).
pub const X_MULT: u128 = 2_246_822_519;
pub const BASE_Q: u64 = 700;
pub const BASE_D: u64 = 500;
pub const RHO_BASE: u64 = 1000;
/// Salt of the linear-algebra RNG stream (A10).
pub const LA_SALT: u64 = 0x4C41;
/// Salt of the 32-column BSGS check (A9 ii).
pub const BSGS_SALT: u64 = 0xB565;
/// Columns checked by BSGS at `k = 0` (A9 ii).
pub const BSGS_COLUMNS: usize = 32;
/// Salt of the tampered-row index (A9 iii).
pub const TAMPER_SALT: u64 = 0x7A3B;
/// Stage-1 and stage-2 prime ladders, all `p = 1 (mod 4)` (A2).
pub const STAGE1: [u64; 7] = [229, 421, 821, 1621, 3209, 6421, 12841];
pub const STAGE2: [u64; 3] = [25601, 51217, 102409];
/// Phases of the phase-share table (A16).
pub const PHASES: [&str; 4] = ["setup", "collect", "la", "log"];

/// What the counters of arms Q and D do not see (A6).
pub const LOWER_BOUND_NOTE: &str = "lower bound: not counted are the line de-duplication set (HashSet), the column-weight bookkeeping of the stopping rule, the merge of duplicate column entries inside one row, allocation, sorting and memory writes of the tables, the point list and the rows, the table reads of point enumeration (the fourth-power table), the integer arithmetic of the u64 elliptic additions beyond their ec_add_tally field operations, and comparisons; the exact F_p work of a Fermat inversion is one InvP at the inversion weight";

/// One row of the frozen select-only table (A4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct A4Row {
    pub i: usize,
    pub p: u64,
    pub u: u64,
    pub a: u64,
    pub b: u64,
    pub r: u64,
    pub points_c: u64,
    pub n: u64,
}

/// The A4 table of Amendment 1 (no hop at any prime).
#[rustfmt::skip]
pub const A4_TABLE: [A4Row; 10] = [
    A4Row { i: 0, p: 229, u: 38, a: 65, b: 224, r: 223, points_c: 253, n: 63 },
    A4Row { i: 1, p: 421, u: 5, a: 139, b: 134, r: 457, points_c: 485, n: 121 },
    A4Row { i: 2, p: 821, u: 2, a: 313, b: 186, r: 877, points_c: 905, n: 226 },
    A4Row { i: 3, p: 1621, u: 13, a: 384, b: 149, r: 1627, points_c: 1549, n: 387 },
    A4Row { i: 4, p: 3209, u: 2, a: 2442, b: 2430, r: 3119, points_c: 3013, n: 753 },
    A4Row { i: 5, p: 6421, u: 44, a: 3361, b: 1552, r: 6521, points_c: 6381, n: 1595 },
    A4Row { i: 6, p: 12841, u: 22, a: 3731, b: 9102, r: 12919, points_c: 12829, n: 3207 },
    A4Row { i: 7, p: 25601, u: 59, a: 20573, b: 16511, r: 25423, points_c: 25425, n: 6356 },
    A4Row { i: 8, p: 51217, u: 2, a: 8600, b: 30914, r: 51131, points_c: 51533, n: 12883 },
    A4Row { i: 9, p: 102409, u: 4, a: 96014, b: 95982, r: 102359, points_c: 102365, n: 25591 },
];

// ── counters ─────────────────────────────────────────────────────────────

/// Native operations by class (A1, A13).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Ops {
    pub mul_p: u64,
    pub add_p: u64,
    pub inv_p: u64,
    pub mul_r: u64,
    pub add_r: u64,
    pub inv_r: u64,
    pub canon: u64,
    pub lookup: u64,
    pub insert: u64,
    pub draw: u64,
}

impl Ops {
    pub fn merge(&mut self, o: &Ops) {
        self.mul_p += o.mul_p;
        self.add_p += o.add_p;
        self.inv_p += o.inv_p;
        self.mul_r += o.mul_r;
        self.add_r += o.add_r;
        self.inv_r += o.inv_r;
        self.canon += o.canon;
        self.lookup += o.lookup;
        self.insert += o.insert;
        self.draw += o.draw;
    }
    /// The field-unit base of the native counters: `MulP + MulR + w_inv (InvP + InvR)`.
    pub fn base_field(&self, w_inv: f64) -> f64 {
        (self.mul_p + self.mul_r) as f64 + w_inv * (self.inv_p + self.inv_r) as f64
    }
    /// The classes left native in the field unit (band and `w_cal`).
    pub fn native_field(&self) -> [(&'static str, f64); 6] {
        [
            ("canon", self.canon as f64),
            ("lookup", self.lookup as f64),
            ("insert", self.insert as f64),
            ("draw", self.draw as f64),
            ("add_p", self.add_p as f64),
            ("add_r", self.add_r as f64),
        ]
    }
    /// Every class (the GAE reading, where only group operations are the base).
    pub fn native_all(&self) -> [(&'static str, f64); 10] {
        [
            ("mul_p", self.mul_p as f64),
            ("add_p", self.add_p as f64),
            ("inv_p", self.inv_p as f64),
            ("mul_r", self.mul_r as f64),
            ("add_r", self.add_r as f64),
            ("inv_r", self.inv_r as f64),
            ("canon", self.canon as f64),
            ("lookup", self.lookup as f64),
            ("insert", self.insert as f64),
            ("draw", self.draw as f64),
        ]
    }
}

/// Group operations on `E` and the field operations behind them
/// (`full_jacobian_ic::ec_add_tally`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct GroupTally {
    pub adds: u64,
    pub doubles: u64,
    pub field: FieldOps,
}

impl GroupTally {
    pub fn merge(&mut self, o: &GroupTally) {
        self.adds += o.adds;
        self.doubles += o.doubles;
        self.field.merge(o.field);
    }
    pub fn ops(&self) -> u64 {
        self.adds + self.doubles
    }
    /// `muls + sqrs + w_inv · invs`.
    pub fn field_units(&self, w_inv: f64) -> f64 {
        (self.field.muls + self.field.sqrs) as f64 + w_inv * self.field.invs as f64
    }
}

// ── uncounted helpers ────────────────────────────────────────────────────

/// `a^e mod m` for `m < 2^32` (products stay below `2^64`).
fn powu(a: u64, mut e: u64, m: u64) -> u64 {
    let mut b = a % m;
    let mut res = 1 % m;
    while e > 0 {
        if e & 1 == 1 {
            res = res * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    res
}

fn subu(a: u64, b: u64, m: u64) -> u64 {
    let b = b % m;
    if a >= b {
        a - b
    } else {
        a + m - b
    }
}

/// The least prime `q > from` with `q = 1 (mod 4)` (the hop of A2).
pub fn next_prime_1mod4(from: u64) -> u64 {
    let mut q = from + 1;
    while !(q % 4 == 1 && is_prime_u64(q)) {
        q += 1;
    }
    q
}

// ── counted F_p ──────────────────────────────────────────────────────────

/// `F_p` with every operation counted.  `p <= 2^20`: every product of two
/// reduced residues is below `2^40`.
#[derive(Clone, Copy, Debug)]
struct Fq {
    p: u64,
}

impl Fq {
    #[inline]
    fn mul(self, a: u64, b: u64, o: &mut Ops) -> u64 {
        o.mul_p += 1;
        a * b % self.p
    }
    #[inline]
    fn add(self, a: u64, b: u64, o: &mut Ops) -> u64 {
        o.add_p += 1;
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline]
    fn sub(self, a: u64, b: u64, o: &mut Ops) -> u64 {
        o.add_p += 1;
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline]
    fn neg(self, a: u64, o: &mut Ops) -> u64 {
        o.add_p += 1;
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    /// `k · a` for `k` in {2, 4, 6}, by additions (1, 2 and 3 of them).
    fn small(self, k: u64, a: u64, o: &mut Ops) -> u64 {
        o.add_p += match k {
            2 => 1,
            4 => 2,
            6 => 3,
            _ => unreachable!("small constant {k}"),
        };
        k * a % self.p
    }
    /// One Fermat inversion `a^(p-2)`, counted as one `InvP` (A13); `1` is free.
    fn inv(self, a: u64, o: &mut Ops) -> u64 {
        if a == 1 {
            return 1;
        }
        o.inv_p += 1;
        powu(a, self.p - 2, self.p)
    }
}

// ── the elliptic curve E, u64 affine, with the field tally ───────────────

/// A point of `E(F_p)`; `None` is the point at infinity.
pub type EcPt = Option<(u64, u64)>;
/// A point of `C(F_p)`; `None` is the point `(1:0:0)` at infinity.
pub type QPt = Option<(u64, u64)>;

/// `E: y^2 = x^3 + a x + b` in `u64` affine coordinates (the `ic_boundary::invmod`
/// path), with `ec.rs`'s `Curve` kept for the tally and the independent checks.
#[derive(Clone, Copy, Debug)]
pub struct EcU64 {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub curve: Curve,
}

impl EcU64 {
    pub fn new(p: u64, a: u64, b: u64) -> Option<EcU64> {
        let k = Fp::new(p)?;
        let curve = Curve::new(k, 0, a, b)?;
        Some(EcU64 { p, a, b, curve })
    }
    pub fn to_pt(q: EcPt) -> Pt {
        q.map(|(x, y)| (F2::base(x), F2::base(y)))
    }
    pub fn from_pt(q: &Pt) -> EcPt {
        q.map(|(x, y)| (x.a, y.a))
    }
    pub fn on_curve(&self, q: EcPt) -> bool {
        match q {
            None => true,
            Some((x, y)) => {
                let p = self.p;
                x < p && y < p && y * y % p == (x * x % p * x % p + self.a * x % p + self.b) % p
            }
        }
    }
    pub fn neg(&self, q: EcPt) -> EcPt {
        q.map(|(x, y)| (x, if y == 0 { 0 } else { self.p - y }))
    }
    /// The group law, uncounted.
    pub fn add_raw(&self, l: EcPt, r: EcPt) -> EcPt {
        let p = self.p;
        let ((x1, y1), (x2, y2)) = match (l, r) {
            (None, _) => return r,
            (_, None) => return l,
            (Some(u), Some(v)) => (u, v),
        };
        let m = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            let num = (3 * (x1 * x1 % p) + self.a) % p;
            num * invmod(2 * y1 % p, p).expect("2y invertible") % p
        } else {
            subu(y2, y1, p) * invmod(subu(x2, x1, p), p).expect("dx invertible") % p
        };
        let x3 = subu(subu(m * m % p, x1, p), x2, p);
        let y3 = subu(m * subu(x1, x3, p) % p, y1, p);
        Some((x3, y3))
    }
    /// One counted group operation: an addition (or a doubling when `l == r`)
    /// of two affine points, priced by `ec_add_tally`; a sum with the
    /// identity is no operation.
    pub fn add(&self, l: EcPt, r: EcPt, t: &mut GroupTally) -> EcPt {
        if let (Some(_), Some(_)) = (l, r) {
            if l == r {
                t.doubles += 1;
            } else {
                t.adds += 1;
            }
            t.field
                .merge(ec_add_tally(&self.curve, &Self::to_pt(l), &Self::to_pt(r)));
        }
        self.add_raw(l, r)
    }
    /// `[k]P`, double-and-add, counted.
    pub fn mul(&self, q: EcPt, mut k: u64, t: &mut GroupTally) -> EcPt {
        let mut acc = None;
        let mut b = q;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, b, t);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(b, b, t);
            }
        }
        acc
    }
    /// `[k]P` by `ec.rs` (independent of the `u64` code), for certificates.
    pub fn mul_independent(&self, q: EcPt, k: u64) -> EcPt {
        Self::from_pt(&self.curve.mul(&Self::to_pt(q), k as u128))
    }
}

// ── the quartic cover ────────────────────────────────────────────────────

/// A normalised homogeneous line `a X + b V + c Z = 0`: the first nonzero
/// coefficient is 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Line {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

/// A complete line section with all intersection multiplicities.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cert {
    pub line: Line,
    pub terms: Vec<(QPt, u8)>,
}

struct Section {
    poly: Vec<u64>,
    parameter_is_v: bool,
    infinity: u8,
    /// `1/b` when the parameter is `x`.
    inv_b: u64,
    /// The abscissa of a vertical line.
    x0: u64,
}

/// `C: v^4 = x^3 + a x + b` over `F_p`, `5 <= p <= 2^20`, with the
/// square-root and fourth-power tables built by `2p` counted products (A6).
#[derive(Clone, Debug)]
pub struct QuarticCover {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    /// Least square root of each residue, `u32::MAX` for a non-residue.
    sqrt_min: Vec<u32>,
    /// Every fourth root of each residue, ascending.
    fourth: Vec<Vec<u32>>,
}

fn trim(poly: &mut Vec<u64>) {
    while poly.len() > 1 && poly.last() == Some(&0) {
        poly.pop();
    }
}

impl QuarticCover {
    /// The cover without the 257 gate; tables charged to `o`.
    pub fn new(p: u64, a: u64, b: u64, o: &mut Ops) -> Result<QuarticCover, &'static str> {
        if !(5..=MAX_P).contains(&p) || !is_prime_u64(p) {
            return Err("unsupported field");
        }
        if a >= p || b >= p {
            return Err("noncanonical coefficient");
        }
        if (4 * powu(a, 3, p) + 27 * (b * b % p)).is_multiple_of(p) {
            return Err("singular curve");
        }
        let f = Fq { p };
        let mut sqrt_min = vec![u32::MAX; p as usize];
        let mut fourth = vec![Vec::new(); p as usize];
        for v in 0..p {
            let s = f.mul(v, v, o);
            if sqrt_min[s as usize] == u32::MAX {
                sqrt_min[s as usize] = v as u32;
            }
            let q = f.mul(s, s, o);
            fourth[q as usize].push(v as u32);
        }
        Ok(QuarticCover {
            p,
            a,
            b,
            sqrt_min,
            fourth,
        })
    }
    fn fq(&self) -> Fq {
        Fq { p: self.p }
    }
    /// `f(x) = x (x^2 + a) + b`: 2 MulP, 2 AddP.
    pub fn rhs(&self, x: u64, o: &mut Ops) -> u64 {
        let f = self.fq();
        let x2 = f.mul(x, x, o);
        let t = f.add(x2, self.a, o);
        let t = f.mul(t, x, o);
        f.add(t, self.b, o)
    }
    fn rhs_raw(&self, x: u64) -> u64 {
        let p = self.p;
        (x * x % p * x % p + self.a * x % p + self.b) % p
    }
    /// The square roots of `w`, ascending (the order of the old `square_roots`).
    fn sqrt_roots(&self, w: u64) -> Vec<u64> {
        let s = self.sqrt_min[w as usize];
        if s == u32::MAX {
            vec![]
        } else if s == 0 {
            vec![0]
        } else {
            vec![u64::from(s), self.p - u64::from(s)]
        }
    }
    /// The fourth roots of `w`, ascending.
    pub fn fourth_roots(&self, w: u64) -> &[u32] {
        &self.fourth[w as usize]
    }
    pub fn contains(&self, q: QPt) -> bool {
        match q {
            None => true,
            Some((x, v)) => x < self.p && v < self.p && powu(v, 4, self.p) == self.rhs_raw(x),
        }
    }
    /// Every rational point: infinity first, then `x` ascending and `v`
    /// ascending (the order of `bielliptic_quartic::Cover::rational_points`).
    pub fn rational_points(&self) -> Vec<QPt> {
        let mut out = vec![None];
        for x in 0..self.p {
            for &v in &self.fourth[self.rhs_raw(x) as usize] {
                out.push(Some((x, u64::from(v))));
            }
        }
        out
    }
    /// The image `(x, v^2)` on `E`: 1 MulP.
    pub fn image(&self, q: QPt, o: &mut Ops) -> EcPt {
        q.map(|(x, v)| (x, self.fq().mul(v, v, o)))
    }
    fn normalise(&self, a: u64, b: u64, c: u64, o: &mut Ops) -> Result<Line, &'static str> {
        let f = self.fq();
        let co = [a, b, c];
        let k = co.iter().position(|&x| x != 0).ok_or("zero line")?;
        let inv = f.inv(co[k], o);
        let mut out = [0u64; 3];
        for (i, &x) in co.iter().enumerate() {
            out[i] = if i == k {
                1
            } else if x == 0 {
                0
            } else {
                f.mul(x, inv, o)
            };
        }
        Ok(Line {
            a: out[0],
            b: out[1],
            c: out[2],
        })
    }
    /// The line through two distinct points of `C`, counted.
    pub fn secant(&self, l: QPt, r: QPt, o: &mut Ops) -> Result<Line, &'static str> {
        if l == r {
            return Err("equal points");
        }
        let f = self.fq();
        let p = self.p;
        let (a, b, c) = match (l, r) {
            (Some((x, v)), Some((u, w))) => (
                f.sub(v, w, o),
                f.sub(u, x, o),
                f.sub(f.mul(x, w, o), f.mul(v, u, o), o),
            ),
            _ => {
                // the point at infinity (1:0:0) takes part: products by 0 and 1
                let co = |q: QPt| q.map_or((1, 0, 0), |(x, v)| (x, v, 1));
                let ((x, v, z), (u, w, t)) = (co(l), co(r));
                (
                    subu(v * t % p, z * w % p, p),
                    subu(z * u % p, x * t % p, p),
                    subu(x * w % p, v * u % p, p),
                )
            }
        };
        self.normalise(a, b, c, o)
    }
    fn on_line(&self, line: Line, q: QPt) -> bool {
        match q {
            None => line.a == 0,
            Some((x, v)) => (line.a * x + line.b * v + line.c).is_multiple_of(self.p),
        }
    }
    fn section(&self, line: Line, o: &mut Ops) -> Section {
        let f = self.fq();
        if line.b != 0 {
            let ib = f.inv(line.b, o);
            let m = f.neg(f.mul(line.a, ib, o), o);
            let t = f.neg(f.mul(line.c, ib, o), o);
            let t2 = f.mul(t, t, o);
            let t3 = f.mul(t2, t, o);
            let t4 = f.mul(t2, t2, o);
            let m2 = f.mul(m, m, o);
            let m3 = f.mul(m2, m, o);
            let m4 = f.mul(m2, m2, o);
            let c0 = f.sub(t4, self.b, o);
            let mt3 = f.mul(m, t3, o);
            let c1 = f.sub(f.small(4, mt3, o), self.a, o);
            let m2t2 = f.mul(m2, t2, o);
            let c2 = f.small(6, m2t2, o);
            let m3t = f.mul(m3, t, o);
            let c3 = f.sub(f.small(4, m3t, o), 1, o);
            let mut poly = vec![c0, c1, c2, c3, m4];
            trim(&mut poly);
            let infinity = (5 - poly.len()) as u8;
            Section {
                poly,
                parameter_is_v: false,
                infinity,
                inv_b: ib,
                x0: 0,
            }
        } else if line.a != 0 {
            let ia = f.inv(line.a, o);
            let x0 = f.neg(f.mul(line.c, ia, o), o);
            let fx = self.rhs(x0, o);
            Section {
                poly: vec![f.neg(fx, o), 0, 0, 0, 1],
                parameter_is_v: true,
                infinity: 0,
                inv_b: 0,
                x0,
            }
        } else {
            Section {
                poly: vec![1],
                parameter_is_v: false,
                infinity: 4,
                inv_b: 0,
                x0: 0,
            }
        }
    }
    fn divide_root(&self, poly: &[u64], root: u64, o: &mut Ops) -> Result<Vec<u64>, &'static str> {
        let f = self.fq();
        if poly.len() < 2 {
            return Err("invalid certificate");
        }
        let mut q = vec![0; poly.len() - 1];
        let last = q.len() - 1;
        q[last] = poly[last + 1];
        for i in (0..last).rev() {
            q[i] = f.add(poly[i + 1], f.mul(root, q[i + 1], o), o);
        }
        if f.add(poly[0], f.mul(root, q[0], o), o) != 0 {
            return Err("invalid certificate");
        }
        trim(&mut q);
        Ok(q)
    }
    #[allow(clippy::type_complexity)]
    fn residual_roots(
        &self,
        poly: &[u64],
        o: &mut Ops,
    ) -> Result<Option<Vec<(u64, u8)>>, &'static str> {
        let f = self.fq();
        Ok(match poly.len() {
            1 if poly[0] != 0 => Some(vec![]),
            2 => {
                let i1 = f.inv(poly[1], o);
                Some(vec![(f.mul(f.neg(poly[0], o), i1, o), 1)])
            }
            3 => {
                let sq = f.mul(poly[1], poly[1], o);
                let pr = f.mul(poly[2], poly[0], o);
                let disc = f.sub(sq, f.small(4, pr, o), o);
                o.lookup += 1;
                let roots = self.sqrt_roots(disc);
                if roots.is_empty() {
                    None
                } else {
                    let inv = f.inv(f.small(2, poly[2], o), o);
                    let mult = if disc == 0 { 2 } else { 1 };
                    Some(
                        roots
                            .iter()
                            .map(|&s| (f.mul(f.sub(s, poly[1], o), inv, o), mult))
                            .collect(),
                    )
                }
            }
            _ => return Err("invalid certificate"),
        })
    }
    fn section_point(&self, sec: &Section, line: Line, root: u64, o: &mut Ops) -> QPt {
        let f = self.fq();
        if sec.parameter_is_v {
            Some((sec.x0, root))
        } else {
            let s = f.add(f.mul(line.a, root, o), line.c, o);
            Some((root, f.mul(f.neg(s, o), sec.inv_b, o)))
        }
    }
    /// Two known intersections leave a residual of degree at most two; `None`
    /// when it does not split over `F_p`.  Counted; not verified here (the
    /// caller runs [`QuarticCover::verify_relation`] outside the total, A6).
    pub fn line_relation(
        &self,
        left: QPt,
        right: QPt,
        o: &mut Ops,
    ) -> Result<Option<Cert>, &'static str> {
        let line = self.secant(left, right, o)?;
        self.relation_on_line(line, left, right, o)
    }
    fn relation_on_line(
        &self,
        line: Line,
        left: QPt,
        right: QPt,
        o: &mut Ops,
    ) -> Result<Option<Cert>, &'static str> {
        let sec = self.section(line, o);
        let mut residual = sec.poly.clone();
        let mut terms = BTreeMap::<QPt, u8>::new();
        for q in [left, right] {
            if let Some((x, v)) = q {
                let root = if sec.parameter_is_v { v } else { x };
                residual = self.divide_root(&residual, root, o)?;
                *terms.entry(q).or_default() += 1;
            }
        }
        let Some(roots) = self.residual_roots(&residual, o)? else {
            return Ok(None);
        };
        for (root, mult) in roots {
            *terms
                .entry(self.section_point(&sec, line, root, o))
                .or_default() += mult;
        }
        if sec.infinity != 0 {
            terms.insert(None, sec.infinity);
        }
        Ok(Some(Cert {
            line,
            terms: terms.into_iter().collect(),
        }))
    }
    /// Independent section reconstruction and elliptic norm replay (uncounted;
    /// never calls the root solver or `line_relation`).
    pub fn verify_relation(&self, cert: &Cert) -> bool {
        let p = self.p;
        let mut scratch = Ops::default();
        let l = cert.line;
        if [l.a, l.b, l.c].iter().any(|&x| x >= p) {
            return false;
        }
        match [l.a, l.b, l.c].iter().find(|&&x| x != 0) {
            Some(&1) => {}
            _ => return false,
        }
        let ec = EcU64 {
            p,
            a: self.a,
            b: self.b,
            curve: match Fp::new(p).and_then(|k| Curve::new(k, 0, self.a, self.b)) {
                Some(c) => c,
                None => return false,
            },
        };
        let sec = self.section(l, &mut scratch);
        let mut product = vec![1u64];
        let mut degree = 0u16;
        let mut infinity = 0;
        let mut seen = HashSet::new();
        let mut norm: EcPt = None;
        for &(q, mult) in &cert.terms {
            if mult == 0 || mult > 4 || !seen.insert(q) || !self.contains(q) || !self.on_line(l, q)
            {
                return false;
            }
            degree += u16::from(mult);
            match q {
                None => infinity = mult,
                Some((x, v)) => {
                    let root = if sec.parameter_is_v { v } else { x };
                    for _ in 0..mult {
                        let mut next = vec![0; product.len() + 1];
                        for (i, &co) in product.iter().enumerate() {
                            next[i] = subu(next[i], root * co % p, p);
                            next[i + 1] = (next[i + 1] + co) % p;
                        }
                        product = next;
                    }
                }
            }
            let image = q.map(|(x, v)| (x, v * v % p));
            for _ in 0..mult {
                norm = ec.add_raw(norm, image);
            }
        }
        if degree != 4 || infinity != sec.infinity || product.len() != sec.poly.len() {
            return false;
        }
        let leading = *sec.poly.last().expect("nonempty");
        for co in &mut product {
            *co = *co * leading % p;
        }
        product == sec.poly && norm.is_none()
    }
}

/// An altered certificate (the line's `c` moved by one) must be rejected by
/// `verify_relation` (A9 iii).  `true` when it is.
pub fn certificate_tamper_rejected(cover: &QuarticCover, cert: &Cert) -> bool {
    let mut changed = cert.clone();
    changed.line.c = (changed.line.c + 1) % cover.p;
    let mut changed_mult = cert.clone();
    changed_mult.terms[0].1 = if changed_mult.terms[0].1 == 1 { 2 } else { 1 };
    cover.verify_relation(cert)
        && !cover.verify_relation(&changed)
        && !cover.verify_relation(&changed_mult)
}

// ── the factor base ──────────────────────────────────────────────────────

/// The folded columns (one per `x` with `f(x)` a fourth power) and the affine
/// rational points of `C`.
#[derive(Clone, Debug)]
pub struct FactorBase {
    /// `(x, y)` with `y = min(v^2, p - v^2)`, ascending in `x`.
    pub cols: Vec<(u64, u64)>,
    col_of_x: Vec<u32>,
    /// Every affine rational point of `C`.
    pub affine: Vec<(u64, u64)>,
    p: u64,
}

impl FactorBase {
    /// Point enumeration, charged in MulP (`f(x)` and `v^2`), Canon and Insert (A6).
    pub fn build(cover: &QuarticCover, o: &mut Ops) -> FactorBase {
        let p = cover.p;
        let f = cover.fq();
        let mut cols = Vec::new();
        let mut col_of_x = vec![u32::MAX; p as usize];
        let mut affine = Vec::new();
        for x in 0..p {
            let fx = cover.rhs(x, o);
            let roots = cover.fourth_roots(fx);
            let Some(&v0) = roots.first() else {
                continue;
            };
            for &v in roots {
                affine.push((x, u64::from(v)));
            }
            let y = f.mul(u64::from(v0), u64::from(v0), o);
            o.canon += 1;
            let rep = y.min(if y == 0 { 0 } else { p - y });
            o.insert += 1;
            col_of_x[x as usize] = cols.len() as u32;
            cols.push((x, rep));
        }
        FactorBase {
            cols,
            col_of_x,
            affine,
            p,
        }
    }
    pub fn len(&self) -> usize {
        self.cols.len()
    }
    pub fn is_empty(&self) -> bool {
        self.cols.is_empty()
    }
    /// The column of `±q` and whether `q` is the negative of the column point;
    /// `None` when `q` is not a column up to sign.  One Lookup, one Canon.
    pub fn fold(&self, q: EcPt, o: &mut Ops) -> Option<(u32, bool)> {
        let (x, y) = q?;
        o.lookup += 1;
        let c = self.col_of_x[x as usize];
        if c == u32::MAX {
            return None;
        }
        o.canon += 1;
        let rep = self.cols[c as usize].1;
        if y == rep {
            Some((c, false))
        } else if rep != 0 && y == self.p - rep {
            Some((c, true))
        } else {
            None
        }
    }
    pub fn point(&self, c: u32) -> EcPt {
        Some(self.cols[c as usize])
    }
}

/// Push `coef` on column `c` into a sparse row (merging, zero entries dropped
/// by [`finish_row`]); one AddR.
fn push_entry(row: &mut Vec<(u32, u64)>, c: u32, coef: u64, r: u64, o: &mut Ops) {
    o.add_r += 1;
    if let Some(e) = row.iter_mut().find(|e| e.0 == c) {
        e.1 = addm(e.1, coef, r);
    } else {
        row.push((c, coef));
    }
}

fn finish_row(mut row: Vec<(u32, u64)>) -> Vec<(u32, u64)> {
    row.retain(|e| e.1 != 0);
    row.sort_unstable_by_key(|e| e.0);
    row
}

/// The folded row of a certificate (images, folding, signs), counted.
pub fn cert_row(
    cover: &QuarticCover,
    fb: &FactorBase,
    cert: &Cert,
    r: u64,
    o: &mut Ops,
) -> Result<Vec<(u32, u64)>, &'static str> {
    let mut row = Vec::with_capacity(4);
    for &(q, mult) in &cert.terms {
        if q.is_none() {
            continue;
        }
        let img = cover.image(q, o);
        let (c, neg) = fb
            .fold(img, o)
            .ok_or("section image outside the factor base")?;
        let m = u64::from(mult) % r;
        push_entry(&mut row, c, if neg { subm(0, m, r) } else { m }, r, o);
    }
    Ok(finish_row(row))
}

// ── linear algebra: counted copies of ic_control (A5, A13) ───────────────

/// [`peel_no_target`]'s result.
#[derive(Clone, Debug, Default)]
pub struct PeelOut {
    pub core: Vec<Row>,
    pub core_cols: usize,
    /// Old column -> core column, `u32::MAX` if not in the core.
    pub map: Vec<u32>,
    /// `(row, column)` in peeling order.
    pub order: Vec<(u32, u32)>,
    /// Columns left without a row (rank deficient).
    pub free: Vec<u32>,
}

/// `ic_control::peel` without a target column, with the peeling order kept
/// for back-substitution and free columns reported.  The `AddR` count equals
/// `ic_control::peel`'s `Nat.add` on the same rows.
pub fn peel_no_target(rows: &[Row], ncols: usize, o: &mut Ops) -> PeelOut {
    let mut colrows: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut w = vec![0u32; ncols];
    for (i, (t, _)) in rows.iter().enumerate() {
        for &(c, _) in t {
            colrows[c as usize].push(i as u32);
            w[c as usize] += 1;
            o.add_r += 1;
        }
    }
    let mut alive = vec![true; rows.len()];
    let mut stack: Vec<u32> = (0..ncols as u32).filter(|&c| w[c as usize] == 1).collect();
    let mut order = Vec::new();
    let mut pivot = vec![false; ncols];
    while let Some(c) = stack.pop() {
        if w[c as usize] != 1 {
            continue;
        }
        let Some(&ri) = colrows[c as usize].iter().find(|&&i| alive[i as usize]) else {
            continue;
        };
        alive[ri as usize] = false;
        order.push((ri, c));
        pivot[c as usize] = true;
        for &(cc, _) in &rows[ri as usize].0 {
            o.add_r += 1;
            w[cc as usize] -= 1;
            if w[cc as usize] == 1 {
                stack.push(cc);
            }
        }
    }
    let mut map = vec![u32::MAX; ncols];
    let mut n = 0u32;
    let mut free = Vec::new();
    for c in 0..ncols {
        if w[c] > 0 {
            map[c] = n;
            n += 1;
        } else if !pivot[c] {
            free.push(c as u32);
        }
    }
    let mut core = Vec::new();
    for (i, (t, rhs)) in rows.iter().enumerate() {
        if alive[i] {
            let mut nt: Vec<(u32, u64)> = t.iter().map(|&(c, v)| (map[c as usize], v)).collect();
            nt.sort_unstable_by_key(|x| x.0);
            core.push((nt, *rhs));
        }
    }
    PeelOut {
        core,
        core_cols: n as usize,
        map,
        order,
        free,
    }
}

fn mat_vec_c(rows: &[Row], x: &[u64], y: &mut [u64], r: u64, o: &mut Ops) {
    for (i, (t, _)) in rows.iter().enumerate() {
        let mut acc = 0u64;
        for &(c, v) in t {
            let xv = x[c as usize];
            if v == 1 {
                acc = addm(acc, xv, r);
            } else if v == r - 1 {
                acc = subm(acc, xv, r);
            } else {
                o.mul_r += 1;
                acc = addm(acc, mulmod(v, xv, r), r);
            }
            o.add_r += 1;
        }
        y[i] = acc;
    }
}

fn mat_t_vec_c(rows: &[Row], y: &[u64], z: &mut [u64], r: u64, o: &mut Ops) {
    z.iter_mut().for_each(|v| *v = 0);
    for (i, (t, _)) in rows.iter().enumerate() {
        let yv = y[i];
        if yv == 0 {
            continue;
        }
        for &(c, v) in t {
            let term = if v == 1 {
                yv
            } else if v == r - 1 {
                subm(0, yv, r)
            } else {
                o.mul_r += 1;
                mulmod(v, yv, r)
            };
            z[c as usize] = addm(z[c as usize], term, r);
            o.add_r += 1;
        }
    }
}

fn m_apply_c(
    rows: &[Row],
    d: &[u64],
    v: &[u64],
    tmp: &mut [u64],
    out: &mut [u64],
    r: u64,
    o: &mut Ops,
) {
    mat_vec_c(rows, v, tmp, r, o);
    for (t, &di) in tmp.iter_mut().zip(d.iter()) {
        o.mul_r += 1;
        *t = mulmod(*t, di, r);
    }
    mat_t_vec_c(rows, tmp, out, r, o);
}

fn berlekamp_massey_c(s: &[u64], r: u64, o: &mut Ops) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut binv = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            o.mul_r += 1;
            o.add_r += 1;
            d = addm(d, mulmod(c[i], s[n - i], r), r);
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = {
            o.mul_r += 1;
            mulmod(d, binv, r)
        };
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bv) in b.iter().enumerate() {
            o.mul_r += 1;
            o.add_r += 1;
            c[i + m] = subm(c[i + m], mulmod(coef, bv, r), r);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            o.inv_r += 1;
            binv = powu(d, r - 2, r);
            m = 1;
        } else {
            m += 1;
        }
    }
    c.truncate(l + 1);
    c.resize(l + 1, 0);
    c
}

/// `ic_control::wiedemann`, line for line, with `InvR` recorded instead of
/// `pivot_inversion_steps(r)` multiplications (A13): the same random stream,
/// the same result, and `mul_r + pivot_inversion_steps(r) · inv_r` equal to
/// its `Nat.mul` (unit test `la_copy_matches_ic_control_counts`).
pub fn wiedemann_counted(
    rows: &[Row],
    ncols: usize,
    r: u64,
    rng: &mut StdRng,
    dense_u: bool,
    o: &mut Ops,
) -> Option<Vec<u64>> {
    let nr = rows.len();
    let d: Vec<u64> = (0..nr).map(|_| rng.gen_range(1..r)).collect();
    let mut tmp = vec![0u64; nr];
    for ((t, row), &di) in tmp.iter_mut().zip(rows.iter()).zip(d.iter()) {
        o.mul_r += 1;
        *t = mulmod(row.1, di, r);
    }
    let mut bp = vec![0u64; ncols];
    mat_t_vec_c(rows, &tmp, &mut bp, r, o);
    let mut proj: Vec<(usize, u64)> = Vec::new();
    if dense_u || ncols <= 16 {
        proj.extend((0..ncols).map(|i| (i, rng.gen_range(1..r))));
    } else {
        for _ in 0..16 {
            proj.push((rng.gen_range(0..ncols), rng.gen_range(1..r)));
        }
    }
    let len = 2 * ncols + 2;
    let mut seq = Vec::with_capacity(len);
    let mut v = bp.clone();
    let mut w = vec![0u64; ncols];
    for _ in 0..len {
        let mut s = 0u64;
        for &(i, uv) in &proj {
            o.mul_r += 1;
            o.add_r += 1;
            s = addm(s, mulmod(uv, v[i], r), r);
        }
        seq.push(s);
        m_apply_c(rows, &d, &v, &mut tmp, &mut w, r, o);
        std::mem::swap(&mut v, &mut w);
    }
    let c = berlekamp_massey_c(&seq, r, o);
    let l = c.len() - 1;
    if l == 0 || c[l] == 0 {
        return None;
    }
    let mut acc = vec![0u64; ncols];
    let mut v = bp.clone();
    for j in 0..l {
        let coef = c[l - 1 - j];
        for (a, &vi) in acc.iter_mut().zip(v.iter()) {
            o.mul_r += 1;
            o.add_r += 1;
            *a = addm(*a, mulmod(coef, vi, r), r);
        }
        if j + 1 < l {
            m_apply_c(rows, &d, &v, &mut tmp, &mut w, r, o);
            std::mem::swap(&mut v, &mut w);
        }
    }
    o.inv_r += 1;
    let sc = subm(0, powu(c[l], r - 2, r), r);
    for a in acc.iter_mut() {
        o.mul_r += 1;
        *a = mulmod(*a, sc, r);
    }
    let mut ax = vec![0u64; nr];
    mat_vec_c(rows, &acc, &mut ax, r, o);
    if ax.iter().zip(rows.iter()).all(|(a, row)| *a == row.1) {
        Some(acc)
    } else {
        None
    }
}

/// Counted back-substitution of every peeled column, in reverse peeling order.
pub fn back_substitute(rows: &[Row], order: &[(u32, u32)], x: &mut [u64], r: u64, o: &mut Ops) {
    for &(ri, c) in order.iter().rev() {
        let (t, rhs) = &rows[ri as usize];
        let mut acc = *rhs;
        let mut coef = 0;
        for &(cc, v) in t {
            if cc == c {
                coef = v;
                continue;
            }
            let xv = x[cc as usize];
            o.add_r += 1;
            acc = if v == 1 {
                subm(acc, xv, r)
            } else if v == r - 1 {
                addm(acc, xv, r)
            } else {
                o.mul_r += 1;
                subm(acc, mulmod(v, xv, r), r)
            };
        }
        x[c as usize] = if coef == 1 {
            acc
        } else if coef == r - 1 {
            o.add_r += 1;
            subm(0, acc, r)
        } else {
            o.inv_r += 1;
            o.mul_r += 1;
            mulmod(acc, powu(coef, r - 2, r), r)
        };
    }
}

/// One linear-algebra attempt.
#[derive(Clone, Debug, Default)]
pub struct LaAttempt {
    pub x: Option<Vec<u64>>,
    pub failure: Option<&'static str>,
    pub core_rows: usize,
    pub core_cols: usize,
    pub peeled: usize,
    pub free: usize,
}

/// Peel, counted Wiedemann on the core, back-substitution, and the residual
/// `A x = c` over every row (A5, A9 iii).
pub fn la_solve(
    rows: &[Row],
    ncols: usize,
    r: u64,
    rng: &mut StdRng,
    dense_u: bool,
    o: &mut Ops,
) -> LaAttempt {
    let pl = peel_no_target(rows, ncols, o);
    let mut out = LaAttempt {
        core_rows: pl.core.len(),
        core_cols: pl.core_cols,
        peeled: pl.order.len(),
        free: pl.free.len(),
        ..Default::default()
    };
    if !pl.free.is_empty() {
        out.failure = Some("free_column");
        return out;
    }
    let mut x = vec![0u64; ncols];
    if pl.core_cols > 0 {
        let Some(xc) = wiedemann_counted(&pl.core, pl.core_cols, r, rng, dense_u, o) else {
            out.failure = Some("wiedemann");
            return out;
        };
        for c in 0..ncols {
            let m = pl.map[c];
            if m != u32::MAX {
                x[c] = xc[m as usize];
            }
        }
    }
    back_substitute(rows, &pl.order, &mut x, r, o);
    let mut ax = vec![0u64; rows.len()];
    mat_vec_c(rows, &x, &mut ax, r, o);
    if ax.iter().zip(rows).any(|(a, row)| *a != row.1) {
        out.failure = Some("residual");
        return out;
    }
    out.x = Some(x);
    out
}

/// Dense Gauss–Jordan mod `r`, uncounted (A5 cross-check at `p <= 421`):
/// the rank and the unique solution, if the rank is full and the system
/// consistent.
pub fn dense_solve(rows: &[Row], ncols: usize, r: u64) -> (usize, Option<Vec<u64>>) {
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|(t, rhs)| {
            let mut v = vec![0u64; ncols + 1];
            for &(c, x) in t {
                v[c as usize] = addm(v[c as usize], x % r, r);
            }
            v[ncols] = rhs % r;
            v
        })
        .collect();
    let mut rank = 0;
    let mut where_col = vec![usize::MAX; ncols];
    for col in 0..ncols {
        let Some(piv) = (rank..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(rank, piv);
        let inv = powu(m[rank][col], r - 2, r);
        for v in m[rank].iter_mut() {
            *v = mulmod(*v, inv, r);
        }
        let prow = m[rank].clone();
        for (i, row) in m.iter_mut().enumerate() {
            if i != rank && row[col] != 0 {
                let f = row[col];
                for (a, &b) in row.iter_mut().zip(&prow) {
                    *a = subm(*a, mulmod(f, b, r), r);
                }
            }
        }
        where_col[col] = rank;
        rank += 1;
    }
    let consistent = m[rank..].iter().all(|row| row[ncols] == 0);
    if rank < ncols || !consistent {
        return (rank, None);
    }
    (rank, Some(where_col.iter().map(|&i| m[i][ncols]).collect()))
}

// ── instances, seeds (A10) ───────────────────────────────────────────────

/// The cold inputs of every run (A6): `(p, a, b, r, G, T)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Inst {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub r: u64,
    pub g: (u64, u64),
    pub t: (u64, u64),
}

/// `seed_k = seed0 xor (k · 0x1000)`.
pub fn seed_k(seed0: u64, k: u64) -> u64 {
    seed0 ^ (k * 0x1000)
}

/// `x_k = (seed_k · 2246822519 + 7) mod r`; `0` becomes `1` (flag set).
pub fn x_k(seed_k: u64, r: u64) -> (u64, bool) {
    let x = ((seed_k as u128 * X_MULT + 7) % r as u128) as u64;
    if x == 0 {
        (1, true)
    } else {
        (x, false)
    }
}

/// Arm of an index-calculus run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ArmKind {
    Q,
    D,
}

impl ArmKind {
    pub fn name(self) -> &'static str {
        match self {
            ArmKind::Q => "Q",
            ArmKind::D => "D",
        }
    }
}

/// IC seed `(arm, k, j) = base_arm + j + 10 k`.
pub fn ic_seed(arm: ArmKind, k: u64, j: u64) -> u64 {
    let base = match arm {
        ArmKind::Q => BASE_Q,
        ArmKind::D => BASE_D,
    };
    base + j + 10 * k
}

/// Linear-algebra stream seed `IC seed xor 0x4C41`.
pub fn la_seed(ic_seed: u64) -> u64 {
    ic_seed ^ LA_SALT
}

/// Rho seed `(k, j) = 1000 + j + 100 k`.
pub fn rho_seed(k: u64, j: u64) -> u64 {
    RHO_BASE + j + 100 * k
}

/// Index of the tampered row: `mix64(0x7A3B xor p) mod rows`.
pub fn tamper_row_index(p: u64, rows: usize) -> usize {
    (mix64(TAMPER_SALT ^ p) % rows as u64) as usize
}

/// The columns checked by BSGS: distinct draws of `StdRng::seed_from_u64(0xB565 xor p)`.
pub fn bsgs_columns(p: u64, n: usize) -> Vec<usize> {
    let mut rng = StdRng::seed_from_u64(BSGS_SALT ^ p);
    let want = BSGS_COLUMNS.min(n);
    let mut out = Vec::with_capacity(want);
    while out.len() < want {
        let c = rng.gen_range(0..n);
        if !out.contains(&c) {
            out.push(c);
        }
    }
    out
}

/// Instance `k` of a selected curve: `G_k = E.point_from_seed(seed_k)` and
/// `T = [x_k] G_k` (by `ec.rs`).  Returns the instance, `x_k` and whether `x_k`
/// was moved from 0 to 1.
pub fn make_instance(p: u64, a: u64, b: u64, r: u64, seed0: u64, k: u64) -> (Inst, u64, bool) {
    let e = EcU64::new(p, a, b).expect("nonsingular curve");
    let sk = seed_k(seed0, k);
    let g = EcU64::from_pt(&e.curve.point_from_seed(sk)).expect("affine generator");
    let (x, moved) = x_k(sk, r);
    let t = e
        .mul_independent(Some(g), x)
        .expect("x in 1..r gives an affine target");
    (Inst { p, a, b, r, g, t }, x, moved)
}

// ── selection (A2, A3, A4) ───────────────────────────────────────────────

/// `(a_u, b_u)` of A3.
pub fn candidate_ab(p: u64, i: usize, u: u64) -> (u64, u64) {
    let i = i as u64;
    (
        mix64(
            0xA4B1u64
                .wrapping_add(101u64.wrapping_mul(i))
                .wrapping_add(u),
        ) % p,
        mix64(
            0xB7C3u64
                .wrapping_add(101u64.wrapping_mul(i))
                .wrapping_add(u),
        ) % p,
    )
}

/// Why a candidate was rejected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Reject {
    AZero,
    BZero,
    Singular,
    RNotPrime,
    RSmall,
    Anomalous,
    LowEmbedding,
    FewPoints,
}

/// `r | p^k - 1` for some `k = 1..=kmax`.
pub fn low_embedding(p: u64, r: u64, kmax: u32) -> bool {
    r > 1 && (1..=kmax).any(|k| powu(p, u64::from(k), r) == 1 % r)
}

/// The checks of A3 on a candidate whose `r = #E` and `#C(F_p)` are known.
pub fn check_candidate(
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    points_c: u64,
    min_r_frac: f64,
) -> Result<(), Reject> {
    if a == 0 {
        return Err(Reject::AZero);
    }
    if b == 0 {
        return Err(Reject::BZero);
    }
    if (4 * powu(a, 3, p) + 27 * (b * b % p)).is_multiple_of(p) {
        return Err(Reject::Singular);
    }
    if !is_prime_u64(r) {
        return Err(Reject::RNotPrime);
    }
    if r < (min_r_frac * p as f64).ceil() as u64 {
        return Err(Reject::RSmall);
    }
    if r == p {
        return Err(Reject::Anomalous);
    }
    if low_embedding(p, r, 6) {
        return Err(Reject::LowEmbedding);
    }
    if points_c < 8 {
        return Err(Reject::FewPoints);
    }
    Ok(())
}

/// `#E(F_p)`, the number `n` of `x` with `f(x)` a nonzero fourth power, and
/// the number of roots of `f` (uncounted; selection is not an arm).
pub fn curve_counts(p: u64, a: u64, b: u64, sq: &[bool], q4: &[bool]) -> (u64, u64, u64) {
    let (mut cnt, mut n, mut nroot) = (1u64, 0u64, 0u64);
    for x in 0..p {
        let f = (x * x % p * x % p + a * x % p + b) % p;
        if f == 0 {
            cnt += 1;
            nroot += 1;
        } else if sq[f as usize] {
            cnt += 2;
            if q4[f as usize] {
                n += 1;
            }
        }
    }
    (cnt, n, nroot)
}

/// The selected instance curve of one requested prime.
#[derive(Clone, Debug, Serialize)]
pub struct Selected {
    pub index: usize,
    pub p_requested: u64,
    pub p: u64,
    pub passed_over: Vec<u64>,
    pub u: u64,
    pub a: u64,
    pub b: u64,
    pub r: u64,
    pub n: u64,
    pub points_c: u64,
    pub skips: BTreeMap<String, u32>,
}

/// The selection rule of A2 and A3 for the requested prime with list index `i`.
pub fn select_prime_order(
    p_req: u64,
    i: usize,
    min_r_frac: f64,
    max_hops: u32,
) -> Result<Selected, String> {
    if p_req % 4 != 1 || !is_prime_u64(p_req) {
        return Err(format!("{p_req} is not a prime = 1 (mod 4)"));
    }
    if p_req > MAX_P {
        return Err(format!("{p_req} is above the domain 2^20"));
    }
    let mut p = p_req;
    let mut passed = vec![];
    for _hop in 0..=max_hops {
        assert_eq!(p % 4, 1, "selection asserts p = 1 (mod 4)");
        let mut sq = vec![false; p as usize];
        let mut q4 = vec![false; p as usize];
        for v in 1..p {
            let s = v * v % p;
            sq[s as usize] = true;
            q4[(s * s % p) as usize] = true;
        }
        let mut skips: BTreeMap<String, u32> = BTreeMap::new();
        for u in 0..SCAN_LEN {
            let (a, b) = candidate_ab(p, i, u);
            let res = if a == 0 {
                Err(Reject::AZero)
            } else if b == 0 {
                Err(Reject::BZero)
            } else {
                let (r, n, nroot) = curve_counts(p, a, b, &sq, &q4);
                let points_c = 1 + 4 * n + nroot;
                check_candidate(p, a, b, r, points_c, min_r_frac).map(|()| (r, n, points_c))
            };
            match res {
                Ok((r, n, points_c)) => {
                    return Ok(Selected {
                        index: i,
                        p_requested: p_req,
                        p,
                        passed_over: passed,
                        u,
                        a,
                        b,
                        r,
                        n,
                        points_c,
                        skips,
                    })
                }
                Err(why) => *skips.entry(format!("{why:?}")).or_default() += 1,
            }
        }
        passed.push(p);
        p = next_prime_1mod4(p);
    }
    Err(format!(
        "no admissible candidate at {p_req} and {max_hops} next primes = 1 (mod 4)"
    ))
}

/// Row-for-row comparison of a selection with the A4 table: `p, u, a, b, r, n`.
pub fn a4_matches(s: &Selected) -> bool {
    A4_TABLE.get(s.index).is_some_and(|t| {
        t.p == s.p && t.u == s.u && t.a == s.a && t.b == s.b && t.r == s.r && t.n == s.n
    })
}

// ── one run of arm Q or D ────────────────────────────────────────────────

/// Parameters of one run.
#[derive(Clone, Copy, Debug)]
pub struct RunParams {
    /// IC seed; the collection stream is `StdRng::seed_from_u64(seed)`, the
    /// linear-algebra stream `StdRng::seed_from_u64(seed ^ 0x4C41)`.
    pub seed: u64,
    pub max_shifts: u64,
    pub budget_per_column: u64,
    /// Keep the final system (rows, certificates) in the output.
    pub keep_system: bool,
    /// Replay every factor-base log by a scalar multiplication (verification).
    pub replay_logs: bool,
}

impl RunParams {
    pub fn new(seed: u64) -> RunParams {
        RunParams {
            seed,
            max_shifts: MAX_SHIFTS,
            budget_per_column: BUDGET_PER_COLUMN,
            keep_system: false,
            replay_logs: true,
        }
    }
}

/// Verification-only work, outside every total (A6, A9 iv).
#[derive(Clone, Debug, Default, Serialize)]
pub struct Verification {
    /// `verify_relation` (Q) or the elliptic row sum (D) on every collected row.
    pub relation_checks: u64,
    pub relation_failures: u64,
    pub replayed_columns: u64,
    pub replay_mismatches: u64,
    pub replay_group: GroupTally,
    /// A rational fibre point of `C` over the shifted target exists.
    pub fibre_witness: Option<bool>,
    /// `[t]G == T` by `ec.rs`.
    pub final_check: bool,
}

/// The final system of a run, kept on request.
#[derive(Clone, Debug, Default)]
pub struct System {
    pub rows: Vec<Row>,
    pub certs: Vec<Cert>,
    pub cols: Vec<(u64, u64)>,
    pub logs: Option<Vec<u64>>,
}

/// One run of arm Q or D.
#[derive(Clone, Debug, Serialize)]
pub struct RunOut {
    pub arm: &'static str,
    pub seed: u64,
    pub la_seed: u64,
    pub log: Option<u64>,
    pub certified: bool,
    pub exhausted: bool,
    pub failure: Option<String>,
    pub columns: usize,
    /// Relation rows of the system that was solved (the anchor not included).
    pub rows_used: usize,
    pub attempts: u32,
    pub attempt_failures: Vec<&'static str>,
    /// Pair trials (Q) or triples drawn (D), every attempt included.
    pub trials: u64,
    pub budget: u64,
    pub duplicate_lines: u64,
    pub nonsplit: u64,
    pub zero_rows: u64,
    /// D: triples whose `-(P1 + P2 + P3)` is not a column (or the identity).
    pub rejected: u64,
    pub anchor_multiple: u64,
    pub shift: Option<u64>,
    pub core_rows: usize,
    pub core_cols: usize,
    pub peeled: usize,
    /// Native counters by phase ([`PHASES`]).
    pub ops: [Ops; 4],
    /// Group operations by phase.
    pub group: [GroupTally; 4],
    pub verify: Verification,
    pub wall_ns: u64,
    #[serde(skip)]
    pub system: Option<System>,
}

impl RunOut {
    pub fn total_ops(&self) -> Ops {
        let mut t = Ops::default();
        for o in &self.ops {
            t.merge(o);
        }
        t
    }
    pub fn total_group(&self) -> GroupTally {
        let mut t = GroupTally::default();
        for g in &self.group {
            t.merge(g);
        }
        t
    }
    /// Base in the field unit (A1): native base plus the group-operation tally.
    pub fn base_field(&self, w_inv: f64) -> f64 {
        self.total_ops().base_field(w_inv) + self.total_group().field_units(w_inv)
    }
    /// Base of one phase in the field unit.
    pub fn phase_base(&self, ph: usize, w_inv: f64) -> f64 {
        self.ops[ph].base_field(w_inv) + self.group[ph].field_units(w_inv)
    }
}

/// Arm Q: rows from complete line sections of `C`.
pub fn solve_quartic_ic(inst: &Inst, prm: RunParams) -> RunOut {
    run_arm(inst, ArmKind::Q, prm)
}

/// Arm D: rows from elliptic additions of three factor-base points.
pub fn solve_direct_ic(inst: &Inst, prm: RunParams) -> RunOut {
    run_arm(inst, ArmKind::D, prm)
}

fn signed(c: u32, neg: bool, r: u64) -> (u32, u64) {
    (c, if neg { r - 1 } else { 1 })
}

/// Check of one row by `ec.rs`: `sum coef · column point == O` (uncounted).
pub fn row_sum_is_zero(ec: &EcU64, cols: &[(u64, u64)], row: &[(u32, u64)], r: u64) -> bool {
    let mut acc: Pt = None;
    for &(c, v) in row {
        let k: i128 = if v > r / 2 {
            -((r - v) as i128)
        } else {
            v as i128
        };
        let q = ec.curve.mul_i(&EcU64::to_pt(Some(cols[c as usize])), k);
        acc = ec.curve.add(&acc, &q);
    }
    acc.is_none()
}

fn run_arm(inst: &Inst, kind: ArmKind, prm: RunParams) -> RunOut {
    let t0 = Instant::now();
    let (p, r) = (inst.p, inst.r);
    let mut ops = [Ops::default(); 4];
    let mut grp = [GroupTally::default(); 4];
    let mut out = RunOut {
        arm: kind.name(),
        seed: prm.seed,
        la_seed: la_seed(prm.seed),
        log: None,
        certified: false,
        exhausted: false,
        failure: None,
        columns: 0,
        rows_used: 0,
        attempts: 0,
        attempt_failures: vec![],
        trials: 0,
        budget: 0,
        duplicate_lines: 0,
        nonsplit: 0,
        zero_rows: 0,
        rejected: 0,
        anchor_multiple: 0,
        shift: None,
        core_rows: 0,
        core_cols: 0,
        peeled: 0,
        ops,
        group: grp,
        verify: Verification::default(),
        wall_ns: 0,
        system: None,
    };
    let fail = |mut out: RunOut, why: &str, ops, grp, t0: Instant| {
        out.failure = Some(why.to_string());
        out.ops = ops;
        out.group = grp;
        out.wall_ns = t0.elapsed().as_nanos() as u64;
        out
    };
    // ── set-up: tables, enumeration, factor base, anchor (cold) ──
    let cover = match QuarticCover::new(p, inst.a, inst.b, &mut ops[0]) {
        Ok(c) => c,
        Err(e) => return fail(out, e, ops, grp, t0),
    };
    let Some(ec) = EcU64::new(p, inst.a, inst.b) else {
        return fail(out, "singular curve", ops, grp, t0);
    };
    let g: EcPt = Some(inst.g);
    if !ec.on_curve(g) || !ec.on_curve(Some(inst.t)) {
        return fail(out, "point not on E", ops, grp, t0);
    }
    let fb = FactorBase::build(&cover, &mut ops[0]);
    let n = fb.len();
    out.columns = n;
    if n == 0 || (kind == ArmKind::Q && fb.affine.len() < 2) {
        return fail(out, "empty factor base", ops, grp, t0);
    }
    let mut anchor = g;
    let mut m = 1u64;
    let (acol, aneg) = loop {
        if let Some(f) = fb.fold(anchor, &mut ops[0]) {
            break f;
        }
        if m >= r {
            return fail(out, "no anchor", ops, grp, t0);
        }
        anchor = ec.add(anchor, g, &mut grp[0]);
        m += 1;
    };
    out.anchor_multiple = m;
    let anchor_rhs = if aneg { subm(0, m % r, r) } else { m % r };
    let mut rows: Vec<Row> = vec![(vec![(acol, 1)], anchor_rhs)];
    let mut certs: Vec<Cert> = vec![];
    // ── collection and linear algebra (A7) ──
    let mut rng = StdRng::seed_from_u64(prm.seed);
    let mut la_rng = StdRng::seed_from_u64(la_seed(prm.seed));
    let budget = prm.budget_per_column * n as u64;
    out.budget = budget;
    let mut weight = vec![0u32; n];
    let mut covered = 0usize;
    let mut target = n + ROW_MARGIN;
    let mut seen: HashSet<Line> = HashSet::new();
    let npts = fb.affine.len();
    let mut logs: Option<Vec<u64>> = None;
    loop {
        let rel_rows = rows.len() - 1;
        if covered == n && rel_rows >= target {
            out.attempts += 1;
            let la = la_solve(&rows, n, r, &mut la_rng, out.attempts > 1, &mut ops[2]);
            out.core_rows = la.core_rows;
            out.core_cols = la.core_cols;
            out.peeled = la.peeled;
            if let Some(x) = la.x {
                out.rows_used = rel_rows;
                logs = Some(x);
                break;
            }
            out.attempt_failures.push(la.failure.unwrap_or("unknown"));
            target = rel_rows + n.div_ceil(10);
            continue;
        }
        if out.trials >= budget {
            out.exhausted = true;
            break;
        }
        out.trials += 1;
        let o = &mut ops[1];
        let row = match kind {
            ArmKind::Q => {
                o.draw += 2;
                let i = rng.gen_range(0..npts);
                let mut j = rng.gen_range(0..npts - 1);
                if j >= i {
                    j += 1;
                }
                let (left, right) = (Some(fb.affine[i]), Some(fb.affine[j]));
                let line = match cover.secant(left, right, o) {
                    Ok(l) => l,
                    Err(e) => return fail(out, e, ops, grp, t0),
                };
                if !seen.insert(line) {
                    out.duplicate_lines += 1;
                    continue;
                }
                let cert = match cover.relation_on_line(line, left, right, o) {
                    Ok(Some(c)) => c,
                    Ok(None) => {
                        out.nonsplit += 1;
                        continue;
                    }
                    Err(e) => return fail(out, e, ops, grp, t0),
                };
                let row = match cert_row(&cover, &fb, &cert, r, o) {
                    Ok(row) => row,
                    Err(e) => return fail(out, e, ops, grp, t0),
                };
                if !row.is_empty() {
                    certs.push(cert);
                }
                row
            }
            ArmKind::D => {
                o.draw += 6;
                let mut picks = [(0u32, false); 3];
                for pk in picks.iter_mut() {
                    let c = rng.gen_range(0..n) as u32;
                    let neg = rng.gen_range(0..2u32) == 1;
                    *pk = (c, neg);
                }
                let pt = |(c, neg): (u32, bool), o: &mut Ops| {
                    let q = fb.point(c);
                    if neg {
                        o.add_p += 1;
                        ec.neg(q)
                    } else {
                        q
                    }
                };
                let p1 = pt(picks[0], o);
                let p2 = pt(picks[1], o);
                let p3 = pt(picks[2], o);
                let s = ec.add(p1, p2, &mut grp[1]);
                let s = ec.add(s, p3, &mut grp[1]);
                if s.is_none() {
                    out.rejected += 1;
                    continue;
                }
                o.add_p += 1;
                let q4 = ec.neg(s);
                let Some((c4, n4)) = fb.fold(q4, o) else {
                    out.rejected += 1;
                    continue;
                };
                let mut row = Vec::with_capacity(4);
                for (c, neg) in picks.into_iter().chain([(c4, n4)]) {
                    let (c, v) = signed(c, neg, r);
                    push_entry(&mut row, c, v, r, o);
                }
                finish_row(row)
            }
        };
        if row.is_empty() {
            out.zero_rows += 1;
            continue;
        }
        for &(c, _) in &row {
            if weight[c as usize] == 0 {
                covered += 1;
            }
            weight[c as usize] += 1;
        }
        rows.push((row, 0));
    }
    // ── verification of every collected row (uncounted, A9 iv) ──
    for (k, (row, _)) in rows.iter().enumerate().skip(1) {
        out.verify.relation_checks += 1;
        let ok = match kind {
            ArmKind::Q => {
                cover.verify_relation(&certs[k - 1]) && {
                    let mut scratch = Ops::default();
                    cert_row(&cover, &fb, &certs[k - 1], r, &mut scratch).is_ok_and(|v| v == *row)
                }
            }
            ArmKind::D => row_sum_is_zero(&ec, &fb.cols, row, r),
        };
        if !ok {
            out.verify.relation_failures += 1;
        }
    }
    let Some(x) = logs else {
        let why = if out.exhausted {
            "budget_exhausted"
        } else {
            "no_solution"
        };
        if prm.keep_system {
            out.system = Some(System {
                rows,
                certs,
                cols: fb.cols.clone(),
                logs: None,
            });
        }
        return fail(out, why, ops, grp, t0);
    };
    // ── individual logarithm (A1: shifts are group operations) ──
    let tpt: EcPt = Some(inst.t);
    let mut shifted = tpt;
    let mut cand = None;
    for h in 0..prm.max_shifts.min(r) {
        let o = &mut ops[3];
        if shifted.is_none() {
            o.add_r += 1;
            cand = Some((subm(0, h % r, r), h, None));
            break;
        }
        if let Some((c, neg)) = fb.fold(shifted, o) {
            let xc = x[c as usize];
            o.add_r += 1 + u64::from(neg);
            let lc = if neg { subm(0, xc, r) } else { xc };
            cand = Some((subm(lc, h % r, r), h, Some(c)));
            break;
        }
        shifted = ec.add(shifted, g, &mut grp[3]);
    }
    // ── verification-only work (A6) ──
    if prm.replay_logs {
        for (c, &xc) in x.iter().enumerate() {
            out.verify.replayed_columns += 1;
            if ec.mul(g, xc, &mut out.verify.replay_group) != fb.point(c as u32) {
                out.verify.replay_mismatches += 1;
            }
        }
    }
    match cand {
        Some((t, h, col)) => {
            out.log = Some(t);
            out.shift = Some(h);
            out.verify.final_check = ec.mul_independent(g, t) == tpt;
            out.certified = out.verify.final_check;
            out.verify.fibre_witness = col.map(|_| {
                let (sx, sy) = shifted.expect("affine shifted target");
                let fx = cover.rhs_raw(sx);
                cover.fourth_roots(fx).iter().any(|&v| {
                    let y = u64::from(v) * u64::from(v) % p;
                    y == sy || y == subu(0, sy, p)
                })
            });
            if !out.certified {
                out.failure = Some("uncertified".into());
            }
        }
        None => out.failure = Some("target_shifts_exhausted".into()),
    }
    if prm.keep_system {
        out.system = Some(System {
            rows,
            certs,
            cols: fb.cols.clone(),
            logs: Some(x),
        });
    }
    out.ops = ops;
    out.group = grp;
    out.wall_ns = t0.elapsed().as_nanos() as u64;
    out
}

// ── controls ─────────────────────────────────────────────────────────────

/// The individual log of `t` from factor-base logs (uncounted; controls only).
pub fn log_from_columns(
    ec: &EcU64,
    cols: &[(u64, u64)],
    logs: &[u64],
    g: EcPt,
    t: EcPt,
    r: u64,
    max_shifts: u64,
) -> Option<(u64, u64)> {
    let idx: HashMap<u64, usize> = cols.iter().enumerate().map(|(i, c)| (c.0, i)).collect();
    let mut shifted = t;
    for h in 0..max_shifts.min(r) {
        match shifted {
            None => return Some((subm(0, h % r, r), h)),
            Some((x, y)) => {
                if let Some(&c) = idx.get(&x) {
                    let rep = cols[c].1;
                    let lc = if y == rep {
                        logs[c]
                    } else {
                        subm(0, logs[c], r)
                    };
                    return Some((subm(lc, h % r, r), h));
                }
            }
        }
        shifted = ec.add_raw(shifted, g);
    }
    None
}

/// The tamper control of A9 iii.
#[derive(Clone, Debug, Serialize)]
pub struct TamperOut {
    pub rows: usize,
    pub row_index: usize,
    pub column: u32,
    pub old: u64,
    pub new: u64,
    pub la_failure: Option<&'static str>,
    pub target_certified: Option<bool>,
    pub replay_mismatches: Option<u64>,
    pub detected: bool,
    pub detected_by: Vec<&'static str>,
}

/// Run Q with `seed`, take the system of its final solve, change one entry of
/// row `mix64(0x7A3B ^ p) mod rows` (the anchor is row 0), and solve that
/// system once (fresh linear-algebra stream, no retry).  Detected when the
/// solve fails (free column, Wiedemann or residual), the target log is not
/// certified, or a factor-base log fails its replay.  `None` when the clean
/// run did not solve.
pub fn tamper_control(inst: &Inst, seed: u64) -> Option<(RunOut, TamperOut)> {
    let mut prm = RunParams::new(seed);
    prm.keep_system = true;
    prm.replay_logs = false;
    let clean = solve_quartic_ic(inst, prm);
    let sys = clean.system.as_ref()?;
    sys.logs.as_ref()?;
    let r = inst.r;
    let mut rows = sys.rows.clone();
    let idx = tamper_row_index(inst.p, rows.len());
    let (col, old) = rows[idx].0[0];
    let mut new = addm(old, 1, r);
    if new == 0 {
        new = addm(old, 2, r);
    }
    rows[idx].0[0].1 = new;
    let mut la_rng = StdRng::seed_from_u64(la_seed(seed));
    let mut scratch = Ops::default();
    let la = la_solve(&rows, sys.cols.len(), r, &mut la_rng, false, &mut scratch);
    let ec = EcU64::new(inst.p, inst.a, inst.b).expect("curve");
    let mut out = TamperOut {
        rows: rows.len(),
        row_index: idx,
        column: col,
        old,
        new,
        la_failure: la.failure,
        target_certified: None,
        replay_mismatches: None,
        detected: false,
        detected_by: vec![],
    };
    if let Some(f) = la.failure {
        out.detected_by.push(f);
    }
    if let Some(x) = la.x {
        let g = Some(inst.g);
        let cert = log_from_columns(&ec, &sys.cols, &x, g, Some(inst.t), r, MAX_SHIFTS)
            .is_some_and(|(t, _)| ec.mul_independent(g, t) == Some(inst.t));
        out.target_certified = Some(cert);
        if !cert {
            out.detected_by.push("certification");
        }
        let mut tally = GroupTally::default();
        let mism = x
            .iter()
            .enumerate()
            .filter(|&(c, &xc)| ec.mul(g, xc, &mut tally) != Some(sys.cols[c]))
            .count() as u64;
        out.replay_mismatches = Some(mism);
        if mism > 0 {
            out.detected_by.push("factor_log_replay");
        }
    }
    out.detected = !out.detected_by.is_empty();
    Some((clean, out))
}

/// Baby-step giant-step on `E` by `ec.rs` (independent, uncounted; A9 ii).
pub fn bsgs_log(ec: &EcU64, g: EcPt, q: EcPt, r: u64) -> Option<u64> {
    let e = &ec.curve;
    let gp = EcU64::to_pt(g);
    let qp = EcU64::to_pt(q);
    let m = (r as f64).sqrt().ceil() as u64 + 1;
    let mut baby: HashMap<Option<(u64, u64)>, u64> = HashMap::with_capacity(m as usize);
    let mut cur: Pt = None;
    for j in 0..m {
        baby.entry(EcU64::from_pt(&cur)).or_insert(j);
        cur = e.add(&cur, &gp);
    }
    let giant = e.neg(&e.mul(&gp, m as u128));
    let mut y = qp;
    for i in 0..m {
        if let Some(&j) = baby.get(&EcU64::from_pt(&y)) {
            return Some((i * m + j) % r);
        }
        y = e.add(&y, &giant);
    }
    None
}

/// The 32-column BSGS check of A9 ii on a solved system: the number of
/// columns checked and the mismatches.
pub fn bsgs_check(inst: &Inst, cols: &[(u64, u64)], logs: &[u64]) -> (usize, usize) {
    let ec = EcU64::new(inst.p, inst.a, inst.b).expect("curve");
    let pick = bsgs_columns(inst.p, cols.len());
    let bad = pick
        .iter()
        .filter(|&&c| bsgs_log(&ec, Some(inst.g), Some(cols[c]), inst.r) != Some(logs[c]))
        .count();
    (pick.len(), bad)
}

// ── the exhaustive replay of A9 i ────────────────────────────────────────

/// The `--exhaustive` mode: every ordered pair `i < j` of the old pair order
/// (infinity first), as `bielliptic_quartic::collect_relations`, then the new
/// linear algebra.
#[derive(Clone, Debug, Serialize)]
pub struct Exhaustive {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub r: u64,
    pub generator: (u64, u64),
    pub rational_points: usize,
    pub possible_pairs: usize,
    pub pair_trials: usize,
    pub unique_lines: usize,
    pub duplicate_lines: usize,
    pub nonsplit_residuals: usize,
    pub certified_relations: usize,
    pub relation_failures: usize,
    pub columns: usize,
    pub zero_projected_rows: usize,
    pub dense_rank: usize,
    pub anchor_multiple: u64,
    pub la_attempts: u32,
    pub la_agrees_with_dense: bool,
    pub logs: Option<Vec<u64>>,
    pub factor_logs_replayed: usize,
    #[serde(skip)]
    pub cols: Vec<(u64, u64)>,
}

/// Build the exhaustive system and its logs for generator `g` (order `r`).
pub fn exhaustive(p: u64, a: u64, b: u64, r: u64, g: (u64, u64)) -> Result<Exhaustive, String> {
    let mut o = Ops::default();
    let cover = QuarticCover::new(p, a, b, &mut o)?;
    let ec = EcU64::new(p, a, b).ok_or("singular")?;
    let fb = FactorBase::build(&cover, &mut o);
    let n = fb.len();
    let points = cover.rational_points();
    let mut ex = Exhaustive {
        p,
        a,
        b,
        r,
        generator: g,
        rational_points: points.len(),
        possible_pairs: points.len() * (points.len() - 1) / 2,
        pair_trials: 0,
        unique_lines: 0,
        duplicate_lines: 0,
        nonsplit_residuals: 0,
        certified_relations: 0,
        relation_failures: 0,
        columns: n,
        zero_projected_rows: 0,
        dense_rank: 0,
        anchor_multiple: 0,
        la_attempts: 0,
        la_agrees_with_dense: false,
        logs: None,
        factor_logs_replayed: 0,
        cols: fb.cols.clone(),
    };
    // anchor, as `prepare`: the first multiple of G that is a column
    let gp = Some(g);
    let mut anchor = None;
    let mut arow = None;
    for m in 1..r {
        anchor = ec.add_raw(anchor, gp);
        if let Some((c, neg)) = fb.fold(anchor, &mut o) {
            arow = Some((vec![(c, 1u64)], if neg { subm(0, m, r) } else { m }));
            ex.anchor_multiple = m;
            break;
        }
    }
    let mut rows: Vec<Row> = arow.into_iter().collect();
    let mut seen = HashSet::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            ex.pair_trials += 1;
            let line = cover.secant(points[i], points[j], &mut o)?;
            if !seen.insert(line) {
                ex.duplicate_lines += 1;
                continue;
            }
            ex.unique_lines += 1;
            match cover.relation_on_line(line, points[i], points[j], &mut o)? {
                None => ex.nonsplit_residuals += 1,
                Some(cert) => {
                    ex.certified_relations += 1;
                    if !cover.verify_relation(&cert) {
                        ex.relation_failures += 1;
                    }
                    let row = cert_row(&cover, &fb, &cert, r, &mut o)?;
                    if row.is_empty() {
                        ex.zero_projected_rows += 1;
                    } else {
                        rows.push((row, 0));
                    }
                }
            }
        }
    }
    let (rank, dense) = dense_solve(&rows, n, r);
    ex.dense_rank = rank;
    let mut rng = StdRng::seed_from_u64(LA_SALT ^ p);
    for attempt in 1..=32u32 {
        ex.la_attempts = attempt;
        let la = la_solve(&rows, n, r, &mut rng, attempt > 1, &mut o);
        if la.x.is_some() || la.failure == Some("free_column") {
            ex.logs = la.x;
            break;
        }
    }
    ex.la_agrees_with_dense = ex.logs.is_some() && ex.logs == dense;
    if let Some(logs) = &ex.logs {
        for (c, &l) in logs.iter().enumerate() {
            if ec.mul_independent(gp, l) != fb.point(c as u32) {
                return Err(format!("factor-base log of column {c} fails its replay"));
            }
            ex.factor_logs_replayed += 1;
        }
    }
    Ok(ex)
}

impl Exhaustive {
    /// The individual log of `t` (shifts up to 512) and its certificate.
    pub fn recover(&self, t: (u64, u64)) -> Option<(u64, bool)> {
        let ec = EcU64::new(self.p, self.a, self.b)?;
        let logs = self.logs.as_ref()?;
        let g = Some(self.generator);
        let (x, _) = log_from_columns(&ec, &self.cols, logs, g, Some(t), self.r, MAX_SHIFTS)?;
        Some((x, ec.mul_independent(g, x) == Some(t)))
    }
}

// ── timing (A15, A18) ────────────────────────────────────────────────────

/// Nanoseconds per native operation of arms Q and D, best of N, each class
/// timed on the code path the arms run.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct QClassTimes {
    pub mul_p: f64,
    pub add_p: f64,
    pub inv_p: f64,
    pub mul_r: f64,
    pub add_r: f64,
    pub inv_r: f64,
    pub canon: f64,
    pub lookup: f64,
    pub insert: f64,
    pub draw: f64,
}

impl QClassTimes {
    pub fn get(&self, class: &str) -> f64 {
        match class {
            "mul_p" => self.mul_p,
            "add_p" => self.add_p,
            "inv_p" => self.inv_p,
            "mul_r" => self.mul_r,
            "add_r" => self.add_r,
            "inv_r" => self.inv_r,
            "canon" => self.canon,
            "lookup" => self.lookup,
            "insert" => self.insert,
            "draw" => self.draw,
            _ => f64::NAN,
        }
    }
}

/// Time every class of [`Ops`] at `p` and `r`, `batches × batch_len` best-of-N.
pub fn measure_quartic_class_times(
    p: u64,
    r: u64,
    batches: usize,
    batch_len: usize,
) -> QClassTimes {
    use std::hint::black_box;
    let xs: Vec<u64> = (1..=64u64).map(|i| (i * 7919 + 13) % (p - 1) + 1).collect();
    let rs: Vec<u64> = (1..=64u64).map(|i| (i * 7919 + 13) % (r - 1) + 1).collect();
    let f = Fq { p };
    let mut sink = Ops::default();
    let mut i = 0usize;
    let mul_p = best_of(batches, batch_len, || {
        black_box(f.mul(
            black_box(xs[i & 63]),
            black_box(xs[(i + 1) & 63]),
            &mut sink,
        ));
        i += 1;
    });
    let add_p = best_of(batches, batch_len, || {
        black_box(f.add(
            black_box(xs[i & 63]),
            black_box(xs[(i + 1) & 63]),
            &mut sink,
        ));
        i += 1;
    });
    let inv_p = best_of(batches, (batch_len / 20).max(1), || {
        black_box(f.inv(black_box(xs[i & 63]), &mut sink));
        i += 1;
    });
    let mul_r = best_of(batches, batch_len, || {
        black_box(mulmod(
            black_box(rs[i & 63]),
            black_box(rs[(i + 1) & 63]),
            r,
        ));
        i += 1;
    });
    let add_r = best_of(batches, batch_len, || {
        black_box(addm(black_box(rs[i & 63]), black_box(rs[(i + 1) & 63]), r));
        i += 1;
    });
    let inv_r = best_of(batches, (batch_len / 20).max(1), || {
        black_box(powu(black_box(rs[i & 63]), r - 2, r));
        i += 1;
    });
    // the arms' fold: a direct-address column read and a compare with p - y
    let n = (p / 4).max(1) as usize;
    let col_of_x: Vec<u32> = (0..p).map(|x| (x as usize % n) as u32).collect();
    let cols: Vec<(u64, u64)> = (0..n as u64).map(|c| (c, xs[c as usize & 63])).collect();
    let lookup = best_of(batches, batch_len, || {
        black_box(col_of_x[black_box(xs[i & 63]) as usize]);
        i += 1;
    });
    let canon = best_of(batches, batch_len, || {
        let y = black_box(xs[i & 63]);
        let rep = cols[i % n].1;
        black_box(y == rep || y == p - rep);
        i += 1;
    });
    let mut tab = vec![0u32; p as usize];
    let insert = best_of(batches, batch_len, || {
        tab[black_box(xs[i & 63]) as usize] = i as u32;
        i += 1;
    });
    black_box(&tab);
    let mut rng = StdRng::seed_from_u64(1);
    let draw = best_of(batches, batch_len, || {
        black_box(rng.gen_range(0..n));
    });
    black_box(&sink);
    QClassTimes {
        mul_p,
        add_p,
        inv_p,
        mul_r,
        add_r,
        inv_r,
        canon,
        lookup,
        insert,
        draw,
    }
}

/// Time of one plain `u64` affine addition (the `ic_boundary::invmod` path of
/// [`EcU64::add_raw`]), over a ring of 64 multiples of `g`.
pub fn time_add_u64(ec: &EcU64, g: EcPt, batches: usize, batch_len: usize) -> f64 {
    use std::hint::black_box;
    let pts: Vec<EcPt> = (1..=64u64).map(|m| ec.mul_independent(g, m * 7)).collect();
    let mut acc: EcPt = None;
    let mut i = 0usize;
    let t = best_of(batches, batch_len, || {
        acc = ec.add_raw(acc, pts[i % 64]);
        i += 1;
        black_box(&acc);
    });
    black_box(&acc);
    t
}

// ── exponents (A16) ──────────────────────────────────────────────────────

/// Fitted exponents of a cost made of phases: the total's OLS slope (with its
/// 95% interval) and each phase's own slope.
#[derive(Clone, Debug, Serialize)]
pub struct ExponentReport {
    pub total: Option<f64>,
    pub total_ci95: Option<[f64; 2]>,
    pub phases: Vec<Option<f64>>,
    /// The largest phase slope: what a total-cost slope can hide (lesson
    /// `L-eda09df2bc11`).
    pub max_phase: Option<f64>,
}

/// Slopes of `ln(cost)` on `ln p` for the total and for every phase.
pub fn exponent_report(ps: &[f64], phases: &[Vec<f64>]) -> ExponentReport {
    let total: Vec<f64> = (0..ps.len())
        .map(|i| phases.iter().map(|ph| ph[i]).sum())
        .collect();
    let fit = fit_loglog(ps, &total);
    let slopes: Vec<Option<f64>> = phases
        .iter()
        .map(|ph| fit_loglog(ps, ph).map(|f| f.0))
        .collect();
    let max_phase = slopes.iter().flatten().copied().reduce(f64::max);
    ExponentReport {
        total: fit.map(|f| f.0),
        total_ci95: fit.map(|f| f.3),
        phases: slopes,
        max_phase,
    }
}

#[cfg(test)]
mod tests {
    use super::super::ic_control::{peel, wiedemann, Nat};
    use super::*;
    use crate::cryptanalysis::ic_boundary::pivot_inversion_steps;

    const TSV: &str =
        include_str!("../../../research/chained_transfer_next_20261115/select_only.tsv");

    fn inst_229(k: u64) -> Inst {
        let t = A4_TABLE[0];
        make_instance(t.p, t.a, t.b, t.r, SEED0, k).0
    }

    #[test]
    fn reproduces_p53_demo() {
        let g = (0, 1);
        let ex = exhaustive(53, 2, 1, 59, g).unwrap();
        assert_eq!(ex.rational_points, 45);
        assert_eq!(ex.certified_relations, 79);
        assert_eq!(ex.relation_failures, 0);
        assert_eq!(ex.columns, 11);
        assert_eq!(ex.dense_rank, 11);
        assert!(ex.la_agrees_with_dense, "Wiedemann logs differ from dense");
        assert_eq!(ex.factor_logs_replayed, 11);
        let e = EcU64::new(53, 2, 1).unwrap();
        let t17 = e.mul_independent(Some(g), 17).unwrap();
        assert_eq!(ex.recover(t17), Some((17, true)));
        // the 59-scalar replay with two generators (as the old module's tests)
        let g17 = t17;
        for gen in [g, g17] {
            let exg = exhaustive(53, 2, 1, 59, gen).unwrap();
            assert_eq!(exg.dense_rank, 11);
            for s in 0..59u64 {
                let t = e.mul_independent(Some(gen), s);
                match t {
                    None => assert_eq!(s, 0),
                    Some(t) => assert_eq!(exg.recover(t), Some((s, true)), "scalar {s}"),
                }
            }
        }
        // the same counts as the old diagnostic
        let old = crate::cryptanalysis::bielliptic_quartic::Cover::new(53, 2, 1).unwrap();
        let oc =
            crate::cryptanalysis::bielliptic_quartic::collect_relations(&old, Default::default())
                .unwrap();
        assert_eq!(oc.stats.certified_relations, ex.certified_relations);
        assert_eq!(oc.stats.unique_lines, ex.unique_lines);
        assert_eq!(oc.stats.nonsplit_residuals, ex.nonsplit_residuals);
    }

    #[test]
    fn sections_match_old_module_at_p53() {
        use crate::cryptanalysis::bielliptic_quartic as old;
        let oc = old::Cover::new(53, 2, 1).unwrap();
        let mut o = Ops::default();
        let nc = QuarticCover::new(53, 2, 1, &mut o).unwrap();
        let pts = nc.rational_points();
        let conv = |q: QPt| match q {
            None => old::QuarticPoint::Infinity,
            Some((x, v)) => old::QuarticPoint::Affine { x, v },
        };
        assert_eq!(pts.len(), oc.rational_points().len());
        for i in 0..pts.len() {
            for j in i + 1..pts.len() {
                let a = oc.line_relation(conv(pts[i]), conv(pts[j])).unwrap();
                let b = nc.line_relation(pts[i], pts[j], &mut o).unwrap();
                match (a, b) {
                    (None, None) => {}
                    (Some(a), Some(b)) => {
                        assert_eq!(
                            (a.line.a, a.line.b, a.line.c),
                            (b.line.a, b.line.b, b.line.c)
                        );
                        let bt: Vec<_> = b.terms.iter().map(|&(q, m)| (conv(q), m)).collect();
                        assert_eq!(a.terms, bt);
                        assert!(nc.verify_relation(&b));
                    }
                    _ => panic!("split mismatch at pair {i},{j}"),
                }
            }
        }
    }

    #[test]
    fn tamper_fails() {
        // stage-1 prime 229, k = 0, seed j = 0 of Q
        let inst = inst_229(0);
        let (clean, t) = tamper_control(&inst, ic_seed(ArmKind::Q, 0, 0)).unwrap();
        assert!(clean.certified, "{:?}", clean.failure);
        assert!(t.detected, "tamper not detected: {t:?}");
        // a tampered certificate is rejected by verify_relation
        let mut o = Ops::default();
        let cover = QuarticCover::new(inst.p, inst.a, inst.b, &mut o).unwrap();
        let sys = clean.system.unwrap();
        assert!(!sys.certs.is_empty());
        for cert in sys.certs.iter().take(50) {
            assert!(certificate_tamper_rejected(&cover, cert));
        }
    }

    #[test]
    fn tamper_detected_at_every_stage1_prime_up_to_3209() {
        for (i, t) in A4_TABLE.iter().enumerate().take(5) {
            let inst = make_instance(t.p, t.a, t.b, t.r, SEED0, 0).0;
            let (clean, tc) = tamper_control(&inst, ic_seed(ArmKind::Q, 0, 0)).unwrap();
            assert!(clean.certified, "p={} {:?}", t.p, clean.failure);
            assert!(tc.detected, "i={i} p={}: {tc:?}", t.p);
        }
    }

    #[test]
    fn selection_binds_r_to_p() {
        for (i, &p) in STAGE1.iter().enumerate().take(5) {
            let s = select_prime_order(p, i, 0.85, 20).unwrap();
            let p = s.p as f64;
            assert!((s.r as f64 - (p + 1.0)).abs() <= 2.0 * p.sqrt());
            assert!(s.r >= (0.85 * p).ceil() as u64);
            assert!(is_prime_u64(s.r) && s.r != s.p);
            assert_eq!(s.p % 4, 1);
            // the cover's order is the independent Legendre count of ec.rs
            let e = EcU64::new(s.p, s.a, s.b).unwrap();
            assert_eq!(e.curve.order(), s.r);
        }
        assert!(select_prime_order(419, 1, 0.85, 20).is_err(), "p = 3 mod 4");
    }

    #[test]
    fn selection_rejects_j0_j1728_anomalous_low_embedding() {
        assert_eq!(
            check_candidate(229, 0, 5, 223, 253, 0.85),
            Err(Reject::AZero)
        );
        assert_eq!(
            check_candidate(229, 5, 0, 223, 253, 0.85),
            Err(Reject::BZero)
        );
        // the anomalous curve of the first selection pass (A3): i = 0, u = 3
        assert_eq!(candidate_ab(229, 0, 3), (149, 109));
        let e = EcU64::new(229, 149, 109).unwrap();
        assert_eq!(e.curve.order(), 229);
        assert_eq!(
            check_candidate(229, 149, 109, 229, 100, 0.85),
            Err(Reject::Anomalous)
        );
        // embedding degree <= 6: find a prime r >= 0.85 p dividing p^k - 1
        let mut found = false;
        'outer: for p in (13u64..4000).filter(|&p| p % 4 == 1 && is_prime_u64(p)) {
            let lo = (0.85 * p as f64).ceil() as u64;
            let hi = p + 1 + 2 * ((p as f64).sqrt() as u64 + 1);
            for r in lo..=hi {
                if r != p && is_prime_u64(r) && low_embedding(p, r, 6) {
                    assert_eq!(
                        check_candidate(p, 1, 1, r, 100, 0.85),
                        Err(Reject::LowEmbedding)
                    );
                    found = true;
                    break 'outer;
                }
            }
        }
        assert!(found);
        assert!(low_embedding(5, 31, 6)); // 5^3 = 125 = 1 (mod 31)
        assert!(!low_embedding(229, 223, 6));
        assert_eq!(
            check_candidate(229, 65, 224, 223, 7, 0.85),
            Err(Reject::FewPoints)
        );
        assert_eq!(
            check_candidate(229, 65, 224, 150, 253, 0.85),
            Err(Reject::RNotPrime)
        );
        assert_eq!(
            check_candidate(229, 65, 224, 191, 253, 0.85),
            Err(Reject::RSmall)
        );
        assert_eq!(check_candidate(229, 65, 224, 223, 253, 0.85), Ok(()));
    }

    #[test]
    fn selection_reproduces_a4_table() {
        // the constant is the frozen file, row for row
        let rows: Vec<Vec<&str>> = TSV
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 10);
        for (t, f) in A4_TABLE.iter().zip(&rows) {
            let num = |k: usize| f[k].parse::<u64>().unwrap();
            assert_eq!(t.i as u64, num(0));
            assert_eq!(t.p, num(2));
            assert_eq!(f[3], "[]");
            assert_eq!(
                (t.u, t.a, t.b, t.r, t.points_c, t.n),
                (num(4), num(5), num(6), num(7), num(9), num(10))
            );
        }
        // and the binary's selection reproduces it
        for (i, &p) in STAGE1.iter().chain(&STAGE2).enumerate() {
            let s = select_prime_order(p, i, 0.85, 20).unwrap();
            assert!(a4_matches(&s), "row {i}: {s:?}");
            assert!(s.passed_over.is_empty());
            assert_eq!(s.points_c, A4_TABLE[i].points_c);
        }
    }

    #[test]
    fn folding_is_sign_consistent() {
        let inst = inst_229(0);
        let mut o = Ops::default();
        let cover = QuarticCover::new(inst.p, inst.a, inst.b, &mut o).unwrap();
        let fb = FactorBase::build(&cover, &mut o);
        assert_eq!(fb.len() as u64, A4_TABLE[0].n);
        assert_eq!(fb.affine.len() as u64 + 1, A4_TABLE[0].points_c);
        let ec = EcU64::new(inst.p, inst.a, inst.b).unwrap();
        for c in 0..fb.len() as u32 {
            let q = fb.point(c);
            assert_eq!(fb.fold(q, &mut o), Some((c, false)));
            assert_eq!(fb.fold(ec.neg(q), &mut o), Some((c, true)));
        }
        // every affine point's image folds into its column
        for &(x, v) in &fb.affine {
            let img = cover.image(Some((x, v)), &mut o);
            let (c, neg) = fb.fold(img, &mut o).unwrap();
            let col = fb.point(c);
            assert_eq!(if neg { ec.neg(col) } else { col }, img);
        }
        // and every row of a run sums to O with its signs
        let mut prm = RunParams::new(ic_seed(ArmKind::Q, 0, 0));
        prm.keep_system = true;
        let out = solve_quartic_ic(&inst, prm);
        let sys = out.system.unwrap();
        for (row, _) in sys.rows.iter().skip(1) {
            assert!(row_sum_is_zero(&ec, &sys.cols, row, inst.r));
        }
        assert_eq!(out.verify.relation_failures, 0);
    }

    #[test]
    fn section_matches_u128_reference() {
        let p = 102_409u64;
        let (a, b) = (96_014u64, 95_982u64);
        let mut o = Ops::default();
        let cover = QuarticCover::new(p, a, b, &mut o).unwrap();
        let mut rng = StdRng::seed_from_u64(0x5EC7);
        let pp = p as u128;
        for _ in 0..10_000 {
            // a normalised line with b != 0: X + bV + cZ, or V + cZ
            let line = if rng.gen_bool(0.5) {
                Line {
                    a: 1,
                    b: rng.gen_range(1..p),
                    c: rng.gen_range(0..p),
                }
            } else {
                Line {
                    a: 0,
                    b: 1,
                    c: rng.gen_range(0..p),
                }
            };
            let sec = cover.section(line, &mut o);
            // reference: m = -a/b, t = -c/b, (m x + t)^4 - x^3 - a x - b, in u128
            let ib = powu(line.b, p - 2, p) as u128;
            let m = (pp - (line.a as u128 * ib % pp)) % pp;
            let t = (pp - (line.c as u128 * ib % pp)) % pp;
            let pw = |x: u128, e: u32| (0..e).fold(1u128, |acc, _| acc * x % pp);
            let mut refp = vec![
                (pw(t, 4) + pp - b as u128) % pp,
                (4 * m % pp * pw(t, 3) % pp + pp - a as u128) % pp,
                6 * pw(m, 2) % pp * pw(t, 2) % pp,
                (4 * pw(m, 3) % pp * t % pp + pp - 1) % pp,
                pw(m, 4),
            ];
            while refp.len() > 1 && refp.last() == Some(&0) {
                refp.pop();
            }
            let got: Vec<u128> = sec.poly.iter().map(|&x| x as u128).collect();
            assert_eq!(got, refp, "line {line:?}");
            assert_eq!(sec.infinity as usize, 5 - refp.len());
        }
    }

    fn random_system(r: u64, ncols: usize, nrows: usize, seed: u64) -> (Vec<Row>, Vec<u64>) {
        let mut rng = StdRng::seed_from_u64(seed);
        let x: Vec<u64> = (0..ncols).map(|_| rng.gen_range(1..r)).collect();
        let mut rows = vec![];
        for _ in 0..nrows {
            let mut t: Vec<(u32, u64)> = vec![];
            for _ in 0..4 {
                let c = rng.gen_range(0..ncols) as u32;
                let v = [1, r - 1, 2, r - 2][rng.gen_range(0..4)];
                if !t.iter().any(|e| e.0 == c) {
                    t.push((c, v));
                }
            }
            t.sort_unstable_by_key(|e| e.0);
            let rhs = t
                .iter()
                .fold(0, |acc, &(c, v)| addm(acc, mulmod(v, x[c as usize], r), r));
            rows.push((t, rhs));
        }
        (rows, x)
    }

    #[test]
    fn peel_without_target_column() {
        let r = 223;
        // a chain x0 = 5, x1 - x0 = 1, x2 - x1 = 1: all peeled, solved by back-substitution
        let rows: Vec<Row> = vec![
            (vec![(0, 1)], 5),
            (vec![(0, r - 1), (1, 1)], 1),
            (vec![(1, r - 1), (2, 2)], 3),
        ];
        let mut o = Ops::default();
        let pl = peel_no_target(&rows, 3, &mut o);
        assert_eq!(pl.core_cols, 0);
        assert!(pl.free.is_empty());
        assert_eq!(pl.order.len(), 3);
        let mut rng = StdRng::seed_from_u64(1);
        let la = la_solve(&rows, 3, r, &mut rng, false, &mut o);
        // x2 = (3 + 6) / 2
        assert_eq!(la.x, Some(vec![5, 6, mulmod(9, powu(2, r - 2, r), r)]));
        assert!(o.inv_r >= 1);
        // a column that loses its only row to another pivot is free
        let rows2: Vec<Row> = vec![(vec![(0, 1), (1, 1)], 4)];
        let pl2 = peel_no_target(&rows2, 2, &mut o);
        assert_eq!(pl2.free.len(), 1);
        let la2 = la_solve(&rows2, 2, r, &mut rng, false, &mut o);
        assert_eq!(la2.failure, Some("free_column"));
        // a random overdetermined system: peel + Wiedemann + back-substitution
        let r = 102_359;
        let (rows, mut x) = random_system(r, 300, 900, 7);
        let mut rows = rows;
        rows.push((vec![(0, 1)], x[0]));
        // column 300 appears in one row only: it is peeled and back-substituted
        x.push(4242);
        rows.push((vec![(0, 1), (300, 2)], addm(x[0], 2 * 4242, r)));
        let mut o = Ops::default();
        let la = la_solve(&rows, 301, r, &mut rng, false, &mut o);
        if la.failure != Some("free_column") {
            assert_eq!(la.x, Some(x.clone()), "{:?}", la.failure);
            assert!(la.peeled > 0 && la.core_cols > 0);
        }
        // the peel matches ic_control::peel with an unused target column
        let mut o1 = Ops::default();
        let mut nat = Nat::default();
        let mine = peel_no_target(&rows, 301, &mut o1);
        let (core, nc, tcol) = peel(&rows, 302, 301, &mut nat);
        assert_eq!(o1.add_r, nat.add);
        assert_eq!(mine.core, core);
        assert_eq!(mine.core_cols + 1, nc);
        assert_eq!(tcol as usize, mine.core_cols);
    }

    #[test]
    fn la_copy_matches_ic_control_counts() {
        for (r, ncols, nrows, seed, dense) in [
            (223u64, 60usize, 120usize, 3u64, false),
            (102_359, 400, 900, 4, false),
            (102_359, 40, 90, 5, true),
            (12_919, 10, 30, 6, false),
        ] {
            let (rows, x) = random_system(r, ncols, nrows, seed);
            let mut o = Ops::default();
            let mut nat = Nat::default();
            let mut r1 = StdRng::seed_from_u64(seed ^ LA_SALT);
            let mut r2 = StdRng::seed_from_u64(seed ^ LA_SALT);
            let a = wiedemann_counted(&rows, ncols, r, &mut r1, dense, &mut o);
            let b = wiedemann(&rows, ncols, r, &mut r2, dense, &mut nat);
            assert_eq!(a, b);
            if let (Some(sol), (rank, _)) = (&a, dense_solve(&rows, ncols, r)) {
                if rank == ncols {
                    assert_eq!(sol, &x);
                }
            }
            assert_eq!(o.mul_r + pivot_inversion_steps(r) * o.inv_r, nat.mul);
            assert_eq!(o.add_r, nat.add);
            assert!(o.inv_r >= 1);
        }
    }

    #[test]
    fn seeds_match_formulas() {
        assert_eq!(SEED0, 0x5EED0415);
        assert_eq!(seed_k(SEED0, 0), 0x5EED0415);
        assert_eq!(seed_k(SEED0, 1), 0x5EED0415 ^ 0x1000);
        assert_eq!(seed_k(SEED0, 2), 0x5EED0415 ^ 0x2000);
        let r = 223;
        let sk = seed_k(SEED0, 1);
        assert_eq!(
            x_k(sk, r).0,
            ((sk as u128 * 2_246_822_519 + 7) % 223) as u64
        );
        // x = 0 becomes 1: find a seed with (s · 2246822519 + 7) = 0 (mod 223)
        let s0 = (0..223u64)
            .find(|&s| (s as u128 * 2_246_822_519 + 7).is_multiple_of(223))
            .unwrap();
        assert_eq!(x_k(s0, 223), (1, true));
        assert_eq!(ic_seed(ArmKind::Q, 0, 0), 700);
        assert_eq!(ic_seed(ArmKind::Q, 2, 3), 723);
        assert_eq!(ic_seed(ArmKind::D, 1, 2), 512);
        assert_eq!(la_seed(700), 700 ^ 0x4C41);
        assert_eq!(rho_seed(0, 0), 1000);
        assert_eq!(rho_seed(2, 15), 1215);
        assert_eq!(
            tamper_row_index(229, 1000),
            (mix64(0x7A3B ^ 229) % 1000) as usize
        );
        let cols = bsgs_columns(229, 63);
        assert_eq!(cols.len(), 32);
        let mut rng = StdRng::seed_from_u64(0xB565 ^ 229);
        let first = rng.gen_range(0..63usize);
        assert_eq!(cols[0], first);
        assert_eq!(bsgs_columns(53, 11).len(), 11);
        // the instance: G_k from point_from_seed(seed_k), T = [x_k] G_k
        let t = A4_TABLE[0];
        let (inst, x, _) = make_instance(t.p, t.a, t.b, t.r, SEED0, 1);
        let e = EcU64::new(t.p, t.a, t.b).unwrap();
        let g = EcU64::from_pt(&e.curve.point_from_seed(seed_k(SEED0, 1)));
        assert_eq!(Some(inst.g), g);
        assert_eq!(x, x_k(seed_k(SEED0, 1), t.r).0);
        assert_eq!(e.mul_independent(g, x), Some(inst.t));
    }

    #[test]
    fn slope_rule_two_phase_toy() {
        let ps: Vec<f64> = STAGE1.iter().map(|&p| p as f64).collect();
        // a dominant p^0.5 phase and a steep p^2 phase with a 1% share at 229
        let a: Vec<f64> = ps.iter().map(|p| 100.0 * p.sqrt()).collect();
        let k = 0.01 / 0.99 * a[0] / (ps[0] * ps[0]);
        let s: Vec<f64> = ps.iter().map(|p| k * p * p).collect();
        let rep = exponent_report(&ps, &[a.clone(), s.clone()]);
        let total = rep.total.unwrap();
        assert!((rep.max_phase.unwrap() - 2.0).abs() < 1e-9);
        assert!((rep.phases[0].unwrap() - 0.5).abs() < 1e-9);
        assert!(
            total < 2.0 - 0.3,
            "total {total} should hide the steep phase"
        );
        assert!(total > 0.5);
        // a fixed cost pulls the total exponent down; the phase slope does not
        let fixed: Vec<f64> = ps.iter().map(|_| 1.0e6).collect();
        let lin: Vec<f64> = ps.clone();
        let rep2 = exponent_report(&ps, &[fixed, lin]);
        assert!(rep2.total.unwrap() < 0.5);
        assert!((rep2.max_phase.unwrap() - 1.0).abs() < 1e-9);
        assert!(rep2.phases[0].unwrap().abs() < 1e-9);
    }

    #[test]
    fn inv_r_count_check() {
        // every InvR of a run is recorded, and none is also charged as MulR steps
        let inst = inst_229(0);
        let out = solve_quartic_ic(&inst, RunParams::new(ic_seed(ArmKind::Q, 0, 0)));
        assert!(out.certified, "{:?}", out.failure);
        let la = out.ops[2];
        assert!(la.inv_r >= 1);
        // the collection, set-up and log phases do no F_r inversion
        assert_eq!(out.ops[0].inv_r + out.ops[1].inv_r + out.ops[3].inv_r, 0);
        // InvP: the collection inverts in F_p, and every group operation is tallied
        assert!(out.ops[1].inv_p > 0);
        let g = out.total_group();
        assert_eq!(g.field.invs, g.ops());
    }

    #[test]
    fn arms_q_and_d_certify_and_are_deterministic() {
        for k in 0..2 {
            let inst = inst_229(k);
            for j in 0..2 {
                let q = solve_quartic_ic(&inst, RunParams::new(ic_seed(ArmKind::Q, k, j)));
                assert!(q.certified, "Q k={k} j={j}: {:?}", q.failure);
                assert_eq!(q.verify.replay_mismatches, 0);
                assert_eq!(q.verify.relation_failures, 0);
                assert_eq!(q.verify.fibre_witness, Some(true));
                assert!(q.rows_used >= q.columns + ROW_MARGIN);
                let d = solve_direct_ic(&inst, RunParams::new(ic_seed(ArmKind::D, k, j)));
                assert!(d.certified, "D k={k} j={j}: {:?}", d.failure);
                assert_eq!(d.verify.relation_failures, 0);
                assert!(d.group[1].ops() > 0);
                // the determinism check of A9
                let q2 = solve_quartic_ic(&inst, RunParams::new(ic_seed(ArmKind::Q, k, j)));
                assert_eq!(q.ops, q2.ops);
                assert_eq!(q.group, q2.group);
                assert_eq!(q.log, q2.log);
            }
        }
        // dense cross-check and BSGS check at 229
        let inst = inst_229(0);
        let mut prm = RunParams::new(700);
        prm.keep_system = true;
        let q = solve_quartic_ic(&inst, prm);
        let sys = q.system.unwrap();
        let logs = sys.logs.unwrap();
        let (rank, dense) = dense_solve(&sys.rows, sys.cols.len(), inst.r);
        assert_eq!(rank, sys.cols.len());
        assert_eq!(dense.as_ref(), Some(&logs));
        assert_eq!(bsgs_check(&inst, &sys.cols, &logs), (32, 0));
    }

    #[test]
    fn budget_exhaustion_is_a_failed_run() {
        let inst = inst_229(0);
        let mut prm = RunParams::new(700);
        prm.budget_per_column = 1;
        let out = solve_quartic_ic(&inst, prm);
        assert!(!out.certified);
        assert!(out.exhausted);
        assert_eq!(out.trials, out.budget);
        assert_eq!(out.failure.as_deref(), Some("budget_exhausted"));
    }
}
