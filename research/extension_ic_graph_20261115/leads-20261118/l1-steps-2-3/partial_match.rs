//! L1 step 3, part 2: partial classes of the descent sextic, looked up in D_exact.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1-steps-2-3.
//! Everything here is [pcc:unverified]. No descent-saving figure is computed.
//!
//! The descent stream is reconstructed exactly as l1-pattern-tally/pattern_tally.rs
//! (and jv_extension_deferred_pair_dlp.rs): same public transfer, factor-base and
//! stream seeds, per-residual RNG seeds, staircase-64 F4 with LinMode::Fast via
//! cover::nagao_decompose_using and a recording callback. For every
//! F_p-solution (lambda0, lambda1, mu) this tool, with its own F_p / F_{p^2} code:
//!  1. evaluates S's coefficients with WeilForms::eval (as the materializer) and
//!     its pattern (squarefree test + distinct-degree factorization);
//!  2. for patterns made only of degrees 1 and 2, finds the linear roots by brute
//!     force over F_p and the quadratic factors U by brute-force root search of
//!     the cofactor over F_{p^2} \ F_p;
//!  3. builds A_k = -v_{k-1} + l0 u_k + l1 u_{k-1} - mu v_k, C = x + mu, and
//!     checks the polynomial identity C^2 H - A^2 = u_R * S over F_{p^2};
//!  4. per quadratic factor U = (x - r)(x - r^p): requires C(r), C(r^p) != 0,
//!     u_R(r), u_R(r^p) != 0, V(r), V(r^p) != 0 (no ramified point); sets
//!     V = -A/C mod U by interpolation through (r, -A(r)/C(r)), (r^p, ...),
//!     checks V^2 = H mod U by polynomial remainder, and canonicalizes (U, V)
//!     to the graph's signed key [u0,u1,v0re,v0im,v1re,v1im];
//!  5. per linear root x: x in the descent factor base, mu(x) != 0, y = -A(x)/C(x)
//!     nonzero, y^2 = H(x), y = +-base.y (the materializer's conditions);
//!  6. looks up every class key in D_exact.
//! A matched row: target pattern (1^4,2), (1^2,2^2) or (2^3); every class in
//! D_exact; all checks in 4-5 pass; classes pairwise distinct (else the
//! coefficient would be doubled exactly; with S squarefree the U's differ, so
//! this cannot occur, and it is counted anyway). Every matched row, and every
//! full split, is also checked in the group with the library's Cantor
//! arithmetic: R + sum Q_i + sum D_j = 0 (principal divisor of A + C y), and
//! each matched class's anchor identity sum_i small_i B_i + D_key = 0 (from the
//! exported anchor witness), which is what fixes its log by the kernel.
//!
//! Run: partial_match PUBLIC_JSON SEED N PRIMARY_N DEXACT_JSONL RUN50_SUMMARY OUT_ROWS OUT_SUMMARY
#[path = "ffield.rs"]
mod ff;
use crypto_lib::cryptanalysis::{
    f4_fp::{F4Options, Poly},
    jv_bitops as work,
    jv_cover::{self as cover, Ctx, Div, Fld, LinMode, PtE6, StopMode, ZeroDimStats, E2, E6},
};
use ff::{invm, mulm, powm, E, F2};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use std::{cell::RefCell, collections::BTreeMap, collections::HashMap, io::BufRead, io::Write};

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

// ---- F_p polynomials (little-endian), as in pattern_tally.rs ----
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

/// Poisson exact (Garwood) 95% interval for a count k.
fn poisson_ci(k: u64) -> (f64, f64) {
    // invert the Poisson CDF numerically by bisection
    let cdf = |k: i64, mu: f64| -> f64 {
        if k < 0 {
            return 0.0;
        }
        let (mut term, mut s) = ((-mu).exp(), 0.0);
        for i in 0..=k {
            if i > 0 {
                term *= mu / i as f64;
            }
            s += term;
        }
        s
    };
    let solve = |target: f64, kk: i64| -> f64 {
        let (mut lo, mut hi) = (0.0f64, 200.0f64);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if cdf(kk, mid) > target {
                lo = mid
            } else {
                hi = mid
            }
        }
        0.5 * (lo + hi)
    };
    let lower = if k == 0 { 0.0 } else { solve(0.975, k as i64 - 1) };
    let upper = solve(0.025, k as i64);
    (lower, upper)
}
/// Exact Clopper-Pearson 95% interval for k of n.
fn clopper_pearson(k: u64, n: u64) -> (f64, f64) {
    let lnc = |n: u64, k: u64| -> f64 {
        (1..=k).map(|i| ((n - k + i) as f64).ln() - (i as f64).ln()).sum()
    };
    let cdf = |k: i64, n: u64, q: f64| -> f64 {
        if k < 0 {
            return 0.0;
        }
        (0..=k as u64).map(|i| (lnc(n, i) + i as f64 * q.ln() + (n - i) as f64 * (1.0 - q).ln()).exp()).sum()
    };
    let solve = |target: f64, kk: i64| -> f64 {
        let (mut lo, mut hi) = (1e-12f64, 1.0 - 1e-12);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if cdf(kk, n, mid) > target {
                lo = mid
            } else {
                hi = mid
            }
        }
        0.5 * (lo + hi)
    };
    let lower = if k == 0 { 0.0 } else { solve(0.975, k as i64 - 1) };
    let upper = if k == n { 1.0 } else { solve(0.025, k as i64) };
    (lower, upper)
}

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

fn main() -> Result<(), String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    pool.install(run)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 9 {
        return Err("usage: partial_match PUBLIC_JSON SEED N PRIMARY_N DEXACT_JSONL RUN50_SUMMARY OUT_ROWS OUT_SUMMARY".into());
    }
    let public: Value = serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let n: u64 = args[3].parse().map_err(|_| "N")?;
    let primary_n: u64 = args[4].parse().map_err(|_| "PRIMARY_N")?;
    let summary50: Value = serde_json::from_slice(&std::fs::read(&args[6]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    // D_exact: canonical key -> small-base integer part of its anchor
    let mut d_exact: HashMap<[u64; 6], Vec<(usize, i64)>> = HashMap::new();
    for line in std::io::BufReader::new(std::fs::File::open(&args[5]).map_err(|e| e.to_string())?).lines() {
        let v: Value = serde_json::from_str(&line.map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        let k: [u64; 6] = core::array::from_fn(|i| v["class"][i].as_u64().unwrap());
        let small = v["small"].as_array().ok_or("small")?.iter().map(|t| (t[0].as_u64().unwrap() as usize, t[1].as_i64().unwrap())).collect();
        d_exact.insert(k, small);
    }
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let ctx = Ctx::new(&spec);
    let fl = &ctx.f.f;
    let ff = F2::new(p);
    if ff.w != fl.w {
        return Err("F_{p^2} presentation differs from the library's".into());
    }
    let h: Vec<E> = ctx.cov.hx.iter().map(|&c| to_e(c)).collect();
    let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    // the descent base must be the collection's base (recorded by the Run50 rerun)
    let rec = summary50["base"].as_array().ok_or("base")?;
    if rec.len() != base.len()
        || rec.iter().zip(&base).any(|(r, b)| r["x"].as_u64() != Some(b.x) || r["y"][0].as_u64() != Some(b.y.0[0]) || r["y"][1].as_u64() != Some(b.y.0[1]))
    {
        return Err("descent factor base differs from the collection's".into());
    }
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let jac = ctx.jac();
    // group helpers
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

    let mut stream = StdRng::seed_from_u64(seed ^ 0x51E7E);
    let al = work::uniform_mod_u64(&mut stream, 1, l);
    let be = work::uniform_mod_u64(&mut stream, 1, l);
    let step = jac.add(&jac.mul(&spec.gj, al as u128), &jac.mul(&spec.qj, be as u128));
    let mut a = work::uniform_mod_u64(&mut stream, 0, l);
    let mut b = work::uniform_mod_u64(&mut stream, 1, l);
    let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let opts = cover::opts_for_mode(24, 600.0, StopMode::Staircase(64));
    let mut out = std::io::BufWriter::new(
        std::fs::OpenOptions::new().write(true).create_new(true).open(&args[7]).map_err(|e| e.to_string())?,
    );
    let mut primary = Tally::default();
    let mut all = Tally::default();
    let mut decomposing = Vec::new();
    for i in 0..n {
        let mut t = Tally { residuals: 1, ..Default::default() };
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let forms = cover::weil_forms(fl, &ctx.cov.hx, &residual.u, &v);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        LAST_SOLS.with(|s| s.borrow_mut().clear());
        let (decs, _cost) =
            cover::nagao_decompose_using(&ctx, &base, &by_x, &residual, &opts, &mut rng, Some(production_solver))?;
        if !decs.is_empty() {
            decomposing.push(i);
        }
        t.decompositions = decs.len() as u64;
        let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
        sols.sort();
        let ur: Vec<E> = residual.u.iter().map(|&c| to_e(c)).collect();
        let vr: Vec<E> = v.iter().map(|&c| to_e(c)).collect();
        let uc = |k: isize| -> E { if k >= 0 && (k as usize) < ur.len() { ur[k as usize] } else { (0, 0) } };
        let vc = |k: isize| -> E { if k >= 0 && (k as usize) < vr.len() { vr[k as usize] } else { (0, 0) } };
        let mut sol_rows = Vec::new();
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
            // own A, C and the norm identity C^2 H - A^2 = u_R S
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
            // linear roots, then quadratic factors of the cofactor
            let roots: Vec<u64> = (0..p).filter(|&x| fp_eval(&coeffs, x, p) == 0).collect();
            let mut cof = coeffs.clone();
            for &x in &roots {
                cof = div_exact(&cof, &[(p - x) % p, 1], p);
            }
            let mut quads: Vec<(u64, u64, E)> = Vec::new(); // (u0, u1, a root r)
            if cof.len() > 1 {
                let cof_e: Vec<E> = cof.iter().map(|&c| (c, 0)).collect();
                for re in 0..p {
                    for im in 1..=(p - 1) / 2 {
                        let r = (re, im);
                        if ff.peval(&cof_e, r) == (0, 0) {
                            // U = (x - r)(x - r^p) = x^2 - 2 re x + (re^2 - w im^2)
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
            // linear part, materializer conditions
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
                    lin_ok = false; // ordinate undetermined; the group would decide
                    break;
                }
                let y = ff.neg(ff.mul(ff.peval(&ak, xe), ff.inv(cx)));
                let by = to_e(base[bi].y);
                if y == (0, 0) || ff.mul(y, y) != ff.peval(&h, xe) || (y != by && y != ff.neg(by)) {
                    lin_ok = false;
                    break;
                }
                points.push((x, y));
            }
            if lab == "1^6" && lin_ok {
                t.full_split_valid += 1;
                let mut acc = jac.add(&residual, &jac.identity());
                for &(x, y) in &points {
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
                let mut entry = json!({"u":[u0,u1]});
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
                        let hit = d_exact.contains_key(&key);
                        entry = json!({"u":[u0,u1],"key":key,"coefficient":coef,"in_d_exact":hit});
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
                    entry["failure"] = json!(reason);
                    all_ok = false;
                    all_in = false;
                }
                let _ = entry;
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
                // principal divisor of A + C y: R + sum Q_i + sum D_j = 0
                let mut acc = jac.add(&residual, &jac.identity());
                for &(x, y) in &points {
                    acc = jac.add(&acc, &point_div(x, y));
                }
                for &(_, _, u0, u1, v0, v1, _) in &classes {
                    let d = class_div(u0, u1, v0, v1);
                    if !jac.is_valid(&d) {
                        return Err("library rejects a matched class as a Mumford pair".into());
                    }
                    acc = jac.add(&acc, &d);
                }
                let ok = jac.is_identity(&acc);
                group_ok = Some(ok);
                if ok {
                    t.group_check_matched_ok += 1;
                } else {
                    t.group_check_matched_bad += 1;
                }
                // anchor identity: sum_i small_i B_i + D_key = 0
                for &(key, _, u0, u1, _, _, _) in &classes {
                    let ok = *anchor_checked.entry(key).or_insert_with(|| {
                        let dkey = class_div(u0, u1, (key[2], key[3]), (key[4], key[5]));
                        let mut acc = dkey;
                        for &(bi, c) in &d_exact[&key] {
                            acc = jac.add(&acc, &signed_mul(&base[bi].d, c));
                        }
                        jac.is_identity(&acc)
                    });
                    if ok {
                        t.anchor_identity_ok += 1;
                    } else {
                        t.anchor_identity_bad += 1;
                    }
                }
            }
            sol_rows.push(json!({"pattern":lab,"linear_valid":lin_ok,"classes":classes.iter().map(|c| json!({"key":c.0,"coefficient":c.1,"in_d_exact":c.6})).collect::<Vec<_>>(),
                "class_failures":quads.len()-classes.len(),"matched_row":matched,"group_identity":group_ok}));
        }
        writeln!(out, "{}", json!({"residual_index":i,"a":a,"b":b,"solutions":sols.len(),"decompositions":decs.len(),"sols":sol_rows}))
            .map_err(|e| e.to_string())?;
        for tt in [&mut all].into_iter().chain((i < primary_n).then_some(&mut primary)) {
            merge(tt, &t);
        }
        residual = jac.add(&residual, &step);
        a = ((a as u128 + al as u128) % l as u128) as u64;
        b = ((b as u128 + be as u128) % l as u128) as u64;
    }
    out.flush().map_err(|e| e.to_string())?;
    let report = |t: &Tally| -> Value {
        let k = t.matched_rows_total;
        let fsplit = t.full_split;
        let n = t.solutions;
        let rate_m = k as f64 / n as f64;
        let rate_f = fsplit as f64 / n as f64;
        let model_f = n as f64 / 720.0;
        // conditional binomial interval for k/(k+fsplit) -> ratio of rates
        let (blo, bhi) = clopper_pearson(k, k + fsplit);
        let ratio_ci = (blo / (1.0 - blo), if bhi >= 1.0 { f64::INFINITY } else { bhi / (1.0 - bhi) });
        json!({"tally":t,"matched_rows_per_solution":rate_m,"matched_rows_poisson95":poisson_ci(k),
            "matched_rows_cp95_per_solution":clopper_pearson(k,n),
            "full_split_per_solution":rate_f,"full_split_poisson95":poisson_ci(fsplit),
            "ratio_matched_over_full_split":if fsplit>0 {rate_m/rate_f} else {f64::INFINITY},
            "ratio_conditional_binomial95":[ratio_ci.0, if ratio_ci.1.is_finite(){json!(ratio_ci.1)}else{json!("inf")}],
            "model_full_split_n_over_720":model_f,"ratio_vs_model_full_split":k as f64/model_f,
            "ratio_vs_model_poisson95":[poisson_ci(k).0/model_f, poisson_ci(k).1/model_f],
            "kill_threshold_rows_1p5x":1.5*fsplit as f64,"promote_threshold_rows_2x":2.0*fsplit as f64})
    };
    let summary = json!({"schema":"l1-steps-2-3.partial-match/v1","pcc":"unverified","seed":seed,"p":p,
        "d_exact_classes":d_exact.len(),"primary_residuals":primary_n,"extended_residuals":n,
        "decomposing_residual_indices":decomposing,"primary":report(&primary),"extended":report(&all)});
    let mut w = std::fs::OpenOptions::new().write(true).create_new(true).open(&args[8]).map_err(|e| e.to_string())?;
    writeln!(w, "{}", serde_json::to_string_pretty(&summary).unwrap()).map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    let _ = powm;
    Ok(())
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
