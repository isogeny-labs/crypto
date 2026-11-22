//! Public synthetic correctness fixtures for shared S3 Macaulay elimination.
//!
//! No arguments (or `--plan`) print the bounded fixture plan. `--check` runs
//! correctness checks and prints JSON, without collecting elapsed-time samples.
//! This is not a performance harness or a production complete point-decomposition
//! adapter. The affine chain has no coordinate for an intermediate infinity;
//! the exceptional-prefix control rejects that chart and retains the valid
//! group relation separately. The production chart work belongs to its own lane.
//!
//! cargo run --release --example shared_s3_matrix -- --plan
//! cargo run --release --example shared_s3_matrix -- --check

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use crypto_lib::cryptanalysis::exact_modular_matrix::{Backend, ModMatrix32};
use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly, VisitReport};
use crypto_lib::cryptanalysis::ic_boundary::{
    lift_abscissae, CountedGroup, FactorBase, GroupOps, IncrementalGauss, PrimeCurve, PrimePoint,
    RowStatus,
};
use crypto_lib::cryptanalysis::shared_macaulay::{
    FreshMacaulay, Limits, PreparedTarget, SharedMacaulay,
};
use crypto_lib::cryptanalysis::symmetrized_semaev::eval_s3_u64;
use serde_json::{json, Value};

const VARIABLES: usize = 4;
const DEGREE: u32 = 4;
const TOY_ORDER: u64 = 19;
const TOY_BASE_X: [u64; 3] = [0, 3, 5];
const BACKENDS: [Backend; 3] = [Backend::Integer, Backend::Fma, Backend::Split16];

#[derive(Clone)]
struct FamilyFixture {
    curve: PrimeCurve,
    factor_x: Vec<u64>,
    common: Vec<Poly>,
    pencil: Vec<Vec<Poly>>,
}

fn mul(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
}

fn neg(a: u64, p: u64) -> u64 {
    if a == 0 {
        0
    } else {
        p - a
    }
}

/// Symmetric expanded S3 terms, independently compared below with the existing
/// quadratic evaluator. Coefficient blocks are NOT separately made monic.
fn s3_terms(curve: &PrimeCurve) -> Vec<([u32; 3], u64)> {
    let p = curve.p;
    let minus_two = p - 2;
    let minus_two_a = neg(mul(2, curve.a, p), p);
    let minus_four_b = neg(mul(4, curve.b, p), p);
    vec![
        ([2, 2, 0], 1),
        ([2, 0, 2], 1),
        ([0, 2, 2], 1),
        ([2, 1, 1], minus_two),
        ([1, 2, 1], minus_two),
        ([1, 1, 2], minus_two),
        ([1, 1, 0], minus_two_a),
        ([1, 0, 1], minus_two_a),
        ([0, 1, 1], minus_two_a),
        ([1, 0, 0], minus_four_b),
        ([0, 1, 0], minus_four_b),
        ([0, 0, 1], minus_four_b),
        ([0, 0, 0], mul(curve.a, curve.a, p)),
    ]
}

fn membership(values: &[u64], variable: usize, p: u64) -> Poly {
    let mut coefficients = vec![1];
    for &value in values {
        let mut next = vec![0; coefficients.len() + 1];
        for (i, &coefficient) in coefficients.iter().enumerate() {
            next[i] = (next[i] + mul(neg(value, p), coefficient, p)) % p;
            next[i + 1] = (next[i + 1] + coefficient) % p;
        }
        coefficients = next;
    }
    coefficients
        .into_iter()
        .enumerate()
        .filter(|(_, c)| *c != 0)
        .map(|(degree, coefficient)| {
            let mut exponents = vec![0; VARIABLES];
            exponents[variable] = degree as u32;
            (exponents, coefficient)
        })
        .collect()
}

impl FamilyFixture {
    fn new(curve: PrimeCurve, factor_x: Vec<u64>) -> Result<Self, String> {
        if factor_x.is_empty() || factor_x.len() > 3 || factor_x.iter().any(|&x| x >= curve.p) {
            return Err("fixture requires one to three canonical factor-base x values".into());
        }
        let mut unique = factor_x.clone();
        unique.sort_unstable();
        unique.dedup();
        if unique.len() != factor_x.len() {
            return Err("duplicate fixture factor-base x".into());
        }
        let mut first = Vec::new();
        let mut target = vec![Vec::new(); 3];
        for (powers, coefficient) in s3_terms(&curve) {
            let mut exponent = vec![0; VARIABLES];
            // Common equation S3(x1,x2,u).
            exponent[0] = powers[0];
            exponent[1] = powers[1];
            exponent[3] = powers[2];
            first.push((exponent, coefficient));
            // Dynamic equation S3(u,x3,t), separated by t's power.
            let mut exponent = vec![0; VARIABLES];
            exponent[3] = powers[0];
            exponent[2] = powers[1];
            target[powers[2] as usize].push((exponent, coefficient));
        }
        let mut common = vec![first];
        for variable in 0..3 {
            common.push(membership(&factor_x, variable, curve.p));
        }
        Ok(Self {
            curve,
            factor_x,
            common,
            pencil: vec![target],
        })
    }

    fn shared(&self, backend: Backend) -> Result<SharedMacaulay, String> {
        SharedMacaulay::prepare(
            self.curve.p,
            VARIABLES,
            DEGREE,
            &self.common,
            &self.pencil,
            fixture_limits(),
            backend,
        )
        .map_err(|e| e.to_string())
    }

    fn fresh(&self, backend: Backend) -> Result<FreshMacaulay, String> {
        FreshMacaulay::prepare(
            self.curve.p,
            VARIABLES,
            DEGREE,
            &self.common,
            &self.pencil,
            fixture_limits(),
            backend,
        )
        .map_err(|e| e.to_string())
    }

    fn domains(&self) -> Vec<Option<Vec<u64>>> {
        vec![
            Some(self.factor_x.clone()),
            Some(self.factor_x.clone()),
            Some(self.factor_x.clone()),
            None,
        ]
    }

    fn direct_solutions(&self, target_x: u64) -> Result<BTreeSet<[u64; 4]>, String> {
        if self.curve.p > 17 {
            return Err("exhaustive fixture restricted to p<=17".into());
        }
        let mut out = BTreeSet::new();
        for &x1 in &self.factor_x {
            for &x2 in &self.factor_x {
                for &x3 in &self.factor_x {
                    for u in 0..self.curve.p {
                        if eval_s3_u64(x1, x2, u, self.curve.a, self.curve.b, self.curve.p) == 0
                            && eval_s3_u64(
                                u,
                                x3,
                                target_x,
                                self.curve.a,
                                self.curve.b,
                                self.curve.p,
                            ) == 0
                        {
                            out.insert([x1, x2, x3, u]);
                        }
                    }
                }
            }
        }
        Ok(out)
    }
}

fn fixture_limits() -> Limits {
    Limits {
        max_variables: 4,
        max_degree: 4,
        max_monomials: 70,
        max_rows: 128,
        max_cells: 65_536,
        max_input_terms: 128,
        max_parameter_powers: 3,
        max_scalar_products: 4_000_000,
    }
}

fn toy_fixture() -> FamilyFixture {
    FamilyFixture::new(PrimeCurve { p: 17, a: 2, b: 2 }, TOY_BASE_X.to_vec()).unwrap()
}

fn toy_generator() -> PrimePoint {
    PrimePoint::affine(5, 1)
}

fn factor_base(
    curve: &PrimeCurve,
    xs: &[u64],
    order: u64,
) -> Result<FactorBase<PrimePoint>, String> {
    let mut points = Vec::new();
    let mut columns = Vec::new();
    let mut coefficients = Vec::new();
    for (column, &x) in xs.iter().enumerate() {
        let y = curve
            .sqrt(curve.rhs(x))
            .ok_or("fixture abscissa has no rational point")?;
        let y = y.min((curve.p - y) % curve.p);
        let point = PrimePoint::affine(x, y);
        let negative = curve.neg(point);
        if point == negative {
            return Err("fixture does not admit order-two factor-base points".into());
        }
        points.extend([point, negative]);
        columns.extend([column, column]);
        coefficients.extend([1, order - 1]);
    }
    FactorBase::from_column_map(
        "shared-S3 synthetic fixture".into(),
        points,
        columns,
        coefficients,
        xs.len(),
        |point| curve.key(point),
        |point| curve.key(&curve.neg(*point)),
        |point| point.x,
    )
}

fn equations_satisfied(equations: &[Poly], point: &[u64], p: u64) -> bool {
    equations
        .iter()
        .all(|equation| f4_fp::eval(equation, point, p) == 0)
}

fn expanded_identity_check(fixture: &FamilyFixture) -> Result<usize, String> {
    let shared = fixture.shared(Backend::Integer)?;
    let p = fixture.curve.p;
    if p > 17 {
        return Err("exhaustive polynomial identity fixture restricted to p<=17".into());
    }
    let mut checked = 0;
    for x in 0..p {
        for y in 0..p {
            for z in 0..p {
                let first = f4_fp::eval(&fixture.common[0], &[x, y, 0, z], p);
                let expected = eval_s3_u64(x, y, z, fixture.curve.a, fixture.curve.b, p);
                if first != expected {
                    return Err("common expanded S3 differs from existing evaluator".into());
                }
                let specialized = shared.original_equations(z).map_err(|e| e.to_string())?;
                let target = specialized
                    .last()
                    .ok_or("missing specialized target equation")?;
                if f4_fp::eval(target, &[0, 0, y, x], p) != expected {
                    return Err(
                        "target coefficient pencil differs from existing S3 evaluator".into(),
                    );
                }
                checked += 2;
            }
        }
    }
    Ok(checked)
}

fn exhaustive_preconditioner_check(fixture: &FamilyFixture) -> Result<Value, String> {
    let mut arms = Vec::new();
    let mut reference_consequences: Option<Vec<Vec<Poly>>> = None;
    for backend in BACKENDS {
        let shared = fixture.shared(backend)?;
        let fresh = fixture.fresh(backend)?;
        let mut targets = Vec::new();
        let mut consequences = Vec::new();
        for target_x in 0..fixture.curve.p {
            let original = shared
                .original_equations(target_x)
                .map_err(|e| e.to_string())?;
            if original
                != fresh
                    .original_equations(target_x)
                    .map_err(|e| e.to_string())?
            {
                return Err("fresh/shared original equations disagree".into());
            }
            let cached = shared.specialize(target_x).map_err(|e| e.to_string())?;
            let uncached = fresh.specialize(target_x).map_err(|e| e.to_string())?;
            if cached.total_rank != uncached.total_rank {
                return Err("fresh/shared rank differs".into());
            }
            let expected = fixture.direct_solutions(target_x)?;
            let mut original_set = BTreeSet::new();
            let mut shared_set = BTreeSet::new();
            let mut fresh_set = BTreeSet::new();
            for &x1 in &fixture.factor_x {
                for &x2 in &fixture.factor_x {
                    for &x3 in &fixture.factor_x {
                        for u in 0..fixture.curve.p {
                            let point = [x1, x2, x3, u];
                            if equations_satisfied(&original, &point, fixture.curve.p) {
                                original_set.insert(point);
                                if equations_satisfied(
                                    &cached.consequences,
                                    &point,
                                    fixture.curve.p,
                                ) {
                                    shared_set.insert(point);
                                }
                                if equations_satisfied(
                                    &uncached.consequences,
                                    &point,
                                    fixture.curve.p,
                                ) {
                                    fresh_set.insert(point);
                                }
                            }
                        }
                    }
                }
            }
            if original_set != expected || shared_set != expected || fresh_set != expected {
                return Err(format!(
                    "solution set changed at target_x={target_x}, backend={backend:?}"
                ));
            }
            targets.push(json!({"parameter":target_x,"solutions":expected.len(),"shared_total_rank":cached.total_rank,"fresh_total_rank":uncached.total_rank,"shared_target_report":cached.report,"fresh_target_report":uncached.report}));
            consequences.push(cached.consequences);
        }
        if let Some(reference) = &reference_consequences {
            if reference != &consequences {
                return Err("arithmetic backends produced different shared consequences".into());
            }
        } else {
            reference_consequences = Some(consequences);
        }
        arms.push(json!({"backend":backend.name(),"shared_setup":shared.setup_report(),"fresh_setup":fresh.setup_report(),"targets":targets}));
    }
    Ok(
        json!({"parameter_count":fixture.curve.p,"assignments_per_parameter":fixture.factor_x.len().pow(3) as u64 * fixture.curve.p,"arms":arms,"cost_counter_coverage_complete":false}),
    )
}

fn solver_options() -> F4Options {
    let mut options = F4Options::new(Ordering::Grevlex, DEGREE);
    // A termination guard, not a timing sample or a performance observation.
    options.deadline = Some(Instant::now() + Duration::from_secs(10));
    options.max_pairs_per_step = Some(32);
    options
}

fn visit_solutions(
    fixture: &FamilyFixture,
    original: &[Poly],
    extra: &[Poly],
) -> Result<(BTreeSet<[u64; 4]>, VisitReport), String> {
    if fixture.curve.p > 17 {
        return Err("full-field F4 fixture restricted to p<=17".into());
    }
    let mut equations = original.to_vec();
    equations.extend_from_slice(extra);
    let mut solutions = BTreeSet::new();
    let report = f4_fp::solve_visit(
        &equations,
        VARIABLES,
        fixture.curve.p,
        &solver_options(),
        &fixture.domains(),
        |point| {
            if point.len() == VARIABLES {
                solutions.insert([point[0], point[1], point[2], point[3]]);
            }
            // solve_visit stops on true; keep visiting within the finite
            // domain cap, and let the complete-outcome guard reject a stop.
            solutions.len() > 459
        },
    )?;
    if !report.complete || report.timed_out || report.trace_stopped || report.stopped_early {
        return Err(
            "bounded F4 fixture was incomplete; no exhaustion conclusion is available".into(),
        );
    }
    for point in &solutions {
        if point[3] >= fixture.curve.p
            || point[..3].iter().any(|x| !fixture.factor_x.contains(x))
            || !equations_satisfied(original, point, fixture.curve.p)
        {
            return Err("F4 visitor returned a witness outside original equations/domains".into());
        }
    }
    Ok((solutions, report))
}

fn f4_integration_check(fixture: &FamilyFixture) -> Result<Value, String> {
    let shared = fixture.shared(Backend::Integer)?;
    let fresh = fixture.fresh(Backend::Integer)?;
    let target_x = 7;
    let original = shared
        .original_equations(target_x)
        .map_err(|e| e.to_string())?;
    let cached = shared.specialize(target_x).map_err(|e| e.to_string())?;
    let uncached = fresh.specialize(target_x).map_err(|e| e.to_string())?;
    let expected = fixture.direct_solutions(target_x)?;
    let (plain_set, plain_report) = visit_solutions(fixture, &original, &[])?;
    let (cached_set, cached_report) = visit_solutions(fixture, &original, &cached.consequences)?;
    let (fresh_set, fresh_report) = visit_solutions(fixture, &original, &uncached.consequences)?;
    if plain_set != expected || cached_set != expected || fresh_set != expected {
        return Err("existing F4 visitor changed the complete p17 fixture solution set".into());
    }
    Ok(
        json!({"target_x":target_x,"solutions":expected.len(),"original_equations_retained":true,"factor_coordinate_domains":fixture.factor_x,"auxiliary_domain":"full F17","original":plain_report,"shared":cached_report,"fresh":fresh_report,"cost_counter_coverage_complete":false}),
    )
}

fn chain_indices_valid(
    curve: &PrimeCurve,
    fb: &FactorBase<PrimePoint>,
    witness: &[usize],
    auxiliary_x: u64,
    target: PrimePoint,
    ops: &mut GroupOps,
) -> bool {
    if witness.len() != 3 || witness.iter().any(|&i| i >= fb.points.len()) {
        return false;
    }
    let prefix = curve.add(ops, fb.points[witness[0]], fb.points[witness[1]]);
    !prefix.infinity
        && prefix.x == auxiliary_x
        && curve.add(ops, prefix, fb.points[witness[2]]) == target
}

/// Resolve a *finite-chart* witness using existing counted group operations.
/// The existing sign lifter first checks the unqualified point-sum relation;
/// the additional finite-prefix check binds this particular auxiliary value.
fn lift_chain(
    curve: &PrimeCurve,
    fb: &FactorBase<PrimePoint>,
    point: &[u64],
    target: PrimePoint,
    ops: &mut GroupOps,
) -> Option<Vec<usize>> {
    if point.len() != 4 || point.iter().any(|&x| x >= curve.p) || target.infinity {
        return None;
    }
    let plain = lift_abscissae(curve, fb, ops, &point[..3], target)?;
    if chain_indices_valid(curve, fb, &plain, point[3], target, ops) {
        return Some(plain);
    }
    for &i in fb.x_index.get(&point[0])? {
        for &j in fb.x_index.get(&point[1])? {
            for &k in fb.x_index.get(&point[2])? {
                let witness = vec![i, j, k];
                if chain_indices_valid(curve, fb, &witness, point[3], target, ops) {
                    return Some(witness);
                }
            }
        }
    }
    None
}

fn point_value(point: PrimePoint) -> Value {
    if point.infinity {
        json!({"infinity":true})
    } else {
        json!({"x":point.x,"y":point.y,"infinity":false})
    }
}

fn signed_witness_check(fixture: &FamilyFixture) -> Result<Value, String> {
    let curve = &fixture.curve;
    let fb = factor_base(curve, &fixture.factor_x, TOY_ORDER)?;
    let mut ops = GroupOps::default();
    let mut checked = 0;
    for scalar in 1..TOY_ORDER {
        let target = curve.mul(&mut ops, toy_generator(), scalar);
        for point in fixture.direct_solutions(target.x)? {
            let witness = lift_chain(curve, &fb, &point, target, &mut ops)
                .ok_or("p17 finite-chart polynomial solution failed signed lift")?;
            if !chain_indices_valid(curve, &fb, &witness, point[3], target, &mut ops) {
                return Err("accepted signed witness failed exact group replay".into());
            }
            checked += 1;
        }
    }
    let g = toy_generator();
    let gi = *fb
        .point_index
        .get(&curve.key(&g))
        .ok_or("generator absent from fixture base")?;
    let ngi = fb.neg_index[gi];
    let cancelling = [gi, ngi, gi];
    let prefix = curve.add(&mut ops, fb.points[gi], fb.points[ngi]);
    if !prefix.infinity || curve.add(&mut ops, prefix, fb.points[gi]) != g {
        return Err("exceptional-prefix control is not the intended group relation".into());
    }
    for u in 0..curve.p {
        if chain_indices_valid(curve, &fb, &cancelling, u, g, &mut ops) {
            return Err("finite auxiliary coordinate admitted an infinite prefix".into());
        }
    }
    Ok(
        json!({"signed_nonexceptional_witnesses_replayed":checked,"exceptional_prefix":{"relation":"G + (-G) + G = G","group_relation_valid":true,"finite_auxiliary_representations_for_this_sign_choice":0,"handling":"reject this finite chart; full chart fallback belongs to production chain adapter"},"scope":"p17 finite affine chart; no full point-decomposition exhaustiveness claim"}),
    )
}

fn find_relation(
    fixture: &FamilyFixture,
    shared: &SharedMacaulay,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    ops: &mut GroupOps,
) -> Result<(Option<Vec<usize>>, PreparedTarget, VisitReport), String> {
    if fixture.curve.p > 17 {
        return Err("full-field relation fixture restricted to p<=17".into());
    }
    if target.infinity {
        return Err("fixture relation request is outside the finite target chart".into());
    }
    let original = shared
        .original_equations(target.x)
        .map_err(|e| e.to_string())?;
    let prepared = shared.specialize(target.x).map_err(|e| e.to_string())?;
    let mut equations = original.clone();
    equations.extend(prepared.consequences.iter().cloned());
    let mut witness = None;
    let report = f4_fp::solve_visit(
        &equations,
        VARIABLES,
        fixture.curve.p,
        &solver_options(),
        &fixture.domains(),
        |point| {
            if equations_satisfied(&original, point, fixture.curve.p) {
                witness = lift_chain(&fixture.curve, fb, point, target, ops);
            }
            // Stop only after the original equations and exact group lift
            // have supplied a valid relation.
            witness.is_some()
        },
    )?;
    if witness.is_none()
        && (!report.complete || report.timed_out || report.trace_stopped || report.stopped_early)
    {
        return Err("relation extraction interrupted; finite-chart exhaustion is unknown".into());
    }
    Ok((witness, prepared, report))
}

/// Solver receives a public Q only. The known scalar is constructed and checked
/// by the caller. This tiny fixture includes an explicit anchor relation for G
/// and uses the existing incremental linear algebra modulo 19.
fn recover_toy(fixture: &FamilyFixture, q: PrimePoint) -> Result<Value, String> {
    let curve = &fixture.curve;
    if (curve.p, curve.a, curve.b) != (17, 2, 2) {
        return Err("toy recovery is restricted to the declared p17 curve".into());
    }
    let g = toy_generator();
    let mut ops = GroupOps::default();
    if !curve.is_on_curve(q) || q.infinity || !curve.mul(&mut ops, q, TOY_ORDER).infinity {
        return Err("invalid toy target".into());
    }
    let shared = fixture.shared(Backend::Integer)?;
    let fb = factor_base(curve, &fixture.factor_x, TOY_ORDER)?;
    let target_column = fb.columns;
    let mut gauss = IncrementalGauss::new(fb.columns + 1, TOY_ORDER);
    let gi = *fb
        .point_index
        .get(&curve.key(&g))
        .ok_or("missing G anchor")?;
    let mut anchor = vec![0; fb.columns + 1];
    anchor[fb.col_of[gi]] = fb.coef_of[gi];
    if gauss.add_row(anchor, 1) == RowStatus::Inconsistent {
        return Err("G anchor contradicted itself".into());
    }
    let mut attempts = Vec::new();
    let mut relations = 0;
    let mut skipped_exceptional_or_base = 0;
    for a in 0..TOY_ORDER {
        let ag = curve.mul(&mut ops, g, a);
        let target = curve.add(&mut ops, ag, q);
        // Avoid counting a direct collision or one-point lookup as a three-term
        // algebraic relation in this deliberately scoped fixture.
        if target.infinity || fb.point_index.contains_key(&curve.key(&target)) {
            skipped_exceptional_or_base += 1;
            continue;
        }
        let (witness, prepared, report) = find_relation(fixture, &shared, &fb, target, &mut ops)?;
        let status = if let Some(indices) = witness {
            let mut row = vec![0; fb.columns + 1];
            let mut replay = curve.identity();
            for index in indices {
                row[fb.col_of[index]] = (row[fb.col_of[index]] + fb.coef_of[index]) % TOY_ORDER;
                replay = curve.add(&mut ops, replay, fb.points[index]);
            }
            if replay != target {
                return Err("relation failed point-sum replay before linear algebra".into());
            }
            row[target_column] = TOY_ORDER - 1;
            if gauss.add_row(row, a) == RowStatus::Inconsistent {
                return Err("relation matrix inconsistent modulo subgroup order".into());
            }
            relations += 1;
            "found"
        } else {
            "finite_chart_exhausted"
        };
        attempts.push(json!({"a":a,"b":1,"target":point_value(target),"status":status,"shared_target_report":prepared.report,"f4":report}));
        if let Some(recovered) = gauss.pinned(target_column) {
            if curve.mul(&mut ops, g, recovered) != q {
                return Err("recovered scalar failed public target check".into());
            }
            return Ok(json!({
                "modulus": TOY_ORDER,
                "coordinate_modulus": curve.p,
                "recovered_scalar": recovered,
                "public_target": point_value(q),
                "relation_count": relations,
                "anchor_relations": 1,
                "rank": gauss.rank(),
                "scoped_setup_report": shared.setup_report(),
                "attempts": attempts,
                "skipped_exceptional_or_base_queries": skipped_exceptional_or_base,
                "linear_algebra_row_ops": gauss.row_ops,
                "linear_algebra_work": gauss.work_counters(),
                "group_operations": ops,
                "group_field_operations": ops.field_ops,
                "group_count_scope": "target admission, query generation, sign lifting, relation replay and final scalar check; factor-base construction excluded",
                "answer_checked_in_group": true,
                "cost_counter_coverage_complete": false,
                "scope": "known-answer p17 fixture using finite-chart F4 relations; no timing or complete-cost comparison"
            }));
        }
    }
    Err("bounded fixture relations did not determine the target logarithm".into())
}

fn p31_fixture_check() -> Result<Value, String> {
    let curve = PrimeCurve {
        p: 2_147_483_647,
        a: 847_514_846,
        b: 2_087_444_724,
    };
    let order = 2_147_446_831;
    let generator = PrimePoint::affine(625_037_715, 2_100_627_251);
    let mut ops = GroupOps::default();
    if !curve.is_on_curve(generator) || !curve.mul(&mut ops, generator, order).infinity {
        return Err("committed p31 fixture failed point/order replay".into());
    }
    let points: Vec<_> = [1, 2, 4]
        .into_iter()
        .map(|scalar| curve.mul(&mut ops, generator, scalar))
        .collect();
    let prefix = curve.add(&mut ops, points[0], points[1]);
    let target = curve.add(&mut ops, prefix, points[2]);
    if prefix.infinity || target.infinity {
        return Err("p31 fixture selected an exceptional chart".into());
    }
    let fixture = FamilyFixture::new(curve.clone(), points.iter().map(|point| point.x).collect())?;
    let witness = [points[0].x, points[1].x, points[2].x, prefix.x];
    let mut reference = None;
    let mut reports = Vec::new();
    for backend in BACKENDS {
        let shared = fixture.shared(backend)?;
        let fresh = fixture.fresh(backend)?;
        let cached = shared.specialize(target.x).map_err(|e| e.to_string())?;
        let uncached = fresh.specialize(target.x).map_err(|e| e.to_string())?;
        let original = shared
            .original_equations(target.x)
            .map_err(|e| e.to_string())?;
        if !equations_satisfied(&original, &witness, curve.p)
            || !equations_satisfied(&cached.consequences, &witness, curve.p)
            || !equations_satisfied(&uncached.consequences, &witness, curve.p)
            || cached.total_rank != uncached.total_rank
        {
            return Err("p31 witness or fresh/shared rank differed".into());
        }
        if let Some(rows) = &reference {
            if rows != &cached.consequences {
                return Err("p31 backends differ".into());
            }
        } else {
            reference = Some(cached.consequences.clone());
        }
        reports.push(json!({"backend":backend.name(),"shared_setup":shared.setup_report(),"fresh_setup":fresh.setup_report(),"shared_target":cached.report,"fresh_target":uncached.report}));
    }
    // In particular these are distinct arithmetic contexts, not interchangeable
    // Mersenne reductions for coordinates and relation coefficients.
    let coordinate = ModMatrix32::new(curve.p).map_err(|e| e.to_string())?;
    let subgroup = ModMatrix32::new(order).map_err(|e| e.to_string())?;
    if coordinate.modulus() == subgroup.modulus() {
        return Err("fixture accidentally conflates field and subgroup moduli".into());
    }
    Ok(
        json!({"curve":curve,"subgroup_order":order,"cofactor":1,"generator":point_value(generator),"public_target":point_value(target),"point_sum_fixture":"G+2G+4G=7G","coordinate_witness":witness,"source":"examples/prime31_batched_ic_same_1024q.rs","registry_slug":null,"checks":"point/order replay, original equations, consequences, fresh/shared rank and all arithmetic arms","f4_full_field_search_run":false,"complete_dlp_recovery_run":false,"arms":reports,"cost_counter_coverage_complete":false}),
    )
}

fn plan() -> Value {
    json!({"schema":"shared-s3-matrix.fixture-plan/v1","status":"planned","mode":"correctness-only","variables":["x1","x2","x3","u"],"common":["S3(x1,x2,u)","membership(x1)","membership(x2)","membership(x3)"],"parameterized":"S3(u,x3,t)","macaulay_degree":DEGREE,"limits":fixture_limits(),"backends":["integer","fma","split16"],"p17":{"curve":{"p":17,"a":2,"b":2},"subgroup_order":19,"generator":{"x":5,"y":1},"factor_base_x":TOY_BASE_X,"auxiliary_domain":"full F17","exhaustive_parameters":17,"assignments_per_parameter":459},"checks":["expanded S3 versus existing numeric evaluator","original/shared/fresh complete p17 polynomial solution sets","existing F4 visitor with original equations retained","signed finite-prefix witnesses and exceptional-prefix control","public toy logarithm recovered modulo19","committed p31 point witness and arithmetic equality"],"benchmark_executed":false,"native_blas_or_gpu":false,"complete_cost_comparison":false,"next_run":"--check"})
}

fn run_checks() -> Result<Value, String> {
    let fixture = toy_fixture();
    let mut ops = GroupOps::default();
    if !fixture
        .curve
        .mul(&mut ops, toy_generator(), TOY_ORDER)
        .infinity
    {
        return Err("toy group order check failed".into());
    }
    let q = fixture.curve.mul(&mut ops, toy_generator(), 9);
    let identities = expanded_identity_check(&fixture)?;
    let preconditioners = exhaustive_preconditioner_check(&fixture)?;
    let f4 = f4_integration_check(&fixture)?;
    let signed = signed_witness_check(&fixture)?;
    let recovery = recover_toy(&fixture, q)?;
    if recovery["recovered_scalar"].as_u64() != Some(9) {
        return Err("toy recovered scalar differs from caller-only known answer".into());
    }
    let p31 = p31_fixture_check()?;
    Ok(
        json!({"schema":"shared-s3-matrix.fixture-check/v1","status":"passed","evidence_status":"producer local correctness checks; PCC-unverified","identity_evaluations":identities,"p17_preconditioning":preconditioners,"f4_integration":f4,"signed_witnesses":signed,"toy_recovery":recovery,"p31":p31,"benchmark_executed":false,"complete_cost_comparison":false,"cost_counter_coverage_complete":false}),
    )
}

fn main() {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    let result = match arguments.as_slice() {
        [] => Ok(plan()),
        [flag] if flag == "--plan" => Ok(plan()),
        [flag] if flag == "--check" => run_checks(),
        _ => Err("usage: shared_s3_matrix [--plan|--check]; timing/benchmark execution is not implemented in this correctness fixture".into()),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => {
            println!(
                "{}",
                json!({"schema":"shared-s3-matrix.fixture-check/v1","status":"incomplete_or_failed","error":error,"benchmark_executed":false})
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_toy_solution_sets_match_all_backends() {
        let fixture = toy_fixture();
        assert_eq!(
            expanded_identity_check(&fixture).unwrap(),
            2 * 17usize.pow(3)
        );
        exhaustive_preconditioner_check(&fixture).unwrap();
    }

    #[test]
    fn existing_f4_keeps_original_equations_and_domains() {
        f4_integration_check(&toy_fixture()).unwrap();
    }

    #[test]
    fn signed_witnesses_and_infinite_prefix_control() {
        signed_witness_check(&toy_fixture()).unwrap();
    }

    #[test]
    fn toy_public_target_recovers_through_existing_relation_matrix() {
        let fixture = toy_fixture();
        let q = fixture
            .curve
            .mul(&mut GroupOps::default(), toy_generator(), 9);
        let recovered = recover_toy(&fixture, q).unwrap();
        assert_eq!(recovered["recovered_scalar"], 9);
        assert!(recovered["relation_count"].as_u64().unwrap() > 0);
        assert_eq!(recovered["answer_checked_in_group"], true);
        for attempt in recovered["attempts"].as_array().unwrap() {
            if attempt["status"] == "found" {
                assert_eq!(attempt["f4"]["stopped_early"], true);
            }
        }
    }

    #[test]
    fn committed_p31_nonexceptional_witness_matches_all_backends() {
        p31_fixture_check().unwrap();
    }
}
