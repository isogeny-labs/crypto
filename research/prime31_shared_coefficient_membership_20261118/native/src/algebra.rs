use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AffinePoint {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u64>,
}

impl Curve {
    pub fn validate(&self) -> Result<(), String> {
        if self.p < 5 || self.p & 1 == 0 {
            return Err("field modulus must be an odd integer >= 5".into());
        }
        if self.a >= self.p || self.b >= self.p {
            return Err("curve coefficients must be canonical field elements".into());
        }
        let discriminant = self.add(
            self.mul(4, self.mul(self.mul(self.a, self.a), self.a)),
            self.mul(27, self.mul(self.b, self.b)),
        );
        if discriminant == 0 {
            return Err("singular short-Weierstrass curve".into());
        }
        Ok(())
    }

    #[inline]
    pub fn add(&self, left: u64, right: u64) -> u64 {
        ((u128::from(left) + u128::from(right)) % u128::from(self.p)) as u64
    }

    #[inline]
    pub fn neg(&self, value: u64) -> u64 {
        if value == 0 {
            0
        } else {
            self.p - value
        }
    }

    #[inline]
    pub fn sub(&self, left: u64, right: u64) -> u64 {
        self.add(left, self.neg(right))
    }

    #[inline]
    pub fn mul(&self, left: u64, right: u64) -> u64 {
        (u128::from(left) * u128::from(right) % u128::from(self.p)) as u64
    }

    pub fn pow(&self, mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = self.mul(result, base);
            }
            base = self.mul(base, base);
            exponent >>= 1;
        }
        result
    }

    pub fn inverse(&self, value: u64) -> Option<u64> {
        (value != 0).then(|| self.pow(value, self.p - 2))
    }

    pub fn rhs(&self, x: u64) -> u64 {
        self.add(
            self.add(self.mul(self.mul(x, x), x), self.mul(self.a, x)),
            self.b,
        )
    }

    pub fn on_curve(&self, point: AffinePoint) -> bool {
        point.x < self.p && point.y < self.p && self.mul(point.y, point.y) == self.rhs(point.x)
    }

    pub fn sqrt(&self, value: u64) -> Option<u64> {
        if value == 0 {
            return Some(0);
        }
        if self.p & 3 != 3 {
            return None;
        }
        let root = self.pow(value, (self.p + 1) / 4);
        (self.mul(root, root) == value).then_some(root)
    }

    pub fn point_neg(&self, point: Option<AffinePoint>) -> Option<AffinePoint> {
        point.map(|point| AffinePoint {
            x: point.x,
            y: self.neg(point.y),
        })
    }

    pub fn point_add(
        &self,
        left: Option<AffinePoint>,
        right: Option<AffinePoint>,
    ) -> Option<AffinePoint> {
        let (left, right) = match (left, right) {
            (None, point) | (point, None) => return point,
            (Some(left), Some(right)) => (left, right),
        };
        if left.x == right.x && left.y == self.neg(right.y) {
            return None;
        }
        let slope = if left == right {
            if left.y == 0 {
                return None;
            }
            let numerator = self.add(self.mul(3, self.mul(left.x, left.x)), self.a);
            self.mul(numerator, self.inverse(self.mul(2, left.y))?)
        } else {
            self.mul(
                self.sub(right.y, left.y),
                self.inverse(self.sub(right.x, left.x))?,
            )
        };
        let x = self.sub(self.sub(self.mul(slope, slope), left.x), right.x);
        let y = self.sub(self.mul(slope, self.sub(left.x, x)), left.y);
        Some(AffinePoint { x, y })
    }

    pub fn scalar_mul(&self, mut scalar: u64, point: AffinePoint) -> Option<AffinePoint> {
        let mut result = None;
        let mut addend = Some(point);
        while scalar != 0 {
            if scalar & 1 != 0 {
                result = self.point_add(result, addend);
            }
            addend = self.point_add(addend, addend);
            scalar >>= 1;
        }
        result
    }

    pub fn canonical_base(&self, size: usize) -> Result<Vec<AffinePoint>, String> {
        let mut result = Vec::with_capacity(size);
        for x in 0..self.p {
            if let Some(y) = self.sqrt(self.rhs(x)) {
                if y != 0 {
                    let point = AffinePoint {
                        x,
                        y: y.min(self.p - y),
                    };
                    if !self.on_curve(point) {
                        return Err("square-root routine returned an off-curve point".into());
                    }
                    result.push(point);
                    if result.len() == size {
                        return Ok(result);
                    }
                }
            }
        }
        Err(format!(
            "field contains fewer than {size} admissible base points"
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{prime31_curve, PRIME31_GENERATOR, PRIME31_N};

    #[test]
    fn prime31_curve_and_order_certificate() {
        let curve = prime31_curve();
        curve.validate().unwrap();
        assert!(curve.on_curve(PRIME31_GENERATOR));
        assert_eq!(curve.scalar_mul(PRIME31_N, PRIME31_GENERATOR), None);
        assert_ne!(curve.scalar_mul(1, PRIME31_GENERATOR), None);
    }

    #[test]
    fn canonical_prime31_base_prefix_is_frozen() {
        let base = prime31_curve().canonical_base(8).unwrap();
        assert_eq!(
            base.iter().map(|point| point.x).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 5, 6, 8, 9]
        );
    }
}
