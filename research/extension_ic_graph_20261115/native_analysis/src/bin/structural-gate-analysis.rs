//! Independent graph connectivity and matrix-rank replay for optional checkpoint skips.
#[allow(dead_code)]
#[path = "rank-stop-analysis.rs"]
pub(crate) mod graph;
use graph::full::{
    self,
    shared::{canonical, n, prime, Terms},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

// This checker uses graph traversal and independent dense elimination rather
// than the production union-find helper or its counters.
pub(crate) fn structural_state(inputs: &[Value], p: u64, l: u64) -> Result<Value, String> {
    let mut adjacent = BTreeMap::<u64, BTreeSet<u64>>::new();
    let mut anchors = BTreeSet::new();
    let mut supports = Vec::new();
    let (mut finds, mut entries) = (0, 0);
    for (i, input) in inputs.iter().enumerate() {
        if input["id"] != i as u64 || input["rhs"] != 0 {
            return Err("homogeneous source prefix identity".into());
        }
        let row = canonical(&input["large"], p, l)?;
        if row.len() > 2 {
            return Err("two-LP support required".into());
        }
        entries += row.len();
        finds += row.len();
        for &(v, _) in &row {
            adjacent.entry(v).or_default();
        }
        if row.len() == 1 {
            anchors.insert(row[0].0);
        }
        if row.len() == 2 {
            adjacent.get_mut(&row[0].0).unwrap().insert(row[1].0);
            adjacent.get_mut(&row[1].0).unwrap().insert(row[0].0);
        }
        supports.push(row);
    }
    let (mut seen, mut components, mut anchored) = (BTreeSet::new(), 0, 0);
    for &start in adjacent.keys() {
        if !seen.insert(start) {
            continue;
        }
        components += 1;
        let mut stack = vec![start];
        let mut has_anchor = false;
        while let Some(v) = stack.pop() {
            has_anchor |= anchors.contains(&v);
            for &next in &adjacent[&v] {
                if seen.insert(next) {
                    stack.push(next);
                }
            }
        }
        if has_anchor {
            anchored += 1;
        }
    }
    let labels: BTreeMap<_, _> = adjacent
        .keys()
        .enumerate()
        .map(|(i, &v)| (v, i as u64))
        .collect();
    let matrix: Vec<(Terms, u64)> = supports
        .into_iter()
        .map(|row| (row.into_iter().map(|(v, c)| (labels[&v], c)).collect(), 0))
        .collect();
    let (actual_lp_rank, _) = full::target_solution(&matrix, labels.len(), l)?;
    let lower = labels.len() - components + anchored;
    if lower > actual_lp_rank {
        return Err("connectivity bound exceeds independent large-column rank".into());
    }
    Ok(
        json!({"sources":inputs.len(),"vertices":labels.len(),"components":components,"anchored_components":anchored,
        "large_rank_lower_bound":lower,"small_rank_upper_bound":inputs.len()-lower,"find_calls":finds,
        "successful_unions":labels.len()-components,"support_entries":entries,"independent_large_rank":actual_lp_rank,
        "independent_kernel_dimension":inputs.len()-actual_lp_rank}),
    )
}

fn phase_conservation(
    a: &Value,
    b: &Value,
    omitted: &[Value],
    words: u64,
) -> Result<Value, String> {
    let mut removed: BTreeMap<String, u64> = full::shared::KINDS
        .into_iter()
        .map(|k| (k.into(), 0))
        .collect();
    for row in omitted {
        for (key, value) in full::shared::counts(row)? {
            let slot = removed.get_mut(&key).ok_or("primitive")?;
            *slot = slot.checked_add(value).ok_or("checkpoint sum overflow")?;
        }
    }
    let old = a["work_native"]["phases"]
        .as_array()
        .ok_or("baseline phases")?;
    let new = b["work_native"]["phases"]
        .as_array()
        .ok_or("candidate phases")?;
    if old.len() != full::shared::PHASES.len() || new.len() != old.len() {
        return Err("phase count".into());
    }
    for ((before, after), name) in old.iter().zip(new).zip(full::shared::PHASES) {
        if before["phase"] != name || after["phase"] != name {
            return Err("phase order".into());
        }
        let mut expected = full::shared::counts(&before["primitives"])?;
        if name == "linear_algebra" {
            for (key, &value) in &removed {
                let slot = expected.get_mut(key).ok_or("primitive")?;
                *slot = slot
                    .checked_sub(value)
                    .ok_or("omitted checkpoint exceeds baseline phase")?;
            }
            let slot = expected.get_mut("word_op").ok_or("word counter")?;
            *slot = slot.checked_add(words).ok_or("gate word event overflow")?;
        }
        if full::shared::counts(&after["primitives"])? != expected {
            return Err(format!("unexplained primitive change in {name}"));
        }
    }
    Ok(
        json!({"all_phase_counts_conserved":true,"skipped_checkpoint_primitives":removed,
        "added_gate_word_events_lower_bound":words,"other_native_gate_work_complete":false}),
    )
}

fn gate_proof(baseline: &Value, candidate: &Value, p: u64, l: u64) -> Result<Value, String> {
    let (a, b) = (&baseline["work"], &candidate["work"]);
    if a["source_relations"] != b["source_relations"]
        || a["descents"] != b["descents"]
        || a["cohort_digest"] != b["cohort_digest"]
        || a["b_count_completed"] != b["b_count_completed"]
        || !a["collection_structural_gate"].is_null()
    {
        return Err("original relation/descent/collection identity".into());
    }
    let sources: Vec<_> = b["source_relations"]
        .as_array()
        .ok_or("sources")?
        .iter()
        .map(|r| r["partial"].clone())
        .collect();
    let report = &b["collection_structural_gate"];
    let checks = report["checks"].as_array().ok_or("gate checks")?;
    let old = a["collection_rank_checks"]
        .as_array()
        .ok_or("baseline checkpoints")?;
    let new = b["collection_rank_checks"]
        .as_array()
        .ok_or("candidate checkpoints")?;
    if report["schema"] != "two-large-prime.structural-rank-gate/v1"
        || report["native_bookkeeping_complete"] != false
        || report["known_uncounted"].as_str().is_none_or(str::is_empty)
        || checks.len() != old.len()
        || checks.len() > 64
        || checks.is_empty()
    {
        return Err("gate scope/schedule".into());
    }
    let mut checked = Vec::new();
    let mut permitted = 0;
    let mut omitted = Vec::new();
    for (c, old_check) in checks.iter().zip(old) {
        let count = n(c, "source_rows")? as usize;
        if count > sources.len()
            || c["source_rows"] != old_check["source_rows"]
            || c["b_count"] != old_check["b_count"]
            || c["rank_target"] != b["collection_rank_target"]
        {
            return Err("gate checkpoint source schedule".into());
        }
        let exact = structural_state(&sources[..count], p, l)?;
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
            if c["state"][key] != exact[key] {
                return Err(format!("independent gate state {key}"));
            }
        }
        if n(&c["state"], "parent_probes")? < n(&exact, "find_calls")? {
            return Err("parent probe emission".into());
        }
        let ready = n(&exact, "small_rank_upper_bound")? >= n(c, "rank_target")?;
        if c["permits_exact_check"] != ready {
            return Err("structural skip predicate".into());
        }
        if ready {
            if new.get(permitted) != Some(old_check) {
                return Err("permitted exact checkpoint differs".into());
            }
            permitted += 1;
        } else if n(old_check, "rank")? >= n(c, "rank_target")? {
            return Err("skipped a ready baseline checkpoint".into());
        } else {
            omitted.push(old_check["observed_primitives"].clone());
        }
        checked.push(json!({"b_count":c["b_count"],"source_rows":count,"permits_exact_check":ready,"independent":exact}));
    }
    if permitted != new.len() {
        return Err("extra candidate checkpoint".into());
    }
    let final_state = structural_state(&sources, p, l)?;
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
        if report["final_state"][key] != final_state[key] {
            return Err("final gate source state".into());
        }
    }
    let words = 3 * p
        + 2 * n(&final_state, "support_entries")?
        + n(&report["final_state"], "parent_probes")?
        + checks.len() as u64;
    if report["charged_word_events_lower_bound"] != words {
        return Err("gate state event charge".into());
    }
    let phase_proof = phase_conservation(baseline, candidate, &omitted, words)?;
    Ok(
        json!({"same_original_sources":true,"same_descents":true,"same_collection_endpoint":true,"same_checkpoint_schedule":true,
        "checks":checked,"baseline_exact_checks":old.len(),"candidate_exact_checks":new.len(),"skipped_exact_checks":old.len()-new.len(),
            "charged_gate_word_events_lower_bound":words,"phase_conservation":phase_proof,"parent_probe_count_is_source_emission":true,"native_bookkeeping_complete":false}),
    )
}

fn read(path: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(path)?.len() > 32 * 1024 * 1024 {
        return Err("record too large".into());
    }
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: structural-gate-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in [
        "exit-code.txt",
        "fixture.exit-code.txt",
        "rank-stop.exit-code.txt",
        "structural-gate.exit-code.txt",
    ] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
            return Err("failed/censored process retained".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let fixture = read(&run.join("fixture.json"))?;
    let public = &fixture["public"];
    let (p, l, answer) = (
        n(public, "p")?,
        n(public, "l")?,
        n(&fixture, "expected_scalar")?,
    );
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if plan["schema"] != "cover.two-lp-structural-gate.runpod-validation/v1"
        || plan["driver_kind"] != "cover-two-lp-structural-gate-validation"
        || plan["backend"] != "runpod_user_authorized"
        || plan["runpod_compute_type"] != "CPU"
        || plan["cpu_affinity_rule"] != "first_allowed_cpu_bound_before_call"
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|v| v.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || plan["runpod_lifetime_seconds"] != 2700
        || plan["p"] != 1009
        || plan["seed"] != 347
        || plan["small_percent"] != 75
        || plan["b_count"] != 1018081
        || plan["collection_rank_stride"] != 128
        || plan["max_descent_residuals"] != 8192
        || plan["max_accepted_descents"] != 32
        || plan["mode_order"] != json!(["rank-stop", "structural-gate"])
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["allow_comparative_claims"] != false
        || plan["worker_timeout_seconds"] != 900
        || plan["fixture_timeout_seconds"] != 60
        || plan["timeout_seconds"] != 1900
        || plan["p"] != p
        || plan["m"] != 9
        || fixture["source_revision"] != source
        || fixture["seed"] != plan["seed"]
        || fixture["workers_receive"] != "public only"
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || !(101..=1511).contains(&p)
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || answer == 0
        || answer >= l
        || read(&run.join("public.json"))? != *public
    {
        return Err("public fixture/protocol".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 6 {
        return Err("incomplete flow".into());
    }
    for (i, row) in flow.iter().enumerate() {
        let mode = ["fixture", "rank-stop", "structural-gate"][i / 2];
        let kind = match i {
            0 => "fixture_start",
            1 => "fixture",
            n if n % 2 == 0 => "execution_start",
            _ => "execution",
        };
        if row["mode"] != mode
            || row["record_type"] != kind
            || row["source_revision"] != source
            || (i % 2 == 1 && row["exit_code"] != 0)
        {
            return Err("flow identity/order/exit".into());
        }
    }
    let workers = [
        read(&run.join("rank-stop.raw.json"))?,
        read(&run.join("structural-gate.raw.json"))?,
    ];
    let mut outcomes = Vec::new();
    for (i, w) in workers.iter().enumerate() {
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["seed"] != plan["seed"]
            || w["method"] != "two-lp"
            || w["public"] != *public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("public scalar identity".into());
        }
        outcomes.push(json!({"arm":plan["mode_order"][i],"actual_scalar":answer,"verified":true,"accounting":full::counter_check(w,p,l)?,
            "candidate_proof":full::rank_stopped_candidate_proof(w,&plan,l,answer)?,"checkpoint_proof":graph::checkpoints(w,&plan,p,l)?,
            "recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"]}));
    }
    let proof = gate_proof(&workers[0], &workers[1], p, l)?;
    let summary = json!({"schema":"cover.two-lp-structural-gate-analysis/v1","source_revision":source,"p":p,"l":l,"seed":plan["seed"],
        "actual_scalar":answer,"outcomes":outcomes,"gate_proof":proof,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,
        "cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false,"flow":{"planned":2,"attempted":2,"completed":2,"excluded":0,"automatic_retries":0}});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    let export = json!({"schema":"cover.two-lp-structural-gate.raw-export/v1","canonical_schema_conformance_claimed":false,"canonical_experiment_export_schema_available":false,
        "protocol":plan,"fixture":fixture,"workers":workers,"flow":flow,"analysis":summary,"independent_signed_verdict":false});
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run.join("raw-export.json"))?;
    serde_json::to_writer(&mut out, &export)?;
    writeln!(out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn inputs(rows: &[Vec<(u64, u64)>]) -> Vec<Value> {
        rows.iter()
            .enumerate()
            .map(|(i, row)| json!({"id":i,"rhs":0,"large":row}))
            .collect()
    }
    #[test]
    fn independent_traversal_and_dense_rank_distinguish_anchors_from_weighted_cycles() {
        let rows = inputs(&[vec![(0, 1), (1, 1)], vec![(0, 1), (1, 2)]]);
        let s = structural_state(&rows, 101, 103).unwrap();
        assert_eq!(s["large_rank_lower_bound"], 1);
        assert_eq!(s["independent_large_rank"], 2);
        let mut anchored = rows;
        anchored.push(json!({"id":2,"rhs":0,"large":[[1,7]]}));
        let s = structural_state(&anchored, 101, 103).unwrap();
        assert_eq!(s["large_rank_lower_bound"], 2);
        assert_eq!(s["anchored_components"], 1);
        anchored[0]["rhs"] = json!(1);
        assert!(structural_state(&anchored, 101, 103).is_err());
    }
    #[test]
    fn unexplained_changes_in_any_phase_or_gate_event_charge_are_rejected() {
        let zeros: BTreeMap<String, u64> = full::shared::KINDS
            .into_iter()
            .map(|k| (k.into(), 0))
            .collect();
        let phases: Vec<_> = full::shared::PHASES
            .into_iter()
            .map(|name| json!({"phase":name,"primitives":zeros}))
            .collect();
        let mut a = json!({"work_native":{"phases":phases}});
        let mut b = a.clone();
        a["work_native"]["phases"][7]["primitives"]["modr_mul"] = json!(10);
        a["work_native"]["phases"][7]["primitives"]["word_op"] = json!(20);
        b["work_native"]["phases"][7]["primitives"]["modr_mul"] = json!(5);
        b["work_native"]["phases"][7]["primitives"]["word_op"] = json!(26);
        let mut removed = json!(zeros);
        removed["modr_mul"] = json!(5);
        removed["word_op"] = json!(7);
        assert!(phase_conservation(&a, &b, &[removed.clone()], 13).is_ok());
        assert!(phase_conservation(&a, &b, &[removed.clone()], 12).is_err());
        b["work_native"]["phases"][0]["primitives"]["word_op"] = json!(1);
        assert!(phase_conservation(&a, &b, &[removed.clone()], 13).is_err());
        b["work_native"]["phases"][0]["primitives"]["word_op"] = json!(0);
        b["work_native"]["phases"][7]["primitives"]["modr_mul"] = json!(6);
        assert!(phase_conservation(&a, &b, &[removed], 13).is_err());
    }
}
