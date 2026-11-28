//! Selected native generic field/polynomial/Mumford/Cantor definitions.
//! Exact bodies from crypto816f9c3a:jv_cover.rs; see INHERITANCE.json.

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
pub(crate) type Bezout<E> = (Poly<E>, Poly<E>, Poly<E>);

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
pub(crate) fn pxgcd<F: Fld>(f: &F, a: &[F::E], b: &[F::E]) -> Bezout<F::E> {
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

