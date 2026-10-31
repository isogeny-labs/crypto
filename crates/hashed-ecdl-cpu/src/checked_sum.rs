//! Checked projective tuple sums for the CPU oracle boundary.
//! Full certificate replay remains in the independent shared reduction.
use super::{mul, reciprocal, sub};
use crate::paper::{Curve, Error, Point};

#[derive(Clone, Copy)]
struct Field {
    p: u64,
    reciprocal: u64,
}
impl Field {
    fn new(p: u64) -> Self {
        Self {
            p,
            reciprocal: reciprocal(p),
        }
    }
    fn mul(self, a: u64, b: u64) -> u64 {
        mul(a, b, self.p, self.reciprocal)
    }
}
fn addmod(a: u64, b: u64, p: u64) -> u64 {
    let sum = a + b;
    if sum >= p {
        sum - p
    } else {
        sum
    }
}

pub(super) fn matches(curve: &Curve, points: &[Point], target: Point) -> Result<bool, Error> {
    if !curve.contains(target) {
        return Err(Error::Invalid("point"));
    }
    let field = Field::new(curve.modulus());
    let mut out = Jacobian::from_affine(None);
    for &point in points {
        if !curve.contains(point) {
            return Err(Error::Invalid("point"));
        }
        out = out.add_affine(point, curve.coefficients().0, field);
    }
    Ok(out.matches_affine(target, field))
}

// Jacobian coordinates x=X/Z^2, y=Y/Z^3; Z=0 is the identity.
// EFD madd-2004-hmv and standard Jacobian doubling, with explicit equal,
// inverse and identity cases. Variable time on public research inputs.
#[derive(Clone, Copy)]
struct Jacobian {
    x: u64,
    y: u64,
    z: u64,
}
impl Jacobian {
    fn from_affine(point: Point) -> Self {
        point.map_or(Self { x: 0, y: 1, z: 0 }, |(x, y)| Self { x, y, z: 1 })
    }
    fn matches_affine(self, point: Point, modulus: Field) -> bool {
        // x=X/Z^2 and y=Y/Z^3: compare by cross multiplication, without
        // inversion. Handle infinity separately; Z=0 is never a finite point.
        match point {
            None => self.z == 0,
            Some((x, y)) if self.z != 0 => {
                let z2 = modulus.mul(self.z, self.z);
                self.x == modulus.mul(x, z2) && self.y == modulus.mul(y, modulus.mul(z2, self.z))
            }
            Some(_) => false,
        }
    }
    fn double(self, a: u64, modulus: Field) -> Self {
        if self.z == 0 || self.y == 0 {
            return Self::from_affine(None);
        }
        let p = modulus.p;
        let mul = |x, y| modulus.mul(x, y);
        let twice = |x| addmod(x, x, p);
        let xx = mul(self.x, self.x);
        let yy = mul(self.y, self.y);
        let yyyy = mul(yy, yy);
        let zz = mul(self.z, self.z);
        let s = twice(twice(mul(self.x, yy)));
        let m = addmod(addmod(twice(xx), xx, p), mul(a, mul(zz, zz)), p);
        let x = sub(mul(m, m), twice(s), p);
        let y = sub(mul(m, sub(s, x, p)), twice(twice(twice(yyyy))), p);
        Self {
            x,
            y,
            z: twice(mul(self.y, self.z)),
        }
    }
    fn add_affine(self, point: Point, a: u64, modulus: Field) -> Self {
        let Some((x, y)) = point else {
            return self;
        };
        if self.z == 0 {
            return Self::from_affine(point);
        }
        let p = modulus.p;
        let mul = |x, y| modulus.mul(x, y);
        let z2 = mul(self.z, self.z);
        let h = sub(mul(x, z2), self.x, p);
        let r = sub(mul(y, mul(z2, self.z)), self.y, p);
        if h == 0 {
            return if r == 0 {
                self.double(a, modulus)
            } else {
                Self::from_affine(None)
            };
        }
        let hh = mul(h, h);
        let hhh = mul(hh, h);
        let v = mul(self.x, hh);
        let nx = sub(sub(mul(r, r), addmod(v, v, p), p), hhh, p);
        let ny = sub(mul(r, sub(v, nx, p)), mul(self.y, hhh), p);
        Self {
            x: nx,
            y: ny,
            z: mul(self.z, h),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_sums_match_independent_affine_sequences() {
        for (p, a, b, r, g, limit) in [
            (211, 0, 7, 199, Some((3, 33)), 199),
            (
                1_099_511_627_563,
                1_099_511_627_560,
                56,
                1_099_512_551_701,
                Some((1, 15_267_301_451)),
                64,
            ),
        ] {
            let curve = Curve::with_order_witness(p, a, b, r, g).unwrap();
            let points: Vec<_> = (0..limit).map(|k| curve.mul(g, k).unwrap()).collect();
            assert!(matches(&curve, &[], None).unwrap());
            assert!(!matches(&curve, &[], g).unwrap());
            for &first in &points {
                for &second in &points {
                    let neg = first.map(|(x, y)| (x, sub(0, y, p)));
                    let sequence = [first, None, second, neg];
                    let expected = sequence
                        .iter()
                        .try_fold(None, |acc, &point| curve.add(acc, point))
                        .unwrap();
                    assert!(matches(&curve, &sequence, expected).unwrap());
                    let different = curve.add(expected, g).unwrap();
                    assert!(!matches(&curve, &sequence, different).unwrap());
                    let opposite = expected.map(|(x, y)| (x, sub(0, y, p)));
                    assert_eq!(
                        matches(&curve, &sequence, opposite).unwrap(),
                        expected == opposite
                    );
                }
            }
        }
    }
    #[test]
    fn checked_sums_reject_noncanonical_and_off_curve_points() {
        let curve = Curve::new(211, 0, 7, 199).unwrap();
        for bad in [Some((211, 33)), Some((3, 211)), Some((0, 0))] {
            assert!(matches(&curve, &[None, bad, None], None).is_err());
            assert!(matches(&curve, &[], bad).is_err());
        }
    }
}
