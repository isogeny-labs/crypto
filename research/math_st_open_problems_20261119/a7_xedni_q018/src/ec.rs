//! Exact rational numbers and elliptic-curve arithmetic over Q.
//! Everything here is exact (num-bigint); no floating point.

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// A reduced rational number n/d with d > 0.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    pub n: BigInt,
    pub d: BigInt,
}

impl Q {
    pub fn new(n: BigInt, d: BigInt) -> Q {
        assert!(!d.is_zero(), "zero denominator");
        let g = n.gcd(&d);
        let (mut n, mut d) = if g.is_one() { (n, d) } else { (n / &g, d / &g) };
        if d.is_negative() {
            n = -n;
            d = -d;
        }
        Q { n, d }
    }
    pub fn int(v: BigInt) -> Q {
        Q { n: v, d: BigInt::one() }
    }
    pub fn from_i64(v: i64) -> Q {
        Q::int(BigInt::from(v))
    }
    pub fn is_zero(&self) -> bool {
        self.n.is_zero()
    }
    pub fn add(&self, o: &Q) -> Q {
        if self.d == o.d {
            return Q::new(&self.n + &o.n, self.d.clone());
        }
        Q::new(&self.n * &o.d + &o.n * &self.d, &self.d * &o.d)
    }
    pub fn sub(&self, o: &Q) -> Q {
        if self.d == o.d {
            return Q::new(&self.n - &o.n, self.d.clone());
        }
        Q::new(&self.n * &o.d - &o.n * &self.d, &self.d * &o.d)
    }
    pub fn mul(&self, o: &Q) -> Q {
        Q::new(&self.n * &o.n, &self.d * &o.d)
    }
    pub fn mul_int(&self, k: &BigInt) -> Q {
        Q::new(&self.n * k, self.d.clone())
    }
    pub fn div(&self, o: &Q) -> Q {
        assert!(!o.n.is_zero(), "division by zero");
        Q::new(&self.n * &o.d, &self.d * &o.n)
    }
    pub fn neg(&self) -> Q {
        Q { n: -&self.n, d: self.d.clone() }
    }
    pub fn to_string(&self) -> String {
        if self.d.is_one() {
            self.n.to_string()
        } else {
            format!("{}/{}", self.n, self.d)
        }
    }
}

pub type Pt = Option<(Q, Q)>; // None = point at infinity O

/// Short Weierstrass y^2 = x^3 + A x + B with integer A, B.
#[derive(Clone, Debug)]
pub struct Short {
    pub a: BigInt,
    pub b: BigInt,
}

impl Short {
    pub fn disc4(&self) -> BigInt {
        // D = 4A^3 + 27B^2 (nonzero iff the curve is smooth)
        BigInt::from(4) * &self.a * &self.a * &self.a + BigInt::from(27) * &self.b * &self.b
    }
    pub fn contains(&self, p: &Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let lhs = y.mul(y);
                let rhs = x.mul(x).mul(x).add(&x.mul_int(&self.a)).add(&Q::int(self.b.clone()));
                lhs == rhs
            }
        }
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| (x.clone(), y.neg()))
    }
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q.clone(),
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p.clone(),
            Some(v) => v,
        };
        let lambda = if x1 == x2 {
            if y1.add(y2).is_zero() {
                return None;
            }
            // tangent
            let three = BigInt::from(3);
            let num = x1.mul(x1).mul_int(&three).add(&Q::int(self.a.clone()));
            num.div(&y1.mul_int(&BigInt::from(2)))
        } else {
            y2.sub(y1).div(&x2.sub(x1))
        };
        let x3 = lambda.mul(&lambda).sub(x1).sub(x2);
        let y3 = lambda.mul(&x1.sub(&x3)).sub(y1);
        Some((x3, y3))
    }
    pub fn mul(&self, k: i64, p: &Pt) -> Pt {
        let mut result: Pt = None;
        let mut base = if k < 0 { self.neg(p) } else { p.clone() };
        let mut m = k.unsigned_abs();
        while m > 0 {
            if m & 1 == 1 {
                result = self.add(&result, &base);
            }
            m >>= 1;
            if m > 0 {
                base = self.add(&base, &base);
            }
        }
        result
    }
}

/// General integral Weierstrass y^2 + a1 xy + a3 y = x^3 + a2 x^2 + a4 x + a6.
#[derive(Clone, Debug)]
pub struct General {
    pub a1: BigInt,
    pub a2: BigInt,
    pub a3: BigInt,
    pub a4: BigInt,
    pub a6: BigInt,
}

pub struct Invariants {
    pub b2: BigInt,
    pub b4: BigInt,
    pub b6: BigInt,
    pub b8: BigInt,
    pub c4: BigInt,
    pub c6: BigInt,
    pub disc: BigInt,
}

impl General {
    pub fn invariants(&self) -> Invariants {
        let (a1, a2, a3, a4, a6) = (&self.a1, &self.a2, &self.a3, &self.a4, &self.a6);
        let b2 = a1 * a1 + BigInt::from(4) * a2;
        let b4 = BigInt::from(2) * a4 + a1 * a3;
        let b6 = a3 * a3 + BigInt::from(4) * a6;
        let b8 = a1 * a1 * a6 + BigInt::from(4) * a2 * a6 - a1 * a3 * a4 + a2 * a3 * a3 - a4 * a4;
        let c4 = &b2 * &b2 - BigInt::from(24) * &b4;
        let c6 = -(&b2 * &b2 * &b2) + BigInt::from(36) * &b2 * &b4 - BigInt::from(216) * &b6;
        let disc = -(&b2 * &b2 * &b8) - BigInt::from(8) * &b4 * &b4 * &b4 - BigInt::from(27) * &b6 * &b6
            + BigInt::from(9) * &b2 * &b4 * &b6;
        Invariants { b2, b4, b6, b8, c4, c6, disc }
    }
    pub fn contains(&self, p: &Pt) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let lhs = y.mul(y).add(&x.mul(y).mul_int(&self.a1)).add(&y.mul_int(&self.a3));
                let rhs = x
                    .mul(x)
                    .mul(x)
                    .add(&x.mul(x).mul_int(&self.a2))
                    .add(&x.mul_int(&self.a4))
                    .add(&Q::int(self.a6.clone()));
                lhs == rhs
            }
        }
    }
    pub fn neg(&self, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| (x.clone(), y.neg().sub(&x.mul_int(&self.a1)).sub(&Q::int(self.a3.clone()))))
    }
    /// Silverman, AEC III.2.3.
    pub fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q.clone(),
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p.clone(),
            Some(v) => v,
        };
        let a1 = Q::int(self.a1.clone());
        let a2 = Q::int(self.a2.clone());
        let a3 = Q::int(self.a3.clone());
        let a4 = Q::int(self.a4.clone());
        let a6 = Q::int(self.a6.clone());
        let (lambda, nu);
        if x1 == x2 {
            // either inverse or doubling
            let s = y1.add(y2).add(&a1.mul(x2)).add(&a3);
            if s.is_zero() {
                return None;
            }
            let den = y1.mul_int(&BigInt::from(2)).add(&a1.mul(x1)).add(&a3);
            let x1sq = x1.mul(x1);
            lambda = x1sq
                .mul_int(&BigInt::from(3))
                .add(&a2.mul(x1).mul_int(&BigInt::from(2)))
                .add(&a4)
                .sub(&a1.mul(y1))
                .div(&den);
            nu = x1sq
                .mul(x1)
                .neg()
                .add(&a4.mul(x1))
                .add(&a6.mul_int(&BigInt::from(2)))
                .sub(&a3.mul(y1))
                .div(&den);
        } else {
            let dx = x2.sub(x1);
            lambda = y2.sub(y1).div(&dx);
            nu = y1.mul(x2).sub(&y2.mul(x1)).div(&dx);
        }
        let x3 = lambda.mul(&lambda).add(&a1.mul(&lambda)).sub(&a2).sub(x1).sub(x2);
        let y3 = lambda.add(&a1).mul(&x3).neg().sub(&nu).sub(&a3);
        Some((x3, y3))
    }
    pub fn mul(&self, k: i64, p: &Pt) -> Pt {
        let mut result: Pt = None;
        let mut base = if k < 0 { self.neg(p) } else { p.clone() };
        let mut m = k.unsigned_abs();
        while m > 0 {
            if m & 1 == 1 {
                result = self.add(&result, &base);
            }
            m >>= 1;
            if m > 0 {
                base = self.add(&base, &base);
            }
        }
        result
    }
    /// Short model y^2 = x^3 - 27 c4 x - 54 c6 and the map (x,y) -> (36x+3b2, 108(2y+a1x+a3)).
    pub fn to_short(&self) -> (Short, Invariants) {
        let inv = self.invariants();
        let s = Short { a: BigInt::from(-27) * &inv.c4, b: BigInt::from(-54) * &inv.c6 };
        (s, inv)
    }
    pub fn map_to_short(&self, inv: &Invariants, p: &Pt) -> Pt {
        p.as_ref().map(|(x, y)| {
            let xs = x.mul_int(&BigInt::from(36)).add(&Q::int(BigInt::from(3) * &inv.b2));
            let ys = y
                .mul_int(&BigInt::from(2))
                .add(&x.mul_int(&self.a1))
                .add(&Q::int(self.a3.clone()))
                .mul_int(&BigInt::from(108));
            (xs, ys)
        })
    }
}

pub fn pt_string(p: &Pt) -> String {
    match p {
        None => "O".to_string(),
        Some((x, y)) => format!("({}:{})", x.to_string(), y.to_string()),
    }
}

pub fn bigint_abs_bits(v: &BigInt) -> u64 {
    v.abs().bits()
}
