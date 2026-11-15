//! Treewidth and contraction-cost census of ECDLP relation networks on toy
//! prime-order curves.
//!
//! For each bit size a prime-order short-Weierstrass curve is found (random
//! `p ≡ 3 mod 4`, baby-step giant-step point counting, both checked by the
//! crate's `Curve::new`), a target is planted as a sum of factor-base points,
//! and each (relation, multiplier) network is built by the crate's encoder.
//! The primal graph of the finished network is measured: clique and MMD+ lower
//! bounds, min-degree and min-fill elimination widths with their `log2` table
//! work. At small sizes every network is also checked against direct field
//! arithmetic on all positives and random negatives. One JSON row per network.

use hashed_ecdl_tensor::curve::{Curve, Point};
use hashed_ecdl_tensor::encoding::Multiplier;
use hashed_ecdl_tensor::reduction::{FactorBase, HashSeed};
use hashed_ecdl_tensor::relations::{self, Relation};
use hashed_ecdl_tensor::treewidth::{
    analyse, read_pace, verify_tree_decomposition, write_pace, PrimalGraph,
};
use hashed_ecdl_tensor::{BigUint, Limits, Meter};
use num_traits::{ToPrimitive, Zero};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Toy curve search in machine words (p < 2^62).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
struct ToyCurve {
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    generator: (u64, u64),
}

type Pt = Option<(u64, u64)>;

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
fn submod(a: u64, b: u64, p: u64) -> u64 {
    addmod(a, p - b % p, p)
}
fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}
fn invmod(a: u64, p: u64) -> u64 {
    powmod(a, p - 2, p)
}
fn isqrt(n: u64) -> u64 {
    let mut x = (n as f64).sqrt() as u64;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'witness: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    // p ≡ 3 (mod 4) only.
    let r = powmod(a, (p + 1) / 4, p);
    (mulmod(r, r, p) == a % p).then_some(r)
}

fn pt_neg(c: &ToyCurve, q: Pt) -> Pt {
    q.map(|(x, y)| (x, if y == 0 { 0 } else { c.p - y }))
}

fn pt_add(c: &ToyCurve, q: Pt, r: Pt) -> Pt {
    let p = c.p;
    match (q, r) {
        (None, _) => r,
        (_, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && addmod(y1, y2, p) == 0 {
                return None;
            }
            let lambda = if x1 == x2 {
                mulmod(
                    addmod(mulmod(3, mulmod(x1, x1, p), p), c.a, p),
                    invmod(addmod(y1, y1, p), p),
                    p,
                )
            } else {
                mulmod(submod(y2, y1, p), invmod(submod(x2, x1, p), p), p)
            };
            let x3 = submod(submod(mulmod(lambda, lambda, p), x1, p), x2, p);
            let y3 = submod(mulmod(lambda, submod(x1, x3, p), p), y1, p);
            Some((x3, y3))
        }
    }
}

fn pt_mul(c: &ToyCurve, mut k: u64, mut q: Pt) -> Pt {
    let mut acc = None;
    while k > 0 {
        if k & 1 == 1 {
            acc = pt_add(c, acc, q);
        }
        q = pt_add(c, q, q);
        k >>= 1;
    }
    acc
}

fn rhs(c: &ToyCurve, x: u64) -> u64 {
    let p = c.p;
    addmod(
        addmod(mulmod(mulmod(x, x, p), x, p), mulmod(c.a, x, p), p),
        c.b,
        p,
    )
}

fn lift(c: &ToyCurve, x: u64) -> Option<(u64, u64)> {
    sqrt_mod(rhs(c, x), c.p).map(|y| (x, y))
}

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Multiples of `point` in the Hasse interval that vanish, by baby-step
/// giant-step. Several candidates mean the point has small order.
fn vanishing_multiples(c: &ToyCurve, point: Pt) -> Vec<u64> {
    let p = c.p;
    let s = isqrt(p);
    let lo = (p + 1).saturating_sub(2 * s + 2).max(1);
    let hi = p + 1 + 2 * s + 2;
    let m = isqrt(4 * s + 5) + 1;
    let mut table: HashMap<u64, (u64, u64)> = HashMap::new();
    let mut baby = point;
    for j in 1..=m {
        if let Some((x, y)) = baby {
            table.entry(x).or_insert((j, y));
        }
        baby = pt_add(c, baby, point);
    }
    let step = pt_mul(c, m, point);
    let mut giant = pt_mul(c, lo, point);
    let mut found = Vec::new();
    let mut i = 0;
    while lo + i * m <= hi + m {
        let base = lo + i * m;
        match giant {
            None => found.push(base),
            Some((x, y)) => {
                if let Some(&(j, yj)) = table.get(&x) {
                    if y == yj {
                        found.push(base - j);
                    } else {
                        found.push(base + j);
                    }
                }
            }
        }
        giant = pt_add(c, giant, step);
        i += 1;
    }
    found.sort_unstable();
    found.dedup();
    found
        .into_iter()
        .filter(|&n| n >= lo && n <= hi && pt_mul(c, n, point).is_none())
        .collect()
}

fn find_curve(bits: u32, rng: &mut SplitMix) -> ToyCurve {
    assert!((7..=60).contains(&bits), "bits must be in 7..=60");
    loop {
        let p = (1u64 << (bits - 1)) | rng.below(1u64 << (bits - 1)) | 3;
        if p % 4 != 3 || !is_prime(p) {
            continue;
        }
        for _ in 0..4000 {
            let a = rng.below(p);
            let b = rng.below(p);
            let disc = addmod(
                mulmod(4, mulmod(mulmod(a, a, p), a, p), p),
                mulmod(27, mulmod(b, b, p), p),
                p,
            );
            if disc == 0 {
                continue;
            }
            let mut curve = ToyCurve {
                p,
                a,
                b,
                order: 0,
                generator: (0, 0),
            };
            let generator = loop {
                if let Some(point) = lift(&curve, rng.below(p)) {
                    break point;
                }
            };
            let candidates = vanishing_multiples(&curve, Some(generator));
            if candidates.len() != 1 {
                continue;
            }
            let order = candidates[0];
            if !is_prime(order) || order <= 4 * isqrt(p) + 4 {
                continue;
            }
            let other = loop {
                if let Some(point) = lift(&curve, rng.below(p)) {
                    break point;
                }
            };
            if pt_mul(&curve, order, Some(other)).is_some() {
                continue;
            }
            curve.order = order;
            curve.generator = generator;
            return curve;
        }
    }
}

fn big(n: u64) -> BigUint {
    BigUint::from(n)
}

fn to_point(q: Pt) -> Point {
    match q {
        None => Point::Infinity,
        Some((x, y)) => Point::Affine {
            x: big(x),
            y: big(y),
        },
    }
}

fn from_point(q: &Point) -> Pt {
    match q {
        Point::Infinity => None,
        Point::Affine { x, y } => Some((x.to_u64().unwrap(), y.to_u64().unwrap())),
    }
}

// ---------------------------------------------------------------------------
// Options.
// ---------------------------------------------------------------------------

struct Options {
    bits: Vec<u32>,
    out: Option<String>,
    relations: Vec<Relation>,
    multipliers: Vec<Multiplier>,
    minfill_max_bits: u32,
    minfill_max_arity: usize,
    restarts: usize,
    validate_max_bits: u32,
    seed: u64,
    export_gr: Option<String>,
}

const USAGE: &str = "tensor-treewidth --bits B[,B...] [--out rows.json] \
[--relations grouplaw-hash,grouplaw-smallx,semaev-s3-smallx,chained-s3-smallx] \
[--multipliers fourier,schoolbook] [--minfill-max-bits 24] [--minfill-max-arity 16] \
[--restarts 1] [--validate-max-bits 12] [--seed 20261111] [--export-gr DIR]\n\
tensor-treewidth --verify-td FILE.td --gr FILE.gr";

fn parse(args: &[String]) -> Result<Options, String> {
    let mut options = Options {
        bits: Vec::new(),
        out: None,
        relations: Relation::ALL.to_vec(),
        multipliers: vec![Multiplier::Fourier, Multiplier::Schoolbook],
        minfill_max_bits: 24,
        minfill_max_arity: 16,
        restarts: 1,
        validate_max_bits: 12,
        seed: 20_261_111,
        export_gr: None,
    };
    let mut i = 0;
    while i < args.len() {
        let value = args
            .get(i + 1)
            .ok_or_else(|| format!("{} needs a value", args[i]))?;
        match args[i].as_str() {
            "--bits" => {
                options.bits = value
                    .split(',')
                    .map(|b| b.parse().map_err(|_| format!("bad bits {b}")))
                    .collect::<Result<_, _>>()?;
            }
            "--out" => options.out = Some(value.clone()),
            "--relations" => {
                options.relations = value
                    .split(',')
                    .map(|r| Relation::parse(r).ok_or_else(|| format!("unknown relation {r}")))
                    .collect::<Result<_, _>>()?;
            }
            "--multipliers" => {
                options.multipliers = value
                    .split(',')
                    .map(|m| match m {
                        "fourier" => Ok(Multiplier::Fourier),
                        "schoolbook" => Ok(Multiplier::Schoolbook),
                        other => Err(format!("unknown multiplier {other}")),
                    })
                    .collect::<Result<_, _>>()?;
            }
            "--minfill-max-bits" => {
                options.minfill_max_bits = value.parse().map_err(|_| "bad bits".to_string())?;
            }
            "--minfill-max-arity" => {
                options.minfill_max_arity = value.parse().map_err(|_| "bad arity".to_string())?;
            }
            "--restarts" => {
                options.restarts = value.parse().map_err(|_| "bad restarts".to_string())?;
            }
            "--export-gr" => options.export_gr = Some(value.clone()),
            "--validate-max-bits" => {
                options.validate_max_bits = value.parse().map_err(|_| "bad bits".to_string())?;
            }
            "--seed" => options.seed = value.parse().map_err(|_| "bad seed".to_string())?,
            other => return Err(format!("unknown option {other}\n{USAGE}")),
        }
        i += 2;
    }
    if options.bits.is_empty() {
        return Err(USAGE.to_string());
    }
    Ok(options)
}

fn multiplier_name(multiplier: Multiplier) -> &'static str {
    match multiplier {
        Multiplier::Fourier => "fourier",
        Multiplier::Schoolbook => "schoolbook",
    }
}

fn meter() -> Meter {
    Meter::new(Limits {
        steps: u64::MAX / 4,
        vector: 1 << 34,
        bits: 1 << 30,
    })
}

// ---------------------------------------------------------------------------
// Targets, hash bases and validation.
// ---------------------------------------------------------------------------

fn random_small_x_point(c: &ToyCurve, low_bits: usize, rng: &mut SplitMix) -> (u64, u64) {
    loop {
        let x = rng.below(1u64 << low_bits);
        if let Some(point) = lift(c, x) {
            return if rng.next() & 1 == 1 {
                point
            } else {
                (point.0, c.p - point.1)
            };
        }
    }
}

fn hash_bases(curve: &Curve, c: &ToyCurve, meter: &mut Meter) -> (Vec<FactorBase>, Value) {
    let mut s = 2 * c.p + 1;
    while !is_prime(s) {
        s += 2;
    }
    assert!(s <= 4 * c.p);
    let root = isqrt(c.order) + 1;
    let tau = ((s as u128 * root as u128).div_ceil(c.order as u128) as u64).clamp(1, s);
    let seeds = [(3u64, 1u64), (5, 2)];
    let bases = seeds
        .iter()
        .map(|&(alpha, beta)| {
            FactorBase::new(
                curve,
                big(s),
                big(tau),
                HashSeed {
                    alpha: big(alpha),
                    beta: big(beta),
                },
                meter,
            )
            .expect("factor base")
        })
        .collect();
    (
        bases,
        json!({"s": s, "tau": tau, "seeds": seeds.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>()}),
    )
}

fn plant_target(
    relation: Relation,
    curve: &Curve,
    c: &ToyCurve,
    bases: &[FactorBase],
    low_bits: usize,
    rng: &mut SplitMix,
    validate: bool,
    meter: &mut Meter,
) -> Pt {
    loop {
        let mut sum: Pt = None;
        for slot in 0..relation.slots() {
            let point = if relation.uses_hash() {
                if !validate {
                    // Any subgroup point serves as a target for the graph.
                    let k = 1 + rng.below(c.order - 1);
                    return pt_mul(c, k, Some(c.generator));
                }
                loop {
                    if let Some(point) = lift(c, rng.below(c.p)) {
                        let point = if rng.next() & 1 == 1 {
                            point
                        } else {
                            (point.0, c.p - point.1)
                        };
                        if bases[slot]
                            .contains(curve, &to_point(Some(point)), meter)
                            .unwrap()
                        {
                            break point;
                        }
                    }
                }
            } else {
                random_small_x_point(c, low_bits, rng)
            };
            sum = pt_add(c, sum, Some(point));
        }
        if sum.is_some() {
            return sum;
        }
    }
}

struct Validation {
    evaluated: usize,
    positives: usize,
    accepted: usize,
    mismatches: usize,
}

fn check(
    network: &hashed_ecdl_tensor::network::Network,
    bits: &[bool],
    expected: bool,
    v: &mut Validation,
    meter: &mut Meter,
) {
    let accepted = relations::accepts(network, bits, meter).expect("evaluation");
    v.evaluated += 1;
    v.positives += usize::from(expected);
    v.accepted += usize::from(accepted);
    v.mismatches += usize::from(accepted != expected);
}

fn validate(
    relation: Relation,
    network: &hashed_ecdl_tensor::network::Network,
    curve: &Curve,
    c: &ToyCurve,
    bases: &[FactorBase],
    target: Pt,
    low_bits: usize,
    rng: &mut SplitMix,
    meter: &mut Meter,
) -> Validation {
    let width = c.p.next_power_of_two().trailing_zeros() as usize;
    let width = if (1u64 << width) == c.p {
        width + 1
    } else {
        width
    };
    debug_assert_eq!(width, curve.modulus().bits() as usize);
    let mut v = Validation {
        evaluated: 0,
        positives: 0,
        accepted: 0,
        mismatches: 0,
    };
    let bound = 1u64 << low_bits;
    let target_point = to_point(target);
    let target_x = from_point(&target_point).unwrap().0;
    let is_small = |q: Pt| matches!(q, Some((x, _)) if x < bound);
    match relation {
        Relation::GroupLawSmallX => {
            for x in 0..bound {
                let Some((_, y)) = lift(c, x) else { continue };
                for y in [y, c.p - y] {
                    let other = pt_add(c, target, pt_neg(c, Some((x, y))));
                    let mut bits = relations::bits_of(&big(x), low_bits);
                    bits.extend(relations::bits_of(&big(y), width));
                    check(network, &bits, is_small(other), &mut v, meter);
                }
            }
            for _ in 0..64 {
                let x = rng.below(bound);
                let y = rng.below(c.p);
                if mulmod(y, y, c.p) == rhs(c, x) {
                    continue;
                }
                let mut bits = relations::bits_of(&big(x), low_bits);
                bits.extend(relations::bits_of(&big(y), width));
                check(network, &bits, false, &mut v, meter);
            }
        }
        Relation::SemaevS3SmallX => {
            for x1 in 0..bound {
                for x2 in 0..bound {
                    let expected =
                        relations::semaev_s3(curve, &big(x1), &big(x2), &big(target_x)).is_zero();
                    let mut bits = relations::bits_of(&big(x1), low_bits);
                    bits.extend(relations::bits_of(&big(x2), low_bits));
                    check(network, &bits, expected, &mut v, meter);
                }
            }
        }
        Relation::ChainedS3SmallX => {
            let holds = |x1: u64, x2: u64, x3: u64, t: u64| {
                relations::semaev_s3(curve, &big(x1), &big(x2), &big(t)).is_zero()
                    && relations::semaev_s3(curve, &big(t), &big(x3), &big(target_x)).is_zero()
            };
            let feed = |x1: u64, x2: u64, x3: u64, t: u64| {
                let mut bits = relations::bits_of(&big(x1), low_bits);
                bits.extend(relations::bits_of(&big(x2), low_bits));
                bits.extend(relations::bits_of(&big(x3), low_bits));
                bits.extend(relations::bits_of(&big(t), width));
                bits
            };
            for x1 in 0..bound {
                for x2 in 0..bound {
                    for t in 0..c.p {
                        if !relations::semaev_s3(curve, &big(x1), &big(x2), &big(t)).is_zero() {
                            continue;
                        }
                        for x3 in 0..bound {
                            let expected =
                                relations::semaev_s3(curve, &big(t), &big(x3), &big(target_x))
                                    .is_zero();
                            check(network, &feed(x1, x2, x3, t), expected, &mut v, meter);
                        }
                    }
                }
            }
            for _ in 0..200 {
                let (x1, x2, x3, t) = (
                    rng.below(bound),
                    rng.below(bound),
                    rng.below(bound),
                    rng.below(c.p),
                );
                check(
                    network,
                    &feed(x1, x2, x3, t),
                    holds(x1, x2, x3, t),
                    &mut v,
                    meter,
                );
            }
        }
        Relation::GroupLawHash => {
            for x in 0..c.p {
                let Some((_, y)) = lift(c, x) else { continue };
                for y in [y, c.p - y] {
                    let first = to_point(Some((x, y)));
                    let other = to_point(pt_add(c, target, pt_neg(c, Some((x, y)))));
                    let expected = bases[0].contains(curve, &first, meter).unwrap()
                        && bases[1].contains(curve, &other, meter).unwrap();
                    let mut bits = relations::bits_of(&big(x), width);
                    bits.extend(relations::bits_of(&big(y), width));
                    check(network, &bits, expected, &mut v, meter);
                }
            }
        }
    }
    v
}

// ---------------------------------------------------------------------------
// Main.
// ---------------------------------------------------------------------------

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help") {
        println!("{USAGE}");
        return Ok(());
    }
    if let Some(position) = args.iter().position(|a| a == "--verify-td") {
        let td_path = args.get(position + 1).ok_or("--verify-td needs a file")?;
        let gr_position = args
            .iter()
            .position(|a| a == "--gr")
            .ok_or("--verify-td needs --gr")?;
        let gr_path = args.get(gr_position + 1).ok_or("--gr needs a file")?;
        let graph = read_pace(&std::fs::read_to_string(gr_path).map_err(|e| e.to_string())?)?;
        let td = std::fs::read_to_string(td_path).map_err(|e| e.to_string())?;
        let verdict = verify_tree_decomposition(&graph, &td);
        println!(
            "{}",
            json!({
                "gr": gr_path, "td": td_path,
                "variables": graph.variable_count(), "primal_edges": graph.edge_count(),
                "width": verdict.as_ref().ok(), "valid": verdict.is_ok(),
                "error": verdict.as_ref().err(),
            })
        );
        return verdict
            .map(|_| ())
            .map_err(|e| format!("invalid tree decomposition: {e}"));
    }
    let options = parse(&args)?;
    if let Some(dir) = &options.export_gr {
        std::fs::create_dir_all(dir).map_err(|e| format!("create {dir}: {e}"))?;
    }
    let mut rows = Vec::new();
    for &bits in &options.bits {
        let mut rng = SplitMix(options.seed ^ (u64::from(bits) << 32));
        let toy = find_curve(bits, &mut rng);
        let mut meter = meter();
        let curve = Curve::new(
            big(toy.p),
            big(toy.a),
            big(toy.b),
            big(toy.order),
            to_point(Some(toy.generator)),
            &mut meter,
        )
        .map_err(|e| format!("curve certification: {e}"))?;
        let width = curve.modulus().bits() as usize;
        assert_eq!(width as u32, bits);
        let (bases, hash_json) = hash_bases(&curve, &toy, &mut meter);
        let validating = bits <= options.validate_max_bits;
        for &relation in &options.relations {
            let low_bits = relation.low_bits(width);
            let target = plant_target(
                relation, &curve, &toy, &bases, low_bits, &mut rng, validating, &mut meter,
            );
            let target_point = to_point(target);
            for &multiplier in &options.multipliers {
                let started = Instant::now();
                let network = relations::build(
                    relation,
                    &curve,
                    &bases,
                    &target_point,
                    multiplier,
                    &mut meter,
                )
                .map_err(|e| format!("{} {}: {e}", relation.name(), multiplier_name(multiplier)))?;
                let build_seconds = started.elapsed().as_secs_f64();
                let edges = network.edge_count();
                let started = Instant::now();
                let graph = PrimalGraph::from_network(&network);
                let primal_seconds = started.elapsed().as_secs_f64();
                let started = Instant::now();
                let arity_ok = graph.maximum_arity() <= options.minfill_max_arity;
                let analysis = analyse(
                    &graph,
                    options.restarts,
                    arity_ok && bits <= options.minfill_max_bits,
                    arity_ok,
                );
                let analysis_seconds = started.elapsed().as_secs_f64();
                let upper_bound = [
                    Some(analysis.min_degree.width),
                    analysis.min_fill.as_ref().map(|r| r.width),
                    analysis.min_fill_lazy.as_ref().map(|r| r.width),
                ]
                .into_iter()
                .flatten()
                .min();
                let gr_file = options.export_gr.as_ref().map(|dir| {
                    let name = format!(
                        "{}/{}-{}-b{}.gr",
                        dir,
                        relation.name(),
                        multiplier_name(multiplier),
                        bits
                    );
                    let mut out =
                        std::io::BufWriter::new(std::fs::File::create(&name).expect("gr file"));
                    write_pace(&graph, &mut out).expect("gr write");
                    name
                });
                let validation = validating.then(|| {
                    let started = Instant::now();
                    let v = validate(
                        relation, &network, &curve, &toy, &bases, target, low_bits, &mut rng,
                        &mut meter,
                    );
                    json!({
                        "evaluated": v.evaluated,
                        "positives": v.positives,
                        "accepted": v.accepted,
                        "mismatches": v.mismatches,
                        "passed": v.mismatches == 0 && v.positives >= 1,
                        "seconds": started.elapsed().as_secs_f64(),
                    })
                });
                let log2_s = (edges as f64).log2();
                let row = json!({
                    "bits": bits,
                    "relation": relation.name(),
                    "multiplier": multiplier_name(multiplier),
                    "slots": relation.slots(),
                    "low_bits": low_bits,
                    "curve": {
                        "p": toy.p, "a": toy.a, "b": toy.b, "order": toy.order,
                        "generator": [toy.generator.0, toy.generator.1],
                        "target": target.map(|(x, y)| json!([x, y])).unwrap_or(Value::Null),
                    },
                    "hash": if relation.uses_hash() { hash_json.clone() } else { Value::Null },
                    "edges": edges,
                    "vertices": network.vertex_count(),
                    "max_degree": network.maximum_degree(),
                    "max_arity": graph.maximum_arity(),
                    "free_bits": network.input_bit_count(),
                    "interface_bits": network.interface().len(),
                    "primal_edges": graph.edge_count(),
                    "lower_bound_clique": graph.clique_lower_bound(),
                    "lower_bound": analysis.lower_bound,
                    "core_variables": analysis.core_variables,
                    "core_edges": analysis.core_edges,
                    "reduction_width": analysis.reduction_width,
                    "core_mmd_plus": analysis.core_mmd_plus,
                    "exact": analysis.exact,
                    "min_degree": {
                        "width": analysis.min_degree.width,
                        "log2_cost": analysis.min_degree.log2_cost,
                    },
                    "min_fill": analysis.min_fill.as_ref().map(|run| json!({
                        "width": run.width,
                        "log2_cost": run.log2_cost,
                    })).unwrap_or(Value::Null),
                    "min_fill_lazy": analysis.min_fill_lazy.as_ref().map(|run| json!({
                        "width": run.width,
                        "log2_cost": run.log2_cost,
                    })).unwrap_or(Value::Null),
                    "upper_bound": upper_bound,
                    "restarts": options.restarts,
                    "gr_file": gr_file,
                    "d1_exponent_c1": edges as f64 / log2_s.sqrt(),
                    "brute_force_exponent": edges,
                    "enumeration_exponent": (relation.slots() - 1) as f64 * width as f64 / relation.slots() as f64,
                    "rho_exponent": width as f64 / 2.0,
                    "validation": validation.unwrap_or(Value::Null),
                    "seconds": {
                        "build": build_seconds,
                        "primal": primal_seconds,
                        "analysis": analysis_seconds,
                    },
                });
                println!("{row}");
                rows.push(row);
            }
        }
    }
    if let Some(out) = &options.out {
        std::fs::write(out, serde_json::to_string_pretty(&rows).unwrap() + "\n")
            .map_err(|e| format!("write {out}: {e}"))?;
    }
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("tensor-treewidth: {message}");
        std::process::exit(1);
    }
}
