// Frozen existing generic Cantor engine;see legacy.sha256.
pub trait Fld {
    type E: Copy + Eq + std::fmt::Debug;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn sub(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn neg(&self, a: &Self::E) -> Self::E;
    fn mul(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn inv(&self, a: &Self::E) -> Self::E;
    fn is_zero(&self, a: &Self::E) -> bool {
        *a == self.zero()
    }
}
pub type Poly<E> = Vec<E>;

pub(crate) fn ptrim<F: Fld>(f: &F, p: &mut Poly<F::E>) {
    while p.last().is_some_and(|c| f.is_zero(c)) {
        p.pop();
    }
}
pub(crate) fn padd<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> Poly<F::E> {
    let n = a.len().max(b.len());
    let mut out: Poly<F::E> = (0..n)
        .map(|i| match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) => f.add(x, y),
            (Some(x), None) => *x,
            (None, Some(y)) => *y,
            _ => f.zero(),
        })
        .collect();
    ptrim(f, &mut out);
    out
}
pub(crate) fn psub<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> Poly<F::E> {
    let n = a.len().max(b.len());
    let mut out: Poly<F::E> = (0..n)
        .map(|i| match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) => f.sub(x, y),
            (Some(x), None) => *x,
            (None, Some(y)) => f.neg(y),
            _ => f.zero(),
        })
        .collect();
    ptrim(f, &mut out);
    out
}
pub(crate) fn pmul<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> Poly<F::E> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![f.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if f.is_zero(x) {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            out[i + j] = f.add(&out[i + j], &f.mul(x, y));
        }
    }
    ptrim(f, &mut out);
    out
}
pub(crate) fn pscale<F: Fld>(f: &F, a: &[F::E], k: &F::E) -> Poly<F::E> {
    let mut out: Poly<F::E> = a.iter().map(|x| f.mul(x, k)).collect();
    ptrim(f, &mut out);
    out
}
/// `a = q b + r`, `deg r < deg b`.
pub(crate) fn pdivrem<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> (Poly<F::E>, Poly<F::E>) {
    assert!(!b.is_empty(), "division by the zero polynomial");
    let mut r: Poly<F::E> = a.to_vec();
    ptrim(f, &mut r);
    if r.len() < b.len() {
        return (Vec::new(), r);
    }
    let lci = f.inv(b.last().unwrap());
    let mut q = vec![f.zero(); r.len() - b.len() + 1];
    while r.len() >= b.len() {
        let k = r.len() - b.len();
        let c = f.mul(r.last().unwrap(), &lci);
        q[k] = c;
        for (i, x) in b.iter().enumerate() {
            let t = f.mul(&c, x);
            r[k + i] = f.sub(&r[k + i], &t);
        }
        // the leading term is now exactly zero
        r.pop();
        ptrim(f, &mut r);
    }
    ptrim(f, &mut q);
    (q, r)
}
pub(crate) fn pmonic<F: Fld>(f: &F, a: &[F::E]) -> Poly<F::E> {
    match a.last() {
        None => Vec::new(),
        Some(lc) => {
            let i = f.inv(lc);
            let mut out: Poly<F::E> = a.iter().map(|x| f.mul(x, &i)).collect();
            *out.last_mut().unwrap() = f.one();
            out
        }
    }
}
/// `(g, s, t)` with `g = s a + t b` monic (or zero).
pub(crate) fn pxgcd<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> (Poly<F::E>, Poly<F::E>, Poly<F::E>) {
    let (mut r0, mut r1) = (a.to_vec(), b.to_vec());
    let (mut s0, mut s1): (Poly<F::E>, Poly<F::E>) = (vec![f.one()], Vec::new());
    let (mut t0, mut t1): (Poly<F::E>, Poly<F::E>) = (Vec::new(), vec![f.one()]);
    ptrim(f, &mut r0);
    ptrim(f, &mut r1);
    while !r1.is_empty() {
        let (q, r) = pdivrem(f, &r0, &r1);
        let s = psub(f, &s0, &pmul(f, &q, &s1));
        let t = psub(f, &t0, &pmul(f, &q, &t1));
        r0 = std::mem::replace(&mut r1, r);
        s0 = std::mem::replace(&mut s1, s);
        t0 = std::mem::replace(&mut t1, t);
    }
    match r0.last() {
        None => (r0, s0, t0),
        Some(lc) => {
            let i = f.inv(lc);
            (pscale(f, &r0, &i), pscale(f, &s0, &i), pscale(f, &t0, &i))
        }
    }
}
pub(crate) fn peval<F: Fld>(f: &F, a: &[F::E], x: &F::E) -> F::E {
    a.iter()
        .rev()
        .fold(f.zero(), |acc, c| f.add(&f.mul(&acc, x), c))
}
/// `a mod m` (`m ≠ 0`).
pub(crate) fn pmod<F: Fld>(f: &F, a: &[F::E], m: &[F::E]) -> Poly<F::E> {
    pdivrem(f, a, m).1
}

// ── Jacobians of y² = f(x), f monic of odd degree, by Cantor ──────────────

/// A divisor class in Mumford form `(u, v)`, `u` monic, `v² ≡ f (mod u)`;
/// reduced when `deg u ≤ g`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Div<E> {
    pub u: Poly<E>,
    pub v: Poly<E>,
}

pub struct Hyp<'a, F: Fld> {
    pub f: &'a F,
    /// `f(x)`, monic of degree `2g + 1`.
    pub h: Poly<F::E>,
    pub g: usize,
}

impl<'a, F: Fld> Hyp<'a, F> {
    pub fn identity(&self) -> Div<F::E> {
        Div {
            u: vec![self.f.one()],
            v: Vec::new(),
        }
    }
    pub fn is_identity(&self, d: &Div<F::E>) -> bool {
        d.u.len() == 1
    }
    pub fn neg(&self, d: &Div<F::E>) -> Div<F::E> {
        Div {
            u: d.u.clone(),
            v: d.v.iter().map(|c| self.f.neg(c)).collect(),
        }
    }
    /// Is `(u, v)` a valid Mumford pair for this curve (`u` monic, `u | f − v²`)?
    pub fn is_valid(&self, d: &Div<F::E>) -> bool {
        let f = self.f;
        if d.u.last() != Some(&f.one()) {
            return false;
        }
        let r = pmod(f, &psub(f, &self.h, &pmul(f, &d.v, &d.v)), &d.u);
        r.is_empty() && d.v.len() < d.u.len().max(1)
    }
    /// Cantor composition, not reduced.
    pub fn compose(&self, a: &Div<F::E>, b: &Div<F::E>) -> Div<F::E> {
        let f = self.f;
        let (d1, e1, e2) = pxgcd(f, &a.u, &b.u);
        let vs = padd(f, &a.v, &b.v);
        let (d, c1, c2) = pxgcd(f, &d1, &vs);
        let s1 = pmul(f, &c1, &e1);
        let s2 = pmul(f, &c1, &e2);
        let s3 = c2;
        let d2 = pmul(f, &d, &d);
        let (u, r) = pdivrem(f, &pmul(f, &a.u, &b.u), &d2);
        debug_assert!(r.is_empty());
        let t1 = pmul(f, &pmul(f, &s1, &a.u), &b.v);
        let t2 = pmul(f, &pmul(f, &s2, &b.u), &a.v);
        let t3 = pmul(f, &s3, &padd(f, &pmul(f, &a.v, &b.v), &self.h));
        let num = padd(f, &padd(f, &t1, &t2), &t3);
        let (vq, vr) = pdivrem(f, &num, &d);
        debug_assert!(vr.is_empty());
        let u = pmonic(f, &u);
        let v = pmod(f, &vq, &u);
        Div { u, v }
    }
    pub fn reduce(&self, mut d: Div<F::E>) -> Div<F::E> {
        let f = self.f;
        while d.u.len() as isize - 1 > self.g as isize {
            let num = psub(f, &self.h, &pmul(f, &d.v, &d.v));
            let (u2, r) = pdivrem(f, &num, &d.u);
            debug_assert!(r.is_empty());
            let u2 = pmonic(f, &u2);
            let nv: Poly<F::E> = d.v.iter().map(|c| f.neg(c)).collect();
            let v2 = pmod(f, &nv, &u2);
            d = Div { u: u2, v: v2 };
        }
        d.v = pmod(f, &d.v, &d.u);
        d
    }
    pub fn add(&self, a: &Div<F::E>, b: &Div<F::E>) -> Div<F::E> {
        if self.is_identity(a) {
            return b.clone();
        }
        if self.is_identity(b) {
            return a.clone();
        }
        self.reduce(self.compose(a, b))
    }
    pub fn sub(&self, a: &Div<F::E>, b: &Div<F::E>) -> Div<F::E> {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, a: &Div<F::E>, mut k: u128) -> Div<F::E> {
        let mut acc = self.identity();
        let mut base = a.clone();
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(&acc, &base);
            }
            k >>= 1;
            if k > 0 {
                base = self.add(&base, &base);
            }
        }
        acc
    }
}
