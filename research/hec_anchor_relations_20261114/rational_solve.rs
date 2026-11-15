//! Complete normalized-rational solver. Correctness gate in RATIONAL-SOLVE-PROTOCOL.md.
#[path = "dense.rs"]
mod dense;
use crate::{
    anchor::{self, Signed},
    rational,
    table_factor::Tables,
};
use hec_native_ic::{
    fp::Fp,
    jac::{Curve, Div},
    rng::Rng,
    solve::Row,
};
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
pub struct Report {
    pub base: anchor::Report,
    pub requested_h: usize,
    pub constructed: u64,
    pub common: u64,
    pub normalized_admitted: u64,
    pub status: &'static str,
}

fn choose_bound(n: usize, k: usize) -> usize {
    let mut v = 1usize;
    for i in 0..k.min(n - k) {
        v = v * (n - i) / (i + 1);
        if v > 1_000_000 {
            return 1_000_001;
        }
    }
    v
}

pub fn solve(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    seed: u64,
    requested_h: usize,
    d: usize,
    gate: bool,
) -> Report {
    solve_variant(c, d1, d2, ell, seed, requested_h, d, gate, false)
}

pub fn solve_linear(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    seed: u64,
    requested_h: usize,
    gate: bool,
) -> Report {
    solve_variant(c, d1, d2, ell, seed, requested_h, 2, gate, true)
}

fn solve_variant(
    c: &Curve,
    d1: &Div,
    d2: &Div,
    ell: u64,
    seed: u64,
    requested_h: usize,
    d: usize,
    gate: bool,
    linear: bool,
) -> Report {
    let all = Instant::now();
    let fp = &c.fp;
    let f = Fp::new(ell);
    let m_tuple = c.g + 2 * d;
    let mut out = Report {
        base: anchor::Report::default(),
        requested_h,
        constructed: 0,
        common: 0,
        normalized_admitted: 0,
        status: "unstarted",
    };
    let r = &mut out.base;
    assert!((1..=3).contains(&d));
    let upper = (fp.p as f64).sqrt() as u128 + 2;
    assert!((ell as u128).pow(2) > upper.pow(2 * c.g as u32));
    let mut rng = Rng::new(seed);
    let tab = Tables::for_degree(fp.p, c.g);
    let fb = tab.factor_base(c);
    let n = fb.len();
    r.m = n;
    if n < m_tuple {
        out.status = "unavailable";
        r.total_ns = all.elapsed().as_nanos();
        return out;
    }
    let mut by = vec![u32::MAX; fp.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by[x as usize] = j as u32;
    }
    let Some(a) = anchor::descend(c, d1, &fb, &by, &tab, &mut rng, gate, &mut r.descent_steps)
    else {
        out.status = "descent_cap";
        r.total_ns = all.elapsed().as_nanos();
        return out;
    };
    let Some(b) = anchor::descend(c, d2, &fb, &by, &tab, &mut rng, gate, &mut r.descent_steps)
    else {
        out.status = "descent_cap";
        r.total_ns = all.elapsed().as_nanos();
        return out;
    };
    let mut hot = vec![false; n];
    for &(j, _) in a.iter().chain(b.iter()) {
        hot[j as usize] = true;
    }
    let support = hot.iter().filter(|&&v| v).count();
    let h = requested_h.max(support).max(m_tuple).min(n);
    let mut rest: Vec<_> = (0..n).filter(|&j| !hot[j]).collect();
    for j in 0..h - support {
        let k = j + rng.below((rest.len() - j) as u64) as usize;
        rest.swap(j, k);
        hot[rest[j]] = true;
    }
    let anchors: Vec<_> = (0..n).filter(|&j| hot[j]).collect();
    r.anchors = h;
    let mut col = vec![u32::MAX; n];
    for (j, &v) in anchors.iter().enumerate() {
        col[v] = j as u32;
    }
    let map = |raw: &Signed| -> Row {
        let raw: Signed = raw.iter().map(|&(j, v)| (col[j as usize], v)).collect();
        anchor::modular(&anchor::signed(raw), &f)
    };
    let mut target = map(&b);
    target.push((h as u32, ell - 1));
    let mut echelon = dense::Echelon::new(h, ell);
    echelon.insert(&map(&a), 1);
    if let Some(k) = echelon.insert(&target, 0) {
        assert_eq!(c.mul(d1, k), *d2);
        r.k = Some(k);
        out.status = "solved";
        r.setup_ns = all.elapsed().as_nanos();
        r.total_ns = r.setup_ns;
        return out;
    }
    let projected = if gate {
        let ps = anchor::projection(c, ell, &fb);
        Some(anchors.iter().map(|&j| ps[j]).collect::<Vec<_>>())
    } else {
        None
    };
    let mut forest = dense::DenseForest::new(n, h, ell);
    let mut tuples = HashSet::new();
    let mut relations = HashSet::new();
    let mut pure = HashSet::new();
    r.memory_bytes = tab.bytes() + forest.bytes() + by.len() * 4 + col.len() * 4 + fb.len() * 16;
    r.setup_ns = all.elapsed().as_nanos();
    let limit = choose_bound(h, m_tuple);
    'outer: loop {
        if r.draws >= 1_000_000 {
            out.status = "draw_cap";
            break;
        }
        if tuples.len() == limit {
            out.status = "exhausted";
            break;
        }
        let clock = Instant::now();
        let mut retry = 0;
        let chosen = loop {
            let mut js = Vec::new();
            while js.len() < m_tuple {
                let j = anchors[rng.below(h as u64) as usize];
                if !js.contains(&j) {
                    js.push(j);
                }
            }
            js.sort_unstable();
            if tuples.insert(js.clone()) {
                break js;
            }
            retry += 1;
            if retry >= 10000 {
                r.relation_ns += clock.elapsed().as_nanos();
                out.status = "tuple_retry_cap";
                break 'outer;
            }
        };
        let points: Vec<_> = chosen.iter().map(|&j| fb[j]).collect();
        let kernel = if linear {
            Some(crate::linear::Kernel::new(c.g, &points, fp))
        } else {
            None
        };
        let batch = if linear {
            Vec::new()
        } else {
            rational::SignedFamily::new(c.g, d, &points, fp).walsh(fp)
        };
        let len = kernel.as_ref().map_or(batch.len(), |k| k.pairs.len());
        out.constructed += len as u64;
        r.relation_ns += clock.elapsed().as_nanos();
        for signs in 0..len {
            if r.draws >= 1_000_000 {
                out.status = "draw_cap";
                break 'outer;
            }
            r.draws += 1;
            let clock = Instant::now();
            let normalized = if let Some(k) = &kernel {
                k.primitive(c, signs)
            } else {
                let (a, b) = batch[signs];
                rational::primitive(c, &points, a, b)
            };
            let Some(p) = normalized else {
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            };
            let common = p.common_degree > 0;
            if common {
                out.common += 1;
            }
            let q = p.q.monic(fp);
            let Some(roots) = tab.split(&q) else {
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            };
            r.splits += 1;
            let mut raw: Signed = chosen
                .iter()
                .enumerate()
                .filter(|(i, _)| p.keep_mask & (1 << i) != 0)
                .map(|(i, &j)| {
                    (
                        j as u32,
                        if i > 0 && signs & (1 << (i - 1)) != 0 {
                            -1
                        } else {
                            1
                        },
                    )
                })
                .collect();
            for &(x, mult) in roots.as_slice() {
                let j = by[x as usize];
                assert_ne!(j, u32::MAX);
                let bx = p.b.eval(x, fp);
                assert_ne!(bx, 0);
                let y = fp.mul(p.a.eval(x, fp), fp.inv(bx));
                assert_eq!(fp.mul(y, y), c.f.eval(x, fp));
                raw.push((
                    j,
                    if y == fb[j as usize].1 {
                        mult as i64
                    } else {
                        -(mult as i64)
                    },
                ));
            }
            let raw = anchor::signed(raw);
            if raw.iter().filter(|&&(j, _)| !hot[j as usize]).count() > 2 {
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            }
            let mut row = anchor::modular(&raw, &f);
            if row.is_empty() {
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            }
            let scale = f.inv(row[0].1);
            for (_, v) in &mut row {
                *v = f.mul(*v, scale);
            }
            if !relations.insert(row.clone()) {
                r.duplicate_relations += 1;
                r.relation_ns += clock.elapsed().as_nanos();
                continue;
            }
            if gate {
                assert_eq!(
                    anchor::sum_signed(c, &fb, &raw),
                    c.zero(),
                    "normalized raw principal"
                );
                r.raw_checks += 1;
            }
            r.admitted += 1;
            if common {
                out.normalized_admitted += 1;
            }
            r.relation_ns += clock.elapsed().as_nanos();
            let clock = Instant::now();
            let mut off = Vec::new();
            let mut on = Row::new();
            for (j, v) in row {
                if hot[j as usize] {
                    on.push((col[j as usize], v));
                } else {
                    off.push((j as usize + 1, v));
                }
            }
            on.sort_unstable();
            let reduced = forest.insert(&off, &on);
            r.forest_ns += clock.elapsed().as_nanos();
            let Some(mut row) = reduced else {
                continue;
            };
            if row.is_empty() {
                continue;
            }
            let scale = f.inv(row[0].1);
            for (_, v) in &mut row {
                *v = f.mul(*v, scale);
            }
            if !pure.insert(row.clone()) {
                continue;
            }
            if let Some(ps) = &projected {
                assert_eq!(
                    anchor::sum_mod(c, ps, &row),
                    c.zero(),
                    "normalized projected anchor"
                );
                r.pure_checks += 1;
            }
            r.pure_rows += 1;
            let clock = Instant::now();
            r.solves += 1;
            let k = echelon.insert(&row, 0);
            r.solve_ns += clock.elapsed().as_nanos();
            if let Some(k) = k {
                assert_eq!(c.mul(d1, k), *d2, "normalized target verification");
                r.k = Some(k);
                out.status = "solved";
                break 'outer;
            }
        }
    }
    r.unions = forest.unions;
    r.pins = forest.pins;
    r.unique_tuples = tuples.len();
    r.total_ns = all.elapsed().as_nanos();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalized_single_target_solves() {
        for (g, p) in [(2, 101), (3, 61), (4, 31)] {
            let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
            for requested in [g + 6, 4 * g + 12] {
                for seed in 0..2 {
                    let k = 113 + seed * 23;
                    let d2 = i.curve.mul(&i.d1, k);
                    let r = solve(
                        &i.curve,
                        &i.d1,
                        &d2,
                        i.ell,
                        20260916 + seed,
                        requested,
                        2,
                        true,
                    );
                    println!("rational_solve g={g} p={p} seed={seed} requested={} effective={} constructed={} consumed={} admitted={} normalized={} principal_checks={} projected_checks={} status={}",r.requested_h,r.base.anchors,r.constructed,r.base.draws,r.base.admitted,r.normalized_admitted,r.base.raw_checks,r.base.pure_checks,r.status);
                    assert_eq!(r.status, "solved");
                    assert_eq!(r.base.k, Some(k));
                    assert_eq!(r.base.raw_checks, r.base.admitted);
                    assert_eq!(r.base.pure_checks, r.base.pure_rows as u64);
                    assert!(r.constructed >= r.base.draws);
                }
            }
        }
    }

    #[test]
    fn linear_and_generic_complete_solvers_are_exact() {
        fn counts(r: &Report) -> Vec<u64> {
            let b = &r.base;
            vec![
                b.k.unwrap(),
                r.requested_h as u64,
                b.m as u64,
                b.anchors as u64,
                r.constructed,
                b.draws,
                b.unique_tuples as u64,
                b.splits,
                b.admitted,
                b.duplicate_relations,
                b.pure_rows as u64,
                b.unions,
                b.pins,
                b.solves as u64,
                b.descent_steps,
                b.raw_checks,
                b.pure_checks,
                r.common,
                r.normalized_admitted,
            ]
        }
        for (g, p) in [(2, 101), (3, 61), (4, 31)] {
            let i = hec_native_ic::instance::pick_instance(p, g).unwrap();
            for h in [g + 6, 4 * g + 12] {
                for seed in 0..2 {
                    let k = 113 + 23 * seed;
                    let target = i.curve.mul(&i.d1, k);
                    let a = solve(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, h, 2, true);
                    let b = solve_linear(&i.curve, &i.d1, &target, i.ell, 20260916 + seed, h, true);
                    assert_eq!(a.status, "solved");
                    assert_eq!(b.status, a.status);
                    assert_eq!(counts(&a), counts(&b));
                    println!("linear_solve g={g} p={p} seed={seed} requested={h} effective={} constructed={} consumed={} exact_counts=true k={k}",b.base.anchors,b.constructed,b.base.draws);
                }
            }
        }
    }
}
