//! Correctness tests on tiny hand-made fixtures: exact identities, planted
//! instances, detector self-tests on synthetic counts. No result is produced.

use g2_orbit_union_dnm::combi::*;
use g2_orbit_union_dnm::curve::*;
use g2_orbit_union_dnm::enumr::Enumerator;
use g2_orbit_union_dnm::family::{self, span_nonzero};
use g2_orbit_union_dnm::field::{irreducible_poly, Fq};
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
                assert_eq!(
                    c.sigma(&f, c.add(&f, p, q)),
                    c.add(&f, c.sigma(&f, p), c.sigma(&f, q))
                );
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
    assert_eq!(
        (2..=4).map(|m| d_nm(7, m)).collect::<Vec<_>>(),
        vec![4, 12, 30]
    );
    assert_eq!(
        (3..=5).map(|m| d_nm(11, m)).collect::<Vec<_>>(),
        vec![26, 91, 273]
    );
    assert_eq!(d_nm(13, 4), 140);
    assert_eq!(d_nm(17, 2), 9);
    assert_eq!(
        [2u64, 3, 4, 5, 6].map(|m| d_nm(17, m)).to_vec(),
        vec![9, 57, 285, 1197, 4389]
    );
    assert_eq!(
        [2u64, 3, 4, 5].map(|m| d_nm(131, m)).to_vec(),
        vec![66, 2926, 98021, 2646567]
    );
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
    for &(n, m) in &[
        (5usize, 2usize),
        (5, 3),
        (5, 4),
        (7, 2),
        (7, 3),
        (7, 4),
        (7, 5),
        (11, 3),
    ] {
        let pt = PatternTable::new(n, m, true);
        assert_eq!(pt.pats.len() as u128, n_patterns(n as u64, m as u64));
        assert_eq!(
            pt.n_classes() as u128,
            d_nm(n as u64, m as u64),
            "n={n} m={m}"
        );
        // every class has exactly n members (free action), they sum to N_m
        assert!(pt.class_pats.iter().all(|c| c.len() == n));
        assert_eq!(
            pt.class_pats.iter().map(|c| c.len()).sum::<usize>(),
            pt.pats.len()
        );
        // Burnside by direct count: shift j != 0 fixes no multiset (n prime, m < n)
        for j in 1..n {
            let fixed = pt
                .pats
                .iter()
                .filter(|p| shift_pattern(p, j, n) == **p)
                .count();
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
    for &(n, m, x) in &[
        (7usize, 3usize, 5u64),
        (7, 5, 3),
        (11, 4, 4),
        (13, 5, 3),
        (5, 2, 9),
    ] {
        let pt = PatternTable::new(n, m, true);
        let xs = vec![x; n];
        let tot: f64 = pt.pats.iter().map(|p| pattern_weight(p, &xs)).sum();
        assert_eq!(tot, binom(n as u64 * x, m as u64) as f64);
        // Red-team factor n: rep-only systems (one pattern per class) hold 1/n of the union
        let rep: f64 = pt
            .rep
            .iter()
            .map(|&r| pattern_weight(&pt.pats[r], &xs))
            .sum();
        assert!((rep * n as f64 - tot).abs() < 1e-6 * tot);
        // leading-order form of the same statement: a pattern with multiplicities mu has weight
        // ~ B^m / prod(mu!) (so repeated shifts weigh MORE than B^m/m!); reps hold 1/n of the union.
    }
    {
        let (n, m, b) = (7usize, 3usize, 4000u64);
        let pt = PatternTable::new(n, m, true);
        let xs = vec![b; n];
        let rep: f64 = pt
            .rep
            .iter()
            .map(|&r| pattern_weight(&pt.pats[r], &xs))
            .sum();
        let lead: f64 = pt
            .rep
            .iter()
            .map(|&r| {
                let sh = shape(&pt.pats[r]);
                (b as f64).powi(m as i32)
                    / sh.iter()
                        .map(|&k| (1..=k as u64).product::<u64>() as f64)
                        .product::<f64>()
            })
            .sum();
        assert!(
            (rep / lead - 1.0).abs() < 1e-2,
            "rep {rep} vs leading {lead}"
        );
        // the red-team shortcut D*B^m/m! UNDERcounts the representative total when shifts repeat
        let d = d_nm(n as u64, m as u64) as f64;
        let shortcut = d * (b as f64).powi(m as i32) / 6.0;
        assert!(
            shortcut < rep,
            "shortcut {shortcut} should be below rep {rep}"
        );
        assert!(rep / shortcut > 1.5, "ratio {}", rep / shortcut);
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
    for &(n, a, m, l) in &[
        (7u32, 0u32, 2usize, 3usize),
        (7, 1, 3, 2),
        (7, 0, 3, 3),
        (5, 1, 2, 2),
        (7, 1, 4, 2),
        (7, 1, 5, 2),
        (7, 0, 5, 3),
    ] {
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
    let broken = targets
        .iter()
        .filter(|&&r| !gauge_identity_holds(&e2, &c, &f, &pt, r))
        .count();
    assert!(
        broken >= 1,
        "detector failed to see a violation on random blocks"
    );
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
        a: 1,
        n: 7,
        m: 3,
        l: 2,
        arm: "frob".into(),
        vd: 3,
        nt: 4,
        nb: 7,
        d: 3,
        order: 128,
        npts: 10,
        nleft: 5,
        total: 9,
        unamb: 8,
        overflow: 0,
        audit: 1,
        ms: 1.5,
        xs: vec![1, 2, 3],
        xodd: vec![0, 1, 1],
        xtwo: vec![1, 0, 0],
        seed: 20261027,
        ycov: vec![1.0, 2.0, 3.0],
        yfrac: vec![0.5, 1.5, 2.0],
        yrep: vec![1.0, 0.0, 0.0],
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

fn synthetic_rows(
    n: u32,
    m: usize,
    nrows: usize,
    bump: f64,
    seed: u64,
) -> (Vec<Row>, PatternTable) {
    synthetic_rows_o(n, m, nrows, bump, seed, 8192)
}

fn synthetic_rows_o(
    n: u32,
    m: usize,
    nrows: usize,
    bump: f64,
    seed: u64,
    order: u64,
) -> (Vec<Row>, PatternTable) {
    let pt = PatternTable::new(n as usize, m, true);
    let mut rng = Rng(seed);
    let mut rows = vec![];
    for vd in 0..nrows {
        let mut r = Row {
            a: 1,
            n,
            m,
            l: 3,
            arm: "synthetic".into(),
            vd,
            nt: 48,
            nb: n as usize,
            d: pt.n_classes(),
            order,
            xs: vec![3; n as usize],
            npts: 2 * 3 * n as usize,
            ..Default::default()
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
    assert!(
        (s.rho / s.rho_se).abs() < 3.5,
        "false positive: rho={} se={}",
        s.rho,
        s.rho_se
    );
    assert!((s.phi - 1.0).abs() < 0.35, "phi={}", s.phi);
    // planted: 10 percent of classes carry +20 percent in every draw
    let (rows, pt) = synthetic_rows(13, 5, 16, 0.2, 2);
    let refs: Vec<&Row> = rows.iter().collect();
    let s = split_stats(&refs, &pt, true, 77, 30, 300);
    assert!(
        s.rho / s.rho_se > 5.0,
        "missed planted structure: rho={} se={}",
        s.rho,
        s.rho_se
    );
    assert!(s.transfer > 1.1, "transfer={}", s.transfer);
}

#[test]
fn plan_grid_is_feasible_and_printed() {
    println!("a n m l D N_m n^(m-1)/D X/block E[dec/target] mu/class/half left left_big left_big_worst est_s/cell rho_min rho_ok");
    let (mut total_s, mut unresolved, mut crit) = (0.0, 0, 0.0f64);
    let cs = cells();
    for &(a, n, m, l) in &cs {
        let r = plan_row(a, n, m, l);
        assert!(
            r.left <= MAX_LEFT as f64,
            "{a} {n} {m} {l} union left {}",
            r.left
        );
        // plainbig: even at mean + 6 sd of its liftable count the left table stays under the cap
        assert!(
            r.left_big_worst <= MAX_LEFT as f64,
            "{a} {n} {m} {l} plainbig worst {}",
            r.left_big_worst
        );
        println!(
            "{} {} {} {} {} {} {:.2} {:.1} {:.3e} {:.2} {:.3e} {:.3e} {:.3e} {:.1} {:.2} {}",
            r.a,
            r.n,
            r.m,
            r.l,
            r.d,
            r.n_pat,
            r.baseline_ratio,
            r.x_block,
            r.e_total,
            r.mu_class_half,
            r.left,
            r.left_big,
            r.left_big_worst,
            r.est_sec_cell,
            r.rho_min,
            r.rho_resolved
        );
        total_s += r.est_sec_cell;
        crit = crit.max(r.est_sec_cell);
        if !r.rho_resolved {
            unresolved += 1;
        }
    }
    println!("cells {}: TOTAL est seconds (single core): {total_s:.0}; longest cell {crit:.0}; rho-unresolved cells (D < {MIN_D_FOR_RHO}): {unresolved}", cs.len());
    // the fixed-l column and the m = 4 ladder are in the frozen set
    for &a in &[0u32, 1] {
        for &n in &[13u32, 17] {
            for m in 2..=5usize {
                assert!(cs.contains(&(a, n, m, 3)));
            }
            for l in 2..=5usize {
                assert!(cs.contains(&(a, n, 4, l)));
            }
        }
    }
}

#[test]
fn plainbig_is_larger_than_the_union_so_it_is_not_size_matched() {
    // documents the mislabelling fix: |plainbig| / |U| > 1 for every cell, up to ~1.9x at n = 17
    for &(n, l) in &[(7u32, 2usize), (11, 3), (13, 3), (17, 3), (17, 9)] {
        let big = ((1u64 << plainbig_dim(n, l)) - 1) as f64;
        let union = n as f64 * ((1u64 << l) - 1) as f64;
        assert!(big > union * 1.05, "n={n} l={l}: {big} vs {union}");
    }
}

fn write_cell(dir: &str, n: u32, m: usize, order: u64, spec: &[(&str, f64, u64)]) {
    // spec: (arm, bump, seed). Pipeline fixtures, not results.
    let mut body = String::from(HEADER);
    body.push('\n');
    for &(arm, bump, seed) in spec {
        let (rows, _) = synthetic_rows_o(n, m, 16, bump, seed + 31 * n as u64 + m as u64, order);
        for mut r in rows {
            r.arm = arm.into();
            r.audit = if arm == "frob" {
                1
            } else if arm == "skew" {
                -1
            } else {
                0
            };
            body.push_str(&r.to_tsv());
            body.push('\n');
        }
    }
    std::fs::write(format!("{dir}/rows_a1_n{n}_m{m}_l3.tsv"), body).unwrap();
}

fn verdict_of(dir: &str, n: u32, m: usize) -> String {
    let t = std::fs::read_to_string(format!("{dir}/verdicts.tsv")).unwrap();
    for line in t.lines().skip(1) {
        let c: Vec<&str> = line.split('\t').collect();
        if c[1] == n.to_string() && c[2] == m.to_string() {
            return c[6].to_string();
        }
    }
    panic!("no verdict for n={n} m={m}");
}

#[test]
fn analysis_pipeline_pins_verdicts_on_planted_fixtures() {
    let base = std::env::var("G2_TEST_TMP")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let dir = format!("{base}/g2_synth_{}", std::process::id());
    std::fs::create_dir_all(&dir).unwrap();
    // D(13,5) = 476: pure null everywhere, positive control (skew) detected -> UNIFORM
    write_cell(
        &dir,
        13,
        5,
        8192,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.25, 5),
        ],
    );
    // D(13,4) = 140: replicable class excess in frob only (strong enough for transfer >= 2) -> CONCENTRATED
    write_cell(
        &dir,
        13,
        4,
        8192,
        &[
            ("frob", 1.5, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.6, 5),
        ],
    );
    // D(11,5) = 273: the SAME excess in frob and frobperm (a shared nuisance) -> NUISANCE_REPLICABLE
    write_cell(
        &dir,
        11,
        5,
        8192,
        &[
            ("frob", 0.4, 1),
            ("frobperm", 0.4, 1),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.4, 5),
        ],
    );
    // D(13,5) analogue with the excess also seen by a negative control -> INSTRUMENT_FAIL
    write_cell(
        &dir,
        17,
        4,
        8192,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.4, 3),
            ("rset", 0.0, 4),
            ("skew", 0.4, 5),
        ],
    );
    // low D: planted effect with large counts -> CONCENTRATED_VS_CONTROLS; pure null -> UNRESOLVED (never UNIFORM)
    write_cell(
        &dir,
        7,
        3,
        64,
        &[
            ("frob", 1.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
        ],
    );
    write_cell(
        &dir,
        7,
        2,
        64,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
        ],
    );
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    println!("{md}");
    assert!(md.contains("| 1,13,5,3 |"));
    assert!(md.contains("l ladder"));
    assert_eq!(verdict_of(&dir, 13, 5), "UNIFORM");
    assert_eq!(verdict_of(&dir, 13, 4), "CONCENTRATED");
    assert_eq!(verdict_of(&dir, 11, 5), "NUISANCE_REPLICABLE");
    assert_eq!(verdict_of(&dir, 17, 4), "INSTRUMENT_FAIL");
    assert_eq!(verdict_of(&dir, 7, 3), "CONCENTRATED_VS_CONTROLS");
    assert_eq!(verdict_of(&dir, 7, 2), "UNRESOLVED");
    assert!(!md.contains("UNIFORM_VS_CONTROLS"));
    assert!(std::fs::metadata(format!("{dir}/rho_z.svg")).is_ok());
    std::fs::remove_dir_all(&dir).unwrap();
}

// ---------------------------------------------------------------------------------------------
// Review fixes: halving criterion, parity-aware null, independent group law, frobperm, targets
// ---------------------------------------------------------------------------------------------

#[test]
fn halving_criterion_p_in_2e_iff_trace_x_equals_trace_a() {
    for &n in &[7u32, 11] {
        let f = Fq::new(n);
        for a in 0..=1u32 {
            let c = Curve::koblitz(a);
            let pts = c.all_points(&f);
            let mut in2e = std::collections::HashSet::new();
            for &q in &pts {
                let d = c.add(&f, q, q);
                if !d.is_inf() {
                    in2e.insert(d.key());
                }
            }
            let ta = f.tr(a);
            let mut checked = 0;
            for &p in &pts {
                if p.x == 0 {
                    continue; // the 2-torsion point (0,1): in 2E iff E[2^inf] is cyclic of order >= 4
                }
                assert_eq!(
                    in2e.contains(&p.key()),
                    f.tr(p.x) == ta,
                    "n={n} a={a} x={}",
                    p.x
                );
                checked += 1;
            }
            assert!(checked > 100);
        }
    }
}

#[test]
fn parity_weight_reduces_to_twice_size_weight_without_odd_points() {
    let pt = PatternTable::new(7, 4, true);
    let xs = vec![5u64, 3, 4, 6, 2, 5, 3];
    for p in &pt.pats {
        let w = pattern_weight(p, &xs);
        let wp = pattern_weight_parity(p, &xs, &[]);
        assert!((wp - 2.0 * w).abs() < 1e-9 * (1.0 + w), "{p:?}");
    }
    // all x odd, odd m: no decomposition can exist (sum of an odd number of non-doubles is not in 2E)
    let odd = xs.clone();
    for p in pt.pats.iter().filter(|p| p.len() == 4) {
        assert_eq!(
            pattern_weight_parity(p, &xs, &odd),
            2.0 * pattern_weight(p, &xs)
        ); // m = 4 even: allowed
    }
    let pt3 = PatternTable::new(7, 3, true);
    for p in &pt3.pats {
        assert_eq!(pattern_weight_parity(p, &xs, &odd), 0.0, "{p:?}");
    }
}

#[test]
fn trace_kernel_family_has_zero_decompositions_exactly_as_the_parity_null_says() {
    // a = 1: Tr(a) = 1; V inside ker Tr makes EVERY liftable x odd. m = 3 (odd): the sum is not in 2E,
    // but every target lies in hE inside 2E. Exact zero. A generic V with the same size does not.
    let (n, a, m, l) = (7u32, 1u32, 3usize, 2usize);
    let f = Fq::new(n);
    let c = Curve::koblitz(a);
    let mut rng = Rng(4242);
    let mut basis = vec![];
    while basis.len() < l {
        let v = (rng.below((1u64 << n) - 1) + 1) as u32;
        if f.tr(v) == 0 && !basis.contains(&v) && (basis.is_empty() || basis[0] ^ v != 0) {
            basis.push(v);
        }
    }
    let v = span_nonzero(&basis);
    assert!(v.iter().all(|&x| f.tr(x) == 0));
    let fam = family::frob_from_v(&f, &v, "frob");
    let e = Enumerator::new(&f, &c, &fam, m, 1 << 24);
    let pt = PatternTable::new(n as usize, m, true);
    let targets = make_targets(&f, &c, a, 12, 8);
    assert!(!targets.is_empty());
    let xs = liftable_counts(&f, &c, &fam);
    let xodd = odd_counts(&f, &c, &fam);
    assert_eq!(
        xs, xodd,
        "every liftable x of a ker-Tr block is odd when Tr(a) = 1"
    );
    for p in &pt.pats {
        assert_eq!(pattern_weight_parity(p, &xs, &xodd), 0.0);
    }
    for &r in &targets {
        assert_eq!(e.count_target(r, &pt).total, 0);
        assert_eq!(e.brute_total(r), 0);
    }
}

/// Independent GF(2^n) schoolbook arithmetic (no exp/log tables) for a cross-check of Curve::add.
struct Slow {
    n: u32,
    poly: u64,
}
impl Slow {
    fn mul(&self, a: u32, b: u32) -> u32 {
        let (mut r, mut a, mut b) = (0u64, a as u64, b as u64);
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if (a >> self.n) & 1 == 1 {
                a ^= self.poly;
            }
        }
        r as u32
    }
    fn inv(&self, a: u32) -> u32 {
        // a^(2^n - 2)
        let mut e = (1u64 << self.n) - 2;
        let (mut r, mut base) = (1u32, a);
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        r
    }
    /// affine chord-and-tangent on y^2 + xy = x^3 + a x^2 + 1; None = infinity
    fn add(&self, a: u32, p: Option<(u32, u32)>, q: Option<(u32, u32)>) -> Option<(u32, u32)> {
        let (p, q) = match (p, q) {
            (None, q) => return q,
            (p, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        let ((x1, y1), (x2, y2)) = (p, q);
        let lam;
        let x3;
        if x1 == x2 {
            if y1 != y2 || x1 == 0 {
                return None; // Q = -P (for x1 = 0 the point is its own negative)
            }
            lam = x1 ^ self.mul(y1, self.inv(x1));
            x3 = self.mul(lam, lam) ^ lam ^ a;
        } else {
            lam = self.mul(y1 ^ y2, self.inv(x1 ^ x2));
            x3 = self.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ a;
        }
        let y3 = self.mul(x1 ^ x3, lam) ^ x3 ^ y1;
        Some((x3, y3))
    }
}

#[test]
fn curve_add_agrees_with_an_independent_table_free_implementation() {
    for &n in &[7u32, 11, 13] {
        let f = Fq::new(n);
        let slow = Slow {
            n,
            poly: irreducible_poly(n) as u64,
        };
        for a in 0..=1u32 {
            let c = Curve::koblitz(a);
            let pts = c.all_points(&f);
            let mut rng = Rng(77 + n as u64 + a as u64);
            let conv = |p: P| if p.is_inf() { None } else { Some((p.x, p.y)) };
            let mut n_doubling = 0;
            for t in 0..4000 {
                let p = pts[rng.below(pts.len() as u64) as usize];
                let q = match t % 4 {
                    0 => p,
                    1 => c.neg(p),
                    _ => pts[rng.below(pts.len() as u64) as usize],
                };
                n_doubling += (p == q) as u32;
                assert_eq!(
                    conv(c.add(&f, p, q)),
                    slow.add(a, conv(p), conv(q)),
                    "n={n} a={a} p={p:?} q={q:?}"
                );
            }
            assert!(n_doubling > 500);
        }
    }
}

#[test]
fn frobperm_has_the_same_union_and_totals_as_frob_but_not_the_gauge() {
    let (n, a, m, l) = (7u32, 1u32, 3usize, 3usize);
    let f = Fq::new(n);
    let c = Curve::koblitz(a);
    // same rng seed => same V (this is what seed_code does in the runner)
    let fr = family::build("frob", &f, &mut Rng(99), l);
    let fp = family::build("frobperm", &f, &mut Rng(99), l);
    let mut u1: Vec<u32> = fr.blocks.iter().flatten().copied().collect();
    let mut u2: Vec<u32> = fp.blocks.iter().flatten().copied().collect();
    u1.sort();
    u2.sort();
    assert_eq!(u1, u2);
    assert_ne!(
        fr.blocks, fp.blocks,
        "labels must differ (n = 7: 1/5040 chance of the identity permutation)"
    );
    let lc = liftable_counts(&f, &c, &fr);
    assert!(
        lc.iter().all(|&v| v == lc[0]),
        "liftability is sigma-invariant: all frob blocks have one size"
    );
    assert_eq!(
        odd_counts(&f, &c, &fr),
        odd_counts(&f, &c, &fp),
        "one common trace profile"
    );
    assert_eq!(seed_code("frobperm"), seed_code("frob"));
    let pt = PatternTable::new(n as usize, m, true);
    let (e1, e2) = (
        Enumerator::new(&f, &c, &fr, m, 1 << 24),
        Enumerator::new(&f, &c, &fp, m, 1 << 24),
    );
    let targets = make_targets(&f, &c, a, 10, 5);
    let mut violated = 0;
    for &r in &targets {
        let (k1, k2) = (e1.count_target(r, &pt), e2.count_target(r, &pt));
        assert_eq!(k1.total, k2.total, "same union, same decompositions");
        assert_eq!(k1.total, e1.brute_total(r));
        assert!(gauge_identity_holds(&e1, &c, &f, &pt, r));
        violated += (!gauge_identity_holds(&e2, &c, &f, &pt, r)) as usize;
    }
    assert!(
        violated >= 1,
        "the permuted labelling must break the per-slot gauge identity somewhere"
    );
}

#[test]
fn targets_are_distinct_one_per_frobenius_orbit() {
    for &(n, a) in &[(7u32, 0u32), (7, 1), (11, 1), (11, 0)] {
        let f = Fq::new(n);
        let c = Curve::koblitz(a);
        let ts = make_targets(&f, &c, a, 1000, 3);
        let mut seen = std::collections::HashSet::new();
        for &r in &ts {
            let mut q = r;
            let mut mn = q.key().min(c.neg(q).key());
            for _ in 1..n {
                q = c.sigma(&f, q);
                mn = mn.min(q.key()).min(c.neg(q).key());
            }
            assert!(
                seen.insert(mn),
                "two targets share a Frobenius-and-negation orbit (+-R have the same x)"
            );
            assert!(
                c.scalar(&f, r, koblitz_order(a, n) / cofactor(a)).is_inf(),
                "target outside hE"
            );
        }
        println!("n={n} a={a}: {} orbit representatives in hE", ts.len());
        assert!(ts.len() >= 2);
        if n == 7 {
            assert!(
                ts.len() < 48,
                "NT = 48 cannot be met at n = 7; the row records the effective nt"
            );
        }
    }
}

#[test]
fn run_cell_smoke_on_a_tiny_fixture_checks_plumbing_not_results() {
    // n = 7, nv = 2: exercises all 8 arms, seeds, xodd, audit, tsv round trip. Plumbing only.
    let cfg = Cfg {
        a: 1,
        n: 7,
        m: 3,
        l: 3,
        nv: 2,
        nt: 48,
        seed: 12345,
        arms: family::ARMS.iter().map(|s| s.to_string()).collect(),
        max_left: MAX_LEFT,
    };
    let mut rows: Vec<Row> = vec![];
    run_cell(&cfg, |r| rows.push(r.clone()));
    assert_eq!(rows.len(), 2 * 8);
    for r in &rows {
        assert_eq!(Row::from_tsv(&r.to_tsv()).to_tsv(), r.to_tsv());
        assert_eq!(r.seed, 12345);
        assert!(r.nt >= 4 && r.nt < 48);
        assert!(r.xodd.iter().zip(&r.xs).all(|(o, x)| o <= x));
        assert_eq!(r.overflow, 0);
        let sf: f64 = r.yfrac.iter().sum();
        assert!(
            (sf - r.total as f64).abs() < 1e-3 * (1.0 + r.total as f64),
            "frac partitions total (5-decimal print)"
        );
    }
    for vd in 0..2 {
        let get = |arm: &str| rows.iter().find(|r| r.arm == arm && r.vd == vd).unwrap();
        assert_eq!(
            get("frob").total,
            get("frobperm").total,
            "same V, same union, same targets"
        );
        assert_eq!(get("frob").audit, if vd == 0 { 1 } else { -1 });
        assert_eq!(get("frob").xs, get("frobperm").xs);
        assert_eq!(get("frob").nb, 7);
        assert_eq!(get("plain").nb, 1);
    }
}

// ---------------------------------------------------------------------------------------------
// Review round 2: mod-4 null for a = 0, calibrated z, direct frob/frobperm contrast, completeness
// ---------------------------------------------------------------------------------------------

/// Type of a liftable x in E/4E for a = 0: 0 (class 0), 2 (class 2), 1 (odd, class +-1), by the
/// independent test "k' Q for k' = #E/4": O, order 2 (not O), order 4.
fn type_of(f: &Fq, c: &Curve, order: u64, x: u32) -> u8 {
    let (q, _) = c.lift(f, x).unwrap();
    let r = c.scalar(f, q, order / 4);
    if r.is_inf() {
        0
    } else if c.add(f, r, r).is_inf() {
        2
    } else {
        1
    }
}

#[test]
fn mod4_sign_count_structure_for_a0() {
    // a = 0: #E = 4 * odd and the 2-part is cyclic. For random distinct-x tuples, the number of sign
    // choices whose sum lies in 4E (k' * sum = O) is 0 if #odd is odd, 2^(m-1) if #odd > 0 is even,
    // and for #odd = 0: 2^m if #type-2 is even else 0. Also: types agree with Tr(x) (odd <=> Tr(x) = 1).
    for &n in &[7u32, 11, 13] {
        let f = Fq::new(n);
        let c = Curve::koblitz(0);
        let order = koblitz_order(0, n);
        assert!(order % 4 == 0 && (order / 4) % 2 == 1);
        let kp = order / 4;
        let lifts: Vec<u32> = (1..f.size() as u32)
            .filter(|&x| c.lift(&f, x).is_some())
            .collect();
        let types: Vec<u8> = lifts.iter().map(|&x| type_of(&f, &c, order, x)).collect();
        for (i, &x) in lifts.iter().enumerate() {
            assert_eq!(types[i] == 1, f.tr(x) == 1, "odd <=> Tr(x) != Tr(a)");
        }
        let cnt = |t: u8| types.iter().filter(|&&v| v == t).count() as f64 / types.len() as f64;
        println!(
            "n={n}: type fractions 0:{:.3} 2:{:.3} odd:{:.3}",
            cnt(0),
            cnt(2),
            cnt(1)
        );
        assert!(
            (cnt(0) - 0.25).abs() < 0.08
                && (cnt(2) - 0.25).abs() < 0.08
                && (cnt(1) - 0.5).abs() < 0.08
        );
        let mut rng = Rng(555 + n as u64);
        let mut seen = [0usize; 3];
        for t in 0..3000 {
            let m = 2 + (t % 4);
            let mut idx: Vec<usize> = vec![];
            while idx.len() < m {
                let i = rng.below(lifts.len() as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            let pts: Vec<P> = idx
                .iter()
                .map(|&i| c.lift(&f, lifts[i]).unwrap().0)
                .collect();
            let mut ok = 0u64;
            for mask in 0..(1u32 << m) {
                let mut s = INF;
                for (j, &p) in pts.iter().enumerate() {
                    s = c.add(&f, s, if (mask >> j) & 1 == 1 { c.neg(p) } else { p });
                }
                ok += c.scalar(&f, s, kp).is_inf() as u64;
            }
            let o = idx.iter().filter(|&&i| types[i] == 1).count();
            let tw = idx.iter().filter(|&&i| types[i] == 2).count();
            let expect = if o % 2 == 1 {
                seen[0] += 1;
                0
            } else if o > 0 {
                seen[1] += 1;
                1u64 << (m - 1)
            } else {
                seen[2] += 1;
                if tw % 2 == 0 {
                    1u64 << m
                } else {
                    0
                }
            };
            assert_eq!(
                ok,
                expect,
                "n={n} m={m} types={:?}",
                idx.iter().map(|&i| types[i]).collect::<Vec<_>>()
            );
        }
        assert!(seen.iter().all(|&v| v > 20), "{seen:?}");
        // the recorded per-block counts are exactly these types
        let fam = family::frob(&f, &mut Rng(7), 4);
        let (xo, xt) = (odd_counts(&f, &c, &fam), two_counts(&f, &c, &fam, order));
        for (b, blk) in fam.blocks.iter().enumerate() {
            let want_o = blk
                .iter()
                .filter(|&&x| c.lift(&f, x).is_some() && type_of(&f, &c, order, x) == 1)
                .count() as u64;
            let want_t = blk
                .iter()
                .filter(|&&x| c.lift(&f, x).is_some() && type_of(&f, &c, order, x) == 2)
                .count() as u64;
            assert_eq!((xo[b], xt[b]), (want_o, want_t));
        }
    }
    // a = 1: nothing of type 2 is recorded
    let f = Fq::new(7);
    let fam = family::frob(&f, &mut Rng(1), 3);
    assert!(
        two_counts(&f, &Curve::koblitz(1), &fam, koblitz_order(1, 7))
            .iter()
            .all(|&v| v == 0)
    );
}

#[test]
fn mod4_weight_matches_tuple_sum_and_reduces_correctly() {
    // brute-force sum over x-tuples of the tuple weight (0, 1/2, [T even]) against the closed form
    let pt = PatternTable::new(3, 3, false);
    let xs = [5u64, 4, 6];
    let xo = [2u64, 1, 3];
    let xt = [1u64, 2, 1];
    for p in &pt.pats {
        // enumerate: per block, choose mu_s of its x's; each x has a type
        let mut tot2 = 0.0;
        let mut blocks: Vec<(usize, usize)> = vec![]; // (block, mult)
        let mut i = 0;
        while i < p.len() {
            let mut j = i;
            while j < p.len() && p[j] == p[i] {
                j += 1;
            }
            blocks.push((p[i] as usize, j - i));
            i = j;
        }
        // type lists per block
        let mut slots: Vec<Vec<u8>> = vec![];
        for &(b, mu) in &blocks {
            let mut ty = vec![];
            ty.extend(std::iter::repeat(1u8).take(xo[b] as usize));
            ty.extend(std::iter::repeat(2u8).take(xt[b] as usize));
            ty.extend(std::iter::repeat(0u8).take((xs[b] - xo[b] - xt[b]) as usize));
            slots.push(ty);
            let _ = mu;
        }
        // recursive subset enumeration
        fn rec(
            bi: usize,
            blocks: &[(usize, usize)],
            slots: &[Vec<u8>],
            o: usize,
            t: usize,
            acc: &mut f64,
        ) {
            if bi == blocks.len() {
                *acc += if o % 2 == 1 {
                    0.0
                } else if o > 0 {
                    0.5
                } else if t % 2 == 0 {
                    1.0
                } else {
                    0.0
                };
                return;
            }
            let (_, mu) = blocks[bi];
            let ty = &slots[bi];
            fn subs(
                start: usize,
                left: usize,
                ty: &[u8],
                o: usize,
                t: usize,
                f: &mut dyn FnMut(usize, usize),
            ) {
                if left == 0 {
                    f(o, t);
                    return;
                }
                for i in start..ty.len() {
                    subs(
                        i + 1,
                        left - 1,
                        ty,
                        o + (ty[i] == 1) as usize,
                        t + (ty[i] == 2) as usize,
                        f,
                    );
                }
            }
            let mut res = vec![];
            subs(0, mu, ty, 0, 0, &mut |oo, tt| res.push((oo, tt)));
            for (oo, tt) in res {
                rec(bi + 1, blocks, slots, o + oo, t + tt, acc);
            }
        }
        rec(0, &blocks, &slots, 0, 0, &mut tot2);
        let w = pattern_weight_mod4(p, &xs, &xo, &xt);
        assert!(
            (w - 2.0 * tot2).abs() < 1e-9,
            "{p:?}: {w} vs {}",
            2.0 * tot2
        );
    }
    // reduces to twice the size weight when no odd and no type-2 points
    let pt4 = PatternTable::new(7, 4, true);
    let xs7 = vec![5u64, 3, 4, 6, 2, 5, 3];
    for p in &pt4.pats {
        let w = pattern_weight(p, &xs7);
        assert!((pattern_weight_mod4(p, &xs7, &[], &[]) - 2.0 * w).abs() < 1e-9 * (1.0 + w));
    }
}

#[test]
fn type2_only_family_has_exactly_zero_decompositions_for_odd_m_when_a_is_zero() {
    // a = 0: a block of only type-2 points (class 2 in Z/4) with m = 3 has T = 3 odd and O = 0: the class sum is 2
    // mod 4, never in 4E, so every target (all in 4E) has exactly 0 decompositions. The mod-2 parity null
    // (which sees only 'even' points) predicts a positive count here; the mod-4 null predicts exactly 0.
    let (n, m) = (11u32, 3usize);
    let f = Fq::new(n);
    let c = Curve::koblitz(0);
    let order = koblitz_order(0, n);
    let blk: Vec<u32> = (1..f.size() as u32)
        .filter(|&x| c.lift(&f, x).is_some() && f.tr(x) == 0 && type_of(&f, &c, order, x) == 2)
        .take(40)
        .collect();
    assert!(blk.len() == 40);
    let fam = family::from_blocks("t2", n, {
        let mut b = vec![blk.clone()];
        b.extend((1..n).map(|_| vec![]));
        b
    });
    let e = Enumerator::new(&f, &c, &fam, m, 1 << 24);
    let pt = PatternTable::new(n as usize, m, true);
    let xs = liftable_counts(&f, &c, &fam);
    let (xo, xt) = (odd_counts(&f, &c, &fam), two_counts(&f, &c, &fam, order));
    assert_eq!((xs[0], xo[0], xt[0]), (40, 0, 40));
    let p000 = pt.pats.iter().find(|p| p.iter().all(|&k| k == 0)).unwrap();
    assert_eq!(pattern_weight_mod4(p000, &xs, &xo, &xt), 0.0);
    assert!(
        pattern_weight_parity(p000, &xs, &xo) > 0.0,
        "the mod-2 null cannot see this zero"
    );
    let targets = make_targets(&f, &c, 0, 12, 3);
    assert!(!targets.is_empty());
    for &r in &targets {
        assert_eq!(e.count_target(r, &pt).total, 0);
    }
    // and for m = 4 (T = 4 even) the count is positive for some target
    let e4 = Enumerator::new(&f, &c, &fam, 4, 1 << 24);
    let pt4 = PatternTable::new(n as usize, 4, true);
    let any: u64 = make_targets(&f, &c, 0, 48, 3)
        .iter()
        .map(|&r| e4.count_target(r, &pt4).total)
        .sum();
    assert!(any > 0);
}

#[test]
fn z_is_calibrated_under_the_null_at_two_values_of_d() {
    // sd of the calibrated z on pure parity-aware Poisson null datasets must be within 10 percent of 1
    // (the permutation-SE z of revision 2 had sd 1.37 to 1.60). Two D values; deterministic seeds.
    for &(n, m, reps) in &[(11u32, 3usize, 400usize), (13, 4, 400)] {
        let mut zs = vec![];
        let mut d = 0;
        for rep in 0..reps {
            let (rows, pt) = synthetic_rows_o(n, m, 16, 0.0, 9000 + rep as u64, 16);
            d = pt.n_classes();
            let refs: Vec<&Row> = rows.iter().collect();
            zs.push(z_of(&split_stats(&refs, &pt, true, 77, 40, 200)));
        }
        let mean = zs.iter().sum::<f64>() / reps as f64;
        let sd = (zs.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (reps - 1) as f64).sqrt();
        let tail3 = zs.iter().filter(|&&v| v >= 3.0).count();
        println!(
            "calibration n={n} m={m} D={d}: z mean {mean:.3} sd {sd:.3}; z>=3 in {tail3}/{reps}"
        );
        assert!((sd - 1.0).abs() < 0.10, "D={d}: sd(z) = {sd}");
        assert!(mean.abs() < 0.25, "D={d}: mean z = {mean}");
        assert!(
            tail3 <= 3,
            "D={d}: {tail3}/{reps} at z >= 3 (normal: 0.13 percent)"
        );
    }
    assert!(
        rho_min(140) < 0.35 && rho_min(61) <= 0.5 && rho_min(57) > 0.5,
        "resolved/unresolved cut"
    );
    assert_eq!(MIN_D_FOR_RHO, 61);
}

fn write_cell_l(
    dir: &str,
    n: u32,
    m: usize,
    l: usize,
    spec: &[(&str, f64, u64)],
    frob_audit: i32,
    nrows: usize,
) {
    // like write_cell, with the ladder level l, the audit value of the frob arm and the rows per arm
    let mut body = String::from(HEADER);
    body.push('\n');
    for &(arm, bump, seed) in spec {
        let (rows, _) = synthetic_rows_o(
            n,
            m,
            nrows,
            bump,
            seed + 31 * n as u64 + m as u64 + 1000 * l as u64,
            8192,
        );
        for mut r in rows {
            r.arm = arm.into();
            r.l = l;
            r.audit = if arm == "frob" {
                frob_audit
            } else if arm == "skew" {
                -1
            } else {
                0
            };
            body.push_str(&r.to_tsv());
            body.push('\n');
        }
    }
    std::fs::write(format!("{dir}/rows_a1_n{n}_m{m}_l{l}.tsv"), body).unwrap();
}

fn fresh_dir(tag: &str) -> String {
    let base = std::env::var("G2_TEST_TMP")
        .unwrap_or_else(|_| std::env::temp_dir().to_string_lossy().to_string());
    let dir = format!("{base}/g2_{tag}_{}", std::process::id());
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn verdict_l(dir: &str, n: u32, m: usize, l: usize) -> String {
    let t = std::fs::read_to_string(format!("{dir}/verdicts.tsv")).unwrap();
    for line in t.lines().skip(1) {
        let c: Vec<&str> = line.split('\t').collect();
        if c[1] == n.to_string() && c[2] == m.to_string() && c[3] == l.to_string() {
            return c[6].to_string();
        }
    }
    panic!("no verdict for n={n} m={m} l={l}");
}

#[test]
fn contrast_decides_frobenius_specificity_not_the_power_dependent_zperm() {
    // D(13,5) = 476. Planted fixtures (synthetic rows, not results).
    let run = |frob: f64, perm: f64| -> (String, String) {
        let dir = fresh_dir(&format!(
            "con{}_{}",
            (frob * 1000.0) as u32,
            (perm * 1000.0) as u32
        ));
        write_cell_l(
            &dir,
            13,
            5,
            3,
            &[
                ("frob", frob, 1),
                ("frobperm", perm, 2),
                ("rsub", 0.0, 3),
                ("rset", 0.0, 4),
                ("skew", 0.25, 5),
            ],
            1,
            16,
        );
        g2_orbit_union_dnm::report::analyze_dir(&dir);
        let v = verdict_l(&dir, 13, 5, 3);
        let line = std::fs::read_to_string(format!("{dir}/verdicts.tsv"))
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .to_string();
        std::fs::remove_dir_all(&dir).unwrap();
        (v, line)
    };
    // frob far above frobperm, BOTH above z = 4: the rev-2 rule called this NUISANCE_REPLICABLE; the contrast says frob-specific
    let (v, line) = run(0.10, 0.04);
    println!("{line}");
    assert!(v == "CONCENTRATED_WEAK" || v == "CONCENTRATED", "{v}");
    // equal shared excess: NUISANCE
    assert_eq!(run(0.10, 0.10).0, "NUISANCE_REPLICABLE");
    // frob replicable, frobperm just below the flag but not significantly lower: specificity NOT resolved
    // (rev-2 rule would have said CONCENTRATED_WEAK on z(frobperm) < 4 alone)
    assert_eq!(run(0.04, 0.01).0, "REPLICABLE_NOT_SEPARATED"); // z(frob) 9.1, z(frobperm) 3.75, contrast 3.4
                                                               // nothing planted
    assert_eq!(run(0.0, 0.0).0, "UNIFORM");
}

#[test]
fn instrument_fail_on_gauge_audit_zero_missed_skew_and_incomplete_cells() {
    // audit = 0 on frob
    let dir = fresh_dir("aud");
    write_cell_l(
        &dir,
        13,
        5,
        3,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.25, 5),
        ],
        0,
        16,
    );
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    assert_eq!(verdict_l(&dir, 13, 5, 3), "INSTRUMENT_FAIL");
    std::fs::remove_dir_all(&dir).unwrap();
    // positive control (skew) not planted -> not detected -> INSTRUMENT_FAIL
    let dir = fresh_dir("skw");
    write_cell_l(
        &dir,
        13,
        5,
        3,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.0, 5),
        ],
        1,
        16,
    );
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    assert_eq!(verdict_l(&dir, 13, 5, 3), "INSTRUMENT_FAIL");
    std::fs::remove_dir_all(&dir).unwrap();
    // 12 rows per arm instead of NV = 16 -> INCOMPLETE; and a truncated last line does not abort the analysis
    let dir = fresh_dir("inc");
    write_cell_l(
        &dir,
        13,
        5,
        3,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.25, 5),
        ],
        1,
        12,
    );
    let path = format!("{dir}/rows_a1_n13_m5_l3.tsv");
    let mut body = std::fs::read_to_string(&path).unwrap();
    body.push_str("1\t13\t5\t3\tfrob\t99\t4"); // killed mid-line
    std::fs::write(&path, body).unwrap();
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    assert_eq!(verdict_l(&dir, 13, 5, 3), "INCOMPLETE");
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    assert!(md.contains("truncated or malformed"));
    assert!(md.contains("no m = 83 evidence") || md.contains("NO m = 83 evidence"));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn ladder_persistence_is_coded() {
    let (n, m) = (13u32, 4usize);
    let dir = fresh_dir("lad");
    // l = 3 and l = 5 both carry the same frob-only excess -> PERSISTENT
    for l in [3usize, 5] {
        write_cell_l(
            &dir,
            n,
            m,
            l,
            &[
                ("frob", 1.5, 1),
                ("frobperm", 0.0, 2),
                ("rsub", 0.0, 3),
                ("rset", 0.0, 4),
                ("skew", 0.6, 5),
            ],
            1,
            16,
        );
    }
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    assert!(md.contains("ladder PERSISTENT"), "{md}");
    std::fs::remove_dir_all(&dir).unwrap();
    // excess at l = 3 only -> FALLING_OR_ABSENT
    let dir = fresh_dir("lad2");
    write_cell_l(
        &dir,
        n,
        m,
        3,
        &[
            ("frob", 1.5, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.6, 5),
        ],
        1,
        16,
    );
    write_cell_l(
        &dir,
        n,
        m,
        5,
        &[
            ("frob", 0.0, 1),
            ("frobperm", 0.0, 2),
            ("rsub", 0.0, 3),
            ("rset", 0.0, 4),
            ("skew", 0.6, 5),
        ],
        1,
        16,
    );
    g2_orbit_union_dnm::report::analyze_dir(&dir);
    let md = std::fs::read_to_string(format!("{dir}/summary.md")).unwrap();
    assert!(md.contains("ladder FALLING_OR_ABSENT"), "{md}");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn ladder_l5_instrument_failures_are_not_evaluable_not_falling() {
    use g2_orbit_union_dnm::report::ladder_status;
    let mk = |v5: &str, e5: f64| {
        vec![
            (3usize, Some(0.39), "CONCENTRATED".to_string()),
            (5usize, Some(e5), v5.to_string()),
        ]
    };
    // same excess at l = 5, but the verdict is a failure of the instrument or of specificity resolution
    for bad in [
        "INSTRUMENT_FAIL",
        "NUISANCE_REPLICABLE",
        "REPLICABLE_NOT_SEPARATED",
        "INCOMPLETE",
        "NO_DATA",
        "CONCENTRATED_VS_CONTROLS",
    ] {
        assert!(
            ladder_status(&mk(bad, 0.3945)).starts_with("NOT_EVALUABLE"),
            "{bad}"
        );
    }
    // readable l = 5 verdicts keep their meaning
    assert_eq!(ladder_status(&mk("CONCENTRATED", 0.3945)), "PERSISTENT");
    assert_eq!(
        ladder_status(&mk("CONCENTRATED_WEAK", 0.3945)),
        "PERSISTENT"
    );
    assert_eq!(
        ladder_status(&mk("CONCENTRATED", 0.05)),
        "FALLING_OR_ABSENT"
    );
    assert_eq!(ladder_status(&mk("UNIFORM", 0.0)), "FALLING_OR_ABSENT");
    assert_eq!(ladder_status(&mk("UNRESOLVED", 0.0)), "FALLING_OR_ABSENT");
}

#[test]
fn specificity_saturation_threshold_is_an_integer_count_comparison() {
    use g2_orbit_union_dnm::report::{specificity_resolvable, SATURATION_COUNT};
    assert_eq!(SATURATION_COUNT, 10_000);
    let d = 140usize;
    // total yield giving exactly 9_999 and 10_000 per class per half
    assert!(specificity_resolvable(
        9_999.0 * 2.0 * d as f64 + 2.0 * d as f64 - 1.0,
        d
    )); // floor = 9_999
    assert!(!specificity_resolvable(10_000.0 * 2.0 * d as f64, d));
    assert!(!specificity_resolvable(3.3e5 * 2.0 * d as f64, d));
    assert!(specificity_resolvable(816.0 * 2.0 * d as f64, d));
}
