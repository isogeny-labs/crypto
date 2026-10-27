//! Plan sanity and a structural smoke test of the drivers on a toy fixture (n = 11). The rows written by the
//! smoke test go to cargo's per-target tmp dir and are never read for findings; only schemas and exact
//! instrument identities (which must hold on any input) are asserted.
use g5_propt_census::run::*;

#[test]
fn plan_is_sane_and_prints_cost_table() {
    let cells = cell_list();
    assert!(!cells.is_empty());
    let mut total = 0.0;
    for c in &cells {
        assert!(c.l >= 3 && c.l <= c.n - 2);
        assert!((8..=200).contains(&c.reps));
        assert!((2..=8).contains(&c.m));
    }
    for m in 2..=4usize {
        assert!(
            cells
                .iter()
                .any(|c| c.a == 1 && c.n == 17 && c.l == 8 && c.m == m),
            "stable cell m={m}"
        );
    }
    for &(a, n) in &[(1u32, 11u32), (0, 13), (1, 17), (0, 19)] {
        for m in 2..=8usize {
            assert!(
                cells.iter().any(|c| c.a == a && c.n == n && c.m == m),
                "missing primary cell a={a} n={n} m={m}"
            );
        }
    }
    println!("a n m l reps nx_exp nu_exp est_ops_per_base est_ops_cell tuned_n");
    let mut ops_all = 0.0;
    for c in &cells {
        let f = g5_propt_census::field::Fq::new(c.n);
        let (_ne, r, _) = g5_propt_census::curve::curve_summary(&f, c.a);
        let nx = ((1u64 << c.l) as f64 * (r as f64 - 1.0) / (2.0 * f.q as f64)).round();
        let jn = ((c.n - 1) / 2) as usize;
        let ops = g5_propt_census::census::est_ops(nx as usize, c.m, jn);
        let tn = tuned_size(c.m, r);
        let tops = g5_propt_census::census::est_ops(tn, c.m, jn);
        let cell_ops = c.reps as f64 * 2.0 * (2.0 * ops + tops);
        ops_all += cell_ops;
        println!(
            "{} {} {} {} {} {} {:.2} {:.2e} {:.2e} {}",
            c.a,
            c.n,
            c.m,
            c.l,
            c.reps,
            nx,
            nu_exp(nx, c.m, r),
            ops,
            cell_ops,
            tn
        );
        total += 1.0;
    }
    println!("cells: {total} total est ops {ops_all:.3e}");
}

#[test]
fn drivers_write_consistent_schemas() {
    let dir = format!("{}/g5_smoke", env!("CARGO_TARGET_TMPDIR"));
    let o = RunOpts {
        seed: 7,
        reps: Some(2),
        out: dir.clone(),
    };
    run_propt(11, 3, 120, 2, &o).unwrap();
    let cell = Cell {
        a: 1,
        n: 11,
        m: 3,
        l: 6,
        reps: 2,
    };
    run_overlap(&cell, &o).unwrap();
    run_rank(&cell, 6, 30, &o).unwrap();
    for f in [
        "propt_n11_m3.tsv",
        "overlap_a1_n11_m3_l6.tsv",
        "rank_a1_n11_m3_l6.tsv",
    ] {
        let txt = std::fs::read_to_string(format!("{dir}/{f}")).unwrap();
        let mut lines = txt.lines();
        let h = lines.next().unwrap().split('\t').count();
        let mut n = 0;
        for l in lines {
            assert_eq!(l.split('\t').count(), h, "{f}: ragged row");
            n += 1;
        }
        assert!(n > 0, "{f}");
    }
    g5_propt_census::analyze::analyze(&dir).unwrap();
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    if std::env::var("G5_SHOW").is_ok() {
        println!("{md}"); // formatting review of the generated summary on the smoke fixture (not a result)
    }
    assert!(md.contains("Voids and failures"));
    // exact identities must hold on any input: no void may mention Proposition T, halving, chain or closure
    assert!(!md.contains("INSTRUMENT_FAIL"), "{md}");
    assert!(!md.contains("PROP_T_FALSIFIED"), "{md}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// Design-stage power table of the frozen rule on synthetic Poisson counts (no experiment data are read or produced).
#[test]
fn decision_rule_operating_characteristic() {
    use g5_propt_census::analyze::oc_probs;
    let cells = cell_list();
    println!("OC: a n m l reps U_per_base | D=1: E-OVERLAP E-EXTRA E-DEFICIT UNRES AAFAIL | D=1.15: E-OVERLAP E-EXTRA E-DEFICIT UNRES AAFAIL | D=0.87: E-DEFICIT");
    for &(a, n) in &[(1u32, 11u32), (0, 13), (1, 17), (0, 19)] {
        for &m in &[2usize, 4, 8] {
            let c = match cells
                .iter()
                .filter(|c| c.a == a && c.n == n && c.m == m)
                .min_by_key(|c| c.l)
            {
                Some(c) => c.clone(),
                None => continue,
            };
            let f = g5_propt_census::field::Fq::new(c.n);
            let (_ne, r, _) = g5_propt_census::curve::curve_summary(&f, c.a);
            let nx = ((1u64 << c.l) as f64 * (r as f64 - 1.0) / (2.0 * f.q as f64)).round();
            let jn = ((c.n - 1) / 2) as f64;
            let cc = (1u64 << (c.m - 1)) as f64;
            let u = g5_propt_census::census::binom(nx as usize, c.m) * cc * (cc - 1.0) * jn
                / ((r as f64 - 3.0) / 2.0);
            let b = c.reps; // reps per half, as run
            let p1 = oc_probs(u, b, 1.0, 60, 1);
            let p2 = oc_probs(u, b, 1.15, 60, 2);
            let p3 = oc_probs(u, b, 0.87, 60, 3);
            println!("OC: {} {} {} {} {} {:.1} (bases/half used {}) | {:.2} {:.2} {:.2} {:.2} {:.2} | {:.2} {:.2} {:.2} {:.2} {:.2} | {:.2}", c.a, c.n, c.m, c.l, c.reps, u, b, p1[0], p1[1], p1[2], p1[3], p1[4], p2[0], p2[1], p2[2], p2[3], p2[4], p3[2]);
        }
    }
    // sanity: a large effect is always called, no effect with plenty of counts is called equivalent
    let big = oc_probs(2000.0, 20, 1.0, 100, 11);
    assert!(big[0] + big[4] > 0.97 && big[0] > 0.8, "{big:?}");
    let ex = oc_probs(2000.0, 20, 1.4, 100, 12);
    assert!(ex[1] > 0.85, "{ex:?}");
    let low = oc_probs(1.0, 8, 1.0, 100, 13);
    assert!(
        low[0] < 0.2,
        "a starved cell must not be called equivalent: {low:?}"
    );
}

/// The analyzer reconciles planned versus present jobs, survives a ragged last line and reports unfinished jobs.
#[test]
fn reconciliation_flags_missing_short_and_ragged_jobs() {
    let dir = format!("{}/g5_recon", env!("CARGO_TARGET_TMPDIR"));
    let _ = std::fs::remove_dir_all(&dir);
    let o = RunOpts {
        seed: 9,
        reps: Some(2),
        out: dir.clone(),
    };
    run_propt(11, 3, 120, 2, &o).unwrap();
    let cell = Cell {
        a: 1,
        n: 11,
        m: 3,
        l: 6,
        reps: 2,
    };
    run_overlap(&cell, &o).unwrap();
    run_rank(&cell, 6, 30, &o).unwrap();
    std::fs::write(
        format!("{dir}/jobs.txt"),
        "propt 0 11 3 0 120\noverlap 1 11 3 6 2\nrank 1 11 3 6 2\n",
    )
    .unwrap();
    for stem in [
        "propt_a0_n11_m3_l0",
        "overlap_a1_n11_m3_l6",
        "rank_a1_n11_m3_l6",
    ] {
        std::fs::write(format!("{dir}/exit_{stem}.txt"), "0\n").unwrap();
    }
    assert_eq!(g5_propt_census::analyze::propt_gate(&dir).unwrap(), 3);
    g5_propt_census::analyze::analyze(&dir).unwrap();
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    assert!(md.contains("Jobs planned 3, complete 3"), "{md}");
    // a missing exit code and a truncated (ragged) rank file are reported, not panicked on
    std::fs::remove_file(format!("{dir}/exit_overlap_a1_n11_m3_l6.txt")).unwrap();
    let rp = format!("{dir}/rank_a1_n11_m3_l6.tsv");
    let mut txt = std::fs::read_to_string(&rp).unwrap();
    txt.push_str("1\t11\t3\n");
    std::fs::write(&rp, txt).unwrap();
    g5_propt_census::analyze::analyze(&dir).unwrap();
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    assert!(md.contains("NOT_FINISHED"), "{md}");
    assert!(md.contains("SHORT_OR_RAGGED"), "{md}");
    assert!(md.contains("INCOMPLETE"), "{md}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// PROTOCOL 8.5: the trend rule counts identical class sequences, including E-OVERLAP / E-EXTRA / E-DEFICIT ones
/// (which contain '-'), and never counts a missing-cell placeholder or an empty sequence.
#[test]
fn trend_rule_counts_identical_sequences() {
    use g5_propt_census::analyze::{classify, trend_agreement, Ci};
    let rep = |s: &str| vec![s.to_string(); 4];
    let ov = "E-OVERLAP,E-OVERLAP,E-OVERLAP,E-OVERLAP,E-OVERLAP,E-OVERLAP,E-OVERLAP";
    assert_eq!(trend_agreement(&rep(ov)), 4);
    assert_eq!(trend_agreement(&rep("E-DEFICIT,E-DEFICIT")), 4);
    assert_eq!(trend_agreement(&rep("UNRESOLVED,UNRESOLVED")), 4);
    assert_eq!(trend_agreement(&rep("")), 0);
    assert_eq!(trend_agreement(&rep("E-OVERLAP,-")), 0);
    let mixed = vec![
        ov.to_string(),
        ov.to_string(),
        ov.to_string(),
        "E-EXTRA".to_string(),
    ];
    assert_eq!(trend_agreement(&mixed), 3);
    let two = vec![
        ov.to_string(),
        ov.to_string(),
        "E-EXTRA".to_string(),
        "".to_string(),
    ];
    assert_eq!(trend_agreement(&two), 2);
    let ci = |lo90: f64, hi90: f64, lo95: f64, hi95: f64| Ci {
        pt: 1.0,
        lo95,
        hi95,
        lo90,
        hi90,
    };
    assert_eq!(classify(&ci(0.95, 1.05, 0.94, 1.06), 0.10), "E-OVERLAP");
    assert_eq!(classify(&ci(1.3, 1.5, 1.25, 1.55), 0.10), "E-EXTRA");
    assert_eq!(classify(&ci(0.5, 0.7, 0.45, 0.75), 0.10), "E-DEFICIT");
    assert_eq!(classify(&ci(0.7, 1.4, 0.6, 1.5), 0.10), "UNRESOLVED");
}

#[test]
fn driver_is_job_safe_gate_free_and_uploads_its_evidence() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let src = std::fs::read_to_string(format!("{dir}/run_cells.sh")).unwrap();
    let code: String = src
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !code.contains("scripts/checkpoint.sh gate )")
            && !code.contains("&& scripts/checkpoint.sh"),
        "the session-side gate must not be invoked inside the job"
    );
    assert!(
        code.contains("G5_CHECKPOINT_STAMP") && code.contains("checkpoint_stamp"),
        "stamp required and recorded"
    );
    assert!(
        code.contains("trap finish EXIT"),
        "evidence is archived and uploaded on every exit"
    );
    assert!(
        code.contains("aws s3 cp")
            && code.contains("ISOLAB_RESULTS_URI")
            && code.contains("sha256sum")
    );
    assert!(
        code.contains("exit 90"),
        "an upload failure is never swallowed"
    );
    let stamp_at = code.find("G5_CHECKPOINT_STAMP").unwrap();
    assert!(
        stamp_at < code.find("cargo test").unwrap(),
        "the stamp check precedes any build or test"
    );
    // behavioural: without the stamp the driver exits 3 before touching anything (no build, no run directory)
    let out = std::process::Command::new("bash")
        .arg(format!("{dir}/run_cells.sh"))
        .env_remove("G5_CHECKPOINT_STAMP")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&out.stderr).contains("G5_CHECKPOINT_STAMP"));
    // the run card carries an argv array and the .git / toolchain requirements
    let card = std::fs::read_to_string(format!("{dir}/RUN_CARD.md")).unwrap();
    assert!(card.contains("\"/bin/bash\"") && card.contains(".git") && card.contains(">= 1.78"));
    assert!(!card.contains("must pass inside the job"));
}
