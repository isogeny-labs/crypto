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
//! cargo run --release --example shared_s3_matrix -- --check-prime31

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
const P31: u64 = 2_147_483_647;
const P31_A: u64 = 847_514_846;
const P31_B: u64 = 2_087_444_724;
const P31_ORDER: u64 = 2_147_446_831;
// Public coordinates retained from the committed development receipt. These
// inputs do not contain the target's scalar or factor-base logarithms.
const P31_BASE_X: [u64; 3] = [625_037_715, 1_923_055_775, 1_731_809_510];
const P31_ATTEMPTS: u64 = 16;
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

    /// Limit this development executable to its two declared curve/base
    /// fixtures. In particular, removing a small-field guard does not silently
    /// turn an exhaustive diagnostic into an arbitrary full-field search.
    fn declared_group(&self) -> Result<(PrimePoint, u64), String> {
        match (self.curve.p, self.curve.a, self.curve.b) {
            (17, 2, 2) if self.factor_x == TOY_BASE_X => Ok((toy_generator(), TOY_ORDER)),
            (P31, P31_A, P31_B) if self.factor_x == P31_BASE_X => Ok((p31_generator(), P31_ORDER)),
            _ => Err("search is restricted to the declared p17 and p31 curve/base fixtures".into()),
        }
    }

    /// Independent finite-chart reference, used only for validation after the
    /// F4 calls. Rational lifts of x1 and x2 give all finite roots of their S3
    /// equation as signed pair-sum abscissae. Repeated coordinates use doubling;
    /// an opposite-sign cancellation has no affine auxiliary coordinate.
    /// Neither this set nor its construction is passed to the relation solver.
    fn signed_pair_reference(&self, target: PrimePoint) -> Result<BTreeSet<[u64; 4]>, String> {
        let (_, order) = self.declared_group()?;
        let curve = &self.curve;
        if target.infinity
            || target.x >= curve.p
            || target.y >= curve.p
            || !curve.is_on_curve(target)
        {
            return Err("reference target must be a finite point on the declared curve".into());
        }
        let fb = factor_base(curve, &self.factor_x, order)?;
        let mut ops = GroupOps::default();
        let mut out = BTreeSet::new();
        for &left in &fb.points {
            for &right in &fb.points {
                let prefix = curve.add(&mut ops, left, right);
                if prefix.infinity {
                    continue;
                }
                if eval_s3_u64(left.x, right.x, prefix.x, curve.a, curve.b, curve.p) != 0 {
                    return Err("finite signed pair failed original first S3 equation".into());
                }
                for &x3 in &self.factor_x {
                    if eval_s3_u64(prefix.x, x3, target.x, curve.a, curve.b, curve.p) == 0 {
                        out.insert([left.x, right.x, x3, prefix.x]);
                    }
                }
            }
        }
        Ok(out)
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

fn p31_curve() -> PrimeCurve {
    PrimeCurve {
        p: P31,
        a: P31_A,
        b: P31_B,
    }
}

fn p31_generator() -> PrimePoint {
    PrimePoint::affine(625_037_715, 2_100_627_251)
}

fn p31_public_target() -> PrimePoint {
    PrimePoint::affine(864_723_287, 1_209_733_093)
}

fn p31_fixture() -> FamilyFixture {
    FamilyFixture::new(p31_curve(), P31_BASE_X.to_vec()).unwrap()
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
    visit_solutions_with_options(fixture, original, extra, &solver_options())
}

fn visit_solutions_with_options(
    fixture: &FamilyFixture,
    original: &[Poly],
    extra: &[Poly],
    options: &F4Options,
) -> Result<(BTreeSet<[u64; 4]>, VisitReport), String> {
    fixture.declared_group()?;
    let mut equations = original.to_vec();
    equations.extend_from_slice(extra);
    let mut solutions = BTreeSet::new();
    // The first S3 is a nonzero polynomial of degree at most two in u
    // for these rational, non-order-two leaf coordinates. This is a stop
    // guard only: no reference roots or restricted auxiliary domain are used.
    let solution_cap = 2 * fixture.factor_x.len().pow(3);
    let report = f4_fp::solve_visit(
        &equations,
        VARIABLES,
        fixture.curve.p,
        options,
        &fixture.domains(),
        |point| {
            if point.len() == VARIABLES {
                solutions.insert([point[0], point[1], point[2], point[3]]);
            }
            // solve_visit stops on true; keep visiting within the finite
            // domain cap, and let the complete-outcome guard reject a stop.
            solutions.len() > solution_cap
        },
    )?;
    if !report.complete || report.timed_out || report.trace_stopped || report.stopped_early {
        return Err(format!(
            "bounded F4 fixture was incomplete; no exhaustion conclusion is available: {}",
            json!({"visit":report,"retained_solutions":solutions})
        ));
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
    fixture.declared_group()?;
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
        return Err(format!(
            "relation extraction interrupted; finite-chart exhaustion is unknown: {}",
            json!({"target":point_value(target),"shared_target_report":prepared.report,"f4":report})
        ));
    }
    Ok((witness, prepared, report))
}

/// Solver receives the public Q, generator, subgroup order and bounded query
/// policy. It receives neither the target scalar, factor-base logarithms,
/// reference solutions nor a point-sum lookup table. Its only initial scalar
/// relation is the explicit G anchor. All later rows come from the actual F4
/// visitor and signed group replay; relation arithmetic uses `order`, not p.
fn recover_public(
    fixture: &FamilyFixture,
    q: PrimePoint,
    g: PrimePoint,
    order: u64,
    max_attempts: u64,
    backend: Backend,
) -> Result<Value, String> {
    let curve = &fixture.curve;
    if fixture.declared_group()? != (g, order) {
        return Err("generator/order differs from the declared fixture".into());
    }
    if max_attempts == 0 || max_attempts > 32 || max_attempts > order {
        return Err(
            "fixture requires an explicit attempt bound between 1 and min(32, order)".into(),
        );
    }
    let mut ops = GroupOps::default();
    if g.infinity || !curve.is_on_curve(g) || !curve.mul(&mut ops, g, order).infinity {
        return Err("invalid fixture generator".into());
    }
    if q.infinity
        || q.x >= curve.p
        || q.y >= curve.p
        || !curve.is_on_curve(q)
        || !curve.mul(&mut ops, q, order).infinity
    {
        return Err("invalid public target".into());
    }
    let shared = fixture.shared(backend)?;
    let fb = factor_base(curve, &fixture.factor_x, order)?;
    let target_column = fb.columns;
    let mut gauss = IncrementalGauss::new(fb.columns + 1, order);
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
    for a in 0..max_attempts {
        let ag = curve.mul(&mut ops, g, a);
        let target = curve.add(&mut ops, ag, q);
        // Avoid counting a direct collision or one-point lookup as a three-term
        // algebraic relation in this deliberately scoped fixture.
        if target.infinity || fb.point_index.contains_key(&curve.key(&target)) {
            skipped_exceptional_or_base += 1;
            continue;
        }
        let (witness, prepared, report) =
            find_relation(fixture, &shared, &fb, target, &mut ops).map_err(|error| {
                format!(
                    "bounded recovery interrupted: {}",
                    json!({"error":error,"backend":backend.name(),"a":a,"attempt_limit":max_attempts,"retained_attempts":attempts})
                )
            })?;
        let rank_before = gauss.rank();
        let mut relation = Value::Null;
        let status = if let Some(indices) = witness {
            let mut row = vec![0; fb.columns + 1];
            let mut replay = curve.identity();
            for &index in &indices {
                row[fb.col_of[index]] = (row[fb.col_of[index]] + fb.coef_of[index]) % order;
                replay = curve.add(&mut ops, replay, fb.points[index]);
            }
            if replay != target {
                return Err("relation failed point-sum replay before linear algebra".into());
            }
            row[target_column] = order - 1;
            relation = json!({"factor_base_indices":indices,"row":row,"rhs":a});
            if gauss.add_row(row, a) == RowStatus::Inconsistent {
                return Err("relation matrix inconsistent modulo subgroup order".into());
            }
            relations += 1;
            "found"
        } else {
            "finite_chart_exhausted"
        };
        attempts.push(json!({"a":a,"b":1,"target":point_value(target),"status":status,"relation":relation,"rank_before":rank_before,"rank_after":gauss.rank(),"shared_target_report":prepared.report,"f4":report}));
        if let Some(recovered) = gauss.pinned(target_column) {
            if curve.mul(&mut ops, g, recovered) != q {
                return Err("recovered scalar failed public target check".into());
            }
            return Ok(json!({
                "modulus": order,
                "coordinate_modulus": curve.p,
                "preconditioner_backend": backend.name(),
                "f4_backend": "unchanged production integer F4 visitor",
                "relation_linear_algebra": "existing integer IncrementalGauss modulo subgroup order",
                "generator": point_value(g),
                "factor_base_x": fixture.factor_x,
                "factor_base_points": fb.points.iter().copied().map(point_value).collect::<Vec<_>>(),
                "factor_base_columns": fb.col_of,
                "factor_base_coefficients": fb.coef_of,
                "target_column": target_column,
                "recovered_scalar": recovered,
                "public_target": point_value(q),
                "relation_count": relations,
                "anchor_relations": 1,
                "anchor_column": fb.col_of[gi],
                "anchor_coefficient": fb.coef_of[gi],
                "attempt_limit": max_attempts,
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
                "known_answer_passed_to_solver": false,
                "reference_solutions_passed_to_solver": false,
                "uniform_random_target_tested": false,
                "benchmark_executed": false,
                "cost_counter_coverage_complete": false,
                "scope": "structured known-answer target/base; public-Q recovery with finite-chart F4 relations; no uniform random-target or complete-cost conclusion"
            }));
        }
    }
    Err(format!(
        "bounded fixture relations did not determine the target logarithm: {}",
        json!({"attempt_limit":max_attempts,"backend":backend.name(),"rank":gauss.rank(),"relation_count":relations,"retained_attempts":attempts,"skipped_exceptional_or_base_queries":skipped_exceptional_or_base})
    ))
}

fn recover_toy(fixture: &FamilyFixture, q: PrimePoint) -> Result<Value, String> {
    recover_public(
        fixture,
        q,
        toy_generator(),
        TOY_ORDER,
        TOY_ORDER,
        Backend::Integer,
    )
}

fn p31_fixture_check() -> Result<Value, String> {
    let curve = p31_curve();
    let order = P31_ORDER;
    let generator = p31_generator();
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
    if prefix.infinity || target.infinity || target != p31_public_target() {
        return Err("p31 fixture selected an exceptional chart".into());
    }
    let fixture = FamilyFixture::new(curve.clone(), points.iter().map(|point| point.x).collect())?;
    fixture.declared_group()?;
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

fn check_lifted_set(
    fixture: &FamilyFixture,
    fb: &FactorBase<PrimePoint>,
    solutions: &BTreeSet<[u64; 4]>,
    target: PrimePoint,
) -> Result<usize, String> {
    let mut ops = GroupOps::default();
    for point in solutions {
        let witness = lift_chain(&fixture.curve, fb, point, target, &mut ops)
            .ok_or("p31 polynomial solution failed its signed finite-prefix lift")?;
        if !chain_indices_valid(&fixture.curve, fb, &witness, point[3], target, &mut ops) {
            return Err("p31 signed witness failed exact finite-prefix replay".into());
        }
    }
    Ok(solutions.len())
}

fn p31_f4_integration_check() -> Result<Value, String> {
    let fixture = p31_fixture();
    let target = p31_public_target();
    let fb = factor_base(&fixture.curve, &fixture.factor_x, P31_ORDER)?;
    let original = fixture
        .shared(Backend::Integer)?
        .original_equations(target.x)
        .map_err(|e| e.to_string())?;
    let (plain_set, plain_report) = visit_solutions(&fixture, &original, &[])?;
    let plain_lifts = check_lifted_set(&fixture, &fb, &plain_set, target)?;
    let mut arms = Vec::new();
    for backend in BACKENDS {
        let shared = fixture.shared(backend)?;
        let fresh = fixture.fresh(backend)?;
        if shared
            .original_equations(target.x)
            .map_err(|e| e.to_string())?
            != original
            || fresh
                .original_equations(target.x)
                .map_err(|e| e.to_string())?
                != original
        {
            return Err("p31 arithmetic/preconditioning arms changed original equations".into());
        }
        let cached = shared.specialize(target.x).map_err(|e| e.to_string())?;
        let uncached = fresh.specialize(target.x).map_err(|e| e.to_string())?;
        if cached.total_rank != uncached.total_rank {
            return Err("p31 fresh/shared rank differed during actual F4 qualification".into());
        }
        let (cached_set, cached_report) =
            visit_solutions(&fixture, &original, &cached.consequences)?;
        let (fresh_set, fresh_report) =
            visit_solutions(&fixture, &original, &uncached.consequences)?;
        if cached_set != plain_set || fresh_set != plain_set {
            return Err(format!(
                "p31 F4 finite-chart solutions changed under {} preconditioning",
                backend.name()
            ));
        }
        let shared_lifts = check_lifted_set(&fixture, &fb, &cached_set, target)?;
        let fresh_lifts = check_lifted_set(&fixture, &fb, &fresh_set, target)?;
        arms.push(json!({
            "preconditioner_backend":backend.name(),
            "solutions":cached_set.len(),
            "shared_signed_lifts":shared_lifts,
            "fresh_signed_lifts":fresh_lifts,
            "shared_total_rank":cached.total_rank,
            "fresh_total_rank":uncached.total_rank,
            "shared_setup":shared.setup_report(),
            "fresh_setup":fresh.setup_report(),
            "shared_target":cached.report,
            "fresh_target":uncached.report,
            "shared_f4":cached_report,
            "fresh_f4":fresh_report
        }));
    }
    // Construct the independent check only after all production solver calls.
    // It neither supplies a restricted u-domain nor acts as a solver fallback.
    let expected = fixture.signed_pair_reference(target)?;
    if plain_set != expected || expected.is_empty() {
        return Err(
            "p31 F4 solution set differs from the independent signed-pair reference".into(),
        );
    }
    Ok(json!({
        "coordinate_modulus":P31,
        "public_target":point_value(target),
        "factor_coordinate_domains":fixture.factor_x,
        "auxiliary_domain":"full Fp (None in the production visitor)",
        "solutions":plain_set,
        "solution_count":expected.len(),
        "reference":"finite signed pair sums, followed by original second-S3 evaluation; validation only",
        "reference_computed_after_solver_calls":true,
        "reference_solutions_passed_to_solver":false,
        "original_equations_retained":true,
        "original_signed_lifts":plain_lifts,
        "original_f4":plain_report,
        "arms":arms,
        "scope":"complete solution sets for the declared three-coordinate factor base and finite auxiliary chart only",
        "complete_point_decomposition_coverage":false,
        "cost_counter_coverage_complete":false
    }))
}

fn p31_recovery_check() -> Result<Value, String> {
    let fixture = p31_fixture();
    let target = p31_public_target();
    // This caller-only check binds the historical public point. Neither 7 nor
    // this known-answer comparison is passed to recover_public.
    if fixture
        .curve
        .mul(&mut GroupOps::default(), p31_generator(), 7)
        != target
    {
        return Err("historical p31 public target differs from the caller's known answer".into());
    }
    let mut arms = Vec::new();
    for backend in BACKENDS {
        let recovered = recover_public(
            &fixture,
            target,
            p31_generator(),
            P31_ORDER,
            P31_ATTEMPTS,
            backend,
        )?;
        if recovered["recovered_scalar"].as_u64() != Some(7) {
            return Err("p31 recovered scalar differs from the caller-only known answer".into());
        }
        arms.push(recovered);
    }
    Ok(json!({
        "coordinate_modulus":P31,
        "subgroup_modulus":P31_ORDER,
        "public_target":point_value(target),
        "caller_only_expected_scalar":7,
        "attempt_limit_per_arm":P31_ATTEMPTS,
        "arms":arms,
        "scope":"structured known-answer target/base; recovery correctness on the declared ordinary prime-field curve",
        "uniform_random_target_tested":false,
        "benchmark_executed":false,
        "complete_cost_comparison":false
    }))
}

fn run_prime31_checks() -> Result<Value, String> {
    let witness = p31_fixture_check()?;
    let f4 = p31_f4_integration_check()?;
    let recovery = p31_recovery_check()?;
    Ok(json!({
        "schema":"shared-s3-matrix.prime31-check/v1",
        "status":"passed",
        "evidence_status":"producer local correctness checks; PCC-unverified",
        "coordinate_modulus":P31,
        "subgroup_modulus":P31_ORDER,
        "curve":p31_curve(),
        "generator":point_value(p31_generator()),
        "historical_point_witness":witness,
        "f4_integration":f4,
        "public_target_recovery":recovery,
        "scope":"structured known-answer target/base and finite-chart relation qualification",
        "uniform_random_target_tested":false,
        "benchmark_executed":false,
        "complete_cost_comparison":false,
        "cost_counter_coverage_complete":false
    }))
}

fn plan() -> Value {
    json!({"schema":"shared-s3-matrix.fixture-plan/v2","status":"planned","mode":"correctness-only","primary_coordinate_modulus":P31,"variables":["x1","x2","x3","u"],"common":["S3(x1,x2,u)","membership(x1)","membership(x2)","membership(x3)"],"parameterized":"S3(u,x3,t)","macaulay_degree":DEGREE,"limits":fixture_limits(),"backends":["integer","fma","split16"],"p31":{"curve":p31_curve(),"subgroup_order":P31_ORDER,"generator":point_value(p31_generator()),"public_target":point_value(p31_public_target()),"factor_base_x":P31_BASE_X,"auxiliary_domain":"full Fp","attempt_limit":P31_ATTEMPTS,"scope":"structured known-answer target/base; no uniform random-target or complete-cost conclusion"},"p17":{"role":"retained exhaustive diagnostic","curve":{"p":17,"a":2,"b":2},"subgroup_order":19,"generator":{"x":5,"y":1},"factor_base_x":TOY_BASE_X,"auxiliary_domain":"full F17","exhaustive_parameters":17,"assignments_per_parameter":459},"checks":["p31 original/fresh/shared F4 solution sets and independent signed-pair reference","p31 public-Q recovery modulo subgroup order with each preconditioner backend","p31 signed finite-prefix replay and historical point witness","retained exhaustive p17 identities and diagnostics"],"benchmark_executed":false,"native_blas_or_gpu":false,"complete_cost_comparison":false,"next_run":"--check-prime31","all_diagnostics":"--check"})
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
    let p31 = run_prime31_checks()?;
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
        [flag] if flag == "--check-prime31" => run_prime31_checks(),
        _ => Err("usage: shared_s3_matrix [--plan|--check|--check-prime31]; timing/benchmark execution is not implemented in this correctness fixture".into()),
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

    #[test]
    fn signed_pair_reference_matches_exhaustive_small_field_roots() {
        let fixture = toy_fixture();
        for scalar in 1..TOY_ORDER {
            let q = fixture
                .curve
                .mul(&mut GroupOps::default(), toy_generator(), scalar);
            assert_eq!(
                fixture.signed_pair_reference(q).unwrap(),
                fixture.direct_solutions(q.x).unwrap()
            );
        }
    }

    #[test]
    fn p31_original_fresh_shared_solutions_and_signed_lifts_agree() {
        let report = p31_f4_integration_check().unwrap();
        assert_eq!(report["coordinate_modulus"], P31);
        assert!(report["solution_count"].as_u64().unwrap() > 0);
        assert_eq!(report["reference_solutions_passed_to_solver"], false);
        assert_eq!(report["original_f4"]["complete"], true);
        for arm in report["arms"].as_array().unwrap() {
            assert_eq!(arm["shared_f4"]["complete"], true);
            assert_eq!(arm["fresh_f4"]["complete"], true);
            assert_eq!(arm["shared_signed_lifts"], report["solution_count"]);
            assert_eq!(arm["fresh_signed_lifts"], report["solution_count"]);
        }
    }

    #[test]
    fn p31_public_target_recovers_modulo_subgroup_for_all_backends() {
        let report = p31_recovery_check().unwrap();
        assert_eq!(report["arms"].as_array().unwrap().len(), BACKENDS.len());
        for arm in report["arms"].as_array().unwrap() {
            assert_eq!(arm["coordinate_modulus"], P31);
            assert_eq!(arm["modulus"], P31_ORDER);
            assert_ne!(arm["coordinate_modulus"], arm["modulus"]);
            assert_eq!(arm["recovered_scalar"], 7);
            assert_eq!(arm["known_answer_passed_to_solver"], false);
            assert_eq!(arm["reference_solutions_passed_to_solver"], false);
            assert_eq!(arm["answer_checked_in_group"], true);
            assert!(arm["relation_count"].as_u64().unwrap() >= 3);
            for attempt in arm["attempts"].as_array().unwrap() {
                if attempt["status"] == "found" {
                    assert_eq!(attempt["f4"]["stopped_early"], true);
                }
            }
        }
    }

    #[test]
    fn p31_solver_uses_supplied_public_point_including_negation() {
        let fixture = p31_fixture();
        let negative = fixture.curve.neg(p31_public_target());
        let report = recover_public(
            &fixture,
            negative,
            p31_generator(),
            P31_ORDER,
            P31_ATTEMPTS,
            Backend::Integer,
        )
        .unwrap();
        assert_eq!(report["recovered_scalar"], P31_ORDER - 7);
        assert_eq!(report["public_target"], point_value(negative));
        assert_eq!(report["answer_checked_in_group"], true);
    }

    #[test]
    fn p31_insufficient_budget_and_wrong_modulus_are_not_successes() {
        let fixture = p31_fixture();
        let insufficient = recover_public(
            &fixture,
            p31_public_target(),
            p31_generator(),
            P31_ORDER,
            1,
            Backend::Integer,
        )
        .unwrap_err();
        assert!(insufficient.contains("did not determine"));
        assert!(insufficient.contains("retained_attempts"));
        assert!(insufficient.contains("\"attempt_limit\":1"));
        let wrong_modulus = recover_public(
            &fixture,
            p31_public_target(),
            p31_generator(),
            P31,
            P31_ATTEMPTS,
            Backend::Integer,
        )
        .unwrap_err();
        assert!(wrong_modulus.contains("generator/order differs"));
    }

    #[test]
    fn p31_noncanonical_public_coordinates_are_rejected() {
        let fixture = p31_fixture();
        let q = p31_public_target();
        for malformed in [
            PrimePoint::affine(q.x + P31, q.y),
            PrimePoint::affine(q.x, q.y + P31),
        ] {
            let failure = recover_public(
                &fixture,
                malformed,
                p31_generator(),
                P31_ORDER,
                P31_ATTEMPTS,
                Backend::Integer,
            )
            .unwrap_err();
            assert!(failure.contains("invalid public target"));
            assert!(fixture.signed_pair_reference(malformed).is_err());
        }
    }

    #[test]
    fn p31_deadline_does_not_become_finite_chart_exhaustion() {
        let fixture = p31_fixture();
        let original = fixture
            .shared(Backend::Integer)
            .unwrap()
            .original_equations(p31_public_target().x)
            .unwrap();
        let mut options = solver_options();
        options.deadline = Some(Instant::now());
        let failure = visit_solutions_with_options(&fixture, &original, &[], &options).unwrap_err();
        assert!(failure.contains("no exhaustion conclusion"));
        assert!(failure.contains("\"timed_out\":true"));
    }
}
