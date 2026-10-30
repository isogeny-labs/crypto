//! Bounded semantic SAT-prefix checker. This is neither native execution replay
//! nor an UNSAT proof checker. Encoding is bound by native regeneration; Boolean
//! installation and learned-clause entailment are checked independently here.
use super::super::curves;
use crate::binary_ecc::BinaryPoint;
use crate::cryptanalysis::binary_semaev_chain_sat::{
    ChainBuildError, ChainedS3Encoding, GateMap, GateMapError, GateMapLimits, GATE_MAP_SCHEMA,
};
use crate::cryptanalysis::sat::{SolverEvent, SolverEventConstraint, SolverEventReason};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, Write};

pub(super) struct PatternSource {
    pub gate_map: GateMap,
    pub formula: String,
}
#[derive(Clone, Debug, Serialize)]
pub(super) struct ReplayLimits {
    pub raw_events: u64,
    pub raw_bytes: u64,
    pub decisions: u64,
    pub derived_bytes: u64,
    pub operations: u64,
    pub retained_bytes: u64,
    pub variables: u32,
    pub source_bytes: u64,
}
impl Default for ReplayLimits {
    fn default() -> Self {
        Self {
            raw_events: 100_000,
            raw_bytes: 16 << 20,
            decisions: 4096,
            derived_bytes: 64 << 20,
            operations: 10_000_000,
            retained_bytes: 64 << 20,
            variables: 100_000,
            source_bytes: 64 << 20,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize)]
pub(super) struct Work {
    /// One charged visit to a literal/variable or one 64-bit parity-word operation.
    pub literal_visits: u64,
    pub parity_word_operations: u64,
    pub raw_events: u64,
    pub raw_bytes: u64,
    pub derived_bytes: u64,
    /// Logical payload accounting, not process RSS or allocator overhead.
    pub retained_logical_peak_bytes: u64,
    pub source_payload_logical_bytes: u64,
}
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum StopReason {
    Budget { resource: &'static str },
    Unsupported { transition: &'static str },
    ProofNotEstablished,
    Truncated,
    NativeUnknown,
}
#[derive(Debug)]
enum Issue {
    Hard(String),
    Stop(StopReason),
}
type Check<T> = Result<T, Issue>;
impl From<String> for Issue {
    fn from(x: String) -> Self {
        Self::Hard(x)
    }
}
impl From<&str> for Issue {
    fn from(x: &str) -> Self {
        Self::Hard(x.into())
    }
}
fn ensure(ok: bool, msg: &str) -> Check<()> {
    if ok {
        Ok(())
    } else {
        Err(msg.into())
    }
}
fn cap(ok: bool, resource: &'static str) -> Check<()> {
    if ok {
        Ok(())
    } else {
        Err(Issue::Stop(StopReason::Budget { resource }))
    }
}
fn unsupported<T>(s: &'static str) -> Check<T> {
    Err(Issue::Stop(StopReason::Unsupported { transition: s }))
}
struct Budget {
    limits: ReplayLimits,
    work: Work,
}
impl Budget {
    fn charge(&mut self, n: usize, parity: bool) -> Check<()> {
        let remaining = self
            .limits
            .operations
            .saturating_sub(self.work.literal_visits + self.work.parity_word_operations);
        let charged = (n as u64).min(remaining);
        if parity {
            self.work.parity_word_operations += charged
        } else {
            self.work.literal_visits += charged
        }
        if n as u64 > remaining {
            return Err(Issue::Stop(StopReason::Budget {
                resource: "literal_and_parity_operations",
            }));
        }
        Ok(())
    }
    fn retained(&mut self, n: u64) -> Check<()> {
        let n = n.saturating_add(self.work.source_payload_logical_bytes);
        self.work.retained_logical_peak_bytes = self
            .work
            .retained_logical_peak_bytes
            .max(n.min(self.limits.retained_bytes));
        if n > self.limits.retained_bytes {
            return Err(Issue::Stop(StopReason::Budget {
                resource: "retained_logical_bytes",
            }));
        }
        Ok(())
    }
}
#[derive(Serialize)]
pub(super) struct ReplaySummary {
    pub verified_decisions: u64,
    pub unknown_suffix_records: u64,
    pub patterns_admitted: u64,
    pub work: Work,
    pub limits: ReplayLimits,
    pub native_execution_trace_verified: bool,
    pub native_instruction_count_verified: bool,
    pub unsat_proof_checked: bool,
}
struct Counter {
    count: u64,
    cap: u64,
    hit: bool,
}
impl Write for Counter {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if b.len() as u64 > self.cap.saturating_sub(self.count) {
            self.hit = true;
            return Err(std::io::Error::other("size bound"));
        }
        self.count += b.len() as u64;
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn size<T: Serialize>(v: &T, cap: u64) -> Check<u64> {
    let mut w = Counter {
        count: 0,
        cap,
        hit: false,
    };
    let r = serde_json::to_writer(&mut w, v);
    if w.hit {
        return Err(Issue::Stop(StopReason::Budget {
            resource: "serialized_bytes",
        }));
    }
    r.map_err(|e| Issue::Hard(e.to_string()))?;
    Ok(w.count)
}
fn index(l: i32, n: u32) -> Check<usize> {
    ensure(
        l != 0 && l != i32::MIN && l.unsigned_abs() <= n,
        "invalid SAT literal",
    )?;
    Ok((l.unsigned_abs() - 1) as usize)
}
fn canonical(ls: &[i32], n: u32, b: &mut Budget) -> Check<Vec<i32>> {
    b.charge(ls.len(), false)?;
    for &l in ls {
        index(l, n)?;
    }
    let mut v = ls.to_vec();
    v.sort_unstable_by_key(|l| (l.unsigned_abs(), *l));
    v.dedup();
    Ok(v)
}
fn val(a: &[i8], l: i32) -> i8 {
    a[(l.unsigned_abs() - 1) as usize] * if l > 0 { 1 } else { -1 }
}
#[derive(Clone, Debug, PartialEq)]
struct Row {
    mask: Vec<u64>,
    rhs: bool,
}
impl Row {
    fn variables(&self) -> Vec<u32> {
        self.mask
            .iter()
            .enumerate()
            .flat_map(|(i, &w)| {
                (0..64).filter_map(move |j| ((w >> j) & 1 == 1).then_some((i * 64 + j + 1) as u32))
            })
            .collect()
    }
}
fn mask_check(mask: &[u64], n: u32) -> Check<()> {
    ensure(
        mask.len() == (n as usize).div_ceil(64),
        "invalid XOR mask width",
    )?;
    if !n.is_multiple_of(64) {
        ensure(
            mask.last().copied().unwrap_or(0) >> (n % 64) == 0,
            "XOR padding bit set",
        )?;
    }
    Ok(())
}
#[derive(Clone)]
struct Clause {
    id: u32,
    lits: Vec<i32>,
    original: bool,
    active: bool,
}
struct Installed {
    n: u32,
    clauses: Vec<Clause>,
    rows: Vec<Row>,
    units: Vec<(i32, u32)>,
    assignment: Vec<i8>,
    unsat: bool,
    next: u32,
    logical_bytes: u64,
}
impl Installed {
    fn new(n: u32) -> Self {
        Self {
            n,
            clauses: vec![],
            rows: vec![],
            units: vec![],
            assignment: vec![0; n as usize],
            unsat: false,
            next: 0,
            logical_bytes: n as u64 * 64,
        }
    }
    fn clause(&mut self, ls: &[i32], b: &mut Budget) -> Check<()> {
        let c = canonical(ls, self.n, b)?;
        for pair in c.windows(2) {
            b.charge(2, false)?;
            if pair[0] == -pair[1] {
                return Ok(());
            }
        }
        for &l in &c {
            b.charge(1, false)?;
            if val(&self.assignment, l) > 0 {
                return Ok(());
            }
        }
        let mut simplified = Vec::with_capacity(c.len());
        for l in c {
            b.charge(1, false)?;
            if val(&self.assignment, l) >= 0 {
                simplified.push(l)
            }
        }
        let c = simplified;
        if c.is_empty() {
            self.unsat = true;
            return Ok(());
        }
        if c.len() == 1 {
            let l = c[0];
            self.assignment[index(l, self.n)?] = if l > 0 { 1 } else { -1 };
            self.units.push((l, self.next));
        }
        let next = self
            .next
            .checked_add(1 + c.len() as u32)
            .ok_or("clause arena overflow")?;
        self.logical_bytes += 32 + c.len() as u64 * 4 + if c.len() == 1 { 8 } else { 0 };
        b.retained(self.logical_bytes)?;
        self.clauses.push(Clause {
            id: self.next,
            lits: c,
            original: true,
            active: true,
        });
        self.next = next;
        b.retained(self.logical())
    }
    fn logical(&self) -> u64 {
        self.logical_bytes
    }
}
fn domain_clauses(
    inst: &mut Installed,
    block: &[u32],
    codes: &[u64],
    bit: usize,
    prefix: &mut Vec<i32>,
    count: &mut usize,
    b: &mut Budget,
) -> Check<()> {
    b.charge(1, false)?;
    if codes.is_empty() {
        *count += 1;
        return inst.clause(prefix, b);
    }
    if bit == 0 {
        return Ok(());
    }
    let pos = bit - 1;
    b.charge(codes.len(), false)?;
    let split = codes.partition_point(|x| x & (1u64 << pos) == 0);
    prefix.push(block[pos] as i32);
    domain_clauses(inst, block, &codes[..split], pos, prefix, count, b)?;
    *prefix.last_mut().ok_or("missing prefix")? = -(block[pos] as i32);
    domain_clauses(inst, block, &codes[split..], pos, prefix, count, b)?;
    prefix.pop();
    Ok(())
}
fn decimal(s: &str, n: u32) -> Check<u64> {
    ensure(!s.is_empty() && s.len() <= 19, "invalid field code length")?;
    let x = s.parse::<u64>().map_err(|_| "invalid decimal field code")?;
    ensure(
        x.to_string() == s && x < (1u64 << n),
        "noncanonical field code",
    )?;
    Ok(x)
}
fn normalize(map: &GateMap, b: &mut Budget) -> Check<Installed> {
    b.retained(map.variables as u64 * 64)?;
    let mut out = Installed::new(map.variables);
    for &[o, l, r] in &map.and_gates {
        for v in [o, l, r] {
            ensure(v > 0 && v <= map.variables, "invalid AND wire")?;
        }
        out.clause(&[-(o as i32), l as i32], b)?;
        out.clause(&[-(o as i32), r as i32], b)?;
        out.clause(&[o as i32, -(l as i32), -(r as i32)], b)?;
    }
    for row in &map.xor_rows {
        b.charge((map.variables as usize).div_ceil(64), true)?;
        let mut mask = vec![0u64; (map.variables as usize).div_ceil(64)];
        for &v in &row.variables {
            b.charge(1, false)?;
            ensure(v > 0 && v <= map.variables, "invalid XOR wire")?;
            mask[(v as usize - 1) / 64] ^= 1 << ((v - 1) % 64);
        }
        if mask.iter().all(|&x| x == 0) {
            out.unsat |= row.rhs
        } else {
            out.logical_bytes += 16 + mask.len() as u64 * 8;
            b.retained(out.logical_bytes)?;
            out.rows.push(Row { mask, rhs: row.rhs });
        }
        b.retained(out.logical())?;
    }
    for &l in &map.unit_literals {
        out.clause(&[l], b)?;
    }
    if map.impossible {
        out.clause(&[], b)?;
    }
    let domain = map
        .finite_domain
        .as_ref()
        .ok_or("missing installed finite domain")?;
    let codes = domain
        .codes
        .iter()
        .map(|s| decimal(s, map.n))
        .collect::<Check<Vec<_>>>()?;
    ensure(
        codes.windows(2).all(|p| p[0] < p[1]),
        "domain codes not sorted and unique",
    )?;
    let mut count = 0;
    for block in map.coordinate_blocks.iter().take(map.m) {
        domain_clauses(
            &mut out,
            block,
            &codes,
            map.n as usize,
            &mut vec![],
            &mut count,
            b,
        )?;
    }
    ensure(
        count == domain.raw_clause_count,
        "domain raw clause count mismatch",
    )?;
    Ok(out)
}
fn check_dimacs(source: &str, f: &Installed, b: &mut Budget) -> Check<()> {
    let mut lines = source.lines();
    let mut h = lines
        .next()
        .ok_or("missing DIMACS header")?
        .split_whitespace();
    ensure(
        h.next() == Some("p") && h.next() == Some("cnf"),
        "invalid DIMACS header",
    )?;
    ensure(
        h.next().and_then(|v| v.parse::<u32>().ok()) == Some(f.n),
        "DIMACS variable mismatch",
    )?;
    let count = f.clauses.len() + f.rows.len() + usize::from(f.unsat);
    ensure(
        h.next().and_then(|v| v.parse::<usize>().ok()) == Some(count) && h.next().is_none(),
        "DIMACS row count/header mismatch",
    )?;
    for c in &f.clauses {
        let mut words = lines
            .next()
            .ok_or("missing DIMACS clause")?
            .split_whitespace();
        for &expected in &c.lits {
            b.charge(1, false)?;
            ensure(
                words.next().and_then(|v| v.parse::<i32>().ok()) == Some(expected),
                "DIMACS clause mismatch",
            )?;
        }
        ensure(
            words.next() == Some("0") && words.next().is_none(),
            "DIMACS clause terminator/length mismatch",
        )?;
    }
    for row in &f.rows {
        let mut words = lines.next().ok_or("missing DIMACS XOR")?.split_whitespace();
        ensure(words.next() == Some("x"), "missing XOR marker")?;
        b.charge(row.mask.len(), true)?;
        let mut first = true;
        for v in row.variables() {
            b.charge(1, false)?;
            let expected = if first && !row.rhs {
                -(v as i32)
            } else {
                v as i32
            };
            first = false;
            ensure(
                words.next().and_then(|v| v.parse::<i32>().ok()) == Some(expected),
                "DIMACS XOR mismatch",
            )?;
        }
        ensure(
            words.next() == Some("0") && words.next().is_none(),
            "DIMACS XOR terminator/length mismatch",
        )?;
    }
    if f.unsat {
        ensure(
            lines.next() == Some("0"),
            "missing justified DIMACS contradiction",
        )?;
    }
    ensure(lines.next().is_none(), "extra DIMACS constraints")
}
fn build_issue(e: ChainBuildError) -> Issue {
    match e {
        ChainBuildError::VariableCap { .. } => Issue::Stop(StopReason::Budget {
            resource: "regeneration_variables",
        }),
        ChainBuildError::DomainClauseCap { .. } => Issue::Stop(StopReason::Budget {
            resource: "regeneration_domain_clauses",
        }),
        ChainBuildError::Unsupported => Issue::Stop(StopReason::Unsupported {
            transition: "native_encoding_profile",
        }),
    }
}
/// Declared retained input payload: typed scalar/vector/string elements and
/// vector headers, excluding allocator capacities/metadata and native encoder
/// temporary allocations. Working state adds to this same logical budget.
fn source_logical(s: &PatternSource) -> u64 {
    let m = &s.gate_map;
    let nested = |v: &[Vec<usize>]| v.iter().map(|x| 24 + x.len() as u64 * 8).sum::<u64>();
    std::mem::size_of::<PatternSource>() as u64
        + s.formula.len() as u64
        + m.schema.len() as u64
        + m.b.len() as u64
        + m.target_x.as_ref().map_or(0, |x| x.len() as u64)
        + m.irreducible_low_terms.len() as u64 * 4
        + m.and_gates.len() as u64 * 12
        + m.coordinate_blocks
            .iter()
            .map(|v| 24 + v.len() as u64 * 4)
            .sum::<u64>()
        + m.xor_rows
            .iter()
            .map(|r| 32 + r.variables.len() as u64 * 4)
            .sum::<u64>()
        + m.unit_literals.len() as u64 * 4
        + nested(&m.reduction_outputs)
        + nested(&m.square_inputs)
        + m.finite_domain.as_ref().map_or(0, |d| {
            d.codes.iter().map(|s| 24 + s.len() as u64).sum::<u64>()
        })
}

fn admit(
    public: &Value,
    base: &Value,
    pattern: u32,
    src: &PatternSource,
    b: &mut Budget,
) -> Check<Installed> {
    let map = &src.gate_map;
    ensure(map.schema == GATE_MAP_SCHEMA, "unknown gate-map schema")?;
    ensure(
        (3..=61).contains(&map.n) && (2..=15).contains(&map.m),
        "invalid gate-map field/arity",
    )?;
    ensure(
        map.variables > 0 && map.variables <= i32::MAX as u32,
        "invalid gate-map variable count",
    )?;
    cap(map.variables <= b.limits.variables, "input_variables")?;
    ensure(
        map.identity_mask == pattern && pattern < (1u32 << (map.m - 2)),
        "gate-map pattern mismatch",
    )?;
    size(map, 16 << 20)?;
    cap(
        src.formula.len() as u64 <= b.limits.source_bytes,
        "source_formula_bytes",
    )?;
    ensure(
        map.coordinate_blocks.len() >= map.m,
        "missing coordinate blocks",
    )?;
    for block in &map.coordinate_blocks {
        ensure(block.len() == map.n as usize, "coordinate block width")?;
        for &v in block {
            ensure(v > 0 && v <= map.variables, "invalid coordinate wire")?;
        }
    }
    ensure(
        map.irreducible_low_terms.len() <= map.n as usize,
        "field polynomial size",
    )?;
    decimal(&map.b, map.n)?;
    if let Some(x) = &map.target_x {
        decimal(x, map.n)?;
    }
    let d = map.finite_domain.as_ref().ok_or("missing map domain")?;
    cap(
        d.codes.len() <= 4096 && d.raw_clause_count <= 1_000_000,
        "domain_inputs",
    )?;
    for x in &d.codes {
        decimal(x, map.n)?;
    }
    ensure(
        public["n"].as_u64() == Some(map.n as u64) && public["m"].as_u64() == Some(map.m as u64),
        "cell field/arity mismatch",
    )?;
    let curve = curves::binary_curve(public)?;
    let target = curves::binary_point(&public["target"], map.n)?;
    let target_x = match &target {
        BinaryPoint::Affine { x, .. } => Some(x),
        BinaryPoint::Infinity => None,
    };
    let points = base["points"]
        .as_array()
        .ok_or("missing factor-base points")?;
    cap(points.len() <= 4096, "factor_base_points")?;
    let mut codes = Vec::new();
    for p in points {
        match curves::binary_point(p, map.n)? {
            BinaryPoint::Affine { x, .. } => codes.push(x.to_biguint()),
            BinaryPoint::Infinity => {}
        }
    }
    codes.sort();
    codes.dedup();
    let mut enc = ChainedS3Encoding::build_with_gate_map(
        map.m,
        &curve.irreducible,
        &curve.b,
        target_x,
        pattern,
        b.limits.variables,
        GateMapLimits::default(),
    )
    .map_err(build_issue)?;
    enc.constrain_summands(&codes, 1_000_000)
        .map_err(build_issue)?;
    ensure(
        enc.gate_map().map_err(|e| match e {
            GateMapError::ByteCap { .. } => Issue::Stop(StopReason::Budget {
                resource: "regenerated_gate_map_bytes",
            }),
            other => Issue::Hard(format!("gate retention: {other:?}")),
        })? == map,
        "gate-map source substitution",
    )?;
    let f = normalize(map, b)?;
    check_dimacs(&src.formula, &f, b)?;
    Ok(f)
}

struct Pending {
    id: u32,
    lits: Vec<i32>,
    backjump: usize,
    confirmed: bool,
    assigned: bool,
}
struct Decision {
    literal: i32,
    level: usize,
    sequence: u64,
    activity: u64,
    priority: bool,
    inputs: Value,
    input_bytes: u64,
}
struct State {
    f: Installed,
    matrix: Vec<Row>,
    trail: Vec<i32>,
    levels: Vec<usize>,
    reasons: Vec<Option<SolverEventReason>>,
    boundaries: Vec<usize>,
    snapshot_pos: usize,
    begun: bool,
    ended: bool,
    decision: Option<Decision>,
    learned: Option<Pending>,
    implication: Option<(i32, Vec<i32>)>,
    xor_conflict: Option<Vec<i32>>,
    conflict: Option<Vec<i32>>,
    restart: u64,
    matrix_bytes: u64,
    reason_bytes: u64,
    restart_backjump: bool,
}
impl State {
    fn new(mut f: Installed) -> Self {
        let n = f.n as usize;
        f.assignment.fill(0);
        Self {
            f,
            matrix: vec![],
            trail: vec![],
            levels: vec![0; n],
            reasons: vec![None; n],
            boundaries: vec![],
            snapshot_pos: 0,
            begun: false,
            ended: false,
            decision: None,
            learned: None,
            implication: None,
            xor_conflict: None,
            conflict: None,
            restart: 0,
            matrix_bytes: 0,
            reason_bytes: 0,
            restart_backjump: false,
        }
    }
    fn logical(&self) -> u64 {
        self.f.logical()
            + self.matrix_bytes
            + self.reason_bytes
            + self.trail.len() as u64 * 4
            + self.boundaries.len() as u64 * 8
            + self
                .learned
                .as_ref()
                .map_or(0, |p| 32 + p.lits.len() as u64 * 4)
            + self.decision.as_ref().map_or(0, |d| 64 + d.input_bytes)
            + self
                .implication
                .as_ref()
                .map_or(0, |(_, v)| 24 + v.len() as u64 * 4)
            + self
                .xor_conflict
                .as_ref()
                .map_or(0, |v| 24 + v.len() as u64 * 4)
            + self
                .conflict
                .as_ref()
                .map_or(0, |v| 24 + v.len() as u64 * 4)
    }

    fn clause(&self, id: u32) -> Check<&[i32]> {
        if let Some(c) = self.f.clauses.iter().find(|c| c.id == id && c.active) {
            return Ok(&c.lits);
        }
        if let Some(p) = &self.learned {
            if p.id == id {
                return Ok(&p.lits);
            }
        }
        Err("unknown or deleted clause reference".into())
    }
    fn unit(&self, c: &[i32], l: i32, b: &mut Budget) -> Check<()> {
        b.charge(c.len(), false)?;
        ensure(
            c.contains(&l) && c.iter().all(|&x| x == l || val(&self.f.assignment, x) < 0),
            "reason is not unit on preceding trail",
        )
    }
    fn explanation(&self, row: usize, l: Option<i32>, b: &mut Budget) -> Check<Vec<i32>> {
        let r = self.matrix.get(row).ok_or("unknown parity row")?;
        b.charge(r.mask.len(), true)?;
        let vars = r.variables();
        b.charge(vars.len(), false)?;
        let mut parity = false;
        let mut out = Vec::new();
        if let Some(l) = l {
            index(l, self.f.n)?;
            out.push(l);
        }
        let mut found = l.is_none();
        for v in vars {
            let a = self.f.assignment[v as usize - 1];
            if l.is_some_and(|x| x.unsigned_abs() == v) {
                ensure(a == 0, "XOR implication already assigned")?;
                parity ^= l.is_some_and(|x| x > 0);
                found = true;
            } else {
                ensure(a != 0, "XOR explanation has unassigned support")?;
                parity ^= a > 0;
                out.push(if a > 0 { -(v as i32) } else { v as i32 });
            }
        }
        ensure(
            found
                && if l.is_some() {
                    parity == r.rhs
                } else {
                    parity != r.rhs
                },
            "false XOR implication/conflict",
        )?;
        Ok(out)
    }
    fn reason(&self, r: &SolverEventReason, l: i32, b: &mut Budget) -> Check<Vec<i32>> {
        match r {
            SolverEventReason::Clause { clause_id } => {
                let c = self.clause(*clause_id)?.to_vec();
                if l != 0 {
                    self.unit(&c, l, b)?;
                }
                Ok(c)
            }
            SolverEventReason::Xor { explanation } => {
                let expected = if l == 0 {
                    self.xor_conflict.as_ref()
                } else {
                    self.implication
                        .as_ref()
                        .filter(|(x, _)| *x == l)
                        .map(|(_, e)| e)
                };
                ensure(expected == Some(explanation), "unchecked XOR explanation")?;
                Ok(explanation.clone())
            }
            SolverEventReason::Decision => Err("decision is not a propagation reason".into()),
        }
    }
    fn features(&self, initial: &Value, map: &GateMap, b: &mut Budget) -> Check<Value> {
        // Conservative serialized-payload reservation before cloning public
        // data. This is an upper bound, not an allocator/RSS measurement.
        let map_bytes = size(map, 16 << 20)?;
        let initial_bytes = size(initial, b.limits.source_bytes)?;
        let clause_bytes = self
            .f
            .clauses
            .iter()
            .filter(|c| c.active)
            .map(|c| 16 + c.lits.len() as u64 * 12)
            .sum::<u64>();
        b.charge(
            self.f
                .rows
                .iter()
                .chain(&self.matrix)
                .map(|r| r.mask.len())
                .sum(),
            true,
        )?;
        let xor_bytes = self
            .f
            .rows
            .iter()
            .chain(&self.matrix)
            .map(|r| {
                64 + r
                    .mask
                    .iter()
                    .map(|w| w.count_ones() as u64 * 12)
                    .sum::<u64>()
            })
            .sum::<u64>();
        let support_visits = self
            .f
            .rows
            .iter()
            .chain(&self.matrix)
            .map(|r| {
                r.mask
                    .iter()
                    .map(|w| w.count_ones() as usize)
                    .sum::<usize>()
            })
            .sum();
        b.charge(support_visits, false)?;
        let reservation = 1024
            + map_bytes
            + initial_bytes
            + clause_bytes
            + xor_bytes
            + self.trail.len() as u64 * 64
            + self.boundaries.len() as u64 * 24;
        cap(
            reservation <= b.limits.derived_bytes.saturating_sub(b.work.derived_bytes),
            "derived_feature_reservation",
        )?;
        b.retained(self.logical() + reservation)?;
        b.charge(
            self.f
                .clauses
                .iter()
                .filter(|c| c.active)
                .map(|c| c.lits.len())
                .sum::<usize>()
                + self.trail.len(),
            false,
        )?;
        b.charge(
            self.f
                .rows
                .iter()
                .chain(&self.matrix)
                .map(|r| r.mask.len())
                .sum(),
            true,
        )?;
        let clauses = self
            .f
            .clauses
            .iter()
            .filter(|c| c.active)
            .map(|c| canonical(&c.lits, self.f.n, b))
            .collect::<Check<Vec<_>>>()?;
        let rows = |rows: &[Row]| {
            rows.iter()
                .map(|r| json!({"variables":r.variables(),"rhs":r.rhs}))
                .collect::<Vec<_>>()
        };
        let v = json!({"task":"binary-sat-decision/v1","initial":initial,"gate_map":map,
            "active_state":{"clauses":clauses,"xor_rows":rows(&self.f.rows),"xor_matrix":rows(&self.matrix),
            "trail":self.trail.iter().map(|l|json!({"literal":l,"level":self.levels[(l.unsigned_abs()-1) as usize]})).collect::<Vec<_>>(),"decision_boundaries":self.boundaries}});
        let n = size(
            &v,
            b.limits.derived_bytes.saturating_sub(b.work.derived_bytes),
        )?;
        b.retained(self.logical() + n)?;
        Ok(v)
    }
}
/// RUP plus single-free parity propagation from negated candidate only. Active
/// search decisions are deliberately absent. Deleted certified lemmas remain in
/// the proof archive. Matrix rows are exact combinations of admitted XORs.
fn rup(s: &State, candidate: &[i32], b: &mut Budget) -> Check<bool> {
    b.retained(s.logical() + s.f.n as u64)?;
    let mut a = vec![0i8; s.f.n as usize];
    for &l in candidate {
        b.charge(1, false)?;
        let i = index(l, s.f.n)?;
        let v = if l > 0 { -1 } else { 1 };
        if a[i] == -v {
            return Ok(true);
        }
        a[i] = v;
    }
    loop {
        let mut changed = false;
        for c in &s.f.clauses {
            let mut free = 0;
            let mut unit = 0;
            let mut sat = false;
            for &l in &c.lits {
                b.charge(1, false)?;
                match val(&a, l) {
                    1 => {
                        sat = true;
                        break;
                    }
                    0 => {
                        free += 1;
                        unit = l
                    }
                    _ => {}
                }
            }
            if sat {
                continue;
            }
            if free == 0 {
                return Ok(true);
            }
            if free == 1 {
                a[index(unit, s.f.n)?] = if unit > 0 { 1 } else { -1 };
                changed = true;
            }
        }
        for r in s.f.rows.iter().chain(&s.matrix) {
            b.charge(r.mask.len(), true)?;
            let mut parity = false;
            let mut free = 0;
            let mut last = 0;
            for v in r.variables() {
                b.charge(1, false)?;
                match a[v as usize - 1] {
                    0 => {
                        free += 1;
                        last = v as usize - 1
                    }
                    x => parity ^= x > 0,
                }
            }
            if free == 0 && parity != r.rhs {
                return Ok(true);
            }
            if free == 1 {
                a[last] = if parity ^ r.rhs { 1 } else { -1 };
                changed = true;
            }
        }
        if !changed {
            return Ok(false);
        }
    }
}
impl State {
    fn snapshot_event(&mut self, e: &SolverEvent, b: &mut Budget) -> Check<bool> {
        let nc = self.f.clauses.len();
        let nx = self.f.rows.len();
        if self.snapshot_pos < nc {
            let c = &self.f.clauses[self.snapshot_pos];
            let SolverEvent::Clause {
                clause_id,
                literals,
                existing,
                detached,
            } = e
            else {
                return Err("incomplete original clause snapshot".into());
            };
            ensure(
                *existing
                    && !*detached
                    && *clause_id == c.id
                    && literals.len() == c.lits.len()
                    && canonical(literals, self.f.n, b)? == c.lits,
                "original clause snapshot mismatch",
            )?;
        } else if self.snapshot_pos < nc + nx {
            let i = self.snapshot_pos - nc;
            let r = &self.f.rows[i];
            let SolverEvent::XorConstraint { row_id, mask, rhs } = e else {
                return Err("incomplete original XOR snapshot".into());
            };
            mask_check(mask, self.f.n)?;
            b.charge(mask.len(), true)?;
            ensure(
                *row_id == i && *mask == r.mask && *rhs == r.rhs,
                "original XOR snapshot mismatch",
            )?;
        } else if self.snapshot_pos < nc + nx + self.f.units.len() {
            if matches!(e, SolverEvent::XorMatrixRow { .. }) {
                return unsupported("prepropagated_snapshot");
            }
            let i = self.snapshot_pos - nc - nx;
            let (l, id) = self.f.units[i];
            let SolverEvent::Assignment {
                literal,
                level,
                trail_index,
                initial,
                reason,
            } = e
            else {
                return Err("incomplete initial assignment snapshot".into());
            };
            ensure(
                *literal == l
                    && *level == 0
                    && *trail_index == i
                    && *initial
                    && *reason == SolverEventReason::Clause { clause_id: id },
                "initial assignment mismatch",
            )?;
            self.unit(self.clause(id)?, l, b)?;
            let v = index(l, self.f.n)?;
            self.f.assignment[v] = if l > 0 { 1 } else { -1 };
            self.reasons[v] = Some(reason.clone());
            self.trail.push(l);
        } else {
            return Ok(false);
        }
        self.snapshot_pos += 1;
        Ok(true)
    }
    fn event(
        &mut self,
        e: SolverEvent,
        seq: u64,
        initial: &Value,
        map: &GateMap,
        b: &mut Budget,
    ) -> Check<Option<(Value, Value)>> {
        if !self.begun {
            if self.snapshot_event(&e, b)? {
                return Ok(None);
            }
            match e {
                SolverEvent::SolveBegin { conflicts, .. } => {
                    ensure(conflicts == 0, "nonfresh initial conflict count")?;
                    self.begun = true;
                    return Ok(None);
                }
                SolverEvent::XorMatrixRow { .. }
                | SolverEvent::Assignment { initial: true, .. } => {
                    return unsupported("prepropagated_snapshot")
                }
                _ => return Err("expected solve begin after complete snapshot".into()),
            }
        }
        if self.ended {
            return unsupported("additional_solve_or_postsolve_transition");
        }
        if self.decision.is_some() && !matches!(e, SolverEvent::Assignment { .. }) {
            return Err("decision missing immediate assignment".into());
        }
        if self.restart_backjump && !matches!(e, SolverEvent::Backjump { to_level: 0, .. }) {
            return Err("restart missing root backjump".into());
        }
        if self.learned.is_some()
            && !matches!(
                e,
                SolverEvent::Backjump { .. }
                    | SolverEvent::Clause { .. }
                    | SolverEvent::Assignment { .. }
            )
        {
            return Err("interrupted learned installation".into());
        }
        match e {
            SolverEvent::Decision {
                literal,
                level,
                trail_index,
                activity_bits,
                priority,
            } => {
                let v = index(literal, self.f.n)?;
                ensure(
                    self.learned.is_none()
                        && self.conflict.is_none()
                        && !self.f.unsat
                        && self.f.assignment[v] == 0
                        && level == self.boundaries.len() + 1
                        && trail_index == self.trail.len(),
                    "invalid decision boundary",
                )?;
                let inputs = self.features(initial, map, b)?;
                self.decision = Some(Decision {
                    literal,
                    level,
                    sequence: seq,
                    activity: activity_bits,
                    priority,
                    input_bytes: size(&inputs, b.limits.derived_bytes)?,
                    inputs,
                });
                self.boundaries.push(self.trail.len());
            }
            SolverEvent::Assignment {
                literal,
                level,
                trail_index,
                initial,
                reason,
            } => {
                let v = index(literal, self.f.n)?;
                ensure(
                    !initial
                        && level >= 0
                        && level as usize == self.boundaries.len()
                        && trail_index == self.trail.len()
                        && self.f.assignment[v] == 0,
                    "invalid assignment index/level/duplicate",
                )?;
                if let Some(d) = &self.decision {
                    ensure(
                        d.literal == literal
                            && d.level == level as usize
                            && reason == SolverEventReason::Decision,
                        "decision assignment mismatch",
                    )?;
                } else {
                    self.reason(&reason, literal, b)?;
                }
                self.f.assignment[v] = if literal > 0 { 1 } else { -1 };
                self.levels[v] = level as usize;
                if let SolverEventReason::Xor { explanation } = &reason {
                    self.reason_bytes += explanation.len() as u64 * 4;
                }
                self.reasons[v] = Some(reason);
                self.trail.push(literal);
                self.implication = None;
                if let Some(p) = &mut self.learned {
                    if p.lits[0] == literal {
                        ensure(
                            level as usize == p.backjump,
                            "learned asserting level mismatch",
                        )?;
                        p.assigned = true;
                    }
                }
                if self
                    .learned
                    .as_ref()
                    .is_some_and(|p| p.confirmed && p.assigned)
                {
                    self.learned = None;
                }
                b.retained(self.logical())?;
                if let Some(d) = self.decision.take() {
                    return Ok(Some((
                        d.inputs,
                        json!({"kind":"sat_decision","admission_status":"verified_decision",
                        "event_sequence":d.sequence,"assignment_event_sequence":seq,"chosen_literal":literal,
                        "level":level,"activity_bits":d.activity.to_string(),"priority":d.priority}),
                    )));
                }
            }
            SolverEvent::XorMatrixReset => {
                b.charge(self.f.rows.iter().map(|r| r.mask.len()).sum(), true)?;
                self.matrix_bytes = self
                    .f
                    .rows
                    .iter()
                    .map(|r| 16 + r.mask.len() as u64 * 8)
                    .sum();
                b.retained(self.logical())?;
                self.matrix = self.f.rows.clone();
                self.implication = None;
                self.xor_conflict = None;
            }
            SolverEvent::XorRowXor {
                destination,
                source,
            } => {
                ensure(
                    destination != source
                        && destination < self.matrix.len()
                        && source < self.matrix.len(),
                    "invalid parity row operation",
                )?;
                b.charge(self.matrix[source].mask.len(), true)?;
                for j in 0..self.matrix[source].mask.len() {
                    self.matrix[destination].mask[j] ^= self.matrix[source].mask[j];
                }
                self.matrix[destination].rhs ^= self.matrix[source].rhs;
                self.implication = None;
                self.xor_conflict = None;
            }
            SolverEvent::XorImplication {
                row_id,
                literal,
                explanation,
            } => {
                ensure(
                    self.explanation(row_id, Some(literal), b)? == explanation,
                    "forged XOR implication explanation",
                )?;
                self.implication = Some((literal, explanation));
            }
            SolverEvent::XorConflict {
                row_id,
                explanation,
            } => {
                ensure(
                    self.explanation(row_id, None, b)? == explanation,
                    "forged XOR conflict explanation",
                )?;
                self.xor_conflict = Some(explanation);
            }
            SolverEvent::Conflict {
                level,
                reason,
                literals,
            } => {
                ensure(level == self.boundaries.len(), "conflict level mismatch")?;
                let c = self.reason(&reason, 0, b)?;
                ensure(
                    canonical(&c, self.f.n, b)? == canonical(&literals, self.f.n, b)?,
                    "conflict reason differs",
                )?;
                b.charge(c.len(), false)?;
                ensure(
                    c.iter().all(|&l| val(&self.f.assignment, l) < 0),
                    "conflict not falsified",
                )?;
                self.conflict = Some(c);
            }
            SolverEvent::AnalysisReason {
                resolved_literal,
                reason,
                literals,
            } => {
                ensure(self.conflict.is_some(), "analysis outside conflict")?;
                let expected = if resolved_literal == 0 {
                    self.conflict.clone().ok_or("analysis without conflict")?
                } else {
                    let v = index(resolved_literal, self.f.n)?;
                    ensure(
                        val(&self.f.assignment, resolved_literal) > 0,
                        "analysis resolves unassigned or opposite literal",
                    )?;
                    ensure(
                        self.reasons[v].as_ref() == Some(&reason),
                        "analysis reason provenance mismatch",
                    )?;
                    match &reason {
                        SolverEventReason::Clause { clause_id } => {
                            self.clause(*clause_id)?.to_vec()
                        }
                        SolverEventReason::Xor { explanation } => explanation.clone(),
                        SolverEventReason::Decision => {
                            return Err("analysis resolves decision reason".into())
                        }
                    }
                };
                ensure(
                    canonical(&expected, self.f.n, b)? == canonical(&literals, self.f.n, b)?,
                    "analysis reason payload mismatch",
                )?;
                if resolved_literal == 0 {
                    let c = self.reason(&reason, 0, b)?;
                    ensure(
                        canonical(&c, self.f.n, b)? == canonical(&expected, self.f.n, b)?,
                        "analysis conflict reference mismatch",
                    )?;
                }
            }
            SolverEvent::Learned {
                clause_id,
                literals,
                backjump_level,
            } => {
                ensure(
                    self.learned.is_none()
                        && self.conflict.is_some()
                        && clause_id == self.f.next
                        && !literals.is_empty()
                        && backjump_level >= 0,
                    "invalid learned declaration",
                )?;
                let c = canonical(&literals, self.f.n, b)?;
                ensure(
                    c.len() == literals.len() && !c.windows(2).any(|w| w[0] == -w[1]),
                    "duplicate or tautological learned clause",
                )?;
                b.charge(c.len(), false)?;
                ensure(
                    c.iter().all(|&l| val(&self.f.assignment, l) < 0),
                    "learned clause not falsified before backjump",
                )?;
                let current = self.boundaries.len();
                ensure(
                    current > 0 && self.levels[index(literals[0], self.f.n)?] == current,
                    "learned asserting literal level",
                )?;
                let rest = literals[1..]
                    .iter()
                    .map(|l| self.levels[(l.unsigned_abs() - 1) as usize])
                    .collect::<Vec<_>>();
                ensure(
                    rest.iter().all(|&l| l < current)
                        && rest.iter().copied().max().unwrap_or(0) == backjump_level as usize,
                    "invalid learned backjump level",
                )?;
                if !rup(self, &literals, b)? {
                    return Err(Issue::Stop(StopReason::ProofNotEstablished));
                }
                self.learned = Some(Pending {
                    id: clause_id,
                    lits: literals,
                    backjump: backjump_level as usize,
                    confirmed: false,
                    assigned: false,
                });
            }
            SolverEvent::Clause {
                clause_id,
                literals,
                existing,
                detached,
            } => {
                if self.learned.is_none() {
                    canonical(&literals, self.f.n, b)?;
                    return unsupported("dynamic_clause");
                }
                let p = self
                    .learned
                    .as_ref()
                    .ok_or("missing pending learned clause")?;
                ensure(
                    !existing
                        && !detached
                        && !p.confirmed
                        && clause_id == p.id
                        && literals.len() == p.lits.len()
                        && canonical(&literals, self.f.n, b)? == canonical(&p.lits, self.f.n, b)?
                        && self.boundaries.len() == p.backjump,
                    "learned installation mismatch",
                )?;
                self.f.next = self
                    .f
                    .next
                    .checked_add(1 + p.lits.len() as u32)
                    .ok_or("clause arena overflow")?;
                self.f.logical_bytes += 32 + p.lits.len() as u64 * 4;
                b.retained(self.f.logical_bytes + self.matrix_bytes + self.reason_bytes)?;
                self.f.clauses.push(Clause {
                    id: clause_id,
                    lits: p.lits.clone(),
                    original: false,
                    active: true,
                });
                self.learned
                    .as_mut()
                    .ok_or("missing pending learned clause")?
                    .confirmed = true;
                if self.learned.as_ref().is_some_and(|p| p.assigned) {
                    self.learned = None;
                }
            }
            SolverEvent::Backjump {
                from_level,
                to_level,
                retained_trail,
                removed,
            } => {
                ensure(
                    from_level == self.boundaries.len()
                        && to_level < from_level
                        && retained_trail == self.boundaries[to_level]
                        && removed == self.trail[retained_trail..],
                    "invalid backjump suffix/boundary",
                )?;
                if let Some(p) = &self.learned {
                    ensure(
                        to_level == p.backjump,
                        "backjump differs from learned declaration",
                    )?;
                }
                b.charge(removed.len(), false)?;
                for &l in &removed {
                    let v = index(l, self.f.n)?;
                    self.f.assignment[v] = 0;
                    if let Some(SolverEventReason::Xor { explanation }) = &self.reasons[v] {
                        self.reason_bytes -= explanation.len() as u64 * 4;
                    }
                    self.reasons[v] = None;
                    self.levels[v] = 0;
                }
                self.restart_backjump = false;
                self.trail.truncate(retained_trail);
                self.boundaries.truncate(to_level);
                self.implication = None;
                self.xor_conflict = None;
                self.conflict = None;
            }
            SolverEvent::Restart { restart, .. } => {
                ensure(
                    self.learned.is_none() && restart == self.restart + 1,
                    "invalid restart sequence",
                )?;
                self.restart = restart;
                self.restart_backjump = !self.boundaries.is_empty();
            }
            SolverEvent::DeleteClause { clause_id } => {
                ensure(self.boundaries.is_empty(), "deletion outside root level")?;
                ensure(
                    !self
                        .reasons
                        .iter()
                        .any(|r| r.as_ref() == Some(&SolverEventReason::Clause { clause_id })),
                    "deletion invalidates active assignment reason",
                )?;
                let c = self
                    .f
                    .clauses
                    .iter_mut()
                    .find(|c| c.id == clause_id)
                    .ok_or("delete unknown clause")?;
                ensure(
                    c.active && !c.original && c.lits.len() > 2,
                    "invalid clause deletion",
                )?;
                c.active = false;
            }
            SolverEvent::SolveEnd { result } => {
                ensure(
                    self.learned.is_none(),
                    "solve ended during learned installation",
                )?;
                match result.as_str() {
                    "Unknown" | "unknown" => return Err(Issue::Stop(StopReason::NativeUnknown)),
                    "Sat" | "sat" => {
                        ensure(
                            !self.f.unsat,
                            "SAT endpoint contradicts construction-time contradiction",
                        )?;
                        b.charge(self.f.assignment.len(), false)?;
                        ensure(
                            self.f.assignment.iter().all(|&x| x != 0),
                            "incomplete SAT assignment",
                        )?;
                        for c in &self.f.clauses {
                            b.charge(c.lits.len(), false)?;
                            ensure(
                                c.lits.iter().any(|&l| val(&self.f.assignment, l) > 0),
                                "false SAT clause",
                            )?;
                        }
                        for r in &self.f.rows {
                            b.charge(r.mask.len(), true)?;
                            let vars = r.variables();
                            b.charge(vars.len(), false)?;
                            ensure(
                                vars.iter().fold(false, |p, &v| {
                                    p ^ (self.f.assignment[v as usize - 1] > 0)
                                }) == r.rhs,
                                "false SAT parity",
                            )?;
                        }
                    }
                    "Unsat" | "unsat" => {
                        ensure(
                            self.f.unsat || (self.boundaries.is_empty() && self.conflict.is_some()),
                            "UNSAT endpoint lacks checked root contradiction",
                        )?;
                    }
                    _ => return Err("unknown solve result".into()),
                }
                self.ended = true;
            }
            SolverEvent::Snapshot { .. } => return Err("duplicate snapshot".into()),
            SolverEvent::XorConstraint { mask, .. } | SolverEvent::XorMatrixRow { mask, .. } => {
                mask_check(&mask, self.f.n)?;
                return unsupported("dynamic_or_snapshot_xor");
            }
            SolverEvent::FormulaContradiction { constraint, .. } => {
                match constraint {
                    SolverEventConstraint::Clause { literals } => {
                        canonical(&literals, self.f.n, b)?;
                    }
                    SolverEventConstraint::Xor { variables, .. } => {
                        b.charge(variables.len(), false)?;
                        for v in variables {
                            ensure(v > 0 && v <= self.f.n, "invalid dynamic XOR variable")?;
                        }
                    }
                }
                return unsupported("dynamic_constraint_contradiction");
            }
            SolverEvent::SolveBegin { .. } => return unsupported("additional_solve"),
        }
        b.retained(self.logical())?;
        Ok(None)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    schema: String,
    pattern: u32,
    sequence: u64,
    native: SolverEvent,
}
fn finish_record(mut record: Value, pattern: Option<u32>, work: &Work) -> Value {
    record["pattern"] = json!(pattern);
    record["work"] = json!(work);
    record["native_execution_trace_verified"] = json!(false);
    record["native_instruction_count_verified"] = json!(false);
    record["unsat_proof_checked"] = json!(false);
    record
}
fn censor(
    reason: StopReason,
    global: bool,
    pattern: Option<u32>,
    seq: Option<u64>,
    b: &Budget,
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<(), String> {
    let record = finish_record(
        json!({"kind":"sat_unknown_suffix","admission_status":"bounded_unknown_suffix","scope":if global{"global"}else{"pattern"},"reason":reason,"event_sequence":seq}),
        pattern,
        &b.work,
    );
    size(&record, 4096).map_err(|_| "terminal SAT record exceeds separate bound")?;
    emit(None, record)
}
/// Caller supplies already verified public/base artifacts and hash/role-checked
/// map/formula bytes. These source values are used only for regeneration; only
/// `initial_public` (the caller's strict allowlist) may enter public features.
pub(super) fn replay(
    initial_public: &Value,
    public_instance: &Value,
    factor_base: &Value,
    patterns: &BTreeMap<u32, PatternSource>,
    mut reader: impl BufRead,
    limits: ReplayLimits,
    emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
) -> Result<ReplaySummary, String> {
    let mut b = Budget {
        limits,
        work: Work::default(),
    };
    let mut summary = ReplaySummary {
        verified_decisions: 0,
        unknown_suffix_records: 0,
        patterns_admitted: 0,
        work: Work::default(),
        limits: b.limits.clone(),
        native_execution_trace_verified: false,
        native_instruction_count_verified: false,
        unsat_proof_checked: false,
    };
    let mut current: Option<(u32, Option<State>)> = None;
    let mut seen = BTreeSet::new();
    let mut source_total = 0u64;
    for s in patterns.values() {
        b.work.source_payload_logical_bytes = b
            .work
            .source_payload_logical_bytes
            .saturating_add(source_logical(s));
        let amount = size(&s.gate_map, 16 << 20).and_then(|n| {
            n.checked_add(s.formula.len() as u64)
                .ok_or_else(|| Issue::Hard("source byte overflow".into()))
        });
        match amount {
            Ok(n) => source_total = source_total.saturating_add(n),
            Err(Issue::Hard(e)) => return Err(e),
            Err(Issue::Stop(reason)) => {
                censor(reason, true, None, None, &b, emit)?;
                summary.unknown_suffix_records += 1;
                summary.work = b.work;
                return Ok(summary);
            }
        }
    }
    if let Err(Issue::Stop(reason)) = b.retained(0) {
        censor(reason, true, None, None, &b, emit)?;
        summary.unknown_suffix_records += 1;
        summary.work = b.work;
        return Ok(summary);
    }
    if source_total > b.limits.source_bytes {
        censor(
            StopReason::Budget {
                resource: "source_bytes",
            },
            true,
            None,
            None,
            &b,
            emit,
        )?;
        summary.unknown_suffix_records += 1;
        summary.work = b.work;
        return Ok(summary);
    }
    loop {
        let remaining = b.limits.raw_bytes.saturating_sub(b.work.raw_bytes);
        let mut line = Vec::new();
        let len = std::io::Read::take(&mut reader, remaining + 1)
            .read_until(b'\n', &mut line)
            .map_err(|e| e.to_string())?;
        if len == 0 {
            if let Some((p, Some(s))) = &current {
                if !s.ended {
                    censor(StopReason::Truncated, false, Some(*p), None, &b, emit)?;
                    summary.unknown_suffix_records += 1;
                }
            }
            break;
        }
        b.work.raw_bytes = b
            .work
            .raw_bytes
            .saturating_add(len as u64)
            .min(b.limits.raw_bytes);
        let global_stop = if len as u64 > remaining {
            Some(StopReason::Budget {
                resource: "raw_bytes",
            })
        } else if b.work.raw_events >= b.limits.raw_events {
            Some(StopReason::Budget {
                resource: "raw_events",
            })
        } else {
            None
        };
        if let Some(reason) = global_stop {
            censor(reason, true, current.as_ref().map(|x| x.0), None, &b, emit)?;
            summary.unknown_suffix_records += 1;
            break;
        }
        if line.last() != Some(&b'\n') {
            censor(
                StopReason::Truncated,
                false,
                current.as_ref().map(|x| x.0),
                None,
                &b,
                emit,
            )?;
            summary.unknown_suffix_records += 1;
            break;
        }
        // Typed events have only flat scalar vectors. Reserve a conservative
        // expansion allowance before serde allocates them, including line bytes
        // and vector capacity growth. This is not an RSS measurement.
        let state_bytes = current
            .as_ref()
            .and_then(|(_, s)| s.as_ref())
            .map_or(0, State::logical);
        if let Err(Issue::Stop(reason)) = b.retained(state_bytes + 4096 + line.len() as u64 * 16) {
            censor(reason, true, current.as_ref().map(|x| x.0), None, &b, emit)?;
            summary.unknown_suffix_records += 1;
            break;
        }
        let env: Envelope =
            serde_json::from_slice(&line).map_err(|e| format!("malformed SAT event: {e}"))?;
        b.work.raw_events += 1;
        if env.schema != "ml-dataset.sat-event/v1" || env.sequence != b.work.raw_events {
            return Err("SAT event schema/sequence mismatch".into());
        }
        let p = env.pattern;
        let seq = env.sequence;
        let result: Check<Option<(Value, Value)>> = (|| {
            if current.as_ref().map(|x| x.0) != Some(p) {
                if let Some((old, Some(s))) = &current {
                    if !s.ended {
                        censor(
                            StopReason::Truncated,
                            false,
                            Some(*old),
                            Some(seq),
                            &b,
                            emit,
                        )
                        .map_err(Issue::Hard)?;
                        summary.unknown_suffix_records += 1;
                    }
                }
                ensure(seen.insert(p), "reordered or duplicate SAT pattern")?;
                let SolverEvent::Snapshot {
                    version,
                    variables,
                    trail_limits,
                    already_unsat,
                } = &env.native
                else {
                    return Err("pattern does not begin with snapshot".into());
                };
                let src = patterns.get(&p).ok_or("missing pattern source")?;
                current = Some((p, None));
                ensure(
                    *version == 1 && *variables == src.gate_map.variables,
                    "snapshot version/variables mismatch",
                )?;
                if !trail_limits.is_empty() {
                    return unsupported("prepropagated_snapshot");
                }
                let f = admit(public_instance, factor_base, p, src, &mut b)?;
                ensure(
                    *already_unsat == f.unsat,
                    "snapshot contradiction differs from normalized formula",
                )?;
                current = Some((p, Some(State::new(f))));
                summary.patterns_admitted += 1;
                return Ok(None);
            }
            let Some((_, Some(s))) = &mut current else {
                return Ok(None);
            };
            if summary.verified_decisions >= b.limits.decisions
                && matches!(env.native, SolverEvent::Decision { .. })
            {
                return Err(Issue::Stop(StopReason::Budget {
                    resource: "decision_contexts",
                }));
            }
            s.event(
                env.native,
                seq,
                initial_public,
                &patterns.get(&p).ok_or("missing pattern source")?.gate_map,
                &mut b,
            )
        })();
        match result {
            Ok(Some((inputs, record))) => {
                let record = finish_record(record, Some(p), &b.work);
                let remaining = b.limits.derived_bytes.saturating_sub(b.work.derived_bytes);
                match size(&inputs, remaining)
                    .and_then(|n| size(&record, remaining - n).map(|m| n + m))
                {
                    Ok(n) => {
                        b.work.derived_bytes += n;
                        emit(Some(inputs), record)?;
                        summary.verified_decisions += 1;
                    }
                    Err(Issue::Hard(e)) => return Err(e),
                    Err(Issue::Stop(reason)) => {
                        censor(reason, true, Some(p), Some(seq), &b, emit)?;
                        summary.unknown_suffix_records += 1;
                        break;
                    }
                }
            }
            Ok(None) => {}
            Err(Issue::Hard(e)) => return Err(format!("SAT pattern {p} event {seq}: {e}")),
            Err(Issue::Stop(reason)) => {
                let global = matches!(reason, StopReason::Budget { .. });
                censor(reason, global, Some(p), Some(seq), &b, emit)?;
                summary.unknown_suffix_records += 1;
                if global {
                    break;
                }
                current = Some((p, None));
            }
        }
    }
    summary.work = b.work;
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::binary_semaev_chain_sat::GateMapDomain;
    use crate::ml_dataset::types::CaseConfig;
    use std::io::Cursor;

    fn budget() -> Budget {
        Budget {
            limits: ReplayLimits::default(),
            work: Work::default(),
        }
    }
    fn map() -> GateMap {
        GateMap {
            schema: GATE_MAP_SCHEMA.into(),
            n: 3,
            m: 2,
            irreducible_low_terms: vec![0, 1],
            b: "1".into(),
            target_x: None,
            identity_mask: 0,
            variables: 3,
            coordinate_blocks: vec![],
            and_gates: vec![],
            xor_rows: vec![],
            unit_literals: vec![],
            impossible: false,
            finite_domain: Some(GateMapDomain {
                codes: vec![],
                raw_clause_count: 0,
            }),
            reduction_outputs: vec![],
            square_inputs: vec![],
        }
    }
    fn state(clauses: &[&[i32]]) -> State {
        let mut b = budget();
        let mut f = Installed::new(4);
        for c in clauses {
            f.clause(c, &mut b).unwrap();
        }
        let mut s = State::new(f);
        s.begun = true;
        s
    }
    fn step(s: &mut State, e: SolverEvent) -> Check<Option<(Value, Value)>> {
        s.event(
            e,
            1,
            &json!({"structural":"fixture"}),
            &map(),
            &mut budget(),
        )
    }
    fn decide(s: &mut State, literal: i32) -> Value {
        let level = s.boundaries.len() + 1;
        let trail_index = s.trail.len();
        assert!(step(
            s,
            SolverEvent::Decision {
                literal,
                level,
                trail_index,
                activity_bits: 0,
                priority: false
            }
        )
        .unwrap()
        .is_none());
        step(
            s,
            SolverEvent::Assignment {
                literal,
                level: level as i32,
                trail_index,
                initial: false,
                reason: SolverEventReason::Decision,
            },
        )
        .unwrap()
        .unwrap()
        .0
    }
    #[test]
    fn installation_units_tautologies_and_sticky_contradiction() {
        let mut f = Installed::new(3);
        let mut b = budget();
        f.clause(&[1, 1], &mut b).unwrap();
        f.clause(&[-1, 2], &mut b).unwrap();
        f.clause(&[3, -3], &mut b).unwrap();
        f.clause(&[-2], &mut b).unwrap();
        assert_eq!(f.units, vec![(1, 0), (2, 2)]);
        assert!(f.unsat);
        assert_eq!(f.next, 4);
        check_dimacs("p cnf 3 3\n1 0\n2 0\n0\n", &f, &mut b).unwrap();
        assert!(check_dimacs("p cnf 3 3\n1 0\n-2 0\n0\n", &f, &mut b).is_err());
        assert!(f.clause(&[i32::MIN], &mut b).is_err());
        assert!(mask_check(&[1u64 << 3], 3).is_err());
        assert!(mask_check(&[], 3).is_err());
    }
    #[test]
    fn rup_uses_formula_not_search_decisions_and_retains_partial_budget() {
        let mut s = state(&[]);
        decide(&mut s, 1);
        assert!(!rup(&s, &[1], &mut budget()).unwrap());
        let s = state(&[&[-1, 2], &[-1, -2]]);
        assert!(rup(&s, &[-1], &mut budget()).unwrap());
        let mut b = budget();
        b.limits.operations = 2;
        assert!(matches!(
            rup(&s, &[-1], &mut b),
            Err(Issue::Stop(StopReason::Budget { .. }))
        ));
        assert_eq!(b.work.literal_visits + b.work.parity_word_operations, 2);
        let mut s = state(&[]);
        s.f.rows.push(Row {
            mask: vec![3],
            rhs: false,
        });
        assert!(rup(&s, &[-1, 2], &mut budget()).unwrap());
    }
    #[test]
    fn learned_unit_assignment_before_installation_and_causal_context() {
        let mut s = state(&[&[-1, 2], &[-1, -2]]);
        let before = decide(&mut s, 1);
        assert_eq!(before["active_state"]["trail"], json!([]));
        step(
            &mut s,
            SolverEvent::Assignment {
                literal: 2,
                level: 1,
                trail_index: 1,
                initial: false,
                reason: SolverEventReason::Clause { clause_id: 0 },
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Conflict {
                level: 1,
                reason: SolverEventReason::Clause { clause_id: 3 },
                literals: vec![-1, -2],
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Learned {
                clause_id: 6,
                literals: vec![-1],
                backjump_level: 0,
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Backjump {
                from_level: 1,
                to_level: 0,
                retained_trail: 0,
                removed: vec![1, 2],
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Assignment {
                literal: -1,
                level: 0,
                trail_index: 0,
                initial: false,
                reason: SolverEventReason::Clause { clause_id: 6 },
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Clause {
                clause_id: 6,
                literals: vec![-1],
                existing: false,
                detached: false,
            },
        )
        .unwrap();
        assert!(s.learned.is_none());
        let after = decide(&mut s, 3);
        assert_eq!(
            after["active_state"]["trail"],
            json!([{"literal":-1,"level":0}])
        );
        assert_eq!(
            before["active_state"]["clauses"].as_array().unwrap().len(),
            2
        );
        assert_eq!(
            after["active_state"]["clauses"].as_array().unwrap().len(),
            3
        );
    }
    #[test]
    fn learned_nonunit_installation_then_assignment_and_backjump_corruption() {
        let mut s = state(&[&[-1, -2, 3], &[-1, -2, -3]]);
        decide(&mut s, 1);
        decide(&mut s, 2);
        step(
            &mut s,
            SolverEvent::Assignment {
                literal: 3,
                level: 2,
                trail_index: 2,
                initial: false,
                reason: SolverEventReason::Clause { clause_id: 0 },
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Conflict {
                level: 2,
                reason: SolverEventReason::Clause { clause_id: 4 },
                literals: vec![-1, -2, -3],
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Learned {
                clause_id: 8,
                literals: vec![-2, -1],
                backjump_level: 1,
            },
        )
        .unwrap();
        assert!(step(
            &mut s,
            SolverEvent::Backjump {
                from_level: 2,
                to_level: 1,
                retained_trail: 0,
                removed: vec![1, 2, 3]
            }
        )
        .is_err());
        step(
            &mut s,
            SolverEvent::Backjump {
                from_level: 2,
                to_level: 1,
                retained_trail: 1,
                removed: vec![2, 3],
            },
        )
        .unwrap();
        assert!(step(
            &mut s,
            SolverEvent::Clause {
                clause_id: 8,
                literals: vec![-2, -1, -1],
                existing: false,
                detached: false
            }
        )
        .is_err());
        assert_eq!(s.f.next, 8);
        assert!(!s.learned.as_ref().unwrap().confirmed);
        step(
            &mut s,
            SolverEvent::Clause {
                clause_id: 8,
                literals: vec![-1, -2],
                existing: false,
                detached: false,
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Assignment {
                literal: -2,
                level: 1,
                trail_index: 1,
                initial: false,
                reason: SolverEventReason::Clause { clause_id: 8 },
            },
        )
        .unwrap();
        assert!(s.learned.is_none());
        assert_eq!(s.trail, vec![1, -2]);
        assert_eq!(s.f.next, 11);
    }
    #[test]
    fn xor_reasons_and_row_operations_are_checked() {
        let mut s = state(&[]);
        s.f.rows = vec![
            Row {
                mask: vec![3],
                rhs: false,
            },
            Row {
                mask: vec![6],
                rhs: true,
            },
        ];
        step(&mut s, SolverEvent::XorMatrixReset).unwrap();
        decide(&mut s, 1);
        assert!(step(
            &mut s,
            SolverEvent::XorImplication {
                row_id: 0,
                literal: -2,
                explanation: vec![-2, -1]
            }
        )
        .is_err());
        step(
            &mut s,
            SolverEvent::XorImplication {
                row_id: 0,
                literal: 2,
                explanation: vec![2, -1],
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::Assignment {
                literal: 2,
                level: 1,
                trail_index: 1,
                initial: false,
                reason: SolverEventReason::Xor {
                    explanation: vec![2, -1],
                },
            },
        )
        .unwrap();
        step(
            &mut s,
            SolverEvent::XorRowXor {
                destination: 1,
                source: 0,
            },
        )
        .unwrap();
        assert_eq!(
            s.matrix[1],
            Row {
                mask: vec![5],
                rhs: true
            }
        );
        assert!(step(
            &mut s,
            SolverEvent::XorRowXor {
                destination: 1,
                source: 1
            }
        )
        .is_err());
        assert!(step(
            &mut s,
            SolverEvent::Assignment {
                literal: 3,
                level: 1,
                trail_index: 2,
                initial: false,
                reason: SolverEventReason::Xor {
                    explanation: vec![3, -1]
                }
            }
        )
        .is_err());
    }
    #[test]
    fn action_is_restricted_and_unsupported_suffix_does_not_change_prefix() {
        let mut a = state(&[]);
        let mut b = state(&[]);
        let first = decide(&mut a, 1);
        let alternate = decide(&mut b, -2);
        assert_eq!(first, alternate);
        assert!(matches!(
            step(
                &mut a,
                SolverEvent::Clause {
                    clause_id: 0,
                    literals: vec![3],
                    existing: false,
                    detached: false
                }
            ),
            Err(Issue::Stop(StopReason::Unsupported { .. }))
        ));
        assert_eq!(first, alternate);
        let mut pending = state(&[]);
        assert!(step(
            &mut pending,
            SolverEvent::Decision {
                literal: 1,
                level: 1,
                trail_index: 0,
                activity_bits: 4,
                priority: true
            }
        )
        .unwrap()
        .is_none());
        assert!(step(
            &mut pending,
            SolverEvent::SolveEnd {
                result: "Unknown".into()
            }
        )
        .is_err());
    }
    #[test]
    fn restart_and_deletion_preserve_locked_and_original_reasons() {
        let mut s = state(&[&[1, 2, 3]]);
        decide(&mut s, 1);
        step(
            &mut s,
            SolverEvent::Restart {
                restart: 1,
                conflicts: 0,
            },
        )
        .unwrap();
        assert!(step(
            &mut s,
            SolverEvent::Decision {
                literal: 2,
                level: 2,
                trail_index: 1,
                activity_bits: 0,
                priority: false
            }
        )
        .is_err());
        step(
            &mut s,
            SolverEvent::Backjump {
                from_level: 1,
                to_level: 0,
                retained_trail: 0,
                removed: vec![1],
            },
        )
        .unwrap();
        assert!(step(&mut s, SolverEvent::DeleteClause { clause_id: 0 }).is_err());
        // A certified duplicate lemma may remain in the proof archive after deletion.
        s.f.clauses.push(Clause {
            id: 4,
            lits: vec![1, 2, 3],
            original: false,
            active: true,
        });
        step(&mut s, SolverEvent::DeleteClause { clause_id: 4 }).unwrap();
        assert!(!s.f.clauses[1].active);
        assert!(rup(&s, &[1, 2, 3], &mut budget()).unwrap());
    }
    fn source() -> (Value, Value, BTreeMap<u32, PatternSource>) {
        let data = curves::generate(&CaseConfig {
            family: "koblitz".into(),
            n: 7,
            m: 2,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 1,
            seed: 42,
            factor_base_size: 8,
            degree_bound: 4,
            work_limit: 1000,
            solver: "sat".into(),
        })
        .unwrap();
        let curve = curves::binary_curve(&data.public_instance).unwrap();
        let target = curves::binary_point(&data.public_instance["target"], 7).unwrap();
        let tx = match &target {
            BinaryPoint::Affine { x, .. } => Some(x),
            _ => None,
        };
        let mut codes = data.factor_base["points"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|p| match curves::binary_point(p, 7).unwrap() {
                BinaryPoint::Affine { x, .. } => Some(x.to_biguint()),
                _ => None,
            })
            .collect::<Vec<_>>();
        codes.sort();
        codes.dedup();
        let mut enc = ChainedS3Encoding::build_with_gate_map(
            2,
            &curve.irreducible,
            &curve.b,
            tx,
            0,
            100_000,
            GateMapLimits::default(),
        )
        .unwrap();
        enc.constrain_summands(&codes, 1_000_000).unwrap();
        let src = PatternSource {
            gate_map: enc.gate_map().unwrap().clone(),
            formula: crate::cryptanalysis::sat::to_dimacs_xor(&enc.solver),
        };
        (
            data.public_instance,
            data.factor_base,
            BTreeMap::from([(0, src)]),
        )
    }
    #[test]
    fn native_source_admission_rejects_substitution_and_budget_is_unknown() {
        let (public, base, mut sources) = source();
        admit(&public, &base, 0, &sources[&0], &mut budget()).unwrap();
        let mut b = budget();
        b.limits.variables = 1;
        assert!(matches!(
            admit(&public, &base, 0, &sources[&0], &mut b),
            Err(Issue::Stop(StopReason::Budget { .. }))
        ));
        sources.get_mut(&0).unwrap().gate_map.b = "0".into();
        assert!(matches!(
            admit(&public, &base, 0, &sources[&0], &mut budget()),
            Err(Issue::Hard(_))
        ));
    }
    fn snapshot_stream(
        public: &Value,
        base: &Value,
        sources: &BTreeMap<u32, PatternSource>,
    ) -> Vec<Value> {
        let f = admit(public, base, 0, &sources[&0], &mut budget()).unwrap();
        let mut events = vec![SolverEvent::Snapshot {
            version: 1,
            variables: f.n,
            trail_limits: vec![],
            already_unsat: f.unsat,
        }];
        for c in &f.clauses {
            events.push(SolverEvent::Clause {
                clause_id: c.id,
                literals: c.lits.clone(),
                existing: true,
                detached: false,
            });
        }
        for (row_id, r) in f.rows.iter().enumerate() {
            events.push(SolverEvent::XorConstraint {
                row_id,
                mask: r.mask.clone(),
                rhs: r.rhs,
            });
        }
        for (trail_index, &(literal, id)) in f.units.iter().enumerate() {
            events.push(SolverEvent::Assignment {
                literal,
                level: 0,
                trail_index,
                initial: true,
                reason: SolverEventReason::Clause { clause_id: id },
            });
        }
        events.push(SolverEvent::SolveBegin {
            conflicts: 0,
            conflict_budget: 100,
        });
        let literal = (1..=f.n)
            .find(|&v| f.assignment[v as usize - 1] == 0)
            .unwrap() as i32;
        events.push(SolverEvent::Decision {
            literal,
            level: 1,
            trail_index: f.units.len(),
            activity_bits: 0,
            priority: false,
        });
        events.push(SolverEvent::Assignment {
            literal,
            level: 1,
            trail_index: f.units.len(),
            initial: false,
            reason: SolverEventReason::Decision,
        });
        events.into_iter().enumerate().map(|(i,native)|json!({"schema":"ml-dataset.sat-event/v1","pattern":0,"sequence":i+1,"native":native})).collect()
    }
    fn stream(events: &[Value]) -> Vec<u8> {
        events
            .iter()
            .flat_map(|e| {
                let mut v = serde_json::to_vec(e).unwrap();
                v.push(b'\n');
                v
            })
            .collect()
    }
    #[test]
    fn complete_snapshot_pair_prefix_preservation_truncation_and_sequence() {
        let (public, base, sources) = source();
        let mut events = snapshot_stream(&public, &base, &sources);
        let mut rows = Vec::new();
        let summary = replay(
            &json!({"strict":true}),
            &public,
            &base,
            &sources,
            Cursor::new(stream(&events)),
            ReplayLimits::default(),
            &mut |i, r| {
                rows.push((i, r));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(summary.verified_decisions, 1);
        assert_eq!(summary.unknown_suffix_records, 1);
        let admitted = rows[0].0.clone();
        assert!(admitted.is_some());
        let seq = events.len() + 1;
        events.push(json!({"schema":"ml-dataset.sat-event/v1","pattern":0,"sequence":seq,"native":{"kind":"solve_end","result":"Unknown"}}));
        let mut alternate = Vec::new();
        replay(
            &json!({"strict":true}),
            &public,
            &base,
            &sources,
            Cursor::new(stream(&events)),
            ReplayLimits::default(),
            &mut |i, r| {
                alternate.push((i, r));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(admitted, alternate[0].0);
        events.pop();
        events.pop();
        let mut partial = Vec::new();
        let summary = replay(
            &json!({}),
            &public,
            &base,
            &sources,
            Cursor::new(stream(&events)),
            ReplayLimits::default(),
            &mut |i, r| {
                partial.push((i, r));
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(summary.verified_decisions, 0);
        events[0]["sequence"] = json!(2);
        assert!(replay(
            &json!({}),
            &public,
            &base,
            &sources,
            Cursor::new(stream(&events)),
            ReplayLimits::default(),
            &mut |_, _| Ok(())
        )
        .is_err());
    }
    #[test]
    fn proof_miss_is_distinct_from_false_propagation_and_cap() {
        let mut s = state(&[&[-1, -2]]);
        decide(&mut s, 1);
        decide(&mut s, 2);
        step(
            &mut s,
            SolverEvent::Conflict {
                level: 2,
                reason: SolverEventReason::Clause { clause_id: 0 },
                literals: vec![-1, -2],
            },
        )
        .unwrap();
        assert!(matches!(
            step(
                &mut s,
                SolverEvent::Learned {
                    clause_id: 3,
                    literals: vec![-2],
                    backjump_level: 0
                }
            ),
            Err(Issue::Stop(StopReason::ProofNotEstablished))
        ));
        let mut s = state(&[&[-1, 2]]);
        assert!(step(
            &mut s,
            SolverEvent::Assignment {
                literal: 2,
                level: 0,
                trail_index: 0,
                initial: false,
                reason: SolverEventReason::Clause { clause_id: 0 }
            }
        )
        .is_err());
        let mut b = budget();
        b.limits.derived_bytes = 32;
        assert!(matches!(
            s.features(&json!({}), &map(), &mut b),
            Err(Issue::Stop(StopReason::Budget { .. }))
        ));
    }
    #[test]
    fn native_mixed_solver_stream_matches_independent_boolean_replay() {
        use crate::cryptanalysis::sat::Solver;
        use std::sync::{Arc, Mutex};
        let mut solver = Solver::new(4);
        let mut f = Installed::new(4);
        for c in [vec![-1, 2], vec![-1, -2], vec![2, 3]] {
            solver.add_clause(c.clone());
            f.clause(&c, &mut budget()).unwrap();
        }
        solver.add_xor(&[3, 4], false);
        f.rows.push(Row {
            mask: vec![12],
            rhs: false,
        });
        f.logical_bytes += 24;
        let events = Arc::new(Mutex::new(Vec::new()));
        let sink = events.clone();
        solver
            .set_event_observer(move |e| {
                sink.lock().unwrap().push(e.clone());
                Ok(())
            })
            .unwrap();
        solver.solve();
        let mut s = State::new(f);
        let mut decisions = 0;
        for (i, e) in events.lock().unwrap().iter().enumerate() {
            if i == 0 {
                assert!(matches!(
                    e,
                    SolverEvent::Snapshot {
                        version: 1,
                        variables: 4,
                        ..
                    }
                ));
                continue;
            }
            if s.event(e.clone(), i as u64 + 1, &json!({}), &map(), &mut budget())
                .unwrap()
                .is_some()
            {
                decisions += 1;
            }
        }
        assert!(s.ended);
        assert!(decisions > 0);
    }
    #[test]
    fn original_snapshot_rejects_duplicates_but_accepts_reordered_literals() {
        let mut f = Installed::new(4);
        f.clause(&[1, 2], &mut budget()).unwrap();
        let mut s = State::new(f);
        assert!(s
            .snapshot_event(
                &SolverEvent::Clause {
                    clause_id: 0,
                    literals: vec![2, 1, 1],
                    existing: true,
                    detached: false
                },
                &mut budget()
            )
            .is_err());
        assert_eq!(s.snapshot_pos, 0);
        assert_eq!(s.f.next, 3);
        assert!(s
            .snapshot_event(
                &SolverEvent::Clause {
                    clause_id: 0,
                    literals: vec![2, 1],
                    existing: true,
                    detached: false
                },
                &mut budget()
            )
            .unwrap());
        assert_eq!(s.snapshot_pos, 1);
        assert_eq!(s.f.next, 3);
    }
    #[test]
    fn sticky_construction_contradiction_cannot_end_sat() {
        let mut f = Installed::new(4);
        for literal in 1..=4 {
            f.clause(&[literal], &mut budget()).unwrap();
        }
        f.clause(&[], &mut budget()).unwrap();
        assert!(f.unsat);
        let mut s = State::new(f);
        for i in 0..4 {
            step(
                &mut s,
                SolverEvent::Clause {
                    clause_id: i * 2,
                    literals: vec![i as i32 + 1],
                    existing: true,
                    detached: false,
                },
            )
            .unwrap();
        }
        for i in 0..4 {
            step(
                &mut s,
                SolverEvent::Assignment {
                    literal: i as i32 + 1,
                    level: 0,
                    trail_index: i,
                    initial: true,
                    reason: SolverEventReason::Clause {
                        clause_id: i as u32 * 2,
                    },
                },
            )
            .unwrap();
        }
        step(
            &mut s,
            SolverEvent::SolveBegin {
                conflicts: 0,
                conflict_budget: 10,
            },
        )
        .unwrap();
        assert!(step(
            &mut s,
            SolverEvent::SolveEnd {
                result: "Sat".into()
            }
        )
        .is_err());
        assert!(!s.ended);
        step(
            &mut s,
            SolverEvent::SolveEnd {
                result: "Unsat".into(),
            },
        )
        .unwrap();
        assert!(s.ended);
    }
}
