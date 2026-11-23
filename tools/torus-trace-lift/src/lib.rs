//! Exact guarded trace-to-original-coordinate adapter; it discovers no relation roots.
//! Primality is an external contract, matching the existing Field arithmetic.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use torus_s4::Field;
use torus_s4_tree::Fp2;

pub mod controls;
pub mod identities;
pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
struct Dual {
    value: BigUint,
    derivative: BigUint,
}

/// Holds public power/label data only. No original imaginary coordinate is supplied.
pub struct TraceMap {
    f: Field,
    q: u64,
    omega: Fp2,
    inverse_q: BigUint,
    inverse_two_d: BigUint,
}
impl TraceMap {
    pub fn new(f: &Field, q: u64, omega: Fp2) -> Result<Self> {
        if q == 0 || (BigUint::from(q) % &f.p).is_zero() {
            return Err("positive exponent must be nonzero in the field".into());
        }
        if omega.a >= f.p || omega.b >= f.p || omega.norm(f) != BigUint::one() {
            return Err("omega coordinates/norm invalid".into());
        }
        if omega.b.is_zero() {
            return Err("ramified omega-imaginary-zero chart is excluded".into());
        }
        Ok(Self {
            f: f.clone(),
            q,
            inverse_q: f.inv(&(BigUint::from(q) % &f.p))?,
            inverse_two_d: f.inv(&f.scale(&omega.b, 2))?,
            omega,
        })
    }
    /// Evaluate classical T_q and U_(q-1) in logarithmically many ladder steps.
    /// The derivative is propagated explicitly; q's inverse is cached at setup.
    pub fn evaluate(&self, x: &BigUint) -> Result<(BigUint, BigUint)> {
        if x >= &self.f.p {
            return Err("noncanonical real coordinate".into());
        }
        let f = &self.f;
        let mut a = Dual {
            value: 1u32.into(),
            derivative: 0u32.into(),
        };
        let mut b = Dual {
            value: x.clone(),
            derivative: 1u32.into(),
        };
        for bit in (0..64 - self.q.leading_zeros()).rev() {
            let cross = Dual {
                value: f.sub(&f.scale(&f.mul(&a.value, &b.value), 2), x),
                derivative: f.sub(
                    &f.scale(
                        &f.add(
                            &f.mul(&a.derivative, &b.value),
                            &f.mul(&a.value, &b.derivative),
                        ),
                        2,
                    ),
                    &1u32.into(),
                ),
            };
            let square = |v: &Dual| Dual {
                value: f.sub(&f.scale(&f.square(&v.value), 2), &1u32.into()),
                derivative: f.scale(&f.mul(&v.value, &v.derivative), 4),
            };
            if (self.q >> bit) & 1 == 0 {
                a = square(&a);
                b = cross;
            } else {
                b = square(&b);
                a = cross;
            }
        }
        Ok((a.value, f.mul(&a.derivative, &self.inverse_q)))
    }
    /// None means this x fails this scalar membership equation, not DLP absence.
    /// A reported candidate must still pass the original equations in its consumer.
    pub fn lift(&self, x: &BigUint) -> Result<Option<Fp2>> {
        let (t, u) = self.evaluate(x)?;
        if t != self.omega.a {
            return Ok(None);
        }
        if u.is_zero() {
            return Err("nonramified trace root has zero U: invariant failure".into());
        }
        let a = self.f.sub(&self.f.square(x), &1u32.into());
        Ok(Some(Fp2 {
            a: x.clone(),
            b: self.f.mul(&self.f.mul(&a, &u), &self.inverse_two_d),
        }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixed_equivalence_and_original_coordinate_controls() {
        super::controls::run().expect("fixed predeclared trace-lift controls");
    }
}
