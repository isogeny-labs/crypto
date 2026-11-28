//! Exact signed triple-sum support and target intersections on toy fields.
//! Run: cargo run --release --example triple_intersection
use std::collections::{HashMap, HashSet};
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pt {
    x: u64,
    y: u64,
    o: bool,
}
const O: Pt = Pt {
    x: 0,
    y: 0,
    o: true,
};
fn mm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn pw(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if e & 1 == 1 {
            z = mm(z, a, p)
        }
        a = mm(a, a, p);
        e >>= 1;
    }
    z
}
fn neg(q: Pt, p: u64) -> Pt {
    if q.o {
        q
    } else {
        Pt {
            y: if q.y == 0 { 0 } else { p - q.y },
            ..q
        }
    }
}
fn add(u: Pt, v: Pt, p: u64, adds: &mut u64) -> Pt {
    *adds += 1;
    if u.o {
        return v;
    }
    if v.o {
        return u;
    }
    if u.x == v.x && (u.y + v.y).is_multiple_of(p) {
        return O;
    }
    let (num, den) = if u.x == v.x {
        ((3 * mm(u.x, u.x, p) + p - 3) % p, (2 * u.y) % p)
    } else {
        ((v.y + p - u.y) % p, (v.x + p - u.x) % p)
    };
    let lam = mm(num, pw(den, p - 2, p), p);
    let x = (mm(lam, lam, p) + 2 * p - u.x - v.x) % p;
    Pt {
        x,
        y: (mm(lam, (u.x + p - x) % p, p) + p - u.y) % p,
        o: false,
    }
}
fn lift(x: u64, p: u64) -> Option<Pt> {
    let rhs = (mm(mm(x, x, p), x, p) + p - mm(3, x, p) + 40) % p;
    if rhs == 0 {
        return Some(Pt { x, y: 0, o: false });
    }
    let y = pw(rhs, (p + 1) / 4, p);
    (mm(y, y, p) == rhs).then_some(Pt {
        x,
        y: y.min(p - y),
        o: false,
    })
}
fn roots(p: u64, d: u64, z: u64) -> Vec<Pt> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut x = 1;
    for _ in 0..d {
        assert!(seen.insert(x), "root generator has order below d");
        if let Some(q) = lift(x, p) {
            if q.y != 0 {
                out.push(q)
            }
        }
        x = mm(x, z, p)
    }
    assert_eq!(x, 1);
    out
}
fn rnd(state: &mut u64) -> u64 {
    let mut z = (*state).wrapping_add(0x9e3779b97f4a7c15);
    *state = z;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}
fn random_base(p: u64, b: usize, seed: u64) -> Vec<Pt> {
    let mut st = seed;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    while out.len() < b {
        let x = 1 + rnd(&mut st) % (p - 1);
        if seen.insert(x) {
            if let Some(q) = lift(x, p) {
                if q.y != 0 {
                    out.push(q)
                }
            }
        }
    }
    out
}
#[derive(Clone, Copy)]
struct Wit {
    ids: [u16; 3],
    mask: u8,
}
struct Inventory {
    buckets: HashMap<Pt, Vec<Wit>>,
    triples: u64,
    xdegree: usize,
    additions: u64,
}
fn inventory(base: &[Pt], p: u64) -> Inventory {
    let mut buckets: HashMap<Pt, Vec<Wit>> = HashMap::new();
    let mut xs = HashSet::new();
    let mut triples = 0;
    let mut additions = 0;
    for i in 0..base.len() {
        for j in i + 1..base.len() {
            for k in j + 1..base.len() {
                for mask in 0..8 {
                    let a = if mask & 1 == 0 {
                        base[i]
                    } else {
                        neg(base[i], p)
                    };
                    let b = if mask & 2 == 0 {
                        base[j]
                    } else {
                        neg(base[j], p)
                    };
                    let c = if mask & 4 == 0 {
                        base[k]
                    } else {
                        neg(base[k], p)
                    };
                    let s = add(add(a, b, p, &mut additions), c, p, &mut additions);
                    triples += 1;
                    if !s.o {
                        xs.insert(s.x);
                    }
                    buckets.entry(s).or_default().push(Wit {
                        ids: [i as u16, j as u16, k as u16],
                        mask,
                    });
                }
            }
        }
    }
    Inventory {
        buckets,
        triples,
        xdegree: xs.len(),
        additions,
    }
}
fn mul(mut k: u64, mut q: Pt, p: u64, adds: &mut u64) -> Pt {
    let mut out = O;
    while k > 0 {
        if k & 1 == 1 {
            out = add(out, q, p, adds)
        }
        k >>= 1;
        if k > 0 {
            q = add(q, q, p, adds)
        }
    }
    out
}
fn point_count(p: u64) -> u64 {
    let mut n = 1;
    for x in 0..p {
        let rhs = (mm(mm(x, x, p), x, p) + p - mm(3, x, p) + 40) % p;
        n += if rhs == 0 {
            1
        } else if pw(rhs, (p - 1) / 2, p) == 1 {
            2
        } else {
            0
        };
    }
    n
}
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1
    }
    true
}
fn point_from_wit(base: &[Pt], w: Wit, p: u64, adds: &mut u64) -> Pt {
    let q = |j: usize| {
        let u = base[w.ids[j] as usize];
        if w.mask & (1 << j) == 0 {
            u
        } else {
            neg(u, p)
        }
    };
    add(add(q(0), q(1), p, adds), q(2), p, adds)
}
fn disjoint(u: Wit, v: Wit) -> bool {
    u.ids.iter().all(|i| !v.ids.contains(i))
}
fn six_term_coverage(base: &[Pt], p: u64) -> (u64, usize, u64, u64) {
    assert_eq!(base.len(), 8);
    let mut targets: HashMap<Pt, u64> = HashMap::new();
    let mut tuples = 0;
    let mut pair_collisions = 0;
    let mut additions = 0;
    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                for l in k + 1..6 {
                    for m in l + 1..7 {
                        for n in m + 1..8 {
                            let indices = [i, j, k, l, m, n];
                            for signs in 0..64 {
                                let first = base[indices[0]];
                                let mut sum = if signs & 1 == 0 { first } else { neg(first, p) };
                                for (position, &index) in indices.iter().enumerate().skip(1) {
                                    let point = base[index];
                                    let term = if signs & (1 << position) == 0 {
                                        point
                                    } else {
                                        neg(point, p)
                                    };
                                    sum = add(sum, term, p, &mut additions);
                                }
                                let prior = targets.entry(sum).or_default();
                                pair_collisions += *prior;
                                *prior += 1;
                                tuples += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!((tuples, additions), (1_792, 8_960));
    (tuples, targets.len(), pair_collisions, additions)
}
fn measure(base: &[Pt], p: u64, h: u64, label: &str) {
    let mut projection_adds = 0;
    let projected = base
        .iter()
        .map(|&q| mul(h, q, p, &mut projection_adds))
        .collect::<Vec<_>>();
    for (kind, pts, projection_adds) in [
        ("full", base.to_vec(), 0),
        ("projected", projected, projection_adds),
    ] {
        let inv = inventory(&pts, p);
        let mut tally = 0;
        let planted = pts.iter().take(6).fold(O, |s, &q| add(s, q, p, &mut tally));
        let mut targets = vec![planted];
        for k in [1, 5, 17, 101, 513] {
            targets.push(mul(k, pts[0], p, &mut tally));
        }
        let mut intersections = Vec::new();
        let mut translate_additions = 0;
        for &target in &targets {
            let n = inv
                .buckets
                .keys()
                .filter(|&&s| {
                    let c = add(target, neg(s, p), p, &mut translate_additions);
                    inv.buckets.contains_key(&c)
                })
                .count();
            intersections.push(n);
        }
        let mut witness = None;
        let mut witness_probes = 0;
        let mut candidate_checks = 0;
        let mut wadds = 0;
        let mut keys = inv.buckets.keys().copied().collect::<Vec<_>>();
        keys.sort_by_key(|q| (q.o, q.x, q.y));
        'outer: for s in keys {
            let us = &inv.buckets[&s];
            witness_probes += 1;
            let c = add(planted, neg(s, p), p, &mut wadds);
            if let Some(vs) = inv.buckets.get(&c) {
                for &u in us {
                    for &v in vs {
                        candidate_checks += 1;
                        if disjoint(u, v) {
                            witness = Some((u, v));
                            break 'outer;
                        }
                    }
                }
            }
        }
        let (u, v) = witness.expect("planted six-term target must have a disjoint witness");
        let left = point_from_wit(&pts, u, p, &mut wadds);
        let right = point_from_wit(&pts, v, p, &mut wadds);
        assert_eq!(add(left, right, p, &mut wadds), planted);
        println!("p={p} B={} {label} {kind} zero_base_points={} triples={} group_support={} squarefree_x_degree={} collision_excess={} projection_adds={projection_adds} prep_adds={} intersections={intersections:?} translate_adds={translate_additions} witness_key_probes={witness_probes} candidate_checks={candidate_checks} witness_adds={wadds}",pts.len(),pts.iter().filter(|q|q.o).count(),inv.triples,inv.buckets.len(),inv.xdegree,inv.triples-inv.buckets.len() as u64,inv.additions);
        if pts.len() == 8 {
            let (tuples, covered, pair_collisions, additions) = six_term_coverage(&pts, p);
            println!("p={p} B=8 {label} {kind} six_distinct_signed_tuples={tuples} six_target_coverage={covered} six_pair_collisions={pair_collisions} six_coverage_additions={additions}");
        }
    }
}
fn main() {
    for (p, d, z, h, r, n) in [
        (8191, 42, 7036, 136, 59, 8024),
        (131071, 102, 37206, 37, 3541, 131017),
        (524287, 171, 477103, 32, 16421, 525472),
    ] {
        assert!(prime(p));
        assert_eq!(point_count(p), n);
        assert_eq!(h * r, n);
        assert!(prime(r));
        let all = roots(p, d, z);
        assert!(all
            .iter()
            .all(|&q| mul(r, mul(h, q, p, &mut 0), p, &mut 0).o));
        for size in [8, all.len()] {
            if size > all.len() {
                continue;
            }
            measure(&all[..size], p, h, "root");
            measure(&random_base(p, size, 2026), p, h, "random");
        }
        let mut projected_controls = Vec::new();
        let mut control_projection_adds = 0;
        let mut control_six_adds = 0;
        for seed in 2026..2042 {
            let random = random_base(p, 8, seed);
            let projected = random
                .iter()
                .map(|&q| mul(h, q, p, &mut control_projection_adds))
                .collect::<Vec<_>>();
            let (_, coverage, _, additions) = six_term_coverage(&projected, p);
            projected_controls.push(coverage);
            control_six_adds += additions;
        }
        println!("p={p} B=8 projected_random_coverage_16={projected_controls:?} control_projection_adds={control_projection_adds} control_six_adds={control_six_adds}");
        if p != 8191 {
            let mut root_projection_adds = 0;
            let root_projected = all[..8]
                .iter()
                .map(|&q| mul(h, q, p, &mut root_projection_adds))
                .collect::<Vec<_>>();
            let (_, root_coverage, root_pairs, _) = six_term_coverage(&root_projected, p);
            let mut controls = Vec::new();
            let mut control_pairs = Vec::new();
            let mut controls_with_zero = 0;
            for seed in 2026..2154 {
                let random = random_base(p, 8, seed);
                let projected = random
                    .iter()
                    .map(|&q| mul(h, q, p, &mut control_projection_adds))
                    .collect::<Vec<_>>();
                controls_with_zero += usize::from(projected.iter().any(|q| q.o));
                let (_, coverage, pairs, _) = six_term_coverage(&projected, p);
                controls.push(coverage);
                control_pairs.push(pairs);
            }
            let total: usize = controls.iter().sum();
            let at_least_root = controls
                .iter()
                .filter(|&&value| value >= root_coverage)
                .count();
            println!("p={p} B=8 root_projected_coverage={root_coverage} control_128_mean={:.3} control_128_min={} control_128_max={} control_128_at_least_root={at_least_root} controls_with_zero={controls_with_zero}", total as f64 / controls.len() as f64, controls.iter().min().unwrap(), controls.iter().max().unwrap());
            println!("p={p} B=8 root_projected_pair_collisions={root_pairs} control_128_pair_mean={:.3} control_128_pair_min={} control_128_pair_max={}", control_pairs.iter().sum::<u64>() as f64 / control_pairs.len() as f64, control_pairs.iter().min().unwrap(), control_pairs.iter().max().unwrap());
            let mut window_coverages = Vec::new();
            let mut window_pairs = Vec::new();
            let mut nonzero_window_coverages = Vec::new();
            for start in 0..all.len() {
                let window = (0..8)
                    .map(|offset| all[(start + offset) % all.len()])
                    .collect::<Vec<_>>();
                let projected = window
                    .iter()
                    .map(|&q| mul(h, q, p, &mut root_projection_adds))
                    .collect::<Vec<_>>();
                let (_, coverage, pairs, _) = six_term_coverage(&projected, p);
                window_coverages.push(coverage);
                window_pairs.push(pairs);
                if projected.iter().all(|q| !q.o) {
                    nonzero_window_coverages.push(coverage);
                }
            }
            let window_total: usize = window_coverages.iter().sum();
            let windows_at_least_first = window_coverages
                .iter()
                .filter(|&&value| value >= root_coverage)
                .count();
            println!("p={p} B=8 root_windows={} window_mean={:.3} window_min={} window_max={} windows_at_least_first={windows_at_least_first}", window_coverages.len(), window_total as f64 / window_coverages.len() as f64, window_coverages.iter().min().unwrap(), window_coverages.iter().max().unwrap());
            println!("p={p} B=8 root_window_pair_mean={:.3} root_window_pair_min={} root_window_pair_max={}", window_pairs.iter().sum::<u64>() as f64 / window_pairs.len() as f64, window_pairs.iter().min().unwrap(), window_pairs.iter().max().unwrap());
            let nonzero_total: usize = nonzero_window_coverages.iter().sum();
            println!("p={p} B=8 nonzero_root_windows={} nonzero_window_mean={:.3} nonzero_window_min={} nonzero_window_max={}", nonzero_window_coverages.len(), nonzero_total as f64 / nonzero_window_coverages.len() as f64, nonzero_window_coverages.iter().min().unwrap(), nonzero_window_coverages.iter().max().unwrap());
        }
        if p == 131071 {
            let survivors = all
                .iter()
                .copied()
                .filter(|&q| !mul(h, q, p, &mut 0).o)
                .collect::<Vec<_>>();
            assert_eq!(survivors.len(), 53);
            measure(&survivors, p, h, "root_nonzero_sensitivity");
            measure(
                &random_base(p, survivors.len(), 2027),
                p,
                h,
                "random_nonzero_sensitivity",
            );
        }
    }
}
