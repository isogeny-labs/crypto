//! Independent native audit of the orbit-distinct flat control.
//! Shares the independent group arithmetic of gls_symmetry_verify.rs.
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

fn main() {
    let root = std::env::args().nth(1).expect("v2 NDJSON path");
    let control = std::env::args().nth(2).expect("distinct-control NDJSON path");
    let original = fs::read_to_string(root).unwrap();
    let measured = fs::read_to_string(control).unwrap();
    let mut curves = HashMap::new();
    for line in original.lines().filter(|s| s.contains("\"kind\":\"curve\"")) {
        let p = number(line, "p");
        let r = number(line, "r");
        let d = number(line, "d");
        assert!(prime(p) && prime(r));
        assert_eq!(powm(d, (p - 1) / 2, p), p - 1);
        let twv = array(line, "c");
        let tw = F(twv[0], twv[1]);
        let mut c = C {
            p, d, a: F(0, 0), b: F(0, 0), r,
            cof: number(line, "cofactor"), alpha: F(0, 0),
        };
        assert_eq!(power(tw, (p * p - 1) / 2, &c), F(p - 1, 0));
        let t2 = times(tw, tw, &c);
        c.a = times(F(number(line, "a"), 0), t2, &c);
        c.b = times(F(number(line, "b_curve"), 0), times(t2, tw, &c), &c);
        c.alpha = power(tw, p * p - 1 - (p - 1) / 2, &c);
        let g = point(&array(line, "generator"));
        assert!(valid(g, &c) && !g.zero);
        assert_eq!(mult(r, g, &c), I);
        assert_eq!(psi(g, &c), mult(number(line, "lambda"), g, &c));
        assert!(curves.insert(p, (c, g)).is_none());
    }
    assert_eq!(curves.len(), 3);

    let mut censuses = HashMap::new();
    let mut solves: HashMap<(u64, u64), &str> = HashMap::new();
    for line in measured.lines() {
        if line.contains("\"kind\":\"distinct_census\"") {
            assert!(censuses.insert(number(line, "p"), line).is_none());
        } else if line.contains("\"kind\":\"distinct_solve\"") {
            assert!(solves.insert((number(line, "p"), number(line, "seed")), line).is_none());
        } else {
            panic!("unexpected control record");
        }
    }
    assert_eq!(censuses.len(), 3);
    assert_eq!(solves.len(), 9);

    for p in [101, 211, 307] {
        let &(c, g) = curves.get(&p).expect("missing panel curve");
        let full = build(&c, 24);
        for pair in full.chunks_exact(2) {
            assert!(valid(pair[0], &c) && valid(pair[1], &c));
            assert_eq!(psi(pair[0], &c), pair[1]);
            assert_eq!(mult(c.r, pair[0], &c), I);
            assert_eq!(mult(c.r, pair[1], &c), I);
        }
        let base: Vec<Q> = full.iter().step_by(2).copied().collect();
        assert_eq!(base.len(), 12);
        let mut logs = HashMap::new();
        let mut q = I;
        for scalar in 1..c.r {
            q = add(q, g, &c);
            assert!(logs.insert(q, scalar).is_none());
        }
        assert_eq!(add(q, g, &c), I);
        let base_logs: Vec<u64> = base.iter().map(|q| logs[q]).collect();

        let mut targets: HashMap<Q, Vec<Vec<u64>>> = HashMap::new();
        let mut rows = HashSet::new();
        let mut raw = 0u64;
        let mut zero = 0u64;
        for i in 0..12 {
            for j in i + 1..12 {
                for k in j + 1..12 {
                    for mask in 0..8u8 {
                        let idx = [i, j, k];
                        let mut sum = I;
                        let mut coefficients = vec![0; 12];
                        for z in 0..3 {
                            let sign = mask & (1 << z) != 0;
                            let factor = if sign { ne(base[idx[z]], &c) } else { base[idx[z]] };
                            sum = add(sum, factor, &c);
                            coefficients[idx[z]] = if sign { c.r - 1 } else { 1 };
                        }
                        if sum.zero {
                            zero += 1;
                            continue;
                        }
                        let expected_scalar = coefficients.iter().zip(&base_logs).fold(
                            0u64,
                            |acc, (&coefficient, &factor_log)| {
                                ((acc as u128 + coefficient as u128 * factor_log as u128)
                                    % c.r as u128) as u64
                            },
                        );
                        assert_eq!(logs[&sum], expected_scalar);
                        raw += 1;
                        rows.insert(coefficients.clone());
                        targets.entry(sum).or_default().push(coefficients);
                    }
                }
            }
        }
        let record = censuses[&p];
        for (key, expected) in [
            ("b", 12), ("columns", 12), ("targets", c.r - 1),
            ("covered", targets.len() as u64), ("raw_hits", raw),
            ("zero_sums", zero), ("unique_rows", rows.len() as u64),
            ("rank", rank(&rows, c.r, 12) as u64),
        ] {
            assert_eq!(number(record, key), expected, "{} mismatch for p={}", key, p);
        }
        assert_eq!(rank(&rows, c.r, 12), 12);

        let mut collected = HashSet::new();
        let mut q = I;
        let mut stop = 0u64;
        for trial in 1..c.r {
            q = add(q, g, &c);
            if let Some(vectors) = targets.get(&q) {
                collected.extend(vectors.iter().cloned());
            }
            if rank(&collected, c.r, 12) == 12 {
                stop = trial;
                break;
            }
        }
        assert!(stop > 0);

        for (seed, scalar) in [37u64, 113, 271].iter().enumerate() {
            let line = solves[&(p, seed as u64)];
            let target = point(&array(line, "target"));
            assert_eq!(target, mult(*scalar, g, &c));
            assert_eq!(number(line, "recovered"), *scalar);
            assert!(line.contains("\"correct\":true"));
            assert_eq!(number(line, "columns"), 12);
            assert_eq!(number(line, "rank"), 12);
            assert_eq!(number(line, "trials"), stop);
            let mut query = target;
            let mut descent = 0u64;
            for _ in 0..c.r {
                descent += 1;
                if targets.contains_key(&query) { break; }
                query = add(query, g, &c);
            }
            assert_eq!(number(line, "online_queries"), descent);
        }
        println!("p={} covered={} raw_hits={} rank=12 trials={} solves=3 OK", p, targets.len(), raw, stop);
    }
}
