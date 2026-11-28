//! Bounded connected slice; no multiplication of equations or copied System.
use crate::{Arithmetic, Consequence, DynamicCounts, DynamicLimits, Result, Stop};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::{BTreeMap, BTreeSet};
use torus_s4::System;

struct Row {
    terms: BTreeMap<usize, BigUint>,
    constant: BigUint,
    origins: BTreeMap<usize, BigUint>,
}
impl Row {
    fn scale(&mut self, by: &BigUint, ctx: &mut Arithmetic) -> Result<()> {
        ctx.event((self.terms.len() + self.origins.len() + 1) as u64)?;
        for c in self.terms.values_mut().chain(self.origins.values_mut()) {
            *c = ctx.mul(c, by)?;
        }
        self.constant = ctx.mul(&self.constant, by)?;
        Ok(())
    }
    fn subtract(&mut self, other: &Row, by: &BigUint, ctx: &mut Arithmetic) -> Result<()> {
        for (v, c) in &other.terms {
            ctx.event(1)?;
            ctx.reserve(128)?;
            let x = ctx.mul(c, by)?;
            let x = ctx.sub(self.terms.get(v).unwrap_or(&BigUint::zero()), &x)?;
            if x.is_zero() {
                self.terms.remove(v);
            } else {
                self.terms.insert(*v, x);
            }
        }
        let x = ctx.mul(&other.constant, by)?;
        self.constant = ctx.sub(&self.constant, &x)?;
        for (v, c) in &other.origins {
            ctx.event(1)?;
            ctx.reserve(128)?;
            let x = ctx.mul(c, by)?;
            let x = ctx.sub(self.origins.get(v).unwrap_or(&BigUint::zero()), &x)?;
            if x.is_zero() {
                self.origins.remove(v);
            } else {
                self.origins.insert(*v, x);
            }
        }
        Ok(())
    }
    fn certificate(self, ctx: &mut Arithmetic) -> Result<Consequence> {
        ctx.event((self.terms.len() + self.origins.len() + 1) as u64)?;
        ctx.reserve(256 + 192 * (self.terms.len() + self.origins.len()) as u64)?;
        Ok(Consequence {
            coefficients: self
                .terms
                .into_iter()
                .map(|(v, c)| (v, c.to_string()))
                .collect(),
            constant: self.constant.to_string(),
            origins: self
                .origins
                .into_iter()
                .map(|(e, c)| (e, c.to_string()))
                .collect(),
        })
    }
}

pub(super) fn derive(
    s: &System,
    assignment: &[Option<BigUint>],
    sources: impl Iterator<Item = usize>,
    limits: &DynamicLimits,
    counts: &mut DynamicCounts,
    ctx: &mut Arithmetic,
) -> Result<Vec<Consequence>> {
    if limits.rows == 0 || limits.variables == 0 || limits.coefficients == 0 {
        return Err(Stop::Cap("zero affine slice bound".into()));
    }
    // Limits may shrink this frozen schedule, never enlarge it.
    if limits.rows > 32 || limits.variables > 64 || limits.coefficients > 256 {
        return Err(Stop::Invalid(
            "affine slice limit exceeds versioned schedule".into(),
        ));
    }
    let mut variables = BTreeSet::new();
    let mut selected = Vec::new();
    let mut entries = 0;
    for equation in sources {
        if selected.len() >= limits.rows {
            break;
        }
        ctx.event(1)?;
        let mut residual: BTreeMap<Vec<usize>, BigUint> = BTreeMap::new();
        for (monomial, coefficient) in &s.equations[equation].terms {
            ctx.event(1)?;
            ctx.reserve(128 + 8 * monomial.len() as u64)?;
            counts.specialization_terms += 1;
            let mut c = coefficient.clone();
            let mut unknown = Vec::new();
            for &v in monomial {
                ctx.event(1)?;
                if let Some(x) = &assignment[v] {
                    c = ctx.mul(&c, x)?;
                } else {
                    unknown.push(v);
                }
            }
            let old = residual.remove(&unknown).unwrap_or_default();
            let c = ctx.add(&old, &c)?;
            if !c.is_zero() {
                residual.insert(unknown, c);
            }
        }
        if residual.keys().any(|m| m.len() > 1) {
            counts.skipped_rows += 1;
            continue;
        }
        let mut terms = BTreeMap::new();
        let mut constant = BigUint::zero();
        for (m, c) in residual {
            ctx.event(1)?;
            if m.is_empty() {
                constant = c;
            } else {
                terms.insert(m[0], c);
            }
        }
        if terms.is_empty() && constant.is_zero() {
            continue;
        }
        let overlaps = variables.is_empty() || terms.keys().any(|v| variables.contains(v));
        let new_variables = terms.keys().filter(|v| !variables.contains(*v)).count();
        let size = terms.len() + usize::from(!constant.is_zero());
        if !overlaps
            || variables.len() + new_variables > limits.variables
            || entries + size > limits.coefficients
        {
            counts.skipped_rows += 1;
            continue;
        }
        ctx.reserve(256 + 160 * (terms.len() + 1) as u64)?;
        variables.extend(terms.keys().copied());
        entries += size;
        selected.push(Row {
            terms,
            constant,
            origins: BTreeMap::from([(equation, BigUint::one())]),
        });
    }
    counts.selected_rows += selected.len() as u64;
    counts.peak_slice_rows = counts.peak_slice_rows.max(selected.len());
    counts.peak_slice_variables = counts.peak_slice_variables.max(variables.len());
    let mut basis: BTreeMap<usize, Row> = BTreeMap::new();
    for mut row in selected {
        for (pivot, other) in &basis {
            ctx.event(1)?;
            if let Some(c) = row.terms.get(pivot).cloned() {
                row.subtract(other, &c, ctx)?;
            }
        }
        if let Some((&pivot, c)) = row.terms.first_key_value() {
            let inverse = ctx.inv(c)?;
            row.scale(&inverse, ctx)?;
            basis.insert(pivot, row);
        } else if !row.constant.is_zero() {
            // Contradiction in this slice is sound; no converse completeness claim.
            let inverse = ctx.inv(&row.constant)?;
            row.scale(&inverse, ctx)?;
            return Ok(vec![row.certificate(ctx)?]);
        }
    }
    let mut consequences = Vec::new();
    for row in basis.into_values() {
        ctx.event(1)?;
        if row.terms.len() == 1 {
            consequences.push(row.certificate(ctx)?);
        }
    }
    Ok(consequences)
}
