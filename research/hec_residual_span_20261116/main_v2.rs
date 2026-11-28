use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, thread_curve},
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
fn main() {
    let scalar = Fp::new(1_000_003);
    let mut coverage = [[0usize; 4]; 3];
    println!("g\tp\tc\tH\tmode\tsubsets\tsplit\tnonsplit\tmixed\tlabels\tcache_rows\tdirect_candidates\tdirect_rows\trepeated_direct\tpartition_checks\tbase_rank\tcache_rank\tdirect_rank\tunion_rank");
    for g in 2..=4 {
        for p in [19u64, 31] {
            let primary = (1..p).find_map(|cc| {
                thread_curve(p, cc, g)
                    .filter(|c| irreducible(&c.f, &c.fp))
                    .map(|c| (cc, c))
            });
            let (cc, c) = primary
                .or_else(|| {
                    if g != 4 {
                        return None;
                    }
                    (1..p).find_map(|constant| {
                        let fp = Fp::new(p);
                        let mut coeffs = vec![0; 2 * g + 2];
                        coeffs[0] = constant;
                        coeffs[2 * g + 1] = 1;
                        let polynomial = Poly::from_coeffs(&coeffs);
                        if Poly::gcd(&polynomial, &polynomial.deriv(&fp), &fp).deg() != 0
                            || !irreducible(&polynomial, &fp)
                        {
                            return None;
                        }
                        Some((constant, Curve::new(fp, polynomial)))
                    })
                })
                .expect("no irreducible curve in amended deterministic families");
            eprintln!(
                "fixture g={g} p={p} parameter={cc} coefficients={:?}",
                &c.f.c[..c.f.n]
            );
            // For odd-degree hyperelliptic models, irreducible f gives no rational 2-torsion,
            // so the finite rational Jacobian has odd order. No subgroup or scalar oracle.
            let fb = factor_base(&c);
            assert!(fb.iter().all(|p| p.1 != 0));
            let m = fb.len();
            let h = 9.min(m);
            assert!(h >= g + 2);
            for mode in 0..2 {
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
                                let row: Vec<_> =
                                    raw.iter().zip(&first.row).map(|(a, b)| a - b).collect();
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
                let (mut direct_candidates, mut repeats, mut partitions) = (0, 0, 0);
                let mut direct_rows = vec![];
                for tuple in combinations(h, g + 2) {
                    for signs in 0..(1usize << (g + 1)) {
                        direct_candidates += 1;
                        let points: Vec<_> = tuple
                            .iter()
                            .enumerate()
                            .map(|(i, &j)| {
                                let p = fb[chosen[j]];
                                (
                                    p.0,
                                    if i > 0 && signs & (1 << (i - 1)) != 0 {
                                        c.fp.neg(p.1)
                                    } else {
                                        p.1
                                    },
                                )
                            })
                            .collect();
                        let a = interpolate(&points, &c.fp);
                        if a.deg() != (g + 1) as isize {
                            continue;
                        }
                        let norm = a.mul(&a, &c.fp).sub(&c.f, &c.fp);
                        let (rr, rest) = rational_roots(norm, &c.fp);
                        if rest.deg() != 0
                            || rr.iter().any(|&(x, k)| {
                                k > 2 || !on[fb.iter().position(|p| p.0 == x).unwrap()]
                            })
                        {
                            continue;
                        }
                        let row = norm_row(&c, &fb, &a, &rr);
                        assert!(c.is_zero(&group_sum(&c, &fb, &row)));
                        if rr.iter().any(|r| r.1 == 2) {
                            repeats += 1
                        }
                        let (mut left, mut right, mut singles) = (vec![], vec![], vec![]);
                        for &(x, k) in &rr {
                            let point = (x, a.eval(x, &c.fp));
                            if k == 2 {
                                left.push(point);
                                right.push(point)
                            } else {
                                singles.push(point)
                            }
                        }
                        for point in singles {
                            if left.len() < g + 1 {
                                left.push(point)
                            } else {
                                right.push(point)
                            }
                        }
                        assert_eq!(left.len(), g + 1);
                        assert_eq!(right.len(), g + 1);
                        for point in &mut right {
                            point.1 = c.fp.neg(point.1)
                        }
                        let (_, e1) = divisor(&c, interpolate(&left, &c.fp), &left);
                        let (_, e2) = divisor(&c, interpolate(&right, &c.fp), &right);
                        assert_eq!(e1, e2);
                        partitions += 1;
                        direct_rows.push(modular(&row, &scalar));
                    }
                }
                coverage[g - 2][0] += mixed;
                coverage[g - 2][1] += cache_rows.len();
                coverage[g - 2][2] += direct_rows.len();
                coverage[g - 2][3] += repeats;
                let rb = rank(split_rows.clone(), &scalar);
                let mut cached = split_rows.clone();
                cached.extend(cache_rows.iter().cloned());
                let rc = rank(cached.clone(), &scalar);
                let mut direct = split_rows;
                direct.extend(direct_rows.iter().cloned());
                let rd = rank(direct, &scalar);
                cached.extend(direct_rows.iter().cloned());
                let ru = rank(cached, &scalar);
                assert_eq!(rc, rd);
                assert_eq!(rc, ru);
                println!("{g}\t{p}\t{cc}\t{h}\t{mode}\t{attempts}\t{split}\t{nonsplit}\t{mixed}\t{}\t{}\t{direct_candidates}\t{}\t{repeats}\t{partitions}\t{rb}\t{rc}\t{rd}\t{ru}",cache.len(),cache_rows.len(),direct_rows.len());
            }
        }
    }
    for (i, c) in coverage.iter().enumerate() {
        eprintln!(
            "coverage g{} mixed={} cache={} direct={} repeated={}",
            i + 2,
            c[0],
            c[1],
            c[2],
            c[3]
        );
    }
    if coverage.iter().any(|c| c[1] == 0 || c[2] == 0)
        || coverage.iter().map(|c| c[3]).sum::<usize>() == 0
        || coverage[1..].iter().map(|c| c[0]).sum::<usize>() == 0
    {
        eprintln!("INCOMPLETE_COVERAGE");
        std::process::exit(2)
    }
}
