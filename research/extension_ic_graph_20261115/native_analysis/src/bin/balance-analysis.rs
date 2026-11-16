//! Fixed seven-arm parameter diagnostic; failures remain in the exported flow.
#[allow(dead_code)]
#[path = "structural-gate-analysis.rs"]
pub(crate) mod gate;
use gate::graph::{
    self,
    full::{
        self,
        shared::{n, prime},
    },
};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

const MODES: [&str; 7] = [
    "ic", "s75-r32", "s75-r128", "s85-r32", "s85-r128", "s95-r32", "s95-r128",
];
fn parameters(i: usize) -> Result<(u64, u64), String> {
    if !(1..7).contains(&i) {
        return Err("graph arm index".into());
    }
    Ok((
        [75, 85, 95][(i - 1) / 2],
        if (i - 1).is_multiple_of(2) { 32 } else { 128 },
    ))
}
fn prefix(a: &Value, b: &Value) -> Result<(), String> {
    let a = a.as_array().ok_or("prefix array")?;
    let b = b.as_array().ok_or("prefix array")?;
    if a[..a.len().min(b.len())] != b[..a.len().min(b.len())] {
        return Err("common prefix differs".into());
    }
    Ok(())
}
fn identity(w: &Value, public: &Value, plan: &Value, method: &str) -> Result<(), String> {
    if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
        || w["source_revision"] != plan["source_revision"]
        || w["seed"] != plan["seed"]
        || w["method"] != method
        || w["public"] != *public
        || w["solver_answer_placeholder"] != 0
    {
        return Err("worker public/source/method identity".into());
    }
    Ok(())
}
fn solved(w: &Value, answer: u64) -> Result<bool, String> {
    if w["verified"] == true && w["capped"] == false && w["actual_scalar"] == answer {
        return Ok(true);
    }
    if w["verified"] == false && w["capped"] == true && w["actual_scalar"].is_null() {
        return Ok(false);
    }
    Err("inconsistent or invalid scalar success".into())
}
pub(crate) fn gate_check(w: &Value, plan: &Value, p: u64, l: u64) -> Result<Value, String> {
    let v = &w["work"];
    let sources = v["source_relations"].as_array().ok_or("sources")?;
    let inputs: Vec<_> = sources.iter().map(|r| r["partial"].clone()).collect();
    let report = &v["collection_structural_gate"];
    let checks = report["checks"].as_array().ok_or("gate checks")?;
    let exact = v["collection_rank_checks"]
        .as_array()
        .ok_or("exact checks")?;
    if report["schema"] != "two-large-prime.structural-rank-gate/v1"
        || report["native_bookkeeping_complete"] != false
        || report["known_uncounted"].as_str().is_none_or(str::is_empty)
        || checks.is_empty()
        || checks.len() > 128
    {
        return Err("gate coverage/check count".into());
    }
    let target = n(v, "collection_rank_target")?;
    let (mut required, mut previous_b, mut permitted) = (target.max(1), 0, 0);
    let mut independent = Vec::new();
    for c in checks {
        let count = n(c, "source_rows")?;
        let b = n(c, "b_count")?;
        if count < required
            || count > inputs.len() as u64
            || b <= previous_b
            || b > n(v, "b_count_completed")?
            || (!b.is_multiple_of(1024) && b != n(plan, "b_count")?)
            || c["rank_target"] != target
        {
            return Err("gate source/stride/batch schedule".into());
        }
        let s = gate::structural_state(&inputs[..count as usize], p, l)?;
        state_equal(&c["state"], &s)?;
        let allow = n(&s, "small_rank_upper_bound")? >= target;
        if c["permits_exact_check"] != allow {
            return Err("gate predicate".into());
        }
        if allow {
            let e = exact
                .get(permitted)
                .ok_or("missing permitted exact check")?;
            if e["source_rows"] != count || e["b_count"] != b {
                return Err("permitted exact prefix".into());
            }
            permitted += 1;
        }
        independent.push(
            json!({"b_count":b,"source_rows":count,"permits_exact_check":allow,"independent":s}),
        );
        required = count
            .checked_add(n(plan, "collection_rank_stride")?)
            .ok_or("stride overflow")?;
        previous_b = b;
    }
    if permitted != exact.len() || previous_b != n(v, "b_count_completed")? {
        return Err("gate final endpoint".into());
    }
    let final_state = gate::structural_state(&inputs, p, l)?;
    state_equal(&report["final_state"], &final_state)?;
    let words = 3 * p
        + 2 * n(&final_state, "support_entries")?
        + n(&report["final_state"], "parent_probes")?
        + checks.len() as u64;
    if report["charged_word_events_lower_bound"] != words {
        return Err("gate event charge".into());
    }
    let la = w["work_native"]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .find(|r| r["phase"] == "linear_algebra")
        .ok_or("LA")?;
    if n(&la["primitives"], "word_op")? < words {
        return Err("gate word containment".into());
    }
    Ok(
        json!({"checks":independent,"known_gate_word_events":words,"all_prefix_bounds_checked":true,"native_bookkeeping_complete":false}),
    )
}
fn state_equal(emitted: &Value, independent: &Value) -> Result<(), String> {
    for key in [
        "sources",
        "vertices",
        "components",
        "anchored_components",
        "large_rank_lower_bound",
        "small_rank_upper_bound",
        "find_calls",
        "successful_unions",
    ] {
        if emitted[key] != independent[key] {
            return Err(format!("independent gate state {key}"));
        }
    }
    if n(emitted, "parent_probes")? < n(independent, "find_calls")? {
        return Err("parent-probe lower bound".into());
    }
    Ok(())
}
fn protocol(plan: &Value, fixture: &Value) -> Result<(u64, u64, u64), String> {
    let public = &fixture["public"];
    let (p, l, answer) = (
        n(public, "p")?,
        n(public, "l")?,
        n(fixture, "expected_scalar")?,
    );
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if plan["schema"] != "cover.two-lp-balance.runpod-validation/v1"
        || plan["driver_kind"] != "cover-two-lp-factor-base-checkpoint-balance"
        || plan["backend"] != "runpod_user_authorized"
        || plan["runpod_compute_type"] != "CPU"
        || plan["cpu_affinity_rule"] != "first_allowed_cpu_bound_before_call"
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|s| s.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || plan["mode_order"] != json!(MODES)
        || plan["p"] != 1009
        || p != 1009
        || plan["seed"] != 359
        || plan["m"] != 9
        || plan["b_count"] != p * p
        || plan["max_descent_residuals"] != 8192
        || plan["max_accepted_descents"] != 32
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["allow_comparative_claims"] != false
        || plan["fixture_timeout_seconds"] != 60
        || plan["worker_timeout_seconds"] != 300
        || plan["timeout_seconds"] != 2300
        || plan["runpod_lifetime_seconds"] != 2700
        || plan["memory_gib"] != 8
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || fixture["workers_receive"] != "public only"
        || fixture["seed"] != plan["seed"]
        || fixture["source_revision"] != source
        || source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || answer == 0
        || answer >= l
    {
        return Err("frozen balance protocol/domain".into());
    }
    Ok((p, l, answer))
}
fn read(path: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(path)?.len() > 32 * 1024 * 1024 {
        return Err("record too large".into());
    }
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn write_new(path: &Path, v: &Value, pretty: bool) -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    if pretty {
        serde_json::to_writer_pretty(&mut f, v)?;
    } else {
        serde_json::to_writer(&mut f, v)?;
    }
    writeln!(f)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: balance-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for file in ["exit-code.txt", "fixture.exit-code.txt"] {
        if fs::read_to_string(run.join(file))?.trim() != "0" {
            return Err("whole/fixture failure retained".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let fixture = read(&run.join("fixture.json"))?;
    let (p, l, answer) = protocol(&plan, &fixture)?;
    if read(&run.join("public.json"))? != fixture["public"] {
        return Err("public file mismatch".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 16 {
        return Err("incomplete flow".into());
    }
    let mut outcomes = Vec::new();
    let mut workers = Vec::new();
    let mut completed = 0;
    let mut recovered = 0;
    for (i, mode) in std::iter::once("fixture").chain(MODES).enumerate() {
        let code: i32 = fs::read_to_string(run.join(format!("{mode}.exit-code.txt")))?
            .trim()
            .parse()?;
        for j in 0..2 {
            let row = &flow[2 * i + j];
            let kind = if i == 0 {
                if j == 0 {
                    "fixture_start"
                } else {
                    "fixture"
                }
            } else if j == 0 {
                "execution_start"
            } else {
                "execution"
            };
            if row["mode"] != mode
                || row["record_type"] != kind
                || row["source_revision"] != plan["source_revision"]
                || (j == 1 && row["exit_code"] != code)
            {
                return Err("flow order/source/status".into());
            }
        }
        if i == 0 {
            continue;
        }
        let index = i - 1;
        if code != 0 {
            outcomes.push(json!({"arm":mode,"status":"execution_failed","exit_code":code,"verified":false,"ranking_eligible":false}));
            workers.push(Value::Null);
            continue;
        }
        completed += 1;
        let w = read(&run.join(format!("{mode}.raw.json")))?;
        identity(
            &w,
            &fixture["public"],
            &plan,
            if index == 0 { "ic" } else { "two-lp" },
        )?;
        let accounting = full::counter_check(&w, p, l)?;
        let success = solved(&w, answer)?;
        let mut proof = Value::Null;
        let mut checkpoint = Value::Null;
        let mut gate = Value::Null;
        if success {
            recovered += 1;
            if index > 0 {
                let (small, stride) = parameters(index)?;
                let mut arm_plan = plan.clone();
                arm_plan["small_percent"] = json!(small);
                arm_plan["collection_rank_stride"] = json!(stride);
                if w["work"]["options"]["collection_rank_stride"] != stride {
                    return Err("executed stride".into());
                }
                proof = full::rank_stopped_candidate_proof(&w, &arm_plan, l, answer)?;
                checkpoint = graph::checkpoints(&w, &arm_plan, p, l)?;
                gate = gate_check(&w, &arm_plan, p, l)?;
            }
        }
        outcomes.push(json!({"arm":mode,"status":if success {"recovered"} else {"capped"},"exit_code":code,"verified":success,
            "actual_scalar":w["actual_scalar"],"accounting":accounting,"candidate_proof":proof,"checkpoint_proof":checkpoint,"gate_proof":gate,
            "full_base":w["work"]["full_base"],"small_base":w["work"]["small_base"],"b_count":w["work"]["b_count_completed"],
            "source_rows":w["work"]["verified_source_rows"],"descent_residuals":w["work"]["descent_residuals"],"accepted_descents":w["work"]["descents"].as_array().map(Vec::len),
            "recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"],"unpriced_work":w["work_native"]["known_uncounted"],
            "ranking_eligible":false,"pricing_band":null,"total_bitops":null}));
        workers.push(w);
    }
    let successful: Vec<_> = (1..7)
        .filter(|&i| outcomes[i]["verified"] == true)
        .collect();
    for pair in successful.windows(2) {
        let (a, b) = (&workers[pair[0]]["work"], &workers[pair[1]]["work"]);
        if a["full_base"] != b["full_base"] {
            return Err("full base changed".into());
        }
        prefix(&a["descents"], &b["descents"])?;
    }
    for (a, b) in [(1, 2), (3, 4), (5, 6)] {
        if outcomes[a]["verified"] == true && outcomes[b]["verified"] == true {
            prefix(
                &workers[a]["work"]["source_relations"],
                &workers[b]["work"]["source_relations"],
            )?;
        }
    }
    let summary = json!({"schema":"cover.two-lp-balance-analysis/v1","source_revision":plan["source_revision"],"p":p,"l":l,"seed":plan["seed"],"actual_scalar":answer,
        "outcomes":outcomes,"successful_graph_common_descent_prefix_checked":true,"same_fraction_original_source_prefix_checked":true,
        "complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false,
        "flow":{"planned":7,"attempted":7,"completed":completed,"recovered":recovered,"failed_or_capped":7-recovered,"excluded":0,"automatic_retries":0}});
    write_new(Path::new(&args[2]), &summary, true)?;
    let mut files = Vec::new();
    for entry in fs::read_dir(run)? {
        let path = entry?.path();
        let name = path.file_name().ok_or("file name")?.to_string_lossy();
        if path.is_file()
            && (name.ends_with(".raw.json")
                || name.ends_with(".argv.json")
                || name.ends_with(".exit-code.txt")
                || name == "flow.jsonl")
        {
            let bytes = fs::read(&path)?;
            files.push(json!({"file":name,"bytes":bytes.len(),"blake3":blake3::hash(&bytes).to_hex().to_string()}));
        }
    }
    let export = json!({"schema":"cover.two-lp-balance.raw-export/v1","canonical_schema_conformance_claimed":false,"canonical_experiment_export_schema_available":false,"protocol":plan,"fixture":fixture,"workers":workers,"flow":flow,"analysis":summary,"exact_original_files":files});
    write_new(&run.join("raw-export.json"), &export, false)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_cross_product() {
        assert!(parameters(0).is_err());
        assert!(parameters(7).is_err());
        assert_eq!(parameters(1).unwrap(), (75, 32));
        assert_eq!(parameters(2).unwrap(), (75, 128));
        assert_eq!(parameters(5).unwrap(), (95, 32));
        assert_eq!(parameters(6).unwrap(), (95, 128));
    }
    #[test]
    fn success_cannot_be_forged() {
        let mut w = json!({"verified":true,"capped":false,"actual_scalar":7});
        assert!(solved(&w, 7).unwrap());
        w["actual_scalar"] = json!(8);
        assert!(solved(&w, 7).is_err());
        w = json!({"verified":false,"capped":true,"actual_scalar":null});
        assert!(!solved(&w, 7).unwrap());
        w["actual_scalar"] = json!(7);
        assert!(solved(&w, 7).is_err());
    }
    #[test]
    fn changed_common_prefix_rejected() {
        assert!(prefix(&json!([1, 2]), &json!([1, 2, 3])).is_ok());
        assert!(prefix(&json!([1, 2]), &json!([1, 3])).is_err());
        assert!(prefix(&Value::Null, &json!([])).is_err());
    }
    #[test]
    fn independent_state_forgery_rejected() {
        let rows = vec![
            json!({"id":0,"rhs":0,"large":[[2,1],[3,1]]}),
            json!({"id":1,"rhs":0,"large":[[2,1]]}),
        ];
        let s = gate::structural_state(&rows, 101, 103).unwrap();
        let mut emitted = s.clone();
        emitted["parent_probes"] = json!(3);
        assert!(state_equal(&emitted, &s).is_ok());
        emitted["large_rank_lower_bound"] = json!(1);
        assert!(state_equal(&emitted, &s).is_err());
    }
    #[test]
    fn frozen_protocol_rejects_seed_order_and_budget_changes() {
        let mut plan: Value =
            serde_json::from_str(include_str!("../../../balance-plan.json")).unwrap();
        plan["source_revision"] = json!("a".repeat(40));
        plan["pinned_cpu"] = json!("3");
        plan["runpod_allocation"] =
            json!({"provider":"runpod","compute_type":"CPU","vcpuCount":4,"memoryInGb":8});
        let fixture = json!({"schema":"cover.two-large-prime.dlp-fixture/v1","workers_receive":"public only","source_revision":"a".repeat(40),"seed":359,"public":{"p":1009,"l":103},"expected_scalar":7});
        assert_eq!(protocol(&plan, &fixture).unwrap(), (1009, 103, 7));
        for (key, value) in [
            ("seed", json!(347)),
            ("mode_order", json!(["ic"])),
            ("worker_timeout_seconds", json!(301)),
            ("allow_comparative_claims", json!(true)),
        ] {
            let mut changed = plan.clone();
            changed[key] = value;
            assert!(protocol(&changed, &fixture).is_err());
        }
    }
    #[test]
    fn gate_schedule_predicate_and_charge_mutants_are_rejected() {
        let inputs = vec![
            json!({"id":0,"rhs":0,"large":[[2,1],[3,1]]}),
            json!({"id":1,"rhs":0,"large":[[2,1]]}),
        ];
        let mut state = gate::structural_state(&inputs, 101, 103).unwrap();
        state["parent_probes"] = json!(3);
        let sources: Vec<_> = inputs.iter().map(|v| json!({"partial":v})).collect();
        let w = json!({"work":{"source_relations":sources,"collection_rank_target":1,"b_count_completed":1024,
            "collection_rank_checks":[],"collection_structural_gate":{"schema":"two-large-prime.structural-rank-gate/v1","native_bookkeeping_complete":false,"known_uncounted":"DSU",
            "checks":[{"b_count":1024,"source_rows":2,"rank_target":1,"permits_exact_check":false,"state":state}],"final_state":state,"charged_word_events_lower_bound":313}},
            "work_native":{"phases":[{"phase":"linear_algebra","primitives":{"word_op":313}}]}});
        let plan = json!({"collection_rank_stride":32,"b_count":1024});
        assert!(gate_check(&w, &plan, 101, 103).is_ok());
        let mut changed = w.clone();
        changed["work"]["collection_structural_gate"]["checks"][0]["permits_exact_check"] =
            json!(true);
        assert!(gate_check(&changed, &plan, 101, 103).is_err());
        let mut changed = w.clone();
        changed["work"]["collection_structural_gate"]["checks"][0]["b_count"] = json!(1023);
        assert!(gate_check(&changed, &plan, 101, 103).is_err());
        let mut changed = w.clone();
        changed["work"]["collection_structural_gate"]["charged_word_events_lower_bound"] =
            json!(312);
        assert!(gate_check(&changed, &plan, 101, 103).is_err());
        let mut changed = w;
        changed["work_native"]["phases"][0]["primitives"]["word_op"] = json!(312);
        assert!(gate_check(&changed, &plan, 101, 103).is_err());
    }
}
