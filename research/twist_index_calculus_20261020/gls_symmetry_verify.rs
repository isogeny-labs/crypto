//! Independent native audit of the GLS twist symmetry NDJSON records.
//! rustc -O gls_symmetry_verify.rs -o /tmp/gls_symmetry_verify
use std::collections::{HashMap, HashSet};
use std::fs;
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct F(u64, u64);
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Q {
    x: F,
    y: F,
    zero: bool,
}
const I: Q = Q {
    x: F(0, 0),
    y: F(0, 0),
    zero: true,
};
#[derive(Clone, Copy)]
struct C {
    p: u64,
    d: u64,
    a: F,
    b: F,
    r: u64,
    cof: u64,
    alpha: F,
}
fn md(x: i128, p: u64) -> u64 {
    x.rem_euclid(p as i128) as u64
}
fn powm(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut z = 1;
    while n > 0 {
        if n % 2 == 1 {
            z = (z as u128 * x as u128 % p as u128) as u64;
        }
        x = (x as u128 * x as u128 % p as u128) as u64;
        n /= 2;
    }
    z
}
fn prime(n: u64) -> bool {
    n >= 2 && (2..=n / 2).all(|v| n % v != 0)
}
fn plus(x: F, y: F, p: u64) -> F {
    F((x.0 + y.0) % p, (x.1 + y.1) % p)
}
fn opp(x: F, p: u64) -> F {
    F(md(-(x.0 as i128), p), md(-(x.1 as i128), p))
}
fn minus(x: F, y: F, p: u64) -> F {
    plus(x, opp(y, p), p)
}
fn times(x: F, y: F, c: &C) -> F {
    F(
        md(
            x.0 as i128 * y.0 as i128 + c.d as i128 * x.1 as i128 * y.1 as i128,
            c.p,
        ),
        md(x.0 as i128 * y.1 as i128 + x.1 as i128 * y.0 as i128, c.p),
    )
}
fn power(mut x: F, mut n: u64, c: &C) -> F {
    let mut z = F(1, 0);
    while n > 0 {
        if n % 2 == 1 {
            z = times(z, x, c);
        }
        x = times(x, x, c);
        n /= 2;
    }
    z
}
fn inverse(x: F, c: &C) -> F {
    let norm = md(
        x.0 as i128 * x.0 as i128 - c.d as i128 * x.1 as i128 * x.1 as i128,
        c.p,
    );
    let v = powm(norm, c.p - 2, c.p);
    F(x.0 * v % c.p, md(-(x.1 as i128), c.p) * v % c.p)
}
fn ne(q: Q, c: &C) -> Q {
    if q.zero {
        q
    } else {
        Q {
            y: opp(q.y, c.p),
            ..q
        }
    }
}
fn add(u: Q, v: Q, c: &C) -> Q {
    if u.zero {
        return v;
    }
    if v.zero {
        return u;
    }
    if u.x == v.x && plus(u.y, v.y, c.p) == F(0, 0) {
        return I;
    }
    let slope = if u == v {
        let num = plus(times(F(3, 0), times(u.x, u.x, c), c), c.a, c.p);
        times(num, inverse(times(F(2, 0), u.y, c), c), c)
    } else {
        times(minus(v.y, u.y, c.p), inverse(minus(v.x, u.x, c.p), c), c)
    };
    let x = minus(minus(times(slope, slope, c), u.x, c.p), v.x, c.p);
    let y = minus(times(slope, minus(u.x, x, c.p), c), u.y, c.p);
    Q { x, y, zero: false }
}
fn mult(mut n: u64, mut q: Q, c: &C) -> Q {
    let mut z = I;
    while n > 0 {
        if n % 2 == 1 {
            z = add(z, q, c);
        }
        q = add(q, q, c);
        n /= 2;
    }
    z
}
fn psi(q: Q, c: &C) -> Q {
    if q.zero {
        return q;
    }
    let f = |x: F| F(x.0, md(-(x.1 as i128), c.p));
    let a2 = times(c.alpha, c.alpha, c);
    let a3 = times(a2, c.alpha, c);
    Q {
        x: times(a2, f(q.x), c),
        y: times(a3, f(q.y), c),
        zero: false,
    }
}
fn valid(q: Q, c: &C) -> bool {
    if q.zero {
        return true;
    }
    times(q.y, q.y, c)
        == plus(
            plus(times(times(q.x, q.x, c), q.x, c), times(c.a, q.x, c), c.p),
            c.b,
            c.p,
        )
}
fn after<'a>(s: &'a str, k: &str) -> &'a str {
    let pattern = format!("\"{}\":", k);
    &s[s.find(&pattern).unwrap_or_else(|| panic!("missing {}", k)) + pattern.len()..]
}
fn number(s: &str, k: &str) -> u64 {
    after(s, k)
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}
fn array(s: &str, k: &str) -> Vec<u64> {
    let t = after(s, k);
    let end = t.find(']').unwrap();
    t[1..end]
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}
fn point(a: &[u64]) -> Q {
    assert_eq!(a.len(), 4);
    Q {
        x: F(a[0], a[1]),
        y: F(a[2], a[3]),
        zero: false,
    }
}
fn build(c: &C, b: usize) -> Vec<Q> {
    let mut sq: HashMap<F, Vec<F>> = HashMap::new();
    for y0 in 0..c.p {
        for y1 in 0..c.p {
            let y = F(y0, y1);
            sq.entry(times(y, y, c)).or_default().push(y);
        }
    }
    let mut seen = HashSet::new();
    let mut base = Vec::new();
    for x0 in 0..c.p {
        for x1 in 0..c.p {
            let x = F(x0, x1);
            let rhs = plus(
                plus(times(times(x, x, c), x, c), times(c.a, x, c), c.p),
                c.b,
                c.p,
            );
            if let Some(ys) = sq.get(&rhs) {
                for &y in ys {
                    let u = mult(c.cof, Q { x, y, zero: false }, c);
                    if u.zero {
                        continue;
                    }
                    let v = psi(u, c);
                    let orbit = [u, v, ne(u, c), ne(v, c)];
                    if orbit.iter().any(|x| seen.contains(x)) {
                        continue;
                    }
                    for x in orbit {
                        seen.insert(x);
                    }
                    base.push(u);
                    base.push(v);
                    if base.len() == b {
                        return base;
                    }
                }
            }
        }
    }
    panic!("base incomplete")
}
fn rank(rows: &HashSet<Vec<u64>>, r: u64, n: usize) -> usize {
    let mut pivots = vec![None::<Vec<u64>>; n];
    let mut count = 0;
    for row in rows {
        let mut v = row.clone();
        for j in 0..n {
            if v[j] == 0 {
                continue;
            }
            if let Some(p) = &pivots[j] {
                let t = v[j];
                for k in j..n {
                    v[k] = md(v[k] as i128 - t as i128 * p[k] as i128, r);
                }
            } else {
                let inv = powm(v[j], r - 2, r);
                for k in j..n {
                    v[k] = (v[k] as u128 * inv as u128 % r as u128) as u64;
                }
                pivots[j] = Some(v);
                count += 1;
                break;
            }
        }
    }
    count
}
fn census(c: &C, g: Q, b: usize, lambda: u64) -> (u64, u64, u64, usize, usize, usize, usize) {
    let base = build(c, b);
    let mut log = HashMap::new();
    let mut q = I;
    for k in 1..c.r {
        q = add(q, g, c);
        assert!(log.insert(q, k).is_none());
    }
    assert_eq!(add(q, g, c), I);
    for pair in base.chunks_exact(2) {
        assert!(valid(pair[0], c) && valid(pair[1], c));
        assert_eq!(psi(pair[0], c), pair[1]);
        assert_eq!(mult(lambda, pair[0], c), pair[1]);
        assert_eq!(psi(pair[1], c), ne(pair[0], c));
    }
    let mut covered = HashSet::new();
    let mut flats = HashSet::new();
    let mut folds = HashSet::new();
    let mut orbit_reps = HashSet::new();
    let mut representative_folds = HashSet::new();
    let mut raw = 0;
    let mut zero = 0;
    for i in 0..b {
        for j in i + 1..b {
            for k in j + 1..b {
                for signs in 0..8u8 {
                    let idx = [i, j, k];
                    let mut ps = [I; 3];
                    let mut flat = vec![0; b];
                    let mut fold = vec![0; b / 2];
                    for z in 0..3 {
                        let neg = signs & (1 << z) != 0;
                        ps[z] = if neg {
                            ne(base[idx[z]], c)
                        } else {
                            base[idx[z]]
                        };
                        flat[idx[z]] = if neg { c.r - 1 } else { 1 };
                        let weight = if idx[z] % 2 == 0 { 1 } else { lambda };
                        fold[idx[z] / 2] = md(
                            fold[idx[z] / 2] as i128
                                + if neg {
                                    -(weight as i128)
                                } else {
                                    weight as i128
                                },
                            c.r,
                        );
                    }
                    let sum = add(add(ps[0], ps[1], c), ps[2], c);
                    let transformed = |v: &[u64]| {
                        let mut out = vec![0; b];
                        for h in 0..b / 2 {
                            out[2 * h] = md(-(v[2 * h + 1] as i128), c.r);
                            out[2 * h + 1] = v[2 * h];
                        }
                        out
                    };
                    let mut rotated = flat.clone();
                    let mut orbit = Vec::with_capacity(4);
                    for power in 0..4 {
                        let weighted: Vec<u64> = (0..b / 2)
                            .map(|h| md(rotated[2 * h] as i128
                                + lambda as i128 * rotated[2 * h + 1] as i128, c.r))
                            .collect();
                        let factor = powm(lambda, power, c.r);
                        assert_eq!(weighted, fold.iter().map(|x| x * factor % c.r).collect::<Vec<_>>());
                        let mut transformed_sum = I;
                        for (index, &coefficient) in rotated.iter().enumerate() {
                            if coefficient == 1 {
                                transformed_sum = add(transformed_sum, base[index], c);
                            } else if coefficient == c.r - 1 {
                                transformed_sum = add(transformed_sum, ne(base[index], c), c);
                            } else {
                                assert_eq!(coefficient, 0);
                            }
                        }
                        assert_eq!(transformed_sum, mult(factor, sum, c));
                        orbit.push(rotated.clone());
                        rotated = transformed(&rotated);
                    }
                    assert_eq!(rotated, flat);
                    assert_eq!(orbit.iter().collect::<HashSet<_>>().len(), 4);
                    if sum.zero {
                        zero += 1;
                        continue;
                    }
                    raw += 1;
                    let representative = orbit.into_iter().min().unwrap();
                    if orbit_reps.insert(representative.clone()) {
                        let rep_fold: Vec<u64> = (0..b / 2)
                            .map(|h| md(representative[2 * h] as i128
                                + lambda as i128 * representative[2 * h + 1] as i128, c.r))
                            .collect();
                        representative_folds.insert(rep_fold);
                    }
                    covered.insert(*log.get(&sum).unwrap());
                    flats.insert(flat);
                    folds.insert(fold);
                }
            }
        }
    }
    let fr = rank(&flats, c.r, b);
    let qr = rank(&folds, c.r, b / 2);
    assert_eq!(orbit_reps.len() * 4, raw as usize);
    assert_eq!(rank(&representative_folds, c.r, b / 2), qr);
    (
        covered.len() as u64,
        raw,
        zero,
        flats.len(),
        folds.len(),
        fr,
        qr,
    )
}
fn recount(c: &C) -> u64 {
    let mut total = 1u64;
    for a in 0..c.p {
        for b in 0..c.p {
            let x = F(a, b);
            let rhs = plus(
                plus(times(times(x, x, c), x, c), times(c.a, x, c), c.p),
                c.b,
                c.p,
            );
            if rhs == F(0, 0) {
                total += 1;
                continue;
            }
            let norm = md(
                rhs.0 as i128 * rhs.0 as i128 - c.d as i128 * rhs.1 as i128 * rhs.1 as i128,
                c.p,
            );
            total += if powm(norm, (c.p - 1) / 2, c.p) == 1 {
                2
            } else {
                0
            };
        }
    }
    total
}
fn main() {
    let path = std::env::args().nth(1).expect("path to NDJSON");
    let raw = fs::read_to_string(path).unwrap();
    let mut curves = HashMap::new();
    let mut ncurves = 0;
    let mut ncensus = 0;
    let mut nsolves = 0;
    let mut nrho = 0;
    let mut negatives = 0;
    let mut matched: HashMap<(u64, u64, u64), HashMap<String, (Vec<u64>, u64)>> = HashMap::new();
    for line in raw.lines() {
        if line.contains("\"kind\":\"curve\"") {
            let p = number(line, "p");
            let a = number(line, "a");
            let b = number(line, "b_curve");
            let d = number(line, "d");
            let tw0 = array(line, "c");
            let tw = F(tw0[0], tw0[1]);
            let r = number(line, "r");
            let cof = number(line, "cofactor");
            assert!(prime(p) && prime(r));
            assert_eq!(powm(d, (p - 1) / 2, p), p - 1);
            assert!((2..d).all(|x| powm(x, (p - 1) / 2, p) != p - 1));
            let mut c = C {
                p,
                d,
                a: F(0, 0),
                b: F(0, 0),
                r,
                cof,
                alpha: F(0, 0),
            };
            assert_eq!(power(tw, (p * p - 1) / 2, &c), F(p - 1, 0));
            for a0 in 0..=tw.0 {
                for b0 in 1..p {
                    if (a0, b0) == (tw.0, tw.1) {
                        break;
                    }
                    assert_eq!(power(F(a0, b0), (p * p - 1) / 2, &c), F(1, 0));
                }
            }
            let t2 = times(tw, tw, &c);
            let t3 = times(t2, tw, &c);
            c.a = times(F(a, 0), t2, &c);
            c.b = times(F(b, 0), t3, &c);
            c.alpha = power(tw, p * p - 1 - (p - 1) / 2, &c);
            assert_eq!(recount(&c), number(line, "order"));
            assert_eq!(number(line, "order"), r * cof);
            let g = point(&array(line, "generator"));
            assert!(valid(g, &c) && !g.zero);
            assert_eq!(mult(r, g, &c), I);
            let lam = number(line, "lambda");
            assert_eq!(lam * lam % r, r - 1);
            assert_eq!(psi(g, &c), mult(lam, g, &c));
            assert_eq!(psi(psi(g, &c), &c), ne(g, &c));
            let mut q = I;
            let mut weighted = I;
            let step = psi(g, &c);
            for _ in 0..r {
                assert_eq!(psi(q, &c), weighted);
                assert_eq!(psi(psi(q, &c), &c), ne(q, &c));
                q = add(q, g, &c);
                weighted = add(weighted, step, &c);
            }
            assert_eq!((q, weighted), (I, I));
            curves.insert(p, (c, g, lam));
            ncurves += 1;
        } else if line.contains("\"kind\":\"census\"") {
            let p = number(line, "p");
            let b = number(line, "b") as usize;
            let (c, g, lam) = curves[&p];
            let got = census(&c, g, b, lam);
            assert_eq!(got.0, number(line, "covered"));
            assert_eq!(got.1, number(line, "raw_hits"));
            assert_eq!(got.2, number(line, "zero_sums"));
            assert_eq!(got.3, number(line, "flat_rows") as usize);
            assert_eq!(got.4, number(line, "folded_rows") as usize);
            assert_eq!(got.5, number(line, "flat_rank") as usize);
            assert_eq!(got.6, number(line, "folded_rank") as usize);
            ncensus += 1;
        } else if line.contains("\"kind\":\"solve\"") || line.contains("\"kind\":\"rho\"") {
            let p = number(line, "p");
            let b = number(line, "b");
            let seed = number(line, "seed");
            let (c, g, lam) = curves[&p];
            let target = array(line, "target");
            let q = point(&target);
            assert!(valid(q, &c));
            let k = number(line, "recovered");
            assert_eq!(mult(k, g, &c), q);
            assert!(line.contains("\"correct\":true"));
            let arm = if line.contains("\"arm\":\"signed-flat\"") {
                "flat"
            } else if line.contains("\"arm\":\"psi-folded\"") {
                "fold"
            } else {
                "rho"
            };
            if arm != "rho" {
                assert_eq!(number(line, "rank"), number(line, "columns"));
                let online = number(line, "target_query_ns")
                    + number(line, "target_pdp_ns")
                    + number(line, "target_descent_ns")
                    + number(line, "target_recovery_check_ns");
                assert_eq!(online, number(line, "online_ns"));
                let total = number(line, "base_ns")
                    + number(line, "table_ns")
                    + number(line, "collect_ns")
                    + number(line, "linear_ns")
                    + online
                    + number(line, "overhead_ns");
                assert_eq!(total, number(line, "total_ns"));
                if arm == "fold" {
                    assert_eq!(number(line, "lambda"), lam);
                }
                nsolves += 1;
            } else {
                nrho += 1;
            }
            assert!(matched
                .entry((p, b, seed))
                .or_default()
                .insert(arm.to_string(), (target, k))
                .is_none());
        } else if line.contains("\"kind\":\"negative_control\"") {
            assert!(line.contains("\"corrupted_orbit_rejected\":true"));
            negatives += 1;
        } else {
            panic!("unknown NDJSON record");
        }
    }
    for arms in matched.values() {
        assert_eq!(arms.len(), 3);
        assert_eq!(arms["flat"], arms["fold"]);
        assert_eq!(arms["flat"], arms["rho"]);
    }
    assert_eq!(
        (ncurves, ncensus, nsolves, nrho, negatives, matched.len()),
        (3, 9, 54, 27, 9, 27)
    );
    println!("{{\"status\":\"pass\",\"curves\":{},\"census_cells\":{},\"ic_solves\":{},\"rho_solves\":{},\"negative_controls\":{},\"matched_targets\":{},\"independent_group_law\":true,\"independent_order_count\":true,\"all_subgroup_psi_actions\":true,\"independent_census_and_rank\":true,\"four_witness_orbits_and_folded_row_covariance\":true,\"orbit_representatives_preserve_folded_rank\":true}}",ncurves,ncensus,nsolves,nrho,negatives,matched.len());
}
