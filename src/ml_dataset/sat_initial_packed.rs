//! Versioned exact packed initial events. All encoding scratch is stack-owned.
use crate::cryptanalysis::sat::{SolverEvent, SolverEventReason};
use serde::{Serialize, Serializer};
use std::fmt;
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(super) struct State {
    next_clause: u64,
    history: [[u32; 3]; 3],
}
fn bucket(n: usize) -> usize {
    match n {
        2 => 1,
        3 => 2,
        _ => 0,
    }
}
fn zz(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}
fn unzz(v: u64) -> i64 {
    ((v >> 1) as i64) ^ (-((v & 1) as i64))
}
fn var<E>(mut n: u64, out: &mut impl FnMut(u8) -> Result<(), E>) -> Result<(), E> {
    while n >= 128 {
        out((n as u8) | 128)?;
        n >>= 7;
    }
    out(n as u8)
}
pub(super) fn supported(e: &SolverEvent) -> bool {
    matches!(
        e,
        SolverEvent::Snapshot { .. }
            | SolverEvent::Clause { .. }
            | SolverEvent::XorConstraint { .. }
            | SolverEvent::Assignment {
                reason: SolverEventReason::Clause { .. },
                ..
            }
    )
}
/// Caller checks supported before entry. State is copied by the caller until
/// this entire event has been counted/admitted, so refusal keeps prior state.
pub(super) fn encode<E>(
    e: &SolverEvent,
    state: &mut State,
    out: &mut impl FnMut(u8) -> Result<(), E>,
) -> Result<(), E> {
    match e {
        SolverEvent::Snapshot {
            version,
            variables,
            trail_limits,
            already_unsat,
        } => {
            out(u8::from(*already_unsat) << 2)?;
            var(*version as u64, out)?;
            var(*variables as u64, out)?;
            var(trail_limits.len() as u64, out)?;
            for v in trail_limits {
                var(*v as u64, out)?;
            }
        }
        SolverEvent::Clause {
            clause_id,
            literals,
            existing,
            detached,
        } => {
            let n = literals.len();
            let short = if (1..=3).contains(&n) { n as u8 } else { 0 };
            let contiguous = *clause_id as u64 == state.next_clause;
            out(1
                | (u8::from(*existing) << 2)
                | (u8::from(*detached) << 3)
                | (short << 4)
                | (u8::from(contiguous) << 6))?;
            if short == 0 {
                var(n as u64, out)?;
            }
            if !contiguous {
                var(*clause_id as u64, out)?;
            }
            let hist = &mut state.history[bucket(n)];
            for (i, l) in literals.iter().enumerate() {
                let value = l.unsigned_abs();
                let prior = if i < 3 { hist[i] } else { 0 };
                var(zz(value as i64 - prior as i64) * 2 + u64::from(*l < 0), out)?;
                if i < 3 {
                    hist[i] = value;
                }
            }
            state.next_clause = *clause_id as u64 + n as u64 + 1;
        }
        SolverEvent::XorConstraint { row_id, mask, rhs } => {
            out(2 | (u8::from(*rhs) << 2))?;
            var(*row_id as u64, out)?;
            var(mask.len() as u64, out)?;
            var(mask.iter().map(|w| w.count_ones() as u64).sum(), out)?;
            let mut previous = 0;
            for (i, word) in mask.iter().enumerate() {
                let mut word = *word;
                while word != 0 {
                    let bit = i as u64 * 64 + word.trailing_zeros() as u64;
                    var(bit - previous, out)?;
                    previous = bit;
                    word &= word - 1;
                }
            }
        }
        SolverEvent::Assignment {
            literal,
            level,
            trail_index,
            initial,
            reason: SolverEventReason::Clause { clause_id },
        } => {
            out(3 | (u8::from(*initial) << 2))?;
            var(zz(*literal as i64), out)?;
            var(zz(*level as i64), out)?;
            var(*trail_index as u64, out)?;
            var(*clause_id as u64, out)?;
        }
        _ => unreachable!("caller admitted closed initial variant"),
    }
    Ok(())
}
pub(super) struct Packed<'a> {
    pub events: &'a [SolverEvent],
    pub state: State,
}
struct Text<'a>(&'a Packed<'a>);
impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const ABC: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut state = self.0.state;
        let mut buf = [0u8; 3];
        let mut used = 0;
        let mut emit = |b: u8| -> fmt::Result {
            buf[used] = b;
            used += 1;
            if used == 3 {
                let s = [
                    ABC[(buf[0] >> 2) as usize],
                    ABC[(((buf[0] & 3) << 4) | (buf[1] >> 4)) as usize],
                    ABC[(((buf[1] & 15) << 2) | (buf[2] >> 6)) as usize],
                    ABC[(buf[2] & 63) as usize],
                ];
                f.write_str(std::str::from_utf8(&s).unwrap())?;
                used = 0;
            }
            Ok(())
        };
        for e in self.0.events {
            encode(e, &mut state, &mut emit)?;
        }
        drop(emit);
        if used != 0 {
            let s = [
                ABC[(buf[0] >> 2) as usize],
                ABC[(((buf[0] & 3) << 4) | if used == 2 { buf[1] >> 4 } else { 0 }) as usize],
                if used == 2 {
                    ABC[((buf[1] & 15) << 2) as usize]
                } else {
                    b'='
                },
                b'=',
            ];
            f.write_str(std::str::from_utf8(&s).unwrap())?;
        }
        Ok(())
    }
}
impl Serialize for Packed<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.events.iter().any(|e| !supported(e)) {
            return Err(serde::ser::Error::custom(
                "unsupported packed initial event",
            ));
        }
        // serde_json 1.0.154 collect_str streams Formatter writes directly.
        s.collect_str(&Text(self))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DecodeError {
    Truncated,
    Overflow,
    Noncanonical,
    Tag,
    Cap,
    Admission,
}
pub(super) struct DecodeLimits {
    pub events: usize,
    pub slots: usize,
    pub words: usize,
}
struct Reader<'a> {
    raw: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn byte(&mut self) -> Result<u8, DecodeError> {
        let b = *self.raw.get(self.at).ok_or(DecodeError::Truncated)?;
        self.at += 1;
        Ok(b)
    }
    fn var(&mut self) -> Result<u64, DecodeError> {
        let mut n = 0u64;
        for i in 0..10 {
            let b = self.byte()?;
            if i == 9 && b > 1 {
                return Err(DecodeError::Overflow);
            }
            n |= ((b & 127) as u64) << (7 * i);
            if b < 128 {
                if i > 0 && b == 0 {
                    return Err(DecodeError::Noncanonical);
                }
                return Ok(n);
            }
        }
        Err(DecodeError::Overflow)
    }
    fn size(&mut self) -> Result<usize, DecodeError> {
        usize::try_from(self.var()?).map_err(|_| DecodeError::Overflow)
    }
    fn u32(&mut self) -> Result<u32, DecodeError> {
        u32::try_from(self.var()?).map_err(|_| DecodeError::Overflow)
    }
    fn i32(&mut self) -> Result<i32, DecodeError> {
        i32::try_from(unzz(self.var()?)).map_err(|_| DecodeError::Overflow)
    }
}
/// Raw binary is already bounded, decoded from canonical base64 and retained by
/// caller. Each event and all payload storage are prepaid through reserve before
/// allocation. Completed events remain in output on every error. No success seal.
pub(super) fn decode_into(
    raw: &[u8],
    state: &mut State,
    output: &mut Vec<SolverEvent>,
    limits: DecodeLimits,
    reserve: &mut impl FnMut(u64, u64) -> Result<(), DecodeError>,
) -> Result<(), DecodeError> {
    let mut r = Reader { raw, at: 0 };
    let mut slots = 0usize;
    let mut words = 0usize;
    reserve(
        (raw.len() as u64)
            .checked_mul(32)
            .ok_or(DecodeError::Overflow)?,
        0,
    )?;
    while r.at < raw.len() {
        if output.len() >= limits.events {
            return Err(DecodeError::Cap);
        }
        // Geometric retained-vector growth is prepaid, including relocation.
        // Repeated reserve_exact(1) would introduce uncharged quadratic copying.
        if output.len() == output.capacity() {
            let target = output
                .len()
                .checked_mul(2)
                .unwrap_or(limits.events)
                .max(1)
                .min(limits.events);
            let bytes = (target as u64)
                .checked_mul(std::mem::size_of::<SolverEvent>() as u64)
                .ok_or(DecodeError::Overflow)?;
            let work = (output.len() as u64)
                .checked_mul(4)
                .and_then(|n| n.checked_add(128))
                .ok_or(DecodeError::Overflow)?;
            reserve(work, bytes)?;
            output
                .try_reserve_exact(target - output.len())
                .map_err(|_| DecodeError::Admission)?;
        } else {
            reserve(128, 0)?;
        }
        let h = r.byte()?;
        let mut next = *state;
        match h & 3 {
            0 => {
                if h & !7 != 0 {
                    return Err(DecodeError::Tag);
                }
                let version = r.u32()?;
                let variables = r.u32()?;
                let n = r.size()?;
                slots = slots
                    .checked_add(n)
                    .filter(|n| *n <= limits.slots)
                    .ok_or(DecodeError::Cap)?;
                reserve(
                    (n as u64).checked_mul(16).ok_or(DecodeError::Overflow)?,
                    (n as u64)
                        .checked_mul(std::mem::size_of::<usize>() as u64)
                        .ok_or(DecodeError::Overflow)?,
                )?;
                output.push(SolverEvent::Snapshot {
                    version,
                    variables,
                    trail_limits: Vec::new(),
                    already_unsat: h & 4 != 0,
                });
                let SolverEvent::Snapshot { trail_limits, .. } = output.last_mut().unwrap() else {
                    unreachable!()
                };
                trail_limits
                    .try_reserve_exact(n)
                    .map_err(|_| DecodeError::Admission)?;
                for _ in 0..n {
                    trail_limits.push(r.size()?);
                }
            }
            1 => {
                if h & 128 != 0 {
                    return Err(DecodeError::Tag);
                }
                let short = (h >> 4) & 3;
                let n = if short == 0 {
                    let n = r.size()?;
                    if (1..=3).contains(&n) {
                        return Err(DecodeError::Noncanonical);
                    }
                    n
                } else {
                    short as usize
                };
                let cid = if h & 64 != 0 {
                    u32::try_from(next.next_clause).map_err(|_| DecodeError::Overflow)?
                } else {
                    let c = r.u32()?;
                    if c as u64 == next.next_clause {
                        return Err(DecodeError::Noncanonical);
                    }
                    c
                };
                slots = slots
                    .checked_add(n)
                    .filter(|n| *n <= limits.slots)
                    .ok_or(DecodeError::Cap)?;
                reserve(
                    (n as u64).checked_mul(32).ok_or(DecodeError::Overflow)?,
                    (n as u64).checked_mul(4).ok_or(DecodeError::Overflow)?,
                )?;
                output.push(SolverEvent::Clause {
                    clause_id: cid,
                    literals: Vec::new(),
                    existing: h & 4 != 0,
                    detached: h & 8 != 0,
                });
                let SolverEvent::Clause { literals, .. } = output.last_mut().unwrap() else {
                    unreachable!()
                };
                literals
                    .try_reserve_exact(n)
                    .map_err(|_| DecodeError::Admission)?;
                let hist = &mut next.history[bucket(n)];
                for i in 0..n {
                    let code = r.var()?;
                    let delta = unzz(code >> 1);
                    let prior = if i < 3 { hist[i] as i64 } else { 0 };
                    let value = prior
                        .checked_add(delta)
                        .filter(|n| *n >= 0 && *n <= 2147483648)
                        .ok_or(DecodeError::Overflow)?;
                    let signed = if code & 1 != 0 { -value } else { value };
                    if value == 0 && code & 1 != 0 {
                        return Err(DecodeError::Noncanonical);
                    }
                    let l = i32::try_from(signed).map_err(|_| DecodeError::Overflow)?;
                    literals.push(l);
                    if i < 3 {
                        hist[i] = value as u32;
                    }
                }
                next.next_clause = (cid as u64)
                    .checked_add(n as u64)
                    .and_then(|n| n.checked_add(1))
                    .ok_or(DecodeError::Overflow)?;
            }
            2 => {
                if h & !7 != 0 {
                    return Err(DecodeError::Tag);
                }
                let row_id = r.size()?;
                let n = r.size()?;
                let set = r.size()?;
                words = words
                    .checked_add(n)
                    .filter(|n| *n <= limits.words)
                    .ok_or(DecodeError::Cap)?;
                let bits = n.checked_mul(64).ok_or(DecodeError::Overflow)?;
                if set > bits {
                    return Err(DecodeError::Cap);
                }
                reserve(
                    (n as u64)
                        .checked_mul(16)
                        .and_then(|v| v.checked_add((set as u64).checked_mul(32)?))
                        .ok_or(DecodeError::Overflow)?,
                    (n as u64).checked_mul(8).ok_or(DecodeError::Overflow)?,
                )?;
                output.push(SolverEvent::XorConstraint {
                    row_id,
                    mask: Vec::new(),
                    rhs: h & 4 != 0,
                });
                let SolverEvent::XorConstraint { mask, .. } = output.last_mut().unwrap() else {
                    unreachable!()
                };
                mask.try_reserve_exact(n)
                    .map_err(|_| DecodeError::Admission)?;
                mask.resize(n, 0);
                let mut previous = 0usize;
                for i in 0..set {
                    let delta = r.size()?;
                    if i > 0 && delta == 0 {
                        return Err(DecodeError::Noncanonical);
                    }
                    let bit = previous
                        .checked_add(delta)
                        .filter(|b| *b < bits)
                        .ok_or(DecodeError::Overflow)?;
                    mask[bit / 64] |= 1u64 << (bit % 64);
                    previous = bit;
                }
            }
            3 => {
                if h & !7 != 0 {
                    return Err(DecodeError::Tag);
                }
                output.push(SolverEvent::Assignment {
                    literal: r.i32()?,
                    level: r.i32()?,
                    trail_index: r.size()?,
                    initial: h & 4 != 0,
                    reason: SolverEventReason::Clause {
                        clause_id: r.u32()?,
                    },
                });
            }
            _ => unreachable!(),
        }
        *state = next;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    fn raw(events: &[SolverEvent], state: State) -> Vec<u8> {
        let json = serde_json::to_vec(&Packed { events, state }).unwrap();
        let text: String = serde_json::from_slice(&json).unwrap();
        base64::engine::general_purpose::STANDARD
            .decode(text)
            .unwrap()
    }
    fn limits() -> DecodeLimits {
        DecodeLimits {
            events: 100,
            slots: 1000,
            words: 1000,
        }
    }
    #[test]
    fn exact_fields_split_chunks_and_canonical_refusals() {
        let events = vec![
            SolverEvent::Snapshot {
                version: u32::MAX,
                variables: u32::MAX,
                trail_limits: vec![0, usize::MAX],
                already_unsat: true,
            },
            SolverEvent::Clause {
                clause_id: 0,
                literals: vec![-7, 10],
                existing: true,
                detached: false,
            },
            SolverEvent::Clause {
                clause_id: 3,
                literals: vec![-8, 11],
                existing: false,
                detached: true,
            },
            SolverEvent::Clause {
                clause_id: 19,
                literals: vec![i32::MIN, 0, i32::MAX],
                existing: true,
                detached: true,
            },
            SolverEvent::Clause {
                clause_id: u32::MAX,
                literals: vec![],
                existing: false,
                detached: false,
            },
            SolverEvent::XorConstraint {
                row_id: usize::MAX,
                mask: vec![0, 1, u64::MAX, 0],
                rhs: true,
            },
            SolverEvent::Assignment {
                literal: i32::MIN,
                level: i32::MAX,
                trail_index: usize::MAX,
                initial: false,
                reason: SolverEventReason::Clause {
                    clause_id: u32::MAX,
                },
            },
        ];
        let full = raw(&events, State::default());
        let mut decoded = Vec::new();
        let mut state = State::default();
        let mut charges = (0u64, 0u64);
        decode_into(&full, &mut state, &mut decoded, limits(), &mut |w, b| {
            charges.0 += w;
            charges.1 += b;
            Ok(())
        })
        .unwrap();
        assert_eq!(decoded, events);
        assert!(charges.0 > 0 && charges.1 > 0);
        assert_eq!(raw(&decoded, State::default()), full);
        for split in 0..=events.len() {
            let mut start = State::default();
            for event in &events[..split] {
                encode(event, &mut start, &mut |_| Ok::<_, ()>(())).unwrap();
            }
            let a = raw(&events[..split], State::default());
            let b = raw(&events[split..], start);
            let mut state = State::default();
            let mut out = Vec::new();
            decode_into(&a, &mut state, &mut out, limits(), &mut |_, _| Ok(())).unwrap();
            assert_eq!(state, start);
            decode_into(&b, &mut state, &mut out, limits(), &mut |_, _| Ok(())).unwrap();
            assert_eq!(out, events);
        }
        for bytes in [
            &[128u8][..],
            &[0, 128, 0][..],
            &[0x21, 0][..],
            &[2, 0, 1, 2, 0, 0][..],
        ] {
            assert!(decode_into(
                bytes,
                &mut State::default(),
                &mut Vec::new(),
                limits(),
                &mut |_, _| Ok(())
            )
            .is_err());
        }
        let truncated = &full[..full.len() - 1];
        let mut retained = Vec::new();
        let mut paid = 0;
        assert!(decode_into(
            truncated,
            &mut State::default(),
            &mut retained,
            limits(),
            &mut |w, _| {
                paid += w;
                Ok(())
            }
        )
        .is_err());
        assert!(!retained.is_empty());
        assert!(paid > 0);
        let mut retained = Vec::new();
        let mut calls = 0;
        assert_eq!(
            decode_into(
                &full,
                &mut State::default(),
                &mut retained,
                limits(),
                &mut |_, _| {
                    calls += 1;
                    if calls == 5 {
                        Err(DecodeError::Admission)
                    } else {
                        Ok(())
                    }
                }
            ),
            Err(DecodeError::Admission)
        );
        assert!(!retained.is_empty());
        assert!(serde_json::to_vec(&Packed {
            events: &[SolverEvent::XorMatrixReset],
            state: State::default()
        })
        .is_err());
    }
}
