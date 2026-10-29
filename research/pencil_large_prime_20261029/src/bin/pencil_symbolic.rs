//! Target-labelled pencil geometry on a valid Scholten source fixture.
//! Every accepted three-point relation is checked in the Jacobian.
//! Cohort mode does not perform an independent dual-norm check.
//! Fork of frozen pencil_sparse producer blob e974121a123e13316ce3fcc820b4eca3f65a8a3a.
//! Graph accounting is incomplete: outputs MUST NOT be priced by old analyzers.
use five_leads_study::three_leads::{
    solve_rho, Counts, Divisor, Elliptic, Field, Point, Scholten, StudyInstance,
};
use pencil_symbolic_graph::{
    collector::{Admission, Collector},
    residual::Limits,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;
use std::time::Instant;

struct GraphSolver {
    collector: Collector,
    nodes: Vec<Option<usize>>,
    field_prime: u64,
    modulus: u64,
    node_count: usize,
    denominator: usize,
    failure: Option<&'static str>,
}

#[cfg(test)]
mod graph_adapter_tests {
    use super::*;

    #[test]
    #[should_panic(expected = "inconsistent verified relation")]
    fn constant_only_relation_is_rejected() {
        let reference = LinearSolver::new(7, 7, &[vec![1, 6]]);
        let mut graph = GraphSolver::new(&reference, 100, 2);
        graph.relation(&[], 1, 0);
    }

    #[test]
    fn signed_point_encoding_matches_frozen_eliminator() {
        // Algebraic fixtures only: exercise the legacy point-label convention,
        // not a claim that these coordinates lie on a particular curve.
        for points in [
            vec![[0, 1]],
            vec![[0, 6]],
            vec![[0, 1], [0, 6]],
            vec![[0, 0]],
            vec![[0, 1], [1, 6]],
        ] {
            let ys = vec![vec![1, 6], vec![1, 6]];
            let mut reference = LinearSolver::new(7, 7, &ys);
            let mut graph = GraphSolver::new(&reference, 100, 2);
            let first = graph.relation(&points, 2, 1);
            assert_eq!(first, reference.relation(&points, 2, 1));
            if first.is_none() {
                assert_eq!(
                    graph.relation(&points, 3, 2),
                    reference.relation(&points, 3, 2)
                );
            }
            assert!(graph.failure.is_none());
        }
    }
}
impl GraphSolver {
    fn new(source: &LinearSolver, cap: usize, denominator: usize) -> Self {
        assert!([2, 4, 8, 16].contains(&denominator));
        assert_eq!(source.node_order, "identity");
        let n = source.node_count;
        let partition: Vec<_> = (0..n + 2).map(|i| i >= n || i % denominator == 0).collect();
        Self {
            collector: Collector::new(
                source.modulus,
                &partition,
                Limits {
                    retained_coefficients: cap,
                    working_coefficients: cap,
                },
            )
            .expect("prime subgroup field"),
            nodes: source.nodes.clone(),
            field_prime: source.field_prime,
            modulus: source.modulus,
            node_count: n,
            denominator,
            failure: None,
        }
    }
    fn relation(&mut self, points: &[[u64; 2]], a: u64, b: u64) -> Option<u64> {
        let r = self.modulus;
        let n = self.node_count;
        let mut row = Vec::new();
        for &[x, y] in points {
            if y == 0 {
                continue;
            }
            let col = self.nodes[x as usize].expect("checked rational point");
            row.push((col, if y <= self.field_prime / 2 { 1 } else { r - 1 }));
        }
        row.push((n, b));
        row.push((n + 1, sub(0, a, r)));
        match self.collector.insert(&row).expect("bound columns") {
            Admission::StoppedWorkingCap => {
                self.failure = Some("symbolic_working_cap");
                return None;
            }
            Admission::StoppedRetainedCap => {
                self.failure = Some("symbolic_retained_cap");
                return None;
            }
            Admission::AlreadyStopped => panic!("terminal cap was ignored"),
            Admission::Independent => {}
            _ => return None,
        }
        let target_column = self.collector.small_columns().len() - 2;
        assert!(
            !self
                .collector
                .basis()
                .rows()
                .any(|row| row.first_key_value().map(|(&i, _)| i) == Some(target_column + 1)),
            "inconsistent verified relation"
        );
        for row in self.collector.basis().rows() {
            if row.first_key_value().map(|(&i, _)| i) == Some(target_column) {
                // Normalized target pivot is one; same signed convention as
                // the frozen LinearSolver, with independent source replay below.
                return Some(row.get(&(target_column + 1)).copied().unwrap_or(0));
            }
        }
        None
    }
    fn detail(&self) -> Value {
        let c = self.collector.counts();
        let b = self.collector.basis();
        let a = b.counts();
        json!({"partition":"identity_column_mod_denominator", "denominator":self.denominator,
            "submitted_rows":c.submitted_rows,"submitted_terms":c.submitted_terms,
            "admitted_rows":c.admitted_rows,"rejected_three_large":c.rejected_three_large,
            "emitted_constraints":c.emitted_constraints,"residual_rank":b.rank(),
            "retained_coefficients":b.retained_coefficients(),
            "input_terms":a.input_terms,"pivot_lookups":a.pivot_lookups,
            "coefficient_updates":a.coefficient_updates,
            "normalization_multiplications":a.normalization_multiplications,
            "inversions":a.inversions,"peak_working_coefficients":a.peak_working_coefficients,
            "pending_constraint":self.collector.pending_constraint(),
            "cost_status":"incomplete_graph_and_allocation_accounting",
            "candidate_over_rho":Value::Null})
    }
}

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
type SieveResult = (
    BTreeSet<u64>,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    Vec<Vec<u64>>,
);
fn point_sieve_candidates(d: Divisor, h: &[u64], p: u64, rational_y: &[Vec<u64>]) -> SieveResult {
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
struct LinearSolver {
    field_prime: u64,
    modulus: u64,
    nodes: Vec<Option<usize>>,
    node_count: usize,
    #[cfg(test)]
    pivots: Vec<Option<BTreeMap<usize, u64>>>,
    #[cfg(test)]
    rank: usize,
    #[cfg(test)]
    stored_nonzero_coefficients: usize,
    #[cfg(test)]
    row_updates: u64,
    #[cfg(test)]
    inversions: u64,
    #[cfg(test)]
    inserted_relations: u64,
    #[cfg(test)]
    dependent_relations: u64,
    node_order: String,
    node_order_ops: u64,
}
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
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
        let node_order = std::env::var("PENCIL_NODE_ORDER").unwrap_or_else(|_| "identity".into());
        let mut node_order_ops = 0;
        match node_order.as_str() {
            "identity" => {}
            "reverse" => {
                for column in nodes.iter_mut().flatten() {
                    *column = count - 1 - *column;
                    node_order_ops += 2;
                }
            }
            "affine" => {
                let mut stride = (0x9e37_79b9_7f4a_7c15_u64 % count as u64) as usize;
                while gcd(stride, count) != 1 {
                    stride = (stride + 1) % count;
                }
                let shift = (0x5bd1_e995_u64 % count as u64) as usize;
                for column in nodes.iter_mut().flatten() {
                    *column = (*column * stride + shift) % count;
                    node_order_ops += 3;
                }
            }
            _ => panic!("PENCIL_NODE_ORDER must be identity, reverse or affine"),
        }
        Self {
            field_prime: p,
            modulus: r,
            nodes,
            node_count: count,
            #[cfg(test)]
            pivots: vec![None; count],
            #[cfg(test)]
            rank: 0,
            #[cfg(test)]
            stored_nonzero_coefficients: 0,
            #[cfg(test)]
            row_updates: 0,
            #[cfg(test)]
            inversions: 0,
            #[cfg(test)]
            inserted_relations: 0,
            #[cfg(test)]
            dependent_relations: 0,
            node_order,
            node_order_ops,
        }
    }
    #[cfg(test)]
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

// A fresh exact sparse elimination after each completed walk. Rows with a
// degree-one factor-base column cannot belong to a zero-factor-base
// dependency, so they can be removed before core elimination.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 4 && args[1] == "--cohort",
        "usage: pencil-symbolic --cohort BITS P; freeze protocol before use"
    );
    cohort_main(args[2].parse().unwrap(), args[3].parse().unwrap());
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
    let matrix_coefficient_cap: usize = std::env::var("PENCIL_MATRIX_CAP")
        .ok()
        .map(|value| value.parse().expect("PENCIL_MATRIX_CAP integer"))
        .unwrap_or(10_000_000);
    assert!((1_000_000..=25_000_000).contains(&matrix_coefficient_cap));
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
        json!({"schema":"pencil-symbolic-cohort-setup/v1","bits":bits,"p":p,"r":r,
        "cost_status":"incomplete_graph_and_allocation_accounting",
        "order":order,"cofactor":cofactor,"rational_curve_points":rational_curve_points,
        "projected_points_checked":projected_points_checked,
        "matrix_coefficient_cap":matrix_coefficient_cap,"field":setup_field,
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
    let target_start: usize = std::env::var("PENCIL_TARGET_START")
        .ok()
        .map(|value| value.parse().expect("PENCIL_TARGET_START integer"))
        .unwrap_or(0);
    assert!(target_start + target_limit <= 60);
    let solver_mode = "symbolic".to_string();
    let denominator: usize = std::env::var("PENCIL_SMALL_DENOMINATOR")
        .expect("explicit frozen small-base denominator required")
        .parse()
        .unwrap();
    assert!([2, 4, 8, 16].contains(&denominator));
    for row in rows.into_iter().skip(target_start).take(target_limit) {
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
        let solver = LinearSolver::new(p, r, &rational_y);
        let mut graph = GraphSolver::new(&solver, matrix_coefficient_cap, denominator);
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
                    graph.relation(&points, a, b)
                });
                if recovered.is_some() {
                    solution_key = Some([walk_index, lambda]);
                    break 'walks;
                }
                if let Some(reason) = graph.failure {
                    failure = Some(reason);
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
            json!({"schema":"pencil-symbolic-cohort-target/v1","bits":bits,"p":p,
            "symbolic":graph.detail(),"cost_status":"incomplete_graph_and_allocation_accounting",
            "candidate_over_rho":Value::Null,
            "r":r,"workload_id":workload["workload_id"],"workload_sha256":workload["workload_sha256"],
            "target_index":workload["target_index"],"algorithm_seed":seed.to_string(),"cap":cap,
            "planted":scalar,"recovered":recovered,"verified":verified,"failure":failure,
            "walks":walks,"candidate_lambdas":candidate_lambdas,"verified_relations":verified_relations,
            "rejected_candidates":rejected_candidates,"solution_key":solution_key,
            "solver_mode":solver_mode,
            "matrix_coefficient_cap":matrix_coefficient_cap,
            "solver_nodes":solver.node_count,"solver_rank":graph.collector.basis().rank(),
            "solver_node_order":solver.node_order,"solver_node_order_ops":solver.node_order_ops,
            "solver_stored_nonzero_coefficients":graph.collector.basis().retained_coefficients(),
            "counts":pencil_field,"arithmetic":pencil_arithmetic,"phases":phases,"source_add":source_add,
            "rho":{"recovered":rho.recovered,"verified":rho_verified,"counts":rho.counts,
                   "native":rho.native,"phases":rho.phases,"detail":rho.detail}})
        );
    }
}
