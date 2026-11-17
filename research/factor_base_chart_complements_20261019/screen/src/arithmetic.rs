//! Independent small binary-field arithmetic audit; no scalar-log search.
use crate::Result;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}
pub const O: Point = Point {
    x: 0,
    y: 0,
    infinity: true,
};
pub struct Curve {
    pub n: u32,
    pub modulus: u64,
    pub a: u64,
}
impl Curve {
    pub fn mul(&self, mut x: u64, mut y: u64) -> u64 {
        let mut z = 0;
        let high = 1u64 << self.n;
        while y != 0 {
            if y & 1 != 0 {
                z ^= x;
            }
            y >>= 1;
            x <<= 1;
            if x & high != 0 {
                x ^= self.modulus;
            }
        }
        z
    }
    pub fn sq(&self, x: u64) -> u64 {
        self.mul(x, x)
    }
    pub fn inv(&self, x: u64) -> Result<u64> {
        if x == 0 {
            return Err("inverse of zero".into());
        }
        let (mut y, mut base, mut exponent) = (1, x, (1u64 << self.n) - 2);
        while exponent != 0 {
            if exponent & 1 != 0 {
                y = self.mul(y, base);
            }
            base = self.sq(base);
            exponent >>= 1;
        }
        if self.mul(x, y) != 1 {
            return Err("invalid field inverse".into());
        }
        Ok(y)
    }
    pub fn on_curve(&self, p: Point) -> bool {
        p.infinity
            || self.sq(p.y) ^ self.mul(p.x, p.y)
                == self.mul(self.sq(p.x), p.x) ^ self.mul(self.a, self.sq(p.x)) ^ 1
    }
    pub fn add(&self, p: Point, q: Point) -> Result<Point> {
        if p.infinity {
            return Ok(q);
        }
        if q.infinity {
            return Ok(p);
        }
        let (x, y) = if p.x == q.x {
            if p.y != q.y || p.x == 0 {
                return Ok(O);
            }
            let l = p.x ^ self.mul(p.y, self.inv(p.x)?);
            let x = self.sq(l) ^ l ^ self.a;
            (x, self.sq(p.x) ^ self.mul(l ^ 1, x))
        } else {
            let l = self.mul(p.y ^ q.y, self.inv(p.x ^ q.x)?);
            let x = self.sq(l) ^ l ^ p.x ^ q.x ^ self.a;
            (x, self.mul(l, p.x ^ x) ^ x ^ p.y)
        };
        let p = Point {
            x,
            y,
            infinity: false,
        };
        if !self.on_curve(p) {
            return Err("independent addition is off curve".into());
        }
        Ok(p)
    }
    pub fn scale(&self, mut p: Point, mut k: u64) -> Result<Point> {
        let mut q = O;
        while k != 0 {
            if k & 1 != 0 {
                q = self.add(q, p)?;
            }
            p = self.add(p, p)?;
            k >>= 1;
        }
        Ok(q)
    }
}
pub fn hex(s: &str) -> Result<u64> {
    Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16)?)
}
pub fn prime(r: u64) -> bool {
    r >= 2
        && (2..)
            .take_while(|d| d <= &(r / d))
            .all(|d| !r.is_multiple_of(d))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn c() -> Curve {
        Curve {
            n: 7,
            modulus: 0x83,
            a: 1,
        }
    }
    fn g() -> Point {
        Point {
            x: 0x65,
            y: 0x59,
            infinity: false,
        }
    }
    #[test]
    fn every_nonzero_field_element_has_a_checked_inverse() {
        let c = c();
        for x in 1..128 {
            assert_eq!(c.mul(x, c.inv(x).unwrap()), 1);
        }
    }
    #[test]
    fn recorded_generator_has_order71_and_scalar_addition_agrees_with_repeated_addition() {
        let c = c();
        assert!(c.on_curve(g()));
        let mut p = O;
        for k in 0..=71 {
            assert_eq!(p, c.scale(g(), k).unwrap());
            p = c.add(p, g()).unwrap();
        }
        assert_eq!(c.scale(g(), 71).unwrap(), O);
    }
    #[test]
    fn inverse_points_and_associativity_cover_both_addition_branches() {
        let c = c();
        assert_eq!(
            c.add(
                g(),
                Point {
                    x: g().x,
                    y: g().x ^ g().y,
                    infinity: false
                }
            )
            .unwrap(),
            O
        );
        for i in 1..8 {
            for j in 1..8 {
                for k in 1..8 {
                    let (a, b, d) = (
                        c.scale(g(), i).unwrap(),
                        c.scale(g(), j).unwrap(),
                        c.scale(g(), k).unwrap(),
                    );
                    assert_eq!(
                        c.add(c.add(a, b).unwrap(), d).unwrap(),
                        c.add(a, c.add(b, d).unwrap()).unwrap()
                    );
                }
            }
        }
    }
}
