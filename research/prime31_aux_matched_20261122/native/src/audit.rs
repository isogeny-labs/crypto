//! Offline mathematical replay of one native JSONL cell. This module never
//! calls production curve arithmetic, the oracle, or IncrementalGauss.
//! Negative-search completeness, operation counts, timings and execution
//! provenance are separate claims, not established by this witness replay.

use crate::bases::{self, BaseDefinition, BaseFamily, CurveDefinition, PointDefinition};
use crate::collector::{CollectionLimits, PublicTarget};
use crate::oracle::{OracleResult, OracleStatus, Presentation};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

fn field<'a>(value: &'a Value, key: &str) -> Result<&'a Value, String> {
    value.get(key).ok_or_else(|| format!("missing {key}"))
}
fn integer(value: &Value, key: &str) -> Result<u64, String> {
    field(value, key)?
        .as_u64()
        .ok_or_else(|| format!("{key} is not a nonnegative integer"))
}
fn boolean(value: &Value, key: &str) -> Result<bool, String> {
    field(value, key)?
        .as_bool()
        .ok_or_else(|| format!("{key} is not a boolean"))
}
fn decode<T: DeserializeOwned>(value: &Value, key: &str) -> Result<T, String> {
    serde_json::from_value(field(value, key)?.clone()).map_err(|e| format!("invalid {key}: {e}"))
}
fn optional_scalar(value: &Value, key: &str) -> Result<Option<u64>, String> {
    let v = field(value, key)?;
    if v.is_null() {
        Ok(None)
    } else {
        v.as_u64().map(Some).ok_or_else(|| format!("invalid {key}"))
    }
}
fn point(q: crate::replay::Point) -> PointDefinition {
    PointDefinition {
        x: q.x,
        y: q.y,
        infinity: q.infinity,
    }
}
fn target_equal(a: &PublicTarget, b: &PublicTarget) -> bool {
    (a.p, a.a, a.b, a.order, a.generator, a.target)
        == (b.p, b.a, b.b, b.order, b.generator, b.target)
}
fn independent_target_check(t: &PublicTarget) -> Result<(), String> {
    bases::verify_target_fixture(
        &t.curve(),
        t.order,
        crate::PrimePoint {
            x: t.generator.x,
            y: t.generator.y,
            infinity: t.generator.infinity,
        },
    )?;
    let e = CurveDefinition {
        p: t.p,
        a: t.a,
        b: t.b,
    };
    let q = point(t.target);
    if q.infinity
        || !bases::independent_on_curve(&e, q)
        || !bases::independent_scalar(&e, q, t.order).infinity
    {
        return Err("public target fails independent subgroup replay".into());
    }
    Ok(())
}
fn curve(t: &PublicTarget) -> CurveDefinition {
    CurveDefinition {
        p: t.p,
        a: t.a,
        b: t.b,
    }
}
fn mul(a: u64, b: u64, p: u64) -> u64 {
    (a as u128 * b as u128 % p as u128) as u64
}
fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n != 0 {
        if n & 1 != 0 {
            out = mul(out, a, p);
        }
        a = mul(a, a, p);
        n >>= 1;
    }
    out
}

/// Independent batch Gauss-Jordan, with columns visited in order. RHS is
/// optional; when present, inconsistent zero coefficient rows are rejected.
fn rref(
    mut matrix: Vec<Vec<u64>>,
    columns: usize,
    modulus: u64,
) -> Result<(usize, Vec<Vec<u64>>), String> {
    let mut rank = 0;
    for column in 0..columns {
        let Some(pivot) = (rank..matrix.len()).find(|&i| matrix[i][column] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let inverse = power(matrix[rank][column], modulus - 2, modulus);
        for value in &mut matrix[rank] {
            *value = mul(*value, inverse, modulus);
        }
        for i in 0..matrix.len() {
            if i == rank {
                continue;
            }
            let scale = matrix[i][column];
            for j in 0..matrix[i].len() {
                matrix[i][j] =
                    (matrix[i][j] + modulus - mul(scale, matrix[rank][j], modulus)) % modulus;
            }
        }
        rank += 1;
    }
    if matrix.iter().any(|row| {
        row.len() == columns + 1 && row[..columns].iter().all(|&x| x == 0) && row[columns] != 0
    }) {
        return Err("independent modular elimination found inconsistent rows".into());
    }
    Ok((rank, matrix))
}

#[derive(Default)]
struct IndependentRows {
    a: Vec<Vec<u64>>,
    augmented: Vec<Vec<u64>>,
    unique_a: BTreeSet<Vec<u64>>,
    rank_a: usize,
    rank_augmented: usize,
    pinned: Option<u64>,
    duplicates: u64,
    useful_dependencies: u64,
}
impl IndependentRows {
    fn insert(&mut self, row: &[i64], alpha: u64, beta: u64, n: u64) -> Result<(), String> {
        let c = row.len();
        let a: Vec<u64> = row
            .iter()
            .map(|&x| (x as i128).rem_euclid(n as i128) as u64)
            .collect();
        if !self.unique_a.insert(a.clone()) {
            self.duplicates += 1;
        }
        self.a.push(a.clone());
        let mut augmented = a;
        augmented.push((n - beta) % n);
        augmented.push(alpha);
        self.augmented.push(augmented);
        let previous_rank = self.rank_a;
        let previously_pinned = self.pinned;
        self.rank_a = rref(self.a.clone(), c, n)?.0;
        let (rank, reduced) = rref(self.augmented.clone(), c + 1, n)?;
        self.rank_augmented = rank;
        self.pinned = reduced.iter().find_map(|row| {
            (row[c] == 1 && row[..c].iter().all(|&x| x == 0)).then_some(row[c + 1])
        });
        if self.rank_a == previous_rank && previously_pinned.is_none() && self.pinned.is_some() {
            self.useful_dependencies += 1;
        }
        Ok(())
    }
}

/// Independent implementation of the frozen SplitMix/rejection-sampling
/// specification. This verifies that query labels were not selected afterwards.
struct PublicStream {
    state: u64,
    words: u64,
}
impl PublicStream {
    fn scalar(&mut self, n: u64) -> Result<u64, String> {
        let threshold = ((1u128 << 64) % n as u128) as u64;
        for _ in 0..1_000_000 {
            self.words += 1;
            self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = self.state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^= z >> 31;
            if z >= threshold {
                return Ok(z % n);
            }
        }
        Err("query stream exceeded rejection-sampling verification cap".into())
    }
}

#[derive(Clone)]
struct Witness {
    row: Vec<i64>,
    points: Vec<PointDefinition>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum QueryOutcome {
    Pending,
    Positive,
    Negative,
    Unknown,
}
struct Query {
    number: u64,
    alpha: u64,
    beta: u64,
    target: PointDefinition,
    outcome: QueryOutcome,
    witnesses: Vec<Witness>,
    consumed: usize,
}

struct State {
    cell: Option<Value>,
    target: Option<PublicTarget>,
    reference: Option<BaseDefinition>,
    selected: Option<BaseDefinition>,
    signed_points: Vec<PointDefinition>,
    signed_columns: BTreeMap<(u64, u64), (usize, i64)>,
    arity: usize,
    presentation: Option<Presentation>,
    stream: Option<PublicStream>,
    max_queries: u64,
    current: Option<Query>,
    queries: u64,
    identities: u64,
    positives: u64,
    negatives: u64,
    unknowns: u64,
    oracle_witnesses: u64,
    rows: IndependentRows,
    collection_summary: Option<Value>,
    cell_summary: bool,
    failure_terminal: bool,
    rho_terminal: bool,
    rho_scalar: Option<u64>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            cell: None,
            target: None,
            reference: None,
            selected: None,
            signed_points: vec![],
            signed_columns: BTreeMap::new(),
            arity: 0,
            presentation: None,
            stream: None,
            max_queries: 0,
            current: None,
            queries: 0,
            identities: 0,
            positives: 0,
            negatives: 0,
            unknowns: 0,
            oracle_witnesses: 0,
            rows: IndependentRows::default(),
            collection_summary: None,
            cell_summary: false,
            failure_terminal: false,
            rho_terminal: false,
            rho_scalar: None,
        }
    }
}

impl State {
    fn public_target(&self) -> Result<&PublicTarget, String> {
        self.target
            .as_ref()
            .ok_or("missing cell_start public target".into())
    }
    fn selected_base(&self) -> Result<&BaseDefinition, String> {
        self.selected.as_ref().ok_or("missing selected_base".into())
    }

    fn verify_witness(
        &self,
        points: &[PointDefinition],
        row: &[i64],
        target: PointDefinition,
    ) -> Result<(), String> {
        let base = self.selected_base()?;
        if points.len() != self.arity || row.len() != base.columns {
            return Err("signed relation dimensions differ from frozen input".into());
        }
        let mut recomputed = vec![0i64; base.columns];
        let mut sum = PointDefinition {
            x: 0,
            y: 0,
            infinity: true,
        };
        for &q in points {
            if q.infinity {
                return Err("factor-base witness includes infinity".into());
            }
            let &(column, sign) = self
                .signed_columns
                .get(&(q.x, q.y))
                .ok_or("relation point outside canonical signed base")?;
            recomputed[column] += sign;
            sum = bases::independent_add(&base.curve, sum, q);
        }
        if recomputed != row || sum != target {
            return Err("signed points, column coefficients or target sum disagree".into());
        }
        Ok(())
    }

    fn scalar_replays(&self, d: u64) -> Result<(), String> {
        let target = self.public_target()?;
        if d >= target.order
            || bases::independent_scalar(&curve(target), point(target.generator), d)
                != point(target.target)
        {
            return Err("independently recovered scalar fails the public-point equation".into());
        }
        Ok(())
    }

    fn compare_summary(&self, summary: &Value) -> Result<(), String> {
        let pending = self
            .current
            .as_ref()
            .is_some_and(|q| q.outcome == QueryOutcome::Pending);
        if pending {
            return Err("collection summary omits a started query's outcome".into());
        }
        let fields = [
            ("queries", self.queries),
            ("identity_queries", self.identities),
            ("positive_queries", self.positives),
            ("complete_negative_queries", self.negatives),
            ("unknown_queries", self.unknowns),
            ("accepted_rows", self.rows.a.len() as u64),
            ("duplicate_coefficient_rows", self.rows.duplicates),
            ("rank_a", self.rows.rank_a as u64),
            ("rank_augmented", self.rows.rank_augmented as u64),
            ("useful_dependencies", self.rows.useful_dependencies),
            ("independent_replay_rows", self.rows.a.len() as u64),
        ];
        for (name, expected) in fields {
            if integer(summary, name)? != expected {
                return Err(format!("summary {name} differs from independent replay"));
            }
        }
        if self.queries != self.positives + self.negatives + self.unknowns {
            return Err("query classifications do not cover every attempted query".into());
        }
        if optional_scalar(summary, "recovered_scalar")? != self.rows.pinned
            || boolean(summary, "recovery_replayed")? != self.rows.pinned.is_some()
        {
            return Err("summary recovery differs from independent RREF".into());
        }
        let status = field(summary, "status")?
            .as_str()
            .ok_or("invalid summary status")?;
        if !matches!(
            status,
            "recovered" | "query_limit" | "wall_limit" | "oracle_error"
        ) || (status == "query_limit" && self.queries != self.max_queries)
        {
            return Err("summary status is unknown or contradicts its query cap".into());
        }
        if (status == "recovered") != self.rows.pinned.is_some() {
            return Err("summary status contradicts recovery".into());
        }
        if let Some(d) = self.rows.pinned {
            self.scalar_replays(d)?;
        }
        if self.rows.pinned.is_none()
            && self
                .current
                .as_ref()
                .is_some_and(|q| q.consumed != q.witnesses.len())
        {
            return Err("unfinished relation insertion hidden by collection summary".into());
        }
        Ok(())
    }

    fn event(&mut self, value: Value) -> Result<(), String> {
        let event = field(&value, "event")?
            .as_str()
            .ok_or("event is not a string")?;
        if self.cell_summary || self.failure_terminal || self.rho_terminal {
            return Err("records continue after a terminal cell event".into());
        }
        match event {
            "cell_start" => {
                if self.cell.is_some() {
                    return Err("audit accepts exactly one cell per JSONL stream".into());
                }
                let cell = field(&value, "cell")?.clone();
                let target: PublicTarget = decode(&cell, "public_target")?;
                independent_target_check(&target)?;
                self.target = Some(target);
                self.cell = Some(cell);
            }
            "public_target_validation" | "reference_build_start" => {
                self.public_target()?;
            }
            "reference_build_result" => {
                if self.reference.is_some() || self.selected.is_some() {
                    return Err("duplicate or late reference base".into());
                }
                let definition: BaseDefinition = decode(&value, "base")?;
                self.check_base_target(&definition)?;
                bases::verify_base_definition(&definition)?;
                self.check_reference_input(&definition)?;
                self.reference = Some(definition);
            }
            "selected_base" => {
                if self.selected.is_some() || self.stream.is_some() {
                    return Err("duplicate or late selected base".into());
                }
                let definition: BaseDefinition = decode(&value, "base")?;
                self.check_base_target(&definition)?;
                bases::verify_base_definition(&definition)?;
                if let Some(reference) = &self.reference {
                    if definition.columns != reference.columns {
                        return Err("plain/reference cardinalities differ".into());
                    }
                }
                let cell = self.cell.as_ref().unwrap();
                if cell.get("plain_control").is_some() {
                    let reference = self
                        .reference
                        .as_ref()
                        .ok_or("selected arm omits its reference base")?;
                    if boolean(cell, "plain_control")? {
                        if definition.family != BaseFamily::Plain
                            || definition.seed != integer(cell, "plain_seed")?
                        {
                            return Err("plain base differs from frozen arm or seed".into());
                        }
                    } else if serde_json::to_value(&definition).map_err(|e| e.to_string())?
                        != serde_json::to_value(reference).map_err(|e| e.to_string())?
                    {
                        return Err(
                            "selected auxiliary arm is not the exact certified reference base"
                                .into(),
                        );
                    }
                }
                for name in ["matched_columns", "reference_columns"] {
                    if value.get(name).is_some()
                        && integer(&value, name)? != definition.columns as u64
                    {
                        return Err(format!("{name} differs from certified base"));
                    }
                }
                for (column, &q) in definition.canonical_points.iter().enumerate() {
                    let neg = PointDefinition {
                        x: q.x,
                        y: definition.curve.p - q.y,
                        infinity: false,
                    };
                    self.signed_points.extend([q, neg]);
                    self.signed_columns.insert((q.x, q.y), (column, 1));
                    self.signed_columns.insert((neg.x, neg.y), (column, -1));
                }
                self.selected = Some(definition);
            }
            "collection_start" => {
                if self.stream.is_some() {
                    return Err("duplicate collection_start".into());
                }
                let target: PublicTarget = decode(&value, "public_target")?;
                if !target_equal(self.public_target()?, &target) {
                    return Err("collection target differs from cell".into());
                }
                let columns = self.selected_base()?.columns;
                if integer(&value, "columns")? != columns as u64 {
                    return Err("collection columns differ from certified base".into());
                }
                self.arity = integer(&value, "arity")? as usize;
                if !(2..=4).contains(&self.arity) {
                    return Err("unsupported relation arity".into());
                }
                self.presentation = Some(decode(&value, "presentation")?);
                let seed = integer(&value, "query_seed")?;
                let cell = self.cell.as_ref().unwrap();
                for key in ["arity", "query_seed", "presentation"] {
                    if cell.get(key).is_some() && cell[key] != value[key] {
                        return Err(format!("collection {key} differs from cell"));
                    }
                }
                if boolean(&value, "secret_given_to_collector")?
                    || boolean(&value, "planted_query_selection")?
                {
                    return Err(
                        "collector input scope is not the frozen public natural-query contract"
                            .into(),
                    );
                }
                let limits: CollectionLimits = decode(&value, "limits")?;
                if limits.max_queries == 0 || limits.max_seconds == 0 {
                    return Err("collection has nonpositive limits".into());
                }
                if let Some(frozen) = cell.get("collection") {
                    let original: CollectionLimits = serde_json::from_value(frozen.clone())
                        .map_err(|e| format!("invalid frozen collection limits: {e}"))?;
                    if limits.max_queries != original.max_queries
                        || limits.max_seconds > original.max_seconds
                        || serde_json::to_value(&limits.oracle).map_err(|e| e.to_string())?
                            != serde_json::to_value(&original.oracle).map_err(|e| e.to_string())?
                    {
                        return Err("collection limits differ from the frozen cell".into());
                    }
                }
                self.max_queries = limits.max_queries;
                self.stream = Some(PublicStream {
                    state: seed,
                    words: 0,
                });
            }
            "query_start" => {
                if self.collection_summary.is_some() || self.rows.pinned.is_some() {
                    return Err("query starts after recovery or collection summary".into());
                }
                if self.current.as_ref().is_some_and(|q| {
                    q.outcome == QueryOutcome::Pending || q.consumed != q.witnesses.len()
                }) {
                    return Err(
                        "next query discards an unresolved query or uninserted relation".into(),
                    );
                }
                let number = integer(&value, "query")?;
                if number != self.queries + 1 || number > self.max_queries {
                    return Err("query sequence is not consecutive".into());
                }
                let public = self.public_target()?.clone();
                let alpha = integer(&value, "alpha")?;
                let beta = integer(&value, "beta")?;
                let stream = self
                    .stream
                    .as_mut()
                    .ok_or("query before collection_start")?;
                if alpha != stream.scalar(public.order)?
                    || beta != stream.scalar(public.order)?
                    || integer(&value, "random_words")? != stream.words
                {
                    return Err(
                        "query labels differ from independently replayed public seed stream".into(),
                    );
                }
                let actual: PointDefinition = decode(&value, "target")?;
                let e = curve(&public);
                let expected = bases::independent_add(
                    &e,
                    bases::independent_scalar(&e, point(public.generator), alpha),
                    bases::independent_scalar(&e, point(public.target), beta),
                );
                if actual != expected {
                    return Err("query target is not alpha*G + beta*Q".into());
                }
                self.queries += 1;
                self.identities += u64::from(actual.infinity);
                self.current = Some(Query {
                    number,
                    alpha,
                    beta,
                    target: actual,
                    outcome: QueryOutcome::Pending,
                    witnesses: vec![],
                    consumed: 0,
                });
            }
            "query_result" => {
                self.require_current(&value, QueryOutcome::Pending)?;
                let oracle: OracleResult = decode(&value, "oracle")?;
                if oracle.summands != self.arity
                    || Some(oracle.presentation) != self.presentation
                    || oracle.positive != !oracle.relations.is_empty()
                    || oracle.complete
                        != matches!(
                            oracle.status,
                            OracleStatus::CompletePositive | OracleStatus::CompleteNegative
                        )
                    || (oracle.status == OracleStatus::CompletePositive && !oracle.positive)
                    || (oracle.status == OracleStatus::CompleteNegative && oracle.positive)
                    || (oracle.status == OracleStatus::FoundFirst && !oracle.positive)
                {
                    return Err("oracle outcome flags contradict its recorded witnesses".into());
                }
                let target = self.current.as_ref().unwrap().target;
                let mut witnesses = Vec::new();
                let mut unique = BTreeSet::new();
                for relation in oracle.relations {
                    let points: Vec<PointDefinition> = relation
                        .indices
                        .iter()
                        .map(|&i| {
                            self.signed_points
                                .get(i)
                                .copied()
                                .ok_or("oracle signed index outside canonical base")
                        })
                        .collect::<Result<_, _>>()?;
                    self.verify_witness(&points, &relation.row, target)?;
                    if !unique.insert(relation.row.clone()) {
                        return Err(
                            "oracle repeats a coefficient row it claims to deduplicate".into()
                        );
                    }
                    witnesses.push(Witness {
                        row: relation.row,
                        points,
                    });
                }
                self.oracle_witnesses += witnesses.len() as u64;
                let outcome = if !witnesses.is_empty() {
                    self.positives += 1;
                    QueryOutcome::Positive
                } else if oracle.complete {
                    self.negatives += 1;
                    QueryOutcome::Negative
                } else {
                    self.unknowns += 1;
                    QueryOutcome::Unknown
                };
                let current = self.current.as_mut().unwrap();
                current.outcome = outcome;
                current.witnesses = witnesses;
            }
            "query_error" | "query_aborted" => {
                self.require_current(&value, QueryOutcome::Pending)?;
                self.unknowns += 1;
                self.current.as_mut().unwrap().outcome = QueryOutcome::Unknown;
            }
            "relation" => {
                self.require_current(&value, QueryOutcome::Positive)?;
                let query = self.current.as_ref().unwrap();
                if integer(&value, "alpha")? != query.alpha
                    || integer(&value, "beta")? != query.beta
                {
                    return Err("relation labels differ from its natural query".into());
                }
                let row: Vec<i64> = decode(&value, "row")?;
                let signed: Vec<PointDefinition> = decode(&value, "signed_points")?;
                self.verify_witness(&signed, &row, query.target)?;
                let expected = query
                    .witnesses
                    .get(query.consumed)
                    .ok_or("relation not present in preceding oracle result")?;
                if expected.row != row || expected.points != signed {
                    return Err("inserted witness differs from oracle output order".into());
                }
                let (alpha, beta, n) = (query.alpha, query.beta, self.public_target()?.order);
                self.rows.insert(&row, alpha, beta, n)?;
                if integer(&value, "rank_a")? != self.rows.rank_a as u64
                    || integer(&value, "rank_augmented")? != self.rows.rank_augmented as u64
                    || optional_scalar(&value, "candidate")? != self.rows.pinned
                {
                    return Err(
                        "relation rank or target pin differs from independent batch RREF".into(),
                    );
                }
                if let Some(d) = self.rows.pinned {
                    self.scalar_replays(d)?;
                }
                self.current.as_mut().unwrap().consumed += 1;
            }
            "collection_summary" => {
                if self.collection_summary.is_some() || self.stream.is_none() {
                    return Err("duplicate or premature collection summary".into());
                }
                let summary = field(&value, "summary")?.clone();
                self.compare_summary(&summary)?;
                self.collection_summary = Some(summary);
            }
            "cell_summary" => {
                self.public_target()?;
                if let Some(summary) = &self.collection_summary {
                    if field(&value, "collection")? != summary
                        || boolean(&value, "recovered")? != self.rows.pinned.is_some()
                        || field(&value, "status")? != field(summary, "status")?
                    {
                        return Err("cell summary differs from checked collection summary".into());
                    }
                } else if field(&value, "status")?.as_str() != Some("setup_exhausted_deadline")
                    || boolean(&value, "recovered")?
                    || self.stream.is_some()
                {
                    return Err("successful cell summary has no checked collection summary".into());
                }
                self.cell_summary = true;
            }
            "setup_failure" | "fatal_error" => {
                self.public_target()?;
                self.failure_terminal = true;
            }
            "rho_summary" => {
                if self.stream.is_some() || self.selected.is_some() {
                    return Err("rho summary mixed with an index-calculus cell".into());
                }
                let result = field(&value, "result")?;
                self.rho_scalar = optional_scalar(result, "recovered")?;
                if let Some(d) = self.rho_scalar {
                    self.scalar_replays(d)?;
                }
                if boolean(&value, "independent_replay")? != self.rho_scalar.is_some() {
                    return Err("rho replay flag differs from scalar verification".into());
                }
                self.rho_terminal = true;
            }
            _ => return Err(format!("unsupported event {event}")),
        }
        Ok(())
    }

    fn check_base_target(&self, base: &BaseDefinition) -> Result<(), String> {
        let public = self.public_target()?;
        if base.curve != curve(public) || base.subgroup_order != public.order {
            return Err("base curve or subgroup differs from the frozen public target".into());
        }
        Ok(())
    }
    fn check_reference_input(&self, base: &BaseDefinition) -> Result<(), String> {
        let cell = self.cell.as_ref().ok_or("reference before cell_start")?;
        if cell.get("reference_family").is_some()
            && decode::<BaseFamily>(cell, "reference_family")? != base.family
        {
            return Err("reference family differs from the frozen cell".into());
        }
        for (key, actual) in [("depth", base.depth as u64), ("base_seed", base.seed)] {
            if cell.get(key).is_some() && integer(cell, key)? != actual {
                return Err(format!("reference {key} differs from the frozen cell"));
            }
        }
        Ok(())
    }

    fn require_current(&self, value: &Value, outcome: QueryOutcome) -> Result<(), String> {
        if self.collection_summary.is_some() {
            return Err("query event follows collection summary".into());
        }
        let q = self
            .current
            .as_ref()
            .ok_or("query event without query_start")?;
        if integer(value, "query")? != q.number || q.outcome != outcome {
            return Err("query event has wrong identifier or event order".into());
        }
        Ok(())
    }
}

/// Replay one complete or interrupted native cell. An unterminated malformed
/// final line is treated as a killed writer's partial fragment, never as a
/// successful record. Malformed newline-terminated records are errors.
pub fn audit_jsonl(mut reader: impl BufRead) -> Result<Value, String> {
    let mut state = State::default();
    let mut buffer = String::new();
    let mut lines = 0u64;
    let mut records = 0u64;
    let mut trailing_fragment = false;
    loop {
        buffer.clear();
        let bytes = reader.read_line(&mut buffer).map_err(|e| e.to_string())?;
        if bytes == 0 {
            break;
        }
        lines += 1;
        if bytes > 64 * 1024 * 1024 {
            return Err(format!(
                "line {lines}: record exceeds the 64 MiB replay limit"
            ));
        }
        if buffer.trim().is_empty() {
            continue;
        }
        let value = match serde_json::from_str::<Value>(&buffer) {
            Ok(value) => value,
            Err(_) if !buffer.ends_with('\n') => {
                trailing_fragment = true;
                break;
            }
            Err(error) => return Err(format!("line {lines}: invalid JSON: {error}")),
        };
        state
            .event(value)
            .map_err(|e| format!("line {lines}: {e}"))?;
        records += 1;
    }
    if state.cell.is_none() {
        return Err("no cell_start record to audit".into());
    }
    let pending = state
        .current
        .as_ref()
        .is_some_and(|q| q.outcome == QueryOutcome::Pending);
    let terminal = state.cell_summary || state.failure_terminal || state.rho_terminal;
    let stream_complete = terminal && !trailing_fragment;
    let recovered = if state.rho_terminal {
        state.rho_scalar
    } else {
        state.rows.pinned
    };
    if let Some(d) = recovered {
        state.scalar_replays(d)?;
    }
    let success_summary =
        state.rho_terminal || (state.cell_summary && state.collection_summary.is_some());
    let complete = stream_complete
        && success_summary
        && recovered.is_some()
        && !state.failure_terminal
        && !pending;
    Ok(json!({
        "schema":"prime31-aux-matched.offline-audit/v1", "audit_valid":true,
        "complete":complete, "stream_complete":stream_complete,
        "terminal_record_present":terminal, "trailing_partial_json":trailing_fragment,
        "records_checked":records, "lines_read":lines,
        "queries_replayed":state.queries, "identity_queries":state.identities,
        "positive_queries":state.positives, "reported_complete_negative_queries":state.negatives,
        "unknown_queries":state.unknowns, "pending_query":pending,
        "oracle_witnesses_replayed":state.oracle_witnesses,
        "accepted_rows_replayed":state.rows.a.len(), "rank_a":state.rows.rank_a,
        "rank_augmented":state.rows.rank_augmented,
        "duplicate_coefficient_rows":state.rows.duplicates,
        "useful_dependencies":state.rows.useful_dependencies,
        "recovered_scalar":recovered, "public_scalar_equation_verified":recovered.is_some(),
        "collection_summary_checked":state.collection_summary.is_some(),
        "negative_exhaustion_independently_replayed":false,
        "performance_independently_validated":false,
        "scope":"Independent whole-fiber certificate, frozen public query-stream replay, signed-point and column-row checks, batch modular RREF and public scalar verification. Negative F4 exhaustion, counters, timings, execution provenance and signed PCC standing require separate evidence. complete means a finished recovered cell, not enumeration of all decompositions."
    }))
}
