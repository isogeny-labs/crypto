//! Supplied group-equation validation; collection provenance is a separate fact.
use crate::{large_prime_filter::PartialRelation, sha256, COORDINATE_PRIME, ELL, MAX_INPUT_BYTES, MAX_ROWS, TARGET};
use prime31_algebraic_ic::{check_curve, Arithmetic, Counts, PRIME31};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: String,
    pub coordinate_prime: u64,
    pub relation_prime: u64,
    pub curve_a: u64,
    pub curve_b: u64,
    pub generator: (u64, u64),
    pub target: (u64, u64),
    pub target_column: u32,
    pub input_sha256: String,
    pub small_points: Vec<(u32, (u64, u64))>,
    pub large_points: Vec<(u64, (u64, u64))>,
    /// Description only. This harness does not attest collector provenance.
    pub provenance_label: String,
}

pub struct Admitted {
    pub(crate) manifest: Manifest,
    pub(crate) rows: Vec<PartialRelation>,
    pub(crate) group_check_counts: Counts,
}
impl Admitted {
    pub fn manifest(&self) -> &Manifest { &self.manifest }
    pub fn rows(&self) -> &[PartialRelation] { &self.rows }
    pub fn group_check_counts(&self) -> Counts { self.group_check_counts }
}

/// Canonical coefficients, ordering, stable IDs and arity are required, not repaired.
pub fn validate_rows(rows: &[PartialRelation]) -> Result<(), String> {
    if rows.is_empty() || rows.len() > MAX_ROWS { return Err("row count must be 1..=4096".into()); }
    let mut ids = BTreeSet::new();
    let mut terms = 0usize;
    for (i, r) in rows.iter().enumerate() {
        if !ids.insert(r.id) { return Err(format!("duplicate source id {}", r.id)); }
        if r.rhs >= ELL || r.large.len() > 2
            || r.small.iter().any(|x| x.1 == 0 || x.1 >= ELL)
            || r.large.iter().any(|x| x.1 == 0 || x.1 >= ELL)
            || r.small.windows(2).any(|w| w[0].0 >= w[1].0)
            || r.large.windows(2).any(|w| w[0].0 >= w[1].0)
        { return Err(format!("noncanonical or >2-LP input row {i}")); }
        terms = terms.checked_add(r.small.len() + r.large.len()).ok_or("term overflow")?;
        if terms > 1_000_000 { return Err("input term cap".into()); }
    }
    Ok(())
}

/// Bounded byte/row/shape parsing only. The result has no group-admission status.
/// Coordinators can inspect prefixes cheaply; full admission belongs in children.
pub fn parse_rows(bytes: &[u8]) -> Result<Vec<PartialRelation>, String> {
    if bytes.len() as u64 > MAX_INPUT_BYTES { return Err("input byte cap".into()); }
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() { return Err(format!("blank source row {i}")); }
        if rows.len() == MAX_ROWS { return Err("row cap".into()); }
        let value: serde_json::Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let object = value.as_object().ok_or("row must be an object")?;
        if object.len() != 4 || !["id", "small", "large", "rhs"].iter().all(|k| object.contains_key(*k)) {
            return Err(format!("unexpected source-row keys at {i}"));
        }
        rows.push(serde_json::from_value(value).map_err(|e| e.to_string())?);
    }
    validate_rows(&rows)?;
    Ok(rows)
}

pub fn admit(manifest: Manifest, bytes: &[u8]) -> Result<Admitted, String> {
    if bytes.len() as u64 > MAX_INPUT_BYTES { return Err("input byte cap".into()); }
    if manifest.schema != "m31-graph-input/v1"
        || manifest.coordinate_prime != COORDINATE_PRIME || manifest.relation_prime != ELL
        || manifest.curve_a != PRIME31.a || manifest.curve_b != PRIME31.b
        || manifest.generator != PRIME31.g || manifest.target_column != TARGET
    { return Err("wrong fixed M31 curve, subgroup or target-column identity".into()); }
    if sha256(bytes) != manifest.input_sha256 { return Err("input SHA-256 mismatch".into()); }
    if manifest.small_points.len() + manifest.large_points.len() > 8192 { return Err("point-map cap".into()); }
    let rows = parse_rows(bytes)?;
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let mut small = BTreeMap::new();
    let mut large = BTreeMap::new();
    let mut coordinates = BTreeSet::new();
    for &(id, point) in &manifest.small_points {
        if small.insert(id, point).is_some() || !coordinates.insert(point) {
            return Err("duplicate small key or point identity".into());
        }
    }
    for &(id, point) in &manifest.large_points {
        if large.insert(id, point).is_some() || !coordinates.insert(point) {
            return Err("duplicate large key or point identity".into());
        }
    }
    if small.get(&TARGET).copied() != Some(manifest.target) { return Err("protected target map must equal Q".into()); }
    for point in coordinates {
        if !ar.on_curve(PRIME31, Some(point)) || ar.scalar(PRIME31, ELL, Some(point)).is_some() {
            return Err("invalid curve/subgroup point mapping".into());
        }
    }
    for row in &rows {
        let mut sum = None;
        for &(id, coefficient) in &row.small {
            let point = small.get(&id).ok_or("missing small point")?;
            let term = ar.scalar(PRIME31, coefficient, Some(*point));
            sum = ar.add(PRIME31, sum, term);
        }
        for &(id, coefficient) in &row.large {
            let point = large.get(&id).ok_or("missing large point")?;
            let term = ar.scalar(PRIME31, coefficient, Some(*point));
            sum = ar.add(PRIME31, sum, term);
        }
        let expected = ar.scalar(PRIME31, row.rhs, Some(PRIME31.g));
        if sum != expected { return Err(format!("group equation failed for source id {}", row.id)); }
    }
    Ok(Admitted { manifest, rows, group_check_counts: ar.counts })
}
