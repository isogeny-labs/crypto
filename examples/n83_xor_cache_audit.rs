//! Audit the frozen v2 XOR-cache card. This audits receipts, not mathematical UNSAT.
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
const REV: &str = "1de47a9d4186bff08abc84c224a111183e820106";
const BIN: &str = "a9b8599761c6cce8a169602086c3426af9f9f7a55a8108124daed56059ffb3bc";
const REPLAYER: &str = "9ed4476c8869d0fc124a4f68392fc968071c1b804fa1688796ca2ab062e55861";
const CORPUS: &str = "2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5";
const IMAGE: &str =
    "amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f";
const IMAGE_ID: &str = "sha256:24b8580b081b53113046267a47ad12858ad97752c9735e1125fcdb7a62d1d2c9";
const REMOTE: &str = "/home/ec2-user/n83-m23-20261019";
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
const SAME_WORK: [&str; 6] = [
    "completed_conflicts",
    "completed_decisions",
    "completed_restarts",
    "completed_xor_passes",
    "completed_xor_row_ops",
    "completed_xor_row_scans",
];
const PHASES: [&str; 9] = [
    "import_entered",
    "corpus_validation_entered",
    "model_entered",
    "model_done",
    "root_entered",
    "root_returned",
    "search_entered",
    "search_returned",
    "worker_returned",
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
fn integer(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing integer {k}").into())
}
fn number(v: &Value, k: &str) -> Result<f64> {
    let n = v[k]
        .as_f64()
        .ok_or_else(|| format!("missing numeric {k}"))?;
    require(n.is_finite() && n >= 0.0, "invalid numeric metric")?;
    Ok(n)
}
fn jsonl(p: &Path) -> Result<Vec<Value>> {
    let s = fs::read_to_string(p)?;
    require(s.ends_with('\n'), "truncated JSONL")?;
    s.lines().map(|s| Ok(serde_json::from_str(s)?)).collect()
}
fn safe_path(p: &str) -> Result<()> {
    require(
        (p.starts_with("host/") || p.starts_with("runs/"))
            && Path::new(p)
                .components()
                .all(|c| matches!(c, Component::Normal(_))),
        "unsafe archive path",
    )
}
fn inventory(root: &Path, dir: &Path, files: &mut BTreeSet<String>) -> Result<()> {
    for e in fs::read_dir(dir)? {
        let e = e?;
        let t = e.file_type()?;
        require(!t.is_symlink(), "symlink in evidence")?;
        if t.is_dir() {
            inventory(root, &e.path(), files)?;
        } else {
            require(t.is_file(), "non-file in evidence")?;
            files.insert(
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
fn manifest(root: &Path, archive: &str, manifest: &str) -> Result<usize> {
    for h in [archive, manifest] {
        require(
            h.len() == 64 && h.bytes().all(|b| b.is_ascii_hexdigit()),
            "invalid expected digest",
        )?;
    }
    require(sha(&root.join("runs.tar.xz"))? == archive, "archive hash")?;
    require(
        sha(&root.join("files.sha256"))? == manifest,
        "manifest hash",
    )?;
    let mut expected = BTreeSet::new();
    for l in fs::read_to_string(root.join("files.sha256"))?.lines() {
        let (h, p) = l.split_once("  ").ok_or("checksum syntax")?;
        safe_path(p)?;
        require(expected.insert(p.to_owned()), "duplicate manifest entry")?;
        require(sha(&root.join(p))? == h, "file hash")?;
    }
    let mut actual = BTreeSet::new();
    for d in ["host", "runs"] {
        inventory(root, &root.join(d), &mut actual)?;
    }
    require(actual == expected, "incomplete/extra manifest files")?;
    Ok(actual.len())
}
fn core_identity(w: &Value, arm: &str, cache: bool, target: usize, mode: &str) -> Result<()> {
    require(
        w["schema"] == "n83.small-m4-worker/v1"
            && w["curve_id"] == "icv1-f2m83-tm6151469093347-debefd74"
            && w["source_commit"] == REV
            && w["binary_sha256"] == BIN
            && w["arm"] == arm
            && w["mode"] == mode
            && w["m"] == 4
            && w["target_index"] == target
            && w["experiment_policy"] == "small-m4-xor-cache/v1"
            && w["xor_decided_cache"] == cache
            && w["conflict_budget"] == 10000
            && w["source_module_blake3"]
                == "385455cf353c041bb1563ffec96ff93f082c134888c0f37d8d7735190d275df8"
            && w["source_worker_blake3"]
                == "5333d10c231494e24ae74f7cc90ca96cafaf7b58fd8e0a1aef784e597d8010ee",
        "worker core identity",
    )?;
    for k in ["candidate_id", "selected_best_total_runtime"] {
        require(w.get(k) == Some(&Value::Null), "unsupported selection")?;
    }
    Ok(())
}
fn full_identity(w: &Value, arm: &str, cache: bool, target: usize, mode: &str) -> Result<()> {
    core_identity(w, arm, cache, target, mode)?;
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
        _ => return Err("unknown base".into()),
    };
    require(
        w["columns"] == cols
            && w["points"] == cols * 166
            && w["object_blake3"] == obj
            && w["point_set_blake3"] == set
            && w["labeled_rows_blake3"] == labels
            && w["sat_variables"] == vars
            && w["sat_clauses"] == clauses
            && w["sat_xor_rows"] == 3901
            && w["public_corpus_sha256"] == CORPUS,
        "base/model/corpus identity",
    )
}
fn container(
    root: &Path,
    label: &str,
    spec: (&str, bool, usize, &str),
    exit: u64,
    last_finish: &mut String,
) -> Result<Value> {
    let (arm, cache, target, mode) = spec;
    let dir = root.join("runs").join(label);
    require(
        fs::read_to_string(dir.join("host-exit.txt"))?.trim() == exit.to_string(),
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
            && s["Running"] == false
            && s["OOMKilled"] == false
            && s["Error"] == ""
            && s["ExitCode"] == exit,
        "container termination",
    )?;
    require(
        h["Memory"] == 4294967296u64
            && h["MemorySwap"] == 4294967296u64
            && h["NanoCpus"] == 1000000000u64
            && h["CpusetCpus"] == "31"
            && h["NetworkMode"] == "none"
            && h["ReadonlyRootfs"] == true
            && h["PidsLimit"] == 256
            && h["CapDrop"] == json!(["ALL"])
            && h["SecurityOpt"] == json!(["no-new-privileges"]),
        "container policy",
    )?;
    require(
        c["Config"]["Image"] == IMAGE && c["Image"] == IMAGE_ID,
        "container image",
    )?;
    let im = read(&dir.join("image-inspect.json"))?;
    require(
        im[0]["Id"] == IMAGE_ID && im[0]["Architecture"] == "amd64",
        "image identity",
    )?;
    let binary = if mode == "replay" { REPLAYER } else { BIN };
    require(
        fs::read_to_string(dir.join("binary.sha256"))?
            .split_whitespace()
            .next()
            == Some(binary),
        "binary digest",
    )?;
    let mut args = vec![
        "-s".to_owned(),
        "TERM".to_owned(),
        "-k".to_owned(),
        "5".to_owned(),
        "1800".to_owned(),
        "/opt/worker".to_owned(),
    ];
    if mode == "replay" {
        args.extend(
            [
                "/inputs/base",
                "/inputs/public.json",
                "/inputs/receipt.json",
                "/out/worker.json",
            ]
            .map(str::to_owned),
        );
    } else {
        args.extend([
            "small-m4-cache".to_owned(),
            if cache { "on" } else { "off" }.to_owned(),
            mode.to_owned(),
            arm.to_owned(),
            "/inputs/base".to_owned(),
            "/inputs/public.json".to_owned(),
            target.to_string(),
            "/out/worker.json".to_owned(),
        ]);
    }
    require(
        c["Path"] == "/usr/bin/timeout" && c["Args"] == json!(args),
        "container arguments",
    )?;
    require(
        c["Config"]["Env"]
            .as_array()
            .is_some_and(|a| a.contains(&json!(format!("N83_SOURCE_COMMIT={REV}")))),
        "container revision",
    )?;
    let mounts = c["Mounts"].as_array().ok_or("mounts")?;
    let mut seen = BTreeSet::new();
    for m in mounts {
        let dest = m["Destination"].as_str().ok_or("mount destination")?;
        let source = match dest {
            "/out" => format!("{REMOTE}/xor-cache-v2/runs/{label}"),
            "/opt/worker" => format!(
                "{REMOTE}/xor-cache-v2/{}",
                if mode == "replay" {
                    "replayer"
                } else {
                    "worker"
                }
            ),
            "/inputs/base" => format!("{REMOTE}/inputs/{arm}"),
            "/inputs/public.json" => format!("{REMOTE}/inputs/public/public-targets.json"),
            "/inputs/receipt.json" if mode == "replay" => format!(
                "{REMOTE}/xor-cache-v2/runs/{}/worker.json",
                label.strip_suffix("-replay").ok_or("replay label")?
            ),
            _ => return Err("unexpected mounted input".into()),
        };
        require(
            m["Type"] == "bind"
                && m["RW"] == (dest == "/out")
                && m["Source"] == source
                && seen.insert(dest),
            "mount access/source",
        )?;
    }
    require(
        mounts.len() == if mode == "replay" { 5 } else { 4 },
        "mount coverage",
    )?;
    let start = s["StartedAt"].as_str().ok_or("start")?;
    let end = s["FinishedAt"].as_str().ok_or("end")?;
    require(
        start >= last_finish.as_str() && end > start,
        "overlap/order/clock",
    )?;
    *last_finish = end.to_owned();
    Ok(json!({"started":start,"finished":end,"exit":exit}))
}
fn progress(w: &Value, samples: &[Value], complete: bool) -> Result<Value> {
    require(!samples.is_empty(), "missing progress")?;
    let mut last_time = 0.0;
    let mut gap: f64 = 0.0;
    let mut last = [0u64; 8];
    let mut series = Vec::new();
    for (i, s) in samples.iter().enumerate() {
        require(
            s["schema"] == "n83.small-m4-progress/v1"
                && s["final_sample"] == (complete && i + 1 == samples.len()),
            "progress finality/schema",
        )?;
        for (k, v) in s["identity"].as_object().ok_or("progress identity")? {
            require(w.get(k) == Some(v), "progress identity drift")?;
        }
        let t = number(s, "elapsed_ms")?;
        require(
            t >= last_time && t <= number(w, "process_wall_ms")?,
            "progress clock",
        )?;
        gap = gap.max(t - last_time);
        last_time = t;
        for (j, k) in COUNTERS.iter().enumerate() {
            let n = integer(&s["counters"], k)?;
            require(
                n >= last[j] && n <= integer(&w["counters"], k)?,
                "counter decrease/overshoot",
            )?;
            last[j] = n;
        }
        series.push(json!({"elapsed_s":t/1000.0,"conflicts":s["counters"]["completed_conflicts"]}));
    }
    require(
        samples.last().unwrap()["counters"] == w["counters"],
        "final/latest counter binding",
    )?;
    Ok(json!({"samples":samples.len(),"max_gap_ms":gap,"series":series}))
}
fn paired(rows: &[Value]) -> Result<Value> {
    require(rows.len() == 4, "four runs required")?;
    if rows.iter().any(|r| r["status"] == "UNKNOWN_budget") {
        return Ok(
            json!({"status":"CENSORED_UNMATCHED_ENDPOINT","worker_ratio_off_over_on":null,"row_visit_ratio_off_over_on":null}),
        );
    }
    let a = &rows[0];
    for (i, b) in rows.iter().enumerate() {
        require(
            b["xor_decided_cache"] == (i == 1 || i == 2),
            "ABBA cache order",
        )?;
        for k in [
            "status",
            "target",
            "sat_model_blake3",
            "point_indices",
            "sparse_row",
            "sparse_row_blake3",
            "rank",
            "group_verified",
        ] {
            require(a.get(k) == b.get(k), "matched outcome/model/row differs")?;
        }
        for k in SAME_WORK {
            require(
                integer(&a["counters"], k)? == integer(&b["counters"], k)?,
                "matched search path differs",
            )?;
        }
        let skips = integer(b, "xor_decided_skips")?;
        require((i == 1 || i == 2) || skips == 0, "off cache has skips")?;
        require(
            integer(&b["counters"], "completed_xor_repivots")?.checked_add(skips)
                == Some(integer(&a["counters"], "completed_xor_repivots")?),
            "repivot conservation",
        )?;
    }
    let avg = |indices: [usize; 2], k: &str| -> Result<f64> {
        Ok((number(&rows[indices[0]], k)? + number(&rows[indices[1]], k)?) / 2.0)
    };
    for (i, j) in [(0, 3), (1, 2)] {
        require(
            rows[i]["counters"]["completed_xor_repivots"]
                == rows[j]["counters"]["completed_xor_repivots"]
                && rows[i]["xor_decided_skips"] == rows[j]["xor_decided_skips"],
            "repeat cache work differs",
        )?;
    }
    let off = avg([0, 3], "process_wall_ms")?;
    let on = avg([1, 2], "process_wall_ms")?;
    let off_visits = integer(&a["counters"], "completed_xor_repivots")?;
    let on_visits = integer(&rows[1]["counters"], "completed_xor_repivots")?;
    require(
        off > 0.0 && on > 0.0 && on_visits > 0,
        "zero ratio denominator",
    )?;
    Ok(
        json!({"status":"PASS_MATCHED_SEARCH_ENDPOINT","off_mean_worker_s":off/1000.0,"on_mean_worker_s":on/1000.0,"worker_ratio_off_over_on":off/on,
        "off_repivot_visits":off_visits,"on_repivot_visits":on_visits,"cached_skips":rows[1]["xor_decided_skips"],"row_visit_ratio_off_over_on":off_visits as f64/on_visits as f64,
        "row_visit_reduction_fraction":1.0-on_visits as f64/off_visits as f64,"same_search_counters":SAME_WORK,"qualified_timing_improvement":null}),
    )
}
fn replay(
    root: &Path,
    label: &str,
    w: &Value,
    last: &mut String,
    dirs: &mut BTreeSet<String>,
) -> Result<()> {
    let rlabel = format!("{label}-replay");
    dirs.insert(rlabel.clone());
    let arm = w["arm"].as_str().ok_or("arm")?;
    container(
        root,
        &rlabel,
        (
            arm,
            w["xor_decided_cache"].as_bool().ok_or("cache")?,
            integer(w, "target_index")? as usize,
            "replay",
        ),
        0,
        last,
    )?;
    let r = read(&root.join("runs").join(rlabel).join("worker.json"))?;
    require(
        r["schema"] == "n83.small-m4-general-replay/v1"
            && r["status"] == "PASS"
            && r["rank_lower_bound"] == 1
            && r["rows_checked"] == 4
            && r["changed_target_rejected"] == true
            && r["changed_point_rejected"] == true
            && r["changed_coefficient_rejected"] == true,
        "independent replay status",
    )?;
    for k in [
        "arm",
        "curve_id",
        "mode",
        "object_blake3",
        "public_corpus_sha256",
        "point_indices",
        "candidate_id",
        "selected_best_total_runtime",
    ] {
        require(r.get(k) == w.get(k), "replay identity")?;
    }
    require(
        r["worker_blake3"]
            == blake3::hash(&fs::read(
                root.join("runs").join(label).join("worker.json"),
            )?)
            .to_hex()
            .as_str(),
        "replay binding",
    )
}
fn audit(root: &Path, archive: &str, manifest_hash: &str) -> Result<Value> {
    let files = manifest(root, archive, manifest_hash)?;
    require(
        fs::read_to_string(root.join("host/card-status.txt"))?.trim() == "COMPLETE_BOUNDED",
        "incomplete card",
    )?;
    require(
        fs::read_to_string(root.join("host/source-commit.txt"))?.trim() == REV,
        "source revision",
    )?;
    for (p, h) in [
        (
            "run-xor-cache-card.sh",
            "9c9a07cca9ded5c4afe7db03217580c55c806c1c7b0ca120eedf0494721c1a4d",
        ),
        (
            "run-xor-cache-docker.sh",
            "f5c24df168febec056033ddb374d7316185c3dd180e83339aadafe00ae5d7439",
        ),
        ("public-targets.json", CORPUS),
    ] {
        require(
            sha(&root.join("host").join(p))? == h,
            "host source/input hash",
        )?;
    }
    let corpus = read(&root.join("host/public-targets.json"))?;
    let mut last = String::new();
    let mut dirs = BTreeSet::new();
    let mut controls = Vec::new();
    for arm in ["K2048", "D12"] {
        let mut pair = Vec::new();
        for cache in [false, true] {
            let label = format!("positive-{arm}-{}", if cache { "on" } else { "off" });
            dirs.insert(label.clone());
            container(root, &label, (arm, cache, 0, "positive"), 0, &mut last)?;
            let w = read(&root.join("runs").join(&label).join("worker.json"))?;
            full_identity(&w, arm, cache, 0, "positive")?;
            require(
                w["status"] == "PASS_CONSTRUCTED_PENDING_REPLAY"
                    && w["rank"] == 1
                    && w["group_verified"] == true
                    && w["sat_model_blake3"]
                        .as_str()
                        .is_some_and(|s| s.len() == 64),
                "constructed control",
            )?;
            replay(root, &label, &w, &mut last, &mut dirs)?;
            controls.push(w.clone());
            pair.push(w);
        }
        // Reuse the natural endpoint checks with the same off/on controls in ABBA order.
        paired(&[
            pair[0].clone(),
            pair[1].clone(),
            pair[1].clone(),
            pair[0].clone(),
        ])?;
    }
    let mut cells = Vec::new();
    let mut natural = Vec::new();
    let mut wall_caps = 0;
    for (arm, target) in [("K2048", 0), ("D12", 0), ("K2048", 1), ("D12", 1)] {
        let mut rows = Vec::new();
        for (suffix, cache) in [("a1", false), ("b1", true), ("b2", true), ("a2", false)] {
            let label = format!("{arm}-t{target}-{suffix}");
            dirs.insert(label.clone());
            let dir = root.join("runs").join(&label);
            let samples = jsonl(&dir.join("worker.json.progress.jsonl"))?;
            let capped = dir.join("worker.json.cap.json").exists();
            let mut w = if capped {
                wall_caps += 1;
                require(!dir.join("worker.json").exists(), "cap/final ambiguity")?;
                let cap = read(&dir.join("worker.json.cap.json"))?;
                core_identity(&cap, arm, cache, target, "natural")?;
                require(
                    cap["status"] == "UNKNOWN_budget" && number(&cap, "elapsed_ms")? >= 1790000.0,
                    "wall cap receipt",
                )?;
                let latest = samples.last().ok_or("no cap progress")?;
                let mut w = latest["identity"].clone();
                w["status"] = cap["status"].clone();
                w["process_wall_ms"] = cap["elapsed_ms"].clone();
                w["counters"] = latest["counters"].clone();
                w
            } else {
                read(&dir.join("worker.json"))?
            };
            full_identity(&w, arm, cache, target, "natural")?;
            require(
                w["target"] == corpus["points"][target] && w["positive_pin_ms"] == 0.0,
                "natural public point/pins",
            )?;
            for sample in &samples {
                full_identity(&sample["identity"], arm, cache, target, "natural")?;
            }
            let p = progress(&w, &samples, !capped)?;
            let c = container(
                root,
                &label,
                (arm, cache, target, "natural"),
                if capped { 124 } else { 0 },
                &mut last,
            )?;
            let phases = jsonl(&dir.join("worker.json.phases.jsonl"))?;
            let names: Vec<_> = phases
                .iter()
                .map(|p| p["phase"].as_str().unwrap_or(""))
                .collect();
            require(
                names.len() <= PHASES.len()
                    && names == PHASES[..names.len()]
                    && (capped || names.len() == PHASES.len()),
                "phase coverage/order",
            )?;
            let mut phase_time = 0.0;
            for phase in &phases {
                require(
                    phase["schema"] == "n83.small-m4-phase/v1"
                        && phase["arm"] == arm
                        && phase["mode"] == "natural",
                    "phase identity",
                )?;
                let t = number(phase, "elapsed_ms")?;
                require(t >= phase_time, "phase clock")?;
                phase_time = t;
            }
            if !capped {
                require(
                    phases.last().ok_or("missing phases")?["details"] == w,
                    "final phase binding",
                )?;
                require(
                    w["counters"]["phase"] == "Complete"
                        && number(&w, "memory_cgroup_peak_bytes")? <= 4294967296.0
                        && number(&w, "process_wall_ms")? < 1800000.0,
                    "final resources/counters",
                )?;
                match w["status"].as_str().ok_or("status")? {
                    "UNKNOWN_search_cap" => require(
                        w["counters"]["completed_conflicts"] == 10000,
                        "wrong conflict cap",
                    )?,
                    "UNVERIFIED_MODEL_UNSAT" => (),
                    "PASS_NATURAL_PENDING_REPLAY" => {
                        require(
                            w["group_verified"] == true
                                && integer(&w, "rank")? > 0
                                && w["sat_model_blake3"]
                                    .as_str()
                                    .is_some_and(|s| s.len() == 64),
                            "natural model/group/rank",
                        )?;
                        replay(root, &label, &w, &mut last, &mut dirs)?
                    }
                    _ => return Err("invalid natural outcome".into()),
                }
                require(
                    phases[5]["details"]["result"] == "Fixpoint"
                        && phases[7]["details"]["answer"]
                            == match w["status"].as_str().unwrap() {
                                "UNKNOWN_search_cap" => "Unknown",
                                "UNVERIFIED_MODEL_UNSAT" => "Unsat",
                                _ => "Sat",
                            },
                    "solver phase outcome",
                )?;
            }
            if w["status"] != "PASS_NATURAL_PENDING_REPLAY" {
                for k in [
                    "rank",
                    "sparse_row",
                    "point_indices",
                    "group_verified",
                    "sat_model_blake3",
                ] {
                    require(w.get(k).is_none(), "censored/unverified result promoted")?;
                }
            }
            rows.push(w.clone());
            w["label"] = json!(label);
            w["progress_audit"] = p;
            w["container"] = c;
            w["worker_or_cap_sha256"] = json!(sha(&dir.join(if capped {
                "worker.json.cap.json"
            } else {
                "worker.json"
            }))?);
            natural.push(w);
        }
        let mut comparison = paired(&rows)?;
        comparison["arm"] = json!(arm);
        comparison["target_index"] = json!(target);
        cells.push(comparison);
    }
    let actual: BTreeSet<String> = fs::read_dir(root.join("runs"))?
        .map(|e| {
            Ok(e?
                .file_name()
                .into_string()
                .map_err(|_| "non-UTF8 directory")?)
        })
        .collect::<Result<_>>()?;
    require(actual == dirs, "unexpected/missing run directory")?;
    Ok(
        json!({"schema":"n83.xor-cache-card-audit/v1","status":"PASS_RECEIPT_AUDIT","archive_sha256":archive,"manifest_sha256":manifest_hash,"manifest_files":files,
        "source_commit":REV,"worker_sha256":BIN,"replayer_sha256":REPLAYER,"host_isolation":"L0","controls":controls,"natural_attempts":natural,"comparisons":cells,
        "wall_capped_attempts":wall_caps,"candidate_id":null,"selected_best_total_runtime":null,"useful_rank_panel":null,
        "scope":"Receipt, provenance, progress, replay binding and matched search-work audit; timing observations are L0, natural caps are censored and complete IC cost remains open"}),
    )
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    require(
        args.len() == 5,
        "usage: n83_xor_cache_audit EXTRACTED_ROOT ARCHIVE_SHA256 MANIFEST_SHA256 NEW_OUTPUT",
    )?;
    let v = audit(Path::new(&args[1]), &args[2], &args[3])?;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[4])?;
    serde_json::to_writer_pretty(&mut f, &v)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rows() -> Vec<Value> {
        [false,true,true,false].into_iter().map(|cache| {
            let mut c = json!({}); for k in SAME_WORK { c[k] = json!(10000); }
            c["completed_xor_repivots"] = json!(if cache {20} else {100});
            json!({"status":"UNKNOWN_search_cap","xor_decided_cache":cache,"xor_decided_skips":if cache {80} else {0},"counters":c,"process_wall_ms":if cache {200.0} else {300.0}})
        }).collect()
    }
    #[test]
    fn paired_contract_rejects_semantic_and_work_drift() {
        let good = rows();
        assert_eq!(paired(&good).unwrap()["row_visit_ratio_off_over_on"], 5.0);
        for k in SAME_WORK {
            let mut bad = good.clone();
            bad[1]["counters"][k] = json!(9999);
            assert!(paired(&bad).is_err());
        }
        let mut bad = good.clone();
        bad[2]["xor_decided_skips"] = json!(79);
        assert!(paired(&bad).is_err());
        let mut bad = good.clone();
        bad[1]["sat_model_blake3"] = json!("different");
        assert!(paired(&bad).is_err());
        let mut cap = good;
        cap[1]["status"] = json!("UNKNOWN_budget");
        let result = paired(&cap).unwrap();
        assert_eq!(result["status"], "CENSORED_UNMATCHED_ENDPOINT");
        assert!(result["worker_ratio_off_over_on"].is_null());
    }
    #[test]
    fn partial_progress_cannot_be_promoted() {
        let mut c = json!({});
        for k in COUNTERS {
            c[k] = json!(1);
        }
        let w = json!({"arm":"K2048","process_wall_ms":20.0,"counters":c});
        let s = json!({"schema":"n83.small-m4-progress/v1","identity":{"arm":"K2048"},"final_sample":false,"elapsed_ms":10.0,"counters":c});
        assert!(progress(&w, std::slice::from_ref(&s), false).is_ok());
        assert!(progress(&w, &[s], true).is_err());
        for p in ["../secret", "/runs/a", "runs/../../secret", "other/a"] {
            assert!(safe_path(p).is_err());
        }
    }
}
