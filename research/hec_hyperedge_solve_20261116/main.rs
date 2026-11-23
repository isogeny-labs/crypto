#[path = "../hec_anchor_relations_20261114/anchor.rs"]
mod anchor;
#[path = "../hec_anchor_relations_20261114/dense.rs"]
mod dense;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use hec_native_ic::{
    fp::Fp,
    instance::Instance,
    jac::Div,
    poly::Poly,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::collections::{BTreeMap, HashSet};
#[derive(Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub status: &'static str,
    pub visited: usize,
    pub tuples: usize,
    pub admitted: usize,
    pub rank: usize,
    pub descent_steps: u64,
    pub source_support: usize,
    pub target_support: usize,
    pub reference_equal: bool,
}
fn choose(n: usize, k: usize) -> usize {
    (0..k.min(n - k)).fold(1, |v, j| v * (n - j) / (j + 1))
}
pub fn solve(inst: &Instance, target: &Div, h: usize, max_lp: usize, seed: u64) -> Report {
    let c = &inst.curve;
    let f = &c.fp;
    let ell = inst.ell;
    let lf = Fp::new(ell);
    assert!(c.is_valid(&inst.d1) && c.is_valid(target));
    assert!(!c.is_zero(&inst.d1) && !c.is_zero(target));
    assert!(c.is_zero(&c.mul(&inst.d1, ell)) && c.is_zero(&c.mul(target, ell)));
    let mut sq = 0u128;
    while sq * sq < inst.p as u128 {
        sq += 1
    }
    assert!((ell as u128).pow(2) > (sq + 1).pow(2 * inst.genus as u32));
    let tab = table_factor::Tables::for_degree(inst.p, inst.genus);
    let fb = tab.factor_base(c);
    let n = fb.len();
    assert!(h >= inst.genus + 1 && h <= n && max_lp >= 2 && max_lp <= inst.genus);
    let projected = anchor::projection(c, ell, &fb);
    let mut by = vec![u32::MAX; inst.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by[x as usize] = j as u32;
    }
    let mut descent_rng = Rng::new(seed);
    let mut steps = 0;
    let mut rep = Report {
        k: None,
        status: "unstarted",
        visited: 0,
        tuples: 0,
        admitted: 0,
        rank: 0,
        descent_steps: 0,
        source_support: 0,
        target_support: 0,
        reference_equal: false,
    };
    let Some(source) = anchor::descend(
        c,
        &inst.d1,
        &fb,
        &by,
        &tab,
        &mut descent_rng,
        true,
        &mut steps,
    ) else {
        rep.status = "descent_cap";
        rep.descent_steps = steps;
        return rep;
    };
    let Some(dest) = anchor::descend(
        c,
        target,
        &fb,
        &by,
        &tab,
        &mut descent_rng,
        true,
        &mut steps,
    ) else {
        rep.status = "descent_cap";
        rep.descent_steps = steps;
        return rep;
    };
    rep.descent_steps = steps;
    rep.source_support = source.len();
    rep.target_support = dest.len();
    let source = anchor::modular(&source, &lf);
    let mut dest = anchor::modular(&dest, &lf);
    assert!(
        !c.is_zero(&anchor::sum_mod(c, &projected, &source)),
        "nonzero_projected_source"
    );
    dest.push((n as u32, ell - 1));
    let mut rows = vec![(source, 1), (dest, 0)];
    let mut echelon = dense::Echelon::new(n, ell);
    for (row, rhs) in &rows {
        if let Some(k) = echelon.insert(row, *rhs) {
            rep.k = Some(k);
        }
    }
    let mut rng = Rng::new(2026111503 ^ ((inst.genus as u64) << 40) ^ ((h as u64) << 16));
    let m = inst.genus + 1;
    let budget = choose(h, m).min(1024);
    let mut tuples = HashSet::new();
    let mut seen = HashSet::new();
    'generation: while rep.k.is_none() && tuples.len() < budget && rep.visited < 32768 {
        let mut chosen = vec![];
        while chosen.len() < m {
            let j = rng.below(h as u64) as usize;
            if !chosen.contains(&j) {
                chosen.push(j)
            }
        }
        chosen.sort_unstable();
        if !tuples.insert(chosen.clone()) {
            continue;
        }
        let mut anchors = Poly::one();
        for &j in &chosen {
            anchors = anchors.mul(&Poly::x_minus(fb[j].0, f), f);
        }
        for signs in 0..1usize << (m - 1) {
            if rep.visited >= 32768 {
                break 'generation;
            }
            rep.visited += 1;
            let mut a = Poly::zero();
            let mut basis = Poly::one();
            for (i, &j) in chosen.iter().enumerate() {
                let (x, y) = fb[j];
                let y = if i > 0 && signs & (1 << (i - 1)) != 0 {
                    f.neg(y)
                } else {
                    y
                };
                let w = f.mul(f.sub(y, a.eval(x, f)), f.inv(basis.eval(x, f)));
                a = a.add(&basis.scale(w, f), f);
                basis = basis.mul(&Poly::x_minus(x, f), f);
            }
            let (q, rem) = a.mul(&a, f).sub(&c.f, f).divrem(&anchors, f);
            assert!(rem.is_zero() && q.deg() <= inst.genus as isize);
            let Some(roots) = tab.split(&q.monic(f)) else {
                continue;
            };
            let mut raw = BTreeMap::<usize, i64>::new();
            for (i, &j) in chosen.iter().enumerate() {
                *raw.entry(j).or_default() += if i > 0 && signs & (1 << (i - 1)) != 0 {
                    -1
                } else {
                    1
                };
            }
            for &(x, mult) in roots.as_slice() {
                let j = by[x as usize] as usize;
                assert_ne!(j, u32::MAX as usize);
                let y = a.eval(x, f);
                assert_eq!(f.mul(y, y), c.f.eval(x, f));
                *raw.entry(j).or_default() += if y == fb[j].1 {
                    mult as i64
                } else {
                    -(mult as i64)
                };
            }
            raw.retain(|_, v| *v != 0);
            if raw.keys().filter(|&&j| j >= h).count() > max_lp {
                continue;
            }
            let signed: anchor::Signed = raw.iter().map(|(&j, &v)| (j as u32, v)).collect();
            let mut row = anchor::modular(&signed, &lf);
            if row.is_empty() {
                continue;
            }
            let scale = lf.inv(row[0].1);
            for (_, v) in &mut row {
                *v = lf.mul(*v, scale);
            }
            if !seen.insert(row.clone()) {
                continue;
            }
            assert!(
                c.is_zero(&anchor::sum_signed(c, &fb, &signed)),
                "principal_full_group"
            );
            rep.admitted += 1;
            rows.push((row.clone(), 0));
            if let Some(k) = echelon.insert(&row, 0) {
                rep.k = Some(k);
                break 'generation;
            }
        }
    }
    rep.tuples = tuples.len();
    rep.rank = echelon.rank;
    let reference = solve_fast(&rows, n, ell, &mut SolveStats::default());
    assert_eq!(rep.k, reference, "independent_last_column_solver");
    rep.reference_equal = true;
    if let Some(k) = rep.k {
        assert_eq!(c.mul(&inst.d1, k), *target, "source_target_verification");
        rep.status = "verified";
    } else {
        rep.status = if rep.visited >= 32768 {
            "class_cap"
        } else if budget == choose(h, m) {
            "exhausted"
        } else {
            "tuple_cap"
        };
    }
    rep
}
fn main() {
    let mut cases = 0;
    let mut recovered = 0;
    for (g, p, h) in [
        (2, 1009, 12),
        (2, 1009, 20),
        (3, 503, 14),
        (4, 127, 10),
        (4, 127, 16),
    ] {
        let inst = hec_native_ic::instance::pick_instance(p, g).expect("fixed_instance");
        for (index, expected) in [113, 136].into_iter().enumerate() {
            let target = inst.curve.mul(&inst.d1, expected);
            for lp in 2..=g {
                let rep = solve(&inst, &target, h, lp, 20260916 + index as u64);
                if let Some(k) = rep.k {
                    assert_eq!(k, expected);
                    recovered += 1;
                }
                println!("CASE g={g} p={p} h={h} max_lp={lp} target_index={index} status={} k={:?} visited={} tuples={} admitted={} rank={} descent_steps={} source_support={} target_support={} reference_equal={} expected_checked_outside_solver=true",rep.status,rep.k,rep.visited,rep.tuples,rep.admitted,rep.rank,rep.descent_steps,rep.source_support,rep.target_support,rep.reference_equal);
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 20);
    println!("TERMINAL cases={cases} recovered={recovered} unsolved={} all_recovered_source_verified=true timing_claim=false",cases-recovered);
}
