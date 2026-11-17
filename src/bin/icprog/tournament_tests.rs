//! The tournament port's tests: Python numerics, selection and gate
//! rules, refusals, and two opt-in end-to-end checks (a historical round
//! from its evidence archive, and a wiring-control round run through the
//! real worker under Callgrind).

use super::*;

fn row(case: &str, cell: &str, arm: &str, rep: i128, ops: i128, wall: f64, fb: &str) -> J {
    json::obj([
        ("case", s(case)),
        ("cell", s(cell)),
        ("arm", s(arm)),
        ("repetition", J::Int(rep)),
        ("status", s("VERIFIED")),
        ("total_operations", J::Int(ops)),
        ("case_sha256", s(&format!("sha-{case}"))),
        ("certificate", json::obj([("factor_base_sha256", s(fb))])),
        (
            "native_process",
            json::obj([("process_wall_seconds", J::Float(wall))]),
        ),
    ])
}

#[test]
fn mean_is_the_exact_rational_mean_correctly_rounded() {
    // statistics.mean([0.1, 0.2, 0.3]) == 0.2; the float sum over n is not.
    assert_eq!(py_mean(&[0.1, 0.2, 0.3]).unwrap(), 0.2);
    assert_ne!((0.1 + 0.2 + 0.3) / 3.0, 0.2);
    assert_eq!(py_mean(&[-1.5, 0.5]).unwrap(), -0.5);
    assert_eq!(py_mean(&[1e-300, -1e-300]).unwrap(), 0.0);
    assert!(py_mean(&[]).is_err());
}

#[test]
fn builtin_sum_is_cpython_312s_compensated_sum() {
    assert_eq!(py_sum(&[0.1; 10]), 1.0);
    assert_ne!([0.1f64; 10].iter().fold(0.0, |a, b| a + b), 1.0);
    assert_eq!(py_sum(&[]), 0.0);
    assert_eq!(py_sum(&[1e100, 1.0, -1e100]), 1.0);
}

#[test]
fn median_and_division_follow_python_types() {
    assert!(matches!(
        py_median(&[Num::Int(3), Num::Int(1), Num::Int(2)]).unwrap(),
        Num::Int(2)
    ));
    assert!(matches!(py_median(&[Num::Int(1), Num::Int(2)]).unwrap(), Num::Float(x) if x == 1.5));
    // int / int is correctly rounded even past 2^53.
    let big = (1i128 << 60) + 1;
    assert_eq!(
        py_div(Num::Int(big), Num::Int(3)).unwrap(),
        (big as f64) / 3.0
    );
    assert_eq!(py_div(Num::Int(-7), Num::Int(2)).unwrap(), -3.5);
    assert!(py_div(Num::Int(1), Num::Int(0)).is_err());
}

#[test]
fn python_random_draws() {
    // random.seed(42); [random.random() for _ in range(3)]
    let mut r = Random::new(42);
    assert_eq!(r.random(), 0.6394267984578837);
    assert_eq!(r.random(), 0.025010755222666936);
    assert_eq!(r.random(), 0.27502931836911926);
    // choices(range(10), k=3) draws floor(random() * 10).
    assert_eq!(Random::new(42).choices(10, 3), vec![6, 0, 2]);
    let mut x: Vec<u32> = (0..10).collect();
    Random::new(7).shuffle(&mut x);
    let mut y = x.clone();
    y.sort();
    assert_eq!(y, (0..10).collect::<Vec<_>>());
    let mut z: Vec<u32> = (0..10).collect();
    Random::new(7).shuffle(&mut z);
    assert_eq!(x, z);
}

#[test]
fn identical_arms_compare_at_one_and_a_double_cost_at_two() {
    let mut rows = Vec::new();
    for (case, cell) in [
        ("n13a0-000", "n13a0"),
        ("n13a0-001", "n13a0"),
        ("n17a1-000", "n17a1"),
    ] {
        for rep in 0..3 {
            rows.push(row(case, cell, "incumbent", rep, 1000 + rep, 1.0, "fb"));
            rows.push(row(case, cell, "same", rep, 1000 + rep, 1.0, "fb"));
            rows.push(row(case, cell, "double", rep, 2 * (1000 + rep), 2.0, "fb"));
        }
    }
    let same = comparison(&rows, "same", "incumbent", 200, true).unwrap();
    assert_eq!(same.get("eligible"), Some(&J::Bool(true)));
    assert_eq!(f(same.get("candidate_over_baseline")), 1.0);
    assert_eq!(upper(same.get("ci95")), 1.0);
    assert_eq!(same.get("independent_curve_blocks"), Some(&J::Int(2)));
    let double = comparison(&rows, "double", "incumbent", 200, true).unwrap();
    assert!((f(double.get("candidate_over_baseline")) - 2.0).abs() < 1e-12);
    assert!((f(double.get("native_wall_candidate_over_baseline")) - 2.0).abs() < 1e-12);
    // A missing repetition makes the comparison ineligible, not partial.
    let short: Vec<J> = rows
        .iter()
        .filter(|r| !(r.get("arm") == Some(&s("same")) && r.get("repetition") == Some(&J::Int(2))))
        .cloned()
        .collect();
    let c = comparison(&short, "same", "incumbent", 200, true).unwrap();
    assert_eq!(c.get("reason"), Some(&s("missing paired trials")));
    // A failed trial is retained and makes the arm unpriced.
    let mut failed = rows.clone();
    set(&mut failed[1], "status", s("TIMEOUT"));
    let c = comparison(&failed, "same", "incumbent", 200, true).unwrap();
    assert_eq!(c.get("reason"), Some(&s("unverified or unpriced workload")));
    // Fixed support refuses a changed factor base; a policy comparison does not.
    let mut policy = rows.clone();
    for r in policy
        .iter_mut()
        .filter(|r| r.get("arm") == Some(&s("same")))
    {
        set(
            r,
            "certificate",
            json::obj([("factor_base_sha256", s("other"))]),
        );
    }
    assert!(comparison(&policy, "same", "incumbent", 200, true).is_err());
    assert!(comparison(&policy, "same", "incumbent", 200, false).is_ok());
}

fn result(ratio: f64, upper_ci: f64, cell: f64, wall_upper: f64, wall_cell: f64) -> J {
    json::obj([
        ("eligible", J::Bool(true)),
        ("candidate_over_baseline", J::Float(ratio)),
        (
            "ci95",
            J::Arr(vec![J::Float(ratio * 0.9), J::Float(upper_ci)]),
        ),
        ("per_cell", json::obj([("n13a0", J::Float(cell))])),
        ("native_wall_candidate_over_baseline", J::Float(ratio)),
        (
            "native_wall_ci95",
            J::Arr(vec![J::Float(ratio * 0.9), J::Float(wall_upper)]),
        ),
        (
            "native_wall_per_cell",
            json::obj([("n13a0", J::Float(wall_cell))]),
        ),
    ])
}

fn contract(objective: &str, native: bool) -> J {
    json::obj([
        ("objective", s(objective)),
        ("confirmation_ratio", J::Float(0.8)),
        ("max_cell_ratio", J::Float(1.1)),
        ("no_regression_ratio", J::Float(0.98)),
        ("require_native_progress", J::Bool(native)),
    ])
}

#[test]
fn promotion_gates_are_the_frozen_thresholds() {
    let incumbent = contract("incumbent", false);
    assert!(gate(&result(0.8, 0.99, 1.1, 2.0, 2.0), &incumbent));
    assert!(!gate(&result(0.81, 0.99, 1.0, 0.5, 0.5), &incumbent));
    assert!(!gate(&result(0.7, 1.0, 1.0, 0.5, 0.5), &incumbent));
    assert!(!gate(&result(0.7, 0.9, 1.11, 0.5, 0.5), &incumbent));
    // Native progress, when required, gates the native metric too.
    assert!(!gate(
        &result(0.7, 0.9, 1.0, 1.0, 0.5),
        &contract("incumbent", true)
    ));
    let rho = contract("rho", false);
    assert!(gate(&result(0.98, 0.99, 1.1, 0.99, 1.1), &rho));
    assert!(!gate(&result(0.981, 0.99, 1.0, 0.9, 1.0), &rho));
    assert!(!gate(&result(0.9, 0.99, 1.0, 1.0, 1.0), &rho));
    assert!(!gate(&json::obj([("eligible", J::Bool(false))]), &rho));
    // A wiring control never promotes.
    let mut wiring = rho.clone();
    set(&mut wiring, "purpose", s("wiring-control"));
    assert!(!gate(&result(0.5, 0.6, 0.6, 0.6, 0.6), &wiring));
    // Strictly below matched rho: every upper limit and every cell below one.
    assert!(rho_gate(&result(0.5, 0.99, 0.99, 0.99, 0.99)));
    assert!(!rho_gate(&result(0.5, 0.99, 1.0, 0.99, 0.99)));
    assert!(!rho_gate(&result(0.5, 0.99, 0.99, 1.0, 0.99)));
}

fn temp_dir(name: &str) -> PathBuf {
    let base = std::env::var_os("IC_TOURNAMENT_TEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    let dir = base.join(format!("icprog-tournament-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn selection_keeps_the_two_cheapest_eligible_and_drops_smoke_failures() {
    let dir = temp_dir("selection");
    let arm = |id: &str| json::obj([("id", s(id)), ("config", base_config())]);
    let arms = vec![arm("incumbent"), arm("a"), arm("b"), arm("c"), arm("d")];
    let comps = J::Arr(vec![
        json::obj([
            ("candidate", s("a")),
            ("eligible", J::Bool(true)),
            ("candidate_over_baseline", J::Float(0.9)),
        ]),
        json::obj([
            ("candidate", s("b")),
            ("eligible", J::Bool(false)),
            ("reason", s("x")),
        ]),
        json::obj([
            ("candidate", s("c")),
            ("eligible", J::Bool(true)),
            ("candidate_over_baseline", J::Float(0.7)),
        ]),
        json::obj([
            ("candidate", s("d")),
            ("eligible", J::Bool(true)),
            ("candidate_over_baseline", J::Float(0.95)),
        ]),
    ]);
    write(
        &dir.join("summaries/development.json"),
        &json::obj([("comparisons", comps)]),
        true,
    )
    .unwrap();
    let failures = J::Arr(vec![json::obj([("arm", s("b"))])]);
    write(
        &dir.join("summaries/smoke.json"),
        &json::obj([("failures", failures)]),
        true,
    )
    .unwrap();
    write(
        &dir.join("summaries/selection.json"),
        &json::obj([("provisional_challenger", s("c"))]),
        true,
    )
    .unwrap();
    let st = Store::Dir(dir.clone());
    let ids = |v: Vec<J>| -> Vec<String> {
        v.iter()
            .map(|a| a.get("id").unwrap().as_str().unwrap().to_string())
            .collect()
    };
    assert_eq!(
        ids(stage_arms(&st, "aa", &arms).unwrap()),
        ["incumbent", "aa_control"]
    );
    assert_eq!(
        ids(stage_arms(&st, "development", &arms).unwrap()),
        ["incumbent", "a", "c", "d", "rho"]
    );
    assert_eq!(
        ids(stage_arms(&st, "selection", &arms).unwrap()),
        ["incumbent", "c", "a", "rho"]
    );
    assert_eq!(
        ids(stage_arms(&st, "confirmation", &arms).unwrap()),
        ["incumbent", "c", "rho"]
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn panel_parsing_refuses_what_the_frozen_prepare_refuses() {
    assert_eq!(parse_cells("13a0, n17a1").unwrap(), vec![(13, 0), (17, 1)]);
    assert!(parse_cells("13a0,13a0").is_err());
    assert!(parse_cells("13a2").is_err());
    assert!(parse_cells("").is_err());
    let declared = vec!["n13a0".to_string(), "n23a1".to_string()];
    assert_eq!(
        parse_confirmation_allocation("n23a1=48", 12, &declared).unwrap(),
        vec![("n23a1".to_string(), 48)]
    );
    assert!(parse_confirmation_allocation("n23a1=11", 12, &declared).is_err());
    assert!(parse_confirmation_allocation("n29a1=48", 12, &declared).is_err());
    assert!(parse_confirmation_allocation("n23a1", 12, &declared).is_err());
}

#[test]
fn run_refuses_historical_closed_and_bounded_registrations() {
    let native = json::obj([("port", s(PORT))]);
    let registered = json::obj([("kind", s("registered"))]);
    let ok = json::obj([
        ("native_evaluator", native.clone()),
        ("seed", J::Int(4_000_000_001)),
        ("purpose", s("improvement")),
        ("registration", registered.clone()),
    ]);
    assert!(runnable(&ok).is_ok());
    // Every Python-evaluated round, including bounded rounds 1-3.
    let mut historical = ok.clone();
    remove(&mut historical, "native_evaluator");
    assert!(runnable(&historical)
        .unwrap_err()
        .contains("Python evaluator"));
    for &seed in &[
        2026092551i128,
        2026092552,
        2026092553,
        2026092901,
        2026092902,
        2026093001,
        20260915,
        2026092525,
    ] {
        let mut closed = ok.clone();
        set(&mut closed, "seed", J::Int(seed));
        assert!(
            runnable(&closed)
                .unwrap_err()
                .contains("consumed or closed"),
            "{seed}"
        );
    }
    let mut bounded = ok.clone();
    set(&mut bounded, "attempt_number", J::Int(1));
    assert!(runnable(&bounded).is_err());
    let mut qualification = ok.clone();
    set(&mut qualification, "purpose", s("reference-qualification"));
    assert!(runnable(&qualification).is_err());
    let mut unregistered = ok.clone();
    set(
        &mut unregistered,
        "registration",
        json::obj([("kind", s("wiring-control"))]),
    );
    assert!(runnable(&unregistered).is_err());
}

#[test]
fn profile_text_helpers_follow_python() {
    assert_eq!(splitlines("a\r\nb\rc\n\nd"), vec!["a", "b", "c", "", "d"]);
    assert_eq!(collected("==1== Collected : 123\n"), vec!["123"]);
    assert_eq!(collected("Collected:9 Collected x"), vec!["9"]);
    // Built at run time: .github/scripts/ci-scope.mjs scans source text for
    // literal include directives and would take this fixture for a real one.
    let fixture = format!("include_str!( \"a.txt\"); {}(\"b.rs\")", "include!");
    assert_eq!(includes(&fixture), vec!["b.rs", "a.txt"]);
}

/// A historical round's frozen verdict, from its committed archive:
/// `IC_TOURNAMENT_ARCHIVE=…/evidence/round-0020.tar.zst
/// IC_TOURNAMENT_MEMBER=runs/round-0020 cargo test … -- --ignored`.
#[test]
#[ignore]
fn historical_round_verifies_to_its_frozen_audit() {
    let archive = PathBuf::from(std::env::var("IC_TOURNAMENT_ARCHIVE").expect("archive"));
    let member = std::env::var("IC_TOURNAMENT_MEMBER").expect("member");
    let (st, _) = store::load_archive(&archive, &member, None).unwrap();
    let derived = audit(&st, None).unwrap();
    let frozen = read(&st, "audit.json").unwrap();
    assert!(py_eq(&derived, &frozen), "{derived:?} != {frozen:?}");
}

/// The whole native path on a tiny synthetic panel, through the real
/// worker under Callgrind: a wiring control, never a result.
/// `IC_TOURNAMENT_WORKER=target/release/examples/ic_tournament_worker
/// cargo test … wiring -- --ignored`.
#[test]
#[ignore]
fn wiring_control_prepares_runs_and_verifies() {
    let worker = PathBuf::from(std::env::var("IC_TOURNAMENT_WORKER").expect("worker"));
    let worker = std::path::absolute(worker).unwrap();
    let dir = temp_dir("wiring");
    let root = dir.join("round");
    let registry = dir.join("candidates.json");
    let mut arms = default_candidates().as_arr().unwrap()[..2].to_vec();
    assert_eq!(arms[1].get("id"), Some(&s("batch16")));
    arms.truncate(2);
    write(&registry, &J::Arr(arms), true).unwrap();
    let exe = std::env::current_exe().unwrap();
    let cells = std::env::var("IC_TOURNAMENT_CELLS").unwrap_or_else(|_| "7a1".into());
    let holdout = std::env::var("IC_TOURNAMENT_HOLDOUT").unwrap_or_else(|_| "7a0".into());
    let args = PrepareArgs {
        out: root.clone(),
        source_root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        comparison_kind: ComparisonKind::FixedSupport,
        candidates: Some(registry),
        profile: Profile::Wiring,
        confirmation_cases: String::new(),
        cells,
        holdout_cells: holdout,
        seed: 4_242_000_001,
        cpu: None,
        timeout: 120.0,
        max_processes: 200,
        require_native_progress: true,
        targets: 1,
        objective: Objective::Rho,
        protocol: None,
        wiring_control: true,
        worker_binary: Some(worker.clone()),
        evaluator: exe.clone(),
    };
    let prepared = prepare(&args).unwrap();
    assert_eq!(prepared.get("status"), Some(&s("prepared")));
    // Preparation refuses to overwrite a frozen round.
    assert!(prepare(&args).is_err());
    let mut lines = 0;
    run_campaign(&root, "all", &exe, &mut |_| lines += 1).unwrap();
    let st = Store::Dir(root.clone());
    let decision = read(&st, "decision.json").unwrap();
    assert_ne!(decision.get("status"), Some(&s("promoted")));
    let verdict = audit(&st, Some(&exe)).unwrap();
    assert_eq!(verdict.get("status"), Some(&s("VERIFIED")));
    let receipts = verdict.get("trial_receipts").unwrap().as_i128().unwrap();
    assert!(receipts >= 45, "{receipts}");
    // Resuming a finished round runs nothing and rewrites nothing.
    let before = store::digest_file(&root.join("decision.json")).unwrap();
    let mut again = 0;
    run_campaign(&root, "all", &exe, &mut |_| again += 1).unwrap();
    assert_eq!(again, 0);
    // One line per trial, then the decision.
    assert_eq!(lines as i128, receipts + 1);
    assert_eq!(
        store::digest_file(&root.join("decision.json")).unwrap(),
        before
    );
    // A changed artifact is detected.
    let receipt = st
        .files_under("runs/aa")
        .into_iter()
        .find(|f| f.ends_with("native/stdout.json"))
        .unwrap();
    let original = std::fs::read(root.join(&receipt)).unwrap();
    std::fs::write(root.join(&receipt), [&original[..], b" "].concat()).unwrap();
    assert!(audit(&st, Some(&exe))
        .unwrap_err()
        .contains("changed trial artifact"));
    std::fs::write(root.join(&receipt), &original).unwrap();
    // Another evaluator binary is refused.
    assert!(audit(&st, Some(&worker))
        .unwrap_err()
        .contains("frozen evaluator copy"));
    assert_eq!(
        audit(&st, Some(&exe)).unwrap().get("status"),
        Some(&s("VERIFIED"))
    );
    if std::env::var_os("IC_TOURNAMENT_KEEP").is_none() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
