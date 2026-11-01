use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Write, path::Path, process::Command};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const HEADER: &str = "arm,instance_id,seed,replicate,warmup,verification,scalar,attempts,relations,curve_ms,bases_ms,oracle_ms,reduction_other_ms,replay_ms,complete_ms,oracle_calls,preparations,built_rows,probed_rows,query_inversions,table_bytes,digest,wall_seconds,tier";
pub const MANIFESTS: [&str; 5] = [
    "protocol.json",
    "build.json",
    "candidate-materialization.json",
    "previous-materialization.json",
    "inverse-materialization.json",
];

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn file_ref(path: &Path, label: &str) -> Result<Value> {
    let data = fs::read(path)?;
    Ok(json!({"path":label,"sha256":format!("{:x}",Sha256::digest(&data)),"bytes":data.len()}))
}
fn check(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

/// Package a complete six-arm panel. Runs the frozen native analyzer; never
/// trusts a retained PASS string and never overwrites an existing export.
/// The caller supplies the observation time as an RFC3339 UTC timestamp.
pub fn export(study: &Path, analyzer: &Path, created: &str) -> Result<()> {
    let out = study.join("results");
    check(!out.join("export.json").exists(), "export already exists")?;
    check(
        created.len() == 20 && created.ends_with('Z') && created.as_bytes()[10] == b'T',
        "expected UTC timestamp YYYY-MM-DDTHH:MM:SSZ",
    )?;
    let protocol = read_json(&study.join("protocol.json"))?;
    let build = read_json(&study.join("build.json"))?;
    check(
        format!("{:x}", Sha256::digest(fs::read(analyzer)?))
            == build["binaries"]["analyze"]
                .as_str()
                .ok_or("missing analyzer hash")?,
        "analyzer binary hash mismatch",
    )?;
    check(
        format!("{:x}", Sha256::digest(fs::read(study.join("analyze.rs"))?))
            == protocol["analysis_plan"]["script"]["sha256"]
                .as_str()
                .ok_or("missing analyzer source hash")?,
        "analyzer source hash mismatch",
    )?;
    for name in MANIFESTS {
        check(
            fs::read(out.join(name))? == fs::read(study.join(name))?,
            "returned manifest mismatch",
        )?;
    }
    check(
        fs::read_to_string(out.join("exit-code.txt"))?.trim() == "0",
        "worker failed",
    )?;
    let csv = fs::read_to_string(out.join("rows.csv"))?;
    let mut lines = csv.lines();
    check(lines.next() == Some(HEADER), "unexpected CSV header")?;
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split(',').collect()).collect();
    check(
        rows.len() == 180 && rows.iter().all(|row| row.len() == 24),
        "incomplete or malformed rows",
    )?;
    let result = Command::new(analyzer)
        .arg(out.join("rows.csv"))
        .arg(&out)
        .output()?;
    check(
        result.status.success(),
        &format!(
            "native analyzer rejected panel: {}",
            String::from_utf8_lossy(&result.stderr)
        ),
    )?;
    check(
        result.stdout.starts_with(
            b"PASS 180 rows, 162 measured, 18 complete cells, byte-identical full certificates",
        ),
        "unexpected analyzer output",
    )?;
    fs::write(out.join("analysis.log"), result.stdout)?;
    let template = read_json(
        &study
            .parent()
            .ok_or("missing study parent")?
            .join("panel2/results/export.json"),
    )?;
    let mut export = template.clone();
    export["export_id"] = protocol["protocol_id"].clone();
    export["created"] = json!(created);
    export["protocol"] = json!({"path":"../protocol.json","sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]});
    export["source"]["commit"] = protocol["arms"][1]["instrument"]["commit"].clone();
    export["arms"] = protocol["arms"].as_array().ok_or("missing arms")?.iter().map(|a| json!({"id":a["id"],"role":a["role"],"commit":a["instrument"]["commit"],"binary_sha256":a["instrument"]["binary_sha256"]})).collect();
    let round = study
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid study name")?;
    check(
        matches!(
            round,
            "round18" | "round19" | "round20" | "round21" | "round22" | "round23" | "round24"
        ),
        "unsupported study",
    )?;
    let job = format!("hashed-ecdl-cpu-{round}-20261030");
    let host_ref = file_ref(&out.join("cpuinfo.txt"), "cpuinfo.txt")?;
    for key in ["path", "sha256"] {
        export["host"][key] = host_ref[key].clone();
    }
    export["host"]
        .as_object_mut()
        .ok_or("invalid host")?
        .remove("bytes");
    export["host"]["class_id"] = json!(job);
    let cpuinfo = fs::read_to_string(out.join("cpuinfo.txt"))?;
    let model = cpuinfo
        .lines()
        .find(|s| s.starts_with("model name"))
        .and_then(|s| s.split_once(':'))
        .ok_or("missing CPU model")?
        .1
        .trim();
    export["host"]["summary"]["cpu_model"] = json!(model);
    export["host"]["summary"]["kernel"] = json!(fs::read_to_string(out.join("kernel.txt"))?.trim());
    let columns: Vec<_> = HEADER.split(',').collect();
    let mut runs = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let mut run = template["runs"][0].clone();
        run["run_id"] = json!(format!("row-{i}"));
        for (key, col) in [("arm", 0), ("instance_id", 1), ("seed", 2)] {
            run[key] = json!(row[col]);
        }
        run["replicate"] = json!(row[3].parse::<u64>()?);
        run["warmup"] = json!(row[4] == "true");
        run["backend_ref"] = json!(job);
        run["measurements"]["wall_seconds"] = json!(row[22].parse::<f64>()?);
        for category in ["counted", "counters"] {
            let map = run["measurements"][category]
                .as_object_mut()
                .ok_or("invalid measurements")?;
            for (key, value) in map.iter_mut() {
                let col = columns
                    .iter()
                    .position(|&s| s == key)
                    .ok_or("unknown metric")?;
                *value = if key.ends_with("_ms") {
                    json!(row[col].parse::<f64>()?)
                } else {
                    json!(row[col].parse::<u64>()?)
                };
            }
        }
        let cert = format!("cert-{}-{}-{}.txt", row[0], row[1], row[3]);
        run["artifacts"] = json!([file_ref(&out.join(&cert), &cert)?]);
        if round == "round22" && matches!(row[0], "y" | "z") {
            let name = format!("stages-{}-{}-{}.csv", row[0], row[1], row[3]);
            run["artifacts"]
                .as_array_mut()
                .ok_or("invalid artifacts")?
                .push(file_ref(&out.join(&name), &name)?);
            run["isolation"]["blockers"].as_array_mut().ok_or("invalid blockers")?
                .push(json!("Additional stage clocks perturb this L0-profile execution; not an ordinary timing row."));
            let raw = fs::read_to_string(out.join(name))?;
            let fields: Vec<_> = raw
                .lines()
                .nth(1)
                .ok_or("missing stage row")?
                .split(',')
                .collect();
            check(fields.len() == 13, "malformed stage row")?;
            for (column, name) in [
                "solve",
                "bases",
                "targets",
                "oracle",
                "linear",
                "internal_replay",
                "external_replay",
                "complete",
            ]
            .iter()
            .enumerate()
            {
                run["measurements"]["counters"][format!("stage_{name}_ms")] =
                    json!(fields[column + 4].parse::<f64>()?);
            }
        }
        runs.push(run);
    }
    export["runs"] = json!(runs);
    let mut paths: Vec<_> = fs::read_dir(&out)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<_>>()?;
    paths.sort();
    let mut files = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid file name")?;
        if path.is_file() && !matches!(name, "export.json" | "export-check.log") {
            let mut reference = file_ref(&path, name)?;
            reference["kind"] = json!("raw");
            files.push(reference);
        }
    }
    export["files"] = json!(files);
    export["flow"] = json!({"planned":180,"attempted":180,"completed":180,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]});
    export["analysis"]["script"] = file_ref(&study.join("analyze.rs"), "../analyze.rs")?;
    export["analysis"]["outputs"] = json!([
        file_ref(&out.join("summary.csv"), "summary.csv")?,
        file_ref(&out.join("latency.svg"), "latency.svg")?,
        file_ref(&out.join("analysis.log"), "analysis.log")?
    ]);
    if round == "round22" {
        export["analysis"]["outputs"]
            .as_array_mut()
            .ok_or("invalid outputs")?
            .push(file_ref(
                &out.join("stage-summary.csv"),
                "stage-summary.csv",
            )?);
    }
    export["notes"] = json!([
        {"kind":"observation","by":"native-panel-tools","text":"180 complete executions; 162 measured and 18 warmups. Frozen native analysis accepts full certificate equality across all six arms. Effective source identities, including any patched control, are retained in the materialization files. All full phases and both certificate replays remain charged."},
        {"kind":"observation","by":"native-panel-tools","text":"L0 diagnostics; wall_valid=false. Neutral machine labels do not establish human blinding."}
    ]);
    let mut data = serde_json::to_vec_pretty(&export)?;
    data.push(b'\n');
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("export.json"))?
        .write_all(&data)?;
    Ok(())
}
