//! Freeze and independently audit the complete D18 shared-root command card.
//! Container receipts and fresh general-field replays bind every accepted row.
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const CORPUS: &str = "2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5";
const IMAGE: &str =
    "amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f";
const IMAGE_ID: &str = "sha256:24b8580b081b53113046267a47ad12858ad97752c9735e1125fcdb7a62d1d2c9";
const INPUTS: &str = "/home/ec2-user/n83-m23-20261019/inputs";
const A: &str = "per-representative";
const B: &str = "shared-bits";
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
fn b3(p: &Path) -> Result<String> {
    Ok(blake3::hash(&fs::read(p)?).to_hex().to_string())
}
fn text(p: &Path) -> Result<String> {
    Ok(fs::read_to_string(p)?.trim().to_owned())
}
fn n(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing integer {k}").into())
}
fn f(v: &Value, k: &str) -> Result<f64> {
    let x = v[k].as_f64().ok_or_else(|| format!("missing number {k}"))?;
    require(x.is_finite() && x >= 0., "nonfinite/negative metric")?;
    Ok(x)
}
fn write_new(p: &Path, v: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(p)?;
    serde_json::to_writer_pretty(&mut file, v)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn safe(p: &str) -> bool {
    !p.is_empty()
        && Path::new(p)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn inventory(root: &Path, dir: &Path, out: &mut BTreeSet<String>) -> Result<()> {
    require(
        !fs::symlink_metadata(dir)?.is_symlink(),
        "symlink directory",
    )?;
    for e in fs::read_dir(dir)? {
        let e = e?;
        let t = e.file_type()?;
        if t.is_dir() {
            inventory(root, &e.path(), out)?;
        } else {
            require(t.is_file(), "symlink/non-file evidence")?;
            out.insert(
                e.path()
                    .strip_prefix(root)?
                    .to_str()
                    .ok_or("UTF8 path")?
                    .to_owned(),
            );
        }
    }
    Ok(())
}
fn manifest(root: &Path, archive: &str, expected: &str) -> Result<usize> {
    require(digest(archive) && digest(expected), "digest syntax")?;
    require(
        sha(&root.join("runs.tar.xz"))? == archive && sha(&root.join("files.sha256"))? == expected,
        "archive/manifest digest",
    )?;
    let mut listed = BTreeSet::new();
    for line in fs::read_to_string(root.join("files.sha256"))?.lines() {
        let (h, p) = line.split_once("  ").ok_or("manifest syntax")?;
        require(
            digest(h)
                && safe(p)
                && (p.starts_with("host/") || p.starts_with("runs/"))
                && listed.insert(p.to_owned()),
            "unsafe/duplicate manifest entry",
        )?;
        require(sha(&root.join(p))? == h, "manifest file digest")?;
    }
    let mut actual = BTreeSet::new();
    for d in ["host", "runs"] {
        inventory(root, &root.join(d), &mut actual)?;
    }
    require(actual == listed, "missing/extra archive file")?;
    Ok(actual.len())
}

// These are orchestration records only. Their digest is committed before launch.
fn freeze(root: &Path, source: &Path) -> Result<Value> {
    require(!root.join("runs").exists(), "card already consumed")?;
    require(
        text(&root.join("prepare-status.txt"))? == "PREPARED_NOT_LAUNCHED",
        "unprepared card",
    )?;
    let revision = text(&root.join("source-commit.txt"))?;
    require(
        revision.len() == 40 && revision.bytes().all(|b| b.is_ascii_hexdigit()),
        "revision",
    )?;
    let mut files = serde_json::Map::new();
    for p in [
        "run-card.sh",
        "run-docker.sh",
        "prepare-card.sh",
        "construction.json",
        "source-commit.txt",
        "inputs.sha256",
        "build/lib-exit.txt",
        "build/tests-exit.txt",
        "build/build-exit.txt",
        "build/proof-exit.txt",
        "proof/SharedRootBits.lean",
        "proof/lean.log",
        "construction-run/container-inspect.json",
        "construction-run/exit.txt",
        "construction-run/binary.sha256",
        "prelaunch-image.json",
    ] {
        files.insert(p.into(), json!(sha(&root.join(p))?));
    }
    for gate in ["lib", "tests", "build", "proof"] {
        require(
            text(&root.join(format!("build/{gate}-exit.txt")))? == "0",
            "native/proof gate failed",
        )?;
    }
    let construction = read(&root.join("construction.json"))?;
    require(
        construction["status"] == "PASS"
            && construction["columns"] == 16627
            && construction["rows"]
                .as_array()
                .is_some_and(|r| r.len() == 6),
        "construction gate",
    )?;
    let module = source.join("src/cryptanalysis/binary_semaev_chain_sat.rs");
    require(
        construction["source_module_sha256"] == sha(&module)?,
        "construction/source binding",
    )?;
    Ok(
        json!({"schema":"n83.d18-shared-host-card/v1", "source_commit":revision,
        "remote_root":root.canonicalize()?.to_str().ok_or("UTF8 root")?,
        "worker_sha256":sha(&root.join("worker"))?, "replayer_sha256":sha(&root.join("replayer"))?,
        "gate_sha256":sha(&root.join("gate"))?, "source_encoding_sha256":sha(&module)?,
        "source_module_blake3":b3(&source.join("research/n83_prebuild_cost_m23_20261019/small_m4.rs"))?,
        "source_worker_blake3":b3(&source.join("examples/n83_prebuild_cost_m23.rs"))?,
        "files_sha256":files, "public_corpus_sha256":CORPUS, "curve_id":CURVE,
        "controls":22,"control_replays":22,"natural_queries":18,
        "isolation":"L0 until an independent uncontended receipt qualifies timing",
        "candidate_id":null,"selected_best_total_runtime":null}),
    )
}

#[derive(Clone, Debug)]
struct Run {
    label: String,
    encoding: &'static str,
    arm: &'static str,
    target: usize,
    mode: &'static str,
}
fn controls() -> Vec<(String, String, Run, Run)> {
    let mut pairs = Vec::new();
    for kind in ["aa", "ab"] {
        for i in 0..5 {
            let left = format!("{kind}-{i}-{}", if kind == "aa" { "a1" } else { "a" });
            let right = format!("{kind}-{i}-{}", if kind == "aa" { "a2" } else { "b" });
            pairs.push((
                kind.into(),
                format!("{kind}-{i}"),
                Run {
                    label: left,
                    encoding: A,
                    arm: "D",
                    target: 0,
                    mode: "positive",
                },
                Run {
                    label: right,
                    encoding: if kind == "aa" { A } else { B },
                    arm: "D",
                    target: 0,
                    mode: "positive",
                },
            ));
        }
    }
    pairs.push((
        "ab".into(),
        "u".into(),
        Run {
            label: "u-a".into(),
            encoding: A,
            arm: "U",
            target: 0,
            mode: "positive",
        },
        Run {
            label: "u-b".into(),
            encoding: B,
            arm: "U",
            target: 0,
            mode: "positive",
        },
    ));
    pairs
}
fn schedule() -> Vec<Run> {
    let mut out = vec![];
    for (label, encoding) in [("t0-a1", A), ("t0-b1", B), ("t0-b2", B), ("t0-a2", A)] {
        out.push(Run {
            label: label.into(),
            encoding,
            arm: "D",
            target: 0,
            mode: "natural",
        });
    }
    for target in 1..8 {
        for encoding in if target % 2 == 1 { [B, A] } else { [A, B] } {
            out.push(Run {
                label: format!("t{target}-{encoding}"),
                encoding,
                arm: "D",
                target,
                mode: "natural",
            });
        }
    }
    out
}
fn identity(w: &Value, card: &Value, s: &Run, construction: &Value, full: bool) -> Result<()> {
    require(
        w["schema"] == "n83.small-m4-worker/v1"
            && w["curve_id"] == CURVE
            && w["source_commit"] == card["source_commit"]
            && w["binary_sha256"] == card["worker_sha256"]
            && w["arm"] == s.arm
            && w["mode"] == s.mode
            && w["m"] == 4
            && w["target_index"] == s.target
            && w["experiment_policy"] == "d18-shared-root-m4/v1"
            && w["root_encoding"] == s.encoding
            && w["xor_decided_cache"] == true
            && w["conflict_budget"] == 10000,
        "worker core identity",
    )?;
    for k in [
        "source_module_blake3",
        "source_worker_blake3",
        "source_encoding_sha256",
    ] {
        require(
            w.get(k).is_some() && w[k] == card[k],
            "worker source binding",
        )?;
    }
    for k in ["candidate_id", "selected_best_total_runtime"] {
        require(w.get(k) == Some(&Value::Null), "unsupported promotion")?;
    }
    if !full {
        return Ok(());
    }
    let (object, set, labels) = if s.arm == "D" {
        (
            "e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
            "8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0",
            "805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c",
        )
    } else {
        (
            "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d",
            "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0",
            "197c3fd68b84651806cadf54f0b49a75d93d83579476dbabadb36e1486ff34e6",
        )
    };
    let shared = n(w, "root_shared_bit_count")?;
    require(shared < 83, "shared coordinate count")?;
    let removed = if s.encoding == B {
        4 * 16626 * shared
    } else {
        0
    };
    require(
        w["columns"] == 16627
            && w["points"] == 2760082
            && w["object_blake3"] == object
            && w["point_set_blake3"] == set
            && w["labeled_rows_blake3"] == labels
            && w["sat_variables"] == 201598
            && w["sat_xor_rows"] == 3901
            && w["sat_clauses"] == 5915238 - removed
            && w["public_corpus_sha256"] == CORPUS,
        "base/model/corpus identity",
    )?;
    if s.arm == "D" {
        require(
            shared == 66
                && w["root_representatives_blake3"] == construction["root_representatives_blake3"],
            "D18 root inventory",
        )?;
    }
    Ok(())
}

fn container(root: &Path, card: &Value, s: &Run, code: u64, last: &mut String) -> Result<Value> {
    let dir = root.join("runs").join(&s.label);
    require(
        text(&dir.join("host-exit.txt"))? == code.to_string(),
        "host exit",
    )?;
    let a = read(&dir.join("container-inspect.json"))?;
    require(
        a.as_array().is_some_and(|v| v.len() == 1),
        "container count",
    )?;
    let c = &a[0];
    let h = &c["HostConfig"];
    let state = &c["State"];
    require(
        state["Status"] == "exited"
            && state["Running"] == false
            && state["OOMKilled"] == false
            && state["Error"] == ""
            && state["ExitCode"] == code,
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
        "container limits",
    )?;
    require(
        c["Config"]["Image"] == IMAGE && c["Image"] == IMAGE_ID,
        "image",
    )?;
    let image = read(&dir.join("image-inspect.json"))?;
    require(
        image[0]["Id"] == IMAGE_ID && image[0]["Architecture"] == "amd64",
        "image architecture",
    )?;
    let replay = s.mode == "replay";
    let bin = if replay { "replayer" } else { "worker" };
    require(
        fs::read_to_string(dir.join("binary.sha256"))?
            .split_whitespace()
            .next()
            == card[format!("{bin}_sha256")].as_str(),
        "binary hash",
    )?;
    let mut args = vec![
        "-s".into(),
        "TERM".into(),
        "-k".into(),
        "5".into(),
        "1800".into(),
        "/opt/worker".into(),
    ];
    if replay {
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
            "shared-root-m4".into(),
            s.encoding.into(),
            s.mode.into(),
            s.arm.into(),
            "/inputs/base".into(),
            "/inputs/public.json".into(),
            s.target.to_string(),
            "/out/worker.json".into(),
        ]);
    }
    require(
        c["Path"] == "/usr/bin/timeout" && c["Args"] == json!(args),
        "container command",
    )?;
    let revision = card["source_commit"].as_str().ok_or("revision")?;
    require(
        c["Config"]["Env"]
            .as_array()
            .is_some_and(|a| a.contains(&json!(format!("N83_SOURCE_COMMIT={revision}")))),
        "container revision",
    )?;
    let remote = card["remote_root"].as_str().ok_or("remote root")?;
    let mut mounts = BTreeSet::new();
    for m in c["Mounts"].as_array().ok_or("mounts")? {
        let dest = m["Destination"].as_str().ok_or("destination")?;
        let src = match dest {
            "/out" => format!("{remote}/runs/{}", s.label),
            "/opt/worker" => format!("{remote}/{bin}"),
            "/inputs/base" => format!("{INPUTS}/{}", s.arm),
            "/inputs/public.json" => format!("{INPUTS}/public/public-targets.json"),
            "/inputs/receipt.json" if replay => format!(
                "{remote}/runs/{}/worker.json",
                s.label.strip_suffix("-replay").ok_or("replay label")?
            ),
            _ => return Err("unexpected mounted input".into()),
        };
        require(
            m["Type"] == "bind"
                && m["RW"] == (dest == "/out")
                && m["Source"] == src
                && mounts.insert(dest),
            "mount source/access",
        )?;
    }
    require(mounts.len() == if replay { 5 } else { 4 }, "mount coverage")?;
    let start = state["StartedAt"].as_str().ok_or("start")?;
    let end = state["FinishedAt"].as_str().ok_or("end")?;
    require(start >= last.as_str() && end > start, "overlap/order")?;
    *last = end.to_owned();
    Ok(json!({"started":start,"finished":end,"exit":code}))
}

fn replay(
    root: &Path,
    card: &Value,
    s: &Run,
    w: &Value,
    last: &mut String,
    dirs: &mut BTreeSet<String>,
) -> Result<Value> {
    let mut r = s.clone();
    r.mode = "replay";
    r.label.push_str("-replay");
    dirs.insert(r.label.clone());
    container(root, card, &r, 0, last)?;
    let v = read(&root.join("runs").join(&r.label).join("worker.json"))?;
    require(
        v["schema"] == "n83.small-m4-general-replay/v1"
            && v["status"] == "PASS"
            && v["rank_lower_bound"] == 1
            && v["rows_checked"] == 4
            && v["changed_target_rejected"] == true
            && v["changed_point_rejected"] == true
            && v["changed_coefficient_rejected"] == true
            && v["worker_blake3"] == b3(&root.join("runs").join(&s.label).join("worker.json"))?,
        "independent replay binding/status",
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
        require(
            v.get(k).is_some() && v.get(k) == w.get(k),
            "replay identity",
        )?;
    }
    Ok(v)
}

fn jsonl(p: &Path) -> Result<Vec<Value>> {
    let bytes = fs::read_to_string(p)?;
    require(bytes.ends_with('\n'), "truncated JSONL")?;
    bytes
        .lines()
        .map(|s| Ok(serde_json::from_str(s)?))
        .collect()
}
fn progress(w: &Value, samples: &[Value], capped: bool) -> Result<Value> {
    require(!samples.is_empty(), "missing progress")?;
    let mut time = 0.;
    let mut cpu = 0.;
    let mut rss = 0;
    let mut counters = [0; 8];
    let mut gap = 0f64;
    for (i, s) in samples.iter().enumerate() {
        require(
            s["schema"] == "n83.small-m4-progress/v1"
                && s["final_sample"] == (!capped && i + 1 == samples.len()),
            "progress schema/finality",
        )?;
        for (k, v) in s["identity"].as_object().ok_or("identity")? {
            require(w.get(k) == Some(v), "progress identity drift")?;
        }
        let next = f(s, "elapsed_ms")?;
        require(
            next >= time
                && next <= f(w, "process_wall_ms")?
                && f(s, "cpu_seconds")? >= cpu
                && n(s, "peak_rss_bytes")? >= rss,
            "progress resources/clock",
        )?;
        gap = gap.max(next - time);
        time = next;
        cpu = f(s, "cpu_seconds")?;
        rss = n(s, "peak_rss_bytes")?;
        for (j, k) in COUNTERS.iter().enumerate() {
            let x = n(&s["counters"], k)?;
            require(
                x >= counters[j] && x <= n(&w["counters"], k)?,
                "counter decrease/overshoot",
            )?;
            counters[j] = x;
        }
    }
    require(
        samples.last().unwrap()["counters"] == w["counters"],
        "last counters binding",
    )?;
    Ok(
        json!({"samples":samples.len(),"max_gap_ms":gap,"unobserved_tail_ms":f(w,"process_wall_ms")?-time,
        "last_sample_cpu_seconds":cpu,"last_sample_peak_rss_bytes":rss,"capped_resource_scope":"Lower bounds when no final sample exists"}),
    )
}

fn phases(w: &Value, records: &[Value], capped: bool) -> Result<()> {
    let names = [
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
    require(
        !records.is_empty()
            && records.len() <= names.len()
            && (capped || records.len() == names.len()),
        "phase coverage",
    )?;
    let mut time = 0.;
    for (i, p) in records.iter().enumerate() {
        require(
            p["schema"] == "n83.small-m4-phase/v1"
                && p["phase"] == names[i]
                && p["arm"] == "D"
                && p["mode"] == "natural",
            "phase identity/order",
        )?;
        let next = f(p, "elapsed_ms")?;
        // The final phase is written after the worker interval was sampled.
        // Preserve that serialization tail instead of requiring it to be zero.
        require(next >= time, "phase clock")?;
        time = next;
    }
    if !capped {
        require(
            records.last().unwrap()["details"] == *w,
            "final phase binding",
        )?;
        let answer = match w["status"].as_str() {
            Some("UNKNOWN_search_cap") => "Unknown",
            Some("UNVERIFIED_MODEL_UNSAT") => "Unsat",
            Some("PASS_NATURAL_PENDING_REPLAY") => "Sat",
            _ => return Err("unexpected final phase status".into()),
        };
        require(
            records[7]["details"]["answer"] == answer
                && (records[5]["details"]["result"] == "Fixpoint"
                    || (answer == "Unsat" && records[5]["details"]["result"] == "Unsat")),
            "solver phase outcome",
        )?;
    }
    Ok(())
}

fn rank(rows: &[Value], encoding: &str) -> Result<Value> {
    let order: BigUint = "2417851639230796216685689".parse()?;
    let mut pivots: BTreeMap<usize, BTreeMap<usize, BigUint>> = BTreeMap::new();
    let mut accepted = 0;
    for w in rows
        .iter()
        .filter(|w| w["root_encoding"] == encoding && w["status"] == "PASS_NATURAL_PENDING_REPLAY")
    {
        require(
            w["mode"] == "natural" && w["fresh_replay_checked"] == true,
            "unreplayed/control row in rank",
        )?;
        let mut row = BTreeMap::new();
        for pair in w["sparse_row"].as_array().ok_or("sparse row")? {
            require(pair.as_array().is_some_and(|a| a.len() == 2), "pair shape")?;
            let col = pair[0].as_u64().ok_or("column")? as usize;
            let value: BigUint = pair[1].as_str().ok_or("coefficient")?.parse()?;
            require(
                col < 16627
                    && value > BigUint::from(0u8)
                    && value < order
                    && !row.contains_key(&col),
                "coefficient/column",
            )?;
            row.insert(col, value);
        }
        require(!row.is_empty(), "empty accepted row")?;
        accepted += 1;
        while let Some((&pivot, value)) = row.first_key_value() {
            let value = value.clone();
            if let Some(existing) = pivots.get(&pivot) {
                for (&col, v) in existing {
                    let next = (row.remove(&col).unwrap_or_default() + &order
                        - (&value * v) % &order)
                        % &order;
                    if next != BigUint::from(0u8) {
                        row.insert(col, next);
                    }
                }
            } else {
                let inverse = value.modpow(&(&order - 2u8), &order);
                require(
                    (&value * &inverse) % &order == BigUint::from(1u8),
                    "noninvertible pivot",
                )?;
                for v in row.values_mut() {
                    *v = (&*v * &inverse) % &order;
                }
                pivots.insert(pivot, row);
                break;
            }
        }
    }
    Ok(
        json!({"accepted_replayed_rows":accepted,"retained_factor_row_rank":pivots.len(),"natural_yield_estimate":null,"complete_ic_rank":null}),
    )
}

fn audit(root: &Path, card: &Value, archive: &str, manifest_hash: &str) -> Result<Value> {
    let count = manifest(root, archive, manifest_hash)?;
    require(
        card["schema"] == "n83.d18-shared-host-card/v1"
            && card["curve_id"] == CURVE
            && card["public_corpus_sha256"] == CORPUS,
        "host card schema/identity",
    )?;
    for (p, h) in card["files_sha256"].as_object().ok_or("card file hashes")? {
        require(
            safe(p) && h.as_str().is_some_and(digest),
            "card file path/hash",
        )?;
        require(
            sha(&root.join("host").join(p))? == h.as_str().unwrap(),
            "frozen host file changed",
        )?;
    }
    require(
        text(&root.join("host/card-status.txt"))? == "COMPLETE_BOUNDED",
        "incomplete card",
    )?;
    require(
        sha(&root.join("host/public-targets.json"))? == CORPUS,
        "corpus hash",
    )?;
    let corpus = read(&root.join("host/public-targets.json"))?;
    let construction = read(&root.join("host/construction.json"))?;
    let mut last = String::new();
    let mut dirs = BTreeSet::new();
    let mut control_rows = Vec::new();
    for (kind, label, a, b) in controls() {
        let mut pair = Vec::new();
        let mut hashes = Vec::new();
        let mut replay_hashes = Vec::new();
        for s in [&a, &b] {
            dirs.insert(s.label.clone());
            container(root, card, s, 0, &mut last)?;
            let path = root.join("runs").join(&s.label).join("worker.json");
            let w = read(&path)?;
            identity(&w, card, s, &construction, true)?;
            require(
                w["status"] == "PASS_CONSTRUCTED_PENDING_REPLAY"
                    && w["group_verified"] == true
                    && w["rank"] == 1
                    && w["counters"]["phase"] == "Complete",
                "constructed control",
            )?;
            replay(root, card, s, &w, &mut last, &mut dirs)?;
            hashes.push(b3(&path)?);
            replay_hashes.push(b3(&root
                .join("runs")
                .join(format!("{}-replay", s.label))
                .join("worker.json"))?);
            pair.push(w.clone());
            control_rows.push(w);
        }
        for k in [
            "target",
            "sat_model_blake3",
            "sparse_row",
            "sparse_row_blake3",
            "point_indices",
            "root_representatives_blake3",
            "root_shared_bit_count",
        ] {
            require(
                pair[0].get(k).is_some_and(|v| !v.is_null()) && pair[0][k] == pair[1][k],
                "control assignment/relation mismatch",
            )?;
        }
        hashes.extend(replay_hashes);
        let gate = read(&root.join("host").join(format!("pair-{label}.json")))?;
        require(
            gate["schema"] == "n83.shared-root-control-pair/v1"
                && gate["status"] == "PASS"
                && gate["comparison"] == kind
                && gate["arm"] == a.arm
                && gate["input_blake3"] == json!(hashes)
                && gate["matched_assignment_and_relation"] == true,
            "pair gate binding",
        )?;
    }
    let mut natural = Vec::new();
    for s in schedule() {
        dirs.insert(s.label.clone());
        let dir = root.join("runs").join(&s.label);
        let samples = jsonl(&dir.join("worker.json.progress.jsonl"))?;
        let capped = dir.join("worker.json.cap.json").exists();
        let mut w = if capped {
            require(!dir.join("worker.json").exists(), "cap/final ambiguity")?;
            let cap = read(&dir.join("worker.json.cap.json"))?;
            identity(&cap, card, &s, &construction, false)?;
            require(
                cap["status"] == "UNKNOWN_budget"
                    && (1790000.0..1800000.0).contains(&f(&cap, "elapsed_ms")?),
                "wall cap",
            )?;
            let latest = samples.last().ok_or("missing cap progress")?;
            let mut v = latest["identity"].clone();
            v["status"] = cap["status"].clone();
            v["process_wall_ms"] = cap["elapsed_ms"].clone();
            v["counters"] = latest["counters"].clone();
            v
        } else {
            read(&dir.join("worker.json"))?
        };
        identity(&w, card, &s, &construction, true)?;
        require(
            w["target"] == corpus["points"][s.target] && w["positive_pin_ms"] == 0.,
            "natural target/pins",
        )?;
        for p in &samples {
            identity(&p["identity"], card, &s, &construction, true)?;
        }
        let p = progress(&w, &samples, capped)?;
        let c = container(root, card, &s, if capped { 124 } else { 0 }, &mut last)?;
        phases(&w, &jsonl(&dir.join("worker.json.phases.jsonl"))?, capped)?;
        if !capped {
            require(
                f(&w, "memory_cgroup_peak_bytes")? <= 4294967296.
                    && f(&w, "process_wall_ms")? < 1800000.,
                "final resource cap",
            )?;
            match w["status"].as_str().ok_or("natural status")? {
                "UNKNOWN_search_cap" => require(
                    w["counters"]["completed_conflicts"] == 10000,
                    "conflict cap",
                )?,
                "UNVERIFIED_MODEL_UNSAT" => (),
                "PASS_NATURAL_PENDING_REPLAY" => {
                    require(
                        w["group_verified"] == true && w["rank"] == 1,
                        "natural verification",
                    )?;
                    replay(root, card, &s, &w, &mut last, &mut dirs)?;
                    w["fresh_replay_checked"] = json!(true);
                }
                _ => return Err("unexpected natural result".into()),
            }
        }
        if w["status"] != "PASS_NATURAL_PENDING_REPLAY" {
            for k in [
                "rank",
                "sparse_row",
                "point_indices",
                "group_verified",
                "sat_model_blake3",
            ] {
                require(w.get(k).is_none(), "censor/unverified result promoted")?;
            }
        }
        w["label"] = json!(s.label);
        w["progress_audit"] = p;
        w["container"] = c;
        natural.push(w);
    }
    let actual: BTreeSet<String> = fs::read_dir(root.join("runs"))?
        .map(|e| Ok(e?.file_name().into_string().map_err(|_| "UTF8 directory")?))
        .collect::<Result<_>>()?;
    require(actual == dirs, "extra/missing run directory")?;
    let mut summaries = serde_json::Map::new();
    for encoding in [A, B] {
        let rows: Vec<_> = natural
            .iter()
            .filter(|w| w["root_encoding"] == encoding)
            .collect();
        let mut outcomes = BTreeMap::new();
        let mut ms = 0.;
        for w in &rows {
            *outcomes
                .entry(w["status"].as_str().ok_or("status")?)
                .or_insert(0u64) += 1;
            ms += f(w, "process_wall_ms")?;
        }
        summaries.insert(encoding.into(),json!({"attempts":rows.len(),"outcomes":outcomes,"charged_worker_intervals_s":ms/1000.,"rank":rank(&natural,encoding)?}));
    }
    Ok(
        json!({"schema":"n83.d18-shared-card-audit/v1","status":"PASS_RECEIPT_AUDIT",
        "archive_sha256":archive,"manifest_sha256":manifest_hash,"manifest_files":count,
        "host_card":card,"host_isolation":"L0","controls":control_rows,"natural_attempts":natural,"encoding_summaries":summaries,
        "candidate_id":null,"selected_best_total_runtime":null,"timing_claim":null,"useful_rank_panel":null,
        "scope":"22 constructed controls and fresh replays; 18 natural attempts with all censors. Ranks use only fresh-replayed natural rows, separately per encoding. Worker intervals retain failures and exclude setup, final serialization, container overhead and replay. No runtime ratio or complete IC cost is inferred."}),
    )
}

fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    let (value,out)=match a.get(1).map(String::as_str) {
        Some("freeze") if a.len()==5=>(freeze(Path::new(&a[2]),Path::new(&a[3]))?,&a[4]),
        Some("audit") if a.len()==8=>{
            require(sha(Path::new(&a[3]))?==a[4],"host card digest")?;
            (audit(Path::new(&a[2]),&read(Path::new(&a[3]))?,&a[5],&a[6])?,&a[7])
        },
        _=>return Err("usage: freeze PREPARED SOURCE NEW_CARD | audit EXTRACTED CARD CARD_SHA256 ARCHIVE_SHA256 MANIFEST_SHA256 NEW_OUTPUT".into()),
    };
    write_new(Path::new(out), &value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_schedule_has_22_controls_22_replays_and_18_queries() {
        let controls = controls();
        assert_eq!(controls.len(), 11);
        assert_eq!(controls.iter().filter(|p| p.0 == "aa").count(), 5);
        let runs = schedule();
        assert_eq!(runs.len(), 18);
        assert_eq!(
            runs.iter().take(4).map(|r| r.encoding).collect::<Vec<_>>(),
            [A, B, B, A]
        );
        let labels: BTreeSet<_> = runs.iter().map(|r| &r.label).collect();
        assert_eq!(labels.len(), 18);
        for t in 1..8 {
            assert_eq!(runs.iter().filter(|r| r.target == t).count(), 2);
        }
    }
    #[test]
    fn rank_rejects_controls_unreplayed_rows_and_keeps_encodings_separate() {
        let row = json!({"root_encoding":A,"status":"PASS_NATURAL_PENDING_REPLAY","mode":"natural","fresh_replay_checked":true,"sparse_row":[[2,"3"],[8,"4"]]});
        let mut twice = row.clone();
        twice["sparse_row"] = json!([[2, "6"], [8, "8"]]);
        assert_eq!(
            rank(&[row.clone(), twice], A).unwrap()["retained_factor_row_rank"],
            1
        );
        assert_eq!(
            rank(std::slice::from_ref(&row), B).unwrap()["retained_factor_row_rank"],
            0
        );
        let mut bad = row.clone();
        bad["fresh_replay_checked"] = json!(false);
        assert!(rank(&[bad], A).is_err());
        let mut bad = row;
        bad["mode"] = json!("positive");
        assert!(rank(&[bad], A).is_err());
    }
    #[test]
    fn manifest_paths_and_digests_fail_closed() {
        for p in ["../host/x", "/host/x", "host/../../x", ""] {
            assert!(!safe(p));
        }
        assert!(safe("host/build/lib-exit.txt"));
        assert!(digest(&"a".repeat(64)));
        assert!(!digest(&"A".repeat(64)));
        assert!(!digest(&"g".repeat(64)));
    }

    #[test]
    fn progress_rejects_counter_regression_and_false_finality() {
        let mut counts = serde_json::Map::new();
        for k in COUNTERS {
            counts.insert(k.into(), json!(3));
        }
        let worker = json!({"process_wall_ms":30.,"counters":counts,"arm":"D"});
        let sample = json!({"schema":"n83.small-m4-progress/v1","identity":{"arm":"D"},
            "elapsed_ms":10.,"cpu_seconds":0.001,"peak_rss_bytes":4096,
            "counters":counts,"final_sample":false});
        let mut final_sample = sample.clone();
        final_sample["elapsed_ms"] = json!(20.);
        final_sample["final_sample"] = json!(true);
        assert!(progress(&worker, &[sample.clone(), final_sample.clone()], false).is_ok());
        assert!(progress(&worker, &[sample.clone(), final_sample.clone()], true).is_err());
        final_sample["counters"]["completed_conflicts"] = json!(2);
        assert!(progress(&worker, &[sample, final_sample], false).is_err());
    }

    #[test]
    fn identity_rejects_encoding_and_root_inventory_drift() {
        let construction = json!({"root_representatives_blake3":"a".repeat(64)});
        let card = json!({"source_commit":"1".repeat(40),"worker_sha256":"2".repeat(64),
            "source_encoding_sha256":"3".repeat(64),"source_module_blake3":"4".repeat(64),"source_worker_blake3":"5".repeat(64)});
        let s = schedule()[1].clone();
        let mut w = json!({"schema":"n83.small-m4-worker/v1","curve_id":CURVE,
            "arm":"D","mode":"natural","m":4,"target_index":0,
            "experiment_policy":"d18-shared-root-m4/v1","root_encoding":B,
            "xor_decided_cache":true,"conflict_budget":10000,"root_shared_bit_count":66,
            "columns":16627,"points":2760082,"sat_variables":201598,"sat_clauses":1525974,"sat_xor_rows":3901,
            "object_blake3":"e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682",
            "point_set_blake3":"8c1c245cda15bdd0019b487a5535a3b7eac642c259c4dc0bae25875d87d345b0",
            "labeled_rows_blake3":"805d1cbc65cac0a74ebf49315b8c17b0d5229bdceba5725ba4349fb1cc6c9e9c",
            "root_representatives_blake3":"a".repeat(64),"public_corpus_sha256":CORPUS,
            "candidate_id":null,"selected_best_total_runtime":null});
        for k in [
            "source_commit",
            "source_encoding_sha256",
            "source_module_blake3",
            "source_worker_blake3",
        ] {
            w[k] = card[k].clone();
        }
        w["binary_sha256"] = card["worker_sha256"].clone();
        assert!(identity(&w, &card, &s, &construction, true).is_ok());
        for (k, v) in [
            ("root_encoding", json!(A)),
            ("root_shared_bit_count", json!(65)),
            ("sat_clauses", json!(5915238)),
            ("root_representatives_blake3", json!("b".repeat(64))),
            ("source_encoding_sha256", json!("6".repeat(64))),
            ("candidate_id", json!("unadmitted")),
        ] {
            let mut bad = w.clone();
            bad[k] = v;
            assert!(
                identity(&bad, &card, &s, &construction, true).is_err(),
                "{k}"
            );
        }
    }
}
