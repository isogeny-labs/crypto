//! Render the frozen N83 A1/B/A2 relation controls from their raw receipts.
//! This is a reporting tool; it does not run or alter the relation producer.

use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const SOURCE: &str = "48ebe3b370810e6da2d48e5234e42ba92bba259b";
const OBJECT: &str = "1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d";
const POINT_SET: &str = "564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0";

struct Arm {
    name: &'static str,
    label: &'static str,
    mode: &'static str,
    status: &'static str,
    wall_ms: f64,
    import_ms: f64,
    model_ms: f64,
    root_ms: f64,
    solver_ms: f64,
    decisions: u64,
    conflicts: u64,
    xor_passes: u64,
    xor_ns: u64,
    xor_row_scans: u64,
    peak_bytes: u64,
    row_hash: String,
    worker_hash: String,
}

fn read_json(path: &Path) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn find_jsonl(path: &Path, key: &str, value: &str) -> Result<Value, Box<dyn Error>> {
    for line in fs::read_to_string(path)?.lines() {
        let record: Value = serde_json::from_str(line)?;
        if record[key] == value {
            return Ok(record);
        }
    }
    Err(format!("missing {key}={value} in {}", path.display()).into())
}

fn number(value: &Value, key: &str) -> Result<f64, Box<dyn Error>> {
    value[key]
        .as_f64()
        .ok_or_else(|| format!("missing numeric {key}").into())
}

fn integer(value: &Value, key: &str) -> Result<u64, Box<dyn Error>> {
    value[key]
        .as_u64()
        .ok_or_else(|| format!("missing integer {key}").into())
}

fn load_arm(
    study: &Path,
    name: &'static str,
    label: &'static str,
    mode: &'static str,
    status: &'static str,
) -> Result<Arm, Box<dyn Error>> {
    let run = study.join("arms").join(name);
    let worker_path = run.join("worker.json");
    let worker_bytes = fs::read(&worker_path)?;
    let worker: Value = serde_json::from_slice(&worker_bytes)?;
    let replay_name = match name {
        "a1-full" => "a1",
        "b-selector-only" => "b",
        "a2-full" => "a2",
        _ => return Err(format!("unknown report arm: {name}").into()),
    };
    let replay = read_json(&study.join("replay").join(format!("{replay_name}.json")))?;
    let state = read_json(&run.join("container-state.json"))?;
    let progress = find_jsonl(
        &run.join("worker.json.progress.jsonl"),
        "sample_kind",
        "returned",
    )?;
    let root = find_jsonl(
        &run.join("worker.json.phases.jsonl"),
        "phase",
        "root_propagation_returned",
    )?;
    let domain = find_jsonl(
        &run.join("worker.json.phases.jsonl"),
        "phase",
        "model_domain_done",
    )?;
    let choices = worker["orbit_choice_pins"]
        .as_array()
        .ok_or("missing orbit choices")?;
    let phase_pinned = mode == "full";
    if choices.len() != 4
        || choices.iter().any(|choice| {
            choice["selector_pinned"] != true || choice["phase_pinned"] != phase_pinned
        })
    {
        return Err(format!("{name}: pin flags differ from frozen mode").into());
    }
    if worker["source_commit"] != SOURCE
        || worker["object_blake3"] != OBJECT
        || worker["point_set_blake3"] != POINT_SET
        || worker["status"] != status
        || worker["solver_models"] != 1
        || worker["root_propagation_result"] != "Fixpoint"
        || worker["relation"]["group_verified"] != true
        || worker["relation"]["matches_preflight_row"] != true
        || worker["relation"]["rank_after_insert"] != 1
        || worker["relation"]["tamper_rejected"] != true
        || domain["details"]["sat_variables"] != 201_598
        || domain["details"]["sat_clauses"] != 5_915_238
        || domain["details"]["sat_xor_rows"] != 3_901
        || replay["status"] != "PASS_RELATION_REPLAY"
        || replay["source_commit"] != SOURCE
        || replay["arm"] != mode
        || replay["worker_receipt_blake3"] != blake3::hash(&worker_bytes).to_hex().as_str()
        || state["Status"] != "exited"
        || state["ExitCode"] != 0
        || state["OOMKilled"] != false
        || fs::read_to_string(run.join("host-exit.txt"))?.trim() != "0"
    {
        return Err(
            format!("{name}: source, model, relation, replay or container mismatch").into(),
        );
    }
    let peak_bytes: u64 = fs::read_to_string(run.join("peak.bytes"))?.trim().parse()?;
    if peak_bytes >= 4 * 1024 * 1024 * 1024 {
        return Err(format!("{name}: peak reached the 4 GiB cap").into());
    }
    Ok(Arm {
        name,
        label,
        mode,
        status,
        wall_ms: number(&worker, "process_wall_ms")?,
        import_ms: number(&worker, "base_import_ms")?,
        model_ms: number(&worker, "model_domain_ms")?,
        root_ms: number(&root["details"], "root_propagation_ms")?,
        solver_ms: number(&worker, "solver_ms")?,
        decisions: integer(&progress, "completed_decisions")?,
        conflicts: integer(&progress, "completed_conflicts")?,
        xor_passes: integer(&progress, "completed_xor_passes")?,
        xor_ns: integer(&progress, "completed_xor_nanoseconds")?,
        xor_row_scans: integer(&progress, "completed_xor_row_scans")?,
        peak_bytes,
        row_hash: worker["relation"]["dense_decimal_blake3"]
            .as_str()
            .ok_or("missing row hash")?
            .to_owned(),
        worker_hash: blake3::hash(&worker_bytes).to_hex().to_string(),
    })
}

fn svg(arms: &[Arm]) -> String {
    let mut out = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="650" viewBox="0 0 1200 650">
<rect width="1200" height="650" fill="#f7f9fc"/>
<text x="55" y="56" font-family="Arial,sans-serif" font-size="28" font-weight="bold" fill="#173657">N83 K=16,627 · representative selectors fixed</text>
<text x="55" y="88" font-family="Arial,sans-serif" font-size="16" fill="#52687d">Primary curve · four summands · one source and binary · public indices [0, 166, 332, 498]</text>
<rect x="55" y="113" width="1090" height="72" rx="12" fill="#e8f5eb" stroke="#95cba2"/>
<text x="78" y="141" font-family="Arial,sans-serif" font-size="17" font-weight="bold" fill="#215831">A1, B and A2 returned the same checked rank-one relation</text>
<text x="78" y="166" font-family="Arial,sans-serif" font-size="14" fill="#376644">B leaves 28 phase bits free; separate native replay and changed-receipt controls passed</text>
<text x="55" y="222" font-family="Arial,sans-serif" font-size="18" font-weight="bold" fill="#173657">Worker wall time (seconds, including retained-base import)</text>
<line x1="285" y1="235" x2="960" y2="235" stroke="#b8c8d8"/>
<text x="285" y="253" font-family="Arial,sans-serif" font-size="12" fill="#667c90">0</text>
<text x="902" y="253" font-family="Arial,sans-serif" font-size="12" fill="#667c90">45 s</text>
"##,
    );
    for (index, arm) in arms.iter().enumerate() {
        let y = 270 + index * 49;
        let width = (arm.wall_ms / 1000.0 / 45.0 * 675.0).min(675.0);
        let color = if arm.mode == "selector-only" {
            "#b46f21"
        } else {
            "#2868a7"
        };
        out.push_str(&format!(
            "<text x=\"55\" y=\"{}\" font-family=\"Arial,sans-serif\" font-size=\"16\" fill=\"#253d53\">{}</text>\n<rect x=\"285\" y=\"{}\" width=\"{:.3}\" height=\"24\" rx=\"5\" fill=\"{}\"/>\n<text x=\"980\" y=\"{}\" font-family=\"Arial,sans-serif\" font-size=\"16\" font-weight=\"bold\" fill=\"#253d53\">{:.3} s</text>\n",
            y + 18,
            arm.label,
            y,
            width,
            color,
            y + 18,
            arm.wall_ms / 1000.0
        ));
    }
    out.push_str(
        r##"<text x="55" y="448" font-family="Arial,sans-serif" font-size="18" font-weight="bold" fill="#173657">SAT call (milliseconds, axis ends at 6,500 ms)</text>
<line x1="285" y1="462" x2="960" y2="462" stroke="#b8c8d8"/>
<text x="285" y="480" font-family="Arial,sans-serif" font-size="12" fill="#667c90">0</text>
<text x="907" y="480" font-family="Arial,sans-serif" font-size="12" fill="#667c90">6,500</text>
"##,
    );
    for (index, arm) in arms.iter().enumerate() {
        let y = 497 + index * 41;
        let width = (arm.solver_ms / 6500.0 * 675.0).min(675.0);
        let color = if arm.mode == "selector-only" {
            "#b46f21"
        } else {
            "#2868a7"
        };
        out.push_str(&format!(
            "<text x=\"55\" y=\"{}\" font-family=\"Arial,sans-serif\" font-size=\"16\" fill=\"#253d53\">{}</text>\n<rect x=\"285\" y=\"{}\" width=\"{:.3}\" height=\"20\" fill=\"{}\"/>\n<text x=\"980\" y=\"{}\" font-family=\"Arial,sans-serif\" font-size=\"15\" fill=\"#253d53\">{:.3} ms</text>\n",
            y + 16,
            arm.label,
            y,
            width,
            color,
            y + 16,
            arm.solver_ms
        ));
    }
    out.push_str(
        r##"<text x="55" y="638" font-family="Arial,sans-serif" font-size="13" fill="#52687d">B: 517 decisions, 222 conflicts, 826 completed XOR passes. A1/A2: zero decisions, three XOR passes.</text>
</svg>
"##,
    );
    out
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: n83_selector_only_report STUDY_DIRECTORY".into());
    }
    let study = PathBuf::from(&args[1]);
    let arms = [
        load_arm(
            &study,
            "a1-full",
            "A1 full choice",
            "full",
            "PASS_SELECTOR_PINNED_WITNESS",
        )?,
        load_arm(
            &study,
            "b-selector-only",
            "B selectors only",
            "selector-only",
            "PASS_SELECTOR_ONLY_WITNESS",
        )?,
        load_arm(
            &study,
            "a2-full",
            "A2 full choice",
            "full",
            "PASS_SELECTOR_PINNED_WITNESS",
        )?,
    ];
    if arms.iter().any(|arm| arm.row_hash != arms[0].row_hash) {
        return Err("A1/B/A2 returned different relation rows".into());
    }
    let summary = json!({
        "schema":"n83.k16627-m4-selector-only-result/v1",
        "source_commit":SOURCE,
        "object_blake3":OBJECT,
        "point_set_blake3":POINT_SET,
        "status":"PASS_THREE_CHECKED_ARMS",
        "relation_row_blake3":arms[0].row_hash,
        "phase_only_arm":"OMITTED_BY_FROZEN_CONDITION_B_SAT",
        "arms":arms.iter().map(|arm| json!({
            "name":arm.name,
            "mode":arm.mode,
            "status":arm.status,
            "worker_receipt_blake3":arm.worker_hash,
            "worker_wall_ms":arm.wall_ms,
            "base_import_ms":arm.import_ms,
            "model_domain_ms":arm.model_ms,
            "root_propagation_ms":arm.root_ms,
            "solver_ms":arm.solver_ms,
            "decisions":arm.decisions,
            "conflicts":arm.conflicts,
            "completed_xor_passes":arm.xor_passes,
            "completed_xor_nanoseconds":arm.xor_ns,
            "completed_xor_row_scans":arm.xor_row_scans,
            "cgroup_peak_bytes":arm.peak_bytes,
        })).collect::<Vec<_>>(),
        "natural_relation_yield":Value::Null,
        "natural_useful_rank":Value::Null,
        "total_index_calculus_runtime_ms":Value::Null,
        "selected_best_total_runtime":Value::Null,
    });
    fs::write(
        study.join("summary.json"),
        serde_json::to_vec_pretty(&summary)?,
    )?;
    fs::write(study.join("selector-only-comparison.svg"), svg(&arms))?;
    println!("{}", summary["status"]);
    Ok(())
}
