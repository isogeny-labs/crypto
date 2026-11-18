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
        assert!(
            ((g.ne as f64) - (f.q as f64) - 1.0).abs() <= bound,
            "Hasse n={n} a={a}"
        );
        assert_eq!(
            g.ne % (2 * (1 + (a == 0) as u32)),
            0,
            "cofactor 4 for a=0, 2 for a=1"
        );
        assert_eq!(g.logx[0], g.m, "T = (0,1) has log ne/2");
        assert_eq!(c.add(Pt::Aff(0, 1), Pt::Aff(0, 1)), Pt::Inf);
        // log table agrees with the group law on a sample of pairs
        let xs: Vec<u32> = (2..f.q)
            .filter(|&x| g.logx[x as usize] != NONE)
            .step_by(37)
            .take(60)
            .collect();
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
                    assert_eq!(
                        acc.rem_euclid(g.m as i64),
                        (s as u32 % g.m) as i64,
                        "row/log mismatch"
                    );
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
    let wrong = core_set(&involution_union(
        &f,
        &span(&random_basis(f.n, 5, &mut rng)),
        91,
    ));
    for xs in [v, wrong] {
        let base = build_base(&g, &xs);
        for m in 2..=3usize {
            let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
            assert!(!identity_exact(&t, m));
            let ratio = t.total as f64 / t.o.iter().map(|&x| x as f64).sum::<f64>();
            assert!(
                ratio < 1.2,
                "collapse {ratio} should be ~1 on a non-stable base"
            );
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
    let pts: Vec<(u32, u32, u32, u32)> = base
        .items
        .iter()
        .map(|i| (i.ell, class_of(&g, i.ell).0, class_of(&g, i.ell).1, i.x))
        .collect();
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
        assert_eq!(
            t.o.iter().map(|&x| x as usize).sum::<usize>(),
            orbits.len(),
            "orbit count m={m}"
        );
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
        let tr: Vec<u32> = chosen
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if u >> i & 1 == 1 {
                    (c.ell + g.m) % g.ne
                } else {
                    c.ell
                }
            })
            .collect();
        assert!(
            tr.iter().all(|e| ells.contains(e)),
            "translate leaves the stable base"
        );
        assert_eq!(
            tr.iter().map(|&e| e as u64).sum::<u64>() % g.ne as u64,
            s,
            "translate changes the sum"
        );
        let mut sorted = tr.clone();
        sorted.sort_unstable();
        found.insert(sorted);
    }
    assert_eq!(found.len(), 1 << (m - 1), "orbit size is 2^(m-1)");
    assert!(t.d[s as usize] as usize >= 1 << (m - 1));
    assert_eq!(t.d[s as usize], t.o[s as usize] << (m - 1));
    // an odd-weight translate changes the sum by T (leaves the target)
    let odd: u64 = chosen
        .iter()
        .enumerate()
        .map(|(i, c)| if i == 0 { (c.ell + g.m) % g.ne } else { c.ell } as u64)
        .sum::<u64>()
        % g.ne as u64;
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
    let r = rank_stream(
        &t,
        &order,
        m,
        base.ncls,
        largest_prime_factor(g.ne) as u64,
        4,
    );
    assert!(r.active > 0 && r.rank_final <= r.active);
    assert!(
        r.t90.is_some(),
        "dense toy base should reach 90% rank: {r:?}"
    );
    // fractional targets: within (t90 - 1, t90], and the rows used so far never exceed rows90
    let tf = r.t90f.unwrap();
    let t90 = r.t90.unwrap() as f64;
    assert!(tf > t90 - 1.0 && tf <= t90, "t90f {tf} vs t90 {t90}");
    // same stream, same seed: deterministic
    let r2 = rank_stream(
        &t,
        &order,
        m,
        base.ncls,
        largest_prime_factor(g.ne) as u64,
        4,
    );
    assert_eq!(
        (r.t90f, r.rows90, r.rank_final),
        (r2.t90f, r2.rows90, r2.rank_final)
    );
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
    assert_eq!(
        anf(n, &(0..512).filter(|x| x & 1 == 1).collect::<Vec<u32>>()),
        (1, 1)
    );
}

#[test]
fn measure_smoke_on_a_tiny_fixture() {
    let (f, g) = setup(13, 0);
    let raw = make_arm("g4poly", &f, 5, 1, 0);
    let r: std::collections::HashMap<String, String> = measure(&g, &f, &raw, 5, 3, 5)
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(r["ident_ok"], "1");
    assert_eq!(r["quot_ok"], "1");
    assert_eq!(r["closure"].parse::<f64>().unwrap(), 1.0);
    assert!((r["collapse"].parse::<f64>().unwrap() - 4.0).abs() < 1e-12);
    let rv: std::collections::HashMap<String, String> =
        measure(&g, &f, &make_arm("vpoly", &f, 5, 1, 0), 5, 3, 5)
            .unwrap()
            .into_iter()
            .collect();
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
fn seeds_are_distinct_and_cap_logic_is_sound() {
    let s: HashSet<u64> = (0..1000).map(|i| derive_seed(1, &[i, 3])).collect();
    assert_eq!(s.len(), 1000);
    assert_ne!(derive_seed(1, &[1, 2]), derive_seed(2, &[1, 2]));
    assert_ne!(arm_id("vrand"), arm_id("g4rand"));
    // Every matched arm has exactly N = items(g4rand) items, and a g4rand draw is accepted only if
    // C(N, m) <= ENUM_CAP, so the cap can never abort a matched replicate. The only risk is that the
    // acceptance rule truncates N too hard (model: g4rand items = 4 Bin(2^(l-1)-1, 1/2), carriers p = 0.5).
    for c in cell_list() {
        let pairs = (1usize << (c.l - 1)) - 1;
        let mut pmf = 0.5f64.powi(pairs as i32);
        let mut p_ok = 0.0;
        for b in 0..=pairs {
            let n = 4 * b;
            // a stable base has 4 items per class, so ncls = b carriers' classes must be >= m (amendment A1)
            if b >= c.m && binom(n, c.m) <= ENUM_CAP {
                p_ok += pmf;
            }
            pmf *= (pairs - b) as f64 / (b + 1) as f64;
        }
        assert!(
            p_ok > 0.5,
            "cell {c:?}: only {p_ok:.3} of g4rand draws are accepted (truncation too strong)"
        );
        // the unmatched single instances are skipped (not an error) above the cap, so nothing else can abort
        assert!(binom(4 * pairs, c.m).is_finite());
    }
}

#[test]
fn matched_replicates_have_exactly_equal_item_counts() {
    for (a, n, m, l) in [
        (0u32, 13u32, 3usize, 5u32),
        (0, 13, 4, 5),
        (1, 11, 5, 5),
        (0, 13, 2, 7),
    ] {
        let (f, g) = setup(n, a);
        let c = Cell { a, n, m, l };
        for rep in 0..6u64 {
            let mt = draw_matched(&f, &g, &c, 20261027, rep).expect("matched draw");
            assert!(mt.n_items >= 4 * m && binom(mt.n_items, m) <= ENUM_CAP);
            assert_eq!(mt.arms.len(), 5);
            for (arm, b, _seed, _tries) in &mt.arms {
                assert!(
                    shape_of(&g, &b.raw).1 >= m,
                    "{arm} has fewer than m classes"
                );
                assert_eq!(
                    items_of(&g, &b.raw),
                    mt.n_items,
                    "{arm} not matched (a{a} n{n} m{m} l{l} rep {rep})"
                );
                let stable = is_iota_stable(&f, &core_set(&b.raw));
                match *arm {
                    "g4rand" | "riota" => assert!(stable, "{arm} must be iota-stable"),
                    "vrand" | "wrong" => assert!(!stable, "{arm} must not be iota-stable"),
                    _ => {}
                }
            }
            // deterministic
            let mt2 = draw_matched(&f, &g, &c, 20261027, rep).unwrap();
            assert_eq!(mt.n_items, mt2.n_items);
            assert_eq!(
                mt.arms.iter().map(|x| x.2).collect::<Vec<_>>(),
                mt2.arms.iter().map(|x| x.2).collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn counted_mitm_finds_exactly_the_decompositions_and_matches_the_cost_model() {
    // An honest meet-in-the-middle with counted operations, in both table orientations, on tiny bases
    // (one stable, one not): it finds exactly D[s] decompositions for every target, and its table / probe
    // counts equal binom(N, a), binom(N, m-a), the quantities the cost model charges.
    use std::collections::HashMap;
    let (f, g) = setup(13, 0);
    let mut rng = Rng(11);
    let w = span(&random_basis(f.n, 3, &mut rng));
    let sets = [
        core_set(&involution_union(&f, &w, 1)),
        core_set(&involution_union(&f, &w, 77)),
    ];
    for xs in sets {
        let base = build_base(&g, &xs);
        let nit = base.items.len();
        assert!(nit <= 26, "fixture too large: {nit}");
        for m in [3usize, 4] {
            let t = tally(g.ne, enumerate(&base, m, 1e7).unwrap());
            for a in [m / 2, m - m / 2] {
                // all subsets of size k as index lists, counted
                fn subsets(n: usize, k: usize) -> Vec<Vec<usize>> {
                    let mut out = vec![];
                    let mut cur = vec![];
                    fn go(
                        n: usize,
                        k: usize,
                        start: usize,
                        cur: &mut Vec<usize>,
                        out: &mut Vec<Vec<usize>>,
                    ) {
                        if cur.len() == k {
                            out.push(cur.clone());
                            return;
                        }
                        for i in start..n {
                            cur.push(i);
                            go(n, k, i + 1, cur, out);
                            cur.pop();
                        }
                    }
                    go(n, k, 0, &mut cur, &mut out);
                    out
                }
                let left = subsets(nit, a);
                let right = subsets(nit, m - a);
                assert_eq!(left.len() as f64, binom(nit, a), "table size");
                assert_eq!(right.len() as f64, binom(nit, m - a), "probe size");
                let cls = |idx: &[usize]| -> Option<HashSet<u32>> {
                    let h: HashSet<u32> = idx.iter().map(|&i| base.items[i].cid).collect();
                    if h.len() == idx.len() {
                        Some(h)
                    } else {
                        None
                    }
                };
                let mut table: HashMap<u32, Vec<&Vec<usize>>> = HashMap::new();
                for l in &left {
                    if cls(l).is_some() {
                        let s =
                            l.iter().map(|&i| base.items[i].ell as u64).sum::<u64>() % g.ne as u64;
                        table.entry(s as u32).or_default().push(l);
                    }
                }
                let targets: Vec<usize> = (0..g.ne as usize)
                    .filter(|&s| t.d[s] > 0)
                    .step_by(37)
                    .take(12)
                    .chain((0..g.ne as usize).filter(|&s| t.d[s] == 0).take(3))
                    .collect();
                for s in targets {
                    let mut found: HashSet<Vec<usize>> = HashSet::new();
                    let mut probes = 0u64;
                    for r in &right {
                        probes += 1;
                        let Some(rc) = cls(r) else { continue };
                        let rs =
                            r.iter().map(|&i| base.items[i].ell as u64).sum::<u64>() % g.ne as u64;
                        let need = ((s as u64 + g.ne as u64 - rs) % g.ne as u64) as u32;
                        if let Some(ls) = table.get(&need) {
                            for l in ls {
                                if l.iter().all(|&i| !rc.contains(&base.items[i].cid)) {
                                    let mut u: Vec<usize> =
                                        l.iter().chain(r.iter()).copied().collect();
                                    u.sort_unstable();
                                    u.dedup();
                                    if u.len() == m {
                                        found.insert(u);
                                    }
                                }
                            }
                        }
                    }
                    assert_eq!(probes as f64, binom(nit, m - a));
                    assert_eq!(
                        found.len() as u32,
                        t.d[s],
                        "MITM finds exactly D[s]: s={s} m={m} a={a}"
                    );
                }
            }
            // the cost model picks the cheaper orientation for the given number of targets
            for tt in [1.0f64, 5.0, 50.0, 500.0] {
                let c = mitm_total(nit, m, tt);
                let (lo, hi) = (binom(nit, m / 2), binom(nit, m - m / 2));
                let best = (lo + tt * hi).min(hi + tt * lo);
                assert!((c.table + tt * c.probe - best).abs() < 1e-6 * best.max(1.0));
            }
        }
    }
}

#[test]
fn orbit_members_expand_to_points_that_sum_to_the_target() {
    // Certificate discipline: not only logs. Every one of the 2^(m-1) members of an orbit is expanded to
    // curve points with Curve::mul / Curve::add and the point sum is compared with [s]g.
    let (f, g) = setup(13, 0);
    let c = Curve { f: &f, a: 0 };
    let mut rng = Rng(31);
    let w = span(&random_basis(f.n, 5, &mut rng));
    let base = build_base(&g, &core_set(&involution_union(&f, &w, 1)));
    let by_ell: std::collections::HashMap<u32, u32> =
        base.items.iter().map(|i| (i.ell, i.x)).collect();
    for m in [2usize, 3, 4] {
        for _ in 0..25 {
            let mut chosen: Vec<Item> = vec![];
            while chosen.len() < m {
                let it = base.items[rng.below(base.items.len() as u64) as usize];
                if chosen.iter().all(|x| x.cid != it.cid) {
                    chosen.push(it);
                }
            }
            let s = chosen.iter().map(|x| x.ell as u64).sum::<u64>() % g.ne as u64;
            let target = c.mul(g.gen, s);
            let mut members = HashSet::new();
            for u in 0u32..(1 << m) {
                if u.count_ones() % 2 != 0 {
                    continue;
                }
                let ells: Vec<u32> = chosen
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if u >> i & 1 == 1 {
                            (x.ell + g.m) % g.ne
                        } else {
                            x.ell
                        }
                    })
                    .collect();
                let mut acc = Pt::Inf;
                for &e in &ells {
                    let p = c.mul(g.gen, e as u64);
                    assert!(c.on_curve(p));
                    match p {
                        Pt::Aff(x, _) => assert_eq!(by_ell[&e], x, "item x must equal x([ell] g)"),
                        Pt::Inf => panic!("O is not a factor-base point"),
                    }
                    acc = c.add(acc, p);
                }
                assert_eq!(acc, target, "orbit member does not sum to [s]g (m={m})");
                let mut sorted = ells.clone();
                sorted.sort_unstable();
                members.insert(sorted);
            }
            assert_eq!(members.len(), 1 << (m - 1));
        }
    }
}

#[test]
fn pipeline_cost_ratio_has_a_valid_null_and_a_planted_effect_through_the_same_code() {
    // The cost pipeline (measure -> rows -> judge_cell) is run on real bases only for the instrument
    // identities here; the effect and null behaviour of the DECISION code is tested on planted rows below.
    let (f, g) = setup(13, 0);
    let b = make_arm("g4poly", &f, 5, 1, 0);
    let r: std::collections::HashMap<String, String> =
        measure(&g, &f, &b, 5, 3, 5).unwrap().into_iter().collect();
    for k in [
        "cpr_blind",
        "cpr_best",
        "tot_blind",
        "tot_best",
        "cpr_lb_blind",
        "t90f",
        "sat",
        "nsc",
    ] {
        assert!(r.contains_key(k), "missing column {k}");
    }
    let (cb, cq): (f64, f64) = (
        r["cpr_blind"].parse().unwrap(),
        r["cpr_best"].parse().unwrap(),
    );
    assert!(
        cb.is_finite() && cq.is_finite() && cq <= cb + 1e-9,
        "the quotient search never costs more than the blind one: {cq} vs {cb}"
    );
    let (tb, tq): (f64, f64) = (
        r["tot_blind"].parse().unwrap(),
        r["tot_best"].parse().unwrap(),
    );
    assert!(tq <= tb + 1e-9);
}

// ------------------------------------------------------------------ analysis: planted rows ----

mod synth {
    use g4_iota_stable::analyze::*;
    use g4_iota_stable::bases::Rng;
    use std::collections::HashMap;

    pub type Row = HashMap<String, String>;

    pub struct P {
        pub m: usize,
        pub reps: usize,
        pub v: f64,
        pub g_blind: f64,
        pub g_best: f64,
        pub w: f64,
        pub s: f64,
        pub noise: f64,
        pub collapse_g: f64,
        pub ident: &'static str,
        pub aa_shift: f64,
        pub cens_v: usize,
        pub nitems_w: usize,
    }
    impl P {
        pub fn new(m: usize, reps: usize) -> P {
            P {
                m,
                reps,
                v: 1.0,
                g_blind: 1.0,
                g_best: 1.0,
                w: 1.0,
                s: 1.0,
                noise: 0.03,
                collapse_g: (1u64 << (m - 1)) as f64,
                ident: "1",
                aa_shift: 1.0,
                cens_v: 0,
                nitems_w: 30,
            }
        }
    }
    pub fn rows(p: &P) -> Vec<Row> {
        let mut rng = Rng(99);
        let mut out = vec![];
        for arm in ["vrand", "g4rand", "rset", "riota", "wrong"] {
            for half in ["main", "aa"] {
                for rep in 0..p.reps {
                    let mut jit =
                        |x: f64| x * (1.0 + p.noise * ((rng.below(2001) as f64 / 1000.0) - 1.0));
                    let stable = arm == "g4rand" || arm == "riota";
                    let shift = if half == "aa" && arm == "vrand" {
                        p.aa_shift
                    } else {
                        1.0
                    };
                    let (blind, best, tot_b, tot_s) = match arm {
                        "vrand" => (p.v, p.v, 10.0 * p.v, 10.0 * p.v),
                        "g4rand" | "riota" => {
                            (p.g_blind, p.g_best, 10.0 * p.g_blind, 10.0 * p.g_best)
                        }
                        "rset" => (p.s, p.s, 10.0 * p.s, 10.0 * p.s),
                        _ => (p.w, p.w, 10.0 * p.w, 10.0 * p.w),
                    };
                    let (cb, cs, tb, ts) = (
                        jit(blind) * shift,
                        jit(best) * shift,
                        jit(tot_b) * shift,
                        jit(tot_s) * shift,
                    );
                    let censored = arm == "vrand" && half == "main" && rep < p.cens_v;
                    let na = |v: f64| {
                        if censored {
                            "NA".to_string()
                        } else {
                            v.to_string()
                        }
                    };
                    let mut r: Row = HashMap::new();
                    let mut put = |k: &str, v: String| {
                        r.insert(k.to_string(), v);
                    };
                    put("arm", arm.into());
                    put("half", half.into());
                    put("rep", rep.to_string());
                    put("ident_ok", if stable { p.ident.into() } else { "0".into() });
                    put("quot_ok", if stable { "1".into() } else { "NA".into() });
                    put("closure", if stable { "1".into() } else { "0.01".into() });
                    put(
                        "collapse",
                        if stable {
                            (if arm == "g4rand" {
                                p.collapse_g
                            } else {
                                (1u64 << (p.m - 1)) as f64
                            })
                            .to_string()
                        } else {
                            "1.0".into()
                        },
                    );
                    put(
                        "nitems",
                        if arm == "wrong" && rep == 0 && half == "main" {
                            p.nitems_w.to_string()
                        } else {
                            "30".into()
                        },
                    );
                    put("active", if stable { "15".into() } else { "30".into() });
                    put("sat", "0.1".into());
                    put("cpr_blind", na(cb));
                    put("cpr_best", na(cs));
                    put("cpr_lb_blind", cb.to_string());
                    put("cpr_lb_best", cs.to_string());
                    put("tot_blind", na(tb));
                    put("tot_best", na(ts));
                    out.push(r);
                }
            }
        }
        out
    }
    pub fn judge(p: &P) -> Judge {
        judge_cell(&rows(p), (0, 13, p.m, 5), p.reps)
    }
}

#[test]
fn classify_covers_every_class() {
    use g4_iota_stable::analyze::*;
    let ci = |lo: f64, est: f64, hi: f64| Ci { est, lo, hi };
    assert_eq!(classify(ci(1.2, 1.5, 1.8), 0.05), "GAIN");
    assert_eq!(classify(ci(0.4, 0.5, 0.6), 0.05), "LOSS");
    assert_eq!(classify(ci(0.97, 1.0, 1.03), 0.05), "PARITY");
    assert_eq!(classify(ci(0.7, 1.0, 1.4), 0.05), "UNRESOLVED");
    assert_eq!(
        classify(ci(0.99, 1.04, 1.2), 0.05),
        "UNRESOLVED",
        "straddling the margin edge is not parity"
    );
    assert_eq!(
        classify(ci(0.7, 1.0, 1.4), 2.0),
        "UNRESOLVED",
        "a wide A/A delta must not turn a noisy cell into PARITY"
    );
    assert_eq!(classify(ci(f64::NAN, f64::NAN, f64::NAN), 0.05), "NO_DATA");
    // bootstrap sanity on planted data
    let a: Vec<f64> = (0..30).map(|i| 4.0 + 0.01 * (i % 5) as f64).collect();
    let b: Vec<f64> = (0..30).map(|i| 1.0 + 0.01 * (i % 5) as f64).collect();
    let r = boot_ratio(&a, &b, 1);
    assert!(
        r.lo > 3.8 && r.hi < 4.1 && r.est > 3.9 && r.est < 4.0,
        "{r:?}"
    );
    // paired bootstrap: costs proportional to a shared N cancel exactly inside each pair
    let nn: Vec<f64> = (0..30).map(|i| 0.5 + 0.1 * ((i * 7) % 30) as f64).collect();
    let pa: std::collections::BTreeMap<usize, f64> =
        nn.iter().enumerate().map(|(i, n)| (i, 4.0 * n)).collect();
    let pb: std::collections::BTreeMap<usize, f64> =
        nn.iter().enumerate().map(|(i, n)| (i, 1.0 * n)).collect();
    let rp = boot_ratio_paired(&pa, &pb, 1);
    assert!(
        (rp.lo - 4.0).abs() < 1e-9 && (rp.hi - 4.0).abs() < 1e-9,
        "paired ratio of proportional series is exact: {rp:?}"
    );
    let ru = boot_ratio(
        &pa.values().cloned().collect::<Vec<_>>(),
        &pb.values().cloned().collect::<Vec<_>>(),
        1,
    );
    assert!(
        ru.hi - ru.lo > 0.0,
        "the unpaired interval is wider: {ru:?}"
    );
    let k = binom_threshold(72, 0.10, 0.01);
    assert!((11..=17).contains(&k), "A/A gate threshold {k}");
}

#[test]
fn judge_cell_detects_planted_gain_loss_parity_unresolved() {
    use synth::*;
    // planted GAIN: V costs 4, G costs 1, wrong costs 4 (stabiliser-specific)
    let mut p = P::new(4, 24);
    p.v = 4.0;
    p.w = 4.0;
    p.s = 4.0;
    p.g_blind = 1.0;
    p.g_best = 1.0;
    let j = judge(&p);
    assert_eq!(j.instr, "ok");
    assert_eq!(
        (
            j.v_best.as_str(),
            j.v_blind.as_str(),
            j.v_tbest.as_str(),
            j.v_q_best.as_str()
        ),
        ("GAIN", "GAIN", "GAIN", "GAIN"),
        "{j:?}"
    );
    assert!(j.license, "R_best and Q_best GAIN, tot not LOSS");
    assert_eq!(
        j.v_null, "PARITY",
        "S and W equal: the construction null is parity"
    );
    // planted LOSS
    let mut p = P::new(4, 24);
    p.v = 1.0;
    p.g_blind = 4.0;
    p.g_best = 4.0;
    let j = judge(&p);
    assert_eq!((j.v_best.as_str(), j.v_blind.as_str()), ("LOSS", "LOSS"));
    assert!(!j.license);
    // planted PARITY (everything equal)
    let j = judge(&P::new(3, 40));
    assert_eq!(
        (j.v_best.as_str(), j.v_blind.as_str(), j.v_null.as_str()),
        ("PARITY", "PARITY", "PARITY"),
        "{j:?}"
    );
    assert_eq!(j.aa_flags, 0);
    // planted effect but huge noise and few bases -> UNRESOLVED
    let mut p = P::new(3, 6);
    p.g_best = 1.3;
    p.noise = 0.9;
    let j = judge(&p);
    assert_eq!(j.v_best, "UNRESOLVED", "{j:?}");
    // construction effect without stabiliser: V costs 4, W costs 4, S costs 1 -> null S/W is LOSS-ish (S cheaper)
    let mut p = P::new(4, 24);
    p.v = 4.0;
    p.w = 4.0;
    p.s = 1.0;
    p.g_blind = 1.0;
    p.g_best = 1.0;
    let j = judge(&p);
    assert_eq!(
        j.v_null, "LOSS",
        "null = cpr(S)/cpr(W) = 0.25: the construction W costs more than the plain random set"
    );
}

#[test]
fn judge_cell_void_conditions() {
    use synth::*;
    // positive control broken
    let mut p = P::new(3, 24);
    p.collapse_g = 3.0;
    assert_eq!(judge(&p).instr, "CONTROL_FAIL");
    assert_eq!(judge(&p).v_best, "VOID");
    // instrument identity broken
    let mut p = P::new(3, 24);
    p.ident = "0";
    assert_eq!(judge(&p).instr, "INSTRUMENT_FAIL");
    // censoring above 5 percent of V bases
    let mut p = P::new(3, 24);
    p.cens_v = 3;
    assert_eq!(judge(&p).v_best, "CENSORED");
    let mut p = P::new(3, 24);
    p.cens_v = 1; // 4.2 percent: allowed
    assert_eq!(judge(&p).v_best, "PARITY");
    // item counts differ in one replicate
    let mut p = P::new(3, 24);
    p.nitems_w = 31;
    assert_eq!(judge(&p).v_best, "UNMATCHED");
    // A/A false alarm: the second vrand half is 30 percent off, which must be flagged
    let mut p = P::new(3, 24);
    p.aa_shift = 1.3;
    let j = judge(&p);
    assert!(j.aa_flags >= 1, "A/A pair off by 30 percent must exclude 1");
    assert!(j.delta > 0.2, "the noise floor widens delta: {}", j.delta);
}

#[test]
fn partial_and_missing_cells_are_never_classified() {
    use g4_iota_stable::analyze::*;
    use synth::*;
    let p = P::new(3, 10);
    let all = rows(&p);
    assert_eq!(judge_cell(&all, (0, 13, 3, 5), 10).completeness, "COMPLETE");
    assert_eq!(judge_cell(&[], (0, 13, 3, 5), 10).completeness, "NOT_RUN");
    assert_eq!(judge_cell(&[], (0, 13, 3, 5), 10).v_best, "NOT_RUN");
    // drop the whole A/A half of one arm and one main replicate of another
    let partial: Vec<Row> = all
        .into_iter()
        .filter(|r| {
            !(r["arm"] == "vrand" && r["half"] == "aa")
                && !(r["arm"] == "wrong" && r["rep"] == "3" && r["half"] == "main")
        })
        .collect();
    let j = judge_cell(&partial, (0, 13, 3, 5), 10);
    assert!(
        j.completeness.starts_with("INCOMPLETE"),
        "{}",
        j.completeness
    );
    assert!(
        j.completeness.contains("vrand/aa:0of10") && j.completeness.contains("wrong/main:9of10"),
        "{}",
        j.completeness
    );
    assert_eq!(j.v_best, j.completeness, "no class for a partial cell");
    // a plan that expects more replicates than were written is INCOMPLETE, not silently classified
    let j = judge_cell(&rows(&P::new(3, 10)), (0, 13, 3, 5), 12);
    assert!(j.completeness.starts_with("INCOMPLETE"));
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
            println!(
                "n={n} a={a} ne={ne} lpf={} cyclic={ok}",
                largest_prime_factor(ne)
            );
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
    let c = Cell {
        a: 0,
        n: 13,
        m: 3,
        l: 5,
    };
    let o = RunOpts {
        seed: 1,
        reps: Some(3),
        out: d.clone(),
    };
    std::fs::write(dir.join("plan.tsv"), "0 13 3 5 3\n0 13 4 5 3\n").unwrap();
    let p = run_cell(&c, &o).expect("run_cell");
    let txt = std::fs::read_to_string(&p).unwrap();
    let hdr: Vec<&str> = txt.lines().next().unwrap().split('\t').collect();
    assert!(
        txt.lines().all(|l| l.split('\t').count() == hdr.len()),
        "ragged rows"
    );
    // 2 single arms once + 5 replicated arms x (3 main + 3 aa)
    assert_eq!(txt.lines().count() - 1, 2 + 5 * 6);
    g4_iota_stable::analyze::analyze(&d).expect("analyze");
    let md = std::fs::read_to_string(dir.join("summary.md")).unwrap();
    assert!(
        md.contains("| ok |"),
        "instrument/control status should be ok:\n{md}"
    );
    // the second planned cell was never run: it must be reported, not dropped
    assert!(
        md.contains("NOT_RUN"),
        "missing cell must be reported:\n{md}"
    );
    assert!(dir.join("fig_cost_ratio.svg").exists() && dir.join("summary.tsv").exists());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn per_m_closure_wording_ignores_void_cells_and_needs_two_nonvoid() {
    use g4_iota_stable::analyze::*;
    use synth::*;
    let mut lossp = P::new(4, 24);
    lossp.v = 1.0;
    lossp.g_blind = 4.0;
    lossp.g_best = 4.0;
    let loss = judge(&lossp);
    let par = judge(&P::new(3, 40));
    let mut gp = P::new(4, 24);
    gp.v = 4.0;
    gp.w = 4.0;
    gp.s = 4.0;
    gp.g_blind = 1.0;
    gp.g_best = 1.0;
    let gain = judge(&gp);
    let mut void = loss.clone();
    void.v_best = "CENSORED".into();
    void.v_tbest = "CENSORED".into();
    let mut notrun = loss.clone();
    notrun.completeness = "NOT_RUN".into();
    notrun.v_best = "NOT_RUN".into();
    assert!(per_m_closed(&[&loss, &par], false));
    assert!(
        per_m_closed(&[&loss, &par, &void, &notrun], false),
        "void cells neither count nor block"
    );
    assert!(
        !per_m_closed(&[&loss, &void], false),
        "one non-void cell is not enough"
    );
    assert!(
        !per_m_closed(&[&loss, &par, &gain], false),
        "a non-void GAIN blocks closure"
    );
    assert!(
        !per_m_closed(&[&loss, &par], true),
        "A/A failure voids the wording"
    );
}

#[test]
fn bases_with_no_row_in_g_are_rejected_by_the_draw_rule() {
    // A2 (a = 0): a stable base with exactly m classes has one class selection; when its parity is odd it has
    // no decomposition at any target of G (active = 0, cost NaN). The draw rule must reject it.
    let (f, g) = setup(13, 0);
    let m = 4usize;
    let (mut bad, mut good) = (0, 0);
    for seed in 0..4000u64 {
        let b = make_arm("g4rand", &f, 5, seed, 0);
        let (n, ncls) = shape_of(&g, &b.raw);
        if ncls != m || binom(n, m) > ENUM_CAP {
            continue;
        }
        let r: std::collections::HashMap<String, String> =
            measure(&g, &f, &b, 5, m, 7).unwrap().into_iter().collect();
        if has_g_row(&g, &b.raw, m) {
            good += 1;
            assert!(
                r["active"].parse::<usize>().unwrap() > 0 && r["t90f"] != "NA",
                "accepted base must have a finite cost"
            );
        } else {
            bad += 1;
            assert_eq!(r["active"], "0", "rejected base must have no row in G");
            assert_eq!(r["t90f"], "NA");
        }
    }
    assert!(
        bad > 0 && good > 0,
        "expected both parities among {m}-class bases (bad {bad}, good {good})"
    );
    // every accepted matched arm has a row in G
    let c = Cell {
        a: 0,
        n: 13,
        m,
        l: 5,
    };
    for rep in 0..8u64 {
        let mt = draw_matched(&f, &g, &c, 20261027, rep).unwrap();
        for (arm, b, _, _) in &mt.arms {
            assert!(has_g_row(&g, &b.raw, m), "{arm} has no row in G");
        }
    }
}

#[test]
fn censoring_gate_is_an_integer_comparison_at_exactly_five_percent() {
    use g4_iota_stable::analyze::censored_over_5pct;
    assert!(!censored_over_5pct((2, 40)));
    assert!(censored_over_5pct((3, 40)));
    assert!(!censored_over_5pct((5, 100)));
    assert!(censored_over_5pct((6, 100)));
    assert!(!censored_over_5pct((1, 20)));
    assert!(censored_over_5pct((1, 16)));
    assert!(!censored_over_5pct((0, 16)));
    assert!(censored_over_5pct((0, 0)));
}
