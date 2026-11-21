//! `smith_even_census`: Smith's Propositions 2 and 4 on random genus-3 curves
//! `y² = f(x)` with `deg f = 8` (the even-degree, "real" model: no rational
//! Weierstrass point at infinity) over small `F_p`, with the exact group
//! order of every curve from its zeta function, so the pass rate can be
//! conditioned on near-primality of `#J` and on the Frobenius cycle type of
//! the eight Weierstrass points.
//!
//! ```text
//! smith_even_census verify --primes 31,47 --n 30 --out FILE.json
//! smith_even_census run --primes 31,47,67,97 --admitted 150 --prime-order 40
//!     [--max-h 8] [--max-seeds 30000] [--threads 3] --out FILE.jsonl
//! ```
//!
//! Group order: `N_k = #C(F_{p^k})` for `k = 1, 2, 3` by enumeration of
//! `x ∈ F_{p^k}` (quadratic character of `f(x)` from a table of squares of
//! `F_{p^k}`), plus the points at infinity (two when `lc(f)` is a square in
//! `F_{p^k}`, none otherwise; for `deg f = 7` one).  With `s_k = N_k − p^k − 1`
//! the coefficients of `L(T) = 1 + c_1 T + c_2 T² + c_3 T³ + p c_2 T⁴ + p² c_1 T⁵
//! + p³ T⁶` follow from Newton's identities `k c_k = Σ_{i=1..k} s_i c_{k−i}`
//! (as in `genus_ladder::JCurve::jacobian_order`), and `#J = L(1)`; the
//! quadratic twist has `#J' = L(−1)`.
//!
//! `verify` checks this routine against `JCurve::jacobian_order` (exact,
//! Cantor arithmetic) on degree-7 curves, and on degree-8 curves with a
//! rational root `r` through the isomorphism `x = r + 1/u` to an odd-degree
//! model; and the twist identity on both.  `run` writes one JSON line per
//! curve.  Protocol: `research/smith_even_census_20261115/PROTOCOL.md`.  All
//! figures `[pcc:unverified]`.

use std::io::Write;
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crypto_lib::cryptanalysis::chained_transfer::ic_control::split_order;
use crypto_lib::cryptanalysis::genus_ladder::{random_f, JCurve};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use crypto_lib::cryptanalysis::smith_kernel_coverage::has_stable_matching;
use crypto_lib::cryptanalysis::smith_prop24 as smith;
use serde_json::json;

// ---------------------------------------------------------------------------
// F_{p^k} = F_p[w]/(m), k <= 3, p small (p^3 < 2^32).

#[derive(Clone)]
struct Ext {
    p: u64,
    k: usize,
    /// `w^k = −Σ m[i] w^i`.
    m: [u64; 3],
    /// Quadratic character of every element, indexed by `a0 + a1 p + a2 p²`.
    chi: Vec<i8>,
}

impl Ext {
    fn new(p: u64, k: usize) -> Ext {
        assert!((1..=3).contains(&k));
        let mut m = [0u64; 3];
        match k {
            1 => {}
            2 => {
                // w² = ν, ν the least non-residue: modulus w² − ν.
                m[0] = (p - smith::least_nonsquare(p)) % p;
            }
            _ => {
                // An irreducible cubic w³ + a w + b: irreducible iff no root.
                'search: for a in 0..p {
                    for b in 1..p {
                        if (0..p).all(|x| !(x * x % p * x + a * x + b).is_multiple_of(p)) {
                            m = [b, a, 0];
                            break 'search;
                        }
                    }
                }
                assert!(m[0] != 0, "no irreducible cubic found");
            }
        }
        let mut e = Ext {
            p,
            k,
            m,
            chi: Vec::new(),
        };
        let size = p.pow(k as u32) as usize;
        let mut chi = vec![-1i8; size];
        chi[0] = 0;
        for i in 1..size {
            let y = e.dec(i);
            let y2 = e.mul(&y, &y);
            chi[e.enc(&y2)] = 1;
        }
        e.chi = chi;
        e
    }

    fn enc(&self, a: &[u64; 3]) -> usize {
        (a[0] + self.p * (a[1] + self.p * a[2])) as usize
    }

    fn dec(&self, mut i: usize) -> [u64; 3] {
        let p = self.p as usize;
        let mut a = [0u64; 3];
        for c in a.iter_mut().take(self.k) {
            *c = (i % p) as u64;
            i /= p;
        }
        a
    }

    fn mul(&self, a: &[u64; 3], b: &[u64; 3]) -> [u64; 3] {
        let (p, k) = (self.p, self.k);
        let mut t = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        for d in (k..2 * k - 1).rev() {
            let c = t[d];
            if c == 0 {
                continue;
            }
            t[d] = 0;
            for i in 0..k {
                // w^d = w^(d−k) · (−Σ m_i w^i)
                t[d - k + i] = (t[d - k + i] + (p - self.m[i]) % p * c) % p;
            }
        }
        [t[0], t[1], t[2]]
    }

    /// `#{(x, y) : y² = f(x)}` over `F_{p^k}` (affine points only).
    fn affine_count(&self, f: &[u64]) -> u64 {
        let p = self.p;
        let size = p.pow(self.k as u32) as usize;
        let mut n = 0i64;
        for i in 0..size {
            let x = self.dec(i);
            let mut acc = [0u64; 3];
            for &c in f.iter().rev() {
                acc = self.mul(&acc, &x);
                acc[0] = (acc[0] + c) % p;
            }
            n += 1 + self.chi[self.enc(&acc)] as i64;
        }
        n as u64
    }
}

fn trim(f: &[u64]) -> Vec<u64> {
    let mut f = f.to_vec();
    while f.len() > 1 && *f.last().unwrap() == 0 {
        f.pop();
    }
    f
}

/// Zeta data of `y² = f(x)`, `deg f ∈ {7, 8}`, genus 3.
#[derive(Clone, Debug)]
struct Zeta {
    n: [u64; 3],
    c: [i128; 4],
    order: i128,
    twist_order: i128,
}

/// `N_1..N_3`, `L(T)`, `#J = L(1)` and `#J' = L(−1)`; `Err` if a Newton
/// division is inexact or a Weil bound fails (both impossible for a correct
/// count).
fn zeta(exts: &[Ext; 3], f: &[u64]) -> Result<Zeta, String> {
    let f = trim(f);
    let d = f.len() - 1;
    assert!(d == 7 || d == 8);
    let p = exts[0].p;
    let lc_chi = smith::chi(f[d], p);
    let mut n = [0u64; 3];
    for k in 1..=3usize {
        let inf = if d == 7 {
            1
        } else if k % 2 == 0 {
            2
        } else {
            (1 + lc_chi as i64) as u64
        };
        n[k - 1] = exts[k - 1].affine_count(&f) + inf;
    }
    let pi = p as i128;
    let s: Vec<i128> = (1..=3u32)
        .map(|k| n[k as usize - 1] as i128 - pi.pow(k) - 1)
        .collect();
    for (k, sk) in s.iter().enumerate() {
        // |N_k − p^k − 1| <= 2g p^(k/2)
        let b = 6.0 * (p as f64).powf((k + 1) as f64 / 2.0);
        if (sk.abs() as f64) > b + 1e-9 {
            return Err(format!("Weil bound fails for N_{}", k + 1));
        }
    }
    let mut c = [1i128, 0, 0, 0];
    for k in 1..=3usize {
        let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
        if num % k as i128 != 0 {
            return Err(format!("Newton integrality fails at k = {k}"));
        }
        c[k] = num / k as i128;
    }
    let l = |t: i128| -> i128 {
        let coef = [1, c[1], c[2], c[3], pi * c[2], pi * pi * c[1], pi * pi * pi];
        coef.iter()
            .enumerate()
            .map(|(i, &a)| a * t.pow(i as u32))
            .sum()
    };
    let (order, twist_order) = (l(1), l(-1));
    let sp = (p as f64).sqrt();
    let (lo, hi) = ((sp - 1.0).powi(6), (sp + 1.0).powi(6));
    for v in [order, twist_order] {
        if (v as f64) < lo - 1e-6 || (v as f64) > hi + 1e-6 {
            return Err("Weil interval for #J fails".into());
        }
    }
    Ok(Zeta {
        n,
        c,
        order,
        twist_order,
    })
}

fn exts_for(p: u64) -> [Ext; 3] {
    [Ext::new(p, 1), Ext::new(p, 2), Ext::new(p, 3)]
}

/// Random degree-8 `f` (lc uniform nonzero, other coefficients uniform).
fn random_octic(p: u64, seed: u64) -> Vec<u64> {
    let mut rng = smith::SplitMix64(seed.wrapping_mul(0xA24B_AED4_963E_E407) ^ p.rotate_left(29));
    rng.next_u64();
    let mut f: Vec<u64> = (0..8).map(|_| rng.below(p)).collect();
    f.push(1 + rng.below(p - 1));
    f
}

fn pow_mod(b: u64, e: u64, p: u64) -> u64 {
    smith::pow_mod(b % p, e as u128, p)
}

/// For `f` of degree 8 with `f(r) = 0`: the monic degree-7 `h` with
/// `y² = f(x)` ≅ `Y² = h(v)` over `F_p` (`x = r + 1/u`, `y = y'/u⁴`, then
/// `v = c u`, `Y = c³ y'` with `c` the leading coefficient of
/// `g(u) = u⁸ f(r + 1/u)`).
fn odd_model(f: &[u64], r: u64, p: u64) -> Vec<u64> {
    // g(u) = Σ_i f_i (r u + 1)^i u^(8 − i)
    let mut g = [0u64; 9];
    let mut pw = vec![1u64]; // (r u + 1)^i
    for (i, &fi) in f.iter().enumerate() {
        for (j, &cj) in pw.iter().enumerate() {
            let e = j + 8 - i;
            g[e] = (g[e] + fi * cj) % p;
        }
        // pw *= (r u + 1)
        let mut nx = vec![0u64; pw.len() + 1];
        for (j, &cj) in pw.iter().enumerate() {
            nx[j] = (nx[j] + cj) % p;
            nx[j + 1] = (nx[j + 1] + cj * r) % p;
        }
        pw = nx;
    }
    assert_eq!(g[8], 0, "f(r) must vanish");
    let c = g[7];
    assert!(c != 0);
    (0..8)
        .map(|i| {
            if i == 7 {
                1
            } else {
                g[i] * pow_mod(c, (6 - i) as u64, p) % p
            }
        })
        .collect()
}

fn verify(primes: &[u64], n_each: usize, out: &str) -> ExitCode {
    let mut report = Vec::new();
    let mut all_ok = true;
    for &p in primes {
        let exts = exts_for(p);
        // (i) degree 7 against Cantor-based exact orders.
        let (mut done, mut agree, mut n12, mut twist_ok) = (0usize, 0usize, 0usize, 0usize);
        let mut seed = 0u64;
        let mut rows = Vec::new();
        while done < n_each {
            seed += 1;
            let f = random_f(p, 3, seed);
            let Ok(curve) = JCurve::new(p, 3, &f) else {
                continue;
            };
            let Ok(exact) = curve.jacobian_order(seed) else {
                continue;
            };
            done += 1;
            let z = zeta(&exts, &f);
            let ok = matches!(&z, Ok(z) if z.order == exact as i128);
            let ok12 = matches!(&z, Ok(z) if z.n[0] == curve.count_points(1) && z.n[1] == curve.count_points(2));
            // twist: y² = ν f with ν a non-residue, made monic as in odd_model's scaling
            let nu = smith::least_nonsquare(p);
            let ft: Vec<u64> = f.iter().map(|&c| c * nu % p).collect();
            // monic form of ν f (deg 7): h_i = (ν f)_i ν^(6 − i)
            let ftm: Vec<u64> = (0..8)
                .map(|i| {
                    if i == 7 {
                        1
                    } else {
                        ft[i] * pow_mod(nu, (6 - i) as u64, p) % p
                    }
                })
                .collect();
            let tw_exact = JCurve::new(p, 3, &ftm)
                .ok()
                .and_then(|c| c.jacobian_order(seed ^ 0x55).ok());
            let tok = matches!((&z, tw_exact), (Ok(z), Some(t)) if z.twist_order == t as i128);
            agree += ok as usize;
            n12 += ok12 as usize;
            twist_ok += tok as usize;
            rows.push(json!({"seed": seed, "exact": exact, "zeta": z.as_ref().ok().map(|z| z.order as i64),
                "twist_exact": tw_exact, "twist_zeta": z.as_ref().ok().map(|z| z.twist_order as i64)}));
        }
        all_ok &= agree == done && n12 == done && twist_ok == done;
        report.push(
            json!({"check": "deg7_vs_cantor", "p": p, "curves": done, "order_agree": agree,
            "n1_n2_agree": n12, "twist_agree": twist_ok, "rows": rows}),
        );
        // (ii) degree 8 with a rational root, through the odd model.
        let (mut done8, mut agree8, mut tw8) = (0usize, 0usize, 0usize);
        let mut rows8 = Vec::new();
        seed = 0;
        while done8 < n_each {
            seed += 1;
            let f = random_octic(p, seed);
            let mut rng = smith::SplitMix64(seed);
            let Some(fac) = smith::factor_squarefree(&f, p, &mut rng) else {
                continue;
            };
            let Some(lin) = fac.iter().find(|g| g.len() == 2) else {
                continue;
            };
            let r = (p - lin[0]) % p;
            let h = odd_model(&f, r, p);
            let Ok(curve) = JCurve::new(p, 3, &h) else {
                continue;
            };
            let Ok(exact) = curve.jacobian_order(seed) else {
                continue;
            };
            done8 += 1;
            let z = zeta(&exts, &f);
            let ok = matches!(&z, Ok(z) if z.order == exact as i128);
            // twist of the deg-8 model: ν f; its odd model is the twist of h.
            let nu = smith::least_nonsquare(p);
            let ft: Vec<u64> = f.iter().map(|&c| c * nu % p).collect();
            let zt = zeta(&exts, &ft);
            let tok = matches!((&z, &zt), (Ok(a), Ok(b)) if a.twist_order == b.order && b.twist_order == a.order);
            agree8 += ok as usize;
            tw8 += tok as usize;
            rows8.push(
                json!({"seed": seed, "root": r, "lc_square": smith::chi(f[8], p) == 1,
                "exact_via_odd_model": exact, "zeta": z.as_ref().ok().map(|z| z.order as i64)}),
            );
        }
        all_ok &= agree8 == done8 && tw8 == done8;
        report.push(
            json!({"check": "deg8_rational_root_vs_cantor", "p": p, "curves": done8,
            "order_agree": agree8, "twist_selfconsistent": tw8, "rows": rows8}),
        );
        eprintln!("p={p}: deg7 {agree}/{done} (N1,N2 {n12}, twist {twist_ok}); deg8 {agree8}/{done8} (twist {tw8})");
    }
    let doc = json!({"kind": "verify", "all_ok": all_ok, "checks": report});
    std::fs::write(out, serde_json::to_string_pretty(&doc).unwrap()).expect("write --out");
    if all_ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let mode = a.first().cloned().unwrap_or_default();
    if mode != "run" && mode != "verify" {
        eprintln!("usage: smith_even_census (run|verify) --primes P,... --out F [--n 30] [--admitted 150] [--prime-order 40] [--max-h 8] [--max-seeds 30000] [--threads 3]");
        return ExitCode::from(2);
    }
    let (mut primes, mut admitted, mut prime_quota, mut max_h, mut max_seeds, mut threads) = (
        Vec::<u64>::new(),
        150usize,
        40usize,
        8u64,
        30_000u64,
        3usize,
    );
    let (mut out, mut n_each) = (String::new(), 30usize);
    let mut i = 1;
    while i + 1 < a.len() {
        match a[i].as_str() {
            "--primes" => primes = a[i + 1].split(',').map(|s| s.parse().unwrap()).collect(),
            "--admitted" => admitted = a[i + 1].parse().unwrap(),
            "--prime-order" => prime_quota = a[i + 1].parse().unwrap(),
            "--max-h" => max_h = a[i + 1].parse().unwrap(),
            "--max-seeds" => max_seeds = a[i + 1].parse().unwrap(),
            "--threads" => threads = a[i + 1].parse().unwrap(),
            "--n" => n_each = a[i + 1].parse().unwrap(),
            "--out" => out = a[i + 1].clone(),
            f => {
                eprintln!("unknown flag {f}");
                return ExitCode::from(2);
            }
        }
        i += 2;
    }
    if primes.is_empty() || out.is_empty() {
        eprintln!("--primes and --out are required");
        return ExitCode::from(2);
    }
    if primes
        .iter()
        .any(|&p| !(7..=211).contains(&p) || !is_prime_u64(p))
    {
        eprintln!("primes must be odd primes in [7, 211] (enumeration of F_(p^3))");
        return ExitCode::from(2);
    }
    if mode == "verify" {
        return verify(&primes, n_each, &out);
    }
    let file = std::fs::File::create(&out).expect("create --out");
    let w = Mutex::new(std::io::BufWriter::new(file));
    let emit = |v: serde_json::Value| {
        let mut g = w.lock().unwrap();
        writeln!(g, "{v}").unwrap();
        g.flush().unwrap();
    };
    emit(
        json!({"kind": "config", "primes": primes, "admitted": admitted, "prime_order": prime_quota,
        "max_h": max_h, "max_seeds": max_seeds, "threads": threads, "deg_f": 8,
        "monic_normalised": false}),
    );
    let t0 = Instant::now();
    let exts: Vec<[Ext; 3]> = primes.iter().map(|&p| exts_for(p)).collect();
    let q_adm: Vec<AtomicUsize> = primes.iter().map(|_| AtomicUsize::new(0)).collect();
    let q_prime: Vec<AtomicUsize> = primes.iter().map(|_| AtomicUsize::new(0)).collect();
    let next_block: Vec<AtomicUsize> = primes.iter().map(|_| AtomicUsize::new(0)).collect();
    let pi = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| loop {
                let mut found = None;
                for _ in 0..primes.len() {
                    let k = pi.fetch_add(1, Ordering::Relaxed) % primes.len();
                    let need = q_adm[k].load(Ordering::Relaxed) < admitted
                        || q_prime[k].load(Ordering::Relaxed) < prime_quota;
                    if need {
                        let b = next_block[k].fetch_add(1, Ordering::Relaxed) as u64;
                        if b * 64 < max_seeds {
                            found = Some((k, b));
                            break;
                        }
                    }
                }
                let Some((k, b)) = found else { break };
                let p = primes[k];
                for seed in b * 64 + 1..=(b + 1) * 64 {
                    let f = random_octic(p, seed);
                    let lc_square = smith::chi(f[8], p) == 1;
                    let mut frng = smith::SplitMix64(seed ^ 0xF00D ^ p);
                    let Some(fac) = smith::factor_squarefree(&f, p, &mut frng) else {
                        emit(json!({"kind": "curve", "p": p, "seed": seed, "squarefree": false,
                            "f": f, "lc_square": lc_square}));
                        continue;
                    };
                    let mut ct: Vec<u32> = fac.iter().map(|g| (g.len() - 1) as u32).collect();
                    ct.sort_unstable();
                    let kfac = ct.len();
                    let some_odd = ct.iter().any(|d| d % 2 == 1);
                    // Rational 2-rank of J[2] for deg f = 8 over a finite field
                    // (PROTOCOL.md §1): k − 2 with an odd-degree factor, k when
                    // every factor has even degree.  The brief's formula used k − 1
                    // in the second case; both are recorded.
                    let r2 = if some_odd { kfac - 2 } else { kfac };
                    let r2_brief = if some_odd { kfac - 2 } else { kfac - 1 };
                    let mut rng = smith::SplitMix64(seed ^ p.rotate_left(17));
                    let sm = smith::analyze_curve(&f, p, &mut rng);
                    let z = zeta(&exts[k], &f);
                    let mut rec = json!({"kind": "curve", "p": p, "seed": seed, "squarefree": true,
                        "f": f, "lc_square": lc_square, "k": kfac, "cycle_type_8": ct,
                        "stable_matching": has_stable_matching(&ct), "some_odd_factor": some_odd,
                        "r2": r2, "r2_brief": r2_brief, "monic_normalised": false});
                    match &z {
                        Ok(z) => {
                            let n = z.order as u64;
                            let (h, r, adm) = match split_order(n, max_h) {
                                Some((h, r)) => (json!(h), json!(r), true),
                                None => (json!(null), json!(null), false),
                            };
                            let prime = is_prime_u64(n);
                            if adm {
                                q_adm[k].fetch_add(1, Ordering::Relaxed);
                            }
                            if prime {
                                q_prime[k].fetch_add(1, Ordering::Relaxed);
                            }
                            rec["order"] = json!(n);
                            rec["twist_order"] = json!(z.twist_order as u64);
                            rec["n_k"] = json!(z.n);
                            rec["c_k"] = json!([z.c[1] as i64, z.c[2] as i64, z.c[3] as i64]);
                            rec["v2"] = json!(n.trailing_zeros());
                            rec["h"] = h;
                            rec["r"] = r;
                            rec["admitted"] = json!(adm);
                            rec["prime_order"] = json!(prime);
                        }
                        Err(e) => {
                            rec["order"] = json!(null);
                            rec["order_err"] = json!(e);
                            rec["admitted"] = json!(false);
                            rec["prime_order"] = json!(false);
                        }
                    }
                    match sm {
                        Some(o) => {
                            let d = o.diag();
                            let x = smith::x_check(&f, p, &o, false);
                            rec["smith"] = json!({"cycle_type": o.cycle_type, "n_tractable": o.n_tractable(),
                                "n_trig": o.n_trig(), "full_success": o.full_success(false),
                                "full_success_twist": o.full_success(true),
                                "diag_fail": d.pair_condition_fail + d.eval_mismatch + d.lemma4_fail,
                                "x_agree": x.x_agree, "x_disagree": x.x_disagree,
                                "x_count_error": x.x_count_error});
                        }
                        None => rec["smith"] = json!(null),
                    }
                    emit(rec);
                }
            });
        }
    });
    emit(json!({"kind": "done", "wall_s": t0.elapsed().as_secs_f64(),
        "admitted": q_adm.iter().map(|q| q.load(Ordering::Relaxed)).collect::<Vec<_>>(),
        "prime_order": q_prime.iter().map(|q| q.load(Ordering::Relaxed)).collect::<Vec<_>>(),
        "blocks": next_block.iter().map(|q| q.load(Ordering::Relaxed)).collect::<Vec<_>>()}));
    ExitCode::SUCCESS
}
