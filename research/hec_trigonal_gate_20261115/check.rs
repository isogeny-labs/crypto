use hec_native_ic::{fp::Fp, poly::Poly, rng::Rng};
use std::{collections::BTreeMap, fs, path::Path};
const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
type V = [u64; 6];
fn wedge(n: &[u64; 4], d: &[u64; 4], f: &Fp) -> V {
    PAIRS.map(|(i, j)| f.sub(f.mul(n[i], d[j]), f.mul(n[j], d[i])))
}
fn q(v: &V, f: &Fp) -> u64 {
    f.add(
        f.sub(f.mul(v[0], v[5]), f.mul(v[1], v[4])),
        f.mul(v[2], v[3]),
    )
}
fn normal(mut v: V, f: &Fp) -> V {
    let a = f.inv(*v.iter().find(|&&a| a != 0).unwrap());
    for x in &mut v {
        *x = f.mul(*x, a);
    }
    v
}
fn get(v: &V, i: usize, j: usize, f: &Fp) -> u64 {
    if i == j {
        return 0;
    }
    if i > j {
        return f.neg(get(v, j, i, f));
    }
    v[PAIRS.iter().position(|&(a, b)| a == i && b == j).unwrap()]
}
fn recover(v: &V, f: &Fp) -> ([u64; 4], [u64; 4]) {
    assert_eq!(q(v, f), 0);
    let k = v.iter().position(|&a| a != 0).unwrap();
    let (i, j) = PAIRS[k];
    let mut n = [0; 4];
    let mut d = [0; 4];
    n[i] = 1;
    d[j] = v[k];
    for a in 0..4 {
        if a != i && a != j {
            n[a] = f.neg(f.mul(get(v, j, a, f), f.inv(v[k])));
            d[a] = get(v, i, a, f);
        }
    }
    assert_eq!(wedge(&n, &d, f), *v);
    (n, d)
}
fn nullspace(mut a: Vec<V>, f: &Fp) -> Vec<V> {
    let mut pivots = Vec::new();
    let mut r = 0;
    for c in 0..6 {
        let Some(i) = (r..a.len()).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(i, r);
        let z = f.inv(a[r][c]);
        for x in &mut a[r] {
            *x = f.mul(*x, z);
        }
        for i in 0..a.len() {
            if i != r {
                let z = a[i][c];
                for j in 0..6 {
                    a[i][j] = f.sub(a[i][j], f.mul(z, a[r][j]));
                }
            }
        }
        pivots.push(c);
        r += 1;
    }
    let mut out = Vec::new();
    for c in 0..6 {
        if !pivots.contains(&c) {
            let mut v = [0; 6];
            v[c] = 1;
            for (i, &p) in pivots.iter().enumerate() {
                v[p] = f.neg(a[i][c]);
            }
            out.push(v);
        }
    }
    out
}
fn line(b: &[V], f: &Fp) -> Vec<V> {
    assert_eq!(b.len(), 2);
    let mut out = Vec::new();
    for t in 0..f.p {
        let v = std::array::from_fn(|i| f.add(b[0][i], f.mul(t, b[1][i])));
        if q(&v, f) == 0 {
            out.push(normal(v, f));
        }
    }
    if q(&b[1], f) == 0 {
        out.push(normal(b[1], f));
    }
    out.sort();
    out.dedup();
    out
}
fn powers(x: u64, f: &Fp) -> [u64; 4] {
    [1, x, f.mul(x, x), f.pow(x, 3)]
}
fn constraint(a: &[u64; 4], b: &[u64; 4], f: &Fp) -> V {
    PAIRS.map(|(i, j)| f.sub(f.mul(a[i], b[j]), f.mul(a[j], b[i])))
}
fn dot(a: &V, b: &V, f: &Fp) -> u64 {
    (0..6).fold(0, |s, i| f.add(s, f.mul(a[i], b[i])))
}
fn controls() {
    let mut rng = Rng::new(202611150991);
    let mut count = 0;
    for p in [5, 7, 11, 17] {
        let f = Fp::new(p);
        for _ in 0..16 {
            let n = [1, 0, rng.below(p), rng.below(p)];
            let d = [0, 1, rng.below(p), rng.below(p)];
            let v = wedge(&n, &d, &f);
            assert_eq!(q(&v, &f), 0);
            recover(&v, &f);
            count += 1;
        }
    }
    for p in [13, 19, 31] {
        let f = Fp::new(p);
        let mut fibres = BTreeMap::<u64, Vec<u64>>::new();
        for x in 0..p {
            fibres.entry(f.pow(x, 3)).or_default().push(x);
        }
        let pairs: Vec<_> = fibres
            .values()
            .filter(|xs| xs.len() >= 2)
            .take(4)
            .map(|v| (v[0], v[1]))
            .collect();
        assert_eq!(pairs.len(), 4);
        let rows: Vec<_> = pairs
            .iter()
            .map(|&(a, b)| constraint(&powers(a, &f), &powers(b, &f), &f))
            .collect();
        let b = nullspace(rows.clone(), &f);
        let roots = line(&b, &f);
        let known = normal(wedge(&[0, 0, 0, 1], &[1, 0, 0, 0], &f), &f);
        assert!(roots.contains(&known));
        for v in roots {
            let (n, d) = recover(&v, &f);
            for row in &rows {
                assert_eq!(dot(row, &v, &f), 0);
            }
            let n = Poly::from_coeffs(&n);
            let d = Poly::from_coeffs(&d);
            if Poly::gcd(&n, &d, &f) == Poly::one() && n.deg().max(d.deg()) == 3 {
                for &(a, b) in &pairs {
                    assert_eq!(
                        f.mul(n.eval(a, &f), d.eval(b, &f)),
                        f.mul(n.eval(b, &f), d.eval(a, &f))
                    );
                }
            }
        }
    }
    println!("CONTROLS wedges={count} planted_cubic_maps=3 pass=true");
}
fn factors(f: &Poly, fp: &Fp) -> Vec<(usize, Poly)> {
    assert_eq!(Poly::gcd(f, &f.deriv(fp), fp), Poly::one());
    let x = Poly::x();
    let mut xp = x;
    let mut rem = *f;
    let mut out = Vec::new();
    for k in 1..=7 {
        xp = xp.powmod(fp.p, f, fp);
        let part = Poly::gcd(&rem, &xp.sub(&x, fp), fp);
        if part.deg() > 0 {
            assert_eq!(part.deg(), k as isize);
            rem = rem.div_exact(&part, fp);
            out.push((k, part));
        }
    }
    assert_eq!(rem, Poly::one());
    out
}
fn compose(n: &Poly, x: &Poly, m: &Poly, f: &Fp) -> Poly {
    let mut a = Poly::zero();
    for i in (0..n.n).rev() {
        a = a.mulmod(x, m, f).add(&Poly::from_coeffs(&[n.c[i]]), f);
    }
    a
}
fn main() {
    controls();
    let root = std::env::args().nth(1).unwrap();
    for p in [127, 1009] {
        let text =
            fs::read_to_string(Path::new(&root).join(format!("select-g3-p{p}-f1.out"))).unwrap();
        let t: Vec<_> = text.split_whitespace().collect();
        assert_eq!(t[0], "FIXTURE");
        assert_eq!(t[1], "3");
        assert_eq!(t[2].parse::<u64>().unwrap(), p);
        assert_eq!(t[3], "1");
        assert_eq!(t[6], "8");
        let fp = Fp::new(p);
        let f = Poly::from_coeffs(
            &t[7..15]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>(),
        );
        let factors = factors(&f, &fp);
        assert_eq!(
            factors.iter().map(|(d, _)| *d).collect::<Vec<_>>(),
            if p == 127 { vec![1, 2, 4] } else { vec![1, 6] }
        );
        let mut rows = Vec::new();
        let mut modules = Vec::new();
        let mut rational = 0;
        for (e, m) in &factors {
            if *e == 1 {
                rational = fp.neg(fp.mul(m.c[0], fp.inv(m.c[1])));
                rows.push(constraint(&powers(rational, &fp), &[0, 0, 0, 1], &fp));
                continue;
            }
            assert_eq!(e % 2, 0);
            let x = Poly::x();
            let mut tau = x;
            for _ in 0..e / 2 {
                tau = tau.powmod(p, m, &fp);
            }
            assert_ne!(tau, x);
            let mut back = tau;
            for _ in 0..e / 2 {
                back = back.powmod(p, m, &fp);
            }
            assert_eq!(back, x);
            let mut xp = [Poly::one(); 4];
            let mut tp = xp;
            for i in 1..4 {
                xp[i] = xp[i - 1].mulmod(&x, m, &fp);
                tp[i] = tp[i - 1].mulmod(&tau, m, &fp);
            }
            let cs = PAIRS.map(|(i, j)| {
                xp[i]
                    .mulmod(&tp[j], m, &fp)
                    .sub(&xp[j].mulmod(&tp[i], m, &fp), &fp)
            });
            for k in 0..*e {
                rows.push(std::array::from_fn(|i| cs[i].c[k]));
            }
            modules.push((*m, tau));
        }
        let b = nullspace(rows.clone(), &fp);
        assert_eq!(b.len(), 2);
        let roots = line(&b, &fp);
        let sum = std::array::from_fn(|i| fp.add(b[0][i], b[1][i]));
        let (c0, c2) = (q(&b[0], &fp), q(&b[1], &fp));
        let c1 = fp.sub(fp.sub(q(&sum, &fp), c0), c2);
        let disc = fp.sub(fp.mul(c1, c1), fp.mul(4, fp.mul(c0, c2)));
        println!("LINE p={p} basis={b:?} quad=[{c0},{c1},{c2}] discriminant={disc} legendre={} projective_roots={}",fp.legendre(disc),roots.len());
        let mut valid = 0;
        for v in roots {
            for row in &rows {
                assert_eq!(dot(row, &v, &fp), 0);
            }
            let (nc, dc) = recover(&v, &fp);
            let n = Poly::from_coeffs(&nc);
            let d = Poly::from_coeffs(&dc);
            let accepted = Poly::gcd(&n, &d, &fp) == Poly::one() && n.deg().max(d.deg()) == 3;
            println!("MAP p={p} accepted={accepted} N={nc:?} D={dc:?} plucker={v:?}");
            if !accepted {
                continue;
            }
            valid += 1;
            assert_eq!(
                fp.mul(n.eval(rational, &fp), dc[3]),
                fp.mul(nc[3], d.eval(rational, &fp))
            );
            for (m, tau) in &modules {
                let nd = n.mulmod(&compose(&d, tau, m, &fp), m, &fp);
                let dn = d.mulmod(&compose(&n, tau, m, &fp), m, &fp);
                assert_eq!(nd, dn);
            }
        }
        println!("TERMINAL p={p} valid_maps={valid} all_pair_identities=true");
    }
}
