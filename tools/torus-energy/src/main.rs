use serde_json::json;
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};
use torus_energy::{actual_w0, calibration, Plan, BLOCKS, COLLISION_THRESHOLD, ERROR_BITS};
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 || args[1] != "calibrate" {
        return Err("usage: torus-energy calibrate NEW_DIRECTORY".into());
    }
    let dir = Path::new(&args[2]);
    fs::create_dir(dir)?;
    let pre = serde_json::to_vec_pretty(&json!({
        "schema":"torus-energy.pre-call/v1",
        "purpose":"exact arithmetic calibration, not a base-sampling or solver experiment",
        "binary_sha256":format!("{:x}",Sha256::digest(fs::read(env::current_exe()?)?)),
        "blocks":BLOCKS,"threshold":COLLISION_THRESHOLD,"error_bits":ERROR_BITS,
        "good_block_denominator":32,"bad_block_denominator":17
    }))?;
    fs::write(dir.join("pre-call.json"), &pre)?;
    let (good, bad) = calibration()?;
    let plan = Plan::new(actual_w0())?;
    let record = json!({"schema":"torus-energy.calibration/v1","purpose":"exact integer calibration and planning only; no base samples or solver run","blocks":BLOCKS,"collision_block_threshold":COLLISION_THRESHOLD,"per_base_error_bits":ERROR_BITS,"good_block_probability_upper":"1/32","bad_block_probability_lower":"1/17","good_false_reject":{"numerator":good.numerator.to_string(),"denominator":good.denominator.to_string(),"at_most_two_to_minus_error_bits":good.at_most_power_of_two(ERROR_BITS)},"bad_false_accept":{"numerator":bad.numerator.to_string(),"denominator":bad.denominator.to_string(),"at_most_two_to_minus_error_bits":bad.at_most_power_of_two(ERROR_BITS)},"W0":plan.w0.to_string(),"samples_per_block":plan.block_samples.to_string(),"sample_ceiling":plan.sample_ceiling.to_string(),"completeness_1600_n_squared":plan.completeness_lhs.to_string(),"soundness_2304_n_minus_one_squared":plan.soundness_rhs.to_string(),"completeness_energy_factor":100,"soundness_energy_factor":576,"sample_collection_invoked":false,"solver_invoked":false,"formal_probability_proof_available":false,"pcc_standing":"unverified"});
    let mut record = record;
    record["pre_call_sha256"] = json!(format!("{:x}", Sha256::digest(&pre)));
    fs::write(
        dir.join("calibration.json"),
        serde_json::to_vec_pretty(&record)?,
    )?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
