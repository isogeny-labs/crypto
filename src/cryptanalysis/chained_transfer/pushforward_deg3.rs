//! Degree-3 cover push-forward: are the relations an index calculus finds on
//! the transferred subgroup of a genus-4 Jacobian all full fibres of the cover?
//!
//! Setting (the `tower_ladder` rung 4): `E : Y² = c(s)`, `h(x) = x³ + h1 x + h0`,
//! `C : y² = c(h(x))` of genus 4, `c(x, y) = (h(x), y)` of degree 3.  The
//! pull-back of an affine `Q = (s, Y)` is the Mumford pair `(h(x) − s, Y)`, of
//! degree 3 ≤ g = 4, so the reduced class of `[α]D + [β]T` (with `D = c^*(G_E)`,
//! `T = c^*(T_E)`) is `c^*([α]G_E + [β]T_E)`.  It splits over `F_p` exactly when
//! the cubic `h(x) = s` has all its roots in `F_p`, and then its places are
//! the whole fibre of `Q`: the pushed relation is the degenerate `[3]Q`.
//!
//! This module measures that (protocol:
//! `research/cover_pushforward_deg3_20261112/PROTOCOL.md`): the J(C) relation
//! collector, the classification of every relation into `c`-fibres, the
//! push-forward `c_*` checked by point arithmetic on `E`, and the matching
//! E-side collision searches on the same random stream.  `HCurve` limits this
//! to `p < 256` and genus ≤ 4.  All figures are `[pcc:unverified]`.

use super::ec::{Curve, Pt};
use super::field::{Fp, F2};
use super::measure::{solve_linear_free, EcGroup};
use super::pushforward::{certify_ec, ec_scalar_counted};
use super::tower::{build, TowerInstance};
use crate::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
use crate::cryptanalysis::ic_boundary::{
    rho_reference, rho_reference_negation, CountedGroup, GroupOps,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

// ── small helpers ───────────────────────────────────────────────────────

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn pmul(k: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = k.add(c[i + j], k.mul(x, y));
        }
    }
    c
}

/// The xorshift stream shared by every arm (two draws per trial).
struct Stream(u64);

impl Stream {
    fn new(seed: u64) -> Stream {
        Stream(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn pt_base(x: u64, y: u64) -> Pt {
    Some((F2::base(x), F2::base(y)))
}

fn xy(p: &Pt) -> Option<(u64, u64)> {
    p.map(|(x, y)| (x.a, y.a))
}

/// Roots in `F_p` of a polynomial (lowest degree first) with multiplicities,
/// by brute force over `F_p`.
pub fn roots_mult(k: &Fp, poly: &[u64]) -> Vec<(u64, u32)> {
    let mut q: Vec<u64> = poly.iter().map(|&c| c % k.p).collect();
    while q.len() > 1 && *q.last().unwrap() == 0 {
        q.pop();
    }
    let mut out = vec![];
    for x in 0..k.p {
        let mut m = 0u32;
        while q.len() > 1 {
            let n = q.len() - 1;
            let mut b = vec![0u64; n];
            b[n - 1] = q[n];
            for i in (1..n).rev() {
                b[i - 1] = k.add(q[i], k.mul(x, b[i]));
            }
            let rem = k.add(q[0], k.mul(x, b[0]));
            if rem != 0 {
                break;
            }
            q = b;
            m += 1;
        }
        if m > 0 {
            out.push((x, m));
        }
    }
    out
}

fn horner(k: &Fp, coeffs: &[u64], x: u64) -> u64 {
    coeffs
        .iter()
        .rev()
        .fold(0u64, |acc, &c| k.add(k.mul(acc, x), c))
}

fn collision_log(r: u64, a0: u64, b0: u64, e0: i64, a: u64, b: u64, e: i64) -> Option<u64> {
    let rr = r as i128;
    let m = |v: i128| ((v % rr) + rr) % rr;
    let num = m(-(e0 as i128 * a as i128 - e as i128 * a0 as i128));
    let den = m(e0 as i128 * b as i128 - e as i128 * b0 as i128);
    if den == 0 {
        return None;
    }
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

// ── the degree-3 cover ──────────────────────────────────────────────────

/// `C : y² = c(h(x))`, genus 4, with the map `(x, y) ↦ (h(x), y)` to `E`.
#[derive(Clone)]
pub struct Cover3 {
    pub p: u64,
    pub k: Fp,
    pub e: Curve,
    pub h1: u64,
    pub h0: u64,
    pub curve: HCurve,
}

impl Cover3 {
    /// `f = c(h(x))`, lowest degree first (length 10).
    pub fn f_of(k: &Fp, e: &Curve, h1: u64, h0: u64) -> Vec<u64> {
        let h = [h0, h1, 0, 1];
        let h2 = pmul(k, &h, &h);
        let h3 = pmul(k, &h2, &h);
        let mut f = vec![0u64; 10];
        for (i, &v) in h3.iter().enumerate() {
            f[i] = k.add(f[i], v);
        }
        for (i, &v) in h2.iter().enumerate() {
            f[i] = k.add(f[i], k.mul(e.a2, v));
        }
        for (i, &v) in h.iter().enumerate() {
            f[i] = k.add(f[i], k.mul(e.a4, v));
        }
        f[0] = k.add(f[0], e.a6);
        f
    }

    pub fn new(e: Curve, h1: u64, h0: u64) -> Result<Cover3, String> {
        let k = e.k;
        let p = k.p;
        let f = Self::f_of(&k, &e, h1 % p, h0 % p);
        let curve = HCurve::new(p, 4, &f)?;
        Ok(Cover3 {
            p,
            k,
            e,
            h1: h1 % p,
            h0: h0 % p,
            curve,
        })
    }

    pub fn h(&self, x: u64) -> u64 {
        horner(&self.k, &[self.h0, self.h1, 0, 1], x)
    }

    /// `c^*(Q)` as the Mumford pair `(h(x) − s, Y)`; the identity for `O`.
    pub fn pull(&self, pt: &Pt) -> HDiv {
        match pt {
            None => HDiv::IDENTITY,
            Some((s, y)) => {
                let mut d = HDiv {
                    deg: 3,
                    ..HDiv::IDENTITY
                };
                d.u[0] = self.k.sub(self.h0, s.a) as u8;
                d.u[1] = self.h1 as u8;
                d.v[0] = y.a as u8;
                d
            }
        }
    }

    /// The rational part of the fibre `h(x) = s`: roots with multiplicity.
    pub fn fibre_roots(&self, s: u64) -> Vec<(u64, u32)> {
        roots_mult(&self.k, &[self.k.sub(self.h0, s), self.h1, 0, 1])
    }

    /// Whether the fibre over the abscissa `s` is completely rational.
    pub fn fibre_split(&self, s: u64) -> bool {
        self.fibre_roots(s).iter().map(|&(_, m)| m).sum::<u32>() == 3
    }

    /// `c(P)` for the place `(x, y)`.
    pub fn image(&self, x: u64, y: u64) -> (u64, u64) {
        (self.h(x), y)
    }
}

/// A built instance: curve, order-`r` pair, cover, and the pulled-back pair.
pub struct Instance {
    pub p: u64,
    pub r: u64,
    pub x: u64,
    pub n_e: u64,
    pub e: Curve,
    pub gen: Pt,
    pub target: Pt,
    pub cov: Cover3,
    pub jac_order: u64,
    pub d: HDiv,
    pub t: HDiv,
    pub h_attempts: u32,
    pub hom_checks: u32,
    pub tower_seed: u64,
    pub h_seed: u64,
    /// One-time construction wall time (tower, cover search, checks).
    pub build_ms: f64,
}

/// Build instance `j` of the prime at index `i`: `tower::build(p, 11 + i + 37 j)`
/// and the `tower_ladder` search for `h` (stream seed `1000 + i + 37 j`).
pub fn build_instance(p: u64, i: u64, j: u64) -> Result<Instance, String> {
    let t0 = Instant::now();
    let tower_seed = 11 + i + 37 * j;
    let h_seed = 1000 + i + 37 * j;
    let t: TowerInstance = build(p, tower_seed).ok_or("tower::build returned None")?;
    let k = t.e.k;
    for attempt in 0..300u32 {
        let z = mix(h_seed.wrapping_mul(7919).wrapping_add(u64::from(attempt)));
        let (h1, h0) = (z % t.p, (z >> 24) % t.p);
        let Ok(cov) = Cover3::new(t.e, h1, h0) else {
            continue;
        };
        let Ok(jo) = cov.curve.jacobian_order(t.r, h_seed ^ u64::from(attempt)) else {
            continue;
        };
        if !jo.is_multiple_of(t.r) || (jo / t.r).is_multiple_of(t.r) {
            continue;
        }
        let d = cov.pull(&t.gen);
        if !cov.curve.is_valid(&d)
            || d.is_identity()
            || !cov.curve.mul_uncounted(d, t.r).is_identity()
        {
            return Err("pull-back of the generator is invalid or not of order r".into());
        }
        let mut checks = 0;
        for kk in 1..t.r {
            let q = t.e.mul(&t.gen, u128::from(kk));
            let dq = cov.pull(&q);
            if !cov.curve.is_valid(&dq) {
                return Err(format!("pull-back of [{kk}]G is not a valid divisor"));
            }
            if dq != cov.curve.mul_uncounted(d, kk) {
                return Err(format!("homomorphism fails at k = {kk}"));
            }
            checks += 1;
        }
        let tt = cov.pull(&t.target);
        if tt != cov.curve.mul_uncounted(d, t.x) {
            return Err("target pull-back is not [x]D".into());
        }
        let _ = k;
        return Ok(Instance {
            p,
            r: t.r,
            x: t.x,
            n_e: t.e.order(),
            e: t.e,
            gen: t.gen,
            target: t.target,
            cov,
            jac_order: jo,
            d,
            t: tt,
            h_attempts: attempt + 1,
            hom_checks: checks,
            tower_seed,
            h_seed,
            build_ms: t0.elapsed().as_secs_f64() * 1e3,
        });
    }
    Err("no h with r || #J4 in 300 attempts".into())
}

// ── structure of the factor base ────────────────────────────────────────

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct BStats {
    pub n_e: u64,
    /// Rational affine points of `C`.
    pub c_affine_points: u64,
    /// `|B1|` from the places of `C` (distinct images).
    pub b1_from_places: u64,
    /// `|B1|` from `E` and `h` alone.
    pub b1_from_e: u64,
    /// `|B3|`: affine `Q` whose fibre is completely rational.
    pub b3: u64,
    /// Distinct rational places over an affine `Q ∈ E`: counts for 0, 1, 2, 3.
    pub places_hist: [u64; 4],
    /// Affine points of `E` whose fibre has a repeated root.
    pub ramified: u64,
    /// Ramified fibres that are completely rational.
    pub ramified_split: u64,
    /// Fibre sizes (distinct places per image point) among the images of rational places.
    pub image_places_hist: [u64; 4],
    /// Number of `k` in `1..r` with `[k]G_E` in `B3`.
    pub b3_in_g: u64,
    /// Distinct abscissae of those (columns up to sign).
    pub b3_in_g_cols: u64,
}

pub fn b_stats(inst: &Instance) -> BStats {
    let cov = &inst.cov;
    let k = &cov.k;
    let p = inst.p;
    let mut s = BStats {
        n_e: inst.n_e,
        ..Default::default()
    };
    // From the places of C.
    let mut imgs: HashMap<(u64, u64), u64> = HashMap::new();
    let mut fv = Cover3::f_of(k, &inst.e, cov.h1, cov.h0);
    fv.truncate(10);
    for x in 0..p {
        let fx = horner(k, &fv, x);
        let Some(y) = k.sqrt(fx) else { continue };
        let ys: Vec<u64> = if y == 0 { vec![0] } else { vec![y, k.neg(y)] };
        for yy in ys {
            s.c_affine_points += 1;
            *imgs.entry(cov.image(x, yy)).or_default() += 1;
        }
    }
    s.b1_from_places = imgs.len() as u64;
    for &n in imgs.values() {
        s.image_places_hist[(n as usize).min(3)] += 1;
    }
    // From E alone.
    for q in inst.e.rational_points().iter().flatten() {
        let roots = cov.fibre_roots(q.0.a);
        let distinct = roots.len();
        let total: u32 = roots.iter().map(|&(_, m)| m).sum();
        s.places_hist[distinct.min(3)] += 1;
        if distinct > 0 {
            s.b1_from_e += 1;
        }
        if roots.iter().any(|&(_, m)| m >= 2) {
            s.ramified += 1;
            if total == 3 {
                s.ramified_split += 1;
            }
        }
        if total == 3 {
            s.b3 += 1;
        }
    }
    // Inside the transferred subgroup.
    let mut cols = HashSet::new();
    for kk in 1..inst.r {
        let q = inst.e.mul(&inst.gen, u128::from(kk));
        if let Some((sx, _)) = xy(&q) {
            if cov.fibre_split(sx) {
                s.b3_in_g += 1;
                cols.insert(sx);
            }
        }
    }
    s.b3_in_g_cols = cols.len() as u64;
    s
}

// ── decomposition and classification ────────────────────────────────────

/// A place with multiplicity: `(x, y, m)`.
pub type Place = (u64, u64, u32);

/// Split a reduced class over `F_p` into places counted with multiplicity;
/// `None` if `u` has fewer than `deg u` roots in `F_p`.
pub fn decompose(k: &Fp, d: &HDiv) -> Option<Vec<Place>> {
    let u = d.u_coeffs();
    let v = d.v_coeffs();
    let roots = roots_mult(k, &u);
    if roots.iter().map(|&(_, m)| m as usize).sum::<usize>() != d.deg as usize {
        return None;
    }
    Some(
        roots
            .into_iter()
            .map(|(x, m)| (x, horner(k, &v, x), m))
            .collect(),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub enum FibreClass {
    Identity,
    /// The places are exactly the whole fibre `c^*(Q)` of one point `Q`.
    SingleFull,
    /// A disjoint union of two or more whole fibres.
    UnionFull,
    /// Anything else.
    Other,
}

/// Classify a place multiset against the fibres of `c`.
pub fn classify(cov: &Cover3, places: &[Place]) -> FibreClass {
    if places.is_empty() {
        return FibreClass::Identity;
    }
    let mut groups: BTreeMap<(u64, u64), Vec<(u64, u32)>> = BTreeMap::new();
    for &(x, y, m) in places {
        groups.entry(cov.image(x, y)).or_default().push((x, m));
    }
    let mut full = 0;
    for ((s, _y), mut g) in groups.clone() {
        g.sort_unstable();
        let mut fibre = cov.fibre_roots(s);
        fibre.sort_unstable();
        let total: u32 = fibre.iter().map(|&(_, m)| m).sum();
        if total == 3 && g == fibre {
            full += 1;
        }
    }
    if full == groups.len() {
        if full == 1 {
            FibreClass::SingleFull
        } else {
            FibreClass::UnionFull
        }
    } else {
        FibreClass::Other
    }
}

/// `Σ m_i c(P_i)` on `E`, by point arithmetic; `None` in the inner option if
/// a place does not map to a point of `E`.
pub fn push_places(cov: &Cover3, places: &[Place]) -> Option<Pt> {
    let e = &cov.e;
    let mut acc: Pt = None;
    for &(x, y, m) in places {
        let (s, yy) = cov.image(x, y);
        let q = pt_base(s, yy);
        if !e.on_curve(&q) {
            return None;
        }
        for _ in 0..m {
            acc = e.add(&acc, &q);
        }
    }
    Some(acc)
}

// ── counted scalar multiplication on J(C) ───────────────────────────────

/// `[n]d` with the convention of `measure::scalar_counted` (the first addition
/// into the identity is not charged) and, separately, the library's ledger.
fn jac_scalar(c: &HCurve, d: &HDiv, mut n: u64, ledger: &mut GroupOps) -> (HDiv, u64) {
    let mut acc = HDiv::IDENTITY;
    let mut base = *d;
    let mut ops = 0u64;
    while n > 0 {
        if n & 1 == 1 {
            if !acc.is_identity() {
                ops += 1;
            }
            acc = CountedGroup::add(c, ledger, acc, base);
        }
        n >>= 1;
        if n > 0 {
            base = CountedGroup::double(c, ledger, base);
            ops += 1;
        }
    }
    (acc, ops)
}

// ── arm A1: index calculus on J(C), with classification and push-forward ─

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Collision {
    pub relations: u64,
    pub trials: u64,
    pub ops: u64,
    pub log: Option<u64>,
}

/// One accepted J(C) relation, kept for the classification.
#[derive(Clone, Debug)]
pub struct JcRel {
    pub alpha: u64,
    pub beta: u64,
    pub trial: u64,
    pub cum_ops: u64,
    pub q: Pt,
    pub places: Vec<Place>,
    pub class: FibreClass,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct JcRun {
    pub reached_want: bool,
    pub nfb: usize,
    pub trials: u64,
    pub identity_trials: u64,
    pub relations: u64,
    pub accepted_trials: Vec<u64>,
    /// Counted group operations (convention of `scalar_counted` + 1 per combine).
    pub ops: u64,
    /// The library ledger `adds + doubles` over the same operations.
    pub ledger_ops: u64,
    pub la_mulmods: u64,
    pub recovered: Option<u64>,
    pub certified_j: bool,
    pub certified_e: bool,
    /// Trials (all, nonidentity or identity) whose reduced class equals `c^*(Q)`.
    pub class_eq_pullback: u64,
    pub single_full: u64,
    pub union_full: u64,
    pub other: u64,
    /// Rows by multiplicity pattern of the fibre: all distinct roots, or one repeated.
    pub rows_ramified: u64,
    pub push_valid: u64,
    pub push_invalid: u64,
    /// Pushed row not equal to `[3]Q`.
    pub push_not_3q: u64,
    pub first_collision: Option<Collision>,
}

pub fn jc_ic(inst: &Instance, seed: u64, max_trials: u64) -> JcRun {
    let cov = &inst.cov;
    let k = &cov.k;
    let e = &inst.e;
    let c = &cov.curve;
    let r = inst.r;
    // columns: affine places with y <= (p-1)/2
    let mut cols: HashMap<(u64, u64), usize> = HashMap::new();
    let fv = Cover3::f_of(k, e, cov.h1, cov.h0);
    for x in 0..inst.p {
        if let Some(y) = k.sqrt(horner(k, &fv, x)) {
            let y0 = y.min(k.neg(y));
            let n = cols.len();
            cols.entry((x, y0)).or_insert(n);
        }
    }
    let nfb = cols.len();
    let want = nfb + 8;
    let mut st = Stream::new(seed);
    let mut run = JcRun {
        nfb,
        ..Default::default()
    };
    let mut ledger = GroupOps::default();
    let mut rows: Vec<(Vec<i64>, u64, u64)> = vec![];
    let mut rels: Vec<JcRel> = vec![];
    let mut ops = 0u64;
    let mut trials = 0u64;
    while rels.len() < want && trials < max_trials {
        trials += 1;
        let alpha = st.next() % r;
        let beta = st.next() % r;
        let (ra, oa) = jac_scalar(c, &inst.d, alpha, &mut ledger);
        let (rb, ob) = jac_scalar(c, &inst.t, beta, &mut ledger);
        ops += oa + ob + 1;
        let rr = CountedGroup::add(c, &mut ledger, ra, rb);
        // Q on E alone, uncounted.
        let q = e.add(
            &e.mul(&inst.gen, u128::from(alpha)),
            &e.mul(&inst.target, u128::from(beta)),
        );
        if rr == cov.pull(&q) {
            run.class_eq_pullback += 1;
        }
        if rr.is_identity() {
            run.identity_trials += 1;
            continue;
        }
        let Some(places) = decompose(k, &rr) else {
            continue;
        };
        let class = classify(cov, &places);
        match class {
            FibreClass::SingleFull => run.single_full += 1,
            FibreClass::UnionFull => run.union_full += 1,
            _ => run.other += 1,
        }
        if places.iter().any(|&(_, _, m)| m >= 2) {
            run.rows_ramified += 1;
        }
        // c_* by point arithmetic on E.
        let pushed = push_places(cov, &places);
        let want_pt = e.add(
            &e.mul(&inst.gen, 3 * u128::from(alpha)),
            &e.mul(&inst.target, 3 * u128::from(beta)),
        );
        match pushed {
            Some(pp) if pp == want_pt => run.push_valid += 1,
            _ => run.push_invalid += 1,
        }
        if pushed != Some(e.mul(&q, 3)) {
            run.push_not_3q += 1;
        }
        let mut row = vec![0i64; nfb];
        for &(x, y, m) in &places {
            let y0 = y.min(k.neg(y));
            let sgn = if y == y0 { 1 } else { -1 };
            row[cols[&(x, y0)]] += i64::from(m) * sgn;
        }
        rows.push((row, alpha, beta));
        run.accepted_trials.push(trials);
        rels.push(JcRel {
            alpha,
            beta,
            trial: trials,
            cum_ops: ops,
            q,
            places,
            class,
        });
    }
    run.reached_want = rels.len() >= want;
    run.trials = trials;
    run.relations = rels.len() as u64;
    run.ops = ops;
    run.ledger_ops = ledger.adds + ledger.doubles;
    let (rec, la) = solve_linear_free(&rows, nfb, r);
    run.recovered = rec;
    run.la_mulmods = la;
    run.certified_j = rec.is_some_and(|x| c.mul_uncounted(inst.d, x) == inst.t);
    run.certified_e = rec.is_some_and(|x| certify_ec(e, &inst.gen, &inst.target, x));
    // first collision on the relation list, keyed by the image +-Q of the first place
    let mut seen: HashMap<u64, (u64, u64, i64)> = HashMap::new();
    for (i, rel) in rels.iter().enumerate() {
        let Some((qx, qy)) = xy(&rel.q) else { continue };
        let sg = if qy <= k.neg(qy) { 1 } else { -1 };
        if let Some(&(a0, b0, e0)) = seen.get(&qx) {
            if let Some(x) = collision_log(r, a0, b0, e0, rel.alpha, rel.beta, sg) {
                run.first_collision = Some(Collision {
                    relations: i as u64 + 1,
                    trials: rel.trial,
                    ops: rel.cum_ops,
                    log: Some(x),
                });
                break;
            }
        } else {
            seen.insert(qx, (rel.alpha, rel.beta, sg));
        }
    }
    run
}

// ── arm A4: the same search natively on E ───────────────────────────────

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct EFibreRun {
    pub trials: u64,
    pub identity_trials: u64,
    pub relations: u64,
    pub cols: usize,
    pub accepted_trials: Vec<u64>,
    pub ops: u64,
    pub la_mulmods: u64,
    pub recovered: Option<u64>,
    pub certified: bool,
    pub first_collision: Option<Collision>,
}

/// Same stream; `Q = [α]G_E + [β]T_E` accepted iff its fibre is completely
/// rational (tested on `E` and `h` only); one column per `±Q`; stops at `want` rows.
pub fn e_fibre_ic(inst: &Instance, seed: u64, want: usize, max_trials: u64) -> EFibreRun {
    let e = &inst.e;
    let cov = &inst.cov;
    let p = inst.p;
    let r = inst.r;
    let mut st = Stream::new(seed);
    let mut run = EFibreRun::default();
    let mut cols: HashMap<u64, usize> = HashMap::new();
    let mut raw: Vec<(usize, i64, u64, u64)> = vec![];
    let mut first: HashMap<usize, (u64, u64, i64)> = HashMap::new();
    let mut ops = 0u64;
    let mut trials = 0u64;
    while raw.len() < want && trials < max_trials {
        trials += 1;
        let alpha = st.next() % r;
        let beta = st.next() % r;
        let (ra, oa) = ec_scalar_counted(e, &inst.gen, alpha);
        let (rb, ob) = ec_scalar_counted(e, &inst.target, beta);
        ops += oa + ob + 1;
        let q = e.add(&ra, &rb);
        let Some((sx, qy)) = xy(&q) else {
            run.identity_trials += 1;
            continue;
        };
        if !cov.fibre_split(sx) {
            continue;
        }
        let n = cols.len();
        let col = *cols.entry(sx).or_insert(n);
        let sgn = if qy <= (p - qy) % p { 1 } else { -1 };
        raw.push((col, sgn, alpha, beta));
        run.accepted_trials.push(trials);
        if run.first_collision.is_none() {
            if let Some(&(a0, b0, e0)) = first.get(&col) {
                if let Some(x) = collision_log(r, a0, b0, e0, alpha, beta, sgn) {
                    run.first_collision = Some(Collision {
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
        .map(|&(c, s, a, b)| {
            let mut v = vec![0i64; ncols];
            v[c] = s;
            (v, a, b)
        })
        .collect();
    let (rec, la) = solve_linear_free(&rows, ncols, r);
    run.trials = trials;
    run.relations = raw.len() as u64;
    run.cols = ncols;
    run.ops = ops;
    run.la_mulmods = la;
    run.recovered = rec;
    run.certified = rec.is_some_and(|x| certify_ec(e, &inst.gen, &inst.target, x));
    run
}

/// Control: every nonzero `Q` kept, keyed by `±Q`, stop at the first usable
/// collision. No cover, no filter.
pub fn e_birthday(inst: &Instance, seed: u64, max_trials: u64) -> Option<Collision> {
    let e = &inst.e;
    let p = inst.p;
    let r = inst.r;
    let mut st = Stream::new(seed);
    let mut ops = 0u64;
    let mut seen: HashMap<u64, (u64, u64, i64)> = HashMap::new();
    let mut kept = 0u64;
    for trials in 1..=max_trials {
        let alpha = st.next() % r;
        let beta = st.next() % r;
        let (ra, oa) = ec_scalar_counted(e, &inst.gen, alpha);
        let (rb, ob) = ec_scalar_counted(e, &inst.target, beta);
        ops += oa + ob + 1;
        let Some((qx, qy)) = xy(&e.add(&ra, &rb)) else {
            continue;
        };
        kept += 1;
        let sgn = if qy <= (p - qy) % p { 1 } else { -1 };
        if let Some(&(a0, b0, e0)) = seen.get(&qx) {
            if let Some(x) = collision_log(r, a0, b0, e0, alpha, beta, sgn) {
                return Some(Collision {
                    relations: kept,
                    trials,
                    ops,
                    log: Some(x),
                });
            }
        } else {
            seen.insert(qx, (alpha, beta, sgn));
        }
    }
    None
}

// ── one cell, one seed ──────────────────────────────────────────────────

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct Rho {
    pub ops: u64,
    pub ok: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SeedRow {
    pub seed: u64,
    pub rho: Rho,
    pub rho_neg: Rho,
    pub jc: JcRun,
    pub e: EFibreRun,
    pub birthday: Option<Collision>,
    pub birthday_certified: bool,
    /// A1 and A4 accepted the same trials, same relation count, same counted ops.
    pub same_trials: bool,
    pub same_ops: bool,
    pub same_relations: bool,
    pub same_log: bool,
}

pub fn run_seed(inst: &Instance, s: u64, max_trials: u64) -> SeedRow {
    let stream_seed = 300 + s;
    let eg = EcGroup(inst.e);
    let rho = rho_reference(&eg, inst.gen, inst.target, inst.r, 100 + s, 1 << 24);
    let rho_n = rho_reference_negation(&eg, inst.gen, inst.target, inst.r, 100 + s, 1 << 24);
    let jc = jc_ic(inst, stream_seed, max_trials);
    let e = e_fibre_ic(inst, stream_seed, jc.relations as usize, max_trials);
    let bd = e_birthday(inst, stream_seed, max_trials);
    let bd_cert = bd
        .as_ref()
        .and_then(|c| c.log)
        .is_some_and(|x| certify_ec(&inst.e, &inst.gen, &inst.target, x));
    SeedRow {
        seed: s,
        rho: Rho {
            ops: rho.group_ops.adds + rho.group_ops.doubles,
            ok: rho.recovered == Some(inst.x),
        },
        rho_neg: Rho {
            ops: rho_n.group_ops.adds + rho_n.group_ops.doubles,
            ok: rho_n.recovered == Some(inst.x),
        },
        same_trials: jc.accepted_trials == e.accepted_trials,
        same_ops: jc.ops == e.ops,
        same_relations: jc.relations == e.relations,
        same_log: jc.recovered == e.recovered,
        jc,
        e,
        birthday: bd,
        birthday_certified: bd_cert,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inst(p: u64, i: u64) -> Instance {
        for j in 0..12 {
            if let Ok(x) = build_instance(p, i, j) {
                if b_stats(&x).b3_in_g_cols >= 2 {
                    return x;
                }
            }
        }
        panic!("no admissible instance at p = {p}");
    }

    #[test]
    fn push_pull_is_multiplication_by_three() {
        let x = inst(101, 2);
        let cov = &x.cov;
        let mut checked = 0;
        for kk in 1..x.r {
            let q = x.e.mul(&x.gen, u128::from(kk));
            let (sx, _) = xy(&q).unwrap();
            if !cov.fibre_split(sx) {
                continue;
            }
            let d = cov.pull(&q);
            let pl = decompose(&cov.k, &d).expect("split fibre decomposes");
            assert_eq!(classify(cov, &pl), FibreClass::SingleFull);
            let pushed = push_places(cov, &pl).unwrap();
            assert_eq!(pushed, x.e.mul(&q, 3), "c_* c^* = [3]");
            checked += 1;
        }
        assert!(checked >= 2, "checked {checked}");
    }

    #[test]
    fn relations_verify_and_are_fibres_and_match_the_e_arm() {
        let x = inst(131, 3);
        for s in 0..3u64 {
            let row = run_seed(&x, s, 5_000_000);
            let jc = &row.jc;
            assert!(jc.reached_want);
            assert_eq!(jc.push_invalid, 0);
            assert_eq!(jc.push_not_3q, 0);
            assert_eq!(jc.other + jc.union_full, 0);
            assert_eq!(jc.single_full, jc.relations);
            assert_eq!(jc.class_eq_pullback, jc.trials);
            assert_eq!(jc.recovered, Some(x.x));
            assert!(jc.certified_j && jc.certified_e);
            assert!(row.same_trials && row.same_ops && row.same_relations && row.same_log);
            assert!(row.e.certified);
        }
    }

    #[test]
    fn classification_on_hand_built_divisors() {
        let x = inst(151, 4);
        let cov = &x.cov;
        // Find two points of E with completely rational fibres.
        let mut full = vec![];
        for kk in 1..x.r {
            let q = x.e.mul(&x.gen, u128::from(kk));
            let (sx, sy) = xy(&q).unwrap();
            if cov.fibre_split(sx) {
                full.push((sx, sy));
                if full.len() == 2 {
                    break;
                }
            }
        }
        assert_eq!(full.len(), 2);
        let places_of = |(s, y): (u64, u64)| -> Vec<Place> {
            cov.fibre_roots(s)
                .into_iter()
                .map(|(x, m)| (x, y, m))
                .collect()
        };
        let f0 = places_of(full[0]);
        let f1 = places_of(full[1]);
        assert_eq!(classify(cov, &f0), FibreClass::SingleFull);
        let mut both = f0.clone();
        both.extend(f1.iter().copied());
        assert_eq!(classify(cov, &both), FibreClass::UnionFull);
        // two of the three places of one fibre: not a full fibre
        let mut part = f0.clone();
        part.truncate(2);
        assert_eq!(classify(cov, &part), FibreClass::Other);
        // one place from each of two fibres
        let mix2 = vec![f0[0], f1[0]];
        assert_eq!(classify(cov, &mix2), FibreClass::Other);
        assert_eq!(classify(cov, &[]), FibreClass::Identity);
        // a divisor built on J(C) from the three places of a fibre is c^*(Q)
        let mut d = HDiv::IDENTITY;
        for &(px, py, m) in &f0 {
            for _ in 0..m {
                d = cov.curve.cantor(d, cov.curve.point(px, py).unwrap());
            }
        }
        assert_eq!(d, cov.pull(&pt_base(full[0].0, full[0].1)));
        assert_eq!(
            classify(cov, &decompose(&cov.k, &d).unwrap()),
            FibreClass::SingleFull
        );
    }

    #[test]
    fn b_structure_is_consistent() {
        let x = inst(181, 5);
        let s = b_stats(&x);
        assert_eq!(s.b1_from_places, s.b1_from_e);
        assert_eq!(
            s.places_hist.iter().sum::<u64>(),
            x.e.rational_points().len() as u64 - 1
        );
    }
}
