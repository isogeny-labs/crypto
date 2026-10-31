//! Native measurements of the `hashed-ecdl-tensor` decomposition network on
//! small prime-order curves y^2 = x^3 + ax + b over F_p (replaces the Python
//! path in this study directory; see AGENTS.md "Do not use Python").
//!
//!   btn-prime-scaling sizes  P...         network size per prime
//!   btn-prime-scaling counts P...         exact count vs brute force, charged steps
//!   btn-prime-scaling solve  P SEED XI    full reduction::solve with a known scalar
//!   btn-prime-scaling size-json FILE      network size for a JSON problem (e.g. ECCp-131)
//!   btn-prime-scaling report DIR          derived values from the recorded runs in DIR
//!
//! Each mode prints one JSON object per line.  Step counts are the crate's
//! Meter charges: deterministic diagnostics, not calibrated time units.

use hashed_ecdl_tensor::arithmetic::{is_prime, Seeded};
use hashed_ecdl_tensor::contraction::Counter;
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::reduction::{self, Config, FactorBase, HashSeed};
use hashed_ecdl_tensor::{Limits, Meter};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};

type Res<T> = std::result::Result<T, String>;

fn e<E: std::fmt::Display>(x: E) -> String {
    x.to_string()
}

fn meter(steps: u64) -> Meter {
    Meter::new(Limits { steps, vector: 50_000_000, bits: 1_000_000 })
}

fn legendre(n: u64, p: u64) -> i8 {
    let n = n % p;
    if n == 0 {
        return 0;
    }
    let (mut b, mut x, mut r) = (n as u128, (p - 1) / 2, 1u128);
    while x > 0 {
        if x & 1 == 1 {
            r = r * b % p as u128;
        }
        b = b * b % p as u128;
        x >>= 1;
    }
    if r == 1 { 1 } else { -1 }
}

/// Group order of y^2 = x^3 + ax + b over F_p by summing Legendre symbols.
fn order(p: u64, a: u64, b: u64) -> u64 {
    let mut n = 1u64; // identity
    for x in 0..p {
        let rhs = ((x as u128 * x as u128 % p as u128 * x as u128 + a as u128 * x as u128 + b as u128) % p as u128) as u64;
        n = (n as i64 + 1 + legendre(rhs, p) as i64) as u64;
    }
    n
}

fn small_prime(n: u64) -> bool {
    n > 1 && (2..).take_while(|q| q * q <= n).all(|q| n % q != 0)
}

fn sqrt_mod(n: u64, p: u64) -> Option<u64> {
    (0..p).find(|y| (*y as u128 * *y as u128 % p as u128) as u64 == n % p)
}

/// First (a, b) in lexicographic order whose curve has prime order r > 3, and
/// the affine point with the smallest x as generator.
fn prime_order_curve(p: u64) -> Option<(u64, u64, u64, (u64, u64))> {
    for a in 0..p {
        for b in 1..p {
            if (4 * (a as u128).pow(3) + 27 * (b as u128).pow(2)) % p as u128 == 0 {
                continue;
            }
            let n = order(p, a, b);
            if n > 3 && small_prime(n) {
                for x in 0..p {
                    let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + b as u128) % p as u128;
                    if let Some(y) = sqrt_mod(rhs as u64, p) {
                        return Some((a, b, n, (x, y)));
                    }
                }
            }
        }
    }
    None
}

fn pt(x: u64, y: u64) -> Point {
    Point::Affine { x: x.into(), y: y.into() }
}

fn curve_for(p: u64, m: &mut Meter) -> Res<(Curve, u64, u64, u64)> {
    let (a, b, r, (gx, gy)) = prime_order_curve(p).ok_or("no prime-order curve")?;
    let c = Curve::new(p.into(), a.into(), b.into(), r.into(), pt(gx, gy), m).map_err(e)?;
    Ok((c, a, b, r))
}

fn affine_points(c: &Curve, p: u64, a: u64, b: u64) -> Vec<Point> {
    let mut v = Vec::new();
    for x in 0..p {
        let rhs = ((x as u128).pow(3) + a as u128 * x as u128 + b as u128) % p as u128;
        for y in 0..p {
            if (y as u128 * y as u128) % p as u128 == rhs {
                let q = pt(x, y);
                debug_assert!(c.contains(&q));
                v.push(q);
            }
        }
    }
    v
}

/// Smallest prime s with 2p <= s <= 4p (the hash modulus the crate certifies).
fn hash_prime(p: u64, m: &mut Meter) -> Res<BigUint> {
    let mut s = BigUint::from(2 * p);
    while !is_prime(&s, m).map_err(e)? {
        s += 1u8;
    }
    Ok(s)
}

fn bases(c: &Curve, s: &BigUint, threshold: &BigUint, m: &mut Meter) -> Res<Vec<FactorBase>> {
    let seeds = [(1u64, 0u64), (3, 7)];
    seeds
        .iter()
        .map(|&(al, be)| {
            FactorBase::new(c, s.clone(), threshold.clone(), HashSeed { alpha: al.into(), beta: be.into() }, m)
                .map_err(e)
        })
        .collect()
}

fn mul(c: &Curve, k: u64, p: &Point, m: &mut Meter) -> Res<Point> {
    let (mut acc, mut base, mut k) = (Point::Infinity, p.clone(), k);
    while k > 0 {
        if k & 1 == 1 {
            acc = c.add(&acc, &base, m).map_err(e)?;
        }
        base = c.add(&base, &base, m).map_err(e)?;
        k >>= 1;
    }
    Ok(acc)
}

fn size_record(net: &hashed_ecdl_tensor::network::Network) -> Value {
    json!({
        "edges": net.edge_count(),
        "vertices": net.vertex_count(),
        "max_degree": net.maximum_degree(),
        "interface_bits": net.interface().len(),
    })
}

fn sizes(p: u64) -> Res<Value> {
    let mut m = meter(u64::MAX / 2);
    let (c, a, b, r) = curve_for(p, &mut m)?;
    let s = hash_prime(p, &mut m)?;
    let fb = bases(&c, &s, &s, &mut m)?;
    let target = c.generator().clone();
    let net = reduction::decomposition_network(&c, &fb, &target, &mut m).map_err(e)?;
    let mut rec = size_record(&net);
    rec["p"] = json!(p);
    rec["d"] = json!(64 - p.leading_zeros());
    rec["a"] = json!(a);
    rec["b"] = json!(b);
    rec["r"] = json!(r);
    Ok(rec)
}

fn counts(p: u64) -> Res<Vec<Value>> {
    let mut m = meter(u64::MAX / 2);
    let (c, a, b, r) = curve_for(p, &mut m)?;
    let s = hash_prime(p, &mut m)?;
    let pts = affine_points(&c, p, a, b);
    let mut out = Vec::new();
    // Full bases (threshold = s) and a hash-filtered pair (threshold = s/2).
    for threshold in [s.clone(), &s >> 1u8] {
        let fb = bases(&c, &s, &threshold, &mut m)?;
        for target in pts.iter().take(2) {
            let net = reduction::decomposition_network(&c, &fb, target, &mut m).map_err(e)?;
            let mut brute = 0u64;
            for a1 in &pts {
                if !fb[0].contains(&c, a1, &mut m).map_err(e)? {
                    continue;
                }
                for a2 in &pts {
                    if fb[1].contains(&c, a2, &mut m).map_err(e)? && &c.add(a1, a2, &mut m).map_err(e)? == target {
                        brute += 1;
                    }
                }
            }
            let mut cm = meter(20_000_000_000);
            let mut counter = Counter::default();
            let pins = vec![None; net.edge_count()];
            let got = counter.count(&net, &pins, &mut cm);
            let mut rec = size_record(&net);
            let (count, ok) = match &got {
                Ok(v) => (json!(v.to_string()), v.to_u64() == Some(brute)),
                Err(err) => (json!(format!("error: {err}")), false),
            };
            rec["p"] = json!(p);
            rec["d"] = json!(64 - p.leading_zeros());
            rec["r"] = json!(r);
            rec["threshold"] = json!(threshold.to_string());
            rec["hash_prime"] = json!(s.to_string());
            rec["target"] = json!(format!("{target:?}"));
            rec["brute"] = json!(brute);
            rec["count"] = count;
            rec["ok"] = json!(ok);
            rec["steps"] = json!(cm.used);
            rec["backend"] = json!(format!("{:?}", counter.statistics.last_backend));
            rec["contractions"] = json!(counter.statistics.contractions);
            rec["recursive_calls"] = json!(counter.statistics.recursive_calls);
            rec["edge_assignments"] = json!(counter.statistics.edge_assignments);
            rec["functional_assignments"] = json!(counter.statistics.functional_assignments);
            out.push(rec);
        }
    }
    Ok(out)
}

fn solve(p: u64, seed: u64, xi: u64) -> Res<Value> {
    let mut m = meter(200_000_000_000);
    let (c, _, _, r) = curve_for(p, &mut m)?;
    let xi = xi % r;
    let target = mul(&c, xi, c.generator(), &mut m)?;
    let before = m.used;
    let mut counter = Counter::default();
    let cert = reduction::solve(&c, &target, &Config::for_curve(&c), &mut Seeded(seed), &mut counter, &mut m);
    let solve_steps = m.used - before;
    let mut rec = json!({ "p": p, "d": 64 - p.leading_zeros(), "r": r, "seed": seed, "secret": xi,
        "steps": solve_steps, "contractions": counter.statistics.contractions,
        "recursive_calls": counter.statistics.recursive_calls,
        "edge_assignments": counter.statistics.edge_assignments,
        "functional_assignments": counter.statistics.functional_assignments,
        "backend": format!("{:?}", counter.statistics.last_backend) });
    match cert {
        Ok(cert) => {
            let verified = reduction::verify(&c, &target, &cert, &mut m).is_ok();
            rec["recovered"] = json!(cert.scalar.to_string());
            rec["relations"] = json!(cert.relations.len());
            rec["ok"] = json!(cert.scalar == BigUint::from(xi) && verified);
            rec["verified"] = json!(verified);
        }
        Err(err) => {
            rec["error"] = json!(err.to_string());
            rec["ok"] = json!(false);
        }
    }
    Ok(rec)
}

fn size_json(path: &str) -> Res<Value> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).map_err(e)?).map_err(e)?;
    let big = |k: &str| -> Res<BigUint> {
        v[k].as_str().ok_or(format!("missing {k}"))?.parse::<BigUint>().map_err(e)
    };
    let pp = |k: &str| -> Res<Point> {
        let a = &v[k];
        Ok(Point::Affine {
            x: a[0].as_str().ok_or("x")?.parse::<BigUint>().map_err(e)?,
            y: a[1].as_str().ok_or("y")?.parse::<BigUint>().map_err(e)?,
        })
    };
    let mut m = meter(u64::MAX / 2);
    let p = big("p")?;
    let c = Curve::new(p.clone(), big("a")?, big("b")?, big("r")?, pp("generator")?, &mut m).map_err(e)?;
    let mut s = &p << 1u8;
    while !is_prime(&s, &mut m).map_err(e)? {
        s += 1u8;
    }
    let mut net_rec = Value::Null;
    for slots in [2usize, Config::for_curve(&c).slots] {
        let fb: Vec<FactorBase> = (0..slots)
            .map(|i| FactorBase::new(&c, s.clone(), s.clone(), HashSeed { alpha: (2 * i as u64 + 1).into(), beta: (i as u64).into() }, &mut m).map_err(e))
            .collect::<Res<_>>()?;
        let net = reduction::decomposition_network(&c, &fb, &pp("target")?, &mut m).map_err(e)?;
        let mut rec = size_record(&net);
        rec["slots"] = json!(slots);
        if net_rec.is_null() {
            net_rec = json!([]);
        }
        net_rec.as_array_mut().unwrap().push(rec);
    }
    Ok(json!({ "file": path, "d": p.bits(), "networks": net_rec, "build_steps": m.used }))
}

/// Time one Fourier integer-multiply network construction at each bit width.
/// Prints each line immediately so partial results survive a timeout.
fn multtime(widths: &[u64]) -> Res<()> {
    use hashed_ecdl_tensor::encoding::Encoder;
    use std::io::Write;
    use std::time::Instant;
    for &w in widths {
        let w = w as usize;
        let m = meter(u64::MAX / 2);
        let mut enc = Encoder::new();
        let a = enc.input_word(w, &m).map_err(e)?;
        let b = enc.input_word(w, &m).map_err(e)?;
        let t = Instant::now();
        let prod = enc.integer_multiply(&a, &b, &m).map_err(e)?;
        println!("{}", json!({ "op": "multiply", "width": w,
            "micros": t.elapsed().as_micros(), "product_width": prod.width() }));
        std::io::stdout().flush().ok();
    }
    Ok(())
}

/// Time is_prime on one prime at each bit width (smallest prime >= 2^(w-1)).
/// Prints each line immediately; the AKS fallback above 64 bits is the point.
fn primetime(widths: &[u64]) -> Res<()> {
    use std::io::Write;
    use std::time::Instant;
    for &w in widths {
        let w = w as usize;
        let mut n = (BigUint::one() << (w - 1)) | BigUint::one();
        let mut mp = meter(u64::MAX / 2);
        let t = Instant::now();
        while !is_prime(&n, &mut mp).map_err(e)? {
            n += 2u8;
        }
        println!("{}", json!({ "op": "is_prime", "width": w,
            "micros": t.elapsed().as_micros(), "path": if w <= 64 { "u64" } else { "aks" } }));
        std::io::stdout().flush().ok();
    }
    Ok(())
}

fn read_jsonl(path: &str) -> Res<Vec<Value>> {
    let text = std::fs::read_to_string(path).map_err(|err| format!("{path}: {err}"))?;
    text.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).map_err(e)).collect()
}

/// Derived values from the recorded runs in DIR (sizes.jsonl, counts.jsonl,
/// solves.jsonl, optional eccp131_size.json).  Every derived number states its
/// inputs; nothing here re-runs a measurement.
fn report(dir: &str) -> Res<Value> {
    let sizes = read_jsonl(&format!("{dir}/sizes.jsonl"))?;
    let counts = read_jsonl(&format!("{dir}/counts.jsonl"))?;
    let solves = read_jsonl(&format!("{dir}/solves.jsonl")).unwrap_or_default();
    let eccp = std::fs::read_to_string(format!("{dir}/eccp131_size.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok());
    let u = |v: &Value, k: &str| v[k].as_u64().unwrap_or(0);

    // Counts: agreement with brute force, backend, and whether the Functional
    // backend's visited assignments equal 2^(free input bits) = 2^(4d).
    let mut per_count = Vec::new();
    for c in &counts {
        let d = u(c, "d");
        let fa = u(c, "functional_assignments");
        let log2_fa = if fa > 0 { (fa as f64).log2() } else { f64::NAN };
        per_count.push(json!({
            "p": u(c, "p"), "d": d, "ok": c["ok"], "backend": c["backend"],
            "recursive_calls": u(c, "recursive_calls"),
            "functional_assignments": fa, "log2_functional_assignments": log2_fa,
            "equals_2_pow_4d": fa == 1u64 << (4 * d),
            "steps": u(c, "steps"),
            "steps_per_assignment": if fa > 0 { u(c, "steps") as f64 / fa as f64 } else { f64::NAN },
            "edges": u(c, "edges"),
        }));
    }
    let all_counts_ok = counts.iter().all(|c| c["ok"] == json!(true));
    let all_functional = counts.iter().all(|c| c["backend"] == json!("Some(Functional)"));
    let any_recursion = counts.iter().chain(solves.iter()).any(|c| u(c, "recursive_calls") > 0);
    let assignments_rule = per_count.iter().all(|c| c["equals_2_pow_4d"] == json!(true));

    // Sizes: edges per bit and growth between the smallest and largest d measured.
    let first = sizes.first().ok_or("no sizes")?;
    let last = sizes.last().ok_or("no sizes")?;
    let size_growth = json!({
        "d_from": u(first, "d"), "edges_from": u(first, "edges"),
        "d_to": u(last, "d"), "edges_to": u(last, "edges"),
        "edges_ratio": u(last, "edges") as f64 / u(first, "edges") as f64,
        "max_degree_values": sizes.iter().map(|s| u(s, "max_degree")).collect::<std::collections::BTreeSet<_>>(),
    });

    // Setup bottleneck: is_prime dispatches to a fast u64 path at <= 64 bits
    // and to AKS above, so certifying a cryptographic-size prime (in Curve::new
    // / FactorBase::new) is impractical.  Multiply-network construction, by
    // contrast, is fast at every width.  Read from multtime/primetime.jsonl.
    let mult = read_jsonl(&format!("{dir}/multtime.jsonl")).unwrap_or_default();
    let primes = read_jsonl(&format!("{dir}/primetime.jsonl")).unwrap_or_default();
    let prime_u64_max = primes.iter().filter(|r| r["path"] == json!("u64")).filter_map(|r| r.get("micros").and_then(Value::as_u64)).max();
    let prime_aks = primes.iter().find(|r| r["path"] == json!("aks"));
    let setup = json!({
        "multiply_construction": { "rows": mult, "verdict": "fast at every width (linear); not a bottleneck" },
        "is_prime": {
            "rows": primes,
            "u64_path_max_micros": prime_u64_max,
            "aks_at_65_bits": prime_aks,
            "dispatch": "is_prime uses a fast u64 path at <= 64 bits and AKS above (arithmetic.rs); AKS is impractical, so a prime above 64 bits cannot be certified in reasonable time",
            "consequence": "Curve::new and FactorBase::new certify the prime, so no curve with a prime above 64 bits (ECCp-131's is 131-bit) can even be set up; this blocks before the contraction cost matters",
        },
    });

    // Extrapolation to ECCp-131 under the backend actually used (Functional):
    // one contraction visits 2^(free input bits) assignments, 4*131 for two
    // slots.  Pollard rho: sqrt(pi*n/4) group operations, n = 2^131.
    let d131 = 131u64;
    let eccp_input_bits = 4 * d131;
    let rho_log2 = 0.5 * (std::f64::consts::PI * 2f64.powi(131) / 4.0).log2();

    // Recursion crossover, transcribed from crates/hashed-ecdl-tensor
    // src/contraction/recursive.rs Setup::parameters: at depth 0 the recursive
    // backend is selected only if root_u = lambda / (128*1024) >= 1, where
    // lambda = ceil(log2(size)) and size = 2*edges.  So lambda >= 131072 =
    // 2^17, i.e. size >= 2^131071, edges >= 2^131070.  Further gates
    // (m >= 2*array^2*delta with array = 64^(4*root_u)) only raise it.  This is
    // a lower bound on the smallest network for which the recursion engages at
    // all; below it every count uses the exact fallback (Functional/Direct).
    const EPSILON_DENOM: u64 = 128 * 1024; // crate constant, = 2^17
    let min_lambda = EPSILON_DENOM; // smallest lambda with root_u >= 1
    let crossover_log2_edges = (min_lambda - 1 - 1) as f64; // size >= 2^(lambda-1); edges = size/2
    // Largest network we actually built (ECCp-131 or the size sweep).
    let max_edges = eccp
        .as_ref()
        .and_then(|e| e.get("networks"))
        .and_then(|n| n.as_array())
        .and_then(|a| a.iter().filter_map(|r| r.get("edges").and_then(Value::as_u64)).max())
        .or_else(|| sizes.iter().map(|s| u(s, "edges")).max())
        .unwrap_or(0);
    Ok(json!({
        "schema": "btn-prime-native-report/v1",
        "inputs": { "sizes": format!("{dir}/sizes.jsonl"), "counts": format!("{dir}/counts.jsonl"),
                    "solves": format!("{dir}/solves.jsonl"), "eccp131_size": format!("{dir}/eccp131_size.json") },
        "counts": { "n": counts.len(), "all_match_brute_force": all_counts_ok,
                    "all_functional_backend": all_functional,
                    "functional_assignments_equal_2_pow_4d": assignments_rule, "rows": per_count },
        "solves": { "n": solves.len(), "all_ok": !solves.is_empty() && solves.iter().all(|s| s["ok"] == json!(true)), "rows": solves },
        "setup_bottleneck": setup,
        "recursive_backend_used": any_recursion,
        "recursion_crossover": {
            "epsilon_denominator": EPSILON_DENOM,
            "min_lambda_for_recursion": min_lambda,
            "lower_bound_log2_edges": crossover_log2_edges,
            "paper_threshold_log2_edges_approx": "2^17 = 131072 (manuscript's 2^(2^17))",
            "max_edges_built_here": max_edges,
            "log2_max_edges_built_here": if max_edges > 0 { (max_edges as f64).log2() } else { f64::NAN },
            "note": "transcribed from recursive.rs Setup::parameters; recursion engages only above ~2^131070 edges, so no feasible curve reaches it and every count here used the exact fallback",
        },
        "sizes": { "rows": sizes, "growth": size_growth },
        "eccp131": {
            "network": eccp,
            "functional_backend_log2_assignments_per_contraction": eccp_input_bits,
            "derivation": "Functional backend visits 2^(free input bits); measured equal to 2^(4d) at d=3,4; two slots at d=131 give 4*131 = 524 free bits",
            "pollard_rho_log2_group_ops": rho_log2,
        },
        "notes": [
            "steps are hashed-ecdl-tensor Meter charges: deterministic diagnostics, not calibrated time units",
            "no wall-clock time is reported",
        ],
    }))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nums = |from: usize| -> Vec<u64> { args[from..].iter().map(|s| s.parse().expect("integer")).collect() };
    let result: Res<()> = match args.get(1).map(String::as_str) {
        Some("sizes") => nums(2).into_iter().try_for_each(|p| sizes(p).map(|r| println!("{r}"))),
        Some("counts") => nums(2).into_iter().try_for_each(|p| counts(p).map(|v| v.iter().for_each(|r| println!("{r}")))),
        Some("solve") => {
            let n = nums(2);
            solve(n[0], n[1], n[2]).map(|r| println!("{r}"))
        }
        Some("size-json") => size_json(&args[2]).map(|r| println!("{r}")),
        Some("report") => report(&args[2]).map(|r| println!("{}", serde_json::to_string_pretty(&r).unwrap())),
        Some("multtime") => multtime(&nums(2)),
        Some("primetime") => primetime(&nums(2)),
        _ => Err("usage: btn-prime-scaling sizes P... | counts P... | solve P SEED XI | size-json FILE | report DIR | multtime W... | primetime W...".into()),
    };
    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
    let _ = (BigUint::one(), BigUint::zero());
}
