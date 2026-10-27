//! ecbench adapters for the native three-leads engines. Existing method IDs
//! and previously committed sessions keep their implementations and prices.
use super::methods::{PhaseRecord, ResolvedMethod, SolveReport};
use super::workload::{CurveFacts, CurveSpec};
use crate::cryptanalysis::curve_id;
use crate::cryptanalysis::ic_boundary::FieldOps;
use crate::cryptanalysis::three_leads::{
    self as engine, Counts, NativePhase, Outcome, Point, StudyInstance,
};
use num_bigint::BigUint;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::Instant;

fn coefficients(mut x: u64, p: u64, n: usize) -> Vec<u64> {
    (0..n)
        .map(|_| {
            let a = x % p;
            x /= p;
            a
        })
        .collect()
}
pub fn facts(i: &StudyInstance, spec: &CurveSpec) -> CurveFacts {
    let f = &i.curve.f;
    let order = BigUint::from(i.order);
    let id = if i.curve.binary {
        curve_id::binary(
            f.n as u32,
            &BigUint::from(f.modulus),
            &BigUint::from(i.curve.c[2]),
            &BigUint::from(i.curve.c[0]),
            &order,
            None,
        )
    } else if f.n == 1 {
        curve_id::prime(
            &BigUint::from(f.p),
            &BigUint::from(i.curve.c[1]),
            &BigUint::from(i.curve.c[0]),
            &order,
        )
    } else {
        curve_id::extension(
            f.p,
            &coefficients(f.modulus, f.p, f.n),
            &coefficients(i.curve.c[1], f.p, f.n),
            &coefficients(i.curve.c[0], f.p, f.n),
            &order,
        )
    }
    .expect("validated study curve identity");
    static REGISTERED: std::sync::OnceLock<std::collections::BTreeSet<String>> =
        std::sync::OnceLock::new();
    let registered = REGISTERED
        .get_or_init(|| {
            let registry: Value =
                serde_json::from_str(include_str!("../../../docs/curves/registry.json"))
                    .expect("curve registry");
            registry["curves"]
                .as_array()
                .expect("registry rows")
                .iter()
                .map(|row| row["slug"].as_str().expect("curve slug").to_owned())
                .collect()
        })
        .contains(&id.slug);
    CurveFacts {
        slug: id.slug,
        icv1: id.icv1,
        family: if f.p == 2 {
            "binary"
        } else if f.n == 1 {
            "prime"
        } else {
            "extension"
        }
        .into(),
        construction: spec.call(),
        field_degree: Some(f.n as u32),
        field_bits: Some(64 - f.p.leading_zeros()),
        group_order: i.order.into(),
        r: i.r.into(),
        cofactor: i.h.into(),
        generator: [
            format!("0x{:x}", i.generator.x),
            format!("0x{:x}", i.generator.y),
        ],
        automorphisms_available: 2,
        registered,
        ec1: None,
        curve_uid: None,
    }
}
fn arithmetic(c: Counts) -> f64 {
    c.m as f64 + 0.75 * c.s as f64 + 10.0 * c.i as f64
}
fn native_counts(c: Counts, out: &mut BTreeMap<String, u64>) {
    out.insert("field_additions_uncharged".into(), c.field_adds);
    out.insert("linear_bit_operations_uncharged".into(), c.bit_ops);
    out.insert("field_multiplications".into(), c.m);
    out.insert("field_squarings".into(), c.s);
    out.insert("field_inversions".into(), c.i);
    out.insert("source_ec_additions".into(), c.ec_adds);
    out.insert("jacobian_additions".into(), c.jac_adds);
    out.insert("square_root_calls".into(), c.sqrt_calls);
}
pub fn solve(
    m: &ResolvedMethod,
    inst: &StudyInstance,
    target: Point,
    seed: u64,
) -> Result<SolveReport, String> {
    let cap = m
        .params
        .get("trial_cap")
        .ok_or("study method needs trial_cap")?
        .parse::<u64>()
        .map_err(|_| "invalid trial_cap")?;
    let start = Instant::now();
    if (m.id.contains("genus2") || m.id == "rho.three_leads_jacobian_v1")
        && inst.family != "scholten"
    {
        return Err("genus-2 methods require the certified Scholten family".into());
    }
    if matches!(
        m.id.as_str(),
        "ic.three_leads_genus2_v1" | "ic.three_leads_genus2_v2"
    ) {
        for name in ["independent", "unique_relations"] {
            if !matches!(m.params[name].as_str(), "0" | "1") {
                return Err(format!("{name} must be 0 or 1"));
            }
        }
    }
    if matches!(
        m.id.as_str(),
        "ic.three_leads_decomposition_v1" | "ic.three_leads_decomposition_v2"
    ) {
        if !matches!(
            inst.family.as_str(),
            "subfield" | "pkm_kummer" | "pkm_dickson"
        ) {
            return Err("decomposition method requires a subfield or tower fixture".into());
        }
        let oracle = m.params["oracle"].as_str();
        if !matches!(
            oracle,
            "linearized" | "linearized_filtered" | "semaev" | "subtract"
        ) {
            return Err("unknown study oracle".into());
        }
        if !matches!(m.params["random_base"].as_str(), "0" | "1") {
            return Err("random_base must be 0 or 1".into());
        }
        if oracle.starts_with("linearized")
            && (!inst.curve.binary || inst.family != "subfield" || m.params["random_base"] != "0")
        {
            return Err("linearized solver requires the structured binary subfield base".into());
        }
        if oracle == "semaev" && inst.curve.binary {
            return Err("the semaev arm uses the odd-characteristic quadratic; choose linearized or subtract for binary".into());
        }
    }
    let mut out: Outcome = match m.id.as_str() {
        "ic.three_leads_decomposition_v2" => engine::solve_subfield_or_pkm_v2(
            inst,
            target,
            seed,
            cap,
            &m.params["oracle"],
            m.params["random_base"] == "1",
            m.params["base_seed"]
                .parse()
                .map_err(|_| "invalid base_seed")?,
        ),
        "ic.three_leads_genus2_v2" => engine::solve_genus2_v2(
            inst,
            target,
            seed,
            cap,
            m.params["independent"] == "1",
            m.params["unique_relations"] == "1",
        ),
        "rho.three_leads_plain_v1" => engine::solve_rho(inst, target, seed, cap, false, false),
        "rho.three_leads_negation_v1" => engine::solve_rho(inst, target, seed, cap, true, false),
        "rho.three_leads_jacobian_v1" => engine::solve_rho(inst, target, seed, cap, true, true),
        "ic.three_leads_genus2_v1" => engine::solve_genus2(
            inst,
            target,
            seed,
            cap,
            m.params["independent"] == "1",
            m.params["unique_relations"] == "1",
        ),
        "ic.three_leads_decomposition_v1" => engine::solve_subfield_or_pkm(
            inst,
            target,
            seed,
            cap,
            &m.params["oracle"],
            m.params["random_base"] == "1",
            m.params["base_seed"]
                .parse()
                .map_err(|_| "invalid base_seed")?,
        ),
        _ => {
            return Err(format!(
                "{} is not implemented on a three-leads study fixture",
                m.id
            ))
        }
    };
    // Independent post-solver verification is part of the charged cold solve.
    // The ecbench parent then verifies the returned scalar a second time.
    if let Some(k) = out.recovered {
        let verifier = inst.fresh();
        let got = verifier.curve.mul(verifier.generator, k);
        if got != target {
            return Err("fresh source verification rejected recovered scalar".into());
        }
        let c = verifier.curve.f.counts();
        out.counts = out.counts + c;
        out.phases.push(NativePhase {
            name: "source_verification".into(),
            counts: c,
        });
    }
    // This is an operation-count conversion, not a host-time calibration.
    // The complete twelve-cell band is derived from the retained primitives.
    let instrument = inst.fresh();
    let twice = instrument.curve.mul(instrument.generator, 2);
    instrument.curve.f.reset();
    let _ = instrument.curve.add(instrument.generator, twice);
    let price = instrument.curve.f.counts();
    let unit = arithmetic(price);
    if unit <= 0.0 {
        return Err("source addition price is zero".into());
    }
    native_counts(out.counts, &mut out.native);
    out.native
        .insert("source_add_multiplications".into(), price.m);
    out.native.insert("source_add_squarings".into(), price.s);
    out.native.insert("source_add_inversions".into(), price.i);
    let phases = out
        .phases
        .iter()
        .map(|p| PhaseRecord {
            name: p.name.clone(),
            adds: p.counts.ec_adds + p.counts.jac_adds,
            doubles: 0,
            scalar_mults: 0,
            gae: arithmetic(p.counts) / unit,
            wall_ns: None,
            native: BTreeMap::from([
                ("field_multiplications".into(), p.counts.m),
                ("field_squarings".into(), p.counts.s),
                ("field_inversions".into(), p.counts.i),
            ]),
        })
        .collect();
    let unpriced = out
        .native
        .iter()
        .filter(|(k, v)| k.ends_with("_uncharged") && **v > 0)
        .map(|(k, _)| k.clone())
        .collect();
    let detail = json!({"engine":out.detail,"study_family":inst.family,"arithmetic_price":{"square_per_multiply":0.75,"inversion_per_multiply":10,"source_addition":price,"meaning":"declared operation-count conversion; not a wall-time price"},"native_phases":out.phases,"verification":"solver has no planted scalar; fresh source kP checked here and again by ecbench parent"});
    Ok(SolveReport {
        recovered: out.recovered.map(u128::from),
        exhausted: out.exhausted,
        phases,
        total_gae: arithmetic(out.counts) / unit,
        automorphisms_used: if m.id == "rho.three_leads_plain_v1" {
            1
        } else {
            2
        },
        counters: out.native,
        unpriced,
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: start.elapsed().as_nanos() as u64,
        factor_base: None,
        detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops: Some(FieldOps {
            muls: out.counts.m,
            sqrs: out.counts.s,
            invs: out.counts.i,
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jacobian_matches_existing_independent_backend() {
        use crate::cryptanalysis::genus2_fp::Genus2Curve;
        for p in [19, 31, 43, 59, 83] {
            let s = engine::Scholten::new(p);
            let mut coeff = [0; 6];
            coeff.copy_from_slice(&s.j.polynomial);
            let reference = Genus2Curve::new(p, coeff).unwrap();
            let mut points = vec![];
            for x in 0..p {
                if let Some(y) = s.j.f.sqrt(
                    s.j.polynomial
                        .iter()
                        .rev()
                        .fold(0, |r, &a| s.j.f.add(s.j.f.mul(r, x), a)),
                ) {
                    points.push((s.j.point(x, y), reference.point(x, y).unwrap()));
                }
            }
            for i in 0..100 {
                let a = points[i % points.len()];
                let b = points[(i * 7 + 3) % points.len()];
                let ours = s.j.add(a.0, b.0);
                let theirs = reference.add(&a.1, &b.1);
                assert_eq!(ours.u[..=ours.degree].to_vec(), theirs.u_coeffs());
                let mut v = ours.v[..ours.degree].to_vec();
                while v.last() == Some(&0) {
                    v.pop();
                }
                let mut other = theirs.v_coeffs();
                while other.last() == Some(&0) {
                    other.pop();
                }
                assert_eq!(v, other);
            }
        }
    }
}
