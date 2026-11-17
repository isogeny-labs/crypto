//! Round-7 R7.2: is any toy binary elliptic curve weak under GHS, end to end?
//!
//! Library: `crypto_lib::cryptanalysis::ghs_ic` (factor base, relation
//! collection, linear algebra mod ell, extraction) on the real descended
//! Jacobian of `crypto_lib::cryptanalysis::ghs_smooth`.  This example plants
//! discrete logs on E(K), solves each with the index-calculus attack, with
//! table Pollard rho and with baby-step giant-step on E, verifies every answer
//! on E, and prices everything in `bitop/v1` (AGENTS.md; `docs/units/BITOPS.md`).
//!
//! Modes (JSON on `--out`, else stdout):
//!   run     --inst l,n,a,beta [--inst ...] [--tune 10] [--logs 20] [--steps 8]
//!           [--scan 1,2,4,...] [--budget P] [--seed S]
//!   pick    L N [--magic M] [--count C] [--max-cofactor H] [--samples S] [--seed S]
//!   profile L N [--magic M] [--maxk K]       time the phases of R7.1's run_curve
//!   selftest                                 rho / BSGS / pricing self-checks

#![recursion_limit = "1024"]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::env;
use std::fs;
use std::time::Instant;

use crypto_lib::binary_ecc::operation_tally::OperationTally;
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly, MumfordDivisor};
use crypto_lib::cryptanalysis::bitops::{Field, Prim, Widths};
use crypto_lib::cryptanalysis::descent_expansion::enumerate_irreducibles;
use crypto_lib::cryptanalysis::ec_trapdoor::{ghs_type_i, magic_number_full, FieldTower};
use crypto_lib::cryptanalysis::ghs_descent::{ECurve, Pt};
use crypto_lib::cryptanalysis::ghs_ic::{
    affine_points, solve_dlp, FactorBase, IcConfig, RelMode, Rng64,
};
use crypto_lib::cryptanalysis::ghs_smooth::{
    count_e_points, count_points_extension, lift_x, SmoothCover,
};
use num_bigint::BigUint;
use serde_json::{json, Value};

// ── small number theory ────────────────────────────────────────────────

fn factor(mut x: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= x {
        if x.is_multiple_of(p) {
            let mut e = 0;
            while x.is_multiple_of(p) {
                x /= p;
                e += 1;
            }
            out.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        out.push((x, 1));
    }
    out
}

fn bits_of(x: u64) -> u32 {
    64 - x.leading_zeros()
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

// ── fast #E(F_{2^N}) for N <= 32 (own bit arithmetic; cross-checked in selftest) ──

fn fmul(mut a: u64, mut b: u64, irr: u64, n: u32) -> u64 {
    let mut r = 0u64;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 != 0 {
            a ^= irr;
        }
    }
    r
}

fn fpow(mut a: u64, mut e: u64, irr: u64, n: u32) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = fmul(r, a, irr, n);
        }
        a = fmul(a, a, irr, n);
        e >>= 1;
    }
    r
}

fn irr_mask(irr: &IrreduciblePoly) -> u64 {
    let mut m = 1u64 << irr.degree;
    for &t in &irr.low_terms {
        m |= 1u64 << t;
    }
    m
}

/// `#E` for `y^2 + xy = x^3 + a x^2 + beta^2` over `F_{2^n}`: the number of
/// `x != 0` with `Tr(x + a + beta/x) = 0` (since `Tr(b/x^2) = Tr(beta/x)`),
/// times 2, plus `O` and `(0, beta)`.
fn fast_count_e(irr: &IrreduciblePoly, a: u64, beta: u64) -> u64 {
    let n = irr.degree;
    let m = irr_mask(irr);
    // trace mask: bit i = Tr(z^i)
    let mut tmask = 0u64;
    for i in 0..n {
        let mut acc = 1u64 << i;
        let mut cur = acc;
        for _ in 1..n {
            cur = fmul(cur, cur, m, n);
            acc ^= cur;
        }
        debug_assert!(acc <= 1);
        if acc & 1 == 1 {
            tmask |= 1u64 << i;
        }
    }
    let tr = |c: u64| ((c & tmask).count_ones() & 1) as u64;
    let tra = tr(a);
    let mut cnt = 2u64;
    let total = 1u64 << n;
    let chunk = 1024u64;
    let mut start = 1u64;
    while start < total {
        let end = (start + chunk).min(total);
        let xs: Vec<u64> = (start..end).collect();
        let mut pre = Vec::with_capacity(xs.len());
        let mut acc = 1u64;
        for &x in &xs {
            acc = fmul(acc, x, m, n);
            pre.push(acc);
        }
        let mut inv_all = fpow(acc, (1u64 << n) - 2, m, n);
        for i in (0..xs.len()).rev() {
            let before = if i == 0 { 1 } else { pre[i - 1] };
            let inv_x = fmul(inv_all, before, m, n);
            inv_all = fmul(inv_all, xs[i], m, n);
            let c = tr(xs[i]) ^ tra ^ tr(fmul(beta, inv_x, m, n));
            if c == 0 {
                cnt += 2;
            }
        }
        start = end;
    }
    cnt
}

// ── bitop/v1 pricing ───────────────────────────────────────────────────

fn gates(field_n: u32, r_bits: u32, p: Prim) -> u128 {
    Widths {
        field: Field::Binary { n: field_n },
        r_bits,
    }
    .gates(p)
    .expect("priced primitive")
}

/// (multiplications, squarings) that the ledger records inside one `flt_inverse`
/// at width m; they are removed before the inversion is priced as `field_inv`.
fn inverse_inner_counts(m: u32) -> (u64, u64) {
    if m < 2 {
        return (0, 0);
    }
    let irr = enumerate_irreducibles(m, 1).remove(0);
    let x = F2mElement::from_words(&[3], m);
    let t = OperationTally::begin();
    let _ = x.flt_inverse(&irr);
    let map = t.finish();
    (
        map.get(&format!("f2m{m}_mul")).copied().unwrap_or(0),
        map.get(&format!("f2m{m}_square")).copied().unwrap_or(0),
    )
}

#[derive(Default, Clone, Debug)]
struct Priced {
    gates: u128,
    /// per field width: (adds, muls, squares, inversions, gates)
    by_width: BTreeMap<u32, (u64, u64, u64, u64, u128)>,
}

fn price_ledger(ops: &BTreeMap<String, u64>, r_bits: u32) -> Priced {
    let mut widths: HashSet<u32> = HashSet::new();
    for k in ops.keys() {
        if let Some(rest) = k.strip_prefix("f2m") {
            if let Some((m, _)) = rest.split_once('_') {
                if let Ok(m) = m.parse::<u32>() {
                    widths.insert(m);
                }
            }
        }
    }
    let mut out = Priced::default();
    for m in widths {
        let get = |op: &str| ops.get(&format!("f2m{m}_{op}")).copied().unwrap_or(0);
        let (add, mul, sq, inv) = (
            get("add"),
            get("mul"),
            get("square"),
            get("inverse_informational"),
        );
        let (mi, si) = inverse_inner_counts(m);
        let mul_net = mul.saturating_sub(inv * mi);
        let sq_net = sq.saturating_sub(inv * si);
        let g = add as u128 * gates(m, r_bits, Prim::FieldAdd)
            + mul_net as u128 * gates(m, r_bits, Prim::FieldMul)
            + sq_net as u128 * gates(m, r_bits, Prim::FieldSqr)
            + inv as u128 * gates(m, r_bits, Prim::FieldInv);
        out.gates += g;
        out.by_width.insert(m, (add, mul_net, sq_net, inv, g));
    }
    out
}

// ── counted operations on E ────────────────────────────────────────────

#[derive(Default, Clone, Copy, Debug)]
struct EcCount {
    adds: u64,
    dbls: u64,
    probes: u64,
    mm: u64,
    ma: u64,
    mi: u64,
}

impl EcCount {
    fn gates(&self, nn: u32, r_bits: u32) -> u128 {
        self.adds as u128 * gates(nn, r_bits, Prim::GroupAdd)
            + self.dbls as u128 * gates(nn, r_bits, Prim::GroupDouble)
            + self.probes as u128 * gates(nn, r_bits, Prim::HashProbe)
            + self.mm as u128 * gates(nn, r_bits, Prim::ModrMul)
            + self.ma as u128 * gates(nn, r_bits, Prim::ModrAdd)
            + self.mi as u128 * gates(nn, r_bits, Prim::ModrInv)
    }
    fn group_ops(&self) -> u64 {
        self.adds + self.dbls
    }
}

struct Ec<'a> {
    c: &'a ECurve,
    n: EcCount,
}

impl<'a> Ec<'a> {
    fn add(&mut self, p: &Pt, q: &Pt) -> Pt {
        match (p, q) {
            (Pt::Inf, _) | (_, Pt::Inf) => {}
            _ => {
                if p == q {
                    self.n.dbls += 1;
                } else {
                    self.n.adds += 1;
                }
            }
        }
        self.c.add(p, q)
    }
    fn dbl(&mut self, p: &Pt) -> Pt {
        if !matches!(p, Pt::Inf) {
            self.n.dbls += 1;
        }
        self.c.double(p)
    }
    /// double-and-add, counting what it performs
    fn smul(&mut self, p: &Pt, k: u64) -> Pt {
        let mut r = Pt::Inf;
        for i in (0..bits_of(k)).rev() {
            r = self.dbl(&r);
            if (k >> i) & 1 == 1 {
                r = self.add(&r, p);
            }
        }
        r
    }
}

fn key_of(p: &Pt) -> u128 {
    match p {
        Pt::Inf => u128::MAX,
        Pt::Aff { x, y } => ((x.raw_bits()[0] as u128) << 64) | y.raw_bits()[0] as u128,
    }
}

fn x_of(p: &Pt) -> u64 {
    match p {
        Pt::Inf => 0,
        Pt::Aff { x, .. } => x.raw_bits()[0],
    }
}

/// Table-based r-adding Pollard rho: every visited point is kept, so the
/// collision is detected at once (the minimum number of group operations a
/// walk of this kind can use; real rho needs distinguished points and does
/// more).  Walk multipliers have at most `min(w, 6)` bits, r = 8.
fn rho(ec: &mut Ec, p: &Pt, q: &Pt, ell: u64, cbits: u32, rng: &mut Rng64) -> (Option<u64>, u64) {
    let cbits = cbits.clamp(2, bits_of(ell).max(2));
    let r = 8usize;
    let cap = 40 * ((ell as f64).sqrt() as u64 + 2);
    let mut steps_total = 0u64;
    for _attempt in 0..200 {
        let mut ms: Vec<(Pt, u64, u64)> = Vec::new();
        for _ in 0..r {
            let a = 1 + rng.below((1u64 << cbits) - 1);
            let b = 1 + rng.below((1u64 << cbits) - 1);
            let ap = ec.smul(p, a);
            let bq = ec.smul(q, b);
            let m = ec.add(&ap, &bq);
            ms.push((m, a, b));
        }
        let j0 = rng.below(r as u64) as usize;
        let (mut x, mut al, mut be) = (ms[j0].0.clone(), ms[j0].1 % ell, ms[j0].2 % ell);
        let mut table: HashMap<u128, (u64, u64)> = HashMap::new();
        for _ in 0..cap {
            steps_total += 1;
            ec.n.probes += 1;
            match table.get(&key_of(&x)) {
                Some(&(a2, b2)) => {
                    if b2 != be {
                        // a2 + b2 d = al + be d  =>  d (b2 - be) = al - a2
                        let num = (al + ell - a2) % ell;
                        let den = (b2 + ell - be) % ell;
                        let d = mulmod(num, powmod(den, ell - 2, ell), ell);
                        ec.n.mi += 1;
                        ec.n.mm += 1;
                        return (Some(d), steps_total);
                    }
                    break;
                }
                None => {
                    table.insert(key_of(&x), (al, be));
                }
            }
            let j = (x_of(&x) % r as u64) as usize;
            x = ec.add(&x, &ms[j].0);
            al = (al + ms[j].1) % ell;
            be = (be + ms[j].2) % ell;
            ec.n.ma += 2;
        }
    }
    (None, steps_total)
}

/// Baby-step giant-step on the order-ell subgroup, m = ceil(sqrt(ell)).
fn bsgs(ec: &mut Ec, p: &Pt, q: &Pt, ell: u64) -> Option<u64> {
    let m = (ell as f64).sqrt().ceil() as u64;
    let mut table: HashMap<u128, u64> = HashMap::new();
    let mut cur = Pt::Inf;
    for j in 0..m {
        ec.n.probes += 1;
        table.entry(key_of(&cur)).or_insert(j);
        cur = ec.add(&cur, p);
    }
    let g = ec.smul(p, m);
    let gneg = ec.c.neg(&g);
    let mut r = q.clone();
    for i in 0..=m {
        ec.n.probes += 1;
        if let Some(&j) = table.get(&key_of(&r)) {
            return Some((i * m + j) % ell);
        }
        r = ec.add(&r, &gneg);
    }
    None
}

/// Cost of the verification `[d]P == Q` (double-and-add on E), identical in
/// every arm.
fn verify_cost(d: u64) -> EcCount {
    EcCount {
        dbls: bits_of(d).saturating_sub(1) as u64,
        adds: (d.count_ones() as u64).saturating_sub(1),
        ..EcCount::default()
    }
}

// ── statistics ─────────────────────────────────────────────────────────

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

fn boot_mean(v: &[f64], reps: usize, seed: u64) -> (f64, f64, f64) {
    let mut rng = Rng64::new(seed);
    let n = v.len();
    let mut ms: Vec<f64> = (0..reps)
        .map(|_| (0..n).map(|_| v[rng.below(n as u64) as usize]).sum::<f64>() / n as f64)
        .collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        mean(v),
        ms[(reps as f64 * 0.025) as usize],
        ms[(reps as f64 * 0.975) as usize],
    )
}

/// Paired bootstrap of the ratio of means num/den.
fn boot_ratio(num: &[f64], den: &[f64], reps: usize, seed: u64) -> (f64, f64, f64) {
    let mut rng = Rng64::new(seed);
    let n = num.len();
    let mut rs: Vec<f64> = (0..reps)
        .map(|_| {
            let (mut a, mut b) = (0.0, 0.0);
            for _ in 0..n {
                let i = rng.below(n as u64) as usize;
                a += num[i];
                b += den[i];
            }
            a / b
        })
        .collect();
    rs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        mean(num) / mean(den),
        rs[(reps as f64 * 0.025) as usize],
        rs[(reps as f64 * 0.975) as usize],
    )
}

// ── instance set-up ────────────────────────────────────────────────────

fn setup(l: u32, n: u32) -> FieldTower {
    let nn = l * n;
    FieldTower::new(nn, n, l, enumerate_irreducibles(nn, 1).remove(0))
}

fn count_e(tower: &FieldTower, a: u64, beta: u64) -> u64 {
    if tower.big_n <= 16 {
        let nn = tower.big_n;
        let b = F2mElement::from_words(&[beta], nn).square(&tower.irr);
        count_e_points(tower, &F2mElement::from_words(&[a], nn), &b)
    } else {
        fast_count_e(&tower.irr, a, beta)
    }
}

fn host_json() -> Value {
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
        })
        .unwrap_or_default();
    let ncpu = fs::read_to_string("/proc/cpuinfo")
        .map(|s| s.lines().filter(|l| l.starts_with("processor")).count())
        .unwrap_or(0);
    json!({"cpu_model": cpu, "logical_cpus": ncpu, "os": env::consts::OS, "arch": env::consts::ARCH,
           "note": "shared cloud container; wall times are contended and secondary"})
}

#[derive(Clone, Debug)]
struct Inst {
    l: u32,
    n: u32,
    a: u64,
    beta: u64,
}

impl Inst {
    fn id(&self) -> String {
        format!("ghs-l{}-n{}-a{}-b{:#x}", self.l, self.n, self.a, self.beta)
    }
}

struct IcRun {
    d: Option<u64>,
    inconsistent: bool,
    exhausted: bool,
    j_adds: u64,
    tests: u64,
    relations: u64,
    independent: u64,
    dependent: u64,
    rank: usize,
    ncols: usize,
    // gates per phase
    g_phi_q: u128,
    g_steps: u128,
    g_relations: u128,
    g_linalg: u128,
    g_verify: u128,
    phi_k_adds: u64,
    mm: u64,
    ma: u64,
    mi: u64,
    wall_ms: f64,
}

impl IcRun {
    fn online_gates(&self) -> u128 {
        self.g_phi_q + self.g_steps + self.g_relations + self.g_linalg + self.g_verify
    }
}

#[allow(clippy::too_many_arguments)]
fn ic_once(
    cov: &SmoothCover,
    fb: &FactorBase,
    ell: u64,
    d1: &MumfordDivisor,
    q_pt: &Pt,
    seed: u64,
    steps: usize,
    budget: u64,
    nn: u32,
    tallied: bool,
    mode: RelMode,
) -> IcRun {
    let wbits = bits_of(ell);
    let t0 = Instant::now();
    // Φ(Q): target-dependent, charged
    let tally = tallied.then(OperationTally::begin);
    let (d2, phi_adds) = cov.image_alg(q_pt).expect("image of Q");
    let phi_ops = tally.map(|t| t.finish()).unwrap_or_default();
    let cfg = IcConfig {
        steps,
        max_positions: budget,
        seed,
        tally: tallied,
        mode,
    };
    let res = solve_dlp(&cov.curve_k, fb, ell, d1, &d2, &cfg);
    let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
    let st = &res.stats;
    let g_phi_q = price_ledger(&phi_ops, wbits).gates;
    let g_steps = price_ledger(&res.ops_step_setup, wbits).gates;
    let g_relations = price_ledger(&res.ops_relations, wbits).gates;
    let la = EcCount {
        mm: st.modr_mul,
        ma: st.modr_add,
        mi: st.modr_inv,
        ..EcCount::default()
    };
    let g_linalg = la.gates(nn, wbits);
    let g_verify = res.d.map(|d| verify_cost(d).gates(nn, wbits)).unwrap_or(0);
    IcRun {
        d: res.d,
        inconsistent: res.inconsistent,
        exhausted: res.budget_exhausted,
        j_adds: st.j_adds_setup + st.j_adds_walk,
        tests: st.tests,
        relations: st.relations,
        independent: st.independent,
        dependent: st.dependent,
        rank: st.rank,
        ncols: st.ncols,
        g_phi_q,
        g_steps,
        g_relations,
        g_linalg,
        g_verify,
        phi_k_adds: phi_adds,
        mm: st.modr_mul,
        ma: st.modr_add,
        mi: st.modr_inv,
        wall_ms,
    }
}

fn ic_json(r: &IcRun, ell: u64, ok: bool, e_add_gates: u128) -> Value {
    let ge = |g: u128| g as f64 / e_add_gates as f64;
    json!({
        "solved_and_verified": ok, "d_found": r.d, "inconsistent": r.inconsistent,
        "budget_exhausted": r.exhausted,
        "fb_columns": r.ncols, "rank": r.rank,
        "jacobian_additions": r.j_adds, "phi_Q_additions_over_K": r.phi_k_adds,
        "smoothness_tests": r.tests, "relations": r.relations,
        "independent": r.independent, "dependent": r.dependent,
        "modr_mul": r.mm, "modr_add": r.ma, "modr_inv": r.mi,
        "gates": {"phi_Q": r.g_phi_q.to_string(), "step_setup": r.g_steps.to_string(),
                  "relation_search": r.g_relations.to_string(), "linear_algebra": r.g_linalg.to_string(),
                  "verify": r.g_verify.to_string(), "online_total": r.online_gates().to_string()},
        "online_gae": ge(r.online_gates()),
        "S_online": ge(r.online_gates()) / (ell as f64).sqrt(),
        "phase_gae": {"phi_Q": ge(r.g_phi_q), "step_setup": ge(r.g_steps), "relation_search": ge(r.g_relations),
                      "linear_algebra": ge(r.g_linalg), "verify": ge(r.g_verify)},
        "wall_ms_untallied": r.wall_ms,
    })
}

fn arm_json(c: &EcCount, ok: bool, nn: u32, wbits: u32, d: &Option<u64>, ell: u64) -> Value {
    let v = d.map(verify_cost).unwrap_or_default();
    let tot = EcCount {
        adds: c.adds + v.adds,
        dbls: c.dbls + v.dbls,
        ..*c
    };
    let g = tot.gates(nn, wbits);
    let e_add = gates(nn, wbits, Prim::GroupAdd);
    json!({
        "solved_and_verified": ok, "d_found": d,
        "group_additions": tot.adds, "group_doublings": tot.dbls, "hash_probes": tot.probes,
        "modr_mul": tot.mm, "modr_add": tot.ma, "modr_inv": tot.mi,
        "gates": g.to_string(), "online_gae": g as f64 / e_add as f64,
        "S_online": g as f64 / e_add as f64 / (ell as f64).sqrt(),
    })
}

fn pick_point(
    tower: &FieldTower,
    a: u64,
    beta: u64,
    ec: &ECurve,
    ell: u64,
    ne: u64,
    rng: &mut Rng64,
) -> Option<Pt> {
    let nn = tower.big_n;
    let af = F2mElement::from_words(&[a], nn);
    let b = F2mElement::from_words(&[beta], nn).square(&tower.irr);
    for _ in 0..10_000 {
        let xv = 1 + rng.below((1u64 << nn) - 1);
        let x = F2mElement::from_words(&[xv], nn);
        for cand in lift_x(tower, &af, &b, &x) {
            let g = ec.scalar_mul(&cand, &BigUint::from(ne / ell));
            if let Pt::Aff { x, .. } = &g {
                if !x.is_zero() {
                    return Some(g);
                }
            }
        }
    }
    None
}

fn scalar_div_mul(
    d: &MumfordDivisor,
    k: u64,
    cur: &crypto_lib::binary_ecc::HyperellipticCurve,
) -> MumfordDivisor {
    let mut r = MumfordDivisor::identity(cur.m);
    for i in (0..bits_of(k)).rev() {
        r = r.add_unchecked(&r, cur);
        if (k >> i) & 1 == 1 {
            r = r.add_unchecked(d, cur);
        }
    }
    r
}

/// Cost of one walk step (a Jacobian addition) and of one smoothness test, priced in `bitop/v1` from the
/// field-operation ledger over `n` random positions.  The smoothness test here is a copy of the library's
/// (pre-test X^q = X mod u when the factor base is large, then trial division by the factor-base points).
fn calibrate(
    cov: &SmoothCover,
    fb: &FactorBase,
    ell: u64,
    nn: u32,
    e_add: u128,
    n: usize,
) -> Value {
    use crypto_lib::binary_ecc::F2mPoly;
    let cur = &cov.curve_k;
    let irr = &cur.irr;
    let l = cur.m;
    let wbits = bits_of(ell);
    let mut rng = Rng64::new(4242);
    let divs: Vec<MumfordDivisor> = fb.entries.iter().map(|e| e.div.clone()).collect();
    let mk = |rng: &mut Rng64| -> MumfordDivisor {
        let mut d = MumfordDivisor::identity(l);
        for _ in 0..6 {
            d = d.add_unchecked(&divs[rng.below(divs.len() as u64) as usize], cur);
        }
        d
    };
    let steps: Vec<MumfordDivisor> = (0..8).map(|_| mk(&mut rng)).collect();
    let mut t = mk(&mut rng);
    let mut positions = Vec::new();
    let tally = OperationTally::begin();
    for _ in 0..n {
        let s = &steps[rng.below(8) as usize];
        t = t.add_unchecked(s, cur);
        positions.push(t.clone());
    }
    let ops_add = tally.finish();
    let one = F2mElement::one(l);
    let tally = OperationTally::begin();
    let mut smooth = 0usize;
    for d in &positions {
        let u = &d.u;
        let mut ok = true;
        if let Some(du) = u.degree() {
            if du >= 2 && fb.entries.len() > 2 * l as usize * du {
                let x = F2mPoly::from_coeffs(vec![F2mElement::zero(l), one.clone()], l);
                let mut h = x.clone();
                for _ in 0..l {
                    h = h.square_mod(u, irr);
                }
                ok = h.eq_poly(&x);
            }
        }
        if ok {
            let mut rem = u.clone();
            for e in &fb.entries {
                if rem.degree().unwrap_or(0) == 0 {
                    break;
                }
                while rem.degree().unwrap_or(0) > 0 && rem.eval(&e.x, irr).is_zero() {
                    let lin = F2mPoly::from_coeffs(vec![e.x.clone(), one.clone()], l);
                    rem = rem.divrem(&lin, irr).0;
                }
            }
            if rem.degree().unwrap_or(0) == 0 {
                smooth += 1;
            }
        }
    }
    let ops_test = tally.finish();
    let pa = price_ledger(&ops_add, wbits);
    let pt = price_ledger(&ops_test, wbits);
    let counts = |p: &Priced| -> Value {
        let (mut a, mut m, mut s, mut i) = (0u64, 0u64, 0u64, 0u64);
        for v in p.by_width.values() {
            a += v.0;
            m += v.1;
            s += v.2;
            i += v.3;
        }
        json!({"adds": a as f64 / n as f64, "muls": m as f64 / n as f64, "squares": s as f64 / n as f64, "inversions": i as f64 / n as f64})
    };
    let _ = nn;
    json!({"positions": n, "smooth_among_positions": smooth,
        "j_add_gae": pa.gates as f64 / n as f64 / e_add as f64,
        "smoothness_test_gae": pt.gates as f64 / n as f64 / e_add as f64,
        "field_ops_per_j_add_over_Fq": counts(&pa), "field_ops_per_test_over_Fq": counts(&pt),
        "note": "gae = bitop/v1 gates divided by gates of one E addition over F_{2^N}"})
}

fn run_instance(
    inst: &Inst,
    tune_logs: usize,
    logs: usize,
    steps: usize,
    scan: &[usize],
    budget: u64,
    seed0: u64,
) -> Value {
    let t_all = Instant::now();
    let tower = setup(inst.l, inst.n);
    let nn = tower.big_n;
    let a = F2mElement::from_words(&[inst.a], nn);
    let beta = F2mElement::from_words(&[inst.beta], nn);
    let b = beta.square(&tower.irr);
    let id = inst.id();
    eprintln!("== {id}");
    let t0 = Instant::now();
    let cov = match SmoothCover::new(&tower, &a, &beta) {
        Ok(c) => c,
        Err(e) => return json!({"instance": id, "error": format!("{e:?}")}),
    };
    let t_cover = t0.elapsed().as_secs_f64();
    let g = cov.genus as usize;
    let q = 1u64 << inst.l;
    let t0 = Instant::now();
    let ne = count_e(&tower, inst.a, inst.beta);
    let t_count = t0.elapsed().as_secs_f64();
    let facs = factor(ne);
    let ec = ECurve::new(nn, tower.irr.clone(), a.clone(), b.clone());
    let mut rng = Rng64::new(seed0 ^ 0xA5A5);
    // choose the largest prime ell >= 7 whose order-ell subgroup survives Phi
    let mut killed: Vec<u64> = Vec::new();
    let mut chosen: Option<(u64, Pt, MumfordDivisor)> = None;
    let mut primes: Vec<u64> = facs.iter().map(|f| f.0).filter(|&p| p >= 7).collect();
    primes.sort_unstable_by(|x, y| y.cmp(x));
    let ell_e = facs.last().map(|f| f.0).unwrap_or(1);
    for &pr in &primes {
        let Some(pp) = pick_point(&tower, inst.a, inst.beta, &ec, pr, ne, &mut rng) else {
            continue;
        };
        match cov.image_alg(&pp) {
            Ok((d1, _)) if !d1.is_identity() => {
                chosen = Some((pr, pp, d1));
                break;
            }
            _ => killed.push(pr),
        }
    }
    let mut out = json!({
        "instance": id, "l": inst.l, "n": inst.n, "N": nn, "q": q, "a": inst.a, "beta": inst.beta,
        "irr_K_low_terms": tower.irr.low_terms, "genus": g, "magic": cov.magic,
        "model": format!("{:?}", cov.model), "E_order": ne, "E_order_factorisation": facs,
        "largest_prime_of_E": ell_e, "primes_killed_by_phi": killed,
        "wall_s": {"cover_construction": t_cover, "count_E": t_count},
        "host": host_json(),
    });
    let Some((ell, p, d1)) = chosen else {
        out["applicable"] = json!(false);
        out["reason"] = json!(
            "no prime factor >= 7 of #E has an order-ell subgroup with non-trivial image under Phi"
        );
        return out;
    };
    let wbits = bits_of(ell);
    let e_add = gates(nn, wbits, Prim::GroupAdd);
    // hygiene (uncounted): D1 has order ell
    let ell_d1 = scalar_div_mul(&d1, ell, &cov.curve_k);
    assert!(ell_d1.is_identity(), "ell * Phi(P) != 0");
    out["applicable"] = json!(true);
    out["ell"] = json!(ell);
    out["cofactor"] = json!(ne / ell);
    out["ell_is_largest_prime_of_E"] = json!(ell == ell_e);
    out["ell_squared_divides_E"] = json!(ne.is_multiple_of(ell * ell));
    out["E_add_gates"] = json!(e_add.to_string());

    // factor base (reusable setup, priced)
    let t0 = Instant::now();
    let tally = OperationTally::begin();
    let pts = affine_points(&cov.curve_k);
    let ops_fb = tally.finish();
    let g_fb = price_ledger(&ops_fb, wbits).gates;
    let t_fb = t0.elapsed().as_secs_f64();
    let full_fb = FactorBase::from_points(&cov.curve_k, &pts, usize::MAX);
    let avail = full_fb.available_columns;
    out["factor_base_available"] = json!({"affine_points": pts.len(), "columns": avail,
        "ramified_points": full_fb.entries.iter().filter(|e| e.ramified).count(),
        "enumeration_gae": g_fb as f64 / e_add as f64, "wall_s": t_fb});
    if avail == 0 {
        out["applicable"] = json!(false);
        out["reason"] = json!("no non-ramified F_q-point on C: empty factor base");
        return out;
    }
    out["calibration"] = calibrate(&cov, &full_fb, ell, nn, e_add, 300);
    // Φ(P): reusable setup, priced
    let tally = OperationTally::begin();
    let (_, k_adds_p) = cov.image_alg(&p).expect("image P");
    let ops_phi_p = tally.finish();
    let g_phi_p = price_ledger(&ops_phi_p, wbits).gates;
    out["reusable_setup"] = json!({"factor_base_enumeration_gae": g_fb as f64 / e_add as f64,
        "phi_P_gae": g_phi_p as f64 / e_add as f64, "phi_P_additions_over_K": k_adds_p,
        "cover_construction": "not priced (library finds the F_q embedding by search over K; wall time recorded)"});

    // planted logs
    let total_logs = tune_logs + logs;
    let mut plants: Vec<(u64, Pt)> = Vec::new();
    let mut prng = Rng64::new(seed0 ^ 0x1234_5678);
    while plants.len() < total_logs {
        let d = 1 + prng.below(ell - 1);
        let qd = ec.scalar_mul(&p, &BigUint::from(d));
        if matches!(qd, Pt::Inf) {
            continue;
        }
        plants.push((d, qd));
    }

    // scan sizes
    let mut sizes: Vec<usize> = scan
        .iter()
        .copied()
        .filter(|&s| s >= 1 && s <= avail)
        .collect();
    sizes.push(avail);
    sizes.sort_unstable();
    sizes.dedup();
    let fbs: Vec<FactorBase> = sizes
        .iter()
        .map(|&s| FactorBase::from_points(&cov.curve_k, &pts, s))
        .collect();

    // ── tuning: choose the relation mode and the factor-base size on the first `tune_logs` plants ──
    // For each mode, sizes are tried from the largest down; the sweep stops after two consecutive
    // sizes that do not improve the best mean cost for that mode.  Once some configuration has
    // solved, later ones get a position budget of 10x the best mean test count so far (a
    // configuration that cannot beat that is recorded as unsolved at its spent cost).
    let mut scan_rows = Vec::new();
    let mut best: Option<(RelMode, usize, f64)> = None;
    let mut best_tests = f64::INFINITY;
    for mode in [RelMode::Dense, RelMode::Sparse] {
        let mut mode_best = f64::INFINITY;
        let mut worse_streak = 0;
        for si in (0..fbs.len()).rev() {
            let fb = &fbs[si];
            let bud = if best_tests.is_finite() {
                ((10.0 * best_tests) as u64 + 1000).min(budget)
            } else {
                budget
            };
            let mut costs = Vec::new();
            let mut tests = Vec::new();
            let mut solved = 0usize;
            for (i, (d, qd)) in plants.iter().take(tune_logs).enumerate() {
                let r = ic_once(
                    &cov,
                    fb,
                    ell,
                    &d1,
                    qd,
                    seed0 + i as u64,
                    steps,
                    bud,
                    nn,
                    true,
                    mode,
                );
                if r.d == Some(*d) {
                    solved += 1;
                }
                costs.push(r.online_gates() as f64 / e_add as f64);
                tests.push(r.tests as f64);
            }
            let m = mean(&costs);
            eprintln!(
                "  tune {mode:?} fb_cols={} mean online gae={:.1} solved {}/{} (budget {bud})",
                sizes[si], m, solved, tune_logs
            );
            scan_rows.push(json!({"mode": format!("{mode:?}"), "fb_columns": sizes[si], "mean_online_gae": m,
                "solved": solved, "logs": tune_logs, "mean_positions_tested": mean(&tests), "position_budget": bud}));
            let acceptable = solved * 10 >= tune_logs * 8;
            if acceptable && m < mode_best {
                mode_best = m;
                worse_streak = 0;
                if best.map(|b| m < b.2).unwrap_or(true) {
                    best = Some((mode, si, m));
                    best_tests = mean(&tests);
                }
            } else {
                worse_streak += 1;
                if worse_streak >= 2 && mode_best.is_finite() {
                    break;
                }
                if !mode_best.is_finite() && worse_streak >= 3 {
                    break;
                }
            }
        }
    }
    out["tuning_scan"] = json!(scan_rows);
    let Some((best_mode, bi, _)) = best else {
        out["chosen_fb_columns"] = Value::Null;
        out["verdict_instance"] = json!("IC failed to solve at least 80% of tuning logs at every factor-base size within the budget");
        out["wall_s"]["total"] = json!(t_all.elapsed().as_secs_f64());
        return out;
    };
    let fb = &fbs[bi];
    out["chosen_fb_columns"] = json!(sizes[bi]);
    out["chosen_mode"] = json!(format!("{best_mode:?}"));

    // ── rho tuning on the same tuning logs: the walk-multiplier width is chosen by the lowest mean cost ──
    let mut rho_rows = Vec::new();
    let mut rho_best: Option<(u32, f64)> = None;
    {
        let mut cands: Vec<u32> = vec![
            3,
            4,
            5,
            6,
            wbits.div_ceil(2) + 1,
            wbits.div_ceil(2) + 3,
            wbits,
        ];
        cands.retain(|&c| c >= 2 && c <= wbits.max(2));
        cands.sort_unstable();
        cands.dedup();
        for &cb in &cands {
            let mut costs = Vec::new();
            for (i, (d, qd)) in plants.iter().take(tune_logs).enumerate() {
                let mut ecc = Ec {
                    c: &ec,
                    n: EcCount::default(),
                };
                let (rd, _) = rho(&mut ecc, &p, qd, ell, cb, &mut Rng64::new(seed0 + i as u64));
                assert_eq!(rd, Some(*d), "rho failed on a tuning log");
                let v = rd.map(verify_cost).unwrap_or_default();
                let t = EcCount {
                    adds: ecc.n.adds + v.adds,
                    dbls: ecc.n.dbls + v.dbls,
                    ..ecc.n
                };
                costs.push(t.gates(nn, wbits) as f64 / e_add as f64);
            }
            let m = mean(&costs);
            rho_rows.push(json!({"multiplier_bits": cb, "mean_gae": m}));
            if rho_best.map(|b| m < b.1).unwrap_or(true) {
                rho_best = Some((cb, m));
            }
        }
    }
    let rho_cbits = rho_best.map(|b| b.0).unwrap_or(4);
    out["rho_tuning"] =
        json!({"rows": rho_rows, "chosen_multiplier_bits": rho_cbits, "steps_r": 8});

    // ── holdout: paired IC / rho / BSGS on the remaining plants ──
    let mut rows = Vec::new();
    let (mut ic_gae, mut rho_gae, mut bs_gae, mut ic_j, mut rho_e, mut bs_e) =
        (vec![], vec![], vec![], vec![], vec![], vec![]);
    let (mut ic_cold_gae, mut ic_wall, mut rho_wall, mut bs_wall) =
        (vec![], vec![], vec![], vec![]);
    let (mut ok_ic, mut ok_rho, mut ok_bs) = (0, 0, 0);
    for (i, (d, qd)) in plants.iter().enumerate().skip(tune_logs) {
        let seed = seed0 + 1000 + i as u64;
        let r = ic_once(
            &cov, fb, ell, &d1, qd, seed, steps, budget, nn, true, best_mode,
        );
        // untallied replay: wall time and a determinism check
        let r2 = ic_once(
            &cov, fb, ell, &d1, qd, seed, steps, budget, nn, false, best_mode,
        );
        assert_eq!(r.d, r2.d, "IC not deterministic");
        assert_eq!(r.tests, r2.tests, "IC test count not deterministic");
        let ok = r.d == Some(*d) && ec.scalar_mul(&p, &BigUint::from(r.d.unwrap())) == *qd;
        ok_ic += ok as usize;
        // rho
        let mut ecc = Ec {
            c: &ec,
            n: EcCount::default(),
        };
        let t0 = Instant::now();
        let (rd, _steps) = rho(&mut ecc, &p, qd, ell, rho_cbits, &mut Rng64::new(seed));
        let rho_ms = t0.elapsed().as_secs_f64() * 1e3;
        let rho_ok = rd == Some(*d) && ec.scalar_mul(&p, &BigUint::from(rd.unwrap())) == *qd;
        ok_rho += rho_ok as usize;
        let rho_c = ecc.n;
        // bsgs
        let mut ecb = Ec {
            c: &ec,
            n: EcCount::default(),
        };
        let t0 = Instant::now();
        let bd = bsgs(&mut ecb, &p, qd, ell);
        let bs_ms = t0.elapsed().as_secs_f64() * 1e3;
        let bs_ok = bd == Some(*d) && ec.scalar_mul(&p, &BigUint::from(bd.unwrap())) == *qd;
        ok_bs += bs_ok as usize;
        let bs_c = ecb.n;
        let tot = |c: &EcCount, dd: &Option<u64>| {
            let v = dd.map(verify_cost).unwrap_or_default();
            EcCount {
                adds: c.adds + v.adds,
                dbls: c.dbls + v.dbls,
                ..*c
            }
        };
        let rho_t = tot(&rho_c, &rd);
        let bs_t = tot(&bs_c, &bd);
        let rho_g = rho_t.gates(nn, wbits) as f64 / e_add as f64;
        let bs_g = bs_t.gates(nn, wbits) as f64 / e_add as f64;
        let ic_g = r.online_gates() as f64 / e_add as f64;
        ic_gae.push(ic_g);
        rho_gae.push(rho_g);
        bs_gae.push(bs_g);
        ic_j.push(r.j_adds as f64);
        rho_e.push(rho_t.group_ops() as f64);
        bs_e.push(bs_t.group_ops() as f64);
        ic_cold_gae.push(ic_g + (g_fb + g_phi_p) as f64 / e_add as f64);
        ic_wall.push(r2.wall_ms);
        rho_wall.push(rho_ms);
        bs_wall.push(bs_ms);
        rows.push(json!({
            "log_index": i, "planted_d": d, "seed": seed,
            "ic": ic_json(&r, ell, ok, e_add),
            "rho": arm_json(&rho_c, rho_ok, nn, wbits, &rd, ell),
            "bsgs": arm_json(&bs_c, bs_ok, nn, wbits, &bd, ell),
            "rho_wall_ms": rho_ms, "bsgs_wall_ms": bs_ms,
        }));
        eprintln!(
            "  log {i}: d={d} ic_ok={ok} ic_gae={ic_g:.0} (J-adds {}, tests {}, rel {}) rho_gae={rho_g:.0} ({} E-ops) bsgs_gae={bs_g:.0}",
            r.j_adds, r.tests, r.relations, rho_t.group_ops()
        );
    }
    let reps = 10_000;
    let sq = (ell as f64).sqrt();
    let (m_ic, lo_ic, hi_ic) = boot_mean(&ic_gae, reps, 11);
    let (m_rho, lo_rho, hi_rho) = boot_mean(&rho_gae, reps, 12);
    let (m_bs, lo_bs, hi_bs) = boot_mean(&bs_gae, reps, 13);
    let best_name = if m_rho <= m_bs { "rho" } else { "bsgs" };
    let best_g: &Vec<f64> = if m_rho <= m_bs { &rho_gae } else { &bs_gae };
    let r_rho = boot_ratio(&ic_gae, &rho_gae, reps, 21);
    let r_bs = boot_ratio(&ic_gae, &bs_gae, reps, 22);
    let r_best = boot_ratio(&ic_gae, best_g, reps, 23);
    let r_raw = boot_ratio(&ic_j, &rho_e, reps, 24);
    let best_e: &Vec<f64> = if m_rho <= m_bs { &rho_e } else { &bs_e };
    let r_raw_best = boot_ratio(&ic_j, best_e, reps, 26);
    let r_cold = boot_ratio(&ic_cold_gae, best_g, reps, 25);
    out["holdout"] = json!({
        "logs": logs, "solved_ic": ok_ic, "solved_rho": ok_rho, "solved_bsgs": ok_bs,
        "mean_gae": {"ic_online": [m_ic, lo_ic, hi_ic], "rho": [m_rho, lo_rho, hi_rho], "bsgs": [m_bs, lo_bs, hi_bs]},
        "S_online": {"ic": m_ic / sq, "rho": m_rho / sq, "bsgs": m_bs / sq},
        "best_generic": best_name,
        "ratio_ic_over_rho": {"point": r_rho.0, "ci95": [r_rho.1, r_rho.2]},
        "ratio_ic_over_bsgs": {"point": r_bs.0, "ci95": [r_bs.1, r_bs.2]},
        "ratio_ic_over_best_generic": {"point": r_best.0, "ci95": [r_best.1, r_best.2]},
        "ratio_ic_cold_over_best_generic": {"point": r_cold.0, "ci95": [r_cold.1, r_cold.2]},
        "raw_group_op_ratio_ic_Jadds_over_rho_Eops": {"point": r_raw.0, "ci95": [r_raw.1, r_raw.2],
            "note": "counts each Jacobian addition as one group operation: the most favourable pricing for IC"},
        "raw_group_op_ratio_ic_Jadds_over_best_generic_Eops": {"point": r_raw_best.0, "ci95": [r_raw_best.1, r_raw_best.2]},
        "mean_jacobian_additions": mean(&ic_j), "mean_rho_E_ops": mean(&rho_e), "mean_bsgs_E_ops": mean(&bs_e),
        "mean_wall_ms": {"ic_untallied": mean(&ic_wall), "rho": mean(&rho_wall), "bsgs": mean(&bs_wall),
            "note": "contended container; secondary"},
    });
    out["rows"] = json!(rows);
    out["wall_s"]["total"] = json!(t_all.elapsed().as_secs_f64());
    out
}

// ── pick ───────────────────────────────────────────────────────────────

fn pick(l: u32, n: u32, want_m: u32, count: usize, max_h: u64, samples: u64, seed: u64) -> Value {
    let tower = setup(l, n);
    let nn = tower.big_n;
    let mut rng = Rng64::new(seed);
    let mut seen: HashSet<u64> = HashSet::new();
    let mut found = Vec::new();
    for _ in 0..samples {
        let bits = 1 + rng.below((1u64 << nn) - 1);
        let a_bit = rng.below(2);
        let a = F2mElement::from_words(&[a_bit], nn);
        let beta = F2mElement::from_words(&[bits], nn);
        let b = beta.square(&tower.irr);
        let m = magic_number_full(&tower, &a, &b);
        if m < 2 || (want_m != 0 && m != want_m) {
            continue;
        }
        if !tower.is_in_subfield(&a, l) {
            continue;
        }
        let ne = count_e(&tower, a_bit, bits);
        let facs = factor(ne);
        let lp = facs.last().map(|x| x.0).unwrap_or(1);
        let h = ne / lp;
        if h > max_h || lp < 50 || !seen.insert(ne * 2 + a_bit) {
            continue;
        }
        let ti = ghs_type_i(&tower, &b);
        let g = if ti {
            (1u32 << (m - 1)) - 1
        } else {
            1u32 << (m - 1)
        };
        found.push(
            json!({"l": l, "n": n, "a": a_bit, "beta": bits, "magic": m, "genus": g,
            "E_order": ne, "largest_prime": lp, "cofactor": h}),
        );
        if found.len() >= count {
            break;
        }
    }
    json!({"l": l, "n": n, "N": nn, "irr_K_low_terms": tower.irr.low_terms, "candidates": found})
}

// ── profile (the (3,4), g = 8 hang) ────────────────────────────────────

fn profile(l: u32, n: u32, want_m: u32, maxk: u32) -> Value {
    let tower = setup(l, n);
    let nn = tower.big_n;
    let q = 1u64 << l;
    let mut rows = Vec::new();
    // reproduce R7.1's selection scan: first beta with magic == want, lp >= 20
    let t0 = Instant::now();
    let mut chosen = None;
    let mut cand_checked = 0u64;
    for a_bit in [0u64, 1] {
        let a = F2mElement::from_words(&[a_bit], nn);
        for bits in 1u64..(1u64 << nn) {
            let beta = F2mElement::from_words(&[bits], nn);
            let b = beta.square(&tower.irr);
            let m = magic_number_full(&tower, &a, &b);
            if m < 2 || (want_m != 0 && m != want_m) {
                continue;
            }
            cand_checked += 1;
            let ne = count_e_points(&tower, &a, &b);
            let lp = factor(ne).last().map(|x| x.0).unwrap_or(1);
            if lp < 20 {
                continue;
            }
            chosen = Some((a_bit, bits, ne));
            break;
        }
        if chosen.is_some() {
            break;
        }
    }
    let t_select = t0.elapsed().as_secs_f64();
    let Some((a_bit, bits, ne)) = chosen else {
        return json!({"error": "no candidate"});
    };
    rows.push(json!({"phase": "selection scan (magic_number_full + count_e_points per candidate)", "seconds": t_select, "candidates_counted": cand_checked}));
    let a = F2mElement::from_words(&[a_bit], nn);
    let beta = F2mElement::from_words(&[bits], nn);
    let t0 = Instant::now();
    let cov = SmoothCover::new(&tower, &a, &beta).expect("cover");
    let t_cover = t0.elapsed().as_secs_f64();
    rows.push(json!({"phase": "SmoothCover::new", "seconds": t_cover}));
    let g = cov.genus;
    // validated vs unchecked Jacobian addition
    let pts = affine_points(&cov.curve_k);
    let cur = &cov.curve_k;
    let divs: Vec<MumfordDivisor> = pts
        .iter()
        .map(|(x, y)| MumfordDivisor::from_point(cur, x, y).unwrap())
        .collect();
    let mut acc = divs[0].clone();
    let t0 = Instant::now();
    let reps = 200;
    for i in 0..reps {
        acc = acc.add(&divs[i % divs.len()], cur);
    }
    let t_checked = t0.elapsed().as_secs_f64() / reps as f64;
    let mut acc2 = divs[0].clone();
    let t0 = Instant::now();
    for i in 0..reps {
        acc2 = acc2.add_unchecked(&divs[i % divs.len()], cur);
    }
    let t_unchecked = t0.elapsed().as_secs_f64() / reps as f64;
    assert_eq!(acc, acc2, "add_unchecked differs from add");
    rows.push(json!({"phase": "Jacobian add, validated (MumfordDivisor::add)", "seconds_per_op": t_checked}));
    rows.push(json!({"phase": "Jacobian add, add_unchecked", "seconds_per_op": t_unchecked}));
    // point counts per extension degree
    let mut count_rows = Vec::new();
    let mut last = 0.0;
    for k in 1..=maxk.min(g + 1) {
        let t0 = Instant::now();
        let c = count_points_extension(&cov.curve_k, k);
        let dt = t0.elapsed().as_secs_f64();
        count_rows.push(json!({"k": k, "field_bits": l * k, "N_k": c as i64, "seconds": dt, "ratio_to_previous": if last > 0.0 { dt / last } else { 0.0 }}));
        eprintln!("count_points_extension k={k} ({} bits): {dt:.2}s", l * k);
        last = dt;
    }
    // extrapolation of the remaining degrees, 2^l per step
    let mut extrap = 0.0;
    let mut dt = last;
    for _k in (maxk.min(g + 1) + 1)..=(g + 1) {
        dt *= q as f64;
        extrap += dt;
    }
    let t0 = Instant::now();
    let _ = count_e_points(&tower, &a, &beta.square(&tower.irr));
    let t_ce = t0.elapsed().as_secs_f64();
    // closure of a subgroup as in R7.1 (`subgroup`): generators are the affine-point divisors and the
    // images of 12 random points of E, one validated add per generator per element, cap 400000
    let ec = ECurve::new(nn, tower.irr.clone(), a.clone(), beta.square(&tower.irr));
    let mut prng = Rng64::new(99);
    let mut gens_j: Vec<MumfordDivisor> = divs.clone();
    let mut got = 0;
    while got < 12 {
        let xv = 1 + prng.below((1u64 << nn) - 1);
        let x = F2mElement::from_words(&[xv], nn);
        for pt in lift_x(&tower, &a, &beta.square(&tower.irr), &x) {
            if let Ok(img) = cov.image(&pt) {
                gens_j.push(img);
                got += 1;
                break;
            }
        }
    }
    let _ = &ec;
    let cap = 20_000usize;
    let key = |d: &MumfordDivisor| -> Vec<u64> {
        let mut k: Vec<u64> = d.u.coeffs.iter().map(|c| c.raw_bits()[0]).collect();
        k.push(u64::MAX);
        k.extend(d.v.coeffs.iter().map(|c| c.raw_bits()[0]));
        k
    };
    let t0 = Instant::now();
    let mut elems = vec![MumfordDivisor::identity(cur.m)];
    let mut seen: HashSet<Vec<u64>> = HashSet::new();
    seen.insert(key(&elems[0]));
    let mut i = 0;
    let mut adds_done = 0u64;
    while i < elems.len() && elems.len() < cap {
        for gj in &gens_j {
            let s = elems[i].add(gj, cur);
            adds_done += 1;
            if seen.insert(key(&s)) {
                elems.push(s);
            }
        }
        i += 1;
    }
    let t_closure = t0.elapsed().as_secs_f64();
    let per_add = t_closure / adds_done as f64;
    let gens = gens_j.len();
    let closure_adds = 400_000u64 * gens as u64;
    let t0 = Instant::now();
    let es: Vec<F2mElement> = (0..q).map(|w| F2mElement::from_words(&[w], l)).collect();
    let nsplit_tests = 20_000;
    let mut cnt = 0;
    for d in elems.iter().cycle().take(nsplit_tests) {
        // trial division over F_q, as in R7.1's `splits`
        let mut p = d.u.clone();
        for x in &es {
            while p.degree().unwrap_or(0) > 0 && p.eval(x, &cur.irr).is_zero() {
                let lin = crypto_lib::binary_ecc::F2mPoly::from_coeffs(
                    vec![x.clone(), F2mElement::one(l)],
                    l,
                );
                p = p.divrem(&lin, &cur.irr).0;
            }
        }
        if p.degree().unwrap_or(0) == 0 {
            cnt += 1;
        }
    }
    let t_split = t0.elapsed().as_secs_f64() / nsplit_tests as f64;
    let _ = cnt;
    json!({
        "closure_measured": {"cap": cap, "elements_reached": elems.len(), "generators": gens, "validated_adds": adds_done,
            "seconds": t_closure, "seconds_per_validated_add": per_add,
            "extrapolated_seconds_for_R71_cap_400000": closure_adds as f64 * per_add,
            "reached_cap": elems.len() >= cap},
        "split_test_seconds_each": t_split,
        "R71_run_curve_steps_estimate": {
            "note": "run_curve calls subgroup() with cap 400000 once (J0) and with cap 20000 twice (H, l-part); three 100000-step walks (+20000 burn-in each) plus the burn-in; two full filters of J0 with `splits`",
            "J0_closure_seconds": closure_adds as f64 * per_add,
            "J0_split_filter_seconds": 400_000.0 * t_split,
            "walk_seconds_each": 120_000.0 * (per_add + t_split),
        },
        "l": l, "n": n, "N": nn, "q": q, "a": a_bit, "beta": bits, "genus": g, "E_order": ne,
        "affine_points": pts.len(),
        "phases": rows,
        "count_points_extension": count_rows,
        "extrapolated_seconds_for_remaining_counts_up_to_g_plus_1": extrap,
        "count_e_points_seconds": t_ce,
    })
}

// ── fit: log-log regressions over the recorded instances ───────────────

struct Pt3 {
    x: f64,
    y: f64,
}

fn ols(p: &[Pt3]) -> (f64, f64) {
    let n = p.len() as f64;
    let mx = p.iter().map(|q| q.x).sum::<f64>() / n;
    let my = p.iter().map(|q| q.y).sum::<f64>() / n;
    let sxx: f64 = p.iter().map(|q| (q.x - mx).powi(2)).sum();
    let sxy: f64 = p.iter().map(|q| (q.x - mx) * (q.y - my)).sum();
    let slope = if sxx > 0.0 { sxy / sxx } else { f64::NAN };
    (slope, my - slope * mx)
}

/// Bootstrap (over instances) of slope and intercept; returns (slope, intercept, slope ci, intercept ci,
/// and the percentile interval of the x where the fitted line crosses 0 when the slope is negative).
fn boot_fit(p: &[Pt3], reps: usize, seed: u64) -> Value {
    let (s0, i0) = ols(p);
    let mut rng = Rng64::new(seed);
    let mut slopes = Vec::new();
    let mut ints = Vec::new();
    let mut cross = Vec::new();
    for _ in 0..reps {
        let rs: Vec<Pt3> = (0..p.len())
            .map(|_| {
                let q = &p[rng.below(p.len() as u64) as usize];
                Pt3 { x: q.x, y: q.y }
            })
            .collect();
        let xs: HashSet<u64> = rs.iter().map(|q| q.x.to_bits()).collect();
        if xs.len() < 2 {
            continue;
        }
        let (s, i) = ols(&rs);
        slopes.push(s);
        ints.push(i);
        cross.push(if s < 0.0 { -i / s } else { f64::INFINITY });
    }
    let q = |v: &mut Vec<f64>, f: f64| {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        v[((v.len() as f64 * f) as usize).min(v.len() - 1)]
    };
    let frac_neg = cross.iter().filter(|c| c.is_finite()).count() as f64 / cross.len() as f64;
    json!({"slope": s0, "slope_ci95": [q(&mut slopes, 0.025), q(&mut slopes, 0.975)],
        "intercept": i0, "intercept_ci95": [q(&mut ints, 0.025), q(&mut ints, 0.975)],
        "x_at_zero": if s0 < 0.0 { Value::from(-i0 / s0) } else { Value::Null },
        "x_at_zero_ci95": [q(&mut cross, 0.025), q(&mut cross, 0.975)],
        "bootstrap_fraction_with_negative_slope": frac_neg, "points": p.len()})
}

fn fit(files: &[String]) -> Value {
    let mut recs: Vec<Value> = Vec::new();
    for f in files {
        let v: Value = serde_json::from_str(&fs::read_to_string(f).unwrap()).unwrap();
        for i in v["instances"].as_array().unwrap() {
            if !i["holdout"].is_null() {
                recs.push(i.clone());
            }
        }
    }
    // per instance means
    let mut rows = Vec::new();
    for r in &recs {
        let h = &r["holdout"];
        let phase = |k: &str| -> f64 {
            let rs = r["rows"].as_array().unwrap();
            rs.iter()
                .map(|x| x["ic"]["phase_gae"][k].as_f64().unwrap())
                .sum::<f64>()
                / rs.len() as f64
        };
        let ic = h["mean_gae"]["ic_online"][0].as_f64().unwrap();
        let rho = h["mean_gae"]["rho"][0].as_f64().unwrap();
        let bs = h["mean_gae"]["bsgs"][0].as_f64().unwrap();
        rows.push(json!({
            "instance": r["instance"], "l": r["l"], "n": r["n"], "N": r["N"], "genus": r["genus"], "ell": r["ell"],
            "ic": ic, "rho": rho, "bsgs": bs, "best_generic": rho.min(bs),
            "ratio_ic_over_best": ic / rho.min(bs),
            "phi_Q": phase("phi_Q"), "step_setup": phase("step_setup"), "relation_search": phase("relation_search"),
            "linear_algebra": phase("linear_algebra"), "verify": phase("verify"),
            "jacobian_additions": h["mean_jacobian_additions"], "fb_columns": r["chosen_fb_columns"],
            "calibration_j_add_gae": r["calibration"]["j_add_gae"],
            "calibration_smoothness_test_gae": r["calibration"]["smoothness_test_gae"],
            "derived_ratio_if_jacobian_additions_cost_one_E_addition":
                (ic - h["mean_jacobian_additions"].as_f64().unwrap() * (r["calibration"]["j_add_gae"].as_f64().unwrap() - 1.0)) / rho.min(bs),
            "derived_ratio_if_step_setup_free": (ic - phase("step_setup")) / rho.min(bs),
            "derived_ratio_if_jacobian_additions_were_free":
                (ic - h["mean_jacobian_additions"].as_f64().unwrap() * r["calibration"]["j_add_gae"].as_f64().unwrap()) / rho.min(bs),
            "ncols_available": r["factor_base_available"]["columns"],
        }));
    }
    let mut families = serde_json::Map::new();
    for nfam in [3u64, 4u64] {
        let fam: Vec<&Value> = rows
            .iter()
            .filter(|r| r["n"] == nfam && r["genus"] == 4)
            .collect();
        if fam.len() < 3 {
            continue;
        }
        let pts = |key: &str| -> Vec<Pt3> {
            fam.iter()
                .map(|r| Pt3 {
                    x: r["l"].as_f64().unwrap(),
                    y: r[key].as_f64().unwrap().log2(),
                })
                .collect()
        };
        let ratio_pts: Vec<Pt3> = fam
            .iter()
            .map(|r| Pt3 {
                x: r["l"].as_f64().unwrap(),
                y: r["ratio_ic_over_best"].as_f64().unwrap().log2(),
            })
            .collect();
        let free_pts: Vec<Pt3> = fam
            .iter()
            .map(|r| Pt3 {
                x: r["l"].as_f64().unwrap(),
                y: r["derived_ratio_if_jacobian_additions_were_free"]
                    .as_f64()
                    .unwrap()
                    .log2(),
            })
            .collect();
        let mut fits = serde_json::Map::new();
        for k in [
            "ic",
            "rho",
            "bsgs",
            "best_generic",
            "phi_Q",
            "step_setup",
            "relation_search",
            "linear_algebra",
            "jacobian_additions",
        ] {
            fits.insert(k.to_string(), boot_fit(&pts(k), 5000, 77));
        }
        fits.insert(
            "ratio_ic_over_best_generic".to_string(),
            boot_fit(&ratio_pts, 5000, 78),
        );
        fits.insert(
            "derived_ratio_if_jacobian_additions_were_free".to_string(),
            boot_fit(&free_pts, 5000, 79),
        );
        let nosetup_pts: Vec<Pt3> = fam
            .iter()
            .map(|r| Pt3 {
                x: r["l"].as_f64().unwrap(),
                y: r["derived_ratio_if_step_setup_free"]
                    .as_f64()
                    .unwrap()
                    .log2(),
            })
            .collect();
        fits.insert(
            "derived_ratio_if_step_setup_free".to_string(),
            boot_fit(&nosetup_pts, 5000, 80),
        );
        families.insert(
            format!("n{nfam}_g4"),
            json!({"family": format!("n = {nfam}, genus 4, q = 2^l: log2 of the mean cost in E-add equivalents against l"),
                "fits_log2_cost_vs_l": fits}),
        );
    }
    json!({"instances": rows, "families": families})
}

// ── selftest ───────────────────────────────────────────────────────────

fn selftest() -> Value {
    let mut checks = Vec::new();
    // fast #E vs library count
    for (l, n, a, beta) in [
        (2u32, 6u32, 0u64, 0x28u64),
        (2, 6, 1, 0x5),
        (3, 4, 0, 0x8),
        (2, 7, 0, 0x4c),
    ] {
        let tower = setup(l, n);
        let nn = tower.big_n;
        let b = F2mElement::from_words(&[beta], nn).square(&tower.irr);
        let lib = count_e_points(&tower, &F2mElement::from_words(&[a], nn), &b);
        let fast = fast_count_e(&tower.irr, a, beta);
        assert_eq!(lib, fast, "fast #E mismatch");
        checks.push(json!({"check": "fast_count_e == count_e_points", "l": l, "n": n, "a": a, "beta": beta, "E_order": lib}));
    }
    // rho and bsgs verified against planted logs
    let tower = setup(2, 6);
    let nn = tower.big_n;
    let a = F2mElement::zero(nn);
    let b = F2mElement::from_words(&[0x28], nn).square(&tower.irr);
    let ec = ECurve::new(nn, tower.irr.clone(), a, b);
    let mut rng = Rng64::new(5);
    let ne = 4192u64;
    let ell = 131u64;
    let p = pick_point(&tower, 0, 0x28, &ec, ell, ne, &mut rng).expect("point");
    let (mut ok_r, mut ok_b) = (0, 0);
    let mut cost_r = 0.0;
    let mut cost_b = 0.0;
    for d in 1..ell {
        let q = ec.scalar_mul(&p, &BigUint::from(d));
        let mut e1 = Ec {
            c: &ec,
            n: EcCount::default(),
        };
        if rho(&mut e1, &p, &q, ell, 4, &mut Rng64::new(d)).0 == Some(d) {
            ok_r += 1;
        }
        cost_r += e1.n.group_ops() as f64;
        let mut e2 = Ec {
            c: &ec,
            n: EcCount::default(),
        };
        if bsgs(&mut e2, &p, &q, ell) == Some(d) {
            ok_b += 1;
        }
        cost_b += e2.n.group_ops() as f64;
    }
    assert_eq!(ok_r, ell - 1);
    assert_eq!(ok_b, ell - 1);
    checks.push(
        json!({"check": "rho and bsgs recover every d in [1, 130] on order-131 subgroup, N=12",
        "mean_E_ops_rho": cost_r / (ell - 1) as f64, "mean_E_ops_bsgs": cost_b / (ell - 1) as f64,
        "sqrt_ell": (ell as f64).sqrt()}),
    );
    // pricing consistency: 400 E additions run under the ledger and priced from the recorded field
    // operations must land near 400 x the bitop/v1 GroupAdd formula (which assumes 9 adds, 2 mul,
    // 1 sqr and one inversion priced as field_inv)
    {
        let wbits = 8;
        let mut pts: Vec<Pt> = Vec::new();
        while pts.len() < 40 {
            let xv = 1 + rng.below((1u64 << nn) - 1);
            pts.extend(lift_x(
                &tower,
                &F2mElement::zero(nn),
                &ec.b,
                &F2mElement::from_words(&[xv], nn),
            ));
        }
        let t = OperationTally::begin();
        let mut done = 0u64;
        for i in 0..pts.len() {
            for j in 0..pts.len() {
                if i != j && x_of(&pts[i]) != x_of(&pts[j]) {
                    let _ = ec.add(&pts[i], &pts[j]);
                    done += 1;
                    if done >= 400 {
                        break;
                    }
                }
            }
            if done >= 400 {
                break;
            }
        }
        let ledger = t.finish();
        let priced = price_ledger(&ledger, wbits);
        let formula = done as u128 * gates(nn, wbits, Prim::GroupAdd);
        let rel = priced.gates as f64 / formula as f64;
        assert!((0.9..1.1).contains(&rel), "ledger pricing {rel} vs formula");
        checks.push(json!({"check": "ledger-priced E additions vs GroupAdd formula", "additions": done,
            "ledger_gates": priced.gates.to_string(), "formula_gates": formula.to_string(), "ratio": rel}));
    }
    // pricing: one inversion at width 12 is priced as field_inv, its inner mul/sq removed
    let (mi, si) = inverse_inner_counts(12);
    checks.push(json!({"check": "flt_inverse inner counts at m=12", "mul": mi, "square": si}));
    json!({"selftest": "ok", "checks": checks})
}

// ── main ───────────────────────────────────────────────────────────────

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let out = arg(&args, "--out", "");
    let mode = args.first().cloned().unwrap_or_default();
    let l: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(2);
    let n: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(6);
    let result = match mode.as_str() {
        "selftest" => selftest(),
        "fit" => {
            let files: Vec<String> = args
                .iter()
                .skip(1)
                .filter(|a| a.ends_with(".json"))
                .cloned()
                .collect();
            fit(&files)
        }
        "profile" => profile(
            l,
            n,
            arg(&args, "--magic", "0").parse().unwrap(),
            arg(&args, "--maxk", "6").parse().unwrap(),
        ),
        "pick" => pick(
            l,
            n,
            arg(&args, "--magic", "0").parse().unwrap(),
            arg(&args, "--count", "4").parse().unwrap(),
            arg(&args, "--max-cofactor", "8").parse().unwrap(),
            arg(&args, "--samples", "2000").parse().unwrap(),
            arg(&args, "--seed", "1").parse().unwrap(),
        ),
        "run" => {
            let tune: usize = arg(&args, "--tune", "10").parse().unwrap();
            let logs: usize = arg(&args, "--logs", "20").parse().unwrap();
            let steps: usize = arg(&args, "--steps", "8").parse().unwrap();
            let budget: u64 = arg(&args, "--budget", "300000").parse().unwrap();
            let seed: u64 = arg(&args, "--seed", "7").parse().unwrap();
            let scan: Vec<usize> = arg(&args, "--scan", "1,2,3,4,6,8,12,16,24,32,48,64,96,128")
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
            let mut insts = Vec::new();
            for (i, a) in args.iter().enumerate() {
                if a == "--inst" {
                    let v: Vec<u64> = args[i + 1]
                        .split(',')
                        .map(|s| {
                            let s = s.trim();
                            if let Some(h) = s.strip_prefix("0x") {
                                u64::from_str_radix(h, 16).unwrap()
                            } else {
                                s.parse().unwrap()
                            }
                        })
                        .collect();
                    insts.push(Inst {
                        l: v[0] as u32,
                        n: v[1] as u32,
                        a: v[2],
                        beta: v[3],
                    });
                }
            }
            let mut recs = Vec::new();
            for inst in &insts {
                recs.push(run_instance(inst, tune, logs, steps, &scan, budget, seed));
            }
            json!({"unit": "bitop/v1 (docs/units/BITOPS.md); gae = gates / gates(GroupAdd on E)", "instances": recs,
                   "params": {"tune_logs": tune, "holdout_logs": logs, "walk_steps_r": steps, "budget_positions": budget, "seed": seed, "scan": scan}})
        }
        _ => {
            eprintln!("usage: run --inst l,n,a,beta ... | pick L N | profile L N | selftest");
            std::process::exit(2);
        }
    };
    let text = serde_json::to_string_pretty(&result).unwrap();
    if out.is_empty() {
        println!("{text}");
    } else {
        fs::write(&out, text).unwrap();
    }
}
