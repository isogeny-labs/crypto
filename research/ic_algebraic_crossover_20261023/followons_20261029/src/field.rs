//! Polynomial-basis binary fields and complete rational point operations.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Field {
    pub n: u32,
    pub modulus: u32,
}
impl Field {
    pub fn mul(self, mut a: u32, mut b: u32) -> u32 {
        let mut r = 0;
        while b != 0 {
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if a >> self.n != 0 {
                a ^= self.modulus;
            }
        }
        r
    }
    pub fn sq(self, x: u32) -> u32 {
        self.mul(x, x)
    }
    pub fn pow(self, mut a: u32, mut k: u32) -> u32 {
        let mut r = 1;
        while k > 0 {
            if k & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.sq(a);
            k >>= 1;
        }
        r
    }
    pub fn inv(self, x: u32) -> u32 {
        assert_ne!(x, 0);
        self.pow(x, (1 << self.n) - 2)
    }
    pub fn frob(self, mut x: u32, k: u32) -> u32 {
        for _ in 0..k {
            x = self.sq(x);
        }
        x
    }
    pub fn quadratic(self, c: u32) -> Option<u32> {
        let n = self.n as usize;
        let mut rows: Vec<u64> = (0..n)
            .map(|i| {
                let mut row = ((c >> i & 1) as u64) << n;
                for j in 0..n {
                    row |= (((self.sq(1 << j) ^ (1 << j)) >> i & 1) as u64) << j;
                }
                row
            })
            .collect();
        let mut piv = vec![];
        let mut r = 0;
        for col in 0..n {
            if let Some(i) = (r..n).find(|&i| rows[i] >> col & 1 != 0) {
                rows.swap(r, i);
                for j in 0..n {
                    if j != r && rows[j] >> col & 1 != 0 {
                        rows[j] ^= rows[r];
                    }
                }
                piv.push(col);
                r += 1;
            }
        }
        if rows.iter().any(|r| r & ((1 << n) - 1) == 0 && r >> n != 0) {
            return None;
        }
        let mut z = 0;
        for (i, &p) in piv.iter().enumerate() {
            z |= ((rows[i] >> n & 1) as u32) << p;
        }
        assert_eq!(self.sq(z) ^ z, c);
        Some(z)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Point {
    Infinity,
    Affine(u32, u32),
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Curve {
    pub f: Field,
    pub a2: u32,
    pub a4: u32,
    pub a6: u32,
}
impl Curve {
    pub fn neg(self, p: Point) -> Point {
        match p {
            Point::Infinity => p,
            Point::Affine(x, y) => Point::Affine(x, y ^ x),
        }
    }
    pub fn on(self, p: Point) -> bool {
        match p {
            Point::Infinity => true,
            Point::Affine(x, y) => {
                self.f.sq(y) ^ self.f.mul(x, y)
                    == self.f.mul(self.f.sq(x), x)
                        ^ self.f.mul(self.a2, self.f.sq(x))
                        ^ self.f.mul(self.a4, x)
                        ^ self.a6
            }
        }
    }
    pub fn add(self, p: Point, q: Point) -> Point {
        let (Point::Affine(x, y), Point::Affine(u, v)) = (p, q) else {
            return if p == Point::Infinity { q } else { p };
        };
        if x == u && y ^ v == x {
            return Point::Infinity;
        }
        let l = if x == u {
            if x == 0 {
                return Point::Infinity;
            }
            x ^ self.f.mul(y ^ self.a4, self.f.inv(x))
        } else {
            self.f.mul(y ^ v, self.f.inv(x ^ u))
        };
        let z = self.f.sq(l) ^ l ^ x ^ u ^ self.a2;
        let t = self.f.mul(l, x ^ z) ^ z ^ y;
        Point::Affine(z, t)
    }
    pub fn mul(self, mut p: Point, mut k: u64) -> Point {
        let mut r = Point::Infinity;
        while k > 0 {
            if k & 1 != 0 {
                r = self.add(r, p);
            }
            p = self.add(p, p);
            k >>= 1;
        }
        r
    }
    pub fn lift(self, x: u32) -> Vec<Point> {
        if x == 0 {
            return vec![Point::Affine(0, self.f.frob(self.a6, self.f.n - 1))];
        }
        let inv = self.f.inv(x);
        let c = x ^ self.a2 ^ self.f.mul(self.a4, inv) ^ self.f.mul(self.a6, self.f.sq(inv));
        match self.f.quadratic(c) {
            None => vec![],
            Some(z) => {
                let y = self.f.mul(x, z);
                vec![Point::Affine(x, y), Point::Affine(x, y ^ x)]
            }
        }
    }
    pub fn points(self) -> Vec<Point> {
        assert!(self.f.n <= 9);
        let mut v = vec![Point::Infinity];
        for x in 0..1 << self.f.n {
            v.extend(self.lift(x));
        }
        v
    }
    pub fn frob(self, p: Point) -> Point {
        match p {
            Point::Infinity => p,
            Point::Affine(x, y) => Point::Affine(self.f.sq(x), self.f.sq(y)),
        }
    }
    pub fn verschiebung(self, p: Point) -> Point {
        assert_eq!((self.a2, self.a4, self.a6), (0, 0, 1));
        self.neg(self.add(p, self.frob(p)))
    }
}
