//! Full group-trace-zero rational points for E_0 over odd-degree binary fields.
//!
//! The algebraic object is ker(Res(E_0) -> E_0), of dimension n-1.
//! The Boolean systems here describe its F_2-rational affine points, plus
//! an explicit identity branch. They are not claimed to be a birational
//! coordinate chart over arbitrary extension algebras.
//!
//! For odd n, T_n = 4E_0. Both conditions are required:
//! Tr(x)=0 and Tr(y+x H(x))=0, where H is the half-trace.
//! See BinaryCurve::is_in_4e for the halving derivation and prior art.

use super::{
    curve::{point_add, point_neg, scalar_mul},
    BinaryCurve, BinaryPoint, F2mElement,
};
use crate::cryptanalysis::sat::Solver;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceZeroError {
    UnsupportedCurve,
    InvalidField,
    InvalidPoint,
    NotTraceZero,
    InvalidCoordinates,
    TermCap,
    VariableCap { required: u64, maximum: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraceZeroCoordinates {
    Infinity,
    /// The nonconstant polynomial-basis coefficients, n-1 bits each.
    Affine {
        x_free: BigUint,
        y_free: BigUint,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceZeroAffineMode {
    /// Unrestricted: 2n variables, n curve equations and two trace equations.
    Raw,
    /// Unrestricted: 2n-2 variables, n curve equations, degree <=3.
    Compact,
    /// Unrestricted: 2n-1 variables and one quadratic helper equation.
    Factored,
}

/// Canonical squarefree Boolean ANF. Variable identifiers are zero-based.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TraceZeroPolynomial {
    pub terms: BTreeSet<Vec<u32>>,
}
impl TraceZeroPolynomial {
    fn variable(v: u32) -> Self {
        Self {
            terms: [vec![v]].into(),
        }
    }
    fn one() -> Self {
        Self {
            terms: [vec![]].into(),
        }
    }
    fn toggle(&mut self, t: Vec<u32>) {
        if !self.terms.remove(&t) {
            self.terms.insert(t);
        }
    }
    fn xor(&mut self, other: &Self, cap: usize) -> Result<(), TraceZeroError> {
        for t in &other.terms {
            self.toggle(t.clone());
        }
        if self.terms.len() > cap {
            return Err(TraceZeroError::TermCap);
        }
        Ok(())
    }
    fn product(&self, other: &Self, cap: usize) -> Result<Self, TraceZeroError> {
        if self
            .terms
            .len()
            .checked_mul(other.terms.len())
            .ok_or(TraceZeroError::TermCap)?
            > cap
        {
            return Err(TraceZeroError::TermCap);
        }
        let mut out = Self::default();
        for a in &self.terms {
            for b in &other.terms {
                let mut t = a.clone();
                t.extend(b);
                t.sort_unstable();
                t.dedup();
                out.toggle(t);
            }
        }
        if out.terms.len() > cap {
            return Err(TraceZeroError::TermCap);
        }
        Ok(out)
    }
    pub fn degree(&self) -> usize {
        self.terms.iter().map(Vec::len).max().unwrap_or(0)
    }
    pub fn evaluate(&self, bits: &[bool]) -> Option<bool> {
        let mut out = false;
        for t in &self.terms {
            let mut term = true;
            for &v in t {
                term &= *bits.get(v as usize)?;
            }
            out ^= term;
        }
        Some(out)
    }
}

#[derive(Clone)]
pub struct TraceZeroVariety {
    curve: BinaryCurve,
    trace_bits: Vec<bool>,
    /// Quadratic/linear monomials of Tr(x H(x)) in the raw x bits.
    q: TraceZeroPolynomial,
    reductions: Arc<Vec<Vec<usize>>>,
    order: BigUint,
}

#[derive(Clone, Debug)]
pub struct TraceZeroFactorBase {
    pub subspace_basis: Vec<F2mElement>,
    pub points: Vec<BinaryPoint>,
}

/// Exact restriction of q(x)=Tr(x H(x)) to a supplied trace-zero basis.
#[derive(Clone, Debug)]
pub struct TraceZeroSubspaceProfile {
    pub dimension: usize,
    pub polar_rank: usize,
    pub linear_terms: usize,
    pub quadratic_terms: usize,
    pub product_span_dimension: usize,
    pub restricted_q: TraceZeroPolynomial,
}

fn binary_rank(rows: impl IntoIterator<Item = BigUint>, n: usize) -> usize {
    let mut pivots: Vec<Option<BigUint>> = vec![None; n];
    let mut rank = 0;
    for mut row in rows {
        for bit in (0..n).rev() {
            if !row.bit(bit as u64) {
                continue;
            }
            if let Some(pivot) = &pivots[bit] {
                row ^= pivot;
            } else {
                pivots[bit] = Some(row);
                rank += 1;
                break;
            }
        }
    }
    rank
}

fn basis_coordinates(
    basis: &[F2mElement],
    value: &F2mElement,
    n: u32,
) -> Result<BigUint, TraceZeroError> {
    if value.m_value() != n || basis.len() > n as usize {
        return Err(TraceZeroError::InvalidCoordinates);
    }
    let mut pivots: Vec<Option<(BigUint, BigUint)>> = vec![None; n as usize];
    for (i, b) in basis.iter().enumerate() {
        if b.m_value() != n {
            return Err(TraceZeroError::InvalidCoordinates);
        }
        let mut row = b.to_biguint();
        let mut label = BigUint::one() << i;
        for bit in (0..n).rev() {
            if !row.bit(bit as u64) {
                continue;
            }
            if let Some((r, l)) = &pivots[bit as usize] {
                row ^= r;
                label ^= l;
            } else {
                pivots[bit as usize] = Some((row.clone(), label));
                break;
            }
        }
        if row.is_zero() {
            return Err(TraceZeroError::InvalidCoordinates);
        }
    }
    let mut row = value.to_biguint();
    let mut label = BigUint::zero();
    for bit in (0..n).rev() {
        if row.bit(bit as u64) {
            if let Some((r, l)) = &pivots[bit as usize] {
                row ^= r;
                label ^= l;
            }
        }
    }
    if !row.is_zero() {
        return Err(TraceZeroError::InvalidCoordinates);
    }
    Ok(label)
}

fn polynomial_gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        while !a.is_zero() && a.bits() >= b.bits() {
            let shift = (a.bits() - b.bits()) as usize;
            a ^= &b << shift;
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}
fn exact_field(curve: &BinaryCurve) -> bool {
    let n = curve.m;
    let f = &curve.irreducible;
    if f.degree != n
        || !f.low_terms.contains(&0)
        || f.low_terms.iter().any(|&t| t >= n)
        || f.low_terms.iter().copied().collect::<BTreeSet<_>>().len() != f.low_terms.len()
    {
        return false;
    }
    let z = F2mElement::from_bit_positions(&[1], n);
    let mut polynomial = BigUint::one() << n;
    for &i in &f.low_terms {
        polynomial ^= BigUint::one() << i;
    }
    let mut divisors = Vec::new();
    let mut remaining = n;
    let mut d = 2;
    while d * d <= remaining {
        if remaining % d == 0 {
            divisors.push(d);
            while remaining % d == 0 {
                remaining /= d;
            }
        }
        d += 1;
    }
    if remaining > 1 {
        divisors.push(remaining);
    }
    let checks: BTreeSet<_> = divisors.into_iter().map(|p| n / p).collect();
    let mut power = z.clone();
    for i in 1..=n {
        power = power.square(f);
        if checks.contains(&i)
            && polynomial_gcd(power.add(&z).to_biguint(), polynomial.clone()) != BigUint::one()
        {
            return false;
        }
    }
    power == z
}
impl TraceZeroVariety {
    /// Exact E_0/odd-degree admission, including the declared order and field.
    pub fn new(curve: &BinaryCurve) -> Result<Self, TraceZeroError> {
        let n = curve.m;
        if !(3..=131).contains(&n)
            || n % 2 == 0
            || curve.a.m_value() != n
            || !curve.a.is_zero()
            || curve.b != F2mElement::one(n)
        {
            return Err(TraceZeroError::UnsupportedCurve);
        }
        if !exact_field(curve) {
            return Err(TraceZeroError::InvalidField);
        }
        let (mut t0, mut t1) = (BigInt::from(2), BigInt::from(-1));
        for _ in 1..n {
            let t = -&t1 - (&t0 << 1);
            t0 = t1;
            t1 = t;
        }
        let count = ((BigInt::one() << n) + BigInt::one() - t1)
            .to_biguint()
            .ok_or(TraceZeroError::UnsupportedCurve)?;
        let order = &count >> 2usize;
        // A caller may select a proper odd-order subgroup of T_n. The
        // variety still contains the entire trace kernel, not just that
        // selected subgroup. Validate the declared full cardinality.
        if !curve.order.bit(0) || &curve.order * &curve.cofactor != count {
            return Err(TraceZeroError::UnsupportedCurve);
        }
        let basis: Vec<_> = (0..n)
            .map(|i| F2mElement::from_bit_positions(&[i], n))
            .collect();
        let trace_bits: Vec<_> = basis
            .iter()
            .map(|b| b.trace(&curve.irreducible) != 0)
            .collect();
        if !trace_bits[0] {
            return Err(TraceZeroError::InvalidField);
        }
        let half: Vec<_> = basis
            .iter()
            .map(|b| b.half_trace(&curve.irreducible).expect("odd degree"))
            .collect();
        let mut q = TraceZeroPolynomial::default();
        for i in 0..n as usize {
            for j in 0..n as usize {
                if basis[i]
                    .mul(&half[j], &curve.irreducible)
                    .trace(&curve.irreducible)
                    != 0
                {
                    let mut monomial = vec![i as u32, j as u32];
                    monomial.sort_unstable();
                    monomial.dedup();
                    q.toggle(monomial);
                }
            }
        }
        let z = F2mElement::from_bit_positions(&[1], n);
        let mut power = F2mElement::one(n);
        let mut reductions = Vec::new();
        for _ in 0..2 * n - 1 {
            let b = power.to_biguint();
            reductions.push((0..n as usize).filter(|&i| b.bit(i as u64)).collect());
            power = power.mul(&z, &curve.irreducible);
        }
        Ok(Self {
            curve: curve.clone(),
            trace_bits,
            q,
            reductions: Arc::new(reductions),
            order,
        })
    }
    pub fn curve(&self) -> &BinaryCurve {
        &self.curve
    }
    pub fn dimension(&self) -> u32 {
        self.curve.m - 1
    }
    pub fn rational_point_order(&self) -> &BigUint {
        &self.order
    }
    /// Intersect a supplied point base with the full group-trace kernel.
    /// The returned linear basis removes its redundant x-trace bit; the
    /// second membership condition is still enforced on every point.
    pub fn intersect_factor_base(
        &self,
        points: &[BinaryPoint],
        basis: &[F2mElement],
    ) -> Result<TraceZeroFactorBase, TraceZeroError> {
        let n = self.curve.m as usize;
        let mut pivots: Vec<Option<BigUint>> = vec![None; n];
        for element in basis {
            if element.m_value() != self.curve.m {
                return Err(TraceZeroError::InvalidCoordinates);
            }
            let mut row = element.to_biguint();
            for bit in (0..n).rev() {
                if !row.bit(bit as u64) {
                    continue;
                }
                if let Some(pivot) = &pivots[bit] {
                    row ^= pivot;
                } else {
                    pivots[bit] = Some(row.clone());
                    break;
                }
            }
            if row.is_zero() {
                return Err(TraceZeroError::InvalidCoordinates);
            }
        }
        for point in points {
            if !self.valid_point(point) {
                return Err(TraceZeroError::InvalidPoint);
            }
            let BinaryPoint::Affine { x, .. } = point else {
                return Err(TraceZeroError::InvalidPoint);
            };
            let mut row = x.to_biguint();
            for bit in (0..n).rev() {
                if row.bit(bit as u64) {
                    if let Some(pivot) = &pivots[bit] {
                        row ^= pivot;
                    }
                }
            }
            if !row.is_zero() {
                return Err(TraceZeroError::InvalidCoordinates);
            }
        }
        let trace: Vec<_> = basis
            .iter()
            .map(|b| b.trace(&self.curve.irreducible) != 0)
            .collect();
        let reduced = if let Some(pivot) = trace.iter().position(|&t| t) {
            basis
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    if i == pivot {
                        None
                    } else if trace[i] {
                        Some(b.add(&basis[pivot]))
                    } else {
                        Some(b.clone())
                    }
                })
                .collect()
        } else {
            basis.to_vec()
        };
        Ok(TraceZeroFactorBase {
            subspace_basis: reduced,
            points: points
                .iter()
                .filter(|p| self.contains(p))
                .cloned()
                .collect(),
        })
    }
    fn valid_point(&self, p: &BinaryPoint) -> bool {
        match p {
            BinaryPoint::Infinity => true,
            BinaryPoint::Affine { x, y } => {
                x.m_value() == self.curve.m
                    && y.m_value() == self.curve.m
                    && self.curve.is_on_curve(p)
            }
        }
    }
    /// Full point trace, not the one-bit field trace.
    pub fn point_trace(&self, p: &BinaryPoint) -> Result<BinaryPoint, TraceZeroError> {
        if !self.valid_point(p) {
            return Err(TraceZeroError::InvalidPoint);
        }
        let mut sum = BinaryPoint::Infinity;
        let mut power = p.clone();
        for _ in 0..self.curve.m {
            sum = point_add(&self.curve, &sum, &power);
            power = match power {
                BinaryPoint::Infinity => BinaryPoint::Infinity,
                BinaryPoint::Affine { x, y } => BinaryPoint::Affine {
                    x: x.square(&self.curve.irreducible),
                    y: y.square(&self.curve.irreducible),
                },
            };
        }
        Ok(sum)
    }
    pub fn contains(&self, p: &BinaryPoint) -> bool {
        self.valid_point(p) && self.curve.is_in_4e(p) == Some(true)
    }
    /// An idempotent projection that fixes T_n: P-[n^-1 mod 4]Tr(P).
    pub fn project(&self, p: &BinaryPoint) -> Result<BinaryPoint, TraceZeroError> {
        let trace = self.point_trace(p)?;
        let base_part = scalar_mul(&self.curve, &trace, &BigUint::from(self.curve.m % 4));
        Ok(point_add(&self.curve, p, &point_neg(&base_part)))
    }
    /// The cheap norm-four endomorphism tau-1; not an idempotent projection.
    pub fn cofactor_image(&self, p: &BinaryPoint) -> Result<BinaryPoint, TraceZeroError> {
        if !self.valid_point(p) {
            return Err(TraceZeroError::InvalidPoint);
        }
        let fp = match p {
            BinaryPoint::Infinity => BinaryPoint::Infinity,
            BinaryPoint::Affine { x, y } => BinaryPoint::Affine {
                x: x.square(&self.curve.irreducible),
                y: y.square(&self.curve.irreducible),
            },
        };
        Ok(point_add(&self.curve, &fp, &point_neg(p)))
    }
    fn q_value(&self, x: &F2mElement) -> bool {
        x.mul(
            &x.half_trace(&self.curve.irreducible).expect("odd degree"),
            &self.curve.irreducible,
        )
        .trace(&self.curve.irreducible)
            != 0
    }
    pub fn coordinates(&self, p: &BinaryPoint) -> Result<TraceZeroCoordinates, TraceZeroError> {
        if !self.valid_point(p) {
            return Err(TraceZeroError::InvalidPoint);
        }
        if !self.contains(p) {
            return Err(TraceZeroError::NotTraceZero);
        }
        Ok(match p {
            BinaryPoint::Infinity => TraceZeroCoordinates::Infinity,
            BinaryPoint::Affine { x, y } => TraceZeroCoordinates::Affine {
                x_free: x.to_biguint() >> 1usize,
                y_free: y.to_biguint() >> 1usize,
            },
        })
    }
    pub fn from_coordinates(
        &self,
        c: &TraceZeroCoordinates,
    ) -> Result<BinaryPoint, TraceZeroError> {
        match c {
            TraceZeroCoordinates::Infinity => Ok(BinaryPoint::Infinity),
            TraceZeroCoordinates::Affine { x_free, y_free } => {
                if x_free.bits() > u64::from(self.dimension())
                    || y_free.bits() > u64::from(self.dimension())
                {
                    return Err(TraceZeroError::InvalidCoordinates);
                }
                let mut x = x_free << 1usize;
                let mut y = y_free << 1usize;
                let mut tx = false;
                let mut ty = false;
                for i in 1..self.curve.m as usize {
                    if self.trace_bits[i] {
                        tx ^= x.bit(i as u64);
                        ty ^= y.bit(i as u64);
                    }
                }
                if tx {
                    x.set_bit(0, true)
                }
                let fx = F2mElement::from_biguint(&x, self.curve.m);
                if ty ^ self.q_value(&fx) {
                    y.set_bit(0, true)
                }
                let p = BinaryPoint::Affine {
                    x: fx,
                    y: F2mElement::from_biguint(&y, self.curve.m),
                };
                if !self.contains(&p) {
                    return Err(TraceZeroError::InvalidCoordinates);
                }
                Ok(p)
            }
        }
    }
    fn square(
        &self,
        x: &[TraceZeroPolynomial],
        cap: usize,
    ) -> Result<Vec<TraceZeroPolynomial>, TraceZeroError> {
        let mut out = vec![TraceZeroPolynomial::default(); self.curve.m as usize];
        for (i, p) in x.iter().enumerate() {
            for &j in &self.reductions[2 * i] {
                out[j].xor(p, cap)?;
            }
        }
        Ok(out)
    }
    fn multiply(
        &self,
        x: &[TraceZeroPolynomial],
        y: &[TraceZeroPolynomial],
        cap: usize,
    ) -> Result<Vec<TraceZeroPolynomial>, TraceZeroError> {
        let mut out = vec![TraceZeroPolynomial::default(); self.curve.m as usize];
        for (i, a) in x.iter().enumerate() {
            for (j, b) in y.iter().enumerate() {
                if a.terms.is_empty() || b.terms.is_empty() {
                    continue;
                }
                let product = a.product(b, cap)?;
                for &k in &self.reductions[i + j] {
                    out[k].xor(&product, cap)?;
                }
            }
        }
        Ok(out)
    }
    fn symbolic_q(
        &self,
        x: &[TraceZeroPolynomial],
        cap: usize,
    ) -> Result<TraceZeroPolynomial, TraceZeroError> {
        let mut q = TraceZeroPolynomial::default();
        for t in &self.q.terms {
            let term = if t.len() == 1 {
                x[t[0] as usize].clone()
            } else {
                x[t[0] as usize].product(&x[t[1] as usize], cap)?
            };
            q.xor(&term, cap)?;
        }
        Ok(q)
    }
    pub fn affine_system(
        &self,
        mode: TraceZeroAffineMode,
        max_terms: usize,
    ) -> Result<TraceZeroAffineSystem, TraceZeroError> {
        if max_terms == 0 {
            return Err(TraceZeroError::TermCap);
        }
        let n = self.curve.m as usize;
        let mut x = vec![TraceZeroPolynomial::default(); n];
        let mut y = x.clone();
        let variables = match mode {
            TraceZeroAffineMode::Raw => 2 * n,
            TraceZeroAffineMode::Compact => 2 * n - 2,
            TraceZeroAffineMode::Factored => 2 * n - 1,
        };
        if mode == TraceZeroAffineMode::Raw {
            for i in 0..n {
                x[i] = TraceZeroPolynomial::variable(i as u32);
                y[i] = TraceZeroPolynomial::variable((n + i) as u32);
            }
        } else {
            for i in 1..n {
                x[i] = TraceZeroPolynomial::variable((i - 1) as u32);
                y[i] = TraceZeroPolynomial::variable((n + i - 2) as u32);
                if self.trace_bits[i] {
                    let a = x[i].clone();
                    x[0].xor(&a, max_terms)?;
                    let b = y[i].clone();
                    y[0].xor(&b, max_terms)?;
                }
            }
        }
        let q = self.symbolic_q(&x, max_terms)?;
        let mut extra = Vec::new();
        match mode {
            TraceZeroAffineMode::Raw => {
                let mut tx = TraceZeroPolynomial::default();
                let mut ty = q.clone();
                for i in 0..n {
                    if self.trace_bits[i] {
                        tx.xor(&x[i], max_terms)?;
                        ty.xor(&y[i], max_terms)?;
                    }
                }
                extra.extend([tx, ty]);
            }
            TraceZeroAffineMode::Compact => y[0].xor(&q, max_terms)?,
            TraceZeroAffineMode::Factored => {
                let helper = TraceZeroPolynomial::variable((2 * n - 2) as u32);
                y[0].xor(&helper, max_terms)?;
                let mut relation = q.clone();
                relation.xor(&helper, max_terms)?;
                extra.push(relation);
            }
        }
        let mut equations = self.square(&y, max_terms)?;
        let xy = self.multiply(&x, &y, max_terms)?;
        let x2 = self.square(&x, max_terms)?;
        let x3 = self.multiply(&x, &x2, max_terms)?;
        for i in 0..n {
            equations[i].xor(&xy[i], max_terms)?;
            equations[i].xor(&x3[i], max_terms)?;
        }
        equations[0].xor(&TraceZeroPolynomial::one(), max_terms)?;
        equations.extend(extra);
        if equations.iter().map(|p| p.terms.len()).sum::<usize>() > max_terms {
            return Err(TraceZeroError::TermCap);
        }
        Ok(TraceZeroAffineSystem {
            variables: variables as u32,
            equations,
            mode,
            variety: self.clone(),
            x,
            y,
            x_basis: None,
        })
    }

    fn subspace_x(
        &self,
        basis: &[F2mElement],
        cap: usize,
    ) -> Result<Vec<TraceZeroPolynomial>, TraceZeroError> {
        basis_coordinates(basis, &F2mElement::zero(self.curve.m), self.curve.m)?;
        if basis.iter().any(|b| b.trace(&self.curve.irreducible) != 0) {
            return Err(TraceZeroError::InvalidCoordinates);
        }
        let mut x = vec![TraceZeroPolynomial::default(); self.curve.m as usize];
        for (i, b) in basis.iter().enumerate() {
            for (bit, p) in x.iter_mut().enumerate() {
                if b.to_biguint().bit(bit as u64) {
                    p.xor(&TraceZeroPolynomial::variable(i as u32), cap)?;
                }
            }
        }
        Ok(x)
    }

    pub fn subspace_profile(
        &self,
        basis: &[F2mElement],
        max_terms: usize,
    ) -> Result<TraceZeroSubspaceProfile, TraceZeroError> {
        if max_terms == 0 {
            return Err(TraceZeroError::TermCap);
        }
        let x = self.subspace_x(basis, max_terms)?;
        let q = self.symbolic_q(&x, max_terms)?;
        let d = basis.len();
        let mut polar = vec![BigUint::zero(); d];
        let mut linear = 0;
        let mut quadratic = 0;
        for term in &q.terms {
            match term.as_slice() {
                [_] => linear += 1,
                [i, j] => {
                    polar[*i as usize].set_bit(*j as u64, true);
                    polar[*j as usize].set_bit(*i as u64, true);
                    quadratic += 1;
                }
                _ => {}
            }
        }
        let products = (0..d)
            .flat_map(|i| (i..d).map(move |j| (i, j)))
            .map(|(i, j)| {
                basis[i]
                    .mul(&basis[j], &self.curve.irreducible)
                    .to_biguint()
            });
        Ok(TraceZeroSubspaceProfile {
            dimension: d,
            polar_rank: binary_rank(polar, d),
            linear_terms: linear,
            quadratic_terms: quadratic,
            product_span_dimension: binary_rank(products, self.curve.m as usize),
            restricted_q: q,
        })
    }

    /// Exact full-kernel affine model with x restricted to this trace-zero
    /// subspace. Variables are its basis coefficients, then y_1..y_(n-1).
    /// Restrict before eliminating y_0: if q is linear here, degree stays two.
    pub fn affine_system_over_subspace(
        &self,
        basis: &[F2mElement],
        max_terms: usize,
    ) -> Result<TraceZeroAffineSystem, TraceZeroError> {
        self.affine_system_over_subspace_with_mode(basis, TraceZeroAffineMode::Compact, max_terms)
    }

    /// Matched raw/compact/factored presentations of the same restricted
    /// point set. The already implied x-trace row is omitted in every mode.
    pub fn affine_system_over_subspace_with_mode(
        &self,
        basis: &[F2mElement],
        mode: TraceZeroAffineMode,
        max_terms: usize,
    ) -> Result<TraceZeroAffineSystem, TraceZeroError> {
        if max_terms == 0 {
            return Err(TraceZeroError::TermCap);
        }
        let n = self.curve.m as usize;
        let x = self.subspace_x(basis, max_terms)?;
        let mut y = vec![TraceZeroPolynomial::default(); n];
        let q = self.symbolic_q(&x, max_terms)?;
        let mut extra = Vec::new();
        if mode == TraceZeroAffineMode::Raw {
            let mut ty = q;
            for i in 0..n {
                y[i] = TraceZeroPolynomial::variable((basis.len() + i) as u32);
                if self.trace_bits[i] {
                    ty.xor(&y[i], max_terms)?;
                }
            }
            extra.push(ty);
        } else {
            for i in 1..n {
                y[i] = TraceZeroPolynomial::variable((basis.len() + i - 1) as u32);
                if self.trace_bits[i] {
                    let yi = y[i].clone();
                    y[0].xor(&yi, max_terms)?;
                }
            }
            if mode == TraceZeroAffineMode::Factored {
                let helper = TraceZeroPolynomial::variable((basis.len() + n - 1) as u32);
                y[0].xor(&helper, max_terms)?;
                let mut relation = q;
                relation.xor(&helper, max_terms)?;
                extra.push(relation);
            } else {
                y[0].xor(&q, max_terms)?;
            }
        }
        let mut equations = self.square(&y, max_terms)?;
        let xy = self.multiply(&x, &y, max_terms)?;
        let x2 = self.square(&x, max_terms)?;
        let x3 = self.multiply(&x, &x2, max_terms)?;
        for i in 0..n {
            equations[i].xor(&xy[i], max_terms)?;
            equations[i].xor(&x3[i], max_terms)?;
        }
        equations[0].xor(&TraceZeroPolynomial::one(), max_terms)?;
        equations.extend(extra);
        if equations.iter().map(|p| p.terms.len()).sum::<usize>() > max_terms {
            return Err(TraceZeroError::TermCap);
        }
        Ok(TraceZeroAffineSystem {
            variables: (basis.len() + n - usize::from(mode == TraceZeroAffineMode::Compact)) as u32,
            equations,
            mode,
            variety: self.clone(),
            x,
            y,
            x_basis: Some(basis.to_vec()),
        })
    }
}
#[derive(Clone)]
pub struct TraceZeroAffineSystem {
    pub variables: u32,
    pub equations: Vec<TraceZeroPolynomial>,
    pub mode: TraceZeroAffineMode,
    variety: TraceZeroVariety,
    x: Vec<TraceZeroPolynomial>,
    y: Vec<TraceZeroPolynomial>,
    x_basis: Option<Vec<F2mElement>>,
}
impl TraceZeroAffineSystem {
    /// Some for a subspace-restricted model; x variables are coefficients
    /// in this exact ordered basis, not raw polynomial-coordinate bits.
    pub fn x_subspace_basis(&self) -> Option<&[F2mElement]> {
        self.x_basis.as_deref()
    }
    pub fn maximum_degree(&self) -> usize {
        self.equations
            .iter()
            .map(TraceZeroPolynomial::degree)
            .max()
            .unwrap_or(0)
    }
    pub fn monomials(&self) -> usize {
        self.equations.iter().map(|p| p.terms.len()).sum()
    }
    pub fn accepts(&self, assignment: &[bool]) -> bool {
        assignment.len() == self.variables as usize
            && self
                .equations
                .iter()
                .all(|p| p.evaluate(assignment) == Some(false))
    }
    pub fn assignment(&self, p: &BinaryPoint) -> Result<Vec<bool>, TraceZeroError> {
        let BinaryPoint::Affine { x, y } = p else {
            return Err(TraceZeroError::InvalidCoordinates);
        };
        if !self.variety.valid_point(p) {
            return Err(TraceZeroError::InvalidPoint);
        }
        if !self.variety.contains(p) {
            return Err(TraceZeroError::NotTraceZero);
        }
        let n = self.variety.curve.m as usize;
        let xb = x.to_biguint();
        let yb = y.to_biguint();
        let mut bits = Vec::new();
        match self.mode {
            TraceZeroAffineMode::Raw => {
                if let Some(basis) = &self.x_basis {
                    let coefficients = basis_coordinates(basis, x, self.variety.curve.m)?;
                    for i in 0..basis.len() {
                        bits.push(coefficients.bit(i as u64));
                    }
                } else {
                    for i in 0..n {
                        bits.push(xb.bit(i as u64));
                    }
                }
                for i in 0..n {
                    bits.push(yb.bit(i as u64));
                }
            }
            _ => {
                if let Some(basis) = &self.x_basis {
                    let coefficients = basis_coordinates(basis, x, self.variety.curve.m)?;
                    for i in 0..basis.len() {
                        bits.push(coefficients.bit(i as u64));
                    }
                } else {
                    for i in 1..n {
                        bits.push(xb.bit(i as u64));
                    }
                }
                for i in 1..n {
                    bits.push(yb.bit(i as u64));
                }
                if self.mode == TraceZeroAffineMode::Factored {
                    bits.push(self.variety.q_value(x));
                }
            }
        }
        Ok(bits)
    }
    /// Decoding checks the original point and full membership, not only ANF.
    pub fn decode(&self, bits: &[bool]) -> Result<BinaryPoint, TraceZeroError> {
        if !self.accepts(bits) {
            return Err(TraceZeroError::InvalidCoordinates);
        }
        let value = |v: &[TraceZeroPolynomial]| {
            let mut raw = BigUint::zero();
            for (i, p) in v.iter().enumerate() {
                if p.evaluate(bits) == Some(true) {
                    raw.set_bit(i as u64, true);
                }
            }
            F2mElement::from_biguint(&raw, self.variety.curve.m)
        };
        let p = BinaryPoint::Affine {
            x: value(&self.x),
            y: value(&self.y),
        };
        if !self.variety.contains(&p) {
            return Err(TraceZeroError::NotTraceZero);
        }
        Ok(p)
    }
    fn auxiliary_keys(&self) -> BTreeSet<Vec<u32>> {
        let mut keys = BTreeSet::new();
        for p in &self.equations {
            for m in &p.terms {
                for len in 2..=m.len() {
                    keys.insert(m[..len].to_vec());
                }
            }
        }
        keys
    }
    pub fn auxiliary_variables(&self) -> usize {
        self.auxiliary_keys().len()
    }
    /// Append exact CNF AND gates plus native XOR rows. Caps are preflighted.
    pub fn append_to_solver(
        &self,
        solver: &mut Solver,
        wires: &[u32],
        maximum: u32,
    ) -> Result<(), TraceZeroError> {
        if wires.len() != self.variables as usize
            || wires.iter().any(|&w| w == 0 || w > solver.n_vars())
            || wires.iter().copied().collect::<BTreeSet<_>>().len() != wires.len()
        {
            return Err(TraceZeroError::InvalidCoordinates);
        }
        let keys = self.auxiliary_keys();
        let required = u64::from(solver.n_vars()) + keys.len() as u64;
        if required > u64::from(maximum) || required > i32::MAX as u64 {
            return Err(TraceZeroError::VariableCap { required, maximum });
        }
        let allocated = solver.add_vars(keys.len() as u32);
        let gates: BTreeMap<_, _> = keys.into_iter().zip(allocated).collect();
        for (key, &gate) in &gates {
            let a = if key.len() == 2 {
                wires[key[0] as usize]
            } else {
                gates[&key[..key.len() - 1].to_vec()]
            };
            let b = wires[*key.last().expect("nonempty") as usize];
            let (g, a, b) = (gate as i32, a as i32, b as i32);
            solver.add_clause(vec![-g, a]);
            solver.add_clause(vec![-g, b]);
            solver.add_clause(vec![g, -a, -b]);
        }
        for p in &self.equations {
            let mut row = Vec::new();
            let mut rhs = false;
            for m in &p.terms {
                match m.len() {
                    0 => rhs ^= true,
                    1 => row.push(wires[m[0] as usize]),
                    _ => row.push(gates[m]),
                }
            }
            solver.add_xor(&row, rhs);
        }
        Ok(())
    }
}
