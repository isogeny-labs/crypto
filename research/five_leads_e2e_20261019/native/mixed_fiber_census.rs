//! Complete small three-summand relation oracle census, with cold operation
//! counts. This is a root/lift oracle test, not an end-to-end DLP solver.
use five_leads_study::three_leads::{Elliptic, Field, Point, Rng};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

fn curve(p: u64, a: u64, b: u64) -> Elliptic {
    assert_ne!((4 * a * a * a + 27 * b * b) % p, 0);
    Elliptic {
        f: Field::new(p, 1, 0),
        c: [b, a, 0, 1],
        binary: false,
    }
}
fn primitive(f: &Field) -> u64 {
    let n = f.p - 1;
    let mut factors = Vec::new();
    let mut v = n;
    let mut z = 2;
    while z * z <= v {
        if v % z == 0 {
            factors.push(z);
            while v % z == 0 {
                v /= z;
            }
        }
        z += 1;
    }
    if v > 1 {
        factors.push(v);
    }
    (2..f.p)
        .find(|&g| factors.iter().all(|&q| f.pow(g, n / q) != 1))
        .unwrap()
}
fn coset(f: &Field, g: u64, d: u64, power: u64) -> Vec<u64> {
    let omega = f.pow(g, (f.p - 1) / d);
    let mut x = f.pow(g, power);
    let mut xs = Vec::new();
    for _ in 0..d {
        xs.push(x);
        x = f.mul(x, omega);
    }
    xs.sort_unstable();
    xs.dedup();
    assert_eq!(xs.len(), d as usize);
    xs
}
fn lift(e: &Elliptic, xs: &[u64]) -> Vec<Point> {
    let mut out = Vec::new();
    for &x in xs {
        out.extend(e.lift(x));
    }
    out.sort();
    out
}
fn counts_by_x(e: &Elliptic, xs: &[u64]) -> [usize; 3] {
    let mut count = [0; 3];
    for &x in xs {
        count[e.lift(x).len()] += 1;
    }
    count
}
fn matched_cosets(e: &Elliptic, g: u64, d: u64) -> [Vec<u64>; 3] {
    let mut groups: BTreeMap<[usize; 3], Vec<Vec<u64>>> = BTreeMap::new();
    for power in 0..(e.f.p - 1) / d {
        let xs = coset(&e.f, g, d, power);
        groups.entry(counts_by_x(e, &xs)).or_default().push(xs);
    }
    let (_, choices) = groups
        .into_iter()
        .filter(|(counts, choices)| choices.len() >= 3 && counts[1] + 2 * counts[2] > 0)
        .max_by_key(|(counts, choices)| (counts[1] + 2 * counts[2], choices.len()))
        .expect("three point-count matched tower fibers");
    [choices[0].clone(), choices[1].clone(), choices[2].clone()]
}
fn random_matched(e: &Elliptic, want: [usize; 3], seed: u64) -> (Vec<u64>, u64) {
    let mut rng = Rng(seed);
    let mut draw = 0;
    let mut have = [0; 3];
    let mut xs = BTreeSet::new();
    while have != want {
        draw += 1;
        assert!(draw < 100_000, "random matching exhausted");
        let x = rng.next() % e.f.p;
        if xs.contains(&x) {
            continue;
        }
        let c = e.lift(x).len();
        if have[c] == want[c] {
            continue;
        }
        xs.insert(x);
        have[c] += 1;
    }
    (xs.into_iter().collect(), draw)
}
fn targets(e: &Elliptic, seed: u64) -> Vec<Point> {
    let mut rng = Rng(seed);
    let mut out = BTreeSet::new();
    while out.len() < 20 {
        let x = rng.next() % e.f.p;
        if let Some(p) = e.lift(x).first() {
            out.insert(*p);
        }
    }
    out.into_iter().collect()
}
fn exhaustive(e: &Elliptic, pts: &[Vec<Point>; 3], q: Point) -> Vec<[Point; 3]> {
    let mut out = Vec::new();
    for &a in &pts[0] {
        for &b in &pts[1] {
            for &c in &pts[2] {
                if e.add(e.add(a, b), c) == q {
                    out.push([a, b, c]);
                }
            }
        }
    }
    out.sort();
    out
}
fn solve(e: &Elliptic, pts: &[Vec<Point>; 3], q: Point) -> (Vec<[Point; 3]>, u64, u64, u64) {
    let mut table: BTreeMap<Point, Vec<(Point, Point)>> = BTreeMap::new();
    let mut pairs = 0;
    for &a in &pts[0] {
        for &b in &pts[1] {
            table.entry(e.add(a, b)).or_default().push((a, b));
            pairs += 1;
        }
    }
    let mut out = Vec::new();
    let mut lookups = 0;
    let mut checked = 0;
    for &c in &pts[2] {
        lookups += 1;
        let need = e.add(q, e.neg(c));
        if let Some(rows) = table.get(&need) {
            for &(a, b) in rows {
                checked += 1;
                assert_eq!(e.add(e.add(a, b), c), q);
                out.push([a, b, c]);
            }
        }
    }
    out.sort();
    (out, pairs, lookups, checked)
}
fn run(p: u64, a: u64, b: u64, d: u64, target_seed: u64) {
    let preparer = curve(p, a, b);
    let qs = targets(&preparer, target_seed);
    let count = preparer.f.counts();
    for (target_index, &q) in qs.iter().enumerate() {
        for arm in ["homogeneous", "mixed", "random"] {
            let e = curve(p, a, b);
            let g = primitive(&e.f);
            let chosen = matched_cosets(&e, g, d);
            let mut xs: [Vec<u64>; 3] = std::array::from_fn(|j| {
                if arm == "homogeneous" {
                    chosen[0].clone()
                } else {
                    chosen[j].clone()
                }
            });
            let structured_counts: [[usize; 3]; 3] =
                std::array::from_fn(|j| counts_by_x(&e, &xs[j]));
            let mut random_draws = 0;
            if arm == "random" {
                for j in 0..3 {
                    let (chosen, draws) = random_matched(
                        &e,
                        structured_counts[j],
                        20261019 + p * 100_000 + a * 1000 + b * 100 + d * 10 + j as u64,
                    );
                    xs[j] = chosen;
                    random_draws += draws;
                }
            }
            let pts: [Vec<Point>; 3] = std::array::from_fn(|j| lift(&e, &xs[j]));
            let setup = e.f.counts();
            let (found, pairs, lookups, checked) = solve(&e, &pts, q);
            let total = e.f.counts();
            let verifier = curve(p, a, b);
            let expected = exhaustive(&verifier, &pts, q);
            assert_eq!(found, expected, "incomplete relation oracle");
            let after = verifier.f.counts();
            let result = json!({
                "schema":"mixed-fiber-oracle/v1","p":p,"a":a,"b":b,"d":d,
                "target_seed":target_seed,"target_index":target_index,"target":[q.x,q.y],
                "arm":arm,"fibers":xs,"point_counts":pts.iter().map(Vec::len).collect::<Vec<_>>(),
                "random_draws":random_draws,"pair_insertions":pairs,"third_point_lookups":lookups,
                "verified_lifts":checked,"complete_solutions":found,
                "solver_setup_ops":setup,"solver_total_ops":total,
                "exhaustive_verifier_ops":after,"target_generation_ops_excluded":count,
                "all_solutions_match_exhaustive":true
            });
            println!("{}", serde_json::to_string(&result).unwrap());
        }
    }
}
fn main() {
    for (p, d) in [(97, 8), (193, 12), (257, 16)] {
        for (a, b) in [(2, 3), (5, 7), (8, 11)] {
            run(p, a, b, d, 2026101920 + p * 17 + a * 3 + b);
        }
    }
}
