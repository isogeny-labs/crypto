//! Complete deferred-graph comparison and registered four-configuration rho replay.
#![allow(clippy::duplicate_mod)]
#[cfg(test)]
#[allow(dead_code, unused_imports)] // Standalone example's reexport when nested for unit fixtures.
#[path = "jv_sieve_quadratic_pair_relation_controls.rs"]
mod admission_controls;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_pipeline_check.rs"]
mod check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_collection_check.rs"]
mod collection_check;
#[path = "../research/extension_ic_graph_20261115/deferred_pair_graph_check.rs"]
mod deferred_graph_math_check;
#[cfg(test)]
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_deferred_pair_graph.rs"]
mod deferred_pair_graph;
#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[path = "jv_sieve_quartic_discriminant_controls.rs"]
mod discriminant_controls;
#[path = "../research/extension_ic_graph_20261115/quartic_discriminant_check.rs"]
mod discriminant_math_check;
#[allow(dead_code, clippy::type_complexity)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder_driver.rs"]
mod driver;
#[path = "../research/extension_ic_graph_20261115/discriminant_pair_enumeration_check.rs"]
mod enumeration_math_check;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_graph_check.rs"]
mod graph_math_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/deferred_pair_pipeline_check.rs"]
mod pair_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/deferred_pair_collection_check.rs"]
mod pair_collection_check;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_driver.rs"]
mod pair_driver;
#[path = "../research/extension_ic_graph_20261115/quadratic_pair_certificate_check.rs"]
mod pair_math_check;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../research/extension_ic_graph_20261115/extension_remainder.rs"]
mod remainder;
#[allow(dead_code)]
#[path = "../research/extension_ic_graph_20261115/extension_sparse_math_check.rs"]
mod sparse_math_check;
// The production wire type is deserialized only. Independent arithmetic and
// graph replay do not invoke its constructors, splitter or replay helpers.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code, clippy::too_many_arguments)]
#[path = "../src/cryptanalysis/jv_principal_relation.rs"]
mod principal_relation;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_graph.rs"]
mod quadratic_pair_graph;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_quadratic_pair_relation.rs"]
mod quadratic_pair_relation;
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const ROLES: [&str; 8] = [
    "original-ic",
    "quadratic-table",
    "pair-discriminant",
    "pair-deferred",
    "rho-plain-b1",
    "rho-neg-b1",
    "rho-plain-b16",
    "rho-neg-b16",
];
const METHODS: [&str; 8] = [
    "ic",
    "extension-sparse-remainders",
    "discriminant-pairs",
    "deferred-pairs",
    "rho",
    "rho",
    "rho",
    "rho",
];
fn plan_ok(p: &Value) -> bool {
    p["schema"] == "cover.deferred-pair.runpod-validation/v1"
        && p["backend"] == "runpod_user_authorized"
        && p["p"] == 1009
        && p["seed"] == 389
        && p["m"] == 9
        && p["small_percent"] == 100
        && p["b_count"] == 1018081
        && p["source_cap"] == 262144
        && p["max_split_records"] == 524288
        && p["max_screen_refusals"] == 1_000_000
        && p["screen_method"] == "euclidean-discriminant-character"
        && p["max_vertices"] == 65536
        && p["deferred_max_vertices"] == 262144
        && p["max_deferred_edges"] == 262144
        && p["projection_activation_field"] == "activation_source"
        && p["rho_portfolio"]
            == json!([
            {"negation":false,"batch":1},{"negation":true,"batch":1},
            {"negation":false,"batch":16},{"negation":true,"batch":16}])
        && p["max_witness_terms"] == 256
        && p["split_attempt_cap"] == 8
        && p["split_seed_xor"] == "0x5155414450414952"
        && p["collection_batch_size"] == 1024
        && p["sieve_selection"] == "counter-table-five-ascending"
        && p["max_descent_residuals"] == 8192
        && p["max_accepted_descents"] == 32
        && p["mode_order"] == json!(ROLES)
        && p["rounds"] == 1
        && p["warmup_rounds"] == 0
        && p["automatic_retries"] == 0
        && p["fixture_timeout_seconds"] == 60
        && p["worker_timeout_seconds"] == 360
        && p["timeout_seconds"] == 3200
        && p["qualifier_timeout_seconds"] == 1800
        && p["runpod_lifetime_seconds"] == 7200
        && p["memory_gib"] == 8
        && p["complete_total"].is_null()
        && p["rho_parity"] == false
        && p["allow_comparative_claims"] == false
        && p["public_sha256"] == "b3ae4f93974456bdd6a772caf7daa0208d64c17f7341f70ee4187d327fd6462e"
        && p["sparse_rank_options"]
            == json!({"surplus":8,"failed_checkpoint_stride":16,"max_checkpoints":8,"max_preconditioners":3})
        && p["source_revision"]
            .as_str()
            .is_some_and(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        && p["runpod_allocation"]["compute_type"] == "CPU"
        && p["runpod_allocation"]["vcpuCount"] == 4
        && p["runpod_allocation"]["memoryInGb"] == 8
        && p["runpod_allocation"]["lifetime_seconds"] == 7200
        && p["runpod_allocation"]["costPerHr"]
            .as_f64()
            .is_some_and(|x| x > 0.0 && x <= 0.40)
}
fn common_sources(baseline: &Value, candidate: &Value) -> Result<Value> {
    let a = &baseline["work"]["collection"];
    let b = &candidate["work"]["collection"];
    if a["base"] != b["base"] || a["h"] != b["h"] {
        return Err("common model/base".into());
    }
    let mut prefixes = 0usize;
    let mut common_b = 0u64;
    for (x, y) in a["checkpoints"]
        .as_array()
        .ok_or("baseline checkpoints")?
        .iter()
        .zip(b["checkpoints"].as_array().ok_or("candidate checkpoints")?)
    {
        for k in ["b_count", "lines", "cohort_sha256"] {
            if x[k] != y[k] {
                return Err(format!("common cohort {k}").into());
            }
        }
        prefixes += 1;
        common_b = x["b_count"].as_u64().ok_or("common B")?;
    }
    if prefixes == 0 {
        return Err("no common complete prefix".into());
    }
    let keys = |c: &Value, new: bool| -> Result<BTreeSet<String>> {
        let mut out = BTreeSet::new();
        for s in c["sources"].as_array().ok_or("sources")? {
            if s["b_index"].as_u64().ok_or("source B")? >= common_b {
                continue;
            }
            let row = &s["certificate"];
            let large: Vec<Value> = if new {
                row["remainders"].as_array().ok_or("remainders")?.clone()
            } else if row["remainder"].is_null() {
                vec![]
            } else {
                vec![row["remainder"].clone()]
            };
            if large.len() > 1 {
                continue;
            }
            let terms: Vec<(usize, i8)> = serde_json::from_value(row["terms"].clone())?;
            let orient = if let Some(q) = large.first() {
                q["coefficient"].as_i64().ok_or("class sign")? as i8
            } else {
                terms.first().ok_or("small terms")?.1
            };
            let small: Vec<_> = terms.into_iter().map(|(i, s)| (i, s * orient)).collect();
            let large: Vec<_> = large
                .iter()
                .map(|q| {
                    json!([
                        q["key"],
                        q["coefficient"].as_i64().unwrap_or(0) * orient as i64
                    ])
                })
                .collect();
            out.insert(serde_json::to_string(&json!([s["b_index"], small, large]))?);
        }
        Ok(out)
    };
    let left = keys(a, false)?;
    let right = keys(b, true)?;
    if left != right {
        return Err("common full/one-class source set equality".into());
    }
    Ok(
        json!({"common_complete_prefixes":prefixes,"common_b_count":common_b,"equal_full_and_one_class_sources":left.len()}),
    )
}
fn matched_pair_geometry(plain: &Value, screened: &Value) -> Result<Value> {
    let a = &plain["work"]["collection"];
    let b = &screened["work"]["collection"];
    if a["mode"] != "discriminant-quadratic-pairs" || b["mode"] != "deferred-discriminant-pairs" {
        return Err("paired retention policy".into());
    }
    for key in [
        "base",
        "h",
        "rank_target",
        "rank_options",
        "split_options",
        "max_split_records",
        "split_seed_xor",
        "root_finder",
        "sieve_selection",
    ] {
        if a[key] != b[key] {
            return Err(format!("paired common policy {key}").into());
        }
    }
    let ab = a["counts"]["b_count"].as_u64().ok_or("baseline B")?;
    let bb = b["counts"]["b_count"].as_u64().ok_or("candidate B")?;
    let short = if ab <= bb { a } else { b };
    let common_b = ab.min(bb);
    let cutoff = short["counts"]["candidates"]
        .as_u64()
        .ok_or("common candidate prefix")?;
    let sa = a["sources"].as_array().ok_or("baseline sources")?;
    let sb = b["sources"].as_array().ok_or("candidate sources")?;
    let left: Vec<_> = sa
        .iter()
        .take_while(|s| s["b_index"].as_u64().is_some_and(|v| v < common_b))
        .collect();
    let right: Vec<_> = sb
        .iter()
        .take_while(|s| s["b_index"].as_u64().is_some_and(|v| v < common_b))
        .collect();
    if left.len() != right.len() || left.is_empty() {
        return Err("common source prefix size".into());
    }
    for (x, y) in left.iter().zip(&right) {
        for key in [
            "candidate_index",
            "b_index",
            "a0",
            "a1",
            "b",
            "s",
            "certificate",
        ] {
            if x[key] != y[key] {
                return Err(format!("common exact source {key}").into());
            }
        }
    }
    for key in ["splits", "discriminant_screen/refusals"] {
        let xa = if key == "splits" {
            &a["splits"]
        } else {
            &a["discriminant_screen"]["refusals"]
        };
        let xb = if key == "splits" {
            &b["splits"]
        } else {
            &b["discriminant_screen"]["refusals"]
        };
        let la = xa
            .as_array()
            .ok_or("request history")?
            .iter()
            .take_while(|r| r["candidate_index"].as_u64().is_some_and(|id| id <= cutoff));
        let lb = xb
            .as_array()
            .ok_or("request history")?
            .iter()
            .take_while(|r| r["candidate_index"].as_u64().is_some_and(|id| id <= cutoff));
        if !la.eq(lb) {
            return Err("common exact split/screen history".into());
        }
    }
    let ca = a["checkpoints"].as_array().ok_or("baseline cohorts")?;
    let cb = b["checkpoints"].as_array().ok_or("candidate cohorts")?;
    for (x, y) in ca.iter().zip(cb) {
        for key in ["b_count", "lines", "cohort_sha256"] {
            if x[key] != y[key] {
                return Err("common complete cohort prefix".into());
            }
        }
    }
    let post_collection = pair_check::paired_pipeline(plain, screened)?;
    Ok(
        json!({"common_b_count":common_b,"common_candidate_prefix":cutoff,"equal_original_source_prefix":left.len(),"exact_split_and_screen_prefix":true,"exact_common_cohorts":true,"graph_and_rank_replayed_separately":true,"checkpoint_elapsed_times_compared":false,"post_collection":post_collection}),
    )
}
fn write_new(path: &Path, v: &Value) -> Result<()> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    serde_json::to_writer_pretty(&mut f, v)?;
    f.write_all(b"\n")?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: CHECK RUN NEW_SUMMARY".into());
    }
    let dir = Path::new(&args[0]);
    let read =
        |name: &str| -> Result<Value> { Ok(serde_json::from_slice(&fs::read(dir.join(name))?)?) };
    let p = read("runtime-plan.json")?;
    let public = read("public.json")?;
    let fixture = read("fixture.json")?;
    let source = p["source_revision"].as_str().ok_or("source")?;
    let answer = fixture["expected_scalar"]
        .as_u64()
        .ok_or("fixture scalar")?;
    let sha = driver::digest(&fs::read(dir.join("public.json"))?);
    if !plan_ok(&p)
        || fixture["public"] != public
        || fixture["seed"] != 389
        || fixture["source_revision"] != source
        || fixture["workers_receive"] != "public only"
        || p["public_sha256"] != sha
        || public["p"] != 1009
        || public["l"] != 263807419614139553u64
        || answer == 0
        || answer >= 263807419614139553
    {
        return Err("registration/public/fixture".into());
    }
    let curve: driver::PublicCurve = serde_json::from_value(public.clone())?;
    collection_check::validate_public(&curve)?;
    let f = crypto_lib::cryptanalysis::jv_cover::Fq3::new(1009);
    let alpha = crypto_lib::cryptanalysis::jv_cover::E6(
        curve.alpha.map(crypto_lib::cryptanalysis::jv_cover::E2),
    );
    let ec = crypto_lib::cryptanalysis::jv_cover::EllE::new(&f, &alpha);
    if ec.mul(&curve.g.native(), answer) != curve.q.native() {
        return Err("independent elliptic endpoint".into());
    }
    let mut workers = Vec::new();
    let mut outcomes = Vec::new();
    for (i, role) in ROLES.iter().enumerate() {
        if fs::read_to_string(dir.join(format!("{role}.exit-code.txt")))?.trim() != "0" {
            return Err(format!("failed worker {role}").into());
        }
        let w = read(&format!("{role}.raw.json"))?;
        if w["schema"] != "cover.two-large-prime.dlp-worker/v1"
            || w["source_revision"] != source
            || w["method"] != METHODS[i]
            || w["seed"] != 389
            || w["public"] != public
            || w["solver_answer_placeholder"] != 0
            || w["actual_scalar"] != answer
            || w["verified"] != true
            || w["capped"] != false
        {
            return Err(format!("worker identity {role}").into());
        }
        let proof = match i {
            0 => json!({"accounting":check::full::counter_check(&w,1009,263807419614139553)?}),
            1 => {
                if w["work"]["collection"]["root_finder"] != "forward-differences" {
                    return Err("baseline table root policy".into());
                }
                let c = check::collection(&w, &sha)?;
                let cp = collection_check::qualify_sparse(&c, 389, &sha, &w["work"]["collection"])?;
                check::pipeline(&w, &c, &cp, answer, 8192, 32)?
            }
            2 | 3 => {
                let c = pair_check::collection(&w, &sha)?;
                let cp =
                    pair_collection_check::qualify_sparse(&c, 389, &sha, &w["work"]["collection"])?;
                pair_check::pipeline(&w, &c, &cp, answer, 8192, 32)?
            }
            _ => {
                pair_check::rho_identity(
                    &w,
                    i == 5 || i == 7,
                    if i < 6 { 1 } else { 16 },
                    1009,
                    263807419614139553,
                    389,
                )?;
                json!({"accounting":pair_check::rho::account(&w,true,1009,263807419614139553)?.0})
            }
        };
        outcomes.push(json!({"mode":role,"proof":proof,"inner_wall_ns":w["inner_wall_ns"],"observed_primitives":w["work_native"]["observed_primitives"],"b_count":w["work"]["collection"]["counts"]["b_count"],"sources":w["work"]["collection"]["sources"].as_array().map(Vec::len),"projections":w["work"]["collection"]["projections"].as_array().map(Vec::len),"descent_residuals":w["work"]["descent_residuals"]}));
        workers.push(w);
    }
    let common = common_sources(&workers[1], &workers[3])?;
    let matched = matched_pair_geometry(&workers[2], &workers[3])?;
    let flow: Vec<Value> = fs::read_to_string(dir.join("flow.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    if flow.len() != 18 {
        return Err("fixture/eight-arm flow".into());
    }
    for (i, role) in std::iter::once("fixture").chain(ROLES).enumerate() {
        let kind = if i == 0 { "fixture" } else { "execution" };
        if flow[2 * i]["mode"] != role
            || flow[2 * i + 1]["mode"] != role
            || flow[2 * i]["record_type"] != format!("{kind}_start")
            || flow[2 * i + 1]["record_type"] != kind
            || flow[2 * i + 1]["exit_code"] != 0
            || flow[2 * i]["source_revision"] != source
            || flow[2 * i + 1]["source_revision"] != source
        {
            return Err("exact complete flow".into());
        }
    }
    write_new(
        Path::new(&args[1]),
        &json!({"schema":"cover.deferred-pair.qualified/v1","source_revision":source,"public":public,"seed":389,"actual_scalar":answer,"outcomes":outcomes,"common":common,"matched_pair":matched,"planned":8,"completed":8,"excluded":0,"automatic_retries":0,"complete_cost":false,"complete_total":null,"ratios":null,"rho_parity":false,"independent_signed_verdict":false}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::jv_cover::{Fld, Hyp};
    fn symbolic_source(
        large: &[quadratic_pair_graph::LargeTerm],
        events: Value,
    ) -> pair_driver::Source {
        serde_json::from_value(json!({"candidate_index":1,"b_index":0,"a0":[],"a1":[],"b":[],"s":1,
            "certificate":{"terms":[],"remainders":large.iter().map(|t|json!({"key":t.class,"coefficient":t.coefficient})).collect::<Vec<_>>(),"a":[],"c":[],"norm":[]},"graph":events})).unwrap()
    }
    fn term(id: u64, c: i8) -> quadratic_pair_graph::LargeTerm {
        quadratic_pair_graph::LargeTerm {
            class: [id, 0, 0, 0, 0, 0],
            coefficient: c,
        }
    }
    #[test]
    fn balanced_deferred_cycle_can_cancel_newest_activation_source() {
        let mut producer =
            deferred_pair_graph::Graph::new(deferred_pair_graph::Options::default()).unwrap();
        let mut checker = deferred_graph_math_check::Replay::default();
        let mut sources = vec![];
        for (id, large) in [
            vec![term(0, 1), term(1, 1)],
            vec![term(1, 1), term(2, 1)],
            vec![term(2, 1), term(3, 1)],
            vec![term(3, 1), term(0, 1)],
            vec![term(0, 1)],
        ]
        .iter()
        .enumerate()
        {
            let events = producer.accept(id, large).unwrap();
            sources.push(symbolic_source(large, json!(events)));
            let full = checker.accept(&sources).unwrap();
            if id < 4 {
                assert!(full.is_empty());
            } else {
                assert_eq!(full.len(), 1);
                assert!(full[0].iter().all(|&(source, _)| source < id));
            }
        }
        assert_eq!(
            checker.stats(),
            serde_json::to_value(producer.stats()).unwrap()
        );
    }
    #[test]
    fn flat_index_checker_matches_producer_queue_on_synthetic_streams() {
        for seed in 0..64u64 {
            let mut z = seed + 91;
            let mut producer =
                deferred_pair_graph::Graph::new(deferred_pair_graph::Options::default()).unwrap();
            let mut checker = deferred_graph_math_check::Replay::default();
            let mut sources = vec![];
            for id in 0..257 {
                z = z.wrapping_mul(6364136223846793005).wrapping_add(1);
                let x = (z >> 20) % 31;
                let y = (x + 1 + (z >> 32) % 30) % 31;
                let a = if z & 1 == 0 { 1 } else { -1 };
                let b = if z & 2 == 0 { 1 } else { -1 };
                let large = if id % 29 == 0 {
                    vec![]
                } else if id % 11 == 0 {
                    vec![term(x, a)]
                } else {
                    vec![term(x, a), term(y, b)]
                };
                let events = producer.accept(id, &large).unwrap();
                sources.push(symbolic_source(&large, json!(events)));
                checker.accept(&sources).unwrap();
                assert_eq!(
                    checker.stats(),
                    serde_json::to_value(producer.stats()).unwrap()
                );
            }
        }
    }
    #[test]
    fn matched_prefix_refuses_changed_source_history_or_cohort() {
        let primitive: serde_json::Map<String, Value> = collection_check::accounting::KINDS
            .iter()
            .map(|&k| (k.into(), json!(0)))
            .collect();
        let phases: Vec<_> = collection_check::accounting::PHASES
            .iter()
            .map(|name| json!({"phase":name,"primitives":primitive}))
            .collect();
        let kept = json!({"candidate_index":2,"report":{"attempts":[{"polynomial":[1,2,3,4]}]}});
        let plain = json!({"work_native":{"phases":phases},"work":{"collection":{"mode":"discriminant-quadratic-pairs","counts":{"b_count":1024,"candidates":2},"sources":[{"candidate_index":2,"b_index":4,"certificate":{"terms":[[0,1]]}}],"checkpoints":[{"b_count":1024,"lines":8,"cohort_sha256":"a"}],"splits":[kept],"discriminant_screen":{"refusals":[]},"work_native":{"phases":phases}}}});
        let mut screened = plain.clone();
        screened["work"]["collection"]["mode"] = json!("deferred-discriminant-pairs");
        matched_pair_geometry(&plain, &screened).unwrap();
        for path in [
            "/work/collection/sources/0/certificate/terms/0/1",
            "/work/collection/checkpoints/0/cohort_sha256",
            "/work/collection/splits/0/report/attempts/0/polynomial/0",
        ] {
            let mut bad = screened.clone();
            *bad.pointer_mut(path).unwrap() = json!(7);
            assert!(matched_pair_geometry(&plain, &bad).is_err(), "{path}");
        }
        screened["work"]["collection"]["discriminant_screen"]["refusals"] =
            json!([{"candidate_index":1}]);
        assert!(matched_pair_geometry(&plain, &screened).is_err());
    }
    #[test]
    fn registration_refuses_altered_caps_seed_or_trial_flow() {
        let mut plan: Value = serde_json::from_str(include_str!(
            "../research/extension_ic_graph_20261115/deferred-pair-plan.json"
        ))
        .unwrap();
        plan["source_revision"] = json!("a".repeat(40));
        plan["runpod_allocation"] = json!({"compute_type":"CPU","vcpuCount":4,"memoryInGb":8,"lifetime_seconds":7200,"costPerHr":0.14});
        assert!(plan_ok(&plan));
        for path in [
            "/source_cap",
            "/max_vertices",
            "/max_witness_terms",
            "/split_attempt_cap",
            "/max_split_records",
            "/automatic_retries",
            "/rounds",
            "/seed",
            "/worker_timeout_seconds",
            "/runpod_allocation/vcpuCount",
        ] {
            let mut bad = plan.clone();
            let x = bad.pointer(path).and_then(Value::as_u64).unwrap();
            *bad.pointer_mut(path).unwrap() = json!(x + 1);
            assert!(!plan_ok(&bad), "{path}");
        }
        let mut bad = plan;
        bad["mode_order"] = json!(["grounded-pairs"]);
        assert!(!plan_ok(&bad));
    }
    #[test]
    fn actual_two_quadratic_source_has_separate_line_and_cantor_replay() {
        let (f, h, base, a, c, norm, terms, factors) = admission_controls::tests::hand();
        let certificate =
            quadratic_pair_relation::check_equation(&f, &h, &base, &a, &c, &norm, &terms, &factors)
                .unwrap()
                .into_certificate();
        let source = pair_driver::Source {
            candidate_index: 1,
            b_index: 0,
            a0: a.iter().map(|x| x.0[0]).collect(),
            a1: a.iter().map(|x| x.0[1]).collect(),
            b: c.iter().map(|x| x.0).collect(),
            s: 1,
            certificate,
            graph: json!({"kind":"ungrounded"}),
        };
        let domain = pair_math_check::Domain::new(
            f.p,
            f.w,
            h.iter().map(|x| x.0).collect(),
            base.iter().map(|b| (b.x, b.y.0)).collect(),
        )
        .unwrap();
        let raw = serde_json::to_value(&source).unwrap();
        let line = domain.line(&raw).unwrap();
        let factors: [Vec<u64>; 2] = factors.try_into().unwrap();
        assert_eq!(
            domain.candidate(&line, Some(&factors)).unwrap(),
            raw["certificate"]
        );
        let jac = Hyp { f: &f, h, g: 3 };
        pair_collection_check::source_proof(&jac, &base, &source, &c, &domain).unwrap();
        for path in [
            "/a0/0",
            "/a1/0",
            "/b/1/0",
            "/s",
            "/certificate/remainders/0/coefficient",
            "/certificate/remainders/1/key/2",
            "/certificate/norm/0",
        ] {
            let mut bad = raw.clone();
            let old = bad.pointer(path).and_then(Value::as_i64).unwrap();
            *bad.pointer_mut(path).unwrap() = json!((old + 1).rem_euclid(f.p as i64));
            let bad: pair_driver::Source = serde_json::from_value(bad).unwrap();
            assert!(
                pair_collection_check::source_proof(&jac, &base, &bad, &c, &domain).is_err(),
                "{path}"
            );
        }
        assert!(jac.is_valid(&base[0].d));
        assert_eq!(
            f.mul(&base[0].y, &base[0].y),
            remainder::eval(&f, &jac.h, base[0].x)
        );
    }
    #[test]
    fn independent_graph_replay_preserves_signed_coefficient_two_and_refuses_corruption() {
        use quadratic_pair_graph::{Graph, LargeTerm, Options};
        let x = [2, 0, 1, 0, 0, 0];
        let y = [3, 0, 1, 0, 0, 0];
        let pattern = vec![vec![(x, 1)], vec![(x, 1), (y, 1)], vec![(x, 1), (y, -1)]];
        let mut graph = Graph::new(Options::default()).unwrap();
        let mut replay = graph_math_check::Replay::default();
        let mut sources = Vec::new();
        let mut emitted = None;
        for (id, large) in pattern.into_iter().enumerate() {
            let terms: Vec<_> = large
                .iter()
                .map(|&(class, coefficient)| LargeTerm { class, coefficient })
                .collect();
            let outcome = graph.accept(id, &terms).unwrap();
            sources.push(pair_driver::Source {
                candidate_index: id as u64 + 1,
                b_index: 0,
                a0: vec![],
                a1: vec![],
                b: vec![],
                s: 1,
                certificate: quadratic_pair_relation::Certificate {
                    terms: vec![(id, 1)],
                    remainders: large
                        .into_iter()
                        .map(
                            |(key, coefficient)| quadratic_pair_relation::QuadraticClass {
                                key,
                                coefficient,
                            },
                        )
                        .collect(),
                    a: vec![],
                    c: vec![],
                    norm: vec![],
                },
                graph: serde_json::to_value(outcome).unwrap(),
            });
            emitted = replay.accept(&sources).unwrap();
        }
        assert_eq!(emitted, Some(vec![(0, -2), (1, 1), (2, 1)]));
        assert_eq!(replay.stats(), serde_json::to_value(graph.stats()).unwrap());
        sources[2].graph["witness"][0][1] = json!(-1);
        let mut replay = graph_math_check::Replay::default();
        replay.accept(&sources[..1]).unwrap();
        replay.accept(&sources[..2]).unwrap();
        assert!(replay.accept(&sources).is_err());
    }
}
