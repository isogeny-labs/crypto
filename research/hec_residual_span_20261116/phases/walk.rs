#[path = "../multi/algebra.rs"]
mod algebra;
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
fn partition(c: &Curve, fb: &[(u64, u64)], ell: u64) -> (Vec<(usize, u64)>, Vec<usize>) {
    let n = 2 * c.g + 1;
    assert_eq!(c.f.deg(), n as isize);
    assert_eq!(c.f.lc(), 1);
    assert!(c.f.c[1..n].iter().all(|&v| v == 0));
    assert_ne!(c.f.c[0], 0);
    assert_eq!((c.fp.p - 1) % n as u64, 0);
    let f = Fp::new(ell);
    let (z, a) = algebra::character(c);
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
// Public interface deliberately has no cofactor, source point index or expected scalar.
pub fn solve(c: &Curve, source: &Div, target: &Div, ell: u64, seed: u64, mode: Mode) -> Report {
    let f = Fp::new(ell);
    hec_native_ic::counted::stage("factor_base"); // PHASE
    let fb = factor_base(c);
    let m = fb.len();
    assert!(m > 0);
    hec_native_ic::counted::stage("action"); // PHASE
    let (map, reps) = if matches!(mode, Mode::Plain) {
        (vec![], vec![])
    } else {
        partition(c, &fb, ell)
    };
    hec_native_ic::counted::stage("walk_prep"); // PHASE
    let folded = matches!(mode, Mode::Folded);
    let cols = if folded { reps.len() } else { m };
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
    for trial in 1..=200000u64 {
        hec_native_ic::counted::stage("walk"); // PHASE
        rep.trials = trial;
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
        hec_native_ic::counted::stage("oracle"); // PHASE
        let Some(roots) = split_reference(&position.u, &c.fp, &mut oracle) else {
            continue;
        };
        hec_native_ic::counted::stage("rows"); // PHASE
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
        hec_native_ic::counted::stage("la"); // PHASE
        let k = solve_for_last(&rows, cols, ell, Solver::Fast, &[], &mut stats);
        rep.solve_mulmods += stats.mulmods;
        if let Some(k) = k {
            hec_native_ic::counted::stage("verify"); // PHASE
            assert_eq!(c.mul(source, k), *target);
            rep.k = Some(k);
            return rep;
        }
    }
    rep
}
