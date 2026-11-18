//! Independent equation/finite-domain equivalence audit. This is not a search
//! trace, canonical-byte replay, or authority derived from parsed metadata.
pub use super::super::Limits;
use super::{Admission, Attempt, Error, Polynomial, Quote, Route, Stage as ConstructionStage};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Failure {
    Incomplete,
    Owner(Error),
    Cap,
    Overflow,
    Identity,
    Original,
    Domain,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Identity,
    Original,
    Domain,
    Complete,
}
/// Keeps the actual checked construction and partial independent elimination on
/// every refusal. No public constructor accepts polynomials or an identity.
pub struct Audited {
    source: Attempt,
    failure: Option<Failure>,
    charge: Quote,
    stage: Stage,
    block: usize,
    row: usize,
    pivots: [u64; 61],
    rank: usize,
}
impl Audited {
    pub fn status(&self) -> Result<(), Failure> {
        self.failure.map_or(Ok(()), Err)
    }
    pub fn source(&self) -> &Attempt {
        &self.source
    }
    pub fn charged(&self) -> Quote {
        self.charge
    }
    pub fn retained(&self) -> (Stage, usize, usize, &[u64; 61], usize) {
        (self.stage, self.block, self.row, &self.pivots, self.rank)
    }
}
struct Meter<'a, T: Admission> {
    owner: &'a mut T,
    limits: Limits,
    charge: Quote,
}
impl<T: Admission> Meter<'_, T> {
    fn pay(&mut self, work: u64, bytes: u64) -> Result<(), Failure> {
        self.owner.check().map_err(Failure::Owner)?;
        let w = self
            .charge
            .work
            .checked_add(work)
            .ok_or(Failure::Overflow)?;
        let b = self
            .charge
            .bytes
            .checked_add(bytes)
            .ok_or(Failure::Overflow)?;
        if w > self.limits.work || b > self.limits.bytes {
            return Err(Failure::Cap);
        }
        self.owner
            .reserve(Quote { work, bytes })
            .map_err(Failure::Owner)?;
        self.charge = Quote { work: w, bytes: b };
        self.owner.check().map_err(Failure::Owner)
    }
}
fn has(p: &Polynomial, t: &[u32]) -> bool {
    p.binary_search_by(|a| a.as_slice().cmp(t)).is_ok()
}
fn canonical<T: Admission>(
    p: &Polynomial,
    variables: u32,
    m: &mut Meter<'_, T>,
) -> Result<(), Failure> {
    if p.len() > m.limits.terms {
        return Err(Failure::Cap);
    }
    let mut previous: Option<&[u32]> = None;
    for t in p {
        if t.len() > m.limits.degree {
            return Err(Failure::Cap);
        }
        m.pay(
            (t.len() as u64)
                .checked_mul(4)
                .and_then(|v| v.checked_add(4))
                .ok_or(Failure::Overflow)?,
            0,
        )?;
        if t.iter().any(|v| *v >= variables)
            || t.windows(2).any(|w| w[0] >= w[1])
            || previous.is_some_and(|v| v >= t.as_slice())
        {
            return Err(Failure::Identity);
        }
        previous = Some(t);
    }
    Ok(())
}
fn linear<T: Admission>(
    p: &Polynomial,
    wires: &[u32],
    rhs: bool,
    m: &mut Meter<'_, T>,
) -> Result<(), Failure> {
    // Check every coefficient in the union of supplied and expected support.
    // Repeated source wires cancel by parity, without constructor simplification.
    let factor = (wires.len() as u64)
        .checked_add(p.len() as u64)
        .and_then(|v| v.checked_add(1))
        .ok_or(Failure::Overflow)?;
    let work = (wires.len() as u64)
        .checked_add(1)
        .and_then(|v| v.checked_mul(factor))
        .and_then(|v| v.checked_mul(4))
        .ok_or(Failure::Overflow)?;
    m.pay(work, 0)?;
    if has(p, &[]) != rhs {
        return Err(Failure::Original);
    }
    for term in p {
        if term.is_empty() {
            continue;
        }
        if term.len() != 1 || wires.iter().filter(|w| **w - 1 == term[0]).count() % 2 != 1 {
            return Err(Failure::Original);
        }
    }
    for w in wires {
        if has(p, &[*w - 1]) != (wires.iter().filter(|x| *x == w).count() % 2 == 1) {
            return Err(Failure::Original);
        }
    }
    Ok(())
}
pub fn audit(source: Attempt, limits: Limits, owner: &mut impl Admission) -> Audited {
    let mut out = Audited {
        source,
        failure: None,
        charge: Quote { work: 0, bytes: 0 },
        stage: Stage::Identity,
        block: 0,
        row: 0,
        pivots: [0; 61],
        rank: 0,
    };
    let mut meter = Meter {
        owner,
        limits,
        charge: Quote { work: 0, bytes: 0 },
    };
    if let Err(e) = run(&mut out, &mut meter) {
        out.failure = Some(e)
    }
    out.charge = meter.charge;
    out
}
fn run<T: Admission>(out: &mut Audited, meter: &mut Meter<'_, T>) -> Result<(), Failure> {
    meter.pay(64, std::mem::size_of::<Audited>() as u64 + 64)?;
    if out.source.status().is_err() || out.source.stage() != ConstructionStage::Complete {
        return Err(Failure::Incomplete);
    }
    let id = out.source.identity().ok_or(Failure::Incomplete)?;
    let map = &out.source.source().0;
    let n = map.identity.n as usize;
    let blocks = map.identity.m;
    let equations = out.source.equations();
    if !(1..=61).contains(&n)
        || !(2..=15).contains(&blocks)
        || map.blocks.len() < blocks
        || map.variables > meter.limits.variables
        || equations.len() > meter.limits.generators
    {
        return Err(Failure::Cap);
    }
    if id.schema != super::SCHEMA
        || id.order != super::ORDER
        || id.variables != map.variables
        || id.generators != equations.len()
        || id.n != map.identity.n
        || id.factor_blocks != blocks
        || id.original_binding != map.identity.binding
        || id.rank as usize > n
    {
        return Err(Failure::Identity);
    }
    for p in equations {
        canonical(p, map.variables, meter)?
    }
    out.stage = Stage::Original;
    let prefix = map
        .ands
        .len()
        .checked_add(map.xors.len())
        .and_then(|v| v.checked_add(map.units.len()))
        .and_then(|v| v.checked_add(usize::from(map.impossible)))
        .ok_or(Failure::Overflow)?;
    if equations.len() < prefix {
        return Err(Failure::Original);
    }
    for &(g, a, b) in &map.ands {
        let p = &equations[out.row];
        meter.pay(64, 0)?;
        let singleton = [g - 1];
        let pair = [a.min(b) - 1, a.max(b) - 1];
        let product = if a == b { &pair[..1] } else { &pair[..] };
        if singleton.as_slice() == product {
            if !p.is_empty() {
                return Err(Failure::Original);
            }
        } else if p.len() != 2 || !has(p, &singleton) || !has(p, product) {
            return Err(Failure::Original);
        }
        out.row += 1;
    }
    for (wires, rhs) in &map.xors {
        linear(&equations[out.row], wires, *rhs, meter)?;
        out.row += 1
    }
    for &(wire, rhs) in &map.units {
        linear(&equations[out.row], &[wire], rhs, meter)?;
        out.row += 1
    }
    if map.impossible {
        meter.pay(4, 0)?;
        let p = &equations[out.row];
        if p.len() != 1 || !p[0].is_empty() {
            return Err(Failure::Original);
        }
        out.row += 1
    }
    out.stage = Stage::Domain;
    let count = map.identity.codes.len();
    if count == 0 {
        if id.route != Route::Empty
            || equations.len() != prefix + 1
            || equations[prefix].len() != 1
            || !equations[prefix][0].is_empty()
        {
            return Err(Failure::Domain);
        }
    } else {
        if id.route != Route::Affine {
            return Err(Failure::Identity);
        }
        let per_block = n - id.rank as usize;
        if equations.len() != prefix + blocks * per_block {
            return Err(Failure::Domain);
        }
        for block in 0..blocks {
            meter.pay(64, 0)?;
            out.block = block;
            out.rank = 0;
            out.pivots = [0; 61];
            let wires = &map.blocks[block];
            if wires.len() != n {
                return Err(Failure::Identity);
            }
            for (i, p) in equations[prefix + block * per_block..prefix + (block + 1) * per_block]
                .iter()
                .enumerate()
            {
                out.row = prefix + block * per_block + i;
                let mut coefficients = 0u64;
                let mut rhs = false;
                for term in p {
                    meter.pay(n as u64 + 4, 0)?;
                    match term.as_slice() {
                        [] => rhs = !rhs,
                        [v] => {
                            let bit = wires
                                .iter()
                                .position(|w| *w - 1 == *v)
                                .ok_or(Failure::Domain)?;
                            coefficients ^= 1u64 << bit;
                        }
                        _ => return Err(Failure::Domain),
                    }
                }
                // Membership uses the original authenticated codebook, not the
                // constructor's retained origin, basis, or converted code buffer.
                for words in &map.identity.codes {
                    meter.pay(16, 0)?;
                    if words.len() > 2 {
                        return Err(Failure::Identity);
                    }
                    let code = words.first().copied().unwrap_or(0) as u64
                        | ((words.get(1).copied().unwrap_or(0) as u64) << 32);
                    if code >= (1u64 << n) || ((coefficients & code).count_ones() % 2 == 1) != rhs {
                        return Err(Failure::Domain);
                    }
                }
                // Independent high-pivot elimination of equation coefficients.
                // Constructor instead eliminates low pivots of code differences.
                meter.pay((n as u64) * 4, 0)?;
                let mut v = coefficients;
                for bit in (0..n).rev() {
                    if v & (1u64 << bit) != 0 {
                        if out.pivots[bit] != 0 {
                            v ^= out.pivots[bit]
                        } else {
                            out.pivots[bit] = v;
                            out.rank += 1;
                            break;
                        }
                    }
                }
            }
            // Nonempty membership proves consistency. Distinctness belongs to
            // CheckedMap authentication. Equal cardinality then excludes extras.
            meter.pay(4, 0)?;
            if out.rank != per_block || (1u64 << (n - out.rank)) != count as u64 {
                return Err(Failure::Domain);
            }
        }
    }
    meter.pay(1, 0)?;
    out.stage = Stage::Complete;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{build_original, Limits as MapLimits, Original};
    use super::*;
    use crate::binary_ecc::{F2mElement, IrreduciblePoly};
    use num_bigint::BigUint;
    #[derive(Default)]
    struct Ledger {
        work: u64,
        bytes: u64,
        calls: u64,
        deny: Option<u64>,
    }
    impl Admission for Ledger {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            self.calls += 1;
            if self.deny == Some(self.calls) {
                return Err(Error::Admission);
            }
            self.work = self.work.checked_add(q.work).ok_or(Error::Admission)?;
            self.bytes = self.bytes.checked_add(q.bytes).ok_or(Error::Admission)?;
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }
    fn caps() -> Limits {
        Limits {
            variables: 4096,
            generators: 4096,
            terms: 4096,
            degree: 64,
            work: 4_000_000,
            bytes: 8_000_000,
        }
    }
    fn fixture(n: u32, twist: bool, codes: &[u64]) -> Attempt {
        let element = |x: u64| {
            F2mElement::from_bit_positions(
                &(0..n).filter(|b| x & (1u64 << b) != 0).collect::<Vec<_>>(),
                n,
            )
        };
        let irr = IrreduciblePoly {
            degree: n,
            low_terms: if n == 23 { vec![0, 5] } else { vec![0, 1] },
        };
        let a = element(if n == 23 { 1887600 } else { 1 });
        let b = element(if n == 23 { 570608 } else { 2 });
        let target = element(if n == 23 { 7715066 } else { 1 });
        let codes = codes.iter().copied().map(BigUint::from).collect::<Vec<_>>();
        let o = Original {
            binding: [23; 32],
            a: &a,
            b: &b,
            irr: &irr,
            twist,
            m: 3,
            target: Some(&target),
            pattern: 1,
            codes: &codes,
        };
        let mut owner = Ledger::default();
        let map = build_original(
            &o,
            MapLimits {
                variables: 4096,
                codes: 16,
                domain_clauses: 4096,
                work: 4_000_000,
                bytes: 8_000_000,
            },
            &mut owner,
        )
        .unwrap();
        let out = super::super::construct(map, caps(), &mut owner);
        assert_eq!(out.status(), Ok(()));
        out
    }
    fn rehash(a: &mut Attempt) {
        use sha2::Digest;
        let mut h = sha2::Sha256::new();
        h.update(super::super::SCHEMA.as_bytes());
        h.update(super::super::ORDER.as_bytes());
        serde_json::to_writer(&mut DigestWriter(&mut h), &a.equations).unwrap();
        a.identity.as_mut().unwrap().ordered_generators_sha256 = h.finalize().into();
    }
    struct DigestWriter<'a>(&'a mut sha2::Sha256);
    impl std::io::Write for DigestWriter<'_> {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            use sha2::Digest;
            self.0.update(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    fn prefix(a: &Attempt) -> usize {
        let m = &a.source.0;
        m.ands.len() + m.xors.len() + m.units.len() + usize::from(m.impossible)
    }
    #[test]
    fn independent_equivalence_audits_n23_both_twists_and_small_domains() {
        for twist in [false, true] {
            let out = audit(
                fixture(23, twist, &[4833900, 6976212]),
                caps(),
                &mut Ledger::default(),
            );
            assert_eq!(out.status(), Ok(()));
            assert_eq!(out.retained().0, Stage::Complete);
            assert_eq!(out.retained().4, 22);
            assert!(out.charged().work > 0 && out.charged().work <= 4_000_000);
            for codes in [
                vec![],
                vec![3],
                vec![1, 6],
                vec![0, 3, 5, 6],
                (0..8).collect(),
            ] {
                let out = audit(fixture(3, twist, &codes), caps(), &mut Ledger::default());
                assert_eq!(out.status(), Ok(()));
            }
        }
    }
    #[test]
    fn domain_constant_foreign_variable_and_missing_constraint_are_rejected() {
        for mutation in 0..3 {
            let mut a = fixture(3, false, &[1, 6]);
            let at = prefix(&a);
            match mutation {
                0 => {
                    let p = &mut a.equations[at];
                    if p.first().is_some_and(|t| t.is_empty()) {
                        p.remove(0);
                    } else {
                        p.insert(0, vec![]);
                    }
                }
                1 => {
                    a.equations[at] = vec![vec![a.source.0.blocks[1][0] - 1]];
                }
                _ => {
                    for block in (0..3).rev() {
                        a.equations.remove(at + block * 2);
                    }
                    let id = a.identity.as_mut().unwrap();
                    id.generators = a.equations.len();
                    id.rank = 2;
                }
            }
            rehash(&mut a);
            let out = audit(a, caps(), &mut Ledger::default());
            assert_eq!(out.status(), Err(Failure::Domain));
        }
    }
    #[test]
    fn unchanged_original_prefix_is_checked_without_expansion() {
        let mut a = fixture(3, false, &[0, 1]);
        let p = &mut a.equations[0];
        if p.first().is_some_and(|t| t.is_empty()) {
            p.remove(0);
        } else {
            p.insert(0, vec![]);
        }
        rehash(&mut a);
        let out = audit(a, caps(), &mut Ledger::default());
        assert_eq!(out.status(), Err(Failure::Original));
    }
    #[test]
    fn independent_audit_retains_source_and_partial_state_after_paid_refusal() {
        let baseline = audit(fixture(3, false, &[1, 6]), caps(), &mut Ledger::default());
        assert_eq!(baseline.status(), Ok(()));
        let mut counting = Ledger::default();
        assert_eq!(
            audit(fixture(3, false, &[1, 6]), caps(), &mut counting).status(),
            Ok(())
        );
        let mut owner = Ledger {
            deny: Some(counting.calls - 2),
            ..Ledger::default()
        };
        let out = audit(fixture(3, false, &[1, 6]), caps(), &mut owner);
        assert_eq!(out.status(), Err(Failure::Owner(Error::Admission)));
        assert_eq!(out.retained().0, Stage::Domain);
        assert!(out.charged().work > 0);
        assert_eq!(
            out.charged(),
            Quote {
                work: owner.work,
                bytes: owner.bytes
            }
        );
        assert_eq!(out.source().source().original_binding(), [23; 32]);
        assert!(!out.source().equations().is_empty());
        let mut zero = caps();
        zero.bytes = 0;
        let out = audit(fixture(3, false, &[1, 6]), zero, &mut Ledger::default());
        assert_eq!(out.status(), Err(Failure::Cap));
        assert_eq!(out.charged(), Quote { work: 0, bytes: 0 });
        assert!(out.source().identity().is_some());
    }
}
