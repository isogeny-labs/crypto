//! Direct BigUint coefficient reconstruction and separate lease/trail reader.
//! No proof of all assignments, completeness, prime field or signed execution.
use crate::{Answer, Arithmetic, Consequence, DynamicCounts, Event, Limits, Report, Stop};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::BTreeMap;
use torus_s4::{Field, System};

pub(super) enum Failure {
    Budget(String),
    Invalid(String),
}
type Result<T> = std::result::Result<T, Failure>;
fn invalid(s: &str) -> Failure {
    Failure::Invalid(s.into())
}
fn admit_work(ctx: &mut Arithmetic, n: u64) -> Result<()> {
    ctx.event(n)
        .and_then(|_| ctx.reserve(160 * n))
        .map_err(|e| match e {
            Stop::Cap(s) => Failure::Budget(format!("checker {s}")),
            other => Failure::Invalid(format!("checker arithmetic {other:?}")),
        })
}
fn decimal(s: &str, p: &BigUint) -> Result<BigUint> {
    let x = torus_affine_closure::check::decimal(s).map_err(Failure::Invalid)?;
    if &x >= p {
        return Err(invalid("checker noncanonical coefficient"));
    }
    Ok(x)
}
fn mul(
    a: &BigUint,
    b: &BigUint,
    p: &BigUint,
    ctx: &mut Arithmetic,
    c: &mut DynamicCounts,
) -> Result<BigUint> {
    admit_work(ctx, 1)?;
    c.checker_mul += 1;
    Ok((a * b) % p)
}
fn add(
    a: &BigUint,
    b: &BigUint,
    p: &BigUint,
    ctx: &mut Arithmetic,
    c: &mut DynamicCounts,
) -> Result<BigUint> {
    admit_work(ctx, 1)?;
    c.checker_add += 1;
    Ok((a + b) % p)
}
pub(super) fn admit_counted(
    s: &System,
    premise: &[(usize, String)],
    row: &Consequence,
    ctx: &mut Arithmetic,
    counts: &mut DynamicCounts,
) -> Result<()> {
    if !(5..=512).contains(&s.p.bits())
        || !s.p.bit(0)
        || s.variables.is_empty()
        || s.variables.len() > 4096
        || s.equations.len() > 8192
    {
        return Err(invalid("checker public system bounds"));
    }
    if premise.len() > s.variables.len()
        || row.origins.is_empty()
        || row.origins.len() > 32
        || row.coefficients.len() > 1
    {
        return Err(invalid("checker certificate bounds"));
    }
    admit_work(
        ctx,
        (premise.len() + row.origins.len() + row.coefficients.len() + 1) as u64,
    )?;
    let mut values = BTreeMap::new();
    let mut previous = None;
    for (v, text) in premise {
        if *v >= s.variables.len() || previous.is_some_and(|old| old >= *v) {
            return Err(invalid("premise order/index"));
        }
        values.insert(*v, decimal(text, &s.p)?);
        previous = Some(*v);
    }
    let mut sum: BTreeMap<Vec<usize>, BigUint> = BTreeMap::new();
    previous = None;
    for (equation, text) in &row.origins {
        if *equation >= s.equations.len() || previous.is_some_and(|old| old >= *equation) {
            return Err(invalid("origin order/index"));
        }
        previous = Some(*equation);
        let weight = decimal(text, &s.p)?;
        if weight.is_zero() {
            return Err(invalid("zero origin"));
        }
        for (m, c) in &s.equations[*equation].terms {
            if m.len() > 2
                || m.iter().any(|v| *v >= s.variables.len())
                || !m.windows(2).all(|w| w[0] <= w[1])
                || c.is_zero()
                || c >= &s.p
            {
                return Err(invalid("checker original term shape"));
            }
            admit_work(ctx, 1 + m.len() as u64)?;
            counts.checker_terms += 1;
            let mut c = mul(c, &weight, &s.p, ctx, counts)?;
            let mut unknown = Vec::new();
            for v in m {
                if let Some(value) = values.get(v) {
                    c = mul(&c, value, &s.p, ctx, counts)?;
                } else {
                    unknown.push(*v);
                }
            }
            let old = sum.remove(&unknown).unwrap_or_default();
            let c = add(&old, &c, &s.p, ctx, counts)?;
            if !c.is_zero() {
                sum.insert(unknown, c);
            }
        }
    }
    let constant = decimal(&row.constant, &s.p)?;
    let mut expected = BTreeMap::new();
    if !constant.is_zero() {
        expected.insert(vec![], constant);
    }
    for (v, text) in &row.coefficients {
        if *v >= s.variables.len() || values.contains_key(v) {
            return Err(invalid("unit index already assigned"));
        }
        let coefficient = decimal(text, &s.p)?;
        if !coefficient.is_one() {
            return Err(invalid("unit coefficient must be one"));
        }
        expected.insert(vec![*v], coefficient);
    }
    if expected.is_empty() || sum != expected {
        return Err(invalid("conditional coefficient mismatch"));
    }
    Ok(())
}
pub fn admit(
    s: &System,
    premise: &[(usize, String)],
    row: &Consequence,
) -> std::result::Result<DynamicCounts, String> {
    let mut ctx = Arithmetic {
        f: Field { p: s.p.clone() },
        limits: Limits::default(),
        counts: Default::default(),
    };
    let mut counts = DynamicCounts::default();
    admit_counted(s, premise, row, &mut ctx, &mut counts).map_err(|e| match e {
        Failure::Budget(s) | Failure::Invalid(s) => s,
    })?;
    Ok(counts)
}
pub fn verify(s: &System, report: &Report) -> std::result::Result<(), String> {
    if !(5..=512).contains(&s.p.bits())
        || !s.p.bit(0)
        || s.variables.is_empty()
        || s.variables.len() > 4096
        || s.equations.len() > 8192
    {
        return Err("checker public system bounds".into());
    }
    if report.events.len() > 65536 {
        return Err("trace bound".into());
    }
    let mut values = vec![None; s.variables.len()];
    let mut trail = Vec::new();
    let mut active: Option<(u64, &Vec<Consequence>)> = None;
    let mut generation = 0;
    for event in &report.events {
        match event {
            Event::Assign {
                variable,
                value,
                cause,
            } => {
                if *variable >= values.len() || values[*variable].is_some() {
                    return Err("assignment duplicate/index".into());
                }
                let v = torus_affine_closure::check::decimal(value)?;
                if v >= s.p {
                    return Err("assignment canonical".into());
                }
                if let Some((g, at)) = cause {
                    let (current, rows) = active.ok_or("unit applied without lease")?;
                    let row = rows.get(*at).ok_or("unit row index")?;
                    if current != *g
                        || row.coefficients.len() != 1
                        || row.coefficients[0] != (*variable, "1".into())
                    {
                        return Err("unit binding".into());
                    }
                    let c = torus_affine_closure::check::decimal(&row.constant)?;
                    if (&v + c) % &s.p != BigUint::zero() {
                        return Err("unit value".into());
                    }
                }
                values[*variable] = Some(v);
                trail.push(*variable);
            }
            Event::Activate {
                generation: g,
                premise,
                rows,
            } => {
                if active.is_some() || *g <= generation || rows.is_empty() || rows.len() > 32 {
                    return Err("activation order/bounds".into());
                }
                let actual = values
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| v.as_ref().map(|x| (i, x.to_string())))
                    .collect::<Vec<_>>();
                if &actual != premise {
                    return Err("premise differs from trail".into());
                }
                for row in rows {
                    admit(s, premise, row)?;
                }
                generation = *g;
                active = Some((*g, rows));
            }
            Event::Drop { generation: g } => {
                if active.is_none_or(|(current, _)| current != *g) {
                    return Err("drop binding".into());
                }
                active = None;
            }
            Event::Undo { variable } => {
                if active.is_some() || trail.pop() != Some(*variable) {
                    return Err("active lease/non-LIFO undo".into());
                }
                values[*variable] = None;
            }
        }
    }
    if let Answer::Root(strings) = &report.answer {
        let root = strings
            .iter()
            .map(|s| torus_affine_closure::check::decimal(s))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        torus_affine_closure::check::assignment(s, &root)?;
        if !report.events.is_empty()
            && values
                .into_iter()
                .map(|x| x.unwrap_or_default())
                .collect::<Vec<_>>()
                != root
        {
            return Err("root differs from trail".into());
        }
    }
    Ok(())
}
