//! Scaling of exact `hashed-ecdl-tensor` decomposition counts with field size,
//! gadget sizes at cryptographic widths, and the cost composition of the
//! Bochum v0.2 reduction (Remark 2) evaluated with those numbers.
//!
//!   btn-scaling scaling K...      two-slot exact counts on prime-order curves
//!                                 over the largest prime below 2^K, checked
//!                                 against an independent O(p) count
//!   btn-scaling gadget W...       network size of one free-by-free modular
//!                                 product at a W-bit modulus (Auto and Fourier)
//!   btn-scaling report DIR        derived values from the recorded runs in DIR
//!
//! One JSON object per line. Step counts are the crate's Meter charges:
//! deterministic diagnostics, not calibrated time units.

use hashed_ecdl_tensor::arithmetic::is_prime;
use hashed_ecdl_tensor::contraction::Counter;
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::encoding::{Encoder, Multiplier};
use hashed_ecdl_tensor::reduction::{self, FactorBase, HashSeed};
use hashed_ecdl_tensor::{Limits, Meter};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde_json::{json, Value};

type Res<T> = std::result::Result<T, String>;

fn e<E: std::fmt::Display>(x: E) -> String {
    x.to_string()
}

fn meter(steps: u64) -> Meter {
    Meter::new(Limits {
        steps,
        vector: 200_000_000,
        bits: 1_000_000,
    })
}

fn small_prime(n: u64) -> bool {
    n > 1 && (2..).take_while(|q| q * q <= n).all(|q| n % q != 0)
}

/// root[v] = some y with y^2 = v (mod p), or u64::MAX if v is a non-square.
fn roots(p: u64) -> Vec<u64> {
    let mut root = vec![u64::MAX; p as usize];
    for y in 0..p {
        let v = (y as u128 * y as u128 % p as u128) as usize;
        if root[v] == u64::MAX {
            root[v] = y;
        }
    }
    root
}

fn rhs(p: u64, a: u64, b: u64, x: u64) -> u64 {
    let (p, a, b, x) = (p as u128, a as u128, b as u128, x as u128);
    ((x * x % p * x + a * x + b) % p) as u64
}

/// Group order by counting points with the root table: O(p).
fn order(p: u64, a: u64, b: u64, root: &[u64]) -> u64 {
    let mut n = 1u64;
    for x in 0..p {
        let v = rhs(p, a, b, x);
        n += match (v, root[v as usize]) {
            (0, _) => 1,
            (_, u64::MAX) => 0,
            _ => 2,
        };
    }
    n
}

fn pt(x: u64, y: u64) -> Point {
    Point::Affine {
        x: x.into(),
        y: y.into(),
    }
}

/// First (a, b) in lexicographic order (a from 0, b from 1) with prime order
/// r > 3 and r != p; generator: the affine point with the smallest x.
fn curve_for(p: u64, root: &[u64], m: &mut Meter) -> Res<(Curve, u64, u64, u64)> {
    for a in 0..p {
        for b in 1..p {
            if (4 * (a as u128).pow(3) + 27 * (b as u128).pow(2)) % p as u128 == 0 {
                continue;
            }
            let n = order(p, a, b, root);
            if n > 3 && n != p && small_prime(n) {
                let x = (0..p)
                    .find(|&x| root[rhs(p, a, b, x) as usize] != u64::MAX)
                    .ok_or("no point")?;
                let y = root[rhs(p, a, b, x) as usize];
                let c =
                    Curve::new(p.into(), a.into(), b.into(), n.into(), pt(x, y), m).map_err(e)?;
                return Ok((c, a, b, n));
            }
        }
    }
    Err("no prime-order curve".into())
}

fn affine_points(p: u64, a: u64, b: u64, root: &[u64]) -> Vec<Point> {
    let mut v = Vec::new();
    for x in 0..p {
        let r = rhs(p, a, b, x);
        let y = root[r as usize];
        if y == u64::MAX {
            continue;
        }
        v.push(pt(x, y));
        if y != 0 {
            v.push(pt(x, p - y));
        }
    }
    v
}

/// Smallest prime s with 2p <= s (the hash modulus the crate certifies).
fn hash_prime(p: u64, m: &mut Meter) -> Res<BigUint> {
    let mut s = BigUint::from(2 * p);
    while !is_prime(&s, m).map_err(e)? {
        s += 1u8;
    }
    Ok(s)
}

fn largest_prime_below(limit: u64) -> u64 {
    (2..limit).rev().find(|&n| small_prime(n)).unwrap()
}

fn size_record(net: &hashed_ecdl_tensor::network::Network) -> Value {
    json!({ "edges": net.edge_count(), "vertices": net.vertex_count(), "max_degree": net.maximum_degree() })
}

/// Two-slot exact counts at the largest prime below 2^k, thresholds s and s/4,
/// one target (the generator). The independent count walks the first factor
/// base and tests target - A1 against the second: O(p) curve operations.
fn scaling(k: u32) -> Res<Vec<Value>> {
    let p = largest_prime_below(1u64 << k);
    let root = roots(p);
    let mut m = meter(u64::MAX / 2);
    let (c, a, b, r) = curve_for(p, &root, &mut m)?;
    let s = hash_prime(p, &mut m)?;
    let pts = affine_points(p, a, b, &root);
    let target = c.generator().clone();
    let mut out = Vec::new();
    for threshold in [s.clone(), &s >> 2u8] {
        let fb: Vec<FactorBase> = [(1u64, 0u64), (3, 7)]
            .iter()
            .map(|&(al, be)| {
                FactorBase::new(
                    &c,
                    s.clone(),
                    threshold.clone(),
                    HashSeed {
                        alpha: al.into(),
                        beta: be.into(),
                    },
                    &mut m,
                )
                .map_err(e)
            })
            .collect::<Res<_>>()?;
        let mut independent = 0u64;
        for a1 in &pts {
            if fb[0].contains(&c, a1, &mut m).map_err(e)? {
                let a2 = c
                    .add(&target, &c.negate(a1).map_err(e)?, &mut m)
                    .map_err(e)?;
                if a2 != Point::Infinity && fb[1].contains(&c, &a2, &mut m).map_err(e)? {
                    independent += 1;
                }
            }
        }
        let mut bm = meter(u64::MAX / 2);
        let net = reduction::decomposition_network(&c, &fb, &target, &mut bm).map_err(e)?;
        let mut cm = meter(u64::MAX / 2);
        let mut counter = Counter::default();
        let got = counter
            .count(&net, &vec![None; net.edge_count()], &mut cm)
            .map_err(e)?;
        let mut rec = size_record(&net);
        rec["k"] = json!(k);
        rec["p"] = json!(p);
        rec["bits"] = json!(64 - p.leading_zeros());
        rec["r"] = json!(r);
        rec["a"] = json!(a);
        rec["b"] = json!(b);
        rec["threshold"] = json!(threshold.to_string());
        rec["hash_prime"] = json!(s.to_string());
        rec["count"] = json!(got.to_string());
        rec["independent_count"] = json!(independent);
        rec["ok"] = json!(got.to_u64() == Some(independent));
        rec["steps"] = json!(cm.used);
        rec["free_input_bits"] = json!(net.input_bit_count());
        rec["functional_assignments"] = json!(counter.statistics.functional_assignments);
        rec["backend"] = json!(format!("{:?}", counter.statistics.last_backend));
        rec["recursive_calls"] = json!(counter.statistics.recursive_calls);
        out.push(rec);
    }
    Ok(out)
}

/// One free-by-free modular product at a w-bit odd modulus (2^(w-1) + 1;
/// primality does not enter the gadget's size).
fn gadget(w: u32) -> Res<Vec<Value>> {
    let m = meter(u64::MAX / 2);
    let q = (BigUint::from(1u8) << (w - 1)) + 1u8;
    let mut out = Vec::new();
    for (name, multiplier) in [("auto", Multiplier::Auto), ("fourier", Multiplier::Fourier)] {
        let mut enc = Encoder::new();
        enc.set_multiplier(multiplier);
        let x = enc.input_word(w as usize, &m).map_err(e)?;
        let y = enc.input_word(w as usize, &m).map_err(e)?;
        let z = enc.modular_multiply(&x, &y, &q, &m).map_err(e)?;
        enc.mark_interface(&z, &m).map_err(e)?;
        let net = enc.finish(&m).map_err(e)?;
        let mut rec = size_record(&net);
        rec["width"] = json!(w);
        rec["multiplier"] = json!(name);
        out.push(rec);
    }
    Ok(out)
}

fn read_jsonl(path: &str) -> Res<Vec<Value>> {
    std::fs::read_to_string(path)
        .map_err(|err| format!("{path}: {err}"))?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(e))
        .collect()
}

fn num(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}

/// Least-squares line y = slope * x + intercept.
fn fit(points: &[(f64, f64)]) -> Value {
    let n = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x, b + y));
    let (mx, my) = (sx / n, sy / n);
    let sxx: f64 = points.iter().map(|(x, _)| (x - mx).powi(2)).sum();
    let sxy: f64 = points.iter().map(|(x, y)| (x - mx) * (y - my)).sum();
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let ss_res: f64 = points
        .iter()
        .map(|(x, y)| (y - (slope * x + intercept)).powi(2))
        .sum();
    let ss_tot: f64 = points.iter().map(|(_, y)| (y - my).powi(2)).sum();
    json!({ "n": points.len(), "slope": slope, "intercept": intercept, "r2": 1.0 - ss_res / ss_tot })
}

/// Derived values. Inputs: scaling.jsonl, gadget.jsonl, and the ECCp-131
/// network record copied from research/hashed_ecdl_dedupe_20261115 (exact).
fn report(dir: &str) -> Res<Value> {
    let rows = read_jsonl(&format!("{dir}/scaling.jsonl"))?;
    let gadgets = read_jsonl(&format!("{dir}/gadget.jsonl"))?;
    let eccp: Value = serde_json::from_str(
        &std::fs::read_to_string(format!("{dir}/eccp131_network.json")).map_err(e)?,
    )
    .map_err(e)?;
    let all_ok = rows.iter().all(|r| r["ok"] == json!(true));
    let recursive_used: u64 = rows
        .iter()
        .map(|r| r["recursive_calls"].as_u64().unwrap_or(0))
        .sum();
    // Fits over rows with at least 8 bits, separately per threshold.
    let mut fits = serde_json::Map::new();
    for (label, full) in [("threshold_s", true), ("threshold_s_over_4", false)] {
        let sel: Vec<&Value> = rows
            .iter()
            .filter(|r| (r["threshold"] == r["hash_prime"]) == full && num(r, "bits") >= 8.0)
            .collect();
        let pts = |key: &str| -> Vec<(f64, f64)> {
            sel.iter()
                .map(|r| (num(r, "bits"), num(r, key).log2()))
                .collect()
        };
        fits.insert(
            label.into(),
            json!({
                "log2_steps_vs_bits": fit(&pts("steps")),
                "log2_assignments_vs_bits": fit(&pts("functional_assignments")),
                "log2_edges_vs_bits": fit(&pts("edges")),
            }),
        );
    }
    // Rho reference, analytic: expected sqrt(pi r / 4) group operations
    // (log2 = b/2 + log2(pi/4)/2 for r ~ 2^b); slope 0.5 per bit.
    let eccp2 = eccp["networks"]
        .as_array()
        .and_then(|n| n.iter().find(|x| x["slots"] == json!(2)))
        .ok_or("eccp131 2-slot")?
        .clone();
    let eccp3 = eccp["networks"]
        .as_array()
        .and_then(|n| n.iter().find(|x| x["slots"] == json!(3)))
        .ok_or("eccp131 3-slot")?
        .clone();
    let fourier_at = |w: u64| -> Option<f64> {
        gadgets
            .iter()
            .find(|g| g["width"] == json!(w) && g["multiplier"] == json!("fourier"))
            .map(|g| num(g, "edges"))
    };
    let auto_at = |w: u64| -> Option<f64> {
        gadgets
            .iter()
            .find(|g| g["width"] == json!(w) && g["multiplier"] == json!("auto"))
            .map(|g| num(g, "edges"))
    };
    let mult131 = auto_at(131).ok_or("gadget at 131 bits")?;
    let ratio2 = num(&eccp2, "edges") / mult131;
    let ratio3 = num(&eccp3, "edges") / mult131;
    let rho_log2 = |b: f64| b / 2.0 + (std::f64::consts::PI / 4.0).log2() / 2.0;
    let mut table = Vec::new();
    for g in gadgets.iter().filter(|g| g["multiplier"] == json!("auto")) {
        let w = g["width"].as_u64().unwrap_or(0);
        if w < 120 {
            continue;
        }
        let b = w as f64;
        let mult = num(g, "edges");
        for (slots, ratio) in [(2u64, ratio2), (3, ratio3)] {
            let m = slots as f64;
            // Edge count: rigorous lower bound (the network contains at least
            // one such product) and the estimate scaled by the exact ECCp-131 ratio.
            let s_lower = mult;
            let s_est = ratio * mult;
            // Recursive backend (Appendix D.4 guard in recursive.rs): needs
            // lambda = ceil(log2(2s)) >= 131072, i.e. s >= 2^131071 edges.
            let lambda_est = (2.0 * s_est).log2().ceil();
            // Remark 2: O(m^2 b r^(1/m)) solver calls, constant taken as 1.
            let log2_calls = (m * m * b).log2() + b / m;
            // Implemented fallback: 2^(free input bits) assignments per call,
            // free bits (b+1)(m-1) with compressed free points; >= 1 step each.
            let log2_fallback_total_lower = log2_calls + (b + 1.0) * (m - 1.0);
            // Theorem 2 with unknown constant c: per call 2^(c s / sqrt(log2 s)).
            // Break-even c* (total = rho), using the estimated s.
            let per_call_unit = s_est / s_est.log2().sqrt();
            let budget = rho_log2(b) - log2_calls;
            let c_star = budget / per_call_unit;
            table.push(json!({
                "bits": w, "slots": slots,
                "modular_product_edges": mult,
                "edges_lower_bound": s_lower,
                "edges_estimate": s_est,
                "log2_edges_estimate": s_est.log2(),
                "lambda_estimate": lambda_est,
                "recursion_needs_lambda": 131072,
                "recursion_engages": lambda_est >= 131072.0,
                "log2_solver_calls": log2_calls,
                "log2_fallback_total_lower": log2_fallback_total_lower,
                "log2_rho": rho_log2(b),
                "theorem2_budget_bits_per_call": budget,
                "theorem2_break_even_constant": c_star,
            }));
        }
    }
    Ok(json!({
        "scaling": { "rows": rows.len(), "all_match_independent_count": all_ok, "recursive_calls_total": recursive_used,
                     "fits_8_bits_and_up": fits, "rho_slope_per_bit": 0.5 },
        "eccp131": { "two_slot_edges": eccp2["edges"], "three_slot_edges": eccp3["edges"],
                     "modular_product_edges_131": mult131, "ratio_two_slot": ratio2, "ratio_three_slot": ratio3,
                     "fourier_product_edges_131": fourier_at(131) },
        "cost_table": table,
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nums = |from: usize| -> Vec<u32> {
        args[from..]
            .iter()
            .map(|a| a.parse().expect("number"))
            .collect()
    };
    let run = || -> Res<()> {
        match args.get(1).map(String::as_str) {
            Some("scaling") => {
                for k in nums(2) {
                    for rec in scaling(k)? {
                        println!("{rec}");
                    }
                }
            }
            Some("gadget") => {
                for w in nums(2) {
                    for rec in gadget(w)? {
                        println!("{rec}");
                    }
                }
            }
            Some("report") => println!(
                "{}",
                serde_json::to_string_pretty(&report(&args[2])?).map_err(e)?
            ),
            _ => return Err("usage: scaling K... | gadget W... | report DIR".into()),
        }
        Ok(())
    };
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
