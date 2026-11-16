//! Target-labelled pencil geometry on a valid Scholten source fixture.
//! Every accepted three-point relation is checked in the Jacobian; the first
//! of each walk is also checked through the independent dual norm map.
use five_leads_study::three_leads::{
    solve_rho, Counts, Divisor, Elliptic, Field, Point, Scholten, StudyInstance,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::process::Command;
use std::time::Instant;

#[derive(Clone, Copy, Default, Serialize)]
struct ArithmeticCounts {
    base_adds: u64,
    base_muls: u64,
    base_inversions: u64,
    scalar_adds: u64,
    scalar_muls: u64,
    scalar_inversions: u64,
}
impl ArithmeticCounts {
    fn difference(self, before: Self) -> Self {
        Self {
            base_adds: self.base_adds - before.base_adds,
            base_muls: self.base_muls - before.base_muls,
            base_inversions: self.base_inversions - before.base_inversions,
            scalar_adds: self.scalar_adds - before.scalar_adds,
            scalar_muls: self.scalar_muls - before.scalar_muls,
            scalar_inversions: self.scalar_inversions - before.scalar_inversions,
        }
    }
    fn add_assign(&mut self, other: Self) {
        self.base_adds += other.base_adds;
        self.base_muls += other.base_muls;
        self.base_inversions += other.base_inversions;
        self.scalar_adds += other.scalar_adds;
        self.scalar_muls += other.scalar_muls;
        self.scalar_inversions += other.scalar_inversions;
    }
}
#[derive(Default, Serialize)]
struct PhaseTotals {
    elapsed_ns: u128,
    field: Counts,
    arithmetic: ArithmeticCounts,
}
fn group_counts(s: &Scholten) -> Counts {
    s.e.f.counts() + s.j.f.counts()
}
fn measured<T>(
    s: &Scholten,
    phases: &mut BTreeMap<&'static str, PhaseTotals>,
    name: &'static str,
    f: impl FnOnce() -> T,
) -> T {
    let before_field = group_counts(s);
    let before_arithmetic = arithmetic_counts();
    let start = Instant::now();
    let result = f();
    let elapsed_ns = start.elapsed().as_nanos();
    let phase = phases.entry(name).or_default();
    phase.elapsed_ns += elapsed_ns;
    phase.field = phase.field + (group_counts(s) - before_field);
    phase
        .arithmetic
        .add_assign(arithmetic_counts().difference(before_arithmetic));
    result
}
thread_local! {
    static PRICE_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static PRICE_COUNTS: Cell<ArithmeticCounts> = Cell::new(ArithmeticCounts::default());
    static BASE_MODULUS: Cell<u64> = const { Cell::new(739) };
    static SCALAR_MODULUS: Cell<u64> = const { Cell::new(34147) };
}
fn charge(modulus: u64, action: impl FnOnce(&mut ArithmeticCounts)) {
    PRICE_ACTIVE.with(|active| {
        if active.get() {
            PRICE_COUNTS.with(|counts| {
                let mut c = counts.get();
                assert!(
                    BASE_MODULUS.with(Cell::get) == modulus
                        || SCALAR_MODULUS.with(Cell::get) == modulus
                );
                action(&mut c);
                counts.set(c);
            });
        }
    });
}
fn arithmetic_counts() -> ArithmeticCounts {
    PRICE_COUNTS.with(Cell::get)
}
fn reset_arithmetic_counts() {
    PRICE_COUNTS.with(|c| c.set(ArithmeticCounts::default()));
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
fn add(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + b) % p
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_adds += 1
        } else {
            c.scalar_adds += 1
        }
    });
    (a + p - b) % p
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_muls += 1
        } else {
            c.scalar_muls += 1
        }
    });
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    assert_eq!(n, p - 2);
    charge(p, |c| {
        if p == BASE_MODULUS.with(Cell::get) {
            c.base_inversions += 1
        } else {
            c.scalar_inversions += 1
        }
    });
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}
fn pmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = add(out[i + j], mul(x, y, p), p);
        }
    }
    trim(out)
}
fn peval(a: &[u64], x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |v, &c| add(mul(v, x, p), c, p))
}
fn divide_monic(mut a: Vec<u64>, b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    assert_eq!(b.last(), Some(&1));
    let mut q = vec![0; a.len().saturating_sub(b.len()) + 1];
    while a.len() >= b.len() {
        let shift = a.len() - b.len();
        let c = *a.last().unwrap();
        q[shift] = c;
        for (i, &bi) in b.iter().enumerate() {
            a[i + shift] = sub(a[i + shift], mul(c, bi, p), p);
        }
        a = trim(a);
    }
    (trim(q), a)
}
fn complement(d: Divisor, lambda: u64, h: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let u = &d.u[..=d.degree];
    let mut w = vec![0; u.len()];
    for i in 0..u.len() {
        let vi = if i < d.degree { d.v[i] } else { 0 };
        w[i] = add(vi, mul(lambda, u[i], p), p);
    }
    let w2 = pmul(&w, &w, p);
    let numerator = trim(
        (0..h.len())
            .map(|i| sub(*w2.get(i).unwrap_or(&0), h[i], p))
            .collect(),
    );
    let (quotient, remainder) = divide_monic(numerator, u, p);
    assert!(remainder.is_empty());
    (quotient, w)
}
fn rational_y_by_x(h: &[u64], p: u64) -> Vec<Vec<u64>> {
    let mut square_roots = vec![Vec::new(); p as usize];
    for y in 0..p {
        square_roots[mul(y, y, p) as usize].push(y);
    }
    (0..p)
        .map(|x| square_roots[peval(h, x, p) as usize].clone())
        .collect()
}
fn point_sieve_candidates(
    d: Divisor,
    h: &[u64],
    p: u64,
    rational_y: &[Vec<u64>],
) -> (
    BTreeSet<u64>,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    Vec<Vec<u64>>,
) {
    let mut buckets = vec![Vec::<u64>::new(); p as usize];
    let mut point_probes = 0;
    let mut denominators = Vec::new();
    let mut exceptional_denominators = Vec::new();
    let mut exceptional_points = Vec::new();
    let mut all_lambda_points = Vec::new();
    let u = &d.u[..=d.degree];
    let v = &d.v[..d.degree];
    let (q0, _) = complement(d, 0, h, p);
    let mut exceptional = BTreeSet::new();
    for x in 0..p {
        let ux = peval(u, x, p);
        let vx = peval(v, x, p);
        if ux == 0 {
            // q_lambda = q_0 + 2 lambda v + lambda^2 u.  At a root
            // of u, this is linear in lambda even for a double root.
            let qx = peval(&q0, x, p);
            let slope = mul(2, vx, p);
            if slope != 0 {
                exceptional_denominators.push((x, qx, slope));
            } else if qx == 0 {
                exceptional.extend(0..p);
                all_lambda_points.push(x);
            }
            continue;
        }
        if rational_y[x as usize].is_empty() {
            continue;
        }
        denominators.push((x, ux, vx));
    }
    let ordinary_count = denominators.len();
    let all_denominators: Vec<u64> = denominators
        .iter()
        .map(|&(_, ux, _)| ux)
        .chain(exceptional_denominators.iter().map(|&(_, _, slope)| slope))
        .collect();
    let mut prefix = vec![1; all_denominators.len() + 1];
    let mut batch_multiplies = 0;
    for (i, &d) in all_denominators.iter().enumerate() {
        prefix[i + 1] = mul(prefix[i], d, p);
        batch_multiplies += 1;
    }
    let batch_inversions = usize::from(!all_denominators.is_empty());
    let mut suffix = if all_denominators.is_empty() {
        1
    } else {
        pow(*prefix.last().unwrap(), p - 2, p)
    };
    for i in (0..all_denominators.len()).rev() {
        let d_i = all_denominators[i];
        let inv = mul(prefix[i], suffix, p);
        suffix = mul(suffix, d_i, p);
        batch_multiplies += 2;
        assert_eq!(mul(inv, d_i, p), 1);
        if i >= ordinary_count {
            let (x, qx, _) = exceptional_denominators[i - ordinary_count];
            let lambda = mul(sub(0, qx, p), inv, p);
            exceptional.insert(lambda);
            exceptional_points.push((lambda, x));
            continue;
        }
        let (x, ux, vx) = denominators[i];
        assert_eq!(mul(inv, ux, p), 1);
        for &y in &rational_y[x as usize] {
            point_probes += 1;
            let lambda = mul(sub(y, vx, p), inv, p);
            buckets[lambda as usize].push(x);
        }
    }
    let collision_candidates = buckets.iter().filter(|b| b.len() >= 3).count();
    let exceptional_count = exceptional.len();
    let mut candidates = exceptional;
    for (lambda, bucket) in buckets.iter().enumerate() {
        if bucket.len() >= 3 {
            assert_eq!(
                bucket.iter().copied().collect::<BTreeSet<_>>().len(),
                bucket.len()
            );
            candidates.insert(lambda as u64);
        }
    }
    for (lambda, x) in exceptional_points {
        buckets[lambda as usize].push(x);
    }
    for x in all_lambda_points {
        for bucket in &mut buckets {
            bucket.push(x);
        }
    }
    (
        candidates,
        ordinary_count,
        exceptional_denominators.len(),
        batch_inversions,
        batch_multiplies,
        point_probes,
        collision_candidates,
        exceptional_count,
        buckets,
    )
}
fn fixture() -> (u64, u64, Point, u64, u64) {
    let path = "research/three_leads_e2e_20261018/genus-order-fixtures-v1/order-genus-16.json";
    let spec: Value = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let row = &spec["workloads"]["curves"][0];
    let number = |k: &str| row[k].as_u64().unwrap();
    (
        number("p"),
        number("r"),
        Point::affine(number("gx"), number("gy")),
        number("a"),
        number("b"),
    )
}
struct LinearSolver {
    field_prime: u64,
    modulus: u64,
    nodes: Vec<Option<usize>>,
    pivots: Vec<Option<BTreeMap<usize, u64>>>,
    rank: usize,
    stored_nonzero_coefficients: usize,
    row_updates: u64,
    inversions: u64,
    inserted_relations: u64,
    dependent_relations: u64,
}
impl LinearSolver {
    fn new(p: u64, r: u64, rational_y: &[Vec<u64>]) -> Self {
        let mut nodes = vec![None; p as usize];
        let mut count = 0;
        for (x, ys) in rational_y.iter().enumerate() {
            if ys.iter().any(|&y| y != 0) {
                nodes[x] = Some(count);
                count += 1;
            }
        }
        Self {
            field_prime: p,
            modulus: r,
            nodes,
            pivots: vec![None; count],
            rank: 0,
            stored_nonzero_coefficients: 0,
            row_updates: 0,
            inversions: 0,
            inserted_relations: 0,
            dependent_relations: 0,
        }
    }
    fn relation(&mut self, points: &[[u64; 2]], a: u64, b: u64) -> Option<u64> {
        let n = self.pivots.len();
        let r = self.modulus;
        let mut row = BTreeMap::<usize, u64>::new();
        for &[x, y] in points {
            if y == 0 {
                continue;
            }
            let col = self.nodes[x as usize].expect("rational point has a node");
            // The hyperelliptic involution negates the point class.
            let sign = if y <= self.field_prime / 2 { 1 } else { r - 1 };
            let value = add(row.get(&col).copied().unwrap_or(0), sign, r);
            if value == 0 {
                row.remove(&col);
            } else {
                row.insert(col, value);
            }
        }
        if b != 0 {
            row.insert(n, b);
        }
        let neg_a = sub(0, a, r);
        if neg_a != 0 {
            row.insert(n + 1, neg_a);
        }
        self.inserted_relations += 1;
        while let Some((&col, &coefficient)) = row.first_key_value() {
            if col >= n {
                break;
            }
            if let Some(pivot) = &self.pivots[col] {
                for (&j, &v) in pivot {
                    let updated = sub(row.get(&j).copied().unwrap_or(0), mul(coefficient, v, r), r);
                    if updated == 0 {
                        row.remove(&j);
                    } else {
                        row.insert(j, updated);
                    }
                    self.row_updates += 1;
                }
            } else {
                let inv = pow(coefficient, r - 2, r);
                self.inversions += 1;
                for item in row.values_mut() {
                    *item = mul(*item, inv, r);
                    self.row_updates += 1;
                }
                self.stored_nonzero_coefficients += row.len();
                self.pivots[col] = Some(row);
                self.rank += 1;
                return None;
            }
        }
        self.dependent_relations += 1;
        let target_coefficient = row.get(&n).copied().unwrap_or(0);
        let constant = row.get(&(n + 1)).copied().unwrap_or(0);
        if target_coefficient == 0 {
            assert_eq!(constant, 0, "inconsistent verified relation");
            return None;
        }
        self.inversions += 1;
        Some(mul(constant, pow(target_coefficient, r - 2, r), r))
    }
}
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.get(1).map(String::as_str) == Some("--cohort") {
        assert_eq!(arguments.len(), 4, "usage: pencil-full --cohort BITS P");
        cohort_main(arguments[2].parse().unwrap(), arguments[3].parse().unwrap());
        return;
    }
    if std::env::args().any(|arg| arg == "--price-scan") {
        price_main(true);
        return;
    }
    if std::env::args().any(|arg| arg == "--price") {
        price_main(false);
        return;
    }
    let (p, r, generator, a_curve, b_curve) = fixture();
    assert_eq!(p % 4, 3);
    let s = Scholten::new(p);
    let source = s.short_curve();
    assert_eq!(source.c, [b_curve, a_curve, 0, 1]);
    assert!(source.valid(generator));
    assert_eq!(source.mul(generator, r), Point::O);
    let transferred_generator = s.tau(s.from_short(generator));
    assert_eq!(s.j.mul(transferred_generator, r), Divisor::O);
    let h = &s.j.polynomial;
    let rational_y = rational_y_by_x(h, p);
    let rational_curve_points: usize = rational_y.iter().map(Vec::len).sum();
    let cofactor = 16;
    let projected_generator = s.j.mul(transferred_generator, cofactor);
    assert_ne!(projected_generator, Divisor::O);
    assert_eq!(s.j.mul(projected_generator, r), Divisor::O);
    let mut projected_factor_base_checked = 0;
    for (x, ys) in rational_y.iter().enumerate() {
        for &y in ys {
            let projection = s.j.mul(s.j.point(x as u64, y), cofactor);
            assert_eq!(s.j.mul(projection, r), Divisor::O);
            if y == 0 {
                assert_eq!(projection, Divisor::O);
            }
            projected_factor_base_checked += 1;
        }
    }
    let mut total_walks = 0;
    let mut total_members = 0;
    let mut total_complete = 0;
    let mut total_baseline_split = 0;
    let mut total_repeated_root_u = 0;
    let mut degree_drops = 0;
    let mut total_sieve_candidates = 0;
    let mut total_sieve_ordinary_denominators = 0;
    let mut total_sieve_exceptional_denominators = 0;
    let mut total_sieve_batch_inversions = 0;
    let mut total_sieve_batch_multiplies = 0;
    let mut total_sieve_point_probes = 0;
    let mut total_exceptional_lambdas = 0;
    let mut targets_solved = 0;
    for target_index in 0..20u64 {
        let scalar = (123 + 1777 * target_index) % r;
        assert_ne!(scalar, 0);
        let target = source.mul(generator, scalar);
        assert_eq!(source.mul(target, r), Point::O);
        let mut target_complete = 0;
        let mut target_walks = 0;
        let mut solver = LinearSolver::new(p, r, &rational_y);
        let mut solver_relation_keys = Vec::new();
        let mut recovered = None;
        let mut solve_witness = None;
        for walk_index in 0..5u64 {
            let a = (2695 + 7919 * walk_index + 1741 * target_index) % r;
            let b = (409 + 3151 * walk_index + 857 * target_index) % r;
            assert_ne!(b, 0);
            let source_point = source.add(source.mul(generator, a), source.mul(target, b));
            assert_eq!(source_point, source.mul(generator, (a + b * scalar) % r));
            let d = s.tau(s.from_short(source_point));
            assert!(s.j.valid(d));
            if d.degree != 2 {
                degree_drops += 1;
                println!(
                    "{}",
                    json!({"schema":"pencil-target-walk/v1","target_index":target_index,
                    "scalar":scalar,"a":a,"b":b,"divisor":d,"degree_drop":true})
                );
                continue;
            }
            let u_roots = (0..p).filter(|&x| peval(&d.u[..3], x, p) == 0).count();
            let baseline_split = u_roots == 2;
            total_baseline_split += usize::from(baseline_split);
            total_repeated_root_u += usize::from(u_roots == 1);
            let mut complete = 0;
            let mut root_sets = BTreeSet::new();
            let mut complete_lambdas = BTreeSet::new();
            let mut first = None;
            let (
                sieve_candidates,
                ordinary_denominators,
                exceptional_denominators,
                batch_inversions,
                batch_multiplies,
                point_probes,
                collision_candidates,
                exceptional_count,
                sieve_root_buckets,
            ) = point_sieve_candidates(d, h, p, &rational_y);
            total_sieve_candidates += sieve_candidates.len();
            total_sieve_ordinary_denominators += ordinary_denominators;
            total_sieve_exceptional_denominators += exceptional_denominators;
            total_sieve_batch_inversions += batch_inversions;
            total_sieve_batch_multiplies += batch_multiplies;
            total_sieve_point_probes += point_probes;
            total_exceptional_lambdas += exceptional_count;
            for lambda in 0..p {
                let (quotient, w) = complement(d, lambda, h, p);
                assert_eq!(quotient.len(), 4);
                let roots: Vec<_> = (0..p).filter(|&x| peval(&quotient, x, p) == 0).collect();
                if roots.len() != 3 {
                    continue;
                }
                assert!(sieve_candidates.contains(&lambda));
                let mut sieve_roots = sieve_root_buckets[lambda as usize].clone();
                sieve_roots.sort_unstable();
                assert_eq!(sieve_roots, roots);
                assert!(complete_lambdas.insert(lambda));
                assert!(root_sets.insert(roots.clone()));
                let mut sum = Divisor::O;
                let mut points = Vec::new();
                for &x in &roots {
                    let y = peval(&w, x, p);
                    assert_eq!(mul(y, y, p), peval(h, x, p));
                    let pt = s.j.point(x, y);
                    assert!(s.j.valid(pt));
                    sum = s.j.add(sum, pt);
                    points.push([x, y]);
                }
                assert_eq!(s.j.add(d, sum), Divisor::O);
                if recovered.is_none() {
                    solver_relation_keys.push([walk_index, lambda]);
                    if let Some(candidate) = solver.relation(&points, a, b) {
                        assert_eq!(source.mul(generator, candidate), target);
                        recovered = Some(candidate);
                        solve_witness = Some(json!({"walk_index":walk_index,"lambda":lambda,
                            "inserted_relations":solver.inserted_relations,"basis_rank":solver.rank}));
                    }
                }
                if first.is_none() {
                    let original = s.from_short(source_point);
                    let doubled = s.e.add(original, original);
                    assert_eq!(s.norm(d), doubled);
                    assert_eq!(s.e.add(s.norm(sum), doubled), Point::O);
                    first = Some(json!({"lambda":lambda,"points":points,
                        "quotient":quotient,"jacobian_identity":true,"dual_norm_identity":true}));
                }
                complete += 1;
            }
            let mut sieve_complete = BTreeSet::new();
            for &lambda in &sieve_candidates {
                let (quotient, _) = complement(d, lambda, h, p);
                let roots = (0..p).filter(|&x| peval(&quotient, x, p) == 0).count();
                if roots == 3 {
                    sieve_complete.insert(lambda);
                }
            }
            assert_eq!(sieve_complete, complete_lambdas);
            total_walks += 1;
            total_members += p;
            total_complete += complete;
            target_walks += 1;
            target_complete += complete;
            println!(
                "{}",
                json!({"schema":"pencil-target-walk/v1","target_index":target_index,
                "scalar":scalar,"target":target,"a":a,"b":b,"source_point":source_point,
                "divisor":d,"baseline_two_distinct_roots":baseline_split,
                "u_distinct_rational_roots":u_roots,
                "pencil_members":p,"complete_three_root_relations":complete,
                "point_sieve_candidates":sieve_candidates.len(),
                "point_sieve_ordinary_denominators":ordinary_denominators,
                "point_sieve_exceptional_denominators":exceptional_denominators,
                "point_sieve_batch_inversions":batch_inversions,
                "point_sieve_batch_multiplies":batch_multiplies,
                "point_sieve_point_probes":point_probes,
                "point_sieve_collision_candidates":collision_candidates,
                "point_sieve_exceptional_lambdas":exceptional_count,
                "point_sieve_complete_set_equal_exhaustive":true,
                "unique_root_sets":root_sets.len(),"first_relation":first,
                "all_jacobian_identities_checked":true})
            );
        }
        assert!(target_walks > 0);
        targets_solved += usize::from(recovered.is_some());
        println!(
            "{}",
            json!({"schema":"pencil-target-summary/v1","target_index":target_index,
            "scalar":scalar,"walks":target_walks,"complete_relations":target_complete,
            "recovered":recovered,"solve_witness":solve_witness,
            "solver_nodes":solver.pivots.len(),"solver_rank":solver.rank,
            "solver_inserted_relations":solver.inserted_relations,
            "solver_relation_keys":solver_relation_keys,
            "solver_dependent_relations":solver.dependent_relations,
            "solver_row_updates":solver.row_updates,"solver_inversions":solver.inversions})
        );
    }
    println!(
        "{}",
        json!({"schema":"pencil-study-summary/v1","p":p,"r":r,
        "quintic_coefficients_low_to_high":h,
        "targets":20,"planned_walks":100,"processed_degree_two_walks":total_walks,
        "targets_solved_by_three_point_elimination":targets_solved,
        "projected_factor_base_points_checked":projected_factor_base_checked,
        "cofactor_projection":cofactor,
        "degree_drops":degree_drops,"pencil_members":total_members,
        "complete_three_root_relations":total_complete,
        "rational_curve_points":rational_curve_points,
        "point_sieve_candidates":total_sieve_candidates,
        "point_sieve_ordinary_denominators":total_sieve_ordinary_denominators,
        "point_sieve_exceptional_denominators":total_sieve_exceptional_denominators,
        "point_sieve_batch_inversions":total_sieve_batch_inversions,
        "point_sieve_batch_multiplies":total_sieve_batch_multiplies,
        "point_sieve_point_probes":total_sieve_point_probes,
        "point_sieve_exceptional_lambdas":total_exceptional_lambdas,
        "point_sieve_complete_sets_equal_exhaustive":true,
        "baseline_split_walks":total_baseline_split,
        "repeated_root_u_walks":total_repeated_root_u,
        "all_complete_relations_verified_in_jacobian":true,
        "first_relation_per_walk_verified_under_dual_norm":true})
    );
}

fn price_main(full_scan: bool) {
    PRICE_ACTIVE.with(|active| active.set(true));
    reset_arithmetic_counts();
    let (p, r, generator, a_curve, b_curve) = fixture();
    let setup_start = Instant::now();
    let s = Scholten::new(p);
    let source = s.short_curve();
    assert_eq!(source.c, [b_curve, a_curve, 0, 1]);
    assert!(source.valid(generator));
    assert_eq!(source.mul(generator, r), Point::O);
    let h = &s.j.polynomial;
    let rational_y = rational_y_by_x(h, p);
    let mut checked = 0;
    for (x, ys) in rational_y.iter().enumerate() {
        for &y in ys {
            let projected = s.j.mul(s.j.point(x as u64, y), 16);
            assert_eq!(s.j.mul(projected, r), Divisor::O);
            checked += 1;
        }
    }
    let setup_elapsed_ns = setup_start.elapsed().as_nanos();
    println!(
        "{}",
        json!({"schema":"pencil-priced-setup/v1","p":p,"r":r,
            "rational_curve_points":checked,"elapsed_ns":setup_elapsed_ns,
            "field":group_counts(&s),"arithmetic":arithmetic_counts()})
    );
    for run_id in 0..60u64 {
        let round = run_id / 20;
        let target_index = run_id % 20;
        let scalar = (123 + 1777 * target_index) % r;
        let target = source.mul(generator, scalar);
        s.e.f.reset();
        s.j.f.reset();
        reset_arithmetic_counts();
        let online_start = Instant::now();
        let mut phases = BTreeMap::new();
        let mut solver = LinearSolver::new(p, r, &rational_y);
        let mut recovered = None;
        let mut walks = 0;
        let mut candidate_lambdas = 0;
        let mut rejected_candidates = 0;
        let mut verified_relations = 0;
        let mut point_probes = 0;
        let mut batch_inversions = 0;
        let mut batch_multiplies = 0;
        let mut solution_key = None;
        for walk_index in 0..5u64 {
            let a = (2695 + 7919 * walk_index + 1741 * target_index) % r;
            let b = (409 + 3151 * walk_index + 857 * target_index) % r;
            assert_ne!(b, 0);
            let d = measured(&s, &mut phases, "walk_generation", || {
                let source_point = source.add(source.mul(generator, a), source.mul(target, b));
                s.tau(s.from_short(source_point))
            });
            assert_eq!(d.degree, 2);
            walks += 1;
            let (candidates, _, _, inversions, multiplies, probes, _, _, sieve_root_buckets) =
                measured(&s, &mut phases, "sieve", || {
                    point_sieve_candidates(d, h, p, &rational_y)
                });
            candidate_lambdas += candidates.len();
            point_probes += probes;
            batch_inversions += inversions;
            batch_multiplies += multiplies;
            for lambda in candidates {
                let points = measured(&s, &mut phases, "candidate_verification", || {
                    let (quotient, w) = complement(d, lambda, h, p);
                    assert_eq!(quotient.len(), 4);
                    let roots: Vec<u64> = if full_scan {
                        (0..p).filter(|&x| peval(&quotient, x, p) == 0).collect()
                    } else {
                        sieve_root_buckets[lambda as usize].clone()
                    };
                    if roots.len() != 3 {
                        return None;
                    }
                    assert_eq!(roots.iter().copied().collect::<BTreeSet<_>>().len(), 3);
                    let mut sum = Divisor::O;
                    let mut points = Vec::with_capacity(3);
                    for &x in &roots {
                        assert_eq!(peval(&quotient, x, p), 0);
                        let y = peval(&w, x, p);
                        assert_eq!(mul(y, y, p), peval(h, x, p));
                        let point = s.j.point(x, y);
                        sum = s.j.add(sum, point);
                        points.push([x, y]);
                    }
                    assert_eq!(s.j.add(d, sum), Divisor::O);
                    Some(points)
                });
                let Some(points) = points else {
                    rejected_candidates += 1;
                    continue;
                };
                verified_relations += 1;
                recovered = measured(&s, &mut phases, "elimination", || {
                    solver.relation(&points, a, b)
                });
                if recovered.is_some() {
                    solution_key = Some([walk_index, lambda]);
                    break;
                }
            }
            if recovered.is_some() {
                break;
            }
        }
        let recovered = recovered.expect("five walks must solve frozen target");
        measured(&s, &mut phases, "recovery_check", || {
            assert_eq!(source.mul(generator, recovered), target);
        });
        let online_elapsed_ns = online_start.elapsed().as_nanos();
        assert_eq!(recovered, scalar);
        assert_eq!(solver.inserted_relations as usize, verified_relations);
        println!(
            "{}",
            json!({"schema":"pencil-priced-target/v1","round":round,"target_index":target_index,
                "candidate_root_mode":if full_scan {"full_scan"} else {"witness_bucket"},
                "scalar":scalar,"recovered":recovered,"walks":walks,
                "solution_key":solution_key,"candidate_lambdas":candidate_lambdas,
                "rejected_candidates":rejected_candidates,
                "verified_relations":verified_relations,
                "point_probes":point_probes,
                "batch_inversions":batch_inversions,
                "batch_multiplies":batch_multiplies,
                "solver_nodes":solver.pivots.len(),"solver_rank":solver.rank,
                "solver_row_updates":solver.row_updates,
                "solver_inversions":solver.inversions,
                "online_elapsed_ns":online_elapsed_ns,
                "field":group_counts(&s),"arithmetic":arithmetic_counts(),
                "phases":phases})
        );
    }
}

fn walk_label(seed: u64, walk: u64, lane: u64, modulus: u64) -> u64 {
    let mut x = seed ^ walk.wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ lane;
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    (x ^ (x >> 31)) % modulus
}

fn cohort_main(bits: u64, selected_p: u64) {
    assert!([16, 20, 24, 28, 32, 36, 40].contains(&bits));
    let member = format!("sessions_projective_chart_v2/order-genus-{bits}/records.jsonl");
    let archive = "research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst";
    let output = Command::new("tar")
        .args(["--zstd", "-xOf", archive, &member])
        .output()
        .expect("open frozen input archive");
    assert!(
        output.status.success(),
        "archive input: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let input = String::from_utf8(output.stdout).unwrap();
    let rows: Vec<Value> = input
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .filter(|row: &Value| {
            row["arm"] == "genus_projective"
                && row["workload"]["curve_spec"]["p"].as_u64() == Some(selected_p)
        })
        .collect();
    assert_eq!(rows.len(), 60, "one curve has 60 frozen targets");
    let spec = &rows[0]["workload"]["curve_spec"];
    let p = spec["p"].as_u64().unwrap();
    let r = spec["r"].as_u64().unwrap();
    BASE_MODULUS.with(|cell| cell.set(p));
    SCALAR_MODULUS.with(|cell| cell.set(r));
    let order = spec["group_order"].as_u64().unwrap();
    assert_eq!(order % r, 0);
    let cofactor = order / r;
    let generator = Point::affine(spec["gx"].as_u64().unwrap(), spec["gy"].as_u64().unwrap());
    PRICE_ACTIVE.with(|active| active.set(true));
    reset_arithmetic_counts();
    let s = Scholten::new(p);
    let source = s.short_curve();
    assert_eq!(
        source.c,
        [
            spec["b"].as_u64().unwrap(),
            spec["a"].as_u64().unwrap(),
            0,
            1
        ]
    );
    assert!(source.valid(generator));
    assert_eq!(source.mul(generator, r), Point::O);
    let transferred = s.tau(s.from_short(generator));
    assert_eq!(s.j.mul(transferred, r), Divisor::O);
    let h = &s.j.polynomial;
    let rational_y = rational_y_by_x(h, p);
    let rational_curve_points: usize = rational_y.iter().map(Vec::len).sum();
    let mut projected_points_checked = 0;
    for (x, ys) in rational_y.iter().enumerate() {
        for &y in ys {
            if projected_points_checked == 16 {
                break;
            }
            let projected = s.j.mul(s.j.point(x as u64, y), cofactor);
            assert_eq!(s.j.mul(projected, r), Divisor::O);
            projected_points_checked += 1;
        }
        if projected_points_checked == 16 {
            break;
        }
    }
    assert_eq!(projected_points_checked, 16);
    let setup_field = group_counts(&s);
    let setup_arithmetic = arithmetic_counts();
    println!(
        "{}",
        json!({"schema":"pencil-cohort-setup/v1","bits":bits,"p":p,"r":r,
        "order":order,"cofactor":cofactor,"rational_curve_points":rational_curve_points,
        "projected_points_checked":projected_points_checked,"field":setup_field,
        "arithmetic":setup_arithmetic})
    );
    let inst = StudyInstance {
        curve: Elliptic {
            f: Field::new(p, 2, 1),
            c: source.c,
            binary: false,
        },
        order,
        r,
        h: cofactor,
        generator,
        family: "scholten".into(),
        parameter: 0,
        evidence: Value::Null,
    };
    let unit = inst.fresh();
    let twice = unit.curve.mul(unit.generator, 2);
    unit.curve.f.reset();
    let _ = unit.curve.add(unit.generator, twice);
    let source_add = unit.curve.f.counts();
    let target_limit: usize = std::env::var("PENCIL_MAX_TARGETS")
        .ok()
        .map(|value| value.parse().expect("PENCIL_MAX_TARGETS integer"))
        .unwrap_or(60);
    assert!((1..=60).contains(&target_limit));
    for row in rows.into_iter().take(target_limit) {
        let workload = &row["workload"];
        let scalar = workload["planted"].as_u64().unwrap();
        let hx = |v: &Value| {
            u64::from_str_radix(v.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
        };
        let target = Point::affine(hx(&workload["target"][0]), hx(&workload["target"][1]));
        assert!(source.valid(target));
        let seed: u64 = row["algorithm_seed"].as_str().unwrap().parse().unwrap();
        let cap: u64 = row["method"]["params"]["trial_cap"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        s.e.f.reset();
        s.j.f.reset();
        reset_arithmetic_counts();
        let mut phases = BTreeMap::new();
        let mut solver = LinearSolver::new(p, r, &rational_y);
        let mut recovered = None;
        let mut failure = None;
        let mut walks = 0;
        let mut candidate_lambdas = 0;
        let mut verified_relations = 0;
        let mut rejected_candidates = 0;
        let mut solution_key = None;
        'walks: for walk_index in 0..16u64 {
            let a = walk_label(seed, walk_index, 0x4455, r);
            let mut b = walk_label(seed, walk_index, 0x7788, r);
            if b == 0 {
                b = 1;
            }
            let d = measured(&s, &mut phases, "walk_generation", || {
                let point = source.add(source.mul(generator, a), source.mul(target, b));
                s.tau(s.from_short(point))
            });
            walks += 1;
            if d.degree != 2 {
                continue;
            }
            let (candidates, _, _, _, _, _, _, _, buckets) =
                measured(&s, &mut phases, "sieve", || {
                    point_sieve_candidates(d, h, p, &rational_y)
                });
            candidate_lambdas += candidates.len();
            for lambda in candidates {
                let points = measured(&s, &mut phases, "candidate_verification", || {
                    let (quotient, w) = complement(d, lambda, h, p);
                    let roots = &buckets[lambda as usize];
                    if roots.len() != 3 {
                        return None;
                    }
                    let mut sum = Divisor::O;
                    let mut points = Vec::with_capacity(3);
                    for &x in roots {
                        assert_eq!(peval(&quotient, x, p), 0);
                        let y = peval(&w, x, p);
                        assert_eq!(mul(y, y, p), peval(h, x, p));
                        sum = s.j.add(sum, s.j.point(x, y));
                        points.push([x, y]);
                    }
                    assert_eq!(s.j.add(d, sum), Divisor::O);
                    Some(points)
                });
                let Some(points) = points else {
                    rejected_candidates += 1;
                    continue;
                };
                verified_relations += 1;
                recovered = measured(&s, &mut phases, "elimination", || {
                    solver.relation(&points, a, b)
                });
                if recovered.is_some() {
                    solution_key = Some([walk_index, lambda]);
                    break 'walks;
                }
                if solver.stored_nonzero_coefficients > 10_000_000 {
                    failure = Some("sparse_matrix_cap");
                    break 'walks;
                }
            }
        }
        if recovered.is_none() && failure.is_none() {
            failure = Some("walk_cap");
        }
        let verified = if let Some(k) = recovered {
            measured(&s, &mut phases, "recovery_check", || {
                source.mul(generator, k) == target && k == scalar
            })
        } else {
            false
        };
        let mut pencil_arithmetic = arithmetic_counts();
        pencil_arithmetic.add_assign(setup_arithmetic);
        let pencil_field = group_counts(&s) + setup_field;
        let mut rho = solve_rho(&inst, target, seed, cap, true, false);
        let rho_verified = if let Some(k) = rho.recovered {
            let verifier = inst.fresh();
            let valid = verifier.curve.mul(generator, k) == target && k == scalar;
            rho.counts = rho.counts + verifier.curve.f.counts();
            valid
        } else {
            false
        };
        println!(
            "{}",
            json!({"schema":"pencil-cohort-target/v1","bits":bits,"p":p,
            "r":r,"workload_id":workload["workload_id"],"workload_sha256":workload["workload_sha256"],
            "target_index":workload["target_index"],"algorithm_seed":seed.to_string(),"cap":cap,
            "planted":scalar,"recovered":recovered,"verified":verified,"failure":failure,
            "walks":walks,"candidate_lambdas":candidate_lambdas,"verified_relations":verified_relations,
            "rejected_candidates":rejected_candidates,"solution_key":solution_key,
            "solver_nodes":solver.pivots.len(),"solver_rank":solver.rank,
            "solver_row_updates":solver.row_updates,"solver_inversions":solver.inversions,
            "solver_stored_nonzero_coefficients":solver.stored_nonzero_coefficients,
            "counts":pencil_field,"arithmetic":pencil_arithmetic,"phases":phases,"source_add":source_add,
            "rho":{"recovered":rho.recovered,"verified":rho_verified,"counts":rho.counts,
                   "native":rho.native,"phases":rho.phases,"detail":rho.detail}})
        );
    }
}
