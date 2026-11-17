//! One static affine consequence pass. All original equations remain present.
//! Classical row operations; diagnostic reservations are NOT complete cold cost.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use torus_s4::{Field, Poly, System};

pub mod check;
pub mod controls;
pub type Result<T> = std::result::Result<T, Stop>;
#[derive(Debug, Clone, Serialize)]
pub enum Stop {
    Invalid(String),
    Cap(String),
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub variables: usize,
    pub equations: usize,
    pub input_terms: usize,
    pub stored_terms: usize,
    pub pairs: u64,
    pub events: u64,
    pub cumulative_modeled_bytes: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            variables: 4096,
            equations: 8192,
            input_terms: 32768,
            stored_terms: 131072,
            pairs: 1 << 22,
            events: 1 << 24,
            cumulative_modeled_bytes: 128 << 20,
        }
    }
}
#[derive(Clone, Default, Serialize)]
pub struct Counts {
    pub diagnostic_events: u64,
    pub cumulative_modeled_bytes: u64,
    pub multiplication_pairs_reserved: u64,
    pub linear_inputs: usize,
    pub pivots: usize,
    pub side_rows: usize,
    pub stored_certificate_terms: usize,
    pub field_inverses: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Term {
    pub monomial: Vec<usize>,
    pub coefficient: String,
}
pub type Terms = Vec<Term>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    pub equation: usize,
    pub coefficient: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BasisRow {
    pub pivot: Option<usize>,
    pub polynomial: Terms,
    pub origins: Vec<Weight>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quotient {
    pub basis: usize,
    pub polynomial: Terms,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rewrite {
    pub source_equation: usize,
    pub residual: Terms,
    pub quotients: Vec<Quotient>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    pub schema: String,
    pub original_system_sha256: String,
    pub protected: Vec<usize>,
    pub basis: Vec<BasisRow>,
    pub rewrites: Vec<Rewrite>,
}
#[derive(Serialize)]
pub struct Report {
    pub artifact: Option<Artifact>,
    pub stop: Option<Stop>,
    pub counts: Counts,
}
pub fn sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
pub fn system_hash(s: &System) -> String {
    sha256(s.canonical_json().to_string().as_bytes())
}
fn terms(p: &Poly) -> Terms {
    p.terms
        .iter()
        .map(|(m, c)| Term {
            monomial: m.clone(),
            coefficient: c.to_string(),
        })
        .collect()
}
struct Row {
    pivot: Option<usize>,
    polynomial: Poly,
    origins: BTreeMap<usize, BigUint>,
}
struct Context {
    f: Field,
    limits: Limits,
    counts: Counts,
}
impl Context {
    fn event(&mut self, n: u64) -> Result<()> {
        if n > self
            .limits
            .events
            .saturating_sub(self.counts.diagnostic_events)
        {
            return Err(Stop::Cap("diagnostic events".into()));
        }
        self.counts.diagnostic_events += n;
        Ok(())
    }
    fn reserve(&mut self, n: u64) -> Result<()> {
        if n > self
            .limits
            .cumulative_modeled_bytes
            .saturating_sub(self.counts.cumulative_modeled_bytes)
        {
            return Err(Stop::Cap("cumulative modeled bytes".into()));
        }
        self.counts.cumulative_modeled_bytes += n;
        Ok(())
    }
    fn polynomial_reservation(&mut self, n: usize) -> Result<()> {
        self.event(n as u64)?;
        self.reserve(64 + (128 + self.f.p.bits().div_ceil(8)) * n as u64)
    }
    fn sum(&mut self, a: &Poly, b: &Poly, subtract: bool) -> Result<Poly> {
        self.polynomial_reservation(a.terms.len() + b.terms.len())?;
        Ok(if subtract { a - b } else { a + b })
    }
    fn product(&mut self, a: &Poly, b: &Poly) -> Result<Poly> {
        let n = (a.terms.len() as u64)
            .checked_mul(b.terms.len() as u64)
            .ok_or_else(|| Stop::Cap("pair overflow".into()))?;
        if n > self
            .limits
            .pairs
            .saturating_sub(self.counts.multiplication_pairs_reserved)
        {
            return Err(Stop::Cap("polynomial pairs".into()));
        }
        self.counts.multiplication_pairs_reserved += n;
        self.polynomial_reservation(n as usize)?;
        Ok(a * b)
    }
    fn scale(&mut self, p: &Poly, c: &BigUint) -> Result<Poly> {
        self.product(p, &Poly::constant(&self.f.p, c.clone()))
    }
    fn origins_sum(
        &mut self,
        a: &BTreeMap<usize, BigUint>,
        b: &BTreeMap<usize, BigUint>,
        factor: &BigUint,
    ) -> Result<BTreeMap<usize, BigUint>> {
        self.polynomial_reservation(a.len() + b.len())?;
        let mut out = a.clone();
        for (&i, c) in b {
            self.event(2)?;
            let next = self.f.sub(
                out.get(&i).unwrap_or(&BigUint::zero()),
                &self.f.mul(c, factor),
            );
            if next.is_zero() {
                out.remove(&i);
            } else {
                out.insert(i, next);
            }
        }
        Ok(out)
    }
    fn subtract_row(&mut self, row: &mut Row, basis: &Row, factor: &BigUint) -> Result<()> {
        let scaled = self.scale(&basis.polynomial, factor)?;
        row.polynomial = self.sum(&row.polynomial, &scaled, true)?;
        row.origins = self.origins_sum(&row.origins, &basis.origins, factor)?;
        Ok(())
    }
    fn normalize(&mut self, row: &mut Row, c: &BigUint) -> Result<()> {
        if row.polynomial.terms.len() as u64
            > self
                .limits
                .pairs
                .saturating_sub(self.counts.multiplication_pairs_reserved)
        {
            return Err(Stop::Cap("polynomial pairs before inverse".into()));
        }
        self.event(1)?;
        self.reserve(256)?;
        self.counts.field_inverses += 1;
        let inverse = if c.is_one() {
            BigUint::one()
        } else {
            self.f.inv(c).map_err(Stop::Invalid)?
        };
        if self.f.mul(c, &inverse) != BigUint::one() {
            return Err(Stop::Invalid("nonunit pivot; field contract".into()));
        }
        row.polynomial = self.scale(&row.polynomial, &inverse)?;
        self.polynomial_reservation(row.origins.len())?;
        for value in row.origins.values_mut() {
            self.event(1)?;
            *value = self.f.mul(value, &inverse);
        }
        Ok(())
    }
    fn retain_terms(&mut self, n: usize) -> Result<()> {
        if n > self
            .limits
            .stored_terms
            .saturating_sub(self.counts.stored_certificate_terms)
        {
            return Err(Stop::Cap("stored certificate terms".into()));
        }
        self.counts.stored_certificate_terms += n;
        self.polynomial_reservation(n)
    }
}
fn coefficient(p: &Poly, v: usize) -> BigUint {
    p.terms.get(&vec![v]).cloned().unwrap_or_default()
}

fn build(s: &System, protected: &[usize], ctx: &mut Context) -> Result<Artifact> {
    ctx.event(1)?;
    if !(5..=512).contains(&s.p.bits()) || !s.p.bit(0) {
        return Err(Stop::Invalid(
            "odd5..512bit modulus; primality external".into(),
        ));
    }
    if s.variables.is_empty() {
        return Err(Stop::Invalid("no variables".into()));
    }
    if s.variables.len() > ctx.limits.variables {
        return Err(Stop::Cap("variables".into()));
    }
    if s.equations.len() > ctx.limits.equations {
        return Err(Stop::Cap("equations".into()));
    }
    ctx.reserve(128 + 2 * s.p.bits().div_ceil(8) + 64 * s.variables.len() as u64)?;
    ctx.f.p = s.p.clone();
    let mut protected_set = BTreeSet::new();
    for &v in protected {
        ctx.event(1)?;
        ctx.reserve(64)?;
        if v >= s.variables.len() || !protected_set.insert(v) {
            return Err(Stop::Invalid("duplicate/out-of-range public role".into()));
        }
    }
    let mut input_terms = 0usize;
    for p in &s.equations {
        ctx.event(1)?;
        if p.terms.len() > ctx.limits.input_terms.saturating_sub(input_terms) {
            return Err(Stop::Cap("input terms".into()));
        }
        input_terms += p.terms.len();
        for (m, c) in &p.terms {
            ctx.event(1 + m.len() as u64)?;
            if m.len() > 2 {
                return Err(Stop::Invalid("quadratic input only".into()));
            }
            if c.is_zero()
                || c >= &s.p
                || !m.windows(2).all(|w| w[0] <= w[1])
                || m.iter().any(|v| *v >= s.variables.len())
            {
                return Err(Stop::Invalid("noncanonical input term".into()));
            }
        }
    }
    let mut basis: Vec<Row> = vec![];
    let mut side = vec![];
    for (i, p) in s.equations.iter().enumerate() {
        ctx.event(1)?;
        if p.degree() > 1 || p.terms.is_empty() {
            continue;
        }
        ctx.counts.linear_inputs += 1;
        ctx.polynomial_reservation(p.terms.len() + 1)?;
        let mut row = Row {
            pivot: None,
            polynomial: p.clone(),
            origins: BTreeMap::from([(i, BigUint::one())]),
        };
        for b in &basis {
            ctx.event(1)?;
            let v = b.pivot.unwrap();
            let c = coefficient(&row.polynomial, v);
            if !c.is_zero() {
                ctx.subtract_row(&mut row, b, &c)?;
            }
        }
        if row.polynomial.terms.is_empty() {
            continue;
        }
        let pivot = row
            .polynomial
            .terms
            .keys()
            .filter(|m| m.len() == 1 && !protected_set.contains(&m[0]))
            .map(|m| m[0])
            .max();
        row.pivot = pivot;
        let c = if let Some(v) = pivot {
            coefficient(&row.polynomial, v)
        } else {
            row.polynomial.terms.values().next().unwrap().clone()
        };
        ctx.normalize(&mut row, &c)?;
        if pivot.is_some() {
            basis.push(row);
        } else {
            side.push(row);
        }
    }
    for i in (0..basis.len()).rev() {
        let (earlier, later) = basis.split_at_mut(i);
        let b = &later[0];
        let v = b.pivot.unwrap();
        for row in earlier {
            ctx.event(1)?;
            let c = coefficient(&row.polynomial, v);
            if !c.is_zero() {
                ctx.subtract_row(row, b, &c)?;
            }
        }
    }
    ctx.counts.pivots = basis.len();
    ctx.counts.side_rows = side.len();
    basis.extend(side);
    let mut replacements = BTreeMap::new();
    let mut exported = vec![];
    for (i, b) in basis.iter().enumerate() {
        ctx.retain_terms(b.polynomial.terms.len() + b.origins.len())?;
        if let Some(v) = b.pivot {
            let x = Poly::var(&s.p, v);
            replacements.insert(v, (i, ctx.sum(&x, &b.polynomial, true)?));
        }
        exported.push(BasisRow {
            pivot: b.pivot,
            polynomial: terms(&b.polynomial),
            origins: b
                .origins
                .iter()
                .map(|(e, c)| Weight {
                    equation: *e,
                    coefficient: c.to_string(),
                })
                .collect(),
        });
    }
    let mut rewrites = vec![];
    for (source_equation, p) in s.equations.iter().enumerate() {
        ctx.event(1)?;
        let mut residual = Poly::small(&s.p, 0);
        let mut quotients: BTreeMap<usize, Poly> = BTreeMap::new();
        for (m, c) in &p.terms {
            ctx.event(1)?;
            let constant = Poly::constant(&s.p, c.clone());
            let expression = |v: usize| {
                replacements
                    .get(&v)
                    .map(|(_, e)| e.clone())
                    .unwrap_or_else(|| Poly::var(&s.p, v))
            };
            let substituted = match m.as_slice() {
                [] => constant.clone(),
                [v] => ctx.product(&constant, &expression(*v))?,
                [a, b] => {
                    let left = ctx.product(&constant, &expression(*a))?;
                    ctx.product(&left, &expression(*b))?
                }
                _ => unreachable!(),
            };
            residual = ctx.sum(&residual, &substituted, false)?;
            let mut add_q = |variable: usize, q: Poly, ctx: &mut Context| -> Result<()> {
                if let Some((index, _)) = replacements.get(&variable) {
                    let before = quotients
                        .remove(index)
                        .unwrap_or_else(|| Poly::small(&s.p, 0));
                    let next = ctx.sum(&before, &q, false)?;
                    if !next.terms.is_empty() {
                        quotients.insert(*index, next);
                    }
                }
                Ok(())
            };
            match m.as_slice() {
                [] => {}
                [v] => add_q(*v, constant, ctx)?,
                [a, b] => {
                    if replacements.contains_key(a) {
                        let q = ctx.product(&constant, &Poly::var(&s.p, *b))?;
                        add_q(*a, q, ctx)?;
                    }
                    if replacements.contains_key(b) {
                        let q = ctx.product(&constant, &expression(*a))?;
                        add_q(*b, q, ctx)?;
                    }
                }
                _ => unreachable!(),
            }
        }
        ctx.retain_terms(
            residual.terms.len() + quotients.values().map(|p| p.terms.len()).sum::<usize>(),
        )?;
        rewrites.push(Rewrite {
            source_equation,
            residual: terms(&residual),
            quotients: quotients
                .iter()
                .map(|(basis, p)| Quotient {
                    basis: *basis,
                    polynomial: terms(p),
                })
                .collect(),
        });
    }
    Ok(Artifact {
        schema: "torus.affine-certificate/v1".into(),
        original_system_sha256: system_hash(s),
        protected: protected.to_vec(),
        basis: exported,
        rewrites,
    })
}
pub fn preprocess(s: &System, protected: &[usize], limits: Limits) -> Report {
    let mut ctx = Context {
        f: Field { p: BigUint::zero() },
        limits,
        counts: Counts::default(),
    };
    match build(s, protected, &mut ctx) {
        Ok(artifact) => Report {
            artifact: Some(artifact),
            stop: None,
            counts: ctx.counts,
        },
        Err(stop) => Report {
            artifact: None,
            stop: Some(stop),
            counts: ctx.counts,
        },
    }
}
