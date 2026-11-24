//! Bounded, experimental F4 in the fixed projective chart quotient over M31.
//!
//! This follows the repository's `src/cryptanalysis/f4_fp.rs` normal strategy:
//! select minimum-lcm-degree pairs, close the matrix support under reducers,
//! perform exact row elimination, and adjoin rows with new leading terms.
//! It is a small reference prototype, not the production F4 engine. The order
//! is graded reverse lexicographic, with the variable order of `Ring`.
//!
//! The ambient ideal is `J + <inputs>`. All four chart generators per block
//! remain fixed in the basis and **all** pairs are retained, including J/G
//! pairs. There is no product, chain, or principal-ideal criterion in a ring
//! with zero divisors. Products and matrix rows are reduced modulo J; this is
//! an ideal-preserving transformation, not a field-equation substitution.
//!
//! A complete result means the pair queue was exhausted. A resource limit
//! returns an explicitly incomplete result, never an empty solution set.
//! This module computes a basis, not roots or signed point decompositions.
//! Diagnostics distinguish raw product degree from degree after NF_J and are
//! construction counters, not timing, RSS, or a general solving-degree bound.
//!
//! `verify` is deliberately separate: ordinary polynomial long division and
//! textbook Buchberger on J plus the ORIGINAL inputs, with no quotient normal
//! form and no F4 matrices. It checks all candidate S-pairs and both ideal
//! containments. Thus a forged unit basis cannot pass just by containing every
//! input. Verification is bounded too; its limit is not a successful check.

use crate::quotient::{Error, Poly, Ring};
use crate::M31;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

type Monomial = Vec<u64>;
type Pair = (u64, usize, usize);

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Total pairs scheduled, including fixed/fixed pairs.
    pub max_pairs: usize,
    /// Unique product rows in one symbolic matrix, including zero rows.
    pub max_rows: usize,
    pub max_columns: usize,
    /// Retained basis entries, input matrix entries, and live sparse-
    /// elimination entries, each separately.
    /// This is not a bound on the process's RSS.
    pub max_nonzeros: usize,
    /// Fixed and nonfixed polynomials together.
    pub max_basis: usize,
    /// Raw degree BEFORE normal form, including the fixed generators.
    pub max_degree: u64,
    /// Producer: matrix batches. Independent verifier: original-ideal
    /// Buchberger pair steps. Supply separate limit values when comparing
    /// these distinct operations; they are not interchangeable work units.
    pub max_steps: usize,
    /// Polynomial/pivot subtraction operations, over the whole computation.
    pub max_reductions: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_pairs: 50_000,
            max_rows: 10_000,
            max_columns: 20_000,
            max_nonzeros: 1_000_000,
            max_basis: 1_000,
            max_degree: 64,
            max_steps: 10_000,
            max_reductions: 1_000_000,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Complete,
    Incomplete { reason: &'static str },
}

#[derive(Clone, Debug, Default)]
pub struct MatrixStat {
    pub pair_degree: u64,
    pub selected_pairs: usize,
    pub product_rows: usize,
    pub zero_rows: usize,
    pub reducer_rows: usize,
    pub columns: usize,
    pub input_nonzeros: usize,
    pub peak_elimination_nonzeros: usize,
    pub max_raw_product_degree: u64,
    pub max_post_nf_column_degree: u64,
    pub new_basis_polynomials: usize,
    /// Includes normalization and coefficient products during elimination;
    /// monomial operations, additions, and NF_J additions are not counted.
    pub field_multiplications: u64,
    pub completed: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Statistics {
    pub scheduled_pairs: usize,
    pub processed_pairs: usize,
    pub pending_pairs: usize,
    pub polynomial_reductions: usize,
    pub max_raw_product_degree: u64,
    pub max_post_nf_matrix_degree: u64,
    /// Same explicit arithmetic scope as MatrixStat; also includes preparing
    /// input generators and reducing newly adjoined polynomials.
    pub field_multiplications: u64,
}

#[derive(Clone, Debug)]
pub struct Report {
    pub status: Status,
    /// Nonfixed, monic NF_J polynomials; add the four fixed generators per
    /// block to obtain the ambient basis. It is not necessarily reduced.
    pub basis: Vec<Poly>,
    pub fixed_basis_len: usize,
    pub raw_generator_degrees: Vec<u64>,
    pub prepared_generator_degrees: Vec<u64>,
    pub matrices: Vec<MatrixStat>,
    pub statistics: Statistics,
}

impl Report {
    pub fn is_complete(&self) -> bool {
        self.status == Status::Complete
    }

    /// A nonzero constant is a certified inconsistency of the polynomial
    /// ideal even if a later resource limit prevented queue completion.
    /// Absence of a constant says nothing about the rational solution set.
    pub fn contains_unit(&self) -> bool {
        self.basis.iter().any(|g| !g.is_zero() && g.degree() == 0)
    }
}

fn degree(m: &[u64]) -> Result<u64, Error> {
    m.iter().try_fold(0u64, |s, e| s.checked_add(*e).ok_or(Error::ExponentOverflow))
}

fn grevlex(a: &[u64], b: &[u64]) -> Ordering {
    // Every entry point and constructed exponent vector is checked first.
    degree(a).unwrap().cmp(&degree(b).unwrap()).then_with(|| {
        a.iter().rev().zip(b.iter().rev()).find_map(|(x, y)| {
            if x == y { None } else { Some(y.cmp(x)) }
        }).unwrap_or(Ordering::Equal)
    })
}

fn leading(p: &Poly) -> Option<(Monomial, u64)> {
    p.terms.iter().max_by(|(a, _), (b, _)| grevlex(a, b))
        .map(|(m, c)| (m.clone(), *c))
}

fn divides(a: &[u64], b: &[u64]) -> bool {
    a.iter().zip(b).all(|(x, y)| x <= y)
}

fn quotient(a: &[u64], b: &[u64]) -> Monomial {
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}

fn lcm(a: &[u64], b: &[u64]) -> Monomial {
    a.iter().zip(b).map(|(x, y)| (*x).max(*y)).collect()
}

fn bound(value: usize, cap: usize, name: &'static str) -> Result<(), Error> {
    if value > cap { Err(Error::Limit(name)) } else { Ok(()) }
}

fn add_count(value: &mut usize, add: usize, cap: usize, name: &'static str) -> Result<(), Error> {
    let next = value.checked_add(add).ok_or(Error::Limit(name))?;
    bound(next, cap, name)?;
    *value = next;
    Ok(())
}

fn fp_mul(a: u64, b: u64, stats: &mut Statistics) -> Result<u64, Error> {
    stats.field_multiplications = stats.field_multiplications.checked_add(1)
        .ok_or(Error::Limit("field multiplication counter"))?;
    Ok(((u128::from(a) * u128::from(b)) % u128::from(M31)) as u64)
}

fn fp_inverse(a: u64, stats: &mut Statistics) -> Result<u64, Error> {
    if a == 1 { return Ok(1); }
    let mut e = M31 - 2;
    let mut base = a;
    let mut out = 1;
    while e != 0 {
        if e & 1 != 0 { out = fp_mul(out, base, stats)?; }
        e >>= 1;
        if e != 0 { base = fp_mul(base, base, stats)?; }
    }
    Ok(out)
}

fn monic(mut p: Poly, stats: &mut Statistics) -> Result<Poly, Error> {
    if let Some((_, c)) = leading(&p) {
        if c != 1 {
            let inv = fp_inverse(c, stats)?;
            for value in p.terms.values_mut() { *value = fp_mul(*value, inv, stats)?; }
        }
    }
    Ok(p)
}

fn subtract_scaled(p: &mut Poly, q: &Poly, factor: u64, limits: Limits,
                   term_cap: usize, stats: &mut Statistics) -> Result<usize, Error> {
    add_count(&mut stats.polynomial_reductions, 1, limits.max_reductions,
              "polynomial reductions")?;
    let mut peak = p.terms.len();
    for (m, c) in &q.terms {
        let product = fp_mul(factor, *c, stats)?;
        let next = (p.terms.get(m).copied().unwrap_or(0) + M31 - product) % M31;
        if next == 0 {
            p.terms.remove(m);
        } else {
            if !p.terms.contains_key(m) {
                bound(p.terms.len().saturating_add(1), term_cap, "sparse live nonzeros")?;
            }
            p.terms.insert(m.clone(), next);
        }
        peak = peak.max(p.terms.len());
    }
    Ok(peak)
}

fn ordinary_shift(p: &Poly, shift: &[u64]) -> Result<Poly, Error> {
    let mut terms = BTreeMap::new();
    for (m, c) in &p.terms {
        let e: Monomial = m.iter().zip(shift).map(|(x, y)| {
            x.checked_add(*y).ok_or(Error::ExponentOverflow)
        }).collect::<Result<_, _>>()?;
        degree(&e)?;
        terms.insert(e, *c);
    }
    Ok(Poly { variables: p.variables, terms })
}

struct ProductRow {
    poly: Poly,
    reducer: bool,
}

struct Engine<'a> {
    ring: &'a Ring,
    limits: Limits,
    basis: Vec<Poly>,
    leads: Vec<Monomial>,
    fixed: usize,
    basis_nonzeros: usize,
    pairs: BTreeSet<Pair>,
    report: Report,
}

impl Engine<'_> {
    fn check_degree(&self, d: u64) -> Result<(), Error> {
        if d > self.limits.max_degree { Err(Error::Limit("raw polynomial degree")) } else { Ok(()) }
    }

    fn product(&mut self, i: usize, shift: &[u64]) -> Result<(Poly, u64), Error> {
        let raw = self.basis[i].degree().checked_add(degree(shift)?).ok_or(Error::ExponentOverflow)?;
        self.check_degree(raw)?;
        self.report.statistics.max_raw_product_degree = self.report.statistics.max_raw_product_degree.max(raw);
        let out = self.ring.normal_form(&ordinary_shift(&self.basis[i], shift)?)?;
        bound(out.terms.len(), self.limits.max_nonzeros, "product nonzeros")?;
        Ok((out, raw))
    }

    fn enqueue(&mut self, i: usize, j: usize) -> Result<(), Error> {
        let d = degree(&lcm(&self.leads[i], &self.leads[j]))?;
        add_count(&mut self.report.statistics.scheduled_pairs, 1,
                  self.limits.max_pairs, "scheduled critical pairs")?;
        self.pairs.insert((d, i, j));
        Ok(())
    }

    fn append(&mut self, p: Poly) -> Result<(), Error> {
        if p.is_zero() { return Ok(()); }
        bound(self.basis.len().saturating_add(1), self.limits.max_basis, "basis polynomials")?;
        let (lm, coefficient) = leading(&p).unwrap();
        if coefficient != 1 { return Err(Error::Invalid("nonmonic internal basis".into())); }
        add_count(&mut self.basis_nonzeros, p.terms.len(), self.limits.max_nonzeros,
                  "retained basis nonzeros")?;
        let index = self.basis.len();
        self.basis.push(p);
        self.leads.push(lm);
        for i in 0..index { self.enqueue(i, index)?; }
        Ok(())
    }

    /// Full division in the quotient. For a canonical monomial divisible by
    /// LM(g), NF_J((m/LM(g))*g) still has leading monomial m: the chart GB
    /// decreases the ambient order and cannot destroy that canonical term.
    fn reduce(&mut self, mut p: Poly) -> Result<Poly, Error> {
        let mut remainder = Poly::zero(self.ring.nvars());
        while let Some((m, coefficient)) = leading(&p) {
            if let Some(i) = (self.fixed..self.basis.len()).find(|i| divides(&self.leads[*i], &m)) {
                let shift = quotient(&m, &self.leads[i]);
                let (g, _) = self.product(i, &shift)?;
                if leading(&g) != Some((m, 1)) {
                    return Err(Error::Invalid("quotient reducer lost its canonical leading term".into()));
                }
                subtract_scaled(&mut p, &g, coefficient, self.limits,
                                self.limits.max_nonzeros, &mut self.report.statistics)?;
            } else {
                p.terms.remove(&m);
                bound(remainder.terms.len().saturating_add(1), self.limits.max_nonzeros,
                      "remainder nonzeros")?;
                remainder.terms.insert(m, coefficient);
            }
        }
        monic(remainder, &mut self.report.statistics)
    }

    fn add_product(&mut self, i: usize, shift: Monomial, reducer: bool,
                   rows: &mut Vec<ProductRow>, seen: &mut BTreeMap<(usize, Monomial), usize>,
                   support: &mut BTreeSet<Monomial>, queue: &mut Vec<Monomial>,
                   stat: &mut MatrixStat) -> Result<(), Error> {
        if let Some(&index) = seen.get(&(i, shift.clone())) {
            rows[index].reducer |= reducer;
            return Ok(());
        }
        bound(rows.len().saturating_add(1), self.limits.max_rows, "symbolic product rows")?;
        let (poly, raw) = self.product(i, &shift)?;
        stat.max_raw_product_degree = stat.max_raw_product_degree.max(raw);
        add_count(&mut stat.input_nonzeros, poly.terms.len(), self.limits.max_nonzeros,
                  "input matrix nonzeros")?;
        stat.product_rows += 1;
        stat.zero_rows += usize::from(poly.is_zero());
        for m in poly.terms.keys() {
            stat.max_post_nf_column_degree = stat.max_post_nf_column_degree.max(degree(m)?);
            if !support.contains(m) {
                bound(support.len().saturating_add(1), self.limits.max_columns, "matrix columns")?;
                support.insert(m.clone());
                queue.push(m.clone());
                stat.columns = support.len();
            }
        }
        seen.insert((i, shift), rows.len());
        rows.push(ProductRow { poly, reducer });
        self.report.statistics.max_post_nf_matrix_degree = self.report.statistics
            .max_post_nf_matrix_degree.max(stat.max_post_nf_column_degree);
        Ok(())
    }

    fn matrix(&mut self, selected: &[Pair], stat: &mut MatrixStat) -> Result<(), Error> {
        let mut rows = Vec::new();
        let mut seen = BTreeMap::new();
        let mut support = BTreeSet::new();
        let mut queue = Vec::new();
        for &(_, i, j) in selected {
            let common = lcm(&self.leads[i], &self.leads[j]);
            for index in [i, j] {
                let shift = quotient(&common, &self.leads[index]);
                self.add_product(index, shift, false, &mut rows, &mut seen,
                                 &mut support, &mut queue, stat)?;
            }
        }
        let mut done = BTreeSet::new();
        while let Some(m) = queue.pop() {
            if !done.insert(m.clone()) { continue; }
            if let Some(i) = (self.fixed..self.basis.len()).find(|i| divides(&self.leads[*i], &m)) {
                let shift = quotient(&m, &self.leads[i]);
                self.add_product(i, shift, true, &mut rows, &mut seen,
                                 &mut support, &mut queue, stat)?;
            }
        }
        stat.columns = support.len();
        stat.reducer_rows = rows.iter().filter(|r| r.reducer).count();
        self.report.statistics.max_post_nf_matrix_degree = self.report.statistics
            .max_post_nf_matrix_degree.max(stat.max_post_nf_column_degree);

        // Reducers first: one monic row for every old-reducible column.
        // Their leading terms are distinct. Ordinary sparse echelon reduction
        // then leaves novel pivots only in columns outside the old LM ideal.
        rows.sort_by(|a, b| b.reducer.cmp(&a.reducer).then_with(|| {
            match (leading(&a.poly), leading(&b.poly)) {
                (Some((x, _)), Some((y, _))) => grevlex(&y, &x),
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                _ => Ordering::Equal,
            }
        }));
        let mut pivots = BTreeMap::<Monomial, Poly>::new();
        let mut waiting = stat.input_nonzeros;
        let mut pivot_terms = 0usize;
        stat.peak_elimination_nonzeros = waiting;
        for row in rows {
            let mut p = row.poly;
            waiting -= p.terms.len();
            while let Some((m, c)) = leading(&p) {
                if let Some(g) = pivots.get(&m) {
                    let occupied = waiting.checked_add(pivot_terms).ok_or(Error::Limit("sparse live nonzeros"))?;
                    let allowance = self.limits.max_nonzeros.checked_sub(occupied)
                        .ok_or(Error::Limit("sparse live nonzeros"))?;
                    let row_peak = subtract_scaled(&mut p, g, c, self.limits, allowance, &mut self.report.statistics)?;
                    stat.peak_elimination_nonzeros = stat.peak_elimination_nonzeros.max(occupied + row_peak);
                } else {
                    let p = monic(p, &mut self.report.statistics)?;
                    pivot_terms = pivot_terms.checked_add(p.terms.len()).ok_or(Error::Limit("sparse live nonzeros"))?;
                    bound(waiting.saturating_add(pivot_terms), self.limits.max_nonzeros, "sparse live nonzeros")?;
                    pivots.insert(m, p);
                    break;
                }
            }
        }
        let old_length = self.basis.len();
        let mut candidates: Vec<_> = pivots.into_iter().filter(|(m, _)| {
            !(self.fixed..old_length).any(|i| divides(&self.leads[i], m))
        }).collect();
        candidates.sort_by(|(a, _), (b, _)| grevlex(b, a));
        for (_, p) in candidates {
            let reduced = self.reduce(p)?;
            if !reduced.is_zero() {
                self.append(reduced)?;
                stat.new_basis_polynomials += 1;
            }
        }
        Ok(())
    }

    fn run(&mut self, inputs: &[Poly]) -> Result<(), Error> {
        for block in 0..self.ring.block_count() {
            for p in self.ring.chart_generators(block)? {
                self.check_degree(p.degree())?;
                self.append(p)?;
            }
        }
        self.fixed = self.basis.len();
        self.report.fixed_basis_len = self.fixed;
        for p in inputs {
            self.check_degree(p.degree())?;
            bound(p.terms.len(), self.limits.max_nonzeros, "input polynomial nonzeros")?;
            let prepared = self.ring.normal_form(p)?;
            self.report.prepared_generator_degrees.push(prepared.degree());
            let reduced = self.reduce(prepared)?;
            self.append(reduced)?;
        }
        while let Some(&(d, _, _)) = self.pairs.iter().next() {
            bound(self.report.matrices.len().saturating_add(1), self.limits.max_steps, "matrix steps")?;
            self.check_degree(d)?;
            let selected: Vec<_> = self.pairs.iter().take_while(|p| p.0 == d).copied().collect();
            let mut stat = MatrixStat { pair_degree: d, selected_pairs: selected.len(), ..MatrixStat::default() };
            let before = self.report.statistics.field_multiplications;
            let outcome = self.matrix(&selected, &mut stat);
            stat.field_multiplications = self.report.statistics.field_multiplications - before;
            stat.completed = outcome.is_ok();
            self.report.matrices.push(stat);
            outcome?;
            for p in selected { self.pairs.remove(&p); self.report.statistics.processed_pairs += 1; }
        }
        Ok(())
    }
}

pub fn compute(ring: &Ring, inputs: &[Poly], limits: Limits) -> Result<Report, Error> {
    for p in inputs {
        p.validate()?;
        if p.variables != ring.nvars() { return Err(Error::Invalid("F4 input ring width".into())); }
    }
    let report = Report {
        status: Status::Complete, basis: Vec::new(), fixed_basis_len: 0,
        raw_generator_degrees: inputs.iter().map(Poly::degree).collect(),
        prepared_generator_degrees: Vec::new(), matrices: Vec::new(), statistics: Statistics::default(),
    };
    let mut engine = Engine { ring, limits, basis: Vec::new(), leads: Vec::new(), fixed: 0, basis_nonzeros: 0,
                              pairs: BTreeSet::new(), report };
    match engine.run(inputs) {
        Ok(()) => {},
        Err(Error::Limit(reason)) => engine.report.status = Status::Incomplete { reason },
        Err(error) => return Err(error),
    }
    // A limit while constructing J must not expose chart generators as
    // purported nonfixed answers. No completeness is claimed in that case.
    let actual_fixed = engine.fixed.max(engine.basis.len().min(ring.block_count() * 4));
    engine.report.fixed_basis_len = actual_fixed;
    engine.report.basis = engine.basis.into_iter().skip(actual_fixed).collect();
    engine.report.statistics.pending_pairs = engine.pairs.len();
    Ok(engine.report)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationStatus {
    Verified,
    Rejected(String),
    Incomplete { reason: &'static str },
}

#[derive(Clone, Debug)]
pub struct Verification {
    pub status: VerificationStatus,
    pub candidate_pairs_checked: usize,
    pub oracle_pairs_processed: usize,
    pub original_generators_checked: usize,
    pub candidate_generators_checked: usize,
    pub oracle_basis_len: usize,
}

// Independent ordinary Buchberger oracle. These methods neither use Engine,
// its reducers/matrices, nor Ring::normal_form/normal_monomial/multiply.
struct OrdinaryOracle {
    limits: Limits,
    reductions: usize,
    pairs: usize,
}

impl OrdinaryOracle {
    fn lm(&self, p: &Poly) -> Option<(Monomial, u64)> {
        p.terms.iter().max_by(|(a, _), (b, _)| {
            let da: u64 = a.iter().sum();
            let db: u64 = b.iter().sum();
            da.cmp(&db).then_with(|| {
                for k in (0..a.len()).rev() {
                    if a[k] != b[k] { return b[k].cmp(&a[k]); }
                }
                Ordering::Equal
            })
        }).map(|(m, c)| (m.clone(), *c))
    }

    fn inverse(&self, a: u64) -> u64 {
        let (mut e, mut b, mut r) = (M31 - 2, a, 1u64);
        while e != 0 {
            if e % 2 == 1 { r = ((u128::from(r) * u128::from(b)) % u128::from(M31)) as u64; }
            b = ((u128::from(b) * u128::from(b)) % u128::from(M31)) as u64;
            e /= 2;
        }
        r
    }

    fn normalize(&self, mut p: Poly) -> Poly {
        if let Some((_, c)) = self.lm(&p) {
            let inverse = self.inverse(c);
            for c in p.terms.values_mut() { *c = ((u128::from(*c) * u128::from(inverse)) % u128::from(M31)) as u64; }
        }
        p
    }

    fn add_multiple(&mut self, p: &mut Poly, g: &Poly, shift: &[u64], factor: u64) -> Result<(), Error> {
        add_count(&mut self.reductions, 1, self.limits.max_reductions, "ordinary verification reductions")?;
        for (m, c) in &g.terms {
            let m: Vec<_> = m.iter().zip(shift).map(|(x, y)| x.checked_add(*y).ok_or(Error::ExponentOverflow))
                .collect::<Result<_, _>>()?;
            if degree(&m)? > self.limits.max_degree { return Err(Error::Limit("ordinary verification degree")); }
            let value = ((u128::from(*c) * u128::from(factor)) % u128::from(M31)) as u64;
            let value = (p.terms.get(&m).copied().unwrap_or(0) + value) % M31;
            if value == 0 { p.terms.remove(&m); }
            else {
                if !p.terms.contains_key(&m) {
                    bound(p.terms.len().saturating_add(1), self.limits.max_nonzeros, "ordinary verification terms")?;
                }
                p.terms.insert(m, value);
            }
        }
        Ok(())
    }

    fn remainder(&mut self, mut p: Poly, basis: &[Poly]) -> Result<Poly, Error> {
        if p.degree() > self.limits.max_degree { return Err(Error::Limit("ordinary verification degree")); }
        bound(p.terms.len(), self.limits.max_nonzeros, "ordinary verification terms")?;
        let mut rem = Poly::zero(p.variables);
        while let Some((m, c)) = self.lm(&p) {
            let mut divisor = None;
            for g in basis {
                if let Some((gm, gc)) = self.lm(g) {
                    if (0..m.len()).all(|k| gm[k] <= m[k]) { divisor = Some((g, gm, gc)); break; }
                }
            }
            if let Some((g, gm, gc)) = divisor {
                let shift: Vec<_> = (0..m.len()).map(|k| m[k] - gm[k]).collect();
                let scalar = ((u128::from(c) * u128::from(self.inverse(gc))) % u128::from(M31)) as u64;
                self.add_multiple(&mut p, g, &shift, M31 - scalar)?;
            } else {
                p.terms.remove(&m);
                rem.terms.insert(m, c);
                bound(rem.terms.len(), self.limits.max_nonzeros, "ordinary verification terms")?;
            }
        }
        Ok(rem)
    }

    fn spoly(&mut self, a: &Poly, b: &Poly) -> Result<Poly, Error> {
        add_count(&mut self.pairs, 1, self.limits.max_pairs, "ordinary verification pairs")?;
        let (am, ac) = self.lm(a).unwrap();
        let (bm, bc) = self.lm(b).unwrap();
        let common: Vec<_> = (0..am.len()).map(|k| am[k].max(bm[k])).collect();
        if degree(&common)? > self.limits.max_degree { return Err(Error::Limit("ordinary verification pair degree")); }
        let sa: Vec<_> = (0..am.len()).map(|k| common[k] - am[k]).collect();
        let sb: Vec<_> = (0..bm.len()).map(|k| common[k] - bm[k]).collect();
        let mut result = Poly::zero(a.variables);
        self.add_multiple(&mut result, a, &sa, self.inverse(ac))?;
        self.add_multiple(&mut result, b, &sb, M31 - self.inverse(bc))?;
        Ok(result)
    }

    fn buchberger(&mut self, original: &[Poly], verification: &mut Verification) -> Result<Vec<Poly>, Error> {
        let mut basis = Vec::<Poly>::new();
        let mut queue = VecDeque::new();
        for f in original {
            let g = self.remainder(f.clone(), &basis)?;
            if g.is_zero() { continue; }
            bound(basis.len().saturating_add(1), self.limits.max_basis, "ordinary verification basis")?;
            let k = basis.len();
            for i in 0..k {
                bound(queue.len().saturating_add(1), self.limits.max_pairs, "ordinary pending pairs")?;
                queue.push_back((i, k));
            }
            basis.push(self.normalize(g));
        }
        while let Some((i, j)) = queue.pop_front() {
            bound(verification.oracle_pairs_processed.saturating_add(1), self.limits.max_steps,
                  "ordinary verification steps")?;
            let s = self.spoly(&basis[i], &basis[j])?;
            let g = self.remainder(s, &basis)?;
            verification.oracle_pairs_processed += 1;
            if g.is_zero() { continue; }
            bound(basis.len().saturating_add(1), self.limits.max_basis, "ordinary verification basis")?;
            let k = basis.len();
            for i in 0..k {
                bound(queue.len().saturating_add(1), self.limits.max_pairs, "ordinary pending pairs")?;
                queue.push_back((i, k));
            }
            basis.push(self.normalize(g));
        }
        Ok(basis)
    }
}

fn independent_chart_generators(ring: &Ring) -> Result<Vec<Poly>, Error> {
    let mut out = Vec::new();
    let n = ring.nvars();
    for b in 0..ring.block_count() {
        let j = 3 * b;
        for (left, right) in [((j, j), j), ((j, j + 1), j + 1),
                              ((j, j + 2), j + 2), ((j + 1, j + 1), j + 2)] {
            let mut a = vec![0; n];
            a[left.0] += 1; a[left.1] += 1;
            let mut c = vec![0; n]; c[right] = 1;
            out.push(Poly::from_terms(n, [(a, 1), (c, M31 - 1)])?);
        }
    }
    Ok(out)
}

pub fn verify(ring: &Ring, inputs: &[Poly], report: &Report, limits: Limits) -> Result<Verification, Error> {
    let mut v = Verification { status: VerificationStatus::Verified, candidate_pairs_checked: 0,
        oracle_pairs_processed: 0, original_generators_checked: 0,
        candidate_generators_checked: 0, oracle_basis_len: 0 };
    if !report.is_complete() {
        v.status = VerificationStatus::Rejected("producer report is incomplete".into());
        return Ok(v);
    }
    for p in inputs.iter().chain(&report.basis) {
        p.validate()?;
        if p.variables != ring.nvars() { return Err(Error::Invalid("verification polynomial width".into())); }
    }
    let fixed = independent_chart_generators(ring)?;
    if report.fixed_basis_len != fixed.len() {
        v.status = VerificationStatus::Rejected("fixed chart basis count".into());
        return Ok(v);
    }
    let mut oracle = OrdinaryOracle { limits, reductions: 0, pairs: 0 };
    let result = (|| -> Result<(), Error> {
        let candidate: Vec<_> = fixed.iter().chain(&report.basis).filter(|p| !p.is_zero()).cloned().collect();
        bound(candidate.len(), limits.max_basis, "verification candidate basis")?;
        for i in 0..candidate.len() { for j in i + 1..candidate.len() {
            let s = oracle.spoly(&candidate[i], &candidate[j])?;
            let r = oracle.remainder(s, &candidate)?;
            v.candidate_pairs_checked += 1;
            if !r.is_zero() {
                v.status = VerificationStatus::Rejected(format!("candidate S-pair {i},{j} has nonzero remainder"));
                return Ok(());
            }
        } }
        for f in inputs {
            let r = oracle.remainder(f.clone(), &candidate)?;
            v.original_generators_checked += 1;
            if !r.is_zero() {
                v.status = VerificationStatus::Rejected("original generator not in candidate ideal".into());
                return Ok(());
            }
        }
        let original: Vec<_> = fixed.iter().chain(inputs).cloned().collect();
        let original_gb = oracle.buchberger(&original, &mut v)?;
        v.oracle_basis_len = original_gb.len();
        for f in &report.basis {
            let r = oracle.remainder(f.clone(), &original_gb)?;
            v.candidate_generators_checked += 1;
            if !r.is_zero() {
                v.status = VerificationStatus::Rejected("candidate generator not in original ideal".into());
                return Ok(());
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => {},
        Err(Error::Limit(reason)) => v.status = VerificationStatus::Incomplete { reason },
        Err(e) => return Err(e),
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn checked(ring: &Ring, generators: &[Poly]) -> Report {
        let result = compute(ring, generators, Limits::default()).unwrap();
        assert!(result.is_complete(), "{:?}", result.status);
        assert_eq!(result.statistics.pending_pairs, 0);
        assert_eq!(result.statistics.scheduled_pairs, result.statistics.processed_pairs);
        let verification = verify(ring, generators, &result, Limits::default()).unwrap();
        assert_eq!(verification.status, VerificationStatus::Verified, "{verification:?}");
        result
    }

    #[test]
    fn m31_zero_divisor_pairs_remove_spurious_infinity_branch() {
        let ring = Ring::new(1, 0).unwrap();
        let z = Poly::variable(3, 0);
        let w = Poly::variable(3, 1);
        let one = Poly::constant(3, 1);
        let f = w.add(&one).unwrap();
        let r = checked(&ring, &[f.clone()]);
        let mut ordinary = OrdinaryOracle { limits: Limits::default(), reductions: 0, pairs: 0 };
        let mut ambient = independent_chart_generators(&ring).unwrap();
        ambient.extend(r.basis.clone());
        assert!(ordinary.remainder(one.sub(&z).unwrap(), &ambient).unwrap().is_zero());
        assert!(!r.contains_unit());
        assert!(r.statistics.max_raw_product_degree >= 2);
        assert!(r.matrices.iter().any(|m| m.max_raw_product_degree > m.max_post_nf_column_degree));

        // Omitting J/G pairs would leave only <w+1>; the independent checker
        // must reject that false completion even though the input is present.
        let mut incomplete_basis = r.clone();
        incomplete_basis.basis = vec![f];
        assert!(matches!(verify(&ring, &[incomplete_basis.basis[0].clone()], &incomplete_basis,
                                Limits::default()).unwrap().status, VerificationStatus::Rejected(_)));
    }

    #[test]
    fn m31_two_coupled_charts() {
        let ring = Ring::new(2, 0).unwrap();
        let w0 = Poly::variable(6, 1);
        let s0 = Poly::variable(6, 2);
        let w1 = Poly::variable(6, 4);
        let s1 = Poly::variable(6, 5);
        let inputs = [w0.sub(&w1).unwrap(), s0.add(&s1).unwrap().sub(&Poly::constant(6, 2)).unwrap()];
        let r = checked(&ring, &inputs);
        assert!(!r.contains_unit());
        for values in [[1, 1, 1, 1, 1, 1], [1, M31 - 1, 1, 1, M31 - 1, 1]] {
            assert!(r.basis.iter().all(|p| p.evaluate(&values).unwrap() == 0));
        }
    }

    #[test]
    fn m31_no_chart_blocks_is_ordinary_f4() {
        let ring = Ring::new(0, 2).unwrap();
        let x = Poly::variable(2, 0);
        let y = Poly::variable(2, 1);
        let inputs = [x.mul(&x).unwrap().scale(2).sub(&y).unwrap(),
                      x.mul(&y).unwrap().sub(&Poly::constant(2, 1)).unwrap()];
        let r = checked(&ring, &inputs);
        assert_eq!(r.fixed_basis_len, 0);
        assert!(r.basis.len() >= 3);
    }

    #[test]
    fn m31_empty_zero_constant_and_unit_ideals() {
        for blocks in 0..=2 {
            let ring = Ring::new(blocks, 0).unwrap();
            let r = checked(&ring, &[]);
            assert!(r.basis.is_empty());
            let r = checked(&ring, &[Poly::zero(ring.nvars())]);
            assert!(r.basis.is_empty());
            let r = checked(&ring, &[Poly::constant(ring.nvars(), 37)]);
            assert!(r.contains_unit());
        }
    }

    #[test]
    fn m31_forged_unit_basis_fails_ideal_equality() {
        let ring = Ring::new(1, 0).unwrap();
        let mut forged = checked(&ring, &[]);
        forged.basis = vec![Poly::constant(3, 1)];
        let v = verify(&ring, &[], &forged, Limits::default()).unwrap();
        assert!(matches!(v.status, VerificationStatus::Rejected(_)));
    }

    #[test]
    fn m31_caps_never_report_complete_or_empty_solution_set() {
        let ring = Ring::new(1, 0).unwrap();
        let input = [Poly::variable(3, 1).add(&Poly::constant(3, 1)).unwrap()];
        for limits in [Limits { max_pairs: 0, ..Limits::default() },
                       Limits { max_rows: 0, ..Limits::default() },
                       Limits { max_columns: 0, ..Limits::default() },
                       Limits { max_nonzeros: 0, ..Limits::default() },
                       Limits { max_basis: 0, ..Limits::default() },
                       Limits { max_degree: 1, ..Limits::default() },
                       Limits { max_steps: 0, ..Limits::default() },
                       Limits { max_reductions: 0, ..Limits::default() }] {
            let r = compute(&ring, &input, limits).unwrap();
            assert!(matches!(r.status, Status::Incomplete { .. }), "{limits:?}");
        }
        let r = checked(&ring, &input);
        let v = verify(&ring, &input, &r, Limits { max_pairs: 0, ..Limits::default() }).unwrap();
        assert!(matches!(v.status, VerificationStatus::Incomplete { .. }));
    }

    #[test]
    fn m31_input_validation_and_nonmonic_duplicates() {
        let ring = Ring::new(1, 0).unwrap();
        assert!(compute(&ring, &[Poly::constant(4, 1)], Limits::default()).is_err());
        let f = Poly::variable(3, 1).add(&Poly::constant(3, 5)).unwrap();
        let r = checked(&ring, &[f.scale(13), f.scale(M31 - 7)]);
        assert!(!r.contains_unit());
        assert!(r.basis.iter().all(|p| ring.normal_form(p).unwrap() == *p));
    }

    #[test]
    fn m31_fifty_deterministic_chart_ideals_have_independent_ordinary_certificates() {
        // Development differential controls, not a performance panel or a
        // replacement for the requested Semaev projected-solution instances.
        let ring = Ring::new(1, 0).unwrap();
        for i in 0..50u64 {
            let c = (1_000_003 * (i + 1) + 91) % M31;
            let f = Poly::variable(3, 1).sub(&Poly::constant(3, c)).unwrap();
            let r = checked(&ring, &[f]);
            let square = ((u128::from(c) * u128::from(c)) % u128::from(M31)) as u64;
            assert!(r.basis.iter().all(|g| g.evaluate(&[1, c, square]).unwrap() == 0));
        }
    }

    #[test]
    fn m31_one_preclosed_geometric_system_bounded_development_probe() {
        use crate::systems::{self, Arm, WitnessMode};
        let plane = systems::fixture_plane(3, 0).unwrap();
        let model = systems::build(&plane, Arm::PreclosedSequential, WitnessMode::Geometric).unwrap();
        let (ring, equations, _) = model.quotient_presentation().unwrap();
        let limits = Limits { max_pairs: 2_000, max_rows: 512, max_columns: 512,
            max_nonzeros: 25_000, max_basis: 64, max_degree: 8, max_steps: 32,
            max_reductions: 20_000 };
        let result = compute(&ring, &equations, limits).unwrap();
        eprintln!("one_m3_development_probe status={:?} basis={} pairs={}/{} raw_degree={} post_nf_degree={} matrices={}",
            result.status, result.basis.len(), result.statistics.processed_pairs,
            result.statistics.scheduled_pairs, result.statistics.max_raw_product_degree,
            result.statistics.max_post_nf_matrix_degree, result.matrices.len());
        // This particular fixture completed during development. A future
        // regression to an incomplete producer must fail this gate explicitly.
        assert!(result.is_complete(), "frozen m3 producer fixture incomplete: {:?}", result.status);
        // Two earlier verifier attempts hit the shared step and reduction
        // caps; their failed receipts are retained. Ordinary Buchberger has
        // its own explicitly authorized development-verification budget.
        let verifier_limits = Limits { max_pairs: 10_000, max_steps: 10_000,
            max_reductions: 1_000_000, max_basis: 128, max_degree: 12,
            ..limits };
        let verification = verify(&ring, &equations, &result, verifier_limits).unwrap();
        eprintln!("one_m3_ordinary_verification={verification:?}");
        assert_eq!(verification.status, VerificationStatus::Verified);
    }
}
