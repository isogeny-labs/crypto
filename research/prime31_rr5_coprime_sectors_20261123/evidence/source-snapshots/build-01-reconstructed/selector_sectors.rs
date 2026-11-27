//! Additive, construction-only coprime-selector presentation of the B677 RR5 chart.
//!
//! For k=0..4, U=A_k B_(4-k), A_k | (D_1024-tau), and
//! B_(4-k) | G(f^7). The admitted fixture makes the two selectors coprime.
//! The five sectors partition the original chart; one sector alone is a subset.
//! This module constructs quadratic equations. It neither invokes a solver nor
//! enumerates leaves, pairs, assignments, or target scalars. The existing frozen
//! extractor and its 122-variable presentation are unchanged.

use crate::b677_fixture::Hybrid677;
use crate::extractor::{public_polynomials, Builder, MPoly};
use crypto_lib::cryptanalysis::f4_fp::{normalise, Ordering, Poly};
use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use std::time::{Duration, Instant};

/// Exact inventories for this evaluation schedule, indexed by Dickson degree.
pub const SECTOR_VARIABLES: [usize; 5] = [59, 53, 56, 58, 59];
pub const SECTOR_EQUATIONS: [usize; 5] = [60, 54, 57, 59, 60];

#[derive(Clone, Copy, Debug)]
pub struct ConstructionLimits {
    pub seconds: u64,
    pub max_polynomial_terms: usize,
}

impl Default for ConstructionLimits {
    fn default() -> Self {
        Self {
            seconds: 30,
            max_polynomial_terms: 20_000,
        }
    }
}

/// Public equations ready for a separately admitted future solver.
///
/// Coordinates 0..3 are b,c,d. Coordinates 3..3+k are the ascending
/// nonleading coefficients of A_k; 3+k..7 are those of B_(4-k).
/// All subsequent coordinates are uniquely defined graph auxiliaries.
/// Equations use the original F4 `Poly` representation in grevlex order.
pub struct SectorSystem {
    pub dickson_degree: usize,
    pub variables: usize,
    pub equations: Vec<Poly>,
    pub quartic: Vec<Poly>,
    pub max_total_degree: u32,
    pub terms: usize,
    /// Symbolic constructor operations only; these are not solving costs.
    pub symbolic_additions: u64,
    pub symbolic_multiplications: u64,
    #[cfg(test)]
    steps: Vec<Step>,
}

#[cfg(test)]
enum Step {
    Lift {
        variable: usize,
        value: MPoly,
        equation: usize,
    },
    Product {
        modulus: Vec<MPoly>,
        left: Vec<MPoly>,
        right: Vec<MPoly>,
        remainder_start: Option<usize>,
        fixed_remainder: Option<Vec<MPoly>>,
        quotient_start: usize,
        equation_start: usize,
        terminal: Option<&'static str>,
    },
}

struct CircuitBuilder<'a> {
    b: &'a mut Builder,
    next: usize,
    constraints: Vec<MPoly>,
    #[cfg(test)]
    steps: Vec<Step>,
}

impl CircuitBuilder<'_> {
    fn allocate(&mut self, count: usize) -> Result<usize, String> {
        let start = self.next;
        self.next = self
            .next
            .checked_add(count)
            .filter(|&end| end <= self.b.variables)
            .ok_or_else(|| "sector variable inventory overflow".to_string())?;
        Ok(start)
    }

    fn lift(&mut self, value: MPoly) -> Result<MPoly, String> {
        let variable = self.allocate(1)?;
        let out = self.b.var(variable);
        let neg = self.b.scale(&value, self.b.p - 1)?;
        let equation = self.b.add(&out, &neg)?;
        #[cfg(test)]
        self.steps.push(Step::Lift {
            variable,
            value,
            equation: self.constraints.len(),
        });
        self.constraints.push(equation);
        Ok(out)
    }

    fn add(&mut self, a: &[MPoly], b: &[MPoly]) -> Result<Vec<MPoly>, String> {
        if a.len() != b.len() {
            return Err("sector residue shape mismatch".into());
        }
        a.iter().zip(b).map(|(x, y)| self.b.add(x, y)).collect()
    }

    fn constant_add(&mut self, a: &[MPoly], value: u64) -> Result<Vec<MPoly>, String> {
        if a.is_empty() {
            return Err("degree-zero residues are not admitted".into());
        }
        let mut out = a.to_vec();
        out[0] = self.b.add(&out[0], &self.b.c(value))?;
        Ok(out)
    }

    fn linear_combination(
        &mut self,
        constant: u64,
        a: &[MPoly],
        a_scalar: u64,
        b: &[MPoly],
        b_scalar: u64,
    ) -> Result<Vec<MPoly>, String> {
        if a.len() != b.len() {
            return Err("sector linear-combination shape mismatch".into());
        }
        let mut out = Vec::with_capacity(a.len());
        for (x, y) in a.iter().zip(b) {
            let x = self.b.scale(x, a_scalar)?;
            let y = self.b.scale(y, b_scalar)?;
            out.push(self.b.add(&x, &y)?);
        }
        self.constant_add(&out, constant)
    }

    /// All 2d-1 coefficient equations of left*right - modulus*Q - R.
    /// A supplied remainder is affine and receives no new output coordinates.
    /// The scalar case d=1 has no quotient and is handled without empty conv.
    fn multiply(
        &mut self,
        modulus: &[MPoly],
        left: &[MPoly],
        right: &[MPoly],
        fixed_remainder: Option<&[MPoly]>,
        _terminal: Option<&'static str>,
    ) -> Result<Vec<MPoly>, String> {
        let degree = modulus.len().saturating_sub(1);
        if !(1..=4).contains(&degree)
            || left.len() != degree
            || right.len() != degree
            || fixed_remainder.is_some_and(|value| value.len() != degree)
            || modulus[degree] != self.b.c(1)
        {
            return Err("invalid monic sector multiplication shape".into());
        }
        let remainder_start = if fixed_remainder.is_some() {
            None
        } else {
            Some(self.allocate(degree)?)
        };
        let remainder = fixed_remainder.map_or_else(
            || {
                let start = remainder_start.expect("new remainder start");
                (0..degree).map(|j| self.b.var(start + j)).collect()
            },
            <[MPoly]>::to_vec,
        );
        let quotient_start = self.allocate(degree - 1)?;
        let quotient: Vec<_> = (0..degree - 1)
            .map(|j| self.b.var(quotient_start + j))
            .collect();
        let mut equations = self.b.conv(left, right)?;
        let uq = if degree == 1 {
            vec![self.b.c(0)]
        } else {
            self.b.conv(modulus, &quotient)?
        };
        if equations.len() != 2 * degree - 1 || uq.len() != equations.len() {
            return Err("sector coefficient inventory mismatch".into());
        }
        for j in 0..equations.len() {
            let neg = self.b.scale(&uq[j], self.b.p - 1)?;
            equations[j] = self.b.add(&equations[j], &neg)?;
            if j < degree {
                let neg = self.b.scale(&remainder[j], self.b.p - 1)?;
                equations[j] = self.b.add(&equations[j], &neg)?;
            }
        }
        #[cfg(test)]
        self.steps.push(Step::Product {
            modulus: modulus.to_vec(),
            left: left.to_vec(),
            right: right.to_vec(),
            remainder_start,
            fixed_remainder: fixed_remainder.map(<[MPoly]>::to_vec),
            quotient_start,
            equation_start: self.constraints.len(),
            terminal: _terminal,
        });
        self.constraints.extend(equations);
        Ok(remainder)
    }

    fn dickson(&mut self, modulus: &[MPoly], terminal: u64) -> Result<(), String> {
        let degree = modulus.len() - 1;
        let p = self.b.p;
        let (mut residue, squares) = match degree {
            1 => (vec![self.b.scale(&modulus[0], p - 1)?], 10),
            2 => {
                let constant = self.b.scale(&modulus[0], p - 1)?;
                let constant = self.b.add(&constant, &self.b.c(p - 2))?;
                (vec![constant, self.b.scale(&modulus[1], p - 1)?], 9)
            }
            3 => (vec![self.b.c(p - 2), self.b.c(0), self.b.c(1)], 9),
            4 => {
                let mut residue: Vec<_> = modulus[..4]
                    .iter()
                    .map(|coefficient| self.b.scale(coefficient, p - 1))
                    .collect::<Result<_, _>>()?;
                residue[0] = self.b.add(&residue[0], &self.b.c(2))?;
                residue[2] = self.b.add(&residue[2], &self.b.c(p - 4))?;
                (residue, 8)
            }
            _ => return Err("invalid Dickson divisor degree".into()),
        };
        for step in 0..squares {
            if step + 1 == squares {
                let mut fixed = vec![self.b.c(0); degree];
                fixed[0] = self.b.c((terminal + 2) % p);
                self.multiply(modulus, &residue, &residue, Some(&fixed), Some("D"))?;
            } else {
                let square = self.multiply(modulus, &residue, &residue, None, None)?;
                residue = self.constant_add(&square, p - 2)?;
            }
        }
        Ok(())
    }

    fn curve_residue(&mut self, modulus: &[MPoly]) -> Result<Vec<MPoly>, String> {
        let degree = modulus.len() - 1;
        let e = PRIME31;
        match degree {
            1 => {
                // x=-v0: f(x)=-v0^3-A*v0+B. Two graph lifts keep degree two.
                let square = self.b.mul(&modulus[0], &modulus[0])?;
                let square = self.lift(square)?;
                let cube = self.b.mul(&modulus[0], &square)?;
                let cube = self.b.scale(&cube, e.p - 1)?;
                let linear = self.b.scale(&modulus[0], (e.p - e.a) % e.p)?;
                let value = self.b.add(&cube, &linear)?;
                let value = self.b.add(&value, &self.b.c(e.b))?;
                Ok(vec![self.lift(value)?])
            }
            2 => {
                // f mod (x^2+v1*x+v0)=(B+v0*v1)+(A-v0+v1^2)*x.
                let constant = self.b.mul(&modulus[0], &modulus[1])?;
                let constant = self.b.add(&constant, &self.b.c(e.b))?;
                let constant = self.lift(constant)?;
                let linear = self.b.mul(&modulus[1], &modulus[1])?;
                let neg = self.b.scale(&modulus[0], e.p - 1)?;
                let linear = self.b.add(&linear, &neg)?;
                let linear = self.b.add(&linear, &self.b.c(e.a))?;
                Ok(vec![constant, self.lift(linear)?])
            }
            3 => {
                let mut out: Vec<_> = modulus[..3]
                    .iter()
                    .map(|value| self.b.scale(value, e.p - 1))
                    .collect::<Result<_, _>>()?;
                out[0] = self.b.add(&out[0], &self.b.c(e.b))?;
                out[1] = self.b.add(&out[1], &self.b.c(e.a))?;
                Ok(out)
            }
            4 => Ok(vec![self.b.c(e.b), self.b.c(e.a), self.b.c(0), self.b.c(1)]),
            _ => Err("invalid Y7 divisor degree".into()),
        }
    }

    fn y7(&mut self, modulus: &[MPoly], fixture: &Hybrid677) -> Result<(), String> {
        let f = self.curve_residue(modulus)?;
        let f2 = self.multiply(modulus, &f, &f, None, None)?;
        let f3 = self.multiply(modulus, &f2, &f, None, None)?;
        let f6 = self.multiply(modulus, &f3, &f3, None, None)?;
        let t = self.multiply(modulus, &f6, &f, None, None)?;

        let mut g = vec![1];
        for &root in fixture.coset_squares() {
            let mut next = vec![0; g.len() + 1];
            for (j, &coefficient) in g.iter().enumerate() {
                let neg = self.b.mm(coefficient, (self.b.p - root) % self.b.p);
                next[j] = self.b.am(next[j], neg);
                next[j + 1] = self.b.am(next[j + 1], coefficient);
            }
            g = next;
        }
        let t2 = self.multiply(modulus, &t, &t, None, None)?;
        let t3 = self.multiply(modulus, &t2, &t, None, None)?;
        let b0 = self.linear_combination(g[0], &t, g[1], &t2, g[2])?;
        let b1 = self.linear_combination(g[3], &t, g[4], &t2, g[5])?;
        let b2 = self.linear_combination(g[6], &t, g[7], &t2, g[8])?;
        let high = self.multiply(modulus, &b2, &t3, None, None)?;
        let middle = self.add(&b1, &high)?;
        let fixed: Vec<_> = b0
            .iter()
            .map(|value| self.b.scale(value, self.b.p - 1))
            .collect::<Result<_, _>>()?;
        self.multiply(modulus, &middle, &t3, Some(&fixed), Some("Y"))?;
        Ok(())
    }
}

/// Construct one exact degree sector over the fixed admitted M31 fixture.
///
/// A caller establishing full extraction completeness must cover all five
/// sectors. This function supplies equations only; no completeness or solving
/// performance result follows from its return value.
pub fn construct_hybrid677_sector(
    fixture: &Hybrid677,
    target: Point,
    dickson_degree: usize,
    limits: ConstructionLimits,
) -> Result<SectorSystem, String> {
    let Some(&variables) = SECTOR_VARIABLES.get(dickson_degree) else {
        return Err("Dickson degree must be in 0..=4".into());
    };
    if limits.seconds == 0 || limits.max_polynomial_terms == 0 {
        return Err("nonzero construction limits are required".into());
    }
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(limits.seconds))
        .ok_or_else(|| "construction deadline overflow".to_string())?;
    let Some(point) = target else {
        return Err("identity target chart is not supported".into());
    };
    let mut arithmetic = Arithmetic::default();
    if !arithmetic.on_curve(PRIME31, target) {
        return Err("invalid M31 public target".into());
    }
    if fixture
        .points()
        .binary_search_by_key(&point.0, |entry| entry.0)
        .is_ok()
    {
        return Err("target-in-base chart is not supported".into());
    }
    let mut builder = Builder {
        p: PRIME31.p,
        variables,
        deadline,
        term_cap: limits.max_polynomial_terms,
        adds: 0,
        muls: 0,
    };
    let (u, _) = public_polynomials(&mut builder, PRIME31, point, &[])?;
    let mut a: Vec<_> = (0..dickson_degree).map(|j| builder.var(3 + j)).collect();
    a.push(builder.c(1));
    let mut b: Vec<_> = (dickson_degree..4)
        .map(|j| builder.var(3 + j))
        .collect();
    b.push(builder.c(1));
    let product = builder.conv(&a, &b)?;
    let mut constraints = Vec::with_capacity(SECTOR_EQUATIONS[dickson_degree]);
    for j in 0..4 {
        let neg = builder.scale(&u[j], PRIME31.p - 1)?;
        constraints.push(builder.add(&product[j], &neg)?);
    }
    let mut circuit = CircuitBuilder {
        b: &mut builder,
        next: 7,
        constraints,
        #[cfg(test)]
        steps: Vec::new(),
    };
    if dickson_degree != 0 {
        circuit.dickson(&a, fixture.terminal())?;
    }
    if dickson_degree != 4 {
        circuit.y7(&b, fixture)?;
    }
    if circuit.next != variables
        || circuit.constraints.len() != SECTOR_EQUATIONS[dickson_degree]
    {
        return Err("sector construction inventory differs from its contract".into());
    }
    let max_total_degree = circuit
        .constraints
        .iter()
        .flat_map(|f| f.keys())
        .map(|exponents| exponents.iter().sum::<u32>())
        .max()
        .unwrap_or(0);
    if max_total_degree > 2 {
        return Err("sector construction exceeded quadratic input degree".into());
    }
    let terms = circuit.constraints.iter().map(MPoly::len).sum();
    let convert = |f: &MPoly| {
        normalise(
            &f.iter().map(|(powers, &coefficient)| (powers.clone(), coefficient)).collect(),
            PRIME31.p,
            Ordering::Grevlex,
        )
    };
    let equations = circuit.constraints.iter().map(convert).collect();
    let quartic = u.iter().map(convert).collect();
    #[cfg(test)]
    let steps = std::mem::take(&mut circuit.steps);
    drop(circuit);
    if Instant::now() >= deadline {
        return Err("construction deadline".into());
    }
    Ok(SectorSystem {
        dickson_degree,
        variables,
        equations,
        quartic,
        max_total_degree,
        terms,
        symbolic_additions: builder.adds,
        symbolic_multiplications: builder.muls,
        #[cfg(test)]
        steps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: u64 = 2_147_483_647;

    fn mul(a: u64, b: u64) -> u64 {
        ((a as u128 * b as u128) % P as u128) as u64
    }
    fn power(mut a: u64, mut exponent: u64) -> u64 {
        let mut out = 1;
        while exponent != 0 {
            if exponent & 1 != 0 {
                out = mul(out, a);
            }
            a = mul(a, a);
            exponent >>= 1;
        }
        out
    }
    fn polynomial_product(a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] = (out[i + j] + mul(x, y)) % P;
            }
        }
        out
    }
    fn divide(a: &[u64], modulus: &[u64]) -> (Vec<u64>, Vec<u64>) {
        assert_eq!(modulus.last(), Some(&1));
        let degree = modulus.len() - 1;
        assert!(degree > 0);
        let mut out = a.to_vec();
        out.resize(out.len().max(degree), 0);
        let mut quotient = vec![0; out.len().saturating_sub(degree)];
        for j in (degree..out.len()).rev() {
            let coefficient = out[j];
            quotient[j - degree] = coefficient;
            for i in 0..=degree {
                out[j - degree + i] =
                    (out[j - degree + i] + P - mul(coefficient, modulus[i])) % P;
            }
        }
        (quotient, out[..degree].to_vec())
    }
    fn eval_terms<'a>(
        terms: impl Iterator<Item = (&'a Vec<u32>, &'a u64)>,
        assignment: &[u64],
    ) -> u64 {
        terms.fold(0, |sum, (exponents, &coefficient)| {
            assert_eq!(exponents.len(), assignment.len(), "full assignment dimension");
            let value = exponents.iter().zip(assignment).fold(coefficient, |v, (&e, &x)| {
                mul(v, power(x, e as u64))
            });
            (sum + value) % P
        })
    }
    fn eval_mpoly(f: &MPoly, assignment: &[u64]) -> u64 {
        eval_terms(f.iter(), assignment)
    }
    fn eval_poly(f: &Poly, assignment: &[u64]) -> u64 {
        eval_terms(f.iter().map(|(powers, value)| (powers, value)), assignment)
    }
    fn scalar_quartic(target: (u64, u64), parameters: [u64; 3]) -> Vec<u64> {
        let (r, s) = target;
        let [b, c, d] = parameters;
        let v = vec![b, 1];
        let l = vec![d, c];
        let dr = vec![(mul(r, r) + PRIME31.a) % P, r, 1];
        let mut out = polynomial_product(&dr, &polynomial_product(&v, &v));
        let negative = polynomial_product(&[P - r, 1], &polynomial_product(&l, &l));
        for (j, value) in negative.into_iter().enumerate() {
            out[j] = (out[j] + P - value) % P;
        }
        for (j, value) in polynomial_product(&l, &v).into_iter().enumerate() {
            out[j] = (out[j] + P - mul(2 * s % P, value)) % P;
        }
        out
    }
    fn dense_selectors(fixture: &Hybrid677) -> (Vec<u64>, Vec<u64>) {
        let mut d = vec![0, 1];
        for _ in 0..10 {
            d = polynomial_product(&d, &d);
            d[0] = (d[0] + P - 2) % P;
        }
        d[0] = (d[0] + P - fixture.terminal()) % P;
        let f = vec![PRIME31.b, PRIME31.a, 0, 1];
        let mut f7 = vec![1];
        // Independent sevenfold product, not the production addition chain.
        for _ in 0..7 {
            f7 = polynomial_product(&f7, &f);
        }
        let mut y = vec![1];
        for &root in fixture.coset_squares() {
            let mut factor = f7.clone();
            factor[0] = (factor[0] + P - root) % P;
            y = polynomial_product(&y, &factor);
        }
        assert_eq!((d.len(), y.len()), (1025, 169));
        (d, y)
    }
    fn fill_graph(
        system: &SectorSystem,
        assignment: &mut [u64],
        dense_d: &[u64],
        dense_y: &[u64],
    ) -> usize {
        let mut terminals = 0;
        for step in &system.steps {
            match step {
                Step::Lift { variable, value, equation } => {
                    assignment[*variable] = eval_mpoly(value, assignment);
                    assert_eq!(eval_poly(&system.equations[*equation], assignment), 0);
                }
                Step::Product {
                    modulus,
                    left,
                    right,
                    remainder_start,
                    fixed_remainder,
                    quotient_start,
                    equation_start,
                    terminal,
                } => {
                    let evaluate = |polys: &[MPoly]| -> Vec<u64> {
                        polys.iter().map(|f| eval_mpoly(f, assignment)).collect()
                    };
                    let modulus = evaluate(modulus);
                    let left = evaluate(left);
                    let right = evaluate(right);
                    let fixed = fixed_remainder.as_deref().map(evaluate);
                    let degree = modulus.len() - 1;
                    let (quotient, remainder) = divide(&polynomial_product(&left, &right), &modulus);
                    assert_eq!(quotient.len(), degree - 1);
                    assignment[*quotient_start..*quotient_start + quotient.len()]
                        .copy_from_slice(&quotient);
                    if let Some(start) = remainder_start {
                        assignment[*start..*start + degree].copy_from_slice(&remainder);
                    }
                    let observed: Vec<_> = system.equations
                        [*equation_start..*equation_start + 2 * degree - 1]
                        .iter()
                        .map(|f| eval_poly(f, assignment))
                        .collect();
                    assert!(observed[degree..].iter().all(|&value| value == 0));
                    if let Some(which) = terminal {
                        terminals += 1;
                        let selector = if *which == "D" { dense_d } else { dense_y };
                        let (_, expected) = divide(selector, &modulus);
                        assert_eq!(&observed[..degree], expected);
                        let fixed = fixed.as_ref().expect("terminal fixed remainder");
                        let direct: Vec<_> = remainder.iter().zip(fixed)
                            .map(|(&value, &fixed)| (value + P - fixed) % P).collect();
                        assert_eq!(direct, expected);
                    } else {
                        assert!(observed.iter().all(|&value| value == 0));
                    }
                }
            }
        }
        terminals
    }
    fn from_roots(roots: &[u64]) -> Vec<u64> {
        roots.iter().fold(vec![1], |polynomial, &x| {
            polynomial_product(&polynomial, &[(P - x) % P, 1])
        })
    }
    fn on_curve(point: (u64, u64)) -> bool {
        let (x, y) = point;
        x < P && y < P && mul(y, y) == (mul(mul(x, x), x) + mul(PRIME31.a, x) + PRIME31.b) % P
    }
    fn add(left: Point, right: Point) -> Point {
        let (Some((x1, y1)), Some((x2, y2))) = (left, right) else {
            return left.or(right);
        };
        if x1 == x2 && (y1 + y2) % P == 0 {
            return None;
        }
        let (numerator, denominator) = if x1 == x2 {
            assert_eq!(y1, y2);
            ((3 * mul(x1, x1) + PRIME31.a) % P, 2 * y1 % P)
        } else {
            ((y2 + P - y1) % P, (x2 + P - x1) % P)
        };
        assert_ne!(denominator, 0);
        let slope = mul(numerator, power(denominator, P - 2));
        let x = (mul(slope, slope) + 2 * P - x1 - x2) % P;
        let y = (mul(slope, (x1 + P - x) % P) + P - y1) % P;
        assert!(on_curve((x, y)));
        Some((x, y))
    }
    fn interpolate(target: (u64, u64), leaves: &[(u64, u64)]) -> [u64; 3] {
        let (r, s) = target;
        let mut matrix: Vec<[u64; 4]> = leaves.iter().map(|&(x, y)| {
            let delta = (x + P - r) % P;
            let ordinate = (s + y) % P;
            [ordinate, mul(x, delta), delta, (P - mul(x, ordinate)) % P]
        }).collect();
        // Use all four rows: a particular three-row minor need not be invertible.
        for column in 0..3 {
            let pivot = (column..4).find(|&row| matrix[row][column] != 0)
                .expect("RR5 interpolation rank must be three");
            matrix.swap(column, pivot);
            let inverse = power(matrix[column][column], P - 2);
            for value in &mut matrix[column] {
                *value = mul(*value, inverse);
            }
            for row in 0..4 {
                if row == column { continue; }
                let scale = matrix[row][column];
                for j in 0..4 {
                    matrix[row][j] = (matrix[row][j] + P - mul(scale, matrix[column][j])) % P;
                }
            }
        }
        assert_eq!(matrix[3], [0; 4], "fourth RR5 equation must be consistent");
        [matrix[0][3], matrix[1][3], matrix[2][3]]
    }

    #[test]
    fn all_five_m31_sectors_have_exact_dense_residuals() {
        let fixture = Hybrid677::build().unwrap();
        let (d, y) = dense_selectors(&fixture);
        let target = (18, 221_278_090);
        let cases = [([0, 0, 0], [0, 0, 0, 0]), ([1, 1, 1], [1, 1, 1, 1]),
            ([P - 1; 3], [P - 1; 4]), ([17, 31, 43], [47, 61, 73, 89])];
        let mut terminal_checks = 0;
        for k in 0..=4 {
            let system = construct_hybrid677_sector(&fixture, Some(target), k, ConstructionLimits::default()).unwrap();
            assert_eq!(system.variables, SECTOR_VARIABLES[k]);
            assert_eq!(system.equations.len(), SECTOR_EQUATIONS[k]);
            assert_eq!(system.max_total_degree, 2);
            for (parameters, factors) in cases {
                let mut assignment = vec![0; system.variables];
                assignment[..3].copy_from_slice(&parameters);
                assignment[3..7].copy_from_slice(&factors);
                let mut a = factors[..k].to_vec(); a.push(1);
                let mut b = factors[k..].to_vec(); b.push(1);
                let product = polynomial_product(&a, &b);
                let u = scalar_quartic(target, parameters);
                for j in 0..5 {
                    assert_eq!(eval_poly(&system.quartic[j], &assignment), u[j]);
                    if j < 4 {
                        assert_eq!(eval_poly(&system.equations[j], &assignment), (product[j] + P - u[j]) % P);
                    }
                }
                terminal_checks += fill_graph(&system, &mut assignment, &d, &y);
            }
        }
        assert_eq!(terminal_checks, 32);
        println!("B677_SECTOR_RESIDUALS {}", serde_json::json!({"field": P, "sectors": 5, "factor_parameter_controls": 20, "terminal_remainders": terminal_checks}));
    }

    #[test]
    fn all_five_m31_planted_sectors_replay_and_reject_auxiliary_mutations() {
        let fixture = Hybrid677::build().unwrap();
        let (dense_d, dense_y) = dense_selectors(&fixture);
        let (dickson, extras): (Vec<_>, Vec<_>) = fixture.points().iter().copied().partition(|&(x, _)| {
            let mut value = x;
            for _ in 0..10 { value = (mul(value, value) + P - 2) % P; }
            value == fixture.terminal()
        });
        assert_eq!((dickson.len(), extras.len()), (509, 168));
        let mut mutations = 0;
        let mut wrong_partitions = 0;
        for k in 0..=4 {
            // One frozen known-answer constructor fixture per sector. No search.
            let mut leaves: Vec<_> = dickson[..k].iter().chain(&extras[..4-k]).copied().collect();
            leaves[1].1 = P - leaves[1].1; // The same (+,-,+,+) signs in every sector.
            let target = leaves.iter().fold(None, |sum, &point| add(sum, Some(point)))
                .expect("fixed planted target must be finite");
            assert!(fixture.points().binary_search_by_key(&target.0, |entry| entry.0).is_err(), "fixed planted target must be outside the base");
            let parameters = interpolate(target, &leaves);
            let a = from_roots(&leaves[..k].iter().map(|point| point.0).collect::<Vec<_>>());
            let b = from_roots(&leaves[k..].iter().map(|point| point.0).collect::<Vec<_>>());
            assert_eq!(polynomial_product(&a, &b), scalar_quartic(target, parameters));
            let system = construct_hybrid677_sector(&fixture, Some(target), k, ConstructionLimits::default()).unwrap();
            let mut assignment = vec![0; system.variables];
            assignment[..3].copy_from_slice(&parameters);
            assignment[3..3+k].copy_from_slice(&a[..k]);
            assignment[3+k..7].copy_from_slice(&b[..4-k]);
            fill_graph(&system, &mut assignment, &dense_d, &dense_y);
            assert!(system.equations.iter().all(|f| eval_poly(f, &assignment) == 0));
            let before = mutations;
            for variable in 3..system.variables {
                let mut poisoned = assignment.clone();
                poisoned[variable] = (poisoned[variable] + 1) % P;
                assert!(system.equations.iter().any(|f| eval_poly(f, &poisoned) != 0), "k={k}, auxiliary={variable} escaped its equations");
                mutations += 1;
            }
            if (1..4).contains(&k) {
                // Preserve the quartic, signed leaves and target, but give each
                // divisor a root belonging to the other coprime selector.
                let mut wrong_leaves = leaves.clone();
                wrong_leaves.swap(k - 1, k);
                let wrong_a = from_roots(&wrong_leaves[..k].iter().map(|point| point.0).collect::<Vec<_>>());
                let wrong_b = from_roots(&wrong_leaves[k..].iter().map(|point| point.0).collect::<Vec<_>>());
                assert_eq!(polynomial_product(&wrong_a, &wrong_b), scalar_quartic(target, parameters));
                let mut wrong = assignment.clone();
                wrong[3..3+k].copy_from_slice(&wrong_a[..k]);
                wrong[3+k..7].copy_from_slice(&wrong_b[..4-k]);
                fill_graph(&system, &mut wrong, &dense_d, &dense_y);
                assert!(system.equations[..4].iter().all(|f| eval_poly(f, &wrong) == 0));
                assert!(system.equations.iter().any(|f| eval_poly(f, &wrong) != 0));
                assert!(divide(&dense_d, &wrong_a).1.iter().any(|&value| value != 0));
                assert!(divide(&dense_y, &wrong_b).1.iter().any(|&value| value != 0));
                wrong_partitions += 1;
            }
            println!("B677_SECTOR_CONSTRUCTION {}", serde_json::json!({"field": P, "dickson_degree": k, "y7_degree": 4-k, "variables": system.variables, "equations": system.equations.len(), "terms": system.terms, "max_total_degree": system.max_total_degree, "target": target, "accepted_known_assignments": 1, "rejected_auxiliary_mutations": mutations-before, "symbolic_additions": system.symbolic_additions, "symbolic_multiplications": system.symbolic_multiplications}));
        }
        assert_eq!(mutations, 270);
        assert_eq!(wrong_partitions, 3);
    }

    #[test]
    fn sector_constructor_rejects_invalid_chart_limits_and_degree() {
        let fixture = Hybrid677::build().unwrap();
        let valid = Some((18, 221_278_090));
        let defaults = ConstructionLimits::default();
        for k in 0..=4 {
            assert!(construct_hybrid677_sector(&fixture, None, k, defaults).is_err());
            assert!(construct_hybrid677_sector(&fixture, Some((P, 0)), k, defaults).is_err());
            assert!(construct_hybrid677_sector(&fixture, Some((18, 221_278_090 + P)), k, defaults).is_err());
            assert!(construct_hybrid677_sector(&fixture, Some((18 + P, 221_278_090)), k, defaults).is_err());
            assert!(construct_hybrid677_sector(&fixture, Some(fixture.points()[0]), k, defaults).is_err());
        }
        assert!(construct_hybrid677_sector(&fixture, valid, 5, defaults).is_err());
        assert!(construct_hybrid677_sector(&fixture, valid, usize::MAX, defaults).is_err());
        assert!(construct_hybrid677_sector(&fixture, valid, 2, ConstructionLimits { seconds: 0, ..defaults }).is_err());
        assert!(matches!(construct_hybrid677_sector(&fixture, valid, 2, ConstructionLimits { seconds: u64::MAX, ..defaults }), Err(ref reason) if reason == "construction deadline overflow"));
        assert!(construct_hybrid677_sector(&fixture, valid, 2, ConstructionLimits { max_polynomial_terms: 0, ..defaults }).is_err());
        let stopped = construct_hybrid677_sector(&fixture, valid, 2, ConstructionLimits { max_polynomial_terms: 1, ..defaults });
        assert!(matches!(stopped, Err(ref reason) if reason == "symbolic term cap"));
    }
}
