//! Bounded, sequential paired native campaign. Raw samples are never overwritten.
use relation_degree_trace::workload::Fixture;
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
fn next(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut x = *state;
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn median(values: &[f64]) -> f64 {
    let mut x = values.to_vec();
    x.sort_by(f64::total_cmp);
    if x.len() % 2 == 1 {
        x[x.len() / 2]
    } else {
        (x[x.len() / 2 - 1] + x[x.len() / 2]) / 2.
    }
}
fn interval(values: &[f64]) -> Value {
    if values.is_empty() {
        return Value::Null;
    }
    let mut state = 20261115u64;
    let mut draws = Vec::new();
    for _ in 0..10000 {
        let sample: Vec<f64> = (0..values.len())
            .map(|_| values[(next(&mut state) as usize) % values.len()])
            .collect();
        draws.push(median(&sample));
    }
    draws.sort_by(f64::total_cmp);
    json!({"count":values.len(),"median":median(values),"bootstrap_95":[draws[249],draws[9749]],"statistic":"median of paired ratios; paired resampling"})
}
fn allowed_cpu() -> Result<String, Box<dyn Error>> {
    let status = fs::read_to_string("/proc/self/status")?;
    let allowed = status
        .lines()
        .find_map(|l| l.strip_prefix("Cpus_allowed_list:"))
        .ok_or("no CPU affinity metadata")?
        .trim();
    let cpu = allowed.split([',', '-']).next().ok_or("empty affinity")?;
    let _: usize = cpu.parse()?;
    Ok(cpu.into())
}
fn worker(
    binary: &Path,
    fixture: &Path,
    arm: &str,
    prefix: &Path,
    cpu: &str,
    repetitions: usize,
    remaining: Duration,
) -> Result<Value, Box<dyn Error>> {
    if remaining.is_zero() {
        return Ok(json!({"status":"campaign_deadline"}));
    }
    let limit = remaining.min(Duration::from_secs(30));
    let out = prefix.with_extension("stdout.json");
    let err = prefix.with_extension("stderr.log");
    let started = Instant::now();
    let mut child = Command::new("prlimit")
        .args(["--as=2147483648", "--", "taskset"])
        .args(["-c", cpu])
        .arg(binary)
        .args([
            "--worker",
            fixture.to_str().ok_or("non-UTF8 path")?,
            "--arm",
            arm,
            "--repetitions",
            &repetitions.to_string(),
        ])
        .env("OMP_NUM_THREADS", "1")
        .stdout(Stdio::from(File::create(&out)?))
        .stderr(Stdio::from(File::create(&err)?))
        .spawn()?;
    let exit = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if started.elapsed() > limit {
            child.kill()?;
            child.wait()?;
            break None;
        }
        thread::sleep(Duration::from_millis(10));
    };
    let outer_ns = started.elapsed().as_nanos();
    let mut value = if let Some(exit) = exit {
        if exit.success() {
            let raw = fs::read_to_string(&out)?;
            let mut result: Value = serde_json::from_str(&raw)?;
            result["status"] = json!("complete");
            result
        } else {
            json!({"status":"worker_failed","exit_code":exit.code(),"stdout":out,"stderr":err})
        }
    } else {
        json!({"status":"timeout","deadline_seconds":limit.as_secs_f64(),"stdout":out,"stderr":err})
    };
    value["outer_process_ns_excluded_from_algorithm_interval"] = json!(outer_ns);
    Ok(value)
}
fn checked_ratio(left: &Value, right: &Value) -> Option<(f64, u128)> {
    if left["status"] != "complete"
        || right["status"] != "complete"
        || left["counter_build"] != false
        || right["counter_build"] != false
    {
        return None;
    }
    let replay = Instant::now();
    let equal = left["basis"] == right["basis"];
    let replay_ns = replay.elapsed().as_nanos();
    if !equal {
        return None;
    }
    let a = left["charged_ns"].as_u64()? as f64 + replay_ns as f64;
    let b = right["charged_ns"].as_u64()? as f64 + replay_ns as f64;
    if b <= 0. {
        None
    } else {
        Some((a / b, replay_ns))
    }
}
fn controls_eligible(
    sample: usize,
    arms: &serde_json::Map<String, Value>,
    pair_eligible: bool,
) -> bool {
    pair_eligible
        && sample.is_multiple_of(3)
        && arms
            .get("baseline")
            .is_some_and(|v| v["status"] == "complete")
        && arms
            .get("candidate")
            .is_some_and(|v| v["status"] == "complete")
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let (mut fixtures, mut out, mut binary, mut backend) = (None, None, None, None);
    let mut samples = 21usize;
    let mut baseline_arm = "baseline".to_string();
    let mut candidate_arm = "candidate".to_string();
    let mut protocol = "SPEED_PROTOCOL.md".to_string();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--fixtures" => fixtures = Some(PathBuf::from(args.next().ok_or("missing fixtures")?)),
            "--out" => out = Some(PathBuf::from(args.next().ok_or("missing output")?)),
            "--compare-bin" => {
                binary = Some(PathBuf::from(args.next().ok_or("missing worker binary")?))
            }
            "--backend" => backend = Some(args.next().ok_or("missing backend label")?),
            "--candidate-arm" => candidate_arm = args.next().ok_or("missing candidate arm")?,
            "--baseline-arm" => baseline_arm = args.next().ok_or("missing baseline arm")?,
            "--protocol" => protocol = args.next().ok_or("missing protocol label")?,
            "--samples" => samples = args.next().ok_or("missing sample count")?.parse()?,
            "--help" => {
                println!("campaign --fixtures DIR --out NEW_DIR --compare-bin EXE --backend local [--samples 21] [--baseline-arm baseline] [--candidate-arm candidate] [--protocol SPEED_PROTOCOL.md]");
                return Ok(());
            }
            _ => return Err(format!("unknown option {arg}").into()),
        }
    }
    if samples != 21 {
        return Err("frozen protocol requires exactly 21 samples".into());
    }
    if backend.as_deref() != Some("local") {
        return Err("this driver supports an explicitly selected local L0 backend only; do not use to bypass IsoLab admission".into());
    }
    let supported = |arm: &str| {
        matches!(
            arm,
            "baseline"
                | "candidate"
                | "projection"
                | "interreduce"
                | "projection-interreduce"
                | "criteria"
                | "projection-criteria"
                | "quadratic-projection-criteria"
                | "quotient"
        )
    };
    if !supported(&baseline_arm) || !supported(&candidate_arm) {
        return Err("unsupported baseline or candidate arm".into());
    }
    let fixtures = fixtures.ok_or("--fixtures required")?;
    let out = out.ok_or("--out required")?;
    let binary = binary.ok_or("--compare-bin required")?.canonicalize()?;
    if out.exists() {
        return Err("output already exists".into());
    }
    fs::create_dir_all(&out)?;
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(fixtures.join("manifest.json"))?)?;
    let cpu = allowed_cpu()?;
    let started = Instant::now();
    let mut summaries = Vec::new();
    let mut campaign_complete = true;
    fs::write(
        out.join("host.json"),
        serde_json::to_string_pretty(
            &json!({"backend":"local L0, no L2 isolation claim","cpu_affinity":cpu,"cpuinfo":fs::read_to_string("/proc/cpuinfo")?,"worker_binary":binary,"protocol":protocol,"baseline_arm":baseline_arm,"candidate_arm":candidate_arm}),
        )?,
    )?;
    let mut ordered = manifest["cases"]
        .as_array()
        .ok_or("missing manifest cases")?
        .clone();
    if candidate_arm != "candidate" {
        ordered.sort_by_key(|e| {
            (
                e["variables"].as_u64(),
                e["seed"].as_u64(),
                e["prime"].as_u64(),
            )
        });
    }
    for entry in &ordered {
        if started.elapsed() > Duration::from_secs(1200) {
            campaign_complete = false;
            break;
        }
        let id = entry["id"].as_str().ok_or("case ID")?;
        let fixture = fixtures
            .join(entry["fixture"].as_str().ok_or("fixture path")?)
            .canonicalize()?;
        let data: Fixture = serde_json::from_str(&fs::read_to_string(&fixture)?)?;
        if !relation_degree_trace::workload::witness_check(&data.system, &data.witness) {
            return Err("invalid fixture witness".into());
        }
        let directory = out.join(id);
        fs::create_dir_all(&directory)?;
        let (mut ratios, mut aa) = (Vec::new(), Vec::new());
        let mut raw = Vec::new();
        for sample in 0..samples {
            if started.elapsed() > Duration::from_secs(1200) {
                campaign_complete = false;
                break;
            }
            let order = if sample % 2 == 0 {
                ["baseline", "candidate"]
            } else {
                ["candidate", "baseline"]
            };
            let mut arms = serde_json::Map::new();
            for arm in order {
                let value = worker(
                    &binary,
                    &fixture,
                    if arm == "candidate" {
                        &candidate_arm
                    } else {
                        &baseline_arm
                    },
                    &directory.join(format!("sample-{sample:02}-{arm}")),
                    &cpu,
                    1,
                    Duration::from_secs(1200).saturating_sub(started.elapsed()),
                )?;
                arms.insert(arm.into(), value);
            }
            let ratio = checked_ratio(&arms["baseline"], &arms["candidate"]);
            if let Some((r, _)) = ratio {
                ratios.push(r);
            }
            let mut record = json!({"sample":sample,"order":order,"arms":arms,"checked_ratio":ratio.map(|r|r.0),"equality_replay_ns_charged_to_each":ratio.map(|r|r.1)});
            if controls_eligible(sample, &arms, ratio.is_some()) {
                let first = worker(
                    &binary,
                    &fixture,
                    &baseline_arm,
                    &directory.join(format!("aa-{sample:02}-first")),
                    &cpu,
                    1,
                    Duration::from_secs(1200).saturating_sub(started.elapsed()),
                )?;
                let second = worker(
                    &binary,
                    &fixture,
                    &baseline_arm,
                    &directory.join(format!("aa-{sample:02}-second")),
                    &cpu,
                    1,
                    Duration::from_secs(1200).saturating_sub(started.elapsed()),
                )?;
                let ratio = checked_ratio(&first, &second);
                if let Some((r, _)) = ratio {
                    aa.push(r);
                }
                record["aa"] = json!({"first":first,"second":second,"ratio":ratio.map(|r|r.0)});
            }
            fs::write(
                directory.join(format!("sample-{sample:02}.json")),
                serde_json::to_string_pretty(&record)? + "\n",
            )?;
            let failed = ratio.is_none()
                || record["arms"]["baseline"]["status"] != "complete"
                || record["arms"]["candidate"]["status"] != "complete";
            raw.push(record);
            if failed {
                break;
            }
        }
        let estimate = interval(&ratios);
        let aa_estimate = interval(&aa);
        let controls = aa.len() == 7
            && aa_estimate["bootstrap_95"][0]
                .as_f64()
                .is_some_and(|x| x <= 1.)
            && aa_estimate["bootstrap_95"][1]
                .as_f64()
                .is_some_and(|x| x >= 1.);
        let complete = ratios.len() == 21;
        let tenfold = complete
            && controls
            && estimate["bootstrap_95"][0]
                .as_f64()
                .is_some_and(|x| x > 10.);
        let summary = json!({"id":id,"complete":complete,"native_checked_wall_ratio":estimate,"aa_ratio":aa_estimate,"aa_admitted":controls,"baseline_arm":baseline_arm,"candidate_arm":candidate_arm,"protocol":protocol,
            "tenfold_native_criterion_met":tenfold,"external_comparisons":"pending separate supported CAS runs; no fastest claim", "claim_status":"producer observation, not independently signed","samples":raw});
        println!(
            "{id}: ratio={} complete={complete} A/A={controls} tenfold={tenfold}",
            summary["native_checked_wall_ratio"]["median"]
        );
        fs::write(
            directory.join("summary.json"),
            serde_json::to_string_pretty(&summary)? + "\n",
        )?;
        summaries.push(summary);
    }
    fs::write(
        out.join("summary.json"),
        serde_json::to_string_pretty(
            &json!({"protocol":protocol,"baseline_arm":baseline_arm,"candidate_arm":candidate_arm,"campaign_complete":campaign_complete&&summaries.len()==manifest["cases"].as_array().unwrap().len(),"case_summaries":summaries,"elapsed_wall_seconds":started.elapsed().as_secs_f64()}),
        )? + "\n",
    )?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_pair_does_not_launch_control_retries() {
        let arms = serde_json::Map::from_iter([
            ("baseline".to_string(), json!({"status":"timeout"})),
            ("candidate".to_string(), json!({"status":"complete"})),
        ]);
        assert!(!controls_eligible(0, &arms, false));
    }
    #[test]
    fn paired_bootstrap_keeps_the_declared_ratio() {
        let x = interval(&[12.; 21]);
        assert_eq!(x["bootstrap_95"], json!([12., 12.]));
    }
    #[test]
    fn mismatched_outputs_and_counters_cannot_be_wins() {
        let a = json!({"status":"complete","counter_build":false,"charged_ns":1000,"basis":[1]});
        let mut b = a.clone();
        b["basis"] = json!([2]);
        assert!(checked_ratio(&a, &b).is_none());
        b = a.clone();
        b["counter_build"] = json!(true);
        assert!(checked_ratio(&a, &b).is_none());
    }
}
