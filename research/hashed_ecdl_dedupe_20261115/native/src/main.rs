//! Edges, exact counts and charged steps of `hashed-ecdl-tensor` decomposition
//! networks and end-to-end solves on small prime-order curves. The same source
//! is built against two crate revisions (before/after a change); every count is
//! checked against brute force, every solve against `reduction::verify`.
//!
//!   dedupe-measure counts M P...        m-slot exact counts vs brute force
//!   dedupe-measure solve  P SEED XI     reduction::solve + verify, charged steps
//!   dedupe-measure sizes  P...          2- and 3-slot network sizes (no count)
//!   dedupe-measure size-json FILE       network sizes for a JSON problem (ECCp-131)
//!   dedupe-measure report DIR           derived values from the recorded runs in DIR
//!
//! One JSON object per line. Step counts are the crate's Meter charges:
//! deterministic diagnostics, not calibrated time units.

use hashed_ecdl_tensor::arithmetic::{is_prime, Seeded};
use hashed_ecdl_tensor::contraction::Counter;
use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::reduction::{self, Config, FactorBase, HashSeed};
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
        vector: 50_000_000,
        bits: 1_000_000,
    })
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
    if r == 1 {
        1
    } else {
        -1
    }
}

/// Group order of y^2 = x^3 + ax + b over F_p by summing Legendre symbols.
fn order(p: u64, a: u64, b: u64) -> u64 {
    let mut n = 1u64; // identity
    for x in 0..p {
        let rhs =
            ((x as u128 * x as u128 % p as u128 * x as u128 + a as u128 * x as u128 + b as u128)
                % p as u128) as u64;
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
    Point::Affine {
        x: x.into(),
        y: y.into(),
    }
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

const SEEDS: [(u64, u64); 3] = [(1, 0), (3, 7), (5, 2)];

fn bases(
    c: &Curve,
    s: &BigUint,
    threshold: &BigUint,
    slots: usize,
    m: &mut Meter,
) -> Res<Vec<FactorBase>> {
    SEEDS[..slots]
        .iter()
        .map(|&(al, be)| {
            FactorBase::new(
                c,
                s.clone(),
                threshold.clone(),
                HashSeed {
                    alpha: al.into(),
                    beta: be.into(),
                },
                m,
            )
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

/// Brute-force count of (A_1..A_m) in the bases, all affine, summing to target.
fn brute(c: &Curve, fb: &[FactorBase], pts: &[Point], target: &Point, m: &mut Meter) -> Res<u64> {
    let members: Vec<Vec<Point>> = fb
        .iter()
        .map(|b| {
            pts.iter()
                .filter_map(|q| match b.contains(c, q, m) {
                    Ok(true) => Some(Ok(q.clone())),
                    Ok(false) => None,
                    Err(err) => Some(Err(e(err))),
                })
                .collect::<Res<Vec<_>>>()
        })
        .collect::<Res<_>>()?;
    let mut partial = vec![(Point::Infinity, 0usize)];
    let mut total = 0u64;
    while let Some((sum, i)) = partial.pop() {
        if i == members.len() {
            total += u64::from(&sum == target);
            continue;
        }
        for q in &members[i] {
            partial.push((c.add(&sum, q, m).map_err(e)?, i + 1));
        }
    }
    Ok(total)
}

fn counts(slots: usize, p: u64) -> Res<Vec<Value>> {
    let mut m = meter(u64::MAX / 2);
    let (c, a, b, r) = curve_for(p, &mut m)?;
    let s = hash_prime(p, &mut m)?;
    let pts = affine_points(&c, p, a, b);
    let mut out = Vec::new();
    for threshold in [s.clone(), &s >> 1u8, &s >> 2u8] {
        let fb = bases(&c, &s, &threshold, slots, &mut m)?;
        for target in pts.iter().take(2) {
            let mut bm = meter(u64::MAX / 2);
            let net = reduction::decomposition_network(&c, &fb, target, &mut bm).map_err(e)?;
            let build_steps = bm.used;
            let brute = brute(&c, &fb, &pts, target, &mut m)?;
            let mut cm = meter(200_000_000_000);
            let mut counter = Counter::default();
            let got = counter.count(&net, &vec![None; net.edge_count()], &mut cm);
            let mut rec = size_record(&net);
            let (count, ok) = match &got {
                Ok(v) => (json!(v.to_string()), v.to_u64() == Some(brute)),
                Err(err) => (json!(format!("error: {err}")), false),
            };
            rec["slots"] = json!(slots);
            rec["p"] = json!(p);
            rec["d"] = json!(64 - p.leading_zeros());
            rec["r"] = json!(r);
            rec["threshold"] = json!(threshold.to_string());
            rec["hash_prime"] = json!(s.to_string());
            rec["target"] = json!(format!("{target:?}"));
            rec["brute"] = json!(brute);
            rec["count"] = count;
            rec["ok"] = json!(ok);
            rec["build_steps"] = json!(build_steps);
            rec["steps"] = json!(cm.used);
            rec["backend"] = json!(format!("{:?}", counter.statistics.last_backend));
            rec["functional_assignments"] = json!(counter.statistics.functional_assignments);
            out.push(rec);
        }
    }
    Ok(out)
}

fn solve(p: u64, seed: u64, xi: u64) -> Res<Value> {
    let mut m = meter(2_000_000_000_000);
    let (c, _, _, r) = curve_for(p, &mut m)?;
    let xi = xi % r;
    let target = mul(&c, xi, c.generator(), &mut m)?;
    let before = m.used;
    let mut counter = Counter::default();
    let cert = reduction::solve(
        &c,
        &target,
        &Config::for_curve(&c),
        &mut Seeded(seed),
        &mut counter,
        &mut m,
    );
    let solve_steps = m.used - before;
    let mut rec = json!({ "p": p, "d": 64 - p.leading_zeros(), "r": r, "seed": seed, "secret": xi,
        "slots": Config::for_curve(&c).slots, "steps": solve_steps,
        "contractions": counter.statistics.contractions,
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

fn sizes(p: u64) -> Res<Vec<Value>> {
    let mut m = meter(u64::MAX / 2);
    let (c, _, _, r) = curve_for(p, &mut m)?;
    let s = hash_prime(p, &mut m)?;
    let mut out = Vec::new();
    for slots in [2usize, 3] {
        let fb = bases(&c, &s, &s, slots, &mut m)?;
        let mut bm = meter(u64::MAX / 2);
        let net = reduction::decomposition_network(&c, &fb, c.generator(), &mut bm).map_err(e)?;
        let mut rec = size_record(&net);
        rec["slots"] = json!(slots);
        rec["p"] = json!(p);
        rec["d"] = json!(64 - p.leading_zeros());
        rec["r"] = json!(r);
        rec["build_steps"] = json!(bm.used);
        out.push(rec);
    }
    Ok(out)
}

fn size_json(path: &str) -> Res<Value> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).map_err(e)?).map_err(e)?;
    let big = |k: &str| -> Res<BigUint> {
        v[k].as_str()
            .ok_or(format!("missing {k}"))?
            .parse::<BigUint>()
            .map_err(e)
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
    let proofs: Vec<hashed_ecdl_tensor::primality::PrimeCertificate> =
        match v.get("prime_certificates") {
            Some(c) => serde_json::from_value(c.clone()).map_err(e)?,
            None => Vec::new(),
        };
    let cofactor = v
        .get("cofactor")
        .and_then(Value::as_str)
        .unwrap_or("1")
        .parse::<BigUint>()
        .map_err(e)?;
    let c = Curve::with_cofactor_and_certificates(
        p.clone(),
        big("a")?,
        big("b")?,
        big("r")?,
        cofactor,
        pp("generator")?,
        &proofs,
        &mut m,
    )
    .map_err(e)?;
    let s = v
        .get("hash_prime")
        .and_then(Value::as_str)
        .ok_or("hash_prime required")?
        .parse::<BigUint>()
        .map_err(e)?;
    let mut nets = Vec::new();
    for slots in [2usize, 3] {
        let fb: Vec<FactorBase> = (0..slots)
            .map(|i| {
                FactorBase::new(
                    &c,
                    s.clone(),
                    s.clone(),
                    HashSeed {
                        alpha: (2 * i as u64 + 1).into(),
                        beta: (i as u64).into(),
                    },
                    &mut m,
                )
                .map_err(e)
            })
            .collect::<Res<_>>()?;
        let mut bm = meter(u64::MAX / 2);
        let net = reduction::decomposition_network(&c, &fb, &pp("target")?, &mut bm).map_err(e)?;
        let mut rec = size_record(&net);
        rec["slots"] = json!(slots);
        rec["build_steps"] = json!(bm.used);
        nets.push(rec);
    }
    Ok(json!({ "file": path, "d": p.bits(), "hash_prime": s.to_string(), "networks": nets }))
}

fn read_jsonl(path: &str) -> Res<Vec<Value>> {
    std::fs::read_to_string(path)
        .map_err(|err| format!("{path}: {err}"))?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(e))
        .collect()
}

fn spread(mut v: Vec<f64>) -> Value {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n == 0 {
        return Value::Null;
    }
    let median = if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    };
    json!({ "n": n, "min": v[0], "median": median, "max": v[n - 1] })
}

fn num(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or(f64::NAN)
}

/// Derived values from the recorded runs in DIR: every number is read from the
/// raw records, nothing is re-measured.
fn report(dir: &str) -> Res<Value> {
    let variants = [
        "before",
        "skip_only",
        "memo_only",
        "dedupe",
        "fold",
        "valid",
        "chains",
        "implied",
        "auto",
        "zeroadd",
        "compressed",
        "verify",
        "hashfirst",
        "final",
    ];
    let mut counts = serde_json::Map::new();
    for slots in [2, 3] {
        let base = read_jsonl(&format!("{dir}/counts{slots}_before.jsonl"))?;
        let mut per = serde_json::Map::new();
        for v in variants {
            let rows = read_jsonl(&format!("{dir}/counts{slots}_{v}.jsonl"))?;
            if rows.len() != base.len() {
                return Err(format!(
                    "counts{slots}_{v}: {} rows, before has {}",
                    rows.len(),
                    base.len()
                ));
            }
            let same_instances = rows.iter().zip(&base).all(|(r, b)| {
                ["p", "threshold", "target", "brute"]
                    .iter()
                    .all(|k| r[*k] == b[*k])
            });
            per.insert(v.into(), json!({
                "rows": rows.len(),
                "same_instances_as_before": same_instances,
                "all_match_brute_force": rows.iter().all(|r| r["ok"] == json!(true)),
                "same_counts_as_before": rows.iter().zip(&base).all(|(r, b)| r["count"] == b["count"]),
                "same_functional_assignments_as_before": rows.iter().zip(&base).all(|(r, b)| r["functional_assignments"] == b["functional_assignments"]),
                "step_ratio_before_over_variant": spread(rows.iter().zip(&base).map(|(r, b)| num(b, "steps") / num(r, "steps")).collect()),
                "edge_fraction_variant_over_before": spread(rows.iter().zip(&base).map(|(r, b)| num(r, "edges") / num(b, "edges")).collect()),
                "table": rows.iter().zip(&base).map(|(r, b)| json!({
                    "p": r["p"], "threshold": r["threshold"], "brute": r["brute"], "count": r["count"],
                    "edges_before": b["edges"], "edges": r["edges"],
                    "steps_before": b["steps"], "steps": r["steps"],
                    "ratio": num(b, "steps") / num(r, "steps"),
                })).collect::<Vec<_>>(),
            }));
        }
        counts.insert(format!("slots_{slots}"), Value::Object(per));
    }
    let stages = ["dedupe", "chains", "zeroadd", "final"];
    let sb = read_jsonl(&format!("{dir}/solves_before.jsonl"))?;
    let mut solves = serde_json::Map::new();
    for stage in stages {
        let sa = read_jsonl(&format!("{dir}/solves_{stage}.jsonl"))?;
        solves.insert(stage.into(), json!({
            "n": sa.len(),
            "all_verified_before_and_after": sb.iter().chain(&sa).all(|r| r["ok"] == json!(true)),
            "same_scalar_and_relations": sb.len() == sa.len() && sb.iter().zip(&sa).all(|(b, a)| b["recovered"] == a["recovered"] && b["relations"] == a["relations"]),
            "step_ratio": spread(sb.iter().zip(&sa).map(|(b, a)| num(b, "steps") / num(a, "steps")).collect()),
            "rows": sb.iter().zip(&sa).map(|(b, a)| json!({ "p": a["p"], "seed": a["seed"], "secret": a["secret"],
                "recovered": a["recovered"], "relations": a["relations"], "steps_before": b["steps"], "steps": a["steps"],
                "ratio": num(b, "steps") / num(a, "steps") })).collect::<Vec<_>>(),
        }));
    }
    let zb = read_jsonl(&format!("{dir}/sizes_before.jsonl"))?;
    let mut sizes = serde_json::Map::new();
    let eb: Value = serde_json::from_str(
        &std::fs::read_to_string(format!("{dir}/eccp131_before.json")).map_err(e)?,
    )
    .map_err(e)?;
    let mut eccp = serde_json::Map::new();
    for stage in stages {
        let za = read_jsonl(&format!("{dir}/sizes_{stage}.jsonl"))?;
        sizes.insert(stage.into(), json!(zb.iter().zip(&za).map(|(b, a)| json!({ "p": a["p"], "d": a["d"], "slots": a["slots"],
            "edges_before": b["edges"], "edges": a["edges"], "fraction": num(a, "edges") / num(b, "edges"),
            "vertices_before": b["vertices"], "vertices": a["vertices"],
            "max_degree_before": b["max_degree"], "max_degree": a["max_degree"] })).collect::<Vec<_>>()));
        let ea: Value = serde_json::from_str(
            &std::fs::read_to_string(format!("{dir}/eccp131_{stage}.json")).map_err(e)?,
        )
        .map_err(e)?;
        eccp.insert(stage.into(), json!(eb["networks"].as_array().ok_or("eccp131 networks")?.iter()
            .zip(ea["networks"].as_array().ok_or("eccp131 networks")?)
            .map(|(b, a)| json!({ "slots": a["slots"], "edges_before": b["edges"], "edges": a["edges"],
                "vertices_before": b["vertices"], "vertices": a["vertices"],
                "fraction": num(a, "edges") / num(b, "edges"),
                "log2_edges_before": num(b, "edges").log2(), "log2_edges": num(a, "edges").log2() }))
            .collect::<Vec<_>>()));
    }
    Ok(json!({ "counts": counts, "solves": solves, "sizes": sizes, "eccp131": eccp }))
}

/// Network size of one integer product of two w-bit words (both free, or the
/// second a constant with alternating bits) with the Fourier and schoolbook
/// encoders. Build only; no count.
fn mult(widths: &[u64]) -> Res<Vec<Value>> {
    use hashed_ecdl_tensor::encoding::Encoder;
    let m = meter(u64::MAX / 2);
    let mut out = Vec::new();
    for &w in widths {
        let w = w as usize;
        let constant = (0..w).fold(BigUint::from(0u8), |acc, i| {
            if i % 2 == 0 {
                acc | (BigUint::from(1u8) << i)
            } else {
                acc
            }
        });
        for (operand, method) in [
            ("free", "fourier"),
            ("free", "schoolbook"),
            ("constant", "fourier"),
            ("constant", "schoolbook"),
        ] {
            let mut enc = Encoder::new();
            let a = enc.input_word(w, &m).map_err(e)?;
            let b = if operand == "free" {
                enc.input_word(w, &m).map_err(e)?
            } else {
                Encoder::constant(w, &constant).map_err(e)?
            };
            let product = if method == "fourier" {
                enc.integer_multiply(&a, &b, &m)
            } else {
                enc.schoolbook_multiply(&a, &b, &m)
            }
            .map_err(e)?;
            enc.mark_interface(&product, &m).map_err(e)?;
            let net = enc.finish(&m).map_err(e)?;
            let mut rec = size_record(&net);
            rec["width"] = json!(w);
            rec["operand"] = json!(operand);
            rec["method"] = json!(method);
            out.push(rec);
        }
    }
    Ok(out)
}

/// Network size of one modular product x*c mod q (c constant) and one modular
/// sum x+y mod q at the width of q. Build only; no count.
fn modops(primes: &[u64]) -> Res<Vec<Value>> {
    use hashed_ecdl_tensor::encoding::Encoder;
    let m = meter(u64::MAX / 2);
    let mut out = Vec::new();
    for &q in primes {
        let q = BigUint::from(q);
        let w = q.bits() as usize;
        for op in ["multiply_by_constant", "multiply", "add"] {
            let mut enc = Encoder::new();
            let x = enc.input_word(w, &m).map_err(e)?;
            let y = enc.input_word(w, &m).map_err(e)?;
            let c = Encoder::constant(w, &(&q - 2u8)).map_err(e)?;
            let r = match op {
                "multiply_by_constant" => enc.modular_multiply(&x, &c, &q, &m),
                "multiply" => enc.modular_multiply(&x, &y, &q, &m),
                _ => enc.modular_add(&x, &y, &q, &m),
            }
            .map_err(e)?;
            enc.mark_interface(&r, &m).map_err(e)?;
            let net = enc.finish(&m).map_err(e)?;
            let mut rec = size_record(&net);
            rec["q_bits"] = json!(w);
            rec["op"] = json!(op);
            out.push(rec);
        }
    }
    Ok(out)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nums = |from: usize| -> Vec<u64> {
        args[from..]
            .iter()
            .map(|a| a.parse().expect("number"))
            .collect()
    };
    let print = |v: Value| println!("{v}");
    let run = || -> Res<()> {
        match args.get(1).map(String::as_str) {
            Some("counts") => {
                let slots = args[2].parse::<usize>().map_err(e)?;
                for p in nums(3) {
                    counts(slots, p)?.into_iter().for_each(print);
                }
            }
            Some("solve") => {
                let n = nums(2);
                print(solve(n[0], n[1], n[2])?);
            }
            Some("sizes") => nums(2)
                .into_iter()
                .try_for_each(|p| sizes(p).map(|v| v.into_iter().for_each(print)))?,
            Some("size-json") => print(size_json(&args[2])?),
            Some("modops") => modops(&nums(2))?.into_iter().for_each(print),
            Some("mult") => mult(&nums(2))?.into_iter().for_each(print),
            Some("report") => println!(
                "{}",
                serde_json::to_string_pretty(&report(&args[2])?).map_err(e)?
            ),
            _ => return Err(
                "usage: counts M P... | solve P SEED XI | sizes P... | size-json FILE | report DIR"
                    .into(),
            ),
        }
        Ok(())
    };
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
