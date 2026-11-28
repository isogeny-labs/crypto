//! Input/schema and replay contracts. No candidate run or timing campaign.
#[allow(dead_code)]
#[path = "../src/choice_main.rs"]
mod driver;

use prime31_aux_matched::audit::audit_jsonl;
use prime31_aux_matched::collector::{PublicTarget, QueryStream};
use prime31_aux_matched::kernel_fibers::{
    build_fiber_with_factorization, KernelChoice, KernelFactorization,
};
use prime31_aux_matched::oracle::{
    OracleCounters, OracleResult, OracleStatus, PreparedOracle, Presentation,
};
use prime31_aux_matched::specialized_control::SpecializedControl;
use prime31_aux_matched::target_transport::{prepare, TargetArm};
use prime31_aux_matched::GroupOps;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "p31-prepared-contract-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn audit(records: &[Value]) -> Result<Value, String> {
    let bytes: String = records.iter().map(|r| format!("{}\n", r)).collect();
    audit_jsonl(Cursor::new(bytes))
}

#[test]
fn paired_prepared_inputs_preserve_legacy_panels_and_reject_mixed_semantics() {
    let s = Scratch::new();
    let root = s.0.join("panel");
    driver::prepared_panel(root.to_str().unwrap()).unwrap();
    assert!(driver::prepared_panel(root.to_str().unwrap()).is_err());
    let mut contracts = BTreeSet::new();
    for (label, expected) in [("screening", 20), ("recovery-candidates", 100)] {
        let index = read(&root.join(label).join("index.json"));
        assert_eq!(index["prepared_cells"], expected);
        assert_eq!(index["attempted"], 0);
        assert_eq!(index["selected_for_execution"], 0);
        for entry in index["cells"].as_array().unwrap() {
            let path = root.join(label).join(entry["file"].as_str().unwrap());
            driver::read_cell(path.to_str().unwrap()).unwrap();
            let mut cell = read(&path);
            assert_eq!(cell["source_target"]["p"], 2_147_483_647u64);
            assert_eq!(cell["target_arm"], "original");
            assert_eq!(cell["recovery_mode"], "augmented_only");
            if cell["oracle_backend"] == "prepared_polynomial" {
                cell["schema"] = json!("prime31-isogeny-choice.cell/v2");
                let tampered = s.0.join("tampered.json");
                fs::write(&tampered, cell.to_string()).unwrap();
                assert!(driver::read_cell(tampered.to_str().unwrap()).is_err());
                cell["schema"] = json!("prime31-isogeny-choice.cell/v3");
                cell.as_object_mut().unwrap().remove("recovery_mode");
                fs::write(&tampered, cell.to_string()).unwrap();
                assert!(driver::read_cell(tampered.to_str().unwrap()).is_err());
            }
            if label == "screening" {
                contracts.insert(format!(
                    "{}:{}:{}:{}:{}",
                    entry["method"],
                    entry["factorization"],
                    entry["presentation"],
                    entry["oracle_backend"],
                    entry["plain_control"]
                ));
            }
        }
    }
    // Seventeen distinct IC configurations and three distinct rho controls.
    assert_eq!(contracts.len(), 20);
    let legacy = Path::new(env!("CARGO_MANIFEST_DIR")).join("../sparse-h2-followup/panels");
    for label in ["screening", "recovery-candidates"] {
        let index = read(&legacy.join(label).join("index.json"));
        for entry in index["cells"].as_array().unwrap() {
            let path = legacy.join(label).join(entry["file"].as_str().unwrap());
            driver::read_cell(path.to_str().unwrap()).unwrap();
        }
    }
}

#[test]
fn independent_audit_binds_prepared_setup_to_the_checked_full_fiber() {
    let s = Scratch::new();
    let root = s.0.join("panel");
    driver::prepared_panel(root.to_str().unwrap()).unwrap();
    let index = read(&root.join("screening/index.json"));
    let entry = index["cells"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| {
            e["oracle_backend"] == "prepared_polynomial"
                && e["presentation"] == "tower"
                && e["factorization"] == "full_two_torsion_first"
                && e["plain_control"] == false
        })
        .unwrap();
    let cell = read(&root.join("screening").join(entry["file"].as_str().unwrap()));
    let source: PublicTarget = serde_json::from_value(cell["source_target"].clone()).unwrap();
    let fiber = build_fiber_with_factorization(
        KernelChoice::AminusH2,
        1,
        KernelFactorization::FullTwoTorsionFirst,
        &mut GroupOps::default(),
    )
    .unwrap();
    let base = prepare(&fiber, &source, TargetArm::Original).unwrap();
    let fb = base.factor_base().unwrap();
    let oracle = PreparedOracle::prepare(
        &source.curve(),
        source.order,
        &fb,
        &base.v,
        base.tower.as_ref(),
        3,
        Presentation::Tower,
    )
    .unwrap();
    let mut records = vec![
        json!({"event":"choice_start","cell":cell}),
        json!({"event":"choice_kernel","fiber":fiber}),
        json!({"event":"choice_prepared","base":base}),
        json!({"event":"choice_polynomial_setup","setup":oracle.setup_report()}),
        json!({"event":"collection_start","public_target":source,"query_seed":cell["query_seed"],
            "arity":3,"presentation":"tower","limits":cell["collection"],"columns":fb.columns,
            "secret_given_to_collector":false,"planted_query_selection":false,
            "oracle_backend":"prepared_polynomial","recovery_mode":"augmented_only"}),
        json!({"event":"driver_error","error":"synthetic development interruption; no query executed"}),
    ];
    let checked = audit(&records).unwrap();
    assert_eq!(checked["choice_polynomial_setup_checked"], true);
    assert_eq!(checked["complete"], false);
    assert_eq!(checked["queries_replayed"], 0);
    for (index, path, value) in [
        (3, "/setup/full_membership_values", json!(1023)),
        (3, "/setup/leaf_domain_values", json!(0)),
        (3, "/setup/fixed_validation_passes", json!(0)),
        (3, "/setup/validated_tower_depth", json!(9)),
        (4, "/recovery_mode", json!("dual_matrices")),
        (4, "/oracle_backend", json!("polynomial")),
    ] {
        let mut bad = records.clone();
        *bad[index].pointer_mut(path).unwrap() = value;
        assert!(audit(&bad).is_err(), "accepted {path} mutation");
    }
    let mut duplicate = records.clone();
    duplicate.insert(4, records[3].clone());
    assert!(audit(&duplicate).is_err());
    records.remove(3);
    assert!(audit(&records).is_err());
}

#[test]
fn fiber_and_specialized_setup_receipts_pass_independent_metadata_binding() {
    // Fixed setup integration only: no target queries, collection or timing
    // measurement. This catches interface drift at the actual fixed base.
    let s = Scratch::new();
    let root = s.0.join("panel");
    driver::prepared_panel(root.to_str().unwrap()).unwrap();
    let index = read(&root.join("screening/index.json"));
    for backend in ["prepared_fiber", "specialized_pair_table"] {
        let presentation = if backend == "prepared_fiber" {
            "tower4"
        } else {
            "dense"
        };
        let entry = index["cells"]
            .as_array()
            .unwrap()
            .iter()
            .find(|e| {
                e["oracle_backend"] == backend
                    && e["presentation"] == presentation
                    && e["factorization"] == "full_two_torsion_first"
                    && e["plain_control"] == false
            })
            .unwrap();
        let cell = read(&root.join("screening").join(entry["file"].as_str().unwrap()));
        let source: PublicTarget = serde_json::from_value(cell["source_target"].clone()).unwrap();
        let fiber = build_fiber_with_factorization(
            KernelChoice::AminusH2,
            1,
            KernelFactorization::FullTwoTorsionFirst,
            &mut GroupOps::default(),
        )
        .unwrap();
        let base = prepare(&fiber, &source, TargetArm::Original).unwrap();
        let fb = base.factor_base().unwrap();
        let setup = if backend == "prepared_fiber" {
            let prepared = PreparedOracle::prepare_fiber(
                &source.curve(),
                source.order,
                &fb,
                &base.v,
                base.tower.as_ref(),
                3,
                Presentation::Tower4,
            )
            .unwrap();
            json!({"event":"choice_polynomial_setup","setup":prepared.setup_report()})
        } else {
            let prepared = SpecializedControl::prepare(&source.curve(), source.order, &fb).unwrap();
            json!({"event":"choice_specialized_setup","setup":prepared.setup_report()})
        };
        let records = vec![
            json!({"event":"choice_start","cell":cell}),
            json!({"event":"choice_kernel","fiber":fiber}),
            json!({"event":"choice_prepared","base":base}),
            setup,
            json!({"event":"collection_start","public_target":source,"query_seed":cell["query_seed"],
                "arity":3,"presentation":presentation,"limits":cell["collection"],"columns":fb.columns,
                "secret_given_to_collector":false,"planted_query_selection":false,
                "oracle_backend":backend,"recovery_mode":"augmented_only"}),
            json!({"event":"driver_error","error":"synthetic development stop after fixed setup"}),
        ];
        let checked = audit(&records).unwrap();
        assert_eq!(checked["complete"], false);
        assert_eq!(checked["queries_replayed"], 0);
        assert_eq!(
            checked[if backend == "prepared_fiber" {
                "choice_polynomial_setup_checked"
            } else {
                "choice_specialized_setup_checked"
            }],
            true
        );
        let mut changed = records.clone();
        changed[3]["setup"]["signed_points"] = json!(0);
        assert!(audit(&changed).is_err());
        let mut missing = records.clone();
        missing.remove(3);
        assert!(audit(&missing).is_err());
        if backend == "prepared_fiber" {
            let mut missing_marker = records.clone();
            missing_marker[3]["setup"]["fiber_index"]
                .as_object_mut()
                .unwrap()
                .remove("reuses_fixed_validation_paths");
            assert!(audit(&missing_marker).is_err());
            for (key, value) in [
                ("reuses_fixed_validation_paths", json!(false)),
                ("construction_field_multiplications", json!(1)),
                ("reused_block_path_values", json!(6143)),
                ("logical_temporary_path_payload_bytes", json!(49151)),
            ] {
                let mut changed = records.clone();
                changed[3]["setup"]["fiber_index"][key] = value;
                assert!(
                    audit(&changed).is_err(),
                    "accepted path-reuse mutation {key}"
                );
            }
        }
        if backend == "specialized_pair_table" {
            // A synthetic result exercises receipt binding, not oracle search.
            // Its claimed negative exhaustion is deliberately not replayed.
            let mut stream = QueryStream::new(cell["query_seed"].as_u64().unwrap());
            let alpha = stream.scalar(source.order);
            let beta = stream.scalar(source.order);
            let curve = source.replay_curve();
            let target = curve
                .add(
                    curve.scalar(source.generator, alpha).unwrap(),
                    curve.scalar(source.target, beta).unwrap(),
                )
                .unwrap();
            assert!(!target.infinity);
            assert!(!fb.points.iter().any(|p| p.x == target.x && p.y == target.y));
            let oracle = OracleResult {
                status: OracleStatus::CompleteNegative,
                complete: true,
                positive: false,
                presentation: Presentation::Dense,
                summands: 3,
                relations: vec![],
                charts: vec![],
                counters: OracleCounters::default(),
                elapsed_ns: 0,
                polynomial_build_ns: 0,
                reason: None,
                accounting: "Synthetic receipt contract; no oracle called".into(),
                backend_work: Some(json!({
                    "backend":"specialized_pair_table",
                    "source_revision":"457ae1818aaf8134bd833ad9c1b9d57015264510",
                    "native_ledger_merged_into_collector":false,"complete_operation_ledger":false,
                    "outer_process_cap_required":true,
                    "first_witness_policy":"cancellation_first_then_native_abscissa_groups; scalar_signed_index_order_at_infinity",
                    "cancellation_checked":true,"cancellation_hit":false,
                    "native":{"native_called":true,"scalar_identity_fallback":false,
                        "field_operation_counters":null,"group_counts_are_complete_native_arithmetic":false}
                })),
            };
            let mut query_records = records[..records.len() - 1].to_vec();
            query_records.push(
                json!({"event":"query_start","query":1,"alpha":alpha,"beta":beta,
                "random_words":stream.words,"target":target}),
            );
            query_records.push(json!({"event":"query_result","query":1,"oracle":oracle}));
            query_records
                .push(json!({"event":"driver_error","error":"synthetic receipt-contract end"}));
            let checked = audit(&query_records).unwrap();
            assert_eq!(checked["queries_replayed"], 1);
            assert_eq!(checked["negative_exhaustion_independently_replayed"], false);
            for (pointer, value) in [
                ("/oracle/backend_work/cancellation_checked", json!(false)),
                ("/oracle/backend_work/cancellation_hit", json!(true)),
                ("/oracle/backend_work/native", Value::Null),
                ("/oracle/backend_work/native/native_called", json!(false)),
                (
                    "/oracle/backend_work/native/scalar_identity_fallback",
                    json!(true),
                ),
                (
                    "/oracle/backend_work/native/field_operation_counters",
                    json!({"multiplications":0}),
                ),
                (
                    "/oracle/backend_work/native/group_counts_are_complete_native_arithmetic",
                    json!(true),
                ),
            ] {
                let mut bad = query_records.clone();
                *bad[6].pointer_mut(pointer).unwrap() = value;
                assert!(
                    audit(&bad).is_err(),
                    "accepted specialized metadata mutation {pointer}"
                );
            }
        }
    }
}
