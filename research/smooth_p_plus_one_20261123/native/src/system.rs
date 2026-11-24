use std::collections::BTreeMap;
use std::ops::{Add, Mul, Sub};

use crate::{Fixture, Point};

#[derive(Clone, Debug)]
struct Polynomial {
    terms: BTreeMap<Vec<u8>, i64>,
    p: i64,
    width: usize,
}

impl Polynomial {
    fn constant(&self, value: i64) -> Self {
        let mut result = Self::zero(self.p, self.width);
        result.accumulate(vec![0; self.width], value);
        result
    }

    fn zero(p: i64, width: usize) -> Self {
        Self {
            terms: BTreeMap::new(),
            p,
            width,
        }
    }

    fn variable(p: i64, width: usize, index: usize) -> Self {
        let mut result = Self::zero(p, width);
        let mut exponent = vec![0; width];
        exponent[index] = 1;
        result.accumulate(exponent, 1);
        result
    }

    fn accumulate(&mut self, exponent: Vec<u8>, value: i64) {
        let value = (self.terms.get(&exponent).copied().unwrap_or(0) + value).rem_euclid(self.p);
        if value == 0 {
            self.terms.remove(&exponent);
        } else {
            self.terms.insert(exponent, value);
        }
    }

    fn scale(&self, factor: i64) -> Self {
        let mut result = Self::zero(self.p, self.width);
        for (exponent, coefficient) in &self.terms {
            result.accumulate(exponent.clone(), coefficient * factor);
        }
        result
    }

    fn square(&self) -> Self {
        self * self
    }

    fn render(&self, names: &[String]) -> String {
        self.terms
            .iter()
            .rev()
            .map(|(powers, coefficient)| {
                let mut term = coefficient.to_string();
                for (power, name) in powers.iter().zip(names) {
                    if *power != 0 {
                        term.push('*');
                        term.push_str(name);
                        if *power != 1 {
                            term.push('^');
                            term.push_str(&power.to_string());
                        }
                    }
                }
                term
            })
            .collect::<Vec<_>>()
            .join("+")
    }

    #[cfg(test)]
    fn evaluate(&self, assignment: &[i64]) -> i64 {
        self.terms.iter().fold(0, |total, (powers, coefficient)| {
            let mut result = *coefficient;
            for (power, value) in powers.iter().zip(assignment) {
                for _ in 0..*power {
                    result = (result * value).rem_euclid(self.p);
                }
            }
            (total + result).rem_euclid(self.p)
        })
    }
}

impl Add for &Polynomial {
    type Output = Polynomial;
    fn add(self, right: Self) -> Polynomial {
        let mut result = self.clone();
        for (exponent, coefficient) in &right.terms {
            result.accumulate(exponent.clone(), *coefficient);
        }
        result
    }
}

impl Sub for &Polynomial {
    type Output = Polynomial;
    fn sub(self, right: Self) -> Polynomial {
        self + &right.scale(-1)
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;
    fn mul(self, right: Self) -> Polynomial {
        let mut result = Polynomial::zero(self.p, self.width);
        for (left_exponent, left_coefficient) in &self.terms {
            for (right_exponent, right_coefficient) in &right.terms {
                let exponent = left_exponent
                    .iter()
                    .zip(right_exponent)
                    .map(|(i, j)| {
                        i.checked_add(*j)
                            .expect("polynomial degree exceeds study limit")
                    })
                    .collect();
                result.accumulate(exponent, left_coefficient * right_coefficient);
            }
        }
        result
    }
}

fn quartic(
    curve: Fixture,
    target: Point,
    a0: &Polynomial,
    a1: &Polynomial,
    t: &Polynomial,
) -> [Polynomial; 4] {
    let u = target.x;
    let v = -target.y;
    let f = |value| a0.constant(value);
    let z1 = &(&f(v) - a0) + &a1.scale(u);
    let z0 = &t.scale(v) + &a0.scale(u);
    let d1 = &(&(&z1 * &z0).scale(2) - &t.square().scale(curve.a)) - &t.scale(2 * curve.b);
    let d2 = &(&(&z1.square() - &(a1 * &z0).scale(2)) - &t.scale(2 * curve.a)) - &f(curve.b);
    let d3 = &(&(a1 * &z1).scale(-2) - &t.square()) - &f(curve.a);
    let d4 = &a1.square() - &t.scale(2);
    let q3 = &f(u) - &d4;
    let q2 = &q3.scale(u) - &d3;
    let q1 = &q2.scale(u) - &d2;
    let q0 = &q1.scale(u) - &d1;
    [q0, q1, q2, q3]
}

fn advance([c0, c1, c2, c3]: &[Polynomial; 4]) -> [Polynomial; 4] {
    let d0 = c0.square();
    let d1 = &(c0 * c2).scale(2) - &c1.square();
    let d2 = &(&c2.square() + &c0.scale(2)) - &(c1 * c3).scale(2);
    let d3 = &c2.scale(2) - &c3.square();
    let add_constant = |x: Polynomial, c| &x + &c0.constant(c);
    [
        add_constant(&(&(&d0 + &d1.scale(2)) + &d2.scale(4)) + &d3.scale(8), 16),
        add_constant(&(&d1 + &d2.scale(4)) + &d3.scale(12), 32),
        add_constant(&d2 + &d3.scale(6), 24),
        add_constant(d3, 8),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Distinct,
    Expanded,
    Quadratic,
}

pub struct System {
    names: Vec<String>,
    equations: Vec<Polynomial>,
}

impl System {
    pub fn at_cutoff(curve: Fixture, target: Point, levels: usize, expected: [i64; 4]) -> Self {
        assert!(levels <= 4);
        let mut names = vec![];
        for step in (1..levels).rev() {
            names.extend((0..4).map(|i| format!("c{step}_{i}")));
        }
        names.extend(["a0", "a1", "t0"].map(str::to_owned));
        let variables: Vec<_> = (0..names.len())
            .map(|i| Polynomial::variable(curve.p, names.len(), i))
            .collect();
        let variable =
            |name: &str| &variables[names.iter().position(|known| known == name).unwrap()];
        let a0 = variable("a0");
        let mut state = quartic(curve, target, a0, variable("a1"), variable("t0"));
        let mut equations = vec![];
        for step in 1..=levels {
            let advanced = advance(&state);
            if step == levels {
                state = advanced;
            } else {
                state = std::array::from_fn(|i| variable(&format!("c{step}_{i}")).clone());
                equations.extend(advanced.iter().zip(&state).map(|(lhs, rhs)| lhs - rhs));
            }
        }
        equations.extend(
            state
                .iter()
                .zip(expected)
                .map(|(lhs, rhs)| lhs - &a0.constant(rhs)),
        );
        equations.retain(|polynomial| !polynomial.terms.is_empty());
        Self { names, equations }
    }

    pub fn build(curve: Fixture, target: Point, depth: usize, terminal: i64, mode: Mode) -> Self {
        assert!((2..=6).contains(&depth));
        let levels = depth - 2;
        if mode == Mode::Distinct {
            return Self::at_cutoff(curve, target, levels, [2 - terminal, 0, -4, 0]);
        }
        let mut names = vec![];
        match mode {
            Mode::Distinct => unreachable!("handled by the cutoff constructor"),
            Mode::Expanded => assert!(depth <= 4, "expanded control is limited to degree sixteen"),
            Mode::Quadratic => {
                for step in (0..levels).rev() {
                    if step + 1 < levels {
                        names.extend((0..4).map(|i| format!("r{step}_{i}")));
                    }
                    names.extend((4..=6).map(|i| format!("s{step}_{i}")));
                }
                names.extend((0..4).map(|i| format!("q{i}")));
            }
        }
        names.extend(["a0", "a1", "t0"].map(str::to_owned));
        let variables: Vec<_> = (0..names.len())
            .map(|i| Polynomial::variable(curve.p, names.len(), i))
            .collect();
        let variable =
            |name: &str| &variables[names.iter().position(|known| known == name).unwrap()];
        let a0 = variable("a0");
        let q = quartic(curve, target, a0, variable("a1"), variable("t0"));
        let f = |value| a0.constant(value);
        let mut equations = vec![];
        match mode {
            Mode::Distinct => unreachable!("handled by the cutoff constructor"),
            Mode::Expanded => {
                let mut selector = vec![0, 1];
                for _ in 0..depth {
                    selector = crate::polynomial::product(&selector, &selector, curve.p);
                    selector[0] = (selector[0] - 2).rem_euclid(curve.p);
                }
                selector[0] -= terminal;
                let mut remainder: Vec<_> = selector.into_iter().map(f).collect();
                for i in (4..remainder.len()).rev() {
                    for j in 0..4 {
                        remainder[i - 4 + j] = &remainder[i - 4 + j] - &(&remainder[i] * &q[j]);
                    }
                }
                equations.extend(remainder.into_iter().take(4));
            }
            Mode::Quadratic => {
                let modulus: [Polynomial; 4] =
                    std::array::from_fn(|i| variable(&format!("q{i}")).clone());
                equations.extend(q.iter().zip(&modulus).map(|(lhs, rhs)| lhs - rhs));
                let mut state: [Polynomial; 4] =
                    std::array::from_fn(|i| &f([2, 0, -4, 0][i]) - &modulus[i]);
                if levels == 0 {
                    state[0] = &state[0] - &f(terminal);
                    equations.extend(state.iter().cloned());
                }
                for step in 0..levels {
                    let mut conv: [Polynomial; 7] = std::array::from_fn(|_| f(0));
                    for (i, left) in state.iter().enumerate() {
                        for (j, right) in state.iter().enumerate() {
                            conv[i + j] = &conv[i + j] + &(left * right);
                        }
                    }
                    let reduction: [&Polynomial; 3] =
                        std::array::from_fn(|i| variable(&format!("s{step}_{}", 4 + i)));
                    for k in (4..=6).rev() {
                        let mut coefficient = &conv[k] - reduction[k - 4];
                        for higher in k + 1..=6 {
                            coefficient =
                                &coefficient - &(reduction[higher - 4] * &modulus[k + 4 - higher]);
                        }
                        equations.push(coefficient);
                    }
                    let next: [Polynomial; 4] = std::array::from_fn(|i| {
                        let mut coefficient = conv[i].clone();
                        for k in 4..=std::cmp::min(6, i + 4) {
                            coefficient = &coefficient - &(reduction[k - 4] * &modulus[i + 4 - k]);
                        }
                        if i == 0 {
                            coefficient = &coefficient - &f(2);
                        }
                        coefficient
                    });
                    if step + 1 == levels {
                        equations.extend(
                            next.iter()
                                .enumerate()
                                .map(|(i, value)| value - &f(if i == 0 { terminal } else { 0 })),
                        );
                    } else {
                        state = std::array::from_fn(|i| variable(&format!("r{step}_{i}")).clone());
                        equations.extend(next.iter().zip(&state).map(|(lhs, rhs)| lhs - rhs));
                    }
                }
            }
        }
        equations.retain(|polynomial| !polynomial.terms.is_empty());
        Self { names, equations }
    }

    pub fn render(&self, p: i64) -> String {
        let equations = self
            .equations
            .iter()
            .map(|p| p.render(&self.names))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("{}\n{}\n{}\n", self.names.join(","), p, equations)
    }

    pub fn metrics(&self) -> (usize, usize, usize, u32) {
        let terms = self
            .equations
            .iter()
            .map(|equation| equation.terms.len())
            .sum();
        let degree = self
            .equations
            .iter()
            .flat_map(|eq| eq.terms.keys())
            .map(|monomial| monomial.iter().map(|e| u32::from(*e)).sum())
            .max()
            .unwrap_or(0);
        (self.names.len(), self.equations.len(), terms, degree)
    }
}

pub fn numeric_quartic(curve: Fixture, target: Point, alpha: i64, beta: i64, t: i64) -> Vec<i64> {
    let base = Polynomial::zero(curve.p, 0);
    let mut result: Vec<_> = quartic(
        curve,
        target,
        &base.constant(alpha),
        &base.constant(beta),
        &base.constant(t),
    )
    .iter()
    .map(|value| value.terms.values().next().copied().unwrap_or(0))
    .collect();
    result.push(1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_control_is_quadratic() {
        let curve = crate::FIXTURES[0];
        let target = Point { x: 20, y: 91 };
        for depth in 2..=6 {
            let system = System::build(curve, target, depth, 2, Mode::Quadratic);
            assert_eq!(system.metrics().3, 2);
            assert!(system
                .equations
                .iter()
                .any(|eq| eq.evaluate(&vec![0; system.names.len()]) != 0));
        }
    }

    #[test]
    fn quadratic_ladder_agrees_with_expanded_remainder() {
        let curve = crate::FIXTURES[0];
        let target = Point { x: 20, y: 91 };
        let p = curve.p;
        let terminal = 29;
        for [alpha, beta, t] in [[13, 19, 27], [3, 0, 0], [2, 60, 123]] {
            let q = numeric_quartic(curve, target, alpha, beta, t);
            for depth in 2..=4 {
                let compact = System::build(curve, target, depth, terminal, Mode::Quadratic);
                let mut values =
                    BTreeMap::from([("a0".into(), alpha), ("a1".into(), beta), ("t0".into(), t)]);
                for (i, value) in q.iter().take(4).enumerate() {
                    values.insert(format!("q{i}"), *value);
                }
                let mut remainder: Vec<_> = [2, 0, -4, 0]
                    .iter()
                    .zip(&q)
                    .map(|(coefficient, subtract)| (coefficient - subtract).rem_euclid(p))
                    .collect();
                for step in 0..depth - 2 {
                    let mut squared = crate::polynomial::product(&remainder, &remainder, p);
                    squared.resize(7, 0);
                    squared[0] = (squared[0] - 2).rem_euclid(p);
                    for k in (4..=6).rev() {
                        let factor = squared[k];
                        values.insert(format!("s{step}_{k}"), factor);
                        for (i, coefficient) in q.iter().enumerate() {
                            squared[k - 4 + i] =
                                (squared[k - 4 + i] - factor * coefficient).rem_euclid(p);
                        }
                    }
                    remainder = squared[..4].into();
                    for (i, value) in remainder.iter().enumerate() {
                        values.insert(format!("r{step}_{i}"), *value);
                    }
                }
                let assignment: Vec<_> = compact.names.iter().map(|name| values[name]).collect();
                let evaluated: Vec<_> = compact
                    .equations
                    .iter()
                    .map(|eq| eq.evaluate(&assignment))
                    .collect();
                let (graphs, terminal_eqs) = evaluated.split_at(evaluated.len() - 4);
                assert!(graphs.iter().all(|value| *value == 0));
                let expanded = System::build(curve, target, depth, terminal, Mode::Expanded);
                let expanded_values: Vec<_> = expanded
                    .equations
                    .iter()
                    .map(|eq| eq.evaluate(&[alpha, beta, t]))
                    .collect();
                assert_eq!(terminal_eqs, expanded_values);
            }
        }
    }
}
