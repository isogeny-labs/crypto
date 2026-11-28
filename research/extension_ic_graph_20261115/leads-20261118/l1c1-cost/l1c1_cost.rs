//! L1 x C1 descent-cost protocol runner (2x2 factorial on shared descent streams).
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1c1-cost. Frozen with
//! PROTOCOL.md / protocol.json; everything it emits is [pcc:unverified].
//!
//! Subcommands (one process each; one rayon thread; see PROTOCOL.md section 8):
//!   prepare  the pipeline (Run50's deferred-pairs arm and options), D_exact with the
//!            integer-incidence replay of every anchor witness, the certified quotient.
//!   arm      one arm of the 2x2 (base | l1 | c1 | l1c1) on one stream, run as production
//!            would run it, stopping after the residual that brings its two-column affine
//!            system to rank K = 2; [d]G = Q at the end. HOOKS=1 (counted build): bitop/v1
//!            with the complete counted linear stage and per-component tallies. HOOKS=0
//!            (frozen build): thread CPU time of the same window.
//!   audit    the l1_paired.rs instrument on residuals 0..N of the stream: classification of
//!            every F_p-solution, arm A and L1 rows, affine path, certified projection,
//!            elliptic replay of every row, sign-flip negative control. Never charged.
//!   collect  per-target gates and the TSV line the frozen analysis reads.
//!
//! The section between the two banner comments is l1_paired.rs (SHA-256 4a7cafcd...)
//! lines 60-839 byte for byte, with one marked insertion (`audit_chain_push`) after the
//! solution sort in `stream`. PC1 checks that it still reproduces Run50 and steps 2-3.
//! Exit codes: 0 ok; 10 public transfer refused (admission); 11 collection not
//! rank-ready; 1 any other error (the run stops: PROTOCOL.md section 4).
#[path = "../l1-steps-2-3/ffield.rs"]
mod ff;
use crypto_lib::cryptanalysis::{
    f4_fp::{self, F4Options, F4Report, F4Trace, Poly},
    jv_bitops::{self as work, Phase},
    jv_cover::{self as cover, Ctx, Dec6, Div, EllE, Fq3, LinMode, PtE6, StopMode, ZeroDimStats, E2, E6},
    jv_sieve::two_large_prime::CountedRelationSystem,
};
use crypto_lib::run50::extension_deferred_pair_dlp::{run_extension_deferred_pair_dlp_on, DeferredPairDlpOptions};
use crypto_lib::run50::homogeneous_quotient::Quotient;
use ff::{invm, mulm, E, F2};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    fs::OpenOptions,
    io::{BufRead, BufWriter, Write},
    rc::Rc,
    time::Instant,
};

// =====================================================================================
// ===== Audit section: l1_paired.rs lines 60-839 (one marked insertion) ===============
// =====================================================================================
thread_local! {
    static LAST_SOLS: RefCell<Vec<Vec<u64>>> = const { RefCell::new(Vec::new()) };
}
fn production_solver(
    input: &[Poly],
    p: u64,
    opts: &F4Options,
    rng: &mut StdRng,
) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str> {
    let out = cover::solve_zero_dim_mode(input, 6, p, opts, rng, false, LinMode::Fast);
    LAST_SOLS.with(|s| *s.borrow_mut() = out.0.clone());
    Ok(out)
}

// ---- F_p polynomials (little-endian), byte-for-byte the partial_match.rs helpers ----
fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn deg(a: &[u64]) -> isize {
    a.len() as isize - 1
}
fn rem(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = trim(a.to_vec());
    let m = trim(m.to_vec());
    let dm = m.len() - 1;
    let il = invm(m[dm], p);
    while r.len() > dm && !r.is_empty() {
        let c = mulm(*r.last().unwrap(), il, p);
        let sh = r.len() - 1 - dm;
        for (i, &mi) in m.iter().enumerate() {
            r[sh + i] = (r[sh + i] + p - mulm(c, mi, p)) % p;
        }
        r = trim(r);
    }
    r
}
fn div_exact(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = trim(a.to_vec());
    let m = trim(m.to_vec());
    let dm = m.len() - 1;
    let il = invm(m[dm], p);
    let mut q = vec![0u64; r.len().saturating_sub(dm)];
    while r.len() > dm {
        let c = mulm(*r.last().unwrap(), il, p);
        let sh = r.len() - 1 - dm;
        q[sh] = c;
        for (i, &mi) in m.iter().enumerate() {
            r[sh + i] = (r[sh + i] + p - mulm(c, mi, p)) % p;
        }
        r = trim(r);
    }
    assert!(r.is_empty(), "inexact division");
    q
}
fn mulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = (c[i + j] + mulm(x, y, p)) % p;
        }
    }
    rem(&c, m, p)
}
fn compose_pow(h: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = rem(&[1], m, p);
    let mut b = rem(h, m, p);
    let mut e = p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(&r, &b, m, p);
        }
        b = mulmod(&b, &b, m, p);
        e >>= 1;
    }
    r
}
fn psub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + p - b.get(i).copied().unwrap_or(0)) % p).collect())
}
fn gcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !b.is_empty() {
        let r = rem(&a, &b, p);
        a = b;
        b = r;
    }
    if a.is_empty() {
        return a;
    }
    let il = invm(*a.last().unwrap(), p);
    a.iter().map(|&c| mulm(c, il, p)).collect()
}
fn derivative(a: &[u64], p: u64) -> Vec<u64> {
    trim(a.iter().enumerate().skip(1).map(|(i, &c)| mulm(c, i as u64 % p, p)).collect())
}
fn ddf_pattern(f: &[u64], p: u64) -> Option<Vec<usize>> {
    if deg(&gcd(f, &derivative(f, p), p)) > 0 {
        return None;
    }
    let mut f = f.to_vec();
    let mut out = Vec::new();
    let mut h = vec![0u64, 1];
    let mut d = 1usize;
    while deg(&f) >= 2 * d as isize {
        h = compose_pow(&h, &f, p);
        let g = gcd(&psub(&h, &[0, 1], p), &f, p);
        let dg = deg(&g) as usize;
        if dg > 0 {
            for _ in 0..dg / d {
                out.push(d);
            }
            f = div_exact(&f, &g, p);
            h = rem(&h, &f, p);
        }
        d += 1;
    }
    if deg(&f) > 0 {
        out.push(deg(&f) as usize);
    }
    out.sort();
    Some(out)
}
fn label(pat: &[usize]) -> String {
    let mut cnt: BTreeMap<usize, usize> = BTreeMap::new();
    for &d in pat {
        *cnt.entry(d).or_default() += 1;
    }
    cnt.iter().map(|(d, a)| if *a == 1 { format!("{d}") } else { format!("{d}^{a}") }).collect::<Vec<_>>().join(",")
}
fn fp_eval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0u64, |acc, &c| (mulm(acc, x, p) + c) % p)
}
fn e2(v: &Value) -> Result<E2, String> {
    let a = v.as_array().ok_or("e2")?;
    Ok(E2([a[0].as_u64().ok_or("e2")?, a[1].as_u64().ok_or("e2")?]))
}
fn e6(v: &Value) -> Result<E6, String> {
    let a = v.as_array().ok_or("e6")?;
    Ok(E6([e2(&a[0])?, e2(&a[1])?, e2(&a[2])?]))
}
fn pt(v: &Value) -> Result<PtE6, String> {
    Ok(PtE6 { x: e6(&v["x"])?, y: e6(&v["y"])?, inf: v["inf"].as_bool().ok_or("inf")? })
}
fn canonical(f: &F2, u0: u64, u1: u64, v0: E, v1: E) -> ([u64; 6], i8) {
    let key = [u0, u1, v0.0, v0.1, v1.0, v1.1];
    let (n0, n1) = (f.neg(v0), f.neg(v1));
    let neg = [u0, u1, n0.0, n0.1, n1.0, n1.1];
    if neg < key {
        (neg, -1)
    } else {
        (key, 1)
    }
}
fn to_e(x: E2) -> E {
    (x.0[0], x.0[1])
}
fn to_e2(x: E) -> E2 {
    E2([x.0, x.1])
}
fn create(path: &str) -> Result<BufWriter<std::fs::File>, String> {
    Ok(BufWriter::new(OpenOptions::new().write(true).create_new(true).open(path).map_err(|e| format!("{path}: {e}"))?))
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
/// Signed integer coefficient reduced into [0, l).
fn red(x: i128, l: u64) -> u64 {
    x.rem_euclid(l as i128) as u64
}

/// The tallies of partial_match.rs, field for field (the PC1 comparison keys).
#[derive(Default, Clone, serde::Serialize)]
struct Tally {
    residuals: u64,
    solutions: u64,
    non_fp: u64,
    nonsquarefree: u64,
    full_split: u64,
    full_split_valid: u64,
    decompositions: u64,
    patterns: BTreeMap<String, u64>,
    target_solutions: BTreeMap<String, u64>,
    linear_part_valid: BTreeMap<String, u64>,
    classes_formed: u64,
    class_failures: BTreeMap<String, u64>,
    classes_in_d_exact: u64,
    classes_in_d_exact_by_pattern: BTreeMap<String, u64>,
    matched_rows: BTreeMap<String, u64>,
    matched_rows_total: u64,
    class_only_matched: BTreeMap<String, u64>,
    duplicate_class_rows: u64,
    group_check_matched_ok: u64,
    group_check_matched_bad: u64,
    group_check_full_ok: u64,
    group_check_full_bad: u64,
    anchor_identity_ok: u64,
    anchor_identity_bad: u64,
}
fn merge(a: &mut Tally, t: &Tally) {
    a.residuals += t.residuals;
    a.solutions += t.solutions;
    a.non_fp += t.non_fp;
    a.nonsquarefree += t.nonsquarefree;
    a.full_split += t.full_split;
    a.full_split_valid += t.full_split_valid;
    a.decompositions += t.decompositions;
    for (m, src) in [
        (&mut a.patterns, &t.patterns),
        (&mut a.target_solutions, &t.target_solutions),
        (&mut a.linear_part_valid, &t.linear_part_valid),
        (&mut a.class_failures, &t.class_failures),
        (&mut a.classes_in_d_exact_by_pattern, &t.classes_in_d_exact_by_pattern),
        (&mut a.matched_rows, &t.matched_rows),
        (&mut a.class_only_matched, &t.class_only_matched),
    ] {
        for (k, v) in src {
            *m.entry(k.clone()).or_default() += v;
        }
    }
    a.classes_formed += t.classes_formed;
    a.classes_in_d_exact += t.classes_in_d_exact;
    a.matched_rows_total += t.matched_rows_total;
    a.duplicate_class_rows += t.duplicate_class_rows;
    a.group_check_matched_ok += t.group_check_matched_ok;
    a.group_check_matched_bad += t.group_check_matched_bad;
    a.group_check_full_ok += t.group_check_full_ok;
    a.group_check_full_bad += t.group_check_full_bad;
    a.anchor_identity_ok += t.anchor_identity_ok;
    a.anchor_identity_bad += t.anchor_identity_bad;
}

/// A projected affine row: k*kappa + t*d = rhs (mod l).
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize)]
struct Proj {
    k: u64,
    t: u64,
    rhs: u64,
}
#[derive(Clone, Debug, serde::Serialize)]
struct Row {
    kind: &'static str, // "full" (arm A and B) or "l1" (arm B only)
    residual_index: u64,
    pattern: String,
    proj: Proj,
    flipped: Option<Proj>,
    partner: Option<String>,
    candidate: Option<u64>,
    replayed: bool,
    flipped_refused: Option<bool>,
    affine_consistent: Option<bool>,
}

/// Two-column affine solve (the pipeline's CountedRelationSystem), target column 1.
fn solve2(l: u64, a: &Proj, b: &Proj) -> Option<u64> {
    let mut sys = CountedRelationSystem::new(l, 2);
    sys.insert(vec![a.k, a.t], a.rhs);
    sys.insert(vec![b.k, b.t], b.rhs);
    if sys.rank() < 2 {
        return None;
    }
    sys.solved(1)
}

struct Target<'a> {
    p: u64,
    l: u64,
    spec: &'a cover::Spec,
    ctx: &'a Ctx,
    base: &'a [cover::BaseEl],
    by_x: &'a HashMap<u64, usize>,
    d_exact: &'a HashMap<[u64; 6], Vec<(usize, i64)>>,
    quotient: Quotient,
}

#[derive(Default, serde::Serialize)]
struct StreamOut {
    tally_primary: Tally,
    tally_all: Tally,
    rows: Vec<Row>,
    decs_without_valid_pattern: u64,
    valid_full_pattern_without_dec: u64,
    dec_group_bad: u64,
    decomposing_residual_indices: Vec<u64>,
}

/// One paired descent stream: the classification of partial_match.rs plus the
/// arm A rows (pipeline decompositions) and the projected rows of both arms.
fn stream(tg: &mut Target, seed: u64, n: u64, primary_n: u64, rows_path: &str) -> Result<StreamOut, String> {
    let (p, l, spec, ctx, base, by_x) = (tg.p, tg.l, tg.spec, tg.ctx, tg.base, tg.by_x);
    let fl = &ctx.f.f;
    let ff = F2::new(p);
    if ff.w != fl.w {
        return Err("F_{p^2} presentation differs from the library's".into());
    }
    let h: Vec<E> = ctx.cov.hx.iter().map(|&c| to_e(c)).collect();
    let jac = ctx.jac();
    let point_div = |x: u64, y: E| Div { u: vec![to_e2(ff.neg((x, 0))), E2::ONE], v: vec![to_e2(y)] };
    let class_div = |u0: u64, u1: u64, v0: E, v1: E| Div { u: vec![E2([u0, 0]), E2([u1, 0]), E2::ONE], v: vec![to_e2(v0), to_e2(v1)] };
    let signed_mul = |d: &Div<E2>, k: i64| -> Div<E2> {
        let m = jac.mul(d, k.unsigned_abs() as u128);
        if k < 0 {
            jac.neg(&m)
        } else {
            m
        }
    };
    let mut anchor_checked: HashMap<[u64; 6], bool> = HashMap::new();
    let mut out = StreamOut::default();
    let mut w = create(rows_path)?;
    // the pipeline's stream (jv_extension_deferred_pair_dlp.rs, descent loop)
    let mut srng = StdRng::seed_from_u64(seed ^ 0x51E7E);
    let al = work::uniform_mod_u64(&mut srng, 1, l);
    let be = work::uniform_mod_u64(&mut srng, 1, l);
    let step = jac.add(&jac.mul(&spec.gj, al as u128), &jac.mul(&spec.qj, be as u128));
    let mut a = work::uniform_mod_u64(&mut srng, 0, l);
    let mut b = work::uniform_mod_u64(&mut srng, 1, l);
    let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let opts = cover::opts_for_mode(24, 600.0, StopMode::Staircase(64));
    let nb = base.len();
    for i in 0..n {
        let mut t = Tally { residuals: 1, ..Default::default() };
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let forms = cover::weil_forms(fl, &ctx.cov.hx, &residual.u, &v);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        LAST_SOLS.with(|s| s.borrow_mut().clear());
        let (decs, _cost) = cover::nagao_decompose_using(ctx, base, by_x, &residual, &opts, &mut rng, Some(production_solver))?;
        if !decs.is_empty() {
            out.decomposing_residual_indices.push(i);
        }
        t.decompositions = decs.len() as u64;
        // ---- arm A rows: every pipeline-materializer decomposition, group-verified ----
        for dec in &decs {
            let dec: &Dec6 = dec;
            if !cover::verify_dec(&jac, base, &residual, dec) {
                out.dec_group_bad += 1;
                continue;
            }
            let mut dense = vec![0u64; nb + 1];
            for &(idx, e) in &dec.terms {
                dense[idx] = red(e as i128, l);
            }
            dense[nb] = l - b; // b >= 1 on this stream
            let (pr, rhs) = tg.quotient.project(&dense, a)?;
            out.rows.push(Row {
                kind: "full",
                residual_index: i,
                pattern: "1^6".into(),
                proj: Proj { k: pr[0], t: pr[1], rhs },
                flipped: None,
                partner: None,
                candidate: None,
                replayed: false,
                flipped_refused: None,
                affine_consistent: None,
            });
        }
        // ---- classification of every F_p-solution (partial_match.rs) ----
        let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
        sols.sort();
        audit_chain_push(i, &sols); // l1c1-cost: marked insertion (solution-chain digest)
        let ur: Vec<E> = residual.u.iter().map(|&c| to_e(c)).collect();
        let vr: Vec<E> = v.iter().map(|&c| to_e(c)).collect();
        let uc = |k: isize| -> E { if k >= 0 && (k as usize) < ur.len() { ur[k as usize] } else { (0, 0) } };
        let vc = |k: isize| -> E { if k >= 0 && (k as usize) < vr.len() { vr[k as usize] } else { (0, 0) } };
        let mut sol_rows = Vec::new();
        let mut valid_full_here = 0u64;
        for s in &sols {
            t.solutions += 1;
            let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
            let mut coeffs = Vec::with_capacity(7);
            let mut fp = true;
            for k in 0..6 {
                let c = forms.eval(fl, k, &lam);
                fp &= c.0[1] == 0;
                coeffs.push(c.0[0]);
            }
            coeffs.push(1);
            if !fp {
                t.non_fp += 1;
                sol_rows.push(json!({"pattern":"non-Fp"}));
                continue;
            }
            let Some(pat) = ddf_pattern(&coeffs, p) else {
                t.nonsquarefree += 1;
                sol_rows.push(json!({"pattern":"nonsquarefree"}));
                continue;
            };
            let lab = label(&pat);
            *t.patterns.entry(lab.clone()).or_default() += 1;
            if pat.iter().any(|&d| d > 2) {
                sol_rows.push(json!({"pattern":lab}));
                continue;
            }
            let lam_e: [E; 3] = [to_e(lam[0]), to_e(lam[1]), to_e(lam[2])];
            let mu = lam_e[2];
            let ak: Vec<E> = (0..5isize)
                .map(|k| {
                    let mut x = ff.neg(vc(k - 1));
                    x = ff.add(x, ff.mul(lam_e[0], uc(k)));
                    x = ff.add(x, ff.mul(lam_e[1], uc(k - 1)));
                    ff.sub(x, ff.mul(mu, vc(k)))
                })
                .collect();
            let cpoly: Vec<E> = vec![mu, (1, 0)];
            let lhs = ff.psub(&ff.pmul(&ff.pmul(&cpoly, &cpoly), &h), &ff.pmul(&ak, &ak));
            let s_e: Vec<E> = coeffs.iter().map(|&c| (c, 0)).collect();
            if lhs != ff.pmul(&ur, &s_e) {
                return Err(format!("norm identity C^2H - A^2 = u_R S fails at residual {i}"));
            }
            let roots: Vec<u64> = (0..p).filter(|&x| fp_eval(&coeffs, x, p) == 0).collect();
            let mut cof = coeffs.clone();
            for &x in &roots {
                cof = div_exact(&cof, &[(p - x) % p, 1], p);
            }
            let mut quads: Vec<(u64, u64, E)> = Vec::new();
            if cof.len() > 1 {
                let cof_e: Vec<E> = cof.iter().map(|&c| (c, 0)).collect();
                for re in 0..p {
                    for im in 1..=(p - 1) / 2 {
                        let r = (re, im);
                        if ff.peval(&cof_e, r) == (0, 0) {
                            let u1 = mulm(2 * re % p, p - 1, p);
                            let u0 = (mulm(re, re, p) + p - mulm(ff.w, mulm(im, im, p), p)) % p;
                            quads.push((u0, u1, r));
                        }
                    }
                }
            }
            let target = matches!(lab.as_str(), "1^4,2" | "1^2,2^2" | "2^3");
            if quads.len() * 2 + roots.len() != 6 {
                return Err(format!("factorization incomplete at residual {i}: {lab}"));
            }
            if lab == "1^6" {
                t.full_split += 1;
            }
            let mut lin_ok = true;
            let mut points = Vec::new();
            for &x in &roots {
                let xe = (x, 0);
                let Some(&bi) = by_x.get(&x) else {
                    lin_ok = false;
                    break;
                };
                let cx = ff.add(mu, xe);
                if cx == (0, 0) {
                    lin_ok = false;
                    break;
                }
                let y = ff.neg(ff.mul(ff.peval(&ak, xe), ff.inv(cx)));
                let by = to_e(base[bi].y);
                if y == (0, 0) || ff.mul(y, y) != ff.peval(&h, xe) || (y != by && y != ff.neg(by)) {
                    lin_ok = false;
                    break;
                }
                points.push((x, y, bi, if y == by { -1i64 } else { 1i64 }));
            }
            if lab == "1^6" && lin_ok {
                t.full_split_valid += 1;
                valid_full_here += 1;
                let mut acc = jac.add(&residual, &jac.identity());
                for &(x, y, _, _) in &points {
                    acc = jac.add(&acc, &point_div(x, y));
                }
                if jac.is_identity(&acc) {
                    t.group_check_full_ok += 1;
                } else {
                    t.group_check_full_bad += 1;
                }
            }
            if !target {
                sol_rows.push(json!({"pattern":lab,"linear_valid":lin_ok}));
                continue;
            }
            *t.target_solutions.entry(lab.clone()).or_default() += 1;
            *t.linear_part_valid.entry(lab.clone()).or_default() += u64::from(lin_ok);
            let mut classes = Vec::new();
            let mut all_ok = true;
            let mut all_in = true;
            for &(u0, u1, r) in &quads {
                t.classes_formed += 1;
                let rb = (r.0, (p - r.1) % p);
                let uu: Vec<E> = vec![(u0, 0), (u1, 0), (1, 0)];
                let mut fail = None;
                let (cr, crb) = (ff.add(mu, r), ff.add(mu, rb));
                if cr == (0, 0) || crb == (0, 0) {
                    fail = Some("C shares a root with U");
                } else if ff.peval(&ur, r) == (0, 0) || ff.peval(&ur, rb) == (0, 0) {
                    fail = Some("u_R shares a root with U");
                }
                if fail.is_none() {
                    let y1 = ff.neg(ff.mul(ff.peval(&ak, r), ff.inv(cr)));
                    let y2 = ff.neg(ff.mul(ff.peval(&ak, rb), ff.inv(crb)));
                    let v1 = ff.mul(ff.sub(y1, y2), ff.inv(ff.sub(r, rb)));
                    let v0 = ff.sub(y1, ff.mul(v1, r));
                    let vv = ff.pmul(&[v0, v1], &[v0, v1]);
                    if y1 == (0, 0) || y2 == (0, 0) {
                        fail = Some("ramified point");
                    } else if !ff.prem(&ff.psub(&vv, &h), &uu).is_empty() {
                        fail = Some("V^2 != H mod U");
                    } else {
                        let (key, coef) = canonical(&ff, u0, u1, v0, v1);
                        let hit = tg.d_exact.contains_key(&key);
                        classes.push((key, coef, u0, u1, v0, v1, hit));
                        if hit {
                            t.classes_in_d_exact += 1;
                            *t.classes_in_d_exact_by_pattern.entry(lab.clone()).or_default() += 1;
                        }
                        all_in &= hit;
                    }
                }
                if let Some(reason) = fail {
                    *t.class_failures.entry(reason.to_string()).or_default() += 1;
                    all_ok = false;
                    all_in = false;
                }
            }
            let distinct = {
                let mut ks: Vec<_> = classes.iter().map(|c| c.0).collect();
                ks.sort();
                ks.dedup();
                ks.len() == classes.len()
            };
            if !distinct {
                t.duplicate_class_rows += 1;
            }
            if all_ok && all_in {
                *t.class_only_matched.entry(lab.clone()).or_default() += 1;
            }
            let matched = all_ok && all_in && lin_ok && distinct && classes.len() == quads.len();
            let mut group_ok = None;
            if matched {
                *t.matched_rows.entry(lab.clone()).or_default() += 1;
                t.matched_rows_total += 1;
                let mut acc = jac.add(&residual, &jac.identity());
                for &(x, y, _, _) in &points {
                    acc = jac.add(&acc, &point_div(x, y));
                }
                for &(_, _, u0, u1, v0, v1, _) in &classes {
                    let d = class_div(u0, u1, v0, v1);
                    if !jac.is_valid(&d) {
                        return Err("library rejects a matched class as a Mumford pair".into());
                    }
                    acc = jac.add(&acc, &d);
                }
                let gok = jac.is_identity(&acc);
                group_ok = Some(gok);
                if gok {
                    t.group_check_matched_ok += 1;
                } else {
                    t.group_check_matched_bad += 1;
                }
                let mut aok_all = true;
                for &(key, _, u0, u1, _, _, _) in &classes {
                    let aok = *anchor_checked.entry(key).or_insert_with(|| {
                        let dkey = class_div(u0, u1, (key[2], key[3]), (key[4], key[5]));
                        let mut acc = dkey;
                        for &(bi, c) in &tg.d_exact[&key] {
                            acc = jac.add(&acc, &signed_mul(&base[bi].d, c));
                        }
                        jac.is_identity(&acc)
                    });
                    if aok {
                        t.anchor_identity_ok += 1;
                    } else {
                        t.anchor_identity_bad += 1;
                        aok_all = false;
                    }
                }
                // ---- L1 affine row: R = sum e_i B_i - sum coef_j D_key_j, D_key = -sum small B ----
                if gok && aok_all {
                    let mut c = vec![0i128; nb];
                    for &(_, _, bi, e) in &points {
                        c[bi] += e as i128;
                    }
                    for &(key, coef, ..) in &classes {
                        for &(bi, sm) in &tg.d_exact[&key] {
                            c[bi] += coef as i128 * sm as i128;
                        }
                    }
                    let build = |c: &[i128], tg: &mut Target| -> Result<Proj, String> {
                        let mut dense: Vec<u64> = c.iter().map(|&x| red(x, l)).collect();
                        dense.push(l - b);
                        let (pr, rhs) = tg.quotient.project(&dense, a)?;
                        Ok(Proj { k: pr[0], t: pr[1], rhs })
                    };
                    let proj = build(&c, tg)?;
                    // negative control: flip the sign of the first class
                    let (k0, coef0, ..) = classes[0];
                    let mut cf = c.clone();
                    for &(bi, sm) in &tg.d_exact[&k0] {
                        cf[bi] -= 2 * coef0 as i128 * sm as i128;
                    }
                    let flipped = build(&cf, tg)?;
                    out.rows.push(Row {
                        kind: "l1",
                        residual_index: i,
                        pattern: lab.clone(),
                        proj,
                        flipped: Some(flipped),
                        partner: None,
                        candidate: None,
                        replayed: false,
                        flipped_refused: None,
                        affine_consistent: None,
                    });
                }
            }
            sol_rows.push(json!({"pattern":lab,"linear_valid":lin_ok,
                "classes":classes.iter().map(|c| json!({"key":c.0,"coefficient":c.1,"in_d_exact":c.6})).collect::<Vec<_>>(),
                "class_failures":quads.len()-classes.len(),"matched_row":matched,"group_identity":group_ok}));
        }
        // decomposition-level vs pattern-level full splits (differ only through mu(x) = 0 roots)
        let nd = decs.len() as u64;
        if nd > valid_full_here {
            out.decs_without_valid_pattern += nd - valid_full_here;
        } else {
            out.valid_full_pattern_without_dec += valid_full_here - nd;
        }
        writeln!(w, "{}", json!({"residual_index":i,"a":a,"b":b,"solutions":sols.len(),"decompositions":decs.len(),"sols":sol_rows}))
            .map_err(|e| e.to_string())?;
        merge(&mut out.tally_all, &t);
        if i < primary_n {
            merge(&mut out.tally_primary, &t);
        }
        residual = jac.add(&residual, &step);
        a = ((a as u128 + al as u128) % l as u128) as u64;
        b = ((b as u128 + be as u128) % l as u128) as u64;
    }
    w.flush().map_err(|e| e.to_string())?;
    Ok(out)
}

/// Replay every row (both kinds) and every flipped L1 row; fill partner/candidate.
/// Partners: the pipeline's projected rows when its scalar verified, else other
/// stream rows (full rows first, then L1 rows, stream order), first three of rank 2.
fn replay(rows: &mut [Row], pipeline_rows: &[Proj], dstar: Option<u64>, l: u64, elliptic: &dyn Fn(u64) -> bool) {
    let kappa = dstar.and_then(|d| {
        let r = pipeline_rows.first()?;
        if r.k == 0 {
            return None;
        }
        // kappa = (rhs - t d) / k
        let num = (r.rhs as i128 - (r.t as u128 * d as u128 % l as u128) as i128).rem_euclid(l as i128) as u64;
        Some(mulm(num, invm(r.k, l), l))
    });
    let order: Vec<usize> = (0..rows.len()).filter(|&j| rows[j].kind == "full").chain((0..rows.len()).filter(|&j| rows[j].kind == "l1")).collect();
    let snapshot: Vec<Proj> = rows.iter().map(|r| r.proj).collect();
    for idx in 0..rows.len() {
        let me = rows[idx].proj;
        let mut tried: Vec<(String, Proj)> = Vec::new();
        if dstar.is_some() {
            for (j, pr) in pipeline_rows.iter().enumerate() {
                if *pr != me && solve2(l, &me, pr).is_some() {
                    tried.push((format!("pipeline#{j}"), *pr));
                    break;
                }
            }
        } else {
            for &j in &order {
                if j != idx && snapshot[j] != me && solve2(l, &me, &snapshot[j]).is_some() {
                    tried.push((format!("stream#{j}"), snapshot[j]));
                    if tried.len() == 3 {
                        break;
                    }
                }
            }
        }
        let mut ok = false;
        let mut used = None;
        let mut cand = None;
        for (name, pr) in &tried {
            let d = solve2(l, &me, pr);
            if let Some(d) = d {
                cand = Some(d);
                used = Some(name.clone());
                if elliptic(d) {
                    ok = true;
                    break;
                }
            }
        }
        rows[idx].partner = used;
        rows[idx].candidate = cand;
        rows[idx].replayed = ok;
        if let Some(fp) = rows[idx].flipped {
            let refused = match tried.first() {
                Some((_, pr)) => !solve2(l, &fp, pr).is_some_and(elliptic),
                None => true,
            };
            rows[idx].flipped_refused = Some(refused);
        }
        if let (Some(d), Some(k)) = (dstar, kappa) {
            let lhs = ((me.k as u128 * k as u128 + me.t as u128 * d as u128) % l as u128) as u64;
            rows[idx].affine_consistent = Some(lhs == me.rhs);
        }
    }
}

fn summarize(
    tag: &str,
    out: &StreamOut,
    n: u64,
    dstar: Option<u64>,
    owner: Option<u64>,
    l: u64,
) -> (Value, BTreeMap<&'static str, String>) {
    let rows = &out.rows;
    let cnt = |f: &dyn Fn(&Row) -> bool| rows.iter().filter(|r| f(r)).count() as u64;
    let f_rows = cnt(&|r| r.kind == "full");
    let f_rep = cnt(&|r| r.kind == "full" && r.replayed);
    let m_rows = cnt(&|r| r.kind == "l1");
    let m_rep = cnt(&|r| r.kind == "l1" && r.replayed);
    let m_rep_pat = |pat: &str| cnt(&|r| r.kind == "l1" && r.replayed && r.pattern == pat);
    let unrep = cnt(&|r| r.partner.is_none());
    let fail_f = cnt(&|r| r.kind == "full" && r.partner.is_some() && !r.replayed);
    let fail_m = cnt(&|r| r.kind == "l1" && r.partner.is_some() && !r.replayed);
    let flip_total = cnt(&|r| r.flipped.is_some());
    let flip_ref = cnt(&|r| r.flipped_refused == Some(true));
    let aff_bad = cnt(&|r| r.affine_consistent == Some(false));
    // residual-level successes and first/second success per arm
    let mut res_a: Vec<u64> = rows.iter().filter(|r| r.kind == "full" && r.replayed).map(|r| r.residual_index).collect();
    let mut res_b: Vec<u64> = rows.iter().filter(|r| r.replayed).map(|r| r.residual_index).collect();
    res_a.sort();
    res_a.dedup();
    res_b.sort();
    res_b.dedup();
    let at = |v: &[u64], k: usize| v.get(k).map(|&x| x as i64).unwrap_or(-1);
    let owner_check: i64 = match owner {
        None => -1,
        Some(o) => i64::from(dstar == Some(o) && rows.iter().filter(|r| r.replayed).all(|r| r.candidate == Some(o))),
    };
    let t = &out.tally_all;
    let pat = |k: &str| t.patterns.get(k).copied().unwrap_or(0);
    let mut cols: BTreeMap<&'static str, String> = BTreeMap::new();
    cols.insert("n_residuals", n.to_string());
    cols.insert("solutions", t.solutions.to_string());
    cols.insert("pat_1^6", pat("1^6").to_string());
    cols.insert("pat_1^4,2", pat("1^4,2").to_string());
    cols.insert("pat_1^2,2^2", pat("1^2,2^2").to_string());
    cols.insert("pat_2^3", pat("2^3").to_string());
    cols.insert("lin_valid_1^4,2", t.linear_part_valid.get("1^4,2").copied().unwrap_or(0).to_string());
    cols.insert("lin_valid_1^2,2^2", t.linear_part_valid.get("1^2,2^2").copied().unwrap_or(0).to_string());
    cols.insert("lin_valid_2^3", t.linear_part_valid.get("2^3").copied().unwrap_or(0).to_string());
    cols.insert("classes_formed", t.classes_formed.to_string());
    cols.insert("classes_in_dexact", t.classes_in_d_exact.to_string());
    cols.insert("F_rows", f_rows.to_string());
    cols.insert("F", f_rep.to_string());
    cols.insert("M_rows", m_rows.to_string());
    cols.insert("M", m_rep.to_string());
    cols.insert("M_1^4,2", m_rep_pat("1^4,2").to_string());
    cols.insert("M_1^2,2^2", m_rep_pat("1^2,2^2").to_string());
    cols.insert("M_2^3", m_rep_pat("2^3").to_string());
    cols.insert("group_bad", (out.dec_group_bad + t.group_check_full_bad + t.group_check_matched_bad).to_string());
    cols.insert("anchor_bad", t.anchor_identity_bad.to_string());
    cols.insert("replay_fail_F", fail_f.to_string());
    cols.insert("replay_fail_M", fail_m.to_string());
    cols.insert("unreplayable", unrep.to_string());
    cols.insert("flip_total", flip_total.to_string());
    cols.insert("flip_refused", flip_ref.to_string());
    cols.insert("affine_inconsistent", aff_bad.to_string());
    cols.insert("S_A", res_a.len().to_string());
    cols.insert("S_B", res_b.len().to_string());
    cols.insert("first_A", at(&res_a, 0).to_string());
    cols.insert("first_B", at(&res_b, 0).to_string());
    cols.insert("second_A", at(&res_a, 1).to_string());
    cols.insert("second_B", at(&res_b, 1).to_string());
    cols.insert("owner_check", owner_check.to_string());
    cols.insert("decs_vs_pattern_mismatch", (out.decs_without_valid_pattern + out.valid_full_pattern_without_dec).to_string());
    let summary = json!({"tag":tag,"l":l,"n_residuals":n,"columns":cols,"tally_primary":out.tally_primary,"tally_all":out.tally_all,
        "decomposing_residual_indices":out.decomposing_residual_indices,"dec_group_bad":out.dec_group_bad,
        "decs_without_valid_pattern":out.decs_without_valid_pattern,"valid_full_pattern_without_dec":out.valid_full_pattern_without_dec});
    (summary, cols)
}

// =====================================================================================
// ===== New in l1c1-cost (everything below this line; the audit section above is ======
// ===== l1_paired.rs lines 60-839 byte for byte, plus the two marked SOL_CHAIN lines) ==
// =====================================================================================

thread_local! {
    /// Audit: the solution-chain digest after each residual (marked insertion in `stream`).
    static SOL_CHAIN: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    /// C1: the one trace per process (descent-finder traced_solver).
    static TRACE: RefCell<Option<Rc<F4Trace>>> = const { RefCell::new(None) };
    static TTALLY: RefCell<TraceTally> = RefCell::new(TraceTally::default());
    /// Learning-run RREF bound (multiply-adds) not yet charged by the arm loop.
    static PENDING_BOUND: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// FNV-1a over u64 words: an output digest, not a security primitive.
fn fnv(h: &mut u64, x: u64) {
    for byte in x.to_le_bytes() {
        *h ^= byte as u64;
        *h = h.wrapping_mul(0x100_0000_01b3);
    }
}
/// The solution chain: h_{-1} = FNV offset; h_i = fnv(h_{i-1}; i, |sols|, sorted sols).
fn chain_step(h: &mut u64, i: u64, sols: &[Vec<u64>]) {
    fnv(h, i);
    fnv(h, sols.len() as u64);
    for s in sols {
        for &x in s {
            fnv(h, x);
        }
    }
}
const CHAIN0: u64 = 0xcbf2_9ce4_8422_2325;
/// Marked insertion point used by the audit copy of `stream` (after `sols.sort()`).
fn audit_chain_push(i: u64, sols: &[Vec<u64>]) {
    SOL_CHAIN.with(|c| {
        let mut c = c.borrow_mut();
        let mut h = c.last().copied().unwrap_or(CHAIN0);
        chain_step(&mut h, i, sols);
        c.push(h);
    });
}

// ---------------- C1: F4 trace replay (descent-finder traced_solver, unchanged logic) ----------------
const VARS: usize = 6;
const STAIRCASE: usize = 64;

#[derive(Clone, Debug, Default, serde::Serialize)]
struct TraceTally {
    learned: u64,
    replay_attempts: u64,
    accepted: u64,
    accepted_after_mismatch: u64,
    rejected: u64,
    fallback_f4_ops: u64,
    rejected_replay_f4_ops: u64,
    learning_rref_bound: u64,
}
fn certified(r: &F4Report) -> bool {
    !r.timed_out && r.pairs_above_bound == 0 && (r.inconsistent || r.staircase_at_stop.is_some_and(|c| c <= STAIRCASE))
}
fn learning_bound(r: &F4Report) -> u64 {
    let (rows, cols) = (r.max_rows as u64, r.max_cols as u64);
    let rank = rows.min(cols);
    (r.steps as u64).saturating_mul(rows * cols * rank + rows * rank * rows.div_ceil(64))
}
fn finish(r: F4Report, charged_f4: u64, input: &[Poly], p: u64, rng: &mut StdRng, replayed: bool, mismatch: bool) -> (Vec<Vec<u64>>, ZeroDimStats) {
    let t_lin = Instant::now();
    let mut st = ZeroDimStats {
        f4_muls: charged_f4,
        degree_reached: r.degree_reached,
        solving_degree: r.solving_degree,
        max_rows: r.max_rows,
        max_cols: r.max_cols,
        f4_ms: r.ms,
        stopped_at: r.staircase_at_stop,
        replayed,
        trace_mismatch: mismatch,
        ..ZeroDimStats::default()
    };
    if r.timed_out {
        st.timed_out = true;
        return (Vec::new(), st);
    }
    if r.inconsistent {
        st.inconsistent = true;
        return (Vec::new(), st);
    }
    if r.pairs_above_bound > 0 {
        st.incomplete = true;
        return (Vec::new(), st);
    }
    let sols = cover::linear_stage_from_basis(&r.basis, input, VARS, p, rng, LinMode::Fast, &mut st, t_lin);
    (sols, st)
}
/// The opt-in C1 solver: one staircase-stopped trace per process, replay accepted
/// under `certified`; a rejected replay falls back to plain staircase F4 and both
/// runs are charged (every F4 operation of both goes through the jv_bitops hooks).
fn traced_solver(input: &[Poly], p: u64, opts: &F4Options, rng: &mut StdRng) -> Result<(Vec<Vec<u64>>, ZeroDimStats), &'static str> {
    let have = TRACE.with(|t| t.borrow().clone());
    let out = match have {
        Some(trace) => {
            let r = f4_fp::f4_replay(input, VARS, p, opts, &trace);
            TTALLY.with(|t| t.borrow_mut().replay_attempts += 1);
            if certified(&r) {
                let mismatch = r.trace_mismatch;
                TTALLY.with(|t| {
                    let mut t = t.borrow_mut();
                    t.accepted += 1;
                    t.accepted_after_mismatch += u64::from(mismatch);
                });
                let ops = r.field_ops;
                finish(r, ops, input, p, rng, true, mismatch)
            } else {
                let rejected = r.field_ops;
                let full = f4_fp::f4(input, VARS, p, opts);
                TTALLY.with(|t| {
                    let mut t = t.borrow_mut();
                    t.rejected += 1;
                    t.rejected_replay_f4_ops += rejected;
                    t.fallback_f4_ops += full.field_ops;
                });
                let ops = rejected + full.field_ops;
                finish(full, ops, input, p, rng, false, true)
            }
        }
        None => {
            let (r, trace) = f4_fp::f4_record(input, VARS, p, opts);
            let bound = learning_bound(&r);
            TTALLY.with(|t| {
                let mut t = t.borrow_mut();
                t.learned += 1;
                t.learning_rref_bound += bound;
            });
            PENDING_BOUND.with(|c| c.set(c.get() + bound));
            if certified(&r) && trace.stopped && !r.inconsistent {
                TRACE.with(|t| *t.borrow_mut() = Some(Rc::new(trace)));
            }
            let ops = r.field_ops;
            finish(r, ops, input, p, rng, false, false)
        }
    };
    if out.0.iter().any(|s| s.len() != VARS || s.iter().any(|&x| x >= p)) {
        return Err("noncanonical traced root");
    }
    LAST_SOLS.with(|s| *s.borrow_mut() = out.0.clone());
    Ok(out)
}

// ---------------- counted F_p / F_{p^2} arithmetic for the L1 matcher ----------------
/// Every operation goes through the jv_bitops hooks (field_mul / field_add /
/// field_inv at the width of p), so the L1 matcher's work is priced like the
/// library's own descent arithmetic. Presentation as ffield.rs (w = least non-residue).
struct CF {
    p: u64,
    w: u64,
}
impl CF {
    fn m(&self, a: u64, b: u64) -> u64 {
        work::hook_mul(self.p);
        mulm(a, b, self.p)
    }
    fn a(&self, a: u64, b: u64) -> u64 {
        work::hook_add(self.p);
        (a + b) % self.p
    }
    fn s(&self, a: u64, b: u64) -> u64 {
        work::hook_add(self.p);
        (a + self.p - b) % self.p
    }
    fn n(&self, a: u64) -> u64 {
        work::hook_add(self.p);
        (self.p - a) % self.p
    }
    fn i(&self, a: u64) -> u64 {
        work::hook_inv(self.p);
        invm(a, self.p)
    }
    // ---- F_p[x], little-endian, trimmed ----
    fn rem(&self, a: &[u64], md: &[u64]) -> Vec<u64> {
        let mut r = trim(a.to_vec());
        let md = trim(md.to_vec());
        let dm = md.len() - 1;
        let il = if md[dm] == 1 { 1 } else { self.i(md[dm]) };
        while r.len() > dm && !r.is_empty() {
            let lc = *r.last().unwrap();
            let c = if il == 1 { lc } else { self.m(lc, il) };
            let sh = r.len() - 1 - dm;
            for (k, &mk) in md.iter().enumerate() {
                if k == dm {
                    r[sh + k] = 0;
                } else {
                    let t = self.m(c, mk);
                    r[sh + k] = self.s(r[sh + k], t);
                }
            }
            r = trim(r);
        }
        r
    }
    fn mulmod(&self, a: &[u64], b: &[u64], md: &[u64]) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut c = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                let t = self.m(x, y);
                c[i + j] = self.a(c[i + j], t);
            }
        }
        self.rem(&c, md)
    }
    fn powmod(&self, b: &[u64], mut e: u64, md: &[u64]) -> Vec<u64> {
        let mut r = self.rem(&[1], md);
        let mut b = self.rem(b, md);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mulmod(&r, &b, md);
            }
            e >>= 1;
            if e > 0 {
                b = self.mulmod(&b, &b, md);
            }
        }
        r
    }
    fn psub(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let n = a.len().max(b.len());
        trim((0..n).map(|i| self.s(a.get(i).copied().unwrap_or(0), b.get(i).copied().unwrap_or(0))).collect())
    }
    fn monic(&self, a: Vec<u64>) -> Vec<u64> {
        if a.is_empty() || *a.last().unwrap() == 1 {
            return a;
        }
        let il = self.i(*a.last().unwrap());
        a.iter().map(|&c| self.m(c, il)).collect()
    }
    fn gcd(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
        while !b.is_empty() {
            let r = self.rem(&a, &b);
            a = b;
            b = r;
        }
        self.monic(a)
    }
    fn div_exact(&self, a: &[u64], md: &[u64]) -> Result<Vec<u64>, String> {
        let mut r = trim(a.to_vec());
        let md = trim(md.to_vec());
        let dm = md.len() - 1;
        let il = if md[dm] == 1 { 1 } else { self.i(md[dm]) };
        let mut q = vec![0u64; r.len().saturating_sub(dm)];
        while r.len() > dm {
            let lc = *r.last().unwrap();
            let c = if il == 1 { lc } else { self.m(lc, il) };
            let sh = r.len() - 1 - dm;
            q[sh] = c;
            for (k, &mk) in md.iter().enumerate() {
                if k == dm {
                    r[sh + k] = 0;
                } else {
                    let t = self.m(c, mk);
                    r[sh + k] = self.s(r[sh + k], t);
                }
            }
            r = trim(r);
        }
        if !r.is_empty() {
            return Err("L1 matcher: inexact division".into());
        }
        Ok(q)
    }
    fn derivative(&self, a: &[u64]) -> Vec<u64> {
        trim(a.iter().enumerate().skip(1).map(|(i, &c)| self.m(c, i as u64 % self.p)).collect())
    }
    /// Equal-degree splitting (Cantor-Zassenhaus with the deterministic shifts
    /// delta = 0, 1, 2, ...): f monic squarefree with every irreducible factor of
    /// degree d (1 or 2). Pushes the monic factors.
    fn edf(&self, f: Vec<u64>, d: usize, out: &mut Vec<Vec<u64>>) -> Result<(), String> {
        let n = f.len() - 1;
        if n == d {
            out.push(f);
            return Ok(());
        }
        let q = if d == 1 { self.p } else { self.p * self.p };
        let e = (q - 1) / 2;
        for delta in 0..64u64 {
            let h = self.powmod(&[delta % self.p, 1], e, &f);
            let g = self.gcd(&self.psub(&h, &[1]), &f);
            let dg = g.len().saturating_sub(1);
            if dg > 0 && dg < n {
                let other = self.div_exact(&f, &g)?;
                self.edf(g, d, out)?;
                self.edf(other, d, out)?;
                return Ok(());
            }
        }
        Err("L1 matcher: equal-degree splitting found no split in 64 shifts".into())
    }
    // ---- F_{p^2} ----
    fn ea(&self, x: E, y: E) -> E {
        (self.a(x.0, y.0), self.a(x.1, y.1))
    }
    fn es(&self, x: E, y: E) -> E {
        (self.s(x.0, y.0), self.s(x.1, y.1))
    }
    fn en(&self, x: E) -> E {
        (self.n(x.0), self.n(x.1))
    }
    fn em(&self, x: E, y: E) -> E {
        let t = self.m(x.1, y.1);
        let re = self.a(self.m(x.0, y.0), self.m(self.w, t));
        let im = self.a(self.m(x.0, y.1), self.m(x.1, y.0));
        (re, im)
    }
    /// F_{p^2} times F_p.
    fn ek(&self, x: E, k: u64) -> E {
        (self.m(x.0, k), self.m(x.1, k))
    }
    fn einv(&self, x: E) -> E {
        let t = self.m(x.1, x.1);
        let nrm = self.s(self.m(x.0, x.0), self.m(self.w, t));
        let ni = self.i(nrm);
        (self.m(x.0, ni), self.m(self.n(x.1), ni))
    }
    /// Horner evaluation of an F_{p^2} polynomial at x in F_p.
    fn eval_fp(&self, a: &[E], x: u64) -> E {
        let mut acc = (0, 0);
        for &c in a.iter().rev() {
            acc = self.ea(self.ek(acc, x), c);
        }
        acc
    }
    /// An F_{p^2} polynomial modulo the monic F_p quadratic x^2 + u1 x + u0: (c0, c1).
    fn rem_quad(&self, a: &[E], u0: u64, u1: u64) -> (E, E) {
        let mut c: Vec<E> = a.to_vec();
        while c.len() > 2 {
            let top = c.pop().unwrap();
            let k = c.len(); // top was the coefficient of x^k
            c[k - 1] = self.es(c[k - 1], self.ek(top, u1));
            c[k - 2] = self.es(c[k - 2], self.ek(top, u0));
        }
        c.resize(2, (0, 0));
        (c[0], c[1])
    }
    /// Product of two residues modulo x^2 + u1 x + u0.
    fn mul_quad(&self, x: (E, E), y: (E, E), u0: u64, u1: u64) -> (E, E) {
        let c0 = self.em(x.0, y.0);
        let c1 = self.ea(self.em(x.0, y.1), self.em(x.1, y.0));
        let c2 = self.em(x.1, y.1);
        (self.es(c0, self.ek(c2, u0)), self.es(c1, self.ek(c2, u1)))
    }
    /// Res(x^2 + u1 x + u0, g0 + g1 x) = g0^2 - u1 g0 g1 + u0 g1^2.
    fn res_quad(&self, g: (E, E), u0: u64, u1: u64) -> E {
        let g00 = self.em(g.0, g.0);
        let g01 = self.ek(self.em(g.0, g.1), u1);
        let g11 = self.ek(self.em(g.1, g.1), u0);
        self.ea(self.es(g00, g01), g11)
    }
}

/// One L1 row found by the counted matcher (production checks passed).
struct L1Hit {
    pattern: &'static str,
    coeffs: Vec<i128>, // dense over the factor base
}

/// Per-target L1 state: D_exact (key -> small-base part) and the anchor-check cache.
struct L1State {
    d_exact: HashMap<[u64; 6], Vec<(usize, i64)>>,
    anchor_ok: HashMap<[u64; 6], bool>,
}

/// The counted L1 matcher for one F_p-solution `s` of residual R (production path).
/// Returns Some(row) when every condition of PROTOCOL.md section 1 holds. All its
/// arithmetic is hooked (CF above, or the library's own Jacobian arithmetic).
#[allow(clippy::too_many_arguments)]
fn l1_match(
    cf: &CF,
    ctx: &Ctx,
    jac: &cover::Hyp<cover::Fq>,
    base: &[cover::BaseEl],
    by_x: &HashMap<u64, usize>,
    st: &mut L1State,
    residual: &Div<E2>,
    forms: &cover::WeilForms,
    h: &[E],
    ur: &[E],
    vr: &[E],
    s: &[u64],
) -> Result<Option<L1Hit>, String> {
    let p = cf.p;
    let fl = &ctx.f.f;
    let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
    // the sextic S (monic); an F_{p^2} coefficient means no F_p-sextic
    let mut sx = Vec::with_capacity(7);
    for k in 0..6 {
        let c = forms.eval(fl, k, &lam);
        work::words(1);
        if c.0[1] != 0 {
            return Ok(None);
        }
        sx.push(c.0[0]);
    }
    sx.push(1);
    // squarefree
    let g0 = cf.gcd(&sx, &cf.derivative(&sx));
    if g0.len() > 1 {
        return Ok(None);
    }
    // linear part g1 = gcd(x^p - x, S)
    let hx = cf.powmod(&[0, 1], p, &sx);
    let g1 = cf.gcd(&cf.psub(&hx, &[0, 1]), &sx);
    let d1 = g1.len() - 1;
    if !(d1 == 0 || d1 == 2 || d1 == 4) {
        return Ok(None); // (1^6) is arm A's; odd counts mean a factor of degree >= 3
    }
    let s2 = cf.div_exact(&sx, &g1)?;
    if s2.len() - 1 >= 4 {
        // every factor of S2 has degree 2 iff x^(p^2) = x mod S2
        let hm = cf.rem(&hx, &s2);
        let h2 = cf.powmod(&hm, p, &s2);
        if !cf.psub(&h2, &[0, 1]).is_empty() {
            return Ok(None);
        }
    }
    let pattern: &'static str = match d1 {
        4 => "1^4,2",
        2 => "1^2,2^2",
        _ => "2^3",
    };
    let lam_e: [E; 3] = [to_e(lam[0]), to_e(lam[1]), to_e(lam[2])];
    let mu = lam_e[2];
    let uc = |k: isize| -> E { if k >= 0 && (k as usize) < ur.len() { ur[k as usize] } else { (0, 0) } };
    let vc = |k: isize| -> E { if k >= 0 && (k as usize) < vr.len() { vr[k as usize] } else { (0, 0) } };
    let ak: Vec<E> = (0..5isize)
        .map(|k| {
            let mut x = cf.en(vc(k - 1));
            x = cf.ea(x, cf.em(lam_e[0], uc(k)));
            x = cf.ea(x, cf.em(lam_e[1], uc(k - 1)));
            cf.es(x, cf.em(mu, vc(k)))
        })
        .collect();
    // linear part: roots of g1, each a factor-base abscissa with y = -A(x)/C(x) = +-y_B
    let mut roots = Vec::new();
    if d1 > 0 {
        let mut lin = Vec::new();
        cf.edf(g1, 1, &mut lin)?;
        for f in lin {
            roots.push(cf.n(f[0]));
        }
        roots.sort();
    }
    let mut points = Vec::new();
    for &x in &roots {
        work::probe(1);
        let Some(&bi) = by_x.get(&x) else {
            return Ok(None);
        };
        let cx = cf.ea(mu, (x, 0));
        if cx == (0, 0) {
            return Ok(None);
        }
        let y = cf.en(cf.em(cf.eval_fp(&ak, x), cf.einv(cx)));
        let by = to_e(base[bi].y);
        work::words(4);
        if y == (0, 0) || (y != by && y != cf.en(by)) {
            return Ok(None);
        }
        let yy = cf.em(y, y);
        if yy != cf.eval_fp(h, x) {
            return Ok(None);
        }
        points.push((x, y, bi, if y == by { -1i64 } else { 1i64 }));
    }
    // quadratic part
    let mut quads = Vec::new();
    cf.edf(s2, 2, &mut quads)?;
    let mut classes: Vec<([u64; 6], i8, u64, u64, E, E)> = Vec::new();
    for qd in &quads {
        let (u0, u1) = (qd[0], qd[1]);
        // C = x + mu shares no root with U: N = U(-mu) != 0
        let nn = cf.ea(cf.es(cf.em(mu, mu), cf.ek(mu, u1)), (u0, 0));
        if nn == (0, 0) {
            return Ok(None);
        }
        // u_R shares no root with U
        if cf.res_quad(cf.rem_quad(ur, u0, u1), u0, u1) == (0, 0) {
            return Ok(None);
        }
        // no ramified point: A has no root of U
        let am = cf.rem_quad(&ak, u0, u1);
        if cf.res_quad(am, u0, u1) == (0, 0) {
            return Ok(None);
        }
        // V = -A / C mod U = A (x + u1 - mu) / N mod U
        let cinv_num = (cf.es((u1, 0), mu), (1, 0));
        let vn = cf.mul_quad(am, cinv_num, u0, u1);
        let ni = cf.einv(nn);
        let (v0, v1) = (cf.em(vn.0, ni), cf.em(vn.1, ni));
        // V^2 = H mod U
        let vv = cf.mul_quad((v0, v1), (v0, v1), u0, u1);
        let hm = cf.rem_quad(h, u0, u1);
        work::words(4);
        if vv != hm {
            return Ok(None);
        }
        // signed canonical key and the D_exact probe
        let (n0, n1) = (cf.en(v0), cf.en(v1));
        let key = [u0, u1, v0.0, v0.1, v1.0, v1.1];
        let neg = [u0, u1, n0.0, n0.1, n1.0, n1.1];
        work::words(6);
        let (key, coef) = if neg < key { (neg, -1i8) } else { (key, 1i8) };
        work::probe(6);
        if !st.d_exact.contains_key(&key) {
            return Ok(None);
        }
        classes.push((key, coef, u0, u1, v0, v1));
    }
    // classes pairwise distinct
    work::words((classes.len() * classes.len()) as u64);
    for i in 0..classes.len() {
        for j in 0..i {
            if classes[i].0 == classes[j].0 {
                return Ok(None);
            }
        }
    }
    // principal-divisor identity R + sum P + sum D = 0 (library Cantor arithmetic, hooked)
    let ff = F2::new(p);
    let point_div = |x: u64, y: E| Div { u: vec![to_e2(ff.neg((x, 0))), E2::ONE], v: vec![to_e2(y)] };
    let class_div = |u0: u64, u1: u64, v0: E, v1: E| Div { u: vec![E2([u0, 0]), E2([u1, 0]), E2::ONE], v: vec![to_e2(v0), to_e2(v1)] };
    let mut acc = jac.add(residual, &jac.identity());
    for &(x, y, _, _) in &points {
        acc = jac.add(&acc, &point_div(x, y));
    }
    for &(_, _, u0, u1, v0, v1) in &classes {
        let d = class_div(u0, u1, v0, v1);
        if !jac.is_valid(&d) {
            return Err("library rejects a matched class as a Mumford pair".into());
        }
        acc = jac.add(&acc, &d);
    }
    if !jac.is_identity(&acc) {
        return Ok(None);
    }
    // anchor identity sum small_i B_i + D_key = 0 (once per class per target; hooked)
    for &(key, ..) in &classes {
        let ok = match st.anchor_ok.get(&key) {
            Some(&v) => v,
            None => {
                let dkey = class_div(key[0], key[1], (key[2], key[3]), (key[4], key[5]));
                let mut a2 = dkey;
                for &(bi, c) in &st.d_exact[&key] {
                    let m = jac.mul(&base[bi].d, c.unsigned_abs() as u128);
                    let m = if c < 0 { jac.neg(&m) } else { m };
                    a2 = jac.add(&a2, &m);
                }
                let v = jac.is_identity(&a2);
                st.anchor_ok.insert(key, v);
                v
            }
        };
        if !ok {
            return Ok(None);
        }
    }
    // the affine row: c_i = e_i + sum_j coef_j small_{j,i}
    let mut c = vec![0i128; base.len()];
    for &(_, _, bi, e) in &points {
        c[bi] += e as i128;
    }
    let mut terms = points.len() as u64;
    for &(key, coef, ..) in &classes {
        for &(bi, sm) in &st.d_exact[&key] {
            c[bi] += coef as i128 * sm as i128;
            terms += 1;
        }
    }
    work::words(terms);
    Ok(Some(L1Hit { pattern, coeffs: c }))
}

// ---------------- measurement helpers ----------------
/// CPU time of the calling thread in ns (Linux /proc/thread-self/schedstat, field 1).
fn thread_cpu_ns() -> u64 {
    std::fs::read_to_string("/proc/thread-self/schedstat")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|x| x.parse().ok()))
        .unwrap_or(0)
}
type Widths = crypto_lib::cryptanalysis::bitops::Widths;
fn price_all(t: &work::JvTally, w: &Widths) -> Result<u128, String> {
    work::price(&t.total(), w).map_err(|_| "unpriced primitive".to_string())
}
/// Counted components of one arm window (hooks on), each the jv_bitops delta of its sections.
#[derive(Default)]
struct Comp {
    on: bool,
    parts: BTreeMap<&'static str, work::JvTally>,
}
impl Comp {
    fn run<R>(&mut self, name: &'static str, f: impl FnOnce() -> R) -> R {
        if !self.on {
            return f();
        }
        let b = work::snapshot();
        let r = f();
        let d = work::snapshot().since(&b);
        self.parts.entry(name).or_default().add(&d);
        r
    }
}
/// Rank-raising acceptance into the arm's two-column affine system (K <= 2).
/// The determinant test is charged as two products and one addition mod l.
fn try_accept(acc: &mut Vec<Proj>, pr: Proj, l: u64) -> bool {
    match acc.len() {
        0 => {
            work::words(2);
            if pr.k != 0 || pr.t != 0 {
                acc.push(pr);
                true
            } else {
                false
            }
        }
        1 => {
            let a = acc[0];
            work::hook_muls(l, 2);
            work::hook_add(l);
            let det = (mulm(a.k, pr.t, l) + l - mulm(pr.k, a.t, l)) % l;
            if det != 0 {
                acc.push(pr);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
/// d from two accepted rows k kappa + t d = rhs (counted: 4 products, 2 additions, 1 inversion mod l).
fn solve_two(a: &Proj, b: &Proj, l: u64) -> Option<u64> {
    work::hook_muls(l, 4);
    work::hook_adds(l, 2);
    work::hook_inv(l);
    let det = (mulm(a.k, b.t, l) + l - mulm(b.k, a.t, l)) % l;
    if det == 0 {
        return None;
    }
    let num = (mulm(a.k, b.rhs, l) + l - mulm(b.k, a.rhs, l)) % l;
    Some(mulm(num, invm(det, l), l))
}

struct Loaded {
    public_sha: String,
    p: u64,
    l: u64,
    alpha: E6,
    g: PtE6,
    q: PtE6,
}
fn load_public(path: &str) -> Result<Loaded, String> {
    let public_bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let public_sha = hex(&Sha256::digest(&public_bytes));
    let public: Value = serde_json::from_slice(&public_bytes).map_err(|e| e.to_string())?;
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    if p != 1009 || l >= 1u64 << 62 {
        return Err("domain: this protocol admits p = 1009 and l < 2^62 only".into());
    }
    Ok(Loaded { public_sha, p, l, alpha: e6(&public["alpha"])?, g: pt(&public["g"])?, q: pt(&public["q"])? })
}
fn base_digest(base: &[cover::BaseEl]) -> String {
    let mut h = Sha256::new();
    for b in base {
        h.update(b.x.to_le_bytes());
        h.update(b.y.0[0].to_le_bytes());
        h.update(b.y.0[1].to_le_bytes());
    }
    hex(&h.finalize())
}
fn read_json(path: &str) -> Result<Value, String> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| format!("{path}: {e}"))?).map_err(|e| format!("{path}: {e}"))
}
fn load_dexact(path: &str) -> Result<Vec<([u64; 6], Vec<(usize, i64)>)>, String> {
    let f = std::io::BufReader::new(std::fs::File::open(path).map_err(|e| format!("{path}: {e}"))?);
    let mut out = Vec::new();
    for line in f.lines() {
        let v: Value = serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let mut k = [0u64; 6];
        for (i, x) in k.iter_mut().enumerate() {
            *x = v["class"][i].as_u64().ok_or("class")?;
        }
        let small = v["small"]
            .as_array()
            .ok_or("small")?
            .iter()
            .map(|t| Ok((t[0].as_u64().ok_or("small")? as usize, t[1].as_i64().ok_or("small")?)))
            .collect::<Result<Vec<_>, String>>()?;
        out.push((k, small));
    }
    Ok(out)
}

// ---------------- entry ----------------
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // As Run50's driver: clear JV_* overrides, pin the incidence/shared recipe.
    let overrides: Vec<_> = std::env::vars_os().map(|(k, _)| k).filter(|k| k.to_str().is_some_and(|s| s.starts_with("JV_"))).collect();
    for key in overrides {
        std::env::remove_var(key);
    }
    std::env::set_var("JV_ADDS_INC", "1");
    std::env::set_var("JV_ADDS_SHARE", "1");
    std::env::set_var("JV_SIEVE_PROGRESS", "1");
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().expect("pool");
    let code = pool.install(|| {
        let r = match args.get(1).map(String::as_str) {
            Some("prepare") => cmd_prepare(&args[2..]),
            Some("audit") => cmd_audit(&args[2..]),
            Some("arm") => cmd_arm(&args[2..]),
            Some("collect") => cmd_collect(&args[2..]),
            _ => Err(USAGE.into()),
        };
        match r {
            Ok(()) => 0,
            Err(e) if e == "public transfer failed" => {
                eprintln!("ADMISSION_REFUSED: {e}");
                10
            }
            Err(e) if e == "collection not rank-ready" => {
                eprintln!("COLLECTION_NOT_READY: {e}");
                11
            }
            Err(e) => {
                eprintln!("ERROR: {e}");
                1
            }
        }
    });
    std::process::exit(code);
}

const USAGE: &str = "usage:
  l1c1_cost prepare PUBLIC_JSON COLLECT_SEED STATEDIR
  l1c1_cost audit   PUBLIC_JSON COLLECT_SEED STREAM_SEED N PRIMARY_N STATEDIR TAG OWNER|-
  l1c1_cost arm     PUBLIC_JSON COLLECT_SEED STREAM_SEED NCAP FIXED_N|- STATEDIR ARM HOOKS OUT_PREFIX
  l1c1_cost collect STATEDIR TAG NCAP OWNER|- OUT_TSV
ARM in base|l1|c1|l1c1; HOOKS 1 (counted build: counted metric) or 0 (frozen build: wall metric)";

/// prepare: the pipeline (Run50's deferred-pairs arm and options, as l1_paired.rs),
/// D_exact with the integer-incidence replay of every anchor witness, and the state
/// the arms and the audit read: pipeline.summary.json, dexact.jsonl, quotient.json.
fn cmd_prepare(a: &[String]) -> Result<(), String> {
    if a.len() != 3 {
        return Err(USAGE.into());
    }
    let pb = load_public(&a[0])?;
    let collect_seed: u64 = a[1].parse().map_err(|_| "collect seed")?;
    let outdir = &a[2];
    std::fs::create_dir_all(outdir).map_err(|e| e.to_string())?;
    let path = |f: &str| format!("{outdir}/{f}");
    let (p, l) = (pb.p, pb.l);
    let started = Instant::now();
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let before = work::snapshot();
    let model = work::enter(Phase::Model);
    let spec = cover::spec_from_curve(p, pb.alpha, l, pb.g.clone(), pb.q.clone(), 0).ok_or("public transfer failed")?;
    let mut options = DeferredPairDlpOptions::default();
    options.collection.collection.b_count = Some(1018081);
    options.collection.collection.max_descent_residuals = 8192;
    options.collection.collection.max_accepted_descents = 32;
    options.collection.collection.max_split_records = 524288;
    options.max_deferred_edges = 262144;
    let (report, scalar) = run_extension_deferred_pair_dlp_on(&spec, collect_seed, options)?;
    drop(model);
    let f3 = Fq3::new(p);
    let ec = EllE::new(&f3, &pb.alpha);
    let verified = {
        let _v = work::enter(Phase::Verification);
        scalar.is_some_and(|d| d > 0 && d < l && ec.mul(&pb.g, d) == pb.q)
    };
    let pipeline_ns = started.elapsed().as_nanos();
    let tally = work::snapshot().since(&before);
    let widths = work::widths(p, l);
    let phase_rows: Vec<_> = Phase::IC
        .iter()
        .map(|&phase| json!({"phase":phase.name(),"diagnostic_priced_bitops":work::price(tally.phase(phase),&widths).expect("known widths").to_string()}))
        .collect();
    if !report["collection_rank_ready"].as_bool().unwrap_or(false) || report["initial_basis"].is_null() {
        let mut w = create(&path("pipeline.summary.json"))?;
        writeln!(w, "{}", json!({"schema":"l1-paired.pipeline/v1","public_sha256":pb.public_sha,"collection_rank_ready":false,"counts":report["collection"]["counts"],"stop":report["collection"]["stop"]})).map_err(|e| e.to_string())?;
        return Err("collection not rank-ready".into());
    }
    // D_exact (as l1_paired.rs run(), section 2)
    let coll = &report["collection"];
    let sources = coll["sources"].as_array().ok_or("sources")?;
    let rem_of = |id: usize| -> Result<Vec<([u64; 6], i64)>, String> {
        let s = sources.get(id).ok_or("witness source range")?;
        s["certificate"]["remainders"]
            .as_array()
            .ok_or("remainders")?
            .iter()
            .map(|r| {
                let k = r["key"].as_array().ok_or("key")?;
                let mut key = [0u64; 6];
                for (i, x) in k.iter().enumerate() {
                    key[i] = x.as_u64().ok_or("key")?;
                }
                Ok((key, r["coefficient"].as_i64().ok_or("coef")?))
            })
            .collect()
    };
    let terms_of = |id: usize| -> Result<Vec<(usize, i64)>, String> {
        let s = sources.get(id).ok_or("witness source range")?;
        s["certificate"]["terms"]
            .as_array()
            .ok_or("terms")?
            .iter()
            .map(|t| Ok((t[0].as_u64().ok_or("t")? as usize, t[1].as_i64().ok_or("t")?)))
            .collect()
    };
    type Anchor = (usize, Vec<(usize, i64)>, Vec<(usize, i64)>);
    let mut anchors: BTreeMap<[u64; 6], Anchor> = BTreeMap::new();
    let (mut replay_ok, mut replay_bad) = (0u64, 0u64);
    for (sid, s) in sources.iter().enumerate() {
        for ev in s["graph"].as_array().ok_or("graph")? {
            if ev["kind"] != "anchor" {
                continue;
            }
            let k = ev["class"].as_array().ok_or("class")?;
            let mut key = [0u64; 6];
            for (i, x) in k.iter().enumerate() {
                key[i] = x.as_u64().ok_or("class")?;
            }
            let witness: Vec<(usize, i64)> = ev["witness"]
                .as_array()
                .ok_or("witness")?
                .iter()
                .map(|t| Ok((t[0].as_u64().ok_or("w")? as usize, t[1].as_i64().ok_or("w")?)))
                .collect::<Result<_, String>>()?;
            let mut large: BTreeMap<[u64; 6], i64> = BTreeMap::new();
            let mut small: BTreeMap<usize, i64> = BTreeMap::new();
            for &(id, c) in &witness {
                for (rk, rc) in rem_of(id)? {
                    *large.entry(rk).or_default() += c * rc;
                }
                for (i, sgn) in terms_of(id)? {
                    *small.entry(i).or_default() += c * sgn;
                }
            }
            large.retain(|_, v| *v != 0);
            small.retain(|_, v| *v != 0);
            if large.len() == 1 && large.get(&key) == Some(&1) && witness.iter().all(|&(id, _)| id <= sid) {
                replay_ok += 1;
            } else {
                replay_bad += 1;
            }
            if anchors.insert(key, (sid, witness, small.into_iter().collect())).is_some() {
                return Err("class anchored twice".into());
            }
        }
    }
    {
        let mut w = create(&path("dexact.jsonl"))?;
        for (key, (sid, witness, small)) in &anchors {
            writeln!(w, "{}", json!({"class":key,"activation_source":sid,"witness":witness,"small":small})).map_err(|e| e.to_string())?;
        }
        w.flush().map_err(|e| e.to_string())?;
    }
    if replay_bad != 0 {
        return Err(format!("{replay_bad} anchor witnesses fail the integer incidence replay"));
    }
    let mut quotient: Quotient = serde_json::from_value(report["initial_basis"].clone()).map_err(|e| format!("initial_basis: {e}"))?;
    {
        let mut w = create(&path("quotient.json"))?;
        writeln!(w, "{}", report["initial_basis"]).map_err(|e| e.to_string())?;
    }
    let ctx = cover::Ctx::new(&spec);
    let mut base_rng = StdRng::seed_from_u64(collect_seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let rec = coll["base"].as_array().ok_or("base")?;
    if rec.len() != base.len()
        || rec.iter().zip(&base).any(|(r, b)| r["x"].as_u64() != Some(b.x) || r["y"][0].as_u64() != Some(b.y.0[0]) || r["y"][1].as_u64() != Some(b.y.0[1]))
    {
        return Err("descent factor base differs from the collection's".into());
    }
    let nb = base.len();
    let mut pipeline_rows_match = true;
    let descents = report["descents"].as_array().ok_or("descents")?;
    let recorded = report["projected_rows"].as_array().ok_or("projected_rows")?;
    let mut pipeline_desc = Vec::new();
    for (j, dsc) in descents.iter().enumerate() {
        let a = dsc["a"].as_u64().ok_or("a")?;
        let b = dsc["b"].as_u64().ok_or("b")?;
        let terms = dsc["decomposition"]["terms"].as_array().ok_or("decomposition terms")?;
        let mut dense = vec![0u64; nb + 1];
        let mut tv = Vec::new();
        for t in terms {
            let (i, e) = (t[0].as_u64().ok_or("ti")? as usize, t[1].as_i64().ok_or("te")?);
            dense[i] = red(e as i128, l);
            tv.push((i, e));
        }
        dense[nb] = if b == 0 { 0 } else { l - b };
        let (pr, rhs) = quotient.project(&dense, a)?;
        let proj = Proj { k: pr[0], t: pr[1], rhs };
        let rr = recorded.get(j);
        pipeline_rows_match &= rr.is_some_and(|r| {
            r["coefficients"][0].as_u64() == Some(proj.k) && r["coefficients"][1].as_u64() == Some(proj.t) && r["rhs"].as_u64() == Some(proj.rhs)
        });
        pipeline_desc.push(json!({"residual_index":dsc["residual_index"],"terms":tv,"proj":proj}));
    }
    pipeline_rows_match &= recorded.len() == descents.len();
    let summary = json!({
        "schema":"l1-paired.pipeline/v1","pcc":"unverified","public_sha256":pb.public_sha,"collect_seed":collect_seed,
        "actual_scalar":scalar,"verified":verified,"pipeline_wall_ns_practicality_only":pipeline_ns.to_string(),
        "counts":coll["counts"],"graph_stats":coll["graph_stats"],"stop":coll["stop"],"rank_checks":coll["rank_checks"],
        "checkpoints_len":coll["checkpoints"].as_array().map(|a|a.len()),
        "last_checkpoint":coll["checkpoints"].as_array().and_then(|a|a.last()).cloned(),
        "projections":coll["projections"].as_array().map(|a|a.len()),
        "descent_residuals":report["descent_residuals"],"accepted_descents":descents.len(),
        "collection_rank_ready":report["collection_rank_ready"],
        "screen_calls":coll["discriminant_screen"]["calls"],"screen_refusals":coll["discriminant_screen"]["refusals"].as_array().map(|a|a.len()),
        "base_len":nb,"d_exact_classes":anchors.len(),"anchor_witness_replay_ok":replay_ok,"anchor_witness_replay_bad":replay_bad,
        "pipeline_rows_match":pipeline_rows_match,"pipeline_descents":pipeline_desc,"phases":phase_rows,
        "l1c1_base_sha256":base_digest(&base),
    });
    let mut w = create(&path("pipeline.summary.json"))?;
    writeln!(w, "{}", serde_json::to_string_pretty(&summary).unwrap()).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// Shared set-up of audit and arm processes (outside every measured window).
struct Setup {
    pb: Loaded,
    spec: cover::Spec,
    ctx: Ctx,
    base: Vec<cover::BaseEl>,
    by_x: HashMap<u64, usize>,
    quotient: Quotient,
    dexact: Vec<([u64; 6], Vec<(usize, i64)>)>,
    pipe: Value,
}
fn setup(public: &str, collect_seed: u64, statedir: &str) -> Result<Setup, String> {
    let pb = load_public(public)?;
    let pipe = read_json(&format!("{statedir}/pipeline.summary.json"))?;
    if pipe["public_sha256"].as_str() != Some(pb.public_sha.as_str()) || pipe["collect_seed"].as_u64() != Some(collect_seed) {
        return Err("state directory belongs to another target or seed".into());
    }
    let spec = cover::spec_from_curve(pb.p, pb.alpha, pb.l, pb.g.clone(), pb.q.clone(), 0).ok_or("public transfer failed")?;
    let ctx = cover::Ctx::new(&spec);
    let mut base_rng = StdRng::seed_from_u64(collect_seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    if pipe["l1c1_base_sha256"].as_str() != Some(base_digest(&base).as_str()) {
        return Err("factor base differs from the prepared one".into());
    }
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let quotient: Quotient = serde_json::from_value(read_json(&format!("{statedir}/quotient.json"))?).map_err(|e| format!("quotient: {e}"))?;
    let dexact = load_dexact(&format!("{statedir}/dexact.jsonl"))?;
    Ok(Setup { pb, spec, ctx, base, by_x, quotient, dexact, pipe })
}

/// audit: the l1_paired instrument (classification of every F_p-solution, arm A and
/// L1 rows, affine path, certified projection, elliptic replay of every row and the
/// sign-flip negative control) on residuals 0..N of one stream. Hooks off; its work is
/// never charged to an arm. Writes the l1_paired file formats plus soldig.TAG.txt.
fn cmd_audit(a: &[String]) -> Result<(), String> {
    if a.len() != 8 {
        return Err(USAGE.into());
    }
    let collect_seed: u64 = a[1].parse().map_err(|_| "collect seed")?;
    let stream_seed: u64 = a[2].parse().map_err(|_| "stream seed")?;
    let n: u64 = a[3].parse().map_err(|_| "N")?;
    let primary_n: u64 = a[4].parse().map_err(|_| "PRIMARY_N")?;
    let statedir = &a[5];
    let tag = a[6].as_str();
    if tag != "primary" && tag != "aa" {
        return Err("TAG must be primary or aa".into());
    }
    let owner: Option<u64> = if a[7] == "-" { None } else { Some(a[7].parse().map_err(|_| "owner scalar")?) };
    let su = setup(&a[0], collect_seed, statedir)?;
    let path = |f: &str| format!("{statedir}/{f}");
    let (p, l) = (su.pb.p, su.pb.l);
    work::claim();
    work::set_moduli(p, l);
    work::enable(false);
    let d_exact: HashMap<[u64; 6], Vec<(usize, i64)>> = su.dexact.iter().cloned().collect();
    let f3 = Fq3::new(p);
    let ec = EllE::new(&f3, &su.pb.alpha);
    let (g, q) = (su.pb.g.clone(), su.pb.q.clone());
    let elliptic = |d: u64| d > 0 && d < l && ec.mul(&g, d) == q;
    let verified = su.pipe["verified"].as_bool() == Some(true);
    let scalar = su.pipe["actual_scalar"].as_u64();
    let dstar = if verified { scalar } else { None };
    let mut pipeline_rows = Vec::new();
    let mut pipeline_idx = Vec::new();
    for d in su.pipe["pipeline_descents"].as_array().ok_or("pipeline_descents")? {
        let pr = &d["proj"];
        pipeline_rows.push(Proj { k: pr["k"].as_u64().ok_or("k")?, t: pr["t"].as_u64().ok_or("t")?, rhs: pr["rhs"].as_u64().ok_or("rhs")? });
        if let Some(i) = d["residual_index"].as_u64() {
            pipeline_idx.push(i);
        }
    }
    let pipeline_rows_match = su.pipe["pipeline_rows_match"].as_bool() == Some(true);
    let mut tg = Target { p, l, spec: &su.spec, ctx: &su.ctx, base: &su.base, by_x: &su.by_x, d_exact: &d_exact, quotient: su.quotient.clone() };
    let t0 = Instant::now();
    SOL_CHAIN.with(|c| c.borrow_mut().clear());
    let mut out = stream(&mut tg, stream_seed, n, primary_n, &path(&format!("rows.{tag}.jsonl")))?;
    replay(&mut out.rows, &pipeline_rows, dstar, l, &elliptic);
    let consistent = if tag == "primary" && stream_seed == collect_seed {
        let mut firsts: Vec<u64> = out.rows.iter().filter(|r| r.kind == "full").map(|r| r.residual_index).collect();
        firsts.dedup();
        let pd: Vec<u64> = pipeline_idx.iter().copied().filter(|&i| i < n).collect();
        firsts.len() >= pd.len() && firsts[..pd.len()] == pd[..]
    } else {
        true
    };
    let (mut s, cols) = summarize(tag, &out, n, dstar, owner, l);
    s["stream_seed"] = json!(stream_seed);
    s["pipeline_consistency"] = json!(consistent);
    s["stream_wall_ns_practicality_only"] = json!(t0.elapsed().as_nanos().to_string());
    {
        let mut w = create(&path(&format!("accepted.{tag}.jsonl")))?;
        for r in &out.rows {
            writeln!(w, "{}", serde_json::to_string(r).unwrap()).map_err(|e| e.to_string())?;
        }
    }
    {
        let mut w = create(&path(&format!("stream.{tag}.summary.json")))?;
        writeln!(w, "{}", serde_json::to_string_pretty(&s).unwrap()).map_err(|e| e.to_string())?;
    }
    {
        let mut w = create(&path(&format!("soldig.{tag}.txt")))?;
        for h in SOL_CHAIN.with(|c| c.borrow().clone()) {
            writeln!(w, "{h:016x}").map_err(|e| e.to_string())?;
        }
    }
    let mut fixed: Vec<(String, String)> = vec![
        ("tag".into(), tag.into()),
        ("public_sha256".into(), su.pb.public_sha.clone()),
        ("l".into(), l.to_string()),
        ("collect_seed".into(), collect_seed.to_string()),
        ("stream_seed".into(), stream_seed.to_string()),
        ("d_exact".into(), d_exact.len().to_string()),
        ("anchor_replay_bad".into(), su.pipe["anchor_witness_replay_bad"].to_string()),
        ("pipeline_verified".into(), u8::from(verified).to_string()),
        ("pipeline_residuals".into(), su.pipe["descent_residuals"].to_string()),
        ("pipeline_rows_match".into(), u8::from(pipeline_rows_match).to_string()),
        ("pipeline_consistency".into(), u8::from(consistent).to_string()),
    ];
    fixed.extend(cols.into_iter().map(|(k, v)| (k.to_string(), v)));
    let mut w = create(&path(&format!("target.{tag}.tsv")))?;
    writeln!(w, "{}", fixed.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join("\t")).map_err(|e| e.to_string())?;
    writeln!(w, "{}", fixed.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>().join("\t")).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// arm: one arm of the 2x2 on one stream, as production would run it, stopping after
/// the residual that brings its two-column system to rank K = 2 (or at NCAP, or after
/// exactly FIXED_N residuals for the PC1 recount). HOOKS=1: counted (bitop/v1) with
/// components; HOOKS=0: thread CPU time only. Both: the window starts before the
/// stream set-up and ends after [d]G = Q.
fn cmd_arm(a: &[String]) -> Result<(), String> {
    if a.len() != 9 {
        return Err(USAGE.into());
    }
    let collect_seed: u64 = a[1].parse().map_err(|_| "collect seed")?;
    let seed: u64 = a[2].parse().map_err(|_| "stream seed")?;
    let ncap: u64 = a[3].parse().map_err(|_| "NCAP")?;
    let fixed: Option<u64> = if a[4] == "-" { None } else { Some(a[4].parse().map_err(|_| "FIXED_N")?) };
    let statedir = &a[5];
    let arm = a[6].as_str();
    let (use_l1, use_c1) = match arm {
        "base" => (false, false),
        "l1" => (true, false),
        "c1" => (false, true),
        "l1c1" => (true, true),
        _ => return Err("ARM must be base, l1, c1 or l1c1".into()),
    };
    let hooks = match a[7].as_str() {
        "1" => true,
        "0" => false,
        _ => return Err("HOOKS must be 0 or 1".into()),
    };
    if hooks != cfg!(feature = "counted") {
        return Err("HOOKS=1 needs the counted build and HOOKS=0 the frozen build (PROTOCOL.md section 6)".into());
    }
    let out_prefix = &a[8];
    let mut su = setup(&a[0], collect_seed, statedir)?;
    let (p, l) = (su.pb.p, su.pb.l);
    let solver: cover::NagaoSolver = if use_c1 { traced_solver } else { production_solver };
    let f3 = Fq3::new(p);
    let ec = EllE::new(&f3, &su.pb.alpha);
    let jac = su.ctx.jac();
    let fl = &su.ctx.f.f;
    let ff = F2::new(p);
    if ff.w != fl.w {
        return Err("F_{p^2} presentation differs from the library's".into());
    }
    let cf = CF { p, w: ff.w };
    let h: Vec<E> = su.ctx.cov.hx.iter().map(|&c| to_e(c)).collect();
    let nb = su.base.len();
    let opts = cover::opts_for_mode(24, 600.0, StopMode::Staircase(STAIRCASE));
    let widths = work::widths(p, l);
    let dexact_list = std::mem::take(&mut su.dexact);
    work::claim();
    work::set_moduli(p, l);
    work::enable(hooks);
    let mut comp = Comp { on: hooks, ..Default::default() };
    // ======== measured window ========
    let w_before = if hooks { Some(work::snapshot()) } else { None };
    let cpu0 = thread_cpu_ns();
    if cpu0 == 0 {
        return Err("no per-thread CPU clock (/proc/thread-self/schedstat): the wall metric cannot be measured".into());
    }
    let t0 = Instant::now();
    let phase = work::enter(Phase::Descent);
    // L1 set-up: the D_exact hash table (one probe of a 6-word key per class, one word per small term)
    let mut l1st = comp.run("l1_setup", || {
        if !use_l1 {
            return None;
        }
        let mut m = HashMap::with_capacity(dexact_list.len());
        for (k, small) in &dexact_list {
            work::probe(6);
            work::words(small.len() as u64);
            m.insert(*k, small.clone());
        }
        Some(L1State { d_exact: m, anchor_ok: HashMap::new() })
    });
    // the pipeline's stream (jv_extension_deferred_pair_dlp.rs descent loop, as l1_paired.rs `stream`)
    let (al, be, step, mut av, mut bv, mut residual) = comp.run("advance", || {
        let mut srng = StdRng::seed_from_u64(seed ^ 0x51E7E);
        let al = work::uniform_mod_u64(&mut srng, 1, l);
        let be = work::uniform_mod_u64(&mut srng, 1, l);
        let step = jac.add(&jac.mul(&su.spec.gj, al as u128), &jac.mul(&su.spec.qj, be as u128));
        let av = work::uniform_mod_u64(&mut srng, 0, l);
        let bv = work::uniform_mod_u64(&mut srng, 1, l);
        let residual = jac.add(&jac.mul(&su.spec.gj, av as u128), &jac.mul(&su.spec.qj, bv as u128));
        (al, be, step, av, bv, residual)
    });
    let mut accepted: Vec<Proj> = Vec::new();
    let mut accepted_at: Vec<(u64, &'static str)> = Vec::new();
    let mut rows_out: Vec<Value> = Vec::new();
    let mut chain = CHAIN0;
    let (mut tau, mut tau1): (Option<u64>, Option<u64>) = (None, None);
    let mut k1_cost: Option<work::JvTally> = None;
    let mut k1_cpu: Option<u64> = None;
    let mut l1_rows = 0u64;
    let mut full_rows = 0u64;
    let mut learning_bound_charged = 0u64;
    let limit = fixed.unwrap_or(ncap);
    let mut processed = 0u64;
    for i in 0..limit {
        processed = i + 1;
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        LAST_SOLS.with(|s| s.borrow_mut().clear());
        let (decs, _cost) = comp.run("nagao", || cover::nagao_decompose_using(&su.ctx, &su.base, &su.by_x, &residual, &opts, &mut rng, Some(solver)))?;
        // the learning-run RREF bound (C1 only), charged as fp_mul + fp_add per multiply-add
        let pend = PENDING_BOUND.with(|c| c.replace(0));
        if pend > 0 {
            comp.run("learning_bound", || work::row_ops(pend));
            learning_bound_charged += pend;
        }
        let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
        sols.sort();
        chain_step(&mut chain, i, &sols);
        // arm A rows: every materializer decomposition, group-verified and projected
        let mut new_rows: Vec<(&'static str, &'static str, Proj)> = Vec::new();
        for dec in &decs {
            let row = comp.run("base_checks", || -> Result<Option<Proj>, String> {
                if !cover::verify_dec(&jac, &su.base, &residual, dec) {
                    return Ok(None);
                }
                let mut dense = vec![0u64; nb + 1];
                for &(idx, e) in &dec.terms {
                    dense[idx] = red(e as i128, l);
                }
                dense[nb] = l - bv;
                work::words(7);
                let c0 = su.quotient.counts.clone();
                let (pr, rhs) = su.quotient.project(&dense, av)?;
                let c1 = su.quotient.counts.clone();
                work::hook_muls(l, c1.products - c0.products);
                work::hook_adds(l, c1.additions - c0.additions);
                work::words(c1.word_events - c0.word_events);
                Ok(Some(Proj { k: pr[0], t: pr[1], rhs }))
            })?;
            if let Some(pr) = row {
                full_rows += 1;
                new_rows.push(("full", "1^6", pr));
            }
        }
        // L1 rows on the same solutions
        if let Some(st) = l1st.as_mut() {
            let hits = comp.run("l1_match", || -> Result<Vec<(&'static str, Proj)>, String> {
                let forms = cover::weil_forms(fl, &su.ctx.cov.hx, &residual.u, &v);
                let ur: Vec<E> = residual.u.iter().map(|&c| to_e(c)).collect();
                let vr: Vec<E> = v.iter().map(|&c| to_e(c)).collect();
                let mut found = Vec::new();
                for s in &sols {
                    if let Some(hit) = l1_match(&cf, &su.ctx, &jac, &su.base, &su.by_x, st, &residual, &forms, &h, &ur, &vr, s)? {
                        let mut dense: Vec<u64> = hit.coeffs.iter().map(|&x| red(x, l)).collect();
                        dense.push(l - bv);
                        let c0 = su.quotient.counts.clone();
                        let (pr, rhs) = su.quotient.project(&dense, av)?;
                        let c1 = su.quotient.counts.clone();
                        work::hook_muls(l, c1.products - c0.products);
                        work::hook_adds(l, c1.additions - c0.additions);
                        work::words(c1.word_events - c0.word_events);
                        found.push((hit.pattern, Proj { k: pr[0], t: pr[1], rhs }));
                    }
                }
                Ok(found)
            })?;
            for (pat, pr) in hits {
                l1_rows += 1;
                new_rows.push(("l1", pat, pr));
            }
        }
        // rank-raising acceptance, in stream order (arm A rows of the residual first)
        for (kind, pat, pr) in new_rows {
            let acc = comp.run("accept", || try_accept(&mut accepted, pr, l));
            if acc {
                accepted_at.push((i, kind));
            }
            rows_out.push(json!({"residual_index":i,"kind":kind,"pattern":pat,"proj":pr,"accepted":acc}));
        }
        if tau1.is_none() && !accepted.is_empty() {
            tau1 = Some(i);
            k1_cost = w_before.map(|b| work::snapshot().since(&b));
            k1_cpu = Some(thread_cpu_ns() - cpu0);
        }
        if tau.is_none() && accepted.len() == 2 {
            tau = Some(i);
            if fixed.is_none() {
                break;
            }
        }
        comp.run("advance", || {
            residual = jac.add(&residual, &step);
            work::hook_adds(l, 2);
            av = ((av as u128 + al as u128) % l as u128) as u64;
            bv = ((bv as u128 + be as u128) % l as u128) as u64;
        });
    }
    drop(phase);
    // the scalar from the two accepted rows, and [d]G = Q
    let (d, ok) = comp.run("solve_verify", || {
        let _v = work::enter(Phase::Verification);
        if accepted.len() < 2 {
            return (None, false);
        }
        let d = solve_two(&accepted[0], &accepted[1], l);
        let ok = d.is_some_and(|d| d > 0 && d < l && ec.mul(&su.pb.g, d) == su.pb.q);
        (d, ok)
    });
    let cpu_ns = thread_cpu_ns() - cpu0;
    let wall_ns = t0.elapsed().as_nanos();
    // ======== end of window ========
    let window = w_before.map(|b| work::snapshot().since(&b));
    let ttally = TTALLY.with(|t| t.borrow().clone());
    let mut det = json!({
        "schema":"l1c1-cost.arm/v1","pcc":"unverified","arm":arm,"l1":use_l1,"c1":use_c1,"hooks":hooks,
        "build": if cfg!(feature = "counted") { "counted-linear-stage" } else { "frozen-linear-stage" },
        "public_sha256":su.pb.public_sha,"collect_seed":collect_seed,"stream_seed":seed,"ncap":ncap,"fixed_n":fixed,
        "processed":processed,"tau":tau,"tau1":tau1,"censored":tau.is_none() && fixed.is_none(),
        "accepted_at":accepted_at.iter().map(|(i,k)| json!([i,k])).collect::<Vec<_>>(),
        "scalar":d,"verified":ok,"full_rows":full_rows,"l1_rows":l1_rows,
        "chain_at_end":format!("{chain:016x}"),"rows":rows_out,"trace":ttally,
        "learning_rref_bound_multiply_adds":learning_bound_charged,
        "foreign_hook_calls":work::foreign(),
    });
    if let Some(wt) = window {
        let mut parts = serde_json::Map::new();
        let mut sum = work::JvTally::default();
        for (name, t) in &comp.parts {
            parts.insert((*name).into(), json!({"bitops":price_all(t,&widths)?.to_string(),"primitives":work::prims_json(&t.total())}));
            sum.add(t);
        }
        det["counted"] = json!({
            "unit":"bitop/v1 (b = bits(p) = 10, r = bits(l))",
            "window_bitops":price_all(&wt,&widths)?.to_string(),
            "window_primitives":work::prims_json(&wt.total()),
            "unattributed_bitops":work::price(wt.phase(Phase::Unattributed),&widths).map_err(|_| "price")?.to_string(),
            "components_sum_bitops":price_all(&sum,&widths)?.to_string(),
            "components":parts,
            "learning_bound_bitops":comp.parts.get("learning_bound").map(|t| price_all(t,&widths)).transpose()?.unwrap_or(0).to_string(),
            "k1_bitops":k1_cost.map(|t| price_all(&t,&widths)).transpose()?.map(|x| x.to_string()),
        });
    }
    {
        let mut w = create(&format!("{out_prefix}.det.json"))?;
        writeln!(w, "{}", serde_json::to_string(&det).unwrap()).map_err(|e| e.to_string())?;
    }
    let timing = json!({"timing_label":"indicative non-isolated local timing unless the run declares an isolated host",
        "thread_cpu_ns":cpu_ns.to_string(),"wall_ns":wall_ns.to_string(),"k1_thread_cpu_ns":k1_cpu.map(|x| x.to_string())});
    let mut w = create(&format!("{out_prefix}.time.json"))?;
    writeln!(w, "{timing}").map_err(|e| e.to_string())?;
    Ok(())
}

/// collect: one TSV line per target and stream from the audit and arm files, with
/// every per-target gate of PROTOCOL.md section 5 evaluated here (never by hand).
fn cmd_collect(a: &[String]) -> Result<(), String> {
    if a.len() != 5 {
        return Err(USAGE.into());
    }
    let statedir = &a[0];
    let tag = a[1].as_str();
    let ncap: u64 = a[2].parse().map_err(|_| "NCAP")?;
    let owner: Option<u64> = if a[3] == "-" { None } else { Some(a[3].parse().map_err(|_| "owner")?) };
    let path = |f: &str| format!("{statedir}/{f}");
    let pipe = read_json(&path("pipeline.summary.json"))?;
    let stream_sum = read_json(&path(&format!("stream.{tag}.summary.json")))?;
    let cols = &stream_sum["columns"];
    let col = |k: &str| -> i64 { cols[k].as_str().and_then(|s| s.parse().ok()).unwrap_or(-999) };
    let audit_n = col("n_residuals") as u64;
    // audit rows in push order (arm A rows of a residual first, then L1 rows in solution order)
    let mut audit_rows: Vec<Value> = Vec::new();
    {
        let f = std::io::BufReader::new(std::fs::File::open(path(&format!("accepted.{tag}.jsonl"))).map_err(|e| e.to_string())?);
        for line in f.lines() {
            audit_rows.push(serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?);
        }
    }
    let chain: Vec<String> = std::fs::read_to_string(path(&format!("soldig.{tag}.txt"))).map_err(|e| e.to_string())?.lines().map(String::from).collect();
    let dstar = if pipe["verified"].as_bool() == Some(true) { pipe["actual_scalar"].as_u64() } else { None };
    let mut out: Vec<(String, String)> = vec![("tag".into(), tag.into()), ("public_sha256".into(), pipe["public_sha256"].as_str().unwrap_or("").into())];
    let mut gates: Vec<String> = Vec::new();
    // audit gates (as l1-paired)
    let rows_total = (col("F_rows") + col("M_rows")).max(1);
    for (k, ok) in [
        ("group_bad", col("group_bad") == 0),
        ("anchor_bad", col("anchor_bad") == 0),
        ("replay_fail_F", col("replay_fail_F") == 0),
        ("affine_inconsistent", col("affine_inconsistent") == 0),
        ("flips", col("flip_refused") == col("flip_total")),
        ("unreplayable", col("unreplayable") * 100 <= rows_total),
        ("pipeline_rows_match", pipe["pipeline_rows_match"].as_bool() == Some(true)),
        ("pipeline_consistency", stream_sum["pipeline_consistency"].as_bool() == Some(true)),
        ("owner", owner.is_none() || col("owner_check") == 1),
    ] {
        if !ok {
            gates.push(format!("audit:{k}"));
        }
    }
    let mut kill_l1_audit = col("replay_fail_M") > 0;
    let arms = ["base", "l1", "c1", "l1c1"];
    let mut taus: BTreeMap<&str, Option<u64>> = BTreeMap::new();
    let mut kill_c1 = false;
    for arm in arms {
        let det = read_json(&path(&format!("arm-{arm}.{tag}.counted.det.json")))?;
        let tm = read_json(&path(&format!("arm-{arm}.{tag}.counted.time.json")))?;
        let use_l1 = arm == "l1" || arm == "l1c1";
        let tau = det["tau"].as_u64();
        taus.insert(arm, tau);
        let last = tau.unwrap_or(ncap - 1);
        // the audit must cover the arm's residuals
        if audit_n < last + 1 {
            gates.push(format!("{arm}:audit_short"));
        }
        // solutions identical to the audit's production solver (C1 correctness for c1 arms)
        let sol_ok = chain.get(last as usize).map(String::as_str) == det["chain_at_end"].as_str();
        // a traced arm whose solutions differ is a C1 correctness KILL; the gates it
        // would trip downstream (rows, tau, scalar) are its consequences, not INVALID
        let c1_wrong = !sol_ok && (arm == "c1" || arm == "l1c1");
        if c1_wrong {
            kill_c1 = true;
        } else if !sol_ok {
            gates.push(format!("{arm}:solutions"));
        }
        // the arm's production-checked rows equal the audit rows up to its tau (multiset, in order)
        let want: Vec<(u64, String, u64, u64, u64)> = audit_rows
            .iter()
            .filter(|r| r["residual_index"].as_u64().unwrap_or(u64::MAX) <= last && (use_l1 || r["kind"] == "full"))
            .map(|r| (r["residual_index"].as_u64().unwrap(), r["kind"].as_str().unwrap_or("").to_string(), r["proj"]["k"].as_u64().unwrap_or(0), r["proj"]["t"].as_u64().unwrap_or(0), r["proj"]["rhs"].as_u64().unwrap_or(0)))
            .collect();
        let got: Vec<(u64, String, u64, u64, u64)> = det["rows"]
            .as_array()
            .ok_or("rows")?
            .iter()
            .map(|r| (r["residual_index"].as_u64().unwrap(), r["kind"].as_str().unwrap_or("").to_string(), r["proj"]["k"].as_u64().unwrap_or(0), r["proj"]["t"].as_u64().unwrap_or(0), r["proj"]["rhs"].as_u64().unwrap_or(0)))
            .collect();
        if want != got && !c1_wrong {
            gates.push(format!("{arm}:rows_vs_audit"));
        }
        // every accepted row is audit-replayed; an accepted L1 row that the audit has
        // but cannot replay is the L1 correctness KILL (as replay_fail_M), not INVALID
        for r in det["rows"].as_array().unwrap() {
            if r["accepted"] == true && !c1_wrong {
                let same = |x: &&Value| x["residual_index"] == r["residual_index"] && x["proj"] == r["proj"];
                match audit_rows.iter().find(same) {
                    Some(x) if x["replayed"] == true => {}
                    Some(x) if x["kind"] == "l1" => kill_l1_audit = true,
                    _ => gates.push(format!("{arm}:accepted_not_replayed")),
                }
            }
        }
        if tau.is_some() && !c1_wrong {
            if det["verified"] != true || (dstar.is_some() && det["scalar"].as_u64() != dstar) || (owner.is_some() && det["scalar"].as_u64() != owner) {
                gates.push(format!("{arm}:scalar"));
            }
        }
        if det["foreign_hook_calls"].as_u64() != Some(0) {
            gates.push(format!("{arm}:foreign_hooks"));
        }
        let c = &det["counted"];
        let cnum = |k: &str| -> u128 { c[k].as_str().and_then(|s| s.parse().ok()).unwrap_or(0) };
        if c.is_null() || cnum("window_bitops") != cnum("components_sum_bitops") || cnum("unattributed_bitops") != 0 {
            gates.push(format!("{arm}:components"));
        }
        let lb = cnum("learning_bound_bitops");
        out.push((format!("tau_{arm}"), tau.map(|t| t as i64).unwrap_or(-1).to_string()));
        out.push((format!("tau1_{arm}"), det["tau1"].as_i64().unwrap_or(-1).to_string()));
        out.push((format!("C_{arm}"), cnum("window_bitops").to_string()));
        out.push((format!("Cn_{arm}"), (cnum("window_bitops") - lb).to_string()));
        out.push((format!("K1_{arm}"), c["k1_bitops"].as_str().unwrap_or("-1").to_string()));
        out.push((format!("L1work_{arm}"), (c["components"]["l1_match"]["bitops"].as_str().and_then(|s| s.parse::<u128>().ok()).unwrap_or(0) + c["components"]["l1_setup"]["bitops"].as_str().and_then(|s| s.parse::<u128>().ok()).unwrap_or(0)).to_string()));
        out.push((format!("CPUc_{arm}"), tm["thread_cpu_ns"].as_str().unwrap_or("0").to_string()));
        if arm == "c1" || arm == "l1c1" {
            out.push((format!("rejected_{arm}"), det["trace"]["rejected"].to_string()));
        }
    }
    // C1 does not change acceptance; L1 can only move the stop earlier
    if !kill_c1 && (taus["c1"] != taus["base"] || taus["l1c1"] != taus["l1"]) {
        gates.push("tau_c1_mismatch".into());
    }
    if let (Some(tb), Some(tl)) = (taus["base"], taus["l1"]) {
        if tl > tb {
            gates.push("tau_l1_after_base".into());
        }
    }
    let censored = taus["base"].is_none();
    // wall arms (frozen build, hooks off), if run for this target
    let mut wall_ok = true;
    for (label, file) in [("base", "arm-base"), ("l1", "arm-l1"), ("c1", "arm-c1"), ("l1c1", "arm-l1c1"), ("base_aa", "arm-base-aa")] {
        let dp = path(&format!("{file}.{tag}.wall.det.json"));
        if !std::path::Path::new(&dp).exists() {
            out.push((format!("W_{label}"), "-".into()));
            wall_ok = false;
            continue;
        }
        let det = read_json(&dp)?;
        let tm = read_json(&path(&format!("{file}.{tag}.wall.time.json")))?;
        let carm = if label == "base_aa" { "base" } else { label };
        let cdet = read_json(&path(&format!("arm-{carm}.{tag}.counted.det.json")))?;
        if det["tau"] != cdet["tau"] || det["chain_at_end"] != cdet["chain_at_end"] || det["accepted_at"] != cdet["accepted_at"] || det["scalar"] != cdet["scalar"] {
            gates.push(format!("wall_{label}:differs_from_counted"));
        }
        out.push((format!("W_{label}"), tm["thread_cpu_ns"].as_str().unwrap_or("0").to_string()));
    }
    out.push(("wall_run".into(), u8::from(wall_ok).to_string()));
    out.push(("censored".into(), u8::from(censored).to_string()));
    out.push(("audit_n".into(), audit_n.to_string()));
    out.push(("F_audit".into(), col("F").to_string()));
    out.push(("M_audit".into(), col("M").to_string()));
    out.push(("kill_l1".into(), u8::from(kill_l1_audit).to_string()));
    out.push(("kill_c1".into(), u8::from(kill_c1).to_string()));
    out.push(("gates_failed".into(), if gates.is_empty() { "-".into() } else { gates.join(",") }));
    let mut w = create(&a[4])?;
    writeln!(w, "{}", out.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join("\t")).map_err(|e| e.to_string())?;
    writeln!(w, "{}", out.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>().join("\t")).map_err(|e| e.to_string())?;
    w.flush().map_err(|e| e.to_string())?;
    Ok(())
}
