//! L1 paired protocol runner: one target, one collection, one or two paired descent streams.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1-paired. Frozen with
//! PROTOCOL.md / protocol.json; everything it emits is [pcc:unverified].
//!
//! Per target (one process, one rayon thread):
//!  1. Pipeline (arm A as frozen): Run50's deferred-pairs arm with Run50's options
//!     (as ../l1-steps-2-3/run50_deferred.rs), seed COLLECT_SEED, descent cap 8192.
//!     Its scalar is replayed on E. Its accepted descents are re-projected here
//!     and must equal its recorded projected rows.
//!  2. D_exact: every graph anchor at the kernel checkpoint, with the integer
//!     incidence replay of its witness and its small-base part (byte format of
//!     run50_deferred's export).
//!  3. Stream (paired): residuals R_i = a_i G + b_i Q of seed STREAM_SEED, exactly as
//!     the pipeline's descent loop and ../l1-steps-2-3/partial_match.rs. For each
//!     residual and each F_p-solution of its Weil system the sextic S is factored
//!     and classified. On the SAME solutions:
//!       arm A row  = a pipeline-materializer decomposition (1^6) that verifies in
//!                    the group (verify_dec);
//!       arm B row  = an arm A row, or an L1 row: pattern (1^4,2), (1^2,2^2) or
//!                    (2^3), every quadratic class canonical and in D_exact, linear
//!                    part valid, classes distinct, principal-divisor identity
//!                    R + sum P + sum D = 0, and each class's anchor identity
//!                    sum small_i B_i + D_key = 0.
//!  4. Affine path for every row (both kinds): dense row over the base plus the
//!     target column, target coefficient l - b, RHS a, with each L1 class replaced
//!     by its anchor's small-base part (D_key = -sum small_i B_i); certified kernel
//!     projection (Quotient::project of the pipeline's own certified quotient);
//!     two-column affine solve (CountedRelationSystem) against a partner row;
//!     elliptic replay [d]G = Q. Only replayed rows count.
//!  5. Negative control: every L1 row is also replayed with the sign of its first
//!     class flipped; that replay must fail.
//!
//! Usage:
//!   l1_paired PUBLIC_JSON COLLECT_SEED STREAM_SEED N PRIMARY_N OUTDIR AA_STREAM_SEED|- OWNER_SCALAR|-
//! Exit codes: 0 ok; 10 public transfer refused (admission); 11 collection not
//! rank-ready; 1 any other error (the run stops: see PROTOCOL.md "Stop rules").
#[path = "../l1-steps-2-3/ffield.rs"]
mod ff;
use crypto_lib::cryptanalysis::{
    f4_fp::{F4Options, Poly},
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
    time::Instant,
};

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
    let code = pool.install(|| match run(&args) {
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
    });
    std::process::exit(code);
}

fn run(args: &[String]) -> Result<(), String> {
    if args.len() != 9 {
        return Err("usage: l1_paired PUBLIC_JSON COLLECT_SEED STREAM_SEED N PRIMARY_N OUTDIR AA_STREAM_SEED|- OWNER_SCALAR|-".into());
    }
    let public_bytes = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let public_sha = hex(&Sha256::digest(&public_bytes));
    let public: Value = serde_json::from_slice(&public_bytes).map_err(|e| e.to_string())?;
    let collect_seed: u64 = args[2].parse().map_err(|_| "collect seed")?;
    let stream_seed: u64 = args[3].parse().map_err(|_| "stream seed")?;
    let n: u64 = args[4].parse().map_err(|_| "N")?;
    let primary_n: u64 = args[5].parse().map_err(|_| "PRIMARY_N")?;
    let outdir = &args[6];
    let aa_seed: Option<u64> = if args[7] == "-" { None } else { Some(args[7].parse().map_err(|_| "aa seed")?) };
    let owner: Option<u64> = if args[8] == "-" { None } else { Some(args[8].parse().map_err(|_| "owner scalar")?) };
    std::fs::create_dir_all(outdir).map_err(|e| e.to_string())?;
    let path = |f: &str| format!("{outdir}/{f}");
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    if p != 1009 || l >= 1u64 << 62 {
        return Err("domain: this protocol admits p = 1009 and l < 2^62 only".into());
    }
    let (alpha, g, q) = (e6(&public["alpha"])?, pt(&public["g"])?, pt(&public["q"])?);
    let started = Instant::now();
    // ---- 1. pipeline: run50_deferred.rs's worker body, verbatim in its options ----
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let before = work::snapshot();
    let model = work::enter(Phase::Model);
    let spec = cover::spec_from_curve(p, alpha, l, g, q, 0).ok_or("public transfer failed")?;
    let mut options = DeferredPairDlpOptions::default();
    options.collection.collection.b_count = Some(1018081);
    options.collection.collection.max_descent_residuals = 8192;
    options.collection.collection.max_accepted_descents = 32;
    options.collection.collection.max_split_records = 524288;
    options.max_deferred_edges = 262144;
    let (report, scalar) = run_extension_deferred_pair_dlp_on(&spec, collect_seed, options)?;
    drop(model);
    let f3 = Fq3::new(p);
    let ec = EllE::new(&f3, &alpha);
    let verified = {
        let _v = work::enter(Phase::Verification);
        scalar.is_some_and(|d| d > 0 && d < l && ec.mul(&g, d) == q)
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
        writeln!(w, "{}", json!({"schema":"l1-paired.pipeline/v1","public_sha256":public_sha,"collection_rank_ready":false,"counts":report["collection"]["counts"],"stop":report["collection"]["stop"]})).map_err(|e| e.to_string())?;
        return Err("collection not rank-ready".into());
    }
    // ---- 2. D_exact export with integer-incidence replay (run50_deferred.rs format) ----
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
    // only anchors whose witness replays exactly enter D_exact
    let mut d_exact: HashMap<[u64; 6], Vec<(usize, i64)>> = HashMap::new();
    {
        let f = std::io::BufReader::new(std::fs::File::open(path("dexact.jsonl")).map_err(|e| e.to_string())?);
        for line in f.lines() {
            let v: Value = serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            let k: [u64; 6] = core::array::from_fn(|i| v["class"][i].as_u64().unwrap());
            let small = v["small"].as_array().ok_or("small")?.iter().map(|t| (t[0].as_u64().unwrap() as usize, t[1].as_i64().unwrap())).collect();
            d_exact.insert(k, small);
        }
    }
    if replay_bad != 0 {
        return Err(format!("{replay_bad} anchor witnesses fail the integer incidence replay"));
    }
    // ---- certified quotient and the pipeline's own rows ----
    let mut quotient: Quotient = serde_json::from_value(report["initial_basis"].clone()).map_err(|e| format!("initial_basis: {e}"))?;
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
    let mut pipeline_rows = Vec::new();
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
        pipeline_rows.push(proj);
    }
    pipeline_rows_match &= recorded.len() == descents.len();
    let dstar = if verified { scalar } else { None };
    let summary = json!({
        "schema":"l1-paired.pipeline/v1","pcc":"unverified","public_sha256":public_sha,"collect_seed":collect_seed,
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
    });
    {
        let mut w = create(&path("pipeline.summary.json"))?;
        writeln!(w, "{}", serde_json::to_string_pretty(&summary).unwrap()).map_err(|e| e.to_string())?;
    }
    let pipeline_idx: Vec<u64> = descents.iter().filter_map(|d| d["residual_index"].as_u64()).collect();
    drop(report);
    let elliptic = |d: u64| d > 0 && d < l && ec.mul(&g, d) == q;
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let mut tg = Target { p, l, spec: &spec, ctx: &ctx, base: &base, by_x: &by_x, d_exact: &d_exact, quotient };
    // ---- 3-5. paired streams ----
    let mut tsv_lines = Vec::new();
    let mut header = String::new();
    let streams: Vec<(&str, u64)> = std::iter::once(("primary", stream_seed)).chain(aa_seed.map(|s| ("aa", s))).collect();
    for (tag, sseed) in streams {
        let t0 = Instant::now();
        let mut out = stream(&mut tg, sseed, n, primary_n, &path(&format!("rows.{tag}.jsonl")))?;
        replay(&mut out.rows, &pipeline_rows, dstar, l, &elliptic);
        // pipeline consistency: on the primary stream (STREAM_SEED = COLLECT_SEED) the
        // pipeline's accepted descents are the first full rows of the stream
        let consistent = if tag == "primary" && sseed == collect_seed {
            let mut firsts: Vec<u64> = out.rows.iter().filter(|r| r.kind == "full").map(|r| r.residual_index).collect();
            firsts.dedup();
            let pd: Vec<u64> = pipeline_idx.iter().copied().filter(|&i| i < n).collect();
            firsts.len() >= pd.len() && firsts[..pd.len()] == pd[..]
        } else {
            true
        };
        let (mut s, cols) = summarize(tag, &out, n, dstar, owner, l);
        s["stream_seed"] = json!(sseed);
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
        let mut fixed: Vec<(String, String)> = vec![
            ("tag".into(), tag.into()),
            ("public_sha256".into(), public_sha.clone()),
            ("l".into(), l.to_string()),
            ("collect_seed".into(), collect_seed.to_string()),
            ("stream_seed".into(), sseed.to_string()),
            ("d_exact".into(), d_exact.len().to_string()),
            ("anchor_replay_bad".into(), replay_bad.to_string()),
            ("pipeline_verified".into(), u8::from(verified).to_string()),
            ("pipeline_residuals".into(), summary["descent_residuals"].to_string()),
            ("pipeline_rows_match".into(), u8::from(pipeline_rows_match).to_string()),
            ("pipeline_consistency".into(), u8::from(consistent).to_string()),
        ];
        fixed.extend(cols.into_iter().map(|(k, v)| (k.to_string(), v)));
        header = fixed.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>().join("\t");
        tsv_lines.push(fixed.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>().join("\t"));
    }
    let mut w = create(&path("target.tsv"))?;
    writeln!(w, "{header}").map_err(|e| e.to_string())?;
    for line in tsv_lines {
        writeln!(w, "{line}").map_err(|e| e.to_string())?;
    }
    w.flush().map_err(|e| e.to_string())?;
    println!("{}", json!({"public_sha256":public_sha,"verified":verified,"d_exact":d_exact.len(),"wall_ns_practicality_only":started.elapsed().as_nanos().to_string()}));
    Ok(())
}
