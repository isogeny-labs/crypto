//! Proposed cfg(test)-only child of binary_semaev_chain_provenance.
//! Restricted reference calculation, never called by native Solver or verifier.
use super::*;
#[derive(Default, serde::Serialize)]
pub struct Reference {
    pub values: Vec<Option<bool>>,
    pub model: Vec<bool>,
    pub passes: usize,
    pub verified_rows: usize,
}
fn pay(owner: &mut impl Admission, work: u64, bytes: u64) -> Result<(), Error> {
    owner.reserve(Quote { work, bytes })?;
    owner.check()
}
fn set(values: &mut [Option<bool>], wire: u32, value: bool) -> Result<bool, Error> {
    let slot = values
        .get_mut(wire.checked_sub(1).ok_or(Error::Formula)? as usize)
        .ok_or(Error::Formula)?;
    match *slot {
        Some(old) if old != value => Err(Error::Formula),
        Some(_) => Ok(false),
        None => {
            *slot = Some(value);
            Ok(true)
        }
    }
}
fn get(values: &[Option<bool>], wire: u32) -> Result<Option<bool>, Error> {
    values
        .get(wire.checked_sub(1).ok_or(Error::Formula)? as usize)
        .copied()
        .ok_or(Error::Formula)
}
/// The caller provides every named x block from the fixed signed group tuple,
/// including the actual intermediate and target; all auxiliary bits are derived.
/// Restrict to a fully authenticated CheckedMap, not an independently minted map.
pub fn complete(
    map: &CheckedMap,
    xs: &[u64],
    r: &mut Reference,
    owner: &mut impl Admission,
    checkpoint: &mut impl FnMut(&Reference) -> Result<(), Error>,
) -> Result<(), Error> {
    let m = &map.0;
    if !r.values.is_empty() || !r.model.is_empty() || r.passes != 0 || r.verified_rows != 0 {
        return Err(Error::Identity);
    }
    if xs.len() != m.blocks.len() || m.identity.n > 61 {
        return Err(Error::Identity);
    }
    let n = m.variables as usize;
    pay(
        owner,
        n as u64,
        (n.checked_mul(std::mem::size_of::<Option<bool>>())
            .ok_or(Error::Cap)?) as u64,
    )?;
    r.values.try_reserve_exact(n).map_err(|_| Error::Cap)?;
    r.values.resize(n, None);
    for (block, &x) in m.blocks.iter().zip(xs) {
        if x >> m.identity.n != 0 {
            return Err(Error::Semantics);
        }
        for (i, &wire) in block.iter().enumerate() {
            pay(owner, 4, 0)?;
            set(&mut r.values, wire, x >> i & 1 != 0)?;
        }
    }
    for &(wire, value) in &m.units {
        pay(owner, 4, 0)?;
        set(&mut r.values, wire, value)?;
    }
    for _ in 0..=n {
        pay(owner, 4, 0)?;
        r.passes += 1;
        let mut changed = false;
        for &(out, left, right) in &m.ands {
            pay(owner, 8, 0)?;
            if let (Some(a), Some(b)) = (get(&r.values, left)?, get(&r.values, right)?) {
                changed |= set(&mut r.values, out, a && b)?;
            }
        }
        for (wires, rhs) in &m.xors {
            let mut parity = *rhs;
            let mut missing = None;
            let mut multiple = false;
            for &wire in wires {
                pay(owner, 4, 0)?;
                match get(&r.values, wire)? {
                    Some(v) => parity ^= v,
                    None => {
                        if missing.is_some() {
                            multiple = true;
                        }
                        missing = Some(wire);
                    }
                }
            }
            if !multiple {
                if let Some(wire) = missing {
                    changed |= set(&mut r.values, wire, parity)?;
                } else if parity {
                    return Err(Error::Formula);
                }
            }
        }
        checkpoint(r)?;
        if !changed {
            break;
        }
    }
    pay(owner, n as u64, n as u64)?;
    r.model.try_reserve_exact(n).map_err(|_| Error::Cap)?;
    for value in &r.values {
        pay(owner, 2, 0)?;
        r.model.push(value.ok_or(Error::Semantics)?);
    }
    // Exact unsimplified original constraints, including any domain/order rules.
    for row in map.formula() {
        let valid = match row {
            Constraint::Clause { literals } => {
                let mut any = false;
                for &literal in literals {
                    pay(owner, 4, 0)?;
                    let v = get(&r.values, literal.unsigned_abs())?.ok_or(Error::Formula)?;
                    any |= if literal > 0 { v } else { !v };
                }
                any
            }
            Constraint::Xor { wires, rhs } => {
                let mut parity = false;
                for &wire in wires {
                    pay(owner, 3, 0)?;
                    parity ^= get(&r.values, wire)?.ok_or(Error::Formula)?;
                }
                parity == *rhs
            }
        };
        if !valid {
            return Err(Error::Formula);
        }
        r.verified_rows += 1;
        if r.verified_rows % 64 == 0 {
            checkpoint(r)?;
        }
    }
    checkpoint(r)?;
    owner.check()
}

pub fn different_summands(
    map: &CheckedMap,
    a: &Reference,
    b: &Reference,
    owner: &mut impl Admission,
) -> Result<bool, Error> {
    if a.model.len() != map.variables() as usize || b.model.len() != map.variables() as usize {
        return Err(Error::Identity);
    }
    for block in &map.0.blocks[..map.0.identity.m] {
        for &wire in block {
            pay(owner, 4, 0)?;
            if a.model[wire as usize - 1] != b.model[wire as usize - 1] {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
