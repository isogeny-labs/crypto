//! Curve helpers: counted group law through the library's `PrimeCurve`, an
//! uncounted fast affine law for the classifier enumeration, and `F_{p^2}`
//! arithmetic for the algebraic count (the way gen.py's `decomposable` does it).
use crate::util::{addm, invm, jacobi, mulm, sqrt_counted, subm};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};

pub type Aff = (u64, u64);

/// Counted group over `y^2 = x^3 + ax + b` (library `PrimeCurve`).
pub struct Ec {
    pub c: PrimeCurve,
}

impl Ec {
    pub fn new(p: u64, a: u64, b: u64) -> Self {
        Ec {
            c: PrimeCurve { p, a, b },
        }
    }
    pub fn pt(&self, x: u64, y: u64) -> PrimePoint {
        PrimePoint::affine(x, y)
    }
    pub fn add(&self, ops: &mut GroupOps, a: PrimePoint, b: PrimePoint) -> PrimePoint {
        CountedGroup::add(&self.c, ops, a, b)
    }
    pub fn neg(&self, a: PrimePoint) -> PrimePoint {
        CountedGroup::neg(&self.c, a)
    }
    pub fn mul(&self, ops: &mut GroupOps, a: PrimePoint, k: u64) -> PrimePoint {
        CountedGroup::mul(&self.c, ops, a, k)
    }
    /// `[a]P + [b]Q` (counted).
    pub fn combo(&self, ops: &mut GroupOps, pp: PrimePoint, q: PrimePoint, a: u64, b: u64) -> PrimePoint {
        let x = self.mul(ops, pp, a);
        let y = self.mul(ops, q, b);
        self.add(ops, x, y)
    }
}

/// `x^3 + ax + b`.
pub fn rhs(p: u64, a: u64, b: u64, x: u64) -> u64 {
    addm(addm(mulm(mulm(x, x, p), x, p), mulm(a, x, p), p), b, p)
}
/// Canonical lift of `x`: the point with `y <= (p-1)/2`, if `x` is on the curve.
pub fn lift_small(p: u64, a: u64, b: u64, x: u64) -> Option<Aff> {
    let r = rhs(p, a, b, x);
    if r != 0 && jacobi(r, p) != 1 {
        return None;
    }
    let y = sqrt_counted(p, r, &mut 0)?;
    Some((x, y.min((p - y) % p)))
}

/// Uncounted affine addition of two finite points (`None` is the identity).
pub fn add_u(p: u64, a: u64, pp: Aff, qq: Aff) -> Option<Aff> {
    let (x1, y1) = pp;
    let (x2, y2) = qq;
    let lam = if x1 == x2 {
        if addm(y1, y2, p) == 0 {
            return None;
        }
        let num = addm(mulm(3, mulm(x1, x1, p), p), a, p);
        mulm(num, invm(mulm(2, y1, p), p), p)
    } else {
        mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
    };
    let x3 = subm(subm(mulm(lam, lam, p), x1, p), x2, p);
    let y3 = subm(mulm(lam, subm(x1, x3, p), p), y1, p);
    Some((x3, y3))
}

/// Affine addition where either operand may be the identity.
pub fn add_opt(p: u64, a: u64, pp: Option<Aff>, qq: Option<Aff>) -> Option<Aff> {
    match (pp, qq) {
        (None, q) => q,
        (pt, None) => pt,
        (Some(x), Some(y)) => add_u(p, a, x, y),
    }
}
#[cfg(test)]
pub fn neg_u(p: u64, pt: Aff) -> Aff {
    (pt.0, (p - pt.1) % p)
}
// ---- F_{p^2} = F_p[s]/(s^2 - nr) -------------------------------------------

pub type F2e = (u64, u64);

pub struct F2 {
    pub p: u64,
    pub nr: u64,
}
impl F2 {
    pub fn new(p: u64) -> Self {
        let nr = (2..p).find(|&c| jacobi(c, p) == -1).expect("non-residue");
        F2 { p, nr }
    }
    pub fn mul(&self, x: F2e, y: F2e) -> F2e {
        let p = self.p;
        (
            addm(mulm(x.0, y.0, p), mulm(self.nr, mulm(x.1, y.1, p), p), p),
            addm(mulm(x.0, y.1, p), mulm(x.1, y.0, p), p),
        )
    }
    pub fn add(&self, x: F2e, y: F2e) -> F2e {
        (addm(x.0, y.0, self.p), addm(x.1, y.1, self.p))
    }
    pub fn sub(&self, x: F2e, y: F2e) -> F2e {
        (subm(x.0, y.0, self.p), subm(x.1, y.1, self.p))
    }
    pub fn neg(&self, x: F2e) -> F2e {
        ((self.p - x.0) % self.p, (self.p - x.1) % self.p)
    }
    pub fn inv(&self, x: F2e) -> F2e {
        let p = self.p;
        let n = subm(mulm(x.0, x.0, p), mulm(self.nr, mulm(x.1, x.1, p), p), p);
        let ni = invm(n, p);
        (mulm(x.0, ni, p), mulm((p - x.1) % p, ni, p))
    }
}
pub type F2pt = (F2e, F2e);

pub fn add_f2(f: &F2, a: u64, pp: Option<F2pt>, qq: Option<F2pt>) -> Option<F2pt> {
    let (p1, q1) = match (pp, qq) {
        (None, q) => return q,
        (pt, None) => return pt,
        (Some(x), Some(y)) => (x, y),
    };
    let lam = if p1.0 == q1.0 {
        if f.add(p1.1, q1.1) == (0, 0) {
            return None;
        }
        let num = f.add(f.mul((3, 0), f.mul(p1.0, p1.0)), (a % f.p, 0));
        f.mul(num, f.inv(f.mul((2, 0), p1.1)))
    } else {
        f.mul(f.sub(q1.1, p1.1), f.inv(f.sub(q1.0, p1.0)))
    };
    let x3 = f.sub(f.sub(f.mul(lam, lam), p1.0), q1.0);
    let y3 = f.sub(f.mul(lam, f.sub(p1.0, x3)), p1.1);
    Some((x3, y3))
}
/// Lift `x in F_p` to `E(F_{p^2})`: `(x, y)` with `y` real when `x^3+ax+b` is a
/// square, else `y = c*s` with `nr*c^2 = x^3+ax+b`.
pub fn lift_f2(f: &F2, a: u64, b: u64, x: u64) -> F2pt {
    let p = f.p;
    let r = rhs(p, a, b, x);
    if r == 0 || jacobi(r, p) == 1 {
        let y = sqrt_counted(p, r, &mut 0).expect("sqrt");
        ((x, 0), (y, 0))
    } else {
        let c = sqrt_counted(p, mulm(r, invm(f.nr, p), p), &mut 0).expect("sqrt twist");
        ((x, 0), (0, c))
    }
}
