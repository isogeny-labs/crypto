//! Koblitz curves E_a: y^2 + xy = x^3 + a x^2 + 1 over F_{2^n}.

use crate::field::Fq;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct P {
    pub x: u32,
    pub y: u32,
}
pub const INF: P = P { x: u32::MAX, y: 0 };
impl P {
    #[inline]
    pub fn is_inf(self) -> bool {
        self.x == u32::MAX
    }
    #[inline]
    pub fn key(self) -> u64 {
        ((self.x as u64) << 32) | self.y as u64
    }
}

pub struct Curve {
    pub a: u32,
    pub b: u32,
}

impl Curve {
    pub fn koblitz(a: u32) -> Curve {
        assert!(a <= 1);
        Curve { a, b: 1 }
    }
    pub fn on_curve(&self, f: &Fq, p: P) -> bool {
        if p.is_inf() {
            return true;
        }
        let lhs = f.sq(p.y) ^ f.mul(p.x, p.y);
        let x2 = f.sq(p.x);
        let rhs = f.mul(x2, p.x) ^ f.mul(self.a, x2) ^ self.b;
        lhs == rhs
    }
    #[inline]
    pub fn neg(&self, p: P) -> P {
        if p.is_inf() {
            p
        } else {
            P {
                x: p.x,
                y: p.y ^ p.x,
            }
        }
    }
    pub fn add(&self, f: &Fq, p: P, q: P) -> P {
        if p.is_inf() {
            return q;
        }
        if q.is_inf() {
            return p;
        }
        if p.x == q.x {
            if p.y == (q.y ^ q.x) {
                return INF;
            }
            // doubling, x != 0
            let lam = p.x ^ f.mul(p.y, f.inv(p.x));
            let x3 = f.sq(lam) ^ lam ^ self.a;
            let y3 = f.sq(p.x) ^ f.mul(lam ^ 1, x3);
            return P { x: x3, y: y3 };
        }
        let lam = f.mul(p.y ^ q.y, f.inv(p.x ^ q.x));
        let x3 = f.sq(lam) ^ lam ^ p.x ^ q.x ^ self.a;
        let y3 = f.mul(lam, p.x ^ x3) ^ x3 ^ p.y;
        P { x: x3, y: y3 }
    }
    pub fn sigma(&self, f: &Fq, p: P) -> P {
        if p.is_inf() {
            p
        } else {
            P {
                x: f.sq(p.x),
                y: f.sq(p.y),
            }
        }
    }
    pub fn scalar(&self, f: &Fq, p: P, mut k: u64) -> P {
        let (mut acc, mut base) = (INF, p);
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(f, acc, base);
            }
            base = self.add(f, base, base);
            k >>= 1;
        }
        acc
    }
    /// The two points over x != 0, if x lifts to E(F_q).
    pub fn lift(&self, f: &Fq, x: u32) -> Option<(P, P)> {
        assert!(x != 0);
        let w = x ^ self.a ^ f.mul(self.b, f.inv(f.sq(x)));
        if f.tr(w) != 0 {
            return None;
        }
        let y = f.mul(x, f.ht(w));
        Some((P { x, y }, P { x, y: y ^ x }))
    }
    /// All affine points, x = 0 point first.
    pub fn all_points(&self, f: &Fq) -> Vec<P> {
        let mut v = vec![P { x: 0, y: 1 }]; // b = 1: y^2 = 1
        for x in 1..f.size() as u32 {
            if let Some((p, q)) = self.lift(f, x) {
                v.push(p);
                v.push(q);
            }
        }
        v
    }
}

/// #E_a(F_{2^n}) by the Koblitz recursion V_{k+1} = mu V_k - 2 V_{k-1}.
pub fn koblitz_order(a: u32, n: u32) -> u64 {
    let mu: i64 = if a == 1 { 1 } else { -1 };
    let (mut v0, mut v1) = (2i64, mu);
    if n == 0 {
        return 0;
    }
    for _ in 1..n {
        let v2 = mu * v1 - 2 * v0;
        v0 = v1;
        v1 = v2;
    }
    ((1i64 << n) + 1 - v1) as u64
}

/// Cofactor h of E_a(F_2): 2 for a = 1, 4 for a = 0.
pub fn cofactor(a: u32) -> u64 {
    if a == 1 {
        2
    } else {
        4
    }
}
