//! Exact arithmetic in the local chart quotient, at the fixed prime M31.
//!
//! Each block has variables `(z,w,s)` and defining ideal
//! `J = (z^2-z, zw-w, zs-s, w^2-s)`. Blocks precede ordinary free variables.
//! The four leading monomials are `z^2,zw,zs,w^2` for every degree-compatible
//! order. Canonical monomials in one block are `1,z,s^k,w*s^k` (with `k>=1`
//! for the third family). Reduction is coefficient-preserving and does not
//! increase total degree. No field equation is silently added to this ring.
//!
//! `macaulay` enumerates **all** canonical multiplier monomials within its
//! declared bound and forms `NF(m*NF(f_i))`. It does not omit products with
//! zero divisors. This is a complete row presentation of that bounded
//! quotient-multiplier space, not iterative last-fall closure or F4. Replacing
//! raw generators by their normal forms changes the relevant presentation's
//! filtration; both raw and prepared degrees are retained explicitly.

use crate::M31;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Invalid(String),
    ExponentOverflow,
    Limit(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => write!(out, "invalid polynomial input: {message}"),
            Self::ExponentOverflow => write!(out, "monomial exponent or total degree overflow"),
            Self::Limit(name) => write!(out, "finite construction limit reached: {name}"),
        }
    }
}
impl std::error::Error for Error {}

#[inline]
pub fn field_add(a: u64, b: u64) -> u64 {
    (a % M31 + b % M31) % M31
}
#[inline]
pub fn field_sub(a: u64, b: u64) -> u64 {
    (a % M31 + M31 - b % M31) % M31
}
#[inline]
pub fn field_mul(a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(M31)) as u64
}
pub fn field_pow(mut a: u64, mut n: u64) -> u64 {
    a %= M31;
    let mut result = 1;
    while n != 0 {
        if n & 1 != 0 {
            result = field_mul(result, a);
        }
        n >>= 1;
        if n != 0 {
            a = field_mul(a, a);
        }
    }
    result
}

fn monomial_degree(monomial: &[u64]) -> Result<u64, Error> {
    monomial.iter().try_fold(0u64, |sum, &e| {
        sum.checked_add(e).ok_or(Error::ExponentOverflow)
    })
}

fn insert(terms: &mut BTreeMap<Vec<u64>, u64>, exponents: Vec<u64>, coefficient: u64) {
    let coefficient = field_add(terms.get(&exponents).copied().unwrap_or(0), coefficient);
    if coefficient == 0 {
        terms.remove(&exponents);
    } else {
        terms.insert(exponents, coefficient);
    }
}

/// Ordinary sparse polynomial over M31. Terms are unique with canonical,
/// nonzero coefficients. Public fields ease exact export; all fallible methods
/// validate caller-supplied fields before accepting them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Polynomial {
    pub variables: usize,
    pub terms: BTreeMap<Vec<u64>, u64>,
}
pub type Poly = Polynomial;

impl Polynomial {
    pub fn zero(variables: usize) -> Self {
        Self { variables, terms: BTreeMap::new() }
    }
    pub fn constant(variables: usize, coefficient: u64) -> Self {
        let mut result = Self::zero(variables);
        if coefficient % M31 != 0 {
            result.terms.insert(vec![0; variables], coefficient % M31);
        }
        result
    }
    pub fn variable(variables: usize, variable: usize) -> Self {
        assert!(variable < variables, "polynomial variable index is out of range");
        let mut exponents = vec![0; variables];
        exponents[variable] = 1;
        Self { variables, terms: BTreeMap::from([(exponents, 1)]) }
    }
    pub fn from_terms(
        variables: usize,
        terms: impl IntoIterator<Item = (Vec<u64>, u64)>,
    ) -> Result<Self, Error> {
        let mut result = Self::zero(variables);
        for (exponents, coefficient) in terms {
            if exponents.len() != variables {
                return Err(Error::Invalid("exponent-vector width".into()));
            }
            monomial_degree(&exponents)?;
            insert(&mut result.terms, exponents, coefficient % M31);
        }
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), Error> {
        for (exponents, &coefficient) in &self.terms {
            if exponents.len() != self.variables || coefficient == 0 || coefficient >= M31 {
                return Err(Error::Invalid("noncanonical term or exponent-vector width".into()));
            }
            monomial_degree(exponents)?;
        }
        Ok(())
    }
    fn compatible(&self, other: &Self) -> Result<(), Error> {
        self.validate()?;
        other.validate()?;
        if self.variables != other.variables {
            return Err(Error::Invalid("polynomial variable counts differ".into()));
        }
        Ok(())
    }
    pub fn nvars(&self) -> usize {
        self.variables
    }
    /// Zero has degree zero for this construction API; it is explicitly
    /// excluded as a generator by `macaulay`.
    pub fn degree(&self) -> u64 {
        self.terms.keys().map(|m| {
            monomial_degree(m).expect("Polynomial::degree requires a valid polynomial")
        }).max().unwrap_or(0)
    }
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }
    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        self.compatible(other)?;
        let mut result = self.clone();
        for (m, &c) in &other.terms {
            insert(&mut result.terms, m.clone(), c);
        }
        Ok(result)
    }
    pub fn sub(&self, other: &Self) -> Result<Self, Error> {
        self.compatible(other)?;
        self.add(&other.scale(M31 - 1))
    }
    pub fn scale(&self, coefficient: u64) -> Self {
        let mut result = Self::zero(self.variables);
        for (m, &c) in &self.terms {
            let c = field_mul(c, coefficient);
            if c != 0 {
                result.terms.insert(m.clone(), c);
            }
        }
        result
    }
    /// Ordinary multiplication. Use `Ring::multiply` for quotient products.
    pub fn mul(&self, other: &Self) -> Result<Self, Error> {
        self.compatible(other)?;
        let mut result = Self::zero(self.variables);
        for (ma, &ca) in &self.terms {
            for (mb, &cb) in &other.terms {
                let m: Vec<u64> = ma.iter().zip(mb).map(|(&x, &y)| {
                    x.checked_add(y).ok_or(Error::ExponentOverflow)
                }).collect::<Result<_, _>>()?;
                monomial_degree(&m)?;
                insert(&mut result.terms, m, field_mul(ca, cb));
            }
        }
        Ok(result)
    }
    pub fn pow(&self, mut exponent: u32) -> Result<Self, Error> {
        self.validate()?;
        let mut result = Self::constant(self.variables, 1);
        let mut factor = self.clone();
        while exponent != 0 {
            if exponent & 1 != 0 {
                result = result.mul(&factor)?;
            }
            exponent >>= 1;
            if exponent != 0 {
                factor = factor.mul(&factor)?;
            }
        }
        Ok(result)
    }
    pub fn derivative(&self, variable: usize) -> Result<Self, Error> {
        self.validate()?;
        if variable >= self.variables {
            return Err(Error::Invalid("derivative variable index".into()));
        }
        let mut result = Self::zero(self.variables);
        for (m, &c) in &self.terms {
            if m[variable] != 0 {
                let coefficient = field_mul(c, m[variable] % M31);
                let mut exponent = m.clone();
                exponent[variable] -= 1;
                insert(&mut result.terms, exponent, coefficient);
            }
        }
        Ok(result)
    }
    pub fn evaluate(&self, values: &[u64]) -> Result<u64, Error> {
        self.validate()?;
        if values.len() != self.variables || values.iter().any(|&x| x >= M31) {
            return Err(Error::Invalid("evaluation point must have canonical M31 coordinates".into()));
        }
        let mut result = 0;
        for (m, &c) in &self.terms {
            let mut value = c;
            for (&x, &e) in values.iter().zip(m) {
                value = field_mul(value, field_pow(x, e));
            }
            result = field_add(result, value);
        }
        Ok(result)
    }
    /// Representation bridge only. Sorting/monic normalization for a selected
    /// F4 monomial order belongs to its caller and is not hidden here.
    pub fn to_u32_terms(&self) -> Result<Vec<(Vec<u32>, u64)>, Error> {
        self.validate()?;
        self.terms.iter().map(|(m, &c)| {
            let m = m.iter().map(|&e| u32::try_from(e).map_err(|_| Error::ExponentOverflow))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((m, c))
        }).collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ring {
    blocks: usize,
    free: usize,
}

impl Ring {
    pub fn new(blocks: usize, free: usize) -> Result<Self, Error> {
        let variables = blocks.checked_mul(3).and_then(|n| n.checked_add(free))
            .ok_or_else(|| Error::Invalid("variable count overflow".into()))?;
        if variables > 256 {
            return Err(Error::Limit("at most 256 variables"));
        }
        Ok(Self { blocks, free })
    }
    pub fn nvars(&self) -> usize {
        3 * self.blocks + self.free
    }
    pub fn block_count(&self) -> usize {
        self.blocks
    }
    pub fn free_count(&self) -> usize {
        self.free
    }
    fn check_polynomial(&self, polynomial: &Polynomial) -> Result<(), Error> {
        polynomial.validate()?;
        if polynomial.variables != self.nvars() {
            return Err(Error::Invalid("polynomial and chart ring widths differ".into()));
        }
        Ok(())
    }
    pub fn normal_monomial(&self, monomial: &[u64]) -> Result<Vec<u64>, Error> {
        if monomial.len() != self.nvars() {
            return Err(Error::Invalid("chart monomial width".into()));
        }
        monomial_degree(monomial)?;
        let mut result = monomial.to_vec();
        for block in 0..self.blocks {
            let j = 3 * block;
            let s = result[j + 2].checked_add(result[j + 1] / 2)
                .ok_or(Error::ExponentOverflow)?;
            let w = result[j + 1] & 1;
            let z = u64::from(result[j] != 0 && w == 0 && s == 0);
            result[j] = z;
            result[j + 1] = w;
            result[j + 2] = s;
        }
        Ok(result)
    }
    pub fn normal_form(&self, polynomial: &Polynomial) -> Result<Polynomial, Error> {
        self.check_polynomial(polynomial)?;
        let mut result = Polynomial::zero(self.nvars());
        for (m, &c) in &polynomial.terms {
            insert(&mut result.terms, self.normal_monomial(m)?, c);
        }
        Ok(result)
    }
    /// Quotient multiplication, reducing products as they are formed.
    pub fn multiply(&self, left: &Polynomial, right: &Polynomial) -> Result<Polynomial, Error> {
        self.check_polynomial(left)?;
        self.check_polynomial(right)?;
        let mut result = Polynomial::zero(self.nvars());
        for (ma, &ca) in &left.terms {
            for (mb, &cb) in &right.terms {
                let m = ma.iter().zip(mb).map(|(&a, &b)| {
                    a.checked_add(b).ok_or(Error::ExponentOverflow)
                }).collect::<Result<Vec<_>, _>>()?;
                insert(&mut result.terms, self.normal_monomial(&m)?, field_mul(ca, cb));
            }
        }
        Ok(result)
    }
    /// The exact p^iterations power in this quotient. Its exponents remain
    /// symbolic: this neither enumerates the field nor restricts geometric
    /// chart coordinates to F_p or F_(p^2).
    pub fn frobenius(&self, polynomial: &Polynomial, iterations: u32) -> Result<Polynomial, Error> {
        self.check_polynomial(polynomial)?;
        let exponent = M31.checked_pow(iterations).ok_or(Error::ExponentOverflow)?;
        let mut result = Polynomial::zero(self.nvars());
        for (m, &c) in &polynomial.terms {
            let exponent_vector = m.iter().map(|&e| {
                e.checked_mul(exponent).ok_or(Error::ExponentOverflow)
            }).collect::<Result<Vec<_>, _>>()?;
            insert(&mut result.terms, self.normal_monomial(&exponent_vector)?, c);
        }
        Ok(result)
    }
    pub fn chart_generators(&self, block: usize) -> Result<[Polynomial; 4], Error> {
        if block >= self.blocks {
            return Err(Error::Invalid("chart block index".into()));
        }
        let z = Polynomial::variable(self.nvars(), block * 3);
        let w = Polynomial::variable(self.nvars(), block * 3 + 1);
        let s = Polynomial::variable(self.nvars(), block * 3 + 2);
        Ok([z.mul(&z)?.sub(&z)?, z.mul(&w)?.sub(&w)?, z.mul(&s)?.sub(&s)?, w.mul(&w)?.sub(&s)?])
    }
    /// Canonical basis of the quotient filtration through the given degree.
    /// Enumeration is bounded and visits only canonical monomials.
    pub fn monomials(&self, degree: u32, cap: usize) -> Result<Vec<Vec<u64>>, Error> {
        if cap == 0 {
            return Err(Error::Limit("zero monomial capacity"));
        }
        fn recurse(ring: &Ring, index: usize, remaining: u32, monomial: &mut Vec<u64>,
                   output: &mut Vec<Vec<u64>>, cap: usize) -> Result<(), Error> {
            if index == ring.nvars() {
                if output.len() == cap {
                    return Err(Error::Limit("canonical multiplier monomials"));
                }
                output.push(monomial.clone());
                return Ok(());
            }
            let limit = if index < ring.blocks * 3 {
                match index % 3 {
                    0 => remaining.min(1),
                    1 => if monomial[index - 1] != 0 { 0 } else { remaining.min(1) },
                    _ => if monomial[index - 2] != 0 { 0 } else { remaining },
                }
            } else { remaining };
            for exponent in 0..=limit {
                monomial[index] = u64::from(exponent);
                recurse(ring, index + 1, remaining - exponent, monomial, output, cap)?;
            }
            monomial[index] = 0;
            Ok(())
        }
        let mut output = Vec::new();
        recurse(self, 0, degree, &mut vec![0; self.nvars()], &mut output, cap)?;
        Ok(output)
    }
    /// Exact finite quotient Macaulay row construction. This does not compute
    /// a Gröbner basis, solve a system, or establish a solving-degree bound.
    pub fn macaulay(&self, generators: &[Polynomial], degree: u32, limits: MacaulayLimits)
        -> Result<MacaulayMatrix, Error>
    {
        let mut matrix = MacaulayMatrix {
            degree_bound: degree, rows: Vec::new(), columns: Vec::new(),
            raw_generator_degrees: Vec::new(), prepared_generator_degrees: Vec::new(),
            skipped_zero_generators: Vec::new(), skipped_over_degree_generators: Vec::new(),
            max_unreduced_product_degree: 0, max_row_degree: 0, zero_rows: 0, total_terms: 0,
        };
        let mut columns = BTreeSet::new();
        let mut multipliers = BTreeMap::<u32, Vec<Vec<u64>>>::new();
        for (index, generator) in generators.iter().enumerate() {
            self.check_polynomial(generator)?;
            let prepared = self.normal_form(generator)?;
            matrix.raw_generator_degrees.push(generator.degree());
            matrix.prepared_generator_degrees.push(prepared.degree());
            if prepared.is_zero() {
                matrix.skipped_zero_generators.push(index);
                continue;
            }
            if prepared.degree() > u64::from(degree) {
                matrix.skipped_over_degree_generators.push(index);
                continue;
            }
            let multiplier_bound = degree - prepared.degree() as u32;
            if !multipliers.contains_key(&multiplier_bound) {
                multipliers.insert(multiplier_bound, self.monomials(multiplier_bound, limits.max_monomials)?);
            }
            for multiplier in &multipliers[&multiplier_bound] {
                if matrix.rows.len() == limits.max_rows {
                    return Err(Error::Limit("Macaulay rows"));
                }
                let polynomial = Polynomial::from_terms(self.nvars(), [(multiplier.clone(), 1)])?;
                let row = self.multiply(&polynomial, &prepared)?;
                if row.is_zero() {
                    matrix.zero_rows += 1;
                }
                matrix.total_terms = matrix.total_terms.checked_add(row.terms.len())
                    .ok_or(Error::Limit("Macaulay nonzero terms"))?;
                if matrix.total_terms > limits.max_terms {
                    return Err(Error::Limit("Macaulay nonzero terms"));
                }
                let unreduced_product_degree = monomial_degree(multiplier)? + prepared.degree();
                matrix.max_unreduced_product_degree = matrix.max_unreduced_product_degree.max(unreduced_product_degree);
                matrix.max_row_degree = matrix.max_row_degree.max(row.degree());
                for m in row.terms.keys() {
                    columns.insert(m.clone());
                    if columns.len() > limits.max_monomials {
                        return Err(Error::Limit("Macaulay columns"));
                    }
                }
                matrix.rows.push(MacaulayRow { generator_index: index, multiplier: multiplier.clone(),
                    prepared_generator_degree: prepared.degree(), unreduced_product_degree,
                    row_degree: row.degree(), row });
            }
        }
        matrix.columns = columns.into_iter().collect();
        Ok(matrix)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MacaulayLimits {
    pub max_monomials: usize,
    pub max_rows: usize,
    pub max_terms: usize,
}
impl Default for MacaulayLimits {
    fn default() -> Self {
        Self { max_monomials: 100_000, max_rows: 100_000, max_terms: 1_000_000 }
    }
}
#[derive(Clone, Debug)]
pub struct MacaulayRow {
    pub generator_index: usize,
    pub multiplier: Vec<u64>,
    pub prepared_generator_degree: u64,
    pub unreduced_product_degree: u64,
    pub row_degree: u64,
    pub row: Polynomial,
}
#[derive(Clone, Debug)]
pub struct MacaulayMatrix {
    pub degree_bound: u32,
    pub rows: Vec<MacaulayRow>,
    pub columns: Vec<Vec<u64>>,
    pub raw_generator_degrees: Vec<u64>,
    pub prepared_generator_degrees: Vec<u64>,
    pub skipped_zero_generators: Vec<usize>,
    pub skipped_over_degree_generators: Vec<usize>,
    pub max_unreduced_product_degree: u64,
    pub max_row_degree: u64,
    pub zero_rows: usize,
    pub total_terms: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn grevlex(a: &[u64], b: &[u64]) -> Ordering {
        let da: u64 = a.iter().sum();
        let db: u64 = b.iter().sum();
        da.cmp(&db).then_with(|| {
            a.iter().rev().zip(b.iter().rev()).find_map(|(x, y)| {
                if x == y { None } else { Some(y.cmp(x)) }
            }).unwrap_or(Ordering::Equal)
        })
    }
    // Independent ordinary polynomial long division. This test-only reducer
    // does not call Ring::normal_monomial or Ring::normal_form.
    fn divide(mut input: Polynomial, basis: &[Polynomial]) -> Polynomial {
        let mut remainder = Polynomial::zero(input.variables);
        while let Some((lead, coefficient)) = input.terms.iter().max_by(|(x, _), (y, _)| grevlex(x, y))
            .map(|(m, &c)| (m.clone(), c))
        {
            let divisor = basis.iter().find_map(|g| {
                let (lm, &lc) = g.terms.iter().max_by(|(x, _), (y, _)| grevlex(x, y)).unwrap();
                if lm.iter().zip(&lead).all(|(a, b)| a <= b) { Some((g, lm, lc)) } else { None }
            });
            if let Some((g, lm, lc)) = divisor {
                assert_eq!(lc, 1);
                let shift: Vec<_> = lead.iter().zip(lm).map(|(x, y)| x - y).collect();
                for (m, &c) in &g.terms {
                    let e: Vec<_> = m.iter().zip(&shift).map(|(x, y)| x + y).collect();
                    let product = ((u128::from(coefficient) * u128::from(c)) % u128::from(M31)) as u64;
                    let next = (input.terms.get(&e).copied().unwrap_or(0) + M31 - product) % M31;
                    if next == 0 { input.terms.remove(&e); } else { input.terms.insert(e, next); }
                }
            } else {
                input.terms.remove(&lead);
                remainder.terms.insert(lead, coefficient);
            }
        }
        remainder
    }
    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    fn random_poly(ring: &Ring, seed: &mut u64) -> Polynomial {
        Polynomial::from_terms(ring.nvars(), (0..10).map(|_| {
            let exponent = (0..ring.nvars()).map(|_| next(seed) % 5).collect();
            (exponent, next(seed) % M31)
        })).unwrap()
    }

    #[test]
    fn m31_local_groebner_basis_all_s_pairs_and_direct_product() {
        let ring = Ring::new(2, 1).unwrap();
        let mut basis = Vec::new();
        for block in 0..2 { basis.extend(ring.chart_generators(block).unwrap()); }
        let mut pairs = 0;
        for i in 0..basis.len() { for j in i + 1..basis.len() {
            let leading = |p: &Polynomial| p.terms.keys().max_by(|x, y| grevlex(x, y)).unwrap().clone();
            let left = leading(&basis[i]);
            let right = leading(&basis[j]);
            let lcm: Vec<_> = left.iter().zip(&right).map(|(x, y)| (*x).max(*y)).collect();
            let multiplier = |m: &[u64]| Polynomial::from_terms(ring.nvars(),
                [(lcm.iter().zip(m).map(|(x, y)| x - y).collect(), 1)]).unwrap();
            let pair = multiplier(&left).mul(&basis[i]).unwrap()
                .sub(&multiplier(&right).mul(&basis[j]).unwrap()).unwrap();
            assert!(divide(pair, &basis).is_zero(), "S-pair {i},{j}");
            pairs += 1;
        } }
        assert_eq!(pairs, 28);
    }

    #[test]
    fn m31_64_cases_per_block_count_match_independent_long_division() {
        let mut seed = 20_261_123;
        for blocks in 1..=3 {
            let ring = Ring::new(blocks, 2).unwrap();
            let basis: Vec<_> = (0..blocks).flat_map(|b| ring.chart_generators(b).unwrap()).collect();
            for _ in 0..64 {
                let polynomial = random_poly(&ring, &mut seed);
                let normal = ring.normal_form(&polynomial).unwrap();
                assert_eq!(normal, divide(polynomial.clone(), &basis));
                assert!(normal.degree() <= polynomial.degree());
                assert_eq!(ring.normal_form(&normal).unwrap(), normal);
            }
        }
    }

    #[test]
    fn m31_64_cases_per_block_count_preserve_chart_evaluation_and_multiplication() {
        let mut seed = 0x4d33315f63686172;
        for blocks in 1..=3 {
            let ring = Ring::new(blocks, 2).unwrap();
            for _ in 0..64 {
                let mut point = Vec::new();
                for _ in 0..blocks {
                    if next(&mut seed) & 1 == 0 {
                        point.extend([0, 0, 0]);
                    } else {
                        let w = next(&mut seed) % M31;
                        point.extend([1, w, ((u128::from(w) * u128::from(w)) % u128::from(M31)) as u64]);
                    }
                }
                point.extend([next(&mut seed) % M31, next(&mut seed) % M31]);
                let f = random_poly(&ring, &mut seed);
                let g = random_poly(&ring, &mut seed);
                let normal = ring.normal_form(&f).unwrap();
                assert_eq!(f.evaluate(&point).unwrap(), normal.evaluate(&point).unwrap());
                let product = ring.multiply(&f, &g).unwrap();
                assert_eq!(product, ring.normal_form(&f.mul(&g).unwrap()).unwrap());
                assert_eq!(product.evaluate(&point).unwrap(), field_mul(f.evaluate(&point).unwrap(), g.evaluate(&point).unwrap()));
            }
        }
    }

    #[test]
    fn m31_frobenius_preserves_geometry_and_does_not_add_field_equations() {
        let ring = Ring::new(1, 1).unwrap();
        let z = Polynomial::variable(4, 0);
        let w = Polynomial::variable(4, 1);
        let s = Polynomial::variable(4, 2);
        let x = Polynomial::variable(4, 3);
        assert_eq!(ring.frobenius(&z, 1).unwrap(), z);
        for iterations in 1..=2 {
            let q = M31.pow(iterations);
            let expected = Polynomial::from_terms(4, [(vec![0, 1, (q - 1) / 2, 0], 1)]).unwrap();
            assert_eq!(ring.frobenius(&w, iterations).unwrap(), expected);
            assert_ne!(expected, w);
            assert_ne!(ring.frobenius(&s, iterations).unwrap(), s);
            assert_ne!(ring.frobenius(&x, iterations).unwrap(), x);
            let f = w.add(&z.scale(3)).unwrap().add(&x).unwrap();
            let frobenius = ring.frobenius(&f, iterations).unwrap();
            assert_eq!(frobenius.evaluate(&[1, 7, 49, 11]).unwrap(), f.evaluate(&[1, 7, 49, 11]).unwrap());
        }
        assert!(ring.frobenius(&w, 3).is_err());
    }

    #[test]
    fn m31_standard_monomial_counts_and_full_multiplier_coverage() {
        let ring = Ring::new(1, 0).unwrap();
        for degree in 1..=12 {
            assert_eq!(ring.monomials(degree, 1000).unwrap().len(), 2 * degree as usize + 2);
        }
        let ring = Ring::new(2, 2).unwrap();
        let candidates = Ring::new(0, ring.nvars()).unwrap().monomials(4, 1000).unwrap();
        let reduced: BTreeSet<_> = candidates.iter().map(|m| ring.normal_monomial(m).unwrap()).collect();
        let direct: BTreeSet<_> = ring.monomials(4, 1000).unwrap().into_iter().collect();
        assert_eq!(direct, reduced);
        assert!(ring.monomials(20, 8).is_err());
    }

    #[test]
    fn m31_macaulay_rows_retain_zero_divisor_products_and_actual_degrees() {
        let ring = Ring::new(1, 0).unwrap();
        let one = Polynomial::constant(3, 1);
        let z = Polynomial::variable(3, 0);
        let w = Polynomial::variable(3, 1);
        let f = w.add(&one).unwrap();
        let matrix = ring.macaulay(&[f.clone()], 2, MacaulayLimits::default()).unwrap();
        assert_eq!(matrix.rows.len(), 4);
        let row = |m: &[u64]| &matrix.rows.iter().find(|r| r.multiplier == m).unwrap().row;
        // (1-z)*(w+1)=1-z is necessary to remove the infinity component.
        // A principal-ideal algorithm with no quotient pairs misses this row.
        assert_eq!(row(&[0, 0, 0]).sub(row(&[1, 0, 0])).unwrap(), one.sub(&z).unwrap());
        assert_eq!(matrix.max_unreduced_product_degree, 2);
        assert_eq!(matrix.max_row_degree, 2);
        for r in &matrix.rows {
            let multiplier = Polynomial::from_terms(3, [(r.multiplier.clone(), 1)]).unwrap();
            let basis = ring.chart_generators(0).unwrap();
            assert_eq!(r.row, divide(multiplier.mul(&f).unwrap(), &basis));
            assert!(r.row_degree <= r.unreduced_product_degree);
        }
        assert!(ring.macaulay(&[f], 2, MacaulayLimits { max_rows: 1, ..MacaulayLimits::default() }).is_err());
    }

    #[test]
    fn m31_canonical_boundary_rejects_bad_width_and_overflow() {
        let ring = Ring::new(1, 0).unwrap();
        assert!(ring.normal_monomial(&[0, 0]).is_err());
        let malformed = Polynomial { variables: 3, terms: BTreeMap::from([(vec![0, 0], 1)]) };
        assert!(ring.normal_form(&malformed).is_err());
        let large = Polynomial::from_terms(3, [(vec![0, u64::MAX, 0], 1)]).unwrap();
        assert!(large.mul(&Polynomial::variable(3, 1)).is_err());
        assert!(large.to_u32_terms().is_err());
        let x = Polynomial::variable(3, 1);
        assert_eq!(x.pow(3).unwrap().derivative(1).unwrap(), x.pow(2).unwrap().scale(3));
        assert_eq!(x.sub(&x).unwrap(), Polynomial::zero(3));
    }
}
