use crate::divisor_a::H;
use crate::field_d::inv;
use crate::poly_d::{addp, bezout, divrem, pow, scale, subp, times, trim, P};

pub fn exact(a: &[u8], b: &[u8]) -> P {
    let (q, r) = divrem(a, b);
    assert!(r.is_empty(), "nonexact division: {a:?}/{b:?}");
    q
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rat {
    pub n: P,
    pub d: P,
}
impl Rat {
    pub fn new(n: P, d: P) -> Self {
        let (n, d) = (trim(n), trim(d));
        assert!(!d.is_empty());
        if n.is_empty() {
            return Self { n, d: vec![1] };
        }
        let gcd = bezout(&n, &d).0;
        let (n, d) = (exact(&n, &gcd), exact(&d, &gcd));
        let unit = inv(*d.last().unwrap());
        Self {
            n: scale(&n, unit),
            d: scale(&d, unit),
        }
    }
    pub fn polynomial(a: &[u8]) -> Self {
        Self::new(a.to_vec(), vec![1])
    }
    pub fn constant(a: u8) -> Self {
        Self::polynomial(&[a])
    }
    pub fn add(&self, b: &Self) -> Self {
        Self::new(
            addp(&times(&self.n, &b.d), &times(&b.n, &self.d)),
            times(&self.d, &b.d),
        )
    }
    pub fn sub(&self, b: &Self) -> Self {
        Self::new(
            subp(&times(&self.n, &b.d), &times(&b.n, &self.d)),
            times(&self.d, &b.d),
        )
    }
    pub fn mul(&self, b: &Self) -> Self {
        Self::new(times(&self.n, &b.n), times(&self.d, &b.d))
    }
    pub fn div(&self, b: &Self) -> Self {
        assert!(!b.n.is_empty());
        Self::new(times(&self.n, &b.d), times(&self.d, &b.n))
    }
    pub fn scale(&self, c: u8) -> Self {
        Self::new(scale(&self.n, c), self.d.clone())
    }
    pub fn pow(&self, n: usize) -> Self {
        Self::new(pow(&self.n, n), pow(&self.d, n))
    }
}

// (x, Y*r) is a point in E(F7(W,Y)), with Y^2=H(W).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointFn {
    pub x: Rat,
    pub r: Rat,
}
impl PointFn {
    pub fn valid(&self) -> bool {
        Rat::polynomial(&H).mul(&self.r.pow(2))
            == self.x.pow(3).add(&self.x.scale(2)).add(&Rat::constant(6))
    }
    pub fn doubled(&self) -> Self {
        assert!(self.valid());
        let h = Rat::polynomial(&H);
        let l = self
            .x
            .pow(2)
            .scale(3)
            .add(&Rat::constant(2))
            .div(&h.mul(&self.r).scale(2));
        let x = h.mul(&l.pow(2)).sub(&self.x.scale(2));
        let r = l.mul(&self.x.sub(&x)).sub(&self.r);
        let out = Self { x, r };
        assert!(out.valid());
        out
    }
    pub fn added(&self, b: &Self) -> Self {
        assert!(self.valid() && b.valid());
        if self.x == b.x {
            assert_eq!(self.r, b.r, "the selected generic sum cannot be O");
            return self.doubled();
        }
        let l = b.r.sub(&self.r).div(&b.x.sub(&self.x));
        let x = Rat::polynomial(&H)
            .mul(&l.pow(2))
            .sub(&self.x)
            .sub(&b.x);
        let r = l.mul(&self.x.sub(&x)).sub(&self.r);
        let out = Self { x, r };
        assert!(out.valid());
        out
    }
    pub fn frobenius(&self) -> Self {
        let out = Self {
            x: self.x.pow(7),
            r: Rat::polynomial(&H).pow(3).mul(&self.r.pow(7)),
        };
        assert!(out.valid());
        out
    }
}
