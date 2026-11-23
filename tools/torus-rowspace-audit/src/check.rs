//! Reader reconstructs each weighted row directly; it never calls elimination.
use crate::{decimal, decode, Audit, Limits, Meter, Polynomial, RankCertificate, Result, System};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use std::collections::BTreeSet;

fn weighted(
    original: &[Polynomial],
    weights: &[(usize, String)],
    source: usize,
    p: &BigUint,
    m: &mut Meter,
    l: &Limits,
) -> Result<Polynomial> {
    let mut out = Polynomial::new();
    let mut previous = None;
    let mut diagonal = false;
    for (i, c) in weights {
        m.event(1, l)?;
        if *i > source || previous.is_some_and(|old| old >= *i) {
            return Err("Invalid: origin order/triangle".into());
        }
        let c = decimal(c)?;
        if c.is_zero() || &c >= p {
            return Err("Invalid: origin coefficient".into());
        }
        diagonal |= *i == source;
        previous = Some(*i);
        for (monomial, coefficient) in &original[*i] {
            m.event(2, l)?;
            m.write(1, l)?;
            let product = (&c * coefficient) % p;
            let sum = (out.get(monomial).cloned().unwrap_or_default() + product) % p;
            if sum == BigUint::zero() {
                out.remove(monomial);
            } else {
                out.insert(monomial.clone(), sum);
            }
        }
    }
    if !diagonal {
        return Err("Invalid: zero origin diagonal".into());
    }
    Ok(out)
}
fn certificate(
    original: &[Polynomial],
    c: &RankCertificate,
    n: usize,
    p: &BigUint,
    m: &mut Meter,
    l: &Limits,
) -> Result<usize> {
    if c.steps.len() != original.len() {
        return Err("Invalid: source coverage".into());
    }
    let mut pivots = vec![];
    let mut entries = 0;
    for (i, step) in c.steps.iter().enumerate() {
        if step.source != i {
            return Err("Invalid: source sequence".into());
        }
        entries += step.row.len() + step.weights.len();
        if entries > l.entries {
            return Err("Cap: reader entries".into());
        }
        let recomposed = weighted(original, &step.weights, i, p, m, l)?;
        let claimed = decode(&step.row, n, p)?;
        if recomposed != claimed {
            return Err("Invalid: row recomposition".into());
        }
        for old in &pivots {
            m.event(1, l)?;
            if claimed.contains_key(old) {
                return Err("Invalid: earlier pivot not zero".into());
            }
        }
        match &step.pivot {
            Some(pivot) => {
                if claimed.get(pivot) != Some(&BigUint::one()) {
                    return Err("Invalid: unit pivot".into());
                }
                pivots.push(pivot.clone());
            }
            None => {
                if !claimed.is_empty() {
                    return Err("Invalid: nonzero kernel row".into());
                }
            }
        }
    }
    Ok(pivots.len())
}
#[derive(Debug, Serialize)]
pub struct Summary {
    pub variables: usize,
    pub equations: usize,
    pub input_terms: usize,
    pub kept_variables: usize,
    pub original_input_rows: usize,
    pub remaining_rows: usize,
    pub projected_rank: usize,
    pub kernel_dimension: usize,
    pub original_input_rank: usize,
    pub total_input_rank: usize,
    pub new_input_relation_dimension: usize,
    pub reader_meter: Meter,
}
pub fn verify(s: &System, a: &Audit, l: &Limits) -> Result<Summary> {
    if a.schema != "torus.rowspace-audit/v1"
        || !a.keep.windows(2).all(|w| w[0] < w[1])
        || a.keep.iter().any(|&i| i >= s.raw.variables.len())
    {
        return Err("Invalid: audit schema/keep".into());
    }
    let keep: BTreeSet<_> = a.keep.iter().copied().collect();
    let pure = |r: &Polynomial| {
        r.keys()
            .all(|monomial| monomial.iter().all(|i| keep.contains(i)))
    };
    let input_indices: Vec<_> = s
        .equations
        .iter()
        .enumerate()
        .filter(|(_, r)| pure(r))
        .map(|(i, _)| i)
        .collect();
    if input_indices != a.original_input_rows {
        return Err("Invalid: original input rows".into());
    }
    let mut input: Vec<_> = input_indices
        .iter()
        .map(|&i| s.equations[i].clone())
        .collect();
    let remaining: Vec<_> = s.equations.iter().filter(|r| !pure(r)).cloned().collect();
    let projected: Vec<_> = remaining
        .iter()
        .map(|r| {
            r.iter()
                .filter(|(m, _)| m.iter().any(|i| !keep.contains(i)))
                .map(|(m, c)| (m.clone(), c.clone()))
                .collect()
        })
        .collect();
    let mut meter = Meter::default();
    let projected_rank = certificate(
        &projected,
        &a.projection,
        s.raw.variables.len(),
        &s.p,
        &mut meter,
        l,
    )?;
    let original_input_rank = certificate(
        &input,
        &a.baseline,
        s.raw.variables.len(),
        &s.p,
        &mut meter,
        l,
    )?;
    let mut expected = vec![];
    for (i, step) in a.projection.steps.iter().enumerate() {
        if step.pivot.is_none() {
            let full = weighted(&remaining, &step.weights, i, &s.p, &mut meter, l)?;
            if !pure(&full) {
                return Err("Invalid: noninput image".into());
            }
            expected.push(crate::encode(&full));
        }
    }
    if expected != a.images {
        return Err("Invalid: image coverage/value".into());
    }
    for row in &a.images {
        input.push(decode(row, s.raw.variables.len(), &s.p)?);
    }
    let total_input_rank = certificate(
        &input,
        &a.combined,
        s.raw.variables.len(),
        &s.p,
        &mut meter,
        l,
    )?;
    if total_input_rank < original_input_rank {
        return Err("Invalid: rank decreased".into());
    }
    Ok(Summary {
        variables: s.raw.variables.len(),
        equations: s.equations.len(),
        input_terms: s.equations.iter().map(|r| r.len()).sum(),
        kept_variables: a.keep.len(),
        original_input_rows: input_indices.len(),
        remaining_rows: remaining.len(),
        projected_rank,
        kernel_dimension: remaining.len() - projected_rank,
        original_input_rank,
        total_input_rank,
        new_input_relation_dimension: total_input_rank - original_input_rank,
        reader_meter: meter,
    })
}
