//! Correctness fixtures for the encodings: tiny exhaustive/planted instances
//! against the solver-free oracle. These produce no results; they check that
//! the instrument is sound before any census is run.

use crate::curve::*;
use crate::enc::*;
use crate::gf::*;
use crate::satlib::SolveResult;

fn solve(cx: &Cell, arm: Arm, xr: u64, seed: u64) -> (SolveResult, Result<Vec<u64>, String>) {
    let b = build(arm, cx, xr, true);
    let (mut s, pi) = b.load(seed);
    s.conflict_budget = 5_000_000;
    let r = s.solve();
    let v = if r == SolveResult::Sat {
        b.verify(cx, &s.model(), &pi, xr)
    } else {
        Err("n/a".into())
    };
    (r, v)
}

fn oracle(cx: &Cell) -> Oracle<'_> {
    Oracle::new(&cx.f, &cx.nb, &cx.vb_n, cx.g, cx.b)
}

#[test]
fn sound_arms_agree_with_oracle_exhaustively_n7_m2() {
    for l in [3u32, 4] {
        let cx = Cell::new(7, l, 2, 0, 1);
        let mut o = oracle(&cx);
        let mut u_flips = 0;
        let mut n_sat = 0;
        for xr in 1..128u64 {
            let truth = o.exists(2, xr);
            n_sat += truth as usize;
            for (k, arm) in [
                Arm::C,
                Arm::CPoly,
                Arm::CNoBreak,
                Arm::A,
                Arm::B,
                Arm::BNoGauge,
                Arm::G,
            ]
            .into_iter()
            .enumerate()
            {
                let (r, v) = solve(&cx, arm, xr, if k % 2 == 0 { 0 } else { 1000 + xr });
                assert_eq!(
                    r == SolveResult::Sat,
                    truth,
                    "l={l} xr={xr} arm={}",
                    arm.name()
                );
                assert_ne!(r, SolveResult::Unknown);
                if truth {
                    assert!(
                        v.is_ok(),
                        "certificate l={l} xr={xr} arm={}: {:?}",
                        arm.name(),
                        v
                    );
                }
            }
            // the literal per-instance orbit clauses (U) can only lose solutions
            let (r, _) = solve(&cx, Arm::U, xr, 0);
            if r == SolveResult::Sat {
                assert!(truth, "U must never invent solutions");
            } else if truth {
                u_flips += 1;
            }
        }
        assert!(
            n_sat > 5,
            "need SAT targets to test soundness (l={l}, got {n_sat})"
        );
        assert!(
            u_flips > 0,
            "tripwire must change satisfiability on some SAT target (l={l})"
        );
    }
}

#[test]
fn sound_arms_agree_with_oracle_n7_m3() {
    let cx = Cell::new(7, 3, 3, 0, 1);
    let mut o = oracle(&cx);
    let mut n_sat = 0;
    for xr in 1..128u64 {
        let truth = o.exists(3, xr);
        n_sat += truth as usize;
        for arm in [
            Arm::C,
            Arm::CPoly,
            Arm::CNoBreak,
            Arm::A,
            Arm::B,
            Arm::BNoGauge,
            Arm::G,
        ] {
            let (r, v) = solve(&cx, arm, xr, 7 + xr);
            assert_eq!(
                r == SolveResult::Sat,
                truth,
                "m3 xr={xr} arm={}",
                arm.name()
            );
            if truth {
                assert!(
                    v.is_ok(),
                    "m3 certificate xr={xr} arm={}: {:?}",
                    arm.name(),
                    v
                );
            }
        }
    }
    assert!(n_sat > 0 && n_sat < 127, "n_sat={n_sat}");
}

#[test]
fn ordinary_curve_unsoundness_is_detected() {
    // b = t (not in F_2): the orbit system's equivariance fails, so the gauge-fixed
    // orbit encoding must disagree with the plain truth somewhere (detector works);
    // the representative-and-shift reparameterisation stays a bijection and sound.
    let cx = Cell::new(7, 3, 2, 0, 2);
    let mut o = oracle(&cx);
    let mut b_bad = 0;
    for xr in 1..128u64 {
        let truth = o.exists(2, xr);
        for arm in [Arm::C, Arm::A] {
            let (r, v) = solve(&cx, arm, xr, 0);
            assert_eq!(
                r == SolveResult::Sat,
                truth,
                "sound arm {} on ordinary curve",
                arm.name()
            );
            if truth {
                assert!(v.is_ok());
            }
        }
        let (r, v) = solve(&cx, Arm::B, xr, 0);
        if (r == SolveResult::Sat) != truth || (r == SolveResult::Sat && v.is_err()) {
            b_bad += 1;
        }
    }
    assert!(
        b_bad > 0,
        "B on an ordinary curve should be caught as unsound"
    );
}

#[test]
fn j0_embedding_equals_plain() {
    // B0(i): the orbit system with the target index pinned to j=0 and no gauge is the
    // plain single-instance problem (same verdict on every target).
    let cx = Cell::new(7, 4, 2, 0, 1);
    let mut o = oracle(&cx);
    for xr in 1..128u64 {
        let truth = o.exists(2, xr);
        let mut b = build(Arm::BNoGauge, &cx, xr, true);
        if let Decode::Orbit { h, .. } = &b.dec {
            let h = h.clone();
            b.cnf.clause(vec![h[0] as i32]);
        }
        let (mut s, _pi) = b.load(0);
        assert_eq!(s.solve() == SolveResult::Sat, truth, "xr={xr}");
    }
}

#[test]
fn variable_renaming_does_not_change_verdicts() {
    let cx = Cell::new(7, 4, 2, 0, 1);
    let mut o = oracle(&cx);
    for xr in [3u64, 5, 9, 33, 100, 127] {
        let truth = o.exists(2, xr);
        for seed in [0u64, 1, 2, 3, 12345] {
            for arm in [Arm::C, Arm::A, Arm::B] {
                let (r, v) = solve(&cx, arm, xr, seed);
                assert_eq!(r == SolveResult::Sat, truth);
                if truth {
                    assert!(v.is_ok());
                }
            }
        }
    }
}

fn lexkey(c: u64, n: u32) -> u64 {
    (0..n).fold(0, |a, i| a | (((c >> i) & 1) << (n - 1 - i)))
}

fn count_models(b: &Built, proj: &[u32]) -> usize {
    let (mut s, _) = b.load(0);
    let mut cnt = 0;
    loop {
        if s.solve() != SolveResult::Sat {
            break;
        }
        let m = s.model();
        cnt += 1;
        let blk: Vec<i32> = proj
            .iter()
            .map(|&v| {
                if m[(v - 1) as usize] {
                    -(v as i32)
                } else {
                    v as i32
                }
            })
            .collect();
        s.reset_search();
        if !s.add_clause(blk) {
            break;
        }
    }
    cnt
}

fn binom(n: usize, k: usize) -> usize {
    (0..k).fold(1usize, |a, i| a * (n - i) / (i + 1))
}

fn tuples(len: usize, m: usize) -> Vec<Vec<usize>> {
    let mut out = vec![vec![]];
    for _ in 0..m {
        out = out
            .into_iter()
            .flat_map(|t| {
                (0..len).map(move |i| {
                    let mut t = t.clone();
                    t.push(i);
                    t
                })
            })
            .collect();
    }
    out
}

#[test]
fn counting_identity_of_the_three_domains_n7() {
    for (l, m) in [(3u32, 2usize), (4, 2), (3, 3)] {
        let cx = Cell::new(7, l, m, 0, 1);
        let n = cx.n;
        let nv_elems: Vec<u64> = Oracle::new(&cx.f, &cx.nb, &cx.vb_n, cx.g, 1).velems;
        let size_v = nv_elems.len();
        // (C) multisets of V
        let c = build(Arm::C, &cx, 5, false);
        let proj: Vec<u32> = match &c.dec {
            Decode::Plain { u, .. } => u.iter().flatten().copied().collect(),
            _ => unreachable!(),
        };
        assert_eq!(
            count_models(&c, &proj),
            binom(size_v + m - 1, m),
            "C domain l={l} m={m}"
        );
        // (A) predicate: necklace-minimal rho (fixed => s=0), non-decreasing rho
        let coords: Vec<u64> = nv_elems.iter().map(|&x| cx.nb.coords(x)).collect();
        let mut slots: Vec<(u64, usize)> = vec![]; // (rho coords, s)
        for &c0 in &coords {
            let minrot = (0..n).map(|k| lexkey(rot(c0, k, n), n)).min().unwrap();
            if lexkey(c0, n) != minrot {
                continue;
            }
            let fixed = (0..n).all(|k| rot(c0, k, n) == c0);
            for s in 0..n as usize {
                if fixed && s != 0 {
                    continue;
                }
                slots.push((c0, s));
            }
        }
        let want_a = tuples(slots.len(), m)
            .iter()
            .filter(|t| {
                t.windows(2)
                    .all(|w| lexkey(slots[w[0]].0, n) <= lexkey(slots[w[1]].0, n))
            })
            .count();
        let a = build(Arm::A, &cx, 5, false);
        let proj: Vec<u32> = match &a.dec {
            Decode::RepShift { r, h } => r.iter().chain(h.iter()).flatten().copied().collect(),
            _ => unreachable!(),
        };
        assert_eq!(count_models(&a, &proj), want_a, "A domain l={l} m={m}");
        assert!(
            want_a >= binom(size_v + m - 1, m),
            "A must cover every multiset"
        );
        // (B) predicate: x_1 is the minimal rotation of every slot, x_2 <= x_3; j free
        let want_b = tuples(size_v, m)
            .iter()
            .filter(|t| {
                let x: Vec<u64> = t.iter().map(|&i| coords[i]).collect();
                let k1 = lexkey(x[0], n);
                let ok1 = (0..m).all(|i| {
                    (0..n).all(|k| (i == 0 && k == 0) || k1 <= lexkey(rot(x[i], k, n), n))
                });
                let ok2 = (1..m.saturating_sub(1)).all(|i| lexkey(x[i], n) <= lexkey(x[i + 1], n));
                ok1 && ok2
            })
            .count()
            * n as usize;
        let b = build(Arm::B, &cx, 5, false);
        let proj: Vec<u32> = match &b.dec {
            Decode::Orbit { u, h } => u.iter().flatten().chain(h.iter()).copied().collect(),
            _ => unreachable!(),
        };
        assert_eq!(count_models(&b, &proj), want_b, "B domain l={l} m={m}");
        // every C_n x S_m class has at least one canonical representative with all n targets
        assert!(want_b >= (binom(size_v + m - 1, m) / n as usize) * n as usize / 2);
    }
}

#[test]
fn structure_counts() {
    for (n, l, m) in [(7u32, 3u32, 2usize), (7, 3, 3)] {
        let cx = Cell::new(n, l, m, 0, 1);
        let nn = n as usize;
        let c = build(Arm::C, &cx, 5, true);
        let a = build(Arm::A, &cx, 5, true);
        let b = build(Arm::B, &cx, 5, true);
        let g = build(Arm::G, &cx, 5, true);
        let u = build(Arm::U, &cx, 5, true);
        let gauge_cmp = m * nn - 1 + (m - 2);
        assert_eq!(c.n_comparators, m - 1);
        assert_eq!(a.n_comparators, m * (nn - 1) + (m - 1));
        assert_eq!(b.n_comparators, gauge_cmp);
        assert_eq!(g.n_comparators, (m - 1) + gauge_cmp);
        assert_eq!(u.n_comparators, g.n_comparators);
        // size-matched control: same comparator count, same clause literals up to the selectors
        assert_eq!(g.cnf.sizes().xor_rows, u.cnf.sizes().xor_rows);
        assert_eq!(g.cnf.sizes().clauses, u.cnf.sizes().clauses);
        assert_eq!(g.cnf.nv, u.cnf.nv + gauge_cmp as u32); // one free selector per gauge comparator
        assert!(a.cnf.nv > c.cnf.nv && b.cnf.nv > c.cnf.nv);
        // priority sets
        assert_eq!(c.priority.len(), m * l as usize);
        assert_eq!(a.priority.len(), m * (l as usize + nn));
        assert_eq!(b.priority.len(), m * l as usize + nn);
    }
}

#[test]
fn n17_cell_agrees_with_oracle_and_targets_are_sound() {
    let cx = Cell::new(17, 8, 2, 0, 1);
    let mut o = oracle(&cx);
    let targets = gen_targets_small(&cx, &mut o);
    assert!(targets.iter().any(|t| t.truth_sat) && targets.iter().any(|t| !t.truth_sat));
    for t in &targets {
        for (k, arm) in [Arm::C, Arm::CPoly, Arm::A, Arm::B, Arm::G]
            .into_iter()
            .enumerate()
        {
            let (r, v) = solve(&cx, arm, t.xr, 31 * k as u64);
            assert_ne!(r, SolveResult::Unknown);
            assert_eq!(
                r == SolveResult::Sat,
                t.truth_sat,
                "n=17 xr={} arm={} kind={}",
                t.xr,
                arm.name(),
                t.kind
            );
            if t.truth_sat {
                assert!(v.is_ok(), "{:?}", v);
            }
        }
    }
}

fn gen_targets_small(cx: &Cell, o: &mut Oracle) -> Vec<crate::run::Target> {
    let t = crate::run::gen_targets(cx, o, 0, 2, 2, 2, 4242);
    // distinct sigma-orbits
    let mut keys: Vec<u64> = t
        .iter()
        .map(|x| {
            (0..cx.n)
                .map(|k| rot(cx.nb.coords(x.xr), k, cx.n))
                .min()
                .unwrap()
        })
        .collect();
    keys.sort();
    keys.dedup();
    assert_eq!(keys.len(), t.len());
    assert!(
        t.iter().filter(|x| x.kind == "planted").count() == 2
            && t.iter()
                .filter(|x| x.kind == "planted")
                .all(|x| x.truth_sat)
    );
    t
}

#[test]
fn equivariance_of_conflict_counts_is_exact_for_plain_arm_n7() {
    // On a stable V, xR and sigma(xR) are the same problem; verdicts must match for every arm.
    let cx = Cell::new(7, 4, 2, 0, 1);
    let mut o = oracle(&cx);
    for xr in 1..128u64 {
        assert_eq!(o.exists(2, xr), o.exists(2, cx.f.sq(xr)));
    }
}

/// Build-only smoke test (no solve): every census cell and arm encodes without
/// overflow or panic, and the sizes are within the solver's comfortable range.
#[test]
fn all_census_cells_build_all_arms() {
    let cells = [
        (17u32, 8u32, 2usize),
        (17, 9, 2),
        (23, 11, 2),
        (23, 12, 2),
        (31, 15, 2),
        (31, 16, 2),
        (31, 10, 3),
        (31, 11, 3),
    ];
    for (n, l, m) in cells {
        let cx = Cell::new(n, l, m, 0, 1);
        assert!(m as u32 * (l - 1) <= n, "admissible cell m(l-1)<=n");
        let xr = 0x1234567 & ((1u64 << n) - 1) | 1;
        for arm in [
            Arm::C,
            Arm::CPoly,
            Arm::CNoBreak,
            Arm::A,
            Arm::B,
            Arm::BNoGauge,
            Arm::G,
            Arm::U,
            Arm::CHalf,
        ] {
            let t = std::time::Instant::now();
            let b = build(arm, &cx, xr, true);
            let s = b.cnf.sizes();
            assert!(!b.cnf.root_unsat);
            assert!(s.vars < 200_000 && s.xor_rows > 0);
            if std::env::var("G3_PRINT_SIZES").is_ok() {
                eprintln!("SIZES n={n} l={l} m={m} arm={:8} vars={:6} clauses={:7} clause_lits={:8} xor_rows={:5} xor_lits={:8} and={:6} cmp={:4} build_ms={}", arm.name(), s.vars, s.clauses, s.clause_lits, s.xor_rows, s.xor_lits, s.and_aux, b.n_comparators, t.elapsed().as_millis());
            }
        }
    }
}

#[test]
fn runner_rows_parse_and_columns_align() {
    use crate::run::*;
    let cx = Cell::new(7, 4, 2, 0, 1);
    let mut o = oracle(&cx);
    let targets = gen_targets(&cx, &mut o, 0, 2, 1, 1, 777);
    let params = RunParams {
        round: 1,
        conflict_cap: 1_000_000,
        deadline_s: 60,
    };
    let mut text = format!("{HEADER}\n");
    let toks = parse_arms("C,C@2,A,B,G,U,Cnobreak").unwrap();
    for (ti, t) in targets.iter().enumerate() {
        for &tok in &toks {
            text.push_str(&run_one(&cx, 0, 0, t, ti, tok, &params));
            text.push('\n');
        }
    }
    let rows = crate::stats::parse_rows(&text);
    assert_eq!(rows.len(), targets.len() * toks.len());
    for r in &rows {
        assert_eq!(r.n, 7);
        assert_eq!(r.l, 4);
        assert!(r.conflicts >= 0.0 && r.total_ops > 0.0 && r.total_ops >= r.search_ops);
        if r.arm != "U" {
            assert!(r.verdict_ok, "sound arm {} disagreed with truth", r.arm);
        }
        assert!(matches!(r.result.as_str(), "SAT" | "UNSAT"));
    }
    assert!(rows
        .iter()
        .any(|r| r.kind == "planted" && r.result == "SAT" && r.cert_ok == "1"));
    // arms that share an instance but differ by seed slot are distinct runs of the same encoding
    let c1 = rows
        .iter()
        .find(|r| r.arm == "C" && r.slot == 1 && r.tidx == 0)
        .unwrap();
    let c2 = rows
        .iter()
        .find(|r| r.arm == "C" && r.slot == 2 && r.tidx == 0)
        .unwrap();
    assert_eq!(c1.result, c2.result);
    // bad arm token
    assert!(parse_arms("C,Zzz").is_err());
}

#[test]
fn cli_run_summarize_gate_end_to_end_on_a_toy_cell() {
    // Exercises the exact CLI path of RUN_CARD on n = 7 (fixture only; output lives under target/).
    // temp dir outside the source tree: the freeze requires an empty `git status`
    let dir = format!(
        "{}/g3census-test-cli-run-{}",
        std::env::temp_dir().display(),
        std::process::id()
    );
    let _ = std::fs::remove_dir_all(&dir);
    let s = |x: &str| x.to_string();
    let args: Vec<String> = [
        "run",
        "--n",
        "7",
        "--l",
        "4",
        "--m",
        "2",
        "--curve-a",
        "0",
        "--unsat",
        "3",
        "--sat-keep",
        "0",
        "--planted",
        "1",
        "--arms",
        "C,C@2,A,B",
        "--planted-only-arms",
        "U",
        "--round",
        "1",
        "--tag",
        "t",
        "--out",
        &dir,
    ]
    .iter()
    .map(|x| s(x))
    .collect();
    assert_eq!(crate::run::cli(&args), 0);
    let f = format!("{dir}/rows_n7_l4_m2_v0_b1_a0_r1_t.tsv");
    let text = std::fs::read_to_string(&f).unwrap();
    assert!(text.starts_with("cell\t"));
    let rows = crate::stats::parse_rows(&text);
    assert!(rows.len() >= 3 * 4 + 5, "{}", rows.len());
    assert!(rows
        .iter()
        .filter(|r| r.arm == "U")
        .all(|r| r.kind == "planted"));
    assert_eq!(
        crate::run::cli(&args),
        3,
        "must refuse to overwrite an existing rows file"
    );
    assert_eq!(crate::run::cli(&["gate".to_string(), dir.clone()]), 0);
    assert_eq!(
        crate::run::cli(&[
            "summarize".to_string(),
            dir.clone(),
            "--out".to_string(),
            format!("{dir}/out")
        ]),
        0
    );
    assert!(std::path::Path::new(&format!("{dir}/out/SUMMARY.md")).exists());
    assert_eq!(crate::run::cli(&["bogus".to_string()]), 2);
}

/// Pin the u-bits of the plain slots to the given V-elements (as masks over vb_n).
fn pinned(arm: Arm, cx: &Cell, xr: u64, masks: &[u64], xx: u64) -> SolveResult {
    let mut b = build(arm, cx, xr, true);
    let us = match &b.dec {
        Decode::Plain { u, .. } => u.clone(),
        _ => unreachable!(),
    };
    for (slot, &mk) in us.iter().zip(masks) {
        for (p, &v) in slot.iter().enumerate() {
            let lit = if (mk >> p) & 1 == 1 {
                v as i32
            } else {
                -(v as i32)
            };
            b.cnf.clause(vec![lit]);
        }
    }
    // pin the chain variable too (normal-basis coordinates of xx): pure propagation
    let co = cx.nb.coords(xx);
    for (k, &v) in b.xchain.iter().enumerate() {
        b.cnf.clause(vec![if (co >> k) & 1 == 1 {
            v as i32
        } else {
            -(v as i32)
        }]);
    }
    let (mut s, _) = b.load(0);
    s.conflict_budget = 1_000_000;
    s.solve()
}

fn elem(cx: &Cell, mk: u64) -> u64 {
    (0..cx.l as usize).fold(0u64, |a, p| {
        if (mk >> p) & 1 == 1 {
            a ^ cx.vb_n[p]
        } else {
            a
        }
    })
}

/// Defect 1 of the review: G (and C) must accept EVERY unordered V-triple that
/// solves the chain, in C's own u-lex order. Pinning the slots makes this an
/// exact admissibility check with no solution multiplicity to hide behind.
fn check_g_admits_all_sorted_triples(cx: &Cell, triples: &[(u64, u64, u64)]) {
    let f = &cx.f;
    let u_key = |mk: u64| (0..cx.l).fold(0u64, |a, p| a | (((mk >> p) & 1) << (cx.l - 1 - p)));
    let mut checked = 0;
    for &(a, b, c) in triples {
        let mut t = [a, b, c];
        t.sort_by_key(|&mk| u_key(mk));
        let (x0, x1, x2) = (elem(cx, t[0]), elem(cx, t[1]), elem(cx, t[2]));
        let (bb, cc) = (f.mul(x0, x1), f.sq(x0 ^ x1));
        for xx in f
            .solve_quadratic(cc, bb, f.sq(bb) ^ cx.b)
            .unwrap_or_default()
        {
            let (b2, c2) = (f.mul(xx, x2), f.sq(xx ^ x2));
            for xr in f
                .solve_quadratic(c2, b2, f.sq(b2) ^ cx.b)
                .unwrap_or_default()
            {
                if xr == 0 {
                    continue;
                }
                assert_eq!(
                    pinned(Arm::C, cx, xr, &t, xx),
                    SolveResult::Sat,
                    "C rejects its own planted triple"
                );
                assert_eq!(
                    pinned(Arm::G, cx, xr, &t, xx),
                    SolveResult::Sat,
                    "G rejects planted triple {t:?} xr={xr}"
                );
                // and the verdicts agree on a wrong target too
                let wrong = xr ^ 1;
                if wrong != 0 {
                    assert_eq!(
                        pinned(Arm::G, cx, wrong, &t, xx),
                        pinned(Arm::C, cx, wrong, &t, xx)
                    );
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 20, "too few planted checks: {checked}");
}

#[test]
fn g_is_sound_on_every_single_triple_m3_n7() {
    let cx = Cell::new(7, 3, 3, 0, 1);
    let sz = 1u64 << cx.l;
    let mut tr = vec![];
    for a in 0..sz {
        for b in a..sz {
            for c in b..sz {
                tr.push((a, b, c));
            }
        }
    }
    check_g_admits_all_sorted_triples(&cx, &tr);
}

#[test]
fn g_is_sound_on_sampled_triples_m3_n31() {
    let cx = Cell::new(31, 5, 3, 0, 1);
    let mut rng = Rng(0x9e3779b97f4a7c15);
    let tr: Vec<(u64, u64, u64)> = (0..120)
        .map(|_| (rng.below(32), rng.below(32), rng.below(32)))
        .collect();
    check_g_admits_all_sorted_triples(&cx, &tr);
}

/// P3 control Chalf: loses solutions by design but must never invent one, must agree with C on
/// UNSAT targets, and its pin is exactly one unit clause per slot (top u-bit of every slot = 0).
#[test]
fn chalf_never_invents_and_matches_c_on_unsat_targets() {
    for (l, m) in [(3u32, 2usize), (4, 2), (3, 3)] {
        let cx = Cell::new(7, l, m, 0, 1);
        let mut o = oracle(&cx);
        let (mut unsat_seen, mut lost) = (0, 0);
        for xr in 1..128u64 {
            let truth = o.exists(m, xr);
            let (r, _) = solve(&cx, Arm::CHalf, xr, 3);
            if r == SolveResult::Sat {
                assert!(truth, "Chalf invented a solution (l={l} m={m} xr={xr})");
            } else if truth {
                lost += 1;
            } else {
                unsat_seen += 1;
                assert_eq!(r, SolveResult::Unsat);
            }
        }
        assert!(
            unsat_seen > 5 && (lost > 0 || m == 3),
            "l={l} m={m}: unsat {unsat_seen} lost {lost}"
        );
        let c = build(Arm::C, &cx, 5, true);
        let h = build(Arm::CHalf, &cx, 5, true);
        assert_eq!(h.cnf.sizes().clauses, c.cnf.sizes().clauses + m);
        assert_eq!(h.cnf.nv, c.cnf.nv);
    }
}
