//! Push-forward of genus-2 index-calculus relations through the two
//! elliptic quotients of a bielliptic cover, and the elliptic-curve index
//! calculus those relations amount to.
//!
//! Setting: `C : y² = F(x²)`, `φ1 : C → E1`, `(x, y) ↦ (x², y)`, `φ2 : C → E2`.
//! The pulled-back instance is `G = φ1^*(G_E)`, `T = φ1^*(T_E)`. This module
//! answers, by measurement, how a relation found on `J(C)` looks on `E1` and
//! `E2`, and what the target-side factor base
//! `B = φ1(degree-1 places of C)` is.
//!
//! What the code shows (the protocol's predictions are in
//! `research/cover_pushforward_20261112/PROTOCOL.md`):
//! - `B` is the set of affine points of `E1` whose `X` is a square (a
//!   subset of about half of `E1`), each image having the two places `x, −x`
//!   over it, except `X = 0`.
//! - `Bielliptic::pull(Q)` is by construction the fibre `P + ι(P)` of `Q`, so
//!   a reduced class `[α]G + [β]T` is `φ1^*(Q)` for `Q = [α]G_E + [β]T_E` and
//!   splits over `F_p` exactly when `X(Q)` is a square. Its two places have
//!   the same image: the pushed "two-term decomposition" is `[2]Q = Q + Q`.
//! - `φ2_* φ1^* = 0`, so a pushed relation on `E2` has a zero right-hand
//!   side whatever `α, β` are.
//!
//! The relation collector [`collect_jc`] is a copy of the private
//! `measure::collect_relations` (same random stream, same acceptance rule,
//! same operation counts; `measure.rs` is not changed) that additionally keeps
//! each accepted class and the places it splits into.

use super::bielliptic::{Bielliptic, Quotient};
use super::ec::{Curve, Pt};
use super::field::{Fp, F2};
use super::measure::solve_linear_free;
use crate::cryptanalysis::genus2_fp::{Divisor, Genus2Curve};
use std::collections::{BTreeMap, HashMap, HashSet};

// ── helpers ─────────────────────────────────────────────────────────────

fn pt_base(x: u64, y: u64) -> Pt {
    Some((F2::base(x), F2::base(y)))
}

/// `(x, y)` of an `F_p`-rational affine point.
fn xy(p: &Pt) -> Option<(u64, u64)> {
    p.map(|(x, y)| {
        debug_assert!(x.in_base() && y.in_base());
        (x.a, y.a)
    })
}

/// `[n]P` counting every elliptic addition and doubling, with the convention
/// of `measure::scalar_counted` (the first addition into the identity is a
/// copy, not charged).
pub fn ec_scalar_counted(e: &Curve, pt: &Pt, mut n: u64) -> (Pt, u64) {
    let mut acc: Pt = None;
    let mut base = *pt;
    let mut ops = 0u64;
    while n > 0 {
        if n & 1 == 1 {
            if acc.is_some() {
                ops += 1;
            }
            acc = e.add(&acc, &base);
        }
        n >>= 1;
        if n > 0 {
            base = e.add(&base, &base);
            ops += 1;
        }
    }
    (acc, ops)
}

/// Independent check `[k]G = T` by a plain left-to-right double-and-add that
/// shares no code with the solvers.
pub fn certify_ec(e: &Curve, g: &Pt, t: &Pt, k: u64) -> bool {
    let mut acc: Pt = None;
    for bit in (0..64).rev() {
        acc = e.add(&acc, &acc);
        if (k >> bit) & 1 == 1 {
            acc = e.add(&acc, g);
        }
    }
    acc == *t
}

/// Whether the monic-model abscissa `X'` of `E1` has a square original
/// abscissa `X = X'/c3` (zero counts as a square).
pub fn x_is_square_e1(b: &Bielliptic, xm: u64) -> bool {
    xm == 0 || b.k.legendre(xm) == b.k.legendre(b.c[3])
}

/// Image of a quintic-model place `(t, Y')` on the monic quotient `q`.
/// `None` is the point at infinity (only for `E1` at `t = 0`).
pub fn place_image(b: &Bielliptic, q: Quotient, t: u64, yq: u64) -> Pt {
    let k = &b.k;
    match q {
        Quotient::E1 => {
            if t == 0 {
                return None;
            }
            let ti = k.inv(t);
            let x = k.add(b.x0, ti);
            let y = k.mul(yq, k.mul(ti, k.mul(ti, ti)));
            pt_base(k.mul(b.c[3], k.mul(x, x)), k.mul(b.c[3], y))
        }
        Quotient::E2 => {
            let w = k.add(k.mul(b.x0, t), 1);
            if w == 0 {
                return None;
            }
            let wi = k.inv(w);
            let x = k.mul(k.mul(t, t), k.mul(wi, wi));
            let y = k.mul(yq, k.mul(wi, k.mul(wi, wi)));
            pt_base(k.mul(b.c[0], x), k.mul(b.c[0], y))
        }
    }
}

/// The 2-torsion image of `∞_C = (x0, 0)` on the monic quotient.
pub fn t0(b: &Bielliptic, q: Quotient) -> Pt {
    let k = &b.k;
    let x02 = k.mul(b.x0, b.x0);
    let x = match q {
        Quotient::E1 => k.mul(b.c[3], x02),
        Quotient::E2 => k.mul(b.c[0], k.inv(x02)),
    };
    pt_base(x, 0)
}

// ── the target-side factor base B ───────────────────────────────────────

/// Structure of `B = φ1(affine degree-1 places of C)`.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BStructure {
    pub e1_order: u64,
    /// Rational affine points of `C` (original model `y² = F(x²)`).
    pub c_affine_points: u64,
    /// Rational places of `C` at `x = ∞` (they map to the identity of `E1`).
    pub c_inf_places: u64,
    /// `|B|`, distinct affine images in `E1`.
    pub b_size: u64,
    /// `|{P ∈ E1 affine : X square or 0}|` counted on `E1` alone.
    pub b_size_from_e1: u64,
    /// places-per-image histogram.
    pub fibre_hist: BTreeMap<u32, u64>,
    /// Images with `X = 0` (ramified: one place).
    pub ramified_images: u64,
    /// Images that are 2-torsion (`Y = 0`).
    pub two_torsion_images: u64,
}

/// Enumerate `B` from the places of `C` and, separately, from `E1`.
pub fn b_structure(b: &Bielliptic) -> BStructure {
    let k = &b.k;
    let p = k.p;
    let c = b.c;
    let mut fibres: HashMap<(u64, u64), u32> = HashMap::new();
    let mut pts = 0u64;
    for x in 0..p {
        let x2 = k.mul(x, x);
        let mut v = c[3];
        for i in (0..3).rev() {
            v = k.add(k.mul(v, x2), c[i]);
        }
        let Some(y) = k.sqrt(v) else { continue };
        let ys: Vec<u64> = if y == 0 { vec![0] } else { vec![y, k.neg(y)] };
        for yy in ys {
            pts += 1;
            let img = (k.mul(c[3], x2), k.mul(c[3], yy));
            *fibres.entry(img).or_default() += 1;
        }
    }
    let mut hist: BTreeMap<u32, u64> = BTreeMap::new();
    for &n in fibres.values() {
        *hist.entry(n).or_default() += 1;
    }
    let mut from_e1 = 0u64;
    for pt in b.e1.rational_points().iter().flatten() {
        if x_is_square_e1(b, pt.0.a) {
            from_e1 += 1;
        }
    }
    BStructure {
        e1_order: b.e1.order(),
        c_affine_points: pts,
        c_inf_places: if k.legendre(c[3]) == 1 { 2 } else { 0 },
        b_size: fibres.len() as u64,
        b_size_from_e1: from_e1,
        fibre_hist: hist,
        ramified_images: fibres.keys().filter(|(x, _)| *x == 0).count() as u64,
        two_torsion_images: fibres.keys().filter(|(_, y)| *y == 0).count() as u64,
    }
}

// ── relation collection on J(C): a copy of measure::collect_relations ───

/// One accepted weight-2 relation `[α]G + [β]T = Σ e_i (P_i − ∞)`.
#[derive(Clone, Debug)]
pub struct JcRel {
    /// Factor-base exponents (`±1` per place; the rows the library solves).
    pub row: Vec<i64>,
    pub alpha: u64,
    pub beta: u64,
    /// Trial index (1-based) at which the relation was accepted.
    pub trial: u64,
    /// Cantor operations charged up to and including this trial.
    pub cum_ops: u64,
    /// The reduced class `R`.
    pub class: Divisor,
    /// `(t, y, e)` per place: abscissa on the quintic model, ordinate of the
    /// place itself, and the sign `e` against the canonical `y0 = min(y, p−y)`.
    pub places: [(u64, u64, i64); 2],
}

/// Everything the J(C) relation phase produced.
#[derive(Clone, Debug)]
pub struct JcRun {
    pub rels: Vec<JcRel>,
    pub nfb: usize,
    pub ops: u64,
    pub trials: u64,
}

/// `[n]d` counting every Cantor addition and doubling (same convention as
/// `measure::scalar_counted`, copied because that function is private).
fn jac_scalar_counted(c: &Genus2Curve, d: &Divisor, mut n: u64) -> (Divisor, u64) {
    let mut acc = Divisor::IDENTITY;
    let mut base = *d;
    let mut ops = 0u64;
    while n > 0 {
        if n & 1 == 1 {
            if !acc.is_identity() {
                ops += 1;
            }
            acc = c.add(&acc, &base);
        }
        n >>= 1;
        if n > 0 {
            base = c.double(&base);
            ops += 1;
        }
    }
    (acc, ops)
}

/// Split a degree-2 reduced class into its two `F_p` places (copy of the
/// degree-2 branch of `measure::smooth_decompose`); `None` if the roots are
/// not in `F_p` or coincide.
fn split2(k: &Fp, d: &Divisor) -> Option<[(u64, u64, i64); 2]> {
    if d.degree() != 2 {
        return None;
    }
    let p = k.p;
    let u = d.u_coeffs();
    let v = d.v_coeffs();
    let (c0, c1) = (u[0], u[1]);
    let disc = k.sub(k.mul(c1, c1), k.mul(4, c0));
    let s = k.sqrt(disc)?;
    let inv2 = k.inv(2);
    let r1 = k.mul(k.add(k.neg(c1), s), inv2);
    let r2 = k.mul(k.sub(k.neg(c1), s), inv2);
    if r1 == r2 {
        return None;
    }
    let mut out = [(0u64, 0u64, 0i64); 2];
    for (slot, x) in [r1, r2].into_iter().enumerate() {
        let mut y = 0u64;
        for &cf in v.iter().rev() {
            y = k.add(k.mul(y, x), cf);
        }
        let y0 = y.min((p - y) % p);
        out[slot] = (x, y, if y == y0 { 1 } else { -1 });
    }
    Some(out)
}

/// Copy of `measure::collect_relations` (private there), with the full
/// degree-1 factor base of `fb_size` abscissae, that also keeps each accepted
/// class and its places. Same random stream, acceptance rule and counts.
pub fn collect_jc(
    c: &Genus2Curve,
    gen: &Divisor,
    target: &Divisor,
    r: u64,
    fb_size: usize,
    max_trials: u64,
    seed: u64,
) -> JcRun {
    let p = c.p();
    let k = Fp::new(p).expect("prime field");
    let mut nfb = 0usize;
    let mut xindex: HashMap<u64, usize> = HashMap::new();
    for x in 0..p {
        if nfb >= fb_size {
            break;
        }
        if k.sqrt(c.eval_f(x)).is_some() {
            xindex.insert(x, nfb);
            nfb += 1;
        }
    }
    let mut ops = 0u64;
    let mut rels: Vec<JcRel> = vec![];
    let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut trials = 0u64;
    let want = nfb + 8;
    while rels.len() < want && trials < max_trials {
        trials += 1;
        let alpha = next() % r;
        let beta = next() % r;
        let (ra, oa) = jac_scalar_counted(c, gen, alpha);
        let (rb, ob) = jac_scalar_counted(c, target, beta);
        ops += oa + ob + 1;
        let rr = c.add(&ra, &rb);
        if rr.degree() == 2 {
            if let Some(pl) = split2(&k, &rr) {
                if let (Some(&i0), Some(&i1)) = (xindex.get(&pl[0].0), xindex.get(&pl[1].0)) {
                    let mut row = vec![0i64; nfb];
                    row[i0] += pl[0].2;
                    row[i1] += pl[1].2;
                    rels.push(JcRel {
                        row,
                        alpha,
                        beta,
                        trial: trials,
                        cum_ops: ops,
                        class: rr,
                        places: pl,
                    });
                }
            }
        }
    }
    JcRun {
        rels,
        nfb,
        ops,
        trials,
    }
}

/// The library's elimination on a set of relation rows.
pub fn solve_rows(rels: &[JcRel], nfb: usize, r: u64) -> (Option<u64>, u64) {
    let rows: Vec<(Vec<i64>, u64, u64)> = rels
        .iter()
        .map(|x| (x.row.clone(), x.alpha, x.beta))
        .collect();
    solve_linear_free(&rows, nfb, r)
}

// ── pushing relations forward and checking them ─────────────────────────

/// Outcome of pushing one relation through one quotient.
#[derive(Clone, Copy, Debug, Default)]
pub struct PushedRel {
    /// `Σ e_i (φ(P0_i) + T0)` by explicit place arithmetic equals
    /// `[α] φ_*(G) + [β] φ_*(T)` (with `φ_*` from `Bielliptic::push`).
    pub holds: bool,
    /// For `E1` only: also equals `[2α] G_E + [2β] T_E`.
    pub holds_vs_source: bool,
    /// `φ_*(G)` or `φ_*(T)` is not the identity (the relation could carry the
    /// target to this quotient).
    pub carries_target: bool,
    /// `φ_*(R)` by `Bielliptic::push` equals the place sum.
    pub push_matches_places: bool,
    /// The two places have equal images up to the sign already in `e` (fibre
    /// type: `φ(P1) = φ(P2)`).
    pub fibre_type: bool,
}

/// Push one relation forward through `q` and check it by independent point
/// arithmetic. `ge, te` are the source points on `E1` (`G_E, T_E`); `dg, dt`
/// the pulled-back divisors.
pub fn push_check(
    b: &Bielliptic,
    q: Quotient,
    rel: &JcRel,
    (ge, te): (&Pt, &Pt),
    (dg, dt): (&Divisor, &Divisor),
) -> PushedRel {
    let e = b.curve(q);
    let t0p = t0(b, q);
    // Place sum with the canonical y0 and the sign e_i.
    let mut lhs: Pt = None;
    let mut imgs: Vec<Pt> = vec![];
    for &(t, y, s) in &rel.places {
        let y0 = y.min((b.k.p - y) % b.k.p);
        let img = place_image(b, q, t, y0);
        let signed = if s >= 0 { img } else { e.neg(&img) };
        // + T0 per place: e_i (φ(P0_i) − T0) = e_i φ(P0_i) + T0 as −T0 = T0.
        lhs = e.add(&lhs, &e.add(&signed, &t0p));
        imgs.push(place_image(b, q, t, y));
    }
    let pg = b.push(q, dg);
    let pt = b.push(q, dt);
    let rhs = e.add(
        &e.mul(&pg, rel.alpha as u128),
        &e.mul(&pt, rel.beta as u128),
    );
    let holds = lhs == rhs;
    let holds_vs_source = match q {
        Quotient::E1 => {
            let src = e.add(
                &e.mul(ge, 2 * rel.alpha as u128),
                &e.mul(te, 2 * rel.beta as u128),
            );
            lhs == src
        }
        Quotient::E2 => true,
    };
    PushedRel {
        holds,
        holds_vs_source,
        carries_target: pg.is_some() || pt.is_some(),
        push_matches_places: lhs == b.push(q, &rel.class),
        fibre_type: imgs[0] == imgs[1],
    }
}

/// Aggregate of [`push_check`] over a relation set.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct PushSummary {
    pub relations: u64,
    pub e1_valid: u64,
    pub e1_invalid: u64,
    pub e1_vs_source_invalid: u64,
    pub e1_push_mismatch: u64,
    pub e2_valid: u64,
    pub e2_invalid: u64,
    pub e2_push_mismatch: u64,
    /// Rows whose `φ2_*(G)` or `φ2_*(T)` is not the identity.
    pub e2_rows_carrying_target: u64,
    /// Rows whose `φ1_*(G)`, `φ1_*(T)` are the identity (would carry nothing).
    pub e1_rows_carrying_target: u64,
    /// Rows whose two places have the same `φ1` image.
    pub fibre_type_e1: u64,
    /// Rows whose two places have the same `φ2` image.
    pub fibre_type_e2: u64,
}

pub fn push_summary(
    b: &Bielliptic,
    rels: &[JcRel],
    src: (&Pt, &Pt),
    div: (&Divisor, &Divisor),
) -> PushSummary {
    let mut s = PushSummary {
        relations: rels.len() as u64,
        ..Default::default()
    };
    for rel in rels {
        let a = push_check(b, Quotient::E1, rel, src, div);
        let c = push_check(b, Quotient::E2, rel, src, div);
        if a.holds && a.holds_vs_source {
            s.e1_valid += 1;
        } else {
            s.e1_invalid += 1;
        }
        if !a.holds_vs_source {
            s.e1_vs_source_invalid += 1;
        }
        if !a.push_matches_places {
            s.e1_push_mismatch += 1;
        }
        if c.holds {
            s.e2_valid += 1;
        } else {
            s.e2_invalid += 1;
        }
        if !c.push_matches_places {
            s.e2_push_mismatch += 1;
        }
        if c.carries_target {
            s.e2_rows_carrying_target += 1;
        }
        if a.carries_target {
            s.e1_rows_carrying_target += 1;
        }
        if a.fibre_type {
            s.fibre_type_e1 += 1;
        }
        if c.fibre_type {
            s.fibre_type_e2 += 1;
        }
    }
    s
}

// ── E1 arms ─────────────────────────────────────────────────────────────

/// Result of an index-calculus arm run on `E1`.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct E1Run {
    pub recovered: Option<u64>,
    pub certified: bool,
    /// Group operations in relation collection (scalar multiplications,
    /// combine, and for the two-term arm the search subtractions).
    pub group_ops: u64,
    /// Of which: building the target-side factor base (two-term arm only).
    pub base_build_ops: u64,
    /// Of which: search subtractions (two-term arm only).
    pub search_ops: u64,
    pub trials: u64,
    pub relations: u64,
    pub cols: usize,
    pub la_mulmods: u64,
    /// Hash lookups (unpriced native work).
    pub lookups: u64,
}

/// First collision of a column in a 1-term relation stream.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Collision {
    /// Number of accepted relations when the first usable collision appeared.
    pub relations: u64,
    pub trials: u64,
    /// Group operations charged up to that trial.
    pub ops: u64,
    pub log: Option<u64>,
}

/// `x = −(e0 α − e α0) / (e0 β − e β0)` from two relations on one column,
/// or `None` if the denominator vanishes.
fn collision_log(r: u64, a0: u64, b0: u64, e0: i64, a: u64, b: u64, e: i64) -> Option<u64> {
    let rr = r as i128;
    let m = |v: i128| ((v % rr) + rr) % rr;
    let num = m(-(e0 as i128 * a as i128 - e as i128 * a0 as i128));
    let den = m(e0 as i128 * b as i128 - e as i128 * b0 as i128);
    if den == 0 {
        return None;
    }
    // Fermat inverse mod the prime r.
    let (mut base, mut ex, mut inv) = (den as u128, r as u128 - 2, 1u128);
    while ex > 0 {
        if ex & 1 == 1 {
            inv = inv * base % r as u128;
        }
        base = base * base % r as u128;
        ex >>= 1;
    }
    Some((num as u128 * inv % r as u128) as u64)
}

/// First collision among the pushed relations of a J(C) run: relations are
/// keyed by the (sign-folded) `φ1` image of their first place.
pub fn jc_first_collision(b: &Bielliptic, rels: &[JcRel], r: u64) -> Option<Collision> {
    let mut seen: HashMap<u64, (u64, u64, i64)> = HashMap::new();
    for (i, rel) in rels.iter().enumerate() {
        let (t, y, s) = rel.places[0];
        let y0 = y.min((b.k.p - y) % b.k.p);
        let Some((xm, ym)) = xy(&place_image(b, Quotient::E1, t, y0)) else {
            continue;
        };
        // sign of the image against the canonical Y' (min), times the place sign
        let canon = ym <= (b.k.p - ym) % b.k.p;
        let e = if canon { s } else { -s };
        if let Some(&(a0, b0, e0)) = seen.get(&xm) {
            if let Some(x) = collision_log(r, a0, b0, e0, rel.alpha, rel.beta, e) {
                return Some(Collision {
                    relations: i as u64 + 1,
                    trials: rel.trial,
                    ops: rel.cum_ops,
                    log: Some(x),
                });
            }
        } else {
            seen.insert(xm, (rel.alpha, rel.beta, e));
        }
    }
    None
}

/// Arm A4: native 1-term index calculus on `E1`. Same random stream as the
/// J(C) collector; a trial `Q = [α]G_E + [β]T_E` is accepted when `X(Q)` is a
/// nonzero square, with one column per `±Q`. Stops after `want` accepted
/// relations, the same stop rule as the J(C) arm. Also reports the first
/// usable collision.
#[allow(clippy::too_many_arguments)]
pub fn e1_fibre_ic(
    b: &Bielliptic,
    ge: &Pt,
    te: &Pt,
    r: u64,
    want: usize,
    max_trials: u64,
    seed: u64,
) -> (E1Run, Vec<u64>, Option<Collision>) {
    let e = b.e1;
    let p = e.k.p;
    let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut ops = 0u64;
    let mut trials = 0u64;
    let mut cols: HashMap<u64, usize> = HashMap::new();
    let mut raw: Vec<(usize, i64, u64, u64)> = vec![];
    let mut accepted_trials: Vec<u64> = vec![];
    let mut first: HashMap<usize, (u64, u64, i64)> = HashMap::new();
    let mut coll: Option<Collision> = None;
    while raw.len() < want && trials < max_trials {
        trials += 1;
        let alpha = next() % r;
        let beta = next() % r;
        let (ra, oa) = ec_scalar_counted(&e, ge, alpha);
        let (rb, ob) = ec_scalar_counted(&e, te, beta);
        ops += oa + ob + 1;
        let q = e.add(&ra, &rb);
        let Some((xm, ym)) = xy(&q) else { continue };
        if xm == 0 || e.k.legendre(xm) != e.k.legendre(b.c[3]) {
            continue; // X not a nonzero square: not in B (or ramified)
        }
        let n = cols.len();
        let col = *cols.entry(xm).or_insert(n);
        let sgn = if ym <= (p - ym) % p { 1 } else { -1 };
        raw.push((col, sgn, alpha, beta));
        accepted_trials.push(trials);
        if coll.is_none() {
            if let Some(&(a0, b0, e0)) = first.get(&col) {
                if let Some(x) = collision_log(r, a0, b0, e0, alpha, beta, sgn) {
                    coll = Some(Collision {
                        relations: raw.len() as u64,
                        trials,
                        ops,
                        log: Some(x),
                    });
                }
            } else {
                first.insert(col, (alpha, beta, sgn));
            }
        }
    }
    let ncols = cols.len();
    let rows: Vec<(Vec<i64>, u64, u64)> = raw
        .iter()
        .map(|&(c, s, a, bb)| {
            let mut v = vec![0i64; ncols];
            v[c] = s;
            (v, a, bb)
        })
        .collect();
    let (rec, la) = solve_linear_free(&rows, ncols, r);
    let run = E1Run {
        recovered: rec,
        certified: rec.is_some_and(|k| certify_ec(&e, ge, te, k)),
        group_ops: ops,
        trials,
        relations: raw.len() as u64,
        cols: ncols,
        la_mulmods: la,
        ..Default::default()
    };
    (run, accepted_trials, coll)
}

/// Control (protocol amendment 1): generic birthday on `E1`, no cover and no
/// factor base. Same random stream; every nonzero `Q = [α]G_E + [β]T_E` is
/// kept, keyed by `±Q`; stops at the first usable collision. Shows what the
/// "square `X`" acceptance of the cover-derived arms costs.
pub fn e1_birthday_first_collision(
    b: &Bielliptic,
    ge: &Pt,
    te: &Pt,
    r: u64,
    max_trials: u64,
    seed: u64,
) -> Option<Collision> {
    let e = b.e1;
    let p = e.k.p;
    let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut ops = 0u64;
    let mut seen: HashMap<u64, (u64, u64, i64)> = HashMap::new();
    let mut kept = 0u64;
    for trials in 1..=max_trials {
        let alpha = next() % r;
        let beta = next() % r;
        let (ra, oa) = ec_scalar_counted(&e, ge, alpha);
        let (rb, ob) = ec_scalar_counted(&e, te, beta);
        ops += oa + ob + 1;
        let Some((xm, ym)) = xy(&e.add(&ra, &rb)) else {
            continue;
        };
        kept += 1;
        let sgn = if ym <= (p - ym) % p { 1 } else { -1 };
        if let Some(&(a0, b0, e0)) = seen.get(&xm) {
            if let Some(x) = collision_log(r, a0, b0, e0, alpha, beta, sgn) {
                return Some(Collision {
                    relations: kept,
                    trials,
                    ops,
                    log: Some(x),
                });
            }
        } else {
            seen.insert(xm, (alpha, beta, sgn));
        }
    }
    None
}

/// The points `[1]G, …, [r−1]G` of the order-`r` subgroup, built by `r − 2`
/// counted additions (the first is `G` itself).
pub fn subgroup_points(e: &Curve, g: &Pt, r: u64) -> (Vec<Pt>, u64) {
    let mut v = Vec::with_capacity(r as usize);
    let mut cur = *g;
    let mut ops = 0u64;
    for i in 1..r {
        v.push(cur);
        if i + 1 < r {
            cur = e.add(&cur, g);
            ops += 1;
        }
    }
    (v, ops)
}

/// `|B ∩ ⟨G_E⟩|` (points of the subgroup with `X` square or zero).
pub fn b_in_subgroup(b: &Bielliptic, pts: &[Pt]) -> u64 {
    pts.iter()
        .filter(|q| xy(q).is_some_and(|(x, _)| x_is_square_e1(b, x)))
        .count() as u64
}

/// Arm A5: generic two-term index calculus on `E1` over `B_r = B ∩ ⟨G_E⟩`.
/// The base is built by counted additions; a trial `R = [α]G_E + [β]T_E` is
/// decomposed as `R = Q1 + Q2` by scanning a seeded permutation of `B_r`
/// from a random offset, one counted subtraction per candidate.
pub fn e1_two_term_ic(
    b: &Bielliptic,
    ge: &Pt,
    te: &Pt,
    r: u64,
    max_trials: u64,
    seed: u64,
) -> E1Run {
    let e = b.e1;
    let p = e.k.p;
    let (pts, build_ops) = subgroup_points(&e, ge, r);
    // B_r as (x, y) -> (column, sign); one column per +-pair.
    let mut cols: HashMap<u64, usize> = HashMap::new();
    let mut bl: Vec<Pt> = vec![];
    for q in &pts {
        let (x, _) = xy(q).expect("subgroup point is affine");
        if x_is_square_e1(b, x) {
            bl.push(*q);
            let n = cols.len();
            cols.entry(x).or_insert(n);
        }
    }
    let ncols = cols.len();
    // seeded Fisher-Yates
    let mut rng = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    for i in (1..bl.len()).rev() {
        let j = (next() % (i as u64 + 1)) as usize;
        bl.swap(i, j);
    }
    let sign_of = |q: &Pt| -> (usize, i64) {
        let (x, y) = xy(q).unwrap();
        (cols[&x], if y <= (p - y) % p { 1 } else { -1 })
    };
    let inb: HashSet<(u64, u64)> = bl.iter().map(|q| xy(q).unwrap()).collect();
    let want = ncols + 8;
    let mut rows: Vec<(Vec<i64>, u64, u64)> = vec![];
    let mut ops = build_ops;
    let mut search_ops = 0u64;
    let mut lookups = 0u64;
    let mut trials = 0u64;
    while rows.len() < want && trials < max_trials && !bl.is_empty() {
        trials += 1;
        let alpha = next() % r;
        let beta = next() % r;
        let (ra, oa) = ec_scalar_counted(&e, ge, alpha);
        let (rb, ob) = ec_scalar_counted(&e, te, beta);
        ops += oa + ob + 1;
        let rr = e.add(&ra, &rb);
        if rr.is_none() {
            continue;
        }
        let start = (next() % bl.len() as u64) as usize;
        for i in 0..bl.len() {
            let q1 = bl[(start + i) % bl.len()];
            let q2 = e.add(&rr, &e.neg(&q1));
            ops += 1;
            search_ops += 1;
            lookups += 1;
            if let Some(k2) = xy(&q2) {
                if inb.contains(&k2) {
                    let mut v = vec![0i64; ncols];
                    let (c1, s1) = sign_of(&q1);
                    let (c2, s2) = sign_of(&q2);
                    v[c1] += s1;
                    v[c2] += s2;
                    rows.push((v, alpha, beta));
                    break;
                }
            }
        }
    }
    let (rec, la) = solve_linear_free(&rows, ncols, r);
    E1Run {
        recovered: rec,
        certified: rec.is_some_and(|k| certify_ec(&e, ge, te, k)),
        group_ops: ops,
        base_build_ops: build_ops,
        search_ops,
        trials,
        relations: rows.len() as u64,
        cols: ncols,
        la_mulmods: la,
        lookups,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::chained_transfer::measure::genus2_index_calculus;

    fn largest_prime_once(mut n: u64) -> u64 {
        let mut best = 1;
        let mut d = 2;
        while d * d <= n {
            if n.is_multiple_of(d) {
                let mut ex = 0;
                while n.is_multiple_of(d) {
                    n /= d;
                    ex += 1;
                }
                if ex == 1 {
                    best = best.max(d);
                }
            }
            d += 1;
        }
        if n > 1 {
            best = best.max(n);
        }
        best
    }

    /// A small bielliptic curve with a usable prime `r` on `E1`, and an
    /// instance `(G_E, T_E)` of order `r`.
    fn fixture(p: u64) -> (Bielliptic, Pt, Pt, u64, u64) {
        for s in 0..4000u64 {
            let c = [
                1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
                (s.wrapping_mul(40_503) >> 5) % p,
                (s.wrapping_mul(2_246_822_519) >> 7) % p,
                1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
            ];
            let Ok(b) = Bielliptic::new(p, c) else {
                continue;
            };
            let n1 = b.e1.order();
            let n2 = b.e2.order();
            let r = largest_prime_once(n1);
            if r < 50 || n2.is_multiple_of(r) {
                continue;
            }
            let cof = n1 / r;
            for sd in 1..200u64 {
                let g = b.e1.mul(
                    &b.e1.point_from_seed(sd.wrapping_mul(0x9E37_79B9)),
                    cof as u128,
                );
                if g.is_some() && b.e1.mul(&g, r as u128).is_none() {
                    let x = 11 % r;
                    return (b.clone(), g, b.e1.mul(&g, x as u128), r, x);
                }
            }
        }
        panic!("no fixture");
    }

    #[test]
    fn pullback_pushes_to_double_and_cross_composite_vanishes() {
        let (b, g, _t, r, _x) = fixture(211);
        let mut checked = 0;
        for m in 1..60u128 {
            let q = b.e1.mul(&g, m);
            if !b.fiber_generic(Quotient::E1, &q) {
                continue;
            }
            let d = b.pull(Quotient::E1, &q);
            assert_eq!(b.push(Quotient::E1, &d), b.e1.mul(&q, 2), "φ1_* φ1^* = [2]");
            assert_eq!(b.push(Quotient::E2, &d), None, "φ2_* φ1^* = 0");
            checked += 1;
        }
        assert!(checked >= 20 && r >= 50);
    }

    #[test]
    fn j_c_ic_copy_matches_library_and_pushed_relations_hold() {
        let (b, ge, te, r, x) = fixture(211);
        let dg = b.pull(Quotient::E1, &ge);
        let dt = b.pull(Quotient::E1, &te);
        let seed = 0xC0FFEE + 5;
        let run = collect_jc(&b.jac, &dg, &dt, r, 211, 50_000_000, seed);
        let lib = genus2_index_calculus(&b.jac, &dg, &dt, r, 211, 50_000_000, seed);
        assert_eq!(run.ops, lib.group_ops);
        assert_eq!(run.trials, lib.trials);
        assert_eq!(run.rels.len() as u64, lib.relations);
        let (rec, la) = solve_rows(&run.rels, run.nfb, r);
        assert_eq!(rec, lib.recovered);
        assert_eq!(la, lib.la_field_ops);
        assert_eq!(rec, Some(x));
        let s = push_summary(&b, &run.rels, (&ge, &te), (&dg, &dt));
        assert_eq!(s.e1_invalid, 0);
        assert_eq!(s.e2_invalid, 0);
        assert_eq!(s.e1_push_mismatch + s.e2_push_mismatch, 0);
        assert_eq!(s.e2_rows_carrying_target, 0);
        assert_eq!(
            s.fibre_type_e1, s.relations,
            "every J(C) relation is a fibre"
        );
        assert!(s.relations > 50);
    }

    #[test]
    fn b_is_the_square_x_half_and_fibres_have_two_places() {
        let (b, _g, _t, _r, _x) = fixture(419);
        let s = b_structure(&b);
        assert_eq!(s.b_size, s.b_size_from_e1);
        let ratio = s.b_size as f64 / s.e1_order as f64;
        assert!((0.35..0.65).contains(&ratio), "ratio {ratio}");
        for n in s.fibre_hist.keys() {
            assert!(*n == 1 || *n == 2);
        }
    }

    #[test]
    fn e1_arms_recover_and_certify() {
        let (b, ge, te, r, x) = fixture(211);
        let dg = b.pull(Quotient::E1, &ge);
        let dt = b.pull(Quotient::E1, &te);
        let jc = collect_jc(&b.jac, &dg, &dt, r, 211, 50_000_000, 9);
        let (a4, acc, coll) = e1_fibre_ic(&b, &ge, &te, r, jc.rels.len(), 50_000_000, 9);
        // same stream, same acceptance: identical accepted trials and ops
        let jt: Vec<u64> = jc.rels.iter().map(|x| x.trial).collect();
        assert_eq!(acc, jt);
        assert_eq!(a4.group_ops, jc.ops);
        assert_eq!(a4.recovered, Some(x));
        assert!(a4.certified);
        let c = coll.expect("a collision exists");
        assert_eq!(c.log, Some(x));
        let bd = e1_birthday_first_collision(&b, &ge, &te, r, 50_000_000, 9).unwrap();
        assert_eq!(bd.log, Some(x));
        let a5 = e1_two_term_ic(&b, &ge, &te, r, 50_000_000, 9);
        assert_eq!(a5.recovered, Some(x));
        assert!(a5.certified);
    }
}
