//! Exact graph extensions for H mod U, with quadratic coefficient equations.
//!
//! The 122 variables are b,c,d; four coefficients of the monic quartic;
//! sixteen modular multiplication gates (seven auxiliaries each); and the
//! three quotient coefficients of the final zero-remainder gate. No domain,
//! candidate subset, target scalar, or decomposition witness is supplied.

use crate::b677_fixture::Hybrid677;
use crate::extractor::{public_polynomials, Builder, MPoly};
use prime31_algebraic_ic::Curve;

pub(crate) const VARIABLES: usize = 122;

pub(crate) struct Circuit {
    pub(crate) u: Vec<MPoly>,
    pub(crate) constraints: Vec<MPoly>,
    #[cfg(test)]
    plans: Vec<GatePlan>,
    #[cfg(test)]
    bindings: Vec<(usize, std::ops::Range<usize>)>,
}

#[cfg(test)]
struct GatePlan {
    left: Vec<MPoly>,
    right: Vec<MPoly>,
    remainder_start: Option<usize>,
    quotient_start: usize,
    equation_start: usize,
}

struct CircuitBuilder<'a> {
    b: &'a mut Builder,
    quartic: Vec<MPoly>,
    next: usize,
    constraints: Vec<MPoly>,
    #[cfg(test)]
    plans: Vec<GatePlan>,
    #[cfg(test)]
    bindings: Vec<(usize, std::ops::Range<usize>)>,
}

impl CircuitBuilder<'_> {
    fn add(&mut self, a: &[MPoly], b: &[MPoly]) -> Result<Vec<MPoly>, String> {
        assert_eq!(a.len(), 4);
        assert_eq!(b.len(), 4);
        a.iter().zip(b).map(|(x, y)| self.b.add(x, y)).collect()
    }

    fn constant_add(&mut self, a: &[MPoly], scalar: u64) -> Result<Vec<MPoly>, String> {
        let mut out = a.to_vec();
        out[0] = self.b.add(&out[0], &self.b.c(scalar))?;
        Ok(out)
    }

    fn linear_combination(
        &mut self,
        one: u64,
        a: &[MPoly],
        a_scalar: u64,
        b: &[MPoly],
        b_scalar: u64,
    ) -> Result<Vec<MPoly>, String> {
        let mut out = Vec::with_capacity(4);
        for j in 0..4 {
            let x = self.b.scale(&a[j], a_scalar)?;
            let y = self.b.scale(&b[j], b_scalar)?;
            out.push(self.b.add(&x, &y)?);
        }
        self.constant_add(&out, one)
    }

    /// Enforce every coefficient of A*B-R=U*Q. All residues have degree <4,
    /// Q has degree <=2, and U is monic of degree4. In the final gate R=0.
    fn multiply(
        &mut self,
        left: &[MPoly],
        right: &[MPoly],
        final_zero: bool,
    ) -> Result<Vec<MPoly>, String> {
        assert_eq!(left.len(), 4);
        assert_eq!(right.len(), 4);
        let remainder_start = if final_zero {
            None
        } else {
            let start = self.next;
            self.next += 4;
            Some(start)
        };
        let remainder: Vec<_> = (0..4)
            .map(|j| remainder_start.map_or_else(|| self.b.c(0), |start| self.b.var(start + j)))
            .collect();
        let quotient_start = self.next;
        self.next += 3;
        if self.next > self.b.variables {
            return Err("circuit variable inventory overflow".into());
        }
        let quotient: Vec<_> = (0..3).map(|j| self.b.var(quotient_start + j)).collect();
        let mut equations = self.b.conv(left, right)?;
        let uq = self.b.conv(&self.quartic, &quotient)?;
        assert_eq!(equations.len(), 7);
        assert_eq!(uq.len(), 7);
        for j in 0..7 {
            let neg = self.b.scale(&uq[j], self.b.p - 1)?;
            equations[j] = self.b.add(&equations[j], &neg)?;
            if j < 4 {
                let neg = self.b.scale(&remainder[j], self.b.p - 1)?;
                equations[j] = self.b.add(&equations[j], &neg)?;
            }
        }
        #[cfg(test)]
        {
            let equation_start = self.constraints.len();
            if let Some(start) = remainder_start {
                for variable in start..start + 4 {
                    self.bindings
                        .push((variable, equation_start..equation_start + 7));
                }
            }
            // The final gate's high coefficients uniquely determine Q even
            // when its low coefficients are a nonzero tested remainder.
            let begin = if final_zero {
                equation_start + 4
            } else {
                equation_start
            };
            for variable in quotient_start..quotient_start + 3 {
                self.bindings.push((variable, begin..equation_start + 7));
            }
            self.plans.push(GatePlan {
                left: left.to_vec(),
                right: right.to_vec(),
                remainder_start,
                quotient_start,
                equation_start,
            });
        }
        self.constraints.extend(equations);
        Ok(remainder)
    }
}

pub(crate) fn build(
    builder: &mut Builder,
    e: Curve,
    target: (u64, u64),
    fixture: &Hybrid677,
) -> Result<Circuit, String> {
    assert_eq!(builder.variables, VARIABLES);
    let (u, _) = public_polynomials(builder, e, target, &[])?;
    let mut quartic: Vec<_> = (3..7).map(|j| builder.var(j)).collect();
    quartic.push(builder.c(1));
    let mut constraints = Vec::with_capacity(123);
    for j in 0..4 {
        let neg = builder.scale(&u[j], e.p - 1)?;
        constraints.push(builder.add(&quartic[j], &neg)?);
    }
    let mut c = CircuitBuilder {
        b: builder,
        quartic,
        next: 7,
        constraints,
        #[cfg(test)]
        plans: Vec::new(),
        #[cfg(test)]
        bindings: (0..4).map(|j| (3 + j, j..j + 1)).collect(),
    };

    // D4 = X^4 - 4 X^2 + 2. Replacing X^4 by -sum u_j X^j
    // is affine in the explicit quartic coefficients, so no gate is needed.
    let mut dickson: Vec<_> = (0..4)
        .map(|j| c.b.scale(&c.quartic[j], e.p - 1))
        .collect::<Result<_, _>>()?;
    dickson[0] = c.b.add(&dickson[0], &c.b.c(2))?;
    dickson[2] = c.b.add(&dickson[2], &c.b.c(e.p - 4))?;
    for _ in 0..8 {
        let square = c.multiply(&dickson, &dickson, false)?;
        dickson = c.constant_add(&square, e.p - 2)?;
    }
    dickson = c.constant_add(&dickson, (e.p - fixture.terminal()) % e.p)?;

    let f = vec![c.b.c(e.b), c.b.c(e.a), c.b.c(0), c.b.c(1)];
    let f2 = c.multiply(&f, &f, false)?;
    let f3 = c.multiply(&f2, &f, false)?;
    let f6 = c.multiply(&f3, &f3, false)?;
    let t = c.multiply(&f6, &f, false)?;

    // g(T)=product(T-c_i^2), scalar coefficients in ascending order.
    let mut g = vec![1];
    for &root in fixture.coset_squares() {
        let mut next = vec![0; g.len() + 1];
        for (j, &coefficient) in g.iter().enumerate() {
            let neg = c.b.mm(coefficient, (e.p - root) % e.p);
            next[j] = c.b.am(next[j], neg);
            next[j + 1] = c.b.am(next[j + 1], coefficient);
        }
        g = next;
    }
    assert_eq!(g.len(), 9);
    assert_eq!(g[8], 1);

    // Block size3: g=B0+T^3*(B1+T^3*B2), each block degree<=2.
    // T^2,T^3 and the two block products require four modular gates.
    let t2 = c.multiply(&t, &t, false)?;
    let t3 = c.multiply(&t2, &t, false)?;
    let b0 = c.linear_combination(g[0], &t, g[1], &t2, g[2])?;
    let b1 = c.linear_combination(g[3], &t, g[4], &t2, g[5])?;
    let b2 = c.linear_combination(g[6], &t, g[7], &t2, g[8])?;
    let high = c.multiply(&b2, &t3, false)?;
    let middle = c.add(&b1, &high)?;
    let high = c.multiply(&middle, &t3, false)?;
    let y7 = c.add(&b0, &high)?;
    c.multiply(&dickson, &y7, true)?;
    assert_eq!(c.next, VARIABLES);
    assert_eq!(c.constraints.len(), 123);
    Ok(Circuit {
        u,
        constraints: c.constraints,
        #[cfg(test)]
        plans: c.plans,
        #[cfg(test)]
        bindings: c.bindings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime31_algebraic_ic::PRIME31;
    use std::time::{Duration, Instant};

    fn mul(a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % PRIME31.p as u128) as u64
    }
    fn polynomial_product(a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] = (out[i + j] + mul(x, y)) % PRIME31.p;
            }
        }
        out
    }
    fn divide(a: &[u64], u: &[u64]) -> (Vec<u64>, Vec<u64>) {
        assert_eq!(u.len(), 5);
        assert_eq!(u[4], 1);
        let mut remainder = a.to_vec();
        remainder.resize(remainder.len().max(4), 0);
        let mut quotient = vec![0; remainder.len().saturating_sub(4)];
        for degree in (4..remainder.len()).rev() {
            let lead = remainder[degree];
            quotient[degree - 4] = lead;
            for j in 0..5 {
                remainder[degree - 4 + j] =
                    (remainder[degree - 4 + j] + PRIME31.p - mul(lead, u[j])) % PRIME31.p;
            }
            assert_eq!(remainder[degree], 0);
        }
        remainder.truncate(4);
        (quotient, remainder)
    }
    fn evaluate(f: &MPoly, assignment: &[u64]) -> u64 {
        f.iter().fold(0, |acc, (powers, coefficient)| {
            assert_eq!(powers.len(), assignment.len());
            let mut term = *coefficient;
            for (&power, &value) in powers.iter().zip(assignment) {
                for _ in 0..power {
                    term = mul(term, value);
                }
            }
            (acc + term) % PRIME31.p
        })
    }

    // This direct scalar formula does not evaluate the symbolic U builder.
    fn scalar_quartic(target: (u64, u64), parameters: [u64; 3]) -> Vec<u64> {
        let p = PRIME31.p;
        let (r, s) = target;
        let [b, c, d] = parameters;
        let first = polynomial_product(
            &[(mul(r, r) + PRIME31.a) % p, r, 1],
            &[mul(b, b), 2 * b % p, 1],
        );
        let second = polynomial_product(
            &[(p - r) % p, 1],
            &[mul(d, d), 2 * mul(c, d) % p, mul(c, c)],
        );
        let third = polynomial_product(&[b, 1], &[d, c]);
        let mut u = first;
        for j in 0..second.len() {
            u[j] = (u[j] + p - second[j]) % p;
        }
        for j in 0..third.len() {
            u[j] = (u[j] + p - mul(2 * s % p, third[j])) % p;
        }
        assert_eq!(u[4], 1);
        u
    }

    // Dense univariate selector formation is test-only. It is independent of
    // the candidate's blocked quotient-circuit traversal and coefficient maps.
    fn dense_selector(fixture: &Hybrid677) -> Vec<u64> {
        let p = PRIME31.p;
        let mut dickson = vec![0, 1];
        for _ in 0..10 {
            dickson = polynomial_product(&dickson, &dickson);
            dickson[0] = (dickson[0] + p - 2) % p;
        }
        dickson[0] = (dickson[0] + p - fixture.terminal()) % p;
        let f = vec![PRIME31.b, PRIME31.a, 0, 1];
        let mut f7 = vec![1];
        for _ in 0..7 {
            f7 = polynomial_product(&f7, &f);
        }
        let mut y7 = vec![1];
        for &constant in fixture.coset_squares() {
            let mut factor = f7.clone();
            factor[0] = (factor[0] + p - constant) % p;
            y7 = polynomial_product(&y7, &factor);
        }
        let h = polynomial_product(&dickson, &y7);
        assert_eq!(h.len(), 1193);
        assert_eq!(h.last(), Some(&1));
        h
    }

    #[test]
    fn b677_circuit_matches_dense_selector_and_rejects_every_auxiliary_mutation() {
        let fixture = Hybrid677::build().unwrap();
        let target = (18, 221278090);
        let mut builder = Builder {
            p: PRIME31.p,
            variables: VARIABLES,
            deadline: Instant::now() + Duration::from_secs(30),
            term_cap: 20_000,
            adds: 0,
            muls: 0,
        };
        let circuit = build(&mut builder, PRIME31, target, &fixture).unwrap();
        assert_eq!(circuit.plans.len(), 17);
        assert_eq!(circuit.bindings.len(), VARIABLES - 3);
        assert_eq!(circuit.constraints.len(), 123);
        let max_degree = circuit
            .constraints
            .iter()
            .flat_map(|f| f.keys())
            .map(|exponents| exponents.iter().sum::<u32>())
            .max()
            .unwrap();
        assert_eq!(max_degree, 2);
        let h = dense_selector(&fixture);
        let parameters = [[0, 0, 0], [1, 1, 1], [PRIME31.p - 1; 3], [17, 31, 43]];
        let mut mutations = 0;
        for point in parameters {
            let u = scalar_quartic(target, point);
            let mut assignment = vec![0; VARIABLES];
            assignment[..3].copy_from_slice(&point);
            assignment[3..7].copy_from_slice(&u[..4]);
            for (j, coefficient) in circuit.u.iter().enumerate() {
                assert_eq!(evaluate(coefficient, &assignment), u[j]);
            }
            for plan in &circuit.plans {
                let left: Vec<_> = plan.left.iter().map(|f| evaluate(f, &assignment)).collect();
                let right: Vec<_> = plan
                    .right
                    .iter()
                    .map(|f| evaluate(f, &assignment))
                    .collect();
                let (quotient, remainder) = divide(&polynomial_product(&left, &right), &u);
                assert_eq!(quotient.len(), 3);
                assignment[plan.quotient_start..plan.quotient_start + 3].copy_from_slice(&quotient);
                if let Some(start) = plan.remainder_start {
                    assignment[start..start + 4].copy_from_slice(&remainder);
                } else {
                    let (_, expected) = divide(&h, &u);
                    assert_eq!(remainder, expected);
                    let observed: Vec<_> = circuit.constraints
                        [plan.equation_start..plan.equation_start + 4]
                        .iter()
                        .map(|f| evaluate(f, &assignment))
                        .collect();
                    assert_eq!(observed, expected);
                }
            }
            for (variable, equations) in &circuit.bindings {
                assert!(circuit.constraints[equations.clone()]
                    .iter()
                    .all(|f| evaluate(f, &assignment) == 0));
                let mut poisoned = assignment.clone();
                poisoned[*variable] = (poisoned[*variable] + 1) % PRIME31.p;
                assert!(
                    circuit.constraints[equations.clone()]
                        .iter()
                        .any(|f| evaluate(f, &poisoned) != 0),
                    "auxiliary {variable} escaped its defining equations"
                );
                mutations += 1;
            }
        }
        assert_eq!(mutations, 4 * (VARIABLES - 3));
        println!(
            "B677_CIRCUIT {}",
            serde_json::json!({
                "variables": VARIABLES, "equations": circuit.constraints.len(),
                "terms": circuit.constraints.iter().map(MPoly::len).sum::<usize>(),
                "max_total_degree": max_degree, "modular_multiplication_gates": circuit.plans.len(),
                "dense_selector_degree": h.len() - 1, "parameter_controls": 4,
                "rejected_auxiliary_mutations": mutations,
                "symbolic_additions": builder.adds, "symbolic_multiplications": builder.muls
            })
        );
    }
}
