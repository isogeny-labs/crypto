//! Rank-stopped collection: independent certificate/rank/prefix checks.
#[allow(dead_code)]
#[path = "two-lp-dlp-analysis.rs"]
pub(crate) mod full;
use full::shared::{accumulate, canonical, counts, n, prime, Terms};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

fn checkpoint_rows(
    check: &Value,
    inputs: &[Value],
    columns: u64,
    p: u64,
    l: u64,
) -> Result<Vec<(Terms, u64)>, String> {
    let certificates = check["certificates"]
        .as_array()
        .ok_or("checkpoint certificates")?;
    if certificates.len() > 4096
        || check["complete_rows"] != certificates.len() as u64
        || check["replayed_rows"] != certificates.len() as u64
    {
        return Err("checkpoint replay count".into());
    }
    let mut rows = Vec::new();
    for c in certificates {
        let row = canonical(&c["small"], columns, l)?;
        let sources = canonical(&c["sources"], inputs.len() as u64, l)?;
        if sources.is_empty() || c["rhs"] != 0 {
            return Err("homogeneous checkpoint RHS/sources".into());
        }
        let (mut a, mut b) = (BTreeMap::new(), BTreeMap::new());
        for (id, k) in sources {
            let input = &inputs[id as usize];
            if input["id"] != id || input["rhs"] != 0 {
                return Err("checkpoint source binding".into());
            }
            accumulate(&mut a, &canonical(&input["small"], columns, l)?, k, l);
            accumulate(&mut b, &canonical(&input["large"], p, l)?, k, l);
        }
        if b.values().any(|&v| v != 0)
            || a.into_iter().filter(|&(_, v)| v != 0).collect::<Terms>() != row
        {
            return Err("checkpoint combination replay".into());
        }
        rows.push((row, 0));
    }
    Ok(rows)
}

pub(crate) fn checkpoints(w: &Value, plan: &Value, p: u64, l: u64) -> Result<Value, String> {
    let v = &w["work"];
    let columns = n(v, "small_base")?;
    let target = columns.checked_sub(1).ok_or("empty small base")?;
    let stride = n(plan, "collection_rank_stride")?;
    let checks = v["collection_rank_checks"]
        .as_array()
        .ok_or("rank checks")?;
    let sources = v["source_relations"].as_array().ok_or("sources")?;
    let inputs: Vec<_> = sources.iter().map(|r| r["partial"].clone()).collect();
    if checks.is_empty()
        || checks.len() > 64
        || !(1..=4096).contains(&stride)
        || v["collection_rank_target"] != target
        || v["collection_rank_ready"] != true
    {
        return Err("rank-stop criteria".into());
    }
    let (mut previous_b, mut previous_sources, mut required) = (0, 0, target.max(1));
    let mut summed = BTreeMap::<String, u64>::new();
    let mut result = Vec::new();
    for (i, check) in checks.iter().enumerate() {
        let b = n(check, "b_count")?;
        let source_count = n(check, "source_rows")?;
        if b <= previous_b
            || b > n(v, "b_count_completed")?
            || source_count < required
            || source_count <= previous_sources
            || source_count > inputs.len() as u64
            || (!b.is_multiple_of(1024) && b != n(plan, "b_count")?)
        {
            return Err("rank-check collection order/bounds/stride".into());
        }
        let rows = checkpoint_rows(check, &inputs[..source_count as usize], columns, p, l)?;
        let (rank, _) = full::target_solution(&rows, columns as usize, l)?;
        let ready = rank as u64 >= target;
        let ops = n(check, "solver_modular_muls")?;
        let normalization = (rank as u64)
            .checked_mul(columns + 1)
            .ok_or("normalization overflow")?;
        let subtractions = ops
            .checked_sub(normalization)
            .ok_or("rank product coverage")?;
        let primitive = counts(&check["observed_primitives"])?;
        if check["rank"] != rank as u64
            || check["rank_target"] != target
            || check["ready"] != ready
            || ready != (i + 1 == checks.len())
            || primitive["modr_mul"] < ops
            || primitive["modr_add"] < subtractions
            || primitive["modr_inv"] < rank as u64
            || check["graph"]["input_relations"] != source_count
            || check["graph"]["complete_relations"] != rows.len() as u64
            || check["graph"]["rejected_fill"] != 0
        {
            return Err("checkpoint rank/stop/counter conservation".into());
        }
        for (k, value) in primitive {
            let a = summed.entry(k).or_default();
            *a = a.checked_add(value).ok_or("counter overflow")?;
        }
        result.push(json!({"b_count":b,"source_rows":source_count,"complete_rows":rows.len(),"independent_rank":rank,"rank_target":target,"ready":ready,"solver_modular_muls":ops,"graph":check["graph"]}));
        previous_b = b;
        previous_sources = source_count;
        required = source_count.checked_add(stride).ok_or("stride overflow")?;
    }
    let la = w["work_native"]["phases"]
        .as_array()
        .ok_or("phases")?
        .iter()
        .find(|r| r["phase"] == "linear_algebra")
        .ok_or("LA phase")?;
    let available = counts(&la["primitives"])?;
    if summed
        .iter()
        .any(|(k, v)| available.get(k).is_none_or(|a| v > a))
        || previous_b != n(v, "b_count_completed")?
        || previous_sources != inputs.len() as u64
    {
        return Err("checkpoint final-prefix/phase containment".into());
    }
    Ok(
        json!({"checks":result,"checkpoint_primitives":summed,"all_certificates_replayed":true,"independent_rank_checks_passed":true}),
    )
}

fn analyze(fixture: &Value, workers: &[Value], plan: &Value) -> Result<Value, String> {
    let public = &fixture["public"];
    let (p, l, expected) = (
        n(public, "p")?,
        n(public, "l")?,
        n(fixture, "expected_scalar")?,
    );
    let source = plan["source_revision"].as_str().ok_or("source")?;
    if workers.len() != 3
        || plan["driver_kind"] != "cover-two-lp-rank-stop-validation"
        || plan["mode_order"] != json!(["ic", "two-lp", "rank-stop"])
        || plan["rounds"] != 1
        || plan["warmup_rounds"] != 0
        || plan["automatic_retries"] != 0
        || plan["allow_comparative_claims"] != false
        || plan["fixture_timeout_seconds"] != 60
        || plan["worker_timeout_seconds"] != 900
        || plan["timeout_seconds"] != 2800
        || plan["p"] != p
        || plan["m"] != 9
        || n(plan, "b_count")? != p * p
        || fixture["source_revision"] != source
        || fixture["seed"] != plan["seed"]
        || fixture["schema"] != "cover.two-large-prime.dlp-fixture/v1"
        || source.len() != 40
        || !source.bytes().all(|b| b.is_ascii_hexdigit())
        || !(101..=1511).contains(&p)
        || !prime(p)
        || !prime(l)
        || !(3..1u64 << 63).contains(&l)
        || expected == 0
        || expected >= l
    {
        return Err("fixture/frozen protocol/domain".into());
    }
    let mut outcomes = Vec::new();
    for (i, w) in workers.iter().enumerate() {
        let mode = if i == 0 { "ic" } else { "two-lp" };
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["method"] != mode
            || w["seed"] != plan["seed"]
            || w["public"] != *public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != expected
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err("worker public/scalar identity".into());
        }
        outcomes.push(json!({"arm":plan["mode_order"][i],"actual_scalar":expected,"verified":true,"recorded_inner_wall_ns_practicality_only":w["inner_wall_ns"],"accounting":full::counter_check(w,p,l)?}));
    }
    if !workers[1]["work"]["options"]["collection_rank_stride"].is_null()
        || !workers[1]["work"]["collection_rank_checks"].is_null()
    {
        return Err("fixed cohort unexpectedly rank-stopped".into());
    }
    let fixed = full::candidate_proof(&workers[1], plan, l, expected)?;
    let adaptive = full::rank_stopped_candidate_proof(&workers[2], plan, l, expected)?;
    let checked = checkpoints(&workers[2], plan, p, l)?;
    let full_sources = workers[1]["work"]["source_relations"]
        .as_array()
        .ok_or("fixed sources")?;
    let prefix = workers[2]["work"]["source_relations"]
        .as_array()
        .ok_or("rank sources")?;
    if full_sources.get(..prefix.len()) != Some(prefix.as_slice()) {
        return Err("actual original-source prefix differs".into());
    }
    let a = workers[1]["work"]["descents"]
        .as_array()
        .ok_or("fixed descents")?;
    let b = workers[2]["work"]["descents"]
        .as_array()
        .ok_or("rank descents")?;
    if a[..a.len().min(b.len())] != b[..a.len().min(b.len())] {
        return Err("descent stream prefix differs".into());
    }
    Ok(
        json!({"schema":"cover.two-lp-rank-stop-analysis/v1","source_revision":source,"p":p,"l":l,"seed":plan["seed"],"actual_scalar":expected,"outcomes":outcomes,"fixed_proof":fixed,"rank_stopped_proof":adaptive,"rank_check_proof":checked,"original_source_prefix_equal":true,"common_descent_prefix_equal":true,"fixed_b_count":workers[1]["work"]["b_count_completed"],"rank_stopped_b_count":workers[2]["work"]["b_count_completed"],"fewer_b_values":n(&workers[2]["work"],"b_count_completed")?<n(plan,"b_count")?,"full_recovery_and_rank_validation_passed":true,"complete_cost":false,"total_bitops":null,"comparative_eligibility":false,"rho_parity":false,"independent_signed_verdict":false,"flow":{"planned":3,"attempted":3,"completed":3,"excluded":0,"automatic_retries":0}}),
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
        return Err("usage: rank-stop-analysis RUN-DIR NEW-SUMMARY.json".into());
    }
    let run = Path::new(&args[1]);
    for name in [
        "exit-code.txt",
        "fixture.exit-code.txt",
        "ic.exit-code.txt",
        "two-lp.exit-code.txt",
        "rank-stop.exit-code.txt",
    ] {
        if fs::read_to_string(run.join(name))?.trim() != "0" {
            return Err("failed/censored process".into());
        }
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
    if flow.len() != 8 {
        return Err("incomplete flow".into());
    }
    for (i, v) in flow.iter().enumerate() {
        let mode = ["fixture", "ic", "two-lp", "rank-stop"][i / 2];
        let kind = if i == 0 {
            "fixture_start"
        } else if i == 1 {
            "fixture"
        } else if i % 2 == 0 {
            "execution_start"
        } else {
            "execution"
        };
        if v["mode"] != mode
            || v["record_type"] != kind
            || v["source_revision"] != plan["source_revision"]
            || (i % 2 == 1 && v["exit_code"] != 0)
        {
            return Err("flow identity/exit".into());
        }
    }
    let workers = [
        read(&run.join("ic.raw.json"))?,
        read(&run.join("two-lp.raw.json"))?,
        read(&run.join("rank-stop.raw.json"))?,
    ];
    let summary = analyze(&fixture, &workers, &plan)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    // Native, self-described raw export. The encrypted canonical schema is
    // unavailable; do not claim experiment.export/v1 conformance.
    let mut originals = Vec::new();
    for file in [
        "runtime-plan.json",
        "fixture.json",
        "public.json",
        "ic.raw.json",
        "two-lp.raw.json",
        "rank-stop.raw.json",
        "flow.jsonl",
    ] {
        let bytes = fs::read(run.join(file))?;
        originals.push(json!({"file":file,"bytes":bytes.len(),"blake3":blake3::hash(&bytes).to_hex().to_string()}));
    }
    let export = json!({"schema":"cover.two-lp-rank-stop.raw-export/v1","canonical_experiment_export_schema_available":false,"canonical_schema_conformance_claimed":false,"source_revision":plan["source_revision"],"protocol":plan,"fixture":fixture,"workers":workers,"flow":flow,"analysis":summary,"exact_original_files":originals,"independent_signed_verdict":false});
    let mut raw_export = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(run.join("raw-export.json"))?;
    serde_json::to_writer(&mut raw_export, &export)?;
    writeln!(raw_export)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ready_homogeneous_rank_does_not_replace_actual_target_determination() {
        // Two small coordinates, homogeneous rank1=small-1. Two affine
        // descents can nevertheless be dependent, leaving log(Q) unknown.
        let rows = vec![
            (vec![(0, 1), (1, 102)], 0),
            (vec![(0, 1), (2, 102)], 2),
            (vec![(0, 2), (2, 101)], 4),
        ];
        assert_eq!(full::target_solution(&rows, 3, 103).unwrap(), (2, None));
        let mut more = rows;
        more.push((vec![(0, 1), (2, 101)], 5));
        assert_eq!(
            full::target_solution(&more, 3, 103).unwrap(),
            (3, Some(100))
        );
    }
    #[test]
    fn forged_rank_readiness_and_uncontained_counters_are_rejected() {
        let mut primitive: BTreeMap<String, u64> = full::shared::KINDS
            .into_iter()
            .map(|k| (k.into(), 0))
            .collect();
        primitive.insert("modr_mul".into(), 8);
        primitive.insert("modr_inv".into(), 2);
        let inputs = vec![
            json!({"id":0,"small":[[0,1]],"large":[],"rhs":0}),
            json!({"id":1,"small":[[1,1]],"large":[],"rhs":0}),
        ];
        let check = json!({"b_count":1024,"source_rows":2,"complete_rows":2,"replayed_rows":2,"rank":2,"rank_target":2,"ready":true,"solver_modular_muls":8,"graph":{"input_relations":2,"complete_relations":2,"rejected_fill":0},"certificates":[{"small":[[0,1]],"sources":[[0,1]],"rhs":0},{"small":[[1,1]],"sources":[[1,1]],"rhs":0}],"observed_primitives":primitive});
        let worker = json!({"work":{"small_base":3,"collection_rank_target":2,"collection_rank_ready":true,"collection_rank_checks":[check],"b_count_completed":1024,"source_relations":inputs.into_iter().map(|p|json!({"partial":p})).collect::<Vec<_>>()},"work_native":{"phases":[{"phase":"linear_algebra","primitives":primitive}]}});
        let plan = json!({"collection_rank_stride":128,"b_count":1018081});
        assert!(checkpoints(&worker, &plan, 103, 101).is_ok());
        let mut forged = worker.clone();
        forged["work"]["collection_rank_checks"][0]["rank"] = json!(1);
        assert!(checkpoints(&forged, &plan, 103, 101).is_err());
        let mut forged = worker.clone();
        forged["work"]["collection_rank_checks"][0]["ready"] = json!(false);
        assert!(checkpoints(&forged, &plan, 103, 101).is_err());
        let mut forged = worker;
        forged["work_native"]["phases"][0]["primitives"]["modr_mul"] = json!(7);
        assert!(checkpoints(&forged, &plan, 103, 101).is_err());
    }
    #[test]
    fn checkpoint_replay_rejects_target_rhs_and_wrong_original_combinations() {
        let inputs = vec![
            json!({"id":0,"small":[[0,1]],"large":[[7,1]],"rhs":0}),
            json!({"id":1,"small":[[1,1]],"large":[[7,1]],"rhs":0}),
        ];
        let mut c = json!({"complete_rows":1,"replayed_rows":1,"certificates":[{"small":[[0,1],[1,100]],"sources":[[0,1],[1,100]],"rhs":0}]});
        assert_eq!(
            checkpoint_rows(&c, &inputs, 3, 103, 101).unwrap(),
            vec![(vec![(0, 1), (1, 100)], 0)]
        );
        c["certificates"][0]["rhs"] = json!(1);
        assert!(checkpoint_rows(&c, &inputs, 3, 103, 101).is_err());
        c["certificates"][0]["rhs"] = json!(0);
        c["certificates"][0]["sources"][1][1] = json!(1);
        assert!(checkpoint_rows(&c, &inputs, 3, 103, 101).is_err());
        c["certificates"][0]["sources"][1][1] = json!(100);
        c["certificates"][0]["small"][1][0] = json!(3);
        assert!(checkpoint_rows(&c, &inputs, 3, 103, 101).is_err());
    }
}
