use super::*;
pub fn draft(p: u64, seed: u64, count: u64, unit: bool) -> Value {
    let mut arms = vec![];
    let instrument = json!({"repo":"crypto","commit":null,"build":[["make","build"]],"binary_sha256":null,"threads":1,"flags":{"solver":"Z3 5.3.3","codec":"bounded_bv","timeout_seconds":"30"},"env":{}});
    for base in [
        "source",
        "same_support",
        "target_native",
        "mixed",
        "random",
        "positive",
    ] {
        for encoding in ["mod", "carry"] {
            let id = format!("{base}-{encoding}");
            let role = if base == "source" && encoding == "mod" {
                "reference"
            } else if base == "random" {
                "control_negative"
            } else if base == "positive" {
                "control_positive"
            } else {
                "candidate"
            };
            arms.push(json!({"id":id,"role":role,"description":format!("{base} support, {encoding} bounded multiplication encoding"),"instrument":instrument,"same_instrument_as":if id=="source-mod"{None}else{Some("source-mod")},"parameters":{"base":base,"encoding":encoding,"p":p,"seed":seed,"columns":if unit{3}else{8}}}));
        }
    }
    arms.push(json!({"id":"source-aa-mod","role":"control_aa","description":"Byte-identical source-mod query in a fresh solver process","instrument":instrument,"same_instrument_as":"source-mod","parameters":{"p":p,"seed":seed}}));
    let comp = vec![
        json!({"id":"C-isogeny","arms":["source-mod","same_support-mod"],"differs_in":["curve_representation"],"design":"one_factor"}),
        json!({"id":"C-carry","arms":["source-mod","source-carry"],"differs_in":["multiplication_encoding"],"design":"one_factor"}),
        json!({"id":"C-shape","arms":["source-mod","mixed-mod"],"differs_in":["base_support"],"design":"one_factor"}),
        json!({"id":"C-aa","arms":["source-mod","source-aa-mod"],"differs_in":["invocation"],"design":"aa"}),
    ];
    let mut value = json!({
        "schema":"experiment.protocol/v1","protocol_id":if unit{"EXAMPLE-ATLAS-runner-unit".into()}else{format!("ATLAS-p{p}-seed{seed}")},"version":1,"created":"2026-11-04","repository":"crypto","study_dir":"research/curve25519_atlas_carry_rank_20261104",
        "question":"On the fixed miniature cell, do matched representation/support arms change Z3 resource units while every result agrees with the native signed three-sum reference? This is a component comparison, not full IC cost.",
        "links":{"scope":if unit{"Local development fixture; no performance or scientific promotion"}else{"One cell of the original 2029/8173/32749, seeds 13/29/47/71 panel; other cells stay required"}},
        "claims":[{"id":"K-correct","text":"Every successful solver row matches the exact miniature reference, with SAT witnesses checked natively.","class":"exact_computation","decided_by":[]},{"id":"K-resource","text":"Finite-workload differences in solver resource units only; no population, timing, complexity or full pipeline inference.","class":"comparative","decided_by":["C-isogeny","C-carry","C-shape"],"if_supported":"component counts only after all controls and replay pass","if_not_supported":"retain null, negative and censored outcomes"}],
        "arms":arms,
        "controls":{"aa":{"present":true,"comparison":"C-aa","not_applicable_reason":null},"positive":{"present":true,"arm":"positive-mod","expected_detection":"SAT for the three distinct planted signed points through the same unpinned solver path","not_applicable_reason":null},"negative":{"present":true,"arm":"random-mod","null_population":{"description":"Uniform equal-column subgroup sign-orbit base","generator":"C++ atlas random arm with frozen mt19937_64 seed","n":count,"why_n":"All queries of the fixed finite cell; no sampling power claim","matched_on":["field","subgroup","columns","targets","solver"]},"not_applicable_reason":null}},
        "variable_ledger":{"independent":[{"name":"curve_representation","levels":["source","2-isogenous"]},{"name":"multiplication_encoding","levels":["mod","carry"]},{"name":"base_support","levels":["source","target-native","mixed","random"]}],"factors":[{"name":"invocation","treatment":"fixed","value":"fresh single-thread solver process","how":"Same solver image, binary, arguments and query bytes for A/A"},{"name":"host","treatment":"measured","how":"Record actual host facts; no wall clock is admitted"},{"name":"inputs","treatment":"fixed","how":"Freeze configuration, base, target, query and manifest hashes before comparisons; target sign orbits are disjoint between train and holdout"},{"name":"analyst","treatment":"fixed","how":"Native deterministic export; labels unmasked; no discretionary omissions"}],"comparisons":comp,"crosses_map":true,"transfer_obligations":[{"map":"rational 2-isogeny","scope":"native finite homomorphism, dual and subgroup inverse fixtures; same-support control"}]},
        "randomization":{"arm_order":"fixed","arm_order_reason":"Counted software units only, fresh process per query, no machine-time comparison; retain A/A counter variation","seed_derivation":{"method":"harness_drawn","note":format!("C++ mt19937_64 seed {seed}; target shuffle uses seed xor 0x4f5554, representatives 1..(ell-1)/2 with random signs; raw fixtures bind library-specific shuffle")},"instance_sampling":{"rule":"Prepared arm-independent holdout targets, disjoint modulo negation from calibration; planted controls separate","independent_of_arm":true},"blinding":{"analysis_labels_masked":false,"how":"Exact finite counters and native witnesses; no blind population estimate is claimed"}},
        "sample_size":{"instances":count,"replicates":1,"warmups":0,"power":{"method":"exact","alpha":0.05,"tests_in_family":3,"correction":"holm","passes":true,"note":"Finite frozen workload, exact descriptive counters; no statistical significance or population-power claim"},"preflight_runtime":{"fixed_seconds":null,"per_sample_seconds":null,"threads":1,"estimated_total_seconds":null,"measured_on":null}},
        "metrics":{"primary":[{"id":"solver_rlimit","name":"Z3 resource-limit counter","unit":"z3-5.3.3-rlimit","kind":"counted","definition":"One fresh Z3 invocation, including its solver setup; excludes fixture generation, native replay and all other pipeline phases","source":"solver stdout :rlimit-count","aggregation":"per_row","resolution":"1","uncertainty":"none_exact"}],"secondary":[{"id":"producer_check_passed","name":"Native expected-status and witness check","unit":"boolean","kind":"exact","definition":"1 iff expected SAT/UNSAT agrees and every SAT witness replays natively","source":"atlas.solver-row/v1 producer_check_passed","aggregation":"per_row","resolution":"1","uncertainty":"none_exact"}]},
        "measurement":{"isolation_required":"none","warmups":0,"order":"sequential","clock":"none","counters":["solver_rlimit"],"record":["source commit and dirty state","binary hashes","query and base hashes","raw stdout/stderr","host facts","timeouts and failures"],"timed_primary_justification":null},
        "execution":{"backend":if unit{"local"}else{"isolab-k8s"},"queue":if unit{None}else{Some("cpu-spot")},"placement":{"cpu":4,"memory_gib":4,"budget_usd_total":5,"per_query_timeout_seconds":30},"measurement_class":"search","timeout_seconds":900,"checkpoint":{"policy":"none","interval_seconds":null,"grace_seconds":null,"restore_validation":"none","after_restore":{"wall_time":"invalid","counted_units":"valid"}},"retries":{"infrastructure_only":true,"max_attempts":1}},
        "stopping_rules":[{"kind":"fixed_n","rule":"Attempt every planned row once; do not drop timeout, error or null outcomes","preregistered":true},{"kind":"budget","rule":"USD 5 aggregate including builds/idle/failures, original per-job deadline 900 seconds; no automatic retries or quota increases","preregistered":true},{"kind":"invalidation","rule":"Hash mismatch, wrong status, invalid witness, failed positive control or incomplete panel blocks a gain claim","preregistered":true}],
        "decision_rule":{"text":"Read resource-count differences only on a complete fixed cell with matching native outcomes and passing controls; all pipeline costs and larger-size claims remain open","comparison_ids":["C-isogeny","C-carry","C-shape"],"statistic":"paired finite resource counts; no censored-case deletion","interval":"none: exact finite workload, not population inference","multiple_comparison_correction":"holm","effect_size_measure":"paired solver resource-unit differences, component only","verdict_vocabulary":["component_lower","component_equal","component_higher","incomplete","invalid"]},
        "analysis_plan":{"script":null,"inputs":["queries.tsv","generic.csv","selection.txt","solver-run/summary.json"],"outputs":["experiment.export/v1"],"blinded":false,"frozen_before_data":false},
        "inadmissible":["Calling rlimit units wall time, GAE or S","Dropping timeout/error queries","Selecting on held-out data","Treating generic enumeration as an algebraic solver","Promoting a component result into a complete IC gain"],"amendments":[],
        "freeze":{"status":"draft","protocol_sha256":null,"commit":null,"frozen_at":null,"filed_before_affected_data":null}
    });
    if unit {
        value["claims"][1]["class"] = json!("out_of_scope");
        value["claims"][1]["decided_by"] = json!([]);
        value["claims"][1]["text"] = json!(
            "No comparative or performance inference from this local format/integration example."
        );
        value["sample_size"]["preflight_runtime"]["estimated_total_seconds"] = json!(600);
        value["analysis_plan"]["frozen_before_data"] = json!(true);
        value["decision_rule"]["text"]=json!("Local exact integration example only; no scientific performance decision. The 600-second value is a test budget, not a measured runtime estimate.");
    }
    value
}
