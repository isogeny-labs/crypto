//! Exact S3 discriminant and finite-rational S4 projection equivalence audit.
//!
//! On the inherited ordinary prime31 fixture, a fully rational S3
//! quadratic with both input points lifted has discriminant
//!     b^2 - 4ac = 16 (x^3+A*x+B)(z^3+A*z+B).
//! Its square root is therefore +/-4*y*w.  The repeated nested S3
//! projection in EXP-R31-G has a generic EC-addition realization.
//! This is a *negative classification control*, not a faster algebraic
//! relation oracle or a complete unknown-scalar ECDLP solver.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use serde_json::json;
use std::collections::BTreeSet;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;

fn add(x: u64, y: u64) -> u64 {
    let z = x + y;
    if z >= P { z - P } else { z }
}
fn sub(x: u64, y: u64) -> u64 {
    if x >= y { x - y } else { x + P - y }
}
fn mul(x: u64, y: u64) -> u64 {
    ((u128::from(x) * u128::from(y)) % u128::from(P)) as u64
}
fn pow(mut x: u64, mut e: u64) -> u64 {
    let mut acc = 1;
    while e > 0 {
        if e & 1 == 1 { acc = mul(acc, x); }
        x = mul(x, x);
        e >>= 1;
    }
    acc
}
fn rhs(x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(A, x)), B)
}

// Only field arithmetic is used in this certificate. Unlike the earlier
// projection, the caller gives both EC ordinates, making all roots rational
// by construction. The caller's EC group operations are counted separately.
// For a nonzero double root or diagonal, fail closed rather than undercount.
fn s3_known_ordinates(u: (u64, u64), v: (u64, u64)) -> Result<[u64; 2], String> {
    let ((x, y), (z, w)) = (u, v);
    if mul(y, y) != rhs(x) || mul(w, w) != rhs(z) {
        return Err("invalid input EC ordinate".into());
    }
    let aa = mul(sub(x, z), sub(x, z));
    if aa == 0 { return Err("diagonal S3 not supported in this corpus".into()); }
    let xz = mul(x, z);
    let bb = sub(0, mul(2, add(mul(add(x, z), add(xz, A)), mul(2, B))));
    let cc = sub(mul(sub(xz, A), sub(xz, A)), mul(4, mul(B, add(x, z))));
    let discriminant = sub(mul(bb, bb), mul(4, mul(aa, cc)));
    let discriminant_formula = mul(16, mul(rhs(x), rhs(z)));
    let root = mul(4, mul(y, w));
    if discriminant != discriminant_formula || mul(root, root) != discriminant || root == 0 {
        return Err("S3 discriminant-square identity failed".into());
    }
    let inverse = pow(mul(2, aa), P - 2);
    let mut ans = [mul(sub(root, bb), inverse), mul(sub(sub(0, bb), root), inverse)];
    ans.sort_unstable();
    if ans[0] == ans[1] || ans.iter().any(|&r|
        add(add(mul(aa, mul(r, r)), mul(bb, r)), cc) != 0)
    {
        return Err("quadratic root residual or distinctness failed".into());
    }
    Ok(ans)
}

fn fnv_le32(roots: &BTreeSet<u64>) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for &v in roots {
        for byte in (v as u32).to_le_bytes() {
            h = (h ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
        }
    }
    h
}

fn main() -> Result<(), String> {
    if std::env::args().len() != 1 { return Err("no arguments supported".into()); }
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    if PRIME31.p != P || PRIME31.a != A || PRIME31.b != B {
        return Err("prime31 coefficients changed".into());
    }
    let fb = Base::build(PRIME31, 6, &mut ar)?;
    if fb.terminal != 933_677_148 || fb.points.len() != 31 {
        return Err("registered Dickson base changed".into());
    }
    let mut roots = BTreeSet::new();
    let (mut pair_checks, mut triple_checks) = (0usize, 0usize);
    let validator_start = ar.counts.group_additions;
    for i in 0..fb.points.len() {
        for j in (i + 1)..fb.points.len() {
            let pi = fb.points[i];
            let pj = fb.points[j];
            let pair_roots = s3_known_ordinates(pi, pj)?;
            let plus = ar.add(PRIME31, Some(pi), Some(pj))
                .ok_or("pair sum is identity")?;
            let minus = ar.add(PRIME31, Some(pi), ar.neg(PRIME31, Some(pj)))
                .ok_or("pair difference is identity")?;
            let mut semantic = [plus.0, minus.0];
            semantic.sort_unstable();
            if pair_roots != semantic { return Err("S3 pair semantic mismatch".into()); }
            pair_checks += 1;
            for k in (j + 1)..fb.points.len() {
                let pk = fb.points[k];
                for pair in [plus, minus] {
                    let qroots = s3_known_ordinates(pair, pk)?;
                    let pp = ar.add(PRIME31, Some(pair), Some(pk))
                        .ok_or("triple sum is identity")?;
                    let pm = ar.add(PRIME31, Some(pair), ar.neg(PRIME31, Some(pk)))
                        .ok_or("triple difference is identity")?;
                    let mut semantic = [pp.0, pm.0];
                    semantic.sort_unstable();
                    if qroots != semantic { return Err("S3 triple semantic mismatch".into()); }
                    if !roots.insert(qroots[0]) || !roots.insert(qroots[1]) {
                        return Err("unexpected repeated rational S4 abscissa".into());
                    }
                    triple_checks += 1;
                }
            }
        }
    }
    if pair_checks != 465 || triple_checks != 8_990 || roots.len() != 17_980
        || fnv_le32(&roots) != 0xcc15_d719_e3ea_f8c2
    {
        return Err("inherited S4 triple root corpus changed".into());
    }
    println!("{}", json!({
        "standing": "[pcc:unverified]",
        "curve": "ordinary cofactor-one prime-order p=2^31-1",
        "method": "known-ordinate S3 discriminant certificate / group-equivalence control",
        "pairs": pair_checks,
        "triple_s3_calls": triple_checks,
        "rational_quadratic_calls": pair_checks + triple_checks,
        "distinct_triple_abscissae": roots.len(),
        "root_fnv1a_le32": format!("{:016x}", fnv_le32(&roots)),
        "ec_group_additions_post_root_identity": ar.counts.group_additions-validator_start,
        "classification": "generic decomposition; ordinate inputs and EC law reveal signed sums",
        "target_dependent_polynomial_solver": false,
        "cold_ECDLP_solve": false
    }));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn algebraic_discriminant_handles_real_prime31_points() {
        let mut ar = Arithmetic::default();
        let p = Base::build(PRIME31, 6, &mut ar).unwrap();
        assert_eq!(s3_known_ordinates(p.points[0],p.points[1]).unwrap().len(),2);
        assert!(s3_known_ordinates((p.points[0].0, 0),p.points[1]).is_err());
        assert!(s3_known_ordinates(p.points[0],p.points[0]).is_err());
    }
}
