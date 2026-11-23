#![allow(dead_code)]
#[path = "../normalized/source.rs"]
mod source_normalization;
use hec_native_ic::{
    fp::Fp,
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
pub fn solve(
    c: &Curve,
    source: &Div,
    target: &Div,
    ell: u64,
    n: u64,
    seed: u64,
    mode: Mode,
) -> Report {
    hec_native_ic::counted::stage("entry");
    let f = Fp::new(ell);
    let (fb, map, reps) = crate::setup::prepare(c, source, ell, n);
    let m = fb.len();
    let folded = matches!(mode, Mode::Folded);
    let cols = if folded { reps.len() } else { m };
    hec_native_ic::counted::stage("orbit_rows");
    let mut rows: Vec<(Row, u64)> = vec![];
    if matches!(mode, Mode::Expanded) {
        for j in 0..m {
            let (r, w) = map[j];
            if r != NONE && reps[r] == j {
                continue;
            }
            let mut row = vec![0; m];
            row[j] = 1;
            if r != NONE {
                row[reps[r]] = f.sub(row[reps[r]], w);
            }
            rows.push((sparse(&row, 0), 0));
        }
        assert_eq!(rows.len(), m - reps.len());
    }
    hec_native_ic::counted::stage("source_normalization");
    let source_row = source_normalization::equation(c, source, &fb, &map, reps.len(), ell);
    let normalized = source_row.is_some();
    if let Some((full, fold)) = source_row {
        rows.push((sparse(if folded { &fold } else { &full }, 0), 1));
    }
    hec_native_ic::counted::stage("walk_setup");
    let branches = m.min(64);
    let mut rng = Rng::new(seed);
    let mut order: Vec<_> = (0..m).collect();
    for i in 0..branches {
        let j = i + rng.below((m - i) as u64) as usize;
        order.swap(i, j)
    }
    let steps = &order[..branches];
    let mut by_x = vec![NONE; c.fp.p as usize];
    for (j, p) in fb.iter().enumerate() {
        by_x[p.0 as usize] = j
    }
    let mut state = vec![0u64; if folded { cols } else { branches }];
    let mut position = c.add(source, target);
    let (mut a, mut b) = (1, 1);
    let mut oracle = OracleStats::default();
    let mut rep = Report {
        m,
        unknowns: cols,
        state_len: state.len(),
        ..Report::default()
    };
    rep.source_normalized = normalized;
    for trial in 1..=200000u64 {
        rep.trials = trial;
        hec_native_ic::counted::stage("walk");
        match rng.below(4) {
            0 => {
                a = f.add(a, 1);
                position = c.add(&position, source)
            }
            1 => {
                b = f.add(b, 1);
                position = c.add(&position, target)
            }
            _ => {
                let s = rng.below(branches as u64) as usize;
                let j = steps[s];
                if folded {
                    let (r, w) = map[j];
                    if r != NONE {
                        state[r] = f.add(state[r], w)
                    }
                } else {
                    state[s] = f.add(state[s], 1)
                }
                position = c.add_place(&position, fb[j].0, fb[j].1)
            }
        }
        hec_native_ic::counted::stage("smoothness");
        let Some(roots) = split_reference(&position.u, &c.fp, &mut oracle) else {
            continue;
        };
        hec_native_ic::counted::stage("row");
        let mut v = vec![0; cols];
        for &(x, multiplicity) in roots.as_slice() {
            let j = by_x[x as usize];
            assert_ne!(j, NONE);
            let y = position.v.eval(x, &c.fp);
            let co = if y == fb[j].1 {
                f.from_i64(multiplicity as i64)
            } else {
                assert_eq!(y, c.fp.neg(fb[j].1));
                f.from_i64(-(multiplicity as i64))
            };
            if folded {
                let (r, w) = map[j];
                if r != NONE {
                    v[r] = f.add(v[r], f.mul(co, w))
                }
            } else {
                v[j] = f.add(v[j], co)
            }
        }
        if folded {
            for j in 0..cols {
                v[j] = f.sub(v[j], state[j])
            }
        } else {
            for (s, &count) in state.iter().enumerate() {
                v[steps[s]] = f.sub(v[steps[s]], count)
            }
        }
        let row = sparse(&v, f.neg(b));
        rep.max_row_len = rep.max_row_len.max(row.len());
        rows.push((row, a));
        rep.relations += 1;
        let mut stats = SolveStats::default();
        hec_native_ic::counted::stage("elimination");
        let k = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
        rep.solve_mulmods += stats.mulmods;
        if let Some(k) = k {
            hec_native_ic::counted::stage("verification");
            assert_eq!(c.mul(source, k), *target);
            rep.k = Some(k);
            return rep;
        }
    }
    rep
}
