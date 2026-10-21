use crate::{digest, field::*, write_json, Fixture, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
#[derive(Clone, Serialize, Deserialize)]
pub struct Base {
    pub fixture: String,
    pub family: String,
    pub h: u64,
    pub witnesses: usize,
    pub zero_traces: usize,
    pub points: Vec<P>,
}
pub fn canonical(e: Quad, p: P) -> P {
    p.min(e.pn(p))
}
fn image(e: Quad, ss: &BTreeSet<P>, family: &str, h: u64, id: &str) -> Base {
    let mut fs = BTreeSet::new();
    let mut zero = 0;
    for &s in ss {
        let t = e.trace(s);
        if t.is_none() {
            zero += 1;
        } else {
            fs.insert(canonical(e, t));
        }
    }
    Base {
        fixture: id.into(),
        family: family.into(),
        h,
        witnesses: ss.len(),
        zero_traces: zero,
        points: fs.into_iter().collect(),
    }
}
pub fn coordinate(e: Quad, h: u64) -> BTreeSet<P> {
    let mut s = BTreeSet::new();
    for u in 0..h {
        for v in 0..h {
            let x = Q(u, v);
            if let Some(y) = e.sqrt(e.rhs(x)) {
                s.insert(Some((x, y)));
                s.insert(Some((x, e.neg(y))));
            }
        }
    }
    s
}
fn slope(e: Quad, h: u64) -> BTreeSet<P> {
    let mut ss = BTreeSet::new();
    for u in 0..h {
        for v in 1..e.p {
            let x = Q(u, v);
            if let Some(y) = e.sqrt(e.rhs(x)) {
                for y in [y, e.neg(y)] {
                    if y.1 * inv(v, e.p) % e.p < h {
                        ss.insert(Some((x, y)));
                    }
                }
            }
        }
        if let Some(w) = sqrt(e.rhs(Q(u, 0)).0, e.p) {
            for w in [w, (e.p - w) % e.p] {
                if w != 0 {
                    let m = (3 * u * u + e.a) % e.p * inv(2 * w % e.p, e.p) % e.p;
                    if m < h {
                        ss.insert(Some((Q(u, 0), Q(w, 0))));
                    }
                }
            }
        }
    }
    ss
}
fn rational_slope(e: Quad, h: u64, ps: &[P]) -> BTreeSet<P> {
    let p = e.p;
    let mut ss = BTreeSet::new();
    for &(x, y) in ps.iter().flatten() {
        for m in 0..h {
            let u = (m * m + p - x.0) % p * inv(2, p) % p;
            if u >= h {
                continue;
            }
            let delta = (pow(m, 4, p) + p - 6 * x.0 % p * m % p * m % p + p - 8 * y.0 % p * m % p
                + p
                - 3 * x.0 % p * x.0 % p
                + p
                - 4 * e.a % p)
                % p;
            let v2 = delta * inv(4 * e.d % p, p) % p;
            if let Some(v) = sqrt(v2, p) {
                for v in [v, (p - v) % p] {
                    let w = (m * (u + p - x.0) % p + p - y.0) % p;
                    let s = Some((Q(u, v), Q(w, m * v % p)));
                    assert!(e.on(s));
                    assert_eq!(e.trace(s), Some((x, y)));
                    ss.insert(s);
                }
            }
        }
    }
    ss
}
fn norm(e: Quad, x: Q) -> u64 {
    (x.0 * x.0 + e.p - e.d * x.1 % e.p * x.1 % e.p) % e.p
}
fn norm_x(e: Quad, h: u64) -> BTreeSet<P> {
    let mut ss = BTreeSet::new();
    for u in 0..e.p {
        for n in 0..h {
            let v2 = (u * u + e.p - n) % e.p * inv(e.d, e.p) % e.p;
            if let Some(v) = sqrt(v2, e.p) {
                for v in [v, (e.p - v) % e.p] {
                    let x = Q(u, v);
                    if let Some(y) = e.sqrt(e.rhs(x)) {
                        ss.insert(Some((x, y)));
                        ss.insert(Some((x, e.neg(y))));
                    }
                }
            }
        }
    }
    ss
}
fn rank_insert(rows: &mut BTreeMap<usize, Vec<u64>>, mut row: Vec<u64>, r: u64) -> bool {
    for (&pivot, b) in rows.iter() {
        let x = row[pivot];
        if x != 0 {
            for i in pivot..row.len() {
                row[i] = (row[i] + r - x * b[i] % r) % r;
            }
        }
    }
    if let Some(pivot) = row[..row.len() - 1].iter().position(|x| *x != 0) {
        let z = inv(row[pivot], r);
        for x in &mut row {
            *x = *x * z % r;
        }
        rows.insert(pivot, row);
        true
    } else {
        assert_eq!(*row.last().unwrap(), 0, "inconsistent known-answer row");
        false
    }
}
fn span(rows: &BTreeMap<usize, Vec<u64>>, row: &[u64], r: u64) -> Option<u64> {
    let mut v = row.to_vec();
    v.push(0);
    for (&pivot, b) in rows {
        let x = v[pivot];
        for i in pivot..v.len() {
            v[i] = (v[i] + r - x * b[i] % r) % r;
        }
    }
    if v[..row.len()].iter().all(|x| *x == 0) {
        Some((r - v[row.len()]) % r)
    } else {
        None
    }
}
fn tuples(
    e: Quad,
    base: &[P],
    arity: usize,
    index: usize,
    sum: P,
    row: &mut [u64],
    r: u64,
    kernels: &mut BTreeSet<Vec<u64>>,
    first: &mut BTreeMap<P, Vec<u64>>,
    count: &mut u64,
    zeros: &mut u64,
) {
    if arity == 0 {
        *count += 1;
        first.entry(sum).or_insert_with(|| row.to_vec());
        if sum.is_none() {
            if row.iter().all(|x| *x == 0) {
                *zeros += 1;
            } else {
                kernels.insert(row.to_vec());
            }
        }
        return;
    }
    for j in index..base.len() * 2 {
        let col = j / 2;
        let (sign, point) = if j % 2 == 0 {
            (1, base[col])
        } else {
            (r - 1, e.pn(base[col]))
        };
        row[col] = (row[col] + sign) % r;
        tuples(
            e,
            base,
            arity - 1,
            j,
            e.pa(sum, point),
            row,
            r,
            kernels,
            first,
            count,
            zeros,
        );
        row[col] = (row[col] + r - sign) % r;
    }
}
fn census(f: &Fixture, b: &Base, size: usize, m: usize) -> Value {
    if b.points.len() < size {
        return json!({"fixture":f.id,"family":b.family,"H":b.h,"B":size,"m":m,"status":"unsupported-empty-or-undersized-base","available_columns":b.points.len()});
    }
    let e = f.curve();
    let base = &b.points[..size];
    let mut first = BTreeMap::new();
    let mut kernel = BTreeSet::new();
    let (mut count, mut zero) = (0, 0);
    tuples(
        e,
        base,
        m,
        0,
        None,
        &mut vec![0; size],
        f.order,
        &mut kernel,
        &mut first,
        &mut count,
        &mut zero,
    );
    let mut oracle = BTreeSet::from([None]);
    for _ in 0..m {
        let mut next = BTreeSet::new();
        for &s in &oracle {
            for &p in base {
                next.insert(e.pa(s, p));
                next.insert(e.pa(s, e.pn(p)));
            }
        }
        oracle = next;
    }
    assert_eq!(oracle, first.keys().copied().collect());
    let mut rows = BTreeMap::new();
    for row in &kernel {
        let mut v = row.clone();
        v.push(0);
        rank_insert(&mut rows, v, f.order);
    }
    let kernel_rank = rows.len();
    let mut anchor = false;
    if let Some(grow) = first.get(&f.group()) {
        let mut v = grow.clone();
        v.push(1);
        rank_insert(&mut rows, v, f.order);
        anchor = true;
    }
    let mut recovered = 0;
    for (&p, row) in &first {
        if let Some(k) = span(&rows, row, f.order) {
            assert_eq!(e.pm(f.group(), k as u128), p);
            if p.is_some() {
                recovered += 1;
            }
        }
    }
    json!({"fixture":f.id,"family":b.family,"H":b.h,"B":size,"m":m,"status":"verified-exact-census","unordered_signed_multisets":count,"distinct_zero_sum_rows":kernel.len(),"cancelled_zero_rows":zero,"kernel_rank_mod_r":kernel_rank,"generator_anchor":anchor,"rank_after_anchor":rows.len(),"nonidentity_targets_with_decomposition":first.len()-usize::from(first.contains_key(&None)),"nonidentity_targets_recovered_from_target_row_span":recovered,"population_nonidentity_targets":f.order-1,"mitm_coverage_equal":true,"projected_base_sha256":digest(&serde_json::to_vec(base).unwrap()),"performance_comparison":false})
}
fn divisor(e: Quad, id: &str, points: &[P]) -> Value {
    let p = e.p;
    let mut full = 0;
    let mut incomplete = 0;
    let mut generic_pairs = 0;
    let mut twist_pairs = 0;
    let mut rational_zeros = 0;
    let mut failures = vec![];
    let mut relation_hashes = BTreeSet::new();
    for a0 in [0, 1, p - 1] {
        for a1 in [0, 1, p - 1] {
            for a2 in [0, 1, p - 1] {
                if a0 == 0 && a1 == 0 && a2 == 0 {
                    continue;
                }
                let deg = if a2 != 0 {
                    2
                } else if a1 != 0 {
                    1
                } else {
                    0
                };
                let degree = 2 * deg.max(2) - usize::from(deg < 2);
                let mut roots = vec![];
                for x0 in 0..p {
                    for x1 in 0..p {
                        let x = Q(x0, x1);
                        let ax = e.add(e.add(Q(a0, 0), e.scale(x, a1)), e.scale(e.mul(x, x), a2));
                        if e.mul(ax, ax) != e.rhs(x) {
                            continue;
                        }
                        let pt = Some((x, e.neg(ax)));
                        let mut multiplicity = 1;
                        let mut coefficients = vec![
                            Q((a0 * a0 + p - e.b) % p, 0),
                            Q((2 * a0 * a1 + p - e.a) % p, 0),
                            Q((a1 * a1 + 2 * a0 * a2) % p, 0),
                            Q((2 * a1 * a2 + p - 1) % p, 0),
                            Q(a2 * a2 % p, 0),
                        ];
                        while coefficients.last() == Some(&Q(0, 0)) {
                            coefficients.pop();
                        }
                        loop {
                            let mut quotient = vec![Q(0, 0); coefficients.len().saturating_sub(1)];
                            if coefficients.len() < 2 {
                                break;
                            }
                            let mut rest = Q(0, 0);
                            for i in (0..coefficients.len()).rev() {
                                rest = e.add(e.mul(rest, x), coefficients[i]);
                                if i > 0 {
                                    quotient[i - 1] = rest;
                                }
                            }
                            if rest != Q(0, 0) {
                                break;
                            }
                            coefficients = quotient;
                            let value = coefficients
                                .iter()
                                .rev()
                                .fold(Q(0, 0), |v, c| e.add(e.mul(v, x), *c));
                            if value != Q(0, 0) {
                                break;
                            }
                            multiplicity += 1;
                        }
                        roots.push((pt, multiplicity));
                    }
                }
                let total: usize = roots.iter().map(|x| x.1).sum();
                let polynomial_degree = if deg == 2 { 4 } else { 3 };
                assert!(total <= polynomial_degree);
                if total != polynomial_degree {
                    incomplete += 1;
                    failures.push(json!({"a":[a0,a1,a2],"degree":polynomial_degree,"roots_in_quadratic_extension_counting_multiplicity":total}));
                    continue;
                }
                let mut sum = None;
                let mut row = vec![];
                for &(point, mult) in &roots {
                    for _ in 0..mult {
                        sum = e.pa(sum, point);
                    }
                    if e.frob(point) == point {
                        rational_zeros += mult;
                        row.push((point, mult));
                    } else if point < e.frob(point) {
                        let t = e.trace(point);
                        if t.is_none() {
                            twist_pairs += mult;
                        } else {
                            generic_pairs += mult;
                            row.push((t, mult));
                        }
                    }
                }
                assert_eq!(sum, None);
                full += 1;
                relation_hashes.insert(digest(&serde_json::to_vec(&row).unwrap()));
                let _ = degree;
            }
        }
    }
    let pure_pairs = points
        .iter()
        .filter(|p| p.is_some() && e.trace(**p).is_none() && e.frob(**p) != **p)
        .count()
        / 2;
    json!({"fixture":id,"status":"verified-bounded-function-family","function_family":"a0+a1*x+a2*x^2+y, a_i in {0,1,-1}; nonzero a; poles retained","functions_with_full_degree_le_2_splitting":full,"functions_with_unsplit_degree_3_or_4_factors":incomplete,"incomplete_factorizations":failures,"generic_degree_two_places_counted_with_multiplicity":generic_pairs,"principal_pure_twist_pairs_removed_with_multiplicity":twist_pairs,"rational_zeros_counted_with_multiplicity":rational_zeros,"distinct_projected_divisor_row_hashes":relation_hashes.len(),"pure_twist_pair_control_total":pure_pairs,"pure_twist_pair_control_nonzero_traces":0,"all_fully_split_divisor_sums_zero":true,"function_factorization_work_is_not_a_performance_measurement":true})
}
pub fn run(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut fibers = vec![];
    let mut bases = vec![];
    let mut slope_rows = vec![];
    let mut norm_rows = vec![];
    let mut divisors = vec![];
    let mut censuses = vec![];
    for f in fixtures {
        let e = f.curve();
        let rational = e.rational();
        assert_eq!(rational.len(), f.order as usize);
        let all = if f.p <= 827 { Some(e.points()) } else { None };
        if let Some(ref all) = all {
            let mut hist = BTreeMap::<P, usize>::new();
            let mut pure = 0;
            let mut rational_2 = 0;
            for &s in all {
                assert!(e.on(s));
                let t = e.trace(s);
                assert_eq!(e.frob(t), t);
                *hist.entry(t).or_default() += 1;
                if e.frob(s) == e.pn(s) {
                    pure += 1;
                    assert_eq!(t, None);
                }
                if e.frob(s) == s && e.pm(s, 2).is_none() {
                    rational_2 += 1;
                }
            }
            let tw = 2 * (e.p + 1) - f.order;
            assert_eq!(all.len() as u64, f.order * tw);
            assert_eq!(hist.len(), f.order as usize);
            assert!(hist.values().all(|n| *n == tw as usize));
            assert_eq!(pure, tw as usize);
            let doubles: BTreeSet<_> = rational.iter().map(|&p| e.pm(p, 2)).collect();
            assert_eq!(doubles.len() * rational_2, rational.len());
            fibers.push(json!({"fixture":f.id,"extension":f.extension_id,"status":"verified-exhaustive","rational_order":f.order,"twist_order":tw,"extension_order":all.len(),"trace_image":hist.len(),"each_trace_fiber":tw,"anti_invariant_points_including_identity":pure,"rational_2_torsion_points_including_identity":rational_2,"distinct_rational_only_doubles":doubles.len(),"odd_order_half_fiber_contract_applicable":f.order%2==1}));
            if prime(f.order) && f.order % 2 == 1 && f.role == "structural" {
                divisors.push(divisor(e, &f.id, all));
            }
        }
        if !prime(f.order) || f.order % 2 == 0 {
            continue;
        }
        for h in [2, 4, 8] {
            let cs = coordinate(e, h);
            bases.push(image(e, &cs, "coordinate-box", h, &f.id));
            let ss = slope(e, h);
            let control = rational_slope(e, h, &rational);
            assert_eq!(ss, control, "slope/rational witness mismatch");
            slope_rows.push(json!({"fixture":f.id,"H":h,"status":"verified-exact-witness-equivalence","extension_witnesses":ss.len(),"rational_quartic_witnesses":control.len(),"v_zero_branches":ss.iter().flatten().filter(|x|x.0.1==0).count(),"interpretation":"same rational witnesses after coordinate renaming"}));
            bases.push(image(e, &ss, "slope-real-box", h, &f.id));
            let nx = norm_x(e, h);
            let nn = image(e, &nx, "norm-x", h, &f.id);
            norm_rows.push(json!({"fixture":f.id,"H":h,"arm":"norm-x","witnesses":nx.len(),"distinct_signed_columns":nn.points.len(),"membership_degree_if_materialized":h}));
            bases.push(nn);
            let ny: BTreeSet<_> = if let Some(ref all) = all {
                all.iter()
                    .copied()
                    .filter(|x| x.is_some_and(|(_, y)| norm(e, y) < h))
                    .collect()
            } else {
                cs.iter()
                    .copied()
                    .filter(|x| x.is_some_and(|(_, y)| norm(e, y) < h))
                    .collect()
            };
            let label = if all.is_some() {
                "norm-y"
            } else {
                "coordinate-box-and-norm-y"
            };
            bases.push(image(e, &ny, label, h, &f.id));
            if (e.p - 1) % h == 0 {
                let subset: BTreeSet<_> = if let Some(ref all) = all {
                    all.iter()
                        .copied()
                        .filter(|x| {
                            x.is_some_and(|(x, _)| {
                                let n = norm(e, x);
                                n != 0 && pow(n, ((e.p - 1) / h) as u128, e.p) == 1
                            })
                        })
                        .collect()
                } else {
                    cs.iter()
                        .copied()
                        .filter(|x| {
                            x.is_some_and(|(x, _)| {
                                let n = norm(e, x);
                                n != 0 && pow(n, ((e.p - 1) / h) as u128, e.p) == 1
                            })
                        })
                        .collect()
                };
                bases.push(image(
                    e,
                    &subset,
                    if all.is_some() {
                        "norm-power-subgroup"
                    } else {
                        "coordinate-box-and-norm-power-subgroup"
                    },
                    h,
                    &f.id,
                ));
            } else {
                norm_rows.push(json!({"fixture":f.id,"H":h,"arm":"norm-power-subgroup","status":"inapplicable-H-does-not-divide-p-minus-one"}));
            }
        }
        let mut cache: BTreeMap<(String, usize, usize), Value> = BTreeMap::new();
        for b in bases.iter().filter(|b| b.fixture == f.id) {
            for size in [8, 12, 16] {
                for m in [2, 3, 4] {
                    let key = (
                        digest(&serde_json::to_vec(&b.points[..b.points.len().min(size)]).unwrap()),
                        size,
                        m,
                    );
                    let row = if let Some(v) = cache.get(&key) {
                        let mut v: Value = v.clone();
                        v["family"] = json!(b.family);
                        v["H"] = json!(b.h);
                        v["reused_identical_projected_base_census"] = json!(true);
                        v
                    } else {
                        let row = census(f, b, size, m);
                        cache.insert(key, row.clone());
                        row
                    };
                    censuses.push(row);
                }
            }
        }
    }
    write_json(out.join("E01-fibers.json"), &fibers)?;
    write_json(out.join("E02-E04-bases.json"), &bases)?;
    write_json(out.join("E03-rational-equivalence.json"), &slope_rows)?;
    write_json(out.join("E04-norm-controls.json"), &norm_rows)?;
    write_json(out.join("E05-divisors.json"), &divisors)?;
    write_json(out.join("E06-census.json"), &censuses)?;
    Ok(vec![
        json!({"experiment":"E01","status":"executed-exhaustive-structural-controls","rows":fibers.len()}),
        json!({"experiment":"E02","status":"executed-exact-restricted-bases","base_records":bases.len()}),
        json!({"experiment":"E03","status":"executed-exact-rational-equivalence","rows":slope_rows.len()}),
        json!({"experiment":"E04","status":"executed; larger norm-y/power arms use declared box intersections","rows":norm_rows.len()}),
        json!({"experiment":"E05","status":"executed-bounded-function-family-on-two-structural-fixtures","rows":divisors.len()}),
        json!({"experiment":"E06","status":"executed-exact-census-with-unsupported-cells-preserved","rows":censuses.len()}),
    ])
}
