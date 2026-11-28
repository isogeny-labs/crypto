//! Independent, deliberately small affine verifier. It imports no production
//! curve arithmetic and is never used to discover a relation or scalar.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl Point {
    pub const IDENTITY: Self = Self {
        x: 0,
        y: 0,
        infinity: true,
    };
    pub fn affine(x: u64, y: u64) -> Self {
        Self {
            x,
            y,
            infinity: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

impl Curve {
    fn mul(self, a: u64, b: u64) -> u64 {
        ((u128::from(a) * u128::from(b)) % u128::from(self.p)) as u64
    }
    fn sub(self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }
    fn pow(self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e != 0 {
            if e & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    pub fn valid_parameters(self) -> bool {
        self.p > 3
            && self.p < (1u64 << 32)
            && self.a < self.p
            && self.b < self.p
            && (self.mul(4, self.mul(self.mul(self.a, self.a), self.a))
                + self.mul(27 % self.p, self.mul(self.b, self.b)))
                % self.p
                != 0
    }
    pub fn on_curve(self, q: Point) -> bool {
        if q.infinity {
            return q == Point::IDENTITY;
        }
        q.x < self.p
            && q.y < self.p
            && self.mul(q.y, q.y)
                == (self.mul(self.mul(q.x, q.x), q.x) + self.mul(self.a, q.x) + self.b) % self.p
    }
    pub fn add(self, q: Point, r: Point) -> Result<Point, String> {
        if !self.on_curve(q) || !self.on_curve(r) {
            return Err("noncanonical or off-curve point".into());
        }
        if q.infinity {
            return Ok(r);
        }
        if r.infinity {
            return Ok(q);
        }
        if q.x == r.x && (q.y + r.y) % self.p == 0 {
            return Ok(Point::IDENTITY);
        }
        let (num, den) = if q == r {
            (
                (self.mul(3, self.mul(q.x, q.x)) + self.a) % self.p,
                (2 * q.y) % self.p,
            )
        } else {
            (self.sub(r.y, q.y), self.sub(r.x, q.x))
        };
        if den == 0 {
            return Err("invalid same-x addition".into());
        }
        let inverse = self.pow(den, self.p - 2);
        if self.mul(den, inverse) != 1 {
            return Err("noninvertible denominator".into());
        }
        let slope = self.mul(num, inverse);
        let x = self.sub(self.sub(self.mul(slope, slope), q.x), r.x);
        let y = self.sub(self.mul(slope, self.sub(q.x, x)), q.y);
        let out = Point::affine(x, y);
        if !self.on_curve(out) {
            return Err("affine replay left the curve".into());
        }
        Ok(out)
    }
    pub fn scalar(self, mut q: Point, mut k: u64) -> Result<Point, String> {
        if !self.on_curve(q) {
            return Err("off-curve scalar input".into());
        }
        let mut out = Point::IDENTITY;
        while k != 0 {
            if k & 1 != 0 {
                out = self.add(out, q)?;
            }
            q = self.add(q, q)?;
            k >>= 1;
        }
        Ok(out)
    }
    pub fn sum(self, points: impl IntoIterator<Item = Point>) -> Result<Point, String> {
        points
            .into_iter()
            .try_fold(Point::IDENTITY, |a, b| self.add(a, b))
    }
}

pub fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}
