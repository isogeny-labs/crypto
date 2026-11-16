use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    poly::Poly,
};
use std::{fs, path::Path, time::Instant};
type E = [u64; 3];
struct Ext {
    p: u64,
    n: usize,
    a: u64,
    b: u64,
    nu: u64,
}
fn chars(p: u64) -> Vec<i64> {
    let mut c = vec![-1; p as usize];
    c[0] = 0;
    for x in 1..p {
        c[(x * x % p) as usize] = 1;
    }
    c
}
impl Ext {
    fn new(p: u64, n: usize) -> Self {
        if n == 2 {
            let c = chars(p);
            return Self {
                p,
                n,
                a: 0,
                b: 0,
                nu: (1..p).find(|&x| c[x as usize] == -1).unwrap(),
            };
        }
        assert_eq!(n, 3);
        for a in 0..p {
            for b in 1..p {
                if (0..p).all(|x| (x * x % p * x + a * x + b) % p != 0) {
                    return Self { p, n, a, b, nu: 0 };
                }
            }
        }
        panic!("irreducible cubic missing")
    }
    fn mul(&self, x: E, y: E) -> E {
        let p = self.p;
        if self.n == 2 {
            return [
                (x[0] * y[0] + self.nu * x[1] * y[1]) % p,
                (x[0] * y[1] + x[1] * y[0]) % p,
                0,
            ];
        }
        let r0 = x[0] * y[0];
        let r1 = x[0] * y[1] + x[1] * y[0];
        let r2 = x[0] * y[2] + x[1] * y[1] + x[2] * y[0];
        let r3 = x[1] * y[2] + x[2] * y[1];
        let r4 = x[2] * y[2];
        let m = 3 * p * p * p;
        [
            (r0 + m - self.b * r3) % p,
            (r1 + m - self.a * r3 - self.b * r4) % p,
            (r2 + m - self.a * r4) % p,
        ]
    }
    fn pow(&self, mut x: E, mut e: u64) -> E {
        let mut y = [1, 0, 0];
        while e != 0 {
            if e & 1 != 0 {
                y = self.mul(y, x)
            }
            x = self.mul(x, x);
            e >>= 1;
        }
        y
    }
    fn norm(&self, x: E) -> u64 {
        let (a, b, c) = (x[0] as i128, x[1] as i128, x[2] as i128);
        let (aa, bb) = (self.a as i128, self.b as i128);
        let z = if self.n == 2 {
            a * a - self.nu as i128 * b * b
        } else {
            a * a * a - 2 * aa * a * a * c
                + aa * aa * a * c * c
                + aa * a * b * b
                + 3 * bb * a * b * c
                - bb * b * b * b
                - aa * bb * b * c * c
                + bb * bb * c * c * c
        };
        z.rem_euclid(self.p as i128) as u64
    }
    fn eval(&self, f: &Poly, x: E) -> E {
        let mut y = [0, 0, 0];
        for i in (0..f.n).rev() {
            y = self.mul(y, x);
            y[0] = (y[0] + f.c[i]) % self.p;
        }
        y
    }
}
fn count(f: &Poly, p: u64, n: usize, start: &Instant) -> u64 {
    let c = chars(p);
    let mut sum = 0i64;
    if n == 1 {
        let fp = Fp::new(p);
        for x in 0..p {
            sum += c[f.eval(x, &fp) as usize];
        }
    } else {
        let e = Ext::new(p, n);
        let third = if n == 2 { 1 } else { p };
        for a in 0..p {
            assert!(start.elapsed().as_secs() < 600, "aggregate wall cap");
            for b in 0..p {
                for z in 0..third {
                    sum += c[e.norm(e.eval(f, [a, b, z])) as usize];
                }
            }
        }
    }
    (p.pow(n as u32) as i64 + 1 + sum) as u64
}
fn prime(p: u64) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn root(p: u64) -> u64 {
    let mut n = 0;
    while n * n < p {
        n += 1;
    }
    n
}
fn choose(n: usize, k: usize) -> i128 {
    (0..k).fold(1, |v, i| v * (n - i) as i128 / (i + 1) as i128)
}
fn possibilities(s: i128, b: i128, ell: i128) -> Vec<i128> {
    let r = (-s).rem_euclid(ell);
    let lo = (-b - r).div_euclid(ell) - 1;
    let hi = (b - r).div_euclid(ell) + 1;
    (lo..=hi)
        .map(|k| r + k * ell)
        .filter(|&x| x.abs() <= b)
        .collect()
}
fn factor3(mut f: Poly) -> Vec<(Poly, usize)> {
    let fp = Fp::new(3);
    let mut out = Vec::new();
    for d in 1..=8 {
        for code in 0..3u64.pow(d as u32) {
            if f.deg() < d as isize {
                break;
            }
            let mut z = code;
            let mut c = vec![0; d + 1];
            c[d] = 1;
            for x in &mut c[..d] {
                *x = z % 3;
                z /= 3;
            }
            let a = Poly::from_coeffs(&c);
            let mut m = 0;
            loop {
                let (q, r) = f.divrem(&a, &fp);
                if !r.is_zero() {
                    break;
                }
                f = q;
                m += 1;
            }
            if m != 0 {
                out.push((a, m));
            }
        }
    }
    assert_eq!(f, Poly::one());
    out
}
fn dual(f: &Poly, q: u64) -> Poly {
    let fp = Fp::new(3);
    let d = f.n - 1;
    let mut c = vec![0; d + 1];
    let inv = fp.inv(f.c[0]);
    for i in 0..=d {
        c[d - i] = fp.mul(inv, fp.mul(f.c[i], fp.pow(q, i as u64)));
    }
    Poly::from_coeffs(&c)
}
fn screen(f: Poly, q: u64) -> (&'static str, usize) {
    let fs = factor3(f);
    let mut obstruction = false;
    let mut pairs = 0;
    for (a, m) in &fs {
        let b = dual(a, q);
        assert!(fs.iter().any(|(x, n)| *x == b && n == m));
        let selfdual = *a == b;
        if selfdual && a.deg() % 2 == 0 && m % 2 == 1 {
            obstruction = true;
        }
        if !selfdual {
            pairs += 1;
        }
        println!(
            "FACTOR coeff={:?} multiplicity={m} self_dual={selfdual}",
            &a.c[..a.n]
        );
    }
    if obstruction {
        ("obstructed", 0)
    } else if fs.iter().all(|(a, m)| *m == 1 && *a != dual(a, q)) {
        assert_eq!(pairs % 2, 0);
        ("abstract_regular_kernels", 1 << (pairs / 2))
    } else {
        ("unknown_repeated_or_selfdual", 0)
    }
}
fn controls(start: &Instant) {
    let mut elements = 0;
    for p in [5, 7, 11] {
        let c = chars(p);
        for n in [2, 3] {
            let e = Ext::new(p, n);
            for a in 0..p {
                for b in 0..p {
                    for z in 0..if n == 2 { 1 } else { p } {
                        let x = [a, b, z];
                        let got = e.pow(x, (p.pow(n as u32) - 1) / 2);
                        let expected = match c[e.norm(x) as usize] {
                            -1 => [p - 1, 0, 0],
                            0 => [0, 0, 0],
                            1 => [1, 0, 0],
                            _ => unreachable!(),
                        };
                        assert_eq!(got, expected);
                        elements += 1;
                    }
                }
            }
        }
    }
    let mut curves = 0;
    for p in [5, 7, 11] {
        let fp = Fp::new(p);
        let mut used = 0;
        for c in 1..p {
            let f = Poly::from_coeffs(&[c, 1, 0, 0, 0, 1]);
            if Poly::gcd(&f, &f.deriv(&fp), &fp) != Poly::one() {
                continue;
            }
            let c1 = count(&f, p, 1, start) as i128;
            let c2 = count(&f, p, 2, start) as i128;
            let a1 = c1 - p as i128 - 1;
            let numerator = c2 - (p * p) as i128 - 1 + a1 * a1;
            assert_eq!(numerator % 2, 0);
            let a2 = numerator / 2;
            let expected = 1 + (p * p) as i128 + (p + 1) as i128 * a1 + a2;
            let mut actual = 1;
            for u0 in 0..p {
                let u = Poly::from_coeffs(&[u0, 1]);
                for v0 in 0..p {
                    let v = Poly::from_coeffs(&[v0]);
                    if v.mul(&v, &fp).sub(&f, &fp).rem(&u, &fp).is_zero() {
                        actual += 1;
                    }
                }
            }
            for u0 in 0..p {
                for u1 in 0..p {
                    let u = Poly::from_coeffs(&[u0, u1, 1]);
                    for v0 in 0..p {
                        for v1 in 0..p {
                            let v = Poly::from_coeffs(&[v0, v1]);
                            if v.mul(&v, &fp).sub(&f, &fp).rem(&u, &fp).is_zero() {
                                actual += 1;
                            }
                        }
                    }
                }
            }
            assert_eq!(actual, expected);
            used += 1;
            curves += 1;
            if used == 3 {
                break;
            }
        }
        assert_eq!(used, 3);
    }
    for a in -20..=20 {
        assert_eq!(possibilities(101 - a, 20, 101), vec![a]);
    }
    assert!(possibilities(0, 20, 7).len() > 1);
    assert!(possibilities(50, 20, 101).is_empty());
    let fp = Fp::new(3);
    let a = Poly::from_coeffs(&[1, 0, 1]);
    assert_eq!(screen(a.mul(&a, &fp).mul(&a, &fp), 1).0, "obstructed");
    let f = Poly::from_coeffs(&[1, 2, 0, 1]);
    assert_eq!(
        screen(f.mul(&dual(&f, 1), &fp), 1),
        ("abstract_regular_kernels", 2)
    );
    println!("CONTROLS norm_elements={elements} genus2_curves={curves} congruence_cases=43 mod3_cases=2 pass=true");
}
fn orders(path: &str, projectors: bool) {
    let text = fs::read_to_string(path).unwrap();
    let mut label = String::new();
    let mut polys = Vec::new();
    let mut cases = 0;
    let mut factors = 0;
    for line in text.lines() {
        if line.starts_with("CURVE ") {
            label = line
                .split_whitespace()
                .take(4)
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ");
            polys.clear();
        }
        if line.starts_with("FACTOR ") && !label.is_empty() {
            let start = line.find('[').unwrap();
            let end = line.find(']').unwrap();
            let c: Vec<u64> = line[start + 1..end]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            polys.push(Poly::from_coeffs(&c));
        }
        if line.starts_with("CASE ") && line.contains("status=abstract_regular_kernels ") {
            assert_eq!(polys.len(), 2);
            if projectors {
                let fp = Fp::new(3);
                let chi = polys[0].mul(&polys[1], &fp);
                let (one, inverse, _) = Poly::xgcd(&polys[1], &polys[0], &fp);
                assert_eq!(one, Poly::one());
                let e0 = polys[1].mulmod(&inverse, &chi, &fp);
                let e1 = Poly::one().sub(&e0, &fp);
                assert_eq!(e0.add(&e1, &fp), Poly::one());
                assert!(e0.mulmod(&e1, &chi, &fp).is_zero());
                for (i, e) in [e0, e1].iter().enumerate() {
                    assert_eq!(e.mulmod(e, &chi, &fp), *e);
                    assert!(polys[i].mulmod(e, &chi, &fp).is_zero());
                    println!(
                        "PROJECTOR {label} kernel_factor={:?} operator={:?} domain=J[3]",
                        &polys[i].c[..polys[i].n],
                        &e.c[..e.n]
                    );
                }
            }
            for f in &polys {
                assert_eq!(factor3(*f), vec![(*f, 1)]);
                let bound = 3u64.pow(f.deg() as u32) - 1;
                let fp = Fp::new(3);
                let x = Poly::x();
                assert_eq!(x.powmod(bound, f, &fp), Poly::one());
                let order = (1..=bound)
                    .find(|&k| x.powmod(k, f, &fp) == Poly::one())
                    .unwrap();
                println!(
                    "ORDER {label} factor={:?} field_degree={order} kernel_nonzero_points={bound}",
                    &f.c[..f.n]
                );
                factors += 1;
            }
            cases += 1;
        }
    }
    assert_eq!((cases, factors), (3, 6));
    println!("TERMINAL positive_cases={cases} factors={factors}");
}
fn main() {
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some("--orders" | "--projectors")
    ) {
        orders(
            &std::env::args().nth(2).unwrap(),
            std::env::args().nth(1).as_deref() == Some("--projectors"),
        );
        return;
    }
    let start = Instant::now();
    controls(&start);
    let rootdir = std::env::args().nth(1).unwrap();
    let mut cases = 0;
    for (g, ps) in [
        (2, &[1009, 4001, 16381, 65537][..]),
        (3, &[127, 503, 1009][..]),
        (4, &[31, 61, 127][..]),
    ] {
        for &p in ps {
            for family in 0..2 {
                let text = fs::read_to_string(
                    Path::new(&rootdir).join(format!("select-g{g}-p{p}-f{family}.out")),
                )
                .unwrap();
                let mut t = text.split_whitespace();
                assert_eq!(t.next(), Some("FIXTURE"));
                let mut take = || t.next().unwrap().parse::<u64>().unwrap();
                assert_eq!(take(), g as u64);
                assert_eq!(take(), p);
                assert_eq!(take(), family);
                let _attempt = take();
                let ell = take();
                assert!(prime(p) && prime(ell));
                let mut polys = Vec::new();
                for _ in 0..3 {
                    let n = take() as usize;
                    let c: Vec<_> = (0..n).map(|_| take()).collect();
                    assert!(c.iter().all(|&x| x < p));
                    polys.push(Poly::from_coeffs(&c));
                }
                assert_eq!(polys[0].deg(), (2 * g + 1) as isize);
                let fp = Fp::new(p);
                assert_eq!(Poly::gcd(&polys[0], &polys[0].deriv(&fp), &fp), Poly::one());
                let curve = Curve::new(fp, polys[0]);
                let d1 = Div {
                    u: polys[1],
                    v: polys[2],
                };
                assert!(curve.is_valid(&d1) && !curve.is_zero(&d1));
                assert_eq!(curve.mul(&d1, ell), curve.zero());
                let mut cks = Vec::new();
                for n in 1..g {
                    cks.push(count(&polys[0], p, n, &start) as i128);
                }
                let pp = p as i128;
                let mut a = vec![0i128; g + 1];
                a[0] = 1;
                a[1] = cks[0] - pp - 1;
                if g >= 3 {
                    let z = cks[1] - pp * pp - 1 + a[1] * a[1];
                    assert_eq!(z % 2, 0);
                    a[2] = z / 2;
                }
                if g >= 4 {
                    let z = cks[2] - pp.pow(3) - 1 - a[1].pow(3) + 3 * a[1] * a[2];
                    assert_eq!(z % 3, 0);
                    a[3] = z / 3;
                }
                let mut s = 1 + pp.pow(g as u32);
                for j in 1..g {
                    s += (1 + pp.pow((g - j) as u32)) * a[j];
                }
                let b = choose(2 * g, g)
                    * pp.pow((g / 2) as u32)
                    * if g % 2 == 1 { root(p) as i128 } else { 1 };
                let cand = possibilities(s, b, ell as i128);
                assert!(!cand.is_empty());
                if cand.len() != 1 {
                    println!("CASE g={g} p={p} family={family} ell={ell} counts={cks:?} bound={b} candidates={} status=unknown_middle",cand.len());
                    cases += 1;
                    continue;
                }
                a[g] = cand[0];
                let order = s + a[g];
                assert!(order > 0 && order % ell as i128 == 0);
                let lo = (root(p) - 2).pow((2 * g) as u32) as i128;
                let hi = (root(p) + 1).pow((2 * g) as u32) as i128;
                assert!(order >= lo && order <= hi);
                let mut l = vec![0i128; 2 * g + 1];
                for j in 0..=g {
                    l[j] = a[j];
                    l[2 * g - j] = pp.pow((g - j) as u32) * a[j];
                }
                assert_eq!(l.iter().sum::<i128>(), order);
                let coeff: Vec<u64> = l.iter().rev().map(|x| x.rem_euclid(3) as u64).collect();
                println!("CURVE g={g} p={p} family={family} ell={ell} counts={cks:?} bound={b} order={order} L={l:?}");
                let (status, kernels) = screen(Poly::from_coeffs(&coeff), p % 3);
                let count_text = if status.starts_with("unknown") {
                    "null".to_string()
                } else {
                    kernels.to_string()
                };
                println!("CASE g={g} p={p} family={family} ell={ell} candidates=1 status={status} abstract_kernels={count_text}");
                cases += 1;
            }
        }
    }
    println!("TERMINAL cases={cases} all_checks=true");
}
