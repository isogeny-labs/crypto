//! A complete small mixed-fiber relation collector and source DLP recovery.
//! The oracle is an explicitly charged point pair table, not F4 or msolve.
use five_leads_study::three_leads::{
    exhaustive_order, solve_rho, Elliptic, Field, Point, Ring, Rng, StudyInstance,
};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

fn curve(p: u64, a: u64, b: u64) -> Elliptic {
    Elliptic {
        f: Field::new(p, 1, 0),
        c: [b, a, 0, 1],
        binary: false,
    }
}
fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest = 1;
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            largest = d;
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    largest.max(n)
}
fn generator(e: &Elliptic, h: u64) -> Point {
    for x in 0..e.f.p {
        for p in e.lift(x) {
            let g = e.mul(p, h);
            if !g.inf {
                return g;
            }
        }
    }
    panic!("no prime-order generator")
}
fn primitive(f: &Field) -> u64 {
    let mut n = f.p - 1;
    let mut factors = Vec::new();
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            factors.push(d);
            while n % d == 0 {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    (2..f.p)
        .find(|&x| factors.iter().all(|&q| f.pow(x, (f.p - 1) / q) != 1))
        .unwrap()
}
fn coset(f: &Field, g: u64, d: u64, power: u64) -> Vec<u64> {
    let omega = f.pow(g, (f.p - 1) / d);
    let mut x = f.pow(g, power);
    let mut out = Vec::new();
    for _ in 0..d {
        out.push(x);
        x = f.mul(x, omega);
    }
    out.sort_unstable();
    out.dedup();
    assert_eq!(out.len(), d as usize);
    out
}
fn lift_shape(e: &Elliptic, xs: &[u64]) -> [usize; 3] {
    let mut shape = [0; 3];
    for &x in xs {
        shape[e.lift(x).len()] += 1;
    }
    shape
}
fn selected_cosets(e: &Elliptic, g: u64, d: u64) -> Option<[Vec<u64>; 3]> {
    let mut groups: BTreeMap<[usize; 3], Vec<Vec<u64>>> = BTreeMap::new();
    for power in 0..(e.f.p - 1) / d {
        let xs = coset(&e.f, g, d, power);
        groups.entry(lift_shape(e, &xs)).or_default().push(xs);
    }
    let (_, choices) = groups
        .into_iter()
        .filter(|(shape, xs)| xs.len() >= 3 && shape[1] + 2 * shape[2] > 0)
        .max_by_key(|(shape, xs)| (shape[1] + 2 * shape[2], xs.len()))?;
    Some([choices[0].clone(), choices[1].clone(), choices[2].clone()])
}
fn source_curves(p: u64, d: u64) -> Vec<(u64, u64, u64, u64, [Vec<u64>; 3])> {
    let mut out = Vec::new();
    for a in 1..=20 {
        for b in 1..=20 {
            let e = curve(p, a, b);
            if e.f
                .add(e.f.mul(4, e.f.mul(e.f.sq(a), a)), e.f.mul(27, e.f.sq(b)))
                == 0
            {
                continue;
            }
            let order = exhaustive_order(&e);
            let r = largest_prime_factor(order);
            let h = order / r;
            if r < 23 || h % r == 0 {
                continue;
            }
            let Some(cosets) = selected_cosets(&e, primitive(&e.f), d) else {
                continue;
            };
            out.push((a, b, order, r, cosets));
            if out.len() == 3 {
                return out;
            }
        }
    }
    panic!("fewer than three eligible source curves p={p},d={d}")
}
fn random_matched(e: &Elliptic, want: [usize; 3], seed: u64) -> (Vec<u64>, u64) {
    let mut rng = Rng(seed);
    let mut draws = 0;
    let mut shape = [0; 3];
    let mut xs = BTreeSet::new();
    while shape != want {
        draws += 1;
        assert!(draws < 100_000);
        let x = rng.next() % e.f.p;
        if xs.contains(&x) {
            continue;
        }
        let count = e.lift(x).len();
        if shape[count] == want[count] {
            continue;
        }
        xs.insert(x);
        shape[count] += 1;
    }
    (xs.into_iter().collect(), draws)
}
fn lifted(e: &Elliptic, xs: &[u64]) -> Vec<Point> {
    let mut out = Vec::new();
    for &x in xs {
        out.extend(e.lift(x));
    }
    out.sort();
    out
}
struct Elim {
    ring: Ring,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    row_ops: u64,
}
impl Elim {
    fn new(r: u64, unknowns: usize) -> Self {
        Self {
            ring: Ring::new(r),
            pivots: vec![None; unknowns],
            rank: 0,
            row_ops: 0,
        }
    }
    fn insert(&mut self, mut row: Vec<u64>) -> Option<u64> {
        let n = self.pivots.len();
        assert_eq!(row.len(), n + 1);
        for j in 0..n {
            if row[j] == 0 {
                continue;
            }
            if let Some(pivot) = &self.pivots[j] {
                let factor = row[j];
                for k in j..=n {
                    row[k] = self.ring.sub(row[k], self.ring.mul(factor, pivot[k]));
                    self.row_ops += 1;
                }
            } else {
                let inv = self.ring.inv(row[j]);
                for value in &mut row[j..] {
                    *value = self.ring.mul(*value, inv);
                    self.row_ops += 1;
                }
                self.rank += 1;
                if j == n - 1 {
                    return Some(row[n]);
                }
                self.pivots[j] = Some(row);
                return None;
            }
        }
        assert_eq!(row[n], 0, "inconsistent relation system");
        None
    }
}
fn solve(
    e: &Elliptic,
    p: Point,
    q: Point,
    h: u64,
    r: u64,
    xs: &[Vec<u64>; 3],
    seed: u64,
) -> serde_json::Value {
    e.f.reset();
    let mut draw_count = 0;
    let points: [Vec<Point>; 3] = std::array::from_fn(|j| lifted(e, &xs[j]));
    let mut projected = BTreeSet::new();
    for list in &points {
        for &pt in list {
            let hp = e.mul(pt, h);
            if !hp.inf {
                projected.insert(hp);
            }
        }
    }
    let columns: BTreeMap<Point, usize> = projected
        .into_iter()
        .enumerate()
        .map(|(j, pt)| (pt, j))
        .collect();
    let unknowns = columns.len() + 1;
    let mut elim = Elim::new(r, unknowns);
    let mut pairs: BTreeMap<Point, Vec<(Point, Point)>> = BTreeMap::new();
    let mut pair_count = 0;
    for &a in &points[0] {
        for &b in &points[1] {
            pairs.entry(e.add(a, b)).or_default().push((a, b));
            pair_count += 1;
        }
    }
    let setup = e.f.counts();
    let mut rng = Rng(seed);
    let (mut trials, mut lookups, mut relations) = (0, 0, 0);
    let mut recovered = None;
    let cap = 100 * r + 10_000;
    'search: while trials < cap {
        trials += 1;
        draw_count += 2;
        let a = rng.next() % r;
        let b = rng.next() % r;
        let rhs = e.add(e.mul(p, a), e.mul(q, b));
        for &c in &points[2] {
            lookups += 1;
            let needed = e.add(rhs, e.neg(c));
            let Some(rows) = pairs.get(&needed) else {
                continue;
            };
            for &(x, y) in rows {
                relations += 1;
                let mut row = vec![0; unknowns + 1];
                for pt in [x, y, c] {
                    let projection = e.mul(pt, h);
                    if let Some(&col) = columns.get(&projection) {
                        row[col] = elim.ring.add(row[col], 1);
                    }
                }
                row[unknowns - 1] = elim.ring.neg(elim.ring.mul(h % r, b));
                row[unknowns] = elim.ring.mul(h % r, a);
                if let Some(k) = elim.insert(row) {
                    recovered = Some(k);
                    break 'search;
                }
            }
        }
    }
    let before_verify = e.f.counts();
    let verified = recovered.map(|k| e.mul(p, k) == q).unwrap_or(false);
    let total = e.f.counts();
    let scalars = elim.ring.counts.get();
    json!({"recovered":recovered,"verified":verified,"exhausted":recovered.is_none(),
        "cap":cap,"trials":trials,"lookups":lookups,"relations":relations,
        "factor_columns":columns.len(),"rank":elim.rank,"pair_insertions":pair_count,
        "rng_draws_uncharged":draw_count,"table_lookups_uncharged":lookups,
        "row_operations_uncharged":elim.row_ops,"scalar_adds_uncharged":scalars.add,
        "scalar_muls_uncharged":scalars.mul,"scalar_inversions_uncharged":scalars.inv,
        "setup_counts":setup,"preverification_counts":before_verify,"total_counts":total,
        "logical_table_entries":pair_count,"logical_matrix_entries":elim.pivots.len() * (unknowns + 1)})
}
fn main() {
    for (p, d) in [(97, 8), (193, 12), (257, 16)] {
        for (a, b, order, r, chosen) in source_curves(p, d) {
            let preparer = curve(p, a, b);
            let h = order / r;
            assert!(r >= 23 && h % r != 0, "ineligible source subgroup");
            let g = generator(&preparer, h);
            assert!(preparer.mul(g, r).inf);
            let mut target_rng = Rng(20261020 + p * 10_000 + a * 100 + b);
            let mut targets = BTreeSet::new();
            while targets.len() < 20.min((r - 1) as usize) {
                targets.insert(1 + target_rng.next() % (r - 1));
            }
            let target_scalars: Vec<u64> = targets.into_iter().collect();
            for (target_index, &secret) in target_scalars.iter().enumerate() {
                let target = preparer.mul(g, secret);
                let inst = StudyInstance {
                    curve: curve(p, a, b),
                    order,
                    r,
                    h,
                    generator: g,
                    family: "mixed_coset_study".into(),
                    parameter: d,
                    evidence: json!({"exhaustive_order":true}),
                };
                let rho_seed = 20261020 + p * 100_000 + a * 1000 + b * 10 + target_index as u64;
                let rho = solve_rho(&inst, target, rho_seed, 100 * r + 10_000, true, false);
                let rho_check_curve = curve(p, a, b);
                let rho_correct = rho
                    .recovered
                    .map(|k| rho_check_curve.mul(g, k) == target)
                    .unwrap_or(false);
                let rho_verify_counts = rho_check_curve.f.counts();
                for arm in ["homogeneous", "mixed", "random"] {
                    let e = curve(p, a, b);
                    let mut xs: [Vec<u64>; 3] = std::array::from_fn(|j| {
                        if arm == "homogeneous" {
                            chosen[0].clone()
                        } else {
                            chosen[j].clone()
                        }
                    });
                    let mut selection_draws = 0;
                    if arm == "random" {
                        for j in 0..3 {
                            let shape = lift_shape(&e, &xs[j]);
                            let (set, draws) = random_matched(
                                &e,
                                shape,
                                20261019 + p * 100_000 + a * 1000 + b * 100 + d * 10 + j as u64,
                            );
                            xs[j] = set;
                            selection_draws += draws;
                        }
                    }
                    let out = solve(&e, g, target, h, r, &xs, rho_seed);
                    println!(
                        "{}",
                        json!({"schema":"mixed-fiber-dlp/v1","p":p,"a":a,"b":b,"d":d,
                        "order":order,"r":r,"h":h,"generator":[g.x,g.y],"target_index":target_index,
                        "target":[target.x,target.y],"planted_scalar_check_only":secret,
                        "arm":arm,"fibers":xs,"selection_draws_uncharged":selection_draws,
                        "candidate":out,"rho":{"recovered":rho.recovered,"verified":rho_correct,
                        "counts":rho.counts + rho_verify_counts,"native":rho.native}})
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_nine_sources_have_twenty_distinct_nonzero_prime_order_targets() {
        for (p, d) in [(97, 8), (193, 12), (257, 16)] {
            for (a, b, order, r, _) in source_curves(p, d) {
                let e = curve(p, a, b);
                assert_eq!(order, exhaustive_order(&e));
                assert!(
                    r >= 23 && (order / r) % r != 0,
                    "p={p},a={a},b={b},d={d},N={order},r={r}"
                );
            }
        }
    }
}
