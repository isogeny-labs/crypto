#[path = "../hec_confluent_20261116/confluent.rs"]
mod confluent;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use confluent::{gaussian, normalize, reference, Node};
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, pick_instance},
    jac::Div,
    poly::Poly,
    rng::Rng,
    solve::Row,
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
struct Elim {
    f: Fp,
    n: usize,
    h: usize,
    basis: Vec<Option<Vec<u64>>>,
    full: usize,
    off: usize,
    muls: u64,
    maxfill: usize,
    first: Option<usize>,
    admitted: usize,
}
impl Elim {
    fn new(n: usize, h: usize, p: u64) -> Self {
        Self {
            f: Fp::new(p),
            n,
            h,
            basis: vec![None; n],
            full: 0,
            off: 0,
            muls: 0,
            maxfill: 0,
            first: None,
            admitted: 0,
        }
    }
    fn insert(&mut self, row: &Row, visited: usize) {
        self.admitted += 1;
        let mut v = vec![0; self.n];
        for &(j, a) in row {
            let j = j as usize;
            let col = if j >= self.h {
                j - self.h
            } else {
                self.n - self.h + j
            };
            v[col] = a;
        }
        for j in 0..self.n {
            if v[j] == 0 {
                continue;
            }
            if let Some(b) = &self.basis[j] {
                let c = v[j];
                for k in j..self.n {
                    if b[k] != 0 {
                        v[k] = self.f.sub(v[k], self.f.mul(c, b[k]));
                        self.muls += 1;
                    }
                }
            } else {
                let inv = self.f.inv(v[j]);
                for a in &mut v[j..] {
                    if *a != 0 {
                        *a = self.f.mul(*a, inv);
                        self.muls += 1;
                    }
                }
                self.maxfill = self.maxfill.max(v.iter().filter(|&&a| a != 0).count());
                self.basis[j] = Some(v);
                self.full += 1;
                if j < self.n - self.h {
                    self.off += 1;
                }
                break;
            }
        }
        assert!(self.full - self.off <= self.h - 1);
        assert!(self.full < self.n);
        if self.full - self.off == self.h - 1 && self.first.is_none() {
            self.first = Some(visited);
        }
    }
}
fn rank(rows: &[Row], n: usize, skip: usize, f: &Fp) -> usize {
    let mut pivots: Vec<Option<Vec<u64>>> = vec![None; n];
    let mut count = 0;
    for row in rows {
        let mut v = vec![0; n];
        for &(j, a) in row {
            if j as usize >= skip {
                v[j as usize] = a;
            }
        }
        for j in skip..n {
            if v[j] == 0 {
                continue;
            }
            if let Some(b) = &pivots[j] {
                let q = v[j];
                for k in j..n {
                    v[k] = f.sub(v[k], f.mul(q, b[k]));
                }
            } else {
                let q = f.inv(v[j]);
                for x in &mut v[j..] {
                    *x = f.mul(*x, q);
                }
                pivots[j] = Some(v);
                count += 1;
                break;
            }
        }
    }
    count
}
fn choose(n: usize, k: usize) -> usize {
    (0..k.min(n - k)).fold(1, |a, j| a * (n - j) / (j + 1))
}
fn partitions(n: usize, max: usize, p: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if n == 0 {
        out.push(p.clone());
        return;
    }
    for k in (1..=max.min(n)).rev() {
        p.push(k);
        partitions(n - k, k, p, out);
        p.pop();
    }
}
fn factorial(n: usize) -> usize {
    (1..=n).product()
}
fn capacity(h: usize, parts: &[usize]) -> usize {
    let mut freq = BTreeMap::new();
    for &m in parts {
        *freq.entry(m).or_insert(0usize) += 1;
    }
    let num = (0..parts.len()).map(|i| h - i).product::<usize>();
    num / freq.values().map(|&n| factorial(n)).product::<usize>()
}
fn main() {
    let (mut families, mut records, mut residual_checks) = (0usize, 0usize, 0usize);
    let mut scores = vec![];
    for (g, p, h) in [(2, 1009, 8), (3, 503, 9), (4, 127, 10)] {
        let inst = pick_instance(p, g).unwrap();
        let c = &inst.curve;
        let f = &c.fp;
        let lf = Fp::new(inst.ell);
        let mut sq = 0u128;
        while sq * sq < p as u128 {
            sq += 1
        }
        assert!((inst.ell as u128).pow(2) > (sq + 1).pow(2 * g as u32));
        let original = factor_base(c);
        let mut fb: Vec<_> = original
            .iter()
            .copied()
            .filter(|p| p.1 != 0)
            .take(h)
            .collect();
        assert_eq!(fb.len(), h);
        let anchors: HashSet<_> = fb.iter().map(|p| p.0).collect();
        fb.extend(original.into_iter().filter(|p| !anchors.contains(&p.0)));
        let mut by = vec![usize::MAX; p as usize];
        for (j, &(x, _)) in fb.iter().enumerate() {
            by[x as usize] = j;
        }
        let table = table_factor::Tables::for_degree(p, g);
        println!(
            "ANCHORS g={g} p={p} h={h} c={} ell={} fb={} xs={:?}",
            inst.c,
            inst.ell,
            fb.len(),
            fb[..h].iter().map(|p| p.0).collect::<Vec<_>>()
        );
        for d in [0, 2] {
            let total = if d == 0 { g + 1 } else { g + 2 * d };
            let mut patterns = vec![];
            partitions(total, total, &mut vec![], &mut patterns);
            for parts in patterns {
                let label = parts
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("+");
                let repeated = parts.iter().any(|&m| m > 1);
                let code = parts.iter().fold(0u64, |a, &m| a * 16 + m as u64);
                let mut rng = Rng::new(2026111603 ^ ((g as u64) << 40) ^ ((d as u64) << 24) ^ code);
                let cap = capacity(h, &parts);
                let budget = cap.min(256);
                let mut profiles = HashSet::new();
                let mut functions = HashSet::new();
                let mut seenrows = HashSet::new();
                let mut rows = vec![];
                let mut policies: Vec<_> = [2, g]
                    .into_iter()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .map(|k| (k, Elim::new(fb.len(), h, inst.ell)))
                    .collect();
                let (
                    mut draws,
                    mut built,
                    mut common,
                    mut drops,
                    mut fdup,
                    mut nonsplit,
                    mut empty,
                    mut rdup,
                    mut checks,
                    mut rc,
                ) = (
                    0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize,
                );
                'profiles: while profiles.len() < budget && built < 2048 && draws < 50000 {
                    draws += 1;
                    let mut ids = vec![];
                    while ids.len() < parts.len() {
                        let j = rng.below(h as u64) as usize;
                        if !ids.contains(&j) {
                            ids.push(j);
                        }
                    }
                    let mut weights = vec![0usize; h];
                    for (&j, &m) in ids.iter().zip(&parts) {
                        weights[j] = m;
                    }
                    if !profiles.insert(weights.clone()) {
                        continue;
                    }
                    let support: Vec<_> = (0..h).filter(|&j| weights[j] > 0).collect();
                    for signs in 0..1usize << (support.len() - 1) {
                        if built >= 2048 {
                            break 'profiles;
                        }
                        built += 1;
                        let nodes: Vec<_> = support
                            .iter()
                            .enumerate()
                            .map(|(i, &j)| Node {
                                x: fb[j].0,
                                y: fb[j].1,
                                m: weights[j],
                                neg: i > 0 && signs & (1 << (i - 1)) != 0,
                            })
                            .collect();
                        let (a, b, _) = gaussian(c, &nodes, d);
                        let (a, b, q, kept, cd) = normalize(c, &nodes, a, b);
                        assert!(q.deg() <= g as isize);
                        assert_eq!((a, b), reference(c, &nodes, d));
                        common += (cd > 0) as usize;
                        drops += (q.deg() < g as isize) as usize;
                        if built <= 64 {
                            let u = q.monic(f);
                            let residual = if u.deg() == 0 {
                                c.zero()
                            } else {
                                let (gcd, inv, _) = Poly::xgcd(&b, &u, f);
                                assert_eq!(gcd, Poly::one());
                                Div {
                                    u,
                                    v: a.mul(&inv, f).rem(&u, f),
                                }
                            };
                            assert!(c.is_valid(&residual));
                            let mut sum = c.zero();
                            for (node, &m) in nodes.iter().zip(&kept) {
                                let point = c.mul(&c.place(node.x, node.y), m as u64);
                                sum = c.add(&sum, &if node.neg { c.neg(&point) } else { point });
                            }
                            assert_eq!(
                                residual,
                                c.neg(&sum),
                                "independent_weighted_Cantor_residual"
                            );
                            if cd == 0 {
                                assert_eq!(kept, nodes.iter().map(|n| n.m).collect::<Vec<_>>());
                            }
                            rc += 1;
                        }
                        if !functions.insert((a, b)) {
                            fdup += 1;
                            continue;
                        }
                        let Some(roots) = table.split(&q.monic(f)) else {
                            nonsplit += 1;
                            continue;
                        };
                        let mut raw = BTreeMap::<usize, i64>::new();
                        for (i, &j) in support.iter().enumerate() {
                            *raw.entry(j).or_default() += if nodes[i].neg {
                                -(kept[i] as i64)
                            } else {
                                kept[i] as i64
                            };
                        }
                        for &(x, m) in roots.as_slice() {
                            let j = by[x as usize];
                            assert_ne!(j, usize::MAX);
                            let bx = b.eval(x, f);
                            assert_ne!(bx, 0);
                            let y = f.mul(a.eval(x, f), f.inv(bx));
                            assert_eq!(f.mul(y, y), c.f.eval(x, f));
                            *raw.entry(j).or_default() +=
                                if y == fb[j].1 { m as i64 } else { -(m as i64) };
                        }
                        raw.retain(|_, v| *v != 0);
                        let mut row: Row = raw
                            .iter()
                            .filter_map(|(&j, &a)| {
                                let a = lf.from_i64(a);
                                (a != 0).then_some((j as u32, a))
                            })
                            .collect();
                        if row.is_empty() {
                            empty += 1;
                            continue;
                        }
                        let inv = lf.inv(row[0].1);
                        for (_, v) in &mut row {
                            *v = lf.mul(*v, inv);
                        }
                        if !seenrows.insert(row.clone()) {
                            rdup += 1;
                            continue;
                        }
                        let mut sum = c.zero();
                        for (&j, &a) in &raw {
                            let point = c.mul(&c.place(fb[j].0, fb[j].1), a.unsigned_abs());
                            sum = c.add(&sum, &if a < 0 { c.neg(&point) } else { point });
                        }
                        assert!(c.is_zero(&sum));
                        checks += 1;
                        let off = raw.keys().filter(|&&j| j >= h).count();
                        for (k, e) in &mut policies {
                            if off <= *k {
                                e.insert(&row, built);
                            }
                        }
                        rows.push((off, row));
                    }
                }
                let status = if built >= 2048 {
                    "function_cap"
                } else if draws >= 50000 {
                    "draw_cap"
                } else if cap == budget {
                    "exhausted"
                } else {
                    "profile_cap"
                };
                println!("FAMILY g={g} p={p} h={h} d={d} partition={label} repeated={repeated} capacity_profiles={cap} profiles={} draws={draws} constructed={built} common={common} degree_drops={drops} unique_functions={} function_duplicates={fdup} nonsplit={nonsplit} empty={empty} row_duplicates={rdup} principal_checks={checks} residual_class_checks={rc} status={status}",profiles.len(),functions.len());
                residual_checks += rc;
                families += 1;
                for (k, e) in policies {
                    let subset: Vec<_> = rows
                        .iter()
                        .filter(|(off, _)| *off <= k)
                        .map(|(_, r)| r.clone())
                        .collect();
                    let full = rank(&subset, fb.len(), 0, &lf);
                    let off = rank(&subset, fb.len(), h, &lf);
                    assert_eq!((full, off), (e.full, e.off));
                    println!("POLICY g={g} d={d} partition={label} max_lp={k} admitted={} full_rank={full} off_rank={off} anchor_rank={} first_complete={:?} elimination_mulmods={} max_pivot_nnz={} independent_final_ranks=true",e.admitted,full-off,e.first,e.muls,e.maxfill);
                    scores.push((g, k, repeated, e.first, label.clone(), d));
                    records += 1;
                }
            }
        }
    }
    for g in 2..=4 {
        for k in [2, g].into_iter().collect::<BTreeSet<_>>() {
            let baseline = scores
                .iter()
                .filter(|(gg, kk, r, _, _, _)| *gg == g && *kk == k && !*r)
                .filter_map(|(_, _, _, v, _, _)| *v)
                .min();
            let best = scores
                .iter()
                .filter(|(gg, kk, r, _, _, _)| *gg == g && *kk == k && *r)
                .filter_map(|(_, _, _, v, _, _)| *v)
                .min();
            let pass = match (baseline, best) {
                (Some(a), Some(b)) => 10 * b <= a,
                _ => false,
            };
            println!("DENSITY_GATE g={g} max_lp={k} best_distinct={baseline:?} best_repeated={best:?} tenfold_equal_construction_threshold={pass} training_only=true");
        }
    }
    assert_eq!((families, records), (63, 112));
    println!("TERMINAL families={families} policies={records} residual_class_checks={residual_checks} timing_claim=false");
}
