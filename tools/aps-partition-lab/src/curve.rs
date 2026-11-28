//! Ordinary-residue affine reference, independent of polynomial construction.
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Curve {
    pub(crate) p: u64,
    pub(crate) a: u64,
    pub(crate) b: u64,
}

impl Curve {
    pub fn new(p: u64, a: u64, b: u64) -> Result<Self, String> {
        if !(5..=127).contains(&p) || (2..p).any(|d| p.is_multiple_of(d)) {
            return Err("reference requires a prime in 5..=127".into());
        }
        let c = Self {
            p,
            a: a % p,
            b: b % p,
        };
        if (4 * c.a * c.a * c.a + 27 * c.b * c.b).is_multiple_of(p) {
            return Err("singular curve".into());
        }
        Ok(c)
    }

    pub fn contains(self, point: Point) -> bool {
        point.x < self.p
            && point.y < self.p
            && point.y * point.y % self.p
                == (point.x * point.x * point.x + self.a * point.x + self.b) % self.p
    }

    pub fn neg(self, point: Point) -> Point {
        Point {
            x: point.x,
            y: (self.p - point.y) % self.p,
        }
    }

    fn inv(self, x: u64) -> u64 {
        assert_ne!(x % self.p, 0);
        let mut out = 1;
        let mut base = x % self.p;
        let mut e = self.p - 2;
        while e != 0 {
            if e & 1 == 1 {
                out = out * base % self.p;
            }
            base = base * base % self.p;
            e >>= 1;
        }
        out
    }

    pub fn add(self, left: Option<Point>, right: Option<Point>) -> Option<Point> {
        let (s, t) = match (left, right) {
            (None, r) => return r,
            (l, None) => return l,
            (Some(s), Some(t)) => (s, t),
        };
        assert!(self.contains(s) && self.contains(t));
        if s.x == t.x && (s.y + t.y) % self.p == 0 {
            return None;
        }
        let slope = if s == t {
            (3 * s.x * s.x + self.a) * self.inv(2 * s.y % self.p) % self.p
        } else {
            (t.y + self.p - s.y) * self.inv((t.x + self.p - s.x) % self.p) % self.p
        };
        let x = (slope * slope + 2 * self.p - s.x - t.x) % self.p;
        let y = (slope * ((s.x + self.p - x) % self.p) + self.p - s.y) % self.p;
        let out = Point { x, y };
        assert!(self.contains(out));
        Some(out)
    }

    pub fn mul(self, mut point: Option<Point>, mut k: u64) -> Option<Point> {
        let mut out = None;
        while k != 0 {
            if k & 1 == 1 {
                out = self.add(out, point);
            }
            point = self.add(point, point);
            k >>= 1;
        }
        out
    }

    /// One representative per nonzero-ordinate abscissa; not a subgroup sampler.
    pub fn representatives(self) -> Vec<Point> {
        (0..self.p)
            .filter_map(|x| {
                (1..self.p)
                    .find(|&y| self.contains(Point { x, y }))
                    .map(|y| Point { x, y })
            })
            .collect()
    }

    /// Fix the first sign to quotient only the global negation symmetry.
    pub fn has_signed_zero(self, points: &[Point]) -> bool {
        assert!((1..=4).contains(&points.len()));
        (0..1usize << (points.len() - 1)).any(|mask| {
            points
                .iter()
                .enumerate()
                .fold(None, |sum, (i, &point)| {
                    let signed = if i > 0 && mask & (1 << (i - 1)) != 0 {
                        self.neg(point)
                    } else {
                        point
                    };
                    self.add(sum, Some(signed))
                })
                .is_none()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_order_nineteen_fixture_and_affine_group_controls() {
        let c = Curve::new(17, 2, 2).unwrap();
        let g = Point { x: 5, y: 1 };
        assert_eq!(c.mul(Some(g), 2), Some(Point { x: 6, y: 3 }));
        assert_eq!(c.mul(Some(g), 19), None);
        for i in 0..19 {
            for j in 0..19 {
                assert_eq!(
                    c.add(c.mul(Some(g), i), c.mul(Some(g), j)),
                    c.mul(Some(g), (i + j) % 19)
                );
            }
        }
        assert!(Curve::new(15, 1, 1).is_err());
        assert!(Curve::new(17, 0, 0).is_err());
    }

    #[test]
    fn signed_oracle_retains_independent_signs_and_two_torsion_is_not_in_coordinate_pool() {
        let c = Curve::new(17, 2, 2).unwrap();
        let g = Point { x: 5, y: 1 };
        let p2 = c.mul(Some(g), 2).unwrap();
        let p3 = c.mul(Some(g), 3).unwrap();
        assert!(c.has_signed_zero(&[g, p2, p3]));
        assert!(!c.has_signed_zero(&[g, p2, c.mul(Some(g), 5).unwrap()]));
        let torsion = Curve::new(17, 1, 0).unwrap();
        assert!(torsion.contains(Point { x: 0, y: 0 }));
        assert!(!torsion.representatives().iter().any(|p| p.y == 0));
    }
}
