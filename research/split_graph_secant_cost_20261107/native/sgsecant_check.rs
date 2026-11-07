//! Development checks. Labelled as such: they support correctness only and no
//! number printed here is a measurement for any claim.
use crate::alg::*;
use crate::ec::*;
use crate::ops::*;
use crate::sys::*;
use std::collections::BTreeSet;

fn small_curve(p: u64, a: u64, b: u64) -> Curve {
    let pts = enumerate_points(a, b, p);
    Curve { p, a, b, n: pts.len() as u64 + 1, points: pts }
}

pub fn check_unit() {
    reset();
    // univariate roots vs brute force
    let mut bad = 0;
    for &p in &[17u64, 83, 127, 491] {
        let mut seed = 12345u64;
        for t in 0..300 {
            let deg = 2 + t % 9;
            let mut f: Vec<u64> = (0..=deg)
                .map(|_| {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    (seed >> 33) % p
                })
                .collect();
            if f[deg] == 0 {
                f[deg] = 1;
            }
            // include planted roots sometimes
            let mut want: Vec<u64> = (0..p).filter(|x| f.iter().rev().fold(0u64, |a, c| (a * x + c) % p) == 0).collect();
            let mut got = roots_fp(&f, p);
            want.sort();
            got.sort();
            if want != got {
                bad += 1;
            }
        }
    }
    println!("univariate root finding vs brute force: mismatches = {bad}");
    // a small bivariate system with known solutions: circle x^2+z^2=1, line x+z=1 over F_p
    let p = 101;
    let nv = 2;
    let f1 = normalize(vec![(2, 1), (2 << 8, 1), (0, p - 1)], p, nv);
    let f2 = normalize(vec![(1, 1), (1 << 8, 1), (0, p - 1)], p, nv);
    let ex = extract(&[f1, f2], p, nv);
    println!("circle/line over F101: points {:?} dim {} (expect (0,1),(1,0))", ex.points, ex.dim);
    let mut tot = 0;
    for (name, l) in snapshot().iter().enumerate() {
        let _ = name;
        tot += l.field();
    }
    println!("ledger field ops total in unit checks: {tot}");
}

fn exact_all(c: &Curve, bases: &[Base], label: &str, limit_targets: usize) {
    let presses = [Pres::Secant1, Pres::Secant2, Pres::Dense];
    for b in bases {
        for pres in presses {
            if b.graph == false && pres == Pres::Secant1 && false {
                continue;
            }
            let mut mism = 0;
            let mut cens = 0;
            let mut nrel = 0usize;
            let mut cov = 0;
            let s0 = snapshot();
            let t0 = std::time::Instant::now();
            let mut hd = 0;
            let mut dim = 0;
            let mut fbs = 0u32;
            for (ti, rt) in c.points.iter().enumerate() {
                if ti >= limit_targets {
                    break;
                }
                set_budget(2_000_000_000);
                let sol = solve(c, b, *rt, pres);
                set_budget(0);
                if sol.censored {
                    cens += 1;
                    continue;
                }
                let rf = reference_set(c, b, *rt);
                if rf != sol.pairs || sol.replay_fail > 0 || sol.unresolved || sol.not_zero_dim {
                    mism += 1;
                }
                nrel += rf.len();
                if !rf.is_empty() {
                    cov += 1;
                }
                hd = hd.max(sol.handled_degree);
                dim = dim.max(sol.dim_sum);
                fbs += sol.fallbacks;
            }
            let d = diff(&snapshot(), &s0);
            let t = total(&d);
            println!(
                "[{label}] base={:<8} pres={:?}: targets={} mismatches={} censored={} covered={} ordered_pairs={} handled_degree_max={} dim_max={} fallbacks={} field_ops={} native={} secs={:.2}",
                b.name, pres, limit_targets.min(c.points.len()), mism, cens, cov, nrel, hd, dim, fbs, t.field(), t.native, t0.elapsed().as_secs_f64()
            );
        }
    }
}

pub fn check_f17() {
    reset();
    let c = small_curve(17, 2, 2);
    println!("F17 y^2=x^3+2x+2: order {} (expect 19)", c.n);
    let cand = make_base(&c, "graph", vec![5, 6, 10, 16], vec![9, 9, 4], true);
    let ctrl = make_base(&c, "nongraph", vec![0, 5, 6, 10], vec![6, 7, 6, 8], false);
    // graph identity h^2-f = a^2 prod(T-v)
    let ok = (0..17u64).all(|x| {
        let y = ueval(&cand.h, x, 17);
        let hh = y * y % 17;
        let f = (x * x * x + 2 * x + 2) % 17;
        let m: u64 = cand.xs.iter().fold(1, |a, v| a * ((x + 17 - v) % 17) % 17);
        (hh + 17 - f) % 17 == 16 * m % 17
    });
    println!("graph identity h^2 - f = 16 prod(T-v) over F17: {}", ok);
    exact_all(&c, &[cand, ctrl], "F17-fixture", 18);
    println!("(development check; ledger phases: {:?})", PHASES);
}

fn legacy_spec() -> Spec {
    Spec { domain: "secant-graph-ladder/v1".into(), seed: 315, curve_cap: 256, graph_cap: 512 }
}

pub fn check_legacy(p: &str, d: &str) {
    reset();
    let p: u64 = p.parse().unwrap();
    let d: usize = d.parse().unwrap();
    let s = legacy_spec();
    phase(PH_CONSTRUCT);
    let s0 = snapshot();
    let cs = select_curve(&s, p, "curve");
    let c = cs.curve.expect("curve");
    println!("legacy ladder reproduction: p={p} curve a={} b={} order {} attempts {}", c.a, c.b, c.n, cs.attempts);
    let s1 = snapshot();
    let gs = graph_search(&s, &c, d, "forced");
    let s2 = snapshot();
    let g = gs.base.expect("graph");
    let gl = total(&diff(&s2, &s1));
    println!("graph attempts {} (historical record in results-summary.json); field adds {} muls {} inv {}", gs.attempts, gl.add, gl.mul, gl.inv);
    let cl = total(&diff(&s1, &s0));
    println!("curve selection ledger: add {} mul {} inv {}", cl.add, cl.mul, cl.inv);
    let lowx = control_base(&s, &c, d, "low-x", "control");
    let hashx = control_base(&s, &c, d, "hash-x", "control");
    println!("graph xs {:?}", g.xs);
    let lim: usize = std::env::var("SG_LIMIT").ok().and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
    exact_all(&c, &[g, lowx, hashx], &format!("legacy-F{p}-d{d}"), lim);
}

// ---- faithful port of the historical coordinate-cube root extractor (for cap replay only)
use std::collections::BTreeMap;
type SPoly = BTreeMap<Vec<u32>, u64>;
fn spec_poly(f: &Poly, vals: &[Option<u64>], p: u64, nv: usize) -> SPoly {
    let mut out: SPoly = BTreeMap::new();
    for &(m, c) in f {
        let mut e = vec![0u32; nv];
        let mut coef = c;
        for i in 0..nv {
            let k = exp(m, i);
            match vals[i] {
                Some(v) => {
                    let mut pw = 1u64;
                    for _ in 0..k {
                        pw = pw * v % p;
                    }
                    coef = coef * pw % p;
                }
                None => e[i] = k,
            }
        }
        let t = out.entry(e).or_insert(0);
        *t = (*t + coef) % p;
    }
    out.retain(|_, v| *v != 0);
    out
}
fn s_eval(f: &SPoly, z: &[u64], p: u64) -> u64 {
    let mut acc = 0;
    for (m, c) in f {
        let mut t = *c;
        for (i, e) in m.iter().enumerate() {
            for _ in 0..*e {
                t = t * z[i] % p;
            }
        }
        acc = (acc + t) % p;
    }
    acc
}
fn hist_visit(g: &[Poly], vals: &mut Vec<Option<u64>>, p: u64, nv: usize, tries: &mut u64) -> Result<(), ()> {
    fn go(gs: &[SPoly], v: &mut Vec<Option<u64>>, p: u64, tries: &mut u64) -> Result<(), ()> {
        if gs.iter().any(|f| f.len() == 1 && f.keys().next().unwrap().iter().all(|e| *e == 0)) {
            return Ok(());
        }
        if v.iter().all(Option::is_some) {
            return Ok(());
        }
        let univariate = gs
            .iter()
            .filter_map(|f| {
                let used: BTreeSet<usize> = f.keys().flat_map(|m| m.iter().enumerate().filter_map(|(i, e)| (*e != 0).then_some(i))).collect();
                (used.len() == 1).then(|| (*used.iter().next().unwrap(), f))
            })
            .min_by_key(|(_, f)| f.keys().map(|m| m.iter().sum::<u32>()).max().unwrap_or(0));
        let index = univariate.map(|(i, _)| i).unwrap_or_else(|| v.iter().position(Option::is_none).unwrap());
        for x in 0..p {
            *tries += 1;
            if *tries > 2_000_000 {
                return Err(());
            }
            v[index] = Some(x);
            if let Some((_, f)) = univariate {
                let z: Vec<u64> = v.iter().map(|x| x.unwrap_or(0)).collect();
                if s_eval(f, &z, p) != 0 {
                    continue;
                }
            }
            // specialise by the new value
            let next: Vec<SPoly> = gs
                .iter()
                .map(|f| {
                    let mut out: SPoly = BTreeMap::new();
                    for (m, c) in f {
                        let mut m2 = m.clone();
                        let k = m2[index];
                        m2[index] = 0;
                        let mut pw = 1u64;
                        for _ in 0..k {
                            pw = pw * x % p;
                        }
                        let t = out.entry(m2).or_insert(0);
                        *t = (*t + c * pw) % p;
                    }
                    out.retain(|_, v| *v != 0);
                    out
                })
                .filter(|f| !f.is_empty())
                .collect();
            go(&next, v, p, tries)?;
        }
        v[index] = None;
        Ok(())
    }
    let gs: Vec<SPoly> = g.iter().map(|f| spec_poly(f, vals, p, nv)).filter(|f| !f.is_empty()).collect();
    go(&gs, vals, p, tries)
}

/// Replays the historical assignment-cube extractor (cap 2,000,000) on the reduced DRL
/// basis of each F127/d=5 candidate-secant branch system, to count how many original-style
/// extractions would cap. The historical panel itself is not rewritten.
pub fn check_cap() {
    reset();
    let s = legacy_spec();
    let p = 127u64;
    let d = 5usize;
    let c = select_curve(&s, p, "curve").curve.unwrap();
    let g = graph_search(&s, &c, d, "forced").base.unwrap();
    let lowx = control_base(&s, &c, d, "low-x", "control");
    let hashx = control_base(&s, &c, d, "hash-x", "control");
    let only: Option<usize> = std::env::var("SG_ONLY").ok().and_then(|x| x.parse().ok());
    for b in [&g, &lowx, &hashx] {
        let mut capped = 0;
        let mut total_sys = 0;
        let mut capped_ids = Vec::new();
        for (ti, rt) in c.points.iter().enumerate() {
            if let Some(o) = only {
                if o != ti {
                    continue;
                }
            }
            for eps in [1i8, -1] {
                for eta in [1i8, -1] {
                    let sys = secant_branch(&c, b, rt.0, rt.1, eps, eta);
                    let gb = groebner(&sys, p, 3).unwrap();
                    let mut vals = vec![None; 3];
                    let mut tries = 0u64;
                    total_sys += 1;
                    if hist_visit(&gb.polys, &mut vals, p, 3, &mut tries).is_err() {
                        capped += 1;
                        capped_ids.push((ti, eps, eta));
                    }
                }
            }
        }
        println!("historical-extractor replay F127 d5 secant arm {:<8}: systems {} would-cap {} ids {:?}", b.name, total_sys, capped, capped_ids);
    }
    println!("(historical record: 12 candidate-secant, 6 low-x secant, 8 hash-x secant capped; panel stays censored)");
}
