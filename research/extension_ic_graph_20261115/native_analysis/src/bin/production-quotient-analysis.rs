//! Fresh full-pipeline qualification and exact primitive-change conservation.
#![allow(clippy::duplicate_mod)]
#[allow(dead_code)]
#[path = "quotient-analysis.rs"]
mod component;
#[allow(dead_code)]
#[path = "balance-analysis.rs"]
mod graph;
use graph::gate::graph::{
    self as ranks,
    full::{
        self,
        shared::{canonical, counts, n, prime, KINDS, PHASES},
    },
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};
const MODES: [&str; 3] = ["ic", "structural-gate", "quotient-reuse"];
type Counter = BTreeMap<String, u64>;
fn zeros() -> Counter {
    KINDS.into_iter().map(|k| (k.into(), 0)).collect()
}
fn add(c: &mut Counter, key: &str, value: u64) -> Result<(), String> {
    let v = c.get_mut(key).ok_or("counter key")?;
    *v = v.checked_add(value).ok_or("counter overflow")?;
    Ok(())
}
fn dense_words(certs: &[Value], used: usize, columns: usize) -> Result<u64, String> {
    if used > certs.len() {
        return Err("conversion prefix".into());
    }
    let mut words = 0;
    for c in &certs[..used] {
        words +=
            columns as u64 + 2 * c["small"].as_array().ok_or("small coefficients")?.len() as u64;
    }
    Ok(words)
}
fn unchanged_graph(a: &Value, b: &Value) -> Result<(), String> {
    for key in [
        "source_relations",
        "descents",
        "cohort_digest",
        "b_count_completed",
        "collection_rank_target",
        "collection_rank_ready",
        "collection_structural_gate",
        "final_projection",
        "descent_residuals",
        "descent_stream_advances",
    ] {
        if a["work"][key] != b["work"][key] {
            return Err(format!("graph identity {key}"));
        }
    }
    Ok(())
}
fn conservation(a: &Value, b: &Value, removed: &Counter, added: &Counter) -> Result<Value, String> {
    let old = a["work_native"]["phases"]
        .as_array()
        .ok_or("baseline phases")?;
    let new = b["work_native"]["phases"]
        .as_array()
        .ok_or("candidate phases")?;
    if old.len() != PHASES.len() || new.len() != old.len() {
        return Err("phase count".into());
    }
    for ((a, b), phase) in old.iter().zip(new).zip(PHASES) {
        if a["phase"] != phase || b["phase"] != phase {
            return Err("phase order".into());
        }
        let mut expected = counts(&a["primitives"])?;
        if phase == "linear_algebra" {
            for key in KINDS {
                let v = expected.get_mut(key).ok_or("counter")?;
                *v = v
                    .checked_sub(removed[key])
                    .and_then(|v| v.checked_add(added[key]))
                    .ok_or("phase subtraction/addition")?;
            }
        }
        if counts(&b["primitives"])? != expected {
            return Err(format!("unexplained primitive change in {phase}"));
        }
    }
    Ok(
        json!({"all_phase_counts_conserved":true,"removed_original_affine_solver_and_conversion":removed,"added_quotient_solver_kernel_projection_and_conversion":added}),
    )
}
fn reuse(a: &Value, b: &Value, plan: &Value, l: u64, answer: u64) -> Result<Value, String> {
    unchanged_graph(a, b)?;
    if !a["work"]["homogeneous_quotient_reuse"].is_null() {
        return Err("baseline unexpectedly reuses".into());
    }
    let m = component::driver::saved::decode(a)?;
    if m.original_scalar != answer {
        return Err("baseline target".into());
    }
    let base_checks = a["work"]["collection_rank_checks"]
        .as_array()
        .ok_or("base checkpoints")?;
    let new_checks = b["work"]["collection_rank_checks"]
        .as_array()
        .ok_or("new checkpoints")?;
    if base_checks.is_empty() || base_checks.len() != new_checks.len() {
        return Err("checkpoint identity".into());
    }
    let last = base_checks.last().ok_or("checkpoint")?;
    let mut homogeneous = Vec::new();
    for cert in last["certificates"]
        .as_array()
        .ok_or("homogeneous certificates")?
    {
        if cert["rhs"] != 0 {
            return Err("homogeneous RHS".into());
        }
        let mut row = vec![0; m.target];
        for (c, v) in canonical(&cert["small"], m.target as u64, l)? {
            row[c as usize] = v;
        }
        homogeneous.push((row, 0));
    }
    let hom = component::oracle(l, m.target, &homogeneous, None)?;
    let old = component::oracle(l, m.columns, &m.rows, Some(m.target))?;
    if hom.basis.len() + 1 != m.target
        || old.answer != Some(answer)
        || old.counts.products != m.original_products
        || last["solver_modular_muls"] != hom.counts.products
    {
        return Err("independent original rank/arithmetic".into());
    }
    let free = (0..m.target)
        .find(|c| !hom.basis.contains_key(c))
        .ok_or("free column")?;
    let mut h = vec![0; m.target];
    h[free] = 1;
    let mut negations = 0;
    for (&c, (row, rhs)) in &hom.basis {
        if *rhs != 0 {
            return Err("homogeneous basis RHS".into());
        }
        h[c] = if row[free] == 0 {
            0
        } else {
            negations += 1;
            l - row[free]
        };
    }
    let ncols = m.target as u64;
    let rank = hom.basis.len() as u64;
    let initial_words = 4 * ncols + 1 + 8 * rank + 4 * rank * ncols;
    let record = &b["work"]["homogeneous_quotient_reuse"];
    if record["schema"] != "two-large-prime.homogeneous-quotient-reuse/v1"
        || record["native_bookkeeping_complete"] != false
        || record["rank_scope"].as_str().is_none_or(str::is_empty)
        || record["solver_product_scope"]
            .as_str()
            .is_none_or(str::is_empty)
        || record["known_uncounted"].as_str().is_none_or(str::is_empty)
    {
        return Err("explicit reuse scope".into());
    }
    let initial = json!({"free_column":free,"kernel":h,"modulus":l,"counts":{"products":0,"additions":negations,"word_events":initial_words}});
    if record["initial_basis"] != initial {
        return Err("unique normalized retained kernel/charges".into());
    }
    for (i, (a, b)) in base_checks.iter().zip(new_checks).enumerate() {
        let (mut left, mut right) = (a.clone(), b.clone());
        left.as_object_mut()
            .ok_or("checkpoint")?
            .remove("observed_primitives");
        right
            .as_object_mut()
            .ok_or("checkpoint")?
            .remove("observed_primitives");
        if left != right {
            return Err("checkpoint matrix/rank/schedule identity".into());
        }
        let mut expected = counts(&a["observed_primitives"])?;
        if i + 1 == base_checks.len() {
            add(&mut expected, "modr_add", negations)?;
            add(&mut expected, "word_op", initial_words)?;
        }
        if counts(&b["observed_primitives"])? != expected {
            return Err("checkpoint kernel-only primitive change".into());
        }
    }
    let mut projected = Vec::new();
    let mut rows = Vec::new();
    for (i, (row, rhs)) in m.rows.iter().enumerate() {
        let dot = row[..m.target].iter().zip(&h).fold(0, |a, (&v, &c)| {
            full::shared::add(a, full::shared::mul(v, c, l), l)
        });
        projected.push(json!({"row":i,"coefficients":[dot,row[m.target]],"rhs":rhs}));
        rows.push((vec![dot, row[m.target]], *rhs));
    }
    let new = component::oracle(l, 2, &rows, Some(1))?;
    let attempted = record["attempts"].as_array().ok_or("reuse attempts")?;
    if attempted.len() != 1
        || b["work"]["projection_solve_attempts"]
            .as_array()
            .is_none_or(|v| v.len() != 1)
    {
        return Err("one successful affine attempt required".into());
    }
    let r = &attempted[0];
    let used = new.states.len();
    let mut dots = 0u64;
    for (row, _) in &m.rows[..used] {
        dots += row[..m.target]
            .iter()
            .zip(&h)
            .filter(|&(a, b)| *a != 0 && *b != 0)
            .count() as u64;
    }
    let projection_words = (5 * ncols + 4) * used as u64;
    let extra = json!({"products":dots,"additions":dots,"word_events":projection_words});
    let total_extra = json!({"products":dots,"additions":dots+negations,"word_events":projection_words+initial_words});
    if new.answer != Some(answer)
        || r["actual_scalar"] != answer
        || r["accepted_descents"] != b["work"]["descents"].as_array().ok_or("descents")?.len()
        || r["solver_columns"] != 2
        || r["target_column"] != 1
        || r["rank"] != new.basis.len()
        || r["solver_modular_muls"] != new.counts.products
        || r["normalization_products"] != new.counts.normalization_products
        || r["subtraction_ops"] != new.counts.subtractions
        || r["projected_rows"] != json!(&projected[..used])
        || r["extra_counts"] != extra
        || record["final_extra_counts"] != total_extra
        || b["work"]["last_rank"] != new.basis.len()
        || b["work"]["solver_modular_muls"] != new.counts.products
    {
        return Err("executed quotient target/counters/projected prefix".into());
    }
    let actual_attempt = &b["work"]["projection_solve_attempts"][0];
    if actual_attempt["rank"] != new.basis.len()
        || actual_attempt["solver_modular_muls"] != new.counts.products
        || actual_attempt["verified"] != true
        || actual_attempt["candidate_scalar"] != answer
    {
        return Err("actual affine attempt".into());
    }
    let certs = a["work"]["final_projection"]["certificates"]
        .as_array()
        .ok_or("certificates")?;
    let mut removed = zeros();
    let mut added = zeros();
    for (key, value) in [
        ("modr_mul", old.counts.products),
        ("modr_add", old.counts.subtractions),
        ("modr_inv", old.counts.inversions),
        ("word_op", dense_words(certs, old.states.len(), m.columns)?),
    ] {
        add(&mut removed, key, value)?;
    }
    for (key, value) in [
        ("modr_mul", new.counts.products + dots),
        ("modr_add", new.counts.subtractions + dots + negations),
        ("modr_inv", new.counts.inversions),
        (
            "word_op",
            dense_words(certs, used, m.columns)? + initial_words + projection_words,
        ),
    ] {
        add(&mut added, key, value)?;
    }
    let proof = conservation(a, b, &removed, &added)?;
    if plan["collection_rank_stride"] != b["work"]["options"]["collection_rank_stride"] {
        return Err("stride identity".into());
    }
    Ok(
        json!({"same_original_sources_descents_endpoint_projection":true,"same_checkpoint_matrices_and_rank":true,"independent_homogeneous_rank":hom.basis.len(),"independent_full_affine_rank":old.basis.len(),"independent_quotient_rank":new.basis.len(),"baseline_rows_used":old.states.len(),"candidate_rows_used":used,"actual_scalar":answer,"initial_kernel_counts":initial["counts"],"projection_counts":extra,"all_projected_rows_and_target_checked":true,"phase_conservation":proof}),
    )
}
fn protocol(plan: &Value, fixture: &Value) -> Result<(u64, u64, u64), String> {
    let frozen: Value =
        serde_json::from_str(include_str!("../../../production-quotient-plan.json"))
            .map_err(|_| "compiled plan")?;
    for (key, value) in frozen.as_object().ok_or("plan")? {
        if key != "source_revision" && plan[key] != *value {
            return Err(format!("frozen protocol {key}"));
        }
    }
    let source = plan["source_revision"].as_str().ok_or("source")?;
    let public = &fixture["public"];
    let (p, l, answer) = (
        n(public, "p")?,
        n(public, "l")?,
        n(fixture, "expected_scalar")?,
    );
    if source.len() != 40
        || !source.bytes().all(|c| c.is_ascii_hexdigit())
        || plan["pinned_cpu"]
            .as_str()
            .is_none_or(|c| c.parse::<u32>().is_err())
        || plan["runpod_allocation"]["provider"] != "runpod"
        || plan["runpod_allocation"]["compute_type"] != "CPU"
        || plan["runpod_allocation"]["vcpuCount"] != 4
        || plan["runpod_allocation"]["memoryInGb"] != 8
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || fixture["workers_receive"] != "public only"
        || fixture["source_revision"] != source
        || fixture["seed"] != plan["seed"]
        || p != 1009
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || answer == 0
        || answer >= l
    {
        return Err("public fixture/source/CPU identity".into());
    }
    Ok((p, l, answer))
}
fn read(path: &Path) -> Result<Value, Box<dyn Error>> {
    if fs::metadata(path)?.len() > 32 * 1024 * 1024 {
        return Err("record exceeds32MiB".into());
    }
    Ok(serde_json::from_reader(File::open(path)?)?)
}
fn write_new(path: &Path, value: &Value, pretty: bool) -> Result<(), Box<dyn Error>> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(path)?;
    if pretty {
        serde_json::to_writer_pretty(&mut f, value)?;
    } else {
        serde_json::to_writer(&mut f, value)?;
    }
    writeln!(f)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: production-quotient-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in [
        "exit-code.txt",
        "fixture.exit-code.txt",
        "ic.exit-code.txt",
        "structural-gate.exit-code.txt",
        "quotient-reuse.exit-code.txt",
    ] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
            return Err("failed/censored arm retained without scalar admission".into());
        }
    }
    let plan = read(&run.join("runtime-plan.json"))?;
    let fixture = read(&run.join("fixture.json"))?;
    let (p, l, answer) = protocol(&plan, &fixture)?;
    if read(&run.join("public.json"))? != fixture["public"] {
        return Err("public file identity".into());
    }
    let flow: Vec<Value> = fs::read_to_string(run.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    if flow.len() != 8 {
        return Err("incomplete flow".into());
    }
    for (i, row) in flow.iter().enumerate() {
        let mode = ["fixture", "ic", "structural-gate", "quotient-reuse"][i / 2];
        let kind = if i == 0 {
            "fixture_start"
        } else if i == 1 {
            "fixture"
        } else if i.is_multiple_of(2) {
            "execution_start"
        } else {
            "execution"
        };
        if row["mode"] != mode
            || row["record_type"] != kind
            || row["source_revision"] != plan["source_revision"]
            || (!i.is_multiple_of(2) && row["exit_code"] != 0)
        {
            return Err("flow identity/status".into());
        }
    }
    let mut outcomes = Vec::new();
    let mut workers = Vec::new();
    for (i, mode) in MODES.iter().enumerate() {
        let w = read(&run.join(format!("{mode}.raw.json")))?;
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != plan["source_revision"]
            || w["seed"] != plan["seed"]
            || w["public"] != fixture["public"]
            || w["method"] != if i == 0 { "ic" } else { "two-lp" }
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("public worker scalar/identity".into());
        }
        let accounting = full::counter_check(&w, p, l)?;
        let (mut affine, mut checkpoint, mut gate) = (Value::Null, Value::Null, Value::Null);
        if i > 0 {
            affine = full::rank_stopped_candidate_proof(&w, &plan, l, answer)?;
            checkpoint = ranks::checkpoints(&w, &plan, p, l)?;
            gate = graph::gate_check(&w, &plan, p, l)?;
        }
        outcomes.push(json!({"arm":mode,"actual_scalar":answer,"verified":true,"accounting":accounting,"candidate_proof":affine,"checkpoint_proof":checkpoint,"gate_proof":gate,"recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"],"ranking_eligible":false,"total_bitops":null,"pricing_band":null}));
        workers.push(w);
    }
    let proof = reuse(&workers[1], &workers[2], &plan, l, answer)?;
    let summary = json!({"schema":"cover.production-quotient-analysis/v1","source_revision":plan["source_revision"],"p":p,"l":l,"seed":plan["seed"],"actual_scalar":answer,"outcomes":outcomes,"reuse_proof":proof,"flow":{"planned":3,"attempted":3,"completed":3,"excluded":0,"automatic_retries":0},"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"cost_ratios":null,"rho_parity":false,"independent_signed_verdict":false});
    write_new(Path::new(&args[2]), &summary, true)?;
    let export = json!({"schema":"cover.production-quotient.raw-export/v1","canonical_schema_conformance_claimed":false,"canonical_experiment_export_schema_available":false,"protocol":plan,"fixture":fixture,"workers":workers,"analysis":summary,"flow":flow});
    write_new(&run.join("raw-export.json"), &export, false)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_hand_reuse_record_and_forged_fields() {
        let l = 101;
        let hrows = vec![(vec![1, 2], 0)];
        let rows = vec![(vec![1, 2, 0], 0), (vec![3, 4, 1], 17), (vec![1, 0, 2], 9)];
        let old = component::oracle(l, 3, &rows, Some(2)).unwrap();
        let mut cp = zeros();
        cp.insert("modr_mul".into(), 3);
        cp.insert("modr_inv".into(), 1);
        cp.insert("word_op".into(), 6);
        let mut a = json!({"schema":"cover.two-large-prime.dlp-worker/v1","method":"two-lp","verified":true,"capped":false,"public":{"p":103,"l":l},"source_revision":"a".repeat(40),"actual_scalar":93,
            "work":{"source_relations":[],"descents":[0,1],"collection_rank_target":1,"collection_rank_ready":true,"b_count_completed":1024,"options":{"collection_rank_stride":32},
            "collection_rank_checks":[{"rank":1,"ready":true,"rank_target":1,"solver_modular_muls":3,"complete_rows":1,"replayed_rows":1,"certificates":[{"small":[[0,1],[1,2]],"rhs":0}],"observed_primitives":cp}],
            "final_projection":{"target_column":2,"certificates":[{"small":[[0,1],[1,2]],"rhs":0},{"small":[[0,3],[1,4],[2,1]],"rhs":17},{"small":[[0,1],[2,2]],"rhs":9}]},
            "last_rank":3,"solver_modular_muls":old.counts.products,"projection_solve_attempts":[{"rank":3,"solver_modular_muls":old.counts.products,"candidate_scalar":93,"verified":true}]},
            "work_native":{"phases":PHASES.map(|p|json!({"phase":p,"primitives":zeros()}))}});
        let mut base = zeros();
        base.insert("modr_mul".into(), 3 + old.counts.products);
        base.insert("modr_add".into(), old.counts.subtractions);
        base.insert("modr_inv".into(), 1 + old.counts.inversions);
        base.insert("word_op".into(), 29);
        a["work_native"]["phases"][7]["primitives"] = json!(base);
        let mut b = a.clone();
        b["work"]["last_rank"] = json!(2);
        b["work"]["solver_modular_muls"] = json!(11);
        b["work"]["projection_solve_attempts"][0] =
            json!({"rank":2,"solver_modular_muls":11,"candidate_scalar":93,"verified":true});
        b["work"]["homogeneous_quotient_reuse"] = json!({"schema":"two-large-prime.homogeneous-quotient-reuse/v1","native_bookkeeping_complete":false,"rank_scope":"quotient only","solver_product_scope":"two-column RREF only","known_uncounted":"native control",
            "initial_basis":{"free_column":1,"kernel":[99,1],"modulus":l,"counts":{"products":0,"additions":1,"word_events":25}},
            "attempts":[{"actual_scalar":93,"accepted_descents":2,"solver_columns":2,"target_column":1,"rank":2,"solver_modular_muls":11,"normalization_products":6,"subtraction_ops":5,
            "projected_rows":[{"row":0,"coefficients":[0,0],"rhs":0},{"row":1,"coefficients":[99,1],"rhs":17},{"row":2,"coefficients":[99,2],"rhs":9}],"extra_counts":{"products":5,"additions":5,"word_events":42}}],"final_extra_counts":{"products":5,"additions":6,"word_events":67}});
        let mut new = zeros();
        new.insert("modr_mul".into(), 19);
        new.insert("modr_add".into(), 11);
        new.insert("modr_inv".into(), 3);
        new.insert("word_op".into(), 96);
        b["work_native"]["phases"][7]["primitives"] = json!(new);
        b["work"]["collection_rank_checks"][0]["observed_primitives"]["modr_add"] = json!(1);
        b["work"]["collection_rank_checks"][0]["observed_primitives"]["word_op"] = json!(31);
        let plan = json!({"collection_rank_stride":32});
        assert!(reuse(&a, &b, &plan, l, 93).is_ok());
        for pointer in [
            "/work/homogeneous_quotient_reuse/initial_basis/kernel/0",
            "/work/homogeneous_quotient_reuse/attempts/0/projected_rows/1/coefficients/0",
            "/work/homogeneous_quotient_reuse/attempts/0/subtraction_ops",
            "/work/homogeneous_quotient_reuse/final_extra_counts/word_events",
            "/work/solver_modular_muls",
            "/work_native/phases/7/primitives/modr_mul",
        ] {
            let mut bad = b.clone();
            let field = bad.pointer_mut(pointer).unwrap();
            *field = json!(field.as_u64().unwrap() + 1);
            assert!(reuse(&a, &bad, &plan, l, 93).is_err());
        }
        assert_eq!(
            component::oracle(l, 2, &hrows, None).unwrap().basis.len(),
            1
        );
    }
    #[test]
    fn every_phase_unexplained_change_is_rejected() {
        let mut removed = zeros();
        removed.insert("modr_mul".into(), 3);
        let mut added = zeros();
        added.insert("modr_mul".into(), 1);
        added.insert("word_op".into(), 7);
        let mut a = json!({"work_native":{"phases":PHASES.map(|p|json!({"phase":p,"primitives":zeros()}))}});
        a["work_native"]["phases"][7]["primitives"]["modr_mul"] = json!(3);
        let mut b = a.clone();
        b["work_native"]["phases"][7]["primitives"]["modr_mul"] = json!(1);
        b["work_native"]["phases"][7]["primitives"]["word_op"] = json!(7);
        assert!(conservation(&a, &b, &removed, &added).is_ok());
        for i in 0..PHASES.len() {
            let mut bad = b.clone();
            bad["work_native"]["phases"][i]["primitives"]["field_add"] = json!(1);
            assert!(conservation(&a, &bad, &removed, &added).is_err());
        }
    }
    #[test]
    fn conversion_prefix_and_graph_identity_controls() {
        let rows = vec![json!({"small":[[0,1],[1,2]]}), json!({"small":[[2,1]]})];
        assert_eq!(dense_words(&rows, 1, 3).unwrap(), 7);
        assert_eq!(dense_words(&rows, 2, 3).unwrap(), 12);
        assert!(dense_words(&rows, 3, 3).is_err());
        let a = json!({"work":{"descents":[1],"source_relations":[2]}});
        assert!(unchanged_graph(&a, &a).is_ok());
        let mut b = a.clone();
        b["work"]["descents"] = json!([3]);
        assert!(unchanged_graph(&a, &b).is_err());
    }
}
