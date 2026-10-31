use g6_large_m::analyze;
use g6_large_m::cell::{run_cell, CellParams, PLANTS};

fn tmpdir(tag: &str) -> String {
    let d = format!(
        "{}/g6test_{}_{}",
        std::env::temp_dir().display(),
        tag,
        std::process::id()
    );
    let _ = std::fs::remove_dir_all(&d);
    d
}

#[test]
fn tiny_cell_runs_end_to_end_and_the_instrument_checks_hold() {
    let out = tmpdir("a");
    let p = CellParams {
        n: 11,
        a: 0,
        m: 5,
        draws: 200,
        k: 10,
        seed: 42,
        out: out.clone(),
    };
    let msg = run_cell(&p).unwrap();
    assert!(msg.contains("OK") || msg.contains("UNMATCHABLE"), "{msg}");
    let rows =
        analyze::parse_rows(&std::fs::read_to_string(format!("{out}/rows_n11a0m5.tsv")).unwrap());
    for r in &rows {
        assert!(r.conserved, "conservation failed for {} {}", r.arm, r.idx);
        assert_eq!(r.planted, PLANTS);
    }
    let _ = analyze::analyze_dir(&out).unwrap();
    assert!(std::path::Path::new(&format!("{out}/summary.md")).exists());
    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn m9_at_n11_exercises_the_many_dim1_slots_regime() {
    let out = tmpdir("b");
    let p = CellParams {
        n: 11,
        a: 1,
        m: 9,
        draws: 150,
        k: 8,
        seed: 7,
        out: out.clone(),
    };
    let msg = run_cell(&p).unwrap();
    assert!(msg.contains("OK") || msg.contains("UNMATCHABLE"), "{msg}");
    let meta = std::fs::read_to_string(format!("{out}/meta_n11a1m9.txt")).unwrap();
    assert!(meta.contains("feasible_NI"));
    let sizes = std::fs::read_to_string(format!("{out}/sizes_n11a1m9.tsv")).unwrap();
    assert!(sizes.lines().count() > 300);
    let _ = analyze::analyze_dir(&out).unwrap();
    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn m_ge_n_is_refused() {
    let p = CellParams {
        n: 11,
        a: 0,
        m: 11,
        draws: 10,
        k: 8,
        seed: 1,
        out: tmpdir("c"),
    };
    assert!(run_cell(&p).is_err());
}

#[test]
fn ok_path_is_exercised_with_all_arms_present_on_a_tiny_fixture() {
    // tiny fixture only (n = 13, m = 4): checks the plumbing, row accounting and the controls' presence; no result is read from it
    let out = tmpdir("d");
    let p = CellParams {
        n: 13,
        a: 0,
        m: 4,
        draws: 400,
        k: 10,
        seed: 3,
        out: out.clone(),
    };
    let msg = run_cell(&p).unwrap();
    assert!(msg.contains(" OK "), "{msg}");
    let rows =
        analyze::parse_rows(&std::fs::read_to_string(format!("{out}/rows_n13a0m4.tsv")).unwrap());
    for arm in ["NI", "UP", "UP2", "DUP"] {
        assert!(rows.iter().any(|r| r.arm == arm), "arm {arm} missing");
    }
    // NI and UP rows are paired on idx and have identical tuple counts (matched factor-base size by construction)
    let mut paired = 0;
    for r in rows.iter().filter(|r| r.arm == "NI") {
        if let Some(u) = rows.iter().find(|u| u.arm == "UP" && u.idx == r.idx) {
            assert!(
                (u.l2mu - r.l2mu).abs() < 1e-9,
                "profile mismatch at idx {}",
                r.idx
            );
            paired += 1;
        }
    }
    assert!(paired >= 8);
    let meta = std::fs::read_to_string(format!("{out}/meta_n13a0m4.txt")).unwrap();
    let k_eff: usize = analyze::meta_get(&meta, "k_eff").unwrap().parse().unwrap();
    assert!(k_eff >= 8 && k_eff <= 10, "k_eff = {k_eff}");
    let res = analyze::analyze_dir(&out).unwrap();
    assert!(res.contains("n13a0m4"));
    let _ = std::fs::remove_dir_all(&out);
}
