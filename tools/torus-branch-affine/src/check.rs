//! Separate BigUint specialization and LIFO lifetime checker.
//! It does not validate counters, search completeness, primality or signed execution.
use crate::{Answer, Event, Report};
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::BTreeMap;
use torus_affine_closure::{check as coefficient, Artifact};
use torus_s4::{Poly, System};
type Result<T> = std::result::Result<T, String>;

fn specialize(original: &System, premise: &[(usize, String)]) -> Result<System> {
    let n = original.variables.len();
    if n == 0 || n > 4096 || original.equations.len() > 8192 {
        return Err("checker shape refusal".into());
    }
    let mut assignment = vec![None; n];
    let mut previous = None;
    for (v, text) in premise {
        if *v >= n || previous.is_some_and(|p| p >= *v) {
            return Err("premise order/index".into());
        }
        let value = coefficient::decimal(text)?;
        if value >= original.p {
            return Err("premise noncanonical value".into());
        }
        assignment[*v] = Some(value);
        previous = Some(*v);
    }
    let mut equations = vec![];
    let mut terms_left = 32768usize;
    for equation in &original.equations {
        let mut map: BTreeMap<Vec<usize>, BigUint> = BTreeMap::new();
        for (monomial, coefficient) in &equation.terms {
            if terms_left == 0 {
                return Err("specialization term refusal".into());
            }
            terms_left -= 1;
            if monomial.len() > 2
                || monomial.iter().any(|v| *v >= n)
                || !monomial.windows(2).all(|w| w[0] <= w[1])
                || coefficient.is_zero()
                || coefficient >= &original.p
            {
                return Err("original shape/coefficient".into());
            }
            let mut c = coefficient.clone();
            let mut m = vec![];
            for &v in monomial {
                if let Some(value) = &assignment[v] {
                    c = (c * value) % &original.p;
                } else {
                    m.push(v);
                }
            }
            let value = (map.get(&m).cloned().unwrap_or_default() + c) % &original.p;
            if value.is_zero() {
                map.remove(&m);
            } else {
                map.insert(m, value);
            }
        }
        let mut p = Poly::small(&original.p, 0);
        p.terms = map;
        equations.push(p);
    }
    Ok(System {
        p: original.p.clone(),
        variables: original.variables.clone(),
        equations,
    })
}

/// Conditional consequences are admitted only after two coefficient paths agree.
pub fn admit(
    original: &System,
    roles: &[usize],
    premise: &[(usize, String)],
    certificate: &Artifact,
) -> Result<System> {
    let specialized = specialize(original, premise)?;
    let checked = coefficient::verify(&specialized, roles, certificate)?;
    let mut output = original.clone();
    output
        .equations
        .extend(checked.equations.into_iter().skip(original.equations.len()));
    if output.variables != original.variables || output.equations.len() < original.equations.len() {
        return Err("original prefix".into());
    }
    Ok(output)
}

/// Replays only the conditional-row lifetimes and any returned original root.
pub fn verify(original: &System, roles: &[usize], report: &Report) -> Result<()> {
    if report.events.len() > 65536 {
        return Err("trace byte/count refusal".into());
    }
    let mut assignment = vec![None; original.variables.len()];
    let mut trail = vec![];
    let mut active = None;
    let mut last_generation = 0;
    for event in &report.events {
        match event {
            Event::Assign { variable, value } => {
                if *variable >= assignment.len() || assignment[*variable].is_some() {
                    return Err("duplicate/index assignment".into());
                }
                let value = coefficient::decimal(value)?;
                if value >= original.p {
                    return Err("assignment not canonical".into());
                }
                assignment[*variable] = Some(value);
                trail.push(*variable);
            }
            Event::Undo { variable } => {
                if active.is_some() {
                    return Err("conditional generation survived Undo".into());
                }
                if trail.pop() != Some(*variable) {
                    return Err("non-LIFO Undo".into());
                }
                assignment[*variable] = None;
            }
            Event::Drop { generation } => {
                if active != Some(*generation) {
                    return Err("drop generation binding".into());
                }
                active = None;
            }
            Event::Activate {
                generation,
                premise,
                certificate,
                output_sha256,
            } => {
                if active.is_some() || *generation <= last_generation {
                    return Err("activation lifetime/order".into());
                }
                let actual = assignment
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| v.as_ref().map(|x| (i, x.to_string())))
                    .collect::<Vec<_>>();
                if &actual != premise {
                    return Err("premises differ from replayed trail".into());
                }
                let admitted = admit(original, roles, premise, certificate)?;
                if torus_affine_closure::system_hash(&admitted) != *output_sha256 {
                    return Err("conditional output binding".into());
                }
                active = Some(*generation);
                last_generation = *generation;
            }
        }
    }
    if let Answer::Root(strings) = &report.answer {
        let values = strings
            .iter()
            .map(|s| coefficient::decimal(s))
            .collect::<Result<Vec<_>>>()?;
        coefficient::assignment(original, &values)?;
        if !report.events.is_empty()
            && assignment
                .iter()
                .map(|v| v.clone().unwrap_or_default())
                .collect::<Vec<_>>()
                != values
        {
            return Err("root differs from final assignment trail".into());
        }
    }
    Ok(())
}
