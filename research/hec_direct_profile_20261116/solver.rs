use crate::{anchor, confluent, dense, table_factor};
use confluent::Node;
use hec_native_ic::{
    fp::Fp,
    instance::Instance,
    jac::Div,
    rng::Rng,
    solve::{solve_fast, Row, SolveStats},
};
use std::collections::{BTreeMap, BTreeSet, HashSet};
#[derive(Clone, Copy, Debug)]
pub enum Kernel {
    Gaussian,
    Direct,
}
#[derive(Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub status: &'static str,
    pub visited: usize,
    pub profiles: usize,
    pub admitted: usize,
    pub rank: usize,
    pub descent: u64,
    pub source_support: usize,
    pub target_support: usize,
    pub prepared: usize,
    pub cache_muls: usize,
    pub cache_doubles: usize,
    pub profile_adds: usize,
    pub gray_adds: usize,
    pub rows: Vec<(Row, u64)>,
}
fn factorial(n: usize) -> usize {
    (1..=n).product()
}
fn capacity(h: usize, parts: &[usize]) -> usize {
    let mut f = BTreeMap::new();
    for &m in parts {
        *f.entry(m).or_insert(0usize) += 1;
    }
    (0..parts.len()).map(|i| h - i).product::<usize>()
        / f.values().map(|&n| factorial(n)).product::<usize>()
}
fn batch(
    inst: &Instance,
    support: &[usize],
    weights: &[usize],
    cache: &BTreeMap<(usize, usize), (Div, Div)>,
    rep: &mut Report,
) -> Vec<Div> {
    let c = &inst.curve;
    let mut sum = cache[&(support[0], weights[support[0]])].0;
    for &j in &support[1..] {
        sum = c.add(&sum, &cache[&(j, weights[j])].0);
        rep.profile_adds += 1;
    }
    let count = 1usize << (support.len() - 1);
    let mut out = vec![c.zero(); count];
    out[0] = c.neg(&sum);
    for step in 1..count {
        let gray = step ^ (step >> 1);
        let bit = step.trailing_zeros() as usize;
        let j = support[bit + 1];
        let delta = cache[&(j, weights[j])].1;
        sum = c.add(
            &sum,
            &if gray & (1 << bit) != 0 {
                c.neg(&delta)
            } else {
                delta
            },
        );
        rep.gray_adds += 1;
        out[gray] = c.neg(&sum);
    }
    rep.prepared += count;
    out
}
pub fn solve(
    inst: &Instance,
    target: &Div,
    h: usize,
    d: usize,
    parts: &[usize],
    seed: u64,
    kernel: Kernel,
    gate: bool,
) -> Report {
    let c = &inst.curve;
    let f = &c.fp;
    let ell = inst.ell;
    let lf = Fp::new(ell);
    assert!(c.is_valid(&inst.d1) && c.is_valid(target));
    assert!(!c.is_zero(&inst.d1) && !c.is_zero(target));
    assert!(c.is_zero(&c.mul(&inst.d1, ell)) && c.is_zero(&c.mul(target, ell)));
    let mut sq = 0u128;
    while sq * sq < inst.p as u128 {
        sq += 1;
    }
    assert!((ell as u128).pow(2) > (sq + 1).pow(2 * inst.genus as u32));
    assert_eq!(
        parts.iter().sum::<usize>(),
        if d == 0 {
            inst.genus + 1
        } else {
            inst.genus + 2 * d
        }
    );
    let tab = table_factor::Tables::for_degree(inst.p, inst.genus);
    let fb = tab.factor_base(c);
    let n = fb.len();
    let anchors: Vec<_> = fb
        .iter()
        .enumerate()
        .filter(|(_, p)| p.1 != 0)
        .take(h)
        .map(|(j, _)| j)
        .collect();
    assert_eq!(anchors.len(), h);
    let mut by = vec![u32::MAX; inst.p as usize];
    for (j, &(x, _)) in fb.iter().enumerate() {
        by[x as usize] = j as u32;
    }
    let mut rep = Report {
        k: None,
        status: "unstarted",
        visited: 0,
        profiles: 0,
        admitted: 0,
        rank: 0,
        descent: 0,
        source_support: 0,
        target_support: 0,
        prepared: 0,
        cache_muls: 0,
        cache_doubles: 0,
        profile_adds: 0,
        gray_adds: 0,
        rows: vec![],
    };
    let mut rng = Rng::new(seed);
    let Some(source) = anchor::descend(
        c,
        &inst.d1,
        &fb,
        &by,
        &tab,
        &mut rng,
        gate,
        &mut rep.descent,
    ) else {
        rep.status = "descent_cap";
        return rep;
    };
    let Some(dest) = anchor::descend(c, target, &fb, &by, &tab, &mut rng, gate, &mut rep.descent)
    else {
        rep.status = "descent_cap";
        return rep;
    };
    rep.source_support = source.len();
    rep.target_support = dest.len();
    let source = anchor::modular(&source, &lf);
    if gate {
        let projected = anchor::projection(c, ell, &fb);
        assert!(!c.is_zero(&anchor::sum_mod(c, &projected, &source)));
    }
    let mut dest = anchor::modular(&dest, &lf);
    dest.push((n as u32, ell - 1));
    rep.rows = vec![(source, 1), (dest, 0)];
    let mut echelon = dense::Echelon::new(n, ell);
    for (row, rhs) in &rep.rows {
        if let Some(k) = echelon.insert(row, *rhs) {
            rep.k = Some(k);
        }
    }
    let mut cache = BTreeMap::new();
    if matches!(kernel, Kernel::Direct) {
        let ms: BTreeSet<_> = parts.iter().copied().collect();
        for j in 0..h {
            let (x, y) = fb[anchors[j]];
            let p = c.place(x, y);
            for &m in &ms {
                let mp = if m == 1 {
                    p
                } else {
                    rep.cache_muls += 1;
                    c.mul(&p, m as u64)
                };
                let twice = c.add(&mp, &mp);
                rep.cache_doubles += 1;
                cache.insert((j, m), (mp, twice));
            }
        }
    }
    let code = parts.iter().fold(0u64, |a, &m| a * 16 + m as u64);
    let mut rng =
        Rng::new(2026111611 ^ ((inst.genus as u64) << 40) ^ ((inst.p as u64) << 16) ^ code);
    let cap = capacity(h, parts);
    let budget = cap.min(256);
    let mut profiles = HashSet::new();
    let mut seen = HashSet::new();
    let mut draws = 0;
    'profiles: while rep.k.is_none()
        && profiles.len() < budget
        && rep.visited < 2048
        && draws < 50000
    {
        draws += 1;
        let mut ids = vec![];
        while ids.len() < parts.len() {
            let j = rng.below(h as u64) as usize;
            if !ids.contains(&j) {
                ids.push(j);
            }
        }
        let mut weights = vec![0usize; h];
        for (&j, &m) in ids.iter().zip(parts) {
            weights[j] = m;
        }
        if !profiles.insert(weights.clone()) {
            continue;
        }
        let support: Vec<_> = (0..h).filter(|&j| weights[j] > 0).collect();
        let direct = if matches!(kernel, Kernel::Direct) {
            batch(inst, &support, &weights, &cache, &mut rep)
        } else {
            vec![]
        };
        for signs in 0..1usize << (support.len() - 1) {
            if rep.visited >= 2048 {
                break 'profiles;
            }
            rep.visited += 1;
            let nodes: Vec<_> = support
                .iter()
                .enumerate()
                .map(|(i, &j)| Node {
                    x: fb[anchors[j]].0,
                    y: fb[anchors[j]].1,
                    m: weights[j],
                    neg: i > 0 && signs & (1 << (i - 1)) != 0,
                })
                .collect();
            let mut raw = BTreeMap::<usize, i64>::new();
            match kernel {
                Kernel::Gaussian => {
                    rep.prepared += 1;
                    let (a, b, _) = confluent::gaussian(c, &nodes, d);
                    let (a, b, q, kept, _) = confluent::normalize(c, &nodes, a, b);
                    assert!(q.deg() <= inst.genus as isize);
                    let Some(roots) = tab.split(&q.monic(f)) else {
                        continue;
                    };
                    for (i, &j) in support.iter().enumerate() {
                        *raw.entry(anchors[j]).or_default() += if nodes[i].neg {
                            -(kept[i] as i64)
                        } else {
                            kept[i] as i64
                        };
                    }
                    for &(x, m) in roots.as_slice() {
                        let j = by[x as usize] as usize;
                        let y = f.mul(a.eval(x, f), f.inv(b.eval(x, f)));
                        *raw.entry(j).or_default() +=
                            if y == fb[j].1 { m as i64 } else { -(m as i64) };
                    }
                }
                Kernel::Direct => {
                    let residual = direct[signs];
                    let Some(roots) = tab.split(&residual.u) else {
                        continue;
                    };
                    for (i, &j) in support.iter().enumerate() {
                        *raw.entry(anchors[j]).or_default() += if nodes[i].neg {
                            -(weights[j] as i64)
                        } else {
                            weights[j] as i64
                        };
                    }
                    for &(x, m) in roots.as_slice() {
                        let j = by[x as usize] as usize;
                        let y = residual.v.eval(x, f);
                        *raw.entry(j).or_default() +=
                            if y == fb[j].1 { m as i64 } else { -(m as i64) };
                    }
                }
            }
            raw.retain(|_, v| *v != 0);
            let signed: anchor::Signed = raw.iter().map(|(&j, &v)| (j as u32, v)).collect();
            let mut row = anchor::modular(&signed, &lf);
            if row.is_empty() {
                continue;
            }
            let scale = lf.inv(row[0].1);
            for (_, a) in &mut row {
                *a = lf.mul(*a, scale);
            }
            if !seen.insert(row.clone()) {
                continue;
            }
            if gate {
                assert!(
                    c.is_zero(&anchor::sum_signed(c, &fb, &signed)),
                    "principal_full_group"
                );
            }
            rep.admitted += 1;
            rep.rows.push((row.clone(), 0));
            if let Some(k) = echelon.insert(&row, 0) {
                rep.k = Some(k);
                break 'profiles;
            }
        }
    }
    rep.profiles = profiles.len();
    rep.rank = echelon.rank;
    if gate {
        assert_eq!(
            rep.k,
            solve_fast(&rep.rows, n, ell, &mut SolveStats::default())
        );
    }
    if let Some(k) = rep.k {
        assert_eq!(c.mul(&inst.d1, k), *target);
        rep.status = "verified";
    } else {
        rep.status = if rep.visited >= 2048 {
            "class_cap"
        } else if draws >= 50000 {
            "draw_cap"
        } else if cap == budget {
            "exhausted"
        } else {
            "profile_cap"
        };
    }
    rep
}
