//! Independent affine group oracle for the signed three-summand contract.
//!
//! No producer field, polynomial, summation-polynomial, root, quotient-query or
//! extraction routine is called here. The oracle enumerates every ordered
//! first pair and determines the only possible third point by the group law.
use crate::extractor::{Extraction, Input};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub type Affine = [u64; 2];
pub type Point = Option<Affine>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cyclic {
    pub generator: Affine,
    pub order: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spec {
    pub id: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub d: usize,
    pub liftable: bool,
    pub target: Point,
    pub subgroup: Option<Cyclic>,
}

#[derive(Clone, Debug)]
pub struct Oracle {
    pub selector: Vec<u64>,
    pub x_domain: Vec<u64>,
    pub base: Vec<Affine>,
    pub triples: Vec<[usize; 3]>,
    pub prefix_identity: BTreeSet<[usize; 3]>,
    pub prefixes: BTreeMap<[usize; 3], Point>,
    pub permutation_groups: BTreeMap<[usize; 3], Vec<usize>>,
    pub row_groups: BTreeMap<Vec<i32>, Vec<usize>>,
    pub row_xs: Vec<u64>,
    pub target_column: i32,
    pub pair_checks: usize,
}

fn plus(p: u64, a: u64, b: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(p)) as u64
}
fn minus(p: u64, a: u64, b: u64) -> u64 {
    ((u128::from(a) + u128::from(p) - u128::from(b)) % u128::from(p)) as u64
}
fn times(p: u64, a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
}
fn power(p: u64, a: u64, n: u64) -> u64 {
    // Left-to-right powering is intentionally separate from producer helpers.
    let mut z = 1;
    if n == 0 {
        return z;
    }
    for bit in (0..(64 - n.leading_zeros())).rev() {
        z = times(p, z, z);
        if (n >> bit) & 1 != 0 {
            z = times(p, z, a);
        }
    }
    z
}
fn inverse(p: u64, a: u64) -> Result<u64, String> {
    let (mut old_r, mut r) = (i128::from(p), i128::from(a));
    let (mut old_t, mut t) = (0i128, 1i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_t, t) = (t, old_t - q * t);
    }
    if old_r != 1 {
        return Err("independent inverse: nonunit".into());
    }
    Ok(old_t.rem_euclid(i128::from(p)) as u64)
}
pub fn rhs(spec: &Spec, x: u64) -> u64 {
    plus(
        spec.p,
        plus(
            spec.p,
            times(spec.p, times(spec.p, x, x), x),
            times(spec.p, spec.a, x),
        ),
        spec.b,
    )
}
pub fn on_curve(spec: &Spec, q: Point) -> bool {
    match q {
        None => true,
        Some([x, y]) => x < spec.p && y < spec.p && times(spec.p, y, y) == rhs(spec, x),
    }
}
pub fn negate(spec: &Spec, q: Point) -> Point {
    q.map(|[x, y]| [x, if y == 0 { 0 } else { spec.p - y }])
}
pub fn add(spec: &Spec, left: Point, right: Point) -> Result<Point, String> {
    let (Some([x1, y1]), Some([x2, y2])) = (left, right) else {
        return Ok(left.or(right));
    };
    if x1 == x2 && plus(spec.p, y1, y2) == 0 {
        return Ok(None);
    }
    let slope = if x1 == x2 {
        if y1 != y2 {
            return Err("independent add: inconsistent same-x ordinates".into());
        }
        times(
            spec.p,
            plus(spec.p, times(spec.p, 3, times(spec.p, x1, x1)), spec.a),
            inverse(spec.p, times(spec.p, 2, y1))?,
        )
    } else {
        times(
            spec.p,
            minus(spec.p, y2, y1),
            inverse(spec.p, minus(spec.p, x2, x1))?,
        )
    };
    let x3 = minus(spec.p, minus(spec.p, times(spec.p, slope, slope), x1), x2);
    let y3 = minus(spec.p, times(spec.p, slope, minus(spec.p, x1, x3)), y1);
    Ok(Some([x3, y3]))
}

/// All distinct square roots, in increasing order; only the two admitted
/// prime fields are accepted, independently of any sidecar-supplied label.
pub fn square_roots(p: u64, v: u64) -> Result<Vec<u64>, String> {
    if v >= p || ![97, 2147483647].contains(&p) {
        return Err("independent square root: unsupported field/input".into());
    }
    if p == 97 {
        return Ok((0..p).filter(|&y| times(p, y, y) == v).collect());
    }
    if v == 0 {
        return Ok(vec![0]);
    }
    let y = power(p, v, (p + 1) / 4);
    if times(p, y, y) != v {
        return Ok(vec![]);
    }
    let mut ys = vec![y, p - y];
    ys.sort_unstable();
    ys.dedup();
    Ok(ys)
}

pub fn selector(spec: &Spec) -> Result<Vec<u64>, String> {
    if ![97, 2147483647].contains(&spec.p)
        || spec.d == 0
        || spec.d > 128
        || (spec.p - 1) % spec.d as u64 != 0
    {
        return Err("independent selector: unsupported field/degree".into());
    }
    // This bounded search is not a field scan. The complete inventory is
    // checked by distinctness and the monic polynomial's degree.
    for seed in 1..=1024u64.min(spec.p - 1) {
        let omega = power(spec.p, seed, (spec.p - 1) / spec.d as u64);
        let mut roots = BTreeSet::new();
        let mut x = 1;
        for _ in 0..spec.d {
            roots.insert(x);
            x = times(spec.p, x, omega);
        }
        if x == 1
            && roots.len() == spec.d
            && roots.iter().all(|&x| power(spec.p, x, spec.d as u64) == 1)
        {
            return Ok(roots.into_iter().collect());
        }
    }
    Err("independent selector: bounded generator search exhausted".into())
}

pub fn subgroup(spec: &Spec) -> Result<Option<BTreeSet<Affine>>, String> {
    let Some(group) = &spec.subgroup else {
        return Ok(None);
    };
    if !(2..=512).contains(&group.order) || !on_curve(spec, Some(group.generator)) {
        return Err("independent subgroup: invalid generator/order".into());
    }
    let mut points = BTreeSet::new();
    let mut current = None;
    for index in 1..=group.order {
        current = add(spec, current, Some(group.generator))?;
        match current {
            None if index == group.order => return Ok(Some(points)),
            None => return Err("independent subgroup: claimed order is not exact".into()),
            Some(point) => {
                if !points.insert(point) {
                    return Err("independent subgroup: repeated point before identity".into());
                }
            }
        }
    }
    Err("independent subgroup: claimed order does not annihilate generator".into())
}

pub fn oracle(spec: &Spec) -> Result<Oracle, String> {
    if spec.a >= spec.p || spec.b >= spec.p || !on_curve(spec, spec.target) {
        return Err("independent fixture: invalid curve/target".into());
    }
    let discr = plus(
        spec.p,
        times(
            spec.p,
            4,
            times(spec.p, times(spec.p, spec.a, spec.a), spec.a),
        ),
        times(spec.p, 27, times(spec.p, spec.b, spec.b)),
    );
    if discr == 0 {
        return Err("independent fixture: singular curve".into());
    }
    let selector = selector(spec)?;
    let subgroup = subgroup(spec)?;
    let mut base = Vec::new();
    for &x in &selector {
        let ys = square_roots(spec.p, rhs(spec, x))?;
        for y in ys {
            let point = [x, y];
            if subgroup
                .as_ref()
                .map_or(true, |points| points.contains(&point))
            {
                base.push(point);
            }
        }
    }
    base.sort_unstable();
    let x_domain = if spec.liftable {
        base.iter()
            .map(|point| point[0])
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    } else {
        selector.clone()
    };
    let lookup: BTreeMap<_, _> = base
        .iter()
        .enumerate()
        .map(|(id, &point)| (point, id))
        .collect();
    let mut triples = Vec::new();
    let mut prefix_identity = BTreeSet::new();
    let mut prefixes = BTreeMap::new();
    for (i, &left) in base.iter().enumerate() {
        for (j, &right) in base.iter().enumerate() {
            let prefix = add(spec, Some(left), Some(right))?;
            let third = add(spec, spec.target, negate(spec, prefix))?;
            if let Some(k) = third.and_then(|point| lookup.get(&point).copied()) {
                let triple = [i, j, k];
                if add(spec, prefix, Some(base[k]))? != spec.target {
                    return Err("independent oracle: group replay inconsistency".into());
                }
                if prefix.is_none() {
                    prefix_identity.insert(triple);
                }
                prefixes.insert(triple, prefix);
                triples.push(triple);
            }
        }
    }
    let row_xs: Vec<_> = base
        .iter()
        .map(|point| point[0])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let row_lookup: BTreeMap<_, _> = row_xs.iter().enumerate().map(|(id, &x)| (x, id)).collect();
    let target_in_base = spec.target.and_then(|point| lookup.get(&point).copied());
    let target_column = if spec.target.is_some() && target_in_base.is_none() {
        -1
    } else {
        0
    };
    let mut permutation_groups = BTreeMap::<[usize; 3], Vec<usize>>::new();
    let mut row_groups = BTreeMap::<Vec<i32>, Vec<usize>>::new();
    for (id, &triple) in triples.iter().enumerate() {
        let mut multiset = triple;
        multiset.sort_unstable();
        permutation_groups.entry(multiset).or_default().push(id);
        let mut row = vec![0i32; row_xs.len()];
        for point_id in triple {
            let [x, y] = base[point_id];
            row[row_lookup[&x]] += if y <= spec.p - y { 1 } else { -1 };
        }
        if let Some(point_id) = target_in_base {
            let [x, y] = base[point_id];
            row[row_lookup[&x]] -= if y <= spec.p - y { 1 } else { -1 };
        }
        row_groups.entry(row).or_default().push(id);
    }
    Ok(Oracle {
        selector,
        x_domain,
        base,
        pair_checks: lookup.len() * lookup.len(),
        triples,
        prefix_identity,
        prefixes,
        permutation_groups,
        row_groups,
        row_xs,
        target_column,
    })
}

pub fn from_input(input: &Input) -> Spec {
    Spec {
        id: input.case_id.clone(),
        p: input.p,
        a: input.a,
        b: input.b,
        d: input.d,
        liftable: input.restrict_liftable,
        target: input.target,
        subgroup: input.subgroup.as_ref().map(|s| Cyclic {
            generator: s.generator,
            order: s.order,
        }),
    }
}

fn canonical_target(p: u64, a: u64, b: u64, x: u64) -> Result<Affine, String> {
    let spec = Spec {
        id: String::new(),
        p,
        a,
        b,
        d: 1,
        liftable: false,
        target: None,
        subgroup: None,
    };
    let ys = square_roots(p, rhs(&spec, x))?;
    Ok([
        x,
        *ys.first()
            .ok_or("independent registry target is not rational")?,
    ])
}

/// Independent fixture registry. No producer header, sidecar label or output
/// count supplies its curve, target, subgroup, selector or persistence policy.
pub fn registry() -> Result<BTreeMap<String, Spec>, String> {
    let mut fixtures = BTreeMap::new();
    let families: [(&str, u64, u64, u64, &[usize], &[u64]); 3] = [
        ("f97", 97, 2, 3, &[1, 2, 3, 4, 6, 8, 12, 16], &[0, 1, 3, 30]),
        ("rank97", 97, 96, 0, &[2], &[0, 1, 2]),
        (
            "m31",
            2147483647,
            847514846,
            2087444724,
            &[7, 21, 63],
            &[0, 1, 2],
        ),
    ];
    for (family, p, a, b, degrees, xs) in families {
        for &d in degrees {
            for liftable in [false, true] {
                let suffix = if liftable { "liftable" } else { "all" };
                for &x in xs {
                    let id = format!("{family}-d{d}-x{x}-{suffix}");
                    fixtures.insert(
                        id.clone(),
                        Spec {
                            id,
                            p,
                            a,
                            b,
                            d,
                            liftable,
                            target: Some(canonical_target(p, a, b, x)?),
                            subgroup: None,
                        },
                    );
                }
                let id = format!("{family}-d{d}-O-{suffix}");
                fixtures.insert(
                    id.clone(),
                    Spec {
                        id,
                        p,
                        a,
                        b,
                        d,
                        liftable,
                        target: None,
                        subgroup: None,
                    },
                );
            }
        }
    }
    for (family, p, a, b, d) in [
        ("f97", 97, 2, 3, 2),
        ("m31", 2147483647, 847514846, 2087444724, 7),
    ] {
        let [x, y] = canonical_target(p, a, b, 1)?;
        if y == 0 {
            return Err("negative-target control must have two ordinates".into());
        }
        for liftable in [false, true] {
            let suffix = if liftable { "liftable" } else { "all" };
            let id = format!("{family}-d{d}-nx1-{suffix}");
            fixtures.insert(
                id.clone(),
                Spec {
                    id,
                    p,
                    a,
                    b,
                    d,
                    liftable,
                    target: Some([x, p - y]),
                    subgroup: None,
                },
            );
        }
    }
    for (tag, target) in [
        ("plus", Some([80, 10])),
        ("minus", Some([80, 87])),
        ("O", None),
        ("outside", Some([0, 10])),
    ] {
        for liftable in [false, true] {
            let suffix = if liftable { "liftable" } else { "all" };
            let id = format!("sub5-d96-{tag}-{suffix}");
            fixtures.insert(
                id.clone(),
                Spec {
                    id,
                    p: 97,
                    a: 2,
                    b: 3,
                    d: 96,
                    liftable,
                    target,
                    subgroup: Some(Cyclic {
                        generator: [3, 6],
                        order: 5,
                    }),
                },
            );
        }
    }
    for (tag, target) in [
        ("plus", Some([1, 14])),
        ("minus", Some([1, 83])),
        ("O", None),
        ("torsion", Some([0, 0])),
    ] {
        for liftable in [false, true] {
            let suffix = if liftable { "liftable" } else { "all" };
            let id = format!("order4-d1-{tag}-{suffix}");
            fixtures.insert(
                id.clone(),
                Spec {
                    id,
                    p: 97,
                    a: 1,
                    b: 0,
                    d: 1,
                    liftable,
                    target,
                    subgroup: None,
                },
            );
        }
    }
    if fixtures.len() != 132 {
        return Err("independent registry cardinality mismatch".into());
    }
    Ok(fixtures)
}

pub fn persisted(spec: &Spec) -> bool {
    (spec.id.starts_with("f97-") && spec.d == 2)
        || spec.id.starts_with("rank97-")
        || (spec.id.starts_with("m31-") && spec.d == 7)
        || spec.id.starts_with("order4-")
        || (spec.id.starts_with("sub5-") && spec.liftable)
}

fn polynomial_from_roots(p: u64, roots: &[u64]) -> Vec<u64> {
    let mut polynomial = vec![1];
    for &root in roots {
        let mut next = vec![0; polynomial.len() + 1];
        for (degree, &coefficient) in polynomial.iter().enumerate() {
            next[degree] = minus(p, next[degree], times(p, root, coefficient));
            next[degree + 1] = plus(p, next[degree + 1], coefficient);
        }
        polynomial = next;
    }
    polynomial
}

fn evaluate(p: u64, coefficients: &[u64], x: u64) -> u64 {
    let mut value = 0;
    for &coefficient in coefficients.iter().rev() {
        value = plus(p, times(p, value, x), coefficient);
    }
    value
}
fn tensor_value(p: u64, coefficients: &[Vec<u64>], x: u64, t: u64) -> u64 {
    let mut value = 0;
    for coefficient in coefficients.iter().rev() {
        value = plus(p, times(p, value, x), evaluate(p, coefficient, t));
    }
    value
}

/// Fully symmetric S3 expansion, independent of the producer's quadratic
/// coefficient formula and of the direct point oracle above.
pub fn s3(spec: &Spec, x: u64, y: u64, z: u64) -> u64 {
    let p = spec.p;
    let xy = times(p, x, y);
    let xz = times(p, x, z);
    let yz = times(p, y, z);
    let sum = plus(p, plus(p, x, y), z);
    let pair = plus(p, plus(p, xy, xz), yz);
    let square = plus(
        p,
        plus(p, times(p, xy, xy), times(p, xz, xz)),
        times(p, yz, yz),
    );
    let cubic = times(p, 2, times(p, times(p, xy, z), sum));
    plus(
        p,
        minus(
            p,
            minus(
                p,
                minus(p, square, cubic),
                times(p, 2, times(p, spec.a, pair)),
            ),
            times(p, 4, times(p, spec.b, sum)),
        ),
        times(p, spec.a, spec.a),
    )
}

fn quadratic_roots_from_values(spec: &Spec, u: u64, v: u64) -> Result<Vec<u64>, String> {
    let p = spec.p;
    let constant = s3(spec, u, v, 0);
    let at_one = s3(spec, u, v, 1);
    let at_minus_one = s3(spec, u, v, p - 1);
    let half = inverse(p, 2)?;
    let linear = times(p, minus(p, at_one, at_minus_one), half);
    let quadratic = times(
        p,
        minus(
            p,
            plus(p, at_one, at_minus_one),
            plus(p, constant, constant),
        ),
        half,
    );
    if quadratic == 0 {
        if linear == 0 {
            if constant == 0 {
                return Err("independent S3: identically zero specialized factor".into());
            }
            return Ok(vec![]);
        }
        return Ok(vec![times(p, minus(p, 0, constant), inverse(p, linear)?)]);
    }
    let discriminant = minus(
        p,
        times(p, linear, linear),
        times(p, 4, times(p, constant, quadratic)),
    );
    let denominator = inverse(p, plus(p, quadratic, quadratic))?;
    let mut roots: Vec<_> = square_roots(p, discriminant)?
        .into_iter()
        .map(|root| times(p, minus(p, root, linear), denominator))
        .collect();
    roots.sort_unstable();
    roots.dedup();
    if roots.iter().any(|&root| s3(spec, u, v, root) != 0) {
        return Err("independent quadratic root replay".into());
    }
    Ok(roots)
}

/// Complete mathematical comparison for one fixed input. Runtime arithmetic
/// counters remain diagnostics; the structural/output counters are checked.
pub fn compare(input: &Input, output: &Extraction) -> Result<Value, String> {
    let spec = from_input(input);
    let expected = registry()?;
    if expected.get(&spec.id) != Some(&spec) {
        return Err("input differs from independent fixed registry".into());
    }
    if output.case_id != spec.id {
        return Err("output case identity differs".into());
    }
    let oracle = oracle(&spec)?;
    if output.selector != oracle.selector
        || output.x_roots != oracle.x_domain
        || output.base != oracle.base
    {
        return Err(
            "selector, F roots or signed base differs from independent construction".into(),
        );
    }
    let members: Vec<_> = subgroup(&spec)?.unwrap_or_default().into_iter().collect();
    if output.subgroup_points != members {
        return Err("exact cyclic subgroup inventory differs".into());
    }
    let ts: Vec<u64> = if let Some([target_x, _]) = spec.target {
        let mut roots = BTreeSet::new();
        for &z in &oracle.selector {
            roots.extend(quadratic_roots_from_values(&spec, z, target_x)?);
        }
        roots.into_iter().collect()
    } else {
        oracle.row_xs.clone()
    };
    if output.t_roots != ts {
        return Err("target-coordinate inventory differs".into());
    }
    let certificate = &output.certificate;
    if certificate.p != spec.p
        || certificate.f != polynomial_from_roots(spec.p, &oracle.x_domain)
        || certificate.g != polynomial_from_roots(spec.p, &ts)
    {
        return Err("quotient certificate input field/moduli differ".into());
    }
    let verified = crate::query_independent::verify_grid(certificate, &oracle.x_domain, &ts)?;
    let mut support = BTreeMap::<(u64, u64), Vec<u64>>::new();
    for &x in &oracle.x_domain {
        for &t in &ts {
            let mut direct = 1;
            let mut witnesses = Vec::new();
            for &z in &oracle.selector {
                let factor = s3(&spec, x, z, t);
                direct = times(spec.p, direct, factor);
                if factor == 0 {
                    witnesses.push(z);
                }
            }
            if tensor_value(spec.p, &certificate.h, x, t) != direct {
                return Err("quotient H input binding fails".into());
            }
            if !witnesses.is_empty() {
                support.insert((x, t), witnesses);
            }
        }
    }
    if support.len() != certificate.support_count || output.left.len() != support.len() {
        return Err("complete left support inventory differs".into());
    }
    let mut left_indices = BTreeMap::new();
    for (index, (bucket, (&(x, t), roots))) in output.left.iter().zip(&support).enumerate() {
        if bucket.x != x
            || bucket.t != t
            || bucket.roots != *roots
            || bucket.gcd != polynomial_from_roots(spec.p, roots)
        {
            return Err("left collision bucket or eliminated gcd differs".into());
        }
        left_indices.insert((x, t), index);
    }
    let routed_ts: Vec<_> = support
        .keys()
        .map(|&(_, t)| t)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if output.right.len() != routed_ts.len() {
        return Err("right bucket coverage differs".into());
    }
    let mut right_indices = BTreeMap::new();
    for (index, (bucket, &t)) in output.right.iter().zip(&routed_ts).enumerate() {
        let roots: Vec<_> = if let Some([x, _]) = spec.target {
            oracle
                .selector
                .iter()
                .copied()
                .filter(|&z| s3(&spec, t, z, x) == 0)
                .collect()
        } else {
            vec![t]
        };
        if bucket.t != t
            || bucket.roots != roots
            || bucket.gcd != polynomial_from_roots(spec.p, &roots)
        {
            return Err("right collision bucket or eliminated gcd differs".into());
        }
        right_indices.insert(t, index);
    }
    if output.triples != oracle.triples {
        return Err("ordered signed triple set is not complete and exact".into());
    }
    if output.routes.len() != oracle.triples.len() {
        return Err("route count differs from triples".into());
    }
    let mut used_left = BTreeSet::new();
    let mut inverse_pairs = 0usize;
    let mut repeated = 0usize;
    let mut torsion_prefix = 0usize;
    let mut masked = 0usize;
    let mut masked_noncancelling = 0usize;
    for (&triple, route) in oracle.triples.iter().zip(&output.routes) {
        let points = triple.map(|id| oracle.base[id]);
        let prefix = oracle.prefixes[&triple];
        let cancellation =
            (0..3).any(|i| (i + 1..3).any(|j| Some(points[i]) == negate(&spec, Some(points[j]))));
        inverse_pairs += usize::from(cancellation);
        repeated +=
            usize::from(triple[0] == triple[1] || triple[0] == triple[2] || triple[1] == triple[2]);
        if let Some([t, y]) = prefix {
            let left = left_indices[&(points[0][0], t)];
            let right = right_indices[&t];
            if route.chart != 0
                || route.t != Some(t)
                || route.left != Some(left)
                || route.right != Some(right)
            {
                return Err("finite-prefix route differs from original group sum".into());
            }
            used_left.insert(left);
            torsion_prefix += usize::from(y == 0);
            if let Some([target_x, _]) = spec.target {
                let mask = oracle.selector.binary_search(&target_x).is_ok()
                    && (points[0][0] == target_x || s3(&spec, points[0][0], target_x, t) == 0);
                masked += usize::from(mask);
                masked_noncancelling += usize::from(mask && !cancellation);
            }
        } else if route.chart != 1
            || route.t.is_some()
            || route.left.is_some()
            || route.right.is_some()
        {
            return Err("identity-prefix route differs from original group sum".into());
        }
    }
    if output.permutations.len() != oracle.permutation_groups.len() {
        return Err("permutation-group count differs".into());
    }
    for (group, (points, witnesses)) in output.permutations.iter().zip(&oracle.permutation_groups) {
        if &group.points != points || &group.witnesses != witnesses {
            return Err("permutation key/backpointers differ".into());
        }
    }
    if output.rows.len() != oracle.row_groups.len() {
        return Err("integer-row count differs".into());
    }
    for (group, (coefficients, witnesses)) in output.rows.iter().zip(&oracle.row_groups) {
        if &group.coefficients != coefficients
            || &group.witnesses != witnesses
            || group.target != oracle.target_column
        {
            return Err("integer row or collision backpointers differ".into());
        }
    }
    let stats = &output.stats;
    if stats.affine_triples != oracle.triples.len() - oracle.prefix_identity.len()
        || stats.identity_prefix_triples != oracle.prefix_identity.len()
        || stats.inverse_pair_triples != inverse_pairs
        || stats.repeated_point_triples != repeated
        || stats.two_torsion_prefix_triples != torsion_prefix
        || stats.masked_affine_triples != masked
        || stats.masked_noncancelling_triples != masked_noncancelling
        || stats.support_cells_without_signed_output != support.len() - used_left.len()
    {
        return Err("semantic extraction statistics differ".into());
    }
    if spec.id.starts_with("order4-d1-minus-") {
        let positive = oracle
            .base
            .binary_search(&[1, 14])
            .map_err(|_| "missing order4 positive point")?;
        let negative = oracle
            .base
            .binary_search(&[1, 83])
            .map_err(|_| "missing order4 negative point")?;
        for triple in [
            [positive, positive, positive],
            [negative, negative, positive],
            [negative, positive, negative],
        ] {
            if output.triples.binary_search(&triple).is_err() {
                return Err("order4 cancellation-mask control lost a signed witness".into());
            }
        }
        if masked_noncancelling == 0 {
            return Err("order4 noncancelling mask control not exercised".into());
        }
    }
    Ok(
        json!({"pairs":oracle.pair_checks,"grid":verified.cells,"identities":3*certificate.leaves.len(),
        "triples":oracle.triples.len(),"prefixO":oracle.prefix_identity.len(),"masked_noncc":masked_noncancelling}),
    )
}
