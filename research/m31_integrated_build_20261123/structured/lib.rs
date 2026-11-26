//! Three-summand M31 extraction using the production F4/root visitor.
//!
//! The supplied leaf selectors are retained exactly. Optional selector normal
//! forms carry polynomial-identity certificates. Chaining includes the missing
//! intermediate-at-infinity chart; signed point replay is the acceptance test.
//! This is a bounded solver, not a claim about F4 regularity or running time.

use crypto_f4::cryptanalysis::f4_fp::{
    solve_visit_traced, F4Options, F4ResourceLimits, Ordering, Poly,
    VisitEvent, VisitReport,
};
use crypto_f4::cryptanalysis::semaev_prime_poly::Fp;
use prime31_algebraic_ic::{Arithmetic, Counts, Curve};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

pub const M31: u64 = 2_147_483_647;
type Sparse = BTreeMap<Vec<u32>, u64>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Problem {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub target: [u64; 2],
    /// Coefficients low to high. Each polynomial must be monic and nonconstant.
    /// No selector is resampled, enlarged, or replaced by a different base.
    pub selectors: [Vec<u64>; 3],
}

impl Problem {
    fn curve(&self) -> Curve {
        // This coordinate solver does not use a subgroup order or generator.
        Curve { p: self.p, a: self.a, b: self.b, n: 0, g: (self.target[0], self.target[1]) }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.p != M31 || self.a >= M31 || self.b >= M31
            || self.target.iter().any(|&x| x >= M31)
        {
            return Err(Error::InvalidInput("canonical M31 curve and target required".into()));
        }
        let f = Fp { p: M31 };
        if f.add(f.mul(4, f.mul(self.a, f.mul(self.a, self.a))), f.mul(27, f.mul(self.b, self.b))) == 0 {
            return Err(Error::InvalidInput("singular curve".into()));
        }
        if !Arithmetic::default().on_curve(self.curve(), Some((self.target[0], self.target[1]))) {
            return Err(Error::InvalidInput("target is not a finite point on the supplied curve".into()));
        }
        for l in &self.selectors {
            if !(2..=65).contains(&l.len()) || l.last() != Some(&1) || l.iter().any(|&c| c >= M31) {
                return Err(Error::InvalidInput("selectors must be canonical monic polynomials of degree 1..64".into()));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Formulation { Expanded, ChainLeft, ChainRight }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Preprocessing {
    None,
    /// If bounded normal-form construction fails, run the unchanged equations.
    TrySelectorQuotient,
    /// Refuse rather than silently omit the requested preprocessing.
    RequireSelectorQuotient,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub max_degree: u32,
    pub deadline_ms: u64,
    pub max_terms: usize,
    pub max_symbolic_products: u64,
    pub max_assignments: usize,
    pub max_witnesses: usize,
    pub max_pairs: usize,
    pub max_symbolic_monomials: usize,
    pub max_matrix_entries: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_degree: 16, deadline_ms: 10_000, max_terms: 20_000,
            max_symbolic_products: 2_000_000, max_assignments: 4096,
            max_witnesses: 4096, max_pairs: 100_000,
            max_symbolic_monomials: 100_000, max_matrix_entries: 8_000_000,
        }
    }
}

impl Limits {
    fn validate(self) -> Result<(), Error> {
        if self.max_degree == 0 || self.max_degree > 64 || self.deadline_ms == 0
            || self.deadline_ms > 60_000 || self.max_terms == 0 || self.max_terms > 100_000
            || self.max_symbolic_products == 0 || self.max_symbolic_products > 20_000_000
            || self.max_assignments == 0 || self.max_assignments > 100_000
            || self.max_witnesses == 0 || self.max_witnesses > 100_000
            || self.max_pairs == 0 || self.max_pairs > 1_000_000
            || self.max_symbolic_monomials == 0 || self.max_symbolic_monomials > 1_000_000
            || self.max_matrix_entries == 0 || self.max_matrix_entries > 64_000_000
        {
            return Err(Error::InvalidInput("limits outside this component's finite envelope".into()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum Error { InvalidInput(String), ConstructionLimit(String), PreprocessingLimit(String), Native(String) }

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:?}") }
}
impl std::error::Error for Error {}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ConstructionStats {
    pub coefficient_additions: u64,
    pub coefficient_multiplications: u64,
    pub normal_form_steps: u64,
}

struct Algebra {
    variables: usize,
    limits: Limits,
    deadline: Instant,
    stats: ConstructionStats,
}

impl Algebra {
    fn check(&self, terms: usize) -> Result<(), Error> {
        if Instant::now() >= self.deadline { return Err(Error::ConstructionLimit("deadline".into())); }
        if terms > self.limits.max_terms { return Err(Error::ConstructionLimit("polynomial term cap".into())); }
        Ok(())
    }
    fn constant(&self, c: u64) -> Sparse {
        if c == 0 { Sparse::new() } else { BTreeMap::from([(vec![0; self.variables], c)]) }
    }
    fn variable(&self, i: usize) -> Sparse {
        let mut m = vec![0; self.variables]; m[i] = 1;
        BTreeMap::from([(m, 1)])
    }
    fn insert(&mut self, out: &mut Sparse, monomial: Vec<u32>, value: u64) -> Result<(), Error> {
        self.stats.coefficient_additions += 1;
        let v = (out.get(&monomial).copied().unwrap_or(0) + value) % M31;
        if v == 0 { out.remove(&monomial); } else { out.insert(monomial, v); }
        self.check(out.len())
    }
    fn add(&mut self, a: &Sparse, b: &Sparse) -> Result<Sparse, Error> {
        let mut out = a.clone();
        for (m, &c) in b { self.insert(&mut out, m.clone(), c)?; }
        self.check(out.len())?; Ok(out)
    }
    fn scale(&mut self, a: &Sparse, c: u64) -> Result<Sparse, Error> {
        self.mul(a, &self.constant(c))
    }
    fn sub(&mut self, a: &Sparse, b: &Sparse) -> Result<Sparse, Error> {
        let negative = self.scale(b, M31 - 1)?;
        self.add(a, &negative)
    }
    fn mul(&mut self, a: &Sparse, b: &Sparse) -> Result<Sparse, Error> {
        let additional = (a.len() as u64).checked_mul(b.len() as u64)
            .ok_or_else(|| Error::ConstructionLimit("symbolic product overflow".into()))?;
        if self.stats.coefficient_multiplications.saturating_add(additional) > self.limits.max_symbolic_products {
            return Err(Error::ConstructionLimit("symbolic product cap".into()));
        }
        self.check(a.len().max(b.len()))?;
        let mut out = Sparse::new();
        for (ma, &ca) in a {
            for (mb, &cb) in b {
                let mut m = Vec::with_capacity(self.variables);
                for (&x, &y) in ma.iter().zip(mb) {
                    let e = x.checked_add(y).ok_or_else(|| Error::ConstructionLimit("exponent overflow".into()))?;
                    if e > 128 { return Err(Error::ConstructionLimit("exponent cap".into())); }
                    m.push(e);
                }
                self.stats.coefficient_multiplications += 1;
                self.insert(&mut out, m, Fp { p: M31 }.mul(ca, cb))?;
            }
        }
        Ok(out)
    }
    fn polynomial(&self, p: &Sparse) -> Poly {
        let mut terms: Poly = p.iter().map(|(m, &c)| (m.clone(), c)).collect();
        // Production normalise() also makes its argument monic. Scaling each
        // cofactor separately would destroy f-NF=sum(q_i L_i), so retain exact
        // coefficients and use only the production monomial order here.
        terms.sort_unstable_by(|(a, _), (b, _)| {
            crypto_f4::cryptanalysis::groebner_f4::cmp_monomial(b, a, Ordering::Grevlex)
        });
        terms
    }
    fn selector(&self, variable: usize, l: &[u64]) -> Sparse {
        l.iter().enumerate().filter_map(|(degree, &c)| {
            if c == 0 { return None; }
            let mut m = vec![0; self.variables]; m[variable] = degree as u32; Some((m, c))
        }).collect()
    }
    /// Coefficients of S3(x,y,U), low to high, with symbolic x and y.
    fn s3_coefficients(&mut self, x: &Sparse, y: &Sparse, a: u64, b: u64) -> Result<[Sparse; 3], Error> {
        let difference = self.sub(x, y)?;
        let quadratic = self.mul(&difference, &difference)?;
        let sum = self.add(x, y)?;
        let product = self.mul(x, y)?;
        let product_plus_a = self.add(&product, &self.constant(a))?;
        let middle = self.mul(&sum, &product_plus_a)?;
        let middle = self.add(&middle, &self.constant(2 * b % M31))?;
        let linear = self.scale(&middle, M31 - 2)?;
        let product_minus_a = self.sub(&product, &self.constant(a))?;
        let square = self.mul(&product_minus_a, &product_minus_a)?;
        let correction = self.scale(&sum, 4 * b % M31)?;
        let constant = self.sub(&square, &correction)?;
        Ok([constant, linear, quadratic])
    }
    fn evaluate_quadratic(&mut self, q: &[Sparse; 3], z: &Sparse) -> Result<Sparse, Error> {
        let leading = self.mul(&q[2], z)?;
        let middle = self.add(&leading, &q[1])?;
        let high = self.mul(&middle, z)?;
        self.add(&high, &q[0])
    }
    fn resultant(&mut self, f: &[Sparse; 3], g: &[Sparse; 3]) -> Result<Sparse, Error> {
        let af = self.mul(&f[2], &g[0])?;
        let cd = self.mul(&f[0], &g[2])?;
        let first = self.sub(&af, &cd)?;
        let first = self.mul(&first, &first)?;
        let ae = self.mul(&f[2], &g[1])?;
        let bd = self.mul(&f[1], &g[2])?;
        let second = self.sub(&ae, &bd)?;
        let bf = self.mul(&f[1], &g[0])?;
        let ce = self.mul(&f[0], &g[1])?;
        let third = self.sub(&bf, &ce)?;
        let product = self.mul(&second, &third)?;
        self.sub(&first, &product)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Chart {
    pub name: String,
    pub variables: usize,
    /// Relations first, unchanged selectors last.
    pub original: Vec<Poly>,
    pub prepared: Vec<Poly>,
    pub preprocessing: Option<PreprocessingCertificate>,
    pub fallback_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NormalFormCertificate {
    pub relation_index: usize,
    pub normal_form: Poly,
    /// f - normal_form = sum_i cofactors[i] * L_i(x_i).
    pub cofactors: [Poly; 3],
}

#[derive(Clone, Debug, Serialize)]
pub struct PreprocessingCertificate { pub relations: Vec<NormalFormCertificate> }

fn normal_form(a: &mut Algebra, input: &Sparse, selectors: &[Vec<u64>; 3]) -> Result<(Sparse, [Sparse; 3]), Error> {
    let mut remainder = input.clone();
    let mut quotients: [Sparse; 3] = std::array::from_fn(|_| Sparse::new());
    for variable in 0..3 {
        let d = selectors[variable].len() as u32 - 1;
        while let Some((m, c)) = remainder.iter().rev().find(|(m, _)| m[variable] >= d).map(|(m, &c)| (m.clone(), c)) {
            a.check(remainder.len())?;
            a.stats.normal_form_steps += 1;
            let mut quotient_m = m.clone(); quotient_m[variable] -= d;
            a.insert(&mut quotients[variable], quotient_m.clone(), c)?;
            remainder.remove(&m);
            for (j, &lc) in selectors[variable][..d as usize].iter().enumerate() {
                if lc == 0 { continue; }
                if a.stats.coefficient_multiplications == a.limits.max_symbolic_products {
                    return Err(Error::ConstructionLimit("normal form product cap".into()));
                }
                a.stats.coefficient_multiplications += 1;
                let coefficient = Fp { p: M31 }.neg(Fp { p: M31 }.mul(c, lc));
                let mut monomial = quotient_m.clone(); monomial[variable] += j as u32;
                a.insert(&mut remainder, monomial, coefficient)?;
            }
        }
    }
    Ok((remainder, quotients))
}

fn raw_chart(a: &mut Algebra, p: &Problem, name: &str, relations: Vec<Sparse>) -> Result<Chart, Error> {
    let selectors: Vec<_> = p.selectors.iter().enumerate().map(|(i, l)| a.selector(i, l)).collect();
    let mut original: Vec<_> = relations.iter().map(|f| a.polynomial(f)).collect();
    original.extend(selectors.iter().map(|f| a.polynomial(f)));
    for row in &original { a.check(row.len())?; }
    Ok(Chart { name: name.into(), variables: a.variables, prepared: original.clone(), original, preprocessing: None, fallback_reason: None })
}

fn prepare_chart(a: &mut Algebra, p: &Problem, chart: &mut Chart, preprocessing: Preprocessing) -> Result<(), Error> {
    if preprocessing == Preprocessing::None { return Ok(()); }
    a.variables = chart.variables;
    let relation_count = chart.original.len() - 3;
    let attempt = (|| {
        let mut certificates = Vec::new();
        let mut transformed = Vec::new();
        for (relation_index, f) in chart.original[..relation_count].iter().enumerate() {
            let raw = f.iter().cloned().collect();
            let (nf, q) = normal_form(a, &raw, &p.selectors)?;
            let normal_form = a.polynomial(&nf);
            transformed.push(normal_form.clone());
            certificates.push(NormalFormCertificate { relation_index, normal_form, cofactors: q.map(|q| a.polynomial(&q)) });
        }
        transformed.extend_from_slice(&chart.original[relation_count..]);
        Ok::<_, Error>((transformed, PreprocessingCertificate { relations: certificates }))
    })();
    match attempt {
        Ok((transformed, certificate)) => { chart.prepared = transformed; chart.preprocessing = Some(certificate); }
        Err(e) if preprocessing == Preprocessing::TrySelectorQuotient => { chart.fallback_reason = Some(e.to_string()); }
        Err(e) => return Err(Error::PreprocessingLimit(e.to_string())),
    }
    Ok(())
}

fn construct(p: &Problem, formulation: Formulation, preprocessing: Preprocessing, limits: Limits, deadline: Instant) -> Result<(Vec<Chart>, ConstructionStats), Error> {
    let n = if formulation == Formulation::Expanded { 3 } else { 4 };
    let mut a = Algebra { variables: n, limits, deadline, stats: ConstructionStats::default() };
    let xs: Vec<_> = (0..3).map(|i| a.variable(i)).collect();
    let target = a.constant(p.target[0]);
    let (i, j, k) = if formulation == Formulation::ChainRight { (1, 2, 0) } else { (0, 1, 2) };
    let left = a.s3_coefficients(&xs[i], &xs[j], p.a, p.b)?;
    let right = a.s3_coefficients(&xs[k], &target, p.a, p.b)?;
    let mut charts = if formulation == Formulation::Expanded {
        let polynomial = a.resultant(&left, &right)?;
        vec![raw_chart(&mut a, p, "expanded_s4", vec![polynomial])?]
    } else {
        let u = a.variable(3);
        let first = a.evaluate_quadratic(&left, &u)?;
        let second = a.evaluate_quadratic(&right, &u)?;
        let affine = raw_chart(&mut a, p, "finite_intermediate", vec![first, second])?;
        a.variables = 3;
        let first = a.sub(&a.variable(i), &a.variable(j))?;
        let second = a.sub(&a.variable(k), &a.constant(p.target[0]))?;
        let cancellation = raw_chart(&mut a, p, "intermediate_at_infinity", vec![first, second])?;
        vec![affine, cancellation]
    };
    // Construct every raw chart before optional work can consume its budget.
    // A failed TrySelectorQuotient attempt can therefore retain the full raw
    // chart union; it cannot starve construction of the cancellation chart.
    for chart in &mut charts { prepare_chart(&mut a, p, chart, preprocessing)?; }
    Ok((charts, a.stats))
}

/// Construct both the unchanged equations and the candidate presentation.
/// Construction only: no F4 call, root search, or point enumeration.
pub fn build(p: &Problem, formulation: Formulation, preprocessing: Preprocessing, limits: Limits) -> Result<(Vec<Chart>, ConstructionStats), Error> {
    p.validate()?; limits.validate()?;
    construct(p, formulation, preprocessing, limits, Instant::now() + Duration::from_millis(limits.deadline_ms))
}

/// Replay the cofactor identities by direct term accumulation. This checker
/// takes the source equations from the caller and does not trust a certificate
/// to supply its own original ideal. It never invokes F4 or normal-form code.
pub fn replay_preprocessing(p: &Problem, original: &[Poly], chart: &Chart) -> bool {
    if p.validate().is_err() || !(3..=4).contains(&chart.variables) || original != chart.original
        || !(4..=5).contains(&original.len()) || chart.prepared.len() != original.len()
    { return false; }
    let relation_count = original.len() - 3;
    let Some(certificate) = &chart.preprocessing else {
        return original == chart.prepared;
    };
    if certificate.relations.len() != relation_count { return false; }
    let canonical = |poly: &Poly| -> Option<Sparse> {
        if poly.len() > 100_000 { return None; }
        let mut map = Sparse::new();
        for (m, &c) in poly.iter().map(|(m, c)| (m, c)) {
            if m.len() != chart.variables || m.iter().any(|&e| e > 128) || c == 0 || c >= M31 || map.insert(m.clone(), c).is_some() { return None; }
        }
        Some(map)
    };
    for i in 0..3 {
        let expected: Sparse = p.selectors[i].iter().enumerate().filter_map(|(j, &c)| {
            if c == 0 { return None; }
            let mut m = vec![0; chart.variables]; m[i] = j as u32; Some((m, c))
        }).collect();
        if canonical(&original[relation_count + i]) != Some(expected)
            || original[relation_count + i] != chart.prepared[relation_count + i]
        { return false; }
    }
    for (i, proof) in certificate.relations.iter().enumerate() {
        if proof.relation_index != i || proof.normal_form != chart.prepared[i] { return false; }
        let Some(expected) = canonical(&original[i]) else { return false; };
        let Some(mut reconstructed) = canonical(&proof.normal_form) else { return false; };
        if reconstructed.keys().any(|m| (0..3).any(|j| m[j] as usize >= p.selectors[j].len() - 1)) { return false; }
        for variable in 0..3 {
            let Some(q) = canonical(&proof.cofactors[variable]) else { return false; };
            if q.len().saturating_mul(p.selectors[variable].len()) > 2_000_000 { return false; }
            for (m, c) in q {
                for (degree, &lc) in p.selectors[variable].iter().enumerate() {
                    if lc == 0 { continue; }
                    let mut exponent = m.clone(); exponent[variable] += degree as u32;
                    let product = ((c as u128 * lc as u128) % M31 as u128) as u64;
                    let next = (reconstructed.get(&exponent).copied().unwrap_or(0) + product) % M31;
                    if next == 0 { reconstructed.remove(&exponent); } else { reconstructed.insert(exponent, next); }
                    if reconstructed.len() > 100_000 { return false; }
                }
            }
        }
        if reconstructed != expected { return false; }
    }
    true
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedWitness {
    /// Leaf positions are retained because selectors may differ by leaf.
    pub points: [[u64; 2]; 3],
}

fn selector_eval(l: &[u64], x: u64) -> u64 {
    let f = Fp { p: M31 };
    l.iter().rev().fold(0, |v, &c| f.add(f.mul(v, x), c))
}

pub fn replay_witness(p: &Problem, witness: &SignedWitness) -> bool {
    if p.validate().is_err() { return false; }
    let e = p.curve();
    let mut ar = Arithmetic::default();
    let mut sum = None;
    for (i, point) in witness.points.iter().enumerate() {
        if point.iter().any(|&v| v >= M31) || selector_eval(&p.selectors[i], point[0]) != 0
            || !ar.on_curve(e, Some((point[0], point[1])))
        { return false; }
        sum = ar.add(e, sum, Some((point[0], point[1])));
    }
    sum == Some((p.target[0], p.target[1]))
}

fn lift(p: &Problem, values: &[u64], ar: &mut Arithmetic) -> Vec<SignedWitness> {
    let e = p.curve();
    let mut roots = Vec::new();
    for (i, &x) in values[..3].iter().enumerate() {
        if selector_eval(&p.selectors[i], x) != 0 { return Vec::new(); }
        let rhs = ar.rhs(e, x);
        let Some(y) = ar.sqrt(rhs, M31) else { return Vec::new(); };
        roots.push(if y == 0 { vec![0] } else { vec![y, M31 - y] });
    }
    let mut out = Vec::new();
    for &y0 in &roots[0] { for &y1 in &roots[1] { for &y2 in &roots[2] {
        let points = [[values[0], y0], [values[1], y1], [values[2], y2]];
        let left = ar.add(e, Some((values[0], y0)), Some((values[1], y1)));
        if ar.add(e, left, Some((values[2], y2))) == Some((p.target[0], p.target[1])) {
            out.push(SignedWitness { points });
        }
    } } }
    out
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status { Complete, Incomplete }

#[derive(Clone, Debug, Serialize)]
pub struct ChartReport {
    pub chart: Chart,
    pub traversal: VisitReport,
    pub algebraic_assignments: Vec<Vec<u64>>,
    pub rejected_coordinate_assignments: usize,
    pub refused_enumeration_branches: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub formulation: Formulation,
    pub status: Status,
    pub stop_reason: Option<String>,
    pub witnesses: Vec<SignedWitness>,
    pub charts: Vec<ChartReport>,
    pub construction: ConstructionStats,
    pub point_work: Counts,
}

/// Execute bounded F4 extraction. An empty `witnesses` vector means absence
/// only when `status == Complete`; caps and refused branching remain incomplete.
pub fn solve(p: &Problem, formulation: Formulation, preprocessing: Preprocessing, limits: Limits) -> Result<Report, Error> {
    p.validate()?; limits.validate()?;
    let deadline = Instant::now() + Duration::from_millis(limits.deadline_ms);
    let (charts, construction) = construct(p, formulation, preprocessing, limits, deadline)?;
    let mut report = Report { formulation, status: Status::Complete, stop_reason: None, witnesses: Vec::new(), charts: Vec::new(), construction, point_work: Counts::default() };
    let mut witnesses = BTreeSet::new();
    let mut ar = Arithmetic::default();
    let mut total_assignments = 0usize;
    for chart in charts {
        if Instant::now() >= deadline {
            report.status = Status::Incomplete; report.stop_reason = Some("deadline before chart".into()); break;
        }
        if !replay_preprocessing(p, &chart.original, &chart) { return Err(Error::Native("preprocessing certificate replay failed".into())); }
        let mut opts = F4Options::new(Ordering::Grevlex, limits.max_degree);
        opts.deadline = Some(deadline);
        opts.resource_limits = Some(F4ResourceLimits { max_pairs: limits.max_pairs, max_symbolic_monomials: limits.max_symbolic_monomials, max_matrix_entries: limits.max_matrix_entries });
        let mut assignments = Vec::new();
        let mut refused = 0;
        let mut output_cap = false;
        let remaining = limits.max_assignments - total_assignments;
        let traversal = solve_visit_traced(&chart.prepared, chart.variables, M31, &opts, &vec![None; chart.variables], |values| {
            if assignments.len() == remaining { output_cap = true; return true; }
            assignments.push(values.to_vec()); false
        }, |event| {
            if let VisitEvent::Branch { source, .. } = event {
                if source != "univariate_roots" { refused += 1; return Err(format!("refused {source}; algebraic roots required")); }
            }
            Ok(())
        }).map_err(Error::Native)?;
        total_assignments += assignments.len();
        let mut rejected = 0;
        for assignment in &assignments {
            if Instant::now() >= deadline { report.status = Status::Incomplete; report.stop_reason = Some("deadline during signed lifting".into()); break; }
            let lifted = lift(p, assignment, &mut ar);
            if lifted.is_empty() { rejected += 1; }
            for witness in lifted {
                if !witnesses.contains(&witness) && witnesses.len() == limits.max_witnesses {
                    report.status = Status::Incomplete; report.stop_reason = Some("signed witness cap".into()); break;
                }
                witnesses.insert(witness);
            }
            if report.status == Status::Incomplete { break; }
        }
        if !traversal.complete || output_cap {
            report.status = Status::Incomplete;
            report.stop_reason = Some(if output_cap { "algebraic assignment cap".into() }
                else if traversal.timed_out { "F4/root deadline".into() }
                else if let Some(r) = &traversal.resource_limit { format!("native allocation admission: {r}") }
                else if refused > 0 { "nonalgebraic fallback refused".into() }
                else { "native traversal incomplete".into() });
        }
        report.charts.push(ChartReport { chart, traversal, algebraic_assignments: assignments, rejected_coordinate_assignments: rejected, refused_enumeration_branches: refused });
        if report.status == Status::Incomplete { break; }
    }
    report.witnesses = witnesses.into_iter().collect();
    report.point_work = ar.counts;
    Ok(report)
}

#[cfg(test)]
mod tests;
