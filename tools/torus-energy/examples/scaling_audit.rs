//! Exact arithmetic for a CONDITIONAL cost model; no samples, solver or timing.
use num_bigint::BigUint;
use num_rational::BigRational;
use num_traits::{One, Zero};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, path::PathBuf};
type Q = BigRational;
fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}
fn envelope(m: u32, gamma: &Q, beta: &Q) -> Q {
    let query_count = (q(1, 1) - q(m.into(), 1) * beta).max(Q::zero());
    (q(2, 1) * beta).max((q(1, 1) + gamma) * beta + query_count)
}
fn optimum(m: u32, gamma: &Q) -> Result<(Q, Q), String> {
    if m < 2 || gamma < &Q::zero() {
        return Err("invalid conditional model".into());
    }
    let kink = q(1, m.into());
    let mut candidates = vec![Q::zero(), kink.clone()];
    let denominator = q(i64::from(m) + 1, 1) - gamma;
    if denominator > Q::zero() {
        let balance = Q::one() / denominator;
        if balance <= kink {
            candidates.push(balance);
        }
    }
    candidates
        .into_iter()
        .map(|b| {
            let e = envelope(m, gamma, &b);
            (b, e)
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .ok_or("no candidates".into())
}
fn record() -> Result<serde_json::Value, String> {
    let n = (BigUint::one() << 252usize)
        + BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();
    let m = BigUint::from(60824497u64) * 213156431u64;
    let lower = (&m - 1u32) / 2u32;
    let root = n.nth_root(6);
    if root.pow(6) > n || (&root + 1u32).pow(6) <= n || lower.pow(6) <= n {
        return Err("integer root/base arithmetic failed".into());
    }
    let proposals = [60824497u64, 213156431u64].map(|small_m| {
        let k = (&root + small_m - 1u32) / small_m;
        let raw = &k * small_m;
        json!({"subgroup_order":small_m,"proposed_cosets":k.to_string(),
            "proposed_raw_cardinality":raw.to_string(),
            "exact_coset_order_and_admissibility_checked":false,
            "recognition_cost_and_normalized_base_size_checked":false})
    });
    let mut rows = vec![];
    for (arity, gamma) in [(4, q(1, 2)), (5, q(3, 2)), (6, q(3, 2)), (8, q(5, 2))] {
        let (beta, e) = optimum(arity, &gamma)?;
        rows.push(
            json!({"summands":arity,"hypothetical_gamma":gamma.to_string(),
            "beta":beta.to_string(),"modeled_time_exponent":e.to_string(),
            "decomposition_algorithm_available":false,
            "actual_growing_parameter_family_available":false}),
        );
    }
    Ok(json!({"schema":"torus-energy.scaling-audit/v1",
        "purpose":"exact conditional model and current integer parameters only",
        "model":"max(2 beta,(1+gamma) beta+max(0,1-m beta))",
        "hypotheses":["yield is Theta(min(1,B^m/N))", "D is soft-O(B^gamma)",
            "setup and sparse linear algebra are soft-O(B^2)",
            "all remaining phases and constants are charged", "admissible growing family exists"],
        "scalar_order":n.to_string(),"current_torus_order":m.to_string(),
        "current_class_lower_bound_assuming_existing_transport":lower.to_string(),
        "floor_sixth_root_scalar_order":root.to_string(),
        "current_lower_bound_sixth_power_gt_order":lower.pow(6)>n,
        "current_lower_bound_fifth_power_gt_order":lower.pow(5)>n,
        "conditional_examples":rows,"many_coset_parameter_proposals":proposals,
        "base_sampling_invoked":false,"solver_invoked":false,
        "actual_complexity_bound_established":false,"pcc_standing":"unverified"}))
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(env::args().nth(1).ok_or("expected NEW output directory")?);
    fs::create_dir(&out)?;
    let pre = serde_json::to_vec_pretty(&json!({"schema":"scaling-audit.pre-call/v1",
        "binary_sha256":format!("{:x}",Sha256::digest(fs::read(env::current_exe()?)?)),
        "purpose":"exact arithmetic; no benchmark, sampling or solver",
        "scalar_order_source":"RFC7748; not a new point-count certificate"}))?;
    fs::write(out.join("pre-call.json"), &pre)?;
    let mut result = record()?;
    result["pre_call_sha256"] = json!(format!("{:x}", Sha256::digest(&pre)));
    fs::write(
        out.join("scaling-audit.json"),
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
    fn saturation_changes_the_six_summand_balance() {
        assert_eq!(optimum(4, &q(1, 2)).unwrap(), (q(2, 9), q(4, 9)));
        assert_eq!(optimum(5, &q(3, 2)).unwrap(), (q(1, 5), q(1, 2)));
        assert_eq!(optimum(6, &q(3, 2)).unwrap(), (q(1, 6), q(5, 12)));
        assert_eq!(envelope(6, &q(3, 2), &q(2, 11)), q(5, 11));
        assert!(q(5, 11) > q(4, 11));
        assert!(optimum(1, &q(0, 1)).is_err());
        assert!(optimum(4, &q(-1, 1)).is_err());
    }
    #[test]
    fn exact_parameter_gap_and_independent_rational_grid() {
        let r = record().unwrap();
        assert_eq!(r["floor_sixth_root_scalar_order"], "4398046511104");
        assert_eq!(r["current_lower_bound_fifth_power_gt_order"], true);
        for m in 3..=8 {
            for g in 0..=2 * (m - 1) {
                let gamma = q(g.into(), 2);
                let (_, e) = optimum(m, &gamma).unwrap();
                for b in 0..=840 {
                    assert!(envelope(m, &gamma, &q(b, 840)) >= e);
                }
            }
        }
    }
}
