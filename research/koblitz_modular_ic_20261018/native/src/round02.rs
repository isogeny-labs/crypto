use crate::{gf2k::SmallField, lpoly, weil};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;

pub fn save(dir: &Path, name: &str, value: &Value) {
    let path = dir.join(name);
    assert!(!path.exists(), "write-once evidence: {}", path.display());
    std::fs::write(path, serde_json::to_string_pretty(value).unwrap() + "\n").unwrap();
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Point {
    O,
    Aff(u16, u16),
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Elliptic {
    pub a1: u16,
    pub a2: u16,
    pub a3: u16,
    pub a4: u16,
    pub a6: u16,
    pub trace: i64,
}
impl Elliptic {
    pub fn models() -> Vec<Self> {
        vec![
            Self {
                a1: 1,
                a2: 0,
                a3: 0,
                a4: 0,
                a6: 1,
                trace: -1,
            },
            Self {
                a1: 1,
                a2: 1,
                a3: 0,
                a4: 0,
                a6: 1,
                trace: 1,
            },
            Self {
                a1: 0,
                a2: 0,
                a3: 1,
                a4: 1,
                a6: 0,
                trace: -2,
            },
            Self {
                a1: 0,
                a2: 0,
                a3: 1,
                a4: 0,
                a6: 0,
                trace: 0,
            },
            Self {
                a1: 0,
                a2: 0,
                a3: 1,
                a4: 1,
                a6: 1,
                trace: 2,
            },
        ]
    }
    pub fn on(&self, f: &SmallField, p: Point) -> bool {
        match p {
            Point::O => true,
            Point::Aff(x, y) => {
                f.sqr(y)
                    ^ f.mul(self.a1, f.mul(x, y))
                    ^ f.mul(self.a3, y)
                    ^ f.mul(f.sqr(x), x)
                    ^ f.mul(self.a2, f.sqr(x))
                    ^ f.mul(self.a4, x)
                    ^ self.a6
                    == 0
            }
        }
    }
    pub fn points(&self, f: &SmallField) -> Vec<Point> {
        let mut pts = vec![Point::O];
        for x in 0..f.q as u16 {
            for y in 0..f.q as u16 {
                let p = Point::Aff(x, y);
                if self.on(f, p) {
                    pts.push(p);
                }
            }
        }
        pts
    }
    pub fn neg(&self, f: &SmallField, p: Point) -> Point {
        match p {
            Point::O => p,
            Point::Aff(x, y) => Point::Aff(x, y ^ f.mul(self.a1, x) ^ self.a3),
        }
    }
    pub fn add(&self, f: &SmallField, p: Point, q: Point) -> Point {
        let (Point::Aff(x, y), Point::Aff(u, v)) = (p, q) else {
            return if p == Point::O { q } else { p };
        };
        let lambda = if x == u {
            if p == self.neg(f, q) {
                return Point::O;
            }
            f.mul(
                f.sqr(x) ^ self.a4 ^ f.mul(self.a1, y),
                f.inv(f.mul(self.a1, x) ^ self.a3),
            )
        } else {
            f.mul(y ^ v, f.inv(x ^ u))
        };
        let nu = y ^ f.mul(lambda, x);
        let xx = f.sqr(lambda) ^ f.mul(self.a1, lambda) ^ self.a2 ^ x ^ u;
        let yy = f.mul(lambda ^ self.a1, xx) ^ nu ^ self.a3;
        let result = Point::Aff(xx, yy);
        assert!(self.on(f, result));
        result
    }
    pub fn mul(&self, f: &SmallField, mut p: Point, mut k: u64) -> Point {
        let mut q = Point::O;
        while k > 0 {
            if k & 1 == 1 {
                q = self.add(f, q, p);
            }
            p = self.add(f, p, p);
            k >>= 1;
        }
        q
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
enum Term {
    One,
    X,
    Y,
    X2,
    XY,
    X3,
    X2Y,
    W(Point),
    ShiftX(Point),
    ShiftY(Point),
}
fn term_value(e: Elliptic, f: &SmallField, p: Point, term: &Term) -> u16 {
    let Point::Aff(x, y) = p else {
        panic!("O is a declared pole")
    };
    match *term {
        Term::One => 1,
        Term::X => x,
        Term::Y => y,
        Term::X2 => f.sqr(x),
        Term::XY => f.mul(x, y),
        Term::X3 => f.mul(f.sqr(x), x),
        Term::X2Y => f.mul(f.sqr(x), y),
        Term::W(Point::Aff(a, b)) => {
            if x != a {
                f.mul(y ^ b ^ f.mul(e.a1, a) ^ e.a3, f.inv(x ^ a))
            } else {
                assert_eq!(p, e.neg(f, Point::Aff(a, b)));
                f.mul(
                    f.sqr(x) ^ e.a4 ^ f.mul(e.a1, y),
                    f.inv(f.mul(e.a1, x) ^ e.a3),
                )
            }
        }
        Term::ShiftX(a) | Term::ShiftY(a) => {
            let Point::Aff(xx, yy) = e.add(f, p, e.neg(f, a)) else {
                panic!("translated pole")
            };
            if matches!(term, Term::ShiftX(_)) {
                xx
            } else {
                yy
            }
        }
        _ => panic!("invalid term"),
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Cover {
    base: Elliptic,
    pole_orders: Vec<(Point, u32)>,
    basis: Vec<Term>,
    coefficients: u64,
}
fn cover_count(c: &Cover, k: u32) -> i64 {
    let f = SmallField::new(k);
    let mut count = c.pole_orders.len() as i64;
    for p in c.base.points(&f) {
        if c.pole_orders.iter().any(|(q, _)| *q == p) {
            continue;
        }
        let mut value = 0;
        for (j, term) in c.basis.iter().enumerate() {
            if c.coefficients >> j & 1 == 1 {
                value ^= term_value(c.base, &f, p, term);
            }
        }
        count += if f.trace(value) == 0 { 2 } else { 0 };
    }
    count
}
fn variants(base: Elliptic, poles: Vec<(Point, u32)>, basis: Vec<Term>, fixed: u64) -> Vec<Cover> {
    (0..1u64 << basis.len())
        .filter(|bits| bits & fixed == fixed)
        .map(|bits| Cover {
            base,
            pole_orders: poles.clone(),
            basis: basis.clone(),
            coefficients: bits,
        })
        .collect()
}
fn lc0(dir: &Path) {
    let mut covers = Vec::new();
    let f = SmallField::new(1);
    for e in Elliptic::models() {
        assert_eq!(e.points(&f).len() as i64, 3 - e.trace);
        let finite: Vec<_> = e
            .points(&f)
            .into_iter()
            .filter(|p| *p != Point::O)
            .collect();
        covers.extend(variants(
            e,
            vec![(Point::O, 7)],
            vec![
                Term::One,
                Term::X,
                Term::Y,
                Term::X2,
                Term::XY,
                Term::X3,
                Term::X2Y,
            ],
            1 << 6,
        ));
        for &p in &finite {
            covers.extend(variants(
                e,
                vec![(Point::O, 3), (p, 3)],
                vec![
                    Term::One,
                    Term::X,
                    Term::Y,
                    Term::W(p),
                    Term::ShiftX(p),
                    Term::ShiftY(p),
                ],
                (1 << 2) | (1 << 5),
            ));
            covers.extend(variants(
                e,
                vec![(Point::O, 5), (p, 1)],
                vec![Term::One, Term::X, Term::Y, Term::X2, Term::XY, Term::W(p)],
                (1 << 4) | (1 << 5),
            ));
        }
        for i in 0..finite.len() {
            for j in i + 1..finite.len() {
                let (p, q) = (finite[i], finite[j]);
                covers.extend(variants(
                    e,
                    vec![(Point::O, 3), (p, 1), (q, 1)],
                    vec![Term::One, Term::X, Term::Y, Term::W(p), Term::W(q)],
                    (1 << 2) | (1 << 3) | (1 << 4),
                ));
                for &r in &finite[j + 1..] {
                    covers.extend(variants(
                        e,
                        vec![(Point::O, 1), (p, 1), (q, 1), (r, 1)],
                        vec![Term::One, Term::W(p), Term::W(q), Term::W(r)],
                        14,
                    ));
                }
            }
        }
    }
    assert_eq!(covers.len(), 690);
    let targets: Vec<_> = [3, 5]
        .into_iter()
        .flat_map(|n| {
            [-1, 1]
                .into_iter()
                .map(move |t| (n, t, weil::to_i128(&weil::trace_zero_poly(t, n))))
        })
        .collect();
    let mut hits = Vec::new();
    let mut models = Vec::new();
    for (id, c) in covers.iter().enumerate() {
        assert_eq!(c.pole_orders.iter().map(|(_, m)| m + 1).sum::<u32>(), 8);
        let counts: Vec<_> = (1..=5).map(|k| cover_count(c, k)).collect();
        let polynomial = lpoly::char_poly_from_counts(&counts, 2)
            .expect("smooth AS normalization must satisfy Newton integrality");
        assert!(lpoly::divide_exact(&polynomial, &[2, -(c.base.trace as i128), 1]).is_some());
        for (n, t, a) in &targets {
            if let Some(cofactor) = lpoly::divide_exact(&polynomial, a) {
                let extra: Vec<_> = (6..=8).map(|k| cover_count(c, k)).collect();
                let predicted = lpoly::counts_from_char_poly(&polynomial, 2, 8);
                assert_eq!(
                    extra,
                    predicted[5..].iter().map(|v| *v as i64).collect::<Vec<_>>()
                );
                hits.push(
                    json!({"model":id,"n":n,"trace":t,"cofactor":cofactor,"extra_counts":extra}),
                );
            }
        }
        models.push(json!({"id":id,"model":c,"counts":counts,"polynomial":polynomial}));
    }
    save(
        dir,
        "lc0-bielliptic.json",
        &json!({"schema":"koblitz.experiment/lc0-v1","models_tested":models.len(),"genus":5,"degree_over_elliptic":2,"models":models,"hits":hits,"scope":"exact rational-pole families in protocol, no isomorphism deduplication; smooth normalizations via odd-pole AS formula"}),
    );
    println!(
        "LC0 complete: {} models; {} target hits",
        covers.len(),
        hits.len()
    );
}

fn homogeneous_powers(f: &SmallField, x: usize, d: usize) -> Vec<u16> {
    if x == f.q {
        let mut v = vec![0; d + 1];
        v[d] = 1;
        v
    } else {
        (0..=d).map(|i| f.pow(x as u16, i as u64)).collect()
    }
}

fn peval(f: &SmallField, p: &[u16], x: u16) -> u16 {
    p.iter().rev().fold(0, |a, c| f.mul(a, x) ^ c)
}
fn pderiv(p: &[u16]) -> Vec<u16> {
    (1..p.len())
        .map(|i| if i % 2 == 1 { p[i] } else { 0 })
        .collect()
}
fn pmul(f: &SmallField, a: &[u16], b: &[u16]) -> Vec<u16> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] ^= f.mul(*x, *y);
        }
    }
    out
}
fn rational_as_count(e: Elliptic, h: &[u16], a: &[u16], b: &[u16], constant: u16, k: u32) -> i64 {
    let f = SmallField::new(k);
    let mut count = 1; // O is one totally ramified rational point.
    for p in e.points(&f) {
        let Point::Aff(x, y) = p else {
            continue;
        };
        let den = peval(&f, h, x);
        let numerator = f.mul(peval(&f, a, x), y) ^ peval(&f, b, x);
        let value = if den != 0 {
            f.mul(numerator, f.inv(den))
        } else if numerator != 0 {
            count += 1;
            continue;
        } else {
            let slope = f.mul(
                f.sqr(x) ^ e.a4 ^ f.mul(e.a1, y),
                f.inv(f.mul(e.a1, x) ^ e.a3),
            );
            let deriv = f.mul(peval(&f, &pderiv(a), x), y)
                ^ f.mul(peval(&f, a, x), slope)
                ^ peval(&f, &pderiv(b), x);
            f.mul(deriv, f.inv(peval(&f, &pderiv(h), x)))
        };
        if f.trace(value ^ constant) == 0 {
            count += 2;
        }
    }
    count
}
fn lc0b(dir: &Path) {
    let f = SmallField::new(3);
    let e = Elliptic::models()[1];
    let mut seen = Vec::new();
    let mut models = Vec::new();
    let mut hits = Vec::new();
    for p in e.points(&f) {
        let Point::Aff(x, y) = p else {
            continue;
        };
        if x <= 1 && y <= 1 || seen.contains(&p) {
            continue;
        }
        let orbit = vec![
            p,
            Point::Aff(f.sqr(x), f.sqr(y)),
            Point::Aff(f.pow(x, 4), f.pow(y, 4)),
        ];
        assert_eq!(orbit.iter().filter(|q| **q == p).count(), 1);
        seen.extend(orbit.clone());
        let h = orbit.iter().fold(vec![1], |a, p| {
            let Point::Aff(xx, _) = p else { unreachable!() };
            pmul(&f, &a, &[*xx, 1])
        });
        assert!(h.iter().all(|c| *c <= 1));
        for c in 1..8 {
            if f.trace(c) != 1 {
                continue;
            }
            let mut a = vec![0; 3];
            let mut b = vec![0; 3];
            for i in 0..3 {
                let Point::Aff(xx, yy) = orbit[i] else {
                    unreachable!()
                };
                let mut other = vec![1];
                for (j, q) in orbit.iter().enumerate() {
                    if i != j {
                        let Point::Aff(xq, _) = q else { unreachable!() };
                        other = pmul(&f, &other, &[*xq, 1]);
                    }
                }
                let ci = f.pow(c, 1 << i);
                for j in 0..3 {
                    a[j] ^= f.mul(ci, other[j]);
                    b[j] ^= f.mul(f.mul(ci, yy ^ xx), other[j]);
                }
            }
            assert!(a.iter().chain(&b).all(|c| *c <= 1));
            // Verify the traced expression and its removable-value rule at F_8 points.
            for q in e.points(&f) {
                if q == Point::O || orbit.contains(&q) {
                    continue;
                }
                let mut v = 0;
                for i in 0..3 {
                    v ^= f.mul(f.pow(c, 1 << i), term_value(e, &f, q, &Term::W(orbit[i])));
                }
                let Point::Aff(xx, yy) = q else {
                    unreachable!()
                };
                let den = peval(&f, &h, xx);
                if den != 0 {
                    assert_eq!(
                        v,
                        f.mul(f.mul(peval(&f, &a, xx), yy) ^ peval(&f, &b, xx), f.inv(den))
                    );
                } else {
                    // Independent agreement of the removable limit of the
                    // combined rational expression with the sum of limits.
                    assert_eq!(f.mul(peval(&f, &a, xx), yy) ^ peval(&f, &b, xx), 0);
                    let slope = f.mul(f.sqr(xx) ^ yy, f.inv(xx));
                    let derivative = f.mul(peval(&f, &pderiv(&a), xx), yy)
                        ^ f.mul(peval(&f, &a, xx), slope)
                        ^ peval(&f, &pderiv(&b), xx);
                    assert_eq!(v, f.mul(derivative, f.inv(peval(&f, &pderiv(&h), xx))));
                }
            }
            for constant in 0..2 {
                let counts: Vec<_> = (1..=5)
                    .map(|k| rational_as_count(e, &h, &a, &b, constant, k))
                    .collect();
                let poly = lpoly::char_poly_from_counts(&counts, 2).unwrap();
                assert!(lpoly::divide_exact(&poly, &[2, -1, 1]).is_some());
                let id = models.len();
                for t in [-1, 1] {
                    if let Some(cofactor) =
                        lpoly::divide_exact(&poly, &weil::to_i128(&weil::trace_zero_poly(t, 5)))
                    {
                        let extra: Vec<_> = (6..=8)
                            .map(|k| rational_as_count(e, &h, &a, &b, constant, k))
                            .collect();
                        let expected = lpoly::counts_from_char_poly(&poly, 2, 8);
                        assert_eq!(
                            extra,
                            expected[5..].iter().map(|v| *v as i64).collect::<Vec<_>>()
                        );
                        hits.push(
                            json!({"model":id,"trace":t,"cofactor":cofactor,"extra_counts":extra}),
                        );
                    }
                }
                models.push(json!({"id":id,"degree3_place_F8":orbit,"trace_parameter":c,"denominator":h,"numerator_y":a,"numerator_constant":b,"constant_twist":constant,"counts":counts,"polynomial":poly}));
            }
        }
    }
    assert_eq!(models.len(), 32);
    save(
        dir,
        "lc0b-closed-branch.json",
        &json!({"schema":"koblitz.experiment/lc0b-v1","models_tested":models.len(),"genus":5,"base_trace":1,"models":models,"hits":hits,"scope":"O plus each degree-three place on E1; every trace-one parameter and constant twist"}),
    );
    println!(
        "LC0b complete: {} models, {} target hits",
        models.len(),
        hits.len()
    );
}
fn direct_bidegree(a: usize, b: usize, bits: usize, k: u32) -> i64 {
    let f = SmallField::new(k);
    let mut count = 0;
    for x in 0..=f.q {
        let xp = homogeneous_powers(&f, x, a);
        for y in 0..=f.q {
            let yp = homogeneous_powers(&f, y, b);
            let mut val = 0;
            for (i, xx) in xp.iter().enumerate() {
                for (j, yy) in yp.iter().enumerate() {
                    if bits >> (i * (b + 1) + j) & 1 == 1 {
                        val ^= f.mul(*xx, *yy);
                    }
                }
            }
            count += i64::from(val == 0);
        }
    }
    count
}
fn all_bidegree_counts(a: usize, b: usize, k: u32) -> Vec<i16> {
    let f = SmallField::new(k);
    let vars = (a + 1) * (b + 1);
    let len = 1usize << vars;
    let mut histogram = vec![0i64; len];
    for x in 0..=f.q {
        let xp = homogeneous_powers(&f, x, a);
        for y in 0..=f.q {
            let yp = homogeneous_powers(&f, y, b);
            let vals: Vec<_> = xp
                .iter()
                .flat_map(|xx| yp.iter().map(|yy| f.mul(*xx, *yy)))
                .collect();
            for u in 0..f.q as u16 {
                let mut mask = 0usize;
                for (j, v) in vals.iter().enumerate() {
                    mask |= (f.trace(f.mul(u, *v)) as usize) << j;
                }
                histogram[mask] += 1;
            }
        }
    }
    let mut stride = 1;
    while stride < len {
        for base in (0..len).step_by(2 * stride) {
            for j in 0..stride {
                let x = histogram[base + j];
                let y = histogram[base + j + stride];
                histogram[base + j] = x + y;
                histogram[base + j + stride] = x - y;
            }
        }
        stride *= 2;
    }
    histogram
        .into_iter()
        .map(|s| {
            assert_eq!(s % f.q as i64, 0);
            i16::try_from(s / f.q as i64).unwrap()
        })
        .collect()
}
// Exact bivariate F_2 Jacobian ideals, graded lexicographic monomials.
type Poly = BTreeSet<(u16, u16)>;
fn lead(p: &Poly) -> Option<(u16, u16)> {
    p.iter().copied().max_by_key(|(x, y)| (x + y, *x))
}
fn shifted(p: &Poly, x: u16, y: u16) -> Poly {
    p.iter().map(|(a, b)| (a + x, b + y)).collect()
}
fn xor(a: &mut Poly, b: &Poly) {
    for v in b {
        if !a.insert(*v) {
            a.remove(v);
        }
    }
}
fn reduce(mut p: Poly, basis: &[Poly], steps: &mut usize) -> Option<Poly> {
    let mut remainder = Poly::new();
    while let Some((x, y)) = lead(&p) {
        *steps += 1;
        if *steps > 1_000_000 {
            return None;
        }
        let divisor = basis.iter().find(|b| {
            let (a, c) = lead(b).unwrap();
            a <= x && c <= y
        });
        if let Some(b) = divisor {
            let (a, c) = lead(b).unwrap();
            xor(&mut p, &shifted(b, x - a, y - c));
        } else {
            p.remove(&(x, y));
            remainder.insert((x, y));
        }
    }
    Some(remainder)
}
fn unit_ideal(input: Vec<Poly>) -> Option<bool> {
    let mut basis = Vec::new();
    let mut steps = 0;
    for p in input {
        let r = reduce(p, &basis, &mut steps)?;
        if r.contains(&(0, 0)) && r.len() == 1 {
            return Some(true);
        }
        if !r.is_empty() {
            basis.push(r);
        }
    }
    let mut pairs = Vec::new();
    for i in 0..basis.len() {
        for j in i + 1..basis.len() {
            pairs.push((i, j));
        }
    }
    while let Some((i, j)) = pairs.pop() {
        let (a, b) = lead(&basis[i]).unwrap();
        let (c, d) = lead(&basis[j]).unwrap();
        let mut s = shifted(&basis[i], c.max(a) - a, d.max(b) - b);
        xor(&mut s, &shifted(&basis[j], c.max(a) - c, d.max(b) - d));
        let r = reduce(s, &basis, &mut steps)?;
        if r.len() == 1 && r.contains(&(0, 0)) {
            return Some(true);
        }
        if !r.is_empty() {
            let k = basis.len();
            for i in 0..k {
                pairs.push((i, k));
            }
            basis.push(r);
            if basis.len() > 1000 {
                return None;
            }
        }
    }
    Some(false)
}
fn smooth_bidegree(a: usize, b: usize, bits: usize) -> Option<bool> {
    for ix in [false, true] {
        for iy in [false, true] {
            let mut p = Poly::new();
            for i in 0..=a {
                for j in 0..=b {
                    if bits >> (i * (b + 1) + j) & 1 == 1 {
                        p.insert((
                            if ix { a - i } else { i } as u16,
                            if iy { b - j } else { j } as u16,
                        ));
                    }
                }
            }
            let dx = p
                .iter()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(i, j)| (i - 1, *j))
                .collect();
            let dy = p
                .iter()
                .filter(|(_, j)| j % 2 == 1)
                .map(|(i, j)| (*i, j - 1))
                .collect();
            if !unit_ideal(vec![p, dx, dy])? {
                return Some(false);
            }
        }
    }
    Some(true)
}
fn lc1(dir: &Path) {
    let (a, b) = (3, 4);
    let len = 1usize << 20;
    let mut counts = Vec::new();
    let mut controls = Vec::new();
    for k in 1..=6 {
        let nk = all_bidegree_counts(a, b, k);
        for bits in [0, 1, 2, 31, 1023, 142857, 524287, 999983, len - 1] {
            let direct = direct_bidegree(a, b, bits, k);
            assert_eq!(direct, nk[bits] as i64);
            controls.push(json!({"k":k,"bits":bits,"count":direct}));
        }
        counts.push(nk);
        println!("LC1 point-count field degree {k} complete");
    }
    let targets: Vec<_> = [-1, 1]
        .into_iter()
        .map(|t| (t, weil::to_i128(&weil::trace_zero_poly(t, 5))))
        .collect();
    let mut candidates = Vec::new();
    let mut hits = Vec::new();
    let mut integral = 0;
    for bits in 1..len {
        let ns: Vec<_> = counts.iter().map(|c| c[bits] as i64).collect();
        let Some(poly) = lpoly::char_poly_from_counts(&ns, 2) else {
            continue;
        };
        integral += 1;
        for (t, target) in &targets {
            if let Some(cofactor) = lpoly::divide_exact(&poly, target) {
                let smooth = smooth_bidegree(a, b, bits);
                let extra = if smooth == Some(true) {
                    let e: Vec<_> = (7..=9).map(|k| direct_bidegree(a, b, bits, k)).collect();
                    let expected = lpoly::counts_from_char_poly(&poly, 2, 9);
                    assert_eq!(
                        e,
                        expected[6..].iter().map(|v| *v as i64).collect::<Vec<_>>()
                    );
                    Some(e)
                } else {
                    None
                };
                let record = json!({"bits":bits,"trace":t,"counts":ns,"polynomial":poly,"cofactor":cofactor,"smooth":smooth,"extra_counts":extra});
                candidates.push(record.clone());
                if smooth == Some(true) {
                    hits.push(record);
                }
            }
        }
    }
    save(
        dir,
        "lc1-bidegree34.json",
        &json!({"schema":"koblitz.experiment/lc1-v1","bidegree":[a,b],"arithmetic_genus":6,"forms_tested":len-1,"newton_integral":integral,"direct_controls":controls,"candidates":candidates,"smooth_hits":hits,"scope":"all F2 bidegree (3,4) forms; four affine Jacobian ideals classify candidates; cap failures are null, not negative"}),
    );
    println!(
        "LC1 complete: {} forms, {} divisibility candidates, {} smooth hits",
        len - 1,
        candidates.len(),
        hits.len()
    );
}

fn residue(x: BigInt, p: &BigInt) -> BigInt {
    x.mod_floor(p)
}
pub(super) fn rank(mut rows: Vec<Vec<BigInt>>, p: &BigInt) -> usize {
    if rows.is_empty() {
        return 0;
    }
    let mut r = 0;
    for col in 0..rows[0].len() {
        let Some(pivot) = (r..rows.len()).find(|i| !rows[*i][col].is_zero()) else {
            continue;
        };
        rows.swap(r, pivot);
        let gcd = rows[r][col].extended_gcd(p);
        assert!(gcd.gcd.is_one());
        let inv = residue(gcd.x, p);
        for c in col..rows[r].len() {
            rows[r][c] = residue(&rows[r][c] * &inv, p);
        }
        for i in r + 1..rows.len() {
            let factor = rows[i][col].clone();
            if !factor.is_zero() {
                for c in col..rows[i].len() {
                    rows[i][c] = residue(&rows[i][c] - &factor * &rows[r][c], p);
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    r
}
fn mr0(dir: &Path) {
    let mut results = Vec::new();
    for (n, t, rstr) in [
        (3, 1, "7"),
        (5, -1, "11"),
        (5, 1, "11"),
        (7, -1, "29"),
        (131, -1, "680564733841876926932320129493409985129"),
    ] {
        let r = rstr.parse::<BigInt>().unwrap();
        let mut one = Vec::new();
        for j in 0..n {
            let mut row = vec![BigInt::zero(); n];
            row[j] += 1;
            row[(j + 1) % n] -= t;
            row[(j + 2) % n] += 2;
            row.iter_mut().for_each(|x| *x = residue(x.clone(), &r));
            one.push(row);
        }
        let rank_one = rank(one.clone(), &r);
        let mut both = Vec::new();
        for block in 0..2 {
            for row in &one {
                let mut doubled = vec![BigInt::zero(); 2 * n];
                doubled[block * n..(block + 1) * n].clone_from_slice(row);
                both.push(doubled);
            }
        }
        let rank_two = rank(both.clone(), &r);
        let mut anchor = vec![BigInt::zero(); 2 * n];
        anchor[0] = BigInt::one();
        both.push(anchor);
        let anchored = rank(both.clone(), &r);
        let mut cross = vec![BigInt::zero(); 2 * n];
        cross[0] = -BigInt::one();
        cross[n] = BigInt::one();
        both.push(cross);
        let cross_rank = rank(both, &r);
        let mut point_checks = 0;
        if n <= 7 {
            let f = SmallField::new(n as u32);
            let e = Elliptic::models()
                .into_iter()
                .find(|e| e.trace == t as i64)
                .unwrap();
            let rr = rstr.parse::<u64>().unwrap();
            let pts = e.points(&f);
            let g = pts
                .iter()
                .copied()
                .map(|p| e.mul(&f, p, pts.len() as u64 / rr))
                .find(|p| *p != Point::O)
                .unwrap();
            assert_eq!(e.mul(&f, g, rr), Point::O);
            for k in 0..rr {
                let p = e.mul(&f, g, k);
                let sigma_inv = |p| match p {
                    Point::O => p,
                    Point::Aff(x, y) => Point::Aff(f.pow(x, 1 << (n - 1)), f.pow(y, 1 << (n - 1))),
                };
                let z = sigma_inv(p);
                let zz = sigma_inv(z);
                let middle = if t == 1 { e.neg(&f, z) } else { z };
                assert_eq!(e.add(&f, e.add(&f, p, middle), e.mul(&f, zz, 2)), Point::O);
                point_checks += 1;
            }
        }
        assert_eq!(
            (rank_one, rank_two, anchored, cross_rank),
            (n - 1, 2 * n - 2, 2 * n - 1, 2 * n)
        );
        results.push(json!({"n":n,"trace":t,"r":rstr,"rank_one_orbit":rank_one,"rank_two_orbits":rank_two,"rank_two_with_anchor":anchored,"rank_with_cross_orbit_row":cross_rank,"point_relations_verified":point_checks,"cross_orbit_row_is_supplied_control":true}));
    }
    save(
        dir,
        "mr0-orbit-rank.json",
        &json!({"schema":"koblitz.experiment/mr0-v1","results":results,"scope":"exact orbit-recurrence matrices; cross-orbit row is an information control, not generated by the modular recurrence"}),
    );
    println!("MR0 complete");
}
fn carmichael(n: u64) -> u64 {
    crate::modular::factorize(n)
        .into_iter()
        .map(|(p, k)| p.pow(k - 1) * (p - 1))
        .fold(1, |a, b| a.lcm(&b))
}
fn mr1(dir: &Path) {
    let mut rows = Vec::new();
    for (n, ell, rstr) in [
        (3, 7, "7"),
        (5, 11, "11"),
        (131, 787, "680564733841876926932320129493409985129"),
    ] {
        let level = 49 * ell * ell;
        let d = carmichael(level);
        let r = rstr.parse::<BigInt>().unwrap();
        for p in [3u64, 5, 13, 17, 19] {
            if level % p == 0 {
                continue;
            }
            // CM trace from an independent point count on the integral conductor-49 model.
            let mut points = 1;
            for x in 0..p {
                for y in 0..p {
                    let lhs = (y * y + x * y) % p;
                    let rhs = ((x * x % p * x) as i64 - (x * x) as i64 - 2 * x as i64 - 1)
                        .rem_euclid(p as i64) as u64;
                    if lhs == rhs {
                        points += 1;
                    }
                }
            }
            let ap = p as i64 + 1 - points;
            if ap != 0 {
                continue;
            }
            let value = residue(
                BigInt::from(-(p as i64)).modpow(&BigInt::from(d), &r) - 1,
                &r,
            );
            let bezout = value.extended_gcd(&r);
            assert_eq!(&bezout.x * &value + &bezout.y * &r, bezout.gcd);
            rows.push(json!({"n":n,"level":level,"p":p,"cm_a_p":ap,"unit_group_exponent":d,"r":rstr,"power_minus_one_mod_r":value.to_string(),"gcd":bezout.gcd.to_string(),"bezout_value":bezout.x.to_string(),"bezout_r":bezout.y.to_string(),"necessary_condition_passes":bezout.gcd!=BigInt::one()}));
        }
    }
    save(
        dir,
        "mr1-eisenstein-screen.json",
        &json!({"schema":"koblitz.experiment/mr1-v1","rows":rows,"scope":"necessary root-of-unity character-pair congruence screen only; no blanket cuspidal-divisor exclusion"}),
    );
    println!("MR1 complete");
}
fn gl0(dir: &Path) {
    // Convert P(X)=X^g h(X+2/X) into the real Weil polynomial by triangular subtraction.
    let mut rows = Vec::new();
    for t in [-1, 1] {
        let p = weil::to_i128(&weil::trace_zero_poly(t, 5));
        let g = 4usize;
        let mut rem = p;
        let mut h = vec![0i128; g + 1];
        for k in (0..=g).rev() {
            let c = rem[g + k];
            h[k] = c;
            for j in 0..=k {
                let binomial = (0..j).fold(1i128, |acc, s| acc * (k - s) as i128 / (s + 1) as i128);
                rem[g + k - 2 * j] -= c * binomial * 2i128.pow(j as u32);
            }
        }
        assert!(rem.iter().all(|c| *c == 0));
        for base_t in -2i128..=2 {
            let value = h.iter().rev().fold(0, |acc, c| acc * base_t + c);
            let mut factors = Vec::new();
            let mut v = value.unsigned_abs();
            let mut q = 2;
            while q * q <= v {
                if v % q == 0 {
                    factors.push(q);
                    while v % q == 0 {
                        v /= q;
                    }
                }
                q += 1;
            }
            if v > 1 {
                factors.push(v);
            }
            rows.push(json!({"target_n":5,"target_trace":t,"real_weil_coefficients":h,"elliptic_trace":base_t,"real_resultant":value,"candidate_gluing_primes":factors,"polarization_constructed":false,"curve_constructed":false}));
        }
    }
    save(
        dir,
        "gl0-resultants.json",
        &json!({"schema":"koblitz.experiment/gl0-v1","rows":rows,"scope":"necessary resultant data; no gluing/polarization existence assertion"}),
    );
    println!("GL0 complete");
}
pub fn run(dir: &Path) {
    assert!(dir.is_dir());
    assert!(!dir.join("lc0-bielliptic.json").exists());
    lc0(dir);
    lc0b(dir);
    mr0(dir);
    mr1(dir);
    gl0(dir);
    lc1(dir);
}

pub fn unbalanced(path: &Path) {
    assert!(!path.exists());
    let monos: Vec<_> = (0..4)
        .flat_map(|i| (0..=7 - 2 * i).map(move |j| (i, j)))
        .collect();
    assert_eq!(monos.len(), 20);
    let mut vectors = Vec::new();
    let mut controls = Vec::new();
    for k in 1..=6 {
        let f = SmallField::new(k);
        let mut values = Vec::new();
        for x in 0..=f.q {
            for y in 0..=f.q {
                let yp = homogeneous_powers(&f, y, 3);
                values.push(
                    monos
                        .iter()
                        .map(|(i, j)| {
                            let xp = if x == f.q {
                                u16::from(*j == 7 - 2 * i)
                            } else {
                                f.pow(x as u16, *j as u64)
                            };
                            f.mul(xp, yp[*i])
                        })
                        .collect::<Vec<_>>(),
                );
            }
        }
        let mut hist = vec![0i64; 1 << 20];
        for val in &values {
            for u in 0..f.q as u16 {
                let mask = val.iter().enumerate().fold(0usize, |a, (i, v)| {
                    a | ((f.trace(f.mul(u, *v)) as usize) << i)
                });
                hist[mask] += 1;
            }
        }
        let mut s = 1;
        while s < hist.len() {
            for a in (0..hist.len()).step_by(2 * s) {
                for j in 0..s {
                    let x = hist[a + j];
                    let y = hist[a + j + s];
                    hist[a + j] = x + y;
                    hist[a + j + s] = x - y;
                }
            }
            s *= 2;
        }
        let ns: Vec<_> = hist
            .into_iter()
            .map(|x| {
                assert_eq!(x % f.q as i64, 0);
                (x / f.q as i64) as i16
            })
            .collect();
        for bits in [0, 1, 31, 12345, 142857, 999983, 1048575] {
            let direct = values
                .iter()
                .filter(|v| {
                    v.iter()
                        .enumerate()
                        .filter(|(i, _)| bits >> i & 1 == 1)
                        .fold(0, |a, (_, x)| a ^ x)
                        == 0
                })
                .count();
            assert_eq!(direct, ns[bits] as usize);
            controls.push(json!({"k":k,"bits":bits,"count":direct}));
        }
        vectors.push(ns);
        println!("LC1b field degree {k} complete");
    }
    let mut candidates = Vec::new();
    for bits in 1..1 << 20 {
        let ns: Vec<_> = vectors.iter().map(|v| v[bits] as i64).collect();
        let Some(poly) = lpoly::char_poly_from_counts(&ns, 2) else {
            continue;
        };
        for t in [-1, 1] {
            if let Some(q) =
                lpoly::divide_exact(&poly, &weil::to_i128(&weil::trace_zero_poly(t, 5)))
            {
                candidates.push(json!({"bits":bits,"trace":t,"counts":ns,"polynomial":poly,"cofactor":q,"smoothness":"unverified"}));
            }
        }
    }
    std::fs::write(path,serde_json::to_string_pretty(&json!({"schema":"koblitz.experiment/lc1b-v1","surface":"Hirzebruch F2","divisor_class":[3,7],"arithmetic_genus":6,"monomials_fiber_base":monos,"forms_tested":1048575,"direct_controls":controls,"candidates":candidates})).unwrap()+"\n").unwrap();
    println!(
        "LC1b complete: 1048575 forms, {} candidates",
        candidates.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elliptic_laws_on_small_fields() {
        for k in 1..=3 {
            let f = SmallField::new(k);
            for e in Elliptic::models() {
                let pts = e.points(&f);
                for p in &pts {
                    assert_eq!(e.add(&f, *p, e.neg(&f, *p)), Point::O);
                    for q in &pts {
                        assert_eq!(e.add(&f, *p, *q), e.add(&f, *q, *p));
                        for r in &pts {
                            assert_eq!(
                                e.add(&f, e.add(&f, *p, *q), *r),
                                e.add(&f, *p, e.add(&f, *q, *r))
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn walsh_matches_direct_every_small_form() {
        for k in 1..=3 {
            let v = all_bidegree_counts(1, 2, k);
            for (bits, n) in v.iter().enumerate() {
                assert_eq!(*n as i64, direct_bidegree(1, 2, bits, k));
            }
        }
    }
    #[test]
    fn ideals_distinguish_singular_and_empty() {
        let x: Poly = [(1, 0)].into_iter().collect();
        let one: Poly = [(0, 0)].into_iter().collect();
        assert_eq!(unit_ideal(vec![x.clone()]), Some(false));
        assert_eq!(unit_ideal(vec![x, one]), Some(true));
    }
}
