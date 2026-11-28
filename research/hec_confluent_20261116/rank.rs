mod confluent;
#[path = "../../crates/hec-native-ic/src/table_factor.rs"]
mod table_factor;
use confluent::{gaussian, normalize, reference, Node};
use hec_native_ic::{
    fp::Fp,
    instance::{factor_base, pick_instance},
    rng::Rng,
    solve::Row,
};
use std::collections::{BTreeMap, HashSet};
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
fn main() {
    let mut records = 0;
    for (g, p, h) in [(2, 1009, 8), (3, 503, 9), (4, 127, 10)] {
        let inst = pick_instance(p, g).unwrap();
        let c = &inst.curve;
        let f = &c.fp;
        let lf = Fp::new(inst.ell);
        let mut sq = 0u128;
        while sq * sq < p as u128 {
            sq += 1;
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
        let anchor_x: HashSet<_> = fb.iter().map(|p| p.0).collect();
        fb.extend(original.into_iter().filter(|p| !anchor_x.contains(&p.0)));
        let mut by = vec![usize::MAX; p as usize];
        for (j, &(x, _)) in fb.iter().enumerate() {
            by[x as usize] = j;
        }
        println!(
            "ANCHORS g={g} p={p} h={h} c={} ell={} fb={} xs={:?}",
            inst.c,
            inst.ell,
            fb.len(),
            fb[..h].iter().map(|p| p.0).collect::<Vec<_>>()
        );
        let table = table_factor::Tables::for_degree(p, g);
        for d in [0, 2] {
            let total = if d == 0 { g + 1 } else { g + 2 * d };
            for all in [false, true] {
                let capacity = if all {
                    choose(h + total - 1, total)
                } else {
                    choose(h, total)
                };
                let budget = capacity.min(1024);
                let mut rng = Rng::new(
                    2026111601 ^ ((g as u64) << 40) ^ ((h as u64) << 16) ^ ((d as u64) << 8),
                );
                let mut profiles = HashSet::new();
                let mut functions = HashSet::new();
                let mut rowseen = HashSet::new();
                let mut rows = vec![];
                let mut policies: Vec<_> = [2, g]
                    .into_iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .map(|k| (k, Elim::new(fb.len(), h, inst.ell)))
                    .collect();
                let (
                    mut draws,
                    mut built,
                    mut common,
                    mut drops,
                    mut functiondup,
                    mut nonsplit,
                    mut empty,
                    mut rowdup,
                    mut groups,
                ) = (
                    0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize,
                );
                'profiles: while profiles.len() < budget && built < 4096 && draws < 100000 {
                    draws += 1;
                    let mut selected = vec![];
                    while selected.len() < total {
                        let j = rng.below(h as u64) as usize;
                        if all || !selected.contains(&j) {
                            selected.push(j)
                        }
                    }
                    selected.sort_unstable();
                    if !profiles.insert(selected.clone()) {
                        continue;
                    }
                    let mut support = vec![];
                    let mut mults = vec![];
                    for j in selected {
                        if support.last() == Some(&j) {
                            *mults.last_mut().unwrap() += 1;
                        } else {
                            support.push(j);
                            mults.push(1usize);
                        }
                    }
                    for signs in 0..1usize << (support.len() - 1) {
                        if built >= 4096 {
                            break 'profiles;
                        }
                        built += 1;
                        let nodes: Vec<_> = support
                            .iter()
                            .enumerate()
                            .map(|(i, &j)| Node {
                                x: fb[j].0,
                                y: fb[j].1,
                                m: mults[i],
                                neg: i > 0 && signs & (1 << (i - 1)) != 0,
                            })
                            .collect();
                        let (a, b, _) = gaussian(c, &nodes, d);
                        let (a, b, q, kept, cd) = normalize(c, &nodes, a, b);
                        assert!(q.deg() <= g as isize);
                        assert_eq!(
                            (a, b),
                            reference(c, &nodes, d),
                            "independent_confluent_pair"
                        );
                        common += (cd > 0) as usize;
                        drops += (q.deg() < g as isize) as usize;
                        if !functions.insert((a, b)) {
                            functiondup += 1;
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
                                let v = lf.from_i64(a);
                                (v != 0).then_some((j as u32, v))
                            })
                            .collect();
                        if row.is_empty() {
                            empty += 1;
                            continue;
                        }
                        let inv = lf.inv(row[0].1);
                        for (_, a) in &mut row {
                            *a = lf.mul(*a, inv);
                        }
                        if !rowseen.insert(row.clone()) {
                            rowdup += 1;
                            continue;
                        }
                        let mut sum = c.zero();
                        for (&j, &a) in &raw {
                            let mut place = c.place(fb[j].0, fb[j].1);
                            if a < 0 {
                                place = c.neg(&place)
                            }
                            sum = c.add(&sum, &c.mul(&place, a.unsigned_abs()));
                        }
                        assert!(c.is_zero(&sum), "rank_screen_principal");
                        groups += 1;
                        let off = raw.keys().filter(|&&j| j >= h).count();
                        assert!(off <= g);
                        for (k, e) in &mut policies {
                            if off <= *k {
                                e.insert(&row, built)
                            }
                        }
                        rows.push((off, row));
                    }
                }
                let terminal = if built >= 4096 {
                    "function_cap"
                } else if draws >= 100000 {
                    "draw_cap"
                } else if capacity == budget {
                    "exhausted"
                } else {
                    "profile_cap"
                };
                for (k, e) in policies {
                    let subset: Vec<_> = rows
                        .iter()
                        .filter(|(off, _)| *off <= k)
                        .map(|(_, r)| r.clone())
                        .collect();
                    let full = rank(&subset, fb.len(), 0, &lf);
                    let off = rank(&subset, fb.len(), h, &lf);
                    assert_eq!((full, off), (e.full, e.off));
                    println!("RANK g={g} p={p} h={h} d={d} all_multisets={all} max_lp={k} profiles={} draws={draws} constructed={built} common={common} degree_drops={drops} unique_functions={} function_duplicates={functiondup} nonsplit={nonsplit} empty={empty} row_duplicates={rowdup} group_checks={groups} admitted={} full_rank={full} off_rank={off} anchor_rank={} first_complete={:?} elimination_mulmods={} max_pivot_nnz={} status={terminal} independent_ranks=true",profiles.len(),functions.len(),e.admitted,full-off,e.first,e.muls,e.maxfill);
                    records += 1;
                }
            }
        }
    }
    assert_eq!(records, 20);
    println!("TERMINAL family_arms=12 policy_records=20 timing_claim=false");
}
