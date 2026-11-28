//! Cold, matched extraction with explicit signed witnesses and useful rows.
//! Production arithmetic, visitor, base indexes and incremental rank are reused.
//! Wall coverage and partial native primitive ledgers are deliberately distinct.

use super::*;
use crypto_lib::cryptanalysis::challenge_ladder::is_prime_u64;
use crypto_lib::cryptanalysis::f4_fp::{solve_visit_traced, VisitEvent};
use crypto_lib::cryptanalysis::ic_boundary::{IncrementalGauss, RowStatus};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[path = "controls.rs"]
mod controls;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Request {
    p: u64,
    curve_a: u64,
    curve_b: u64,
    scalar_modulus: u64,
    generator: [u64; 2],
    public_q: [u64; 2],
    depth: u32,
    targets: Vec<Target>,
    limits: Limits,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Target {
    m: usize,
    a: u64,
    b: u64,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Limits {
    max_degree: u32,
    seconds_per_target: u64,
    max_assignments: u64,
    max_trace_events: u64,
}

fn elapsed(start: Instant) -> u64 {
    u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

fn group_json(ops: &GroupOps) -> Value {
    json!({"adds":ops.adds,"doubles":ops.doubles,"scalar_mults":ops.scalar_mults,
        "field_muls":ops.field_ops.muls,"field_sqrs":ops.field_ops.sqrs,
        "field_invs":ops.field_ops.invs,"montgomery_reductions":ops.montgomery_reductions,
        "scope":"native group calls; scalar_mults contain the already counted adds/doubles"})
}

fn canonical_point(curve: &PrimeCurve, point: PrimePoint) -> bool {
    !point.infinity && point.x < curve.p && point.y < curve.p && curve.is_on_curve(point)
}

impl Request {
    fn curve(&self) -> PrimeCurve {
        PrimeCurve {
            p: self.p,
            a: self.curve_a,
            b: self.curve_b,
        }
    }
    fn g(&self) -> PrimePoint {
        PrimePoint::affine(self.generator[0], self.generator[1])
    }
    fn q(&self) -> PrimePoint {
        PrimePoint::affine(self.public_q[0], self.public_q[1])
    }

    fn admit(&self, ops: &mut GroupOps) -> Result<(), String> {
        if self.p <= 3
            || self.p >= 1u64 << 32
            || !is_prime_u64(self.p)
            || self.scalar_modulus <= 2
            || self.scalar_modulus >= 1u64 << 32
            || !is_prime_u64(self.scalar_modulus)
        {
            return Err("require prime 3 < p < 2^32 and odd prime scalar modulus r < 2^32".into());
        }
        if self.curve_a >= self.p || self.curve_b >= self.p {
            return Err("noncanonical curve coefficients".into());
        }
        let curve = self.curve();
        let f = Field::new(self.p);
        if f.add(f.mul(4, f.pow(curve.a, 3)), f.mul(27, f.pow(curve.b, 2))) == 0 {
            return Err("singular curve".into());
        }
        if !canonical_point(&curve, self.g()) || !canonical_point(&curve, self.q()) {
            return Err("G and Q must be canonical finite curve points".into());
        }
        if !curve.mul(ops, self.g(), self.scalar_modulus).infinity
            || !curve.mul(ops, self.q(), self.scalar_modulus).infinity
        {
            return Err("G or Q fails the declared prime subgroup check".into());
        }
        let upper = self.p + 1 + (4 * self.p).isqrt();
        if 2 * self.scalar_modulus <= upper {
            return Err("h=1 admission requires the Hasse upper bound below 2r".into());
        }
        if !(1..=12).contains(&self.depth) || self.targets.is_empty() || self.targets.len() > 32 {
            return Err("require depth 1..12 and 1..32 fixed public targets".into());
        }
        if self.targets.iter().any(|t| {
            !(2..=4).contains(&t.m) || t.a >= self.scalar_modulus || t.b >= self.scalar_modulus
        }) {
            return Err("require m=2..4 and canonical schedule scalars modulo r".into());
        }
        if !(1..=16).contains(&self.limits.max_degree)
            || !(1..=10).contains(&self.limits.seconds_per_target)
            || !(1..=20_000).contains(&self.limits.max_assignments)
            || !(1..=100_000).contains(&self.limits.max_trace_events)
        {
            return Err("limits exceed the bounded development contract".into());
        }
        Ok(())
    }
}

struct Base {
    fb: FactorBase<PrimePoint>,
    roots: Vec<u64>,
    sqrt_muls: u64,
    subgroup_ops: GroupOps,
}

fn build_base(request: &Request) -> Result<Base, String> {
    let curve = request.curve();
    let mut roots = dickson_abscissae(curve.p, request.depth, 0, 0)?;
    roots.sort_unstable();
    roots.dedup();
    let mut sqrt_muls = 0;
    let mut subgroup_ops = GroupOps::default();
    let (mut points, mut columns, mut coefficients) = (Vec::new(), Vec::new(), Vec::new());
    let mut ncols = 0;
    for &x in &roots {
        let Some(y) = curve.sqrt_counted(curve.rhs(x), &mut sqrt_muls) else {
            continue;
        };
        // h=1 and odd r exclude finite 2-torsion; reject a broken admission.
        if y == 0 {
            return Err("unexpected 2-torsion in admitted odd prime group".into());
        }
        let point = PrimePoint::affine(x, y.min(curve.p - y));
        if !curve
            .mul(&mut subgroup_ops, point, request.scalar_modulus)
            .infinity
        {
            return Err("base point outside the declared subgroup".into());
        }
        points.extend([point, curve.neg(point)]);
        columns.extend([ncols, ncols]);
        coefficients.extend([1, request.scalar_modulus - 1]);
        ncols += 1;
    }
    let fb = FactorBase::from_column_map(
        "D_(2^depth)(x,1)=0; canonical min-y signed columns; unknown logs".into(),
        points,
        columns,
        coefficients,
        ncols,
        |p| curve.key(p),
        |p| curve.key(&curve.neg(*p)),
        |p| p.x,
    )?;
    if fb.known_logs.is_some() {
        return Err("known logs forbidden in matched extraction".into());
    }
    Ok(Base {
        fb,
        roots,
        sqrt_muls,
        subgroup_ops,
    })
}

#[derive(Default, Serialize)]
struct SignWork {
    nodes: u64,
    divisions_by_nonzero_parent_y: u64,
    zero_parent_branches: u64,
    square_root_calls: u64,
    square_root_muls: u64,
    local_line_sign_flips: u64,
}

/// Recover every compatible signed chain from the actual signed root. L and O
/// consume the supplied local line/slope, including its sign at a zero-y parent;
/// the opposite line is a separate raw assignment. The recursion does not call
/// a base oracle or enumerate arbitrary leaf signs.
fn decode(
    system: &System,
    assignment: &[u64],
    work: &mut SignWork,
) -> Result<Vec<Vec<PrimePoint>>, String> {
    if assignment.len() != system.names.len() || assignment.iter().any(|&v| v >= system.curve.p) {
        return Err("noncanonical assignment".into());
    }
    fn node(
        system: &System,
        a: &[u64],
        j: usize,
        parent: PrimePoint,
        work: &mut SignWork,
    ) -> Vec<Vec<PrimePoint>> {
        let f = Field::new(system.curve.p);
        let block = &system.blocks[j];
        let [x, z, r] = block.coordinates.map(|v| v.evaluate(a));
        if parent.infinity || parent.x != r || !canonical_point(&system.curve, parent) {
            return vec![];
        }
        work.nodes += 1;
        let numerator = f.sub(
            f.mul(f.sub(x, r), f.sub(z, r)),
            f.add(f.mul(3, f.mul(r, r)), system.curve.a),
        );
        let lambda_squared = f.add(f.add(x, z), r);
        let slopes = match system.arm {
            Arm::Oriented => {
                if block.parent_y.unwrap().evaluate(a) != parent.y {
                    return vec![];
                }
                vec![a[block.auxiliaries[0]]]
            }
            Arm::Line => {
                let lambda = a[block.auxiliaries[1]];
                let nu = a[block.auxiliaries[2]];
                let ordinate = f.add(f.mul(lambda, r), nu);
                if ordinate == f.sub(0, parent.y) {
                    vec![lambda]
                } else if ordinate == parent.y {
                    work.local_line_sign_flips += 1;
                    vec![f.sub(0, lambda)]
                } else {
                    vec![]
                }
            }
            Arm::Quartic | Arm::Invariant => {
                if parent.y != 0 {
                    work.divisions_by_nonzero_parent_y += 1;
                    vec![f.mul(numerator, f.inv(f.mul(2, parent.y)).unwrap())]
                } else if numerator != 0 {
                    vec![]
                } else {
                    work.zero_parent_branches += 1;
                    work.square_root_calls += 1;
                    match system
                        .curve
                        .sqrt_counted(lambda_squared, &mut work.square_root_muls)
                    {
                        None => vec![],
                        Some(0) => vec![0],
                        Some(s) => vec![s, f.sub(0, s)],
                    }
                }
            }
        };
        let mut result = Vec::new();
        for lambda in slopes {
            if f.mul(lambda, lambda) != lambda_squared
                || f.mul(f.mul(2, parent.y), lambda) != numerator
            {
                continue;
            }
            let left = PrimePoint::affine(x, f.sub(f.mul(lambda, f.sub(x, r)), parent.y));
            let right = PrimePoint::affine(z, f.sub(f.mul(lambda, f.sub(z, r)), parent.y));
            if !canonical_point(&system.curve, left) || !canonical_point(&system.curve, right) {
                continue;
            }
            if j == 0 {
                result.push(vec![left, right]);
            } else {
                for mut children in node(system, a, j - 1, left, work) {
                    children.push(right);
                    result.push(children);
                }
            }
        }
        result
    }
    let result = node(
        system,
        assignment,
        system.blocks.len() - 1,
        system.target,
        work,
    );
    if result.is_empty() {
        Err("no compatible rational signed chain".into())
    } else {
        Ok(result)
    }
}

#[derive(Default, Serialize)]
struct CallbackPhases {
    equation_validation_ns: u64,
    sign_reconciliation_ns: u64,
    membership_and_replay_ns: u64,
    dedup_and_row_ns: u64,
    rank_ns: u64,
}

impl CallbackPhases {
    fn sum(&self) -> u64 {
        self.equation_validation_ns
            + self.sign_reconciliation_ns
            + self.membership_and_replay_ns
            + self.dedup_and_row_ns
            + self.rank_ns
    }
}

#[derive(Default, Serialize)]
struct Counts {
    raw_assignments: u64,
    invalid_equation_assignments: u64,
    undecodable_assignments: u64,
    decoded_signed_candidates: u64,
    membership_rejections: u64,
    group_or_chart_rejections: u64,
    accepted_signed_occurrences: u64,
    duplicate_ordered_signed: u64,
    duplicate_signed_multisets: u64,
    duplicate_rows_in_cell: u64,
    duplicate_rows_from_previous_cells: u64,
    independent_new_rows: u64,
    dependent_new_rows: u64,
    inconsistent_new_rows: u64,
}

#[derive(Default)]
struct Cell {
    counts: Counts,
    phases: CallbackPhases,
    sign_work: SignWork,
    replay_ops: GroupOps,
    x_chains: BTreeSet<Vec<u64>>,
    leaf_xs: BTreeSet<Vec<u64>>,
    ordered_signed: BTreeSet<Vec<(u64, u64)>>,
    multisets: BTreeSet<Vec<(u64, u64)>>,
    rows: BTreeSet<Vec<u64>>, // includes RHS as final entry
}

impl Cell {
    fn accept(
        &mut self,
        system: &System,
        assignment: &[u64],
        base: &Base,
        target: &Target,
        r: u64,
        global_rows: &mut BTreeSet<Vec<u64>>,
        rank: &mut IncrementalGauss,
    ) {
        self.counts.raw_assignments += 1;
        let start = Instant::now();
        let f = Field::new(system.curve.p);
        let valid = assignment.len() == system.names.len()
            && assignment.iter().all(|&v| v < system.curve.p)
            && system
                .equations
                .iter()
                .all(|p| evaluate(p, assignment, &f) == 0);
        self.phases.equation_validation_ns += elapsed(start);
        if !valid {
            self.counts.invalid_equation_assignments += 1;
            return;
        }
        let start = Instant::now();
        let decoded = decode(system, assignment, &mut self.sign_work);
        self.phases.sign_reconciliation_ns += elapsed(start);
        let Ok(decoded) = decoded else {
            self.counts.undecodable_assignments += 1;
            return;
        };
        for points in decoded {
            self.counts.decoded_signed_candidates += 1;
            let start = Instant::now();
            let indices: Option<Vec<usize>> = points
                .iter()
                .map(|p| base.fb.index_of_key(system.curve.key(p)))
                .collect();
            let Some(indices) = indices else {
                self.counts.membership_rejections += 1;
                self.phases.membership_and_replay_ns += elapsed(start);
                continue;
            };
            let mut sum = PrimePoint::INFINITY;
            let mut chart_matches = true;
            for (i, &point) in points.iter().enumerate() {
                sum = system.curve.add(&mut self.replay_ops, sum, point);
                if i >= 1 && i + 1 < points.len() {
                    chart_matches &=
                        !sum.infinity && sum.x == assignment[system.intermediates[i - 1]];
                    if system.arm == Arm::Oriented {
                        chart_matches &= sum.y == assignment[system.intermediate_ys[i - 1]];
                    }
                }
            }
            self.phases.membership_and_replay_ns += elapsed(start);
            if !chart_matches || sum != system.target {
                self.counts.group_or_chart_rejections += 1;
                continue;
            }
            let start = Instant::now();
            self.counts.accepted_signed_occurrences += 1;
            let leaf_xs = system.leaf_xs(assignment);
            let mut chain = leaf_xs.clone();
            chain.extend(system.intermediates.iter().map(|&i| assignment[i]));
            self.leaf_xs.insert(leaf_xs);
            self.x_chains.insert(chain);
            let signed: Vec<_> = points.iter().map(|p| (p.x, p.y)).collect();
            if !self.ordered_signed.insert(signed.clone()) {
                self.counts.duplicate_ordered_signed += 1;
            }
            let mut multiset = signed;
            multiset.sort_unstable();
            if !self.multisets.insert(multiset) {
                self.counts.duplicate_signed_multisets += 1;
            }
            let scalars = Field::new(r);
            let mut row = vec![0; base.fb.columns + 1];
            for index in indices {
                let col = base.fb.col_of[index];
                row[col] = scalars.add(row[col], base.fb.coef_of[index]);
            }
            row[base.fb.columns] = scalars.sub(0, target.b);
            let mut key = row.clone();
            key.push(target.a);
            if !self.rows.insert(key.clone()) {
                self.counts.duplicate_rows_in_cell += 1;
                self.phases.dedup_and_row_ns += elapsed(start);
                continue;
            }
            let new_global = global_rows.insert(key);
            self.phases.dedup_and_row_ns += elapsed(start);
            if !new_global {
                self.counts.duplicate_rows_from_previous_cells += 1;
                continue;
            }
            let start = Instant::now();
            match rank.add_row(row, target.a) {
                RowStatus::Independent => self.counts.independent_new_rows += 1,
                RowStatus::Dependent => self.counts.dependent_new_rows += 1,
                RowStatus::Inconsistent => self.counts.inconsistent_new_rows += 1,
            }
            self.phases.rank_ns += elapsed(start);
        }
    }
}

struct Run {
    record: Value,
    cells: Vec<(bool, Cell)>,
    base_points: Vec<PrimePoint>,
    targets: Vec<PrimePoint>,
}

fn collect(request: &Request, arm: Arm) -> Result<Run, String> {
    let cold_start = Instant::now();
    let phase = Instant::now();
    let mut admission_ops = GroupOps::default();
    request.admit(&mut admission_ops)?;
    let admission_ns = elapsed(phase);
    let curve = request.curve();
    let phase = Instant::now();
    let base = build_base(request)?;
    let base_ns = elapsed(phase);
    let phase = Instant::now();
    let mut rank = IncrementalGauss::new(base.fb.columns + 1, request.scalar_modulus);
    let mut global_rows = BTreeSet::new();
    let public_anchor = base.fb.index_of_key(curve.key(&request.g()));
    if let Some(i) = public_anchor {
        let mut row = vec![0; base.fb.columns + 1];
        row[base.fb.col_of[i]] = base.fb.coef_of[i];
        if rank.add_row(row, 1) != RowStatus::Independent {
            return Err("public G anchor failed".into());
        }
    }
    let seed_rank = rank.rank();
    let rank_setup_ns = elapsed(phase);
    let mut cells = Vec::new();
    let mut records = Vec::new();
    let mut targets = Vec::new();
    let mut cells_ns = 0;
    let mut target_ops = GroupOps::default();
    for (index, target) in request.targets.iter().enumerate() {
        let cell_start = Instant::now();
        let phase = Instant::now();
        let ag = curve.mul(&mut target_ops, request.g(), target.a);
        let bq = curve.mul(&mut target_ops, request.q(), target.b);
        let point = curve.add(&mut target_ops, ag, bq);
        let target_ns = elapsed(phase);
        if point.infinity {
            return Err(format!("target {index} is infinity, outside this chart"));
        }
        targets.push(point);
        let phase = Instant::now();
        let system = System::new(curve.clone(), point, arm, target.m, request.depth)?;
        let construction_ns = elapsed(phase);
        let mut cell = Cell::default();
        let rank_before = rank.rank();
        let mut trace_events = 0u64;
        let mut native_max_solving_degree = 0;
        let mut native_pairs_above_bound = 0;
        let mut full_field_refusals = 0;
        let opts = F4Options::new(Ordering::Grevlex, request.limits.max_degree)
            .with_budget(Duration::from_secs(request.limits.seconds_per_target))
            .with_visit_timing(true);
        let phase = Instant::now();
        let report = solve_visit_traced(
            &system.equations,
            system.names.len(),
            curve.p,
            &opts,
            &vec![None; system.names.len()],
            |assignment| {
                cell.accept(
                    &system,
                    assignment,
                    &base,
                    target,
                    request.scalar_modulus,
                    &mut global_rows,
                    &mut rank,
                );
                cell.counts.raw_assignments >= request.limits.max_assignments
            },
            |event| {
                trace_events += 1;
                if trace_events > request.limits.max_trace_events {
                    return Err("trace event cap".into());
                }
                match event {
                    VisitEvent::Branch {
                        source: "full_field",
                        ..
                    } => {
                        full_field_refusals += 1;
                        return Err("full-field fallback refused before recursion".into());
                    }
                    VisitEvent::NativeResult { report, .. } => {
                        native_max_solving_degree =
                            native_max_solving_degree.max(report.solving_degree_max);
                        native_pairs_above_bound += report.pairs_above_bound;
                    }
                    _ => {}
                }
                Ok(())
            },
        )?;
        let visitor_call_ns = elapsed(phase);
        let timing = report
            .timings
            .as_ref()
            .ok_or("missing opt-in visitor timing")?;
        if timing.total_ns
            != timing.native_f4_ns
                + timing.root_recovery_ns
                + timing.callback_ns
                + timing.traversal_other_ns
            || cell.phases.sum() > timing.callback_ns
            || timing.total_ns > visitor_call_ns
        {
            return Err("nonexclusive phase accounting".into());
        }
        let mut record = json!({
            "index":index,"schedule":target,"public_target":point_json(point),
            "variables":system.names.len(),"equations":system.equations.len(),
            "complete":report.complete,
            "outcome":if report.complete { if cell.rows.is_empty() { "complete_empty" } else { "complete_rows" } }
                else { "incomplete_lower_bound" },
            "visitor":report,"domain_scan_field_multiplications":report.domain_scan_field_multiplications,
            "native_max_solving_degree":native_max_solving_degree,
            "native_pairs_above_bound_sum":native_pairs_above_bound,"trace_events":trace_events,
            "full_field_refusals":full_field_refusals,"counts":cell.counts,
            "distinct_leaf_x_tuples":cell.leaf_xs.len(),"distinct_full_x_chains":cell.x_chains.len(),
            "distinct_ordered_signed":cell.ordered_signed.len(),"distinct_signed_multisets":cell.multisets.len(),
            "distinct_augmented_rows_in_cell":cell.rows.len(),"cumulative_distinct_relation_rows":global_rows.len(),
            "rank_before":rank_before,"rank_after":rank.rank(),"sign_work":cell.sign_work,
            "replay_group_ops":group_json(&cell.replay_ops),"callback_phases_ns":cell.phases,
            "callback_other_ns":timing.callback_ns - cell.phases.sum(),
            "wall_ns":{"target_construction":target_ns,"equation_construction":construction_ns,
                "visitor_call":visitor_call_ns,"visitor_call_outside_timing":visitor_call_ns-timing.total_ns},
            "ordered_signed_witnesses":cell.ordered_signed,"signed_multisets":cell.multisets,
            "full_x_chains":cell.x_chains,"augmented_rows_with_rhs":cell.rows,
        });
        let total = elapsed(cell_start);
        let other = total
            .checked_sub(target_ns + construction_ns + visitor_call_ns)
            .ok_or("overlapping cell phases")?;
        record["wall_ns"]["total"] = json!(total);
        record["wall_ns"]["other"] = json!(other);
        cells_ns += total;
        records.push(record);
        cells.push((report.complete, cell));
    }
    let phase = Instant::now();
    let recovered_scalar = rank.pinned(base.fb.columns);
    let mut recovery_ops = GroupOps::default();
    let recovered_verified =
        recovered_scalar.map(|d| curve.mul(&mut recovery_ops, request.g(), d) == request.q());
    let recovery_ns = elapsed(phase);
    let mut record = json!({
        "arm":arm.name(),"request":request,"base_columns":base.fb.columns,
        "base_signed_points":base.fb.points.iter().copied().map(point_json).collect::<Vec<_>>(),
        "base_selector_roots":base.roots,"base_logs_supplied":false,
        "admission_group_ops":group_json(&admission_ops),
        "base_sqrt_muls":base.sqrt_muls,"base_subgroup_group_ops":group_json(&base.subgroup_ops),
        "base_index_work":base.fb.cost.work_native,"target_group_ops":group_json(&target_ops),
        "public_g_anchor":public_anchor.map(|i| json!({"column":base.fb.col_of[i],"coefficient":base.fb.coef_of[i],"rhs":1})),
        "seed_rank":seed_rank,"cells":records,"all_cells_complete":cells.iter().all(|(complete,_)| *complete),
        "cumulative_distinct_relation_rows":global_rows.len(),"final_rank":rank.rank(),
        "rank_gain_from_relations":rank.rank()-seed_rank,"rank_columns":base.fb.columns+1,
        "rank_row_ops":rank.row_ops,"rank_dependent_rows":rank.dependent,"rank_inconsistent_rows":rank.inconsistent,
        "rank_work":rank.work_counters(),"recovered_scalar":recovered_scalar,"recovered_verified":recovered_verified,
        "recovery_group_ops":group_json(&recovery_ops),
        "correctness_failure":rank.inconsistent != 0 || recovered_verified == Some(false),
        "cold_wall_ns":{"admission":admission_ns,"base":base_ns,"rank_setup":rank_setup_ns,
            "cells":cells_ns,"scalar_recovery_and_replay":recovery_ns},
    });
    let total = elapsed(cold_start);
    let other = total
        .checked_sub(admission_ns + base_ns + rank_setup_ns + cells_ns + recovery_ns)
        .ok_or("overlapping cold phases")?;
    record["cold_wall_ns"]["total"] = json!(total);
    record["cold_wall_ns"]["other"] = json!(other);
    Ok(Run {
        record,
        cells,
        base_points: base.fb.points,
        targets,
    })
}

fn result_header(request: &Request, runs: Vec<Value>) -> Value {
    json!({"schema":"dickson-s3-matched-accounting/v1","request":request,"arms":runs,
        "evidence_status":"unsigned bounded native development; pcc:unverified",
        "solver_policy":"solve_visit_traced; no supplied domains; no full-field recursion; incoming generators retained",
        "wall_boundary":"cold arm admission through in-memory output assembly; excludes final JSON encoding and process startup",
        "primitive_accounting":"partial native ledgers, not total field/bit cost",
        "duplicate_counter_semantics":"parallel projections of all accepted occurrences; ordered, multiset and row duplicates overlap and must not be summed",
        "primitive_gaps":["selector builder arithmetic","primality/nonsingularity checks","polynomial construction and validation",
            "sign formula arithmetic except counted square roots","sorting/key comparisons and allocation","runtime bookkeeping"],
        "performance_claim":null})
}

pub(super) fn extract_file(path: &str) -> Result<Value, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() > 1_048_576 {
        return Err("request exceeds 1 MiB".into());
    }
    let request: Request = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut records = Vec::new();
    for arm in Arm::ALL {
        records.push(collect(&request, arm)?.record);
    }
    Ok(result_header(&request, records))
}

fn tiny_request() -> Request {
    Request {
        p: 7,
        curve_a: 0,
        curve_b: 3,
        scalar_modulus: 13,
        generator: [4, 2],
        public_q: [1, 2],
        depth: 1,
        targets: (2..=4).map(|m| Target { m, a: 0, b: 1 }).collect(),
        limits: Limits {
            max_degree: 8,
            seconds_per_target: 10,
            max_assignments: 20_000,
            max_trace_events: 100_000,
        },
    }
}

/// Decoder-only 2-torsion chart control: no invalid rank modulus is invented.
fn zero_y_control() -> Result<Value, String> {
    let curve = PrimeCurve { p: 7, a: 3, b: 3 };
    let target = PrimePoint::affine(1, 0);
    let points = [PrimePoint::affine(3, 2), PrimePoint::affine(4, 3)];
    let negative = points.map(|p| curve.neg(p));
    let expected = BTreeSet::from([
        points.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
        negative.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>(),
    ]);
    let mut records = Vec::new();
    for arm in Arm::ALL {
        let system = System::new(curve.clone(), target, arm, 2, 1)?;
        let mut assignments = BTreeSet::new();
        assignments.insert(system.assignment_for_points(&points)?);
        assignments.insert(system.assignment_for_points(&negative)?);
        // The x-only L constructor chooses one square-root representative.
        // Its second rational line is a separate solver assignment at s=0.
        if arm == Arm::Line {
            let mut opposite = assignments.iter().next().unwrap().clone();
            let f = Field::new(curve.p);
            for &i in &system.blocks[0].auxiliaries[1..] {
                opposite[i] = f.sub(0, opposite[i]);
            }
            assignments.insert(opposite);
        }
        let mut decoded = BTreeSet::new();
        let mut work = SignWork::default();
        for assignment in &assignments {
            system.check_assignment(assignment)?;
            for signed in decode(&system, assignment, &mut work)? {
                decoded.insert(signed.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>());
            }
        }
        if decoded != expected {
            return Err(format!("zero-y sign loss in {}", arm.name()));
        }
        records.push(
            json!({"arm":arm.name(),"distinct_assignments":assignments.len(),
            "ordered_signed_for_fixed_leaf_order":decoded,"sign_work":work}),
        );
    }
    Ok(json!({"kind":"constructor_and_decoder_only","rank":null,"arms":records}))
}

pub(super) fn check() -> Result<Value, String> {
    let request = tiny_request();
    let mut records = Vec::new();
    for arm in Arm::ALL {
        let run = collect(&request, arm)?;
        let mut control_records = Vec::new();
        for (i, (complete, cell)) in run.cells.iter().enumerate() {
            let control_start = Instant::now();
            let control = controls::exhaustive(
                &request.curve(),
                &run.base_points,
                request.targets[i].m,
                run.targets[i],
            )?;
            let control_ns = elapsed(control_start);
            if !cell.ordered_signed.is_subset(&control.ordered_signed)
                || !cell.multisets.is_subset(&control.signed_multisets)
                || !cell.x_chains.is_subset(&control.x_chains)
            {
                return Err(format!(
                    "invalid {} signed witness against oracle",
                    arm.name()
                ));
            }
            if *complete
                && (cell.ordered_signed != control.ordered_signed
                    || cell.multisets != control.signed_multisets
                    || cell.x_chains != control.x_chains)
            {
                return Err(format!("{} falsely complete against oracle", arm.name()));
            }
            if !complete {
                return Err(format!(
                    "tiny {} m{} control censored",
                    arm.name(),
                    request.targets[i].m
                ));
            }
            let predicted_raw = if arm == Arm::Line {
                [4, 16, 128][i]
            } else {
                [2, 4, 16][i]
            };
            if cell.counts.raw_assignments != predicted_raw
                || cell.rows.len() != [1, 2, 1][i]
                || cell.counts.invalid_equation_assignments != 0
                || cell.counts.group_or_chart_rejections != 0
            {
                return Err(format!(
                    "{} accounting differs from frozen prediction",
                    arm.name()
                ));
            }
            control_records.push(json!({"m":request.targets[i].m,"oracle_wall_ns":control_ns,
                "visited_tuples":control.visited_tuples,"total_group_matches":control.total_group_matches,
                "infinity_prefix_matches":control.infinity_prefix_matches,
                "finite_ordered_signed":control.ordered_signed.len(),"signed_multisets":control.signed_multisets.len(),
                "full_x_chains":control.x_chains.len(),"full_sum_group_ops":group_json(&control.full_sum_group_ops),
                "prefix_chart_group_ops":group_json(&control.prefix_chart_group_ops),"candidate_sets_equal":true}));
        }
        if run.record["final_rank"] != 3
            || run.record["rank_gain_from_relations"] != 2
            || run.record["recovered_scalar"] != 3
            || run.record["recovered_verified"] != true
            || run.record["rank_inconsistent_rows"] != 0
        {
            return Err(format!("{} useful rank/recovery mismatch", arm.name()));
        }
        let mut record = run.record;
        record["independent_oracle_outside_candidate_cost"] = json!(control_records);
        records.push(record);
    }
    let mut result = result_header(&request, records);
    result["zero_y_decoder_control"] = zero_y_control()?;
    result["frozen_predictions_passed"] = json!(true);
    Ok(result)
}

pub(super) fn check_m31() -> Result<Value, String> {
    let curve = prime31_curve();
    let fixture_start = Instant::now();
    let fixture_point = PrimePoint::affine(69_204_140, 78_579_263);
    if !canonical_point(&curve, fixture_point) {
        return Err("invalid frozen M31 fixture".into());
    }
    let mut fixture_ops = GroupOps::default();
    let q = curve.double(&mut fixture_ops, fixture_point);
    let fixture_ns = elapsed(fixture_start);
    let request = Request {
        p: curve.p,
        curve_a: curve.a,
        curve_b: curve.b,
        scalar_modulus: 2_147_446_831,
        generator: [625_037_715, 2_100_627_251],
        public_q: [q.x, q.y],
        depth: 3,
        targets: vec![Target { m: 2, a: 0, b: 1 }],
        limits: tiny_request().limits,
    };
    let mut records = Vec::new();
    for arm in Arm::ALL {
        let run = collect(&request, arm)?;
        if !run.base_points.contains(&fixture_point) {
            return Err("frozen M31 point is not in the actual selector base".into());
        }
        let control_start = Instant::now();
        let control = controls::exhaustive(&curve, &run.base_points, 2, q)?;
        let control_ns = elapsed(control_start);
        let (complete, cell) = &run.cells[0];
        if cell.counts.invalid_equation_assignments != 0
            || run.record["correctness_failure"] != false
        {
            return Err(format!(
                "M31 {} equation/rank/recovery correctness failure",
                arm.name()
            ));
        }
        let planted_key = vec![(fixture_point.x, fixture_point.y); 2];
        if !control.ordered_signed.contains(&planted_key)
            || !cell.ordered_signed.is_subset(&control.ordered_signed)
            || !cell.multisets.is_subset(&control.signed_multisets)
            || !cell.x_chains.is_subset(&control.x_chains)
        {
            return Err(format!(
                "M31 {} witness disagrees with independent oracle",
                arm.name()
            ));
        }
        if *complete
            && (cell.ordered_signed != control.ordered_signed
                || cell.multisets != control.signed_multisets
                || cell.x_chains != control.x_chains)
        {
            return Err(format!(
                "M31 {} falsely complete against oracle",
                arm.name()
            ));
        }
        let mut record = run.record;
        record["independent_oracle_outside_candidate_cost"] = json!({
            "oracle_wall_ns":control_ns,"visited_tuples":control.visited_tuples,
            "total_group_matches":control.total_group_matches,"infinity_prefix_matches":control.infinity_prefix_matches,
            "finite_ordered_signed":control.ordered_signed.len(),"signed_multisets":control.signed_multisets.len(),
            "full_x_chains":control.x_chains.len(),"planted_witness_present":true,
            "candidate_is_subset":true,"candidate_sets_equal":cell.ordered_signed == control.ordered_signed
                && cell.multisets == control.signed_multisets && cell.x_chains == control.x_chains,
            "full_sum_group_ops":group_json(&control.full_sum_group_ops),
            "prefix_chart_group_ops":group_json(&control.prefix_chart_group_ops)});
        records.push(record);
    }
    let mut result = result_header(&request, records);
    result["fixture"] = json!({"kind":"one fixed planted development target; not a cohort",
        "construction_wall_ns":fixture_ns,"construction_group_ops":group_json(&fixture_ops),
        "point_passed_to_solver":false,"selector_roots_passed_to_solver":false});
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_signed_yield_and_rank_match_independent_oracle() {
        check().unwrap();
    }

    #[test]
    fn two_torsion_signs_survive_without_a_fake_rank_modulus() {
        zero_y_control().unwrap();
    }

    #[test]
    fn admission_and_stopping_do_not_invent_a_complete_result() {
        let mut request = tiny_request();
        request.scalar_modulus = 7;
        assert!(request.admit(&mut GroupOps::default()).is_err());
        request = tiny_request();
        request.public_q = [1, 1];
        assert!(request.admit(&mut GroupOps::default()).is_err());
        request = tiny_request();
        request.targets.truncate(1);
        request.limits.max_assignments = 1;
        let run = collect(&request, Arm::Oriented).unwrap();
        assert!(!run.cells[0].0);
        assert_eq!(run.cells[0].1.counts.raw_assignments, 1);
        assert_eq!(run.record["cells"][0]["outcome"], "incomplete_lower_bound");
        request.limits.max_assignments = 20_000;
        request.limits.max_trace_events = 1;
        let run = collect(&request, Arm::Oriented).unwrap();
        assert!(!run.cells[0].0);
        assert_eq!(run.record["cells"][0]["visitor"]["trace_stopped"], true);
    }
}
