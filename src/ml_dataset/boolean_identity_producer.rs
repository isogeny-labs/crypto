//! Arithmetic kernel only. No source, native-row or capture authority.
use std::time::Instant;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unknown {
    Shape,
    Work,
    Bytes,
    Deadline,
    OutsideSpan,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    pub candidates: u64,
    pub terms: u64,
    pub pivots: u64,
    pub row_xors: u64,
    pub provenance_steps: u64,
    pub work: u64,
}
#[derive(Debug)]
pub struct Attempt {
    pub outcome: Result<Vec<u8>, Unknown>,
    pub counts: Counts,
    pub reserved_work: u64,
    pub reserved_bytes: u64,
    retained: State,
}
// Column order is variable-set lexicographic, not mask order.
const MASKS: [u8; 8] = [0, 1, 3, 7, 5, 2, 6, 4];
#[derive(Debug, Clone, Copy, Default)]
struct Row {
    bits: u8,
    ids: [u16; 9],
    len: usize,
}
#[derive(Debug, Default)]
struct State {
    basis: [Row; 8],
    occupied: [bool; 8],
    row: Row,
    candidate: usize,
    suffix: String,
}
fn tick(c: &mut Counts, deadline: Instant) -> Result<(), Unknown> {
    if Instant::now() >= deadline {
        return Err(Unknown::Deadline);
    }
    c.work += 1;
    Ok(())
}
fn add(a: &mut Row, b: &Row, c: &mut Counts, d: Instant) -> Result<(), Unknown> {
    tick(c, d)?;
    c.row_xors += 1;
    a.bits ^= b.bits;
    let mut ids = [0u16; 9];
    let (mut i, mut j, mut n) = (0, 0, 0);
    while i < a.len || j < b.len {
        c.work += 1;
        c.provenance_steps += 1;
        let v = if j == b.len || (i < a.len && a.ids[i] < b.ids[j]) {
            let v = a.ids[i];
            i += 1;
            Some(v)
        } else if i == a.len || b.ids[j] < a.ids[i] {
            let v = b.ids[j];
            j += 1;
            Some(v)
        } else {
            i += 1;
            j += 1;
            None
        };
        if let Some(v) = v {
            if n == 9 {
                return Err(Unknown::Shape);
            }
            ids[n] = v;
            n += 1;
        }
    }
    a.ids = ids;
    a.len = n;
    Ok(())
}
pub fn quote(generators: usize) -> Result<(u64, u64), Unknown> {
    if generators == 0 || generators > 61 {
        return Err(Unknown::Shape);
    }
    let a = generators as u64 * 8;
    Ok((128 + a * 8 + (a + 1) * 8 * 21, 4096))
}
/// Inputs are canonical coefficient vectors: bit mask k is monomial k, N=3.
/// Caller must reserve returned envelope from its ORIGINAL ledger before calling.
/// This kernel has only invocation-local admission; never authenticates inputs.
pub fn produce(g: &[u8], target: u8, work: u64, bytes: u64, deadline: Instant) -> Attempt {
    let mut out = Attempt {
        outcome: Err(Unknown::Shape),
        counts: Counts::default(),
        reserved_work: 0,
        reserved_bytes: 0,
        retained: State::default(),
    };
    let Ok((quote, allocation)) = quote(g.len()) else {
        return out;
    };
    let bytes_quote = allocation; // fixed arrays plus bounded suffix requested capacity
    if work < quote {
        out.outcome = Err(Unknown::Work);
        return out;
    }
    if bytes < bytes_quote {
        out.outcome = Err(Unknown::Bytes);
        return out;
    }
    out.reserved_work = quote;
    out.reserved_bytes = bytes_quote;
    let result = (|| {
        let State {
            basis,
            occupied,
            row,
            candidate,
            suffix,
        } = &mut out.retained;
        for (gi, &poly) in g.iter().enumerate() {
            for (mi, &mult) in MASKS.iter().enumerate() {
                tick(&mut out.counts, deadline)?;
                out.counts.candidates += 1;
                *candidate = gi * 8 + mi;
                *row = Row::default();
                row.ids[0] = (gi * 8 + mi) as u16;
                row.len = 1;
                for mask in 0..8u8 {
                    tick(&mut out.counts, deadline)?;
                    if poly & (1 << mask) != 0 {
                        out.counts.terms += 1;
                        let product = mask | mult;
                        let column = MASKS.iter().position(|&x| x == product).unwrap();
                        row.bits ^= 1 << column;
                    }
                }
                for p in 0..8 {
                    tick(&mut out.counts, deadline)?;
                    out.counts.pivots += 1;
                    if row.bits & (1 << p) == 0 {
                        continue;
                    }
                    if occupied[p] {
                        add(row, &basis[p], &mut out.counts, deadline)?
                    } else {
                        basis[p] = *row;
                        occupied[p] = true;
                        break;
                    }
                }
            }
        }
        *row = Row::default();
        for (c, &mask) in MASKS.iter().enumerate() {
            if target & (1 << mask) != 0 {
                row.bits |= 1 << c
            }
        }
        for p in 0..8 {
            tick(&mut out.counts, deadline)?;
            out.counts.pivots += 1;
            if row.bits & (1 << p) != 0 {
                if !occupied[p] {
                    return Err(Unknown::OutsideSpan);
                }
                add(row, &basis[p], &mut out.counts, deadline)?
            }
        }
        tick(&mut out.counts, deadline)?;
        suffix.try_reserve_exact(1024).map_err(|_| Unknown::Bytes)?;
        use std::fmt::Write;
        write!(suffix, " {}", row.len).unwrap();
        for &id in &row.ids[..row.len] {
            tick(&mut out.counts, deadline)?;
            let mask = MASKS[id as usize % 8];
            write!(suffix, " {} {}", id / 8, mask.count_ones()).unwrap();
            for v in 0..3 {
                if mask & (1 << v) != 0 {
                    write!(suffix, " {v}").unwrap();
                }
            }
        }
        suffix.push('\n');
        debug_assert!(suffix.len() <= 1024);
        Ok(std::mem::take(suffix).into_bytes())
    })();
    out.outcome = result;
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::identity as donor;
    fn check(g: &[u8], target: u8, suffix: &[u8]) -> bool {
        use std::fmt::Write;
        let mut s = format!("BQI1 3 {}", g.len());
        for poly in g.iter().copied().chain([target]) {
            write!(s, " {}", poly.count_ones()).unwrap();
            for mask in MASKS {
                if poly & (1 << mask) != 0 {
                    write!(s, " {}", mask.count_ones()).unwrap();
                    for v in 0..3 {
                        if mask & (1 << v) != 0 {
                            write!(s, " {v}").unwrap();
                        }
                    }
                }
            }
        }
        let mut wire = s.into_bytes();
        wire.extend(suffix);
        donor::verify_untrusted(
            &wire,
            donor::Limits::software(),
            Instant::now() + std::time::Duration::from_secs(10),
        )
        .result
        .is_ok()
    }
    #[test]
    fn exhaustive_one_generator() {
        for g in 0..=255u8 {
            for t in 0..=255u8 {
                let a = produce(
                    &[g],
                    t,
                    u64::MAX,
                    u64::MAX,
                    Instant::now() + std::time::Duration::from_secs(10),
                );
                assert!(a.counts.work <= a.reserved_work);
                let eval = |poly: u8, x: u8| -> bool {
                    (0..8u8)
                        .filter(|&m| poly & (1 << m) != 0 && m & x == m)
                        .count()
                        % 2
                        == 1
                };
                let member = (0..8u8).all(|x| eval(g, x) || !eval(t, x));
                assert_eq!(a.outcome.is_ok(), member);
                if let Ok(p) = a.outcome {
                    assert!(check(&[g], t, &p));
                    assert!(!check(&[g], t ^ 1, &p));
                } else {
                    assert_eq!(a.outcome.unwrap_err(), Unknown::OutsideSpan);
                }
            }
        }
    }
    #[test]
    fn multiple_generators_and_max_shape() {
        let eval = |p: u8, x: u8| {
            (0..8u8)
                .filter(|&m| p & (1 << m) != 0 && m & x == m)
                .count()
                % 2
                == 1
        };
        for gs in [&[105u8, 232][..], &[0u8, 170, 204][..]] {
            for target in 0..=255u8 {
                let a = produce(
                    gs,
                    target,
                    u64::MAX,
                    u64::MAX,
                    Instant::now() + std::time::Duration::from_secs(10),
                );
                let member = (0..8u8).all(|x| gs.iter().any(|&g| eval(g, x)) || !eval(target, x));
                assert_eq!(a.outcome.is_ok(), member);
                assert!(a.counts.work <= a.reserved_work);
                if let Ok(p) = a.outcome {
                    assert!(check(gs, target, &p));
                    assert!(!check(gs, target ^ 1, &p));
                }
            }
        }
        let gs: Vec<u8> = (0..61u8).map(|i| i.wrapping_mul(37)).collect();
        let a = produce(
            &gs,
            129,
            u64::MAX,
            u64::MAX,
            Instant::now() + std::time::Duration::from_secs(10),
        );
        assert!(check(&gs, 129, a.outcome.as_ref().unwrap()));
        assert!(a.counts.work <= a.reserved_work);
        assert!(a.retained.basis.iter().all(|r| r.len <= 8));
        assert!(std::mem::size_of::<Attempt>() + 1024 <= 4096);
        assert_eq!(
            produce(&[], 0, 0, 0, Instant::now()).outcome.unwrap_err(),
            Unknown::Shape
        );
    }
    #[test]
    fn multi_zero_and_caps() {
        let g = [105, 232, 0, 1];
        let a = produce(
            &g,
            129,
            u64::MAX,
            u64::MAX,
            Instant::now() + std::time::Duration::from_secs(10),
        );
        assert!(check(&g, 129, a.outcome.as_ref().unwrap()));
        assert!(
            std::str::from_utf8(a.outcome.as_ref().unwrap())
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap()
                >= 2
        );
        let exact = produce(
            &g,
            129,
            a.reserved_work,
            a.reserved_bytes,
            Instant::now() + std::time::Duration::from_secs(10),
        );
        assert!(exact.outcome.is_ok());
        assert!(a.counts.work <= a.reserved_work);
        assert_eq!(
            produce(
                &g,
                129,
                a.reserved_work - 1,
                a.reserved_bytes,
                Instant::now()
            )
            .outcome
            .unwrap_err(),
            Unknown::Work
        );
        assert_eq!(
            produce(
                &g,
                129,
                a.reserved_work,
                a.reserved_bytes - 1,
                Instant::now()
            )
            .outcome
            .unwrap_err(),
            Unknown::Bytes
        );
        let expired = produce(&g, 129, a.reserved_work, a.reserved_bytes, Instant::now());
        assert_eq!(expired.outcome.unwrap_err(), Unknown::Deadline);
        assert_eq!(expired.reserved_work, a.reserved_work);
    }
}
