//! Receipt audit of the completed, frozen small-m4-v1 card, not a relation verifier.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    io::Write,
    path::{Component, Path},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const ARCHIVE: &str = "b319353bf163c91b3180d6bf863b5129c580d3b7852751f345e4fd257b5abee9";
const REV: &str = "611aad60e4d9952d6239a3382a77184da3640eeb";
const BIN: &str = "3048550afc4ad1bdad1e390f74802112dfc6fadf96048cbff823f8f65364b0c3";
const CORPUS: &str = "2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5";
const COUNTERS: [&str; 8] = [
    "completed_conflicts",
    "completed_decisions",
    "completed_restarts",
    "completed_xor_passes",
    "completed_xor_nanoseconds",
    "completed_xor_repivots",
    "completed_xor_row_ops",
    "completed_xor_row_scans",
];

fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn sha(p: &Path) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(fs::read(p)?)))
}
fn number(v: &Value, k: &str) -> Result<f64> {
    let n = v[k]
        .as_f64()
        .ok_or_else(|| format!("missing numeric {k}"))?;
    require(n.is_finite() && n >= 0.0, "negative/nonfinite metric")?;
    Ok(n)
}
fn integer(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing integer {k}").into())
}
fn jsonl(p: &Path) -> Result<Vec<Value>> {
    let s = fs::read_to_string(p)?;
    require(s.ends_with('\n'), "truncated JSONL")?;
    s.lines().map(|l| Ok(serde_json::from_str(l)?)).collect()
}
fn manifest_path(p: &str) -> Result<()> {
    require(
        !p.is_empty()
            && Path::new(p)
                .components()
                .all(|c| matches!(c, Component::Normal(_)))
            && (p.starts_with("host/") || p.starts_with("runs/")),
        "unsafe manifest path",
    )
}
fn list_files(root: &Path, dir: &Path, out: &mut BTreeSet<String>) -> Result<()> {
    for e in fs::read_dir(dir)? {
        let e = e?;
        let t = e.file_type()?;
        require(!t.is_symlink(), "symlink in evidence")?;
        if t.is_dir() {
            list_files(root, &e.path(), out)?;
        } else {
            require(t.is_file(), "non-file in evidence")?;
            out.insert(
                e.path()
                    .strip_prefix(root)?
                    .to_str()
                    .ok_or("non-UTF8 path")?
                    .to_owned(),
            );
        }
    }
    Ok(())
}
fn manifest(root: &Path) -> Result<usize> {
    require(
        sha(&root.join("files.sha256"))?
            == "0eef8c9bffc75f0dd239ea5d89de4074965ea9bb48b906415795dc9da7f0533b",
        "manifest SHA256 mismatch",
    )?;
    require(
        sha(&root.join("runs.tar.xz"))? == ARCHIVE,
        "archive SHA256 mismatch",
    )?;
    let mut expected = BTreeSet::new();
    for l in fs::read_to_string(root.join("files.sha256"))?.lines() {
        let (hash, path) = l.split_once("  ").ok_or("bad checksum line")?;
        manifest_path(path)?;
        require(expected.insert(path.to_owned()), "duplicate manifest path")?;
        require(sha(&root.join(path))? == hash, "file checksum mismatch")?;
    }
    let mut actual = BTreeSet::new();
    for d in ["host", "runs"] {
        list_files(root, &root.join(d), &mut actual)?;
    }
    require(
        actual == expected && actual.len() == 132,
        "incomplete/extra evidence files",
    )?;
    Ok(actual.len())
}
fn check_container(dir: &Path, replay: bool) -> Result<Value> {
    require(
        fs::read_to_string(dir.join("host-exit.txt"))?.trim() == "0",
        "host exit",
    )?;
    let a = read(&dir.join("container-inspect.json"))?;
    require(
        a.as_array().is_some_and(|a| a.len() == 1),
        "container count",
    )?;
    let c = &a[0];
    let h = &c["HostConfig"];
    let s = &c["State"];
    require(
        s["Status"] == "exited"
            && s["OOMKilled"] == false
            && s["ExitCode"] == 0
            && s["Running"] == false
            && s["Error"] == "",
        "container did not finish cleanly",
    )?;
    require(
        h["Memory"] == 4294967296u64
            && h["MemorySwap"] == 4294967296u64
            && h["NanoCpus"] == 1000000000u64
            && h["CpusetCpus"] == "31"
            && h["NetworkMode"] == "none"
            && h["ReadonlyRootfs"] == true,
        "container limits",
    )?;
    let mounts = c["Mounts"].as_array().ok_or("mounts")?;
    let wanted: BTreeSet<_> = if replay {
        vec![
            "/out",
            "/opt/worker",
            "/inputs/base",
            "/inputs/public.json",
            "/inputs/receipt.json",
        ]
    } else {
        vec!["/out", "/opt/worker", "/inputs/base", "/inputs/public.json"]
    }
    .into_iter()
    .collect();
    let mut seen = BTreeSet::new();
    for m in mounts {
        let dest = m["Destination"].as_str().ok_or("mount destination")?;
        require(
            m["Type"] == "bind" && m["RW"] == (dest == "/out") && seen.insert(dest),
            "mount access",
        )?;
    }
    require(seen == wanted, "unexpected mounted inputs")?;
    Ok(json!({"started":s["StartedAt"],"finished":s["FinishedAt"]}))
}
fn identity(w: &Value, arm: &str) -> Result<()> {
    require(
        w["schema"] == "n83.small-m4-worker/v1"
            && w["curve_id"] == "icv1-f2m83-tm6151469093347-debefd74"
            && w["source_commit"] == REV
            && w["binary_sha256"] == BIN
            && w["public_corpus_sha256"] == CORPUS
            && w["arm"] == arm
            && w["m"] == 4
            && w["target_index"] == 0,
        "worker identity",
    )?;
    for k in ["candidate_id", "selected_best_total_runtime"] {
        require(w.get(k) == Some(&Value::Null), "unsupported selection")?;
    }
    let (cols, obj, set, labels, vars, clauses) = match arm {
        "K2048" => (
            2048,
            "82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851",
            "d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e",
            "227cd8aefc217772f9c80c0246d132306a001d7573d6ca681ef379803ce1bb65",
            84966,
            900062,
        ),
        "D12" => (
            2001,
            "aea278aff2d79a592d5164d7437ddde8f8e9377fa481058dfc16afa176a31389",
            "fb8ea2ecfeb245a90fecd5506dfc01592916b4ebec756d5fea7cbdd0c25537ce",
            "a58ee192e601d1d8fe9beabec6c43ea168f735db79803fcfda7a39914a1d2b48",
            84590,
            883894,
        ),
        _ => return Err("unknown arm".into()),
    };
    require(
        w["columns"] == cols
            && w["points"] == cols * 166
            && w["object_blake3"] == obj
            && w["point_set_blake3"] == set
            && w["labeled_rows_blake3"] == labels,
        "base identity",
    )?;
    if w["mode"] != "preflight" {
        require(
            w["sat_variables"] == vars && w["sat_clauses"] == clauses && w["sat_xor_rows"] == 3901,
            "model counts",
        )?;
    }
    Ok(())
}
fn progress(w: &Value, samples: &[Value]) -> Result<(f64, Vec<Value>)> {
    require(samples.len() > 1, "missing progress stream")?;
    let mut last_time = 0.0;
    let mut max_gap: f64 = 0.0;
    let mut last = [0u64; 8];
    let mut plot = Vec::new();
    for (i, s) in samples.iter().enumerate() {
        require(s["schema"] == "n83.small-m4-progress/v1", "progress schema")?;
        let id = s["identity"].as_object().ok_or("progress identity")?;
        for (k, v) in id {
            require(w.get(k) == Some(v), "progress identity drift")?;
        }
        require(
            s["final_sample"] == (i + 1 == samples.len()),
            "incomplete/early final sample",
        )?;
        let t = number(s, "elapsed_ms")?;
        require(
            t >= last_time && t <= number(w, "process_wall_ms")?,
            "progress clock",
        )?;
        max_gap = max_gap.max(t - last_time);
        last_time = t;
        for (j, k) in COUNTERS.iter().enumerate() {
            let n = integer(&s["counters"], k)?;
            require(
                n >= last[j] && n <= integer(&w["counters"], k)?,
                "counter decrease/overshoot",
            )?;
            last[j] = n;
        }
        plot.push(json!({"elapsed_s":t/1000.0,"conflicts":s["counters"]["completed_conflicts"]}));
    }
    require(
        samples.last().unwrap()["counters"] == w["counters"],
        "final counters disagree",
    )?;
    Ok((max_gap, plot))
}
fn audit(root: &Path) -> Result<Value> {
    let count = manifest(root)?;
    require(
        fs::read_to_string(root.join("host/card-status.txt"))?.trim() == "COMPLETE_BOUNDED",
        "card incomplete",
    )?;
    require(
        fs::read_to_string(root.join("host/source-commit.txt"))?.trim() == REV,
        "source revision",
    )?;
    require(
        sha(&root.join("host/run-small-m4-card.sh"))?
            == "b6a5dc13c44b44bafab85ec06eb4f83d114dd6e6fcf529a87989a4a124c70fb4",
        "card hash",
    )?;
    require(
        sha(&root.join("host/run-small-m4-docker.sh"))?
            == "2bd6992965af930c5038563dba2cbd6e5a18e0c426a20f00665779e31ba3006f",
        "docker wrapper hash",
    )?;
    let mut last_finish = String::new();
    for label in [
        "preflight-K2048",
        "preflight-D12",
        "positive-K2048",
        "positive-K2048-replay",
        "positive-D12",
        "positive-D12-replay",
    ] {
        let dir = root.join("runs").join(label);
        let replay = label.ends_with("-replay");
        let c = check_container(&dir, replay)?;
        require(
            c["started"].as_str().ok_or("start")? >= last_finish.as_str(),
            "overlapping/misordered controls",
        )?;
        last_finish = c["finished"].as_str().ok_or("end")?.to_owned();
        let w = read(&dir.join("worker.json"))?;
        let arm = if label.contains("K2048") {
            "K2048"
        } else {
            "D12"
        };
        if replay {
            require(
                w["status"] == "PASS"
                    && w["rank_lower_bound"] == 1
                    && w["arm"] == arm
                    && w["mode"] == "positive"
                    && w["changed_target_rejected"] == true
                    && w["changed_point_rejected"] == true
                    && w["changed_coefficient_rejected"] == true,
                "control replay",
            )?;
            let bytes = fs::read(
                root.join("runs")
                    .join(format!("positive-{arm}"))
                    .join("worker.json"),
            )?;
            require(
                w["worker_blake3"] == blake3::hash(&bytes).to_hex().as_str(),
                "replay worker binding",
            )?;
        } else {
            identity(&w, arm)?;
            require(
                w["status"]
                    == if label.starts_with("preflight") {
                        "PASS_PREFLIGHT"
                    } else {
                        "PASS_CONSTRUCTED_PENDING_REPLAY"
                    },
                "control status",
            )?;
            if w["mode"] == "positive" {
                require(
                    w["rank"] == 1 && w["group_verified"] == true,
                    "positive rank",
                )?;
            }
        }
    }
    let mut arms = Vec::new();
    for (label, arm) in [
        ("k1", "K2048"),
        ("d1", "D12"),
        ("d2", "D12"),
        ("k2", "K2048"),
    ] {
        let dir = root.join("runs").join(label);
        let container = check_container(&dir, false)?;
        require(
            container["started"].as_str().ok_or("start")? >= last_finish.as_str(),
            "natural order",
        )?;
        last_finish = container["finished"].as_str().ok_or("end")?.to_owned();
        let w = read(&dir.join("worker.json"))?;
        identity(&w, arm)?;
        require(
            w["status"] == "UNKNOWN_search_cap"
                && w["mode"] == "natural"
                && w["target"] == json!(["1834913121178197242856238", "8797964190732846213981377"])
                && w["positive_pin_ms"] == 0.0
                && w["counters"]["completed_conflicts"] == 100000
                && w["counters"]["phase"] == "Complete",
            "frozen conflict-capped event",
        )?;
        for k in ["rank", "sparse_row", "point_indices", "group_verified"] {
            require(w.get(k).is_none(), "censored result promoted")?;
        }
        let wall = number(&w, "process_wall_ms")?;
        require(
            wall < 1790000.0 && number(&w, "memory_cgroup_peak_bytes")? < 4294967296.0,
            "wall or memory cap",
        )?;
        let samples = jsonl(&dir.join("worker.json.progress.jsonl"))?;
        for sample in &samples {
            identity(&sample["identity"], arm)?;
        }
        let (gap, series) = progress(&w, &samples)?;
        let phases = jsonl(&dir.join("worker.json.phases.jsonl"))?;
        let names: Vec<_> = phases
            .iter()
            .map(|p| p["phase"].as_str().unwrap_or(""))
            .collect();
        require(
            names
                == [
                    "import_entered",
                    "corpus_validation_entered",
                    "model_entered",
                    "model_done",
                    "root_entered",
                    "root_returned",
                    "search_entered",
                    "search_returned",
                    "worker_returned",
                ],
            "phase coverage",
        )?;
        require(
            phases.last().unwrap()["details"] == w,
            "final phase binding",
        )?;
        require(
            phases[5]["details"]["result"] == "Fixpoint"
                && phases[7]["details"]["answer"] == "Unknown",
            "phase outcome",
        )?;
        let xor_s = integer(&w["counters"], "completed_xor_nanoseconds")? as f64 / 1e9;
        arms.push(json!({"label":label,"arm":arm,"status":w["status"],"worker_sha256":sha(&dir.join("worker.json"))?,
            "wall_s":wall/1000.0,"search_s":number(&w,"search_ms")?/1000.0,"root_ms":w["root_ms"],
            "import_ms":w["base_import_index_ms"],"model_ms":w["model_domain_ms"],"xor_s":xor_s,
            "xor_elapsed_fraction":xor_s/(wall/1000.0),"cpu_s":w["cpu_seconds"],"peak_cgroup_bytes":w["memory_cgroup_peak_bytes"],
            "counters":w["counters"],"progress_samples":samples.len(),"max_sample_gap_ms":gap,"progress":series,"container":container,
            "natural_rank":null,"natural_yield":null}));
    }
    for (a, b) in [(0, 3), (1, 2)] {
        for k in COUNTERS {
            if k != "completed_xor_nanoseconds" {
                require(
                    arms[a]["counters"][k] == arms[b]["counters"][k],
                    "repeat search path differs",
                )?;
            }
        }
    }
    Ok(
        json!({"schema":"n83.small-m4-card-audit/v1","status":"PASS_RECEIPT_AUDIT","archive_sha256":ARCHIVE,"manifest_files":count,
        "source_commit":REV,"worker_sha256":BIN,"host_isolation":"L0","arms":arms,"controls":2,"independent_control_replays":2,
        "completed_natural_arms":4,"censored_natural_arms":4,"candidate_id":null,"selected_best_total_runtime":null,
        "scope":"Archived receipts, input identities, resource configuration, progress, and repeat counters; no natural relation or complete IC cost certificate"}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    require(
        args.len() == 3,
        "usage: n83_small_m4_audit EXTRACTED_FINAL_ROOT NEW_AUDIT_JSON",
    )?;
    let result = audit(Path::new(&args[1]))?;
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
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
    fn rejects_manifest_escape() {
        for p in [
            "../secret",
            "/host/a",
            "host/../../secret",
            "runs/a/../b",
            "other/a",
        ] {
            assert!(manifest_path(p).is_err());
        }
        assert!(manifest_path("runs/k1/worker.json").is_ok());
    }
    #[test]
    fn rejects_censored_progress_promotion_or_counter_drift() {
        let mut c = json!({"phase":"Complete"});
        for k in COUNTERS {
            c[k] = json!(1);
        }
        let w = json!({"arm":"K2048","process_wall_ms":30.0,"counters":c});
        let a = json!({"schema":"n83.small-m4-progress/v1","identity":{"arm":"K2048"},"elapsed_ms":10.0,"counters":c,"final_sample":false});
        let mut b = a.clone();
        b["elapsed_ms"] = json!(20.0);
        b["final_sample"] = json!(true);
        assert!(progress(&w, &[a.clone(), b.clone()]).is_ok());
        assert!(progress(&w, &[]).is_err());
        let mut bad = b.clone();
        bad["final_sample"] = json!(false);
        assert!(progress(&w, &[a.clone(), bad]).is_err());
        let mut bad = b.clone();
        bad["identity"]["arm"] = json!("D12");
        assert!(progress(&w, &[a.clone(), bad]).is_err());
        let mut bad = b.clone();
        bad["counters"]["completed_conflicts"] = json!(0);
        assert!(progress(&w, &[a.clone(), bad]).is_err());
        let mut bad = b;
        bad["elapsed_ms"] = json!(5.0);
        assert!(progress(&w, &[a, bad]).is_err());
    }
}
