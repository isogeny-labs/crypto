//! **Round 5, lead R5.1: does a quotient-coordinate oracle remove the
//! kernel-saturation penalty in characteristic 2?**
//!
//! `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.2, §17.4, §17.7.  On a Koblitz
//! curve `y² + xy = x³ + ax² + 1` over `F_{2^n}` (n odd) the rational
//! 2-torsion point is `T = (0, 1)`.  Vélu's formulas for the kernel `⟨T⟩`
//! (derived in `RESULTS.md`, validated by `--exp validate`) give
//!
//! ```text
//!   φ(x, y) = (x + s/x,  y + s(x + y + s)/x² + s),   s = √b,
//!   E/⟨T⟩ :  y² + xy = x³ + ax² + s.
//! ```
//!
//! For `b = 1` the codomain is `E` itself and `φ = ±(π ± 1)` is the
//! Verschiebung, so the quotient coordinate is `x' = x + 1/x`.
//!
//! Arms, at one base size `|F|` (exact: unions of signed Frobenius orbits):
//!
//! | arm | base | oracle |
//! |---|---|---|
//! | a  | `2K` random signed Frobenius orbits | library `mitm-frobenius` |
//! | b  | `K` random orbits closed under `+T` with the library's `saturate_factor_base_two_torsion` | library `mitm-frobenius`, sweeping all of `F` |
//! | c1 | the base of b, pushed to `E/⟨T⟩`: `F' = φ(F_b)` (`|F|/2` points, keyed on `x'`) | library `mitm-frobenius` on `F'`, target `φ(R)`, relations lifted back to `F_b` |
//! | c2 | the base of b | library pair table on `F_b`, sweep one representative per `{P, P+T}` |
//! | d  | `2K` random orbits inside `2E = φ(E)` (index 2) | library `mitm-frobenius` |
//!
//! Two kinds of evidence:
//!
//! * an exact census: every multiset of `m` base points whose sum lies in
//!   the order-`r` subgroup `G`, so `D_m` (decomposable targets) and the
//!   distinct projected relation rows are exact over all of `G`;
//! * the library oracle's counted cost on a shared random target list
//!   (group additions + table lookups per target; canonicalisations are
//!   reported apart), with a paired bootstrap over targets.
//!
//! `--exp a1` re-runs the §7.A1-R comparison point
//! (`icv1-f2m31-tm90707-c95f16f5`, divisor `0;1;2`) with the quotient arm.
//!
//! ```text
//! cargo run --release --example r5_1_quotient_oracle -- --exp validate --json validate.json
//! cargo run --release --example r5_1_quotient_oracle -- --exp grid --json grid.json
//! cargo run --release --example r5_1_quotient_oracle -- --exp a1 --json a1.json
//! ```

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_velu::Curve as VeluCurve;
use crypto_lib::cryptanalysis::ic_boundary::{
    decompose_mitm_frobenius, koblitz_factor_base, koblitz_instance, BinaryGroup, BinaryInstance,
    ColumnFold, CountedGroup, FactorBase, FrobeniusPairTable, GroupOps, OracleCounters,
};
use crypto_lib::cryptanalysis::koblitz_factor_base_search::FactorBaseSpec;
use crypto_lib::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    build_explicit_frobenius_orbit_factor_base, saturate_factor_base_two_torsion,
    FrobeniusFactorBase,
};
use crypto_lib::cryptanalysis::koblitz_symmetrised::{
    build_symmetrised_factor_base, frobenius_view_of_symmetrised,
};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;

// ── the 2-isogeny with kernel ⟨(0, √b)⟩ ────────────────────────────

/// `φ(x, y) = (x + s/x, y + s(x + y + s)/x² + s)` with `s = √b`; the
/// codomain is `y² + xy = x³ + ax² + s`.  `O` and `T = (0, s)` go to `O`.
fn phi(f: &Gf2, s: u64, p: FastPoint) -> FastPoint {
    if p.infinity || p.x == 0 {
        return FastPoint::INFINITY;
    }
    let xi = f.inv(p.x);
    let x2 = p.x ^ f.mul(s, xi);
    let y2 = p.y ^ f.mul(s, f.mul(p.x ^ p.y ^ s, f.sqr(xi))) ^ s;
    FastPoint::affine(x2, y2)
}

fn mask(n: u32) -> u64 {
    (1u64 << n) - 1
}

fn trace(f: &Gf2, n: u32, x: u64) -> u64 {
    let mut t = 0u64;
    let mut v = x;
    for _ in 0..n {
        t ^= v;
        v = f.sqr(v);
    }
    t & 1
}

/// Every affine point of `y² + xy = x³ + ax² + b` (toy `n` only).
fn all_points(c: &VeluCurve) -> Vec<FastPoint> {
    let mut out = Vec::new();
    for x in 0..(1u64 << c.n) {
        out.extend(c.points_with_x(x));
    }
    out
}

/// Validate φ on one curve: codomain membership, homomorphism, kernel,
/// codomain order, and (for `b = 1`) its identity with `±(π ± 1)`.
fn validate_curve(n: u32, a: u8, b: u64, rng: &mut StdRng) -> Value {
    let inst = koblitz_instance(a, n).expect("Koblitz instance");
    let irr = inst.irreducible.clone();
    let dom = VeluCurve::new(n, &irr, a, b).expect("domain");
    let f = &dom.fast.field;
    let s = f.sqr_k(b, n - 1);
    assert_eq!(f.sqr(s), b);
    let cod = VeluCurve::new(n, &irr, a, s).expect("codomain");
    let pts = all_points(&dom);
    let mut off_curve = 0u64;
    let mut kernel = vec![];
    for &p in &pts {
        let q = phi(f, s, p);
        if q.infinity {
            kernel.push(p.x);
        } else if !cod.fast.is_on_curve(q) {
            off_curve += 1;
        }
    }
    let mut hom_fail = 0u64;
    let trials = 4000u64;
    for _ in 0..trials {
        let p = pts[rng.gen_range(0..pts.len())];
        let q = pts[rng.gen_range(0..pts.len())];
        let lhs = phi(f, s, dom.fast.add(p, q));
        let rhs = cod.fast.add(phi(f, s, p), phi(f, s, q));
        if lhs != rhs {
            hom_fail += 1;
        }
    }
    // Image of φ has index 2 and equals 2E; check against [N/2]P = O and
    // against the trace criterion Tr(x) = Tr(a) on the codomain.
    let half = BigUint::from(cod.order / 2);
    let mut image: HashSet<u64> = HashSet::new();
    for &p in &pts {
        let q = phi(f, s, p);
        if !q.infinity {
            image.insert(q.pack());
        }
    }
    let cod_pts = all_points(&cod);
    let (mut img_vs_half, mut img_vs_trace) = (0u64, 0u64);
    for &q in &cod_pts {
        let in_img = image.contains(&q.pack());
        let by_half = cod.fast.mul(q, &half).infinity;
        let by_trace = q.x != 0 && trace(f, n, q.x) == u64::from(a);
        if in_img != by_half {
            img_vs_half += 1;
        }
        if in_img != by_trace {
            img_vs_trace += 1;
        }
    }
    // Identity with the Verschiebung on the Koblitz model.
    let mut v_identity = Value::Null;
    if b == 1 {
        let mut counts = [0u64; 4];
        for &p in &pts {
            let q = phi(f, s, p);
            let fp = dom.fast.frobenius_k(p, 1);
            let cands = [
                dom.fast.add(fp, p),
                dom.fast.add(fp, dom.fast.neg(p)),
                dom.fast.neg(dom.fast.add(fp, p)),
                dom.fast.neg(dom.fast.add(fp, dom.fast.neg(p))),
            ];
            for (k, c) in cands.iter().enumerate() {
                if *c == q {
                    counts[k] += 1;
                }
            }
        }
        v_identity = json!({
            "phi_eq_pi_plus_1": counts[0], "phi_eq_pi_minus_1": counts[1],
            "phi_eq_minus_pi_minus_1": counts[2], "phi_eq_1_minus_pi": counts[3],
            "points": pts.len(),
        });
    }
    json!({
        "n": n, "a": a, "b": b, "s_sqrt_b": s,
        "domain_order": dom.order, "codomain_order": cod.order,
        "orders_equal": dom.order == cod.order,
        "affine_points": pts.len(),
        "kernel_abscissae": kernel,
        "kernel_is_T_only": kernel == vec![0u64],
        "image_off_codomain": off_curve,
        "homomorphism_trials": trials, "homomorphism_failures": hom_fail,
        "image_size_plus_O": image.len() + 1,
        "image_index": (cod.order as f64) / ((image.len() + 1) as f64),
        "image_vs_half_order_test_disagreements": img_vs_half,
        "image_vs_trace_criterion_disagreements": img_vs_trace,
        "verschiebung_identity": v_identity,
    })
}

// ── factor bases ───────────────────────────────────────────────────

struct Built {
    frob: FrobeniusFactorBase,
    fb: FactorBase<FastPoint>,
}

fn build(inst: &BinaryInstance, frob: FrobeniusFactorBase, label: &str) -> Built {
    let fb = koblitz_factor_base(
        inst,
        &frob,
        ColumnFold::ProjectedSignedFrobeniusOrbit,
        label.to_string(),
    )
    .expect("koblitz_factor_base");
    Built { frob, fb }
}

fn from_abscissae(inst: &BinaryInstance, xs: &[u64], label: &str) -> Built {
    let kc = inst.koblitz.as_ref().unwrap();
    let reps: Vec<F2mElement> = xs.iter().map(|&x| inst.gf.to_element(x)).collect();
    let frob = build_explicit_frobenius_orbit_factor_base(kc, &reps).expect("explicit base");
    build(inst, frob, label)
}

/// The Frobenius orbit of `x`, or `None` if shorter than `n`.
fn orbit(f: &Gf2, n: u32, x: u64) -> Option<Vec<u64>> {
    let mut o = vec![x];
    let mut v = f.sqr(x);
    while v != x {
        o.push(v);
        v = f.sqr(v);
    }
    (o.len() == n as usize).then_some(o)
}

struct RandomBases {
    a: Built,
    b: Built,
    d: Built,
    b_parent_orbits: usize,
}

/// `2k` plain orbits, `k` orbits saturated by `+T` (library), `2k` orbits
/// in `2E`; every base has exactly `4nk` points.
fn random_bases(inst: &BinaryInstance, k: usize, rng: &mut StdRng) -> RandomBases {
    let n = inst.n;
    let f = &inst.fast.field;
    let half = BigUint::from(inst.group_order / 2);
    let mut used: HashSet<u64> = HashSet::new();
    let canon = |x: u64| *orbit(f, n, x).unwrap().iter().min().unwrap();
    let mut draw = |pred: &dyn Fn(u64) -> bool, used: &mut HashSet<u64>| -> u64 {
        loop {
            let x = rng.gen::<u64>() & mask(n);
            if x == 0 || inst.points_with_x(x).is_empty() || orbit(f, n, x).is_none() {
                continue;
            }
            let c = canon(x);
            if used.contains(&c) || !pred(x) {
                continue;
            }
            used.insert(c);
            return x;
        }
    };
    let any = |_x: u64| true;
    let plain: Vec<u64> = (0..2 * k).map(|_| draw(&any, &mut used)).collect();
    used.clear();
    let mut parent = Vec::new();
    while parent.len() < k {
        let x = draw(&any, &mut used);
        let xi = f.inv(x);
        let ci = canon(xi);
        if ci == canon(x) || used.contains(&ci) {
            continue;
        }
        used.insert(ci);
        parent.push(x);
    }
    used.clear();
    let in_2e = |x: u64| {
        let p = inst.points_with_x(x)[0];
        inst.fast.mul(p, &half).infinity
    };
    let image: Vec<u64> = (0..2 * k).map(|_| draw(&in_2e, &mut used)).collect();
    let kc = inst.koblitz.as_ref().unwrap();
    let a = from_abscissae(inst, &plain, "a: plain random orbits");
    let parent_b = from_abscissae(inst, &parent, "b parent");
    let sat = saturate_factor_base_two_torsion(kc, &parent_b.frob).expect("saturation");
    let b = build(inst, sat, "b: +T-saturated");
    let d = from_abscissae(inst, &image, "d: inside 2E");
    let want = 4 * n as usize * k;
    assert_eq!(a.fb.points.len(), want);
    assert_eq!(b.fb.points.len(), want);
    assert_eq!(d.fb.points.len(), want);
    RandomBases {
        a,
        b,
        d,
        b_parent_orbits: k,
    }
}

/// The quotient base `F' = φ(F_b)` on `E/⟨T⟩ = E`, with the map from each
/// `F'` point to one preimage index in `F_b`.
fn quotient_base(inst: &BinaryInstance, fb_b: &FactorBase<FastPoint>) -> (Built, Vec<usize>) {
    let f = &inst.fast.field;
    let mut pre: HashMap<u64, usize> = HashMap::new();
    let mut xs: Vec<u64> = Vec::new();
    for (i, &p) in fb_b.points.iter().enumerate() {
        let q = phi(f, 1, p);
        if q.infinity {
            continue;
        }
        pre.entry(q.pack()).or_insert(i);
        xs.push(q.x);
    }
    xs.sort_unstable();
    xs.dedup();
    let c = from_abscissae(inst, &xs, "c1: phi(F_b) on E/<T>");
    let lift: Vec<usize> =
        c.fb.points
            .iter()
            .map(|q| {
                *pre.get(&q.pack())
                    .expect("every F' point has a preimage in F_b")
            })
            .collect();
    (c, lift)
}

/// One representative of each `{P, P + T}` pair of a `+T`-closed base.
fn translate_reps(inst: &BinaryInstance, fb: &FactorBase<FastPoint>) -> Vec<usize> {
    let t = inst.points_with_x(0)[0];
    let mut reps = Vec::new();
    for (i, &p) in fb.points.iter().enumerate() {
        let q = inst.fast.add(p, t);
        match fb.point_index.get(&q.pack()) {
            Some(&j) if j < i => {}
            _ => reps.push(i),
        }
    }
    reps
}

// ── cofactor classes and the exact census ──────────────────────────

struct Classes {
    id_of: HashMap<u64, u32>,
    add: Vec<u32>,
    neg: Vec<u32>,
    size: usize,
}

impl Classes {
    /// The subgroup of `E[h]` generated by `[r]P` over the given points.
    fn new(fast: &FastCurve, r: u64, pts: &[FastPoint]) -> Self {
        let mut seeds: Vec<FastPoint> = Vec::new();
        let mut seen: HashSet<u64> = HashSet::new();
        for &p in pts {
            let q = fast.mul_u64(p, r);
            if seen.insert(q.pack()) {
                seeds.push(q);
            }
        }
        let mut elems = vec![FastPoint::INFINITY];
        let mut id_of: HashMap<u64, u32> = HashMap::new();
        id_of.insert(FastPoint::INFINITY.pack(), 0);
        let mut i = 0;
        while i < elems.len() {
            for &s in &seeds {
                let e = fast.add(elems[i], s);
                if let std::collections::hash_map::Entry::Vacant(v) = id_of.entry(e.pack()) {
                    v.insert(elems.len() as u32);
                    elems.push(e);
                }
            }
            i += 1;
        }
        let h = elems.len();
        let mut add = vec![0u32; h * h];
        for x in 0..h {
            for y in x..h {
                let id = id_of[&fast.add(elems[x], elems[y]).pack()];
                add[x * h + y] = id;
                add[y * h + x] = id;
            }
        }
        let neg = elems.iter().map(|&e| id_of[&fast.neg(e).pack()]).collect();
        Classes {
            id_of,
            add,
            neg,
            size: h,
        }
    }
    fn of(&self, fast: &FastCurve, r: u64, p: FastPoint) -> u32 {
        self.id_of[&fast.mul_u64(p, r).pack()]
    }
}

#[derive(Default, Clone, Copy)]
struct Census {
    reps: u64,
    targets: u64,
    rows: u64,
}

fn row_hash(fb: &FactorBase<FastPoint>, idx: &[usize], r: u64) -> u64 {
    let mut v: Vec<(usize, u64)> = idx.iter().map(|&i| (fb.col_of[i], fb.coef_of[i])).collect();
    v.sort_unstable();
    let mut merged: Vec<(usize, u64)> = Vec::new();
    for (c, k) in v {
        match merged.last_mut() {
            Some((lc, lk)) if *lc == c => *lk = (*lk + k) % r,
            _ => merged.push((c, k)),
        }
    }
    merged.retain(|&(_, k)| k != 0);
    let mut h = DefaultHasher::new();
    merged.hash(&mut h);
    h.finish()
}

/// Every multiset of `m` points of `fb` summing into `G \ {O}`.
fn census(inst: &BinaryInstance, fb: &FactorBase<FastPoint>, cls: &Classes, m: usize) -> Census {
    let fast = &inst.fast;
    let r = inst.r;
    let cl: Vec<u32> = fb.points.iter().map(|&p| cls.of(fast, r, p)).collect();
    let mut bucket: Vec<Vec<usize>> = vec![Vec::new(); cls.size];
    for (i, &c) in cl.iter().enumerate() {
        bucket[c as usize].push(i);
    }
    let mut targets: HashSet<u64> = HashSet::new();
    let mut rows: HashSet<(u64, u64)> = HashSet::new();
    let mut stack: Vec<usize> = Vec::with_capacity(m);
    #[allow(clippy::too_many_arguments)]
    fn rec(
        fb: &FactorBase<FastPoint>,
        fast: &FastCurve,
        cls: &Classes,
        cl: &[u32],
        bucket: &[Vec<usize>],
        m: usize,
        r: u64,
        start: usize,
        sum: FastPoint,
        csum: u32,
        stack: &mut Vec<usize>,
        out: &mut (u64, &mut HashSet<u64>, &mut HashSet<(u64, u64)>),
    ) {
        let f = fb.points.len();
        if stack.len() + 1 == m {
            let need = cls.neg[csum as usize] as usize;
            let b = &bucket[need];
            let from = b.partition_point(|&k| k < start);
            for &k in &b[from..] {
                let s = fast.add(sum, fb.points[k]);
                if s.infinity {
                    continue;
                }
                stack.push(k);
                out.0 += 1;
                out.1.insert(s.pack());
                out.2.insert((s.pack(), row_hash(fb, stack, r)));
                stack.pop();
            }
            return;
        }
        for i in start..f {
            stack.push(i);
            let c = cls.add[csum as usize * cls.size + cl[i] as usize];
            rec(
                fb,
                fast,
                cls,
                cl,
                bucket,
                m,
                r,
                i,
                fast.add(sum, fb.points[i]),
                c,
                stack,
                out,
            );
            stack.pop();
        }
    }
    let mut out = (0u64, &mut targets, &mut rows);
    rec(
        fb,
        fast,
        cls,
        &cl,
        &bucket,
        m,
        r,
        0,
        FastPoint::INFINITY,
        0,
        &mut stack,
        &mut out,
    );
    Census {
        reps: out.0,
        targets: targets.len() as u64,
        rows: rows.len() as u64,
    }
}

// ── the library oracle, counted ────────────────────────────────────

#[derive(Clone, Copy)]
enum Sweep<'a> {
    /// `decompose_mitm_frobenius` (m ≤ 3); for m = 4 every pair `i ≤ j`.
    Library,
    /// Only these indices are swept (one per `{P, P + T}`).
    Reps(&'a [usize]),
}

#[derive(Clone, Copy, Default)]
struct PerTarget {
    hit: bool,
    adds: u64,
    lookups: u64,
    canons: u64,
}

struct OracleArm<'a> {
    fb: &'a FactorBase<FastPoint>,
    table: &'a FrobeniusPairTable,
    sweep: Sweep<'a>,
    /// Quotient arm: map the target by φ and lift relations through this.
    lift: Option<(&'a [usize], &'a FactorBase<FastPoint>)>,
}

fn run_oracle(
    inst: &BinaryInstance,
    arm: &OracleArm,
    m: usize,
    targets: &[FastPoint],
    verify_fail: &mut u64,
) -> Vec<PerTarget> {
    let g = BinaryGroup(&inst.fast);
    let fb = arm.fb;
    let t_pt = inst.points_with_x(0)[0];
    let mut out = Vec::with_capacity(targets.len());
    for &r0 in targets {
        let mut ops = GroupOps::default();
        let mut ctr = OracleCounters::default();
        let target = if arm.lift.is_some() {
            ops.adds += 1; // one evaluation of φ, charged as one addition
            phi(&inst.fast.field, 1, r0)
        } else {
            r0
        };
        let rel: Option<Vec<usize>> = match (arm.sweep, m) {
            (Sweep::Library, 2 | 3) => {
                decompose_mitm_frobenius(&g, fb, arm.table, m as u32, &mut ops, &mut ctr, target)
            }
            (Sweep::Library, 4) => {
                let mut found = None;
                'outer: for i in 0..fb.points.len() {
                    let s1 = g.add(&mut ops, target, g.neg(fb.points[i]));
                    for j in i..fb.points.len() {
                        let s = g.add(&mut ops, s1, g.neg(fb.points[j]));
                        if g.is_identity(&s) {
                            continue;
                        }
                        if let Some((k, l)) = arm.table.probe(&g, fb, &mut ops, &mut ctr, s) {
                            found = Some(vec![i, j, k, l]);
                            break 'outer;
                        }
                    }
                }
                found
            }
            (Sweep::Reps(_), 2) => arm
                .table
                .probe(&g, fb, &mut ops, &mut ctr, target)
                .map(|(i, j)| vec![i, j]),
            (Sweep::Reps(reps), 3) => {
                let mut found = None;
                for &i in reps {
                    let s = g.add(&mut ops, target, g.neg(fb.points[i]));
                    if g.is_identity(&s) {
                        continue;
                    }
                    if let Some((j, k)) = arm.table.probe(&g, fb, &mut ops, &mut ctr, s) {
                        found = Some(vec![i, j, k]);
                        break;
                    }
                }
                found
            }
            (Sweep::Reps(reps), 4) => {
                let mut found = None;
                'outer4: for (ii, &i) in reps.iter().enumerate() {
                    let s1 = g.add(&mut ops, target, g.neg(fb.points[i]));
                    for &j in &reps[ii..] {
                        let s = g.add(&mut ops, s1, g.neg(fb.points[j]));
                        if g.is_identity(&s) {
                            continue;
                        }
                        if let Some((k, l)) = arm.table.probe(&g, fb, &mut ops, &mut ctr, s) {
                            found = Some(vec![i, j, k, l]);
                            break 'outer4;
                        }
                    }
                }
                found
            }
            _ => unreachable!(),
        };
        // Verification (not charged): the relation re-sums to the target;
        // a lifted relation re-sums to R or R + T on the domain.
        if let Some(idx) = &rel {
            let mut s = FastPoint::INFINITY;
            match arm.lift {
                None => {
                    for &i in idx {
                        s = inst.fast.add(s, fb.points[i]);
                    }
                    if s != target {
                        *verify_fail += 1;
                    }
                }
                Some((lift, dom)) => {
                    for &i in idx {
                        s = inst.fast.add(s, dom.points[lift[i]]);
                    }
                    if s != r0 && s != inst.fast.add(r0, t_pt) {
                        *verify_fail += 1;
                    }
                }
            }
        }
        out.push(PerTarget {
            hit: rel.is_some(),
            adds: ops.adds + ops.doubles,
            lookups: ctr.lookups,
            canons: ctr.canonicalisations,
        });
    }
    out
}

fn summarize(v: &[PerTarget]) -> (f64, f64, f64, f64) {
    let t = v.len() as f64;
    let hits = v.iter().filter(|p| p.hit).count() as f64;
    let adds = v.iter().map(|p| p.adds).sum::<u64>() as f64;
    let look = v.iter().map(|p| p.lookups).sum::<u64>() as f64;
    let can = v.iter().map(|p| p.canons).sum::<u64>() as f64;
    (hits / t, adds / t, look / t, can / t)
}

/// Total relation-collection cost `U · W / H` with `W = adds + lookups`.
fn total(u: usize, v: &[PerTarget], idx: &[usize]) -> Option<f64> {
    let (mut h, mut w) = (0u64, 0u64);
    for &i in idx {
        h += v[i].hit as u64;
        w += v[i].adds + v[i].lookups;
    }
    (h > 0).then(|| u as f64 * w as f64 / h as f64)
}

/// Paired bootstrap of `total(arm) / total(ref)` over targets.
fn paired_ratio(
    u_arm: usize,
    arm: &[PerTarget],
    u_ref: usize,
    rf: &[PerTarget],
    rng: &mut StdRng,
    reps: usize,
) -> Value {
    let all: Vec<usize> = (0..arm.len()).collect();
    let point = match (total(u_arm, arm, &all), total(u_ref, rf, &all)) {
        (Some(x), Some(y)) => x / y,
        _ => return Value::Null,
    };
    let mut rs = Vec::with_capacity(reps);
    for _ in 0..reps {
        let idx: Vec<usize> = (0..arm.len())
            .map(|_| rng.gen_range(0..arm.len()))
            .collect();
        if let (Some(x), Some(y)) = (total(u_arm, arm, &idx), total(u_ref, rf, &idx)) {
            rs.push(x / y);
        }
    }
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| rs[((rs.len() as f64 - 1.0) * p).round() as usize];
    json!([point, q(0.025), q(0.975)])
}

fn random_targets(inst: &BinaryInstance, count: usize, rng: &mut StdRng) -> Vec<FastPoint> {
    (0..count)
        .map(|_| {
            let k = rng.gen_range(1..inst.r);
            inst.fast.mul_u64(inst.generator, k)
        })
        .collect()
}

fn sweep_size(m: usize, f: usize) -> f64 {
    match m {
        2 => 1.0,
        3 => f as f64,
        _ => (f * (f + 1) / 2) as f64,
    }
}

// ── experiments ────────────────────────────────────────────────────

struct ArmSpec<'a> {
    name: &'static str,
    fb: &'a FactorBase<FastPoint>,
    table: &'a FrobeniusPairTable,
    sweep: Sweep<'a>,
    lift: Option<(&'a [usize], &'a FactorBase<FastPoint>)>,
    census_fb: &'a FactorBase<FastPoint>,
    /// Size of the swept set (the census cost model's `L_m` input).
    swept: usize,
    /// The domain base size this arm is charged against.
    base_points: usize,
}

#[allow(clippy::too_many_arguments)]
fn run_arms(
    inst: &BinaryInstance,
    arms: &[ArmSpec],
    cls: &Classes,
    m: usize,
    targets: &[FastPoint],
    rng: &mut StdRng,
    boot: usize,
    reference: usize,
) -> Value {
    let r = inst.r as f64;
    let mut census_rows: Vec<Census> = Vec::new();
    let mut per: Vec<Vec<PerTarget>> = Vec::new();
    let mut out = Vec::new();
    let mut fails_all = 0u64;
    for arm in arms {
        let c = census(inst, arm.census_fb, cls, m);
        let mut fails = 0u64;
        let v = if targets.is_empty() {
            Vec::new()
        } else {
            run_oracle(
                inst,
                &OracleArm {
                    fb: arm.fb,
                    table: arm.table,
                    sweep: arm.sweep,
                    lift: arm.lift,
                },
                m,
                targets,
                &mut fails,
            )
        };
        fails_all += fails;
        census_rows.push(c);
        per.push(v);
    }
    let c_ref = census_rows[reference];
    let cost_census = |i: usize| -> f64 {
        let c = census_rows[i];
        arms[i].fb.columns as f64 * sweep_size(m, arms[i].swept) * r / c.targets.max(1) as f64
    };
    let cost_census_rows = |i: usize| -> f64 {
        let c = census_rows[i];
        arms[i].fb.columns as f64 * sweep_size(m, arms[i].swept) * r / c.rows.max(1) as f64
    };
    for (i, arm) in arms.iter().enumerate() {
        let c = census_rows[i];
        let fsz = arm.base_points as f64;
        let mf: f64 = (1..=m).map(|k| k as f64).product();
        let class_factor = c.reps as f64 / (fsz.powi(m as i32) / mf);
        let (hit, adds, look, can) = if per[i].is_empty() {
            (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
        } else {
            summarize(&per[i])
        };
        let oracle_total = if per[i].is_empty() {
            None
        } else {
            total(
                arm.fb.columns,
                &per[i],
                &(0..per[i].len()).collect::<Vec<_>>(),
            )
        };
        let ratio = if per[i].is_empty() {
            Value::Null
        } else {
            paired_ratio(
                arm.fb.columns,
                &per[i],
                arms[reference].fb.columns,
                &per[reference],
                rng,
                boot,
            )
        };
        out.push(json!({
            "arm": arm.name,
            "base_points": arm.base_points,
            "oracle_base_points": arm.fb.points.len(),
            "columns_U": arm.fb.columns,
            "swept": arm.swept,
            "table_entries": arm.table.entries,
            "table_build_adds": arm.table.build_ops.adds,
            "census": {
                "reps_into_G": c.reps,
                "decomposable_targets_D": c.targets,
                "distinct_rows": c.rows,
                "rows_per_target": c.rows as f64 / r,
                "hit_probability_D_over_r": c.targets as f64 / r,
                "reps_per_row": c.reps as f64 / c.rows.max(1) as f64,
                "class_factor_reps": class_factor,
                "D_ratio_vs_ref": c.targets as f64 / c_ref.targets.max(1) as f64,
                "rows_ratio_vs_ref": c.rows as f64 / c_ref.rows.max(1) as f64,
                "cost_D": cost_census(i),
                "cost_D_ratio_vs_ref": cost_census(i) / cost_census(reference),
                "cost_rows_ratio_vs_ref": cost_census_rows(i) / cost_census_rows(reference),
            },
            "oracle": {
                "targets": per[i].len(),
                "hit_rate": hit,
                "adds_per_target": adds,
                "lookups_per_target": look,
                "canonicalisations_per_target": can,
                "work_per_relation": (adds + look) / hit,
                "total_U_W_over_H": oracle_total,
                "total_ratio_vs_ref_boot95": ratio,
            },
        }));
    }
    json!({"m": m, "arms": out, "verify_failures": fails_all})
}

fn grid(args: &Args) -> Value {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let mut cells = Vec::new();
    for &(n, a) in &args.curves {
        let Some(inst) = koblitz_instance(a, n) else {
            eprintln!("no instance n={n} a={a}");
            continue;
        };
        let kc = inst.koblitz.as_ref().unwrap();
        eprintln!(
            "curve n={n} a={a} {} N={} r={} h={}",
            kc.label(),
            inst.group_order,
            inst.r,
            inst.cofactor
        );
        for &m in &args.ms {
            // Base size targeting an expected hit rate of `args.fill` for arm a.
            let nn = inst.group_order as f64;
            let mf: f64 = (1..=m).map(|k| k as f64).product();
            let want = (args.fill * mf * nn).powf(1.0 / m as f64);
            let k = ((want / (4.0 * n as f64)).round() as usize).max(1);
            for seed in 0..args.seeds {
                let t0 = Instant::now();
                let rb = random_bases(&inst, k, &mut rng);
                let (c1, lift) = quotient_base(&inst, &rb.b.fb);
                let reps = translate_reps(&inst, &rb.b.fb);
                let mut all_pts = rb.a.fb.points.clone();
                all_pts.extend(&rb.b.fb.points);
                all_pts.extend(&rb.d.fb.points);
                all_pts.extend(&c1.fb.points);
                let cls = Classes::new(&inst.fast, inst.r, &all_pts);
                let ta = FrobeniusPairTable::build(&inst, &rb.a.fb).expect("table a");
                let tb = FrobeniusPairTable::build(&inst, &rb.b.fb).expect("table b");
                let tc = FrobeniusPairTable::build(&inst, &c1.fb).expect("table c1");
                let td = FrobeniusPairTable::build(&inst, &rb.d.fb).expect("table d");
                let f = rb.a.fb.points.len();
                let arms = [
                    ArmSpec {
                        name: "a_plain",
                        fb: &rb.a.fb,
                        table: &ta,
                        sweep: Sweep::Library,
                        lift: None,
                        census_fb: &rb.a.fb,
                        swept: f,
                        base_points: f,
                    },
                    ArmSpec {
                        name: "b_saturated_sweep_F",
                        fb: &rb.b.fb,
                        table: &tb,
                        sweep: Sweep::Library,
                        lift: None,
                        census_fb: &rb.b.fb,
                        swept: f,
                        base_points: f,
                    },
                    ArmSpec {
                        name: "c1_quotient_E_over_T",
                        fb: &c1.fb,
                        table: &tc,
                        sweep: Sweep::Library,
                        lift: Some((&lift, &rb.b.fb)),
                        census_fb: &c1.fb,
                        swept: c1.fb.points.len(),
                        base_points: f,
                    },
                    ArmSpec {
                        name: "c2_halfsweep_on_E",
                        fb: &rb.b.fb,
                        table: &tb,
                        sweep: Sweep::Reps(&reps),
                        lift: None,
                        census_fb: &rb.b.fb,
                        swept: reps.len(),
                        base_points: f,
                    },
                    ArmSpec {
                        name: "d_image_2E",
                        fb: &rb.d.fb,
                        table: &td,
                        sweep: Sweep::Library,
                        lift: None,
                        census_fb: &rb.d.fb,
                        swept: f,
                        base_points: f,
                    },
                ];
                let ntarg = if m == 4 { args.targets4 } else { args.targets };
                let targets = random_targets(&inst, ntarg, &mut rng);
                let res = run_arms(&inst, &arms, &cls, m, &targets, &mut rng, args.boot, 0);
                eprintln!(
                    "  m={m} seed={seed} |F|={f} k={k} U=({},{},{},{}) {:.1}s",
                    rb.a.fb.columns,
                    rb.b.fb.columns,
                    c1.fb.columns,
                    rb.d.fb.columns,
                    t0.elapsed().as_secs_f64()
                );
                cells.push(json!({
                    "n": n, "a": a, "curve": kc.label(),
                    "N": inst.group_order, "r": inst.r, "h": inst.cofactor,
                    "m": m, "seed": seed, "k_parent_orbits": rb.b_parent_orbits,
                    "base_points": f, "quotient_points": c1.fb.points.len(),
                    "translate_reps": reps.len(), "class_group_size": cls.size,
                    "result": res,
                }));
            }
        }
    }
    json!({"experiment": "grid", "seed": args.seed, "fill": args.fill, "cells": cells})
}

fn validate(args: &Args) -> Value {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let mut out = Vec::new();
    for &(n, a) in &args.curves {
        if n > 17 {
            continue;
        }
        if koblitz_instance(a, n).is_none() {
            eprintln!("validate n={n} a={a}: no Koblitz instance (r too small), skipped");
            continue;
        }
        eprintln!("validate n={n} a={a}");
        out.push(validate_curve(n, a, 1, &mut rng));
        // Two non-Koblitz models b ∉ F_2 to check the general formula.
        for _ in 0..2 {
            let b = loop {
                let b = rng.gen::<u64>() & mask(n);
                if b > 1 {
                    break b;
                }
            };
            out.push(validate_curve(n, a, b, &mut rng));
        }
    }
    json!({"experiment": "validate", "curves": out})
}

fn a1(args: &Args) -> Value {
    let mut rng = StdRng::seed_from_u64(args.seed);
    let inst = koblitz_instance(0, 31).expect("n = 31 instance");
    let kc = inst.koblitz.as_ref().unwrap();
    eprintln!(
        "A1 curve {} N={} r={} h={}",
        kc.label(),
        inst.group_order,
        inst.r,
        inst.cofactor
    );
    let x = build(
        &inst,
        FactorBaseSpec::Divisor {
            indices: vec![0, 1, 2],
        }
        .materialize(kc)
        .expect("x base"),
        "x: koblitz-orbit divisor 0;1;2",
    );
    let sym = build_symmetrised_factor_base(kc, &[0, 1, 2]).expect("u base");
    let u = build(
        &inst,
        frobenius_view_of_symmetrised(kc, &sym).expect("u frobenius view"),
        "u: symmetrised divisor 0;1;2",
    );
    let tz = build(
        &inst,
        FactorBaseSpec::Divisor {
            indices: vec![1, 2],
        }
        .materialize(kc)
        .expect("tz base"),
        "tz: koblitz-trace-zero divisor 1;2",
    );
    let (c1, lift) = quotient_base(&inst, &u.fb);
    let reps = translate_reps(&inst, &u.fb);
    // Overlap of φ(F_u) with the trace-zero base.
    let tz_x: HashSet<u64> = tz.fb.points.iter().map(|p| p.x).collect();
    let c1_x: HashSet<u64> = c1.fb.points.iter().map(|p| p.x).collect();
    let overlap = c1_x.intersection(&tz_x).count();
    // x ↦ 1/x is translation by T: is φ(F_u) the T-translate of the
    // trace-zero base?
    let inv_overlap = c1_x
        .iter()
        .filter(|&&x| tz_x.contains(&inst.fast.field.inv(x)))
        .count();
    let u_closed = {
        let t = inst.points_with_x(0)[0];
        u.fb.points
            .iter()
            .all(|&p| u.fb.point_index.contains_key(&inst.fast.add(p, t).pack()))
    };
    let mut all_pts = x.fb.points.clone();
    all_pts.extend(&u.fb.points);
    all_pts.extend(&tz.fb.points);
    all_pts.extend(&c1.fb.points);
    let cls = Classes::new(&inst.fast, inst.r, &all_pts);
    let tx = FrobeniusPairTable::build(&inst, &x.fb).unwrap();
    let tu = FrobeniusPairTable::build(&inst, &u.fb).unwrap();
    let tc = FrobeniusPairTable::build(&inst, &c1.fb).unwrap();
    let tt = FrobeniusPairTable::build(&inst, &tz.fb).unwrap();
    let fu = u.fb.points.len();
    let arms = [
        ArmSpec {
            name: "x_proj",
            fb: &x.fb,
            table: &tx,
            sweep: Sweep::Library,
            lift: None,
            census_fb: &x.fb,
            swept: x.fb.points.len(),
            base_points: x.fb.points.len(),
        },
        ArmSpec {
            name: "u_proj_sweep_F",
            fb: &u.fb,
            table: &tu,
            sweep: Sweep::Library,
            lift: None,
            census_fb: &u.fb,
            swept: fu,
            base_points: fu,
        },
        ArmSpec {
            name: "u_c1_quotient",
            fb: &c1.fb,
            table: &tc,
            sweep: Sweep::Library,
            lift: Some((&lift, &u.fb)),
            census_fb: &c1.fb,
            swept: c1.fb.points.len(),
            base_points: fu,
        },
        ArmSpec {
            name: "u_c2_halfsweep",
            fb: &u.fb,
            table: &tu,
            sweep: Sweep::Reps(&reps),
            lift: None,
            census_fb: &u.fb,
            swept: reps.len(),
            base_points: fu,
        },
        ArmSpec {
            name: "tz_proj",
            fb: &tz.fb,
            table: &tt,
            sweep: Sweep::Library,
            lift: None,
            census_fb: &tz.fb,
            swept: tz.fb.points.len(),
            base_points: tz.fb.points.len(),
        },
    ];
    let mut results = Vec::new();
    for &m in &args.ms {
        if m > 3 {
            continue;
        }
        let ntarg = if m == 2 {
            args.targets * 50
        } else {
            args.targets
        };
        let t0 = Instant::now();
        let targets = random_targets(&inst, ntarg, &mut rng);
        let res = run_arms(&inst, &arms, &cls, m, &targets, &mut rng, args.boot, 0);
        eprintln!("  m={m} {:.1}s", t0.elapsed().as_secs_f64());
        results.push(res);
    }
    json!({
        "experiment": "a1", "curve": kc.label(), "N": inst.group_order, "r": inst.r,
        "h": inst.cofactor, "seed": args.seed,
        "x_points": x.fb.points.len(), "u_points": fu, "tz_points": tz.fb.points.len(),
        "c1_points": c1.fb.points.len(), "u_translate_reps": reps.len(),
        "u_is_T_closed": u_closed,
        "c1_abscissae": c1_x.len(), "tz_abscissae": tz_x.len(),
        "c1_tz_abscissa_overlap": overlap,
        "c1_inverse_abscissae_in_tz": inv_overlap,
        "columns": {"x": x.fb.columns, "u": u.fb.columns, "c1": c1.fb.columns, "tz": tz.fb.columns},
        "class_group_size": cls.size,
        "results": results,
    })
}

// ── cross-seed summary (reads the grid JSON files) ─────────────────

/// Per-seed raw quantities of one arm, enough to pool across seeds.
#[derive(Clone, Copy)]
struct SeedArm {
    u: f64,
    sweep: f64,
    d: f64,
    rows: f64,
    hits: f64,
    work: f64,
    targets: f64,
}

fn seed_arm(cell: &Value, arm: &Value) -> SeedArm {
    let m = cell["m"].as_u64().unwrap() as usize;
    let o = &arm["oracle"];
    let t = o["targets"].as_f64().unwrap_or(0.0);
    let hit = o["hit_rate"].as_f64().unwrap_or(0.0);
    let w = o["adds_per_target"].as_f64().unwrap_or(0.0)
        + o["lookups_per_target"].as_f64().unwrap_or(0.0);
    SeedArm {
        u: arm["columns_U"].as_f64().unwrap(),
        sweep: sweep_size(m, arm["swept"].as_u64().unwrap() as usize),
        d: arm["census"]["decomposable_targets_D"].as_f64().unwrap(),
        rows: arm["census"]["distinct_rows"].as_f64().unwrap(),
        hits: hit * t,
        work: w * t,
        targets: t,
    }
}

/// Pooled cost ratios of arm `x` to arm `a` over the given seeds:
/// census (via D and via rows) and oracle (U · ΣW / ΣH).
fn pooled(xs: &[SeedArm], a: &[SeedArm], idx: &[usize], phi_charge: bool) -> [f64; 4] {
    let agg = |v: &[SeedArm]| {
        let (mut u, mut sw, mut d, mut rw, mut h, mut w) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for &i in idx {
            u += v[i].u;
            sw += v[i].sweep;
            d += v[i].d;
            rw += v[i].rows;
            h += v[i].hits;
            w += v[i].work;
        }
        let t: f64 = idx.iter().map(|&i| v[i].targets).sum();
        (u * sw / d, u * sw / rw, u * w / h, u * (w - t) / h)
    };
    let (x1, x2, x3, x4) = agg(xs);
    let (a1, a2, a3, _) = agg(a);
    // The φ charge is one addition per target, on the quotient arm only.
    [
        x1 / a1,
        x2 / a2,
        x3 / a3,
        if phi_charge { x4 / a3 } else { x3 / a3 },
    ]
}

fn summarize_grid(args: &Args) -> Value {
    let mut rng = StdRng::seed_from_u64(args.seed);
    // (n, a, m) -> arm name -> per-seed rows
    let mut groups: Vec<((u64, u64, u64), Vec<(String, Vec<SeedArm>)>, Value)> = Vec::new();
    for path in &args.inputs {
        let v: Value = serde_json::from_str(&fs::read_to_string(path).expect("read")).unwrap();
        for cell in v["cells"].as_array().unwrap() {
            let key = (
                cell["n"].as_u64().unwrap(),
                cell["a"].as_u64().unwrap(),
                cell["m"].as_u64().unwrap(),
            );
            let pos = match groups.iter().position(|g| g.0 == key) {
                Some(p) => p,
                None => {
                    groups.push((
                        key,
                        Vec::new(),
                        json!({"curve": cell["curve"], "r": cell["r"], "h": cell["h"],
                               "base_points": cell["base_points"]}),
                    ));
                    groups.len() - 1
                }
            };
            for arm in cell["result"]["arms"].as_array().unwrap() {
                let name = arm["arm"].as_str().unwrap().to_string();
                let sa = seed_arm(cell, arm);
                let g = &mut groups[pos].1;
                match g.iter_mut().find(|(nm, _)| *nm == name) {
                    Some((_, v)) => v.push(sa),
                    None => g.push((name, vec![sa])),
                }
            }
        }
    }
    let predicted = |arm: &str, m: u64| -> f64 {
        let k = 2f64.powi(m as i32 - 2);
        match arm {
            "a_plain" => 1.0,
            "b_saturated_sweep_F" => k,
            "d_image_2E" => 0.5,
            _ => 1.0,
        }
    };
    let mut out = Vec::new();
    for ((n, a, m), arms, meta) in &groups {
        let reference = &arms.iter().find(|(nm, _)| nm == "a_plain").unwrap().1;
        let s = reference.len();
        let mut rows = Vec::new();
        for (name, v) in arms {
            let all: Vec<usize> = (0..s).collect();
            let quotient = name.starts_with("c1");
            let point = pooled(v, reference, &all, quotient);
            let mut boots: [Vec<f64>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
            for _ in 0..args.boot {
                let idx: Vec<usize> = (0..s).map(|_| rng.gen_range(0..s)).collect();
                let p = pooled(v, reference, &idx, quotient);
                for k in 0..4 {
                    if p[k].is_finite() {
                        boots[k].push(p[k]);
                    }
                }
            }
            let ci = |b: &mut Vec<f64>| -> Value {
                if b.is_empty() {
                    return Value::Null;
                }
                b.sort_by(|x, y| x.partial_cmp(y).unwrap());
                let q = |p: f64| b[((b.len() as f64 - 1.0) * p).round() as usize];
                json!([q(0.025), q(0.975)])
            };
            let hit_rate: f64 = v.iter().map(|x| x.hits).sum::<f64>()
                / v.iter().map(|x| x.targets).sum::<f64>().max(1.0);
            let mean_u = v.iter().map(|x| x.u).sum::<f64>() / s as f64;
            let rows_per_t = v.iter().map(|x| x.rows).sum::<f64>() / s as f64;
            rows.push(json!({
                "arm": name,
                "predicted_cost_ratio": predicted(name, *m),
                "census_cost_ratio_D": point[0], "census_cost_ratio_D_ci95_seeds": ci(&mut boots[0]),
                "census_cost_ratio_rows": point[1], "census_cost_ratio_rows_ci95_seeds": ci(&mut boots[1]),
                "oracle_cost_ratio": point[2], "oracle_cost_ratio_ci95_seeds": ci(&mut boots[2]),
                "oracle_cost_ratio_excl_phi": point[3], "oracle_cost_ratio_excl_phi_ci95_seeds": ci(&mut boots[3]),
                "mean_columns_U": mean_u,
                "mean_distinct_rows": rows_per_t,
                "pooled_oracle_hit_rate": hit_rate,
            }));
        }
        out.push(json!({"n": n, "a": a, "m": m, "seeds": s, "meta": meta, "arms": rows}));
    }
    json!({"experiment": "summary", "inputs": args.inputs, "groups": out})
}

// ── CLI ────────────────────────────────────────────────────────────

struct Args {
    exp: String,
    curves: Vec<(u32, u8)>,
    ms: Vec<usize>,
    seeds: usize,
    targets: usize,
    targets4: usize,
    boot: usize,
    fill: f64,
    seed: u64,
    json: Option<String>,
    inputs: Vec<String>,
}

fn parse() -> Args {
    let mut a = Args {
        exp: "grid".into(),
        curves: vec![
            (13, 0),
            (13, 1),
            (15, 0),
            (15, 1),
            (17, 0),
            (17, 1),
            (19, 0),
            (19, 1),
            (21, 0),
            (21, 1),
            (23, 0),
            (23, 1),
        ],
        ms: vec![2, 3, 4],
        seeds: 6,
        targets: 2000,
        targets4: 600,
        boot: 1000,
        fill: 0.1,
        seed: 20261012,
        json: None,
        inputs: Vec::new(),
    };
    let v: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;
    while i < v.len() {
        let val = v.get(i + 1).cloned().unwrap_or_default();
        match v[i].as_str() {
            "--exp" => a.exp = val,
            "--curves" => {
                a.curves = val
                    .split(',')
                    .map(|c| {
                        let (n, aa) = c.split_once(':').expect("n:a");
                        (n.parse().unwrap(), aa.parse().unwrap())
                    })
                    .collect()
            }
            "--ms" => a.ms = val.split(',').map(|x| x.parse().unwrap()).collect(),
            "--seeds" => a.seeds = val.parse().unwrap(),
            "--targets" => a.targets = val.parse().unwrap(),
            "--targets4" => a.targets4 = val.parse().unwrap(),
            "--boot" => a.boot = val.parse().unwrap(),
            "--fill" => a.fill = val.parse().unwrap(),
            "--seed" => a.seed = val.parse().unwrap(),
            "--json" => a.json = Some(val),
            "--inputs" => a.inputs = val.split(',').map(String::from).collect(),
            other => panic!("unknown flag {other}"),
        }
        i += 2;
    }
    a
}

fn main() {
    let args = parse();
    let t0 = Instant::now();
    let mut out = match args.exp.as_str() {
        "validate" => validate(&args),
        "grid" => grid(&args),
        "a1" => a1(&args),
        "summarize" => summarize_grid(&args),
        e => panic!("unknown experiment {e}"),
    };
    out["wall_seconds_contended"] = json!(t0.elapsed().as_secs_f64());
    let s = serde_json::to_string_pretty(&out).unwrap();
    match &args.json {
        Some(p) => fs::write(p, s).expect("write json"),
        None => println!("{s}"),
    }
}
