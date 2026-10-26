//! Correctness tests on tiny hand-made fixtures: exact identities, planted
//! instances, detector self-tests on synthetic counts. No result is produced.

use g2_orbit_union_dnm::combi::*;
use g2_orbit_union_dnm::curve::*;
use g2_orbit_union_dnm::enumr::Enumerator;
use g2_orbit_union_dnm::family::{self, span_nonzero};
use g2_orbit_union_dnm::field::Fq;
use g2_orbit_union_dnm::plan::*;
use g2_orbit_union_dnm::runner::*;
use g2_orbit_union_dnm::stats::*;
use g2_orbit_union_dnm::Rng;

#[test]
fn field_and_curve_basics() {
    for &n in &[3u32, 5, 7, 11] {
        let f = Fq::new(n);
        for a in 0..=1 {
            let c = Curve::koblitz(a);
            let pts = c.all_points(&f);
            assert_eq!(pts.len() as u64 + 1, koblitz_order(a, n), "n={n} a={a}");
            for &p in &pts {
                assert!(c.on_curve(&f, p));
                assert!(c.on_curve(&f, c.sigma(&f, p)));
                assert_eq!(c.add(&f, p, c.neg(p)), INF);
            }
            // group law: associativity and Frobenius is a homomorphism on samples
            let mut rng = Rng(7);
            for _ in 0..50 {
                let (p, q, r) = (
                    pts[rng.below(pts.len() as u64) as usize],
                    pts[rng.below(pts.len() as u64) as usize],
                    pts[rng.below(pts.len() as u64) as usize],
                );
                assert_eq!(c.add(&f, c.add(&f, p, q), r), c.add(&f, p, c.add(&f, q, r)));
                assert_eq!(c.sigma(&f, c.add(&f, p, q)), c.add(&f, c.sigma(&f, p), c.sigma(&f, q)));
            }
            // scalar by the group order kills every point
            for &p in pts.iter().take(20) {
                assert_eq!(c.scalar(&f, p, koblitz_order(a, n)), INF);
            }
        }
    }
    assert_eq!(koblitz_order(0, 3), 4); // V_3 = 5 for mu = -1
    assert_eq!(koblitz_order(1, 3), 14); // V_3 = -5 for mu = +1
}

#[test]
fn class_count_values() {
    assert_eq!((2..=4).map(|m| d_nm(7, m)).collect::<Vec<_>>(), vec![4, 12, 30]);
    assert_eq!((3..=5).map(|m| d_nm(11, m)).collect::<Vec<_>>(), vec![26, 91, 273]);
    assert_eq!(d_nm(13, 4), 140);
    assert_eq!(d_nm(17, 2), 9);
    assert_eq!(
        [2u64, 3, 4, 5, 6].map(|m| d_nm(17, m)).to_vec(),
        vec![9, 57, 285, 1197, 4389]
    );
    assert_eq!([2u64, 3, 4, 5].map(|m| d_nm(131, m)).to_vec(), vec![66, 2926, 98021, 2646567]);
    assert!((baseline_ratio(131, 5) - 111.3).abs() < 0.1);
    assert!((baseline_ratio(17, 2) - 17.0 / 9.0).abs() < 1e-12);
    let d10 = d_nm(131, 10) as f64;
    assert!((d10 / 4.379e12 - 1.0).abs() < 1e-3, "{d10}");
}

#[test]
#[should_panic]
fn class_count_requires_free_action() {
    let _ = d_nm(6, 2); // 6 does not divide C(7,2)=21
}

#[test]
fn classes_match_formula_and_burnside() {
    for &(n, m) in &[(5usize, 2usize), (5, 3), (5, 4), (7, 2), (7, 3), (7, 4), (7, 5), (11, 3)] {
        let pt = PatternTable::new(n, m, true);
        assert_eq!(pt.pats.len() as u128, n_patterns(n as u64, m as u64));
        assert_eq!(pt.n_classes() as u128, d_nm(n as u64, m as u64), "n={n} m={m}");
        // every class has exactly n members (free action), they sum to N_m
        assert!(pt.class_pats.iter().all(|c| c.len() == n));
        assert_eq!(pt.class_pats.iter().map(|c| c.len()).sum::<usize>(), pt.pats.len());
        // Burnside by direct count: shift j != 0 fixes no multiset (n prime, m < n)
        for j in 1..n {
            let fixed = pt.pats.iter().filter(|p| shift_pattern(p, j, n) == **p).count();
            assert_eq!(fixed, 0, "n={n} m={m} j={j}");
        }
        // representative is canonical and a member of its class
        for (c, &r) in pt.rep.iter().enumerate() {
            assert_eq!(pt.class_of[r], c);
            assert_eq!(canon(&pt.pats[r], n), pt.pats[r]);
        }
    }
    // non-free case documented: n = 4, m = 2: the multiset {0,2} is fixed by shift 2
    let pt = PatternTable::new(4, 2, true);
    assert_eq!(pt.n_classes(), 3); // not C(5,2)/4 = 2.5
}

#[test]
fn vandermonde_and_factor_n_relation() {
    // sum over shift patterns of prod C(X_s, mu_s) = C(nX, m)  (distinct x in disjoint blocks)
    for &(n, m, x) in &[(7usize, 3usize, 5u64), (7, 5, 3), (11, 4, 4), (13, 5, 3), (5, 2, 9)] {
        let pt = PatternTable::new(n, m, true);
        let xs = vec![x; n];
        let tot: f64 = pt.pats.iter().map(|p| pattern_weight(p, &xs)).sum();
        assert_eq!(tot, binom(n as u64 * x, m as u64) as f64);
        // Red-team factor n: rep-only systems (one pattern per class) hold 1/n of the union
        let rep: f64 = pt.rep.iter().map(|&r| pattern_weight(&pt.pats[r], &xs)).sum();
        assert!((rep * n as f64 - tot).abs() < 1e-6 * tot);
        // D * B^m / m!-style claim is n times too small as a whole-union total:
        let distinct_only: f64 = pt.rep.iter().map(|&r| pattern_weight(&pt.pats[r], &xs)).sum();
        assert!(distinct_only * (n as f64) <= tot * (1.0 + 1e-9));
    }
    // unequal block sizes: Vandermonde still holds for the sum of X_k
    let n = 7;
    let m = 4;
    let xs: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7];
    let pt = PatternTable::new(n, m, true);
    let tot: f64 = pt.pats.iter().map(|p| pattern_weight(p, &xs)).sum();
    assert_eq!(tot, binom(28, 4) as f64);
    // asymptotics: C(nB,m) ~ n^m B^m/m!, and rep-only ~ n^{m-1} B^m / m!
    let b = 1000u64;
    let r = binom(7 * b, 3) as f64 / (7f64.powi(3) * (b as f64).powi(3) / 6.0);
    assert!((r - 1.0).abs() < 0.01);
}

fn tiny(n: u32, a: u32, _m: usize, l: usize, seed: u64) -> (Fq, Curve, family::Family, Rng) {
    let f = Fq::new(n);
    let c = Curve::koblitz(a);
    let mut rng = Rng(seed);
    let fam = family::frob(&f, &mut rng, l);
    (f, c, fam, rng)
}

#[test]
fn enumerator_matches_brute_force_and_partitions() {
    for &(n, a, m, l) in &[(7u32, 0u32, 2usize, 3usize), (7, 1, 3, 2), (7, 0, 3, 3), (5, 1, 2, 2), (7, 1, 4, 2)] {
        let (f, c, fam, _) = tiny(n, a, m, l, 11 + n as u64);
        let e = Enumerator::new(&f, &c, &fam, m, 1 << 24);
        let pt = PatternTable::new(n as usize, m, true);
        let targets = make_targets(&f, &c, a, 6, 99);
        for &r in &targets {
            let k = e.count_target(r, &pt);
            assert_eq!(k.total, e.brute_total(r), "n={n} a={a} m={m}");
            assert_eq!(k.overflow, 0);
            let sf: f64 = k.frac.iter().sum();
            assert!((sf - k.total as f64).abs() < 1e-9, "frac partitions total");
            let sc: u64 = k.cover.iter().map(|&v| v as u64).sum();
            assert!(sc >= k.total && sc >= k.unamb);
        }
    }
}

#[test]
fn gauge_identity_exact_for_frob_and_broken_for_random_blocks() {
    // Proposition T per slot: count(sigma R, pattern+1) = count(R, pattern), exactly.
    let (n, a, m) = (7u32, 1u32, 3usize);
    let (f, c, fam, mut rng) = tiny(n, a, m, 3, 5);
    let pt = PatternTable::new(n as usize, m, true);
    let e = Enumerator::new(&f, &c, &fam, m, 1 << 24);
    let targets = make_targets(&f, &c, a, 8, 3);
    for &r in &targets {
        assert!(gauge_identity_holds(&e, &c, &f, &pt, r));
    }
    // class vectors of R and sigma R coincide (the diagonal gauge)
    let k0 = e.count_target(targets[0], &pt);
    let k1 = e.count_target(c.sigma(&f, targets[0]), &pt);
    for ps in &pt.class_pats {
        let s0: u32 = ps.iter().map(|&p| k0.cover[p]).sum();
        let s1: u32 = ps.iter().map(|&p| k1.cover[p]).sum();
        assert_eq!(s0, s1);
    }
    // the same instrument must SEE the violation for blocks without the Frobenius tie
    let fam2 = family::rset(&f, &mut rng, 3);
    let e2 = Enumerator::new(&f, &c, &fam2, m, 1 << 24);
    let broken = targets.iter().filter(|&&r| !gauge_identity_holds(&e2, &c, &f, &pt, r)).count();
    assert!(broken >= 1, "detector failed to see a violation on random blocks");
}

#[test]
fn pure_class_equals_sum_of_plain_conjugates() {
    let (n, a, m) = (7u32, 0u32, 2usize);
    let f = Fq::new(n);
    let c = Curve::koblitz(a);
    let mut rng = Rng(21);
    let v = span_nonzero(&family::random_basis(&mut rng, n, 3));
    let fam = family::frob_from_v(&f, &v, "frob");
    let plain = family::from_blocks("plain", n, vec![v.clone()]);
    let pt = PatternTable::new(n as usize, m, true);
    let pt1 = PatternTable::new(1, m, false);
    let eu = Enumerator::new(&f, &c, &fam, m, 1 << 24);
    let ep = Enumerator::new(&f, &c, &plain, m, 1 << 24);
    for &r in &make_targets(&f, &c, a, 6, 4) {
        let ku = eu.count_target(r, &pt);
        let pure: u64 = (0..n as usize)
            .map(|k| ku.cover[pt.index[&pack(&vec![k as u8; m])]] as u64)
            .sum();
        let mut conj = 0u64;
        let mut s = r;
        for _ in 0..n {
            conj += ep.count_target(s, &pt1).total;
            s = c.sigma(&f, s);
        }
        assert_eq!(pure, conj);
    }
}

#[test]
fn planted_decomposition_lands_in_the_right_class() {
    let (n, a, m) = (7u32, 1u32, 2usize);
    let f = Fq::new(n);
    let c = Curve::koblitz(a);
    let mut rng = Rng(8);
    let v = span_nonzero(&family::random_basis(&mut rng, n, 3));
    let fam = family::frob_from_v(&f, &v, "frob");
    let pt = PatternTable::new(n as usize, m, true);
    let e = Enumerator::new(&f, &c, &fam, m, 1 << 24);
    // plant Q1 in sigma^2 V, Q2 in sigma^5 V with distinct x
    let pick = |k: u32| {
        for &x in &v {
            let xs = f.frob_pow(x, k);
            if let Some((p, _)) = c.lift(&f, xs) {
                return p;
            }
        }
        panic!("no liftable x");
    };
    let (q1, q2) = (pick(2), pick(5));
    assert!(q1.x != q2.x);
    let r = c.add(&f, q1, q2);
    let k = e.count_target(r, &pt);
    let idx = pt.index[&pack(&[2, 5])];
    assert!(k.cover[idx] >= 1, "planted pattern (2,5) missing");
    // sigma-shifted copy: target sigma^-2 R decomposes with pattern (0,3), same class
    let r2 = (0..5).fold(r, |s, _| c.sigma(&f, s)); // sigma^5 = sigma^-2 as n = 7
    let k2 = e.count_target(r2, &pt);
    let idx2 = pt.index[&pack(&[0, 3])];
    assert!(k2.cover[idx2] >= 1);
    assert_eq!(pt.class_of[idx], pt.class_of[idx2]);
}

#[test]
fn row_roundtrip() {
    let r = Row {
        a: 1, n: 7, m: 3, l: 2, arm: "frob".into(), vd: 3, nt: 4, nb: 7, d: 3, order: 128, npts: 10,
        nleft: 5, total: 9, unamb: 8, overflow: 0, audit: 1, ms: 1.5,
        xs: vec![1, 2, 3], ycov: vec![1.0, 2.0, 3.0], yfrac: vec![0.5, 1.5, 2.0], yrep: vec![1.0, 0.0, 0.0],
    };
    let r2 = Row::from_tsv(&r.to_tsv());
    assert_eq!(r2.to_tsv(), r.to_tsv());
}

fn poisson(rng: &mut Rng, lam: f64) -> f64 {
    if lam > 30.0 {
        // normal approximation, adequate for a detector self-test
        let (u1, u2) = (rng.unit().max(1e-12), rng.unit());
        let g = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        return (lam + lam.sqrt() * g).round().max(0.0);
    }
    let (l, mut k, mut p) = ((-lam).exp(), 0.0, 1.0);
    loop {
        p *= rng.unit();
        if p <= l {
            return k;
        }
        k += 1.0;
    }
}

fn synthetic_rows(n: u32, m: usize, nrows: usize, bump: f64, seed: u64) -> (Vec<Row>, PatternTable) {
    let pt = PatternTable::new(n as usize, m, true);
    let mut rng = Rng(seed);
    let mut rows = vec![];
    for vd in 0..nrows {
        let mut r = Row {
            a: 1, n, m, l: 3, arm: "synthetic".into(), vd, nt: 48, nb: n as usize, d: pt.n_classes(),
            order: 8192, xs: vec![3; n as usize], ..Default::default()
        };
        let mu = mu_classes(&r, &pt, true);
        let y: Vec<f64> = mu
            .iter()
            .enumerate()
            .map(|(c, &u)| poisson(&mut rng, if c % 10 == 3 { u * (1.0 + bump) } else { u }))
            .collect();
        r.total = y.iter().sum::<f64>() as u64;
        r.yfrac = y.clone();
        r.ycov = y;
        rows.push(r);
    }
    (rows, pt)
}

#[test]
fn detector_sees_planted_replicable_concentration_and_not_noise() {
    // D(13,5) = 476 classes; null: Poisson around the aware null, no structure
    let (rows, pt) = synthetic_rows(13, 5, 16, 0.0, 1);
    let refs: Vec<&Row> = rows.iter().collect();
    let s = split_stats(&refs, &pt, true, 77, 30, 300);
    assert!((s.rho / s.rho_se).abs() < 3.5, "false positive: rho={} se={}", s.rho, s.rho_se);
    assert!((s.phi - 1.0).abs() < 0.35, "phi={}", s.phi);
    // planted: 10 percent of classes carry +20 percent in every draw
    let (rows, pt) = synthetic_rows(13, 5, 16, 0.2, 2);
    let refs: Vec<&Row> = rows.iter().collect();
    let s = split_stats(&refs, &pt, true, 77, 30, 300);
    assert!(s.rho / s.rho_se > 5.0, "missed planted structure: rho={} se={}", s.rho, s.rho_se);
    assert!(s.transfer > 1.1, "transfer={}", s.transfer);
}

#[test]
fn plan_grid_is_feasible_and_printed() {
    println!("a n m l D N_m n^(m-1)/D X/block E[dec/target] mu/class/half left est_s/arm rho_min rho_ok");
    let (mut total_s, mut unresolved) = (0.0, 0);
    for (a, n, m) in grid() {
        let l = choose_l(a, n, m);
        let r = plan_row(a, n, m, l);
        assert!(r.left <= MAX_LEFT as f64 * 1.0 + 1.0 || l <= 2, "{a} {n} {m} {l} left {}", r.left);
        println!(
            "{} {} {} {} {} {} {:.2} {:.1} {:.3e} {:.2} {:.3e} {:.1} {:.2} {}",
            r.a, r.n, r.m, r.l, r.d, r.n_pat, r.baseline_ratio, r.x_block, r.e_total, r.mu_class_half,
            r.left, r.est_sec_arm, r.rho_min, r.rho_resolved
        );
        total_s += r.est_sec_arm * 7.0; // seven arms (plain arms are cheaper)
        if !r.rho_resolved {
            unresolved += 1;
        }
    }
    println!("TOTAL est seconds (7 arms, 1 core, upper): {total_s:.0}; cells with D<113: {unresolved}");
}

#[test]
fn analysis_pipeline_end_to_end_on_synthetic_rows() {
    // Exercises report::analyze_dir on synthetic Poisson rows (a pipeline fixture, not a result).
    let base = std::env::var("G2_TEST_TMP").unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let dir = format!("{base}/g2_synth_{}", std::process::id());
    std::fs::create_dir_all(&dir).unwrap();
    for &(n, m) in &[(13u32, 5usize), (7, 3)] {
        let mut body = String::from(HEADER);
        body.push('\n');
        for (arm, bump, seed) in [("frob", 0.0, 1u64), ("rsub", 0.0, 2), ("rset", 0.0, 3), ("skew", 0.25, 4)] {
            let (rows, _) = synthetic_rows(n, m, 16, bump, seed + n as u64);
            for mut r in rows {
                r.arm = arm.into();
                r.audit = if arm == "frob" { 1 } else if arm == "skew" { -1 } else { 0 };
                body.push_str(&r.to_tsv());
                body.push('\n');
            }
        }
        std::fs::write(format!("{dir}/rows_a1_n{n}_m{m}.tsv"), body).unwrap();
    }
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    println!("{md}");
    assert!(md.contains("| 1,13,5,3 |"));
    assert!(md.contains("UNIFORM"), "null frob should read UNIFORM at D=476");
    assert!(std::fs::metadata(format!("{dir}/rho_z.svg")).is_ok());
    std::fs::remove_dir_all(&dir).unwrap();
}
