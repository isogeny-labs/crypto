#![allow(dead_code)]
#[path = "../multi/algebra.rs"]
mod algebra;
#[path = "../setup/fast.rs"]
mod fast;
#[path = "../normalized/source.rs"]
mod source_normalization;
use hec_native_ic::{
    fp::Fp,
    instance::factor_base,
    jac::{Curve, Div},
    oracle::{split_reference, OracleStats},
    rng::Rng,
    solve::{solve_for_last, Row, SolveStats, Solver},
};
const NONE: usize = usize::MAX;
#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Folded,
    Expanded,
    Plain,
}
#[derive(Default, Debug)]
pub struct Report {
    pub k: Option<u64>,
    pub source_normalized: bool,
    pub trials: u64,
    pub relations: usize,
    pub m: usize,
    pub unknowns: usize,
    pub state_len: usize,
    pub solve_mulmods: u64,
    pub max_row_len: usize,
}
fn sparse(v: &[u64], k: u64) -> Row {
    let mut out: Row = v
        .iter()
        .enumerate()
        .filter_map(|(j, &v)| (v != 0).then_some((j as u32, v)))
        .collect();
    if k != 0 {
        out.push((v.len() as u32, k))
    }
    out
}
fn partition(
    c: &Curve,
    fb: &[(u64, u64)],
    ell: u64,
    z: u64,
    a: &[i64],
) -> (Vec<(usize, u64)>, Vec<usize>) {
    let n = 2 * c.g + 1;
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&v| v == 0));
    assert_ne!(c.f.c[0], 0);
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let f = Fp::new(ell);
    let (q, _, degree, lambda) = algebra::certificate(&a, ell);
    assert_eq!(q.deg(), n as isize - 1);
    assert_eq!(degree, 1);
    let lambda = lambda.unwrap();
    assert_ne!(lambda, 1);
    assert_eq!(q.eval(lambda, &f), 0);
    let mut seen = vec![false; fb.len()];
    let mut map = vec![(NONE, 0); fb.len()];
    let mut reps = vec![];
    for start in 0..fb.len() {
        if seen[start] {
            continue;
        }
        if fb[start].0 == 0 {
            seen[start] = true;
            continue;
        }
        let r = reps.len();
        reps.push(start);
        let (mut x, y, mut weight) = (fb[start].0, fb[start].1, 1);
        for _ in 0..n {
            let j = fb.iter().position(|v| v.0 == x).unwrap();
            assert!(!seen[j]);
            seen[j] = true;
            map[j] = (
                r,
                if fb[j].1 == y {
                    weight
                } else {
                    assert_eq!(fb[j].1, c.fp.neg(y));
                    f.neg(weight)
                },
            );
            x = c.fp.mul(x, z);
            weight = f.mul(weight, lambda);
        }
        assert_eq!(x, fb[start].0);
        assert_eq!(weight, 1);
    }
    assert!(seen.iter().all(|v| *v));
    (map, reps)
}

#[path = "../online-lp/online.rs"]
mod incremental;
#[path = "sparse.rs"]
mod sparse_full;
#[derive(Clone, Copy, Debug)]
pub enum Engine {
    Gate,
    Online,
    Sparse,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Out {
    pub k: Option<u64>,
    pub trials: u64,
    pub accepted: u64,
    pub rejected: u64,
    pub core: usize,
    pub rank: usize,
}
pub fn run(
    c: &Curve,
    source: &Div,
    target: &Div,
    ell: u64,
    seed: u64,
    engine: Engine,
    mut log: Option<&mut std::fs::File>,
) -> Out {
    use std::io::Write;
    let f = Fp::new(ell);
    let (fb, z, a) = fast::prepare(c);
    let (map, reps) = partition(c, &fb, ell, z, &a);
    let cols = reps.len();
    assert!(cols > 0);
    let normalization = source_normalization::equation(c, source, &fb, &map, cols, ell);
    let mut cores = vec![0usize];
    if let Some((_, v)) = &normalization {
        for j in 0..cols {
            if v[j] != 0 && !cores.contains(&j) {
                cores.push(j)
            }
        }
    }
    if c.g >= 3 {
        for j in 0..cols {
            if cores.len() >= cols.div_ceil(2) {
                break;
            }
            if !cores.contains(&j) {
                cores.push(j)
            }
        }
    }
    let small = cores.len();
    let lps: Vec<_> = (0..cols).filter(|j| !cores.contains(j)).collect();
    let perm: Vec<_> = cores.iter().chain(&lps).copied().collect();
    let lpfirst: Vec<_> = lps.iter().chain(&cores).copied().collect();
    let mut online = incremental::Online::new(small, lps.len(), ell);
    let mut sparse = sparse_full::Sparse::new(cols + 1, ell);
    let mut prefix = 0;
    let mut insert = |v: &[u64], rhs: u64, trial: u64| -> (Option<u64>, usize) {
        let mut vo: Vec<_> = perm.iter().map(|&j| v[j]).collect();
        vo.push(v[cols]);
        let mut vs: Vec<_> = lpfirst.iter().map(|&j| v[j]).collect();
        vs.push(v[cols]);
        if !matches!(engine, Engine::Sparse) {
            online.insert(&vo, rhs)
        }
        if !matches!(engine, Engine::Online) {
            sparse.insert(&vs, rhs)
        }
        let ok = online.k();
        let sk = sparse.k();
        if matches!(engine, Engine::Gate) {
            assert_eq!(ok, sk);
            assert_eq!(online.lp_rank + online.label_rank, sparse.rank);
            assert!(online.max_lp_width <= 2);
            prefix += 1;
            if let Some(file) = log.as_deref_mut() {
                writeln!(
                    file,
                    "{}\t{}\t{trial}\t{prefix}\t{}\t{}",
                    c.g,
                    c.fp.p,
                    sparse.rank,
                    ok.map_or("none".to_string(), |x| x.to_string())
                )
                .unwrap();
            }
        }
        if matches!(engine, Engine::Sparse) {
            (sk, sparse.rank)
        } else {
            (ok, online.lp_rank + online.label_rank)
        }
    };
    if let Some((_, v)) = normalization {
        let mut row = v;
        row.push(0);
        insert(&row, 1, 0);
    }
    let steps: Vec<_> = (0..fb.len()).filter(|&j| map[j].0 == 0).collect();
    assert_eq!(steps.len(), 2 * c.g + 1);
    let mut rng = Rng::new(seed);
    let mut position = c.add(source, target);
    let (mut aa, mut bb, mut state) = (1, 1, 0);
    let mut oracle = OracleStats::default();
    let mut by_x = vec![NONE; c.fp.p as usize];
    for (j, p) in fb.iter().enumerate() {
        by_x[p.0 as usize] = j
    }
    let mut out = Out {
        k: None,
        trials: 0,
        accepted: 0,
        rejected: 0,
        core: small,
        rank: 0,
    };
    for trial in 1..=200000 {
        out.trials = trial;
        match rng.below(4) {
            0 => {
                aa = f.add(aa, 1);
                position = c.add(&position, source)
            }
            1 => {
                bb = f.add(bb, 1);
                position = c.add(&position, target)
            }
            _ => {
                let j = steps[rng.below(steps.len() as u64) as usize];
                let negative = rng.below(2) == 1;
                let y = if negative { c.fp.neg(fb[j].1) } else { fb[j].1 };
                let weight = if negative { f.neg(map[j].1) } else { map[j].1 };
                state = f.add(state, weight);
                position = c.add_place(&position, fb[j].0, y)
            }
        }
        let Some(roots) = split_reference(&position.u, &c.fp, &mut oracle) else {
            continue;
        };
        let mut v = vec![0; cols + 1];
        for &(x, m) in roots.as_slice() {
            let j = by_x[x as usize];
            assert_ne!(j, NONE);
            let y = position.v.eval(x, &c.fp);
            let co = if y == fb[j].1 {
                f.from_i64(m as i64)
            } else {
                assert_eq!(y, c.fp.neg(fb[j].1));
                f.from_i64(-(m as i64))
            };
            let (r, w) = map[j];
            if r != NONE {
                v[r] = f.add(v[r], f.mul(co, w))
            }
        }
        v[0] = f.sub(v[0], state);
        v[cols] = f.neg(bb);
        if lps.iter().filter(|&&j| v[j] != 0).count() > 2 {
            out.rejected += 1;
            continue;
        }
        out.accepted += 1;
        let (k, rank) = insert(&v, aa, trial);
        out.rank = rank;
        if let Some(k) = k {
            assert_eq!(c.mul(source, k), *target);
            out.k = Some(k);
            return out;
        }
    }
    out
}
