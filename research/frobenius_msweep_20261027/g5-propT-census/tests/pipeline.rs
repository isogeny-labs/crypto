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
        assert!(cells.iter().any(|c| c.a == 1 && c.n == 17 && c.l == 8 && c.m == m), "stable cell m={m}");
    }
    for &(a, n) in &[(1u32, 11u32), (0, 13), (1, 17), (0, 19)] {
        for m in 2..=8usize {
            assert!(cells.iter().any(|c| c.a == a && c.n == n && c.m == m), "missing primary cell a={a} n={n} m={m}");
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
        println!("{} {} {} {} {} {} {:.2} {:.2e} {:.2e} {}", c.a, c.n, c.m, c.l, c.reps, nx, nu_exp(nx, c.m, r), ops, cell_ops, tn);
        total += 1.0;
    }
    println!("cells: {total} total est ops {ops_all:.3e}");
}

#[test]
fn drivers_write_consistent_schemas() {
    let dir = format!("{}/g5_smoke", env!("CARGO_TARGET_TMPDIR"));
    let o = RunOpts { seed: 7, reps: Some(2), out: dir.clone() };
    run_propt(11, 3, 120, 2, &o).unwrap();
    let cell = Cell { a: 1, n: 11, m: 3, l: 6, reps: 2 };
    run_overlap(&cell, &o).unwrap();
    run_rank(&cell, 6, 30, &o).unwrap();
    for f in ["propt_n11_m3.tsv", "overlap_a1_n11_m3_l6.tsv", "rank_a1_n11_m3_l6.tsv"] {
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
    assert!(md.contains("Voids and failures"));
    // exact identities must hold on any input: no void may mention Proposition T, halving, chain or closure
    assert!(!md.contains("INSTRUMENT_FAIL"), "{md}");
    assert!(!md.contains("PROP_T_FALSIFIED"), "{md}");
    let _ = std::fs::remove_dir_all(&dir);
}
