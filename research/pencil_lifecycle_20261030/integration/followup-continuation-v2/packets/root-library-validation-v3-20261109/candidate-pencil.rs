//! Adapter for the public synthetic Scholten pencil workload.
//! Full native ledgers are retained; group-only GAE is explicitly a lower bound.
use crate::cryptanalysis::ic_measurement as measurement;
use crate::cryptanalysis::three_leads as engine;
use pencil_symbolic_graph as kernel;
#[path = "../../../research/pencil_large_prime_20261029/src/native_solver.rs"]
mod native_solver;
#[path = "pencil_progress_sink.rs"]
pub(super) mod progress;
#[path = "pencil_progress_native.rs"]
mod progress_native;

use super::super::methods::{PhaseRecord, ResolvedMethod, SolveReport};
use engine::{Counts, Point, StudyInstance};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    path::Path,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};
use std::{collections::BTreeMap, time::Instant};

struct InstalledProgress {
    token: u64,
    sink: progress::AtomicSink,
}
thread_local! {
    static PREFIX_OWNER: Cell<u64> = const { Cell::new(0) };
    static PREFIX: RefCell<Option<InstalledProgress>> = const { RefCell::new(None) };
}
static NEXT_PREFIX: AtomicU64 = AtomicU64::new(1);

/// Explicit child-installed candidate sink. Never inferred from environment.
/// Drop clears ownership only; finish returns diagnostic IO totals separately.
pub(crate) struct ScopedPencilProgress {
    token: u64,
    _thread: PhantomData<Rc<()>>,
}
impl ScopedPencilProgress {
    pub(crate) fn install(
        path: &Path,
        identity: super::super::diagnostics::Identity,
    ) -> Result<Self, String> {
        super::super::diagnostics::Context {
            schema: "ecbench.diagnostic_context/v1".into(),
            relative_leaf: "prefix".into(),
            identity: identity.clone(),
        }
        .validate()?;
        PREFIX.with(|cell| {
            let mut state = cell
                .try_borrow_mut()
                .map_err(|_| "pencil sink is borrowed")?;
            if PREFIX_OWNER.with(Cell::get) != 0 {
                return Err("pencil progress already installed".into());
            }
            // A guard dropped during a callback invalidates ownership immediately.
            // Any stale sink is reclaimed only after that borrow has ended.
            *state = None;
            let sink = progress::AtomicSink::create(
                path,
                progress::Identity {
                    session: identity.session,
                    execution: identity.execution,
                    workload_sha256: identity.workload_sha256,
                    binary_sha256: identity.binary_sha256,
                    protocol_sha256: identity.protocol_sha256,
                    seed: identity.seed,
                    attempt: identity.attempt,
                    source_sha256: identity.source_sha256,
                    method_config_sha256: identity.method_config_sha256,
                },
            )
            .map_err(|e| e.to_string())?;
            let token = NEXT_PREFIX
                .try_update(Ordering::Relaxed, Ordering::Relaxed, |x| x.checked_add(1))
                .map_err(|_| "pencil ownership exhausted")?;
            *state = Some(InstalledProgress { token, sink });
            PREFIX_OWNER.with(|owner| owner.set(token));
            Ok(Self {
                token,
                _thread: PhantomData,
            })
        })
    }
    pub(crate) fn finish(self) -> Result<progress::IoTotals, String> {
        PREFIX.with(|cell| {
            let mut state = cell
                .try_borrow_mut()
                .map_err(|_| "pencil sink is borrowed")?;
            if PREFIX_OWNER.with(Cell::get) != self.token
                || state.as_ref().map(|s| s.token) != Some(self.token)
            {
                return Err("pencil progress ownership mismatch".into());
            }
            let owned = state.take().ok_or("pencil sink absent")?;
            PREFIX_OWNER.with(|o| o.set(0));
            Ok(owned.sink.totals)
        })
    }
}
impl Drop for ScopedPencilProgress {
    fn drop(&mut self) {
        PREFIX_OWNER.with(|owner| {
            if owner.get() == self.token {
                owner.set(0)
            }
        });
        PREFIX.with(|cell| {
            if let Ok(mut state) = cell.try_borrow_mut() {
                if state.as_ref().map(|s| s.token) == Some(self.token) {
                    *state = None;
                }
            }
        });
    }
}
pub(super) fn with_active_progress<T>(
    callback: impl FnOnce(&mut dyn progress::Sink) -> T,
) -> Result<Option<T>, String> {
    if PREFIX_OWNER.with(Cell::get) == 0 {
        return Ok(None);
    }
    PREFIX.with(|cell| {
        let mut state = cell
            .try_borrow_mut()
            .map_err(|_| "reentrant pencil sink dispatch")?;
        let active = state.as_mut().ok_or("pencil owner without sink")?;
        if active.token != PREFIX_OWNER.with(Cell::get) {
            return Err("pencil sink ownership mismatch".into());
        }
        Ok(Some(callback(&mut active.sink)))
    })
}

fn config(m: &ResolvedMethod) -> Result<native_solver::Config, String> {
    let parse = |key: &str| -> Result<usize, String> {
        m.params
            .get(key)
            .ok_or_else(|| format!("missing {key}"))?
            .parse()
            .map_err(|_| format!("invalid {key}"))
    };
    let config = native_solver::Config {
        denominator: parse("small_denominator")?,
        matrix_coefficient_cap: parse("matrix_coefficient_cap")?,
    };
    config.validate().map_err(|error| error.to_string())?;
    Ok(config)
}

// ecbench replay compares counters, not arbitrary detail. Bind the entire
// deterministic ledger (including failure and phase attribution) into four
// exact u64 words. Timings and allocator observations are not deterministic.
fn replay_ledger(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let sorted: BTreeMap<_, _> = map
                .iter()
                .filter(|(key, _)| {
                    !matches!(
                        key.as_str(),
                        "allocation"
                            | "candidate_envelope_allocation"
                            | "cold_setup_allocation"
                            | "allocation_observer_available"
                            | "elapsed_ns"
                            | "candidate_elapsed_ns"
                            | "cold_setup_elapsed_ns"
                            | "candidate_phase_overhead_ns"
                    )
                })
                .map(|(key, value)| (key.clone(), replay_ledger(value)))
                .collect();
            serde_json::to_value(sorted).expect("JSON ledger")
        }
        Value::Array(values) => Value::Array(values.iter().map(replay_ledger).collect()),
        other => other.clone(),
    }
}
fn ledger_digest(candidate: &Value, setup: &Value) -> [u64; 4] {
    let value = replay_ledger(&json!({"candidate":candidate,"setup":setup}));
    let hash = Sha256::digest(serde_json::to_vec(&value).expect("JSON ledger"));
    std::array::from_fn(|i| u64::from_be_bytes(hash[i * 8..i * 8 + 8].try_into().unwrap()))
}

// Preserve each native result's private graph/solver owners through verification,
// calibration and report accounting. No large graph clone or early owner drop.
enum NativeResult {
    Original(native_solver::PencilResult),
    Progress(progress_native::PencilResult),
}
impl NativeResult {
    fn recovered(&self) -> Option<u64> {
        match self {
            Self::Original(r) => r.recovered,
            Self::Progress(r) => r.recovered,
        }
    }
    fn verified(&self) -> bool {
        match self {
            Self::Original(r) => r.source_verified,
            Self::Progress(r) => r.source_verified,
        }
    }
    fn detail(&self) -> &Value {
        match self {
            Self::Original(r) => &r.detail,
            Self::Progress(r) => &r.detail,
        }
    }
    fn setup(&self) -> &Value {
        match self {
            Self::Original(r) => &r.setup,
            Self::Progress(r) => &r.setup,
        }
    }
    fn take_detail(&mut self) -> Value {
        match self {
            Self::Original(r) => std::mem::take(&mut r.detail),
            Self::Progress(r) => std::mem::take(&mut r.detail),
        }
    }
    fn take_setup(&mut self) -> Value {
        match self {
            Self::Original(r) => std::mem::take(&mut r.setup),
            Self::Progress(r) => std::mem::take(&mut r.setup),
        }
    }
}

pub(super) fn solve(
    m: &ResolvedMethod,
    inst: &StudyInstance,
    target: Point,
    seed: u64,
) -> Result<SolveReport, String> {
    solve_with_progress(m, inst, target, seed, None)
}

/// Caller must supply an explicitly admitted sink; absent sink is the original route.
/// Runtime ownership/launch selection is separate and is not enabled by this API.
pub(super) fn solve_with_progress(
    m: &ResolvedMethod,
    inst: &StudyInstance,
    target: Point,
    seed: u64,
    progress: Option<&mut dyn progress::Sink>,
) -> Result<SolveReport, String> {
    let adapter_start = Instant::now();
    let geometry_start = engine::geometry_events::snapshot();
    let config = config(m)?;
    let start = Instant::now();
    // Root ecbench has no installed ObservedSystem. Never fabricate zero memory.
    let mut result = match progress {
        None => NativeResult::Original(
            native_solver::solve_one(inst, target, seed, config, None)
                .map_err(|error| error.to_string())?,
        ),
        Some(sink) => NativeResult::Progress(
            progress_native::solve_one_with_progress(
                inst,
                target,
                seed,
                progress_native::Config {
                    denominator: config.denominator,
                    matrix_coefficient_cap: config.matrix_coefficient_cap,
                },
                None,
                sink,
            )
            .map_err(|error| error.to_string())?,
        ),
    };
    let geometry_after_solver = engine::geometry_events::snapshot();
    let mut group: Counts = serde_json::from_value(result.detail()["counts"].clone())
        .map_err(|e| format!("pencil group ledger: {e}"))?;
    let mut verifier_counts = Counts::default();
    if let Some(k) = result.recovered() {
        let verifier = inst.fresh();
        if !result.verified() || verifier.curve.mul(verifier.generator, k) != target {
            return Err("fresh source verification rejected pencil result".into());
        }
        verifier_counts = verifier.curve.f.counts();
        group = group + verifier_counts;
    }
    let solve_wall_ns =
        u64::try_from(start.elapsed().as_nanos()).map_err(|_| "pencil interval overflow")?;
    let geometry_after_verification = engine::geometry_events::snapshot();
    let unit = inst.fresh();
    let twice = unit.curve.mul(unit.generator, 2);
    let calibration_preparation = unit.curve.f.counts();
    unit.curve.f.reset();
    let _ = unit.curve.add(unit.generator, twice);
    let source_add = unit.curve.f.counts();
    let geometry_after_calibration = engine::geometry_events::snapshot();
    let price = |c: Counts| c.m as f64 + 0.75 * c.s as f64 + 10.0 * c.i as f64;
    if price(source_add) <= 0.0 {
        return Err("zero source-addition unit".into());
    }
    let group_only_gae = price(group) / price(source_add);
    let mut counters = BTreeMap::from([
        ("group_field_multiplications".into(), group.m),
        ("group_field_squarings".into(), group.s),
        ("group_field_inversions".into(), group.i),
        ("group_field_adds_uncharged".into(), group.field_adds),
        ("group_bit_ops_uncharged".into(), group.bit_ops),
    ]);
    for (i, word) in ledger_digest(result.detail(), result.setup())
        .into_iter()
        .enumerate()
    {
        counters.insert(format!("pencil_replay_ledger_sha256_word_{i}"), word);
    }
    let mut report = SolveReport {
        recovered: result.recovered().map(u128::from),
        exhausted: result.recovered().is_none(),
        phases: vec![PhaseRecord {
            name: "counted_group_component_only".into(),
            adds: group.ec_adds + group.jac_adds,
            doubles: 0,
            scalar_mults: 0,
            gae: group_only_gae,
            wall_ns: None,
            native: counters.clone(),
            work_native: BTreeMap::new(),
        }],
        total_gae: group_only_gae,
        automorphisms_used: 1,
        counters,
        unpriced: vec![
            "pencil_native_and_symbolic_arithmetic".into(),
            "pencil_logical_work".into(),
            "allocation_observer_unavailable".into(),
        ],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns,
        factor_base: None,
        detail: json!({"schema":"ecbench-pencil-symbolic/v2","candidate":result.take_detail(),
            "cold_setup":result.take_setup(),"additional_source_verification":verifier_counts,
            "source_addition":source_add,"cost_status":"incomplete",
            "total_gae_scope":"counted group arithmetic only; lower bound",
            "candidate_over_rho":null,"allocation_observer_available":false}),
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    };
    let geometry_end = engine::geometry_events::snapshot();
    report.detail["adapter_accounting"] = json!({
        "schema":"pencil-adapter-accounting/v1-draft",
        "geometry": engine::geometry_events::partition(geometry_start, &[
            ("solver_including_cold_setup", geometry_after_solver),
            ("additional_source_verification", geometry_after_verification),
            ("source_unit_calibration", geometry_after_calibration),
            ("report_assembly", geometry_end),
        ]).ok_or("invalid adapter geometry partition")?,
        "source_unit_calibration_arithmetic": {
            "preparation":calibration_preparation,"addition":source_add,
            "total":calibration_preparation+source_add,
            "included_in_legacy_group_total":false
        },
        "adapter_elapsed_ns_before_accounting_attachment":adapter_start.elapsed().as_nanos(),
        "coverage_complete":false,
        "report_allocation_and_serialization_counted":false,
        "partition_policy":"whole geometry is total; named segments partition it and must not be added again",
        "accounting_attachment_is_observer_overhead":true
    });
    let accounting_replay = json!({
        "geometry":report.detail["adapter_accounting"]["geometry"],
        "calibration":report.detail["adapter_accounting"]["source_unit_calibration_arithmetic"]
    });
    for (i, word) in ledger_digest(&accounting_replay, &Value::Null)
        .into_iter()
        .enumerate()
    {
        report
            .counters
            .insert(format!("adapter_accounting_sha256_word_{i}"), word);
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn diagnostic_identity() -> super::super::super::diagnostics::Identity {
        super::super::super::diagnostics::Identity {
            session: "fixture".into(),
            execution: 0,
            seed: u64::MAX,
            attempt: 0,
            workload_sha256: "a".repeat(64),
            binary_sha256: "b".repeat(64),
            protocol_sha256: "c".repeat(64),
            source_sha256: "d".repeat(64),
            method_config_sha256: "e".repeat(64),
        }
    }
    #[test]
    fn scoped_prefix_fake_dispatch_and_shared_meter() {
        let path =
            std::env::temp_dir().join(format!("pencil-adapter-scope-{}", std::process::id()));
        assert!(with_active_progress(|_| panic!("absent callback"))
            .unwrap()
            .is_none());
        let scope = ScopedPencilProgress::install(&path, diagnostic_identity()).unwrap();
        assert!(ScopedPencilProgress::install(&path, diagnostic_identity()).is_err());
        kernel::meter::reset();
        with_active_progress(|_| {
            kernel::meter::collection(3, 0, 0, 0);
            assert!(with_active_progress(|_| ()).is_err());
        })
        .unwrap()
        .unwrap();
        assert_eq!(
            pencil_symbolic_graph::meter::snapshot().collection_entry_visits,
            3
        );
        let totals = scope.finish().unwrap();
        assert_eq!(totals.attempted, 0);
        assert!(with_active_progress(|_| ()).unwrap().is_none());
        std::fs::remove_dir_all(path).unwrap();
    }
    #[test]
    fn scoped_prefix_panic_clears_ownership() {
        let path =
            std::env::temp_dir().join(format!("pencil-adapter-panic-{}", std::process::id()));
        let result = std::panic::catch_unwind(|| {
            let _scope = ScopedPencilProgress::install(&path, diagnostic_identity()).unwrap();
            with_active_progress(|_| panic!("synthetic callback panic")).unwrap();
        });
        assert!(result.is_err());
        assert!(with_active_progress(|_| ()).unwrap().is_none());
        std::fs::remove_dir_all(path).unwrap();
    }
    #[test]
    fn audited_digest_covers_work_and_failures_but_excludes_observations() {
        let a = json!({"arithmetic":{"base_muls":17},"failure":"walk_cap",
            "phases":{"sieve":{"elapsed_ns":10,"symbolic":{"map_calls":2}}},
            "candidate_elapsed_ns":19,"allocation":{"alloc_calls":4}});
        let mut b = a.clone();
        b["candidate_elapsed_ns"] = json!(91);
        b["phases"]["sieve"]["elapsed_ns"] = json!(80);
        b["allocation"]["alloc_calls"] = json!(40);
        assert_eq!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
        let mut future_work = a.clone();
        future_work["allocation_work_units"] = json!(7);
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&future_work, &Value::Null)
        );
        b["phases"]["sieve"]["symbolic"]["map_calls"] = json!(3);
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
        b = a.clone();
        b["failure"] = Value::Null;
        assert_ne!(
            ledger_digest(&a, &Value::Null),
            ledger_digest(&b, &Value::Null)
        );
    }

    #[test]
    fn actual_some_none_known_answer_and_diagnostic_ledger() {
        use crate::cryptanalysis::ecbench::methods::{resolve, MethodSpec};
        // Fixed first execution of the existing prepared p739 gate; no search.
        let method = resolve(&MethodSpec {
            id: "ic.three_leads_genus2_pencil_symbolic_v2".into(),
            params: BTreeMap::from([
                ("small_denominator".into(), "8".into()),
                ("matrix_coefficient_cap".into(), "25000000".into()),
            ]),
        }).unwrap();
        assert_eq!(method.method_sha256,
            "187cc5c6d747a3b25cfd9708e970b511886a46c49b5db9fd3b09ce5b8a7fc983");
        let inst = StudyInstance {
            curve: engine::Elliptic {
                f: engine::Field::new(739, 2, 1),
                c: [193650, 493102, 0, 1], binary: false,
            },
            order: 546352, r: 34147, h: 16,
            generator: Point::affine(433246, 59741),
            family: "scholten".into(), parameter: 0, evidence: Value::Null,
        };
        inst.validate().unwrap();
        let target = Point::affine(0x15ed, 0x5b1d4);
        let known = 31124u64;
        let verifier = inst.fresh();
        assert_eq!(verifier.curve.mul(verifier.generator, known), target);
        assert_ne!(verifier.curve.mul(verifier.generator, known + 1), target);
        #[derive(Default)]
        struct Capture(Vec<(String, String, Option<u64>, u64, Value)>);
        impl progress::Sink for Capture {
            fn emit(&mut self, boundary: &str, epoch: &str, walk: Option<u64>, completed: u64, value: Value) {
                assert!(self.0.len() < progress::MAX_SNAPSHOTS as usize, "bounded software-control capture");
                self.0.push((boundary.into(), epoch.into(), walk, completed, value));
            }
        }
        let seed = 17823097723983109480u64;
        let session = measurement::Session::begin().unwrap();
        let plain = solve_with_progress(&method, &inst.fresh(), target, seed, None).unwrap();
        assert!(session.finish().unwrap().online_wall_ns.is_some());
        let mut capture = Capture::default();
        let session = measurement::Session::begin().unwrap();
        let traced = solve_with_progress(&method, &inst.fresh(), target, seed, Some(&mut capture)).unwrap();
        assert!(session.finish().unwrap().online_wall_ns.is_some());
        for report in [&plain, &traced] {
            assert_eq!(report.recovered, Some(u128::from(known)));
            assert!(!report.exhausted);
            assert_eq!(report.detail["candidate"]["verified"], json!(true));
            assert_eq!(report.detail["candidate"]["failure"], Value::Null);
            assert_eq!(report.detail["candidate_over_rho"], Value::Null);
            assert_eq!(report.detail["allocation_observer_available"], json!(false));
            let fresh = inst.fresh();
            assert_eq!(fresh.curve.mul(fresh.generator, report.recovered.unwrap() as u64), target);
        }
        assert_eq!(plain.counters, traced.counters);
        assert_eq!(replay_ledger(&plain.detail["candidate"]), replay_ledger(&traced.detail["candidate"]));
        assert_eq!(replay_ledger(&plain.detail["cold_setup"]), replay_ledger(&traced.detail["cold_setup"]));
        let boundaries: Vec<_> = capture.0.iter().map(|r|r.0.as_str()).collect();
        assert_eq!(&boundaries[..3], &["rational_table_complete", "cold_setup_complete", "candidate_setup_complete"]);
        assert_eq!(boundaries.last(), Some(&"solver_returning"));
        let walks = traced.detail["candidate"]["walks"].as_u64().unwrap();
        assert!(walks > 0 && walks <= 16);
        let observed: Vec<_> = capture.0.iter().filter(|r|r.0=="walk_generation_complete").map(|r|r.2.unwrap()).collect();
        assert_eq!(observed, (0..walks).collect::<Vec<_>>());
        assert!(boundaries.contains(&"sieve_complete"));
        assert!(boundaries.contains(&"candidate_processing_complete"));
        for (boundary, epoch, walk, completed, value) in &capture.0 {
            assert!(value.get("field").is_some() && value.get("arithmetic").is_some() && value.get("symbolic").is_some());
            match boundary.as_str() {
                "rational_table_complete" | "cold_setup_complete" => {assert_eq!(epoch,"cold_setup");assert_eq!(*walk,None);}
                "candidate_setup_complete" | "solver_returning" => {assert_eq!(epoch,"candidate");assert_eq!(*walk,None);}
                "walk_generation_complete" | "sieve_complete" => {assert_eq!(epoch,"candidate");assert!(walk.unwrap()<walks);}
                "candidate_processing_complete" => {assert_eq!(epoch,"candidate");assert!(walk.unwrap()<walks);assert!(progress::milestone(*completed));}
                _ => panic!("unexpected diagnostic boundary"),
            }
        }
        println!("ROOT_ADAPTER_POSITIVE_CONTROL recovered=31124 actual_routes=2 deterministic_ledgers_equal=true diagnostic_events={}", capture.0.len());
    }
}
