//! Independent replay of deterministic initial constraint insertion. Only a
//! fresh attachment before solve/root propagation is admitted. Gate provenance
//! authenticates the original stream; this module checks its initial reduction.
use super::{Admission, CheckedMap, Constraint, Error, Quote};
use crate::cryptanalysis::sat::{Lit, SolverEvent, SolverEventReason};

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub work: u64,
    pub bytes: u64,
    pub events: usize,
    pub constraints: usize,
}
#[derive(Debug, PartialEq, Eq)]
pub enum InitialError {
    Admission(Error),
    Cap,
    InvalidWire,
    Snapshot,
    Clause,
    Xor,
    Assignment,
    ExtraEvents,
}
impl From<Error> for InitialError {
    fn from(e: Error) -> Self {
        Self::Admission(e)
    }
}
#[derive(Debug)]
pub struct Clause {
    id: u32,
    literals: Vec<Lit>,
}
#[derive(Debug)]
pub struct Xor {
    variables: Vec<u32>,
    rhs: bool,
}
#[derive(Debug)]
pub struct Root {
    literal: Lit,
    reason: u32,
}
/// The shared borrows prevent mutation or substitution of the checked map and
/// observed slice during this result's lifetime. No capture watermark is implied.
pub struct VerifiedInitial<'a> {
    original: &'a CheckedMap,
    events: &'a [SolverEvent],
    clauses: Vec<Clause>,
    xors: Vec<Xor>,
    roots: Vec<Root>,
    insertion_contradiction: bool,
}
impl VerifiedInitial<'_> {
    pub fn original(&self) -> &CheckedMap {
        self.original
    }
    pub fn events(&self) -> &[SolverEvent] {
        self.events
    }
    pub fn clause_count(&self) -> usize {
        self.clauses.len()
    }
    pub fn xor_count(&self) -> usize {
        self.xors.len()
    }
    pub fn root_count(&self) -> usize {
        self.roots.len()
    }
    /// Consistency with initial insertion only, NOT an UNSAT proof API.
    pub fn insertion_contradiction(&self) -> bool {
        self.insertion_contradiction
    }
}
fn add(a: u64, b: u64) -> Result<u64, InitialError> {
    a.checked_add(b).ok_or(InitialError::Cap)
}
fn mul(a: u64, b: u64) -> Result<u64, InitialError> {
    a.checked_mul(b).ok_or(InitialError::Cap)
}
fn reserve(
    admission: &mut impl Admission,
    used: &mut Quote,
    q: Quote,
    limits: Limits,
) -> Result<(), InitialError> {
    let next = Quote {
        work: add(used.work, q.work)?,
        bytes: add(used.bytes, q.bytes)?,
    };
    if next.work > limits.work || next.bytes > limits.bytes {
        return Err(InitialError::Cap);
    }
    admission.reserve(q)?;
    *used = next;
    admission.check()?;
    Ok(())
}

pub fn verify_initial<'a>(
    original: &'a CheckedMap,
    events: &'a [SolverEvent],
    limits: Limits,
    admission: &mut impl Admission,
) -> Result<VerifiedInitial<'a>, InitialError> {
    let state = replay_insertion(
        original.formula(),
        original.variables(),
        events,
        limits,
        admission,
    )?;
    Ok(VerifiedInitial {
        original,
        events,
        clauses: state.clauses,
        xors: state.xors,
        roots: state.roots,
        insertion_contradiction: state.contradiction,
    })
}
struct Inserted {
    clauses: Vec<Clause>,
    xors: Vec<Xor>,
    roots: Vec<Root>,
    contradiction: bool,
}
fn replay_insertion(
    raw: &[Constraint],
    variables: u32,
    events: &[SolverEvent],
    limits: Limits,
    admission: &mut impl Admission,
) -> Result<Inserted, InitialError> {
    if raw.len() > limits.constraints
        || events.len() > limits.events
        || variables == 0
        || variables > i32::MAX as u32
    {
        return Err(InitialError::Cap);
    }
    let mut used = Quote { work: 0, bytes: 0 };
    // Pay for the length-only preflight BEFORE scanning either slice. No
    // uncharged walk of attacker-controlled event payloads or raw literals.
    reserve(
        admission,
        &mut used,
        Quote {
            work: mul(add(add(raw.len() as u64, events.len() as u64)?, 1)?, 32)?,
            bytes: 0,
        },
        limits,
    )?;
    let (mut slots, mut sort_work, mut observed_words) = (0u64, 0u64, 0u64);
    for (scan_index, constraint) in raw.iter().enumerate() {
        if scan_index % 64 == 0 {
            admission.check()?;
        }
        let len = match constraint {
            Constraint::Clause { literals } => literals.len(),
            Constraint::Xor { wires, .. } => wires.len(),
        } as u64;
        slots = add(slots, len)?;
        let log = 64 - len.leading_zeros() as u64;
        sort_work = add(sort_work, mul(len, add(log, 8)?)?)?;
    }
    for (scan_index, event) in events.iter().enumerate() {
        if scan_index % 64 == 0 {
            admission.check()?;
        }
        let len = match event {
            SolverEvent::Snapshot { trail_limits, .. } => trail_limits.len(),
            SolverEvent::Clause { literals, .. } => literals.len(),
            SolverEvent::XorConstraint { mask, .. } => mask.len(),
            SolverEvent::Assignment { reason, .. } => match reason {
                SolverEventReason::Xor { explanation } => explanation.len(),
                _ => 0,
            },
            _ => 0,
        } as u64;
        observed_words = add(observed_words, len)?;
    }
    let n = variables as u64;
    // Capacity growth/headers for every possibly retained constraint, sparse
    // XOR slots, normalized clause copies, root values/trail/reasons, plus
    // comparison against all observed dense XOR words. Never copy dense rows.
    let bytes = add(
        add(mul(slots, 32)?, mul(raw.len() as u64, 128)?)?,
        add(mul(n, 64)?, 4096)?,
    )?;
    let work = add(
        add(mul(sort_work, 8)?, mul(observed_words, 8)?)?,
        add(mul(n, 8)?, mul(raw.len() as u64, 32)?)?,
    )?;
    reserve(admission, &mut used, Quote { work, bytes }, limits)?;
    let mut values = vec![None; variables as usize];
    let mut clauses = Vec::new();
    let mut xors = Vec::new();
    let mut roots = Vec::new();
    let mut arena = 0u32;
    let mut contradiction = false;
    for constraint in raw {
        admission.check()?;
        match constraint {
            Constraint::Clause { literals } => {
                if literals
                    .iter()
                    .any(|l| *l == 0 || l.unsigned_abs() > variables)
                {
                    return Err(InitialError::InvalidWire);
                }
                let mut row = literals.clone();
                row.sort_unstable_by_key(|l| (l.unsigned_abs(), *l));
                row.dedup();
                if row.windows(2).any(|w| w[0] == -w[1]) {
                    continue;
                }
                let value =
                    |l: Lit| values[(l.unsigned_abs() - 1) as usize].map(|v: bool| v == (l > 0));
                if row.iter().any(|&l| value(l) == Some(true)) {
                    continue;
                }
                row.retain(|&l| value(l) != Some(false));
                if row.is_empty() {
                    contradiction = true;
                    continue;
                }
                let words = u32::try_from(row.len())
                    .map_err(|_| InitialError::Cap)?
                    .checked_add(1)
                    .ok_or(InitialError::Cap)?;
                let next = arena.checked_add(words).ok_or(InitialError::Cap)?;
                if row.len() == 1 {
                    let literal = row[0];
                    let index = (literal.unsigned_abs() - 1) as usize;
                    // A repeated true unit was dropped above; a conflicting
                    // unit became an empty row. Only genuinely new units land.
                    if values[index].is_some() {
                        return Err(InitialError::Assignment);
                    }
                    values[index] = Some(literal > 0);
                    roots.push(Root {
                        literal,
                        reason: arena,
                    });
                }
                clauses.push(Clause {
                    id: arena,
                    literals: row,
                });
                arena = next;
            }
            Constraint::Xor { wires, rhs } => {
                if wires.iter().any(|w| *w == 0 || *w > variables) {
                    return Err(InitialError::InvalidWire);
                }
                let mut row = wires.clone();
                row.sort_unstable();
                let (mut read, mut write) = (0, 0);
                while read < row.len() {
                    let value = row[read];
                    let start = read;
                    while read < row.len() && row[read] == value {
                        read += 1;
                    }
                    if (read - start) % 2 == 1 {
                        row[write] = value;
                        write += 1;
                    }
                }
                row.truncate(write);
                if row.is_empty() {
                    contradiction |= *rhs;
                } else {
                    xors.push(Xor {
                        variables: row,
                        rhs: *rhs,
                    });
                }
            }
        }
    }
    admission.check()?;
    let mut observed = events.iter();
    match observed.next() {
        Some(SolverEvent::Snapshot {
            version: 1,
            variables: n,
            trail_limits,
            already_unsat,
        }) if *n == variables && trail_limits.is_empty() && *already_unsat == contradiction => {}
        _ => return Err(InitialError::Snapshot),
    }
    for (scan_index, expected) in clauses.iter().enumerate() {
        if scan_index % 64 == 0 {
            admission.check()?;
        }
        match observed.next() {
            Some(SolverEvent::Clause {
                clause_id,
                literals,
                existing: true,
                detached: false,
            }) if *clause_id == expected.id && *literals == expected.literals => {}
            _ => return Err(InitialError::Clause),
        }
    }
    let words = (variables as usize).div_ceil(64);
    for (row_id, expected) in xors.iter().enumerate() {
        admission.check()?;
        let mask = match observed.next() {
            Some(SolverEvent::XorConstraint {
                row_id: id,
                mask,
                rhs,
            }) if *id == row_id && *rhs == expected.rhs && mask.len() == words => mask,
            _ => return Err(InitialError::Xor),
        };
        let mut pos = 0;
        for (word, &actual) in mask.iter().enumerate() {
            if word % 64 == 0 {
                admission.check()?;
            }
            let mut expected_word = 0u64;
            while pos < expected.variables.len()
                && ((expected.variables[pos] - 1) as usize) / 64 == word
            {
                expected_word |= 1u64 << ((expected.variables[pos] - 1) % 64);
                pos += 1;
            }
            if actual != expected_word {
                return Err(InitialError::Xor);
            }
        }
        if pos != expected.variables.len() {
            return Err(InitialError::Xor);
        }
    }
    for (index, expected) in roots.iter().enumerate() {
        if index % 64 == 0 {
            admission.check()?;
        }
        match observed.next() {
            Some(SolverEvent::Assignment {
                literal,
                level: 0,
                trail_index,
                initial: true,
                reason: SolverEventReason::Clause { clause_id },
            }) if *literal == expected.literal
                && *trail_index == index
                && *clause_id == expected.reason => {}
            _ => return Err(InitialError::Assignment),
        }
    }
    if observed.next().is_some() {
        return Err(InitialError::ExtraEvents);
    }
    admission.check()?;
    Ok(Inserted {
        clauses,
        xors,
        roots,
        contradiction,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use crate::cryptanalysis::sat::Solver;
    use num_bigint::BigUint;
    use std::sync::{Arc, Mutex};
    #[derive(Default)]
    struct Budget {
        quotes: Vec<Quote>,
        refuse: bool,
    }
    impl Admission for Budget {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            if self.refuse {
                return Err(Error::Admission);
            }
            self.quotes.push(q);
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
    fn limits() -> Limits {
        Limits {
            work: u64::MAX,
            bytes: u64::MAX,
            events: 100_000,
            constraints: 100_000,
        }
    }
    fn native_snapshot(raw: &[Constraint], variables: u32) -> Vec<SolverEvent> {
        let mut solver = Solver::new(variables);
        for row in raw {
            match row {
                Constraint::Clause { literals } => {
                    solver.add_clause(literals.clone());
                }
                Constraint::Xor { wires, rhs } => {
                    solver.add_xor(wires, *rhs);
                }
            }
        }
        let observed = Arc::new(Mutex::new(Vec::new()));
        let sink = observed.clone();
        solver
            .set_event_observer(move |e| {
                sink.lock().unwrap().push(e.clone());
                Ok(())
            })
            .unwrap();
        // Deliberately do not propagate or solve before attachment.
        drop(solver);
        Arc::try_unwrap(observed).unwrap().into_inner().unwrap()
    }
    #[test]
    fn actual_checked_chain_snapshot_and_mutations() {
        use super::super::{build_original, Limits as GateLimits, Original};
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let b = F2mElement::from_bit_positions(&[1], 3);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        for pattern in 0..4 {
            for target in [Some(&a), None] {
                let o = Original {
                    binding: [3; 32],
                    a: &a,
                    b: &b,
                    irr: &irr,
                    twist: true,
                    m: 4,
                    target,
                    pattern,
                    codes: &codes,
                };
                let map = build_original(
                    &o,
                    GateLimits {
                        variables: 10000,
                        codes: 8,
                        domain_clauses: 10000,
                        bytes: u64::MAX,
                        work: u64::MAX,
                    },
                    &mut Budget::default(),
                )
                .unwrap();
                let events = native_snapshot(map.formula(), map.variables());
                let valid =
                    verify_initial(&map, &events, limits(), &mut Budget::default()).unwrap();
                assert!(std::ptr::eq(valid.original(), &map));
                assert_eq!(valid.events(), events.as_slice());
                assert!(valid.clause_count() > 0);
                assert!(valid.root_count() > 0);
                for mutation in 0..7 {
                    let mut changed = events.clone();
                    match mutation {
                        0 => {
                            if let SolverEvent::Snapshot { already_unsat, .. } = &mut changed[0] {
                                *already_unsat = !*already_unsat;
                            }
                        }
                        1 => {
                            if let SolverEvent::Clause { clause_id, .. } = changed
                                .iter_mut()
                                .find(|e| matches!(e, SolverEvent::Clause { .. }))
                                .unwrap()
                            {
                                *clause_id += 1;
                            }
                        }
                        2 => {
                            if let SolverEvent::Clause { detached, .. } = changed
                                .iter_mut()
                                .find(|e| matches!(e, SolverEvent::Clause { .. }))
                                .unwrap()
                            {
                                *detached = true;
                            }
                        }
                        3 => {
                            if let SolverEvent::Assignment { reason, .. } = changed
                                .iter_mut()
                                .find(|e| matches!(e, SolverEvent::Assignment { .. }))
                                .unwrap()
                            {
                                *reason = SolverEventReason::Decision;
                            }
                        }
                        4 => {
                            changed.pop();
                        }
                        5 => changed.push(SolverEvent::SolveBegin {
                            conflicts: 0,
                            conflict_budget: 1,
                        }),
                        6 => {
                            if let SolverEvent::Snapshot { trail_limits, .. } = &mut changed[0] {
                                trail_limits.push(0);
                            }
                        }
                        _ => unreachable!(),
                    }
                    // Roundtrip stands in for a coherently resealed event artifact;
                    // semantic comparison cannot be bypassed by new file hashes.
                    let changed: Vec<SolverEvent> =
                        serde_json::from_slice(&serde_json::to_vec(&changed).unwrap()).unwrap();
                    assert!(
                        verify_initial(&map, &changed, limits(), &mut Budget::default()).is_err()
                    );
                }
            }
        }
    }
    #[test]
    fn actual_native_insertion_edge_cases_and_sparse_xor_masks() {
        let c = |v: Vec<Lit>| Constraint::Clause { literals: v };
        let raw = vec![
            c(vec![2, -2, 1]),
            c(vec![1, 1]),
            c(vec![1]),
            c(vec![-1, 2, 2]),
            c(vec![-1, -2, 3]),
            c(vec![5, 4]),
            c(vec![-3]),
            c(vec![]),
            Constraint::Xor {
                wires: vec![1, 1],
                rhs: false,
            },
            Constraint::Xor {
                wires: vec![2, 2],
                rhs: true,
            },
            Constraint::Xor {
                wires: vec![65, 1, 65, 64, 127],
                rhs: true,
            },
        ];
        let events = native_snapshot(&raw, 127);
        let got = replay_insertion(&raw, 127, &events, limits(), &mut Budget::default()).unwrap();
        assert!(got.contradiction);
        assert_eq!(
            got.clauses.iter().map(|c| c.id).collect::<Vec<_>>(),
            vec![0, 2, 4, 6]
        );
        assert_eq!(
            got.roots
                .iter()
                .map(|r| (r.literal, r.reason))
                .collect::<Vec<_>>(),
            vec![(1, 0), (2, 2), (3, 4)]
        );
        assert_eq!(got.xors.len(), 1);
        assert_eq!(got.xors[0].variables, vec![1, 64, 127]);
        let mut bad = events.clone();
        if let SolverEvent::XorConstraint { mask, .. } = bad
            .iter_mut()
            .find(|e| matches!(e, SolverEvent::XorConstraint { .. }))
            .unwrap()
        {
            mask[1] ^= 1;
        }
        assert!(matches!(
            replay_insertion(&raw, 127, &bad, limits(), &mut Budget::default()),
            Err(InitialError::Xor)
        ));
        let mut bad = events.clone();
        bad.push(SolverEvent::XorMatrixRow {
            row_id: 0,
            mask: vec![0, 0],
            rhs: false,
        });
        assert!(replay_insertion(&raw, 127, &bad, limits(), &mut Budget::default()).is_err());
    }
    #[test]
    fn preflight_is_charged_before_payload_scan_and_no_charge_reset() {
        let raw = vec![Constraint::Clause { literals: vec![1] }];
        let events = native_snapshot(&raw, 1);
        let mut owner = Budget {
            refuse: true,
            ..Budget::default()
        };
        assert!(matches!(
            replay_insertion(&raw, 1, &events, limits(), &mut owner),
            Err(InitialError::Admission(Error::Admission))
        ));
        assert!(owner.quotes.is_empty());
        owner.refuse = false;
        let mut cap = limits();
        cap.bytes = 0;
        assert!(matches!(
            replay_insertion(&raw, 1, &events, cap, &mut owner),
            Err(InitialError::Cap)
        ));
        assert_eq!(owner.quotes.len(), 1); // length scan remains charged on refusal.
        replay_insertion(&raw, 1, &events, limits(), &mut owner).unwrap();
        assert_eq!(owner.quotes.len(), 3);
    }
}
