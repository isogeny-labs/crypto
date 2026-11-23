//! Public-target relation collection. Neither this API nor the oracle receives
//! a target logarithm. Every query is formed before decomposition as aG+bQ.

use std::collections::HashSet;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::cryptanalysis::ic_boundary::{
    CountedGroup, FactorBase, GroupOps, IncrementalGauss, PrimeCurve, PrimePoint, RowStatus,
};
use crate::oracle::{decompose, OracleLimits, Presentation, TowerDescription};
use crate::replay;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicTarget {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub generator: replay::Point,
    pub target: replay::Point,
}

impl PublicTarget {
    pub fn curve(&self) -> PrimeCurve {
        PrimeCurve {
            p: self.p,
            a: self.a,
            b: self.b,
        }
    }
    pub fn replay_curve(&self) -> replay::Curve {
        replay::Curve {
            p: self.p,
            a: self.a,
            b: self.b,
        }
    }
    /// Prime-order target certificate. For small correctness fixtures, count
    /// the curve independently; for Prime31, Hasse plus nG=O establishes #E=n.
    pub fn validate(&self, ops: &mut GroupOps) -> Result<(), String> {
        let e = self.curve();
        let independent = self.replay_curve();
        if self.order >= (1u64 << 32)
            || self.order == self.p
            || !independent.valid_parameters()
            || !replay::prime(self.p)
            || !replay::prime(self.order)
        {
            return Err("invalid prime-field or prime-order parameters".into());
        }
        if self.generator.infinity
            || self.target.infinity
            || !independent.on_curve(self.generator)
            || !independent.on_curve(self.target)
        {
            return Err("generator and target must be finite canonical curve points".into());
        }
        for q in [self.generator, self.target] {
            if !e.mul(ops, from_replay(q), self.order).infinity
                || independent.scalar(q, self.order)? != replay::Point::IDENTITY
            {
                return Err("point has wrong subgroup order".into());
            }
        }
        let root = self.p.isqrt();
        let upper = self.p + 1 + 2 * (root + u64::from(root * root < self.p));
        if 2 * self.order <= upper {
            if self.p > 4096 {
                return Err("cofactor-one certificate does not follow from Hasse bound".into());
            }
            let mut cardinality = 1;
            for x in 0..self.p {
                for y in 0..self.p {
                    cardinality += u64::from(independent.on_curve(replay::Point::affine(x, y)));
                }
            }
            if cardinality != self.order {
                return Err("small fixture is not cofactor one".into());
            }
        }
        Ok(())
    }
}

pub fn from_replay(p: replay::Point) -> PrimePoint {
    PrimePoint {
        x: p.x,
        y: p.y,
        infinity: p.infinity,
    }
}
pub fn to_replay(p: PrimePoint) -> replay::Point {
    if p.infinity {
        replay::Point::IDENTITY
    } else {
        replay::Point::affine(p.x, p.y)
    }
}
pub fn group_ledger(ops: &GroupOps) -> Value {
    json!({"group":ops,"field":ops.field_ops,"montgomery_reductions":ops.montgomery_reductions})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionLimits {
    pub max_queries: u64,
    pub max_seconds: u64,
    pub oracle: OracleLimits,
}

#[derive(Clone, Debug, Serialize)]
pub struct CollectionSummary {
    pub status: String,
    pub queries: u64,
    pub identity_queries: u64,
    pub positive_queries: u64,
    pub complete_negative_queries: u64,
    pub unknown_queries: u64,
    pub accepted_rows: u64,
    pub duplicate_coefficient_rows: u64,
    pub rank_a: usize,
    pub rank_augmented: usize,
    pub useful_dependencies: u64,
    pub recovered_scalar: Option<u64>,
    pub recovery_replayed: bool,
    pub elapsed_seconds_diagnostic: f64,
    pub group_and_field: Value,
    pub rank_a_work: Value,
    pub augmented_work: Value,
    pub independent_replay_rows: u64,
    pub oracle_counters: Value,
    pub error: Option<String>,
}

/// SplitMix stream with rejection sampling: unlike `% n`, the accepted output
/// is exactly uniform if the source word is uniform. Every arm uses this same
/// fixed public seed, including query attempts that return no relation.
pub struct QueryStream {
    state: u64,
    pub words: u64,
}
impl QueryStream {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed,
            words: 0,
        }
    }
    fn word(&mut self) -> u64 {
        self.words += 1;
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn scalar(&mut self, n: u64) -> u64 {
        assert!(n > 1);
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.word();
            if x >= threshold {
                return x % n;
            }
        }
    }
}

/// Accumulates A*l = alpha + beta*d. The last unknown is d, so RREF on
/// [A|-beta] is enough to discover useful left dependencies without ever
/// knowing a factor-base logarithm. Duplicate A rows must still be inserted.
pub struct Recovery {
    a: IncrementalGauss,
    augmented: IncrementalGauss,
    rows: HashSet<Vec<u64>>,
    n: u64,
    c: usize,
    pub duplicates: u64,
    pub useful_dependencies: u64,
}
impl Recovery {
    pub fn new(c: usize, n: u64) -> Self {
        Self {
            a: IncrementalGauss::new(c, n),
            augmented: IncrementalGauss::new(c + 1, n),
            rows: HashSet::new(),
            n,
            c,
            duplicates: 0,
            useful_dependencies: 0,
        }
    }
    pub fn insert(&mut self, row: &[i64], alpha: u64, beta: u64) -> Result<Option<u64>, String> {
        if row.len() != self.c || alpha >= self.n || beta >= self.n {
            return Err("malformed modular relation".into());
        }
        let a: Vec<u64> = row
            .iter()
            .map(|&v| i128::from(v).rem_euclid(i128::from(self.n)) as u64)
            .collect();
        if !self.rows.insert(a.clone()) {
            self.duplicates += 1;
        }
        let before = self.augmented.pinned(self.c);
        let rank_status = self.a.add_row(a.clone(), 0);
        let mut aug = a;
        aug.push((self.n - beta) % self.n);
        if self.augmented.add_row(aug, alpha) == RowStatus::Inconsistent {
            return Err("replayed rows became inconsistent modulo the subgroup order".into());
        }
        let after = self.augmented.pinned(self.c);
        if rank_status == RowStatus::Dependent && before.is_none() && after.is_some() {
            self.useful_dependencies += 1;
        }
        Ok(after)
    }
    pub fn rank_a(&self) -> usize {
        self.a.rank()
    }
    pub fn rank_augmented(&self) -> usize {
        self.augmented.rank()
    }
    pub fn work(&self) -> (Value, Value) {
        let snapshot = |g: &IncrementalGauss| {
            json!({"row_mul_subs":g.row_ops,"dependent":g.dependent,
            "inconsistent":g.inconsistent,"other_counters":g.work_counters()})
        };
        (snapshot(&self.a), snapshot(&self.augmented))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn collect(
    target: &PublicTarget,
    fb: &FactorBase<PrimePoint>,
    values: &[u64],
    tower: Option<&TowerDescription>,
    arity: usize,
    presentation: Presentation,
    query_seed: u64,
    limits: &CollectionLimits,
    emit: &mut dyn FnMut(&Value) -> Result<(), String>,
) -> Result<CollectionSummary, String> {
    if limits.max_queries == 0 || limits.max_seconds == 0 || !(2..=4).contains(&arity) {
        return Err("nonpositive collection cap or unsupported arity".into());
    }
    let curve = target.curve();
    let verifier = target.replay_curve();
    let g = from_replay(target.generator);
    let q = from_replay(target.target);
    let n = target.order;
    let start = Instant::now();
    let mut ops = GroupOps::default();
    target.validate(&mut ops)?;
    let mut stream = QueryStream::new(query_seed);
    let mut recovery = Recovery::new(fb.columns, n);
    let mut out = CollectionSummary {
        status: "query_limit".into(),
        queries: 0,
        identity_queries: 0,
        positive_queries: 0,
        complete_negative_queries: 0,
        unknown_queries: 0,
        accepted_rows: 0,
        duplicate_coefficient_rows: 0,
        rank_a: 0,
        rank_augmented: 0,
        useful_dependencies: 0,
        recovered_scalar: None,
        recovery_replayed: false,
        elapsed_seconds_diagnostic: 0.,
        group_and_field: Value::Null,
        rank_a_work: Value::Null,
        augmented_work: Value::Null,
        independent_replay_rows: 0,
        oracle_counters: json!({}),
        error: None,
    };
    emit(
        &json!({"event":"collection_start","public_target":target,"query_seed":query_seed,
        "arity":arity,"presentation":presentation,"limits":limits,"columns":fb.columns,
        "secret_given_to_collector":false,"planted_query_selection":false}),
    )?;
    'queries: while out.queries < limits.max_queries {
        if start.elapsed().as_secs() >= limits.max_seconds {
            out.status = "wall_limit".into();
            break;
        }
        let alpha = stream.scalar(n);
        let beta = stream.scalar(n);
        let ag = curve.mul(&mut ops, g, alpha);
        let bq = curve.mul(&mut ops, q, beta);
        let r = curve.add(&mut ops, ag, bq);
        let independent_r = verifier.add(
            verifier.scalar(target.generator, alpha)?,
            verifier.scalar(target.target, beta)?,
        )?;
        if independent_r != to_replay(r) {
            return Err("independent natural-query replay mismatch".into());
        }
        out.queries += 1;
        out.identity_queries += u64::from(r.infinity);
        // Flush before entering F4: a killed process still records the query
        // it attempted instead of silently deleting its most expensive sample.
        emit(
            &json!({"event":"query_start","query":out.queries,"alpha":alpha,"beta":beta,
            "target":to_replay(r),"random_words":stream.words}),
        )?;
        let mut oracle_limits = limits.oracle.clone();
        let remaining_ms =
            (limits.max_seconds as f64 - start.elapsed().as_secs_f64()).max(0.) * 1000.;
        if remaining_ms < 1. {
            out.unknown_queries += 1;
            emit(
                &json!({"event":"query_aborted","query":out.queries,"reason":"collection_deadline_before_oracle"}),
            )?;
            out.status = "wall_limit".into();
            break;
        }
        oracle_limits.timeout_ms = oracle_limits.timeout_ms.min(remaining_ms as u64);
        let oracle = match decompose(
            &curve,
            n,
            fb,
            values,
            tower,
            r,
            arity,
            presentation,
            &oracle_limits,
            &mut ops,
        ) {
            Ok(v) => v,
            Err(error) => {
                out.unknown_queries += 1;
                emit(&json!({"event":"query_error","query":out.queries,"error":error}))?;
                out.status = "oracle_error".into();
                out.error = Some(error);
                break;
            }
        };
        emit(&json!({"event":"query_result","query":out.queries,"oracle":oracle}))?;
        let counters = serde_json::to_value(&oracle.counters).map_err(|e| e.to_string())?;
        for (key, value) in counters
            .as_object()
            .ok_or("oracle counters must be an object")?
        {
            let value = value
                .as_u64()
                .ok_or("oracle counter must be an unsigned integer")?;
            let before = out.oracle_counters[key].as_u64().unwrap_or(0);
            let total = if matches!(key.as_str(), "solving_degree" | "max_rows" | "max_columns") {
                before.max(value)
            } else {
                before.checked_add(value).ok_or("oracle counter overflow")?
            };
            out.oracle_counters[key] = json!(total);
        }
        if oracle.relations.is_empty() {
            if oracle.complete {
                out.complete_negative_queries += 1;
            } else {
                out.unknown_queries += 1;
            }
        } else {
            out.positive_queries += 1;
        }
        for relation in &oracle.relations {
            if relation.indices.len() != arity || relation.row.len() != fb.columns {
                return Err("oracle returned malformed signed witness".into());
            }
            let mut row = vec![0i64; fb.columns];
            let mut signed = Vec::with_capacity(arity);
            for &i in &relation.indices {
                let point = *fb
                    .points
                    .get(i)
                    .ok_or("signed point index outside factor base")?;
                let coefficient = *fb.coef_of.get(i).ok_or("missing signed coefficient")?;
                let column = *fb.col_of.get(i).ok_or("missing signed column")?;
                let sign = if coefficient == 1 {
                    1
                } else if coefficient == n - 1 {
                    -1
                } else {
                    return Err("factor base coefficient is not a sign".into());
                };
                *row.get_mut(column)
                    .ok_or("signed column outside factor base")? += sign;
                signed.push(to_replay(point));
            }
            if row != relation.row || verifier.sum(signed.iter().copied())? != independent_r {
                return Err("independent signed relation replay mismatch".into());
            }
            out.independent_replay_rows += 1;
            let candidate = recovery.insert(&row, alpha, beta)?;
            out.accepted_rows += 1;
            emit(
                &json!({"event":"relation","query":out.queries,"row":row,"signed_points":signed,
                "alpha":alpha,"beta":beta,"rank_a":recovery.rank_a(),
                "rank_augmented":recovery.rank_augmented(),"candidate":candidate,"independent_replay":true}),
            )?;
            if let Some(d) = candidate {
                if curve.mul(&mut ops, g, d) != q
                    || verifier.scalar(target.generator, d)? != target.target
                {
                    return Err("candidate scalar failed independent public-point replay".into());
                }
                out.recovered_scalar = Some(d);
                out.recovery_replayed = true;
                out.status = "recovered".into();
                break 'queries;
            }
        }
    }
    out.rank_a = recovery.rank_a();
    out.rank_augmented = recovery.rank_augmented();
    out.duplicate_coefficient_rows = recovery.duplicates;
    out.useful_dependencies = recovery.useful_dependencies;
    (out.rank_a_work, out.augmented_work) = recovery.work();
    out.group_and_field = group_ledger(&ops);
    out.elapsed_seconds_diagnostic = start.elapsed().as_secs_f64();
    emit(&json!({"event":"collection_summary","summary":out}))?;
    Ok(out)
}
