//! Local scalar-multiplication observations on the original counted group.
//! This is neither an ECDLP method nor an isolation/admission certificate.
use crypto_lib::cryptanalysis::ecbench::{
    canonical, host, isolation, runner, spec, stats, workload,
};
use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::hint::black_box;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub fn events(plan: &spec::Plan) -> Result<Vec<spec::Execution>, String> {
    if plan.spec.measurement.order != spec::Order::PairedCurvesV1
        || !plan
            .instances
            .iter()
            .all(|i| matches!(i, workload::Instance::PrimeMont4(_)))
    {
        return Err("scalar timing requires paired_curves_v1 prime_mont4 inputs".into());
    }
    let mut seen = BTreeSet::new();
    Ok(plan
        .executions
        .iter()
        .filter(|e| seen.insert((e.workload, e.round, e.warmup)))
        .cloned()
        .collect())
}

pub fn validate_batch(batch: u64) -> Result<(), String> {
    if !(1..=65536).contains(&batch) {
        return Err("scalar batch must be in1..65536".into());
    }
    Ok(())
}

fn write_json(path: &Path, value: &impl serde::Serialize) -> Result<(), String> {
    std::fs::write(
        path,
        serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

/// Same scalar and base-target index on all models of one subgroup order.
/// Explicit modulo reduction is reproducible, not claimed uniformly sampled.
pub fn scalar(seed: u64, r: u64) -> u64 {
    1 + seed % (r - 1)
}

pub fn run(spec_path: &Path, out: &Path, batch: u64, lock: &str) -> Result<(), String> {
    validate_batch(batch)?;
    let text = std::fs::read_to_string(spec_path).map_err(|e| e.to_string())?;
    canonical::reject_duplicate_keys(&text)?;
    let plan = spec::plan(spec::Spec::from_json(&text)?)?;
    let schedule = events(&plan)?;
    let _lock = isolation::BenchLock::acquire(lock, false)?;
    std::fs::create_dir(out).map_err(|e| format!("fresh scalar output: {e}"))?;
    std::fs::write(out.join("spec.json"), &text).map_err(|e| e.to_string())?;
    let doc = runner::PlanDoc {
        schema: "ecbench.plan/v1".into(),
        spec_id: plan.spec_id.clone(),
        arms: plan.arms.clone(),
        workloads: plan.workloads.clone(),
        executions: plan.executions.clone(),
    };
    write_json(&out.join("plan.json"), &doc)?;
    write_json(&out.join("host.json"), &host::capture()?)?;
    let executable = std::env::current_exe().map_err(|e| e.to_string())?;
    let (binary, _) = crypto_lib::store::hash_file(&executable).map_err(|e| e.to_string())?;
    let mut manifest = json!({"schema":"ecbench.scalar_timing/v1","status":"running","scope":"local descriptive scalar timing; not ECDLP solve or wall admission","spec_id":plan.spec_id,"spec_sha256":plan.spec_sha256,"binary_sha256":binary,"batch":batch,"planned_samples":schedule.len(),"base_point":"each workload target Q","scalar_law":"1+paired_algorithm_seed%(r-1); not asserted uniform","timed_interval":"batch of CountedGroup::mul calls, loop/black_box and ledger updates","excluded":"curve/target construction, source/input loading, independent expected-result construction, verification and output","isolation":"L0; not an isolation grader","wall_admitted":false,"conductor_gap":null,"ECDLP_solved":false});
    write_json(&out.join("manifest.json"), &manifest)?;
    let mut rows = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("samples.jsonl"))
        .map_err(|e| e.to_string())?;
    let mut grouped = BTreeMap::<String, (Option<u32>, Vec<f64>)>::new();
    for (seq, e) in schedule.iter().enumerate() {
        let w = &plan.workloads[e.workload];
        let workload::Instance::PrimeMont4(curve) = plan.instance(e.workload) else {
            unreachable!()
        };
        let base = curve.decode(&w.target)?;
        let k = scalar(e.algorithm_seed, curve.r);
        let planted = w
            .planted
            .and_then(|x| x.checked_u64())
            .ok_or("known scalar required for scalar timing verification")?;
        let combined = ((u128::from(planted) * u128::from(k)) % u128::from(curve.r)) as u64;
        let mut verification = GroupOps::default();
        let expected = curve.mul(&mut verification, curve.generator, combined);
        let mut ops = GroupOps::default();
        let mut result = curve.identity();
        let started = Instant::now();
        for _ in 0..batch {
            result = black_box(curve.mul(&mut ops, black_box(base), black_box(k)));
        }
        let elapsed_ns = started.elapsed().as_nanos();
        if result != expected || !curve.on_curve(result) {
            return Err(format!(
                "scalar output mismatch at workload{} round{}",
                w.workload_id, e.round
            ));
        }
        let row = json!({"schema":"ecbench.scalar_sample/v1","seq":seq,"plan_seq":e.seq,"workload_id":w.workload_id,"curve_slug":w.curve.slug,"round":e.round,"warmup":e.warmup,"algorithm_seed":e.algorithm_seed.to_string(),"scalar":k.to_string(),"iterations":batch,"elapsed_ns":elapsed_ns,"result":curve.encode(result),"verified":true,"group_ops":ops,"field_ops":ops.field_ops,"verification_group_ops":verification,"verification_field_ops":verification.field_ops});
        serde_json::to_writer(&mut rows, &row).map_err(|e| e.to_string())?;
        rows.write_all(b"\n")
            .and_then(|_| rows.flush())
            .map_err(|e| e.to_string())?;
        if !e.warmup {
            grouped
                .entry(w.curve.slug.clone())
                .or_insert_with(|| (w.curve.field_bits, Vec::new()))
                .1
                .push(elapsed_ns as f64 / batch as f64);
        }
    }
    let descriptions: Vec<Value> = grouped.into_iter().map(|(curve_slug,(field_bits,ns))|json!({"curve_slug":curve_slug,"field_bits":field_bits,"samples":ns.len(),"mean_ns_per_mul":stats::mean(&ns),"median_ns_per_mul":stats::median(&ns),"wall_admitted":false})).collect();
    let (digest, _) =
        crypto_lib::store::hash_file(&out.join("samples.jsonl")).map_err(|e| e.to_string())?;
    manifest["status"] = json!("complete");
    manifest["sample_file_sha256"] = json!(digest);
    manifest["descriptive_by_curve"] = json!(descriptions);
    write_json(&out.join("manifest.json"), &manifest)?;
    println!(
        "{}",
        serde_json::to_string(&manifest).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plan() -> spec::Plan {
        let s = include_str!("../../../research/prime_conductor_gap80_20261031/paired-schedule-20261118/paired-cli-spec.json");
        spec::plan(spec::Spec::from_json(s).unwrap()).unwrap()
    }
    #[test]
    fn schedule_deduplicates_arms_preserves_all_workload_rounds_and_pairs() {
        let p = plan();
        let es = events(&p).unwrap();
        assert_eq!(es.len(), 84);
        assert_eq!(es.iter().filter(|e| e.warmup).count(), 12);
        let mut paired = BTreeMap::new();
        for e in es {
            let w = &p.workloads[e.workload];
            let r = w.curve.r.checked_u64().unwrap();
            let k = scalar(e.algorithm_seed, r);
            let old = paired.insert((r, w.target_index, e.round, e.warmup), k);
            if let Some(old) = old {
                assert_eq!(old, k);
            }
            assert!((1..r).contains(&k));
        }
    }
    #[test]
    fn actual_counted_mul_identity_on_varied_target_bases() {
        let p = plan();
        for w in 0..p.workloads.len() {
            let workload::Instance::PrimeMont4(c) = p.instance(w) else {
                panic!()
            };
            let b = c.decode(&p.workloads[w].target).unwrap();
            let t = p.workloads[w].planted.unwrap().checked_u64().unwrap();
            for seed in [0, 1, u64::MAX] {
                let k = scalar(seed, c.r);
                let expected = ((u128::from(t) * u128::from(k)) % u128::from(c.r)) as u64;
                let mut a = GroupOps::default();
                let got = c.mul(&mut a, b, k);
                let want = c.mul(&mut GroupOps::default(), c.generator, expected);
                assert_eq!(got, want);
                assert_eq!(a.scalar_mults, 1);
            }
        }
    }
    #[test]
    fn bounds_and_incompatible_schedule_reject() {
        assert!(validate_batch(0).is_err());
        assert!(validate_batch(65537).is_err());
        assert!(validate_batch(64).is_ok());
        let mut p = plan();
        p.spec.measurement.order = spec::Order::Alternate;
        assert!(events(&p).is_err());
    }
}
