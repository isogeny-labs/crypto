use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    poly::Poly,
};
use std::collections::HashMap;

// A deliberately separate scalar rank implementation: no production solver or table oracle.
fn rank(mut a: Vec<Vec<u64>>, f: &Fp) -> usize {
    if a.is_empty() {
        return 0;
    }
    let n = a[0].len();
    let mut r = 0;
    for j in 0..n {
        let Some(i) = (r..a.len()).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(r, i);
        let inv = f.inv(a[r][j]);
        for k in j..n {
            a[r][k] = f.mul(a[r][k], inv)
        }
        for i in r + 1..a.len() {
            let z = a[i][j];
            for k in j..n {
                a[i][k] = f.sub(a[i][k], f.mul(z, a[r][k]))
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn rec(n: usize, k: usize, start: usize, v: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if v.len() == k {
            out.push(v.clone());
            return;
        }
        for i in start..n {
            if n - i < k - v.len() {
                break;
            }
            v.push(i);
            rec(n, k, i + 1, v, out);
            v.pop();
        }
    }
    let mut out = vec![];
    rec(n, k, 0, &mut vec![], &mut out);
    out
}
fn interpolate(points: &[(u64, u64)], f: &Fp) -> Poly {
    let mut h = Poly::zero();
    for (i, &(x, y)) in points.iter().enumerate() {
        let mut b = Poly::one();
        let mut den = 1;
        for (j, &(z, _)) in points.iter().enumerate() {
            if i != j {
                b = b.mul(&Poly::x_minus(z, f), f);
                den = f.mul(den, f.sub(x, z));
            }
        }
        h = h.add(&b.scale(f.mul(y, f.inv(den)), f), f);
    }
    h
}
// Exhaustive linear division, independent of production split/root tables.
fn rational_roots(mut p: Poly, f: &Fp) -> (Vec<(u64, usize)>, Poly) {
    let mut roots = vec![];
    for x in 0..f.p {
        let mut m = 0;
        while p.deg() > 0 && p.eval(x, f) == 0 {
            p = p.div_exact(&Poly::x_minus(x, f), f);
            m += 1;
        }
        if m > 0 {
            roots.push((x, m));
        }
    }
    (roots, p)
}
fn irreducible(p: &Poly, f: &Fp) -> bool {
    let n = p.deg() as usize;
    let mut xp = Poly::x();
    for i in 1..=n {
        xp = xp.powmod(f.p, p, f);
        if i <= n / 2 && Poly::gcd(p, &xp.sub(&Poly::x(), f), f).deg() > 0 {
            return false;
        }
    }
    xp == Poly::x()
}
fn divisor(c: &Curve, h: Poly, points: &[(u64, u64)]) -> (Poly, Div) {
    let u = points
        .iter()
        .fold(Poly::one(), |u, p| u.mul(&Poly::x_minus(p.0, &c.fp), &c.fp));
    let q = c.f.sub(&h.mul(&h, &c.fp), &c.fp).div_exact(&u, &c.fp);
    assert_eq!(q.deg(), c.g as isize);
    assert_eq!(q.lc(), 1);
    let e = Div {
        u: q,
        v: h.rem(&q, &c.fp),
    };
    assert!(c.is_valid(&e));
    (u, e)
}
fn group_sum(c: &Curve, fb: &[(u64, u64)], row: &[i64]) -> Div {
    let mut d = c.zero();
    for (p, &s) in fb.iter().zip(row) {
        if s != 0 {
            let z = c.mul(&c.place(p.0, p.1), s.unsigned_abs());
            d = c.add(&d, &if s < 0 { c.neg(&z) } else { z });
        }
    }
    d
}
fn modular(row: &[i64], f: &Fp) -> Vec<u64> {
    row.iter().map(|&v| f.from_i64(v)).collect()
}
fn norm_row(c: &Curve, fb: &[(u64, u64)], h: &Poly, roots: &[(u64, usize)]) -> Vec<i64> {
    let mut row = vec![0; fb.len()];
    for &(x, m) in roots {
        let j = fb.iter().position(|p| p.0 == x).unwrap();
        let y = h.eval(x, &c.fp);
        assert!(y == fb[j].1 || y == c.fp.neg(fb[j].1));
        row[j] += if y == fb[j].1 { m as i64 } else { -(m as i64) };
    }
    row
}
#[derive(Clone)]
struct Stored {
    h: Poly,
    u: Poly,
    row: Vec<i64>,
}

fn catalogue_logs(
    c: &Curve,
    fb: &[(u64, u64)],
    source: &Div,
) -> (Vec<u64>, usize, usize, usize, u64) {
    let g = c.g;
    let m = fb.len();
    let h = m;
    let mode = 0;
    let scalar = Fp::new(8623);
    let mut chosen: Vec<_> = (0..m).collect();
    if mode == 1 {
        chosen.rotate_left(m / 2)
    }
    chosen.truncate(h);
    chosen.sort_unstable();
    let mut on = vec![false; m];
    for &j in &chosen {
        on[j] = true
    }
    let mut split_rows = vec![];
    let mut cache_rows = vec![];
    let mut cache: HashMap<Div, Stored> = HashMap::new();
    let (mut attempts, mut split, mut nonsplit, mut mixed) = (0, 0, 0, 0);
    for tuple in combinations(h, g + 1) {
        for signs in 0..(1usize << g) {
            attempts += 1;
            let mut raw = vec![0i64; m];
            let points: Vec<_> = tuple
                .iter()
                .enumerate()
                .map(|(i, &j)| {
                    let idx = chosen[j];
                    let s = if i > 0 && signs & (1 << (i - 1)) != 0 {
                        -1
                    } else {
                        1
                    };
                    raw[idx] = s;
                    (
                        fb[idx].0,
                        if s < 0 {
                            c.fp.neg(fb[idx].1)
                        } else {
                            fb[idx].1
                        },
                    )
                })
                .collect();
            let mut hp = interpolate(&points, &c.fp);
            let (u, mut e) = divisor(&c, hp, &points);
            assert!(c.is_zero(&c.add(&group_sum(&c, &fb, &raw), &e)));
            let (roots, remaining) = rational_roots(e.u, &c.fp);
            if remaining.deg() == 0 {
                split += 1;
                let mut r = norm_row(&c, &fb, &hp, &roots);
                for j in 0..m {
                    r[j] += raw[j]
                }
                assert!(c.is_zero(&group_sum(&c, &fb, &r)));
                split_rows.push(modular(&r, &scalar));
            } else {
                nonsplit += 1;
                if !roots.is_empty() {
                    mixed += 1
                }
                assert!(!e.v.is_zero());
                let neg = e.v.neg(&c.fp);
                if neg.c < e.v.c {
                    e.v = neg;
                    hp = hp.neg(&c.fp);
                    for z in &mut raw {
                        *z = -*z
                    }
                }
                if let Some(first) = cache.get(&e) {
                    let row: Vec<_> = raw.iter().zip(&first.row).map(|(a, b)| a - b).collect();
                    let delta = hp.sub(&first.h, &c.fp);
                    let quotient = delta.div_exact(&e.u, &c.fp);
                    assert!(quotient.deg() <= 0);
                    if quotient.is_zero() {
                        assert!(row.iter().all(|&z| z == 0));
                        continue;
                    }
                    let coeff = quotient.c[0];
                    let a =
                        c.f.sub(&hp.mul(&first.h, &c.fp), &c.fp)
                            .div_exact(&e.u, &c.fp);
                    assert_eq!(a.deg(), (g + 1) as isize);
                    assert_eq!(
                        a.mul(&a, &c.fp)
                            .sub(&c.f.scale(c.fp.mul(coeff, coeff), &c.fp), &c.fp),
                        u.mul(&first.u, &c.fp)
                    );
                    let (rr, rest) = rational_roots(u.mul(&first.u, &c.fp), &c.fp);
                    assert_eq!(rest.deg(), 0);
                    assert!(rr.iter().all(|r| r.1 <= 2));
                    let normalized = a.scale(c.fp.inv(coeff), &c.fp);
                    assert_eq!(norm_row(&c, &fb, &normalized, &rr), row);
                    assert!(c.is_zero(&group_sum(&c, &fb, &row)));
                    cache_rows.push(modular(&row, &scalar));
                } else {
                    cache.insert(e, Stored { h: hp, u, row: raw });
                }
            }
        }
    }
    let base_rank = rank(split_rows.clone(), &scalar);
    let mut all = split_rows;
    all.extend(cache_rows.iter().cloned());
    let augmented = rank(all.clone(), &scalar);
    assert_eq!(augmented, m - 1);
    let (logs, muls) = normalized_solve(all, &scalar, m);
    for (j, &log) in logs.iter().enumerate() {
        assert_eq!(c.mul(source, log), c.mul(&c.place(fb[j].0, fb[j].1), 13));
    }
    eprintln!("catalogue candidates={attempts} split={split} nonsplit={nonsplit} mixed={mixed} labels={} cache={} base_rank={base_rank} augmented_rank={augmented} solve_mulmods={muls}",cache.len(),cache_rows.len());
    (logs, attempts, cache.len(), cache_rows.len(), muls)
}
fn normalized_solve(mut rows: Vec<Vec<u64>>, f: &Fp, m: usize) -> (Vec<u64>, u64) {
    for row in &mut rows {
        row.push(0)
    }
    let mut normalization = vec![0; m + 1];
    normalization[0] = 1;
    normalization[m] = 1;
    rows.push(normalization);
    let mut pivot = 0;
    let mut places = vec![];
    let mut muls = 0;
    for j in 0..m {
        let Some(i) = (pivot..rows.len()).find(|&i| rows[i][j] != 0) else {
            continue;
        };
        rows.swap(pivot, i);
        let inverse = f.inv(rows[pivot][j]);
        for k in j..=m {
            rows[pivot][k] = f.mul(rows[pivot][k], inverse);
            muls += 1
        }
        for i in 0..rows.len() {
            if i == pivot {
                continue;
            }
            let z = rows[i][j];
            if z == 0 {
                continue;
            }
            for k in j..=m {
                rows[i][k] = f.sub(rows[i][k], f.mul(z, rows[pivot][k]));
                muls += 1
            }
        }
        places.push(j);
        pivot += 1;
    }
    assert_eq!(pivot, m);
    for row in rows.iter().skip(pivot) {
        assert_eq!(row[m], 0)
    }
    let mut logs = vec![0; m];
    for (i, &j) in places.iter().enumerate() {
        logs[j] = rows[i][m]
    }
    (logs, muls)
}
fn recover(
    c: &Curve,
    fb: &[(u64, u64)],
    source: &Div,
    target: &Div,
    logs: &[u64],
) -> (Option<u64>, usize, usize) {
    let scalar = Fp::new(8623);
    let inv13 = scalar.inv(13);
    let mut r = *target;
    for offset in 0..2048usize {
        let (roots, rest) = rational_roots(r.u, &c.fp);
        if rest.deg() == 0 {
            let signed = norm_row(c, fb, &r.v, &roots);
            assert_eq!(group_sum(c, fb, &signed), r);
            let projected = signed.iter().zip(logs).fold(0, |s, (&a, &b)| {
                scalar.add(s, scalar.mul(scalar.from_i64(a), b))
            });
            // Projection gives log(13*R)=13*(k+offset); do not omit inverse13.
            let k = scalar.sub(scalar.mul(inv13, projected), offset as u64 % scalar.p);
            assert_eq!(c.mul(source, k), *target);
            return (Some(k), offset + 1, offset);
        }
        if offset + 1 < 2048 {
            r = c.add(&r, source)
        }
    }
    (None, 2048, 2047)
}
fn main() {
    use hec_native_ic::ic::{solve_dlp, IcParams};
    let c = Curve::new(
        Fp::new(19),
        Poly::from_coeffs(&[2, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
    );
    assert_eq!(c.g, 4);
    assert!(irreducible(&c.f, &c.fp));
    assert_eq!(hec_native_ic::instance::prime_factors(8623), vec![8623]);
    let fb = factor_base(&c);
    assert_eq!(fb.len(), 9);
    let source = c.mul(&c.place(fb[0].0, fb[0].1), 13);
    assert!(!c.is_zero(&source));
    assert!(c.is_zero(&c.mul(&source, 8623)));
    println!("case\tcandidates\tlabels\tcache_rows\tlog_solve_mulmods\trecovered\tdescent_trials\tdescent_adds\tnative_k\tnative_trials\tnative_precompute_ops\tnative_group_ops\tnative_solve_mulmods");
    let mut failures = 0;
    for (case, known) in [37u64, 2026, 7001].into_iter().enumerate() {
        let target = c.mul(&source, known);
        eprintln!("public_input case={case} source={source:?} target={target:?}");
        let (logs, candidates, labels, cache_rows, muls) = catalogue_logs(&c, &fb, &source);
        let (k, trials, adds) = recover(&c, &fb, &source, &target, &logs);
        let params = IcParams {
            seed: 2026111600 + case as u64,
            max_trials: 200000,
            ..IcParams::default()
        };
        let native = solve_dlp(&c, &source, &target, 8623, &params);
        if k != Some(known) || native.k != Some(known) {
            failures += 1
        }
        println!("{case}\t{candidates}\t{labels}\t{cache_rows}\t{muls}\t{}\t{trials}\t{adds}\t{}\t{}\t{}\t{}\t{}",k.map_or_else(||"incomplete".into(),|x|x.to_string()),native.k.map_or_else(||"incomplete".into(),|x|x.to_string()),native.trials,native.precompute_ops,native.group_ops,native.solve.mulmods);
    }
    if failures != 0 {
        eprintln!("INCOMPLETE_SINGLE_TARGET_CONTROLS count={failures}");
        std::process::exit(2)
    }
}
