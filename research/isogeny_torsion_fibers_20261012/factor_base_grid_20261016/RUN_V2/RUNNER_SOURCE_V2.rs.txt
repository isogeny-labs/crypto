//! Frozen factor-base scaling on the explicit different-j degree-three pair.
//! Base construction and exhaustive chi pair checks precede the solve panel.
#![recursion_limit = "256"]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::time::Instant;

use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ext_curve::{
    take_field_counters, ExtCurve, ExtField, ExtPoint, Fp2,
};
use crypto_lib::cryptanalysis::ic_boundary::rho_reference_negation;
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

type El = [u64; 2];
type Point = ExtPoint<El>;
type Curve = ExtCurve<Fp2>;
type Poly = Vec<El>;

fn trim(f: &Fp2, p: &mut Poly) {
    while p.last().is_some_and(|&c| c == f.zero()) {
        p.pop();
    }
}

fn add(f: &Fp2, a: &Poly, b: &Poly) -> Poly {
    let mut out = vec![f.zero(); a.len().max(b.len())];
    for (i, &v) in a.iter().enumerate() {
        out[i] = f.add(out[i], v);
    }
    for (i, &v) in b.iter().enumerate() {
        out[i] = f.add(out[i], v);
    }
    trim(f, &mut out);
    out
}

fn sub(f: &Fp2, a: &Poly, b: &Poly) -> Poly {
    let mut out = vec![f.zero(); a.len().max(b.len())];
    for (i, &v) in a.iter().enumerate() {
        out[i] = f.add(out[i], v);
    }
    for (i, &v) in b.iter().enumerate() {
        out[i] = f.sub(out[i], v);
    }
    trim(f, &mut out);
    out
}

fn scale(f: &Fp2, a: &Poly, k: El) -> Poly {
    let mut out: Poly = a.iter().map(|&v| f.mul(v, k)).collect();
    trim(f, &mut out);
    out
}

fn mul(f: &Fp2, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![f.zero(); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = f.add(out[i + j], f.mul(x, y));
        }
    }
    trim(f, &mut out);
    out
}

fn div_rem(f: &Fp2, a: &Poly, b: &Poly) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut rem = a.clone();
    let mut quo = vec![f.zero(); a.len().saturating_sub(b.len()) + 1];
    let inv_lead = f.inv(*b.last().unwrap()).unwrap();
    while rem.len() >= b.len() {
        let shift = rem.len() - b.len();
        let coeff = f.mul(*rem.last().unwrap(), inv_lead);
        quo[shift] = coeff;
        for (j, &term) in b.iter().enumerate() {
            rem[shift + j] = f.sub(rem[shift + j], f.mul(coeff, term));
        }
        trim(f, &mut rem);
    }
    trim(f, &mut quo);
    (quo, rem)
}

fn gcd(f: &Fp2, mut a: Poly, mut b: Poly) -> Poly {
    while !b.is_empty() {
        let rem = div_rem(f, &a, &b).1;
        a = b;
        b = rem;
    }
    if let Some(&lead) = a.last() {
        let inverse = f.inv(lead).unwrap();
        for coeff in &mut a {
            *coeff = f.mul(*coeff, inverse);
        }
    }
    a
}

fn mul_mod(f: &Fp2, a: &Poly, b: &Poly, modulus: &Poly) -> Poly {
    div_rem(f, &mul(f, a, b), modulus).1
}

fn pow_mod(f: &Fp2, a: &Poly, mut exponent: u64, modulus: &Poly) -> Poly {
    let mut out = vec![f.one()];
    let mut base = div_rem(f, a, modulus).1;
    while exponent > 0 {
        if exponent & 1 != 0 {
            out = mul_mod(f, &out, &base, modulus);
        }
        exponent >>= 1;
        if exponent > 0 {
            base = mul_mod(f, &base, &base, modulus);
        }
    }
    out
}

fn eval(f: &Fp2, a: &Poly, x: El) -> El {
    a.iter()
        .rev()
        .fold(f.zero(), |acc, &c| f.add(f.mul(acc, x), c))
}

fn root_multiplicity(f: &Fp2, a: &Poly, root: El) -> u64 {
    let mut q = a.clone();
    let mut count = 0;
    let linear = vec![f.neg(root), f.one()];
    while q.len() > 1 {
        let (next, rem) = div_rem(f, &q, &linear);
        if !rem.is_empty() {
            break;
        }
        q = next;
        count += 1;
    }
    count
}

#[derive(Default)]
struct Stats {
    resultants: u64,
    gcds: u64,
    split_attempts: u64,
    exhaustive_fallback_evaluations: u64,
    root_candidates: u64,
    chi_lift_scans: u64,
    exact_checks: u64,
    false_candidates: u64,
    vertical_cases: u64,
    tangent_cases: u64,
    horizontal_cases: u64,
    infinity_cases: u64,
    exceptional_chi_cases: u64,
    multiple_resultant_roots: u64,
}

#[derive(Default)]
struct S3Stats {
    quadratics: u64,
    roots: u64,
    lift_scans: u64,
    exact_checks: u64,
    false_candidates: u64,
    degenerate: u64,
    sqrt_failures: u64,
    infinity_cases: u64,
}

fn s3_coefficients(curve: &Curve, x: El, z: El) -> [El; 3] {
    let f = &curve.f;
    let x2 = f.sqr(x);
    let z2 = f.sqr(z);
    let leading = f.sqr(f.sub(z, x));
    let middle = f.neg(f.scale(
        f.add(
            f.add(f.mul(x, z2), f.mul(f.add(x2, curve.a), z)),
            f.add(f.mul(x, curve.a), f.scale(curve.b, 2)),
        ),
        2,
    ));
    let constant = f.sub(
        f.sub(
            f.mul(x2, z2),
            f.scale(f.mul(f.add(f.mul(x, curve.a), f.scale(curve.b, 2)), z), 2),
        ),
        f.sub(f.scale(f.mul(curve.b, x), 4), f.sqr(curve.a)),
    );
    [constant, middle, leading]
}

fn s3_pairs(
    curve: &Curve,
    signed: &[Point],
    target: Point,
    ops: &mut GroupOps,
    stats: &mut S3Stats,
) -> BTreeSet<(usize, usize)> {
    let f = &curve.f;
    let mut out = BTreeSet::new();
    if target.infinity {
        stats.infinity_cases += 1;
        for (i, &p) in signed.iter().enumerate() {
            for (j, &q) in signed.iter().enumerate().skip(i) {
                stats.exact_checks += 1;
                if curve.add(ops, p, q).infinity {
                    out.insert((i, j));
                }
            }
        }
        return out;
    }
    for (i, &p) in signed.iter().enumerate() {
        let [c, b, a] = s3_coefficients(curve, p.x, target.x);
        stats.quadratics += 1;
        let mut roots = Vec::new();
        if a == f.zero() {
            stats.degenerate += 1;
            if b != f.zero() {
                roots.push(f.mul(f.neg(c), f.inv(b).unwrap()));
            } else if c == f.zero() {
                roots.extend(signed.iter().map(|q| q.x));
            }
        } else {
            let discriminant = f.sub(f.sqr(b), f.scale(f.mul(a, c), 4));
            if let Some(square_root) = f.sqrt(discriminant) {
                let inv = f.inv(f.scale(a, 2)).unwrap();
                roots.push(f.mul(f.add(f.neg(b), square_root), inv));
                roots.push(f.mul(f.sub(f.neg(b), square_root), inv));
            } else {
                stats.sqrt_failures += 1;
            }
        }
        roots.sort_unstable();
        roots.dedup();
        stats.roots += roots.len() as u64;
        for root in roots {
            for (j, &q) in signed.iter().enumerate() {
                stats.lift_scans += 1;
                if q.x == root {
                    stats.exact_checks += 1;
                    if curve.add(ops, p, q) == target {
                        out.insert((i.min(j), i.max(j)));
                    } else {
                        stats.false_candidates += 1;
                    }
                }
            }
        }
    }
    out
}

fn all_roots(f: &Fp2, polynomial: &Poly, stats: &mut Stats) -> Vec<El> {
    if polynomial.len() <= 1 {
        return Vec::new();
    }
    let x = vec![f.zero(), f.one()];
    let rational = gcd(
        f,
        polynomial.clone(),
        sub(f, &pow_mod(f, &x, f.order(), polynomial), &x),
    );
    stats.gcds += 1;
    fn split(f: &Fp2, a: Poly, stats: &mut Stats, output: &mut Vec<El>) {
        if a.len() <= 1 {
            return;
        }
        if a.len() == 2 {
            output.push(f.neg(f.mul(a[0], f.inv(a[1]).unwrap())));
            return;
        }
        if a.len() == 3 {
            let four_ac = f.scale(f.mul(a[2], a[0]), 4);
            let disc = f.sub(f.sqr(a[1]), four_ac);
            if let Some(root_disc) = f.sqrt(disc) {
                let inv = f.inv(f.scale(a[2], 2)).unwrap();
                output.push(f.mul(f.add(f.neg(a[1]), root_disc), inv));
                output.push(f.mul(f.sub(f.neg(a[1]), root_disc), inv));
                return;
            }
        }
        if a.len() == 4 {
            for shift in 0..128u64 {
                stats.split_attempts += 1;
                let linear = vec![[shift % f.p, 1], f.one()];
                let character = pow_mod(f, &linear, (f.order() - 1) / 2, &a);
                for test in [
                    sub(f, &character, &vec![f.one()]),
                    add(f, &character, &vec![f.one()]),
                    character,
                ] {
                    let divisor = gcd(f, a.clone(), test);
                    stats.gcds += 1;
                    if divisor.len() > 1 && divisor.len() < a.len() {
                        let (quotient, rem) = div_rem(f, &a, &divisor);
                        assert!(rem.is_empty());
                        split(f, divisor, stats, output);
                        split(f, quotient, stats, output);
                        return;
                    }
                }
            }
        }
        // Complete, explicitly charged fallback for a failed deterministic split.
        for packed in 0..f.order() {
            let z = [packed % f.p, packed / f.p];
            stats.exhaustive_fallback_evaluations += 1;
            if eval(f, &a, z) == f.zero() {
                output.push(z);
            }
        }
    }
    let mut roots = Vec::new();
    split(f, rational, stats, &mut roots);
    roots.sort_unstable();
    roots.dedup();
    assert!(roots.iter().all(|&z| eval(f, polynomial, z) == f.zero()));
    roots
}

fn chi(curve: &Curve, point: Point) -> Option<El> {
    if point.infinity {
        return None;
    }
    let f = &curve.f;
    f.inv(f.sub(point.y, f.one()))
        .map(|inv| f.mul(f.add(point.y, f.one()), inv))
}

fn chi_base(curve: &Curve, signed: &[Point]) -> Poly {
    let f = &curve.f;
    let mut base = vec![f.one()];
    for &point in signed {
        if let Some(z) = chi(curve, point) {
            base = mul(f, &base, &vec![f.neg(z), f.one()]);
        }
    }
    base
}

fn chord_resultant(curve: &Curve, point: Point, slope: El) -> Poly {
    let f = &curve.f;
    let d = vec![f.neg(f.one()), f.one()];
    let n = vec![f.one(), f.one()];
    let m3 = f.mul(f.sqr(slope), slope);
    let l = add(f, &scale(f, &d, f.sub(f.mul(slope, point.x), point.y)), &n);
    let d2 = mul(f, &d, &d);
    let d3 = mul(f, &d2, &d);
    let l3 = mul(f, &mul(f, &l, &l), &l);
    let ax = scale(f, &mul(f, &d2, &l), f.mul(curve.a, f.sqr(slope)));
    let b = scale(f, &d3, f.mul(curve.b, m3));
    let y2 = scale(f, &mul(f, &d, &mul(f, &n, &n)), m3);
    sub(f, &add(f, &add(f, &l3, &ax), &b), &y2)
}

fn parse_point(v: &Value) -> Point {
    if v["infinity"] == true {
        return Point::infinity([0, 0]);
    }
    Point::affine(
        [v["x"][0].as_u64().unwrap(), v["x"][1].as_u64().unwrap()],
        [v["y"][0].as_u64().unwrap(), v["y"][1].as_u64().unwrap()],
    )
}

fn point_tuple(point: Point) -> String {
    format!(
        "{},{},{},{},{}",
        u64::from(point.infinity),
        point.x[0],
        point.x[1],
        point.y[0],
        point.y[1]
    )
}

fn velu(src: &Curve, torsion: Point, point: Point, ops: &mut GroupOps) -> Point {
    if point.infinity || point == torsion || point == src.neg(torsion) {
        return src.identity();
    }
    let plus = src.add(ops, point, torsion);
    let minus = src.add(ops, point, src.neg(torsion));
    assert!(!plus.infinity && !minus.infinity);
    let f = &src.f;
    Point::affine(
        f.sub(
            f.add(f.add(point.x, plus.x), minus.x),
            f.scale(torsion.x, 2),
        ),
        f.add(f.add(point.y, plus.y), minus.y),
    )
}

fn freeze_targets(parent_path: &str, public_path: &str, checker_path: &str) {
    assert!(!Path::new(public_path).exists() && !Path::new(checker_path).exists());
    let parent_bytes = fs::read(parent_path).unwrap();
    let mut public: Value = serde_json::from_slice(&parent_bytes).unwrap();
    assert_eq!(public["p"], 211);
    assert_eq!(public["r"], 229);
    assert_eq!(
        public["target_order_domain"],
        "different-j-order-batch-target-v1:13"
    );
    assert_eq!(public["targets"].as_array().unwrap().len(), 64);
    let f = Fp2::new(211).unwrap();
    let source = Curve {
        f,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f,
        a: [130, 0],
        b: [179, 0],
    };
    let g = parse_point(&public["generator"]);
    let mapped_g = parse_point(&public["mapped_generator"]);
    let t = Point::affine([3, 0], [1, 0]);
    let mut ops = GroupOps::default();
    assert!(source.is_on_curve(g) && destination.is_on_curve(mapped_g));
    assert_eq!(velu(&source, t, g, &mut ops), mapped_g);
    let mut entries = Vec::new();
    for scalar in 1..229u64 {
        let point = source.mul(&mut ops, g, scalar);
        let mapped = destination.mul(&mut ops, mapped_g, scalar);
        assert_eq!(velu(&source, t, point, &mut ops), mapped);
        let tuple = point_tuple(point);
        let digest = sha256_hex(format!("different-j-order-batch-target-v1:13:{tuple}").as_bytes());
        entries.push((digest, tuple, point, mapped, scalar));
    }
    entries.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    for (existing, generated) in public["targets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(entries.iter())
    {
        assert_eq!(parse_point(&existing["source"]), generated.2);
        assert_eq!(parse_point(&existing["destination"]), generated.3);
    }
    let held = &entries[64..68];
    let targets: Vec<Value> = held
        .iter()
        .map(|entry| json!({"source":entry.2,"destination":entry.3}))
        .collect();
    let scalars: Vec<u64> = held.iter().map(|entry| entry.4).collect();
    let parent_sha = sha256_hex(&parent_bytes);
    public["targets"] = json!(targets);
    public["target_count"] = json!(4);
    public["schema"] = json!("different_j_chi_heldout_public/v1");
    public["parent_public_sha256"] = json!(parent_sha);
    public["selection_positions_zero_indexed"] = json!([64, 65, 66, 67]);
    assert!(public.get("scalars").is_none() && public.get("answer_scalar").is_none());
    fs::write(
        public_path,
        format!("{}\n", serde_json::to_string_pretty(&public).unwrap()),
    )
    .unwrap();
    let public_sha = sha256_hex(&fs::read(public_path).unwrap());
    let checker = json!({"schema":"different_j_chi_heldout_checker/v1",
        "public_sha256":public_sha,"parent_public_sha256":parent_sha,
        "target_count":4,"scalars":scalars});
    fs::write(
        checker_path,
        format!("{}\n", serde_json::to_string_pretty(&checker).unwrap()),
    )
    .unwrap();
    println!(
        "{}",
        json!({"schema":"different_j_chi_heldout_freeze/v1",
        "public_sha256":public_sha,"checker_sha256":sha256_hex(&fs::read(checker_path).unwrap()),
        "parent_public_sha256":parent_sha,"selection_positions_zero_indexed":[64,65,66,67],
        "target_count":4,"source_image_replays":228,"group_ops":ops})
    );
}

fn direct_pairs(
    curve: &Curve,
    signed: &[Point],
    target: Point,
    ops: &mut GroupOps,
) -> BTreeSet<(usize, usize)> {
    let mut out = BTreeSet::new();
    for (i, &a) in signed.iter().enumerate() {
        for (j, &b) in signed.iter().enumerate().skip(i) {
            if curve.add(ops, a, b) == target {
                out.insert((i, j));
            }
        }
    }
    out
}

fn full_image_base(
    source: &Curve,
    destination: &Curve,
    torsion: Point,
    columns: usize,
    ops: &mut GroupOps,
) -> (Vec<Point>, Vec<Point>, Vec<Point>, u64) {
    assert!([8, 16, 32, 64].contains(&columns));
    let field = &source.f;
    let mut squares = BTreeMap::<El, Vec<El>>::new();
    for y0 in 0..211 {
        for y1 in 0..211 {
            let y = [y0, y1];
            squares.entry(field.sqr(y)).or_default().push(y);
        }
    }
    let mut image = BTreeMap::<u64, (Point, Point, u64)>::new();
    let mut source_points = 0u64;
    let mut visit = |point: Point, ops: &mut GroupOps| {
        let mapped = velu(source, torsion, point, ops);
        assert!(destination.is_on_curve(mapped));
        let entry = image
            .entry(destination.key(&mapped))
            .or_insert((mapped, point, 0));
        assert_eq!(entry.0, mapped);
        entry.2 += 1;
        source_points += 1;
    };
    visit(source.identity(), ops);
    for x0 in 0..211 {
        for x1 in 0..211 {
            let x = [x0, x1];
            let rhs = field.add(
                field.add(field.mul(field.sqr(x), x), field.mul(source.a, x)),
                source.b,
            );
            if let Some(ys) = squares.get(&rhs) {
                for &y in ys {
                    visit(Point::affine(x, y), ops);
                }
            }
        }
    }
    assert_eq!(source_points, 44_655);
    assert_eq!(image.len(), 14_885);
    assert!(image.values().all(|entry| entry.2 == 3));
    let mut raw = Vec::new();
    let mut projected = Vec::new();
    let mut witnesses = Vec::new();
    let mut seen = BTreeSet::new();
    let mut candidates = 0u64;
    for (_, (mapped, witness, _)) in image {
        if mapped.infinity {
            continue;
        }
        candidates += 1;
        let subgroup_point = destination.mul(ops, mapped, 195);
        if subgroup_point.infinity {
            continue;
        }
        let orbit = destination
            .key(&subgroup_point)
            .min(destination.key(&destination.neg(subgroup_point)));
        if seen.insert(orbit) {
            raw.push(mapped);
            projected.push(subgroup_point);
            witnesses.push(witness);
            if raw.len() == columns {
                break;
            }
        }
    }
    assert_eq!(raw.len(), columns);
    (raw, projected, witnesses, candidates)
}

#[derive(Default)]
struct InverseStats {
    destination_points: u64,
    polynomial_calls: u64,
    split_attempts: u64,
    rational_x_roots: u64,
    rejected_no_root: u64,
    rejected_no_lift: u64,
    exact_map_checks: u64,
    rejected_map_mismatch: u64,
    accepted_image_points: u64,
}

fn inverse_velu_root(field: &Fp2, image_x: El, split_attempts: &mut u64) -> Option<El> {
    let cubic = vec![
        field.neg(field.add(field.scale(image_x, 9), field.from_fp(32))),
        field.add(field.scale(image_x, 6), field.from_fp(21)),
        field.neg(field.add(image_x, field.from_fp(6))),
        field.one(),
    ];
    let x = vec![field.zero(), field.one()];
    let rational = gcd(
        field,
        cubic.clone(),
        sub(field, &pow_mod(field, &x, field.order(), &cubic), &x),
    );
    if rational.len() <= 1 {
        return None;
    }
    let linear = if rational.len() == 2 {
        rational
    } else {
        let mut factor = None;
        for shift in 0..32u64 {
            *split_attempts += 1;
            let shifted_x = vec![[shift, 1], field.one()];
            let character = pow_mod(field, &shifted_x, (field.order() - 1) / 2, &rational);
            for test in [sub(field, &character, &vec![field.one()]), character] {
                let divisor = gcd(field, rational.clone(), test);
                if divisor.len() > 1 && divisor.len() < rational.len() {
                    let linear = if divisor.len() == 2 {
                        divisor
                    } else {
                        let (quotient, remainder) = div_rem(field, &rational, &divisor);
                        assert!(remainder.is_empty());
                        quotient
                    };
                    assert_eq!(linear.len(), 2);
                    factor = Some(linear);
                    break;
                }
            }
            if factor.is_some() {
                break;
            }
        }
        factor.expect("inverse-Velu polynomial split cap")
    };
    let root = field.neg(field.mul(linear[0], field.inv(linear[1]).unwrap()));
    assert_eq!(eval(field, &cubic, root), field.zero());
    Some(root)
}

fn inverse_velu_base(
    source: &Curve,
    destination: &Curve,
    torsion: Point,
    columns: usize,
    ops: &mut GroupOps,
) -> (Vec<Point>, Vec<Point>, Vec<Point>, InverseStats) {
    let field = &source.f;
    let mut raw = Vec::new();
    let mut projected = Vec::new();
    let mut witnesses = Vec::new();
    let mut seen = BTreeSet::new();
    let mut stats = InverseStats::default();
    for packed_x in 0..field.order() {
        let x = [packed_x % 211, packed_x / 211];
        let mut lifts = destination.lift_x(x);
        lifts.sort_by_key(|point| destination.key(point));
        for mapped in lifts {
            stats.destination_points += 1;
            stats.polynomial_calls += 1;
            let Some(root) = inverse_velu_root(field, x, &mut stats.split_attempts) else {
                stats.rejected_no_root += 1;
                continue;
            };
            stats.rational_x_roots += 1;
            let source_lifts = source.lift_x(root);
            if source_lifts.is_empty() {
                stats.rejected_no_lift += 1;
                continue;
            }
            let mut found = None;
            for point in source_lifts {
                stats.exact_map_checks += 1;
                if velu(source, torsion, point, ops) == mapped {
                    found = Some(point);
                    break;
                }
            }
            let Some(point) = found else {
                stats.rejected_map_mismatch += 1;
                continue;
            };
            stats.accepted_image_points += 1;
            let plus = source.add(ops, point, torsion);
            let minus = source.add(ops, point, source.neg(torsion));
            let mut preimages = [point, plus, minus];
            assert!(preimages
                .iter()
                .all(|&candidate| !candidate.infinity
                    && velu(source, torsion, candidate, ops) == mapped));
            preimages.sort_by_key(|candidate| {
                (
                    candidate.x[0],
                    candidate.x[1],
                    candidate.y[0],
                    candidate.y[1],
                )
            });
            let subgroup = destination.mul(ops, mapped, 195);
            if subgroup.infinity {
                continue;
            }
            let orbit = destination
                .key(&subgroup)
                .min(destination.key(&destination.neg(subgroup)));
            if seen.insert(orbit) {
                raw.push(mapped);
                projected.push(subgroup);
                witnesses.push(preimages[0]);
                if raw.len() == columns {
                    return (raw, projected, witnesses, stats);
                }
            }
        }
    }
    panic!("inverse-Velu oracle did not find {columns} columns")
}

fn destination_direct_base(
    destination: &Curve,
    columns: usize,
    ops: &mut GroupOps,
) -> (Vec<Point>, Vec<Point>, u64) {
    let mut raw = Vec::new();
    let mut projected = Vec::new();
    let mut seen = BTreeSet::new();
    let mut scanned = 0u64;
    for packed_x in 0..destination.f.order() {
        let x = [packed_x % 211, packed_x / 211];
        let mut lifts = destination.lift_x(x);
        lifts.sort_by_key(|point| destination.key(point));
        for point in lifts {
            scanned += 1;
            let subgroup = destination.mul(ops, point, 195);
            if subgroup.infinity {
                continue;
            }
            let orbit = destination
                .key(&subgroup)
                .min(destination.key(&destination.neg(subgroup)));
            if seen.insert(orbit) {
                raw.push(point);
                projected.push(subgroup);
                if raw.len() == columns {
                    return (raw, projected, scanned);
                }
            }
        }
    }
    panic!("destination scan did not find {columns} columns")
}

fn freeze_base(parent_path: &str, heldout_path: &str, output_path: &str) {
    assert!(!Path::new(output_path).exists());
    let parent_bytes = fs::read(parent_path).unwrap();
    let heldout_bytes = fs::read(heldout_path).unwrap();
    let parent: Value = serde_json::from_slice(&parent_bytes).unwrap();
    let heldout: Value = serde_json::from_slice(&heldout_bytes).unwrap();
    assert_eq!(parent["p"], 211);
    assert_eq!(parent["r"], 229);
    assert_eq!(
        heldout["parent_public_sha256"].as_str().unwrap(),
        sha256_hex(&parent_bytes)
    );
    assert_eq!(heldout["targets"].as_array().unwrap().len(), 4);
    let field = Fp2::new(211).unwrap();
    let source = Curve {
        f: field,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f: field,
        a: [130, 0],
        b: [179, 0],
    };
    let torsion = Point::affine([3, 0], [1, 0]);
    let mut ops = GroupOps::default();
    let (raw, projected, witnesses, candidates) =
        full_image_base(&source, &destination, torsion, 64, &mut ops);
    for index in 0..8 {
        assert_eq!(raw[index], parse_point(&parent["full_base_raw"][index]));
        assert_eq!(
            projected[index],
            parse_point(&parent["full_base_projected"][index])
        );
        assert_eq!(
            witnesses[index],
            parse_point(&parent["full_base_witness_source"][index])
        );
    }
    for index in 0..64 {
        let source_point = source.mul(&mut ops, witnesses[index], 195);
        assert!(!source_point.infinity);
        assert!(source.mul(&mut ops, source_point, 229).infinity);
        assert_eq!(
            velu(&source, torsion, source_point, &mut ops),
            projected[index]
        );
        assert!(destination.mul(&mut ops, projected[index], 229).infinity);
    }
    let public = json!({
        "schema":"isogeny_fiber_base_grid_public/v1",
        "parent_public_sha256":sha256_hex(&parent_bytes),
        "heldout_public_sha256":sha256_hex(&heldout_bytes),
        "p":211,"r":229,"cofactor":195,"columns":64,
        "base_raw_destination":raw,
        "base_projected_destination":projected,
        "base_witness_source":witnesses,
        "source_points":44_655,"image_points":14_885,
        "candidates_examined":candidates,
        "selection":"ascending destination key, first signed projected orbit"
    });
    fs::write(
        output_path,
        format!("{}\n", serde_json::to_string_pretty(&public).unwrap()),
    )
    .unwrap();
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_freeze/v1",
            "public_sha256":sha256_hex(&fs::read(output_path).unwrap()),
            "columns":64,"candidates_examined":candidates,
            "source_points":44_655,"image_points":14_885,"group_ops":ops})
    );
}

fn modmul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % 229) as u64
}

fn insert_relation(pivots: &mut [Option<Vec<u64>>; 9], mut row: Vec<u64>) -> bool {
    for col in 0..9 {
        if row[col] == 0 {
            continue;
        }
        if let Some(pivot) = &pivots[col] {
            let factor = row[col];
            for j in col..10 {
                row[j] = (row[j] + 229 - modmul(factor, pivot[j])) % 229;
            }
        } else {
            let inverse = crypto_lib::cryptanalysis::residual_walk::pow_mod(row[col], 227, 229);
            for item in row.iter_mut().skip(col) {
                *item = modmul(*item, inverse);
            }
            pivots[col] = Some(row);
            return true;
        }
    }
    assert_eq!(row[9], 0);
    false
}

fn recover(pivots: &[Option<Vec<u64>>; 9]) -> Option<[u64; 9]> {
    if pivots.iter().any(Option::is_none) {
        return None;
    }
    let mut output = [0u64; 9];
    for col in (0..9).rev() {
        let row = pivots[col].as_ref().unwrap();
        let mut rhs = row[9];
        for j in col + 1..9 {
            rhs = (rhs + 229 - modmul(row[j], output[j])) % 229;
        }
        output[col] = rhs;
    }
    Some(output)
}

fn insert_relation_grid(
    pivots: &mut [Option<Vec<u64>>],
    mut row: Vec<u64>,
    columns: usize,
) -> bool {
    assert_eq!(pivots.len(), columns + 1);
    assert_eq!(row.len(), columns + 2);
    for col in 0..=columns {
        if row[col] == 0 {
            continue;
        }
        if let Some(pivot) = &pivots[col] {
            let factor = row[col];
            for j in col..columns + 2 {
                row[j] = (row[j] + 229 - modmul(factor, pivot[j])) % 229;
            }
        } else {
            let inverse = crypto_lib::cryptanalysis::residual_walk::pow_mod(row[col], 227, 229);
            for item in row.iter_mut().skip(col) {
                *item = modmul(*item, inverse);
            }
            pivots[col] = Some(row);
            return true;
        }
    }
    assert_eq!(row[columns + 1], 0);
    false
}

fn recover_grid(pivots: &[Option<Vec<u64>>], columns: usize) -> Option<Vec<u64>> {
    if pivots.len() != columns + 1 || pivots.iter().any(Option::is_none) {
        return None;
    }
    let mut output = vec![0u64; columns + 1];
    for col in (0..=columns).rev() {
        let row = pivots[col].as_ref().unwrap();
        let mut rhs = row[columns + 1];
        for j in col + 1..=columns {
            rhs = (rhs + 229 - modmul(row[j], output[j])) % 229;
        }
        output[col] = rhs;
    }
    Some(output)
}

fn thread_cpu_ns() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) },
        0
    );
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}

fn peak_rss_bytes() -> u64 {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::zeroed();
    assert_eq!(
        unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) },
        0
    );
    let raw = unsafe { usage.assume_init() }.ru_maxrss as u64;
    if cfg!(target_os = "macos") {
        raw
    } else {
        raw * 1024
    }
}

fn run_heldout(path: &str, round: u64, arm: &str, position: usize) {
    assert!(round < 3 && position < 4);
    assert!([
        "chi-resultant",
        "direct-pair",
        "source-s3",
        "quotient-s3",
        "source-bsgs",
        "rho-negation",
        "rho-negation-aa"
    ]
    .contains(&arm));
    let bytes = fs::read(path).unwrap();
    let public: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(public["schema"], "different_j_chi_heldout_public/v1");
    assert_eq!(
        (public["p"].as_u64(), public["r"].as_u64()),
        (Some(211), Some(229))
    );
    assert!(public.get("scalars").is_none() && public.get("answer_scalar").is_none());
    assert_eq!(
        public["selection_positions_zero_indexed"],
        json!([64, 65, 66, 67])
    );
    let f = Fp2::new(211).unwrap();
    let source = Curve {
        f,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f,
        a: [130, 0],
        b: [179, 0],
    };
    let g = parse_point(&public["generator"]);
    let mapped_g = parse_point(&public["mapped_generator"]);
    let t = Point::affine([3, 0], [1, 0]);
    let q = parse_point(&public["targets"][position]["source"]);
    let mapped_q = parse_point(&public["targets"][position]["destination"]);
    let wall = Instant::now();
    let cpu = thread_cpu_ns();
    take_field_counters();
    let mut setup_ops = GroupOps::default();
    assert!(source.is_on_curve(g) && source.is_on_curve(q));
    assert!(destination.is_on_curve(mapped_g) && destination.is_on_curve(mapped_q));
    assert_eq!(velu(&source, t, g, &mut setup_ops), mapped_g);
    assert_eq!(velu(&source, t, q, &mut setup_ops), mapped_q);
    assert!(source.mul(&mut setup_ops, g, 229).infinity);
    assert!(source.mul(&mut setup_ops, q, 229).infinity);
    let relation_arm = ["chi-resultant", "direct-pair", "source-s3", "quotient-s3"].contains(&arm);
    let mut signed = Vec::new();
    let mut mapped_signed = Vec::new();
    if relation_arm {
        for witness in public["full_base_witness_source"].as_array().unwrap() {
            let projected = source.mul(&mut setup_ops, parse_point(witness), 195);
            assert!(!projected.infinity && source.mul(&mut setup_ops, projected, 229).infinity);
            signed.push(projected);
            signed.push(source.neg(projected));
        }
        assert_eq!(signed.len(), 16);
        mapped_signed = signed
            .iter()
            .map(|&p| velu(&source, t, p, &mut setup_ops))
            .collect();
        let expected_destination = public["full_base_projected"].as_array().unwrap();
        for index in 0..8 {
            assert_eq!(
                mapped_signed[2 * index],
                parse_point(&expected_destination[index])
            );
        }
    }
    let base = if arm == "chi-resultant" {
        chi_base(&source, &signed)
    } else {
        Vec::new()
    };
    let mut table = BTreeMap::<u64, Vec<(usize, usize)>>::new();
    if arm == "direct-pair" {
        for i in 0..16 {
            for j in i..16 {
                let sum = source.add(&mut setup_ops, signed[i], signed[j]);
                if !sum.infinity {
                    table.entry(source.key(&sum)).or_default().push((i, j));
                }
            }
        }
    }
    let mut baby = BTreeMap::<u64, u64>::new();
    let mut giant_step = source.identity();
    if arm == "source-bsgs" {
        let mut cursor = source.identity();
        for small in 0..16 {
            baby.insert(source.key(&cursor), small);
            cursor = source.add(&mut setup_ops, cursor, g);
        }
        giant_step = source.neg(source.mul(&mut setup_ops, g, 16));
    }
    let setup_cpu_ns = thread_cpu_ns() - cpu;
    let setup_fp = take_field_counters();
    let seed = (0x2026_1015u64 ^ 211)
        .wrapping_add(round.wrapping_mul(0x9e37_79b9))
        .wrapping_add(((position + 64) as u64).wrapping_mul(0x0123_4567));
    let query_cpu_start = thread_cpu_ns();
    let mut ops = GroupOps::default();
    let mut stats = Stats::default();
    let mut s3_stats = S3Stats::default();
    let mut witnesses = Vec::new();
    let mut trials = 0u64;
    let mut relations = 0u64;
    let mut rank = 0u64;
    let mut recovered = None;
    let mut rho_attempts = Vec::new();
    if relation_arm {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut pivots: [Option<Vec<u64>>; 9] = std::array::from_fn(|_| None);
        let quotient = arm == "quotient-s3";
        let query_curve = if quotient { &destination } else { &source };
        let query_g = if quotient { mapped_g } else { g };
        let query_q = if quotient { mapped_q } else { q };
        for _ in 0..1832 {
            trials += 1;
            let a = rng.gen_range(0..229u64);
            let b = rng.gen_range(0..229u64);
            let a_g = query_curve.mul(&mut ops, query_g, a);
            let b_q = query_curve.mul(&mut ops, query_q, b);
            let candidate = query_curve.add(&mut ops, a_g, b_q);
            let projected = query_curve.mul(&mut ops, candidate, 195);
            if projected.infinity {
                continue;
            }
            let pairs = if arm == "chi-resultant" {
                polynomial_pairs(&source, &signed, &base, projected, &mut ops, &mut stats)
            } else if arm == "source-s3" {
                s3_pairs(&source, &signed, projected, &mut ops, &mut s3_stats)
            } else if arm == "quotient-s3" {
                s3_pairs(
                    &destination,
                    &mapped_signed,
                    projected,
                    &mut ops,
                    &mut s3_stats,
                )
            } else {
                let mut out = BTreeSet::new();
                if let Some(entries) = table.get(&source.key(&projected)) {
                    for &(i, j) in entries {
                        if source.add(&mut ops, signed[i], signed[j]) == projected {
                            out.insert((i, j));
                        }
                    }
                }
                out
            };
            if let Some(&(i, j)) = pairs.first() {
                let mut row = vec![0u64; 10];
                for index in [i, j] {
                    let column = index / 2;
                    let coefficient = if index % 2 == 0 { 1 } else { 228 };
                    row[column] = (row[column] + coefficient) % 229;
                }
                row[8] = (229 - modmul(195, b)) % 229;
                row[9] = modmul(195, a);
                witnesses.push(json!({"a":a,"b":b,"i":i,"j":j,"projected":projected}));
                relations += 1;
                if insert_relation(&mut pivots, row) {
                    rank += 1;
                }
                if rank == 9 {
                    break;
                }
            }
        }
        recovered = recover(&pivots).map(|sol| sol[8]);
    } else if arm == "source-bsgs" {
        let mut cursor = q;
        for giant in 0..15u64 {
            trials += 1;
            if let Some(&small) = baby.get(&source.key(&cursor)) {
                recovered = Some((giant * 16 + small) % 229);
                break;
            }
            cursor = source.add(&mut ops, cursor, giant_step);
        }
    } else {
        for retry in 0..32u64 {
            let try_seed = seed.wrapping_add(retry * 0x0100_0000);
            let result = rho_reference_negation(&source, g, q, 229, try_seed, 4580);
            ops.merge(result.group_ops);
            rho_attempts.push(json!({"retry":retry,"seed":try_seed,"steps":result.steps,
                "walks":result.walks,"group_ops":result.group_ops,"recovered":result.recovered,
                "verified":result.verified}));
            if result.verified {
                recovered = result.recovered;
                break;
            }
        }
    }
    let query_cpu_ns = thread_cpu_ns() - query_cpu_start;
    let query_fp = take_field_counters();
    let mut verify_ops = GroupOps::default();
    let verified = recovered.is_some_and(|scalar| {
        source.mul(&mut verify_ops, g, scalar) == q
            && destination.mul(&mut verify_ops, mapped_g, scalar) == mapped_q
    });
    let verify_fp = take_field_counters();
    let peak_rss = peak_rss_bytes();
    let result = json!({"schema":"different_j_chi_heldout_run/v3",
        "public_sha256":sha256_hex(&bytes),"round":round,"arm":arm,"position":position,
        "seed":seed,"target":q,"mapped_target":mapped_q,
        "setup_group_ops":setup_ops,"query_group_ops":ops,"verify_group_ops":verify_ops,
        "setup_fp_muls":setup_fp.0,"setup_fp_invs":setup_fp.1,
        "query_fp_muls":query_fp.0,"query_fp_invs":query_fp.1,
        "verify_fp_muls":verify_fp.0,"verify_fp_invs":verify_fp.1,
        "setup_cpu_ns":setup_cpu_ns,"query_cpu_ns":query_cpu_ns,
        "total_cpu_ns":thread_cpu_ns()-cpu,"total_wall_ns":wall.elapsed().as_nanos(),
        "peak_rss_bytes":peak_rss,"trials":trials,"relations":relations,
        "rank":rank,"recovered":recovered,"verified":verified,
        "relation_witnesses":witnesses,"rho_attempts":rho_attempts,
        "chi_stats":{"resultants":stats.resultants,"gcds":stats.gcds,
          "split_attempts":stats.split_attempts,
          "exhaustive_fallback_evaluations":stats.exhaustive_fallback_evaluations,
          "root_candidates":stats.root_candidates,"chi_lift_scans":stats.chi_lift_scans,
          "exact_checks":stats.exact_checks,"false_candidates":stats.false_candidates,
          "vertical_cases":stats.vertical_cases,"tangent_cases":stats.tangent_cases,
          "horizontal_cases":stats.horizontal_cases,"infinity_cases":stats.infinity_cases,
          "exceptional_chi_cases":stats.exceptional_chi_cases,
          "multiple_resultant_roots":stats.multiple_resultant_roots},
        "s3_stats":{"quadratics":s3_stats.quadratics,"roots":s3_stats.roots,
          "lift_scans":s3_stats.lift_scans,"exact_checks":s3_stats.exact_checks,
          "false_candidates":s3_stats.false_candidates,
          "degenerate":s3_stats.degenerate,"sqrt_failures":s3_stats.sqrt_failures,
          "infinity_cases":s3_stats.infinity_cases}});
    println!("{result}");
    assert!(verified && peak_rss <= 512 * 1024 * 1024);
}

fn exhaustive_s3(path: &str) {
    let bytes = fs::read(path).unwrap();
    let public: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(public["p"], 211);
    assert_eq!(public["r"], 229);
    let f = Fp2::new(211).unwrap();
    let source = Curve {
        f,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f,
        a: [130, 0],
        b: [179, 0],
    };
    let g = parse_point(&public["generator"]);
    let mg = parse_point(&public["mapped_generator"]);
    let t = Point::affine([3, 0], [1, 0]);
    let mut ops = GroupOps::default();
    let mut signed = Vec::new();
    for witness in public["full_base_witness_source"].as_array().unwrap() {
        let projected = source.mul(&mut ops, parse_point(witness), 195);
        signed.push(projected);
        signed.push(source.neg(projected));
    }
    let mapped: Vec<Point> = signed
        .iter()
        .map(|&q| velu(&source, t, q, &mut ops))
        .collect();
    take_field_counters();
    let start = Instant::now();
    let mut source_stats = S3Stats::default();
    let mut quotient_stats = S3Stats::default();
    let mut zero_source = 0u64;
    let mut zero_quotient = 0u64;
    let mut pairs_source = 0u64;
    let mut pairs_quotient = 0u64;
    let mut sq = source.identity();
    let mut dq = destination.identity();
    for scalar_label in 0..229 {
        let expected_source = direct_pairs(&source, &signed, sq, &mut ops);
        let actual_source = s3_pairs(&source, &signed, sq, &mut ops, &mut source_stats);
        assert_eq!(actual_source, expected_source);
        let expected_quotient = direct_pairs(&destination, &mapped, dq, &mut ops);
        let actual_quotient = s3_pairs(&destination, &mapped, dq, &mut ops, &mut quotient_stats);
        assert_eq!(actual_quotient, expected_quotient);
        assert_eq!(actual_source, actual_quotient);
        println!(
            "{}",
            json!({"schema":"different_j_chi_s3_case/v2",
            "scalar_label":scalar_label,"source_target":sq,
            "quotient_target":dq,
            "source_pairs":actual_source.iter().copied().collect::<Vec<_>>(),
            "quotient_pairs":actual_quotient.iter().copied().collect::<Vec<_>>(),
            "status":"pass"})
        );
        zero_source += u64::from(actual_source.is_empty());
        zero_quotient += u64::from(actual_quotient.is_empty());
        pairs_source += actual_source.len() as u64;
        pairs_quotient += actual_quotient.len() as u64;
        sq = source.add(&mut ops, sq, g);
        dq = destination.add(&mut ops, dq, mg);
    }
    assert!(sq.infinity && dq.infinity);
    let fp = take_field_counters();
    println!(
        "{}",
        json!({"schema":"different_j_chi_s3_exhaustive/v2",
        "public_sha256":sha256_hex(&bytes),"source_cases":229,
        "quotient_cases":229,"source_pairs":pairs_source,
        "quotient_pairs":pairs_quotient,"source_zero_cases":zero_source,
        "quotient_zero_cases":zero_quotient,
        "source_s3":{"quadratics":source_stats.quadratics,
          "roots":source_stats.roots,"lift_scans":source_stats.lift_scans,
          "exact_checks":source_stats.exact_checks,
          "false_candidates":source_stats.false_candidates,
          "degenerate":source_stats.degenerate,
          "sqrt_failures":source_stats.sqrt_failures,
          "infinity_cases":source_stats.infinity_cases},
        "quotient_s3":{"quadratics":quotient_stats.quadratics,
          "roots":quotient_stats.roots,"lift_scans":quotient_stats.lift_scans,
          "exact_checks":quotient_stats.exact_checks,
          "false_candidates":quotient_stats.false_candidates,
          "degenerate":quotient_stats.degenerate,
          "sqrt_failures":quotient_stats.sqrt_failures,
          "infinity_cases":quotient_stats.infinity_cases},
        "group_ops":ops,"fp_muls":fp.0,"fp_invs":fp.1,
        "wall_ns":start.elapsed().as_nanos(),"status":"pass"})
    );
}

fn polynomial_pairs(
    curve: &Curve,
    signed: &[Point],
    base: &Poly,
    target: Point,
    ops: &mut GroupOps,
    stats: &mut Stats,
) -> BTreeSet<(usize, usize)> {
    let f = &curve.f;
    let mut out = BTreeSet::new();
    for (i, &point) in signed.iter().enumerate() {
        if target.infinity {
            stats.infinity_cases += 1;
            for (j, &candidate) in signed.iter().enumerate() {
                if candidate == curve.neg(point) {
                    out.insert((i.min(j), i.max(j)));
                }
            }
            continue;
        }
        if point == target {
            stats.vertical_cases += 1; // The second summand would be infinity.
            continue;
        }
        let minus_target = curve.neg(target);
        let slope = if point == minus_target {
            stats.tangent_cases += 1;
            let numerator = f.add(f.scale(f.sqr(point.x), 3), curve.a);
            let Some(denominator) = f.inv(f.scale(point.y, 2)) else {
                stats.vertical_cases += 1;
                continue;
            };
            f.mul(numerator, denominator)
        } else {
            let Some(denominator) = f.inv(f.sub(minus_target.x, point.x)) else {
                stats.vertical_cases += 1;
                continue;
            };
            f.mul(f.sub(minus_target.y, point.y), denominator)
        };
        if slope == f.zero() {
            stats.horizontal_cases += 1;
            let z = chi(curve, point);
            for (j, &candidate) in signed.iter().enumerate() {
                stats.chi_lift_scans += 1;
                if chi(curve, candidate) == z {
                    stats.exact_checks += 1;
                    if curve.add(ops, point, candidate) == target {
                        out.insert((i.min(j), i.max(j)));
                    } else {
                        stats.false_candidates += 1;
                    }
                }
            }
            continue;
        }
        let resultant = chord_resultant(curve, point, slope);
        stats.resultants += 1;
        let intersection = gcd(f, resultant.clone(), base.clone());
        stats.gcds += 1;
        for root in all_roots(f, &intersection, stats) {
            stats.root_candidates += 1;
            if root_multiplicity(f, &resultant, root) > 1 {
                stats.multiple_resultant_roots += 1;
            }
            for (j, &candidate) in signed.iter().enumerate() {
                stats.chi_lift_scans += 1;
                if chi(curve, candidate) == Some(root) {
                    stats.exact_checks += 1;
                    if curve.add(ops, point, candidate) == target {
                        out.insert((i.min(j), i.max(j)));
                    } else {
                        stats.false_candidates += 1;
                    }
                }
            }
        }
        for (j, &candidate) in signed.iter().enumerate() {
            if chi(curve, candidate).is_none() {
                stats.exceptional_chi_cases += 1;
                stats.exact_checks += 1;
                if curve.add(ops, point, candidate) == target {
                    out.insert((i.min(j), i.max(j)));
                }
            }
        }
    }
    out
}

fn exhaustive_grid(base_path: &str, columns: usize) {
    assert!([8, 16, 32, 64].contains(&columns));
    let wall = Instant::now();
    let bytes = fs::read(base_path).unwrap();
    let public: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(public["schema"], "isogeny_fiber_base_grid_public/v1");
    assert_eq!(public["columns"], 64);
    let field = Fp2::new(211).unwrap();
    let source = Curve {
        f: field,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f: field,
        a: [130, 0],
        b: [179, 0],
    };
    let torsion = Point::affine([3, 0], [1, 0]);
    let mut setup_ops = GroupOps::default();
    let mut signed = Vec::with_capacity(2 * columns);
    for index in 0..columns {
        let witness = parse_point(&public["base_witness_source"][index]);
        let subgroup_point = source.mul(&mut setup_ops, witness, 195);
        assert!(!subgroup_point.infinity);
        assert!(source.mul(&mut setup_ops, subgroup_point, 229).infinity);
        assert_eq!(
            velu(&source, torsion, subgroup_point, &mut setup_ops),
            parse_point(&public["base_projected_destination"][index])
        );
        assert!(destination.is_on_curve(parse_point(&public["base_projected_destination"][index])));
        signed.push(subgroup_point);
        signed.push(source.neg(subgroup_point));
    }
    let chi_polynomial = chi_base(&source, &signed);
    let generator = {
        let parent_path =
            "research/isogeny_torsion_fibers_20261012/different_j_order_batch_20261015/PUBLIC_V1.json";
        let parent_bytes = fs::read(parent_path).unwrap();
        assert_eq!(
            sha256_hex(&parent_bytes),
            public["parent_public_sha256"].as_str().unwrap()
        );
        let parent: Value = serde_json::from_slice(&parent_bytes).unwrap();
        parse_point(&parent["generator"])
    };
    let setup_fp = take_field_counters();
    let mut query_ops = GroupOps::default();
    let mut stats = Stats::default();
    let mut residual = source.identity();
    let mut supported = 0usize;
    let mut total_pairs = 0usize;
    let mut diagonal_pairs = 0usize;
    for scalar_label in 0..229u64 {
        let expected = direct_pairs(&source, &signed, residual, &mut query_ops);
        let actual = polynomial_pairs(
            &source,
            &signed,
            &chi_polynomial,
            residual,
            &mut query_ops,
            &mut stats,
        );
        assert_eq!(
            actual, expected,
            "chi pair-set mismatch at B={columns}, scalar={scalar_label}"
        );
        supported += usize::from(!actual.is_empty());
        total_pairs += actual.len();
        diagonal_pairs += actual.iter().filter(|(i, j)| i == j).count();
        println!(
            "{}",
            json!({"schema":"isogeny_fiber_base_grid_case/v1",
                "public_sha256":sha256_hex(&bytes),"columns":columns,
                "scalar_label":scalar_label,"residual":residual,
                "pairs":actual.iter().copied().collect::<Vec<_>>(),
                "status":"pass"})
        );
        residual = source.add(&mut query_ops, residual, generator);
    }
    assert!(residual.infinity);
    let query_fp = take_field_counters();
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_exhaustive/v1",
            "public_sha256":sha256_hex(&bytes),"columns":columns,
            "cases":229,"supported_residues":supported,
            "zero_residues":229-supported,"unordered_pairs":total_pairs,
            "diagonal_pairs":diagonal_pairs,
            "chi_degree":chi_polynomial.len()-1,
            "setup_group_ops":setup_ops,"query_group_ops":query_ops,
            "setup_fp_muls":setup_fp.0,"setup_fp_invs":setup_fp.1,
            "query_fp_muls":query_fp.0,"query_fp_invs":query_fp.1,
            "resultants":stats.resultants,"gcds":stats.gcds,
            "split_attempts":stats.split_attempts,
            "root_candidates":stats.root_candidates,
            "chi_lift_scans":stats.chi_lift_scans,
            "exact_checks":stats.exact_checks,
            "false_candidates":stats.false_candidates,
            "vertical_cases":stats.vertical_cases,
            "tangent_cases":stats.tangent_cases,
            "horizontal_cases":stats.horizontal_cases,
            "infinity_cases":stats.infinity_cases,
            "exceptional_chi_cases":stats.exceptional_chi_cases,
            "multiple_resultant_roots":stats.multiple_resultant_roots,
            "wall_ns":wall.elapsed().as_nanos(),"status":"pass"})
    );
}

fn run_grid(
    base_path: &str,
    heldout_path: &str,
    round: u64,
    arm: &str,
    columns: usize,
    position: usize,
) {
    assert!(round < 3 && position < 4);
    assert!([8, 16, 32, 64].contains(&columns));
    assert!(matches!(
        arm,
        "chi-inverse"
            | "chi-full"
            | "direct-inverse"
            | "direct-full"
            | "destination-direct"
            | "source-bsgs"
            | "rho-negation"
            | "rho-negation-aa"
    ));
    let wall = Instant::now();
    let cpu = thread_cpu_ns();
    take_field_counters();
    let base_bytes = fs::read(base_path).unwrap();
    let heldout_bytes = fs::read(heldout_path).unwrap();
    let base_public: Value = serde_json::from_slice(&base_bytes).unwrap();
    let heldout: Value = serde_json::from_slice(&heldout_bytes).unwrap();
    let base_sha = sha256_hex(&base_bytes);
    let heldout_sha = sha256_hex(&heldout_bytes);
    assert_eq!(base_public["schema"], "isogeny_fiber_base_grid_public/v1");
    assert_eq!(
        base_public["heldout_public_sha256"].as_str().unwrap(),
        heldout_sha
    );
    assert_eq!(heldout["schema"], "different_j_chi_heldout_public/v1");
    let field = Fp2::new(211).unwrap();
    let source = Curve {
        f: field,
        a: [190, 0],
        b: [37, 0],
    };
    let destination = Curve {
        f: field,
        a: [130, 0],
        b: [179, 0],
    };
    let torsion = Point::affine([3, 0], [1, 0]);
    let generator = parse_point(&heldout["generator"]);
    let mapped_generator = parse_point(&heldout["mapped_generator"]);
    let target = parse_point(&heldout["targets"][position]["source"]);
    let mapped_target = parse_point(&heldout["targets"][position]["destination"]);
    let mut setup_ops = GroupOps::default();
    assert!(source.is_on_curve(generator) && source.is_on_curve(target));
    assert!(destination.is_on_curve(mapped_generator) && destination.is_on_curve(mapped_target));
    assert_eq!(
        velu(&source, torsion, generator, &mut setup_ops),
        mapped_generator
    );
    assert_eq!(
        velu(&source, torsion, target, &mut setup_ops),
        mapped_target
    );
    assert!(source.mul(&mut setup_ops, generator, 229).infinity);
    assert!(source.mul(&mut setup_ops, target, 229).infinity);
    let relation_arm = matches!(
        arm,
        "chi-inverse" | "chi-full" | "direct-inverse" | "direct-full" | "destination-direct"
    );
    let destination_relation = arm == "destination-direct";
    let chi_relation = matches!(arm, "chi-inverse" | "chi-full");
    let query_curve = if destination_relation {
        &destination
    } else {
        &source
    };
    let query_generator = if destination_relation {
        mapped_generator
    } else {
        generator
    };
    let query_target = if destination_relation {
        mapped_target
    } else {
        target
    };
    let mut signed = Vec::new();
    let mut candidate_count = 0u64;
    let mut positive_base = Vec::new();
    let mut inverse_stats = InverseStats::default();
    if destination_relation {
        let (_, projected, scanned) =
            destination_direct_base(&destination, columns, &mut setup_ops);
        candidate_count = scanned;
        for subgroup in projected {
            assert!(!subgroup.infinity && destination.mul(&mut setup_ops, subgroup, 229).infinity);
            positive_base.push(subgroup);
            signed.push(subgroup);
            signed.push(destination.neg(subgroup));
        }
    } else if relation_arm {
        let (raw, projected, witnesses) = if matches!(arm, "chi-inverse" | "direct-inverse") {
            let (raw, projected, witnesses, stats) =
                inverse_velu_base(&source, &destination, torsion, columns, &mut setup_ops);
            candidate_count = stats.destination_points;
            inverse_stats = stats;
            (raw, projected, witnesses)
        } else {
            let (raw, projected, witnesses, count) =
                full_image_base(&source, &destination, torsion, columns, &mut setup_ops);
            candidate_count = count;
            (raw, projected, witnesses)
        };
        for index in 0..columns {
            assert_eq!(
                raw[index],
                parse_point(&base_public["base_raw_destination"][index])
            );
            assert_eq!(
                projected[index],
                parse_point(&base_public["base_projected_destination"][index])
            );
            let subgroup = source.mul(&mut setup_ops, witnesses[index], 195);
            assert!(!subgroup.infinity && source.mul(&mut setup_ops, subgroup, 229).infinity);
            assert_eq!(
                velu(&source, torsion, subgroup, &mut setup_ops),
                projected[index]
            );
            positive_base.push(subgroup);
            signed.push(subgroup);
            signed.push(source.neg(subgroup));
        }
    }
    let base_matches_frozen = if destination_relation {
        positive_base.iter().enumerate().all(|(index, &point)| {
            point == parse_point(&base_public["base_projected_destination"][index])
        })
    } else {
        relation_arm
    };
    let chi_polynomial = if chi_relation {
        chi_base(&source, &signed)
    } else {
        Vec::new()
    };
    let mut table = BTreeMap::<u64, Vec<(usize, usize)>>::new();
    if relation_arm && !chi_relation {
        for i in 0..signed.len() {
            for j in i..signed.len() {
                let sum = query_curve.add(&mut setup_ops, signed[i], signed[j]);
                if !sum.infinity {
                    table.entry(query_curve.key(&sum)).or_default().push((i, j));
                }
            }
        }
    }
    let mut baby = BTreeMap::<u64, u64>::new();
    let mut giant_step = source.identity();
    if arm == "source-bsgs" {
        let mut cursor = source.identity();
        for small in 0..16 {
            baby.insert(source.key(&cursor), small);
            cursor = source.add(&mut setup_ops, cursor, generator);
        }
        giant_step = source.neg(source.mul(&mut setup_ops, generator, 16));
    }
    let setup_cpu_ns = thread_cpu_ns() - cpu;
    let setup_fp = take_field_counters();
    let seed = (0x2026_1015u64 ^ 211)
        .wrapping_add(round.wrapping_mul(0x9e37_79b9))
        .wrapping_add(((position + 64) as u64).wrapping_mul(0x0123_4567));
    let query_cpu_start = thread_cpu_ns();
    let mut row_cpu_ns = 0u64;
    let mut recovery_cpu_ns = 0u64;
    let mut query_ops = GroupOps::default();
    let mut stats = Stats::default();
    let mut witnesses = Vec::new();
    let mut trials = 0u64;
    let mut relations = 0u64;
    let mut rank = 0usize;
    let mut recovered = None;
    let mut rho_attempts = Vec::new();
    if relation_arm {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut pivots = vec![None; columns + 1];
        for _ in 0..1832 {
            trials += 1;
            let a = rng.gen_range(0..229u64);
            let b = rng.gen_range(0..229u64);
            let a_g = query_curve.mul(&mut query_ops, query_generator, a);
            let b_q = query_curve.mul(&mut query_ops, query_target, b);
            let candidate = query_curve.add(&mut query_ops, a_g, b_q);
            let projected = query_curve.mul(&mut query_ops, candidate, 195);
            if projected.infinity {
                continue;
            }
            let pairs = if chi_relation {
                polynomial_pairs(
                    &source,
                    &signed,
                    &chi_polynomial,
                    projected,
                    &mut query_ops,
                    &mut stats,
                )
            } else {
                let mut out = BTreeSet::new();
                if let Some(entries) = table.get(&query_curve.key(&projected)) {
                    for &(i, j) in entries {
                        if query_curve.add(&mut query_ops, signed[i], signed[j]) == projected {
                            out.insert((i, j));
                        }
                    }
                }
                out
            };
            if let Some(&(i, j)) = pairs.first() {
                let row_cpu_start = thread_cpu_ns();
                let mut row = vec![0u64; columns + 2];
                for index in [i, j] {
                    let column = index / 2;
                    let coefficient = if index.is_multiple_of(2) { 1 } else { 228 };
                    row[column] = (row[column] + coefficient) % 229;
                }
                row[columns] = (229 - modmul(195, b)) % 229;
                row[columns + 1] = modmul(195, a);
                witnesses.push(json!({"a":a,"b":b,"i":i,"j":j,"projected":projected}));
                relations += 1;
                if insert_relation_grid(&mut pivots, row, columns) {
                    rank += 1;
                }
                row_cpu_ns += thread_cpu_ns() - row_cpu_start;
                if rank == columns + 1 {
                    break;
                }
            }
        }
        let recovery_cpu_start = thread_cpu_ns();
        recovered = recover_grid(&pivots, columns).map(|solution| solution[columns]);
        recovery_cpu_ns = thread_cpu_ns() - recovery_cpu_start;
    } else if arm == "source-bsgs" {
        let mut cursor = target;
        for giant in 0..15u64 {
            trials += 1;
            if let Some(&small) = baby.get(&source.key(&cursor)) {
                recovered = Some((giant * 16 + small) % 229);
                break;
            }
            cursor = source.add(&mut query_ops, cursor, giant_step);
        }
    } else {
        for retry in 0..32u64 {
            let try_seed = seed.wrapping_add(retry * 0x0100_0000);
            let result = rho_reference_negation(&source, generator, target, 229, try_seed, 4580);
            query_ops.merge(result.group_ops);
            rho_attempts.push(json!({"retry":retry,"seed":try_seed,"steps":result.steps,
                "walks":result.walks,"group_ops":result.group_ops,
                "recovered":result.recovered,"verified":result.verified}));
            if result.verified {
                recovered = result.recovered;
                break;
            }
        }
    }
    let query_cpu_ns = thread_cpu_ns() - query_cpu_start;
    assert!(query_cpu_ns >= row_cpu_ns + recovery_cpu_ns);
    let relation_cpu_ns = query_cpu_ns - row_cpu_ns - recovery_cpu_ns;
    let query_fp = take_field_counters();
    let verify_cpu_start = thread_cpu_ns();
    let mut verify_ops = GroupOps::default();
    let verified = recovered.is_some_and(|scalar| {
        source.mul(&mut verify_ops, generator, scalar) == target
            && destination.mul(&mut verify_ops, mapped_generator, scalar) == mapped_target
    });
    let verify_cpu_ns = thread_cpu_ns() - verify_cpu_start;
    let verify_fp = take_field_counters();
    let peak_rss = peak_rss_bytes();
    let total_cpu_ns = thread_cpu_ns() - cpu;
    assert!(total_cpu_ns >= setup_cpu_ns + query_cpu_ns + verify_cpu_ns);
    let other_cpu_ns = total_cpu_ns - setup_cpu_ns - query_cpu_ns - verify_cpu_ns;
    println!(
        "{}",
        json!({"schema":"isogeny_fiber_base_grid_run/v2",
            "base_public_sha256":base_sha,"heldout_public_sha256":heldout_sha,
            "round":round,"arm":arm,"columns":columns,"position":position,"seed":seed,
            "target":target,"mapped_target":mapped_target,
            "candidate_count":candidate_count,
            "positive_base":positive_base,
            "base_matches_frozen":base_matches_frozen,
            "setup_group_ops":setup_ops,"query_group_ops":query_ops,
            "verify_group_ops":verify_ops,
            "setup_fp_muls":setup_fp.0,"setup_fp_invs":setup_fp.1,
            "query_fp_muls":query_fp.0,"query_fp_invs":query_fp.1,
            "verify_fp_muls":verify_fp.0,"verify_fp_invs":verify_fp.1,
            "setup_cpu_ns":setup_cpu_ns,"query_cpu_ns":query_cpu_ns,
            "relation_cpu_ns":relation_cpu_ns,"row_cpu_ns":row_cpu_ns,
            "recovery_cpu_ns":recovery_cpu_ns,"verify_cpu_ns":verify_cpu_ns,
            "other_cpu_ns":other_cpu_ns,
            "total_cpu_ns":total_cpu_ns,"total_wall_ns":wall.elapsed().as_nanos(),
            "peak_rss_bytes":peak_rss,"trials":trials,"relations":relations,
            "rank":rank,"recovered":recovered,"verified":verified,
            "relation_witnesses":witnesses,"rho_attempts":rho_attempts,
            "chi_stats":{"resultants":stats.resultants,"gcds":stats.gcds,
                "split_attempts":stats.split_attempts,
                "exhaustive_fallback_evaluations":stats.exhaustive_fallback_evaluations,
                "root_candidates":stats.root_candidates,
                "chi_lift_scans":stats.chi_lift_scans,
                "exact_checks":stats.exact_checks,
                "false_candidates":stats.false_candidates,
                "vertical_cases":stats.vertical_cases,
                "tangent_cases":stats.tangent_cases,
                "horizontal_cases":stats.horizontal_cases,
                "infinity_cases":stats.infinity_cases,
                "exceptional_chi_cases":stats.exceptional_chi_cases,
                "multiple_resultant_roots":stats.multiple_resultant_roots},
            "inverse_stats":{"destination_points":inverse_stats.destination_points,
                "polynomial_calls":inverse_stats.polynomial_calls,
                "split_attempts":inverse_stats.split_attempts,
                "rational_x_roots":inverse_stats.rational_x_roots,
                "rejected_no_root":inverse_stats.rejected_no_root,
                "rejected_no_lift":inverse_stats.rejected_no_lift,
                "exact_map_checks":inverse_stats.exact_map_checks,
                "rejected_map_mismatch":inverse_stats.rejected_map_mismatch,
                "accepted_image_points":inverse_stats.accepted_image_points}})
    );
    assert!(verified && peak_rss <= 512 * 1024 * 1024);
}

fn group_count(value: &Value) -> u64 {
    value["adds"].as_u64().unwrap() + value["doubles"].as_u64().unwrap()
}

fn summarize_grid(run_dir: &str, svg_path: &str) {
    let arms = [
        "chi-inverse",
        "chi-full",
        "direct-inverse",
        "direct-full",
        "destination-direct",
        "source-bsgs",
        "rho-negation",
        "rho-negation-aa",
    ];
    let mut rows = Vec::new();
    let mut supports = Vec::new();
    let mut incomplete = Vec::new();
    for columns in [8usize, 16, 32, 64] {
        let exact_path = format!("{run_dir}/exhaustive/b{columns}.jsonl");
        let exact_bytes = fs::read(&exact_path).unwrap();
        let exact_summary: Value = exact_bytes
            .split(|&byte| byte == b'\n')
            .filter(|row| !row.is_empty())
            .last()
            .map(|row| serde_json::from_slice(row).unwrap())
            .unwrap();
        assert_eq!(
            exact_summary["schema"],
            "isogeny_fiber_base_grid_exhaustive/v1"
        );
        assert_eq!(exact_summary["columns"], columns);
        assert_eq!(
            fs::read_to_string(format!("{run_dir}/exhaustive/b{columns}.replay.exit"))
                .unwrap()
                .trim(),
            "0"
        );
        supports.push(json!({"columns":columns,
            "supported_residues":exact_summary["supported_residues"],
            "zero_residues":exact_summary["zero_residues"],
            "unordered_pairs":exact_summary["unordered_pairs"]}));
        for arm in arms {
            let mut group_s = Vec::new();
            let mut cpu_ms = Vec::new();
            let mut setup_cpu_ms = Vec::new();
            let mut relation_cpu_ms = Vec::new();
            let mut row_cpu_ms = Vec::new();
            let mut recovery_cpu_ms = Vec::new();
            let mut verify_cpu_ms = Vec::new();
            let mut other_cpu_ms = Vec::new();
            let mut trials = Vec::new();
            let mut relations = Vec::new();
            let mut ranks = Vec::new();
            let mut matched_bases = 0usize;
            let mut group_total = 0u64;
            let mut rho_group_total = 0u64;
            for round in 0..3 {
                for position in 0..4 {
                    let prefix =
                        format!("{run_dir}/children/r{round}-b{columns}-t{position}-{arm}");
                    let exit = fs::read_to_string(format!("{prefix}.exit")).unwrap();
                    let replay_exit = fs::read_to_string(format!("{prefix}.replay.exit")).unwrap();
                    if exit.trim() != "0" || replay_exit.trim() != "0" {
                        incomplete.push(json!({"columns":columns,"arm":arm,
                            "round":round,"position":position,
                            "exit":exit.trim(),"replay_exit":replay_exit.trim()}));
                        continue;
                    }
                    let child: Value =
                        serde_json::from_slice(&fs::read(format!("{prefix}.json")).unwrap())
                            .unwrap();
                    assert_eq!(child["verified"], true);
                    assert_eq!(child["arm"], arm);
                    assert_eq!(child["columns"], columns);
                    let operations = group_count(&child["setup_group_ops"])
                        + group_count(&child["query_group_ops"])
                        + group_count(&child["verify_group_ops"]);
                    let rho_prefix =
                        format!("{run_dir}/children/r{round}-b{columns}-t{position}-rho-negation");
                    let rho: Value =
                        serde_json::from_slice(&fs::read(format!("{rho_prefix}.json")).unwrap())
                            .unwrap();
                    let rho_operations = group_count(&rho["setup_group_ops"])
                        + group_count(&rho["query_group_ops"])
                        + group_count(&rho["verify_group_ops"]);
                    group_total += operations;
                    rho_group_total += rho_operations;
                    group_s.push(operations as f64 / (229f64).sqrt());
                    cpu_ms.push(child["total_cpu_ns"].as_u64().unwrap() as f64 / 1e6);
                    for (field, destination) in [
                        ("setup_cpu_ns", &mut setup_cpu_ms),
                        ("relation_cpu_ns", &mut relation_cpu_ms),
                        ("row_cpu_ns", &mut row_cpu_ms),
                        ("recovery_cpu_ns", &mut recovery_cpu_ms),
                        ("verify_cpu_ns", &mut verify_cpu_ms),
                        ("other_cpu_ns", &mut other_cpu_ms),
                    ] {
                        destination.push(child[field].as_u64().unwrap() as f64 / 1e6);
                    }
                    trials.push(child["trials"].as_u64().unwrap() as f64);
                    relations.push(child["relations"].as_u64().unwrap() as f64);
                    ranks.push(child["rank"].as_u64().unwrap() as f64);
                    matched_bases += usize::from(child["base_matches_frozen"] == true);
                }
            }
            let mean = |values: &[f64]| -> Option<f64> {
                if values.len() == 12 {
                    Some(values.iter().sum::<f64>() / 12.0)
                } else {
                    None
                }
            };
            let spread = |values: &[f64]| -> Option<[f64; 2]> {
                if values.len() == 12 {
                    Some([
                        values.iter().copied().fold(f64::INFINITY, f64::min),
                        values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                    ])
                } else {
                    None
                }
            };
            rows.push(
                json!({"columns":columns,"arm":arm,"completed":group_s.len(),
                "expected":12,"mean_s_group":mean(&group_s),
                "range_s_group":spread(&group_s),
                "counted_group_ratio_rho_over_arm":if group_s.len()==12 {
                    Some(rho_group_total as f64 / group_total as f64)
                } else {None},
                "mean_cpu_ms":mean(&cpu_ms),"range_cpu_ms":spread(&cpu_ms),
                "mean_setup_cpu_ms":mean(&setup_cpu_ms),
                "mean_relation_cpu_ms":mean(&relation_cpu_ms),
                "mean_row_cpu_ms":mean(&row_cpu_ms),
                "mean_recovery_cpu_ms":mean(&recovery_cpu_ms),
                "mean_verify_cpu_ms":mean(&verify_cpu_ms),
                "mean_other_cpu_ms":mean(&other_cpu_ms),
                "mean_trials":mean(&trials),
                "mean_relations":mean(&relations),"mean_rank":mean(&ranks),
                "matched_base_children":matched_bases,
                "complete_portable_s":Value::Null}),
            );
        }
    }
    let result = json!({"schema":"isogeny_fiber_base_grid_summary/v1",
        "r":229,"generic_two_class_floor_s":(std::f64::consts::PI/4.0).sqrt(),
        "supports":supports,"rows":rows,"incomplete_children":incomplete,
        "complete_portable_s":Value::Null,
        "cost_scope":"whole-child counted group additions and doublings only; field and other native work unpriced"});
    let mut svg = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1100\" height=\"760\" viewBox=\"0 0 1100 760\"><rect width=\"1100\" height=\"760\" fill=\"white\"/><text x=\"24\" y=\"32\" font-size=\"22\">Factor-base grid: cold counted-group work</text><text x=\"24\" y=\"54\" font-size=\"14\">Mean (range), 12 paired children per arm and B. S_group = adds + doubles per sqrt(229); portable price open.</text>",
    );
    for (base_index, support) in result["supports"].as_array().unwrap().iter().enumerate() {
        let columns = support["columns"].as_u64().unwrap();
        let supported = support["supported_residues"].as_u64().unwrap();
        let header_y = 82 + 168 * base_index;
        svg.push_str(&format!(
            "<text x=\"24\" y=\"{header_y}\" font-size=\"16\" font-weight=\"bold\">B={columns}, pair support {supported}/229</text>"
        ));
        for (arm_index, arm) in arms.iter().enumerate() {
            let row = &result["rows"][8 * base_index + arm_index];
            let y = header_y + 9 + 17 * (arm_index + 1);
            let color = if arm.starts_with("chi") {
                "#ea580c"
            } else if arm.contains("inverse") {
                "#0891b2"
            } else if arm == &"rho-negation" || arm == &"rho-negation-aa" {
                "#16a34a"
            } else {
                "#2563eb"
            };
            svg.push_str(&format!(
                "<text x=\"40\" y=\"{y}\" font-size=\"12\">{arm}</text>"
            ));
            if let Some(s) = row["mean_s_group"].as_f64() {
                let width = 140.0 * (1.0 + s).log10();
                let low = row["range_s_group"][0].as_f64().unwrap();
                let high = row["range_s_group"][1].as_f64().unwrap();
                svg.push_str(&format!(
                    "<rect x=\"220\" y=\"{}\" width=\"{width:.2}\" height=\"10\" fill=\"{color}\"/><text x=\"{:.2}\" y=\"{y}\" font-size=\"12\">{s:.1} ({low:.1}-{high:.1})</text>",
                    y - 10,
                    226.0 + width
                ));
            } else {
                svg.push_str(&format!(
                    "<text x=\"226\" y=\"{y}\" font-size=\"12\" fill=\"#6b7280\">incomplete or censored</text>"
                ));
            }
        }
    }
    svg.push_str("</svg>");
    fs::write(svg_path, svg).unwrap();
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}

fn audit_grid_v2(v1_dir: &str, v2_dir: &str) {
    let mut mismatches = Vec::new();
    let mut compared = 0u64;
    let mut witnesses = 0u64;
    let mut fallback_evaluations = 0u64;
    let v1_plan = fs::read(format!("{v1_dir}/RUN_PLAN_V1.tsv")).unwrap();
    let v2_plan = fs::read(format!("{v2_dir}/RUN_PLAN_V2.tsv")).unwrap();
    if v1_plan != v2_plan {
        mismatches.push("run plan differs".to_string());
    }
    if fs::read(format!("{v1_dir}/BASE_PUBLIC_V1.json")).unwrap()
        != fs::read(format!("{v2_dir}/BASE_PUBLIC_V1.json")).unwrap()
    {
        mismatches.push("frozen public base differs".to_string());
    }
    for columns in [8, 16, 32, 64] {
        let path = format!("exhaustive/b{columns}.jsonl");
        let read_rows = |dir: &str| -> Vec<Value> {
            fs::read_to_string(format!("{dir}/{path}"))
                .unwrap()
                .lines()
                .map(|line| {
                    let mut value: Value = serde_json::from_str(line).unwrap();
                    value.as_object_mut().unwrap().remove("wall_ns");
                    value
                })
                .collect()
        };
        if read_rows(v1_dir) != read_rows(v2_dir) {
            mismatches.push(format!("{path}: exhaustive records differ"));
        }
    }
    for line in String::from_utf8(v1_plan).unwrap().lines() {
        let parts: Vec<_> = line.split('\t').collect();
        assert_eq!(parts.len(), 4);
        let label = format!("r{}-b{}-t{}-{}", parts[0], parts[1], parts[2], parts[3]);
        let a = format!("{v1_dir}/children/{label}");
        let b = format!("{v2_dir}/children/{label}");
        if [
            format!("{a}.exit"),
            format!("{a}.replay.exit"),
            format!("{b}.exit"),
            format!("{b}.replay.exit"),
        ]
        .iter()
        .any(|path| fs::read_to_string(path).unwrap().trim() != "0")
        {
            mismatches.push(format!("{label}: child or replay exit nonzero"));
            continue;
        }
        let old: Value = serde_json::from_slice(&fs::read(format!("{a}.json")).unwrap()).unwrap();
        let new: Value = serde_json::from_slice(&fs::read(format!("{b}.json")).unwrap()).unwrap();
        if old["schema"] != "isogeny_fiber_base_grid_run/v1"
            || new["schema"] != "isogeny_fiber_base_grid_run/v2"
        {
            mismatches.push(format!("{label}: run schema differs"));
        }
        for key in [
            "arm",
            "columns",
            "position",
            "round",
            "seed",
            "target",
            "mapped_target",
            "base_public_sha256",
            "heldout_public_sha256",
            "candidate_count",
            "positive_base",
            "base_matches_frozen",
            "setup_group_ops",
            "query_group_ops",
            "verify_group_ops",
            "setup_fp_muls",
            "setup_fp_invs",
            "query_fp_muls",
            "query_fp_invs",
            "verify_fp_muls",
            "verify_fp_invs",
            "trials",
            "relations",
            "rank",
            "recovered",
            "verified",
            "relation_witnesses",
            "rho_attempts",
            "inverse_stats",
        ] {
            if old[key] != new[key] {
                mismatches.push(format!("{label}: {key} differs"));
            }
        }
        for (key, old_value) in old["chi_stats"].as_object().unwrap() {
            if old_value != &new["chi_stats"][key.as_str()] {
                mismatches.push(format!("{label}: chi_stats.{key} differs"));
            }
        }
        let setup = new["setup_cpu_ns"].as_u64().unwrap();
        let relation = new["relation_cpu_ns"].as_u64().unwrap();
        let row = new["row_cpu_ns"].as_u64().unwrap();
        let recovery = new["recovery_cpu_ns"].as_u64().unwrap();
        let verify = new["verify_cpu_ns"].as_u64().unwrap();
        let other = new["other_cpu_ns"].as_u64().unwrap();
        if relation + row + recovery != new["query_cpu_ns"].as_u64().unwrap()
            || setup + relation + row + recovery + verify + other
                != new["total_cpu_ns"].as_u64().unwrap()
        {
            mismatches.push(format!("{label}: CPU phase ledger does not sum"));
        }
        witnesses += new["relation_witnesses"].as_array().unwrap().len() as u64;
        fallback_evaluations += new["chi_stats"]["exhaustive_fallback_evaluations"]
            .as_u64()
            .unwrap();
        compared += 1;
    }
    let passed = mismatches.is_empty() && compared == 384;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"isogeny_fiber_base_grid_v2_audit/v1",
            "v1_plan_sha256":sha256_hex(&fs::read(format!("{v1_dir}/RUN_PLAN_V1.tsv")).unwrap()),
            "v2_plan_sha256":sha256_hex(&v2_plan),
            "compared_children":compared,"expected_children":384,
            "relation_witnesses":witnesses,
            "fallback_evaluations":fallback_evaluations,
            "mismatches":mismatches,"status":if passed {"pass"} else {"fail"}
        }))
        .unwrap()
    );
    if !passed {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("audit-grid-v2") {
        assert_eq!(args.len(), 4);
        audit_grid_v2(&args[2], &args[3]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("summarize-grid") {
        assert_eq!(args.len(), 4);
        summarize_grid(&args[2], &args[3]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("run-grid") {
        assert_eq!(args.len(), 8);
        run_grid(
            &args[2],
            &args[3],
            args[4].parse().unwrap(),
            &args[5],
            args[6].parse().unwrap(),
            args[7].parse().unwrap(),
        );
        return;
    }
    if args.get(1).map(String::as_str) == Some("exhaustive-grid") {
        assert_eq!(args.len(), 4);
        exhaustive_grid(&args[2], args[3].parse().unwrap());
        return;
    }
    if args.get(1).map(String::as_str) == Some("freeze-base") {
        assert_eq!(args.len(), 5);
        freeze_base(&args[2], &args[3], &args[4]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("freeze-targets") {
        assert_eq!(args.len(), 5);
        freeze_targets(&args[2], &args[3], &args[4]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("run-heldout") {
        assert_eq!(args.len(), 6);
        run_heldout(
            &args[2],
            args[3].parse().unwrap(),
            &args[4],
            args[5].parse().unwrap(),
        );
        return;
    }
    if args.get(1).map(String::as_str) == Some("exhaustive-s3") {
        assert_eq!(args.len(), 3);
        exhaustive_s3(&args[2]);
        return;
    }
    assert_eq!(args.get(1).map(String::as_str), Some("exhaustive"));
    let path = std::env::args().nth(2).expect("public input path");
    let bytes = fs::read(&path).unwrap();
    let public: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(public["p"], 211);
    assert_eq!(public["r"], 229);
    assert_eq!(public["cofactor"], 195);
    let f = Fp2::new(211).unwrap();
    let curve = Curve {
        f,
        a: [190, 0],
        b: [37, 0],
    };
    let generator = parse_point(&public["generator"]);
    let mut ops = GroupOps::default();
    assert!(curve.is_on_curve(generator));
    assert!(curve.mul(&mut ops, generator, 229).infinity);
    let mut signed = Vec::new();
    for witness in public["full_base_witness_source"].as_array().unwrap() {
        let raw = parse_point(witness);
        let projected = curve.mul(&mut ops, raw, 195);
        assert!(!projected.infinity && curve.mul(&mut ops, projected, 229).infinity);
        signed.push(projected);
        signed.push(curve.neg(projected));
    }
    assert_eq!(signed.len(), 16);
    let base = chi_base(&curve, &signed);
    take_field_counters();
    let start = Instant::now();
    let mut stats = Stats::default();
    let mut target = curve.identity();
    let mut checked = 0;
    let mut pair_count = 0;
    let mut zero_cases = 0;
    for scalar in 0..229 {
        let expected = direct_pairs(&curve, &signed, target, &mut ops);
        let actual = polynomial_pairs(&curve, &signed, &base, target, &mut ops, &mut stats);
        assert_eq!(
            actual, expected,
            "chi resultant mismatch at scalar {scalar}"
        );
        pair_count += actual.len();
        zero_cases += usize::from(actual.is_empty());
        checked += 1;
        println!(
            "{}",
            json!({"schema":"different_j_chi_resultant_case/v1","scalar_label":scalar,
            "target":target,"pairs":actual.iter().copied().collect::<Vec<_>>(),"status":"pass"})
        );
        target = curve.add(&mut ops, target, generator);
    }
    assert!(target.infinity);
    let torsion = Point::affine(f.from_fp(3), f.one());
    for (label, exceptional) in [
        ("torsion_plus", torsion),
        ("torsion_minus", curve.neg(torsion)),
    ] {
        let expected = direct_pairs(&curve, &signed, exceptional, &mut ops);
        let actual = polynomial_pairs(&curve, &signed, &base, exceptional, &mut ops, &mut stats);
        assert_eq!(actual, expected, "chi resultant mismatch at {label}");
        pair_count += actual.len();
        zero_cases += usize::from(actual.is_empty());
        checked += 1;
        println!(
            "{}",
            json!({"schema":"different_j_chi_resultant_case/v1","scalar_label":label,
            "target":exceptional,"pairs":actual.iter().copied().collect::<Vec<_>>(),"status":"pass"})
        );
    }
    let field = take_field_counters();
    println!(
        "{}",
        json!({"schema":"different_j_chi_resultant_summary/v1",
        "public_sha256":sha256_hex(&bytes),"p":211,"r":229,"factor_base_columns":8,
        "signed_points":signed.len(),"chi_polynomial_degree":base.len()-1,
        "cases":checked,"pair_count":pair_count,"zero_cases":zero_cases,
        "resultants":stats.resultants,"gcds":stats.gcds,"split_attempts":stats.split_attempts,
        "exhaustive_fallback_evaluations":stats.exhaustive_fallback_evaluations,
        "root_candidates":stats.root_candidates,"chi_lift_scans":stats.chi_lift_scans,
        "exact_checks":stats.exact_checks,"false_candidates":stats.false_candidates,
        "vertical_cases":stats.vertical_cases,"tangent_cases":stats.tangent_cases,
        "horizontal_cases":stats.horizontal_cases,"infinity_cases":stats.infinity_cases,
        "exceptional_chi_cases":stats.exceptional_chi_cases,
        "multiple_resultant_roots":stats.multiple_resultant_roots,
        "group_ops":ops,"fp_muls":field.0,"fp_invs":field.1,
        "wall_ns":start.elapsed().as_nanos(),"status":"pass"})
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chord_elimination_vanishes_on_group_sum() {
        let f = Fp2::new(211).unwrap();
        let curve = Curve {
            f,
            a: [190, 0],
            b: [37, 0],
        };
        let generator = Point::affine([170, 0], [0, 109]);
        let mut ops = GroupOps::default();
        let p1 = curve.mul(&mut ops, generator, 17);
        let p2 = curve.mul(&mut ops, generator, 73);
        let target = curve.add(&mut ops, p1, p2);
        let minus_target = curve.neg(target);
        let slope = f.mul(
            f.sub(minus_target.y, p1.y),
            f.inv(f.sub(minus_target.x, p1.x)).unwrap(),
        );
        assert_ne!(slope, f.zero());
        let z = chi(&curve, p2).unwrap();
        let h = chord_resultant(&curve, p1, slope);
        assert_eq!(eval(&f, &h, z), f.zero());
    }

    #[test]
    fn rational_root_split_returns_every_distinct_root() {
        let f = Fp2::new(211).unwrap();
        let roots = [[5, 1], [17, 3], [201, 40]];
        let polynomial = roots.iter().fold(vec![f.one()], |acc, &root| {
            mul(&f, &acc, &vec![f.neg(root), f.one()])
        });
        let mut stats = Stats::default();
        let actual = all_roots(&f, &polynomial, &mut stats);
        let mut expected = roots.to_vec();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }
}
