//! Lossless gate-only JSON wire. The insertion formula is a deterministic
//! expansion; decoded maps still pass the independent original checker.
use super::*;
const SCHEMA: &str = "binary-chain-original-gate-wire/v1";
#[derive(Serialize)]
pub struct Encoded<'a> {
    schema: &'static str,
    map_schema: &'a str,
    identity: &'a Identity,
    variables: u32,
    blocks: &'a [Vec<u32>],
    ands: &'a [(u32, u32, u32)],
    xors: &'a [(Vec<u32>, bool)],
    units: &'a [(u32, bool)],
    impossible: bool,
    domain: &'a [Vec<Lit>],
}
/// Borrow-only view: no clone, allocation, reconstruction or admission reset.
pub fn encode(checked: &CheckedMap) -> Encoded<'_> {
    let m = checked.map();
    Encoded {
        schema: SCHEMA,
        map_schema: &m.schema,
        identity: &m.identity,
        variables: m.variables,
        blocks: &m.blocks,
        ands: &m.ands,
        xors: &m.xors,
        units: &m.units,
        impossible: m.impossible,
        domain: &m.domain,
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Decoded {
    schema: String,
    map_schema: String,
    identity: Identity,
    variables: u32,
    blocks: Vec<Vec<u32>>,
    ands: Vec<(u32, u32, u32)>,
    xors: Vec<(Vec<u32>, bool)>,
    units: Vec<(u32, bool)>,
    impossible: bool,
    domain: Vec<Vec<Lit>>,
}
#[derive(Clone, Copy)]
pub struct DecodeLimits {
    pub wire_bytes: usize,
    pub work: u64,
    pub bytes: u64,
}
#[derive(Debug)]
pub struct DecodeFailure<'a> {
    pub reason: Error,
    /// Borrowed raw evidence remains available on every failure, including parse.
    pub raw: &'a [u8],
    /// Owned gates and partial expansion retained after successful typed parsing.
    pub retained: Option<Map>,
}
fn reserve(
    a: &mut impl Admission,
    total: &mut Quote,
    limits: DecodeLimits,
    work: u64,
    bytes: u64,
) -> Result<(), Error> {
    let q = Quote {
        work: total.work.checked_add(work).ok_or(Error::Cap)?,
        bytes: total.bytes.checked_add(bytes).ok_or(Error::Cap)?,
    };
    if q.work > limits.work || q.bytes > limits.bytes {
        return Err(Error::Cap);
    }
    a.reserve(Quote { work, bytes })?;
    *total = q;
    a.check()
}
/// The caller owns one cumulative admission ledger and raw-read allocation.
/// This function separately admits typed parsing and exact formula expansion.
pub fn decode_checked<'a>(
    raw: &'a [u8],
    original: &Original<'_>,
    limits: DecodeLimits,
    check_limits: Limits,
    admission: &mut impl Admission,
) -> Result<CheckedMap, DecodeFailure<'a>> {
    let mut retained = None;
    let result = (|| {
        if raw.len() > limits.wire_bytes {
            return Err(Error::Cap);
        }
        let mut total = Quote { work: 0, bytes: 0 };
        let len = u64::try_from(raw.len()).map_err(|_| Error::Cap)?;
        reserve(
            admission,
            &mut total,
            limits,
            len.checked_mul(256)
                .and_then(|v| v.checked_add(1024))
                .ok_or(Error::Cap)?,
            len.checked_mul(128).ok_or(Error::Cap)?,
        )?;
        let d: Decoded = serde_json::from_slice(raw).map_err(|_| Error::Semantics)?;
        let schema_ok = d.schema == SCHEMA;
        retained = Some(Map {
            schema: d.map_schema,
            identity: d.identity,
            variables: d.variables,
            blocks: d.blocks,
            ands: d.ands,
            xors: d.xors,
            units: d.units,
            impossible: d.impossible,
            domain: d.domain,
            formula: Vec::new(),
        });
        let m = retained.as_mut().unwrap();
        if !schema_ok {
            return Err(Error::Identity);
        }
        if m.variables > check_limits.variables || m.variables > i32::MAX as u32 {
            return Err(Error::Cap);
        }
        let valid_wire = |w: u32| w > 0 && w <= m.variables;
        for &(g, l, r) in &m.ands {
            admission.check()?;
            if !valid_wire(g) || !valid_wire(l) || !valid_wire(r) {
                return Err(Error::Formula);
            }
        }
        for (w, _) in &m.xors {
            admission.check()?;
            if w.iter().any(|v| !valid_wire(*v)) {
                return Err(Error::Formula);
            }
        }
        for &(w, _) in &m.units {
            admission.check()?;
            if !valid_wire(w) {
                return Err(Error::Formula);
            }
        }
        // Counts and slot arithmetic are checked before expansion allocation.
        let count = m
            .ands
            .len()
            .checked_mul(3)
            .and_then(|n| n.checked_add(m.xors.len()))
            .and_then(|n| n.checked_add(m.units.len()))
            .and_then(|n| n.checked_add(m.domain.len()))
            .and_then(|n| n.checked_add(usize::from(m.impossible)))
            .ok_or(Error::Cap)?;
        let mut slots = m
            .ands
            .len()
            .checked_mul(7)
            .and_then(|n| n.checked_add(m.units.len()))
            .ok_or(Error::Cap)?;
        for (w, _) in &m.xors {
            admission.check()?;
            slots = slots.checked_add(w.len()).ok_or(Error::Cap)?;
        }
        for row in &m.domain {
            admission.check()?;
            slots = slots.checked_add(row.len()).ok_or(Error::Cap)?;
        }
        // Exact outer reservation, upper-bound inner storage and expansion work.
        let bytes = (count as u64)
            .checked_mul(std::mem::size_of::<Constraint>() as u64)
            .and_then(|n| n.checked_add((slots as u64).checked_mul(4)?))
            .ok_or(Error::Cap)?;
        let work = (slots as u64)
            .checked_mul(16)
            .and_then(|n| n.checked_add((count as u64).checked_mul(32)?))
            .ok_or(Error::Cap)?;
        reserve(admission, &mut total, limits, work, bytes)?;
        m.formula
            .try_reserve_exact(count)
            .map_err(|_| Error::Admission)?;
        // Keep every completed clause inside retained Map before the next check.
        // The three fixed arrays live on the stack; heap storage is prepaid.
        for &(gate, left, right) in &m.ands {
            admission.check()?;
            let (g, l, r) = (gate as Lit, left as Lit, right as Lit);
            m.formula.push(Constraint::Clause {
                literals: vec![-g, l],
            });
            m.formula.push(Constraint::Clause {
                literals: vec![-g, r],
            });
            m.formula.push(Constraint::Clause {
                literals: vec![g, -l, -r],
            });
        }
        for (w, rhs) in &m.xors {
            admission.check()?;
            m.formula.push(Constraint::Xor {
                wires: w.clone(),
                rhs: *rhs,
            });
        }
        for &(w, value) in &m.units {
            admission.check()?;
            m.formula.push(Constraint::Clause {
                literals: vec![if value { w as Lit } else { -(w as Lit) }],
            });
        }
        if m.impossible {
            admission.check()?;
            m.formula.push(Constraint::Clause {
                literals: Vec::new(),
            });
        }
        for row in &m.domain {
            admission.check()?;
            m.formula.push(Constraint::Clause {
                literals: row.clone(),
            });
        }
        verify_original(original, m, check_limits, admission)?;
        admission.check()
    })();
    match result {
        Ok(()) => Ok(CheckedMap(retained.take().unwrap())),
        Err(reason) => Err(DecodeFailure {
            reason,
            raw,
            retained,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
        calls: usize,
        refuse_at: Option<usize>,
    }
    impl Admission for Ledger {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            self.calls += 1;
            if self.refuse_at == Some(self.calls) {
                return Err(Error::Admission);
            }
            self.work = self.work.checked_add(q.work).unwrap();
            self.bytes = self.bytes.checked_add(q.bytes).unwrap();
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
    #[test]
    fn actual_map_exact_round_trip_and_corruption_retention() {
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let b = F2mElement::from_bit_positions(&[1], 3);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        let caps = Limits {
            variables: 10000,
            codes: 8,
            domain_clauses: 10000,
            work: u64::MAX,
            bytes: u64::MAX,
        };
        let decode_caps = DecodeLimits {
            wire_bytes: 4_000_000,
            work: u64::MAX,
            bytes: u64::MAX,
        };
        for twist in [false, true] {
            for pattern in 0..4 {
                let original = Original {
                    binding: [7; 32],
                    a: &a,
                    b: &b,
                    irr: &irr,
                    twist,
                    m: 4,
                    target: Some(&a),
                    pattern,
                    codes: &codes,
                };
                let map = build_original(&original, caps, &mut Ledger::default()).unwrap();
                let full = serde_json::to_vec(map.map()).unwrap();
                let raw = serde_json::to_vec(&encode(&map)).unwrap();
                assert!(raw.len() < full.len());
                let checked =
                    decode_checked(&raw, &original, decode_caps, caps, &mut Ledger::default())
                        .unwrap();
                assert_eq!(serde_json::to_vec(checked.map()).unwrap(), full);
                assert_eq!(checked.formula(), map.formula());
                // Pattern 3 has no AND gates: retain its full positive roundtrip,
                // but exercise gate corruption on the accepted pattern-0 map
                // for each twist, whose gate-bearing premise is asserted.
                if pattern == 0 {
                    assert!(map.map().ands.len() >= 2);
                }
                for mutation in 0..6 {
                    if matches!(mutation, 2 | 4) && pattern != 0 {
                        continue;
                    }
                    let mut value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
                    match mutation {
                        0 => value["schema"] = serde_json::json!("unsupported/v2"),
                        1 => value["unexpected"] = serde_json::json!(true),
                        2 => {
                            let arr = value["ands"].as_array_mut().unwrap();
                            arr.swap(0, 1);
                        }
                        3 => value["identity"]["pattern"] = serde_json::json!(pattern + 1),
                        4 => value["ands"][0][0] = serde_json::json!(2147483648u32),
                        _ => value["map_schema"] = serde_json::json!("foreign/v1"),
                    }
                    let changed = serde_json::to_vec(&value).unwrap();
                    let mut ledger = Ledger::default();
                    let err = decode_checked(&changed, &original, decode_caps, caps, &mut ledger)
                        .unwrap_err();
                    assert_eq!(err.raw, changed.as_slice());
                    assert!(ledger.work > 0);
                    if mutation != 1 {
                        assert!(err.retained.is_some());
                    }
                }
                let mut ledger = Ledger {
                    refuse_at: Some(2),
                    ..Ledger::default()
                };
                let failed =
                    decode_checked(&raw, &original, decode_caps, caps, &mut ledger).unwrap_err();
                assert!(ledger.work > 0);
                assert!(failed.retained.is_some());
                assert!(failed.retained.as_ref().unwrap().formula.is_empty());
                let mut ledger = Ledger::default();
                let small = DecodeLimits {
                    wire_bytes: raw.len() - 1,
                    ..decode_caps
                };
                let failed = decode_checked(&raw, &original, small, caps, &mut ledger).unwrap_err();
                assert_eq!(ledger.work, 0);
                assert!(failed.retained.is_none());
            }
        }
    }
}
