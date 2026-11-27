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
//! cargo run --release --example shared_s3_matrix -- --check-prime31-recovery-arms

use std::collections::{BTreeMap, BTreeSet};
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

#[derive(Clone, Copy, Debug)]
enum RecoveryStrategy {
    Original,
    Fresh(Backend),
    Shared(Backend),
}

const RECOVERY_ARMS: [RecoveryStrategy; 7] = [
    RecoveryStrategy::Original,
    RecoveryStrategy::Fresh(Backend::Integer),
    RecoveryStrategy::Fresh(Backend::Fma),
    RecoveryStrategy::Fresh(Backend::Split16),
    RecoveryStrategy::Shared(Backend::Integer),
    RecoveryStrategy::Shared(Backend::Fma),
    RecoveryStrategy::Shared(Backend::Split16),
];

impl RecoveryStrategy {
    fn name(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Fresh(_) => "fresh",
            Self::Shared(_) => "shared",
        }
    }

    fn backend(self) -> Option<Backend> {
        match self {
            Self::Original => None,
            Self::Fresh(backend) | Self::Shared(backend) => Some(backend),
        }
    }

    fn identity(self) -> Value {
        json!({"strategy":self.name(),"preconditioner_backend":self.backend().map(Backend::name)})
    }
}

#[derive(Clone, Copy)]
struct RecoveryPolicy {
    max_attempts: u64,
    /// A correctness control, never a measured elapsed-time sample.
    expire_f4_deadline: bool,
}

impl RecoveryPolicy {
    fn standard(max_attempts: u64) -> Self {
        Self {
            max_attempts,
            expire_f4_deadline: false,
        }
    }
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

    /// Specialize the original pencil without preparing any Macaulay object.
    /// This common integer construction is used by every recovery arm; its
    /// arithmetic, allocations and polynomial copies are explicitly uncounted.
    fn original_equations(&self, parameter: u64) -> Result<Vec<Poly>, String> {
        if parameter >= self.curve.p {
            return Err("target abscissa must be canonical".into());
        }
        let mut equations = self.common.clone();
        for generator in &self.pencil {
            let mut terms = BTreeMap::<Vec<u32>, u64>::new();
            let mut power = 1;
            for block in generator {
                for (exponents, coefficient) in block {
                    let entry = terms.entry(exponents.clone()).or_default();
                    *entry = (*entry + mul(*coefficient, power, self.curve.p)) % self.curve.p;
                }
                power = mul(power, parameter, self.curve.p);
            }
            let polynomial: Poly = terms
                .into_iter()
                .filter(|(_, coefficient)| *coefficient != 0)
                .collect();
            if !polynomial.is_empty() {
                equations.push(polynomial);
            }
        }
        equations.retain(|polynomial| !polynomial.is_empty());
        Ok(equations)
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

#[allow(clippy::large_enum_variant)]
enum RecoveryPreparation {
    Original,
    Fresh(FreshMacaulay),
    Shared(SharedMacaulay),
}

impl RecoveryPreparation {
    fn prepare(fixture: &FamilyFixture, strategy: RecoveryStrategy) -> Result<Self, String> {
        match strategy {
            RecoveryStrategy::Original => Ok(Self::Original),
            RecoveryStrategy::Fresh(backend) => fixture.fresh(backend).map(Self::Fresh),
            RecoveryStrategy::Shared(backend) => fixture.shared(backend).map(Self::Shared),
        }
    }

    fn setup_report(&self) -> Value {
        match self {
            Self::Original => json!({
                "status":"prepared","strategy":"original","fresh":null,"shared":null,
                "numerical_elimination_performed":false,"numerical_projection_performed":false,
                "macaulay_object_constructed":false,"cost_counter_coverage_complete":false
            }),
            Self::Fresh(fresh) => json!({
                "status":"prepared","strategy":"fresh","fresh":fresh.setup_report(),"shared":null,
                "macaulay_object_constructed":true,"cost_counter_coverage_complete":false
            }),
            Self::Shared(shared) => json!({
                "status":"prepared","strategy":"shared","fresh":null,"shared":shared.setup_report(),
                "macaulay_object_constructed":true,"cost_counter_coverage_complete":false
            }),
        }
    }

    fn specialize(&self, parameter: u64) -> Result<Option<PreparedTarget>, String> {
        match self {
            Self::Original => Ok(None),
            Self::Fresh(fresh) => fresh
                .specialize(parameter)
                .map(Some)
                .map_err(|e| e.to_string()),
            Self::Shared(shared) => shared
                .specialize(parameter)
                .map(Some)
                .map_err(|e| e.to_string()),
        }
    }
}

fn group_phase(ops: GroupOps) -> Value {
    // GroupOps deliberately skips its field ledger during serialization.
    json!({
        "group_operations":ops,"group_field_operations":ops.field_ops,
        "montgomery_reductions":ops.montgomery_reductions,
        "cost_counter_coverage_complete":false
    })
}

struct RelationAttempt {
    status: &'static str,
    witness: Option<Vec<usize>>,
    report: Value,
    lifting_ops: GroupOps,
}

/// Every return retains work already performed, including a preparation error
/// or an interrupted visitor. No complete-chart decision is inferred here.
fn find_relation(
    fixture: &FamilyFixture,
    preparation: &RecoveryPreparation,
    fb: &FactorBase<PrimePoint>,
    target: PrimePoint,
    policy: RecoveryPolicy,
) -> RelationAttempt {
    let mut call = RelationAttempt {
        status: "original_equation_error",
        witness: None,
        report: json!({
            "original_equations":null,"preconditioning":null,"f4":null,
            "original_equation_construction_invoked":true,
            "preconditioning_invoked":false,"f4_invoked":false,
            "original_equations_retained":false,"relation":null,
            "accepted_assignment":null,"accepted_factor_base_indices":null,
            "error":null,"cost_counter_coverage_complete":false
        }),
        lifting_ops: GroupOps::default(),
    };
    let original = match fixture.original_equations(target.x) {
        Ok(original) => original,
        Err(error) => {
            call.report["error"] = json!(error);
            return call;
        }
    };
    call.report["original_equations"] = json!({
        "equations":original.len(),"terms":original.iter().map(Vec::len).sum::<usize>(),
        "arithmetic_counters":null,"construction_arithmetic_counted":false
    });
    call.report["original_equations_retained"] = json!(true);
    call.report["preconditioning_invoked"] =
        json!(!matches!(preparation, RecoveryPreparation::Original));
    let prepared = match preparation.specialize(target.x) {
        Ok(prepared) => prepared,
        Err(error) => {
            call.status = "specialization_error";
            call.report["error"] = json!(error);
            call.report["preconditioning"] = json!({
                "status":"failed","partial_arithmetic_counters":null,
                "reason":"preparation API returns no partial target report on error"
            });
            return call;
        }
    };
    let mut equations = original.clone();
    if let Some(prepared) = prepared {
        call.report["preconditioning"] = json!(prepared.report);
        call.report["consequence_count"] = json!(prepared.consequences.len());
        if matches!(preparation, RecoveryPreparation::Shared(_)) {
            // Compatibility with the historical Shared-only correctness output.
            call.report["shared_target_report"] = json!(prepared.report);
        }
        equations.extend(prepared.consequences);
    } else {
        call.report["consequence_count"] = json!(0);
    }
    let mut options = solver_options();
    if policy.expire_f4_deadline {
        options.deadline = Some(Instant::now());
    }
    call.report["f4_invoked"] = json!(true);
    let outcome = f4_fp::solve_visit(
        &equations,
        VARIABLES,
        fixture.curve.p,
        &options,
        &fixture.domains(),
        |point| {
            if point.len() == VARIABLES
                && point.iter().all(|&x| x < fixture.curve.p)
                && point[..3].iter().all(|x| fixture.factor_x.contains(x))
                && equations_satisfied(&original, point, fixture.curve.p)
            {
                call.witness = lift_chain(&fixture.curve, fb, point, target, &mut call.lifting_ops);
                if let Some(indices) = &call.witness {
                    call.report["accepted_assignment"] = json!(point);
                    call.report["accepted_factor_base_indices"] = json!(indices);
                }
            }
            // A verified witness permits stopping; it does not prove exhaustion.
            call.witness.is_some()
        },
    );
    let report = match outcome {
        Ok(report) => report,
        Err(error) => {
            call.status = "f4_error";
            call.report["error"] = json!(error);
            call.report["f4_report_unavailable"] = json!(true);
            return call;
        }
    };
    call.status = if call.witness.is_some() {
        "found"
    } else if report.complete && !report.timed_out && !report.trace_stopped && !report.stopped_early
    {
        "finite_chart_exhausted"
    } else {
        call.report["error"] =
            json!("relation extraction interrupted; finite-chart exhaustion is unknown");
        "f4_incomplete"
    };
    call.report["f4"] = json!(report);
    call
}

struct RecoveryRecord {
    report: Value,
    attempts: Vec<Value>,
    admission: GroupOps,
    queries: GroupOps,
    lifting: GroupOps,
    replay: GroupOps,
    verification: GroupOps,
    original_calls: u64,
    preconditioner_calls: u64,
    f4_calls: u64,
    relations: u64,
    skipped: u64,
}

impl RecoveryRecord {
    fn new(
        fixture: &FamilyFixture,
        q: PrimePoint,
        g: PrimePoint,
        order: u64,
        policy: RecoveryPolicy,
        strategy: RecoveryStrategy,
    ) -> Self {
        Self {
            report: json!({
                "strategy":strategy.name(),
                "preconditioner_backend":strategy.backend().map(Backend::name),
                "coordinate_modulus":fixture.curve.p,"modulus":order,
                "curve":fixture.curve,"generator":point_value(g),"public_target":point_value(q),
                "factor_base_x":fixture.factor_x,"factor_coordinate_domains":fixture.domains(),
                "auxiliary_domain":"full Fp (None in production visitor)",
                "family":{"variables":["x1","x2","x3","u"],"degree":DEGREE,
                    "common_equations":fixture.common,"target_pencil":fixture.pencil},
                "attempt_limit":policy.max_attempts,
                "query_policy":"Q + [a]G, a=0..attempt_limit; recompute [a]G with counted group mul",
                "solver_policy":{"ordering":"Grevlex","max_degree":DEGREE,"max_pairs_per_step":32,
                    "deadline_guard_seconds":10,"forced_expired_deadline":policy.expire_f4_deadline},
                "f4_backend":"unchanged production integer F4 visitor",
                "relation_linear_algebra":"existing integer IncrementalGauss modulo subgroup order",
                "admission":{"status":"not_completed"},
                "factor_base_construction":{"status":"not_run","arithmetic_counters":null},
                "setup":{"status":"not_run","fresh":null,"shared":null},
                "linear_algebra_phase":{"status":"not_run","work_counters":null},
                "rank":0,"anchor_relations":0,
                "known_answer_passed_to_solver":false,"reference_solutions_passed_to_solver":false,
                "uniform_random_target_tested":false,"benchmark_executed":false,
                "complete_point_decomposition_coverage":false,"cost_counter_coverage_complete":false,
                "scope":"declared structured public-target/base fixtures and finite-chart F4 relations"
            }),
            attempts: Vec::new(),
            admission: GroupOps::default(),
            queries: GroupOps::default(),
            lifting: GroupOps::default(),
            replay: GroupOps::default(),
            verification: GroupOps::default(),
            original_calls: 0,
            preconditioner_calls: 0,
            f4_calls: 0,
            relations: 0,
            skipped: 0,
        }
    }

    fn matrix(&mut self, gauss: &IncrementalGauss) {
        // Merely reporting the matrix must not call pinned(), whose scans count.
        self.report["rank"] = json!(gauss.rank());
        self.report["linear_algebra_row_ops"] = json!(gauss.row_ops);
        self.report["linear_algebra_work"] = json!(gauss.work_counters());
        self.report["linear_algebra_phase"] = json!({
            "status":"started","rank":gauss.rank(),"row_ops":gauss.row_ops,
            "dependent_rows":gauss.dependent,"inconsistent_rows":gauss.inconsistent,
            "pivot_inversions":gauss.pivot_inversions,"work_counters":gauss.work_counters(),
            "coverage":"row_ops includes normalization; pivot_inversion_steps is the existing 2w model, not an executed-loop count",
            "cost_counter_coverage_complete":false
        });
    }

    fn finish(mut self, status: &str, error: Option<&str>, recovered: Option<u64>) -> Value {
        let mut total = GroupOps::default();
        for phase in [
            self.admission,
            self.queries,
            self.lifting,
            self.replay,
            self.verification,
        ] {
            total.merge(phase);
        }
        self.report["status"] = json!(status);
        self.report["stop_reason"] = json!(status);
        self.report["error"] = json!(error);
        self.report["recovered_scalar"] = json!(recovered);
        self.report["answer_checked_in_group"] = json!(status == "recovered");
        self.report["relation_count"] = json!(self.relations);
        self.report["skipped_exceptional_or_base_queries"] = json!(self.skipped);
        self.report["actual_queries"] = json!(self.attempts.len());
        self.report["group_operations"] = json!(total);
        self.report["group_field_operations"] = json!(total.field_ops);
        self.report["group_count_scope"] = json!("target admission, query generation, sign lifting, relation replay and final scalar check; factor-base construction excluded");
        self.report["phase_ledger"] = json!({
            "admission":group_phase(self.admission),
            "factor_base_construction":self.report["factor_base_construction"],
            "preparation":self.report["setup"],
            "query_generation":group_phase(self.queries),
            "original_equation_construction":{"calls":self.original_calls,"arithmetic_counters":null},
            "preconditioning":{"calls":self.preconditioner_calls,"reports":"attempts[].preconditioning"},
            "f4":{"calls":self.f4_calls,"reports":"attempts[].f4"},
            "sign_lifting":group_phase(self.lifting),
            "relation_replay":group_phase(self.replay),
            "relation_linear_algebra":self.report["linear_algebra_phase"],
            "final_verification":group_phase(self.verification),
            "counter_relationships":[
                "group scalar_mults overlaps the adds/doubles used inside scalar multiplication",
                "group field muls and sqrs are separate categories; invs excludes inversion internals",
                "Macaulay logical_products already includes kernel field products; nested kernels are detail, not an additional total",
                "F4 field_ops counts row-reduction products; root_field_multiplications is separate; domain_evaluations counts events"
            ],
            "counter_exclusions":[
                "is_on_curve/rhs, field additions/subtractions/negations/comparisons and inversion internals",
                "factor-base roots, curve RHS work, sign/index construction and lookup comparisons",
                "original-equation specialization and polynomial copies/normalization/evaluation for signed witness admission",
                "preparation validation/primality testing, monomial enumeration, sorting, allocation and unreturned partial error costs",
                "floating quotient floors, conditional remainder corrections and other uncounted instructions",
                "complete F4 and extraction cost, relation-row assembly, process memory and complete CPU work"
            ],
            "cold_boundary_exclusions":[
                "FamilyFixture construction precedes each recovery invocation and has no per-arm cost counters",
                "caller-only target construction and correctness/reference checks are outside recovery"
            ],
            "memory_scope":"Macaulay payload bounds and F4 dimensions are not measured process RSS",
            "timing_samples_collected":false,"cost_counter_coverage_complete":false
        });
        self.report["attempts"] = json!(self.attempts);
        self.report
    }
}

/// Public-input recovery shared by all preparation strategies. Expected scalars,
/// factor-base logarithms and reference roots never enter this boundary.
fn recover_public_arm(
    fixture: &FamilyFixture,
    q: PrimePoint,
    g: PrimePoint,
    order: u64,
    policy: RecoveryPolicy,
    strategy: RecoveryStrategy,
) -> Value {
    let curve = &fixture.curve;
    let mut record = RecoveryRecord::new(fixture, q, g, order, policy, strategy);
    match fixture.declared_group() {
        Ok(declared) if declared == (g, order) => {}
        _ => {
            return record.finish(
                "admission_error",
                Some("generator/order differs from the declared fixture"),
                None,
            )
        }
    }
    if policy.max_attempts == 0 || policy.max_attempts > 32 || policy.max_attempts > order {
        return record.finish(
            "admission_error",
            Some("fixture requires an explicit attempt bound between 1 and min(32, order)"),
            None,
        );
    }
    if g.infinity
        || g.x >= curve.p
        || g.y >= curve.p
        || !curve.is_on_curve(g)
        || !curve.mul(&mut record.admission, g, order).infinity
    {
        return record.finish("admission_error", Some("invalid fixture generator"), None);
    }
    if q.infinity
        || q.x >= curve.p
        || q.y >= curve.p
        || !curve.is_on_curve(q)
        || !curve.mul(&mut record.admission, q, order).infinity
    {
        return record.finish("admission_error", Some("invalid public target"), None);
    }
    record.report["admission"] = json!({"status":"passed","canonical_public_coordinates":true});
    let fb = match factor_base(curve, &fixture.factor_x, order) {
        Ok(fb) => fb,
        Err(error) => {
            record.report["factor_base_construction"] =
                json!({"status":"failed","error":error,"partial_arithmetic_counters":null});
            return record.finish("factor_base_error", Some(&error), None);
        }
    };
    record.report["factor_base_construction"] = json!({
        "status":"completed","abscissae":fixture.factor_x.len(),"points":fb.points.len(),
        "columns":fb.columns,"arithmetic_counters":null,
        "coverage":"construction performed independently in this arm; its arithmetic and indexing are uncounted"
    });
    record.report["factor_base_points"] = json!(fb
        .points
        .iter()
        .copied()
        .map(point_value)
        .collect::<Vec<_>>());
    record.report["factor_base_columns"] = json!(fb.col_of);
    record.report["factor_base_coefficients"] = json!(fb.coef_of);
    record.report["target_column"] = json!(fb.columns);
    let preparation = match RecoveryPreparation::prepare(fixture, strategy) {
        Ok(preparation) => preparation,
        Err(error) => {
            record.report["setup"] = json!({
                "status":"failed","strategy":strategy.name(),"error":error,
                "fresh":null,"shared":null,"partial_arithmetic_counters":null,
                "reason":"setup API returns no partial report on error"
            });
            return record.finish("setup_error", Some(&error), None);
        }
    };
    record.report["setup"] = preparation.setup_report();
    record.report["scoped_setup_report"] = match &preparation {
        RecoveryPreparation::Original => preparation.setup_report(),
        RecoveryPreparation::Fresh(fresh) => json!(fresh.setup_report()),
        RecoveryPreparation::Shared(shared) => json!(shared.setup_report()),
    };
    let target_column = fb.columns;
    let mut gauss = IncrementalGauss::new(fb.columns + 1, order);
    let Some(&gi) = fb.point_index.get(&curve.key(&g)) else {
        record.matrix(&gauss);
        return record.finish("anchor_error", Some("missing G anchor"), None);
    };
    let mut anchor = vec![0; fb.columns + 1];
    anchor[fb.col_of[gi]] = fb.coef_of[gi];
    let anchor_status = gauss.add_row(anchor, 1);
    record.matrix(&gauss);
    if anchor_status == RowStatus::Inconsistent {
        return record.finish("anchor_error", Some("G anchor contradicted itself"), None);
    }
    record.report["anchor_relations"] = json!(1);
    record.report["anchor_column"] = json!(fb.col_of[gi]);
    record.report["anchor_coefficient"] = json!(fb.coef_of[gi]);
    for a in 0..policy.max_attempts {
        let mut query_ops = GroupOps::default();
        let ag = curve.mul(&mut query_ops, g, a);
        let target = curve.add(&mut query_ops, ag, q);
        record.queries.merge(query_ops);
        let rank_before = gauss.rank();
        if target.infinity || fb.point_index.contains_key(&curve.key(&target)) {
            record.skipped += 1;
            record.attempts.push(json!({
                "a":a,"b":1,"target":point_value(target),
                "status":if target.infinity {"skipped_identity"} else {"skipped_direct_base"},
                "rank_before":rank_before,"rank_after":rank_before,
                "query_generation":group_phase(query_ops),"relation":null,"preconditioning":null,"f4":null,
                "reason":"direct/exceptional lookup is excluded from this finite-chart three-term fixture"
            }));
            continue;
        }
        let RelationAttempt {
            status,
            witness,
            mut report,
            lifting_ops,
        } = find_relation(fixture, &preparation, &fb, target, policy);
        record.original_calls +=
            u64::from(report["original_equation_construction_invoked"] == true);
        record.preconditioner_calls += u64::from(report["preconditioning_invoked"] == true);
        record.f4_calls += u64::from(report["f4_invoked"] == true);
        record.lifting.merge(lifting_ops);
        report["a"] = json!(a);
        report["b"] = json!(1);
        report["target"] = point_value(target);
        report["status"] = json!(status);
        report["rank_before"] = json!(rank_before);
        report["rank_after"] = json!(rank_before);
        report["query_generation"] = group_phase(query_ops);
        report["sign_lifting"] = group_phase(lifting_ops);
        if !matches!(status, "found" | "finite_chart_exhausted") {
            let error = report["error"]
                .as_str()
                .unwrap_or("relation extraction failed")
                .to_owned();
            record.attempts.push(report);
            record.matrix(&gauss);
            return record.finish(status, Some(&error), None);
        }
        if let Some(indices) = witness {
            let mut row = vec![0; fb.columns + 1];
            let mut replay_ops = GroupOps::default();
            let mut replay = curve.identity();
            for &index in &indices {
                row[fb.col_of[index]] = (row[fb.col_of[index]] + fb.coef_of[index]) % order;
                replay = curve.add(&mut replay_ops, replay, fb.points[index]);
            }
            record.replay.merge(replay_ops);
            report["relation_replay"] = group_phase(replay_ops);
            report["replayed_point"] = point_value(replay);
            report["replay_passed"] = json!(replay == target);
            report["candidate_factor_row"] = json!(row);
            if replay != target {
                record.attempts.push(report);
                record.matrix(&gauss);
                return record.finish(
                    "relation_error",
                    Some("relation failed point-sum replay before linear algebra"),
                    None,
                );
            }
            row[target_column] = order - 1;
            report["relation"] = json!({"factor_base_indices":indices,"row":row,"rhs":a});
            let row_status = gauss.add_row(row, a);
            report["row_status"] = json!(format!("{row_status:?}"));
            report["rank_after"] = json!(gauss.rank());
            record.relations += 1;
            if row_status == RowStatus::Inconsistent {
                record.attempts.push(report);
                record.matrix(&gauss);
                return record.finish(
                    "relation_error",
                    Some("relation matrix inconsistent modulo subgroup order"),
                    None,
                );
            }
        }
        // Exactly one decision call, then capture its scan work in the ledger.
        let recovered = gauss.pinned(target_column);
        record.matrix(&gauss);
        record.attempts.push(report);
        if let Some(recovered) = recovered {
            if curve.mul(&mut record.verification, g, recovered) != q {
                record.report["candidate_scalar"] = json!(recovered);
                return record.finish(
                    "verification_error",
                    Some("recovered scalar failed public target check"),
                    None,
                );
            }
            return record.finish("recovered", None, Some(recovered));
        }
    }
    record.matrix(&gauss);
    record.finish(
        "query_budget_exhausted",
        Some("bounded fixture relations did not determine the target logarithm"),
        None,
    )
}

/// Preserve the historical Shared-only entry point and Result contract.
fn recover_public(
    fixture: &FamilyFixture,
    q: PrimePoint,
    g: PrimePoint,
    order: u64,
    max_attempts: u64,
    backend: Backend,
) -> Result<Value, String> {
    let report = recover_public_arm(
        fixture,
        q,
        g,
        order,
        RecoveryPolicy::standard(max_attempts),
        RecoveryStrategy::Shared(backend),
    );
    if report["status"] == "recovered" {
        Ok(report)
    } else {
        Err(format!(
            "{}; retained_attempts and full report: {}",
            report["error"].as_str().unwrap_or("public recovery failed"),
            report
        ))
    }
}

/// The matched arm's enclosing boundary includes family construction. A future
/// cold timer may enclose this whole call; this fixture records no timing.
#[allow(clippy::too_many_arguments)]
fn recover_public_arm_from_parameters(
    curve: PrimeCurve,
    factor_x: &[u64],
    q: PrimePoint,
    g: PrimePoint,
    order: u64,
    policy: RecoveryPolicy,
    strategy: RecoveryStrategy,
) -> Value {
    let fixture = match FamilyFixture::new(curve.clone(), factor_x.to_vec()) {
        Ok(fixture) => fixture,
        Err(error) => {
            return json!({
                "status":"family_construction_error","stop_reason":"family_construction_error",
                "error":error,"strategy":strategy.name(),
                "preconditioner_backend":strategy.backend().map(Backend::name),
                "coordinate_modulus":curve.p,"modulus":order,"public_target":point_value(q),
                "attempt_limit":policy.max_attempts,"attempts":[],"recovered_scalar":null,
                "answer_checked_in_group":false,"known_answer_passed_to_solver":false,
                "setup":{"status":"not_run","fresh":null,"shared":null},
                "phase_ledger":{"family_construction":{"status":"failed","partial_arithmetic_counters":null}},
                "benchmark_executed":false,"cost_counter_coverage_complete":false
            });
        }
    };
    let mut report = recover_public_arm(&fixture, q, g, order, policy, strategy);
    report["phase_ledger"]["family_construction"] = json!({
        "status":"completed_inside_arm_entry","arithmetic_counters":null,
        "coverage":"S3 pencil and factor membership polynomials constructed in this arm; arithmetic and allocation unpriced"
    });
    report["phase_ledger"]["cold_boundary_exclusions"] = json!([
        "caller-only target construction and correctness/reference checks are outside recovery"
    ]);
    report["cold_enclosing_entry"] = json!("public parameters -> family construction -> admission -> factor base -> preparation -> queries/relations/LA -> verification");
    report
}

#[allow(clippy::too_many_arguments)]
fn execute_recovery_arms(
    curve: PrimeCurve,
    factor_x: &[u64],
    q: PrimePoint,
    g: PrimePoint,
    order: u64,
    policies: &[RecoveryPolicy; RECOVERY_ARMS.len()],
) -> Vec<Value> {
    // No early propagation or success-dependent scheduling: a failed arm
    // cannot suppress another arm. Calls are sequential for F4 counters.
    RECOVERY_ARMS
        .iter()
        .copied()
        .zip(policies.iter().copied())
        .enumerate()
        .map(|(index, (strategy, policy))| {
            let mut report = recover_public_arm_from_parameters(
                curve.clone(),
                factor_x,
                q,
                g,
                order,
                policy,
                strategy,
            );
            report["planned_arm_index"] = json!(index);
            report
        })
        .collect()
}

fn compare_query_prefixes(arms: &[Value]) -> Value {
    let mut identities = BTreeMap::new();
    let mut consistent = true;
    let mut repeated_query_checks = 0;
    let lengths: Vec<_> = arms
        .iter()
        .map(|arm| arm["attempts"].as_array().map_or(0, Vec::len))
        .collect();
    for arm in arms {
        if let Some(attempts) = arm["attempts"].as_array() {
            for (index, attempt) in attempts.iter().enumerate() {
                let identity =
                    json!({"a":attempt["a"],"b":attempt["b"],"target":attempt["target"]});
                consistent &= attempt["a"].as_u64() == Some(index as u64) && attempt["b"] == 1;
                if let Some(prior) = identities.insert(index, identity.clone()) {
                    repeated_query_checks += 1;
                    consistent &= prior == identity;
                }
            }
        }
    }
    json!({
        "same_identity_at_each_overlapping_query":consistent,
        "repeated_query_checks":repeated_query_checks,
        "actual_query_counts":lengths,
        "identical_stopping_prefix_lengths":lengths.windows(2).all(|pair| pair[0] == pair[1]),
        "qualification":"same policy and overlapping queries; valid witnesses may change rank progress and stopping prefixes"
    })
}

fn arm_structure_valid(index: usize, arm: &Value) -> bool {
    let strategy = RECOVERY_ARMS[index];
    arm["planned_arm_index"].as_u64() == Some(index as u64)
        && arm["strategy"] == strategy.name()
        && arm["preconditioner_backend"] == json!(strategy.backend().map(Backend::name))
        && arm["known_answer_passed_to_solver"] == false
        && arm["cost_counter_coverage_complete"] == false
        && (matches!(strategy, RecoveryStrategy::Shared(_)) || arm["setup"]["shared"].is_null())
}

fn checked_recovery_panel(label: &str, arms: Vec<Value>, expected: u64) -> Value {
    // The held answer is used only here, after all public-input calls finish.
    let query_comparison = compare_query_prefixes(&arms);
    let passed = arms.len() == RECOVERY_ARMS.len()
        && arms.iter().enumerate().all(|(index, arm)| {
            arm_structure_valid(index, arm)
                && arm["status"] == "recovered"
                && arm["recovered_scalar"].as_u64() == Some(expected)
                && arm["answer_checked_in_group"] == true
        })
        && query_comparison["same_identity_at_each_overlapping_query"] == true;
    json!({
        "label":label,"passed":passed,"caller_only_expected_scalar":expected,
        "planned_arm_count":RECOVERY_ARMS.len(),"retained_arm_count":arms.len(),
        "query_comparison":query_comparison,"arms":arms
    })
}

fn checked_failure_control(
    label: &str,
    arms: Vec<Value>,
    expected_first_status: &str,
    expected_other_scalar: u64,
) -> Value {
    let query_comparison = compare_query_prefixes(&arms);
    let passed = arms.len() == RECOVERY_ARMS.len()
        && arms.iter().enumerate().all(|(index, arm)| {
            arm_structure_valid(index, arm)
                && if index == 0 {
                    arm["status"] == expected_first_status
                        && arm["recovered_scalar"].is_null()
                        && arm["answer_checked_in_group"] == false
                        && arm["attempts"]
                            .as_array()
                            .is_some_and(|calls| !calls.is_empty())
                        && (expected_first_status != "f4_incomplete"
                            || (arm["attempts"][0]["f4"]["timed_out"] == true
                                && arm["attempts"][0]["f4"]["complete"] == false
                                && arm["attempts"][0]["status"] != "finite_chart_exhausted"))
                } else {
                    arm["status"] == "recovered"
                        && arm["recovered_scalar"].as_u64() == Some(expected_other_scalar)
                        && arm["answer_checked_in_group"] == true
                }
        })
        && query_comparison["same_identity_at_each_overlapping_query"] == true;
    json!({
        "label":label,"passed":passed,"expected_first_status":expected_first_status,
        "caller_only_expected_other_scalar":expected_other_scalar,
        "planned_arm_count":RECOVERY_ARMS.len(),"retained_arm_count":arms.len(),
        "query_comparison":query_comparison,"arms":arms
    })
}

fn run_matched_recovery_checks() -> Value {
    let standard = [RecoveryPolicy::standard(P31_ATTEMPTS); RECOVERY_ARMS.len()];
    let mut prime31 = Vec::new();
    for (label, target, expected) in [
        ("positive_public_target", p31_public_target(), 7),
        (
            "negated_public_target",
            p31_curve().neg(p31_public_target()),
            P31_ORDER - 7,
        ),
    ] {
        let arms = execute_recovery_arms(
            p31_curve(),
            &P31_BASE_X,
            target,
            p31_generator(),
            P31_ORDER,
            &standard,
        );
        prime31.push(checked_recovery_panel(label, arms, expected));
    }
    let mut capped = standard;
    capped[0].max_attempts = 1;
    let cap_control = checked_failure_control(
        "first_arm_query_budget_then_remaining_normal_arms",
        execute_recovery_arms(
            p31_curve(),
            &P31_BASE_X,
            p31_public_target(),
            p31_generator(),
            P31_ORDER,
            &capped,
        ),
        "query_budget_exhausted",
        7,
    );
    let mut interrupted = standard;
    interrupted[0].expire_f4_deadline = true;
    let interruption_control = checked_failure_control(
        "first_arm_expired_deadline_then_remaining_normal_arms",
        execute_recovery_arms(
            p31_curve(),
            &P31_BASE_X,
            p31_public_target(),
            p31_generator(),
            P31_ORDER,
            &interrupted,
        ),
        "f4_incomplete",
        7,
    );
    let passed = prime31.iter().all(|panel| panel["passed"] == true)
        && cap_control["passed"] == true
        && interruption_control["passed"] == true;
    json!({
        "schema":"shared-s3-matrix.matched-public-recovery/v1",
        "status":if passed {"passed"} else {"incomplete_or_failed"},
        "evidence_status":"producer correctness qualification; PCC-unverified",
        "coordinate_modulus":P31,"subgroup_modulus":P31_ORDER,
        "planned_arms":RECOVERY_ARMS.iter().copied().map(RecoveryStrategy::identity).collect::<Vec<_>>(),
        "all_planned_arm_outcomes_retained":true,"prime31":prime31,
        "exhaustive_F17":{"execution":"separate unit test, not executed by this CLI mode",
            "test":"matched_arms_recover_every_nonidentity_f17_public_target",
            "identity_policy":"rejected by the strict public-target admission boundary"},
        "controls":{"query_budget":cap_control,"interruption":interruption_control},
        "scope":"matched correctness on the declared structured Prime31 public targets and base",
        "floating_point_role":"preconditioner only; F4 and relation LA remain integer",
        "complete_point_decomposition_coverage":false,"uniform_random_target_tested":false,
        "benchmark_executed":false,"timing_samples_collected":false,
        "complete_cost_comparison":false,"cost_counter_coverage_complete":false
    })
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
    json!({
        "schema":"shared-s3-matrix.fixture-plan/v3","status":"planned","mode":"correctness-only",
        "primary_coordinate_modulus":P31,"variables":["x1","x2","x3","u"],
        "common":["S3(x1,x2,u)","membership(x1)","membership(x2)","membership(x3)"],
        "parameterized":"S3(u,x3,t)","macaulay_degree":DEGREE,"limits":fixture_limits(),
        "backends":["integer","fma","split16"],
        "p31":{"curve":p31_curve(),"subgroup_order":P31_ORDER,
            "generator":point_value(p31_generator()),"public_target":point_value(p31_public_target()),
            "factor_base_x":P31_BASE_X,"auxiliary_domain":"full Fp","attempt_limit":P31_ATTEMPTS,
            "scope":"structured known-answer target/base; no uniform random-target or complete-cost conclusion"},
        "p17":{"role":"retained exhaustive diagnostic","curve":{"p":17,"a":2,"b":2},
            "subgroup_order":TOY_ORDER,"generator":point_value(toy_generator()),"factor_base_x":TOY_BASE_X,
            "auxiliary_domain":"full F17","exhaustive_parameters":17,"assignments_per_parameter":459},
        "checks":["p31 original/fresh/shared F4 solution sets and independent signed-pair reference",
            "matched Original/Fresh/Shared public-Q recovery, both signs and cap/interruption controls",
            "p31 signed finite-prefix replay and historical point witness",
            "retained exhaustive p17 identities and diagnostics",
            "separate unit test: all 18 nonidentity F17 public targets across seven recovery arms"],
        "planned_recovery_arms":RECOVERY_ARMS.iter().copied().map(RecoveryStrategy::identity).collect::<Vec<_>>(),
        "benchmark_executed":false,"native_blas_or_gpu":false,"complete_cost_comparison":false,
        "next_run":"--check-prime31-recovery-arms",
        "legacy_diagnostics":"--check",
        "all_diagnostics":["--check","--check-prime31-recovery-arms",
            "cargo test --release --example shared_s3_matrix -- --test-threads=1 --nocapture"]
    })
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
        [flag] if flag == "--check-prime31-recovery-arms" => Ok(run_matched_recovery_checks()),
        _ => Err("usage: shared_s3_matrix [--plan|--check|--check-prime31|--check-prime31-recovery-arms]; timing/benchmark execution is not implemented in this correctness fixture".into()),
    };
    match result {
        Ok(value) => {
            // Full matched-arm records are emitted even on a failed check.
            // Compact JSON changes only whitespace, not evidence or outcomes.
            if arguments
                .first()
                .is_some_and(|arg| arg == "--check-prime31-recovery-arms")
            {
                println!("{}", serde_json::to_string(&value).unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(&value).unwrap());
            }
            if value["status"] == "incomplete_or_failed" {
                std::process::exit(1);
            }
        }
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

    fn assert_retained_assignments(fixture: &FamilyFixture, arm: &Value, order: u64) {
        let fb = factor_base(&fixture.curve, &fixture.factor_x, order).unwrap();
        for attempt in arm["attempts"].as_array().unwrap() {
            if attempt["status"] != "found" {
                continue;
            }
            let assignment: Vec<u64> =
                serde_json::from_value(attempt["accepted_assignment"].clone()).unwrap();
            let indices: Vec<usize> =
                serde_json::from_value(attempt["accepted_factor_base_indices"].clone()).unwrap();
            assert_eq!(assignment.len(), VARIABLES);
            assert_eq!(indices.len(), 3);
            assert!(assignment.iter().all(|&x| x < fixture.curve.p));
            for (position, &index) in indices.iter().enumerate() {
                assert_eq!(fb.points[index].x, assignment[position]);
            }
            let target = PrimePoint::affine(
                attempt["target"]["x"].as_u64().unwrap(),
                attempt["target"]["y"].as_u64().unwrap(),
            );
            let original = fixture.original_equations(target.x).unwrap();
            assert!(equations_satisfied(&original, &assignment, fixture.curve.p));
            let mut ops = GroupOps::default();
            let prefix = fixture
                .curve
                .add(&mut ops, fb.points[indices[0]], fb.points[indices[1]]);
            assert!(!prefix.infinity);
            assert_eq!(prefix.x, assignment[3]);
            assert_eq!(
                fixture.curve.add(&mut ops, prefix, fb.points[indices[2]]),
                target
            );
            assert_eq!(attempt["replayed_point"], attempt["target"]);
            assert_eq!(attempt["replay_passed"], true);
            assert_eq!(
                attempt["relation"]["factor_base_indices"],
                attempt["accepted_factor_base_indices"]
            );
        }
    }

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

    #[test]
    fn matched_prime31_arms_recover_both_signs_and_retain_control_failures() {
        let report = run_matched_recovery_checks();
        let fixture = p31_fixture();
        assert_eq!(report["status"], "passed", "{report}");
        assert_eq!(report["all_planned_arm_outcomes_retained"], true);
        for panel in report["prime31"].as_array().unwrap() {
            for arm in panel["arms"].as_array().unwrap() {
                assert_retained_assignments(&fixture, arm, P31_ORDER);
                assert_eq!(
                    arm["phase_ledger"]["family_construction"]["status"],
                    "completed_inside_arm_entry"
                );
                assert_eq!(arm["coordinate_modulus"], P31);
                assert_eq!(arm["modulus"], P31_ORDER);
                assert_eq!(arm["cost_counter_coverage_complete"], false);
            }
        }
        let interrupted = &report["controls"]["interruption"]["arms"][0];
        assert_eq!(interrupted["attempts"][0]["f4"]["f4_runs"], 0);
        assert_eq!(interrupted["attempts"][0]["f4"]["solutions_visited"], 0);
        assert_eq!(interrupted["attempts"][0]["f4"]["stopped_early"], false);
    }

    #[test]
    fn matched_arms_recover_every_nonidentity_f17_public_target() {
        let curve = PrimeCurve { p: 17, a: 2, b: 2 };
        let policies = [RecoveryPolicy::standard(TOY_ORDER); RECOVERY_ARMS.len()];
        let mut panels = Vec::new();
        // Retain and finish every panel before asserting; the expected scalar
        // is used only by the caller after all seven public-input calls.
        for expected in 1..TOY_ORDER {
            let q = curve.mul(&mut GroupOps::default(), toy_generator(), expected);
            let panel = checked_recovery_panel(
                "finite_public_F17_target",
                execute_recovery_arms(
                    curve.clone(),
                    &TOY_BASE_X,
                    q,
                    toy_generator(),
                    TOY_ORDER,
                    &policies,
                ),
                expected,
            );
            let outcomes: Vec<_> = panel["arms"].as_array().unwrap().iter().map(|arm| {
                json!({"arm":arm["planned_arm_index"],"status":arm["status"],
                    "recovered_scalar":arm["recovered_scalar"],"rank":arm["rank"],
                    "actual_queries":arm["actual_queries"],"verified":arm["answer_checked_in_group"]})
            }).collect();
            println!(
                "{}",
                json!({"f17_caller_expected":expected,"passed":panel["passed"],
                "outcomes":outcomes,"query_comparison":panel["query_comparison"]})
            );
            panels.push(panel);
        }
        assert_eq!(panels.len(), (TOY_ORDER - 1) as usize);
        let failures: Vec<_> = panels
            .iter()
            .filter(|panel| panel["passed"] != true)
            .collect();
        assert!(
            failures.is_empty(),
            "failed F17 panels with full outcomes: {}",
            json!(failures)
        );
        let fixture = toy_fixture();
        for panel in &panels {
            for arm in panel["arms"].as_array().unwrap() {
                assert_retained_assignments(&fixture, arm, TOY_ORDER);
            }
        }
    }

    #[test]
    fn matched_prepared_arms_keep_setup_on_interruption() {
        let mut policy = RecoveryPolicy::standard(P31_ATTEMPTS);
        policy.expire_f4_deadline = true;
        let arms: Vec<_> = [
            RecoveryStrategy::Fresh(Backend::Integer),
            RecoveryStrategy::Shared(Backend::Integer),
        ]
        .into_iter()
        .map(|strategy| {
            recover_public_arm_from_parameters(
                p31_curve(),
                &P31_BASE_X,
                p31_public_target(),
                p31_generator(),
                P31_ORDER,
                policy,
                strategy,
            )
        })
        .collect();
        // Both calls finish before validation; retain complete interrupted
        // preparation records even though native F4 never starts.
        for arm in &arms {
            println!("prepared_interruption_control={arm}");
        }
        for arm in &arms {
            assert_eq!(arm["status"], "f4_incomplete", "{arm}");
            assert_eq!(arm["setup"]["status"], "prepared");
            assert_eq!(arm["attempts"][0]["f4"]["f4_runs"], 0);
            assert_eq!(arm["attempts"][0]["f4"]["timed_out"], true);
            assert!(!arm["attempts"][0]["preconditioning"].is_null());
            if arm["strategy"] == "fresh" {
                assert!(arm["setup"]["shared"].is_null());
                assert!(!arm["setup"]["fresh"].is_null());
            } else {
                assert!(arm["setup"]["fresh"].is_null());
                assert!(!arm["setup"]["shared"].is_null());
            }
        }
    }

    #[test]
    fn matched_arm_admission_errors_do_not_suppress_other_arms() {
        let policies = [RecoveryPolicy::standard(P31_ATTEMPTS); RECOVERY_ARMS.len()];
        let arms = execute_recovery_arms(
            p31_curve(),
            &P31_BASE_X,
            p31_public_target(),
            p31_generator(),
            P31,
            &policies,
        );
        assert_eq!(arms.len(), RECOVERY_ARMS.len());
        for (index, arm) in arms.iter().enumerate() {
            assert!(arm_structure_valid(index, arm));
            assert_eq!(arm["status"], "admission_error");
            assert_eq!(arm["setup"]["status"], "not_run");
            assert_eq!(arm["actual_queries"], 0);
            assert_eq!(arm["answer_checked_in_group"], false);
        }
    }

    #[test]
    fn matched_arm_family_errors_keep_all_planned_identities() {
        let policies = [RecoveryPolicy::standard(P31_ATTEMPTS); RECOVERY_ARMS.len()];
        let arms = execute_recovery_arms(
            p31_curve(),
            &[],
            p31_public_target(),
            p31_generator(),
            P31_ORDER,
            &policies,
        );
        assert_eq!(arms.len(), RECOVERY_ARMS.len());
        for (index, arm) in arms.iter().enumerate() {
            assert!(arm_structure_valid(index, arm));
            assert_eq!(arm["status"], "family_construction_error");
            assert_eq!(
                arm["phase_ledger"]["family_construction"]["status"],
                "failed"
            );
            assert!(
                arm["phase_ledger"]["family_construction"]["partial_arithmetic_counters"].is_null()
            );
            assert_eq!(arm["attempts"].as_array().unwrap().len(), 0);
        }
    }
}
