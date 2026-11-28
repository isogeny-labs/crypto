//! Separate residue arithmetic for the tiny public control curves.
pub type Point = Option<(u64, u64)>;

#[derive(Clone, Copy)]
pub struct Curve {
    pub modulus: u64,
    pub a: u64,
    pub b: u64,
}

pub fn power(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 != 0 {
            result = result * base % modulus;
        }
        base = base * base % modulus;
        exponent >>= 1;
    }
    result
}

impl Curve {
    pub fn contains(self, (x, y): (u64, u64)) -> bool {
        y * y % self.modulus == (x * x * x + self.a * x + self.b) % self.modulus
    }

    pub fn negate(self, (x, y): (u64, u64)) -> (u64, u64) {
        (x, (self.modulus - y) % self.modulus)
    }

    pub fn add(self, lhs: Point, rhs: Point) -> Point {
        let (Some((x, y)), Some((u, v))) = (lhs, rhs) else {
            return lhs.or(rhs);
        };
        let p = self.modulus;
        if x == u && (y + v) % p == 0 {
            return None;
        }
        let (n, d) = if x == u {
            ((3 * x * x + self.a) % p, 2 * y % p)
        } else {
            ((p + v - y) % p, (p + u - x) % p)
        };
        let slope = n * power(d, p - 2, p) % p;
        let xx = (2 * p + slope * slope - x - u) % p;
        let yy = (p * p + slope * x - slope * xx - y) % p;
        Some((xx, yy))
    }

    pub fn mul(self, mut point: Point, mut scalar: u64) -> Point {
        let mut result = None;
        while scalar > 0 {
            if scalar & 1 != 0 {
                result = self.add(result, point);
            }
            point = self.add(point, point);
            scalar >>= 1;
        }
        result
    }
}
