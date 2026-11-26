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
    /// Monic factors of the separate fine fibers, coupled to the RR5 quartic.
    /// Four factor coefficients precede a0,a1,t0. No solver is called here.
    pub fn factorwise(
        curve: Fixture,
        target: Point,
        levels: usize,
        images: [i64; 4],
    ) -> Result<Self, String> {
        if !(1..=2).contains(&levels)
            || !images.windows(2).all(|w| w[0] <= w[1])
            || images.iter().any(|b| !(0..curve.p).contains(b))
        {
            return Err(
                "factorwise constructor requires sorted canonical images and levels 1..=2".into(),
            );
        }
        let mut phi = vec![0, 1];
        for _ in 0..levels {
            phi = crate::polynomial::product(&phi, &phi, curve.p);
            phi[0] = (phi[0] - 2).rem_euclid(curve.p);
        }
        let fibers = images
            .into_iter()
            .map(|b| {
                let mut fiber = phi.clone();
                fiber[0] = (fiber[0] - b).rem_euclid(curve.p);
                (b, fiber)
            })
            .collect();
        Self::fiber_factors(curve, target, images, &fibers)
    }

    /// Shared monic-divisor equations for explicit small fiber polynomials.
    pub fn fiber_factors(
        curve: Fixture,
        target: Point,
        images: [i64; 4],
        fibers: &BTreeMap<i64, Vec<i64>>,
    ) -> Result<Self, String> {
        if !images.windows(2).all(|w| w[0] <= w[1])
            || images.iter().any(|b| !(0..curve.p).contains(b))
        {
            return Err("fiber factors require sorted canonical images".into());
        }
        let mut multiplicities = BTreeMap::<i64, usize>::new();
        for b in images {
            *multiplicities.entry(b).or_default() += 1;
        }
        for (b, m) in &multiplicities {
            let fiber = fibers.get(b).ok_or("missing fiber polynomial")?;
            if !(2..=5).contains(&fiber.len())
                || fiber.last() != Some(&1)
                || fiber.iter().any(|c| !(0..curve.p).contains(c))
                || *m >= fiber.len()
            {
                return Err("invalid small monic fiber or excess multiplicity".into());
            }
        }
        let names = (0..4)
            .map(|i| format!("q{i}"))
            .chain(["a0", "a1", "t0"].map(str::to_owned))
            .collect::<Vec<_>>();
        let vars = (0..7)
            .map(|i| Polynomial::variable(curve.p, 7, i))
            .collect::<Vec<_>>();
        let constant = |v| vars[0].constant(v);
        let mut offset = 0;
        let mut product = vec![constant(1)];
        let mut equations = vec![];
        for (b, m) in multiplicities {
            let mut factor = vars[offset..offset + m].to_vec();
            factor.push(constant(1));
            let fiber = &fibers[&b];
            let degree = fiber.len() - 1;
            let mut remainder = fiber.iter().map(|c| constant(*c)).collect::<Vec<_>>();
            for shift in (0..=degree - m).rev() {
                let leading = remainder[shift + m].clone();
                for j in 0..=m {
                    remainder[shift + j] = &remainder[shift + j] - &(&leading * &factor[j]);
                }
            }
            equations.extend_from_slice(&remainder[..m]);
            let mut next = vec![constant(0); product.len() + m];
            for (i, a) in product.iter().enumerate() {
                for (j, b) in factor.iter().enumerate() {
                    next[i + j] = &next[i + j] + &(a * b);
                }
            }
            product = next;
            offset += m;
        }
        let norm = quartic(curve, target, &vars[4], &vars[5], &vars[6]);
        equations.extend(norm.iter().zip(&product).map(|(a, b)| a - b));
        Ok(Self { names, equations })
    }

    /// Four-leaf composition of the existing oriented S3 pair identity.
    /// Slots: x0..x3, y0,t0,w0, y2,t1,w1, outer slope, then tower wires.
    #[cfg(feature = "production-controls")]
    pub fn oriented(base: &crate::Base, target: Point, tower: bool) -> Self {
        let curve = base.curve;
        let mut names = [
            "x0", "x1", "x2", "x3", "y0", "t0", "w0", "y2", "t1", "w1", "s",
        ]
        .map(str::to_owned)
        .to_vec();
        if tower {
            for slot in 0..4 {
                names.extend((1..base.depth).map(|j| format!("d{slot}_{j}")));
            }
        }
        let vars: Vec<_> = (0..names.len())
            .map(|i| Polynomial::variable(curve.p, names.len(), i))
            .collect();
        let c = |v| vars[0].constant(v);
        let mut equations = vec![];
        for (x, z, y, t, w) in [(0, 1, 4, 5, 6), (2, 3, 7, 8, 9)] {
            let (x, z, y, t, w) = (&vars[x], &vars[z], &vars[y], &vars[t], &vars[w]);
            equations
                .push(&(&y.square() - &(&x.square() * x)) - &(&x.scale(curve.a) + &c(curve.b)));
            equations.push(&(&(&t.square() - w) - x) - z);
            equations.push(
                &(&(&(y * t).scale(2) + &(&(z - x) * w)) - &x.square().scale(2))
                    - &(&(x * z) + &c(curve.a)),
            );
        }
        // The line through the two pair sums and -target. Its cubic x roots
        // have sum s^2, which also enforces the tangent case of equal sums.
        let slope = &vars[10];
        equations.push(&(&(&slope.square() - &vars[6]) - &vars[9]) - &c(target.x));
        for (x, y, t, w) in [(0, 4, 5, 6), (2, 7, 8, 9)] {
            let sum_y = &(&vars[t] * &(&vars[x] - &vars[w])) - &vars[y];
            equations.push(&(&sum_y - &(slope * &(&vars[w] - &c(target.x)))) + &c(target.y));
        }
        for slot in 0..4 {
            if tower {
                let mut previous = &vars[slot];
                for level in 1..=base.depth {
                    let next = if level == base.depth {
                        c(base.terminal)
                    } else {
                        vars[11 + slot * (base.depth - 1) + level - 1].clone()
                    };
                    equations.push(&(&previous.square() - &c(2)) - &next);
                    if level < base.depth {
                        previous = &vars[11 + slot * (base.depth - 1) + level - 1];
                    }
                }
            } else {
                let member = base
                    .unsigned
                    .iter()
                    .fold(c(1), |f, point| &f * &(&vars[slot] - &c(point.x)));
                equations.push(member);
            }
        }
        Self { names, equations }
    }

    #[cfg(feature = "production-controls")]
    pub fn native_equations(&self) -> Vec<crypto_lib::cryptanalysis::f4_fp::Poly> {
        use crypto_lib::cryptanalysis::f4_fp::{normalise, Ordering};
        self.equations
            .iter()
            .map(|poly| {
                let terms: Vec<_> = poly
                    .terms
                    .iter()
                    .map(|(e, c)| (e.iter().map(|v| u32::from(*v)).collect(), *c as u64))
                    .collect();
                normalise(&terms, poly.p as u64, Ordering::Grevlex)
            })
            .collect()
    }

    pub fn satisfied(&self, assignment: &[i64]) -> bool {
        assignment.len() == self.names.len()
            && self.equations.iter().all(|f| f.evaluate(assignment) == 0)
    }

    /// Formal derivatives at a recorded assignment, used by the finite audit.
    pub fn jacobian(&self, assignment: &[i64]) -> Vec<Vec<i64>> {
        assert_eq!(assignment.len(), self.names.len());
        self.equations
            .iter()
            .map(|f| {
                (0..self.names.len())
                    .map(|j| {
                        f.terms.iter().fold(0, |sum, (powers, coefficient)| {
                            if powers[j] == 0 {
                                return sum;
                            }
                            let mut term = coefficient * i64::from(powers[j]) % f.p;
                            for (i, (&power, &value)) in powers.iter().zip(assignment).enumerate() {
                                let degree = power - u8::from(i == j);
                                for _ in 0..degree {
                                    term = term * value % f.p;
                                }
                            }
                            (sum + term).rem_euclid(f.p)
                        })
                    })
                    .collect()
            })
            .collect()
    }

    #[cfg(all(test, feature = "production-controls"))]
    pub fn fix_leaf_x(&mut self, xs: [i64; 4]) {
        let sample = &self.equations[0];
        self.equations.extend(
            xs.iter()
                .enumerate()
                .map(|(i, x)| {
                    &Polynomial::variable(sample.p, sample.width, i) - &sample.constant(*x)
                })
                .collect::<Vec<_>>(),
        );
    }

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
    fn liftable_remainders_reject_an_extra_full_map_root() {
        let curve = crate::FIXTURES[0];
        let base = crate::Base::new(curve, 4);
        let fibers = crate::liftable::fibers(&base).unwrap();
        let (xs, images) = crate::cutoff::tuples(base.unsigned.len())
            .find_map(|tuple| {
                let mut xs = tuple.map(|i| base.unsigned[i].x);
                for slot in 0..4 {
                    let b = crate::dickson(xs[slot], 1, curve.p);
                    if let Some(extra) = (0..curve.p).find(|x| {
                        crate::dickson(*x, 1, curve.p) == b
                            && !base.unsigned.iter().any(|point| point.x == *x)
                    }) {
                        xs[slot] = extra;
                        return Some((xs, crate::cutoff::images(&base, xs)));
                    }
                }
                None
            })
            .expect("fixture has a nonliftable root in an occupied fiber");
        let mut pools = BTreeMap::<i64, Vec<i64>>::new();
        for x in xs {
            pools
                .entry(crate::dickson(x, 1, curve.p))
                .or_default()
                .push(x);
        }
        let mut assignment = vec![];
        for roots in pools.values() {
            let q = crate::cutoff::root_polynomial(roots, curve.p);
            assignment.extend_from_slice(&q[..roots.len()]);
        }
        assignment.extend([0, 0, 0]);
        let old = System::factorwise(curve, curve.q, 1, images).unwrap();
        let new = System::fiber_factors(curve, curve.q, images, &fibers).unwrap();
        assert!(old.equations[..4]
            .iter()
            .all(|q| q.evaluate(&assignment) == 0));
        assert!(new.equations[..4]
            .iter()
            .any(|q| q.evaluate(&assignment) != 0));
    }

    #[test]
    fn factorwise_remainders_match_values_at_roots_for_each_partition_shape() {
        for curve in crate::FIXTURES {
            for (levels, images) in [
                (1, [10, 11, 12, 13]),
                (1, [10, 10, 11, 11]),
                (2, [10, 10, 11, 12]),
                (2, [10, 10, 10, 11]),
                (2, [10, 10, 10, 10]),
                (2, [10, 11, 12, 13]),
                (2, [10, 10, 11, 11]),
            ] {
                let roots = [2, 3, 5, 7];
                let mut pools = BTreeMap::<i64, Vec<i64>>::new();
                for (b, x) in images.into_iter().zip(roots) {
                    pools.entry(b).or_default().push(x);
                }
                let mut assignment = vec![];
                for xs in pools.values() {
                    let factor = crate::cutoff::root_polynomial(xs, curve.p);
                    assignment.extend(&factor[..xs.len()]);
                }
                assignment.extend([3, 5, 7]);
                let system = System::factorwise(curve, curve.q, levels, images).unwrap();
                let mut offset = 0;
                for (b, xs) in pools {
                    let values = system.equations[offset..offset + xs.len()]
                        .iter()
                        .map(|eq| eq.evaluate(&assignment))
                        .collect::<Vec<_>>();
                    for x in &xs {
                        assert_eq!(
                            crate::polynomial::evaluate(&values, *x, curve.p),
                            (crate::dickson(*x, levels, curve.p) - b).rem_euclid(curve.p)
                        );
                    }
                    offset += xs.len();
                }
                let expected = crate::cutoff::root_polynomial(&roots, curve.p);
                let actual = numeric_quartic(curve, curve.q, 3, 5, 7);
                for i in 0..4 {
                    assert_eq!(
                        system.equations[4 + i].evaluate(&assignment),
                        (actual[i] - expected[i]).rem_euclid(curve.p)
                    );
                }
                assert_eq!(system.metrics().0, 7);
                assert_eq!(system.metrics().1, 8);
                assert!(system.metrics().3 <= 4);
            }
        }
    }

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
