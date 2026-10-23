//! Derived correctness inventory, with fresh scalar/row and energy replay.
//! This is not a timer or an admission of a complete experiment.
use crate::{digest, toy_field::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

fn read(path: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn array(v: &Value) -> &[Value] {
    v.as_array().expect("required array")
}
fn number(v: &Value) -> u64 {
    v.as_u64().expect("required nonnegative integer")
}
fn text(v: &Value) -> &str {
    v.as_str().expect("required string")
}
fn rows(root: &Path, name: &str) -> Result<Vec<Value>> {
    Ok(array(&read(root.join(name).join("SUMMARY.json"))?["rows"]).to_vec())
}
fn files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut ps = fs::read_dir(root)?
        .map(|p| p.map(|p| p.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    ps.sort();
    Ok(ps)
}
fn statuses(rs: &[Value]) -> BTreeMap<String, u64> {
    let mut counts = BTreeMap::new();
    for r in rs {
        *counts.entry(text(&r["status"]).to_owned()).or_default() += 1;
    }
    counts
}
fn fixture<'a>(fixtures: &'a [Fixture], v: &Value) -> &'a Fixture {
    fixtures
        .iter()
        .find(|f| f.id == text(v))
        .expect("frozen fixture")
}
fn verify_scalar(f: &Fixture, q: P, scalar: &Value) -> bool {
    if scalar.is_null() {
        return false;
    }
    assert_eq!(
        f.curve().pm(f.group(), number(scalar) as u128),
        q,
        "source scalar replay"
    );
    true
}
fn accepted(root: &Path, name: &str) -> Result<()> {
    assert_eq!(
        read(root.join(name).join("ADJUDICATION.json"))?["accepted"],
        true
    );
    Ok(())
}
fn relative_manifest(root: &Path, dir: &Path, out: &mut Vec<Value>) -> Result<()> {
    for p in files(dir)? {
        if p.is_dir() {
            relative_manifest(root, &p, out)?;
        } else {
            let bytes = fs::read(&p)?;
            out.push(
                json!({"archive_member":format!("runs/{}",p.strip_prefix(root)?.display()),
                "bytes":bytes.len(),"sha256":digest(&bytes)}),
            );
        }
    }
    Ok(())
}
fn row_value(e: Quad, base: &[P], coefficients: &[Value]) -> P {
    assert_eq!(base.len(), coefficients.len());
    base.iter().zip(coefficients).fold(None, |sum, (p, c)| {
        let c = c.as_i64().expect("integer row coefficient");
        let p = if c < 0 { e.pn(*p) } else { *p };
        e.pa(sum, e.pm(p, c.unsigned_abs() as u128))
    })
}
fn energy_replay(f: &Fixture, base: &[P], v: &Value) -> u64 {
    let e = f.curve();
    let signed = base.iter().flat_map(|p| [*p, e.pn(*p)]).collect::<Vec<_>>();
    let mut pair = BTreeMap::<P, u64>::new();
    for a in &signed {
        for b in &signed {
            *pair.entry(e.pa(*a, *b)).or_default() += 1;
        }
    }
    let en = pair.values().map(|n| n * n).sum::<u64>();
    assert_eq!(number(&v["additive_energy_signed_base"]), en);
    assert_eq!(number(&v["pair_histogram_support"]), pair.len() as u64);
    let mut triple = BTreeMap::<P, u64>::new();
    for (a, n) in &pair {
        for b in &signed {
            *triple.entry(e.pa(*a, *b)).or_default() += n;
        }
    }
    assert_eq!(number(&v["triple_histogram_support"]), triple.len() as u64);
    for y in array(&v["target_yields"]) {
        let p: P = serde_json::from_value(y["target"].clone()).unwrap();
        assert_eq!(number(&y["pair"]), *pair.get(&p).unwrap_or(&0));
        assert_eq!(number(&y["triple"]), *triple.get(&p).unwrap_or(&0));
    }
    en
}

pub fn run(fixtures: &[Fixture], execution: &Path, out: &Path) -> Result<Vec<Value>> {
    let root = execution.join("runs");
    // The output must be outside runs: freeze the complete pre-existing tree.
    assert!(!out.starts_with(&root));
    let mut manifest = vec![];
    relative_manifest(&root, &root, &mut manifest)?;
    write_json(out.join("RAW_MEMBERS.json"), &manifest)?;
    let mut result = vec![];
    let mut admissions = vec![];
    for p in files(&root)? {
        let ap = p.join("ADJUDICATION.json");
        if !ap.exists() {
            continue;
        }
        let old = read(&ap)?;
        if old["accepted"] == true {
            let target = out.join("rechecked-cas").join(p.file_name().unwrap());
            fs::create_dir_all(&target)?;
            super::adjudicate(Path::new(text(&old["log_prefix"])), &target)?;
        }
        admissions.push(json!({"archive_member":format!("runs/{}/ADJUDICATION.json",p.file_name().unwrap().to_string_lossy()),
            "historical_accepted":old["accepted"],"accepted_raw_hashes_and_gate_replayed":old["accepted"]==true}));
    }
    write_json(out.join("CAS_REPLAYS.json"), &admissions)?;

    let names = [
        "icv1-fp8-tm21-71e3bcb7",
        "icv1-fp10-tm21-b87c79a9",
        "icv1-fp14-t33-d98086ae",
        "P-521",
    ];
    let mut torsion = vec![];
    for name in names {
        let n = format!("torsion-{name}-20261023-v2");
        let rs = rows(&root, &n)?;
        assert_eq!(
            rs,
            rows(&root, &format!("torsion-{name}-20261023-v1"))?,
            "only the GP emitter changed"
        );
        accepted(&root, &format!("{n}-adjudication"))?;
        let r = &rs[0];
        assert_eq!(r["minimum_full_torsion_degree"], 4);
        torsion.push(json!({"source":name,"degree":r["minimum_full_torsion_degree"],
            "anti_isometries":array(&r["anti_isometries"]).len(),"nonzero_points":array(&r["nonzero_torsion_points"]).len(),
            "bilinearity_checks":r["bilinearity_checks"],"native_rows_unchanged_in_corrected_emitter":true,"independent_CAS_accepted":true}));
    }
    result.push(json!({"experiment":"E18","actual_torsion":torsion,"status":"partial"}));

    let geometry_index: Vec<String> =
        serde_json::from_value(read(execution.join("GEOMETRY_INPUTS.json"))?)?;
    let mut geometry = vec![];
    for path in &geometry_index {
        let v = read(path)?;
        let rs = array(&v["rows"]);
        assert!(!rs.is_empty());
        for r in rs {
            assert_eq!(r["curve"]["squarefree"], true);
            assert_eq!(array(&r["maps"]).len(), 2);
            for map in array(&r["maps"]) {
                assert_eq!(map["degree"], 3);
                assert_eq!(map["separable"], true);
                assert_eq!(map["cleared_identity_lhs"], map["cleared_identity_rhs"]);
                assert_eq!(map["false_shifted_x_rejected"], true);
            }
        }
        let parent = Path::new(path)
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_string_lossy();
        accepted(&root, &parent.replace("-verified", "-adjudication"))?;
        geometry.push(
            json!({"source":rs[0]["source"],"parameter_presentations":rs.len(),
            "independent_CAS_accepted":true,"presentations_not_asserted_nonisomorphic":true}),
        );
    }
    let mut transfers = vec![];
    let transfer = rows(&root, "three-transfer-20261023-v2")?;
    for r in &transfer {
        if r["fixture"].is_null() {
            continue;
        }
        let f = fixture(fixtures, &r["fixture"]);
        let suffix = match f.p {
            751 => "-stack-v2",
            10111 => "-stack-v4",
            _ => "",
        };
        accepted(
            &root,
            &format!("three-transfer-{}-20261023-v2-adjudication{suffix}", f.id),
        )?;
        assert_eq!(r["all_source_points"], f.order);
        assert_eq!(r["all_generator_translation_checks"], f.order);
        let mut targets = BTreeSet::new();
        for c in array(&r["toy_destination_BSGS_controls"]) {
            let q: P = serde_json::from_value(c["public_source_target"].clone())?;
            assert!(verify_scalar(f, q, &c["recovered_scalar"]));
            assert_eq!(c["source_scalar_verified"], true);
            targets.insert(q);
        }
        transfers.push(json!({"fixture":f.id,"source_points":f.order,"generator_translation_checks":f.order,
            "recovery_attempts":array(&r["toy_destination_BSGS_controls"]).len(),"distinct_targets":targets.len(),
            "independent_Cantor_and_norm_replay_accepted":true}));
    }
    result.push(
        json!({"experiment":"E19","geometry":geometry,"transfers":transfers,"status":"partial"}),
    );

    let modules = rows(&root, "module-20261023-v1")?;
    accepted(&root, "module-20261023-v1-adjudication")?;
    let mut higher = vec![];
    for p in files(&root)? {
        let name = p.file_name().unwrap().to_string_lossy();
        if !name.starts_with("higher-") || !name.ends_with("-20261023-v2-native-verify") {
            continue;
        }
        let rs = rows(&root, &name)?;
        assert_eq!(rs.len(), 1);
        let r = &rs[0];
        accepted(&root, &name.replace("-native-verify", "-adjudication"))?;
        let ell = number(&r["ell"]);
        assert_eq!(number(&r["actual_torsion_points"]), ell * ell);
        assert_eq!(r["all_graph_points_Galois_stable"], true);
        assert_eq!(r["independent_PARl_pairing_equal"], true);
        for g in array(&r["graphs"]) {
            assert_eq!(array(&g["points"]).len() as u64, ell * ell);
        }
        higher.push(json!({"source":r["source"],"ell":ell,"extension_degree":r["extension_degree"],
            "torsion_points_per_curve":ell*ell,"anti_isometries":array(&r["anti_isometries"]).len(),
            "graphs":array(&r["graphs"]).len(),"independent_pairing_equal":true,"quotient_constructed":false}));
    }
    result.push(json!({"experiment":"E20","module_rows":modules.len(),
        "main_rows":modules.iter().filter(|r|r["ell"]!=3).count(),"ell3_regression_rows":modules.iter().filter(|r|r["ell"]==3).count(),
        "actual_higher_prime_cases":higher,"status":"partial"}));

    let census = rows(&root, "coupled-census-20261023-v1")?;
    accepted(&root, "coupled-census-20261023-v1-adjudication")?;
    let relation = rows(&root, "coupled-relations-20261023-v1")?;
    let mut target_checks = 0;
    let mut recovery_checks = 0;
    let mut unique_fixture_targets = BTreeSet::new();
    let mut unique_profiles = BTreeSet::new();
    let mut by_role = BTreeMap::<String, u64>::new();
    for r in &relation {
        if r["file"].is_null() {
            continue;
        }
        let v = read(
            root.join("coupled-relations-20261023-v1")
                .join(text(&r["file"])),
        )?;
        let f = fixture(fixtures, &v["fixture"]);
        unique_profiles.insert((
            f.id.clone(),
            number(&v["arity"]),
            digest(&serde_json::to_vec(&v["base"])?),
        ));
        for t in array(&v["targets"]) {
            assert_eq!(t["same_base_sets_equal"], true);
            let q: P = serde_json::from_value(t["target"].clone())?;
            let recovered = verify_scalar(f, q, &t["recovered_scalar"]);
            assert_eq!(t["scalar_verified"], recovered);
            recovery_checks += u64::from(recovered);
            target_checks += 1;
            *by_role.entry(text(&v["role"]).to_owned()).or_default() += 1;
            unique_fixture_targets.insert((f.id.clone(), q));
        }
    }
    let selection = read(root.join("coupled-relations-20261023-v1/SELECTION.json"))?;
    assert_eq!(selection["selected_before_holdout_target_evaluation"], true);
    result.push(json!({"experiment":"E21","census_cells":census.len(),"census":census,
        "relation_ledger_entries":relation.len(),"statuses":statuses(&relation),"target_label_attempts_across_configs":target_checks,
        "distinct_fixture_targets":unique_fixture_targets.len(),"distinct_fixture_base_arity_profiles":unique_profiles.len(),
        "verified_recovery_attempts":recovery_checks,"all_evaluated_projected_row_sets_equal":true,"attempts_by_role":by_role,
        "selected_configs":selection["selected_configs"],"selection_is_uncalibrated":true,"status":"partial"}));

    let energy = rows(&root, "coupled-energy-20261023-v1")?;
    let mut random = BTreeMap::<(String, u64), Vec<u64>>::new();
    let mut coupled = vec![];
    for p in files(&root.join("coupled-energy-20261023-v1"))? {
        if p.extension().is_none_or(|s| s != "json") || p.file_name().unwrap() == "SUMMARY.json" {
            continue;
        }
        let v = read(&p)?;
        let f = fixture(fixtures, &v["fixture"]);
        let base: Vec<P> = serde_json::from_value(v["base"].clone())?;
        let en = energy_replay(f, &base, &v);
        if v["random_seed"].is_number() {
            random
                .entry((f.id.clone(), base.len() as u64))
                .or_default()
                .push(en);
        } else {
            assert_eq!(
                number(&v["fiber_duplication_control"]["weighted_energy_if_duplicated"]),
                16 * en
            );
            assert_eq!(
                v["fiber_duplication_control"]["new_independent_rows_from_duplicates"],
                0
            );
            coupled.push(json!({"fixture":f.id,"B":base.len(),"energy":en,"configs":v["configs_sharing_this_base"]}));
        }
    }
    let mut random_summary = vec![];
    for ((id, b), values) in &random {
        assert_eq!(values.len(), 32);
        random_summary.push(json!({"fixture":id,"B":b,"n":values.len(),"min":values.iter().min(),"max":values.iter().max(),
            "sum":values.iter().sum::<u64>(),"mean":values.iter().sum::<u64>() as f64/values.len() as f64}));
    }
    result.push(json!({"experiment":"E22","ledger_entries":energy.len(),"unique_coupled_bases":coupled,
        "matched_random_controls":random.values().map(|v|v.len()).sum::<usize>(),"matched_random_groups":random_summary,
        "all_pair_triple_histograms_recomputed":true,"fiber_energy_is_derived_not_measured_span":true,"status":"partial"}));

    let partial = rows(&root, "partial-20261023-v2")?;
    let mut profiles = 0;
    let mut attempts = 0;
    let mut distinct_targets = BTreeSet::new();
    let mut verified = [0u64; 4];
    let partial_root = root.join("partial-20261023-v2");
    let mut witness_cache = BTreeMap::new();
    for p in files(&partial_root)? {
        if !p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("profile-")
        {
            continue;
        }
        let v = read(&p)?;
        let f = fixture(fixtures, &v["fixture"]);
        assert_eq!(v["fake_twist_log_column_rejected"], true);
        profiles += 1;
        for t in array(&v["targets"]) {
            let arms = array(&t["arms"]);
            assert_eq!(arms.len(), 4);
            assert_eq!(arms[0]["row_sha256"], arms[1]["row_sha256"]);
            assert_eq!(arms[2]["row_sha256"], arms[3]["row_sha256"]);
            assert_eq!(arms[0]["scalar_verified"], arms[1]["scalar_verified"]);
            assert_eq!(arms[2]["scalar_verified"], arms[3]["scalar_verified"]);
            for (i, a) in arms.iter().enumerate() {
                verified[i] += u64::from(a["scalar_verified"] == true);
                let file = text(&a["file"]);
                let witness = if let Some(v) = witness_cache.get(file) {
                    v
                } else {
                    let w = read(partial_root.join(file))?;
                    assert_eq!(
                        digest(&serde_json::to_vec(&w["raw_projected_equations"])?),
                        text(&a["row_sha256"])
                    );
                    assert_eq!(w["all_input_extension_relations_verified"], true);
                    assert_eq!(w["all_eliminated_rows_replayed_in_source_group"], true);
                    let base: Vec<P> = serde_json::from_value(w["base"].clone())?;
                    let helpers: Vec<P> = serde_json::from_value(w["helper_projections"].clone())?;
                    let q: P = serde_json::from_value(w["target"].clone())?;
                    let mut columns = base;
                    columns.extend(helpers);
                    columns.push(q);
                    assert_eq!(
                        serde_json::to_value(&columns)?,
                        w["columns_including_target"]
                    );
                    // The final matrix column is the known generator coefficient.
                    columns.push(f.curve().pn(f.group()));
                    for row in array(&w["raw_projected_equations"]) {
                        assert_eq!(row_value(f.curve(), &columns, array(row)), None);
                    }
                    for row in w["eliminated_echelon_rows"]
                        .as_object()
                        .expect("pivot map")
                        .values()
                    {
                        assert_eq!(row_value(f.curve(), &columns, array(row)), None);
                    }
                    verify_scalar(f, q, &w["scalar"]);
                    witness_cache.entry(file.to_owned()).or_insert(w)
                };
                let q: P = serde_json::from_value(witness["target"].clone())?;
                assert_eq!(a["scalar_verified"], !witness["scalar"].is_null());
                distinct_targets.insert((f.id.clone(), q));
            }
            attempts += 1;
        }
    }
    result.push(json!({"experiment":"E23","profile_ledger_entries":partial.len(),"statuses":statuses(&partial),
        "evaluated_distinct_profiles":profiles,"target_label_attempts_across_profiles":attempts,
        "distinct_fixture_targets":distinct_targets.len(),"unique_arm_witnesses_replayed":witness_cache.len(),
        "verified_recovery_attempts_by_arm":verified,"arm_order":["zero-helper","pure-twist","general-extension","direct-rational"],
        "zero_and_pure_twist_rows_and_recovery_equal":true,"general_and_rational_rows_and_recovery_equal":true,"status":"partial"}));

    let jacobian = rows(&root, "jacobian-independent-20261023-v2")?;
    let mut classes = 0;
    for r in &jacobian {
        let suffix = if r["fixture"] == "icv1-fp6-tm7-fc20da94" {
            "-stack-v2"
        } else {
            ""
        };
        accepted(
            &root,
            &format!(
                "jacobian-{}-20261023-v2-adjudication{suffix}",
                text(&r["fixture"])
            ),
        )?;
        assert_eq!(r["native_generic_norm_matches_old_both_maps"], true);
        classes += number(&r["classes"]);
    }
    result.push(json!({"experiment":"E24","fixtures":jacobian.len(),"classes":classes,
        "all_independent_CAS_replays_accepted":true,"balanced_base_field_arithmetic_implemented":false,"status":"partial"}));

    let solver = rows(&root, "homogeneous-20261023-leading-v4")?;
    let mut engine_status = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut certificates = BTreeMap::<String, u64>::new();
    let mut lex = 0;
    let mut controls = Value::Null;
    for r in &solver {
        if r["experiment"] == "E25-control" {
            controls = r.clone();
            continue;
        }
        if r["order"] == "lex" {
            lex += 1;
            continue;
        }
        for engine in array(&r["engines"]) {
            let k = format!(
                "exit_{}_parsed_{}",
                engine["exit_code"], engine["basis_parsed"]
            );
            *engine_status
                .entry(text(&engine["engine"]).to_owned())
                .or_default()
                .entry(k)
                .or_default() += 1;
        }
        for kind in [
            "homogeneous_complete_basis_gate",
            "dehomogenized_affine_complete_basis_gate",
        ] {
            let gate = &r[kind];
            if gate["completion_and_clean_output"] == true {
                for b in array(&gate["bases"]) {
                    if b["complete"] == true {
                        *certificates
                            .entry(format!("{kind}:{}", text(&b["basis"])))
                            .or_default() += 1;
                    }
                }
            }
        }
    }
    assert_eq!(controls["affine"]["completion_and_clean_output"], true);
    for c in array(&controls["affine"]["bases"]) {
        assert_eq!(c["complete"], c["basis"] == "Q0");
    }
    let manual = rows(&root, "homogeneous-20261023-recheck-v3")?;
    let mut manual_exits = BTreeMap::<String, u64>::new();
    for r in &manual {
        for kind in [
            "homogeneous_complete_basis_gate",
            "dehomogenized_affine_complete_basis_gate",
        ] {
            if !r[kind].is_null() {
                *manual_exits
                    .entry(format!("{kind}:{}", r[kind]["exit_code"]))
                    .or_default() += 1;
            }
        }
    }
    result.push(json!({"experiment":"E25","requested_cells":solver.len()-1,"lex_signature_unsupported":lex,
        "engine_statuses":engine_status,"accepted_complete_basis_certificates":certificates,"controls":controls,
        "manual_S_pair_checker_exit_counts":manual_exits,"manual_wrapper_receipt_missing":true,
        "manual_native_finished_marker_present":true,"signature_speedup_admitted":false,"status":"partial"}));

    let projection = rows(&root, "projection-20261023-v1")?;
    accepted(&root, "projection-20261023-v1-adjudication")?;
    let audit_path = execution
        .parent()
        .unwrap()
        .join("lean-v2/receipts/resume-20261023-v1/axiom-audit.stdout");
    let audit = fs::read_to_string(&audit_path)?;
    let theorems = audit.lines().filter(|s| s.starts_with("ok    ")).count();
    assert!(audit.lines().any(|s| s
        == format!(
            "audited {theorems} theorems in [TwistFollowup]; 0 failing, 0 trusted (native_decide)"
        )));
    result.push(json!({"experiment":"E29","finite_replay":projection,"actual_Frobenius_checks":projection.iter().map(|r|number(&r["actual_power_Frobenius_checks"])).sum::<u64>(),
        "twist_addition_pairs":projection.iter().map(|r|number(&r["twist_embedding_addition_pairs"])).sum::<u64>(),
        "Lean_clean_theorems":theorems,"Lean_audit_sha256":digest(audit.as_bytes()),"full_formal_bridge_complete":false,"status":"partial"}));
    for (id,gap) in [("E26","No four-candidate block-elimination implementation or runs."),
        ("E27","Complete construction, solver, elimination and verification cost accounting and calibration missing."),
        ("E28","Final labels 32–63 withheld pending complete-cost selection; no final holdout executed.")] {
        result.push(json!({"experiment":id,"status":"not-attempted","remaining_gap":gap}));
    }
    write_json(
        out.join("ASSESSMENT.json"),
        &json!({"schema":"twist.derived-assessment/v1","results":result,
        "CAS_adjudications":admissions,"raw_members":manifest.len(),"statistics_implementation":"native Rust",
        "performance_claim":false,"novelty_established":false,"NIST_DLP_attempted":false,"full_campaign_complete":false}),
    )?;
    Ok(result)
}
