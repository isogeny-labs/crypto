//! Fixed development controls for a many-coset membership predicate.
//! No decomposition solver, base enumeration, sampling or performance run.
use num_bigint::BigUint;
use num_traits::One;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, path::PathBuf};
use torus_s4::Field;
use torus_s4_tree::{Fp2, TorusParameters};

const M: u64 = 60_824_497;
const Q: u64 = 213_156_431;
const K: u64 = 20_633;
type Table = BTreeMap<(BigUint, BigUint), u64>;
type Result<T> = std::result::Result<T, String>;

fn table(h: &Fp2, f: &Field) -> Result<Table> {
    let omega = h.pow(&Q.into(), f);
    let mut cur = omega.clone();
    let mut out = Table::new();
    for j in 1..=K {
        if out.insert((cur.a.clone(), cur.b.clone()), j).is_some() {
            return Err("duplicate membership-table entry".into());
        }
        if j != K {
            cur = cur.mul(&omega, f);
        }
    }
    Ok(out)
}
fn classify(z: &Fp2, f: &Field, labels: &Table) -> Option<u64> {
    let power = z.pow(&Q.into(), f);
    labels.get(&(power.a, power.b)).copied()
}
fn control(
    name: String,
    z: &Fp2,
    expected: Option<u64>,
    f: &Field,
    labels: &Table,
) -> Result<Value> {
    let actual = classify(z, f, labels);
    if actual != expected {
        return Err(format!("{name}: expected {expected:?}, got {actual:?}"));
    }
    Ok(json!({"case":name,"z":[z.a.to_string(),z.b.to_string()],
        "expected_coset":expected,"returned_coset":actual}))
}
fn record() -> Result<Value> {
    let old = TorusParameters::actual_four()?;
    let expected_order = u128::from(M) * u128::from(Q);
    if old.m != expected_order || 2 * K >= M {
        return Err("fixed order/residue bounds failed".into());
    }
    let labels = table(&old.h, &old.f)?;
    let mut cases = vec![];
    for j in [1, K / 2, K] {
        for t in [0, 1, Q - 1] {
            let e = j + M * t;
            let z = old.h.pow(&e.into(), &old.f);
            if z.norm(&old.f) != BigUint::one() {
                return Err("encoded value is outside norm-one torus".into());
            }
            cases.push(control(
                format!("inside-j{j}-t{t}"),
                &z,
                Some(j),
                &old.f,
                &labels,
            )?);
            for (kind, outside) in [
                ("negative", z.neg(&old.f)),
                ("inverse", z.inverse(&old.f)?),
                ("negative-inverse", z.inverse(&old.f)?.neg(&old.f)),
            ] {
                cases.push(control(
                    format!("{kind}-j{j}-t{t}"),
                    &outside,
                    None,
                    &old.f,
                    &labels,
                )?);
            }
        }
    }
    for e in [0, K + 1, M - 1, M] {
        cases.push(control(
            format!("outside-exponent-{e}"),
            &old.h.pow(&e.into(), &old.f),
            None,
            &old.f,
            &labels,
        )?);
    }
    for (name, z) in [
        (
            "zero",
            Fp2 {
                a: 0u32.into(),
                b: 0u32.into(),
            },
        ),
        ("minus-one", Fp2::one().neg(&old.f)),
        ("old-generator-outside-parent", old.g.clone()),
    ] {
        cases.push(control(name.into(), &z, None, &old.f, &labels)?);
    }
    let scalar = (BigUint::one() << 252usize)
        + BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();
    let raw = BigUint::from(K) * Q;
    Ok(json!({"schema":"torus-energy.many-coset-audit/v1",
        "scope":"fixed construction and membership development controls only",
        "cofactor_m":M,"subgroup_order_q":Q,"cosets_k":K,
        "parent_order":old.m.to_string(),"raw_cardinality":raw.to_string(),
        "h":[old.h.a.to_string(),old.h.b.to_string()],
        "p":old.f.p.to_string(),"old_exact_order_control_replayed":true,
        "floor_sixth_root_scalar_order":scalar.nth_root(6).to_string(),
        "raw_sixth_power_exceeds_scalar_order":raw.pow(6)>scalar,
        "table_entries":labels.len(),"query_q_power_squarings":Q.ilog2(),
        "query_q_power_multiplications":Q.count_ones(),
        "setup_q_powers":1,"setup_extra_fp2_multiplications":K-1,
        "lookup":"BTreeMap on both canonical field coordinates; O(log k) comparisons",
        "control_count":cases.len(),"controls":cases,
        "full_raw_base_enumerated":false,"decomposition_solver_invoked":false,
        "elliptic_point_normalization_certified":false,
        "subgroup_index_t_recovery_implemented":false,
        "growing_parameter_family_certified":false,
        "performance_or_complexity_result":false,"pcc_standing":"unverified"}))
}
fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let pre = serde_json::to_vec_pretty(&json!({"schema":"many-coset.pre-call/v1",
        "purpose":"fixed membership correctness; no sampling, timing or solver",
        "m":M,"q":Q,"k":K,"expected_controls":43,
        "example_source_sha256":format!("{:x}",Sha256::digest(include_bytes!("many_coset_audit.rs"))),
        "binary_sha256":format!("{:x}",Sha256::digest(fs::read(env::current_exe()?)?))}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut result = record()?;
    result["pre_call_sha256"] = json!(format!("{:x}", Sha256::digest(&pre)));
    fs::write(
        out.join("many-coset.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_field_membership_and_orbit_exclusions() {
        let r = record().unwrap();
        assert_eq!(r["raw_cardinality"], "4398056640823");
        assert_eq!(r["control_count"], 43);
        assert_eq!(r["table_entries"], 20633);
        assert_eq!(r["raw_sixth_power_exceeds_scalar_order"], true);
        let positives = r["controls"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| !c["returned_coset"].is_null())
            .count();
        assert_eq!(positives, 9);
    }
}
