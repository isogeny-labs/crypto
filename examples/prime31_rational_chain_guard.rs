//! Iteration 6: exact rational-chain certificates on the original Prime31 curve.
//! Fixed correctness fixtures only: no solver, timing, yield or speedup claim.
//!
//! Reuses the original native Curve/Arithmetic/Base and crypto_lib field,
//! polynomial, Fp2 and extension-curve implementations without modification.
//! Prior art includes native/examples/prime31_m8_oriented_f4.rs; orienting an
//! S3 node is established chord-and-tangent arithmetic, not a novelty claim.

#[allow(dead_code)]
#[path = "../research/prime31_ic_20261115/native/lib.rs"]
mod original;
// The unchanged native source and its own tests use these crate-root paths.
pub use original::{cryptanalysis, f4_fp_tower, sig_fp_tower};

use crypto_lib::cryptanalysis::ext_curve::{ExtCurve, ExtField, ExtPoint, Fp2};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crypto_lib::cryptanalysis::semaev_prime_poly::{
    interpolate, poly_gcd, poly_powmod, trim, Curve as SemaevCurve,
};
use original::{check_curve, Arithmetic, Base, Curve, Point, PRIME31};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize)]
pub struct ChainCertificate {
    /// Ordered leaf abscissae, with repetitions retained.
    pub leaf_x: Vec<u64>,
    /// Abscissae of prefixes of lengths 2 through m-1, all finite in Fp.
    pub prefix_x: Vec<u64>,
    /// A signed, finite, nonzero target; infinity has no affine encoding.
    pub target: (u64, u64),
}

#[derive(Clone, Debug, Serialize)]
pub struct OrientedNode {
    pub parent: (u64, u64),
    pub left: (u64, u64),
    pub right: (u64, u64),
    pub lambda: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct SignedLeaf {
    pub base_index: usize,
    pub sign: i8,
}

#[derive(Clone, Debug, Serialize)]
pub struct RecoveredChain {
    pub points: Vec<(u64, u64)>,
    pub signed_leaves: Vec<SignedLeaf>,
    /// Root first, then down the left-prefix chain.
    pub nodes: Vec<OrientedNode>,
}

/// Curve/subgroup and factor-base construction are validated once, separately
/// from checking supplied chain certificates. This is not a root finder.
pub struct RationalChainGuard {
    curve: Curve,
    base: Base,
    semaev: SemaevCurve,
}

impl RationalChainGuard {
    pub fn new(curve: Curve, depth: usize, ar: &mut Arithmetic) -> Result<Self, String> {
        check_curve(curve, ar)?;
        let base = Base::build(curve, depth, ar)?;
        Ok(Self {
            curve,
            base,
            semaev: SemaevCurve::new(curve.p, curve.a, curve.b),
        })
    }

    /// Check the two quadratic oriented-node equations as well as the original
    /// S3 equation, rational point equations, and the independent group sum.
    pub fn verify_node(&self, node: &OrientedNode, ar: &mut Arithmetic) -> Result<(), String> {
        let e = self.curve;
        let f = self.semaev.f;
        let (r, s) = node.parent;
        let (x, yx) = node.left;
        let (z, yz) = node.right;
        if [r, s, x, yx, z, yz, node.lambda].iter().any(|&v| v >= e.p) || s == 0 {
            return Err("node coordinates are noncanonical or parent has zero ordinate".into());
        }
        if !ar.on_curve(e, Some(node.parent)) {
            return Err("node parent is not on the original curve".into());
        }
        if self.semaev.s3(x, z, r) != 0 {
            return Err("original S3 equation is not zero".into());
        }
        let dx = f.sub(x, r);
        let dz = f.sub(z, r);
        let tangent = f.add(f.mul(3, f.mul(r, r)), e.a);
        if f.mul(node.lambda, node.lambda) != f.add(f.add(x, z), r)
            || f.mul(f.mul(2, s), node.lambda) != f.sub(f.mul(dx, dz), tangent)
            || yx != f.sub(f.mul(node.lambda, dx), s)
            || yz != f.sub(f.mul(node.lambda, dz), s)
        {
            return Err("quadratic oriented-node certificate failed".into());
        }
        if !ar.on_curve(e, Some(node.left)) || !ar.on_curve(e, Some(node.right)) {
            return Err("recovered child is not a rational curve point".into());
        }
        if ar.add(e, Some(node.left), Some(node.right)) != Some(node.parent) {
            return Err("oriented-node group replay failed".into());
        }
        Ok(())
    }

    fn recover_node(
        &self,
        x: u64,
        z: u64,
        parent: (u64, u64),
        ar: &mut Arithmetic,
    ) -> Result<OrientedNode, String> {
        let e = self.curve;
        let f = self.semaev.f;
        let (r, s) = parent;
        if [x, z, r, s].iter().any(|&v| v >= e.p) || s == 0 {
            return Err("finite canonical coordinates and nonzero parent ordinate required".into());
        }
        if self.semaev.s3(x, z, r) != 0 {
            return Err("original S3 equation is not zero".into());
        }
        let dx = f.sub(x, r);
        let dz = f.sub(z, r);
        let tangent = f.add(f.mul(3, f.mul(r, r)), e.a);
        let lambda = f.mul(f.sub(f.mul(dx, dz), tangent), f.inv(f.mul(2, s)));
        let node = OrientedNode {
            parent,
            left: (x, f.sub(f.mul(lambda, dx), s)),
            right: (z, f.sub(f.mul(lambda, dz), s)),
            lambda,
        };
        self.verify_node(&node, ar)?;
        Ok(node)
    }

    /// Recover signed rational leaves from an actual all-Fp affine chain.
    /// The caller supplies a certificate; this method performs no base-tuple
    /// search, no Fp scan and no extension-field root search.
    pub fn recover(
        &self,
        cert: &ChainCertificate,
        ar: &mut Arithmetic,
    ) -> Result<RecoveredChain, String> {
        let e = self.curve;
        let m = cert.leaf_x.len();
        if m < 2 || m as u64 > e.n || cert.prefix_x.len() != m - 2 {
            return Err("certificate needs 2 <= m <= n and exactly m-2 auxiliaries".into());
        }
        if cert
            .leaf_x
            .iter()
            .chain(cert.prefix_x.iter())
            .any(|&x| x >= e.p)
            || cert.target.0 >= e.p
            || cert.target.1 >= e.p
            || cert.target.1 == 0
            || !ar.on_curve(e, Some(cert.target))
        {
            return Err("certificate is outside the finite Fp target domain".into());
        }
        if cert.leaf_x.iter().any(|x| !self.base.values.contains(x)) {
            return Err("leaf lies outside the original full Dickson fibre".into());
        }
        let mut current = cert.target;
        let mut points = vec![(0, 0); m];
        let mut nodes = Vec::with_capacity(m - 1);
        for k in (2..=m).rev() {
            let left_x = if k == 2 {
                cert.leaf_x[0]
            } else {
                cert.prefix_x[k - 3]
            };
            let node = self.recover_node(left_x, cert.leaf_x[k - 1], current, ar)?;
            points[k - 1] = node.right;
            current = node.left;
            nodes.push(node);
        }
        points[0] = current;
        let mut signed_leaves = Vec::with_capacity(m);
        for (&point, &expected_x) in points.iter().zip(&cert.leaf_x) {
            if point.0 != expected_x {
                return Err("leaf-order replay failed".into());
            }
            let base_index = self
                .base
                .points
                .iter()
                .position(|p| p.0 == point.0)
                .ok_or("recovered leaf is absent from the rational factor base")?;
            let canonical = self.base.points[base_index];
            let sign = if point == canonical {
                1
            } else if Some(point) == ar.neg(e, Some(canonical)) {
                -1
            } else {
                return Err("leaf ordinate does not match either original rational lift".into());
            };
            signed_leaves.push(SignedLeaf { base_index, sign });
        }
        if sum_points(e, &points, ar) != Some(cert.target) {
            return Err("complete signed relation replay failed".into());
        }
        Ok(RecoveredChain {
            points,
            signed_leaves,
            nodes,
        })
    }
}

fn sum_points(e: Curve, points: &[(u64, u64)], ar: &mut Arithmetic) -> Point {
    points.iter().fold(None, |sum, &p| ar.add(e, sum, Some(p)))
}

/// Validator-only construction from supplied points. Infinity is not silently
/// assigned an affine x coordinate. No known discrete log is used.
fn certificate_for_order(
    e: Curve,
    points: &[(u64, u64)],
    ar: &mut Arithmetic,
) -> Result<ChainCertificate, String> {
    if points.len() < 2 {
        return Err("at least two leaves required".into());
    }
    let mut total = None;
    let mut prefix_x = Vec::new();
    for (i, &p) in points.iter().enumerate() {
        if !ar.on_curve(e, Some(p)) {
            return Err("input leaf is not a rational point".into());
        }
        total = ar.add(e, total, Some(p));
        let current = total.ok_or("identity prefix has no affine certificate")?;
        if i >= 1 && i + 1 < points.len() {
            prefix_x.push(current.0);
        }
    }
    Ok(ChainCertificate {
        leaf_x: points.iter().map(|p| p.0).collect(),
        prefix_x,
        target: total.ok_or("identity target has no affine certificate")?,
    })
}

/// Constructive ordering lemma, applied only to already-supplied signed points.
/// It preserves the exact signed multiset. It does not find a relation.
fn no_zero_prefix_order(
    e: Curve,
    points: &[(u64, u64)],
    ar: &mut Arithmetic,
) -> Result<Vec<usize>, String> {
    if points.is_empty() || points.len() as u64 > e.n {
        return Err("ordering lemma requires 1 <= m <= n".into());
    }
    if points.iter().any(|&p| !ar.on_curve(e, Some(p))) {
        return Err("ordering input has a nonpoint".into());
    }
    let mut total = sum_points(e, points, ar).ok_or("ordering lemma excludes zero total")?;
    let mut remaining: Vec<usize> = (0..points.len()).collect();
    let mut removed = Vec::with_capacity(points.len());
    while remaining.len() > 1 {
        let position = remaining
            .iter()
            .position(|&i| points[i] != total)
            .ok_or("prime-order nonzero-total peeling lemma failed")?;
        let i = remaining.remove(position);
        total = ar
            .add(e, Some(total), ar.neg(e, Some(points[i])))
            .ok_or("peeling unexpectedly produced zero subtotal")?;
        removed.push(i);
    }
    if points[remaining[0]] != total {
        return Err("last peeling point differs from subtotal".into());
    }
    removed.push(remaining[0]);
    removed.reverse();
    let permutation: BTreeSet<_> = removed.iter().copied().collect();
    if permutation.len() != points.len() || permutation.iter().copied().ne(0..points.len()) {
        return Err("peeling output is not a permutation".into());
    }
    Ok(removed)
}

fn exact_signed_witnesses(
    guard: &RationalChainGuard,
    xs: &[u64],
    target: (u64, u64),
    ar: &mut Arithmetic,
) -> Result<Vec<Vec<i8>>, String> {
    let canonical: Vec<_> = xs
        .iter()
        .map(|x| {
            guard
                .base
                .points
                .iter()
                .find(|p| p.0 == *x)
                .copied()
                .ok_or("reference x is not in rational base")
        })
        .collect::<Result<_, _>>()?;
    let mut answers = Vec::new();
    for bits in 0..(1usize << xs.len()) {
        let mut sum = None;
        let mut signs = Vec::with_capacity(xs.len());
        for (i, &p) in canonical.iter().enumerate() {
            let sign = if bits & (1 << i) == 0 { 1 } else { -1 };
            let q = if sign == 1 {
                Some(p)
            } else {
                ar.neg(guard.curve, Some(p))
            };
            sum = ar.add(guard.curve, sum, q);
            signs.push(sign);
        }
        if sum == Some(target) {
            answers.push(signs);
        }
    }
    Ok(answers)
}

fn quadratic(guard: &RationalChainGuard, x: u64, z: u64) -> Vec<u64> {
    let nodes = [0, 1, 2];
    let values: Vec<_> = nodes.iter().map(|&t| guard.semaev.s3(x, z, t)).collect();
    interpolate(guard.semaev.f, &nodes, &values)
}

fn ghost_control(guard: &RationalChainGuard, ar: &mut Arithmetic) -> Result<Value, String> {
    let e = guard.curve;
    let f = guard.semaev.f;
    let r = guard.base.points[0];
    let w = *guard
        .base
        .values
        .iter()
        .find(|&&x| !guard.base.points.iter().any(|p| p.0 == x))
        .ok_or("frozen fibre has no nonliftable abscissa")?;
    let rhs_w = ar.rhs(e, w);
    if rhs_w == 0 || ar.sqrt(rhs_w, e.p).is_some() {
        return Err("ghost leaf is not strictly nonliftable".into());
    }
    let q = quadratic(guard, w, r.0);
    if q.len() != 3 || q[2] == 0 {
        return Err("mixed S3 should be a genuine quadratic".into());
    }
    let discriminant = f.sub(f.mul(q[1], q[1]), f.mul(4, f.mul(q[2], q[0])));
    let disc_identity = f.mul(16, f.mul(rhs_w, ar.rhs(e, r.0)));
    if discriminant != disc_identity || f.pow(discriminant, (e.p - 1) / 2) != e.p - 1 {
        return Err("mixed S3 nonsquare-discriminant certificate failed".into());
    }
    let mut xp_minus_x = poly_powmod(f, &[0, 1], e.p, &q);
    xp_minus_x.resize(xp_minus_x.len().max(2), 0);
    xp_minus_x[1] = f.sub(xp_minus_x[1], 1);
    let rational_gcd = poly_gcd(f, &q, &trim(xp_minus_x));
    if rational_gcd != vec![1] || guard.semaev.summation(&[w, r.0, w], r.0) != 0 {
        return Err("S4 projection / rational-auxiliary separation failed".into());
    }

    let f2 = Fp2::new(e.p)?;
    let ec = ExtCurve {
        f: f2,
        a: f2.from_fp(e.a),
        b: f2.from_fp(e.b),
    };
    let p2 = ec
        .lift_x(f2.from_fp(w))
        .into_iter()
        .next()
        .ok_or("Fp2 lift missing")?;
    let r2 = ExtPoint::affine(f2.from_fp(r.0), f2.from_fp(r.1));
    let frobenius_p = ExtPoint::affine(f2.frob(p2.x), f2.frob(p2.y));
    if p2.y[1] == 0 || frobenius_p != ec.neg(p2) || !ec.is_on_curve(p2) || !ec.is_on_curve(r2) {
        return Err("anti-rational Fp2 point certificate failed".into());
    }
    let mut ops = GroupOps::default();
    let mixed = ec.add(&mut ops, p2, r2);
    let mixed_other = ec.add(&mut ops, ec.neg(p2), r2);
    if mixed.infinity
        || mixed_other.infinity
        || mixed.x[1] == 0
        || mixed_other.x[1] == 0
        || !ec.is_on_curve(mixed)
        || !ec.is_on_curve(mixed_other)
    {
        return Err("mixed sums must have nonrational finite abscissae".into());
    }
    for u in [mixed.x, mixed_other.x] {
        let value = q
            .iter()
            .rev()
            .fold(f2.zero(), |v, &c| f2.add(f2.mul(v, u), f2.from_fp(c)));
        if !f2.is_zero(value) {
            return Err("explicit Fp2 auxiliary does not satisfy S3".into());
        }
    }
    if mixed_other.x != f2.frob(mixed.x)
        || ec.add(&mut ops, mixed, ec.neg(p2)) != r2
        || !ec.add(&mut ops, p2, ec.neg(p2)).infinity
    {
        return Err("Fp2 ghost signed group replay failed".into());
    }
    // A literal Fp auxiliary cannot certify this ghost. The gcd above proves
    // that for every Fp value; this one damaged certificate exercises the API.
    let bad = ChainCertificate {
        leaf_x: vec![w, r.0, w],
        prefix_x: vec![mixed.x[0]],
        target: r,
    };
    if guard.recover(&bad, ar).is_ok() {
        return Err("rational-chain guard accepted the nonrational ghost".into());
    }
    Ok(json!({
        "nonliftable_w":w,"rational_R":r,"q_coefficients_ascending":q,
        "discriminant":discriminant,"discriminant_is_nonsquare":true,
        "gcd_q_Xp_minus_X":rational_gcd,"projected_S4":0,
        "Fp2_nonresidue":f2.nu,"P":p2,"R":r2,"P_plus_R":mixed,
        "minus_P_plus_R":mixed_other,"Fp2_signed_replay":true,
        "no_Fp_auxiliary":true,"guard_rejects_ghost":true,
        "group_ledger":ops
    }))
}

fn identity_prefix_control(
    guard: &RationalChainGuard,
    ar: &mut Arithmetic,
) -> Result<Value, String> {
    let e = guard.curve;
    let p = guard.base.points[0];
    let q = guard.base.points[1];
    let minus_p = ar
        .neg(e, Some(p))
        .ok_or("negative of affine point is identity")?;
    let points = [p, minus_p, q];
    if certificate_for_order(e, &points, ar).is_ok() || sum_points(e, &points, ar) != Some(q) {
        return Err("fixed cancelling-prefix control failed".into());
    }
    let double_p = ar
        .add(e, Some(p), Some(p))
        .ok_or("doubling became identity")?;
    let forced = ChainCertificate {
        leaf_x: vec![p.0, p.0, q.0],
        prefix_x: vec![double_p.0],
        target: q,
    };
    if guard.recover(&forced, ar).is_ok() || guard.semaev.summation(&[p.0, p.0, q.0], q.0) != 0 {
        return Err("infinity versus affine S4 control failed".into());
    }
    let order = no_zero_prefix_order(e, &points, ar)?;
    let reordered: Vec<_> = order.iter().map(|&i| points[i]).collect();
    let cert = certificate_for_order(e, &reordered, ar)?;
    let recovered = guard.recover(&cert, ar)?;
    Ok(json!({
        "fixed_points":points,"fixed_first_sum_is_identity":true,
        "collapsed_S4":0,"forced_finite_auxiliary_rejected":true,
        "permutation":order,"reordered_certificate":cert,"recovered":recovered
    }))
}

/// Reordering completeness belongs to a chain (comb), not every fixed binary
/// tree. Every balanced 2+2 partition of P,P,P,-P has an identity child.
fn balanced_tree_exception(
    guard: &RationalChainGuard,
    ar: &mut Arithmetic,
) -> Result<Value, String> {
    let e = guard.curve;
    let p = guard.base.points[0];
    let minus_p = ar
        .neg(e, Some(p))
        .ok_or("negative affine point is identity")?;
    let points = [p, p, p, minus_p];
    let total = sum_points(e, &points, ar).ok_or("balanced counterexample has zero total")?;
    let pairings = [([0, 1], [2, 3]), ([0, 2], [1, 3]), ([0, 3], [1, 2])];
    let mut records = Vec::new();
    for (left, right) in pairings {
        let l = ar.add(e, Some(points[left[0]]), Some(points[left[1]]));
        let r = ar.add(e, Some(points[right[0]]), Some(points[right[1]]));
        if l.is_some() && r.is_some() {
            return Err("balanced P,P,P,-P pairing unexpectedly has two finite children".into());
        }
        records.push(json!({"left_indices":left,"right_indices":right,"left_sum":l,"right_sum":r}));
    }
    let order = no_zero_prefix_order(e, &points, ar)?;
    let reordered: Vec<_> = order.iter().map(|&i| points[i]).collect();
    let cert = certificate_for_order(e, &reordered, ar)?;
    let recovered = guard.recover(&cert, ar)?;
    Ok(json!({
        "signed_multiset":points,"nonzero_total":total,
        "all_three_balanced_pairings_have_identity_child":true,"pairings":records,
        "comb_permutation":order,"comb_certificate":cert,"comb_recovered":recovered,
        "scope":"Permutation completeness is proved for a comb; it is not completeness of every prescribed balanced tree."
    }))
}

#[derive(Default, Debug, Serialize)]
struct PanelCounts {
    arity: usize,
    signed_alphabet_size: usize,
    ordered_signed_tuples: u64,
    zero_total_tuples: u64,
    nonzero_total_tuples: u64,
    original_order_with_identity_prefix: u64,
    verified_after_permutation: u64,
    fixed_order_verified: u64,
    reference_signed_assignments: u64,
    corrupted_auxiliary_rejections: u64,
    corrupted_valid_target_rejections: u64,
    corrupted_lambda_rejections: u64,
}

fn tiny_panel(guard: &RationalChainGuard, m: usize) -> Result<Value, String> {
    let e = guard.curve;
    let f = guard.semaev.f;
    let mut fixture_ar = Arithmetic::default();
    let mut verifier_ar = Arithmetic::default();
    let mut reference_ar = Arithmetic::default();
    let p = guard.base.points[0];
    let q = guard.base.points[1];
    let alphabet = [
        p,
        fixture_ar.neg(e, Some(p)).unwrap(),
        q,
        fixture_ar.neg(e, Some(q)).unwrap(),
    ];
    let public_targets: Vec<_> = (2..=5)
        .map(|k| fixture_ar.scalar(e, k, Some(e.g)).unwrap())
        .collect();
    let mut count = PanelCounts {
        arity: m,
        signed_alphabet_size: alphabet.len(),
        ..PanelCounts::default()
    };
    let mut unique = BTreeSet::new();
    for mut code in 0..alphabet.len().pow(m as u32) {
        let points: Vec<_> = (0..m)
            .map(|_| {
                let point = alphabet[code % alphabet.len()];
                code /= alphabet.len();
                point
            })
            .collect();
        count.ordered_signed_tuples += 1;
        let Some(total) = sum_points(e, &points, &mut fixture_ar) else {
            count.zero_total_tuples += 1;
            if no_zero_prefix_order(e, &points, &mut fixture_ar).is_ok() {
                return Err("ordering accepted zero total".into());
            }
            continue;
        };
        count.nonzero_total_tuples += 1;
        match certificate_for_order(e, &points, &mut fixture_ar) {
            Ok(cert) => {
                guard.recover(&cert, &mut verifier_ar)?;
                count.fixed_order_verified += 1;
            }
            Err(_) => {
                count.original_order_with_identity_prefix += 1;
            }
        }
        let order = no_zero_prefix_order(e, &points, &mut fixture_ar)?;
        let reordered: Vec<_> = order.iter().map(|&i| points[i]).collect();
        let cert = certificate_for_order(e, &reordered, &mut fixture_ar)?;
        if cert.target != total {
            return Err("ordering changed the signed total".into());
        }
        let recovered = guard.recover(&cert, &mut verifier_ar)?;
        let reference = exact_signed_witnesses(guard, &cert.leaf_x, total, &mut reference_ar)?;
        count.reference_signed_assignments += 1 << m;
        let signs: Vec<_> = recovered
            .signed_leaves
            .iter()
            .map(|leaf| leaf.sign)
            .collect();
        if !reference.contains(&signs) {
            return Err("recovered signs absent from complete reference enumeration".into());
        }
        count.verified_after_permutation += 1;
        unique.insert((cert.leaf_x.clone(), cert.prefix_x.clone(), cert.target));

        let invalid_aux = (1..=3)
            .map(|delta| f.add(cert.prefix_x[0], delta))
            .find(|&u| guard.semaev.s3(cert.leaf_x[0], cert.leaf_x[1], u) != 0)
            .ok_or("three distinct mutations cannot all be roots of a nonzero quadratic")?;
        let mut damaged = cert.clone();
        damaged.prefix_x[0] = invalid_aux;
        if guard.recover(&damaged, &mut verifier_ar).is_ok() {
            return Err("corrupted auxiliary was accepted".into());
        }
        count.corrupted_auxiliary_rejections += 1;

        let top_left = *cert.prefix_x.last().unwrap();
        let top_right = *cert.leaf_x.last().unwrap();
        let wrong_target = *public_targets
            .iter()
            .find(|t| guard.semaev.s3(top_left, top_right, t.0) != 0)
            .ok_or("four public target x values cannot all solve a nonzero quadratic")?;
        damaged = cert.clone();
        damaged.target = wrong_target;
        if guard.recover(&damaged, &mut verifier_ar).is_ok() {
            return Err("corrupted valid curve target was accepted".into());
        }
        count.corrupted_valid_target_rejections += 1;

        let mut damaged_node = recovered.nodes[0].clone();
        damaged_node.lambda = f.add(damaged_node.lambda, 1);
        if guard.verify_node(&damaged_node, &mut verifier_ar).is_ok() {
            return Err("corrupted slope certificate was accepted".into());
        }
        count.corrupted_lambda_rejections += 1;
    }
    if count.ordered_signed_tuples != 4u64.pow(m as u32)
        || count.verified_after_permutation != count.nonzero_total_tuples
        || count.fixed_order_verified + count.original_order_with_identity_prefix
            != count.nonzero_total_tuples
    {
        return Err("frozen panel census is incomplete".into());
    }
    Ok(json!({
        "counts":count,"unique_reordered_certificates":unique.len(),
        "fixture_group_ledger":fixture_ar.counts,
        "verifier_group_ledger":verifier_ar.counts,
        "independent_enumeration_group_ledger":reference_ar.counts,
        "counter_scope":"Original Arithmetic counters only; imported polynomial/Fp operations are not combined into a total cost. No performance inference."
    }))
}

fn run() -> Result<Value, String> {
    let mut setup_ar = Arithmetic::default();
    let guard = RationalChainGuard::new(PRIME31, 6, &mut setup_ar)?;
    if guard.base.terminal != 933_677_148
        || guard.base.values.len() != 64
        || guard.base.points.len() != 31
    {
        return Err("original frozen depth-six Dickson fibre changed".into());
    }
    let mut controls_ar = Arithmetic::default();
    let ghost = ghost_control(&guard, &mut controls_ar)?;
    let identity = identity_prefix_control(&guard, &mut controls_ar)?;
    let balanced = balanced_tree_exception(&guard, &mut controls_ar)?;
    let panels = [tiny_panel(&guard, 3)?, tiny_panel(&guard, 4)?];
    Ok(json!({
        "schema":"prime31-rational-chain-guard-v1","iteration":6,
        "standing":"producer correctness evidence; pcc:unverified",
        "scope":"fixed finite-Fp chain certificate verifier; no relation finder, benchmark or asymptotic speedup",
        "p":PRIME31.p,"n":PRIME31.n,"curve_a":PRIME31.a,"curve_b":PRIME31.b,
        "depth":6,"terminal":guard.base.terminal,"full_fibre_size":64,"rational_base_size":31,
        "control_canonical_points":&guard.base.points[..2],
        "domain":"All leaf and auxiliary abscissae are canonical Fp values; target is signed nonzero rational; infinity branches require permutation or separate projective handling.",
        "prior_art":"Existing prime31_m8_oriented_f4 chord-tangent leaf-pair lift; standard Semaev and group law.",
        "ghost":ghost,"identity_prefix":identity,"balanced_tree_exception":balanced,"exhaustive_panels":panels,
        "setup_group_ledger":setup_ar.counts,"control_group_ledger":controls_ar.counts,
        "root_search_performed":false,"timings_recorded":false,"asymptotic_gain_established":false
    }))
}

fn main() -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string_pretty(&run()?).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod guard_tests {
    use super::*;

    #[test]
    fn fixed_projected_ghost_and_rational_auxiliary_separation() {
        let mut ar = Arithmetic::default();
        let guard = RationalChainGuard::new(PRIME31, 6, &mut ar).unwrap();
        let report = ghost_control(&guard, &mut ar).unwrap();
        assert_eq!(report["gcd_q_Xp_minus_X"], json!([1]));
        assert_eq!(report["guard_rejects_ghost"], true);
    }

    #[test]
    fn identity_prefix_requires_an_explicit_order_change() {
        let mut ar = Arithmetic::default();
        let guard = RationalChainGuard::new(PRIME31, 6, &mut ar).unwrap();
        let report = identity_prefix_control(&guard, &mut ar).unwrap();
        assert_eq!(report["forced_finite_auxiliary_rejected"], true);
    }

    #[test]
    fn comb_completeness_does_not_imply_balanced_tree_completeness() {
        let mut ar = Arithmetic::default();
        let guard = RationalChainGuard::new(PRIME31, 6, &mut ar).unwrap();
        let report = balanced_tree_exception(&guard, &mut ar).unwrap();
        assert_eq!(
            report["all_three_balanced_pairings_have_identity_child"],
            true
        );
    }

    #[test]
    fn exhaustive_four_signed_value_controls_for_arities_three_and_four() {
        let mut ar = Arithmetic::default();
        let guard = RationalChainGuard::new(PRIME31, 6, &mut ar).unwrap();
        for m in [3, 4] {
            let report = tiny_panel(&guard, m).unwrap();
            assert_eq!(
                report["counts"]["ordered_signed_tuples"],
                4u64.pow(m as u32)
            );
            assert_eq!(
                report["counts"]["verified_after_permutation"],
                report["counts"]["nonzero_total_tuples"]
            );
        }
    }
}
