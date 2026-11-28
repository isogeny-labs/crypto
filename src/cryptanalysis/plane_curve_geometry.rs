//! Rational line sections of a homogeneous plane curve over a small prime field.
//!
//! This supplies geometry shared by degree-`d` relation collectors. It does
//! not construct the Jacobian, certify geometric smoothness, or normalize a
//! singular model. In particular, a quintic or sextic model of genus 4 or 5
//! needs a separate singularity and normalization layer.

use std::collections::BTreeMap;

/// A term `coeff * x^x_exp * y^y_exp * z^z_exp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaneTerm {
    pub x_exp: usize,
    pub y_exp: usize,
    pub z_exp: usize,
    pub coeff: u32,
}

#[derive(Clone, Debug)]
pub struct PlaneCurve {
    pub p: u32,
    pub degree: usize,
    pub terms: Vec<PlaneTerm>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineRoot {
    pub parameter: [u32; 2],
    pub multiplicity: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalLineFactorization {
    pub roots: Vec<LineRoot>,
    /// Degree left after removing all rational linear factors.
    pub residual_degree: usize,
}

/// A completely rational line section on a smooth plane curve.
/// Points use the first nonzero coordinate scaled to one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RationalLineDivisor {
    pub points: Vec<([u32; 3], usize)>,
}

fn prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += if d == 2 { 1 } else { 2 };
    }
    true
}

fn mul(a: u32, b: u32, p: u32) -> u32 {
    (u64::from(a) * u64::from(b) % u64::from(p)) as u32
}

fn pow(mut a: u32, mut n: usize, p: u32) -> u32 {
    let mut out = 1;
    while n > 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}

impl PlaneCurve {
    /// Combine duplicate monomials and reject malformed homogeneous input.
    /// The bounded degree keeps direct line substitution inexpensive.
    pub fn new(p: u32, degree: usize, terms: &[PlaneTerm]) -> Result<Self, &'static str> {
        if !prime(p) || p < 3 {
            return Err("field must have odd prime order");
        }
        if !(2..=16).contains(&degree) {
            return Err("degree must be between 2 and 16");
        }
        let mut combined = BTreeMap::<(usize, usize, usize), u32>::new();
        for term in terms {
            if term
                .x_exp
                .checked_add(term.y_exp)
                .and_then(|n| n.checked_add(term.z_exp))
                != Some(degree)
            {
                return Err("term is not homogeneous of the requested degree");
            }
            let entry = combined
                .entry((term.x_exp, term.y_exp, term.z_exp))
                .or_default();
            *entry = ((u64::from(*entry) + u64::from(term.coeff)) % u64::from(p)) as u32;
        }
        let terms: Vec<_> = combined
            .into_iter()
            .filter_map(|((x_exp, y_exp, z_exp), coeff)| {
                (coeff != 0).then_some(PlaneTerm {
                    x_exp,
                    y_exp,
                    z_exp,
                    coeff,
                })
            })
            .collect();
        if terms.is_empty() {
            return Err("zero polynomial");
        }
        Ok(Self { p, degree, terms })
    }

    pub fn eval(&self, point: [u32; 3]) -> u32 {
        let [x, y, z] = point.map(|v| v % self.p);
        self.terms.iter().fold(0u32, |sum, term| {
            let value = mul(
                mul(
                    mul(term.coeff, pow(x, term.x_exp, self.p), self.p),
                    pow(y, term.y_exp, self.p),
                    self.p,
                ),
                pow(z, term.z_exp, self.p),
                self.p,
            );
            ((u64::from(sum) + u64::from(value)) % u64::from(self.p)) as u32
        })
    }

    /// The three partial derivatives at a projective point.
    pub fn gradient(&self, point: [u32; 3]) -> [u32; 3] {
        let xyz = point.map(|v| v % self.p);
        let mut out = [0u32; 3];
        for term in &self.terms {
            let exps = [term.x_exp, term.y_exp, term.z_exp];
            for axis in 0..3 {
                if exps[axis] == 0 {
                    continue;
                }
                let mut exps = exps;
                exps[axis] -= 1;
                let value = (0..3).fold(
                    mul(term.coeff, (exps[axis] + 1) as u32 % self.p, self.p),
                    |v, i| mul(v, pow(xyz[i], exps[i], self.p), self.p),
                );
                out[axis] = ((u64::from(out[axis]) + u64::from(value)) % u64::from(self.p)) as u32;
            }
        }
        out
    }

    /// Tangent line coefficients at a nonsingular rational point.
    pub fn tangent(&self, point: [u32; 3]) -> Result<[u32; 3], &'static str> {
        if point.iter().all(|&v| v % self.p == 0) || self.eval(point) != 0 {
            return Err("point is not on the curve");
        }
        let gradient = self.gradient(point);
        if gradient == [0; 3] {
            return Err("singular rational point");
        }
        Ok(gradient)
    }

    /// Coefficients of `F(s*u + t*v)` in ascending powers of `t`.
    /// Returns an all-zero vector when the line is a curve component.
    pub fn line_section(&self, u: [u32; 3], v: [u32; 3]) -> Result<Vec<u32>, &'static str> {
        let u = u.map(|a| a % self.p);
        let v = v.map(|a| a % self.p);
        let independent = (0..3)
            .any(|i| ((i + 1)..3).any(|j| mul(u[i], v[j], self.p) != mul(u[j], v[i], self.p)));
        if !independent {
            return Err("line basis is dependent");
        }
        let mut section = vec![0u32; self.degree + 1];
        for term in &self.terms {
            let mut poly = vec![term.coeff];
            for (axis, exponent) in [term.x_exp, term.y_exp, term.z_exp].into_iter().enumerate() {
                for _ in 0..exponent {
                    let mut next = vec![0u32; poly.len() + 1];
                    for (i, &c) in poly.iter().enumerate() {
                        next[i] = ((u64::from(next[i]) + u64::from(mul(c, u[axis], self.p)))
                            % u64::from(self.p)) as u32;
                        next[i + 1] = ((u64::from(next[i + 1])
                            + u64::from(mul(c, v[axis], self.p)))
                            % u64::from(self.p)) as u32;
                    }
                    poly = next;
                }
            }
            for (i, c) in poly.into_iter().enumerate() {
                section[i] = ((u64::from(section[i]) + u64::from(c)) % u64::from(self.p)) as u32;
            }
        }
        Ok(section)
    }

    /// Factor `F(s*u+t*v)` by all rational linear factors, with multiplicity.
    /// A positive residual degree means the section has nonrational factors.
    /// The field cap prevents accidental whole-field scans on large inputs.
    pub fn rational_line_factors(
        &self,
        section: &[u32],
    ) -> Result<RationalLineFactorization, &'static str> {
        if section.len() != self.degree + 1 || section.iter().all(|&c| c % self.p == 0) {
            return Err("section is zero or has the wrong degree");
        }
        if self.p > 65_536 {
            return Err("rational line scan exceeds field cap");
        }
        // Coefficients of F(s*u+v), ascending in s. Leading zeros in this
        // dehomogenization are precisely the multiplicity at [1:0].
        let mut poly: Vec<u32> = section.iter().rev().map(|&c| c % self.p).collect();
        let mut infinity = 0;
        while poly.len() > 1 && poly.last() == Some(&0) {
            poly.pop();
            infinity += 1;
        }
        let mut roots = Vec::new();
        for s in 0..self.p {
            let mut multiplicity = 0;
            while poly.len() > 1 {
                let value = poly.iter().rev().fold(0u32, |acc, &c| {
                    ((u64::from(mul(acc, s, self.p)) + u64::from(c)) % u64::from(self.p)) as u32
                });
                if value != 0 {
                    break;
                }
                // Synthetic division by (s_variable - s), exact because
                // the Horner remainder above is zero.
                let mut quotient = vec![0u32; poly.len() - 1];
                let top = quotient.len() - 1;
                quotient[top] = poly[top + 1];
                for i in (0..top).rev() {
                    quotient[i] = ((u64::from(poly[i + 1])
                        + u64::from(mul(s, quotient[i + 1], self.p)))
                        % u64::from(self.p)) as u32;
                }
                poly = quotient;
                multiplicity += 1;
            }
            if multiplicity > 0 {
                roots.push(LineRoot {
                    parameter: [s, 1],
                    multiplicity,
                });
            }
        }
        if infinity > 0 {
            roots.push(LineRoot {
                parameter: [1, 0],
                multiplicity: infinity,
            });
        }
        Ok(RationalLineFactorization {
            roots,
            residual_degree: poly.len() - 1,
        })
    }

    /// Rational intersection parameters `[s:t]`, without multiplicities.
    pub fn rational_line_roots(&self, section: &[u32]) -> Result<Vec<[u32; 2]>, &'static str> {
        Ok(self
            .rational_line_factors(section)?
            .roots
            .into_iter()
            .map(|root| root.parameter)
            .collect())
    }

    /// Return a line's effective divisor when every intersection is rational.
    /// `None` means a nonrational factor remains. A rational singular point
    /// is rejected because its branches need a normalization model.
    pub fn rational_line_divisor(
        &self,
        u: [u32; 3],
        v: [u32; 3],
    ) -> Result<Option<RationalLineDivisor>, &'static str> {
        let section = self.line_section(u, v)?;
        let factors = self.rational_line_factors(&section)?;
        if factors.residual_degree != 0 {
            return Ok(None);
        }
        let mut points = BTreeMap::<[u32; 3], usize>::new();
        for root in factors.roots {
            let [s, t] = root.parameter;
            let point = std::array::from_fn(|i| {
                ((u64::from(mul(s, u[i] % self.p, self.p))
                    + u64::from(mul(t, v[i] % self.p, self.p)))
                    % u64::from(self.p)) as u32
            });
            let Some(&leading) = point.iter().find(|&&coordinate| coordinate != 0) else {
                return Err("line parameter gave the zero projective point");
            };
            let inverse = pow(leading, (self.p - 2) as usize, self.p);
            let normalized = point.map(|coordinate| mul(coordinate, inverse, self.p));
            if self.eval(normalized) != 0 {
                return Err("line root is not on the curve");
            }
            if self.gradient(normalized) == [0; 3] {
                return Err("line meets a singular rational point");
            }
            *points.entry(normalized).or_default() += root.multiplicity;
        }
        debug_assert_eq!(points.values().sum::<usize>(), self.degree);
        Ok(Some(RationalLineDivisor {
            points: points.into_iter().collect(),
        }))
    }

    /// Coefficients of `div(line_a / line_b)` at rational points.
    /// The caller must independently establish geometric smoothness of the
    /// whole curve. This method rejects only singularities it encounters.
    pub fn rational_line_relation(
        &self,
        line_a: ([u32; 3], [u32; 3]),
        line_b: ([u32; 3], [u32; 3]),
    ) -> Result<Option<BTreeMap<[u32; 3], i32>>, &'static str> {
        let Some(a) = self.rational_line_divisor(line_a.0, line_a.1)? else {
            return Ok(None);
        };
        let Some(b) = self.rational_line_divisor(line_b.0, line_b.1)? else {
            return Ok(None);
        };
        let mut relation = BTreeMap::<[u32; 3], i32>::new();
        for (point, multiplicity) in a.points {
            *relation.entry(point).or_default() += multiplicity as i32;
        }
        for (point, multiplicity) in b.points {
            *relation.entry(point).or_default() -= multiplicity as i32;
        }
        relation.retain(|_, coefficient| *coefficient != 0);
        Ok(Some(relation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn term(x: usize, y: usize, z: usize) -> PlaneTerm {
        PlaneTerm {
            x_exp: x,
            y_exp: y,
            z_exp: z,
            coeff: 1,
        }
    }

    #[test]
    fn cubic_line_section_and_tangent() {
        let c = PlaneCurve::new(5, 3, &[term(3, 0, 0), term(0, 3, 0), term(0, 0, 3)]).unwrap();
        let u = [1, 0, 0];
        let v = [0, 1, 0];
        let section = c.line_section(u, v).unwrap();
        assert_eq!(section, vec![1, 0, 0, 1]);
        assert_eq!(c.rational_line_roots(&section).unwrap(), vec![[4, 1]]);
        assert_eq!(
            c.rational_line_factors(&section).unwrap().roots[0].multiplicity,
            1
        );
        assert_eq!(c.tangent([1, 4, 0]).unwrap(), [3, 3, 0]);
        let tangent_section = c.line_section([1, 4, 0], [0, 0, 1]).unwrap();
        assert_eq!(tangent_section, vec![0, 0, 0, 1]);
        assert_eq!(
            c.rational_line_factors(&tangent_section).unwrap(),
            RationalLineFactorization {
                roots: vec![LineRoot {
                    parameter: [1, 0],
                    multiplicity: 3
                }],
                residual_degree: 0
            }
        );
    }

    #[test]
    fn singular_quintic_section() {
        let c = PlaneCurve::new(7, 5, &[term(5, 0, 0), term(0, 3, 2), term(0, 0, 5)]).unwrap();
        assert_eq!(c.eval([0, 1, 0]), 0);
        assert_eq!(c.gradient([0, 1, 0]), [0; 3]);
        assert!(c.tangent([0, 1, 0]).is_err());
        let section = c.line_section([1, 0, 0], [0, 1, 0]).unwrap();
        assert_eq!(section, vec![1, 0, 0, 0, 0, 0]);
        assert_eq!(c.rational_line_roots(&section).unwrap(), vec![[0, 1]]);
        assert_eq!(
            c.rational_line_factors(&section).unwrap().roots[0].multiplicity,
            5
        );
    }

    #[test]
    fn reject_invalid_models_and_lines() {
        assert!(PlaneCurve::new(9, 3, &[term(3, 0, 0)]).is_err());
        assert!(PlaneCurve::new(5, 3, &[term(2, 0, 0)]).is_err());
        let c = PlaneCurve::new(5, 3, &[term(3, 0, 0)]).unwrap();
        assert!(c.line_section([1, 0, 0], [2, 0, 0]).is_err());
        assert!(c.rational_line_roots(&[0, 0, 0, 0]).is_err());
    }

    #[test]
    fn retain_nonrational_residual_degree() {
        let c = PlaneCurve::new(3, 2, &[term(2, 0, 0), term(0, 2, 0), term(0, 0, 2)]).unwrap();
        let section = c.line_section([1, 0, 0], [0, 1, 0]).unwrap();
        assert_eq!(section, vec![1, 0, 1]);
        assert_eq!(
            c.rational_line_factors(&section).unwrap(),
            RationalLineFactorization {
                roots: vec![],
                residual_degree: 2,
            }
        );
        assert_eq!(c.rational_line_divisor([1, 0, 0], [0, 1, 0]).unwrap(), None);
    }

    #[test]
    fn cubic_line_divisors_give_a_principal_relation() {
        // Smooth elliptic cubic y^2 z = x^3 + x z^2 over F_5.
        let c = PlaneCurve::new(
            5,
            3,
            &[
                PlaneTerm {
                    x_exp: 0,
                    y_exp: 2,
                    z_exp: 1,
                    coeff: 1,
                },
                PlaneTerm {
                    x_exp: 3,
                    y_exp: 0,
                    z_exp: 0,
                    coeff: 4,
                },
                PlaneTerm {
                    x_exp: 1,
                    y_exp: 0,
                    z_exp: 2,
                    coeff: 4,
                },
            ],
        )
        .unwrap();
        let x_zero = ([0, 1, 0], [0, 0, 1]);
        let z_zero = ([1, 0, 0], [0, 1, 0]);
        let divisor = c
            .rational_line_divisor(x_zero.0, x_zero.1)
            .unwrap()
            .unwrap();
        assert_eq!(divisor.points, vec![([0, 0, 1], 2), ([0, 1, 0], 1)]);
        let relation = c.rational_line_relation(x_zero, z_zero).unwrap().unwrap();
        assert_eq!(relation, BTreeMap::from([([0, 0, 1], 2), ([0, 1, 0], -2)]));
        assert_eq!(relation.values().sum::<i32>(), 0);
    }

    #[test]
    fn reject_singular_line_divisor() {
        let c = PlaneCurve::new(7, 5, &[term(5, 0, 0), term(0, 3, 2), term(0, 0, 5)]).unwrap();
        assert!(c.rational_line_divisor([1, 0, 0], [0, 1, 0]).is_err());
    }

    #[test]
    fn line_substitution_matches_direct_evaluation() {
        for degree in 2..=6 {
            let terms: Vec<_> = (0..=degree)
                .flat_map(|x_exp| {
                    (0..=degree - x_exp).map(move |y_exp| PlaneTerm {
                        x_exp,
                        y_exp,
                        z_exp: degree - x_exp - y_exp,
                        coeff: (1 + 3 * x_exp + 5 * y_exp) as u32,
                    })
                })
                .collect();
            let curve = PlaneCurve::new(11, degree, &terms).unwrap();
            let u = [1, 2, 3];
            let v = [4, 5, 7];
            let section = curve.line_section(u, v).unwrap();
            let factors = curve.rational_line_factors(&section).unwrap();
            assert_eq!(
                factors.residual_degree
                    + factors
                        .roots
                        .iter()
                        .map(|root| root.multiplicity)
                        .sum::<usize>(),
                degree
            );
            for s in 0..11u32 {
                for t in 0..11u32 {
                    let point = std::array::from_fn(|i| (s * u[i] + t * v[i]) % 11);
                    let expected = curve.eval(point);
                    let observed = section.iter().enumerate().fold(0u32, |acc, (i, &c)| {
                        (acc + mul(mul(c, pow(s, degree - i, 11), 11), pow(t, i, 11), 11)) % 11
                    });
                    assert_eq!(observed, expected, "degree={degree} s={s} t={t}");
                }
            }
        }
    }
}
