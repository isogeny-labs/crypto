//! Same-binary IC/graph/rho producer qualification. Incomplete costs stay unrankable.
// Preserve the existing standalone qualifiers and their module-local tests.
// Both intentionally include the same read-only primitive-price helpers.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "rank-stop-analysis.rs"]
mod graph;
#[allow(dead_code)]
#[path = "rho-native-price-analysis.rs"]
mod rho;
use graph::full::{
    self,
    shared::{n, prime},
};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

const MODES: [&str; 6] = [
    "ic",
    "rank-stop",
    "rho-plain-1",
    "rho-negation-1",
    "rho-plain-16",
    "rho-negation-16",
];

fn rho_identity(
    w: &Value,
    negation: bool,
    batch: u64,
    p: u64,
    l: u64,
    seed: &Value,
) -> Result<(), String> {
    let r = &w["work"]["run"];
    let options = json!({"negation":negation,"batch":batch,"r":256,"dp_bits":16.min((l.ilog2()/2).saturating_sub(4))});
    if w["rho_options"] != options
        || w["recovered_scalar"] != w["actual_scalar"]
        || w["work"]["verified"] != true
        || r["correct"] != false
        || r["p"] != p
        || r["l"] != l
        || r["seed"] != *seed
        || r["run"] != 0
        || r["capped"] != false
        || n(r, "steps")? == 0
    {
        return Err("rho public worker/configuration/endpoint identity".into());
    }
    for key in ["negation", "batch", "r", "dp_bits"] {
        if r[key] != options[key] {
            return Err(format!("rho executed {key} differs from requested option"));
        }
    }
    Ok(())
}

fn analyze(fixture: &Value, workers: &[Value], plan: &Value) -> Result<Value, String> {
    let public = &fixture["public"];
    let (p, l, answer) = (
        n(public, "p")?,
        n(public, "l")?,
        n(fixture, "expected_scalar")?,
    );
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if workers.len() != MODES.len()
        || plan["schema"] != "cover.ic-graph-rho.runpod-validation/v1"
        || plan["driver_kind"] != "same-binary-ic-graph-rho-portfolio"
        || plan["backend"] != "runpod_user_authorized"
        || plan["mode_order"] != json!(MODES)
        || plan["p"] != 1009
        || p != 1009
        || plan["seed"] != 337
        || plan["m"] != 9
        || plan["small_percent"] != 75
        || plan["b_count"] != p * p
        || plan["collection_rank_stride"] != 128
        || plan["max_descent_residuals"] != 8192
        || plan["max_accepted_descents"] != 32
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["fixture_timeout_seconds"] != 60
        || plan["worker_timeout_seconds"] != 900
        || plan["timeout_seconds"] != 5500
        || plan["runpod_lifetime_seconds"] != 7200
        || plan["memory_gib"] != 8
        || plan["allow_comparative_claims"] != false
        || plan["runpod_compute_type"] != "CPU"
        || plan["cpu_affinity_rule"] != "first_allowed_cpu_bound_before_call"
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|v| v.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || fixture["workers_receive"] != "public only"
        || fixture["source_revision"] != source
        || fixture["seed"] != plan["seed"]
        || source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || answer == 0
        || answer >= l
    {
        return Err("frozen Runpod portfolio protocol/domain".into());
    }
    let mut outcomes = Vec::new();
    for (i, w) in workers.iter().enumerate() {
        let method = if i == 0 {
            "ic"
        } else if i == 1 {
            "two-lp"
        } else {
            "rho"
        };
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["seed"] != plan["seed"]
            || w["method"] != method
            || w["public"] != *public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err(format!("{} public input/actual scalar binding", MODES[i]));
        }
        let accounting = if i < 2 {
            full::counter_check(w, p, l)?
        } else {
            rho_identity(
                w,
                i == 3 || i == 5,
                if i < 4 { 1 } else { 16 },
                p,
                l,
                &plan["seed"],
            )?;
            rho::account(w, true, p, l)?.0
        };
        outcomes.push(json!({"arm":MODES[i],"actual_scalar":answer,"verified":true,
            "recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"],"accounting":accounting,
            "unpriced_work":w["work_native"]["known_uncounted"],"ranking_eligible":false,
            "pricing_band":null,"unrankable_reason":"native work categories without complete emitted operation counts",
            "rho_options":w["rho_options"],"rho_steps":w["work"]["run"]["steps"]}));
    }
    let graph_proof = full::rank_stopped_candidate_proof(&workers[1], plan, l, answer)?;
    let checkpoint_proof = graph::checkpoints(&workers[1], plan, p, l)?;
    Ok(
        json!({"schema":"cover.ic-graph-rho-portfolio-analysis/v1","source_revision":source,"p":p,"l":l,
        "seed":plan["seed"],"actual_scalar":answer,"outcomes":outcomes,"graph_proof":graph_proof,
        "rank_check_proof":checkpoint_proof,"all_public_workers_recovered_actual_scalar":true,
        "target_class":plan["target_class"],"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,
        "cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false,
        "flow":{"planned":6,"attempted":6,"completed":6,"excluded":0,"automatic_retries":0}}),
    )
}
fn read(p: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(p)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB".into());
    }
    Ok(serde_json::from_reader(File::open(p)?)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: portfolio-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for mode in std::iter::once("fixture").chain(MODES) {
        if fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?.trim() != "0" {
            return Err("failed/censored process retained".into());
        }
    }
    if fs::read_to_string(run.join("exit-code.txt"))?.trim() != "0" {
        return Err("failed/censored cohort retained".into());
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let fixture = read(&run.join("fixture.json"))?;
    if read(&run.join("public.json"))? != fixture["public"] {
        return Err("public fixture".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 14 {
        return Err("incomplete flow".into());
    }
    for (i, row) in flow.iter().enumerate() {
        let mode = if i < 2 { "fixture" } else { MODES[i / 2 - 1] };
        let kind = match i {
            0 => "fixture_start",
            1 => "fixture",
            x if x % 2 == 0 => "execution_start",
            _ => "execution",
        };
        if row["mode"] != mode
            || row["record_type"] != kind
            || row["source_revision"] != plan["source_revision"]
            || (i % 2 == 1 && row["exit_code"] != 0)
        {
            return Err("flow order/identity/exit".into());
        }
    }
    let workers: Vec<_> = MODES
        .iter()
        .map(|m| read(&run.join(format!("{m}.raw.json"))))
        .collect::<Result<_, _>>()?;
    let summary = analyze(&fixture, &workers, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    let names = std::iter::once("runtime-plan.json".into())
        .chain(["fixture.json", "public.json", "flow.jsonl"].map(String::from))
        .chain(MODES.map(|m| format!("{m}.raw.json")));
    let mut originals = Vec::new();
    for file in names {
        let bytes = fs::read(run.join(&file))?;
        originals.push(json!({"file":file,"bytes":bytes.len(),"blake3":blake3::hash(&bytes).to_hex().to_string()}));
    }
    let export = json!({"schema":"cover.ic-graph-rho-portfolio.raw-export/v1","canonical_experiment_export_schema_available":false,
        "canonical_schema_conformance_claimed":false,"protocol":plan,"fixture":fixture,"workers":workers,"flow":flow,
        "analysis":summary,"exact_original_files":originals,"independent_signed_verdict":false});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run.join("raw-export.json"))?;
    serde_json::to_writer(&mut out, &export)?;
    writeln!(out)?;
    Ok(())
}
