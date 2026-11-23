//! Fixed Prime31 degree-1024 auxiliary quotients for the isogeny-choice study.
//!
//! This adds auxiliary inputs to the existing relation oracle, not a solver.
//! H2 is constructed from two independent generators. Every cold construction
//! verifies the complete kernel and coset and an exact polynomial identity.

use crate::bases::{CurveDefinition, GroupWork, PointDefinition};
use crate::oracle::TowerDescription;
use crate::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const P: u64 = 2_147_483_647;
pub const DEGREE: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelChoice {
    AplusCyclic,
    AminusH0,
    AminusH1,
    AminusH2,
}

/// Actual modular operations in map construction/evaluation and certification.
/// inversion_calls is metadata: its powering work is already included in muls
/// and sqrs, so do not price the inversion a second time.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapWork {
    pub adds: u64,
    pub subs: u64,
    pub muls: u64,
    pub sqrs: u64,
    pub inversion_calls: u64,
    pub point_checks: u64,
    pub point_map_calls: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelStep {
    pub domain: CurveDefinition,
    pub codomain: CurveDefinition,
    pub alpha: u64,
    pub tau: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelCertificate {
    pub numerator: Vec<u64>,
    pub denominator: Vec<u64>,
    pub fiber_polynomial: Vec<u64>,
    pub kernel_points_checked: usize,
    pub coset_points_checked: usize,
    pub kernel_x_poles_checked: usize,
    pub denominator_nonzero_on_v: bool,
    pub fiber_polynomial_equals_root_product: bool,
    pub full_kernel_velu_codomain_matches: bool,
    pub auxiliary_order_method: String,
    /// Hashing, sorting, allocation and integer bookkeeping require the outer
    /// wall-clock timer. These field/group counters alone are not total cost.
    pub native_operation_counts_partial: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KernelFiber {
    pub choice: KernelChoice,
    pub coset_scalar: u64,
    pub v: Vec<u64>,
    pub tower: TowerDescription,
    pub endpoint: PointDefinition,
    pub auxiliary_curve: CurveDefinition,
    pub auxiliary_order: u64,
    pub source_generator: PointDefinition,
    pub source_order: u64,
    pub coset_point: PointDefinition,
    pub kernel_generators: Vec<PointDefinition>,
    pub kernel_orders: Vec<u64>,
    pub kernel_points: Vec<PointDefinition>,
    pub steps: Vec<KernelStep>,
    pub certificate: KernelCertificate,
    pub group_work: GroupWork,
    pub map_work: MapWork,
}

#[derive(Default)]
struct Arithmetic {
    work: MapWork,
}
impl Arithmetic {
    fn add(&mut self, a: u64, b: u64) -> u64 {
        self.work.adds += 1;
        (a + b) % P
    }
    fn sub(&mut self, a: u64, b: u64) -> u64 {
        self.work.subs += 1;
        (a + P - b) % P
    }
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.work.muls += 1;
        a * b % P
    }
    fn sqr(&mut self, a: u64) -> u64 {
        self.work.sqrs += 1;
        a * a % P
    }
    fn inv(&mut self, mut a: u64) -> Result<u64, String> {
        if a == 0 {
            return Err("attempted inversion of a kernel pole".into());
        }
        self.work.inversion_calls += 1;
        let mut n = P - 2;
        let mut out = 1;
        while n != 0 {
            if n & 1 != 0 {
                out = self.mul(out, a);
            }
            a = self.sqr(a);
            n >>= 1;
        }
        Ok(out)
    }
    fn on_curve(&mut self, e: &CurveDefinition, point: PrimePoint) -> bool {
        self.work.point_checks += 1;
        if point.infinity {
            return true;
        }
        if e.p != P || point.x >= P || point.y >= P {
            return false;
        }
        let xx = self.sqr(point.x);
        let xxx = self.mul(xx, point.x);
        let ax = self.mul(e.a, point.x);
        let sum = self.add(xxx, ax);
        let rhs = self.add(sum, e.b);
        self.sqr(point.y) == rhs
    }
    fn step(&mut self, e: &PrimeCurve, kernel: PrimePoint) -> Result<KernelStep, String> {
        let domain = CurveDefinition::from(e);
        if kernel.infinity || kernel.y != 0 || !self.on_curve(&domain, kernel) {
            return Err("degree-two step kernel is not rational nonzero 2-torsion".into());
        }
        let xx = self.sqr(kernel.x);
        let three_xx = self.mul(3, xx);
        let tau = self.add(three_xx, e.a);
        if tau == 0 {
            return Err("singular degree-two kernel transition".into());
        }
        let five_tau = self.mul(5, tau);
        let alpha_tau = self.mul(kernel.x, tau);
        let seven_alpha_tau = self.mul(7, alpha_tau);
        let codomain = CurveDefinition {
            p: P,
            a: self.sub(e.a, five_tau),
            b: self.sub(e.b, seven_alpha_tau),
        };
        let aa = self.sqr(codomain.a);
        let aaa = self.mul(aa, codomain.a);
        let four_aaa = self.mul(4, aaa);
        let bb = self.sqr(codomain.b);
        let twentyseven_bb = self.mul(27, bb);
        if self.add(four_aaa, twentyseven_bb) == 0 {
            return Err("singular isogeny codomain".into());
        }
        Ok(KernelStep {
            domain,
            codomain,
            alpha: kernel.x,
            tau,
        })
    }
    fn apply(&mut self, step: &KernelStep, point: PrimePoint) -> Result<PrimePoint, String> {
        self.work.point_map_calls += 1;
        if point.infinity || point.x == step.alpha {
            return Ok(PrimePoint::INFINITY);
        }
        let delta = self.sub(point.x, step.alpha);
        let inv = self.inv(delta)?;
        let correction = self.mul(step.tau, inv);
        let x = self.add(point.x, correction);
        let inv2 = self.sqr(inv);
        let derivative_term = self.mul(step.tau, inv2);
        let derivative = self.sub(1, derivative_term);
        let y = self.mul(point.y, derivative);
        Ok(PrimePoint::affine(x, y))
    }
    fn chain(&mut self, steps: &[KernelStep], mut point: PrimePoint) -> Result<PrimePoint, String> {
        for step in steps {
            point = self.apply(step, point)?;
        }
        Ok(point)
    }
    fn cyclic_chain(
        &mut self,
        e: &PrimeCurve,
        generator: PrimePoint,
        exponent: u32,
        ops: &mut GroupOps,
    ) -> Result<Vec<KernelStep>, String> {
        let mut curve = e.clone();
        let mut h = generator;
        let mut steps = Vec::new();
        for j in 0..exponent {
            let kernel = curve.mul(ops, h, 1 << (exponent - j - 1));
            let step = self.step(&curve, kernel)?;
            h = self.apply(&step, h)?;
            curve = step.codomain.production();
            steps.push(step);
        }
        if !h.infinity {
            return Err("cyclic quotient did not kill its declared generator".into());
        }
        Ok(steps)
    }
    fn poly_mul(&mut self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                let product = self.mul(x, y);
                out[i + j] = self.add(out[i + j], product);
            }
        }
        trim(out)
    }
    fn poly_scaled_add(&mut self, a: &[u64], b: &[u64], scale: u64) -> Vec<u64> {
        let mut out = vec![0; a.len().max(b.len())];
        out[..a.len()].copy_from_slice(a);
        for (i, &value) in b.iter().enumerate() {
            let product = self.mul(scale, value);
            out[i] = self.add(out[i], product);
        }
        trim(out)
    }
    fn poly_eval(&mut self, polynomial: &[u64], x: u64) -> u64 {
        let mut out = 0;
        for &coefficient in polynomial.iter().rev() {
            let product = self.mul(out, x);
            out = self.add(product, coefficient);
        }
        out
    }
}

fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn cyclic_points(
    e: &PrimeCurve,
    h: PrimePoint,
    order: usize,
    ops: &mut GroupOps,
) -> Result<Vec<PrimePoint>, String> {
    let mut point = PrimePoint::INFINITY;
    let mut out = Vec::with_capacity(order);
    for _ in 0..order {
        out.push(point);
        point = e.add(ops, point, h);
    }
    if !point.infinity {
        return Err("kernel enumeration did not close".into());
    }
    Ok(out)
}

/// Nonzero odd scalars below 2^20 give deterministic admissible cosets for all
/// four arms. Opposite cosets can have the same x-fiber; a caller requiring
/// distinct x-fibers may use scalars below 2^19 or check the returned sets.
/// Even, zero and out-of-range inputs are rejected; no source point or kernel
/// is silently substituted.
pub fn build_fiber(
    choice: KernelChoice,
    coset_scalar: u64,
    ops: &mut GroupOps,
) -> Result<KernelFiber, String> {
    if coset_scalar == 0 || coset_scalar & 1 == 0 || coset_scalar >= 1 << 20 {
        return Err("coset scalar must be nonzero, odd and less than 2^20".into());
    }
    let mut local_ops = GroupOps::default();
    let result = build_inner(choice, coset_scalar, &mut local_ops);
    ops.merge(local_ops);
    result.map(|mut fiber| {
        fiber.group_work = GroupWork::from_ops(&local_ops);
        fiber
    })
}

fn build_inner(
    choice: KernelChoice,
    coset_scalar: u64,
    ops: &mut GroupOps,
) -> Result<KernelFiber, String> {
    let plus = choice == KernelChoice::AplusCyclic;
    let e = PrimeCurve {
        p: P,
        a: if plus { 1 } else { P - 1 },
        b: 0,
    };
    let source = if plus {
        PrimePoint::affine(3, 615_668_188)
    } else {
        PrimePoint::affine(4, 639_874_715)
    };
    let source_order = 1u64 << if plus { 31 } else { 30 };
    let mut f = Arithmetic::default();
    if !f.on_curve(&CurveDefinition::from(&e), source)
        || !e.mul(ops, source, source_order).infinity
        || e.mul(ops, source, source_order / 2).infinity
    {
        return Err("fixed auxiliary generator exact-order certificate failed".into());
    }
    // The Hasse interval contains exactly one multiple of this certified order.
    let lower = P + 1 - 92_681;
    let upper = P + 1 + 92_681;
    if lower.div_ceil(source_order) != upper / source_order
        || (upper / source_order) * source_order != P + 1
    {
        return Err("auxiliary order is not certified by the fixed Hasse interval".into());
    }
    let k = e.mul(ops, source, source_order / DEGREE as u64);
    let expected_k = if plus {
        PrimePoint::affine(1_106_623_295, 606_392_145)
    } else {
        PrimePoint::affine(1_724_938_470, 923_814_573)
    };
    if k != expected_k {
        return Err("fixed kernel generator certificate changed".into());
    }
    let u = PrimePoint::affine(0, 0);
    let (steps, kernel, generators, orders) = if plus {
        (
            f.cyclic_chain(&e, k, 10, ops)?,
            cyclic_points(&e, k, DEGREE, ops)?,
            vec![k],
            vec![1024],
        )
    } else {
        let twice_k = e.double(ops, k);
        let mut steps = f.cyclic_chain(&e, twice_k, 9, ops)?;
        let prefix_curve = steps
            .last()
            .ok_or("empty common prefix")?
            .codomain
            .production();
        let ki = f.chain(&steps, k)?;
        let ui = f.chain(&steps, u)?;
        let kui = prefix_curve.add(ops, ki, ui);
        if ki.infinity || ui.infinity || kui.infinity || ki == ui || ki == kui || ui == kui {
            return Err("three distinct final order-two kernels were not obtained".into());
        }
        let (last, kernel, generators, orders) = match choice {
            KernelChoice::AminusH0 => (ki, cyclic_points(&e, k, DEGREE, ops)?, vec![k], vec![1024]),
            KernelChoice::AminusH1 => {
                let ku = e.add(ops, k, u);
                (
                    kui,
                    cyclic_points(&e, ku, DEGREE, ops)?,
                    vec![ku],
                    vec![1024],
                )
            }
            KernelChoice::AminusH2 => {
                let mut points = cyclic_points(&e, twice_k, DEGREE / 2, ops)?;
                let translated = points.iter().map(|&q| e.add(ops, q, u)).collect::<Vec<_>>();
                points.extend(translated);
                (ui, points, vec![twice_k, u], vec![512, 2])
            }
            KernelChoice::AplusCyclic => unreachable!(),
        };
        steps.push(f.step(&prefix_curve, last)?);
        (steps, kernel, generators, orders)
    };
    let keys = kernel
        .iter()
        .map(|q| (q.infinity, q.x, q.y))
        .collect::<BTreeSet<_>>();
    if steps.len() != 10 || kernel.len() != DEGREE || keys.len() != DEGREE {
        return Err("kernel or chain cardinality mismatch".into());
    }
    let auxiliary_curve = CurveDefinition::from(&e);
    let endpoint_curve = steps
        .last()
        .ok_or("missing quotient step")?
        .codomain
        .clone();
    let coset_point = e.mul(ops, source, coset_scalar);
    let endpoint = f.chain(&steps, coset_point)?;
    if endpoint.infinity
        || !f.on_curve(&endpoint_curve, endpoint)
        || endpoint_curve.production().double(ops, endpoint).infinity
    {
        return Err("coset endpoint is infinity, invalid, or two-torsion".into());
    }
    let mut v = BTreeSet::new();
    let mut tsum = 0;
    let mut wsum = 0;
    for &h in &kernel {
        if !f.on_curve(&auxiliary_curve, h) || !f.chain(&steps, h)?.infinity {
            return Err("a declared kernel point survives the quotient".into());
        }
        let q = e.add(ops, coset_point, h);
        if q.infinity
            || !f.on_curve(&auxiliary_curve, q)
            || f.chain(&steps, q)? != endpoint
            || !v.insert(q.x)
        {
            return Err("coset is not a distinct complete rational x-fiber".into());
        }
        if !h.infinity {
            let xx = f.sqr(h.x);
            let three_xx = f.mul(3, xx);
            let t = f.add(three_xx, e.a);
            tsum = f.add(tsum, t);
            let xxx = f.mul(xx, h.x);
            let five_xxx = f.mul(5, xxx);
            let ax = f.mul(e.a, h.x);
            let three_ax = f.mul(3, ax);
            let two_b = f.mul(2, e.b);
            let first = f.add(five_xxx, three_ax);
            let w = f.add(first, two_b);
            wsum = f.add(wsum, w);
        }
    }
    let five_tsum = f.mul(5, tsum);
    let seven_wsum = f.mul(7, wsum);
    if endpoint_curve.a != f.sub(e.a, five_tsum) || endpoint_curve.b != f.sub(e.b, seven_wsum) {
        return Err("full-kernel Vélu codomain disagrees with the chain".into());
    }
    let v = v.into_iter().collect::<Vec<_>>();
    let mut numerator = vec![0, 1];
    let mut denominator = vec![1];
    for step in &steps {
        let nn = f.poly_mul(&numerator, &numerator);
        let nd = f.poly_mul(&numerator, &denominator);
        let dd = f.poly_mul(&denominator, &denominator);
        let negative_alpha = f.sub(0, step.alpha);
        let first = f.poly_scaled_add(&nn, &nd, negative_alpha);
        numerator = f.poly_scaled_add(&first, &dd, step.tau);
        denominator = f.poly_scaled_add(&nd, &dd, negative_alpha);
    }
    let negative_terminal = f.sub(0, endpoint.x);
    let fiber_polynomial = f.poly_scaled_add(&numerator, &denominator, negative_terminal);
    let kernel_x = kernel
        .iter()
        .filter(|point| !point.infinity)
        .map(|point| point.x)
        .collect::<BTreeSet<_>>();
    for &x in &kernel_x {
        if f.poly_eval(&denominator, x) != 0 || f.poly_eval(&numerator, x) == 0 {
            return Err("finite kernel x is not a genuine uncancelled dense-map pole".into());
        }
    }
    let mut root_product = vec![1];
    for &x in &v {
        let negative_x = f.sub(0, x);
        root_product = f.poly_mul(&root_product, &[negative_x, 1]);
        if f.poly_eval(&denominator, x) == 0 {
            return Err("rational map denominator vanishes on a fiber root".into());
        }
    }
    if numerator.len() != DEGREE + 1
        || denominator.len() != DEGREE
        || fiber_polynomial != root_product
    {
        return Err("expanded map degrees or exact fiber polynomial identity failed".into());
    }
    let tower = TowerDescription::Velu {
        steps: steps.iter().map(|s| (s.alpha, s.tau)).collect(),
        terminal: endpoint.x,
    };
    Ok(KernelFiber {
        choice,
        coset_scalar,
        v,
        tower,
        endpoint: endpoint.into(),
        auxiliary_curve,
        auxiliary_order: P + 1,
        source_generator: source.into(),
        source_order,
        coset_point: coset_point.into(),
        kernel_generators: generators.into_iter().map(Into::into).collect(),
        kernel_orders: orders,
        kernel_points: kernel.into_iter().map(Into::into).collect(),
        steps,
        certificate: KernelCertificate {
            numerator,
            denominator,
            fiber_polynomial,
            kernel_points_checked: DEGREE,
            coset_points_checked: DEGREE,
            kernel_x_poles_checked: kernel_x.len(),
            denominator_nonzero_on_v: true,
            fiber_polynomial_equals_root_product: true,
            full_kernel_velu_codomain_matches: true,
            auxiliary_order_method: "fixed source point has certified exact 2-power order; Hasse interval contains only its multiple p+1".into(),
            native_operation_counts_partial: true,
        },
        group_work: GroupWork::default(),
        map_work: f.work,
    })
}
