//! L1 step 1: factorization-pattern tally of the descent sextic S over F_p.
//!
//! Lane: research/extension_ic_graph_20261115/leads-20261118/l1-pattern-tally.
//! Lead: scout-graph-ic/LEADS.md, L1, "Cheapest local falsification" step 1.
//! Counted algebra only; no timing claim. [pcc:unverified]
//!
//! For each residual of the reconstructed stream (same derivations as
//! descent-finder/traced_descent_check.rs and jv_extension_deferred_pair_dlp.rs),
//! the production descent (staircase-64 F4 without trace, linear stage
//! LinMode::Fast) is run through `cover::nagao_decompose_using` with a
//! recording `NagaoSolver` callback, so the decomposition outcome is the
//! pipeline's own (control: Run50 stream decomposes at residuals 263 and 3414).
//! For every F_p-solution (lambda0, lambda1, mu) the monic sextic
//! S = (C^2 H - A^2)/u_R is evaluated exactly as nagao_decompose_using does
//! (`WeilForms::eval`, k = 0..5, plus the monic 1) and its factorization
//! pattern over F_p is computed twice, independently:
//!   (a) squarefree test gcd(S, S') then distinct-degree factorization;
//!   (b) brute-force root count over all of F_p plus the Berlekamp count
//!       r = 6 - rank(Q - I) of irreducible factors.
//! For squarefree S, (L, r) determines the patterns L1 needs uniquely:
//! (1^6) L=6; (1^4,2) L=4,r=5; (1^2,2^2) L=2,r=4; (2^3) L=0,r=3.
//! Every solution's (a) pattern is checked for consistency with (b).
//!
//! Also tallied (secondary, not part of LEADS.md's prediction): how many of
//! the linear roots lie in the descent factor base (`by_x`), since L1's rows
//! need the linear factors supported too.
//!
//! Run: pattern_tally PUBLIC_JSON SEED N OUT_JSONL
use crypto_lib::cryptanalysis::{
    f4_fp::{F4Options, Poly},
    jv_bitops as work,
    jv_cover::{self as cover, Ctx, LinMode, StopMode, ZeroDimStats, E2, E6, PtE6},
};
use rand::{rngs::StdRng, SeedableRng};
use std::{cell::RefCell, collections::BTreeMap, collections::HashMap, io::Write};

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

// ---- dense polynomials over F_p, little-endian coefficient vectors ----

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0);
    powm(a, p - 2, p)
}
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
fn powmod_x(e: u64, m: &[u64], p: u64) -> Vec<u64> {
    // x^e mod m
    let mut r = rem(&[1], m, p);
    let mut b = rem(&[0, 1], m, p);
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(&r, &b, m, p);
        }
        b = mulmod(&b, &b, m, p);
        e >>= 1;
    }
    r
}
fn compose_pow(h: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    // h^p mod m
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
fn sub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut c = vec![0u64; n];
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        c[i] = (x + p - y) % p;
    }
    trim(c)
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

/// Method (a): None if S is not squarefree; else the multiset of factor degrees.
fn ddf_pattern(f: &[u64], p: u64) -> Option<Vec<usize>> {
    let g = gcd(f, &derivative(f, p), p);
    if deg(&g) > 0 {
        return None;
    }
    let mut f = f.to_vec();
    let mut out = Vec::new();
    let mut h = vec![0u64, 1];
    let mut d = 1usize;
    while deg(&f) >= 2 * d as isize {
        h = compose_pow(&h, &f, p);
        let g = gcd(&sub(&h, &[0, 1], p), &f, p);
        let dg = deg(&g) as usize;
        if dg > 0 {
            assert!(dg % d == 0);
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

/// Method (b): (number of distinct F_p roots by brute force, Berlekamp factor count).
fn berlekamp(f: &[u64], p: u64) -> (usize, usize) {
    let n = deg(f) as usize;
    let mut roots = 0;
    for x in 0..p {
        let mut acc = 0u64;
        for &c in f.iter().rev() {
            acc = (mulm(acc, x, p) + c) % p;
        }
        roots += usize::from(acc == 0);
    }
    // Q rows: x^{p i} mod f, i = 0..n-1; matrix Q - I, rank by Gaussian elimination.
    let xp = powmod_x(p, f, p);
    let mut row = rem(&[1], f, p);
    let mut m = vec![vec![0u64; n]; n];
    for i in 0..n {
        for j in 0..n {
            m[i][j] = row.get(j).copied().unwrap_or(0);
        }
        m[i][i] = (m[i][i] + p - 1) % p;
        row = mulmod(&row, &xp, f, p);
    }
    let mut rank = 0;
    for c in 0..n {
        if let Some(r) = (rank..n).find(|&r| m[r][c] != 0) {
            m.swap(rank, r);
            let iv = invm(m[rank][c], p);
            for r2 in 0..n {
                if r2 != rank && m[r2][c] != 0 {
                    let t = mulm(m[r2][c], iv, p);
                    for c2 in 0..n {
                        m[r2][c2] = (m[r2][c2] + p - mulm(t, m[rank][c2], p)) % p;
                    }
                }
            }
            rank += 1;
        }
    }
    (roots, n - rank)
}

fn label(pat: &[usize]) -> String {
    let mut cnt: BTreeMap<usize, usize> = BTreeMap::new();
    for &d in pat {
        *cnt.entry(d).or_default() += 1;
    }
    cnt.iter().map(|(d, a)| if *a == 1 { format!("{d}") } else { format!("{d}^{a}") }).collect::<Vec<_>>().join(",")
}

/// Random-polynomial cycle-type density 1/prod(k^{a_k} a_k!) (the LEADS.md heuristic).
fn density(pat: &[usize]) -> f64 {
    let mut cnt: BTreeMap<usize, u32> = BTreeMap::new();
    for &d in pat {
        *cnt.entry(d).or_default() += 1;
    }
    let mut z = 1f64;
    for (k, a) in cnt {
        z *= (k as f64).powi(a as i32) * (1..=a).map(|i| i as f64).product::<f64>();
    }
    1.0 / z
}

fn partitions(n: usize, max: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for k in (1..=max.min(n)).rev() {
        for mut rest in partitions(n - k, k) {
            rest.push(k);
            rest.sort();
            out.push(rest);
        }
    }
    out
}

fn ln_choose(n: u64, k: u64) -> f64 {
    let lg = |x: u64| (1..=x).map(|i| (i as f64).ln()).sum::<f64>();
    lg(n) - lg(k) - lg(n - k)
}
/// P(X <= k) for X ~ Bin(n, q).
fn bin_cdf(k: u64, n: u64, q: f64) -> f64 {
    if q <= 0.0 {
        return 1.0;
    }
    if q >= 1.0 {
        return if k >= n { 1.0 } else { 0.0 };
    }
    (0..=k).map(|i| (ln_choose(n, i) + i as f64 * q.ln() + (n - i) as f64 * (1.0 - q).ln()).exp()).sum::<f64>().min(1.0)
}
/// Exact Clopper-Pearson 95% interval for k successes in n.
fn clopper_pearson(k: u64, n: u64) -> (f64, f64) {
    let bisect = |f: &dyn Fn(f64) -> bool| {
        let (mut lo, mut hi) = (0.0f64, 1.0f64);
        for _ in 0..100 {
            let mid = 0.5 * (lo + hi);
            if f(mid) {
                hi = mid
            } else {
                lo = mid
            }
        }
        0.5 * (lo + hi)
    };
    let lower = if k == 0 { 0.0 } else { bisect(&|q| 1.0 - bin_cdf(k - 1, n, q) >= 0.025) };
    let upper = if k == n { 1.0 } else { bisect(&|q| bin_cdf(k, n, q) <= 0.025) };
    (lower, upper)
}

fn add_mod(a: u64, b: u64, l: u64) -> u64 {
    ((a as u128 + b as u128) % l as u128) as u64
}

fn main() -> Result<(), String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(1).build().map_err(|e| e.to_string())?;
    pool.install(run)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: pattern_tally PUBLIC_JSON SEED N OUT_JSONL".into());
    }
    let public: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&args[1]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let seed: u64 = args[2].parse().map_err(|_| "seed")?;
    let n: u64 = args[3].parse().map_err(|_| "N")?;
    let e2 = |v: &serde_json::Value| -> Result<E2, String> {
        let a = v.as_array().ok_or("e2")?;
        Ok(E2([a[0].as_u64().ok_or("e2")?, a[1].as_u64().ok_or("e2")?]))
    };
    let e6 = |v: &serde_json::Value| -> Result<E6, String> {
        let a = v.as_array().ok_or("e6")?;
        Ok(E6([e2(&a[0])?, e2(&a[1])?, e2(&a[2])?]))
    };
    let pt = |v: &serde_json::Value| -> Result<PtE6, String> {
        Ok(PtE6 { x: e6(&v["x"])?, y: e6(&v["y"])?, inf: v["inf"].as_bool().ok_or("inf")? })
    };
    let p = public["p"].as_u64().ok_or("p")?;
    let l = public["l"].as_u64().ok_or("l")?;
    let spec = cover::spec_from_curve(p, e6(&public["alpha"])?, l, pt(&public["g"])?, pt(&public["q"])?, 0)
        .ok_or("public transfer failed")?;
    // As traced_descent_check.rs: claim the jv_bitops tally on this thread.
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let ctx = Ctx::new(&spec);
    let f = &ctx.f.f;
    // Same derivations as jv_extension_deferred_pair_dlp.rs / traced_descent_check.rs.
    let mut base_rng = StdRng::seed_from_u64(seed ^ 0xBACE_0001);
    let base = cover::factor_base(&ctx, &mut base_rng);
    let by_x: HashMap<u64, usize> = base.iter().enumerate().map(|(i, b)| (b.x, i)).collect();
    let jac = ctx.jac();
    let mut stream = StdRng::seed_from_u64(seed ^ 0x51E7E);
    let al = cover_uniform(&mut stream, 1, l);
    let be = cover_uniform(&mut stream, 1, l);
    let step = jac.add(&jac.mul(&spec.gj, al as u128), &jac.mul(&spec.qj, be as u128));
    let mut a = cover_uniform(&mut stream, 0, l);
    let mut b = cover_uniform(&mut stream, 1, l);
    let mut residual = jac.add(&jac.mul(&spec.gj, a as u128), &jac.mul(&spec.qj, b as u128));
    let opts = cover::opts_for_mode(24, 600.0, StopMode::Staircase(64));
    let mut out = std::io::BufWriter::new(
        std::fs::OpenOptions::new().write(true).create_new(true).open(&args[4]).map_err(|e| e.to_string())?,
    );
    let mut tally: BTreeMap<String, u64> = BTreeMap::new();
    let mut supported: BTreeMap<String, (u64, u64)> = BTreeMap::new(); // (all linear roots in base, total)
    let (mut solutions, mut non_fp, mut nonsquarefree, mut method_disagree) = (0u64, 0u64, 0u64, 0u64);
    let mut decomposing = Vec::new();
    let mut residuals_with_target = 0u64;
    for i in 0..n {
        let mut v = residual.v.clone();
        v.resize(3, E2::ZERO);
        let forms = cover::weil_forms(f, &ctx.cov.hx, &residual.u, &v);
        let mut rng = StdRng::seed_from_u64(seed ^ i.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        LAST_SOLS.with(|s| s.borrow_mut().clear());
        let (decs, _cost) =
            cover::nagao_decompose_using(&ctx, &base, &by_x, &residual, &opts, &mut rng, Some(production_solver))?;
        if !decs.is_empty() {
            decomposing.push(i);
        }
        let mut sols = LAST_SOLS.with(|s| s.borrow().clone());
        sols.sort();
        let mut pats = Vec::new();
        let mut has_target = false;
        for s in &sols {
            solutions += 1;
            let lam = [E2([s[0], s[1]]), E2([s[2], s[3]]), E2([s[4], s[5]])];
            let mut coeffs = Vec::with_capacity(7);
            let mut fp = true;
            for k in 0..6 {
                let c = forms.eval(f, k, &lam);
                fp &= c.0[1] == 0;
                coeffs.push(c.0[0]);
            }
            coeffs.push(1);
            if !fp {
                non_fp += 1;
                pats.push("non-Fp".to_string());
                continue;
            }
            let (roots, r) = berlekamp(&coeffs, p);
            let lab = match ddf_pattern(&coeffs, p) {
                None => {
                    nonsquarefree += 1;
                    "nonsquarefree".to_string()
                }
                Some(pat) => {
                    let lin = pat.iter().filter(|&&d| d == 1).count();
                    if lin != roots || pat.len() != r {
                        method_disagree += 1;
                    }
                    label(&pat)
                }
            };
            // secondary: are all linear roots in the descent factor base?
            let lin_roots: Vec<u64> = (0..p)
                .filter(|&x| coeffs.iter().rev().fold(0u64, |acc, &c| (mulm(acc, x, p) + c) % p) == 0)
                .collect();
            let all_in = lin_roots.iter().all(|x| by_x.contains_key(x));
            let e = supported.entry(lab.clone()).or_default();
            e.0 += u64::from(all_in);
            e.1 += 1;
            has_target |= matches!(lab.as_str(), "1^4,2" | "1^2,2^2" | "2^3" | "1^6");
            *tally.entry(lab.clone()).or_default() += 1;
            pats.push(lab);
        }
        residuals_with_target += u64::from(has_target);
        writeln!(out, "{}", serde_json::json!({"residual_index": i, "a": a, "b": b, "solutions": sols.len(),
            "patterns": pats, "decompositions": decs.len()})).map_err(|e| e.to_string())?;
        residual = jac.add(&residual, &step);
        a = add_mod(a, al, l);
        b = add_mod(b, be, l);
    }
    // Comparison against the random-sextic densities for every partition of 6.
    let sf = solutions - non_fp - nonsquarefree;
    let rows: Vec<serde_json::Value> = partitions(6, 6)
        .iter()
        .map(|pat| {
            let lab = label(pat);
            let k = tally.get(&lab).copied().unwrap_or(0);
            let d = density(pat);
            let (lo, hi) = clopper_pearson(k, solutions);
            let (s_in, s_tot) = supported.get(&lab).copied().unwrap_or((0, 0));
            serde_json::json!({"pattern": lab, "observed": k, "density": d, "predicted": d * solutions as f64,
                "obs_rate": k as f64 / solutions as f64, "cp95": [lo, hi],
                "ratio_obs_over_pred": k as f64 / (d * solutions as f64),
                "cp95_ratio": [lo / d, hi / d],
                "linear_roots_all_in_base": s_in, "of": s_tot})
        })
        .collect();
    let summary = serde_json::json!({
        "schema": "l1-pattern-tally/v1", "pcc": "unverified", "seed": seed, "residuals": n, "p": p,
        "solutions": solutions, "non_fp_sextic": non_fp, "nonsquarefree": nonsquarefree,
        "squarefree": sf, "method_disagreements": method_disagree,
        "decomposing_residual_indices": decomposing,
        "residuals_with_a_target_pattern": residuals_with_target,
        "factor_base_size": base.len(), "patterns": rows,
        "note": "density = random-sextic cycle-type density used by LEADS.md L1; predicted = density * solutions; cp95 = exact Clopper-Pearson 95% on obs_rate"
    });
    writeln!(out, "{summary}").map_err(|e| e.to_string())?;
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
    Ok(())
}

fn cover_uniform(rng: &mut StdRng, lo: u64, hi: u64) -> u64 {
    work::uniform_mod_u64(rng, lo, hi)
}
