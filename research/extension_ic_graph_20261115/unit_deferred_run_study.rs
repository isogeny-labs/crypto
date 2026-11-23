//! Recorded Run51 observations. Original qualification137 is never reclassified.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: STUDY COLLECTED_STAGE NEW_JSON".into());
    }
    let stage = Path::new(&args[0]);
    let run = stage.join("returned/task/run");
    let read = |p: &Path| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) };
    let deletion = read(&stage.join("deletion.json"))?;
    if deletion["delete_http"] != 204 || deletion["independent_get_http"] != 404 {
        return Err("owned cleanup".into());
    }
    let original = fs::read_to_string(run.join("analysis-original.exit-code.txt"))?;
    if original.trim() != "137" {
        return Err("this observation producer requires preserved original137".into());
    }
    let mut rows = vec![];
    let mut expected = None;
    for role in [
        "original-ic",
        "quadratic-table",
        "pair-discriminant",
        "pair-deferred",
        "pair-adapted-six",
        "pair-unit-five",
        "rho-plain-b1",
        "rho-neg-b1",
        "rho-plain-b16",
        "rho-neg-b16",
    ] {
        if fs::read_to_string(run.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err("worker exit".into());
        }
        let bytes = fs::read(run.join(format!("{role}.raw.json")))?;
        let w: Value = serde_json::from_slice(&bytes)?;
        let answer = w["actual_scalar"].as_u64().ok_or("scalar")?;
        if w["verified"] != true || w["capped"] != false || expected.is_some_and(|x| x != answer) {
            return Err("worker-reported scalar/flags".into());
        }
        expected = Some(answer);
        let mut total = 0u128;
        let mut phases = vec![];
        for p in w["work_native"]["phases"].as_array().ok_or("phases")? {
            let cost = p["diagnostic_priced_bitops"]
                .as_str()
                .ok_or("phase subtotal")?
                .parse::<u128>()?;
            total = total.checked_add(cost).ok_or("subtotal overflow")?;
            phases.push(json!({"phase":p["phase"],"observed_incomplete_bitops":cost.to_string()}));
        }
        let times = fs::read_to_string(run.join(format!("{role}.time.txt")))?;
        let real = times
            .lines()
            .find_map(|s| s.strip_prefix("real_s="))
            .ok_or("process wall")?
            .parse::<f64>()?;
        let mut attempts = 0;
        let mut roots = 0;
        let mut empty = 0;
        let mut incomplete = 0;
        let mut timeouts = 0;
        let mut nonseparating = 0;
        if let Some(a) = w["work"]["nagao_attempts"].as_array() {
            for t in a {
                attempts += 1;
                let n = t["audit"]["lifted_roots"].as_array().map_or(0, Vec::len);
                roots += n;
                empty += usize::from(n == 0);
                incomplete += usize::from(t["audit"]["stats"]["incomplete"] == true);
                timeouts += usize::from(t["audit"]["stats"]["timed_out"] == true);
                nonseparating += t["audit"]["stats"]["nonseparating"].as_u64().unwrap_or(0);
            }
        }
        let inner = w["inner_wall_ns"]
            .as_str()
            .map(str::parse::<u128>)
            .transpose()?
            .or_else(|| w["inner_wall_ns"].as_u64().map(u128::from))
            .ok_or("inner wall")?;
        println!("{role}: observed={total}; inner_s={:.6}; process_s={real:.3}; residuals={}; attempts={attempts}",inner as f64/1e9,w["work"]["descent_residuals"]);
        rows.push(json!({"mode":role,"raw_sha256":hash(&bytes),"raw_bytes":bytes.len(),"source_revision":w["source_revision"],"worker_verified":true,"worker_scalar_consistent":true,
            "observed_incomplete_bitops":total.to_string(),"phases":phases,"inner_wall_ns_practicality_only":inner.to_string(),"process_wall_seconds_practicality_only":real,
            "descent_residuals":w["work"]["descent_residuals"],"accepted_descents":w["work"]["descents"].as_array().map(Vec::len),
            "algebraic_audit":{"attempts":attempts,"roots":roots,"empty":empty,"incomplete":incomplete,"timed_out":timeouts,"nonseparating":nonseparating}}));
    }
    let replay_path = stage.join("post-execution-qualification/exit-code.txt");
    let replay = fs::read_to_string(replay_path).ok();
    let replay_summary = stage.join("post-execution-qualification/replay-original-binary.json");
    let replay_bytes = fs::read(replay_summary).ok();
    let v = json!({"schema":"cover.unit-deferred.recorded-observation/v1","original_qualification_exit":137,"original_run_qualified":false,
        "post_execution_replay_exit":replay.as_deref().map(str::trim),"post_execution_replay_sha256":replay_bytes.as_deref().map(hash),"original_status_unchanged":true,
        "deletion":deletion,"outcomes":rows,"complete_cost":false,"complete_total":null,"cost_ratios":null,"rho_parity":false,"comparative_eligibility":false,
        "scope":"worker observations; separate mathematical replay does not change original frozen qualifier137; one exposed fixed target, incomplete counters and nonadmitted elapsed observations"});
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[1])?;
    serde_json::to_writer_pretty(&mut out, &v)?;
    out.write_all(b"\n")?;
    Ok(())
}
