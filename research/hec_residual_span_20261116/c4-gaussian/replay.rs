use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
type C = (i128, i128);
fn add(a: C, b: C) -> C {
    (a.0 + b.0, a.1 + b.1)
}
fn mul(a: C, b: C) -> C {
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}
fn conj(a: C) -> C {
    (a.0, -a.1)
}
fn norm(a: C) -> i128 {
    a.0 * a.0 + a.1 * a.1
}
fn mp(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut b = 1;
    while n > 0 {
        if n & 1 != 0 {
            b = mp(b, a, p)
        }
        a = mp(a, a, p);
        n >>= 1
    }
    b
}
fn emul(a: (u64, u64), b: (u64, u64), d: u64, p: u64) -> (u64, u64) {
    (
        (mp(a.0, b.0, p) + mp(d, mp(a.1, b.1, p), p)) % p,
        (mp(a.0, b.1, p) + mp(a.1, b.0, p)) % p,
    )
}
fn epow(mut a: (u64, u64), mut n: u64, d: u64, p: u64) -> (u64, u64) {
    let mut b = (1, 0);
    while n > 0 {
        if n & 1 != 0 {
            b = emul(b, a, d, p)
        }
        a = emul(a, a, d, p);
        n >>= 1
    }
    b
}
fn eval(x: (u64, u64), f: &[u64], d: u64, p: u64) -> (u64, u64) {
    let mut y = (0, 0);
    for &a in f.iter().rev() {
        y = emul(y, x, d, p);
        y.0 = (y.0 + a) % p
    }
    y
}
fn poly_norm(g: &[C]) -> Vec<i128> {
    let mut v = vec![(0, 0); 9];
    for j in 0..5 {
        for k in 0..5 {
            v[j + k] = add(v[j + k], mul(g[j], conj(g[k])))
        }
    }
    assert!(v.iter().all(|x| x.1 == 0));
    v.into_iter().map(|x| x.0).collect()
}
fn main() {
    let root = "research/hec_residual_span_20261116";
    let mut seen = BTreeMap::new();
    let mut coefficient = BTreeMap::new();
    for l in fs::read_to_string(format!("{root}/order4-strong/WEIL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<_> = l.split('\t').collect();
        let f: Vec<i128> = v[2]
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect();
        coefficient.insert(v[0].parse::<u64>().unwrap(), f);
    }
    let mut raw = 0;
    for l in fs::read_to_string(format!("{root}/c4-gaussian/CONTROL.tsv"))
        .unwrap()
        .lines()
        .skip(1)
    {
        let v: Vec<String> = l.split('\t').map(str::to_string).collect();
        let key = (
            v[0].parse::<u64>().unwrap(),
            v[2].parse::<u64>().unwrap(),
            v[11].parse::<u64>().unwrap(),
        );
        if let Some(old) = seen.insert(key, v.clone()) {
            assert_eq!(old, v)
        }
        raw += 1;
    }
    assert_eq!(raw, 24);
    assert_eq!(seen.len(), 12);
    println!("p\ti\tC1\ttwisted_count\tindependent_candidates\tnorm_polynomial\tlambda");
    let mut checked = 0;
    for (&(p, i, m), v) in &seen {
        if m != p {
            continue;
        }
        let code: u64 = v[1].parse().unwrap();
        let f = [0, 2, 0, code % 4, 0, (code / 4) % 4, 0, code / 16, 0, 1];
        let d = (2..p).find(|&d| pow(d, (p - 1) / 2, p) == p - 1).unwrap();
        let mut count = 1;
        let mut twist = 1;
        for x in 0..p {
            let val = eval((x, 0), &f, d, p).0;
            count += if val == 0 {
                1
            } else if pow(val, (p - 1) / 2, p) == 1 {
                2
            } else {
                0
            };
            let x2 = (0, x);
            let xp = epow(x2, p, d, p);
            assert_eq!(xp, (0, (p - x) % p));
            let val = eval(x2, &f, d, p);
            twist += if val == (0, 0) {
                1
            } else if epow(val, (p - 1) / 2, d, p) == ((p - i) % p, 0) {
                2
            } else {
                0
            };
        }
        assert_eq!(count, v[3].parse::<u64>().unwrap());
        assert_eq!(twist, v[4].parse::<u64>().unwrap());
        let re = (count as i128 - p as i128 - 1) / 2;
        let im = (p as i128 + 1 - twist as i128) / 2;
        assert_eq!(p as i128 + 1 - twist as i128, v[6].parse::<i128>().unwrap());
        let a = (re, im);
        let n: i128 = coefficient[&p].iter().sum();
        let ell = (n / 2) as u64;
        let mut candidates = BTreeSet::new();
        for u in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let c = mul(u, conj(a));
            let c = (c.0 * p as i128, c.1 * p as i128);
            let constant = (u.0 * (p as i128).pow(2), u.1 * (p as i128).pow(2));
            for t in -(6 * p as i128)..=6 * p as i128 {
                let b = match u {
                    (1, 0) => (t, 0),
                    (-1, 0) => (0, t),
                    (0, 1) => (t, t),
                    _ => (t, -t),
                };
                if norm(b) > 36 * (p as i128).pow(2) {
                    continue;
                }
                let g = [constant, c, b, a, (1, 0)];
                let z = g.iter().copied().fold((0, 0), add);
                if norm(z) != n {
                    continue;
                }
                let bi = z.1.rem_euclid(ell as i128) as u64;
                if bi == 0 {
                    continue;
                }
                let lambda = mp(
                    (-z.0).rem_euclid(ell as i128) as u64,
                    pow(bi, ell - 2, ell),
                    ell,
                );
                if mp(lambda, lambda, ell) != ell - 1 {
                    continue;
                }
                candidates.insert((u, b, z, lambda));
                assert_eq!(poly_norm(&g), coefficient[&p]);
            }
        }
        assert_eq!(candidates.len(), 1);
        let (_, _, z, lambda) = candidates.into_iter().next().unwrap();
        assert_eq!(z, (v[8].parse().unwrap(), v[9].parse().unwrap()));
        assert_eq!(lambda, v[10].parse().unwrap());
        println!("{p}\t{i}\t{count}\t{twist}\t1\ttrue\t{lambda}");
        checked += 1;
    }
    assert_eq!(checked, 6);
    eprintln!("orientations=6 independent_Fp2_twist_arithmetic=true brute_norm_line_candidates=true no_producer_imports=true raw_repetitions=24");
}
