//! Bounded, opt-in provenance of original chain construction, before Solver
//! simplification. This does not authenticate a native snapshot or search trace.
use super::*;

/// Fresh pre-solve attachment only; not a search-prefix or UNSAT verifier.
#[path = "binary_semaev_chain_initial.rs"]
pub mod initial;
use serde::{Deserialize, Serialize};

/// The caller supplies authenticated originals; the dataset binding is opaque
/// here and must be minted by its existing verified-original owner.
pub struct Original<'a> {
    pub binding: [u8; 32],
    pub a: &'a F2mElement,
    pub b: &'a F2mElement,
    pub irr: &'a IrreduciblePoly,
    pub twist: bool,
    pub m: usize,
    pub target: Option<&'a F2mElement>,
    pub pattern: u32,
    pub codes: &'a [BigUint],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quote {
    pub work: u64,
    pub bytes: u64,
}
/// Reserve against the ORIGINAL cumulative owner; no local reset or fallback.
/// This construction-only interface does not install a dataset adapter.
pub trait Admission {
    fn reserve(&mut self, quote: Quote) -> Result<(), Error>;
    fn check(&mut self) -> Result<(), Error>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidOriginal,
    Cap,
    Admission,
    Identity,
    Semantics,
    Formula,
}
#[derive(Clone, Copy)]
pub struct Limits {
    pub variables: u32,
    pub codes: usize,
    pub domain_clauses: usize,
    pub work: u64,
    pub bytes: u64,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Identity {
    binding: [u8; 32],
    n: u32,
    low_terms: Vec<u32>,
    a: Vec<u32>,
    b: Vec<u32>,
    twist: bool,
    m: usize,
    target: Option<Vec<u32>>,
    pattern: u32,
    codes: Vec<Vec<u32>>,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Constraint {
    Clause { literals: Vec<Lit> },
    Xor { wires: Vec<u32>, rhs: bool },
}
/// Deserializing is not verification. Fields are private; use verify_original
/// before treating any imported map as an authenticated construction.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    schema: String,
    identity: Identity,
    variables: u32,
    blocks: Vec<Vec<u32>>,
    ands: Vec<(u32, u32, u32)>,
    xors: Vec<(Vec<u32>, bool)>,
    units: Vec<(u32, bool)>,
    impossible: bool,
    domain: Vec<Vec<Lit>>,
    formula: Vec<Constraint>,
}
#[derive(Debug)]
pub struct CheckedMap(Map);
#[derive(Debug)]
pub struct BuildFailure {
    pub reason: Error,
    /// Completed raw construction retained when final checking/admission fails.
    /// It has no verified status; the caller still owns borrowed originals.
    pub retained: Option<Map>,
}
impl From<Error> for BuildFailure {
    fn from(reason: Error) -> Self {
        Self {
            reason,
            retained: None,
        }
    }
}
impl CheckedMap {
    pub fn variables(&self) -> u32 {
        self.0.variables
    }
    pub fn map(&self) -> &Map {
        &self.0
    }
    /// Exact unsimplified insertion order, not Solver's simplified snapshot.
    pub fn formula(&self) -> &[Constraint] {
        &self.0.formula
    }
}

fn pay(
    owner: &mut impl Admission,
    total: &mut Quote,
    q: Quote,
    limits: Limits,
) -> Result<(), Error> {
    let next = Quote {
        work: total.work.checked_add(q.work).ok_or(Error::Cap)?,
        bytes: total.bytes.checked_add(q.bytes).ok_or(Error::Cap)?,
    };
    if next.work > limits.work || next.bytes > limits.bytes {
        return Err(Error::Cap);
    }
    owner.reserve(q)?;
    *total = next;
    owner.check()
}
fn admit(
    o: &Original<'_>,
    limits: Limits,
    owner: &mut impl Admission,
) -> Result<(Vec<Vec<usize>>, usize), Error> {
    let n = o.irr.degree as u64;
    let m = o.m as u64;
    let k = o.codes.len() as u64;
    // Only scalar/length gates precede the first paid validation pass.
    let variables = chain_s3_max_variables(o.irr.degree, o.m).ok_or(Error::InvalidOriginal)?;
    if variables > limits.variables as u64
        || variables > i32::MAX as u64
        || o.codes.len() > limits.codes
        || o.irr.low_terms.len() > n as usize
    {
        return Err(Error::Cap);
    }
    let mut total = Quote { work: 0, bytes: 0 };
    let validation_work = k
        .checked_add(n * n + 32)
        .and_then(|x| x.checked_mul(32))
        .ok_or(Error::Cap)?;
    pay(
        owner,
        &mut total,
        Quote {
            work: validation_work,
            bytes: 1024,
        },
        limits,
    )?;
    if o.a.m_value() != o.irr.degree
        || o.b.m_value() != o.irr.degree
        || o.b.is_zero()
        || o.target.is_some_and(|x| x.m_value() != o.irr.degree)
        || o.pattern >= (1 << (o.m - 2))
    {
        return Err(Error::InvalidOriginal);
    }
    let mut last = None;
    for (i, code) in o.codes.iter().enumerate() {
        if i % 64 == 0 {
            owner.check()?;
        }
        if code.bits() > n || last.is_some_and(|v| v >= code) {
            return Err(Error::InvalidOriginal);
        }
        last = Some(code);
    }
    let mut zero = false;
    for (i, &term) in o.irr.low_terms.iter().enumerate() {
        owner.check()?;
        if term >= o.irr.degree || o.irr.low_terms[..i].contains(&term) {
            return Err(Error::InvalidOriginal);
        }
        zero |= term == 0;
    }
    if !zero {
        return Err(Error::InvalidOriginal);
    }
    // Paid independent count/reduction pass. Bound reductions, numeric-domain
    // interval scans and their temporary BigUint owners before allocating.
    let domain_nodes = k
        .checked_mul(n)
        .and_then(|x| x.checked_mul(2))
        .and_then(|x| x.checked_add(1))
        .ok_or(Error::Cap)?;
    let count_work = (4 * n * n * (n + o.irr.low_terms.len() as u64 + 4))
        .checked_add(
            domain_nodes
                .checked_mul(k + 1)
                .and_then(|x| x.checked_mul(8))
                .ok_or(Error::Cap)?,
        )
        .ok_or(Error::Cap)?;
    pay(
        owner,
        &mut total,
        Quote {
            work: count_work,
            bytes: 32 * n * n + 256 * n + 4096,
        },
        limits,
    )?;
    let mut reductions = Vec::with_capacity((2 * n - 1) as usize);
    for degree in 0..2 * n - 1 {
        owner.check()?;
        let mut bits = vec![false; (2 * n) as usize];
        bits[degree as usize] = true;
        for high in (n..2 * n).rev() {
            if high % 32 == 0 {
                owner.check()?;
            }
            if bits[high as usize] {
                bits[high as usize] = false;
                for &term in &o.irr.low_terms {
                    bits[(high - n + term as u64) as usize] ^= true;
                }
            }
        }
        reductions.push((0..n as usize).filter(|&i| bits[i]).collect::<Vec<_>>());
    }
    fn domain_count(
        codes: &[BigUint],
        bit: usize,
        base: BigUint,
        depth: u64,
        owner: &mut impl Admission,
    ) -> Result<(u64, u64), Error> {
        owner.check()?;
        let upper = &base + (BigUint::from(1u8) << bit);
        let mut present = false;
        for (i, x) in codes.iter().enumerate() {
            if i % 64 == 0 {
                owner.check()?;
            }
            present |= x >= &base && x < &upper;
        }
        if !present {
            return Ok((1, depth));
        }
        if bit == 0 {
            return Ok((0, 0));
        }
        let l = domain_count(codes, bit - 1, base.clone(), depth + 1, owner)?;
        let r = domain_count(
            codes,
            bit - 1,
            base + (BigUint::from(1u8) << (bit - 1)),
            depth + 1,
            owner,
        )?;
        Ok((
            l.0.checked_add(r.0).ok_or(Error::Cap)?,
            l.1.checked_add(r.1).ok_or(Error::Cap)?,
        ))
    }
    let (domain_count, domain_slots) =
        domain_count(o.codes, n as usize, BigUint::from(0u8), 0, owner)?;
    let domains = domain_count.checked_mul(m).ok_or(Error::Cap)?;
    let domain_slots = domain_slots.checked_mul(m).ok_or(Error::Cap)?;
    if domains > limits.domain_clauses as u64 {
        return Err(Error::Cap);
    }
    let (mut s3, mut equal, mut units) = (0u64, 0u64, n);
    for step in 1..o.m {
        owner.check()?;
        let pi = step > 1 && o.pattern & (1 << (step - 2)) != 0;
        let oi = if step == o.m - 1 {
            o.target.is_none()
        } else {
            o.pattern & (1 << (step - 1)) != 0
        };
        if step < o.m - 1 && oi {
            units += n;
        }
        if !pi && !oi {
            s3 += 1;
        } else if pi != oi {
            equal += 1;
        }
    }
    let mut product_incidence = 0u64;
    for i in 0..n as usize {
        owner.check()?;
        for j in 0..n as usize {
            product_incidence += reductions[i + j].len() as u64;
        }
    }
    let square_incidence = (0..n as usize)
        .map(|i| reductions[2 * i].len() as u64)
        .sum::<u64>();
    let ands = 3 * n * n * s3;
    let xors = 5 * n * s3 + n * equal;
    let xor_slots = s3 * (7 * n + 3 * product_incidence + 2 * square_incidence) + 2 * n * equal;
    let constraints = (3 * ands + xors + units + 1)
        .checked_add(domains)
        .ok_or(Error::Cap)?;
    let formula_slots = (7 * ands + xor_slots + units)
        .checked_add(domain_slots)
        .ok_or(Error::Cap)?;
    // Worst live capacity: producer gates and formula, independent checker
    // scratch and formula comparison copy. Headers/capacity growth are counted
    // separately from literal slots; no blanket n^3 byte multiplier.
    let bytes = ands
        .checked_mul(24)
        .and_then(|v| v.checked_add(64 * xors + 16 * units))
        .and_then(|v| v.checked_add(domains.checked_mul(48)?))
        .and_then(|v| v.checked_add(xor_slots.checked_add(domain_slots)?.checked_mul(8)?))
        .and_then(|v| v.checked_add(constraints.checked_mul(128)?))
        .and_then(|v| v.checked_add(formula_slots.checked_mul(16)?))
        .and_then(|v| v.checked_add(128 * n * m + 64 * n * n + 8192))
        .and_then(|v| v.checked_add(k.checked_mul(128)?))
        .ok_or(Error::Cap)?;
    let work = formula_slots
        .checked_add(xor_slots)
        .and_then(|v| v.checked_add(ands))
        .and_then(|v| v.checked_add(constraints))
        .and_then(|v| v.checked_mul(32))
        .and_then(|v| v.checked_add(count_work))
        .and_then(|v| {
            v.checked_add(
                domain_nodes
                    .checked_mul(k + 1)?
                    .checked_mul(8)?
                    .checked_mul(m)?,
            )
        })
        .ok_or(Error::Cap)?;
    pay(owner, &mut total, Quote { work, bytes }, limits)?;
    Ok((reductions, domains as usize))
}
fn identity(o: &Original<'_>, owner: &mut impl Admission) -> Result<Identity, Error> {
    let mut codes = Vec::new();
    for (i, code) in o.codes.iter().enumerate() {
        if i % 64 == 0 {
            owner.check()?;
        }
        codes.push(code.to_u32_digits());
    }
    Ok(Identity {
        binding: o.binding,
        n: o.irr.degree,
        low_terms: o.irr.low_terms.clone(),
        a: o.a.to_biguint().to_u32_digits(),
        b: o.b.to_biguint().to_u32_digits(),
        twist: o.twist,
        m: o.m,
        target: o.target.map(|x| x.to_biguint().to_u32_digits()),
        pattern: o.pattern,
        codes,
    })
}
#[cfg(test)]
fn formula(map: &Map) -> Vec<Constraint> {
    formula_checked(map, &mut || Ok(())).unwrap()
}
fn formula_checked(
    map: &Map,
    check: &mut impl FnMut() -> Result<(), Error>,
) -> Result<Vec<Constraint>, Error> {
    let mut out = Vec::new();
    for &(gate, left, right) in &map.ands {
        check()?;
        let (g, l, r) = (gate as Lit, left as Lit, right as Lit);
        for literals in [vec![-g, l], vec![-g, r], vec![g, -l, -r]] {
            out.push(Constraint::Clause { literals });
        }
    }
    for (wires, rhs) in &map.xors {
        check()?;
        out.push(Constraint::Xor {
            wires: wires.clone(),
            rhs: *rhs,
        });
    }
    for &(wire, value) in &map.units {
        check()?;
        out.push(Constraint::Clause {
            literals: vec![if value { wire as Lit } else { -(wire as Lit) }],
        });
    }
    if map.impossible {
        out.push(Constraint::Clause { literals: vec![] });
    }
    for literals in &map.domain {
        check()?;
        out.push(Constraint::Clause {
            literals: literals.clone(),
        });
    }
    Ok(out)
}

pub fn build_original(
    o: &Original<'_>,
    limits: Limits,
    admission: &mut impl Admission,
) -> Result<CheckedMap, BuildFailure> {
    let (reductions, required) = admit(o, limits, admission)?;
    if required > limits.domain_clauses {
        return Err(Error::Cap.into());
    }
    let mut refusal = None;
    let (c, blocks, _) = prepare_circuit_checked(
        o.m,
        o.irr,
        o.b,
        o.target,
        o.pattern,
        limits.variables,
        &mut || {
            admission.check().map_err(|e| {
                refusal = Some(e);
                ChainBuildError::Unsupported
            })
        },
    )
    .map_err(|_| refusal.unwrap_or(Error::InvalidOriginal))?;
    admission.check()?;
    let mut domain = Vec::new();
    for block in &blocks[..o.m] {
        let mut refusal = None;
        visit_coordinate_domain_checked(
            &mut |clause| domain.push(clause),
            &mut || {
                admission.check().map_err(|e| {
                    refusal = Some(e);
                    ChainBuildError::Unsupported
                })
            },
            block,
            o.codes,
            o.irr.degree as usize,
        )
        .map_err(|_| refusal.unwrap_or(Error::Semantics))?;
    }
    if domain.len() != required {
        return Err(Error::Semantics.into());
    }
    let mut map = Map {
        schema: "binary-chain-original-gates/v1".into(),
        identity: identity(o, admission)?,
        variables: c.next - 1,
        blocks,
        ands: c.ands,
        xors: c.xors,
        units: c.units,
        impossible: c.impossible,
        domain,
        formula: Vec::new(),
    };
    map.formula = match formula_checked(&map, &mut || admission.check()) {
        Ok(rows) => rows,
        Err(reason) => {
            return Err(BuildFailure {
                reason,
                retained: Some(map),
            })
        }
    };
    if let Err(reason) =
        check_original(o, &map, &reductions, admission).and_then(|()| admission.check())
    {
        return Err(BuildFailure {
            reason,
            retained: Some(map),
        });
    }
    Ok(CheckedMap(map))
}

/// Imported maps require caller-side bounded decoding before this call.
/// Reserve the same bounded independent-check allowance; rejects oversized
/// vectors before any checker allocation. No Solver is run.
pub fn verify_original(
    o: &Original<'_>,
    map: &Map,
    limits: Limits,
    admission: &mut impl Admission,
) -> Result<(), Error> {
    let (reductions, _) = admit(o, limits, admission)?;
    let n = o.irr.degree as usize;
    let m = o.m;
    if map.blocks.len() != 2 * m - 1
        || map.ands.len() > 3 * n * n * (m - 1)
        || map.xors.len() > 5 * n * (m - 1)
        || map.units.len() > n * (m - 1)
        || map.domain.len() > limits.domain_clauses
        || map.domain.len() > m * (1 + o.codes.len() * n)
        || map.formula.len()
            > 3 * map.ands.len() + map.xors.len() + map.units.len() + map.domain.len() + 1
    {
        return Err(Error::Cap);
    }
    for (i, b) in map.blocks.iter().enumerate() {
        if i % 32 == 0 {
            admission.check()?;
        }
        if b.len() != n {
            return Err(Error::Cap);
        }
    }
    for (i, r) in map.domain.iter().enumerate() {
        if i % 32 == 0 {
            admission.check()?;
        }
        if r.len() > n {
            return Err(Error::Cap);
        }
    }
    for (i, (r, _)) in map.xors.iter().enumerate() {
        if i % 32 == 0 {
            admission.check()?;
        }
        if r.len() > n * n + 2 * n + 1 {
            return Err(Error::Cap);
        }
    }
    for (i, c) in map.formula.iter().enumerate() {
        if i % 32 == 0 {
            admission.check()?;
        }
        if match c {
            Constraint::Clause { literals } => literals.len() > n.max(3),
            Constraint::Xor { wires, .. } => wires.len() > n * n + 2 * n + 1,
        } {
            return Err(Error::Cap);
        }
    }
    check_original(o, map, &reductions, admission)?;
    admission.check()
}

// Independent checker: direct polynomial reduction by XOR long division,
// separate sequential wire/row cursors; never invokes Circuit, Solver, the
// builder's reduction table, or its finite-domain clause generator.
struct Check<'a, A: Admission> {
    map: &'a Map,
    n: usize,
    next: u32,
    and: usize,
    xor: usize,
    unit: usize,
    reductions: &'a [Vec<usize>],
    owner: &'a mut A,
}
impl<A: Admission> Check<'_, A> {
    fn block(&mut self) -> Vec<u32> {
        let start = self.next;
        self.next += self.n as u32;
        (start..self.next).collect()
    }
    fn row(&mut self, row: Vec<u32>, rhs: bool) -> Result<(), Error> {
        self.owner.check()?;
        if self.map.xors.get(self.xor) != Some(&(row, rhs)) {
            return Err(Error::Semantics);
        }
        self.xor += 1;
        Ok(())
    }
    fn unit(&mut self, wire: u32, value: bool) -> Result<(), Error> {
        self.owner.check()?;
        if self.map.units.get(self.unit) != Some(&(wire, value)) {
            return Err(Error::Semantics);
        }
        self.unit += 1;
        Ok(())
    }
    fn product(&mut self, a: &[u32], b: &[u32]) -> Result<Vec<u32>, Error> {
        let mut rows = vec![Vec::new(); self.n];
        for (i, &left) in a.iter().enumerate() {
            self.owner.check()?;
            for (j, &right) in b.iter().enumerate() {
                let g = self.next;
                self.next += 1;
                if self.map.ands.get(self.and) != Some(&(g, left, right)) {
                    return Err(Error::Semantics);
                }
                self.and += 1;
                for &out in &self.reductions[i + j] {
                    rows[out].push(g);
                }
            }
        }
        let out = self.block();
        for (i, mut row) in rows.into_iter().enumerate() {
            row.push(out[i]);
            self.row(row, false)?;
        }
        Ok(out)
    }
    fn s3(&mut self, x: &[u32], y: &[u32], z: &[u32], b: &BigUint) -> Result<(), Error> {
        let sum = self.block();
        for i in 0..self.n {
            self.row(vec![sum[i], x[i], y[i]], false)?;
        }
        let xy = self.product(x, y)?;
        let sz = self.product(&sum, z)?;
        let xyz = self.product(&xy, z)?;
        for out in 0..self.n {
            let mut row = vec![xyz[out]];
            for i in 0..self.n {
                if self.reductions[2 * i].contains(&out) {
                    row.extend([xy[i], sz[i]]);
                }
            }
            self.row(row, b.bit(out as u64))?;
        }
        Ok(())
    }
}
fn check_original(
    o: &Original<'_>,
    map: &Map,
    reductions: &[Vec<usize>],
    owner: &mut impl Admission,
) -> Result<(), Error> {
    let expected_identity = identity(o, owner)?;
    let a = &map.identity;
    let b = &expected_identity;
    if map.schema != "binary-chain-original-gates/v1"
        || a.binding != b.binding
        || a.n != b.n
        || a.low_terms != b.low_terms
        || a.a != b.a
        || a.b != b.b
        || a.twist != b.twist
        || a.m != b.m
        || a.target != b.target
        || a.pattern != b.pattern
        || a.codes.len() != b.codes.len()
    {
        return Err(Error::Identity);
    }
    for (i, (x, y)) in a.codes.iter().zip(&b.codes).enumerate() {
        if i % 64 == 0 {
            owner.check()?;
        }
        if x != y {
            return Err(Error::Identity);
        }
    }
    let mut c = Check {
        map,
        n: o.irr.degree as usize,
        next: 1,
        and: 0,
        xor: 0,
        unit: 0,
        reductions,
        owner,
    };
    let blocks: Vec<_> = (0..2 * o.m - 1).map(|_| c.block()).collect();
    if blocks != map.blocks {
        return Err(Error::Semantics);
    }
    let target = &blocks[2 * o.m - 2];
    let target_bits = o.target.map(F2mElement::to_biguint);
    for (i, &w) in target.iter().enumerate() {
        c.unit(w, target_bits.as_ref().is_some_and(|x| x.bit(i as u64)))?;
    }
    let mut impossible = false;
    for step in 1..o.m {
        let previous = if step == 1 {
            &blocks[0]
        } else {
            &blocks[o.m + step - 2]
        };
        let output = if step == o.m - 1 {
            target
        } else {
            &blocks[o.m + step - 1]
        };
        let pi = step > 1 && o.pattern & (1 << (step - 2)) != 0;
        let oi = if step == o.m - 1 {
            o.target.is_none()
        } else {
            o.pattern & (1 << (step - 1)) != 0
        };
        if step < o.m - 1 && oi {
            for &w in output {
                c.unit(w, false)?;
            }
        }
        if pi && oi {
            impossible = true;
        } else if pi || oi {
            let (left, right) = if pi {
                (&blocks[step], output)
            } else {
                (previous, &blocks[step])
            };
            for (&a, &b) in left.iter().zip(right) {
                c.row(vec![a, b], false)?;
            }
        } else {
            c.s3(previous, &blocks[step], output, &o.b.to_biguint())?;
        }
    }
    if c.next - 1 != map.variables
        || c.and != map.ands.len()
        || c.xor != map.xors.len()
        || c.unit != map.units.len()
        || impossible != map.impossible
    {
        return Err(Error::Semantics);
    }
    // Independently walk missing binary prefixes, filtering actual codes by
    // numeric interval instead of using producer partition_point recursion.
    fn domain(
        map: &Map,
        block: &[u32],
        codes: &[BigUint],
        bit: usize,
        base: BigUint,
        path: &mut Vec<Lit>,
        at: &mut usize,
        owner: &mut impl Admission,
    ) -> Result<(), Error> {
        owner.check()?;
        let upper = &base + (BigUint::from(1u8) << bit);
        let mut present = false;
        for (i, x) in codes.iter().enumerate() {
            if i % 64 == 0 {
                owner.check()?;
            }
            present |= x >= &base && x < &upper;
        }
        if !present {
            if map.domain.get(*at) != Some(path) {
                return Err(Error::Semantics);
            }
            *at += 1;
            return Ok(());
        }
        if bit == 0 {
            return Ok(());
        }
        path.push(block[bit - 1] as Lit);
        domain(map, block, codes, bit - 1, base.clone(), path, at, owner)?;
        *path.last_mut().unwrap() = -(block[bit - 1] as Lit);
        domain(
            map,
            block,
            codes,
            bit - 1,
            base + (BigUint::from(1u8) << (bit - 1)),
            path,
            at,
            owner,
        )?;
        path.pop();
        Ok(())
    }
    let mut at = 0;
    for block in &blocks[..o.m] {
        domain(
            map,
            block,
            o.codes,
            c.n,
            BigUint::from(0u8),
            &mut Vec::new(),
            &mut at,
            c.owner,
        )?;
    }
    if at != map.domain.len() {
        return Err(Error::Semantics);
    }
    let expected = formula_checked(map, &mut || c.owner.check())?;
    if expected.len() != map.formula.len() {
        return Err(Error::Formula);
    }
    for (i, (a, b)) in expected.iter().zip(&map.formula).enumerate() {
        if i % 32 == 0 {
            c.owner.check()?;
        }
        if a != b {
            return Err(Error::Formula);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Default)]
    struct Budget {
        reserved: Vec<Quote>,
        fail_reserve: bool,
        fail_check: bool,
    }
    impl Admission for Budget {
        fn reserve(&mut self, q: Quote) -> Result<(), Error> {
            if self.fail_reserve {
                return Err(Error::Admission);
            }
            self.reserved.push(q);
            Ok(())
        }
        fn check(&mut self) -> Result<(), Error> {
            if self.fail_check {
                Err(Error::Admission)
            } else {
                Ok(())
            }
        }
    }
    fn limits() -> Limits {
        Limits {
            variables: 100_000,
            codes: 32,
            domain_clauses: 100_000,
            work: u64::MAX,
            bytes: u64::MAX,
        }
    }
    fn copy(m: &Map) -> Map {
        serde_json::from_slice(&serde_json::to_vec(m).unwrap()).unwrap()
    }
    #[test]
    fn actual_construction_matches_legacy_formula_and_rejects_resealed_mutations() {
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let b = F2mElement::from_bit_positions(&[1], 3);
        let x = F2mElement::one(3);
        let codes = vec![BigUint::from(0u8), BigUint::from(1u8), BigUint::from(3u8)];
        for target in [Some(&x), None] {
            for pattern in 0..4 {
                let o = Original {
                    binding: [7; 32],
                    a: &a,
                    b: &b,
                    irr: &irr,
                    twist: true,
                    m: 4,
                    target,
                    pattern,
                    codes: &codes,
                };
                let mut budget = Budget::default();
                let checked = build_original(&o, limits(), &mut budget).unwrap();
                assert_eq!(budget.reserved.len(), 3);
                verify_original(&o, checked.map(), limits(), &mut Budget::default()).unwrap();
                let mut native =
                    ChainedS3Encoding::build(4, &irr, &b, target, pattern, 100_000).unwrap();
                native.constrain_summands(&codes, 100_000).unwrap();
                let mut from_map = Solver::new(checked.map().variables);
                for c in checked.formula() {
                    match c {
                        Constraint::Clause { literals } => {
                            from_map.add_clause(literals.clone());
                        }
                        Constraint::Xor { wires, rhs } => {
                            from_map.add_xor(wires, *rhs);
                        }
                    }
                }
                // Equality is a legacy compatibility control, not authentication
                // of an arbitrary imported native snapshot.
                assert_eq!(
                    crate::cryptanalysis::sat::to_dimacs_xor(&native.solver),
                    crate::cryptanalysis::sat::to_dimacs_xor(&from_map)
                );
                for change in 0..5 {
                    let mut bad = copy(checked.map());
                    match change {
                        0 => bad.identity.binding[0] ^= 1,
                        1 => bad.domain.pop().map(|_| ()).unwrap(),
                        2 => bad.xors[0].1 ^= true,
                        3 => bad.units[0].1 ^= true,
                        4 => bad.blocks[0][0] += 1,
                        _ => unreachable!(),
                    }
                    bad.formula = formula(&bad); // coherent resealing cannot bless wrong semantics.
                    assert!(verify_original(&o, &bad, limits(), &mut Budget::default()).is_err());
                }
                let mut bad = copy(checked.map());
                bad.formula.pop();
                assert_eq!(
                    verify_original(&o, &bad, limits(), &mut Budget::default()),
                    Err(Error::Formula)
                );
            }
        }
    }
    #[test]
    fn refusal_is_preconstruction_and_does_not_renew_owner() {
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let codes = vec![BigUint::from(1u8)];
        let o = Original {
            binding: [0; 32],
            a: &a,
            b: &a,
            irr: &irr,
            twist: false,
            m: 15,
            target: None,
            pattern: 1 << 12,
            codes: &codes,
        };
        let mut budget = Budget::default();
        let mut cap = limits();
        cap.variables = 1;
        assert!(matches!(
            build_original(&o, cap, &mut budget),
            Err(BuildFailure {
                reason: Error::Cap,
                ..
            })
        ));
        assert!(budget.reserved.is_empty());
        cap = limits();
        cap.bytes = 1;
        assert!(matches!(
            build_original(&o, cap, &mut budget),
            Err(BuildFailure {
                reason: Error::Cap,
                ..
            })
        ));
        assert!(budget.reserved.is_empty());
        budget.fail_reserve = true;
        assert!(matches!(
            build_original(&o, limits(), &mut budget),
            Err(BuildFailure {
                reason: Error::Admission,
                ..
            })
        ));
        assert!(budget.reserved.is_empty());
        budget.fail_reserve = false;
        budget.fail_check = true;
        assert!(matches!(
            build_original(&o, limits(), &mut budget),
            Err(BuildFailure {
                reason: Error::Admission,
                ..
            })
        ));
        assert_eq!(budget.reserved.len(), 1);
        budget.fail_check = false;
        let good = build_original(&o, limits(), &mut budget).unwrap();
        assert_eq!(budget.reserved.len(), 4); // prior admitted refusal still charged.
        verify_original(&o, good.map(), limits(), &mut budget).unwrap();
        assert_eq!(budget.reserved.len(), 7);
    }
    #[test]
    fn tiny_field_formula_has_exact_scalar_s3_meaning() {
        let irr = IrreduciblePoly {
            degree: 2,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(2);
        let b = F2mElement::from_bit_positions(&[1], 2);
        let codes: Vec<_> = (0u8..4).map(BigUint::from).collect();
        for z in 0u8..4 {
            let target = F2mElement::from_biguint(&BigUint::from(z), 2);
            let original = Original {
                binding: [9; 32],
                a: &a,
                b: &b,
                irr: &irr,
                twist: false,
                m: 2,
                target: Some(&target),
                pattern: 0,
                codes: &codes,
            };
            let checked = build_original(&original, limits(), &mut Budget::default()).unwrap();
            for x in 0u8..4 {
                for y in 0u8..4 {
                    let mut solver = Solver::new(checked.variables());
                    for c in checked.formula() {
                        match c {
                            Constraint::Clause { literals } => {
                                solver.add_clause(literals.clone());
                            }
                            Constraint::Xor { wires, rhs } => {
                                solver.add_xor(wires, *rhs);
                            }
                        }
                    }
                    for (block, value) in [(0, x), (1, y)] {
                        for bit in 0..2 {
                            let wire = checked.map().blocks[block][bit] as Lit;
                            solver.add_clause(vec![if value & (1 << bit) != 0 {
                                wire
                            } else {
                                -wire
                            }]);
                        }
                    }
                    let expected = binary_semaev_s3(
                        &F2mElement::from_biguint(&BigUint::from(x), 2),
                        &F2mElement::from_biguint(&BigUint::from(y), 2),
                        &target,
                        &b,
                        &irr,
                    )
                    .is_zero();
                    assert_eq!(
                        solver.solve() == crate::cryptanalysis::sat::SolveResult::Sat,
                        expected
                    );
                }
            }
        }
    }
    #[test]
    fn final_refusal_retains_complete_untrusted_map() {
        struct Late {
            checks: usize,
            charges: usize,
            stop_at: usize,
        }
        impl Admission for Late {
            fn reserve(&mut self, _: Quote) -> Result<(), Error> {
                self.charges += 1;
                Ok(())
            }
            fn check(&mut self) -> Result<(), Error> {
                self.checks += 1;
                if self.checks == self.stop_at {
                    Err(Error::Admission)
                } else {
                    Ok(())
                }
            }
        }
        let irr = IrreduciblePoly {
            degree: 3,
            low_terms: vec![0, 1],
        };
        let a = F2mElement::one(3);
        let codes = vec![BigUint::from(1u8)];
        let o = Original {
            binding: [0; 32],
            a: &a,
            b: &a,
            irr: &irr,
            twist: false,
            m: 3,
            target: Some(&a),
            pattern: 0,
            codes: &codes,
        };
        let mut count = Late {
            checks: 0,
            charges: 0,
            stop_at: usize::MAX,
        };
        build_original(&o, limits(), &mut count).unwrap();
        let mut budget = Late {
            checks: 0,
            charges: 0,
            stop_at: count.checks,
        };
        let failure = build_original(&o, limits(), &mut budget).unwrap_err();
        assert_eq!(failure.reason, Error::Admission);
        assert_eq!(budget.charges, 3);
        let retained = failure.retained.unwrap();
        verify_original(&o, &retained, limits(), &mut Budget::default()).unwrap();
    }
    #[test]
    fn validation_refusals_pay_original_owner_and_actual_incidence_quote() {
        let irr = IrreduciblePoly {
            degree: 23,
            low_terms: vec![0, 5],
        };
        let a = F2mElement::one(23);
        let codes = vec![BigUint::from(3086644u32), BigUint::from(7502454u32)];
        let original = Original {
            binding: [0; 32],
            a: &a,
            b: &a,
            irr: &irr,
            twist: false,
            m: 3,
            target: Some(&a),
            pattern: 0,
            codes: &codes,
        };
        let mut owner = Budget::default();
        let (_, domains) = admit(&original, limits(), &mut owner).unwrap();
        assert_eq!(domains, 132);
        assert_eq!(owner.reserved.len(), 3);
        assert_eq!(
            owner.reserved.iter().map(|q| q.work).sum::<u64>(),
            1_668_992
        );
        assert_eq!(
            owner.reserved.iter().map(|q| q.bytes).sum::<u64>(),
            1_965_584
        );
        let reversed = vec![codes[1].clone(), codes[0].clone()];
        let invalid = Original {
            codes: &reversed,
            ..original
        };
        let mut refusal = Budget::default();
        assert!(matches!(
            build_original(&invalid, limits(), &mut refusal),
            Err(BuildFailure {
                reason: Error::InvalidOriginal,
                ..
            })
        ));
        assert_eq!(refusal.reserved.len(), 1); // paid before validating either code.
    }
}
