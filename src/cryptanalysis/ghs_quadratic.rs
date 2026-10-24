//! Executable quadratic binary GHS descent and trace-conorm map.
//!
//! K=F_(q^2), beta=sqrt(b), D=beta+beta^q != 0. Choose lambda with
//! lambda+lambda^q=1. Put A=a+a^q, B=beta+lambda^2*D,
//! C0=a+lambda^2*A, mu=sqrt(B/D), L=B/D+mu+lambda^2+lambda,
//! H=u^2+u+A. Then C: W^2+H*W=D*H+(L*u+(B/D)*A+C0)*H^2.
//! The K-morphism C -> E is
//! X=beta*H/D, Y=beta*W/D+(mu+lambda)*X*u+beta.
//! The source-point-class map is trace(conorm(P-O)); its prime-subgroup
//! injectivity must be checked on the actual generator, never from genus.

use super::ghs_descent::{ECurve, Pt};
use super::ghs_generalized::{GeneralizedContext, frobenius};
use super::ghs_screen::GhsCurveInput;
use crate::binary_ecc::hyperelliptic::{HyperellipticCurve, MumfordDivisor};
use crate::binary_ecc::poly_f2m::F2mPoly;
use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use num_traits::ToPrimitive;
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn elt(x: u64, m: u32) -> F2mElement {
    F2mElement::from_biguint(&x.into(), m)
}
fn poly(xs: Vec<F2mElement>, m: u32) -> F2mPoly {
    F2mPoly::from_coeffs(xs, m)
}
fn inv(x: &F2mElement, i: &IrreduciblePoly) -> Result<F2mElement, String> {
    x.flt_inverse(i).ok_or("division by zero".into())
}

#[derive(Clone, Debug)]
pub struct QuadraticDescent {
    pub source: ECurve,
    pub curve: HyperellipticCurve,
    pub curve_over_top: HyperellipticCurve,
    pub base_degree: u32,
    pub beta: F2mElement,
    pub trace_beta: F2mElement,
    pub trace_a: F2mElement,
    pub lambda: F2mElement,
    pub mu: F2mElement,
    pub embedding: Vec<F2mElement>,
    pub inverse_embedding: BTreeMap<u64, u64>,
}

impl QuadraticDescent {
    /// Explicit bounded constructor. The map formulas themselves are general;
    /// choosing and inverting the subfield basis here uses enumeration.
    pub fn construct(input: &GhsCurveInput) -> Result<Self, String> {
        let n = input.absolute_degree;
        if n < 4 || n > 16 || !n.is_multiple_of(2) {
            return Err("quadratic constructor requires even absolute degree in 4..=16".into());
        }
        let s = n / 2;
        let ctx = GeneralizedContext::new(input)?;
        let i = &ctx.irr;
        let beta = ctx.beta.clone();
        let d = beta.add(&frobenius(beta.clone(), s, i));
        if d.is_zero() {
            return Err("beta lies in the base field; this is the trace branch, not this genus-two constructor".into());
        }
        let one = F2mElement::one(n);
        let lambda = (2..1u64 << n)
            .map(|x| elt(x, n))
            .find(|x| x.add(&frobenius(x.clone(), s, i)) == one)
            .ok_or("no trace-one element")?;
        let a = ctx.a.clone();
        let atr = a.add(&frobenius(a.clone(), s, i));
        let l2 = lambda.square(i);
        let bd = beta.add(&l2.mul(&d, i)).mul(&inv(&d, i)?, i);
        let c0 = a.add(&l2.mul(&atr, i));
        let mu = frobenius(bd.clone(), s - 1, i);
        let linear = bd.add(&mu).add(&l2).add(&lambda);
        if linear.is_zero() {
            return Err("genus-one exceptional model requires a separate infinity adapter".into());
        }
        let constant = bd.mul(&atr, i).add(&c0);
        let h = poly(vec![atr.clone(), one.clone(), one.clone()], n);
        let f = h
            .scalar_mul(&d, i)
            .add(&h.mul(&h, i).mul(&poly(vec![constant, linear], n), i));
        let top = HyperellipticCurve::try_new(n, i.clone(), h.clone(), f.clone(), 2)
            .map_err(|e| e.to_string())?;
        // Least polynomial-basis subfield generator with full degree s.
        let theta = (2..1u64 << n)
            .map(|x| elt(x, n))
            .find(|x| {
                frobenius(x.clone(), s, i) == *x
                    && (1..s).all(|j| !s.is_multiple_of(j) || frobenius(x.clone(), j, i) != *x)
            })
            .ok_or("no full-degree subfield generator")?;
        let mut embedding = vec![one.clone()];
        for j in 1..=s {
            embedding.push(embedding[(j - 1) as usize].mul(&theta, i));
        }
        let mut inverse_embedding = BTreeMap::new();
        for mask in 0..1u64 << s {
            let mut value = F2mElement::zero(n);
            for j in 0..s {
                if (mask >> j) & 1 == 1 {
                    value = value.add(&embedding[j as usize]);
                }
            }
            inverse_embedding.insert(value.to_biguint().to_u64().ok_or("field overflow")?, mask);
        }
        let theta_s = embedding[s as usize]
            .to_biguint()
            .to_u64()
            .ok_or("field overflow")?;
        let lower = *inverse_embedding
            .get(&theta_s)
            .ok_or("basis did not close")?;
        let base_irr = IrreduciblePoly {
            degree: s,
            low_terms: (0..s).filter(|j| (lower >> j) & 1 == 1).collect(),
        };
        let down = |p: &F2mPoly| -> Result<F2mPoly, String> {
            Ok(poly(
                p.coeffs
                    .iter()
                    .map(|x| {
                        let key = x.to_biguint().to_u64().ok_or("field overflow")?;
                        inverse_embedding
                            .get(&key)
                            .map(|v| elt(*v, s))
                            .ok_or("coefficient did not descend".into())
                    })
                    .collect::<Result<_, String>>()?,
                s,
            ))
        };
        let curve = HyperellipticCurve::try_new(s, base_irr, down(&h)?, down(&f)?, 2)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            source: ECurve::new(n, i.clone(), a, F2mElement::from_biguint(&input.b, n)),
            curve,
            curve_over_top: top,
            base_degree: s,
            beta,
            trace_beta: d,
            trace_a: atr,
            lambda,
            mu,
            embedding,
            inverse_embedding,
        })
    }

    pub fn lift_element(&self, x: &F2mElement) -> F2mElement {
        let mut out = F2mElement::zero(self.source.m);
        let mask = x.to_biguint();
        for j in 0..self.base_degree {
            if mask.bit(u64::from(j)) {
                out = out.add(&self.embedding[j as usize]);
            }
        }
        out
    }

    pub fn down_element(&self, x: &F2mElement) -> Result<F2mElement, String> {
        let key = x.to_biguint().to_u64().ok_or("field overflow")?;
        self.inverse_embedding
            .get(&key)
            .map(|v| elt(*v, self.base_degree))
            .ok_or("image coefficient is not Frobenius fixed".into())
    }

    pub fn map(&self, p: &Pt) -> Result<MumfordDivisor, String> {
        if !self.source.is_on_curve(p) {
            return Err("source point is off curve".into());
        }
        let Pt::Aff { x, y } = p else {
            return Ok(MumfordDivisor::identity(self.base_degree));
        };
        let i = &self.source.irr;
        let db = self.trace_beta.mul(&inv(&self.beta, i)?, i);
        let hx = db.mul(x, i);
        let u = poly(
            vec![
                self.trace_a.add(&hx),
                F2mElement::one(self.source.m),
                F2mElement::one(self.source.m),
            ],
            self.source.m,
        );
        let v = poly(
            vec![
                db.mul(&y.add(&self.beta), i),
                self.mu.add(&self.lambda).mul(&hx, i),
            ],
            self.source.m,
        );
        let pull =
            MumfordDivisor::try_new(&self.curve_over_top, u, v).map_err(|e| e.to_string())?;
        let sigma = |p: &F2mPoly| {
            poly(
                p.coeffs
                    .iter()
                    .map(|x| frobenius(x.clone(), self.base_degree, i))
                    .collect(),
                self.source.m,
            )
        };
        let conjugate =
            MumfordDivisor::try_new(&self.curve_over_top, sigma(&pull.u), sigma(&pull.v))
                .map_err(|e| e.to_string())?;
        let trace = pull
            .try_add(&conjugate, &self.curve_over_top)
            .map_err(|e| e.to_string())?;
        let down = |p: &F2mPoly| -> Result<F2mPoly, String> {
            Ok(poly(
                p.coeffs
                    .iter()
                    .map(|x| self.down_element(x))
                    .collect::<Result<_, _>>()?,
                self.base_degree,
            ))
        };
        MumfordDivisor::try_new(&self.curve, down(&trace.u)?, down(&trace.v)?)
            .map_err(|e| e.to_string())
    }

    pub fn model_json(&self) -> Value {
        let coeff = |p: &F2mPoly| {
            p.coeffs
                .iter()
                .map(|x| format!("0x{:x}", x.to_biguint()))
                .collect::<Vec<_>>()
        };
        json!({"schema":"ghs-quadratic-map/v1","absolute_degree":self.source.m,"base_degree":self.base_degree,
            "base_modulus_low_terms":self.curve.irr.low_terms,"h":coeff(&self.curve.h),"f":coeff(&self.curve.f),
            "beta":format!("0x{:x}",self.beta.to_biguint()),"trace_beta":format!("0x{:x}",self.trace_beta.to_biguint()),
            "trace_a":format!("0x{:x}",self.trace_a.to_biguint()),"lambda":format!("0x{:x}",self.lambda.to_biguint()),"mu":format!("0x{:x}",self.mu.to_biguint()),
            "subfield_embedding_basis":self.embedding.iter().take(self.base_degree as usize).map(|x|format!("0x{:x}",x.to_biguint())).collect::<Vec<_>>(),
            "source_map":"trace_K/k(conorm_C/E(P-O))","cover_degree":2,"genus":2})
    }
}

pub fn divisor_json(d: &MumfordDivisor) -> Value {
    let coeff = |p: &F2mPoly| {
        p.coeffs
            .iter()
            .map(|x| format!("0x{:x}", x.to_biguint()))
            .collect::<Vec<_>>()
    };
    json!({"u":coeff(&d.u),"v":coeff(&d.v)})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quadratic_map_is_additive_including_exceptional_points() {
        for (n, modulus) in [(6, 0x43u64), (8, 0x11b)] {
            for (a, b) in [(0u64, 2u64), (1, 3), (2, 5)] {
                let input = GhsCurveInput {
                    absolute_degree: n,
                    modulus: modulus.into(),
                    a: a.into(),
                    b: b.into(),
                };
                let Ok(descent) = QuadraticDescent::construct(&input) else {
                    continue;
                };
                let e = &descent.source;
                let mut points = vec![
                    Pt::Inf,
                    Pt::Aff {
                        x: elt(0, n),
                        y: descent.beta.clone(),
                    },
                ];
                for x in 1..1u64 << n {
                    for y in 0..1u64 << n {
                        let p = Pt::Aff {
                            x: elt(x, n),
                            y: elt(y, n),
                        };
                        if e.is_on_curve(&p) {
                            points.push(p);
                            break;
                        }
                    }
                    if points.len() >= 18 {
                        break;
                    }
                }
                for p in &points {
                    for q in &points {
                        let left = descent.map(&e.add(p, q)).unwrap();
                        let right = descent
                            .map(p)
                            .unwrap()
                            .try_add(&descent.map(q).unwrap(), &descent.curve)
                            .unwrap();
                        assert_eq!(left, right, "n={n},a={a},b={b},p={p:?},q={q:?}");
                    }
                }
            }
        }
    }
}
