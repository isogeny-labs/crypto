//! Offline mathematical replay of one native JSONL cell. This module never
//! calls production curve arithmetic, the oracle, or IncrementalGauss.
//! Negative-search completeness, operation counts, timings and execution
//! provenance are separate claims, not established by this witness replay.

use crate::bases::{self, BaseDefinition, BaseFamily, CurveDefinition, PointDefinition};
use crate::collector::{CollectionLimits, PublicTarget};
use crate::kernel_fibers::{KernelChoice, KernelFactorization, KernelFiber, KernelStep};
use crate::oracle::{OracleResult, OracleStatus, Presentation, TowerDescription};
use crate::plain_reference::PlainReferenceBinding;
use crate::target_transport::{PreparedBase, TargetArm};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
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
fn choice_plain_control(cell: &Value) -> Result<bool, String> {
    if cell.get("plain_control").is_some() {
        boolean(cell, "plain_control")
    } else {
        Ok(false)
    }
}
fn choice_v2(cell: &Value) -> bool {
    matches!(
        cell["schema"].as_str(),
        Some("prime31-isogeny-choice.cell/v2" | "prime31-isogeny-choice.cell/v3")
    )
}
fn choice_v3(cell: &Value) -> bool {
    cell["schema"] == "prime31-isogeny-choice.cell/v3"
}
fn choice_recovery(cell: &Value) -> Result<&str, String> {
    match cell.get("recovery_mode").and_then(Value::as_str) {
        None if cell.get("recovery_mode").is_none() => Ok("dual_matrices"),
        Some("dual_matrices") => Ok("dual_matrices"),
        Some("augmented_only") => Ok("augmented_only"),
        _ => Err("unsupported recovery implementation".into()),
    }
}
fn choice_factorization(cell: &Value) -> Result<KernelFactorization, String> {
    if cell.get("factorization").is_some() {
        decode(cell, "factorization")
    } else {
        Ok(KernelFactorization::Standard)
    }
}
fn choice_backend(cell: &Value) -> Result<&str, String> {
    match cell.get("oracle_backend") {
        None => Ok("polynomial"),
        Some(value) => match value.as_str() {
            Some("polynomial") => Ok("polynomial"),
            Some("prepared_polynomial") => Ok("prepared_polynomial"),
            Some("prepared_fiber") => Ok("prepared_fiber"),
            Some("pair_table") => Ok("pair_table"),
            Some("specialized_pair_table") => Ok("specialized_pair_table"),
            _ => Err("unsupported frozen decomposition backend".into()),
        },
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

// Independent replay of the fixed isogeny-choice setup. These functions never
// call kernel_fibers::build_fiber, target_transport::prepare, image_point, or the
// production curve group law. The imported types are serialization contracts.
const CHOICE_P: u64 = 2_147_483_647;
const CHOICE_N: u64 = 2_147_446_831;
const CHOICE_ALPHA: u64 = 533_591_203;

fn affine(x: u64, y: u64) -> PointDefinition {
    PointDefinition {
        x,
        y,
        infinity: false,
    }
}
fn identity() -> PointDefinition {
    PointDefinition {
        x: 0,
        y: 0,
        infinity: true,
    }
}
fn canonical_point(q: PointDefinition, p: u64) -> bool {
    if q.infinity {
        q.x == 0 && q.y == 0
    } else {
        q.x < p && q.y < p
    }
}
fn point_key(q: PointDefinition) -> (bool, u64, u64) {
    (q.infinity, q.x, q.y)
}
fn csub(a: u64, b: u64) -> u64 {
    (a + CHOICE_P - b) % CHOICE_P
}
fn cmul(a: u64, b: u64) -> u64 {
    mul(a, b, CHOICE_P)
}
fn independent_rhs(e: &CurveDefinition, x: u64) -> u64 {
    (cmul(cmul(x, x), x) + cmul(e.a, x) + e.b) % CHOICE_P
}
fn source_curve() -> CurveDefinition {
    CurveDefinition {
        p: CHOICE_P,
        a: 847_514_846,
        b: 2_087_444_724,
    }
}
fn choice_map_parameters() -> Result<(CurveDefinition, u64, u64), String> {
    let e = source_curve();
    let x = CHOICE_ALPHA;
    let x2 = cmul(x, x);
    let x4 = cmul(x2, x2);
    let psi3 = (cmul(3, x4) + cmul(cmul(6, e.a), x2) + cmul(cmul(12, e.b), x)) % CHOICE_P;
    let rhs = independent_rhs(&e, x);
    if csub(psi3, cmul(e.a, e.a)) != 0 || power(rhs, (CHOICE_P - 1) / 2, CHOICE_P) != CHOICE_P - 1 {
        return Err("independent target-isogeny kernel certificate failed".into());
    }
    let t = cmul(2, (cmul(3, x2) + e.a) % CHOICE_P);
    let u = cmul(4, rhs);
    let codomain = CurveDefinition {
        p: CHOICE_P,
        a: csub(e.a, cmul(5, t)),
        b: csub(e.b, cmul(7, (u + cmul(x, t)) % CHOICE_P)),
    };
    Ok((codomain, t, u))
}
fn independent_image_x(x: u64, t: u64, u: u64) -> Result<Option<(u64, u64)>, String> {
    if x >= CHOICE_P {
        return Err("noncanonical source isogeny abscissa".into());
    }
    if x == CHOICE_ALPHA {
        return Ok(None);
    }
    let inv = power(csub(x, CHOICE_ALPHA), CHOICE_P - 2, CHOICE_P);
    let inv2 = cmul(inv, inv);
    let inv3 = cmul(inv2, inv);
    Ok(Some((
        (x + cmul(t, inv) + cmul(u, inv2)) % CHOICE_P,
        csub(csub(1, cmul(t, inv2)), cmul(cmul(2, u), inv3)),
    )))
}
fn independent_image(q: PointDefinition, t: u64, u: u64) -> Result<PointDefinition, String> {
    if !canonical_point(q, CHOICE_P) || !bases::independent_on_curve(&source_curve(), q) {
        return Err("invalid source point in independent isogeny replay".into());
    }
    if q.infinity {
        return Ok(identity());
    }
    let (x, derivative) = independent_image_x(q.x, t, u)?
        .ok_or("rational source point unexpectedly lies in target-isogeny kernel")?;
    Ok(affine(x, cmul(q.y, derivative)))
}
fn choice_public_target(source: &PublicTarget, arm: TargetArm) -> Result<PublicTarget, String> {
    let e = source_curve();
    if curve(source) != e
        || source.order != CHOICE_N
        || point(source.generator) != affine(625_037_715, 2_100_627_251)
    {
        return Err("choice cell changes the pinned Prime31 source or generator".into());
    }
    independent_target_check(source)?;
    let mut target = source.clone();
    if arm != TargetArm::Original {
        let (codomain, t, u) = choice_map_parameters()?;
        target.a = codomain.a;
        target.b = codomain.b;
        let g = independent_image(point(source.generator), t, u)?;
        let q = independent_image(point(source.target), t, u)?;
        target.generator = crate::replay::Point {
            x: g.x,
            y: g.y,
            infinity: g.infinity,
        };
        target.target = crate::replay::Point {
            x: q.x,
            y: q.y,
            infinity: q.infinity,
        };
        independent_target_check(&target)?;
    }
    Ok(target)
}
fn independent_chain(
    steps: &[KernelStep],
    mut q: PointDefinition,
) -> Result<PointDefinition, String> {
    for step in steps {
        if !canonical_point(q, CHOICE_P) || !bases::independent_on_curve(&step.domain, q) {
            return Err("chain input fails independent curve membership".into());
        }
        if q.infinity || q.x == step.alpha {
            q = identity();
        } else {
            let inv = power(csub(q.x, step.alpha), CHOICE_P - 2, CHOICE_P);
            q = affine(
                (q.x + cmul(step.tau, inv)) % CHOICE_P,
                cmul(q.y, csub(1, cmul(step.tau, cmul(inv, inv)))),
            );
        }
        if !bases::independent_on_curve(&step.codomain, q) {
            return Err("chain output fails independent curve membership".into());
        }
    }
    Ok(q)
}
fn audit_poly_mul(a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = (out[i + j] + cmul(x, y)) % CHOICE_P;
        }
    }
    out
}
fn audit_poly_scaled_add(a: &[u64], b: &[u64], scale: u64) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    out[..a.len()].copy_from_slice(a);
    for (i, &x) in b.iter().enumerate() {
        out[i] = (out[i] + cmul(scale, x)) % CHOICE_P;
    }
    while out.len() > 1 && out.last() == Some(&0) {
        out.pop();
    }
    out
}
fn audit_poly_eval(a: &[u64], x: u64) -> u64 {
    a.iter()
        .rev()
        .fold(0, |acc, &c| (cmul(acc, x) + c) % CHOICE_P)
}

struct IndependentChoiceInventory {
    curve: CurveDefinition,
    source: PointDefinition,
    source_order: u64,
    generators: Vec<PointDefinition>,
    orders: Vec<u64>,
    points: Vec<PointDefinition>,
}

fn independent_choice_inventory(
    choice: KernelChoice,
) -> Result<IndependentChoiceInventory, String> {
    let plus = choice == KernelChoice::AplusCyclic;
    let e = CurveDefinition {
        p: CHOICE_P,
        a: if plus { 1 } else { CHOICE_P - 1 },
        b: 0,
    };
    let source = if plus {
        affine(3, 615_668_188)
    } else {
        affine(4, 639_874_715)
    };
    let source_order = 1u64 << if plus { 31 } else { 30 };
    if !bases::independent_on_curve(&e, source)
        || !bases::independent_scalar(&e, source, source_order).infinity
        || bases::independent_scalar(&e, source, source_order / 2).infinity
    {
        return Err("auxiliary source or exact-order certificate failed independent replay".into());
    }
    // The exact point order leaves a unique multiple in the Hasse interval.
    let lower = CHOICE_P + 1 - 92_681;
    let upper = CHOICE_P + 1 + 92_681;
    if lower.div_ceil(source_order) != upper / source_order
        || (upper / source_order) * source_order != CHOICE_P + 1
    {
        return Err("auxiliary Hasse order certificate failed".into());
    }
    let k = bases::independent_scalar(&e, source, source_order / 1024);
    let u = affine(0, 0);
    let (generators, orders) = match choice {
        KernelChoice::AplusCyclic | KernelChoice::AminusH0 => (vec![k], vec![1024]),
        KernelChoice::AminusH1 => (vec![bases::independent_add(&e, k, u)], vec![1024]),
        KernelChoice::AminusH2 => (vec![bases::independent_add(&e, k, k), u], vec![512, 2]),
    };
    let mut expected_kernel = Vec::new();
    let mut q = identity();
    for _ in 0..orders[0] {
        expected_kernel.push(q);
        q = bases::independent_add(&e, q, generators[0]);
    }
    if q != identity() {
        return Err("independent cyclic kernel did not close".into());
    }
    if generators.len() == 2 {
        let second_coset: Vec<_> = expected_kernel
            .iter()
            .map(|&h| bases::independent_add(&e, h, generators[1]))
            .collect();
        expected_kernel.extend(second_coset);
    }
    let expected_keys: BTreeSet<_> = expected_kernel.iter().copied().map(point_key).collect();
    if expected_kernel.len() != 1024 || expected_keys.len() != 1024 {
        return Err("independent kernel does not have exactly 1024 distinct points".into());
    }
    Ok(IndependentChoiceInventory {
        curve: e,
        source,
        source_order,
        generators,
        orders,
        points: expected_kernel,
    })
}

fn verify_choice_kernel(f: &KernelFiber, cell: &Value) -> Result<(), String> {
    if decode::<KernelChoice>(cell, "kernel")? != f.choice
        || integer(cell, "coset_scalar")? != f.coset_scalar
        || choice_factorization(cell)? != f.factorization
        || f.coset_scalar == 0
        || f.coset_scalar & 1 == 0
        || f.coset_scalar >= 1 << 20
        || (f.factorization != KernelFactorization::Standard && f.choice != KernelChoice::AminusH2)
    {
        return Err("kernel, factorization, or coset differs from the frozen choice cell".into());
    }
    let IndependentChoiceInventory {
        curve: e,
        source,
        source_order,
        generators,
        orders,
        points: expected_kernel,
    } = independent_choice_inventory(f.choice)?;
    if f.auxiliary_curve != e
        || f.auxiliary_order != CHOICE_P + 1
        || f.source_generator != source
        || f.source_order != source_order
        || f.kernel_generators != generators
        || f.kernel_orders != orders
    {
        return Err(
            "declared auxiliary source or kernel inventory differs from the chosen subgroup".into(),
        );
    }
    let expected_keys: BTreeSet<_> = expected_kernel.iter().copied().map(point_key).collect();
    let actual_keys: BTreeSet<_> = f.kernel_points.iter().copied().map(point_key).collect();
    if f.kernel_points.len() != 1024
        || expected_keys.len() != 1024
        || actual_keys != expected_keys
        || f.steps.len() != 10
        || f.kernel_points
            .iter()
            .any(|&q| !canonical_point(q, CHOICE_P))
    {
        return Err("complete independent kernel inventory differs".into());
    }
    let mut domain = e.clone();
    for step in &f.steps {
        if step.domain != domain
            || step.alpha >= CHOICE_P
            || independent_rhs(&domain, step.alpha) != 0
        {
            return Err("invalid independent degree-two kernel or domain".into());
        }
        let tau = (cmul(3, cmul(step.alpha, step.alpha)) + domain.a) % CHOICE_P;
        let codomain = CurveDefinition {
            p: CHOICE_P,
            a: csub(domain.a, cmul(5, tau)),
            b: csub(domain.b, cmul(7, cmul(step.alpha, tau))),
        };
        let disc = (cmul(4, cmul(cmul(codomain.a, codomain.a), codomain.a))
            + cmul(27, cmul(codomain.b, codomain.b)))
            % CHOICE_P;
        if tau == 0 || step.tau != tau || step.codomain != codomain || disc == 0 {
            return Err("independent degree-two transition certificate failed".into());
        }
        domain = codomain;
    }
    match f.factorization {
        KernelFactorization::Standard => {
            let twice_k = bases::independent_scalar(&e, source, source_order / 512);
            if !independent_chain(&f.steps[..9], twice_k)?.infinity {
                return Err(
                    "standard quotient prefix does not kill the specified order-512 subgroup"
                        .into(),
                );
            }
        }
        KernelFactorization::FullTwoTorsionFirst => {
            if (f.steps[0].alpha, f.steps[0].tau) != (0, CHOICE_P - 1)
                || (f.steps[1].alpha, f.steps[1].tau) != (0, 4)
            {
                return Err(
                    "full-two-torsion-first route changes its coordinate kernel prefix".into(),
                );
            }
        }
    }
    let expected_coset = bases::independent_scalar(&e, source, f.coset_scalar);
    let endpoint = independent_chain(&f.steps, expected_coset)?;
    if f.coset_point != expected_coset
        || f.endpoint != endpoint
        || endpoint.infinity
        || endpoint.y == 0
    {
        return Err("independent coset point or non-two-torsion endpoint mismatch".into());
    }
    let expected_tower = TowerDescription::Velu {
        steps: f.steps.iter().map(|s| (s.alpha, s.tau)).collect(),
        terminal: endpoint.x,
    };
    if f.tower != expected_tower {
        return Err("retained tower differs from its checked maps".into());
    }
    let mut xs = BTreeSet::new();
    let mut tsum = 0;
    let mut wsum = 0;
    for h in expected_kernel {
        if !independent_chain(&f.steps, h)?.infinity {
            return Err("declared independent kernel point survives quotient".into());
        }
        let c = bases::independent_add(&e, expected_coset, h);
        if c.infinity || !xs.insert(c.x) || independent_chain(&f.steps, c)? != endpoint {
            return Err("independent coset is not the complete rational x-fibre".into());
        }
        if !h.infinity {
            let x2 = cmul(h.x, h.x);
            tsum = (tsum + cmul(3, x2) + e.a) % CHOICE_P;
            wsum =
                (wsum + cmul(5, cmul(x2, h.x)) + cmul(3, cmul(e.a, h.x)) + cmul(2, e.b)) % CHOICE_P;
        }
    }
    if f.v != xs.into_iter().collect::<Vec<_>>() {
        return Err("recorded x-fibre differs from independent complete coset".into());
    }
    if domain.a != csub(e.a, cmul(5, tsum)) || domain.b != csub(e.b, cmul(7, wsum)) {
        return Err("independent full-kernel Velu codomain mismatch".into());
    }
    let mut n = vec![0, 1];
    let mut d = vec![1];
    for step in &f.steps {
        let nd = audit_poly_mul(&n, &d);
        let dd = audit_poly_mul(&d, &d);
        n = audit_poly_scaled_add(
            &audit_poly_scaled_add(&audit_poly_mul(&n, &n), &nd, csub(0, step.alpha)),
            &dd,
            step.tau,
        );
        d = audit_poly_scaled_add(&nd, &dd, csub(0, step.alpha));
    }
    let polynomial = audit_poly_scaled_add(&n, &d, csub(0, endpoint.x));
    let certificate = &f.certificate;
    let kernel_x: BTreeSet<_> = f
        .kernel_points
        .iter()
        .filter(|q| !q.infinity)
        .map(|q| q.x)
        .collect();
    if n.len() != 1025
        || d.len() != 1024
        || certificate.numerator != n
        || certificate.denominator != d
        || certificate.fiber_polynomial != polynomial
        || polynomial != bases::polynomial_from_roots(&f.v, CHOICE_P)
        || f.v.iter().any(|&x| audit_poly_eval(&d, x) == 0)
        || certificate.kernel_points_checked != 1024
        || certificate.coset_points_checked != 1024
        || certificate.kernel_x_poles_checked != kernel_x.len()
        || kernel_x
            .iter()
            .any(|&x| audit_poly_eval(&d, x) != 0 || audit_poly_eval(&n, x) == 0)
        || !certificate.denominator_nonzero_on_v
        || !certificate.fiber_polynomial_equals_root_product
        || !certificate.full_kernel_velu_codomain_matches
    {
        return Err("expanded map or whole-fibre certificate failed independent replay".into());
    }
    Ok(())
}

fn independent_lifts(e: &CurveDefinition, values: &[u64]) -> Result<Vec<PointDefinition>, String> {
    if e.p != CHOICE_P
        || values.iter().any(|&x| x >= CHOICE_P)
        || values.windows(2).any(|v| v[0] >= v[1])
    {
        return Err(
            "prepared membership values are not canonical sorted distinct coordinates".into(),
        );
    }
    let mut result = Vec::new();
    for &x in values {
        let rhs = independent_rhs(e, x);
        let y = power(rhs, (CHOICE_P + 1) / 4, CHOICE_P);
        if cmul(y, y) == rhs {
            if y == 0 {
                return Err("unexpected two-torsion in independent target lift".into());
            }
            let q = affine(x, y.min(CHOICE_P - y));
            if !bases::independent_on_curve(e, q)
                || !bases::independent_scalar(e, q, CHOICE_N).infinity
            {
                return Err("independent target lift is outside the prime-order subgroup".into());
            }
            result.push(q);
        }
    }
    if result.is_empty() {
        return Err("empty independently replayed factor base".into());
    }
    Ok(result)
}
fn verify_choice_prepared(
    base: &PreparedBase,
    fiber: &KernelFiber,
    source: &PublicTarget,
    cell: &Value,
) -> Result<AuditBase, String> {
    let arm: TargetArm = decode(cell, "target_arm")?;
    let target = choice_public_target(source, arm)?;
    if base.arm != arm
        || !target_equal(&target, &base.public_target)
        || base.curve != curve(&target)
        || base.source_membership_size != fiber.v.len()
    {
        return Err(
            "prepared base changes the frozen target arm, G/Q map, or source membership".into(),
        );
    }
    let (destination, t, u) = choice_map_parameters()?;
    let transported = arm == TargetArm::IsogenousTransported;
    let mut expected_v = fiber.v.clone();
    let mut poles = Vec::new();
    let mut duplicates = 0;
    if transported {
        let mut values = BTreeSet::new();
        for &x in &fiber.v {
            match independent_image_x(x, t, u)? {
                None => poles.push(x),
                Some((x, _)) => {
                    if !values.insert(x) {
                        duplicates += 1;
                    }
                }
            }
        }
        expected_v = values.into_iter().collect();
    }
    let expected_tower = if transported {
        None
    } else {
        Some(fiber.tower.clone())
    };
    if base.v != expected_v
        || base.tower != expected_tower
        || base.dropped_kernel_poles != poles
        || base.duplicate_image_values != duplicates
    {
        return Err("prepared whole membership image, collisions, poles, or tower differs".into());
    }
    let canonical = independent_lifts(&base.curve, &base.v)?;
    if base.canonical_points != canonical {
        return Err(
            "prepared canonical columns are not the complete independent target lift".into(),
        );
    }
    if transported {
        if decode::<Presentation>(cell, "presentation")? != Presentation::Dense {
            return Err(
                "transported membership is certified only in the dense presentation".into(),
            );
        }
        let old = independent_lifts(&source_curve(), &fiber.v)?;
        if base.source_columns != Some(old.len())
            || old.len() != canonical.len()
            || base.column_transport.len() != old.len()
        {
            return Err("transported column count differs from source or destination".into());
        }
        let by_x: BTreeMap<_, _> = canonical
            .iter()
            .enumerate()
            .map(|(i, q)| (q.x, (i, *q)))
            .collect();
        let mut seen = BTreeSet::new();
        for (i, q) in old.iter().enumerate() {
            let image = independent_image(*q, t, u)?;
            if !bases::independent_on_curve(&destination, image) {
                return Err("independent column image fails destination membership".into());
            }
            let &(column, point) = by_x
                .get(&image.x)
                .ok_or("transported column image is missing")?;
            let sign = if image == point {
                1
            } else if image == affine(point.x, CHOICE_P - point.y) {
                -1
            } else {
                return Err("transported column image sign mismatch".into());
            };
            let reported = &base.column_transport[i];
            if reported.source_column != i
                || reported.destination_column != column
                || reported.sign != sign
                || !seen.insert(column)
            {
                return Err(
                    "transported signed column permutation failed independent replay".into(),
                );
            }
        }
        if seen.len() != canonical.len() {
            return Err("incomplete transported column permutation".into());
        }
    } else if base.source_columns.is_some() || !base.column_transport.is_empty() {
        return Err("native base carries an unexpected transported-column mapping".into());
    }
    Ok(AuditBase {
        curve: base.curve.clone(),
        canonical_points: canonical,
        columns: base.canonical_points.len(),
    })
}

fn check_plain_reference_labels(
    reference: &PlainReferenceBinding,
    cell: &Value,
) -> Result<(), String> {
    let hash = format!(
        "{:x}",
        Sha256::digest(crate::plain_reference::fixture_bytes())
    );
    if reference.schema != "prime31-isogeny-choice.plain-reference/v1"
        || reference.fixture_sha256 != hash
        || reference.kernel != decode::<KernelChoice>(cell, "kernel")?
        || reference.coset_scalar != integer(cell, "coset_scalar")?
        || reference.coset_scalar != 1
        || reference.target_arm != decode::<TargetArm>(cell, "target_arm")?
        || reference.factorization != choice_factorization(cell)?
        || (reference.factorization != KernelFactorization::Standard
            && reference.kernel != KernelChoice::AminusH2)
        || reference.columns == 0
        || reference.columns > 1024
    {
        return Err(
            "plain reference changes the frozen fixture, kernel, route, coset, arm, or count"
                .into(),
        );
    }
    Ok(())
}

/// The count lookup used by the candidate is deliberately not called here.
/// Reconstruct the whole coset from the independent group law, compare every
/// retained x value, then independently lift and transport the selected base.
fn verify_plain_reference(
    reference: &PlainReferenceBinding,
    cell: &Value,
) -> Result<AuditBase, String> {
    check_plain_reference_labels(reference, cell)?;
    let fixture: Value = serde_json::from_slice(crate::plain_reference::fixture_bytes())
        .map_err(|e| format!("invalid retained plain-reference fixture: {e}"))?;
    let (destination, t, u) = choice_map_parameters()?;
    if fixture["schema"] != "p31.kernel-comparison/v1"
        || fixture["status"] != "arithmetic_correctness_fixtures_no_timing"
        || fixture["p"] != CHOICE_P
        || fixture["degree"] != 1024
        || fixture["source_scalars"] != json!([1])
        || fixture["target_curves"]
            != json!([
                [source_curve().a, source_curve().b],
                [destination.a, destination.b]
            ])
    {
        return Err("retained fixture changes its independently checked scope".into());
    }
    let name = match reference.kernel {
        KernelChoice::AplusCyclic => "Aplus_cyclic",
        KernelChoice::AminusH0 => "Aminus_H0",
        KernelChoice::AminusH1 => "Aminus_H1",
        KernelChoice::AminusH2 => "Aminus_H2",
    };
    let arms = fixture["arms"]
        .as_array()
        .ok_or("retained fixture lacks arms")?;
    let matching: Vec<_> = arms.iter().filter(|arm| arm["name"] == name).collect();
    if arms.len() != 4 || matching.len() != 1 {
        return Err("retained fixture has an ambiguous kernel inventory".into());
    }
    let arm = matching[0];
    let inventory = independent_choice_inventory(reference.kernel)?;
    if arm["domain"] != json!([inventory.curve.a, inventory.curve.b])
        || arm["source"] != json!([inventory.source.x, inventory.source.y])
    {
        return Err("retained fixture changes the pinned auxiliary coordinates".into());
    }
    let cosets = arm["cosets"]
        .as_array()
        .ok_or("retained fixture lacks cosets")?;
    if cosets.len() != 1 || cosets[0]["source_scalar"] != 1 {
        return Err("retained fixture does not bind exactly the first source coset".into());
    }
    let coset = &cosets[0];
    let retained_v: Vec<u64> = decode(coset, "V")?;
    let mut values = BTreeSet::new();
    for h in inventory.points {
        let q = bases::independent_add(&inventory.curve, inventory.source, h);
        if q.infinity || !bases::independent_on_curve(&inventory.curve, q) || !values.insert(q.x) {
            return Err(
                "independent plain-reference coset is not 1024 distinct finite x values".into(),
            );
        }
    }
    let expected_v: Vec<_> = values.into_iter().collect();
    if expected_v.len() != 1024 || retained_v != expected_v {
        return Err("retained plain-reference V differs from the independent full coset".into());
    }
    let source_points = independent_lifts(&source_curve(), &expected_v)?;
    let native_points = independent_lifts(&destination, &expected_v)?;
    if integer(coset, "accepted_E_x")? != source_points.len() as u64
        || integer(coset, "accepted_E1_x")? != native_points.len() as u64
    {
        return Err("retained usable-column counts fail independent lifting".into());
    }
    let (selected_curve, selected_points) = match reference.target_arm {
        TargetArm::Original => (source_curve(), source_points),
        TargetArm::IsogenousNative => (destination, native_points),
        TargetArm::IsogenousTransported => {
            let mut images = BTreeSet::new();
            for x in expected_v {
                if let Some((image, _)) = independent_image_x(x, t, u)? {
                    images.insert(image);
                }
            }
            let transported =
                independent_lifts(&destination, &images.into_iter().collect::<Vec<_>>())?;
            let mut expected_columns = BTreeSet::new();
            for q in source_points {
                let image = independent_image(q, t, u)?;
                if image.infinity || !bases::independent_on_curve(&destination, image) {
                    return Err("independent plain-reference transport leaves the codomain".into());
                }
                if !expected_columns.insert((image.x, image.y.min(CHOICE_P - image.y))) {
                    return Err("independent plain-reference transport merges point columns".into());
                }
            }
            let actual_columns: BTreeSet<_> = transported.iter().map(|q| (q.x, q.y)).collect();
            if expected_columns != actual_columns {
                return Err(
                    "transported plain-reference columns are not the complete source image".into(),
                );
            }
            (destination, transported)
        }
    };
    if selected_points.len() != reference.columns {
        return Err(
            "preregistered plain count differs from the independent reference inventory".into(),
        );
    }
    Ok(AuditBase {
        curve: selected_curve,
        columns: selected_points.len(),
        canonical_points: selected_points,
    })
}

#[derive(Clone)]
struct AuditBase {
    curve: CurveDefinition,
    canonical_points: Vec<PointDefinition>,
    columns: usize,
}
impl From<&BaseDefinition> for AuditBase {
    fn from(base: &BaseDefinition) -> Self {
        Self {
            curve: base.curve.clone(),
            canonical_points: base.canonical_points.clone(),
            columns: base.columns,
        }
    }
}

struct State {
    cell: Option<Value>,
    target: Option<PublicTarget>,
    reference: Option<BaseDefinition>,
    selected: Option<AuditBase>,
    choice_source: Option<PublicTarget>,
    choice_kernel: Option<KernelFiber>,
    choice_reference: Option<AuditBase>,
    choice_reference_from_fixture: bool,
    choice_plain: bool,
    choice_pair_setup: bool,
    choice_specialized_setup: bool,
    choice_polynomial_setup: Option<Value>,
    selected_membership_values: usize,
    selected_tower_depth: usize,
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
            choice_source: None,
            choice_kernel: None,
            choice_reference: None,
            choice_reference_from_fixture: false,
            choice_plain: false,
            choice_pair_setup: false,
            choice_specialized_setup: false,
            choice_polynomial_setup: None,
            selected_membership_values: 0,
            selected_tower_depth: 0,
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
    fn selected_base(&self) -> Result<&AuditBase, String> {
        self.selected.as_ref().ok_or("missing selected_base".into())
    }

    fn install_base(&mut self, base: AuditBase) {
        self.selected_membership_values = base.columns;
        self.selected_tower_depth = 0;
        for (column, &q) in base.canonical_points.iter().enumerate() {
            let neg = PointDefinition {
                x: q.x,
                y: base.curve.p - q.y,
                infinity: false,
            };
            self.signed_points.extend([q, neg]);
            self.signed_columns.insert((q.x, q.y), (column, 1));
            self.signed_columns.insert((neg.x, neg.y), (column, -1));
        }
        self.selected = Some(base);
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
        if let Some(source) = &self.choice_source {
            if bases::independent_scalar(&curve(source), point(source.generator), d)
                != point(source.target)
            {
                return Err(
                    "recovered scalar fails the original source public-point equation".into(),
                );
            }
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
        if self.cell.as_ref().is_some_and(choice_v3) {
            let work = field(summary, "rank_a_work")?;
            if work["schema"] != "prime31-recovery-work/v2"
                || work["implementation"] != "augmented_only"
                || boolean(work, "a_matrix_materialized")?
                || !boolean(work, "rank_derived_from_augmented")?
                || work["rank_rule"] != "rank_augmented_minus_target_pinned"
                || !field(work, "separate_matrix_counters")?.is_null()
            {
                return Err("v3 summary changes the registered recovery accounting".into());
            }
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
            "choice_start" => {
                if self.cell.is_some() {
                    return Err("audit accepts exactly one cell per JSONL stream".into());
                }
                let cell = field(&value, "cell")?.clone();
                if !matches!(
                    field(&cell, "schema")?.as_str(),
                    Some(
                        "prime31-isogeny-choice.cell/v1"
                            | "prime31-isogeny-choice.cell/v2"
                            | "prime31-isogeny-choice.cell/v3"
                    )
                ) {
                    return Err("unsupported isogeny-choice cell schema".into());
                }
                if !matches!(
                    field(&cell, "method")?.as_str(),
                    Some("ic" | "rho_negation" | "rho_v6" | "rho_fast")
                ) {
                    return Err("unsupported isogeny-choice method".into());
                }
                let kernel: KernelChoice = decode(&cell, "kernel")?;
                let factorization = choice_factorization(&cell)?;
                let backend = choice_backend(&cell)?;
                if choice_v3(&cell) {
                    if choice_recovery(&cell)? != "augmented_only"
                        || (matches!(
                            backend,
                            "prepared_polynomial" | "prepared_fiber" | "specialized_pair_table"
                        ) && cell["method"] != "ic")
                        || (backend == "prepared_fiber" && cell["presentation"] == "dense")
                    {
                        return Err(
                            "v3 requires augmented-only recovery and compatible prepared backend"
                                .into(),
                        );
                    }
                } else if choice_recovery(&cell)? != "dual_matrices"
                    || matches!(
                        backend,
                        "prepared_polynomial" | "prepared_fiber" | "specialized_pair_table"
                    )
                {
                    return Err("v1/v2 choice carries v3-only recovery or prepared backend".into());
                }
                if factorization != KernelFactorization::Standard
                    && kernel != KernelChoice::AminusH2
                {
                    return Err(
                        "nonstandard coordinate factorization requires the H2 kernel".into(),
                    );
                }
                if choice_v2(&cell) {
                    let queries = match field(&cell, "stage")?.as_str() {
                        Some("screening") => 64,
                        Some("complete_recovery") => 65_536,
                        _ => return Err("v2 choice cell has no supported comparison stage".into()),
                    };
                    let limits: CollectionLimits = decode(&cell, "collection")?;
                    if limits.max_queries != queries
                        || limits.max_seconds != 110
                        || !limits.oracle.stop_after_first
                        || limits.oracle.max_degree != 1024
                        || limits.oracle.max_pairs_per_step != 32
                        || limits.oracle.timeout_ms != 2000
                        || limits.oracle.max_solutions != 4096
                        || limits.oracle.max_signed_lift_attempts != 1_000_000
                        || limits.oracle.max_variables != 48
                        || limits.oracle.max_input_terms != 100_000
                        || limits.oracle.domain_roots != crate::oracle::DomainRoots::Auto
                        || !limits.oracle.sibling_batch
                        || integer(&cell, "execution_seconds")? != 120
                        || integer(&cell, "memory_mib")? != 4096
                        || integer(&cell, "arity")? != 3
                    {
                        return Err(
                            "v2 stage changes the registered first-hit strategy or resource limits"
                                .into(),
                        );
                    }
                } else if cell.get("stage").is_some_and(|v| !v.is_null())
                    || factorization != KernelFactorization::Standard
                    || cell.get("plain_reference").is_some_and(|v| !v.is_null())
                    || backend != "polynomial"
                {
                    return Err("legacy choice cell carries v2-only stage, factorization, reference, or backend".into());
                }
                let coset = integer(&cell, "coset_scalar")?;
                if coset == 0 || coset & 1 == 0 || coset >= 1 << 20 {
                    return Err("invalid frozen isogeny-choice coset scalar".into());
                }
                let source: PublicTarget = decode(&cell, "source_target")?;
                let arm: TargetArm = decode(&cell, "target_arm")?;
                let presentation: Presentation = decode(&cell, "presentation")?;
                if choice_plain_control(&cell)?
                    && (presentation != Presentation::Dense
                        || cell["method"] != "ic"
                        || cell
                            .get("plain_seed")
                            .is_some_and(|v| v.as_u64() != Some(1))
                        || cell
                            .get("max_plain_x_trials")
                            .is_some_and(|v| v.as_u64() != Some(1_000_000)))
                {
                    return Err(
                        "matched plain choice requires dense presentation and fixed seed one"
                            .into(),
                    );
                }
                if arm == TargetArm::IsogenousTransported && presentation != Presentation::Dense {
                    return Err(
                        "transported membership is certified only in the dense presentation".into(),
                    );
                }
                if matches!(backend, "pair_table" | "specialized_pair_table")
                    && (cell["method"] != "ic" || presentation != Presentation::Dense)
                {
                    return Err(
                        "pair-table backend requires an IC cell with dense presentation".into(),
                    );
                }
                if choice_v2(&cell) && choice_plain_control(&cell)? {
                    let reference: PlainReferenceBinding = decode(&cell, "plain_reference")?;
                    check_plain_reference_labels(&reference, &cell)?;
                } else if cell.get("plain_reference").is_some_and(|v| !v.is_null()) {
                    return Err(
                        "a frozen plain reference belongs only to a v2 plain IC cell".into(),
                    );
                }
                self.target = Some(choice_public_target(&source, arm)?);
                self.choice_source = Some(source);
                self.cell = Some(cell);
            }
            "choice_kernel" => {
                if self.choice_source.is_none()
                    || self.choice_kernel.is_some()
                    || self.selected.is_some()
                    || self.stream.is_some()
                {
                    return Err("duplicate, premature, or late choice kernel".into());
                }
                if self.cell.as_ref().unwrap()["method"] != "ic" {
                    return Err("rho cell carries an index-calculus kernel".into());
                }
                if choice_v2(self.cell.as_ref().unwrap())
                    && choice_plain_control(self.cell.as_ref().unwrap())?
                {
                    return Err("v2 plain candidate must use its frozen reference without rebuilding an auxiliary kernel".into());
                }
                let fiber: KernelFiber = decode(&value, "fiber")?;
                verify_choice_kernel(&fiber, self.cell.as_ref().unwrap())?;
                self.choice_kernel = Some(fiber);
            }
            "choice_prepared" => {
                if self.choice_reference.is_some()
                    || self.selected.is_some()
                    || self.stream.is_some()
                {
                    return Err("duplicate or late choice prepared base".into());
                }
                if self.cell.as_ref().is_some_and(choice_v2)
                    && choice_plain_control(self.cell.as_ref().unwrap())?
                {
                    return Err(
                        "v2 plain candidate cannot substitute a newly prepared auxiliary reference"
                            .into(),
                    );
                }
                let source = self
                    .choice_source
                    .as_ref()
                    .ok_or("choice base before choice_start")?;
                let fiber = self
                    .choice_kernel
                    .as_ref()
                    .ok_or("choice base before checked kernel")?;
                let base: PreparedBase = decode(&value, "base")?;
                let checked =
                    verify_choice_prepared(&base, fiber, source, self.cell.as_ref().unwrap())?;
                self.choice_reference = Some(checked.clone());
                if !choice_plain_control(self.cell.as_ref().unwrap())? {
                    self.install_base(checked);
                    self.selected_membership_values = base.v.len();
                    self.selected_tower_depth =
                        base.tower.as_ref().map_or(0, TowerDescription::depth);
                }
            }
            "choice_plain_reference" => {
                let cell = self
                    .cell
                    .as_ref()
                    .ok_or("plain reference before choice_start")?;
                if !choice_v2(cell)
                    || !choice_plain_control(cell)?
                    || cell["method"] != "ic"
                    || self.choice_kernel.is_some()
                    || self.choice_reference.is_some()
                    || self.selected.is_some()
                    || self.stream.is_some()
                {
                    return Err("unexpected, duplicate, or late frozen plain reference".into());
                }
                let declared: PlainReferenceBinding = decode(cell, "plain_reference")?;
                let reported: PlainReferenceBinding = decode(&value, "reference")?;
                if reported != declared {
                    return Err("plain reference event differs from the frozen cell binding".into());
                }
                self.choice_reference = Some(verify_plain_reference(&reported, cell)?);
                self.choice_reference_from_fixture = true;
            }
            "choice_plain" => {
                if self.choice_source.is_none()
                    || self.choice_reference.is_none()
                    || self.selected.is_some()
                    || self.stream.is_some()
                    || !choice_plain_control(self.cell.as_ref().ok_or("plain choice before cell")?)?
                {
                    return Err("duplicate, premature, or unexpected matched plain choice".into());
                }
                let base: BaseDefinition = decode(&value, "base")?;
                self.check_base_target(&base)?;
                bases::verify_base_definition(&base)?;
                if value.get("public_target").is_some() {
                    let reported: PublicTarget = decode(&value, "public_target")?;
                    if !target_equal(self.public_target()?, &reported) {
                        return Err(
                            "plain setup changes the frozen original/destination public target"
                                .into(),
                        );
                    }
                }
                let columns = self.choice_reference.as_ref().unwrap().columns;
                if base.family != BaseFamily::Plain
                    || base.seed != 1
                    || base.columns != columns
                    || value.get("seed").is_some_and(|v| v.as_u64() != Some(1))
                    || value
                        .get("max_x_trials")
                        .is_some_and(|v| v.as_u64() != Some(1_000_000))
                    || value
                        .get("reference_columns")
                        .is_some_and(|v| v.as_u64() != Some(columns as u64))
                {
                    return Err(
                        "matched plain base changes the frozen seed or reference cardinality"
                            .into(),
                    );
                }
                let mut expected_v = Vec::new();
                for x in 1..=1_000_000 {
                    let rhs = independent_rhs(&base.curve, x);
                    if rhs != 0 && power(rhs, (CHOICE_P - 1) / 2, CHOICE_P) == 1 {
                        expected_v.push(x);
                        if expected_v.len() == columns {
                            break;
                        }
                    }
                }
                if expected_v.len() != columns || base.v != expected_v {
                    return Err(
                        "plain membership differs from independent fixed-seed coordinate scan"
                            .into(),
                    );
                }
                self.install_base(AuditBase::from(&base));
                self.choice_plain = true;
            }
            "choice_pair_setup" => {
                let cell = self.cell.as_ref().ok_or("pair setup before choice_start")?;
                if !choice_v2(cell)
                    || cell["method"] != "ic"
                    || choice_backend(cell)? != "pair_table"
                    || self.choice_pair_setup
                    || self.stream.is_some()
                {
                    return Err("unexpected, duplicate, or late pair-table setup".into());
                }
                let columns = self.selected_base()?.columns as u64;
                let setup = field(&value, "setup")?;
                let attempts = columns * columns;
                let entries = integer(setup, "table_entries")?;
                if setup["backend"] != "pair_table"
                    || setup["mode"] != "batched"
                    || setup["table_kind"] != "negation_folded"
                    || setup["first_witness_policy"] != "cancellation_first_then_signed_index_order"
                    || integer(setup, "columns")? != columns
                    || integer(setup, "signed_points")? != 2 * columns
                    || integer(setup, "abscissae")? != columns
                    || integer(setup, "insert_attempts")? != attempts
                    || integer(setup, "index_lookups")? != columns
                    || entries == 0
                    || entries > attempts
                    || integer(setup, "requested_hash_entry_capacity")? != attempts + 1
                    || integer(setup, "logical_entry_payload_bytes")? != 16 * entries
                    || integer(setup, "requested_payload_bytes")? != 16 * (attempts + 1)
                {
                    return Err(
                        "pair setup metadata differs from the frozen backend and checked base"
                            .into(),
                    );
                }
                // These are consistency checks on the report and base shape.
                // Table contents, operation ledgers and timings are not replayed.
                self.choice_pair_setup = true;
            }
            "choice_specialized_setup" => {
                let cell = self
                    .cell
                    .as_ref()
                    .ok_or("specialized setup before choice_start")?;
                if !choice_v3(cell)
                    || cell["method"] != "ic"
                    || choice_backend(cell)? != "specialized_pair_table"
                    || self.choice_specialized_setup
                    || self.stream.is_some()
                {
                    return Err("unexpected, duplicate, or late specialized setup".into());
                }
                let columns = self.selected_base()?.columns as u64;
                let setup = field(&value, "setup")?;
                if setup["schema"] != "prime31-specialized-pair-setup/v1"
                    || setup["backend"] != "specialized_pair_table"
                    || setup["repository"] != "isogeny-labs/cryptanalysis"
                    || setup["source_revision"] != "457ae1818aaf8134bd833ad9c1b9d57015264510"
                    || integer(setup,"columns")? != columns
                    || integer(setup,"abscissae")? != columns
                    || integer(setup,"signed_points")? != 2 * columns
                    || integer(setup,"summands")? != 3
                    || setup["config"] != json!({"setup_chunk":1024,"probe_chunk":16})
                    || setup["table_kind"] != "negation_folded_prime_batch_xonly"
                    || setup["first_witness_policy"] != "cancellation_first_then_native_abscissa_groups; scalar_signed_index_order_at_infinity"
                    || boolean(setup,"native_group_operations_are_complete_field_ledger")?
                    || !field(setup,"field_operation_counters")?.is_null()
                    || boolean(setup,"payload_is_total_allocation_or_rss")?
                { return Err("specialized setup differs from pinned source/configuration or checked signed base".into()); }
                self.choice_specialized_setup = true;
            }
            "choice_polynomial_setup" => {
                let cell = self
                    .cell
                    .as_ref()
                    .ok_or("prepared setup before choice_start")?;
                if !choice_v3(cell)
                    || cell["method"] != "ic"
                    || !matches!(
                        choice_backend(cell)?,
                        "prepared_polynomial" | "prepared_fiber"
                    )
                    || self.choice_polynomial_setup.is_some()
                    || self.stream.is_some()
                {
                    return Err("unexpected, duplicate, or late prepared polynomial setup".into());
                }
                let columns = self.selected_base()?.columns as u64;
                let setup = field(&value, "setup")?;
                let presentation: Presentation = decode(cell, "presentation")?;
                let expected_depth = if presentation == Presentation::Dense {
                    0
                } else {
                    self.selected_tower_depth as u64
                };
                let fiber_mode = choice_backend(cell)? == "prepared_fiber";
                if setup["backend"] != "polynomial"
                    || setup["mode"]
                        != if fiber_mode {
                            "prepared_fiber_triangular"
                        } else {
                            "prepared_target_liftable_domains"
                        }
                    || field(setup, "presentation")? != field(cell, "presentation")?
                    || integer(setup, "summands")? != integer(cell, "arity")?
                    || integer(setup, "full_membership_values")?
                        != self.selected_membership_values as u64
                    || integer(setup, "leaf_domain_values")? != columns
                    || integer(setup, "signed_points")? != 2 * columns
                    || integer(setup, "validation_point_checks")? != 2 * columns
                    || integer(setup, "validated_tower_depth")? != expected_depth
                    || integer(setup, "fixed_validation_passes")? != 1
                    || integer(setup, "logical_leaf_domain_payload_bytes")? != 8 * columns
                {
                    return Err(
                        "prepared polynomial setup differs from the independently checked base"
                            .into(),
                    );
                }
                if fiber_mode {
                    let index = field(setup, "fiber_index")?;
                    let width = if presentation == Presentation::Tower4 {
                        2
                    } else {
                        1
                    };
                    if integer(index, "full_leaf_values")? != self.selected_membership_values as u64
                        || integer(index, "live_leaf_values")? != columns
                        || integer(index, "block_width")? != width
                        || integer(index, "variable_levels")? != expected_depth.div_ceil(width)
                    {
                        return Err(
                            "conditional fiber-index metadata differs from the checked tower/base"
                                .into(),
                        );
                    }
                    // The first v3 development snapshot reported a separate
                    // arithmetic traversal and predates these reuse fields.
                    // Preserve its replay; bind the additional fields whenever
                    // a newer report explicitly claims shared validation paths.
                    if [
                        "reuses_fixed_validation_paths",
                        "reused_block_path_values",
                        "logical_temporary_path_payload_bytes",
                    ]
                    .iter()
                    .any(|key| index.get(key).is_some())
                    {
                        let values = self.selected_membership_values as u64
                            * (expected_depth.div_ceil(width) + 1);
                        if !boolean(index, "reuses_fixed_validation_paths")?
                            || integer(index, "construction_field_multiplications")? != 0
                            || integer(index, "reused_block_path_values")? != values
                            || integer(index, "logical_temporary_path_payload_bytes")? != 8 * values
                        {
                            return Err("shared validation-path metadata contradicts the checked tower shape".into());
                        }
                    }
                } else if setup.get("fiber_index").is_some() {
                    return Err(
                        "plain prepared setup carries unexpected conditional-fiber work".into(),
                    );
                }
                // Bind setup metadata to the independently reconstructed base.
                // This is not a timing, allocation or operation-count replay.
                self.choice_polynomial_setup = Some(setup.clone());
            }
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
                if self.choice_source.is_some()
                    || self.reference.is_some()
                    || self.selected.is_some()
                {
                    return Err("duplicate or late reference base".into());
                }
                let definition: BaseDefinition = decode(&value, "base")?;
                self.check_base_target(&definition)?;
                bases::verify_base_definition(&definition)?;
                self.check_reference_input(&definition)?;
                self.reference = Some(definition);
            }
            "selected_base" => {
                if self.choice_source.is_some() || self.selected.is_some() || self.stream.is_some()
                {
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
                self.install_base(AuditBase::from(&definition));
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
                if self.choice_source.is_some() {
                    let expected_backend = choice_backend(self.cell.as_ref().unwrap())?;
                    if choice_backend(&value)? != expected_backend
                        || (expected_backend == "pair_table") != self.choice_pair_setup
                        || matches!(expected_backend, "prepared_polynomial" | "prepared_fiber")
                            != self.choice_polynomial_setup.is_some()
                        || (expected_backend == "specialized_pair_table")
                            != self.choice_specialized_setup
                        || choice_recovery(&value)? != choice_recovery(self.cell.as_ref().unwrap())?
                    {
                        return Err("collection backend or required pair setup differs from the frozen cell".into());
                    }
                }
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
                if self.choice_source.is_some() {
                    let cell = self.cell.as_ref().unwrap();
                    if choice_backend(cell)? == "pair_table" {
                        let work = oracle
                            .backend_work
                            .as_ref()
                            .ok_or("pair result omits its backend work identity")?;
                        if !self.choice_pair_setup
                            || work["backend"] != "pair_table"
                            || work["mode"] != "batched"
                            || !oracle.charts.is_empty()
                            || oracle.polynomial_build_ns != 0
                            || oracle.counters.polynomial_build_multiplications != 0
                            || oracle.counters.f4_field_operations != 0
                            || oracle.counters.f4_runs != 0
                        {
                            return Err(
                                "pair result changes its backend or claims polynomial execution"
                                    .into(),
                            );
                        }
                    } else if choice_backend(cell)? == "specialized_pair_table" {
                        let work = oracle
                            .backend_work
                            .as_ref()
                            .ok_or("specialized query omits backend work identity")?;
                        if !self.choice_specialized_setup
                            || work["backend"] != "specialized_pair_table"
                            || work["source_revision"] != "457ae1818aaf8134bd833ad9c1b9d57015264510"
                            || boolean(work,"native_ledger_merged_into_collector")?
                            || boolean(work,"complete_operation_ledger")?
                            || !boolean(work,"outer_process_cap_required")?
                            || work["first_witness_policy"] != "cancellation_first_then_native_abscissa_groups; scalar_signed_index_order_at_infinity"
                            || !oracle.charts.is_empty() || oracle.polynomial_build_ns != 0
                            || oracle.counters.polynomial_build_multiplications != 0
                            || oracle.counters.f4_field_operations != 0 || oracle.counters.f4_runs != 0
                        { return Err("specialized query changes its source/accounting or claims polynomial execution".into()); }
                        let checked = boolean(work, "cancellation_checked")?;
                        let hit = boolean(work, "cancellation_hit")?;
                        let native = field(work, "native")?;
                        let target = self.current.as_ref().unwrap().target;
                        let in_base = self.signed_points.contains(&target);
                        if (!checked && (hit || !native.is_null()))
                            || (checked && hit != in_base)
                            || (hit
                                && (oracle.status != OracleStatus::FoundFirst || !native.is_null()))
                            || (oracle.status == OracleStatus::CompleteNegative
                                && (!checked || hit || native.is_null()))
                            || (oracle.status == OracleStatus::FoundFirst
                                && !hit
                                && native.is_null())
                        {
                            return Err("specialized cancellation/native path contradicts the target or outcome".into());
                        }
                        if !native.is_null()
                            && (!boolean(native, "native_called")?
                                || boolean(native, "scalar_identity_fallback")? != target.infinity
                                || field(native, "field_operation_counters")? != &Value::Null
                                || boolean(native, "group_counts_are_complete_native_arithmetic")?
                                || !matches!(
                                    oracle.status,
                                    OracleStatus::FoundFirst | OracleStatus::CompleteNegative
                                ))
                        {
                            return Err("specialized native path changes its identity handling or partial ledger".into());
                        }
                    } else if matches!(
                        choice_backend(cell)?,
                        "prepared_polynomial" | "prepared_fiber"
                    ) {
                        let setup = self
                            .choice_polynomial_setup
                            .as_ref()
                            .ok_or("prepared query without setup")?;
                        let work = oracle
                            .backend_work
                            .as_ref()
                            .ok_or("prepared query omits its backend work identity")?;
                        if work["backend"] != "polynomial"
                            || work["mode"] != setup["mode"]
                            || field(work, "full_membership_values")?
                                != field(setup, "full_membership_values")?
                            || field(work, "leaf_domain_values")?
                                != field(setup, "leaf_domain_values")?
                            || boolean(work, "fixed_validation_repeated")?
                            || integer(work, "target_point_checks")? != 1
                            || work["setup_accounting"] != "separate_prepared_setup_record_required"
                            || oracle.counters.validation_field_multiplications != 0
                        {
                            return Err("prepared polynomial query changes its certified setup or work scope".into());
                        }
                        if choice_backend(cell)? == "prepared_fiber"
                            && work["fiber_limits"]
                                != json!({"max_nodes":4096,"max_f4_runs":64,"max_root_degree":1024,"max_retained_terms":100000})
                        {
                            return Err(
                                "conditional-fiber query changes its frozen driver limits".into()
                            );
                        }
                        if choice_backend(cell)? == "prepared_fiber" {
                            let policy = json!({
                                "schema":"prime31-fiber-extraction-policy/v1",
                                "selector_domain":"descendants_of_nearest_fixed_selector_ancestor",
                                "univariate_roots":"direct_domain_horner_at_limit_else_pinned_dense_roots_fp",
                                "direct_domain_limit":4,
                                "root_filter":"all_available_univariates_and_conditional_domain",
                                "substitution":"pinned_substitute_var_per_child_and_forced_ancestor",
                                "selector_branch":"strict_domain_reduction_or_singleton",
                                "preserves_incoming_equations_and_f4_consequences":true,
                                "legacy_domain_roots_applied":false,
                                "legacy_sibling_batch_applied":false,
                                "static_cartesian_fallback":false,
                                "full_field_scan_fallback":false
                            });
                            if work["effective_policy"] != policy
                                || oracle
                                    .charts
                                    .iter()
                                    .any(|chart| match chart.status.as_str() {
                                        "impossible_identity_plus_finite_equals_identity" => {
                                            !chart.visit.is_null()
                                        }
                                        "error" => {
                                            oracle.status != OracleStatus::UnknownError
                                                || !chart.visit["error"].is_string()
                                        }
                                        _ => chart.visit["effective_policy"] != policy,
                                    })
                            {
                                return Err("conditional-fiber query changes its effective extraction policy".into());
                            }
                        }
                    } else if oracle.backend_work.is_some() {
                        return Err(
                            "polynomial cell carries an unexpected alternate-backend result".into(),
                        );
                    }
                    if choice_v2(cell)
                        && (oracle.relations.len() > 1
                            || (oracle.positive && oracle.status != OracleStatus::FoundFirst))
                    {
                        return Err("v2 query result changes the frozen first-hit strategy".into());
                    }
                }
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
                if self.cell.as_ref().is_some_and(choice_v2) {
                    let cell = self.cell.as_ref().unwrap();
                    if field(&value, "stage")? != field(cell, "stage")?
                        || choice_backend(&value)? != choice_backend(cell)?
                        || choice_recovery(&value)? != choice_recovery(cell)?
                    {
                        return Err(
                            "cell summary changes the frozen stage or decomposition backend".into(),
                        );
                    }
                }
                if self.choice_source.is_some()
                    && value.get("original_scalar_replay").is_some()
                    && boolean(&value, "original_scalar_replay")? != self.rows.pinned.is_some()
                {
                    return Err(
                        "source scalar replay flag differs from independent verification".into(),
                    );
                }
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
            "setup_failure" | "fatal_error" | "driver_error" => {
                self.public_target()?;
                self.failure_terminal = true;
            }
            "rho_summary" => {
                if self.stream.is_some() || self.selected.is_some() || self.choice_kernel.is_some()
                {
                    return Err("rho summary mixed with an index-calculus cell".into());
                }
                if self.choice_source.is_some() {
                    if self.cell.as_ref().unwrap()["method"] == "ic" {
                        return Err("index-calculus cell ends with a rho result".into());
                    }
                    let reported: PublicTarget = decode(&value, "public_target")?;
                    if !target_equal(self.public_target()?, &reported) {
                        return Err(
                            "rho target differs from the independently transported input".into(),
                        );
                    }
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
        "isogeny_choice":state.choice_source.is_some(),
        "choice_kernel_independently_verified":state.choice_kernel.is_some(),
        "choice_prepared_independently_verified":state.choice_reference.is_some() && !state.choice_reference_from_fixture,
        "choice_plain_reference_independently_verified":state.choice_reference_from_fixture,
        "choice_plain_independently_verified":state.choice_plain,
        "choice_pair_setup_checked":state.choice_pair_setup,
        "choice_specialized_setup_checked":state.choice_specialized_setup,
        "choice_polynomial_setup_checked":state.choice_polynomial_setup.is_some(),
        "prepared_setup_costs_independently_replayed":false,
        "pair_table_contents_independently_replayed":false,
        "source_public_scalar_equation_verified":state.choice_source.is_some() && recovered.is_some(),
        "negative_exhaustion_independently_replayed":false,
        "performance_independently_validated":false,
        "scope":"Independent whole-fiber or frozen plain-reference cardinality certificate, frozen public query-stream replay, signed-point and column-row checks, batch modular RREF and public scalar verification. Pair setup checks bind report metadata to the selected base; table contents are not independently reconstructed. Negative search exhaustion, counters, timings, execution provenance and signed PCC standing require separate evidence. complete means a finished recovered cell, not enumeration of all decompositions."
    }))
}
