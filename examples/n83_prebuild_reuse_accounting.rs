//! Re-derive the frozen q=1,8,64 finite-query budgets from retained native receipts.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Component, Path},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
const MANIFEST: &str = "ac25c54667d5664257cae8e1183b272312b8eaba84f60f42799d828fac1ef9bc";
fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn numeric(v: &Value, key: &str) -> Result<f64> {
    let n = v[key]
        .as_f64()
        .ok_or_else(|| format!("missing numeric {key}"))?;
    require(n.is_finite() && n >= 0.0, "invalid nonnegative metric")?;
    Ok(n)
}
// The pinned card is entirely on this UTC date. Other dates are not silently
// interpreted as the same day; a new dataset needs an explicit parser extension.
fn clock_ns(s: &str) -> Result<u64> {
    let clock = s
        .strip_prefix("2026-10-19T")
        .and_then(|s| s.strip_suffix('Z'))
        .ok_or("date/zone outside frozen card")?;
    let p: Vec<_> = clock.split(':').collect();
    require(p.len() == 3, "clock shape")?;
    let h: u64 = p[0].parse()?;
    let m: u64 = p[1].parse()?;
    let (seconds, fraction) = p[2].split_once('.').ok_or("missing fractional seconds")?;
    let seconds: u64 = seconds.parse()?;
    require(h < 24 && m < 60 && seconds < 60, "clock range")?;
    require(
        !fraction.is_empty() && fraction.len() <= 9 && fraction.bytes().all(|b| b.is_ascii_digit()),
        "fraction shape",
    )?;
    let ns: u64 = fraction.parse()?;
    Ok(((h * 60 + m) * 60 + seconds) * 1_000_000_000 + ns * 10u64.pow(9 - fraction.len() as u32))
}
fn container_seconds(v: &Value) -> Result<f64> {
    require(
        v.as_array().is_some_and(|a| a.len() == 1),
        "container count",
    )?;
    let s = &v[0]["State"];
    require(
        s["Status"] == "exited"
            && s["ExitCode"] == 0
            && s["OOMKilled"] == false
            && s["Error"] == "",
        "container outcome",
    )?;
    let start = clock_ns(s["StartedAt"].as_str().ok_or("start")?)?;
    let end = clock_ns(s["FinishedAt"].as_str().ok_or("end")?)?;
    require(end > start, "nonpositive container interval")?;
    Ok((end - start) as f64 / 1e9)
}
fn audit(root: &Path) -> Result<Value> {
    let manifest_bytes = fs::read(root.join("runs/paired-mode-v1.sha256"))?;
    require(digest(&manifest_bytes) == MANIFEST, "paired manifest pin")?;
    let mut files = 0;
    for line in std::str::from_utf8(&manifest_bytes)?.lines() {
        let (hash, path) = line.split_once("  ").ok_or("manifest syntax")?;
        require(
            path.starts_with("paired-mode-v1/")
                && Path::new(path)
                    .components()
                    .all(|c| matches!(c, Component::Normal(_))),
            "unsafe path",
        )?;
        require(
            digest(&fs::read(root.join("runs").join(path))?) == hash,
            "paired file digest",
        )?;
        files += 1;
    }
    require(
        fs::read_to_string(root.join("runs/paired-mode-v1/run-card-status.txt"))?.trim()
            == "COMPLETE",
        "incomplete finite card",
    )?;
    let mut setup = Vec::new();
    for (name, path, hash, field) in [
        (
            "census",
            "runs/census-cost-v1/generated/summary.json",
            "654325b07da1ffa8c9e52f904acd01f1a2ac3d0e1fdad58353c6e250ea762d05",
            "process_wall_ms",
        ),
        (
            "export",
            "runs/export-cost/export-cost-v3/generated/manifest.json",
            "d9a1d7c11707e517f1f25b688be9c96f3e233907bb7ddccde5397d64208f6967",
            "construction_elapsed_ms_informational_L0",
        ),
        (
            "generic_replay",
            "runs/export-cost/export-cost-v3/generated/replay.json",
            "05e4f609a46a7741d4b2195186d6261094c91b9b7380d1a7e361f7eea1dd2ef4",
            "replay_elapsed_ms_informational_L0",
        ),
    ] {
        let bytes = fs::read(root.join(path))?;
        require(digest(&bytes) == hash, "setup receipt pin")?;
        let v: Value = serde_json::from_slice(&bytes)?;
        setup.push(
            json!({"phase":name,"path":path,"sha256":hash,"seconds":numeric(&v,field)?/1000.0}),
        );
    }
    let setup_seconds: f64 = setup.iter().map(|v| v["seconds"].as_f64().unwrap()).sum();
    let mut cells = Vec::new();
    for m in [2, 3] {
        for (tag, mode) in [("p", "prebuilt"), ("o", "ondemand")] {
            let mut observations = Vec::new();
            for target in 0..8 {
                for repeat in 1..=if target == 0 { 2 } else { 1 } {
                    let label = if target == 0 {
                        format!("t0-m{m}-{tag}{repeat}")
                    } else {
                        format!("t{target}-m{m}-{tag}")
                    };
                    let dir = root.join("runs/paired-mode-v1/runs").join(&label);
                    let bytes = fs::read(dir.join("worker.json"))?;
                    let w: Value = serde_json::from_slice(&bytes)?;
                    require(
                        w["schema"] == "n83.m23-probe/v1"
                            && w["status"] == "COMPLETE"
                            && w["arm"] == "D"
                            && w["m"] == m
                            && w["target_index"] == target
                            && w["base_prepare_mode"] == mode,
                        "probe identity/status",
                    )?;
                    require(w["point_set_blake3"]=="8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0"
                        && w["labeled_rows_blake3"]=="805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c"
                        && w["object_blake3"]=="e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682"
                        && w["public_corpus_blake3"]=="d6c3c5c493ea9e9ed3dc814a55d919372290401282a70600038b505906c6efa5", "probe input identity")?;
                    let expected = if m == 2 { 2_760_082 } else { 1_048_576 };
                    require(
                        w["completed"] == expected
                            && w["total"] == expected
                            && w["relations"] == 0
                            && w["rank"] == 0
                            && w["independent_rows"] == 0,
                        "finite event outcome",
                    )?;
                    let replay_dir = dir.with_file_name(format!("{label}-replay"));
                    let replay = read(&replay_dir.join("worker.json"))?;
                    require(
                        replay["schema"] == "n83.m23-replay/v1"
                            && replay["status"] == "PASS"
                            && replay["worker_blake3"] == blake3::hash(&bytes).to_hex().as_str()
                            && replay["relations_checked"] == 0
                            && replay["rank"] == 0,
                        "fresh replay binding",
                    )?;
                    observations.push(json!({
                        "label":label,"target_index":target,
                        "worker_seconds":numeric(&w,"process_wall_ms")?/1000.0,
                        "worker_container_seconds":container_seconds(&read(&dir.join("container-inspect.json"))?)?,
                        "fresh_replay_container_seconds":container_seconds(&read(&replay_dir.join("container-inspect.json"))?)?,
                        "worker_peak_rss_bytes":w["peak_rss_bytes"],
                        "worker_sha256":digest(&bytes)
                    }));
                }
            }
            require(observations.len() == 9, "cell coverage")?;
            let mean = |key: &str| {
                observations
                    .iter()
                    .map(|r| r[key].as_f64().unwrap())
                    .sum::<f64>()
                    / 9.0
            };
            let worker = mean("worker_seconds");
            let container = mean("worker_container_seconds");
            let replay = mean("fresh_replay_container_seconds");
            let one_time = if mode == "prebuilt" {
                setup_seconds
            } else {
                0.0
            };
            let projections: Vec<_> = [1u64,8,64].into_iter().map(|q|json!({
                "reuse_queries":q,"derived":true,
                "setup_plus_worker_seconds":one_time + q as f64 * worker,
                "setup_plus_worker_and_fresh_replay_containers_seconds":one_time + q as f64 * (container + replay),
                "setup_seconds":one_time,"fresh_replay_container_seconds":q as f64 * replay
            })).collect();
            cells.push(json!({"m":m,"mode":mode,"observed_probes":9,"worker_mean_seconds":worker,
                "worker_container_mean_seconds":container,"fresh_replay_container_mean_seconds":replay,
                "worker_peak_rss_max_bytes":observations.iter().map(|r|r["worker_peak_rss_bytes"].as_u64().ok_or("missing RSS")).collect::<std::result::Result<Vec<_>,_>>()?.into_iter().max(),
                "observations":observations,"projections":projections}));
        }
    }
    Ok(
        json!({"schema":"n83.finite-reuse-accounting/v1","status":"PASS_PINNED_ACCOUNTING",
        "paired_manifest_sha256":MANIFEST,"paired_files_checked":files,"setup_phases":setup,"setup_seconds":setup_seconds,"cells":cells,
        "classification":"accounting addition; no algorithm change or newly timed experiment",
        "candidate_id":null,"selected_best_total_runtime":null,"complete_ic_seconds":null,"matched_rho_gap":null,
        "scope":"Derived q=1,8,64 budgets from nine observations per mode/m, with target 0 repeated. Worker-only sums preserve the earlier interval; the additional column charges both observed worker and fresh-replay container intervals. Setup uses separately recorded native exporter/census/replayer revisions. All observations are L0. These are finite-event budgets, not 64 executed queries or complete cold IC.",
        "excluded":["Docker launch/teardown outside container intervals","remote custody and persistent storage","earlier implementation and infrastructure retry history","natural useful-rank collection beyond the exact finite events","linear algebra and individual logarithm verification","matched same-point rho and external-fastest comparison"],
        "memory_scope":"Observed worker peak RSS only; no inferred peak for an unexecuted reuse panel."}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    require(
        args.len() == 3,
        "usage: n83_prebuild_reuse_accounting STUDY_ROOT NEW_OUTPUT",
    )?;
    let result = audit(Path::new(&args[1]))?;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut f, &result)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duration_preserves_nanoseconds_and_rejects_censored_or_misdated_records() {
        let mut record = json!([{"State":{"Status":"exited","ExitCode":0,"OOMKilled":false,"Error":"",
            "StartedAt":"2026-10-19T06:13:55.766726329Z","FinishedAt":"2026-10-19T06:13:59.627540302Z"}}]);
        assert_eq!(container_seconds(&record).unwrap(), 3.860813973);
        record[0]["State"]["ExitCode"] = json!(124);
        assert!(container_seconds(&record).is_err());
        record[0]["State"]["ExitCode"] = json!(0);
        record[0]["State"]["FinishedAt"] = record[0]["State"]["StartedAt"].clone();
        assert!(container_seconds(&record).is_err());
        for s in [
            "2026-10-20T06:13:55.1Z",
            "2026-10-19T24:00:00.1Z",
            "2026-10-19T00:00:60.1Z",
            "2026-10-19T00:00:00.1234567890Z",
        ] {
            assert!(clock_ns(s).is_err());
        }
    }
}
