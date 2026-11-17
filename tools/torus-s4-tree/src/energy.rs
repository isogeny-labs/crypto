//! Arithmetic certificate for a PROPOSED geometric average-energy argument.
//! These checks do not validate the map-degree argument or establish solver cost.
use crate::{Fp2, Result, TorusParameters};
use num_bigint::BigUint;
pub fn arithmetic_certificate(params: &TorusParameters) -> Result<serde_json::Value> {
    let f = &params.f;
    let a = BigUint::from(486662u32);
    let q = f.mul(&f.sub(&a, &2u32.into()), &f.inv(&f.add(&a, &2u32.into()))?);
    if q.modpow(&((&f.p - 1u32) >> 1), &f.p) != &f.p - 1u32 {
        return Err("branch ratio parameter is not nonsquare".into());
    }
    let i = f.sqrt(&(&f.p - 1u32))?.ok_or("missing sqrt(-1)")?;
    let b = f
        .sqrt(&f.mul(&q, &f.inv(&2u32.into())?))?
        .ok_or("missing sqrt(q/2)")?;
    let r = Fp2 { a: 0u32.into(), b };
    let ri = r.inverse(f)?;
    let ii = Fp2 {
        a: i,
        b: 0u32.into(),
    };
    let roots = vec![
        ii.clone(),
        ii.neg(f),
        r.clone(),
        r.neg(f),
        ri.clone(),
        ri.neg(f),
    ];
    let ca = f.sub(&4u32.into(), &f.square(&a));
    let cb = f.add(&12u32.into(), &f.square(&a));
    let coefficients = [
        ca.clone(),
        0u32.into(),
        cb.clone(),
        0u32.into(),
        cb,
        0u32.into(),
        ca,
    ];
    let mut norms = vec![];
    for (j, z) in roots.iter().enumerate() {
        if roots[..j].contains(z) {
            return Err("repeated branch root".into());
        }
        let mut value = Fp2 {
            a: 0u32.into(),
            b: 0u32.into(),
        };
        for c in coefficients.iter().rev() {
            value = value.mul(z, f);
            value.a = f.add(&value.a, c);
        }
        if value
            != (Fp2 {
                a: 0u32.into(),
                b: 0u32.into(),
            })
        {
            return Err("branch polynomial root check failed".into());
        }
        norms.push(z.norm(f));
    }
    let mut unit_pairs = 0;
    for z in &roots {
        for w in &roots {
            let ratio = z.mul(&w.inverse(f)?, f);
            if ratio.norm(f) == BigUint::from(1u32) {
                unit_pairs += 1;
                if ratio != Fp2::one() && ratio != Fp2::one().neg(f) {
                    return Err("unexpected norm-one branch ratio".into());
                }
            }
        }
    }
    let l = BigUint::from(4 * params.m);
    let w0 = BigUint::from(16u32) * &l * (&l - 1u32) * (&l - 2u32) * (&l - 3u32);
    let k: BigUint = BigUint::from(1u32) << 28usize;
    let excluded = BigUint::from(28 * params.m);
    let denominator = &f.p + 1u32 - &excluded;
    let numerator = &k * &w0;
    if numerator >= denominator {
        return Err("proposed energy correction is not below one".into());
    }
    Ok(
        serde_json::json!({"schema":"torus-four.energy-arithmetic/v1","status":"arithmetic only; geometric theorem is a draft requiring independent review","p":f.p.to_string(),"A":a.to_string(),"q":q.to_string(),"branch_polynomial_even_coefficients":coefficients.iter().map(ToString::to_string).collect::<Vec<_>>(),"branch_roots":roots.iter().map(|z|[z.a.to_string(),z.b.to_string()]).collect::<Vec<_>>(),"branch_norms":norms.iter().map(ToString::to_string).collect::<Vec<_>>(),"ratio_pairs_checked":36,"unit_norm_pairs":unit_pairs,"all_unit_norm_ratios_are_plus_minus_one":true,"L":l.to_string(),"signed_ordered_distinct_tuple_count_W0":w0.to_string(),"proposed_collision_bound_K":k.to_string(),"admissible_parameter_lower_bound":denominator.to_string(),"K_times_W0":numerator.to_string(),"K_times_W0_less_than_admissible_parameter_bound":true,"proposed_average_energy_factor_upper_bound":25,"formal_geometric_proof_available":false,"pcc_standing":"unverified"}),
    )
}
