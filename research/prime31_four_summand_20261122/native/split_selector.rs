//! Exact two-Dickson/two-Y7 branch of the coprime B677 selector.
//! The unknown quartic factors are variables; no witness enters construction.

use crate::b677_fixture::Hybrid677;
use crate::extractor::{public_polynomials, Builder, MPoly};
use prime31_algebraic_ic::Curve;

// Three RR5 parameters, four monic quadratic coefficients, and 18 modular
// multiplication gates with two remainder and one quotient variable each.
pub(crate) const VARIABLES: usize = 61;

struct Gates<'a> {
    b: &'a mut Builder,
    next: usize,
    constraints: Vec<MPoly>,
}

impl Gates<'_> {
    fn minus(&mut self, left: &MPoly, right: &MPoly) -> Result<MPoly, String> {
        let negative = self.b.scale(right, self.b.p - 1)?;
        self.b.add(left, &negative)
    }

    fn add_pair(&mut self, a: &[MPoly], b: &[MPoly]) -> Result<Vec<MPoly>, String> {
        (0..2).map(|j| self.b.add(&a[j], &b[j])).collect()
    }

    fn linear_pair(
        &mut self,
        constant: u64,
        a: &[MPoly],
        ca: u64,
        b: &[MPoly],
        cb: u64,
    ) -> Result<Vec<MPoly>, String> {
        let mut out = Vec::with_capacity(2);
        for j in 0..2 {
            let one = self.b.scale(&a[j], ca)?;
            let two = self.b.scale(&b[j], cb)?;
            out.push(self.b.add(&one, &two)?);
        }
        out[0] = self.b.add(&out[0], &self.b.c(constant))?;
        Ok(out)
    }

    // A*B = remainder + modulus*quotient, coefficient by coefficient.
    fn multiply(
        &mut self,
        left: &[MPoly],
        right: &[MPoly],
        modulus: &[MPoly],
    ) -> Result<Vec<MPoly>, String> {
        assert_eq!((left.len(), right.len(), modulus.len()), (2, 2, 3));
        let rem = vec![self.b.var(self.next), self.b.var(self.next + 1)];
        let quotient = self.b.var(self.next + 2);
        self.next += 3;
        let product = self.b.conv(left, right)?;
        for j in 0..3 {
            let mq = self.b.mul(&modulus[j], &quotient)?;
            let difference = self.minus(&product[j], &mq)?;
            let equation = if j < 2 {
                self.minus(&difference, &rem[j])?
            } else {
                difference
            };
            self.constraints.push(equation);
        }
        Ok(rem)
    }
}

pub(crate) fn build(
    builder: &mut Builder,
    e: Curve,
    target: (u64, u64),
    fixture: &Hybrid677,
) -> Result<(Vec<MPoly>, Vec<MPoly>), String> {
    assert_eq!(builder.variables, VARIABLES);
    let (u, _) = public_polynomials(builder, e, target, &[])?;
    let d = vec![builder.var(3), builder.var(4), builder.c(1)];
    let y = vec![builder.var(5), builder.var(6), builder.c(1)];
    let product = builder.conv(&d, &y)?;
    let mut gates = Gates {
        b: builder,
        next: 7,
        constraints: Vec::with_capacity(62),
    };
    for j in 0..4 {
        let eq = gates.minus(&product[j], &u[j])?;
        gates.constraints.push(eq);
    }
    assert_eq!(product[4], gates.b.c(1));

    // D_1024(X)-v in F_p[X]/(X^2+d1*X+d0).
    let mut dickson = vec![gates.b.c(0), gates.b.c(1)];
    for _ in 0..10 {
        let square = gates.multiply(&dickson, &dickson, &d)?;
        dickson = square;
        dickson[0] = gates.minus(&dickson[0], &gates.b.c(2))?;
    }
    let dickson_end = gates.minus(&dickson[0], &gates.b.c(fixture.terminal()))?;
    gates.constraints.push(dickson_end);
    gates.constraints.push(dickson[1].clone());

    // f(X)=X^3+A*X+B reduced directly modulo Y2=X^2+y1*X+y0.
    let y1y0 = gates.b.mul(&y[1], &y[0])?;
    let y1sq = gates.b.mul(&y[1], &y[1])?;
    let f0 = gates.b.add(&y1y0, &gates.b.c(e.b))?;
    let f1 = gates.minus(&y1sq, &y[0])?;
    let f1 = gates.b.add(&f1, &gates.b.c(e.a))?;
    let f = vec![f0, f1];
    let f2 = gates.multiply(&f, &f, &y)?;
    let f3 = gates.multiply(&f2, &f, &y)?;
    let f6 = gates.multiply(&f3, &f3, &y)?;
    let t = gates.multiply(&f6, &f, &y)?;

    let mut g = vec![1u64];
    for &root in fixture.coset_squares() {
        let mut next = vec![0; g.len() + 1];
        for (j, &coefficient) in g.iter().enumerate() {
            let negative = gates.b.mm(coefficient, (e.p - root) % e.p);
            next[j] = gates.b.am(next[j], negative);
            next[j + 1] = gates.b.am(next[j + 1], coefficient);
        }
        g = next;
    }
    assert_eq!(g.len(), 9);
    let t2 = gates.multiply(&t, &t, &y)?;
    let t3 = gates.multiply(&t2, &t, &y)?;
    let b0 = gates.linear_pair(g[0], &t, g[1], &t2, g[2])?;
    let b1 = gates.linear_pair(g[3], &t, g[4], &t2, g[5])?;
    let b2 = gates.linear_pair(g[6], &t, g[7], &t2, g[8])?;
    let high = gates.multiply(&b2, &t3, &y)?;
    let middle = gates.add_pair(&b1, &high)?;
    let high = gates.multiply(&middle, &t3, &y)?;
    let y7 = gates.add_pair(&b0, &high)?;
    gates.constraints.extend(y7);
    assert_eq!(gates.next, VARIABLES);
    assert_eq!(gates.constraints.len(), 62);
    Ok((u, gates.constraints))
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime31_algebraic_ic::PRIME31;
    use std::time::{Duration, Instant};

    fn eval(poly: &MPoly, assignment: &[u64]) -> u64 {
        poly.iter().fold(0, |sum, (exponents, coefficient)| {
            let mut term = *coefficient;
            for (&exponent, &value) in exponents.iter().zip(assignment) {
                for _ in 0..exponent {
                    term = ((term as u128 * value as u128) % PRIME31.p as u128) as u64;
                }
            }
            (sum + term) % PRIME31.p
        })
    }

    #[test]
    fn planted_two_two_assignment_satisfies_all_split_gates() {
        let fixture = Hybrid677::build().unwrap();
        let mut builder = Builder {
            p: PRIME31.p,
            variables: VARIABLES,
            deadline: Instant::now() + Duration::from_secs(5),
            term_cap: 20_000,
            adds: 0,
            muls: 0,
        };
        // These values are grader-side coordinates of the frozen positive
        // relation, never passed to the public-query candidate API.
        let (u, constraints) = build(
            &mut builder,
            PRIME31,
            (1_688_097_559, 1_788_262_902),
            &fixture,
        )
        .unwrap();
        let mut assignment = vec![0; VARIABLES];
        assignment[..7].copy_from_slice(&[
            181_466_281,
            1_281_079_767,
            1_031_559_641,
            8_451_459,
            2_144_666_491,
            100_032_994,
            2_132_514_801,
        ]);
        assert_eq!(
            u.iter()
                .map(|poly| eval(poly, &assignment))
                .collect::<Vec<_>>(),
            vec![
                1_237_803_639,
                1_108_181_908,
                1_693_913_937,
                2_129_697_645,
                1
            ]
        );
        for gate in 0..18 {
            let start = 4 + gate * 3 + if gate >= 10 { 2 } else { 0 };
            let variable = 7 + gate * 3;
            assignment[variable + 2] = eval(&constraints[start + 2], &assignment);
            assignment[variable] = eval(&constraints[start], &assignment);
            assignment[variable + 1] = eval(&constraints[start + 1], &assignment);
            let residuals: Vec<_> = constraints[start..start + 3]
                .iter()
                .map(|eq| eval(eq, &assignment))
                .collect();
            assert_eq!(residuals, vec![0, 0, 0], "gate {gate}");
        }
        assert!(constraints.iter().all(|eq| eval(eq, &assignment) == 0));
    }
}
