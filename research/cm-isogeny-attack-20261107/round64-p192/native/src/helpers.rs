use std::sync::atomic::{AtomicU64,Ordering};
static FM:AtomicU64=AtomicU64::new(0);
static FI:AtomicU64=AtomicU64::new(0);
static PA:AtomicU64=AtomicU64::new(0);
fn counts()->[u64;3]{[FM.load(Ordering::Relaxed),FI.load(Ordering::Relaxed),PA.load(Ordering::Relaxed)]}
fn diff(a:[u64;3],b:[u64;3])->[u64;3]{[b[0]-a[0],b[1]-a[1],b[2]-a[2]]}
// Field/curve helpers copied unchanged from Round56; used only by the small exact controls.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct F<const P: i64, const D: i64>(i64, i64);
impl<const P: i64, const D: i64> F<P, D> {
    fn new(a: i64, b: i64) -> Self {
        Self(a.rem_euclid(P), b.rem_euclid(P))
    }
    fn c(a: i64) -> Self {
        Self::new(a, 0)
    }
    fn pow(mut self, mut n: u64) -> Self {
        let mut z = Self::c(1);
        while n > 0 {
            if n & 1 == 1 {
                z = z * self
            }
            self = self * self;
            n >>= 1;
        }
        z
    }
    fn inv(self) -> Self {
        FI.fetch_add(1,Ordering::Relaxed);
        assert_ne!(self, Self::c(0));
        self.pow((P * P - 2) as u64)
    }
    fn code(self) -> Value {
        json!([self.0, self.1])
    }
}
impl<const P: i64, const D: i64> std::ops::Add for F<P, D> {
    type Output = Self;
    fn add(self, y: Self) -> Self {
        Self::new(self.0 + y.0, self.1 + y.1)
    }
}
impl<const P: i64, const D: i64> std::ops::Neg for F<P, D> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0, -self.1)
    }
}
impl<const P: i64, const D: i64> std::ops::Sub for F<P, D> {
    type Output = Self;
    fn sub(self, y: Self) -> Self {
        self + -y
    }
}
impl<const P: i64, const D: i64> std::ops::Mul for F<P, D> {
    type Output = Self;
    fn mul(self, y: Self) -> Self {
        FM.fetch_add(1,Ordering::Relaxed);
        Self::new(self.0 * y.0 + D * self.1 * y.1, self.0 * y.1 + self.1 * y.0)
    }
}
impl<const P: i64, const D: i64> std::ops::Div for F<P, D> {
    type Output = Self;
    fn div(self, y: Self) -> Self {
        self * y.inv()
    }
}
type Pt<const P: i64, const D: i64> = Option<(F<P, D>, F<P, D>)>;
fn pc<const P: i64, const D: i64>(p: Pt<P, D>) -> Value {
    p.map(|(x, y)| json!([x.code(), y.code()]))
        .unwrap_or(Value::Null)
}
fn neg<const P: i64, const D: i64>(p: Pt<P, D>) -> Pt<P, D> {
    p.map(|(x, y)| (x, -y))
}
fn add<const P: i64, const D: i64>(p: Pt<P, D>, q: Pt<P, D>, a: i64) -> Pt<P, D> {
    PA.fetch_add(1,Ordering::Relaxed);
    match (p, q) {
        (None, _) => q,
        (_, None) => p,
        (Some((x, y)), Some((u, v))) => {
            if x == u && y + v == F::c(0) {
                return None;
            }
            let s = if p == q {
                (F::c(3) * x * x + F::c(a)) / (F::c(2) * y)
            } else {
                (v - y) / (u - x)
            };
            let z = s * s - x - u;
            Some((z, s * (x - z) - y))
        }
    }
}
fn mul<const P: i64, const D: i64>(mut n: u64, mut p: Pt<P, D>, a: i64) -> Pt<P, D> {
    let mut z = None;
    while n > 0 {
        if n & 1 == 1 {
            z = add(z, p, a)
        }
        p = add(p, p, a);
        n >>= 1;
    }
    z
}
fn ev<const P: i64, const D: i64>(a: &[F<P, D>], x: F<P, D>) -> F<P, D> {
    a.iter().rev().fold(F::c(0), |v, &c| v * x + c)
}
fn pm<const P: i64, const D: i64>(a: &[F<P, D>], b: &[F<P, D>]) -> Vec<F<P, D>> {
    let mut c = vec![F::c(0); a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = c[i + j] + x * y
        }
    }
    c
}
fn pa<const P: i64, const D: i64>(a: &mut Vec<F<P, D>>, b: &[F<P, D>], scale: F<P, D>) {
    a.resize(a.len().max(b.len()), F::c(0));
    for (i, &v) in b.iter().enumerate() {
        a[i] = a[i] + v * scale
    }
}
fn rd<const P: i64, const D: i64>(a: &[F<P, D>], z: F<P, D>) -> Vec<F<P, D>> {
    let mut q = vec![F::c(0); a.len() - 1];
    q[a.len() - 2] = *a.last().unwrap();
    for i in (1..a.len() - 1).rev() {
        q[i - 1] = a[i] + z * q[i]
    }
    assert_eq!(a[0] + z * q[0], F::c(0));
    q
}
fn der<const P: i64, const D: i64>(a: &[F<P, D>]) -> Vec<F<P, D>> {
    (1..a.len()).map(|i| F::c(i as i64) * a[i]).collect()
}
fn deg<const P: i64, const D: i64>(a: &[F<P, D>]) -> usize {
    a.iter().rposition(|&v| v != F::c(0)).unwrap()
}
fn polypow<const P: i64, const D: i64>(a: &[F<P, D>], n: u64) -> Vec<F<P, D>> {
    let mut z = vec![F::c(1)];
    for _ in 0..n {
        z = pm(&z, a)
    }
    z
}
fn num<const P: i64, const D: i64>(
    graph: &[(Pt<P, D>, Pt<P, D>)],
    shift: Pt<P, D>,
    den: &[F<P, D>],
    a: i64,
    b: i64,
) -> (Vec<F<P, D>>, Vec<F<P, D>>) {
    let mut aa = vec![F::c(0)];
    aa.extend(den.iter().map(|&z| z * shift.unwrap().0));
    let mut bb = vec![F::c(0)];
    let mut seen = vec![];
    for &(u, v) in graph {
        if let Some((x, y)) = u {
            if seen.contains(&x) {
                continue;
            }
            seen.push(x);
            let vm = graph.iter().find(|(z, _)| *z == neg(u)).unwrap().1;
            let up = add(shift, v, a).unwrap().0;
            let um = add(shift, vm, a).unwrap().0;
            let q = rd(&rd(den, x), x);
            pa(
                &mut aa,
                &pm(&q, &[F::c(a) * x + F::c(2 * b), F::c(a) + x * x, x]),
                up + um,
            );
            pa(&mut bb, &q, (um - up) * F::c(2) * y);
        }
    }
    (aa, bb)
}
fn lift<const P: i64, const D: i64>(
    z: F<P, D>,
    shift: Pt<P, D>,
    r: u64,
    a: i64,
    b: i64,
) -> Pt<P, D> {
    let rhs = z * z * z + F::c(a) * z + F::c(b);
    let mut candidates = vec![];
    for i in 0..P {
        for j in 0..P {
            let y = F::new(i, j);
            if y * y == rhs {
                let out = add(Some((z, y)), neg(shift), a);
                if let Some((xx, yy)) = out {
                    if xx.1 == 0 && yy.1 == 0 && mul(r, out, a).is_none() {
                        candidates.push(out)
                    }
                }
            }
        }
    }
    assert_eq!(candidates.len(), 1);
    candidates[0]
}
fn trim<const P: i64, const D: i64>(mut a: Vec<F<P, D>>) -> Vec<F<P, D>> {
    while a.len() > 1 && *a.last().unwrap() == F::c(0) {
        a.pop();
    }
    a
}
fn scaled<const P: i64, const D: i64>(a: &[F<P, D>], c: F<P, D>) -> Vec<F<P, D>> {
    trim(a.iter().map(|&x| x * c).collect())
}
fn sum<const P: i64, const D: i64>(a: &[F<P, D>], b: &[F<P, D>], scale: F<P, D>) -> Vec<F<P, D>> {
    let mut z = a.to_vec();
    pa(&mut z, b, scale);
    trim(z)
}
fn divide<const P: i64, const D: i64>(
    a: &[F<P, D>],
    b: &[F<P, D>],
) -> (Vec<F<P, D>>, Vec<F<P, D>>) {
    let b = trim(b.to_vec());
    assert!(b != vec![F::c(0)]);
    let mut rem = trim(a.to_vec());
    let mut q = vec![F::c(0); a.len().saturating_sub(b.len()) + 1];
    while rem.len() >= b.len() && rem != vec![F::c(0)] {
        let k = rem.len() - b.len();
        let c = *rem.last().unwrap() / *b.last().unwrap();
        q[k] = c;
        for i in 0..b.len() {
            rem[i + k] = rem[i + k] - c * b[i]
        }
        rem = trim(rem);
    }
    (trim(q), rem)
}
fn modulo<const P: i64, const D: i64>(a: &[F<P, D>], h: &[F<P, D>]) -> Vec<F<P, D>> {
    divide(a, h).1
}
fn invpoly<const P: i64, const D: i64>(a: &[F<P, D>], h: &[F<P, D>]) -> Vec<F<P, D>> {
    let (mut r0, mut r1) = (h.to_vec(), modulo(a, h));
    let (mut s0, mut s1) = (vec![F::c(0)], vec![F::c(1)]);
    while r1 != vec![F::c(0)] {
        let (q, r) = divide(&r0, &r1);
        let s = modulo(&sum(&s0, &pm(&q, &s1), F::c(-1)), h);
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = s;
    }
    assert_eq!(r0.len(), 1);
    modulo(&scaled(&s0, r0[0].inv()), h)
}
fn interpolate<const P: i64, const D: i64>(
    xs: &[F<P, D>],
    ys: &[F<P, D>],
    h: &[F<P, D>],
) -> Vec<F<P, D>> {
    let mut out = vec![F::c(0)];
    for (&x, &y) in xs.iter().zip(ys) {
        let q = rd(h, x);
        pa(&mut out, &q, y / ev(&q, x));
    }
    trim(out)
}
fn multmatrix<const P: i64, const D: i64>(a: &[F<P, D>], h: &[F<P, D>]) -> Vec<Vec<F<P, D>>> {
    let n = h.len() - 1;
    let mut matrix = vec![vec![F::c(0); n]; n];
    let mut cur = modulo(a, h);
    for j in 0..n {
        for (i, &z) in cur.iter().enumerate() {
            matrix[i][j] = z
        }
        cur.insert(0, F::c(0));
        cur = modulo(&cur, h);
    }
    matrix
}
fn det<const P: i64, const D: i64>(mut a: Vec<Vec<F<P, D>>>) -> F<P, D> {
    let n = a.len();
    let mut out = F::c(1);
    for k in 0..n {
        let row = (k..n).find(|&i| a[i][k] != F::c(0));
        let Some(row) = row else { return F::c(0) };
        if row != k {
            a.swap(row, k);
            out = -out;
        }
        let v = a[k][k];
        out = out * v;
        let pivot = a[k].clone();
        for line in a.iter_mut().skip(k + 1) {
            let v = line[k] / v;
            for j in k..n {
                line[j] = line[j] - v * pivot[j];
            }
        }
    }
    out
}
fn curvepoly<const P: i64, const D: i64>(a: i64, b: i64) -> Vec<F<P, D>> {
    vec![F::c(b), F::c(a), F::c(0), F::c(1)]
}
fn hnum<const P: i64, const D: i64>(x: &[F<P, D>], u: F<P, D>, a: i64, b: i64) -> Vec<F<P, D>> {
    let mut z = scaled(&pm(x, x), u);
    pa(&mut z, x, F::c(a) + u * u);
    pa(&mut z, &[F::c(a) * u + F::c(2 * b)], F::c(1));
    trim(z)
}
fn evaluate_curve<const P: i64, const D: i64>(x: &[F<P, D>], a: i64, b: i64) -> Vec<F<P, D>> {
    let mut z = pm(&pm(x, x), x);
    pa(&mut z, x, F::c(a));
    pa(&mut z, &[F::c(b)], F::c(1));
    trim(z)
}
