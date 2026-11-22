//! Exact finite-field S3 relation extraction on every sequential-prefix chart.
//! The membership set is the complete V in both presentations. Original point
//! signs and every finite/infinite prefix are replayed before accepting a row.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crate::{CountedGroup, FactorBase, GroupOps, PrimeCurve, PrimePoint};

// The inherited engine exposes process-wide diagnostic counters. Serialize
// study calls so independent study queries do not contaminate their deltas.
static ORACLE_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TowerDescription {
    Velu {
        steps: Vec<(u64, u64)>,
        terminal: u64,
    },
    Dickson {
        depth: usize,
        shift: u64,
        terminal: u64,
    },
}

impl TowerDescription {
    pub fn depth(&self) -> usize {
        match self {
            Self::Velu { steps, .. } => steps.len(),
            Self::Dickson { depth, .. } => *depth,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Presentation {
    Dense,
    Tower,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OracleLimits {
    pub max_degree: u32,
    pub max_pairs_per_step: usize,
    /// Cooperative deadline shared by every chart in one public query.
    pub timeout_ms: u64,
    pub max_solutions: usize,
    pub max_signed_lift_attempts: u64,
    pub stop_after_first: bool,
    pub max_variables: usize,
    pub max_input_terms: usize,
}
impl Default for OracleLimits {
    fn default() -> Self {
        Self {
            max_degree: 8,
            max_pairs_per_step: 32,
            timeout_ms: 2000,
            max_solutions: 4096,
            max_signed_lift_attempts: 1_000_000,
            stop_after_first: true,
            max_variables: 32,
            max_input_terms: 100_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleStatus {
    CompletePositive,
    CompleteNegative,
    FoundFirst,
    UnknownLimit,
    UnknownTimeout,
    UnknownError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedRelation {
    /// Indices into fb.points, which already contains both point signs.
    pub indices: Vec<usize>,
    /// Exact integer multiplicities of canonical columns; reduce modulo r
    /// when inserting a logarithm equation. Zero rows remain visible.
    pub row: Vec<i64>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OracleCounters {
    pub validation_field_multiplications: u64,
    pub polynomial_build_multiplications: u64,
    pub original_equation_check_multiplications: u64,
    pub f4_field_operations: u64,
    pub root_field_multiplications: u64,
    pub domain_evaluations: u64,
    pub full_field_branches: u64,
    pub f4_runs: usize,
    pub coordinate_solutions: usize,
    pub signed_lift_attempts: u64,
    pub rejected_coordinate_solutions: usize,
    pub duplicate_relation_rows: usize,
    pub zero_relation_rows: usize,
    pub solving_degree: u32,
    pub max_rows: usize,
    pub max_columns: usize,
    /// Incomplete lower-bound diagnostics from the production F4 driver.
    pub symbolic_native_operations: u64,
    pub normalization_operations: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChartReport {
    pub chart_mask: usize,
    pub prefix_infinity: Vec<bool>,
    pub variables: usize,
    pub equations: usize,
    pub input_terms: usize,
    pub status: String,
    pub visit: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OracleResult {
    pub status: OracleStatus,
    pub complete: bool,
    pub positive: bool,
    pub presentation: Presentation,
    pub summands: usize,
    pub relations: Vec<SignedRelation>,
    pub charts: Vec<ChartReport>,
    pub counters: OracleCounters,
    pub elapsed_ns: u64,
    pub polynomial_build_ns: u64,
    pub reason: Option<String>,
    pub accounting: String,
}

#[derive(Clone, Copy)]
enum Coordinate {
    Variable(usize),
    Constant(u64),
    Infinity,
}

fn mul(a: u64, b: u64, p: u64, counts: &mut OracleCounters) -> u64 {
    counts.polynomial_build_multiplications += 1;
    ((a as u128 * b as u128) % u128::from(p)) as u64
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}
fn scalar_poly(c: u64, n: usize) -> Poly {
    if c == 0 {
        vec![]
    } else {
        vec![(vec![0; n], c)]
    }
}
fn variable(i: usize, n: usize) -> Poly {
    let mut e = vec![0; n];
    e[i] = 1;
    vec![(e, 1)]
}
fn add_poly(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut out = BTreeMap::<Vec<u32>, u64>::new();
    for (e, c) in a.iter().chain(b) {
        let x = out.entry(e.clone()).or_default();
        *x = (*x + *c) % p;
    }
    out.into_iter().filter(|(_, c)| *c != 0).collect()
}
fn scale_poly(a: &Poly, c: u64, p: u64, counts: &mut OracleCounters) -> Poly {
    a.iter()
        .filter_map(|(e, v)| {
            let v = mul(*v, c, p, counts);
            (v != 0).then(|| (e.clone(), v))
        })
        .collect()
}
fn mul_poly(a: &Poly, b: &Poly, p: u64, counts: &mut OracleCounters) -> Poly {
    let mut out = BTreeMap::<Vec<u32>, u64>::new();
    for (ae, ac) in a {
        for (be, bc) in b {
            let e = ae.iter().zip(be).map(|(x, y)| x + y).collect();
            let v = out.entry(e).or_default();
            *v = (*v + mul(*ac, *bc, p, counts)) % p;
        }
    }
    out.into_iter().filter(|(_, c)| *c != 0).collect()
}
fn coordinate_poly(c: Coordinate, n: usize) -> Poly {
    match c {
        Coordinate::Variable(i) => variable(i, n),
        Coordinate::Constant(x) => scalar_poly(x, n),
        Coordinate::Infinity => panic!("infinity is a chart, not an affine variable"),
    }
}
fn equality(a: Coordinate, b: Coordinate, n: usize, p: u64, counts: &mut OracleCounters) -> Poly {
    add_poly(
        &coordinate_poly(a, n),
        &scale_poly(&coordinate_poly(b, n), p - 1, p, counts),
        p,
    )
}

/// The classical exact S3 expression, assembled over the production F4 Poly
/// format. Unit tests compare it to the repository's independent numeric S3.
fn s3(
    curve: &PrimeCurve,
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    n: usize,
    counts: &mut OracleCounters,
) -> Poly {
    let p = curve.p;
    let a2 = mul(curve.a, curve.a, p, counts);
    let na2 = sub(0, (2 * curve.a) % p, p);
    let nb4 = sub(0, (4 * curve.b) % p, p);
    let terms = [
        ([2, 2, 0], 1),
        ([2, 0, 2], 1),
        ([0, 2, 2], 1),
        ([2, 1, 1], p - 2),
        ([1, 2, 1], p - 2),
        ([1, 1, 2], p - 2),
        ([1, 1, 0], na2),
        ([1, 0, 1], na2),
        ([0, 1, 1], na2),
        ([1, 0, 0], nb4),
        ([0, 1, 0], nb4),
        ([0, 0, 1], nb4),
        ([0, 0, 0], a2),
    ];
    let mut out = BTreeMap::<Vec<u32>, u64>::new();
    for (powers, mut coefficient) in terms {
        let mut exponent = vec![0; n];
        for (coordinate, power) in [x, y, z].into_iter().zip(powers) {
            match coordinate {
                Coordinate::Variable(i) => exponent[i] += power,
                Coordinate::Constant(v) => {
                    for _ in 0..power {
                        coefficient = mul(coefficient, v, p, counts)
                    }
                }
                Coordinate::Infinity => panic!("S3 cannot represent infinity"),
            }
        }
        let c = out.entry(exponent).or_default();
        *c = (*c + coefficient) % p;
    }
    out.into_iter().filter(|(_, c)| *c != 0).collect()
}

fn dense_membership(
    values: &[u64],
    i: usize,
    n: usize,
    p: u64,
    counts: &mut OracleCounters,
) -> Poly {
    let mut c = vec![1];
    for &x in values {
        let mut next = vec![0; c.len() + 1];
        for (j, &a) in c.iter().enumerate() {
            next[j] = sub(next[j], mul(x, a, p, counts), p);
            next[j + 1] = (next[j + 1] + a) % p;
        }
        c = next;
    }
    c.into_iter()
        .enumerate()
        .filter_map(|(d, c)| {
            let mut e = vec![0; n];
            e[i] = d as u32;
            (c != 0).then_some((e, c))
        })
        .collect()
}

struct ChartSystem {
    equations: Vec<Poly>,
    n_vars: usize,
    domains: Vec<Option<Vec<u64>>>,
    prefixes: Vec<Coordinate>,
}

fn build_chart(
    curve: &PrimeCurve,
    values: &[u64],
    tower: Option<&TowerDescription>,
    target: PrimePoint,
    m: usize,
    presentation: Presentation,
    mask: usize,
    limits: &OracleLimits,
    counts: &mut OracleCounters,
) -> Result<Option<ChartSystem>, String> {
    let depth = if presentation == Presentation::Tower {
        tower.ok_or("tower presentation requires a tower")?.depth()
    } else {
        1
    };
    if depth == 0 {
        return Err("tower depth must be positive".into());
    }
    let mut n = m.checked_mul(depth).ok_or("variable count overflow")?;
    let mut prefixes = Vec::with_capacity(m.saturating_sub(2));
    for j in 0..m - 2 {
        if mask & (1 << j) != 0 {
            prefixes.push(Coordinate::Infinity)
        } else {
            prefixes.push(Coordinate::Variable(n));
            n += 1;
        }
    }
    if n > limits.max_variables {
        return Err(format!(
            "input variable cap: {n} > {}",
            limits.max_variables
        ));
    }
    let p = curve.p;
    let mut equations = Vec::new();
    for i in 0..m {
        if presentation == Presentation::Dense {
            equations.push(dense_membership(values, i, n, p, counts));
        } else {
            for j in 0..depth {
                let here = variable(j * m + i, n);
                let next = if j + 1 < depth {
                    variable((j + 1) * m + i, n)
                } else {
                    scalar_poly(
                        match tower.unwrap() {
                            TowerDescription::Velu { terminal, .. }
                            | TowerDescription::Dickson { terminal, .. } => *terminal,
                        },
                        n,
                    )
                };
                match tower.unwrap() {
                    TowerDescription::Velu { steps, .. } => {
                        let (xi, tau) = steps[j];
                        if tau == 0 {
                            return Err("singular/reducible zero-tau tower step".into());
                        }
                        let h2 = mul_poly(&here, &here, p, counts);
                        let neg_next = scale_poly(&next, p - 1, p, counts);
                        let linear = add_poly(&neg_next, &scalar_poly(sub(0, xi, p), n), p);
                        let product = mul_poly(&linear, &here, p, counts);
                        let remainder =
                            add_poly(&scale_poly(&next, xi, p, counts), &scalar_poly(tau, n), p);
                        equations.push(add_poly(&add_poly(&h2, &product, p), &remainder, p));
                    }
                    TowerDescription::Dickson { shift, .. } => {
                        let y = if j == 0 {
                            add_poly(&here, &scalar_poly(sub(0, *shift, p), n), p)
                        } else {
                            here
                        };
                        let h2 = mul_poly(&y, &y, p, counts);
                        let rest = add_poly(
                            &scale_poly(&next, p - 1, p, counts),
                            &scalar_poly(p - 2, n),
                            p,
                        );
                        equations.push(add_poly(&h2, &rest, p));
                    }
                }
            }
        }
    }
    let mut previous = Coordinate::Variable(0);
    for i in 1..m {
        let out = if i + 1 == m {
            if target.infinity {
                Coordinate::Infinity
            } else {
                Coordinate::Constant(target.x)
            }
        } else {
            prefixes[i - 1]
        };
        let leaf = Coordinate::Variable(i);
        match (previous, out) {
            (Coordinate::Infinity, Coordinate::Infinity) => return Ok(None),
            (Coordinate::Infinity, _) => equations.push(equality(out, leaf, n, p, counts)),
            (_, Coordinate::Infinity) => equations.push(equality(previous, leaf, n, p, counts)),
            _ => equations.push(s3(curve, previous, leaf, out, n, counts)),
        }
        previous = out;
    }
    let terms = equations.iter().map(Vec::len).sum::<usize>();
    if terms > limits.max_input_terms {
        return Err(format!(
            "input term cap: {terms} > {}",
            limits.max_input_terms
        ));
    }
    let mut domains = vec![None; n];
    for d in domains.iter_mut().take(m) {
        *d = Some(values.to_vec());
    }
    Ok(Some(ChartSystem {
        equations,
        n_vars: n,
        domains,
        prefixes,
    }))
}

fn poly_eval(poly: &Poly, point: &[u64], p: u64, count: &mut u64) -> u64 {
    let mut answer = 0;
    for (powers, &coefficient) in poly.iter().map(|(e, c)| (e, c)) {
        let mut v = coefficient;
        for (&e, &x) in powers.iter().zip(point) {
            for _ in 0..e {
                v = counted_product(v, x, p, count);
            }
        }
        answer = (answer + v) % p;
    }
    answer
}

fn counted_product(a: u64, b: u64, p: u64, count: &mut u64) -> u64 {
    *count += 1;
    ((a as u128 * b as u128) % p as u128) as u64
}

fn counted_power(mut x: u64, mut exponent: u64, p: u64, count: &mut u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = counted_product(result, x, p, count);
        }
        exponent >>= 1;
        if exponent != 0 {
            x = counted_product(x, x, p, count);
        }
    }
    result
}

fn validate_factor_base(
    curve: &PrimeCurve,
    order: u64,
    fb: &FactorBase<PrimePoint>,
    values: &[u64],
) -> Result<(), String> {
    if fb.points.len() != fb.col_of.len()
        || fb.points.len() != fb.coef_of.len()
        || fb.col_of.iter().any(|&c| c >= fb.columns)
        || fb.coef_of.iter().any(|&c| c != 1 && c != order - 1)
        || fb.points.iter().any(|&q| {
            q.infinity
                || q.y == 0
                || q.y >= curve.p
                || !curve.is_on_curve(q)
                || values.binary_search(&q.x).is_err()
        })
    {
        return Err("factor base must contain valid finite signed +/-1 columns from V".into());
    }
    let expected = FactorBase::from_column_map(
        fb.description.clone(),
        fb.points.clone(),
        fb.col_of.clone(),
        fb.coef_of.clone(),
        fb.columns,
        |q| curve.key(q),
        |q| curve.key(&curve.neg(*q)),
        |q| q.x,
    )?;
    if expected.x_index != fb.x_index
        || expected.point_index != fb.point_index
        || expected.neg_index != fb.neg_index
        || expected.abscissae != fb.abscissae
        || expected.abscissa_list != fb.abscissa_list
        || fb.columns != expected.abscissae
        || fb.points.len() != fb.columns.saturating_mul(2)
    {
        return Err(
            "factor-base signed point inventory, derived index, or column count mismatch".into(),
        );
    }
    let mut canonical = vec![None; fb.columns];
    for (i, &point) in fb.points.iter().enumerate() {
        let positive = point.y < curve.p - point.y;
        if (fb.coef_of[i] == 1) != positive || fb.col_of[i] != fb.col_of[fb.neg_index[i]] {
            return Err(
                "factor-base column signs must use the smaller canonical y and its negative".into(),
            );
        }
        if positive && canonical[fb.col_of[i]].replace(point).is_some() {
            return Err("multiple canonical points occupy a factor-base column".into());
        }
    }
    if canonical.iter().any(Option::is_none) {
        return Err("factor-base column has no canonical point".into());
    }
    Ok(())
}

fn validate_tower_values(
    curve: &PrimeCurve,
    values: &[u64],
    tower: &TowerDescription,
    count: &mut u64,
) -> Result<(), String> {
    for &leaf in values {
        let (result, terminal) = match tower {
            TowerDescription::Velu { steps, terminal } => {
                let mut x = leaf;
                for &(xi, tau) in steps {
                    let denominator = sub(x, xi, curve.p);
                    if denominator == 0 {
                        return Err("V contains a pole of the retained tower".into());
                    }
                    let inverse = counted_power(denominator, curve.p - 2, curve.p, count);
                    x = (x + counted_product(tau, inverse, curve.p, count)) % curve.p;
                }
                (x, *terminal)
            }
            TowerDescription::Dickson {
                depth,
                shift,
                terminal,
            } => {
                let mut x = sub(leaf, *shift, curve.p);
                for _ in 0..*depth {
                    x = sub(counted_product(x, x, curve.p, count), 2, curve.p);
                }
                (x, *terminal)
            }
        };
        if result != terminal {
            return Err("V is not the complete retained-tower fibre".into());
        }
    }
    Ok(())
}

/// Public diagnostic for independently checking original equations and chart
/// coverage without passing any reference witness to the solver.
pub fn equations_for_chart(
    curve: &PrimeCurve,
    values: &[u64],
    tower: Option<&TowerDescription>,
    target: PrimePoint,
    m: usize,
    presentation: Presentation,
    mask: usize,
    limits: &OracleLimits,
) -> Result<Option<(Vec<Poly>, usize)>, String> {
    if !(2..=4).contains(&m) || mask >= (1usize << (m - 2)) {
        return Err("invalid m or chart".into());
    }
    Ok(build_chart(
        curve,
        values,
        tower,
        target,
        m,
        presentation,
        mask,
        limits,
        &mut OracleCounters::default(),
    )?
    .map(|c| (c.equations, c.n_vars)))
}

struct LiftContext<'a> {
    curve: &'a PrimeCurve,
    fb: &'a FactorBase<PrimePoint>,
    order: u64,
    target: PrimePoint,
    point: &'a [u64],
    prefixes: &'a [Coordinate],
    m: usize,
    ops: &'a mut GroupOps,
    counters: &'a mut OracleCounters,
    limits: &'a OracleLimits,
    rows: &'a mut BTreeSet<Vec<i64>>,
    relations: &'a mut Vec<SignedRelation>,
    cap_hit: &'a mut bool,
}
impl LiftContext<'_> {
    fn visit(&mut self, indices: &mut Vec<usize>) -> bool {
        if indices.len() < self.m {
            let Some(choices) = self.fb.x_index.get(&self.point[indices.len()]) else {
                return false;
            };
            for &index in choices {
                indices.push(index);
                let stop = self.visit(indices);
                indices.pop();
                if stop {
                    return true;
                }
            }
            return false;
        }
        if self.counters.signed_lift_attempts >= self.limits.max_signed_lift_attempts {
            *self.cap_hit = true;
            return true;
        }
        self.counters.signed_lift_attempts += 1;
        let mut sum = self.fb.points[indices[0]];
        for i in 1..self.m {
            sum = self.curve.add(self.ops, sum, self.fb.points[indices[i]]);
            if i + 1 < self.m {
                let matches = match self.prefixes[i - 1] {
                    Coordinate::Infinity => sum.infinity,
                    Coordinate::Variable(v) => !sum.infinity && sum.x == self.point[v],
                    Coordinate::Constant(x) => !sum.infinity && sum.x == x,
                };
                if !matches {
                    return false;
                }
            }
        }
        if sum != self.target {
            return false;
        }
        let mut row = vec![0i64; self.fb.columns];
        for &i in indices.iter() {
            let coefficient = self.fb.coef_of[i];
            let sign = if coefficient == 1 {
                1
            } else {
                debug_assert_eq!(coefficient, self.order - 1);
                -1
            };
            row[self.fb.col_of[i]] += sign;
        }
        if self.rows.insert(row.clone()) {
            if row.iter().all(|&x| x == 0) {
                self.counters.zero_relation_rows += 1;
            }
            self.relations.push(SignedRelation {
                indices: indices.clone(),
                row,
            });
        } else {
            self.counters.duplicate_relation_rows += 1;
        }
        self.limits.stop_after_first
    }
}

/// Run the actual production F4 visitor over all sequential-prefix charts.
/// A positive result is certified by complete signed group replay. A negative
/// result requires complete exhaustion of every feasible chart. No reference
/// enumerator or preselected intermediate abscissa enters this function.
#[allow(clippy::too_many_arguments)]
pub fn decompose(
    curve: &PrimeCurve,
    subgroup_order: u64,
    fb: &FactorBase<PrimePoint>,
    values: &[u64],
    tower: Option<&TowerDescription>,
    target: PrimePoint,
    m: usize,
    presentation: Presentation,
    limits: &OracleLimits,
    ops: &mut GroupOps,
) -> Result<OracleResult, String> {
    let _guard = ORACLE_LOCK.lock().map_err(|_| "F4 study lock poisoned")?;
    let started = Instant::now();
    let mut validation_field_multiplications = 0;
    if !(2..=4).contains(&m)
        || subgroup_order <= 2
        || values.is_empty()
        || values.iter().any(|&x| x >= curve.p)
        || values.windows(2).any(|w| w[0] >= w[1])
        || curve.p < 5
        || curve.p >= (1u64 << 32)
        || curve.a >= curve.p
        || curve.b >= curve.p
        || !crate::cryptanalysis::residual_walk::is_prime_u64(curve.p)
        || (!target.infinity && (target.x >= curve.p || target.y >= curve.p))
        || !curve.is_on_curve(target)
    {
        return Err("invalid curve/target/order, sorted V, or arity (requires m=2..4)".into());
    }
    validate_factor_base(curve, subgroup_order, fb, values)?;
    if limits.max_degree == 0 || limits.max_pairs_per_step == 0 || limits.max_variables == 0 {
        return Err("solver degree, pair batch and variable cap must be positive".into());
    }
    if presentation == Presentation::Tower {
        let t = tower.ok_or("tower presentation requires a tower")?;
        if t.depth() == 0
            || t.depth() >= usize::BITS as usize
            || values.len() != 1usize << t.depth()
        {
            return Err(
                "complete tower V must contain exactly 2^depth distinct coordinates".into(),
            );
        }
        match t {
            TowerDescription::Velu { steps, terminal } => {
                if *terminal >= curve.p
                    || steps
                        .iter()
                        .any(|&(x, t)| x >= curve.p || t == 0 || t >= curve.p)
                {
                    return Err("invalid canonical Velu coefficients".into());
                }
            }
            TowerDescription::Dickson {
                shift, terminal, ..
            } => {
                if *shift >= curve.p || *terminal >= curve.p {
                    return Err("invalid canonical Dickson coefficients".into());
                }
            }
        }
        validate_tower_values(curve, values, t, &mut validation_field_multiplications)?;
    }
    let deadline = started
        .checked_add(Duration::from_millis(limits.timeout_ms))
        .ok_or("deadline overflow")?;
    let native_before = f4_fp::native_ops_total();
    let norm_before = f4_fp::norm_ops_total();
    let mut output=OracleResult{status:OracleStatus::CompleteNegative,complete:false,positive:false,
        presentation,summands:m,relations:vec![],charts:vec![],counters:OracleCounters::default(),elapsed_ns:0,
        polynomial_build_ns:0,reason:None,
        accounting:"Production F4 elimination and root counters plus explicit polynomial construction and signed group replay. F4 symbolic/native counters are incomplete diagnostics; allocation, hashing and sorting are not a complete operation total. Deadline is cooperative; dense memory requires an external process envelope.".into()};
    output.counters.validation_field_multiplications = validation_field_multiplications;
    let mut rows = BTreeSet::new();
    let mut all_complete = true;
    for mask in 0..1usize << (m - 2) {
        if Instant::now() >= deadline {
            all_complete = false;
            output.status = OracleStatus::UnknownTimeout;
            output.reason = Some("query deadline before next chart".into());
            break;
        }
        if presentation == Presentation::Dense
            && values.len().saturating_add(1).saturating_mul(m) > limits.max_input_terms
        {
            all_complete = false;
            output.status = OracleStatus::UnknownLimit;
            output.reason = Some("dense membership input term preflight cap".into());
            break;
        }
        let build_started = Instant::now();
        let built = build_chart(
            curve,
            values,
            tower,
            target,
            m,
            presentation,
            mask,
            limits,
            &mut output.counters,
        );
        output.polynomial_build_ns = output
            .polynomial_build_ns
            .saturating_add(build_started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64);
        let chart = match built {
            Ok(Some(c)) => c,
            Ok(None) => {
                output.charts.push(ChartReport {
                    chart_mask: mask,
                    prefix_infinity: (0..m - 2).map(|j| mask & (1 << j) != 0).collect(),
                    variables: 0,
                    equations: 0,
                    input_terms: 0,
                    status: "impossible_identity_plus_finite_equals_identity".into(),
                    visit: serde_json::Value::Null,
                });
                continue;
            }
            Err(e) => {
                all_complete = false;
                output.status = OracleStatus::UnknownLimit;
                output.reason = Some(e);
                break;
            }
        };
        let mut opts = F4Options::new(Ordering::Grevlex, limits.max_degree);
        opts.deadline = Some(deadline);
        opts.max_pairs_per_step = Some(limits.max_pairs_per_step);
        let mut cap_hit = false;
        let mut invalid_equation = false;
        let report = f4_fp::solve_visit(
            &chart.equations,
            chart.n_vars,
            curve.p,
            &opts,
            &chart.domains,
            |point| {
                if output.counters.coordinate_solutions >= limits.max_solutions {
                    cap_hit = true;
                    return true;
                }
                output.counters.coordinate_solutions += 1;
                if point.len() != chart.n_vars
                    || chart.equations.iter().any(|p| {
                        poly_eval(
                            p,
                            point,
                            curve.p,
                            &mut output.counters.original_equation_check_multiplications,
                        ) != 0
                    })
                {
                    invalid_equation = true;
                    return true;
                }
                let before = output.relations.len();
                let stop = LiftContext {
                    curve,
                    fb,
                    order: subgroup_order,
                    target,
                    point,
                    prefixes: &chart.prefixes,
                    m,
                    ops,
                    counters: &mut output.counters,
                    limits,
                    rows: &mut rows,
                    relations: &mut output.relations,
                    cap_hit: &mut cap_hit,
                }
                .visit(&mut Vec::with_capacity(m));
                if before == output.relations.len() {
                    output.counters.rejected_coordinate_solutions += 1;
                }
                stop
            },
        );
        let (status, visit) = match report {
            Ok(report) => {
                output.counters.f4_field_operations += report.field_ops;
                output.counters.root_field_multiplications += report.root_field_multiplications;
                output.counters.domain_evaluations += report.domain_evaluations;
                output.counters.full_field_branches += report.full_field_branches;
                output.counters.f4_runs += report.f4_runs;
                output.counters.solving_degree =
                    output.counters.solving_degree.max(report.solving_degree);
                output.counters.max_rows = output.counters.max_rows.max(report.max_rows);
                output.counters.max_columns = output.counters.max_columns.max(report.max_cols);
                let status = if invalid_equation {
                    all_complete = false;
                    output.status = OracleStatus::UnknownError;
                    output.reason = Some(
                        "production visitor emitted an invalid original-equation witness".into(),
                    );
                    "invalid_witness"
                } else if cap_hit {
                    all_complete = false;
                    output.status = OracleStatus::UnknownLimit;
                    output.reason = Some("coordinate-solution or signed-lift cap".into());
                    "limit"
                } else if limits.stop_after_first && !output.relations.is_empty() {
                    all_complete = false;
                    output.status = OracleStatus::FoundFirst;
                    "found_first"
                } else if !report.complete
                    || report.timed_out
                    || report.trace_stopped
                    || report.stopped_early
                {
                    all_complete = false;
                    output.status = if report.timed_out {
                        OracleStatus::UnknownTimeout
                    } else {
                        OracleStatus::UnknownLimit
                    };
                    output.reason = Some("production F4 visitor did not exhaust chart".into());
                    "incomplete"
                } else {
                    "complete"
                };
                (
                    status.to_owned(),
                    serde_json::to_value(report).map_err(|e| e.to_string())?,
                )
            }
            Err(error) => {
                all_complete = false;
                output.status = OracleStatus::UnknownError;
                output.reason = Some(error.clone());
                ("error".into(), serde_json::json!({"error":error}))
            }
        };
        output.charts.push(ChartReport {
            chart_mask: mask,
            prefix_infinity: (0..m - 2).map(|j| mask & (1 << j) != 0).collect(),
            variables: chart.n_vars,
            equations: chart.equations.len(),
            input_terms: chart.equations.iter().map(Vec::len).sum(),
            status,
            visit,
        });
        if !all_complete {
            break;
        }
    }
    output.positive = !output.relations.is_empty();
    output.complete = all_complete;
    if all_complete {
        output.status = if output.positive {
            OracleStatus::CompletePositive
        } else {
            OracleStatus::CompleteNegative
        };
    }
    output.counters.symbolic_native_operations =
        f4_fp::native_ops_total().saturating_sub(native_before);
    output.counters.normalization_operations = f4_fp::norm_ops_total().saturating_sub(norm_before);
    output.elapsed_ns = started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
    Ok(output)
}
