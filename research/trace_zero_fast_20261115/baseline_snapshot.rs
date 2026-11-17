//! Constant-residual S3 solved in the spans of x1+x2 and x1*x2.
//! Existing symmetric S3 algebra, with exact root/lift checks and conservative
//! treatment of rank-deficient affine solution spaces; no SAT assumption.
use crate::binary_ecc::{
    curve::{point_add, point_neg},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use crate::cryptanalysis::{
    binary_semaev::{binary_semaev_s3, solve_quadratic_f2m},
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
    let mut incomplete = false;
    for (r, p) in base.iter().enumerate() {
        if deadline.is_some_and(|d| Instant::now() >= d) {
            out.relation.reason = Some("linearized total deadline");
            return out;
        }
        let residual = point_add(c, target, &point_neg(p));
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
            let columns: Vec<_> = xs
                .iter()
                .map(|x| z2.mul(&x.square(&c.irreducible), &c.irreducible))
                .chain(
                    ws.iter()
                        .map(|w| w.square(&c.irreducible).add(&z.mul(w, &c.irreducible))),
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
                let roots =
                    solve_quadratic_f2m(&F2mElement::one(c.m), &u, &product, c.m, &c.irreducible);
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
