use g4_iota_stable::bases::*;
use g4_iota_stable::curve::*;
use g4_iota_stable::enumr::*;
use g4_iota_stable::field::Fq;
use g4_iota_stable::run::*;
use std::collections::HashSet;

fn setup(n: u32, a: u32) -> (Fq, Group) {
    let f = Fq::new(n);
    let g = build_group(&f, a).expect("group");
    (f, g)
}

fn minlog(g: &Group, k: u32) -> u32 {
    let k = k % g.ne;
    k.min(g.ne - k)
}

#[test]
fn group_and_point_arithmetic_are_consistent() {
    for (n, a) in [(7, 0), (11, 1), (13, 0), (13, 1)] {
        let (f, g) = setup(n, a);
        let c = Curve { f: &f, a };
        let bound = 2.0 * (f.q as f64).sqrt() + 1.0;
        assert!(((g.ne as f64) - (f.q as f64) - 1.0).abs() <= bound, "Hasse n={n} a={a}");
        assert_eq!(g.ne % (2 * (1 + (a == 0) as u32)), 0, "cofactor 4 for a=0, 2 for a=1");
        assert_eq!(g.logx[0], g.m, "T = (0,1) has log ne/2");
        assert_eq!(c.add(Pt::Aff(0, 1), Pt::Aff(0, 1)), Pt::Inf);
        // log table agrees with the group law on a sample of pairs
        let xs: Vec<u32> = (2..f.q).filter(|&x| g.logx[x as usize] != NONE).step_by(37).take(60).collect();
        for &x1 in &xs {
            for &x2 in xs.iter().step_by(7) {
                let (y1, y2) = (c.lift_x(x1).unwrap(), c.lift_x(x2).unwrap());
                let (p1, p2) = (Pt::Aff(x1, y1), Pt::Aff(x2, y2));
                assert!(c.on_curve(p1) && c.on_curve(p2));
                let s = c.add(p1, p2);
                let (l1, l2) = (g.logx[x1 as usize], g.logx[x2 as usize]);
                let cand = [minlog(&g, l1 + l2), minlog(&g, l1 + g.ne - l2)];
                match s {
                    Pt::Inf => assert!(x1 == x2),
                    Pt::Aff(xs_, _) => {
                        assert!(c.on_curve(s));
                        assert!(cand.contains(&g.logx[xs_ as usize]), "n={n} a={a}");
                    }
                }
            }
        }
    }
}

#[test]
fn translation_by_t_acts_on_x_as_inversion() {
    for (n, a) in [(7, 0), (11, 1), (13, 0), (13, 1), (17, 0)] {
        let (f, g) = setup(n, a);
        let c = Curve { f: &f, a };
        let t = Pt::Aff(0, 1);
        for x in 1..f.q {
            if let Some(y) = c.lift_x(x) {
                let q = Pt::Aff(x, y);
                match c.add(q, t) {
                    Pt::Aff(x2, _) => assert_eq!(x2, f.inv(x), "x(Q+T) = 1/x"),
                    Pt::Inf => panic!("Q+T = O"),
                }
                // exists-point symmetry: x in image iff 1/x in image
                assert!(g.logx[f.inv(x) as usize] != NONE);
                // log level: x(l + ne/2) has x = 1/x
                let l = g.logx[x as usize];
                assert_eq!(g.logx[f.inv(x) as usize], minlog(&g, l + g.m));
            }
        }
    }
}

#[test]
fn base_constructions_have_the_claimed_stability() {
    let (f, g) = setup(13, 0);
    let mut rng = Rng(5);
    let w = span(&random_basis(f.n, 6, &mut rng));
    assert_eq!(w.len(), 64);
    let stable = core_set(&involution_union(&f, &w, 1));
    assert!(is_iota_stable(&f, &stable));
    assert_eq!(stable.len() % 2, 0);
    let bs = build_base(&g, &stable);
    assert_eq!(bs.closure, 1.0);
    let wrong = core_set(&involution_union(&f, &w, 77));
    assert!(!is_iota_stable(&f, &wrong));
    assert!(build_base(&g, &wrong).closure < 0.5);
    let v = core_set(&span(&random_basis(f.n, 6, &mut rng)));
    assert!(build_base(&g, &v).closure < 0.5);
    assert!(is_iota_stable(&f, &random_iota_set(&f, 20, &mut rng)));
    assert!(!is_iota_stable(&f, &random_set(f.q, 40, &mut rng)));
    // size matching of the random controls
    assert_eq!(random_set(f.q, 40, &mut rng).len(), 40);
    assert_eq!(random_iota_set(&f, 20, &mut rng).len(), 40);
}

fn class_of(g: &Group, ell: u32) -> (u32, u32) {
    let lam = ell % g.m;
    let neg = g.m - lam;
    (lam.min(neg), (lam != lam.min(neg)) as u32)
}

#[test]
fn orbit_structure_identity_on_stable_bases_all_targets() {
    for (n, a, l) in [(7u32, 0u32, 4u32), (11, 1, 5), (13, 0, 5), (13, 1, 5)] {
        let (f, g) = setup(n, a);
        let mut rng = Rng(100 + n as u64 + a as u64);
        let w = span(&random_basis(f.n, l - 1, &mut rng));
        let base = build_base(&g, &core_set(&involution_union(&f, &w, 1)));
        assert_eq!(base.closure, 1.0);
        for m in 2..=5usize {
            if binom(base.items.len(), m) > 3e6 {
                continue;
            }
            let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
            // D[s] = 2^(m-1) O[s] at EVERY target: all orbits have full size
            assert!(identity_exact(&t, m), "n={n} a={a} m={m}");
            if base.ncls >= m + 1 {
                assert!(t.total > 0, "fixture must contain decompositions");
            }
            // quotient search (in E/<T>) equals the orbit count at both s and s+T
            let q = quot_counts(&base, m);
            for s in 0..g.ne as usize {
                assert_eq!(q[s % g.m as usize], t.o[s], "quot n={n} a={a} m={m} s={s}");
            }
            // the orbit rows satisfy the logs: sum of sign*class-rep == s (mod ne/2)
            let mut cls = vec![0u32; base.ncls];
            for it in &base.items {
                cls[it.cid as usize] = class_of(&g, it.ell).0;
            }
            for s in 0..g.ne as usize {
                for k in t.off[s]..t.off[s + 1] {
                    let key = t.keys[k as usize];
                    let mut acc: i64 = 0;
                    let mut seen = HashSet::new();
                    for i in 0..m {
                        let sc = ((key >> (12 * i)) & 0xFFF) as usize;
                        assert!(seen.insert(sc >> 1), "classes must be distinct");
                        let v = cls[sc >> 1] as i64;
                        acc += if sc & 1 == 0 { v } else { -v };
                    }
                    assert_eq!(acc.rem_euclid(g.m as i64), (s as u32 % g.m) as i64, "row/log mismatch");
                }
            }
        }
    }
}

#[test]
fn non_stable_bases_have_trivial_orbits() {
    let (f, g) = setup(13, 0);
    let mut rng = Rng(9);
    let v = core_set(&span(&random_basis(f.n, 6, &mut rng)));
    let wrong = core_set(&involution_union(&f, &span(&random_basis(f.n, 5, &mut rng)), 91));
    for xs in [v, wrong] {
        let base = build_base(&g, &xs);
        for m in 2..=3usize {
            let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
            assert!(!identity_exact(&t, m));
            let ratio = t.total as f64 / t.o.iter().map(|&x| x as f64).sum::<f64>();
            assert!(ratio < 1.2, "collapse {ratio} should be ~1 on a non-stable base");
        }
    }
}

#[test]
fn enumerator_matches_naive_brute_force() {
    let (f, g) = setup(13, 0);
    let mut rng = Rng(3);
    let w = span(&random_basis(f.n, 3, &mut rng));
    let mut xs = core_set(&involution_union(&f, &w, 1));
    xs.truncate(xs.len().min(30));
    // truncated set is no longer stable: the brute force must still agree for any set
    let base = build_base(&g, &xs);
    let pts: Vec<(u32, u32, u32, u32)> = base.items.iter().map(|i| (i.ell, class_of(&g, i.ell).0, class_of(&g, i.ell).1, i.x)).collect();
    let nit = pts.len();
    assert!(nit <= 24, "fixture too large for brute force: {nit}");
    for m in 2..=4usize {
        let mut d = vec![0u32; g.ne as usize];
        let mut orbits: HashSet<(u32, Vec<(u32, u32)>)> = HashSet::new();
        for mask in 0u32..(1 << nit) {
            if mask.count_ones() as usize != m {
                continue;
            }
            let idx: Vec<usize> = (0..nit).filter(|i| mask >> i & 1 == 1).collect();
            let mut cl: Vec<u32> = idx.iter().map(|&i| pts[i].1).collect();
            cl.sort_unstable();
            cl.dedup();
            if cl.len() != m {
                continue;
            }
            let s = idx.iter().map(|&i| pts[i].0 as u64).sum::<u64>() % g.ne as u64;
            d[s as usize] += 1;
            let mut key: Vec<(u32, u32)> = idx.iter().map(|&i| (pts[i].1, pts[i].2)).collect();
            key.sort_unstable();
            orbits.insert((s as u32, key));
        }
        let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
        assert_eq!(t.d, d, "D mismatch m={m}");
        assert_eq!(t.o.iter().map(|&x| x as usize).sum::<usize>(), orbits.len(), "orbit count m={m}");
    }
}

#[test]
fn planted_decomposition_has_a_full_translation_orbit() {
    let (f, g) = setup(13, 0);
    let mut rng = Rng(77);
    let w = span(&random_basis(f.n, 5, &mut rng));
    let base = build_base(&g, &core_set(&involution_union(&f, &w, 1)));
    let ells: HashSet<u32> = base.items.iter().map(|i| i.ell).collect();
    let m = 4usize;
    // plant: choose m items in distinct classes, target s = their sum
    let mut chosen: Vec<Item> = vec![];
    while chosen.len() < m {
        let it = base.items[rng.below(base.items.len() as u64) as usize];
        if chosen.iter().all(|c| c.cid != it.cid) {
            chosen.push(it);
        }
    }
    let s = chosen.iter().map(|c| c.ell as u64).sum::<u64>() % g.ne as u64;
    let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
    // all 2^(m-1) even-weight translates are factor-base points with the same sum
    let mut found = HashSet::new();
    for u in 0u32..(1 << m) {
        if u.count_ones() % 2 != 0 {
            continue;
        }
        let tr: Vec<u32> = chosen.iter().enumerate().map(|(i, c)| if u >> i & 1 == 1 { (c.ell + g.m) % g.ne } else { c.ell }).collect();
        assert!(tr.iter().all(|e| ells.contains(e)), "translate leaves the stable base");
        assert_eq!(tr.iter().map(|&e| e as u64).sum::<u64>() % g.ne as u64, s, "translate changes the sum");
        let mut sorted = tr.clone();
        sorted.sort_unstable();
        found.insert(sorted);
    }
    assert_eq!(found.len(), 1 << (m - 1), "orbit size is 2^(m-1)");
    assert!(t.d[s as usize] as usize >= 1 << (m - 1));
    assert_eq!(t.d[s as usize], t.o[s as usize] << (m - 1));
    // an odd-weight translate changes the sum by T (leaves the target)
    let odd: u64 = chosen.iter().enumerate().map(|(i, c)| if i == 0 { (c.ell + g.m) % g.ne } else { c.ell } as u64).sum::<u64>() % g.ne as u64;
    assert_eq!(odd, (s + g.m as u64) % g.ne as u64);
}

#[test]
fn rank_elimination_and_planted_system() {
    let p = 101u64;
    let mut e = Elim::new(p, 5);
    assert!(e.add(&[(0, 1), (1, 1)]));
    assert!(e.add(&[(1, 1), (2, 1)]));
    assert!(!e.add(&[(0, 1), (2, p - 1)])); // (e0+e1) - (e1+e2) = e0 - e2: dependent
    assert!(e.add(&[(3, 1), (4, p - 1)]));
    assert_eq!(e.rank, 3);
    // planted unknowns: rank_stream on a real stable base reaches rank = #active classes
    let (f, g) = setup(13, 0);
    let mut rng = Rng(8);
    let w = span(&random_basis(f.n, 5, &mut rng));
    let base = build_base(&g, &core_set(&involution_union(&f, &w, 1)));
    let m = 3usize;
    let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
    let order: Vec<u32> = (0..g.ne).step_by(g.h as usize).collect();
    let r = rank_stream(&t, &order, m, base.ncls, largest_prime_factor(g.ne) as u64);
    assert!(r.active > 0 && r.rank_final <= r.active);
    assert!(r.t90.is_some(), "dense toy base should reach 90% rank: {r:?}");
}

#[test]
fn anf_degree_of_subspace_and_point() {
    let n = 9u32;
    let v = poly_window(4);
    let (d, _) = anf(n, &v);
    assert_eq!(d, n - 4, "indicator of a dim-4 subspace has degree n-4");
    let mut rng = Rng(2);
    let v2 = span(&random_basis(n, 5, &mut rng));
    assert_eq!(anf(n, &v2).0, n - 5);
    assert_eq!(anf(n, &[37]).0, n, "a point has degree n");
    assert_eq!(anf(n, &(0..512).filter(|x| x & 1 == 1).collect::<Vec<u32>>()), (1, 1));
}

#[test]
fn measure_smoke_on_a_tiny_fixture() {
    let (f, g) = setup(13, 0);
    let raw = make_arm("g4poly", &f, 5, 1, 0);
    let r: std::collections::HashMap<String, String> = measure(&g, &f, &raw, 5, 3, 5).unwrap().into_iter().collect();
    assert_eq!(r["ident_ok"], "1");
    assert_eq!(r["quot_ok"], "1");
    assert_eq!(r["closure"].parse::<f64>().unwrap(), 1.0);
    assert!((r["collapse"].parse::<f64>().unwrap() - 4.0).abs() < 1e-12);
    let rv: std::collections::HashMap<String, String> = measure(&g, &f, &make_arm("vpoly", &f, 5, 1, 0), 5, 3, 5).unwrap().into_iter().collect();
    assert_eq!(rv["ident_ok"], "0");
    assert!(rv["collapse"].parse::<f64>().unwrap() < 1.1);
    // membership degree: subspace n-l (+0), union is higher
    assert_eq!(rv["deg"], "8");
    assert!(r["deg"].parse::<u32>().unwrap() > 8);
    // parametrisation degree: linear for the subspace, higher for the inversion union
    assert_eq!(rv["param_deg"], "1");
    assert!(r["param_deg"].parse::<u32>().unwrap() >= 2);
}

#[test]
fn plan_is_within_enumeration_cap_and_seeds_are_distinct() {
    for c in cell_list() {
        let nominal = binom((1usize << c.l) - 2, c.m);
        assert!(nominal <= ENUM_CAP, "cell {c:?} nominal {nominal:.3e} over cap");
    }
    let s: HashSet<u64> = (0..1000).map(|i| derive_seed(1, &[i, 3])).collect();
    assert_eq!(s.len(), 1000);
    assert_ne!(derive_seed(1, &[1, 2]), derive_seed(2, &[1, 2]));
    // arms are drawn with distinct derived seeds
    assert_eq!(arm_id("vrand") != arm_id("g4rand"), true);
}

#[test]
fn planned_fields_have_cyclic_groups() {
    // Fixture for the plan: every (n, a) used by cell_list must give a cyclic E(F_q).
    let mut bad = vec![];
    for n in [7u32, 11, 13, 17, 19] {
        let f = Fq::new(n);
        for a in [0u32, 1] {
            let ne = Curve { f: &f, a }.count();
            let ok = build_group(&f, a).is_ok();
            println!("n={n} a={a} ne={ne} lpf={} cyclic={ok}", largest_prime_factor(ne));
            if !ok {
                bad.push((n, a));
            }
        }
    }
    println!("non-cyclic: {bad:?}");
    let planned: HashSet<(u32, u32)> = cell_list().iter().map(|c| (c.n, c.a)).collect();
    for b in bad {
        assert!(!planned.contains(&b), "planned field {b:?} is not cyclic");
    }
}

#[test]
fn pipeline_plumbing_on_a_tiny_fixture() {
    // Plumbing only (3 bases per half, one tiny cell): rows are written, parsed and analysed; the
    // output is discarded. It checks formats and the instrument/control status logic, not any result.
    let dir = std::env::temp_dir().join(format!("g4iota-plumbing-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let d = dir.to_string_lossy().to_string();
    let c = Cell { a: 0, n: 13, m: 3, l: 5 };
    let o = RunOpts { seed: 1, reps: Some(3), out: d.clone() };
    let p = run_cell(&c, &o).expect("run_cell");
    let txt = std::fs::read_to_string(&p).unwrap();
    let hdr: Vec<&str> = txt.lines().next().unwrap().split('\t').collect();
    assert!(txt.lines().all(|l| l.split('\t').count() == hdr.len()), "ragged rows");
    // 2 single arms once + 5 replicated arms x (3 main + 3 aa)
    assert_eq!(txt.lines().count() - 1, 2 + 5 * 6);
    g4_iota_stable::analyze::analyze(&d).expect("analyze");
    let md = std::fs::read_to_string(dir.join("summary.md")).unwrap();
    assert!(md.contains("| ok |"), "instrument/control status should be ok:\n{md}");
    assert!(dir.join("fig_cost_ratio.svg").exists() && dir.join("summary.tsv").exists());
    std::fs::remove_dir_all(&dir).unwrap();
}
