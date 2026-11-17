//! Constant-residual S3 solved in the spans of x1+x2 and x1*x2.
//! Existing symmetric S3 algebra, with exact root/lift checks and conservative
//! treatment of rank-deficient affine solution spaces; no SAT assumption.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly, TraceZeroVariety,
};
use crate::cryptanalysis::{
    binary_semaev::binary_semaev_s3,
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
        TraceZeroRelationReport as Report,
    },
};
use num_bigint::BigUint;
use std::{collections::BTreeMap, time::Instant};

#[derive(Clone, Debug, Default)]
pub struct TraceZeroLinearizedReport {
    pub relation: Report,
    pub source_dimension: usize,
    pub product_dimension: usize,
    pub linear_columns: usize,
    pub systems: usize,
    pub maximum_rank: usize,
    pub maximum_nullity: usize,
}

fn basis(values: impl IntoIterator<Item = BigUint>, n: u32) -> Vec<F2mElement> {
    let mut pivots: Vec<Option<BigUint>> = vec![None; n as usize];
    for mut v in values {
        for bit in (0..n as usize).rev() {
            if v.bit(bit as u64) {
                if let Some(p) = &pivots[bit] {
                    v ^= p;
                } else {
                    pivots[bit] = Some(v);
                    break;
                }
            }
        }
    }
    pivots
        .into_iter()
        .flatten()
        .map(|v| F2mElement::from_biguint(&v, n))
        .collect()
}
fn linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    // Three words cover every supported field (n <= 131). Retain the
    // arbitrary-width path for coefficient spaces wider than 192 bits.
    if n <= 192 && columns.len() <= 192 {
        return packed_linear_solutions(columns, target, n);
    }
    generic_linear_solutions(columns, target, n)
}
fn generic_linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    let mut pivots: Vec<Option<(BigUint, BigUint)>> = vec![None; n as usize];
    let mut kernel = Vec::new();
    let mut rank = 0;
    for (j, col) in columns.iter().enumerate() {
        let mut v = col.to_biguint();
        let mut label = BigUint::from(1u32) << j;
        let mut installed = false;
        for bit in (0..n as usize).rev() {
            if v.bit(bit as u64) {
                if let Some((p, l)) = &pivots[bit] {
                    v ^= p;
                    label ^= l;
                } else {
                    pivots[bit] = Some((v.clone(), label.clone()));
                    rank += 1;
                    installed = true;
                    break;
                }
            }
        }
        if !installed {
            kernel.push(label);
        }
    }
    let mut v = target.to_biguint();
    let mut solution = BigUint::default();
    for bit in (0..n as usize).rev() {
        if v.bit(bit as u64) {
            let (p, l) = pivots[bit].as_ref()?;
            v ^= p;
            solution ^= l;
        }
    }
    Some((solution, kernel, rank))
}

fn packed_linear_solutions(
    columns: &[F2mElement],
    target: &F2mElement,
    n: u32,
) -> Option<(BigUint, Vec<BigUint>, usize)> {
    fn packed(x: &F2mElement) -> [u64; 3] {
        let mut a = [0; 3];
        a[..x.raw_bits().len()].copy_from_slice(x.raw_bits());
        a
    }
    fn leading(a: &[u64; 3]) -> Option<usize> {
        (0..3)
            .rev()
            .find(|&i| a[i] != 0)
            .map(|i| i * 64 + 63 - a[i].leading_zeros() as usize)
    }
    fn big(a: &[u64; 3]) -> BigUint {
        BigUint::from_slice(
            &a.iter()
                .flat_map(|w| [*w as u32, (*w >> 32) as u32])
                .collect::<Vec<_>>(),
        )
    }
    let mut rows = [[0u64; 6]; 192];
    let mut present = [false; 192];
    let mut rank = 0;
    let mut kernel = Vec::new();
    for (j, col) in columns.iter().enumerate() {
        let mut a = packed(col);
        let mut label = [0u64; 3];
        label[j / 64] = 1u64 << (j % 64);
        loop {
            let Some(bit) = leading(&a) else {
                kernel.push(big(&label));
                break;
            };
            debug_assert!(bit < n as usize);
            if !present[bit] {
                present[bit] = true;
                rows[bit][..3].copy_from_slice(&a);
                rows[bit][3..].copy_from_slice(&label);
                rank += 1;
                break;
            }
            for w in 0..3 {
                a[w] ^= rows[bit][w];
                label[w] ^= rows[bit][3 + w];
            }
        }
    }
    let mut a = packed(target);
    let mut solution = [0u64; 3];
    while let Some(bit) = leading(&a) {
        if !present[bit] {
            return None;
        }
        for w in 0..3 {
            a[w] ^= rows[bit][w];
            solution[w] ^= rows[bit][3 + w];
        }
    }
    Some((big(&solution), kernel, rank))
}

/// Montgomery's simultaneous inversion, applied only to ordinary additions.
/// Preserve every input position; identity/doubling/opposite cases use the
/// original group routine. Computing a batch charges even unused tail work.
fn residual_batch(c: &BinaryCurve, target: &BinaryPoint, base: &[BinaryPoint]) -> Vec<BinaryPoint> {
    let BinaryPoint::Affine { x: tx, y: ty } = target else {
        return base.iter().map(point_neg).collect();
    };
    let irr = &c.irreducible;
    let mut out = vec![BinaryPoint::Infinity; base.len()];
    let mut ordinary = Vec::new();
    let mut product = F2mElement::one(c.m);
    for (i, p) in base.iter().enumerate() {
        match p {
            BinaryPoint::Affine { x, y } if x != tx => {
                let den = tx.add(x);
                ordinary.push((i, product.clone(), den.clone(), ty.add(&y.add(x))));
                product = product.mul(&den, irr);
            }
            _ => out[i] = point_add(c, target, &point_neg(p)),
        }
    }
    if ordinary.is_empty() {
        return out;
    }
    let mut inverse = product
        .flt_inverse(irr)
        .expect("nonzero residual denominators");
    for (i, prefix, den, numerator) in ordinary.into_iter().rev() {
        let inv_den = inverse.mul(&prefix, irr);
        inverse = inverse.mul(&den, irr);
        let lambda = numerator.mul(&inv_den, irr);
        let x = lambda.square(irr).add(&lambda).add(&den).add(&c.a);
        let y = lambda.mul(&tx.add(&x), irr).add(&x).add(ty);
        crate::cryptanalysis::group_op_count::count_adds(1);
        out[i] = BinaryPoint::Affine { x, y };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small_curve() -> BinaryCurve {
        BinaryCurve {
            m: 5,
            irreducible: IrreduciblePoly {
                degree: 5,
                low_terms: vec![0, 2],
            },
            a: F2mElement::zero(5),
            b: F2mElement::one(5),
            generator: BinaryPoint::Infinity,
            order: 11u32.into(),
            cofactor: 4u32.into(),
        }
    }

    #[test]
    fn packed_elimination_matches_generic_solutions_kernels_and_inconsistency() {
        let mut seed = 20261115u64;
        for n in [5, 7, 9, 83, 131, 192] {
            for width in [0, 1, 12, 35, 83, 131, 150, 192, 193, 262] {
                let mut next = || {
                    let positions: Vec<_> = (0..n)
                        .filter(|_| {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            seed & 1 != 0
                        })
                        .collect();
                    F2mElement::from_bit_positions(&positions, n)
                };
                let columns: Vec<_> = (0..width).map(|_| next()).collect();
                let random = next();
                let consistent = columns
                    .iter()
                    .step_by(3)
                    .fold(F2mElement::zero(n), |s, c| s.add(c));
                for target in [random, consistent] {
                    assert_eq!(
                        linear_solutions(&columns, &target, n),
                        generic_linear_solutions(&columns, &target, n),
                        "n={n}, width={width}"
                    );
                }
            }
        }
    }

    #[test]
    fn batched_residuals_match_scalar_group_law_including_all_exceptions() {
        let c = small_curve();
        let mut points = vec![BinaryPoint::Infinity];
        for x in 0..32u32 {
            for y in 0..32u32 {
                let p = BinaryPoint::Affine {
                    x: F2mElement::from_biguint(&x.into(), 5),
                    y: F2mElement::from_biguint(&y.into(), 5),
                };
                if c.is_on_curve(&p) {
                    points.push(p);
                }
            }
        }
        for target in &points {
            let got = residual_batch(&c, target, &points);
            let want: Vec<_> = points
                .iter()
                .map(|p| point_add(&c, target, &point_neg(p)))
                .collect();
            assert_eq!(got, want);
            assert!(residual_batch(&c, target, &[]).is_empty());
        }
    }

    #[test]
    fn specialized_monic_roots_match_complete_field_enumeration_and_old_solver() {
        let c = small_curve();
        for u in 0..32u32 {
            for v in 0..32u32 {
                let u = F2mElement::from_biguint(&u.into(), 5);
                let v = F2mElement::from_biguint(&v.into(), 5);
                let got = monic_roots(&u, &v, 5, &c.irreducible);
                let old = crate::cryptanalysis::binary_semaev::solve_quadratic_f2m(
                    &F2mElement::one(5),
                    &u,
                    &v,
                    5,
                    &c.irreducible,
                );
                assert_eq!(got, old, "root order must remain unchanged");
                let mut want: Vec<_> = (0..32u32)
                    .map(|x| F2mElement::from_biguint(&x.into(), 5))
                    .filter(|x| {
                        x.square(&c.irreducible)
                            .add(&u.mul(x, &c.irreducible))
                            .add(&v)
                            .is_zero()
                    })
                    .map(|x| x.to_biguint())
                    .collect();
                let mut actual: Vec<_> = got.iter().map(|x| x.to_biguint()).collect();
                want.sort();
                actual.sort();
                assert_eq!(actual, want);
            }
        }
    }
}

/// Monic specialization: avoid inverting the known leading coefficient 1.
/// The variety admits odd fields, so half-trace gives both actual roots.
fn monic_roots(
    u: &F2mElement,
    product: &F2mElement,
    n: u32,
    irr: &IrreduciblePoly,
) -> Vec<F2mElement> {
    if u.is_zero() {
        return vec![product.square_k_times(n - 1, irr)];
    }
    let inv = u
        .flt_inverse(irr)
        .expect("nonzero monic linear coefficient");
    let arg = product.mul(&inv.square(irr), irr);
    if arg.trace(irr) != 0 {
        return Vec::new();
    }
    let root = u.mul(&arg.half_trace(irr).expect("odd field"), irr);
    vec![root.clone(), root.add(u)]
}
fn point_key(p: &BinaryPoint) -> (BigUint, BigUint) {
    match p {
        BinaryPoint::Affine { x, y } => (x.to_biguint(), y.to_biguint()),
        _ => unreachable!("finite base"),
    }
}

/// For each actual final point R, solve the necessary linear equation
/// v²+z*v+z²*u²+b=0 with u in span(B_x), v in span(B_x*B_x),
/// z=x(T-R). Factor X²+uX+v and replay actual supplied point signs.
/// All candidates of a deficient affine space are required for Refuted.
pub fn decompose_trace_zero_linearized(
    v: &TraceZeroVariety,
    base: &[BinaryPoint],
    target: &BinaryPoint,
    limits: Limits,
) -> TraceZeroLinearizedReport {
    let mut out = TraceZeroLinearizedReport::default();
    let deadline = limits.wall_budget.map(|d| Instant::now() + d);
    if !v.contains(target)
        || base
            .iter()
            .any(|p| matches!(p, BinaryPoint::Infinity) || !v.contains(p))
    {
        out.relation.reason = Some("invalid linearized target/base");
        return out;
    }
    if limits.max_models == 0 || limits.max_patterns == 0 || limits.max_variables == 0 {
        out.relation.reason = Some("zero linearized budget");
        return out;
    }
    if base.is_empty() {
        out.relation.disposition = Disposition::Refuted;
        return out;
    }
    let c = v.curve();
    let mut by_x: BTreeMap<BigUint, Vec<usize>> = BTreeMap::new();
    let mut by_point = BTreeMap::new();
    for (i, p) in base.iter().enumerate() {
        let (x, y) = point_key(p);
        by_x.entry(x.clone()).or_default().push(i);
        by_point.entry((x, y)).or_insert(i);
    }
    let xs = basis(by_x.keys().cloned(), c.m);
    let products: Vec<_> = xs
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            xs[i..]
                .iter()
                .map(|y| x.mul(y, &c.irreducible).to_biguint())
        })
        .collect();
    let ws = basis(products, c.m);
    out.source_dimension = xs.len();
    out.product_dimension = ws.len();
    out.linear_columns = xs.len() + ws.len();
    if out.linear_columns > limits.max_variables as usize {
        out.relation.reason = Some("linear-column allocation cap");
        return out;
    }
    let xs_squared: Vec<_> = xs.iter().map(|x| x.square(&c.irreducible)).collect();
    let ws_squared: Vec<_> = ws.iter().map(|w| w.square(&c.irreducible)).collect();
    // Small bases keep scalar admission/search costs. Larger bases amortize
    // each inversion over at most 32 consecutive residuals, in original order.
    let batch_size = if base.len() >= 64 { 32 } else { 1 };
    let mut residuals = Vec::new();
    let mut residual_start = 0;
    let mut incomplete = false;
    for (r, p) in base.iter().enumerate() {
        if deadline.is_some_and(|d| Instant::now() >= d) {
            out.relation.reason = Some("linearized total deadline");
            return out;
        }
        if r == 0 || r == residual_start + residuals.len() {
            residual_start = r;
            // An immediate relation should not pay for 31 unused residuals.
            residuals = if r == 0 || batch_size == 1 {
                vec![point_add(c, target, &point_neg(p))]
            } else {
                residual_batch(c, target, &base[r..base.len().min(r + batch_size)])
            };
        }
        let residual = residuals[r - residual_start].clone();
        out.relation.patterns_attempted += 1;
        let mut residual_incomplete = false;
        let found = if residual == BinaryPoint::Infinity {
            base.iter()
                .enumerate()
                .find_map(|(i, q)| by_point.get(&point_key(&point_neg(q))).map(|&j| (i, j)))
        } else {
            let BinaryPoint::Affine { x: z, .. } = &residual else {
                unreachable!()
            };
            let z2 = z.square(&c.irreducible);
            let columns: Vec<_> = xs_squared
                .iter()
                .map(|x2| z2.mul(x2, &c.irreducible))
                .chain(
                    ws.iter()
                        .zip(&ws_squared)
                        .map(|(w, w2)| w2.add(&z.mul(w, &c.irreducible))),
                )
                .collect();
            out.systems += 1;
            let Some((solution, kernel, rank)) = linear_solutions(&columns, &c.b, c.m) else {
                out.relation.patterns_refuted += 1;
                continue;
            };
            out.maximum_rank = out.maximum_rank.max(rank);
            out.maximum_nullity = out.maximum_nullity.max(kernel.len());
            let available = limits.max_models.saturating_sub(out.relation.models);
            let total = 1usize.checked_shl(kernel.len() as u32);
            let count = total.unwrap_or(usize::MAX).min(available);
            if total.is_none_or(|t| count < t) {
                incomplete = true;
                residual_incomplete = true;
            }
            let mut pair = None;
            for selector in 0..count {
                if deadline.is_some_and(|d| Instant::now() >= d) {
                    out.relation.reason = Some("linearized coefficient deadline");
                    return out;
                }
                let mut coefficients = solution.clone();
                for (j, k) in kernel.iter().enumerate().take(usize::BITS as usize) {
                    if selector >> j & 1 != 0 {
                        coefficients ^= k;
                    }
                }
                let mut u = F2mElement::zero(c.m);
                let mut product = F2mElement::zero(c.m);
                for (j, x) in xs.iter().enumerate() {
                    if coefficients.bit(j as u64) {
                        u = u.add(x);
                    }
                }
                for (j, w) in ws.iter().enumerate() {
                    if coefficients.bit((xs.len() + j) as u64) {
                        product = product.add(w);
                    }
                }
                out.relation.models += 1;
                let roots = monic_roots(&u, &product, c.m, &c.irreducible);
                for x in roots {
                    let y = x.add(&u);
                    let (Some(is), Some(js)) =
                        (by_x.get(&x.to_biguint()), by_x.get(&y.to_biguint()))
                    else {
                        continue;
                    };
                    if !binary_semaev_s3(&x, &y, z, &c.b, &c.irreducible).is_zero() {
                        out.relation.reason = Some("linearized S3 replay failed");
                        return out;
                    }
                    for &i in is {
                        for &j in js {
                            if point_add(c, &base[i], &base[j]) == residual {
                                pair = Some((i, j));
                                break;
                            }
                        }
                        if pair.is_some() {
                            break;
                        }
                    }
                    if pair.is_some() {
                        break;
                    }
                }
                if pair.is_some() {
                    break;
                }
            }
            pair
        };
        if let Some((i, j)) = found {
            if point_add(c, &point_add(c, &base[i], &base[j]), p) != *target {
                out.relation.reason = Some("linearized original replay failed");
                return out;
            }
            out.relation.indices = Some(vec![i, j, r]);
            out.relation.disposition = Disposition::Found;
            return out;
        }
        if !residual_incomplete {
            out.relation.patterns_refuted += 1;
        }
    }
    if incomplete {
        out.relation.reason = Some("linearized affine-model cap");
    } else {
        out.relation.disposition = Disposition::Refuted;
    }
    out
}
