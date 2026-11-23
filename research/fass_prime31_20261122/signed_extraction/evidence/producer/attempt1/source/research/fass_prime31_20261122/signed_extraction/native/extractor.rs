//! Complete ordered three-summand extraction from a certified finite grid.
//!
//! The full power selector is a superset of the signed factor base. Every
//! algebraic witness is retained until point lifting, subgroup membership,
//! consistent signs and direct group replay have been checked. Identity
//! prefixes and the identity target are distinct, complete charts.
use crate::semaev_prime_poly::{poly_gcd, poly_powmod, poly_rem, trim, Curve, Fp};
use crate::{kernel, norm, prior_oracle};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub type Point = [u64; 2];
pub type ProjectivePoint = Option<Point>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SubgroupSpec {
    pub generator: Point,
    pub order: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub case_id: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub d: usize,
    pub target: ProjectivePoint,
    /// Restrict F to actual admissible base coordinates: rational lifts and,
    /// when requested, membership in the exact explicitly generated subgroup.
    /// False retains F=L as an algebraic superset.
    pub restrict_liftable: bool,
    pub subgroup: Option<SubgroupSpec>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LeftBucket {
    pub x: u64,
    pub t: u64,
    pub gcd: Vec<u64>,
    pub roots: Vec<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RightBucket {
    pub t: u64,
    pub gcd: Vec<u64>,
    pub roots: Vec<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Route {
    /// 0: finite first-pair sum; 1: identity first-pair sum.
    pub chart: u8,
    pub t: Option<u64>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PermutationGroup {
    pub points: [usize; 3],
    pub witnesses: Vec<usize>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RowGroup {
    /// One integer coefficient for each distinct base x, sorted increasingly.
    /// Canonical y=min(y,p-y) has sign +1; y=0 occurs only once.
    pub coefficients: Vec<i32>,
    /// -1 for a finite target outside the base, otherwise zero. A target in
    /// the base is subtracted from its own signed base column.
    pub target: i32,
    pub witnesses: Vec<usize>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExtractionStats {
    pub selector_search_candidates: usize,
    pub selector_pow_calls: usize,
    pub selector_multiplications: usize,
    pub square_root_calls: usize,
    pub subgroup_additions: usize,
    pub subgroup_base_checks: usize,
    pub base_index_entries: usize,
    pub base_x_bucket_lookups: usize,
    pub right_quadratics: usize,
    pub right_polynomial_products: usize,
    pub right_radical_queries: usize,
    pub right_candidate_roots: usize,
    pub norm_strategy: String,
    pub tensor_ring_multiplications: usize,
    pub tensor_largest_reduced_slots: usize,
    pub scalar_norm_factors: usize,
    pub positive_block_root_tests: usize,
    pub positive_fibre_root_tests: usize,
    pub left_gcd_queries: usize,
    pub right_gcd_queries: usize,
    pub witness_root_calls: usize,
    pub zero_quadratic_branches: usize,
    pub left_lift_pair_attempts: usize,
    pub prefix_additions: usize,
    pub residual_subtractions: usize,
    pub base_membership_queries: usize,
    pub replay_additions: usize,
    pub dedup_checks: usize,
    pub coefficient_updates: usize,
    pub affine_triples: usize,
    pub identity_prefix_triples: usize,
    pub inverse_pair_triples: usize,
    pub repeated_point_triples: usize,
    pub two_torsion_prefix_triples: usize,
    pub masked_affine_triples: usize,
    pub masked_noncancelling_triples: usize,
    pub support_cells_without_signed_output: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Extraction {
    pub case_id: String,
    pub selector: Vec<u64>,
    pub x_roots: Vec<u64>,
    pub t_roots: Vec<u64>,
    pub base: Vec<Point>,
    /// Finite points of the exact configured cyclic subgroup. Empty in the
    /// unrestricted full-curve mode; identity is implicit in subgroup mode.
    pub subgroup_points: Vec<Point>,
    pub certificate: kernel::Certificate,
    pub left: Vec<LeftBucket>,
    pub right: Vec<RightBucket>,
    pub triples: Vec<[usize; 3]>,
    pub routes: Vec<Route>,
    pub permutations: Vec<PermutationGroup>,
    pub rows: Vec<RowGroup>,
    pub stats: ExtractionStats,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum ExtractionError {
    UnsupportedField,
    InvalidCurve,
    InvalidSelector,
    InvalidTarget,
    InvalidSubgroup,
    SelectorSearchExhausted,
    InvalidPolynomial,
    Internal(&'static str),
    Query(String),
}

pub fn point_on_curve(c: Curve, q: Point) -> bool {
    let f = c.f;
    q[0] < f.p
        && q[1] < f.p
        && f.mul(q[1], q[1]) == f.add(f.add(f.mul(f.mul(q[0], q[0]), q[0]), f.mul(c.a, q[0])), c.b)
}

pub fn negate(f: Fp, q: ProjectivePoint) -> ProjectivePoint {
    q.map(|[x, y]| [x, f.neg(y)])
}

/// Ordinary affine group law, with the identity, inverse points and y=0
/// handled before inversion. Inputs have already passed curve validation.
pub fn add(c: Curve, p: ProjectivePoint, q: ProjectivePoint) -> ProjectivePoint {
    let (Some([x1, y1]), Some([x2, y2])) = (p, q) else {
        return p.or(q);
    };
    let f = c.f;
    if x1 == x2 && f.add(y1, y2) == 0 {
        return None;
    }
    let slope = if x1 == x2 {
        f.mul(f.add(f.mul(3, f.mul(x1, x1)), c.a), f.inv(f.mul(2, y1)))
    } else {
        f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)))
    };
    let x3 = f.sub(f.sub(f.mul(slope, slope), x1), x2);
    let y3 = f.sub(f.mul(slope, f.sub(x1, x3)), y1);
    Some([x3, y3])
}

pub fn eval(f: Fp, polynomial: &[u64], x: u64) -> u64 {
    polynomial
        .iter()
        .rev()
        .fold(0, |a, &b| f.add(f.mul(a, x), b))
}
fn roots_product(f: Fp, roots: &[u64]) -> Vec<u64> {
    roots
        .iter()
        .fold(vec![1], |a, &r| kernel::multiply(f, &a, &[f.neg(r), 1]))
}
fn square_root(f: Fp, a: u64, stats: &mut ExtractionStats) -> Option<u64> {
    stats.square_root_calls += 1;
    if f.p == 97 {
        (0..97).find(|&y| f.mul(y, y) == a)
    } else {
        let y = f.pow(a, (f.p + 1) / 4);
        (f.mul(y, y) == a).then_some(y.min(f.neg(y)))
    }
}
fn lifts(c: Curve, x: u64, stats: &mut ExtractionStats) -> Vec<Point> {
    let f = c.f;
    let rhs = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(c.a, x)), c.b);
    let Some(y) = square_root(f, rhs, stats) else {
        return vec![];
    };
    if y == 0 {
        vec![[x, 0]]
    } else {
        vec![[x, y], [x, f.neg(y)]]
    }
}
fn selector(c: Curve, d: usize, stats: &mut ExtractionStats) -> Result<Vec<u64>, ExtractionError> {
    if d == 1 {
        return Ok(vec![1]);
    }
    let f = c.f;
    let factors = prior_oracle::prime_divisors(d as u64);
    let mut omega = None;
    for candidate in 2..1024 {
        stats.selector_search_candidates += 1;
        stats.selector_pow_calls += 1;
        let w = f.pow(candidate % f.p, (f.p - 1) / d as u64);
        if w == 0 {
            continue;
        }
        let primitive = factors.iter().all(|&q| {
            stats.selector_pow_calls += 1;
            f.pow(w, d as u64 / q) != 1
        });
        if primitive {
            omega = Some(w);
            break;
        }
    }
    let omega = omega.ok_or(ExtractionError::SelectorSearchExhausted)?;
    let mut roots = Vec::with_capacity(d);
    let mut x = 1;
    for _ in 0..d {
        roots.push(x);
        x = f.mul(x, omega);
        stats.selector_multiplications += 1;
    }
    roots.sort_unstable();
    roots.dedup();
    if roots.len() != d || x != 1 {
        return Err(ExtractionError::Internal("selector root coverage"));
    }
    Ok(roots)
}
fn exact_subgroup(
    c: Curve,
    spec: &Option<SubgroupSpec>,
    stats: &mut ExtractionStats,
) -> Result<Vec<Point>, ExtractionError> {
    let Some(spec) = spec else {
        return Ok(vec![]);
    };
    if !(2..=512).contains(&spec.order) || !point_on_curve(c, spec.generator) {
        return Err(ExtractionError::InvalidSubgroup);
    }
    let mut point = None;
    let mut members = Vec::new();
    for i in 1..=spec.order {
        point = add(c, point, Some(spec.generator));
        stats.subgroup_additions += 1;
        match point {
            Some(point) if i < spec.order => members.push(point),
            None if i == spec.order => (),
            _ => return Err(ExtractionError::InvalidSubgroup),
        }
    }
    members.sort_unstable();
    members.dedup();
    if members.len() + 1 != spec.order {
        return Err(ExtractionError::InvalidSubgroup);
    }
    Ok(members)
}
/// S3(u,v,z) as an actual-degree quadratic, including linear and constant
/// specializations. Nonsingularity precludes an all-zero specialization, but
/// the generic witness helper still handles it without division.
fn quadratic(c: Curve, u: u64, v: u64) -> Vec<u64> {
    let f = c.f;
    let uv = f.mul(u, v);
    let sum = f.add(u, v);
    let difference = f.sub(u, v);
    let uv_minus_a = f.sub(uv, c.a);
    trim(vec![
        f.sub(f.mul(uv_minus_a, uv_minus_a), f.mul(4, f.mul(c.b, sum))),
        f.neg(f.mul(2, f.add(f.mul(sum, f.add(uv, c.a)), f.mul(2, c.b)))),
        f.mul(difference, difference),
    ])
}
fn quadratic_roots(
    f: Fp,
    q: &[u64],
    stats: &mut ExtractionStats,
) -> Result<Option<Vec<u64>>, ExtractionError> {
    if q == [0] {
        return Ok(None);
    }
    let mut roots = match q.len() {
        1 => vec![],
        2 => vec![f.mul(f.neg(q[0]), f.inv(q[1]))],
        3 => {
            let delta = f.sub(f.mul(q[1], q[1]), f.mul(4, f.mul(q[0], q[2])));
            if let Some(root) = square_root(f, delta, stats) {
                let inverse = f.inv(f.mul(2, q[2]));
                vec![
                    f.mul(f.add(f.neg(q[1]), root), inverse),
                    f.mul(f.sub(f.neg(q[1]), root), inverse),
                ]
            } else {
                vec![]
            }
        }
        _ => return Err(ExtractionError::InvalidPolynomial),
    };
    roots.sort_unstable();
    roots.dedup();
    Ok(Some(roots))
}
fn witnesses(
    c: Curve,
    l: &[u64],
    selector: &[u64],
    u: u64,
    v: u64,
    stats: &mut ExtractionStats,
) -> Result<(Vec<u64>, Vec<u64>), ExtractionError> {
    let q = quadratic(c, u, v);
    let gcd = kernel::monic(c.f, &poly_gcd(c.f, l, &q));
    stats.witness_root_calls += 1;
    let roots = match quadratic_roots(c.f, &q, stats)? {
        Some(roots) => roots
            .into_iter()
            .filter(|&z| eval(c.f, &gcd, z) == 0)
            .collect::<Vec<_>>(),
        None => {
            stats.zero_quadratic_branches += 1;
            selector.to_vec()
        }
    };
    if roots.len() + 1 != gcd.len() {
        return Err(ExtractionError::Internal(
            "eliminated witness root coverage",
        ));
    }
    Ok((gcd, roots))
}
fn right_message(
    c: Curve,
    selector: &[u64],
    target_x: u64,
    stats: &mut ExtractionStats,
) -> Result<(Vec<u64>, Vec<u64>), ExtractionError> {
    let mut raw = vec![1];
    let mut roots = BTreeSet::new();
    for &z in selector {
        let q = quadratic(c, z, target_x);
        stats.right_quadratics += 1;
        let candidates = quadratic_roots(c.f, &q, stats)?
            .ok_or(ExtractionError::Internal("all-field right factor"))?;
        stats.right_candidate_roots += candidates.len();
        roots.extend(candidates);
        raw = kernel::multiply(c.f, &raw, &q);
        stats.right_polynomial_products += 1;
    }
    stats.right_radical_queries += 1;
    let g = if raw.len() == 1 {
        vec![1]
    } else {
        let power = poly_powmod(c.f, &[0, 1], c.f.p, &raw);
        let variable = poly_rem(c.f, &[0, 1], &raw);
        kernel::monic(
            c.f,
            &poly_gcd(c.f, &raw, &kernel::sub(c.f, &power, &variable)),
        )
    };
    let roots: Vec<_> = roots.into_iter().collect();
    if roots_product(c.f, &roots) != g {
        return Err(ExtractionError::Internal("right rational radical coverage"));
    }
    Ok((g, roots))
}
fn insert_triple(
    c: Curve,
    base: &[Point],
    target: ProjectivePoint,
    triple: [usize; 3],
    route: Route,
    tuples: &mut BTreeMap<[usize; 3], Route>,
    stats: &mut ExtractionStats,
) -> Result<(), ExtractionError> {
    stats.replay_additions += 2;
    if add(
        c,
        add(c, Some(base[triple[0]]), Some(base[triple[1]])),
        Some(base[triple[2]]),
    ) != target
    {
        return Err(ExtractionError::Internal("direct group replay failed"));
    }
    stats.dedup_checks += 1;
    if let Some(old) = tuples.insert(triple, route.clone()) {
        if old != route {
            return Err(ExtractionError::Internal("inconsistent duplicate route"));
        }
    }
    Ok(())
}

/// Exact output for the declared base. Resource exhaustion is an error or
/// stopped invocation, never a negative signed-support result.
pub fn extract(input: &Input) -> Result<Extraction, ExtractionError> {
    if input.p != 97 && input.p != 2147483647 {
        return Err(ExtractionError::UnsupportedField);
    }
    if input.a >= input.p || input.b >= input.p {
        return Err(ExtractionError::InvalidCurve);
    }
    let c = Curve::new(input.p, input.a, input.b);
    if !prior_oracle::discriminant_nonzero(c) {
        return Err(ExtractionError::InvalidCurve);
    }
    if input.d == 0 || input.d > 128 || (input.p - 1) % input.d as u64 != 0 {
        return Err(ExtractionError::InvalidSelector);
    }
    if input.target.is_some_and(|q| !point_on_curve(c, q)) {
        return Err(ExtractionError::InvalidTarget);
    }
    let mut stats = ExtractionStats::default();
    let subgroup_points = exact_subgroup(c, &input.subgroup, &mut stats)?;
    let selector = selector(c, input.d, &mut stats)?;
    let mut base = Vec::new();
    for &x in &selector {
        for point in lifts(c, x, &mut stats) {
            if input.subgroup.is_some() {
                stats.subgroup_base_checks += 1;
                if subgroup_points.binary_search(&point).is_err() {
                    continue;
                }
            }
            base.push(point);
        }
    }
    base.sort_unstable();
    base.dedup();
    let mut base_by_x = BTreeMap::<u64, Vec<usize>>::new();
    for (index, point) in base.iter().enumerate() {
        base_by_x.entry(point[0]).or_default().push(index);
        stats.base_index_entries += 1;
    }
    let base_x: Vec<_> = base
        .iter()
        .map(|q| q[0])
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let x_roots = if input.restrict_liftable {
        base_x.clone()
    } else {
        selector.clone()
    };
    let fpoly = roots_product(c.f, &x_roots);
    let mut l = vec![0; input.d + 1];
    l[0] = c.f.p - 1;
    l[input.d] = 1;
    let (g, t_roots) = if let Some(target) = input.target {
        right_message(c, &selector, target[0], &mut stats)?
    } else {
        // For Q=O, P3=-(P1+P2); its x is necessarily a base coordinate.
        // The right domain is the exact admissible base, even when the left
        // F is the larger full-selector polynomial.
        (roots_product(c.f, &base_x), base_x.clone())
    };
    let h = if input.d <= 21 {
        stats.norm_strategy = "tensor_lucas".into();
        let (h, n) = norm::tensor_norm(c, input.d, &fpoly, &g);
        stats.tensor_ring_multiplications = n.ring_multiplications;
        stats.tensor_largest_reduced_slots = n.largest_tensor_slots;
        h
    } else {
        stats.norm_strategy = "scalar_selector_product".into();
        stats.scalar_norm_factors = selector.len();
        let raw = prior_oracle::partial_norm(c, input.d, &selector);
        kernel::reduce_tensor(c.f, &raw, &fpoly, &g)
    };
    let certificate =
        kernel::solve(c.f, &fpoly, &g, &h).map_err(|e| ExtractionError::Query(format!("{e:?}")))?;
    let mut support = BTreeSet::new();
    for leaf in &certificate.leaves {
        if leaf.gcd.len() <= 1 {
            continue;
        }
        let mut block_roots = 0;
        for &t in &t_roots {
            stats.positive_block_root_tests += 1;
            if eval(c.f, &leaf.modulus, t) != 0 {
                continue;
            }
            block_roots += 1;
            let specialized: Vec<_> = leaf
                .gcd
                .iter()
                .map(|coefficient| eval(c.f, coefficient, t))
                .collect();
            let mut fibre_roots = 0;
            for &x in &x_roots {
                stats.positive_fibre_root_tests += 1;
                if eval(c.f, &specialized, x) == 0 {
                    fibre_roots += 1;
                    if !support.insert((x, t)) {
                        return Err(ExtractionError::Internal("overlapping support blocks"));
                    }
                }
            }
            if fibre_roots + 1 != leaf.gcd.len() {
                return Err(ExtractionError::Internal("fibre root coverage"));
            }
        }
        if block_roots + 1 != leaf.modulus.len() {
            return Err(ExtractionError::Internal("positive block root coverage"));
        }
    }
    if support.len() != certificate.support_count {
        return Err(ExtractionError::Internal("support count mismatch"));
    }
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut right_index = BTreeMap::new();
    for t in support.iter().map(|&(_, t)| t).collect::<BTreeSet<_>>() {
        stats.right_gcd_queries += 1;
        let (gcd, roots) = if let Some(target) = input.target {
            witnesses(c, &l, &selector, t, target[0], &mut stats)?
        } else {
            let gcd = kernel::monic(c.f, &poly_gcd(c.f, &l, &[c.f.neg(t), 1]));
            let roots = if gcd.len() == 2 { vec![t] } else { vec![] };
            (gcd, roots)
        };
        let index = right.len();
        right.push(RightBucket { t, gcd, roots });
        right_index.insert(t, index);
    }
    let mut tuples = BTreeMap::new();
    let mut emitted_left = BTreeSet::new();
    for (x, t) in support {
        stats.left_gcd_queries += 1;
        let (gcd, roots) = witnesses(c, &l, &selector, x, t, &mut stats)?;
        let left_index = left.len();
        left.push(LeftBucket {
            x,
            t,
            gcd,
            roots: roots.clone(),
        });
        let right_idx = right_index[&t];
        stats.base_x_bucket_lookups += 1;
        for &i in base_by_x.get(&x).map(Vec::as_slice).unwrap_or(&[]) {
            let p1 = base[i];
            for &z in &roots {
                stats.base_x_bucket_lookups += 1;
                for &j in base_by_x.get(&z).map(Vec::as_slice).unwrap_or(&[]) {
                    let p2 = base[j];
                    stats.left_lift_pair_attempts += 1;
                    stats.prefix_additions += 1;
                    let Some(r) = add(c, Some(p1), Some(p2)) else {
                        continue;
                    };
                    if r[0] != t {
                        continue;
                    }
                    stats.residual_subtractions += 1;
                    let Some(p3) = add(c, input.target, negate(c.f, Some(r))) else {
                        continue;
                    };
                    stats.base_membership_queries += 1;
                    let Ok(k) = base.binary_search(&p3) else {
                        continue;
                    };
                    if right[right_idx].roots.binary_search(&p3[0]).is_err() {
                        return Err(ExtractionError::Internal("missing right backpointer"));
                    }
                    insert_triple(
                        c,
                        &base,
                        input.target,
                        [i, j, k],
                        Route {
                            chart: 0,
                            t: Some(t),
                            left: Some(left_index),
                            right: Some(right_idx),
                        },
                        &mut tuples,
                        &mut stats,
                    )?;
                    emitted_left.insert(left_index);
                }
            }
        }
    }
    if let Some(target) = input.target {
        stats.base_membership_queries += 1;
        if let Ok(k) = base.binary_search(&target) {
            for (i, &p) in base.iter().enumerate() {
                let opposite = negate(c.f, Some(p)).unwrap();
                let j = base
                    .binary_search(&opposite)
                    .map_err(|_| ExtractionError::Internal("base not sign closed"))?;
                insert_triple(
                    c,
                    &base,
                    input.target,
                    [i, j, k],
                    Route {
                        chart: 1,
                        t: None,
                        left: None,
                        right: None,
                    },
                    &mut tuples,
                    &mut stats,
                )?;
            }
        }
    }
    stats.support_cells_without_signed_output = left.len() - emitted_left.len();
    let triples: Vec<_> = tuples.keys().copied().collect();
    let routes: Vec<_> = tuples.values().cloned().collect();
    let mut permutations = BTreeMap::<[usize; 3], Vec<usize>>::new();
    let mut rows = BTreeMap::<(Vec<i32>, i32), Vec<usize>>::new();
    for (index, (&triple, route)) in triples.iter().zip(&routes).enumerate() {
        if route.chart == 0 {
            stats.affine_triples += 1;
        } else {
            stats.identity_prefix_triples += 1;
        }
        let points = [base[triple[0]], base[triple[1]], base[triple[2]]];
        let cancellation =
            (0..3).any(|i| (i + 1..3).any(|j| Some(points[i]) == negate(c.f, Some(points[j]))));
        stats.inverse_pair_triples += usize::from(cancellation);
        stats.repeated_point_triples +=
            usize::from(triple[0] == triple[1] || triple[0] == triple[2] || triple[1] == triple[2]);
        if let Some(t) = route.t {
            let rhs =
                c.f.add(c.f.add(c.f.mul(c.f.mul(t, t), t), c.f.mul(c.a, t)), c.b);
            stats.two_torsion_prefix_triples += usize::from(rhs == 0);
            if let Some(target) = input.target {
                let mask = x_in_selector(&selector, target[0])
                    && (points[0][0] == target[0] || c.s3(points[0][0], target[0], t) == 0);
                stats.masked_affine_triples += usize::from(mask);
                stats.masked_noncancelling_triples += usize::from(mask && !cancellation);
            }
        }
        let mut permuted = triple;
        permuted.sort_unstable();
        permutations.entry(permuted).or_default().push(index);
        let mut coefficients = vec![0; base_x.len()];
        for point in points {
            let column = base_x.binary_search(&point[0]).unwrap();
            coefficients[column] += if point[1] <= c.f.neg(point[1]) { 1 } else { -1 };
            stats.coefficient_updates += 1;
        }
        let mut target_column = 0;
        if let Some(target) = input.target {
            if base.binary_search(&target).is_ok() {
                let column = base_x.binary_search(&target[0]).unwrap();
                coefficients[column] -= if target[1] <= c.f.neg(target[1]) {
                    1
                } else {
                    -1
                };
                stats.coefficient_updates += 1;
            } else {
                target_column = -1;
            }
        }
        rows.entry((coefficients, target_column))
            .or_default()
            .push(index);
    }
    let permutations = permutations
        .into_iter()
        .map(|(points, witnesses)| PermutationGroup { points, witnesses })
        .collect();
    let rows = rows
        .into_iter()
        .map(|((coefficients, target), witnesses)| RowGroup {
            coefficients,
            target,
            witnesses,
        })
        .collect();
    Ok(Extraction {
        case_id: input.case_id.clone(),
        selector,
        x_roots,
        t_roots,
        base,
        subgroup_points,
        certificate,
        left,
        right,
        triples,
        routes,
        permutations,
        rows,
        stats,
    })
}
fn x_in_selector(selector: &[u64], x: u64) -> bool {
    selector.binary_search(&x).is_ok()
}
