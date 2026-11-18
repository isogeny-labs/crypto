//! LANE nested-dissection (research/ic_sketch_mixed_solver_20261115/alt/nested-dissection).
//!
//! Fill-reducing elimination orders for the 2-core that mixed-peel leaves.
//! The peeling and the Markowitz elimination are re-implemented from
//! `solve_square` (src is not edited); the matched square system comes from
//! the library `MatchGate`. Orders: markowitz (dynamic, baseline), amd
//! (approximate minimum degree on the column-interaction graph, quotient
//! graph, no supervariables), nd (recursive bisection with BFS level-set
//! separators). All counts are exact and deterministic.
//!
//! cargo run --release --example ic_alt_nested_dissection -- OUT.jsonl [SEEDS]
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crypto_lib::cryptanalysis::ic_boundary::RowStatus;
use crypto_lib::cryptanalysis::ic_framework::linalg::matrix_by_name;
use crypto_lib::cryptanalysis::ic_framework::linalg_mixed::MatchGate;

const MODULUS: u64 = (1u64 << 61) - 1;
const CAP: u64 = 40_000_000;

fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MODULUS as u128) as u64
}
fn addmod(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % MODULUS as u128) as u64
}
fn submod(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + MODULUS - b
    }
}
fn invmod(a: u64) -> Option<u64> {
    if a == 0 {
        return None;
    }
    // Fermat: a^(p-2)
    let (mut r, mut b, mut e) = (1u64, a, MODULUS - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, b);
        }
        b = mulmod(b, b);
        e >>= 1;
    }
    Some(r)
}

type Row = (Vec<(u32, u64)>, u64);

/// Same RNG order as ic_mixed_solver_stream::stream, rows produced lazily and sparse.
struct Stream {
    rng: StdRng,
    fb: usize,
    m: usize,
    x: Vec<u64>,
}
impl Stream {
    fn new(seed: u64, fb: usize, m: usize) -> Self {
        let cols = fb + 1;
        let mut rng = StdRng::seed_from_u64(
            0x1C_5EED ^ seed.wrapping_mul(0x9E37_79B9) ^ (fb as u64) << 40 ^ (m as u64) << 56,
        );
        let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..MODULUS)).collect();
        Stream { rng, fb, m, x }
    }
    fn next_row(&mut self) -> Row {
        let mut mp: BTreeMap<u32, u64> = BTreeMap::new();
        for _ in 0..self.m {
            let c = self.rng.gen_range(0..self.fb) as u32;
            let v = self.rng.gen_range(1..MODULUS);
            mp.insert(c, v);
        }
        mp.insert(self.fb as u32, self.rng.gen_range(1..MODULUS));
        let rhs = mp
            .iter()
            .fold(0, |a, (&c, &p)| addmod(a, mulmod(p, self.x[c as usize])));
        (mp.into_iter().collect(), rhs)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Markowitz,
    Amd,
    Nd,
}

#[derive(Default, Debug, Clone)]
struct Rep {
    arith: u64,
    book: u64,
    order_ops: u64, // part of book: graph build + ordering
    inversions: u64,
    forward: u64,
    backward: u64,
    core_dim: u64,
    core_edges: u64,
    fill: u64,
    l_nnz: u64, // sum of pivot-row lengths over core pivots
    max_row: u64,
    aborted: bool,
}
impl Rep {
    fn total(&self) -> u64 {
        self.arith + self.book
    }
}

// ───────────────────────── orderings ─────────────────────────

/// Approximate minimum degree on a quotient graph (no supervariables, no mass elimination).
fn amd_order(n: usize, adj: &[Vec<u32>], ops: &mut u64) -> Vec<u32> {
    let mut a: Vec<BTreeSet<u32>> = adj.iter().map(|v| v.iter().copied().collect()).collect();
    let mut e: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); n];
    let mut l: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); n];
    let mut elem_alive = vec![false; n];
    let mut alive = vec![true; n];
    let mut deg: Vec<usize> = a.iter().map(|s| s.len()).collect();
    let mut heap: BTreeSet<(usize, u32)> = (0..n as u32).map(|i| (deg[i as usize], i)).collect();
    let mut w: Vec<i64> = vec![-1; n];
    let mut stamp = vec![0u32; n];
    let mut epoch = 0u32;
    let mut order = Vec::with_capacity(n);
    for k in 0..n {
        let &(_, p) = heap.iter().next().expect("var");
        heap.remove(&(deg[p as usize], p));
        *ops += 1;
        let pu = p as usize;
        let mut lp: BTreeSet<u32> = BTreeSet::new();
        for &v in &a[pu] {
            *ops += 1;
            if alive[v as usize] && v != p {
                lp.insert(v);
            }
        }
        let ep: Vec<u32> = e[pu].iter().copied().collect();
        for &el in &ep {
            for &v in &l[el as usize] {
                *ops += 1;
                if v != p && alive[v as usize] {
                    lp.insert(v);
                }
            }
            elem_alive[el as usize] = false;
        }
        alive[pu] = false;
        order.push(p);
        epoch += 1;
        for &v in &lp {
            stamp[v as usize] = epoch;
        }
        for &el in &ep {
            l[el as usize].clear();
        }
        // pass 1: prune and compute w(e)=|Le \ Lp|
        let lpv: Vec<u32> = lp.iter().copied().collect();
        let mut touched: Vec<u32> = Vec::new();
        for &i in &lpv {
            let iu = i as usize;
            heap.remove(&(deg[iu], i));
            *ops += 1;
            // A_i := A_i \ (Lp u {p})
            let before = a[iu].len();
            a[iu].retain(|&v| v != p && stamp[v as usize] != epoch);
            *ops += before as u64;
            // E_i := E_i \ Ep
            for &el in &ep {
                e[iu].remove(&el);
            }
            *ops += ep.len() as u64;
            for &el in e[iu].iter() {
                *ops += 1;
                if w[el as usize] < 0 {
                    w[el as usize] = l[el as usize].len() as i64;
                    touched.push(el);
                }
                w[el as usize] -= 1;
            }
        }
        // pass 2: new degrees
        let remaining = n - k - 1;
        let lpn = lpv.len();
        for &i in &lpv {
            let iu = i as usize;
            let mut sumw = 0usize;
            let mut dead: Vec<u32> = Vec::new();
            for &el in e[iu].iter() {
                *ops += 1;
                let we = w[el as usize];
                if we == 0 {
                    dead.push(el);
                } else {
                    sumw += we as usize;
                }
            }
            for el in dead {
                e[iu].remove(&el);
                elem_alive[el as usize] = false;
            }
            e[iu].insert(p);
            let d = (deg[iu] + lpn - 1)
                .min(a[iu].len() + (lpn - 1) + sumw)
                .min(remaining.saturating_sub(1));
            deg[iu] = d;
            heap.insert((d, i));
            *ops += 1;
        }
        for &el in &touched {
            w[el as usize] = -1;
            if !elem_alive[el as usize] {
                l[el as usize].clear();
            }
        }
        l[pu] = lp;
        elem_alive[pu] = true;
    }
    order
}

/// Nested dissection by recursive bisection with BFS level-set separators.
fn nd_order(n: usize, adj: &[Vec<u32>], ops: &mut u64, stats: &mut (u64, u64)) -> Vec<u32> {
    let mut order: Vec<u32> = Vec::with_capacity(n);
    let mut in_set = vec![false; n];
    let all: Vec<u32> = (0..n as u32).collect();
    nd_rec(&all, adj, &mut in_set, &mut order, ops, 0, stats);
    order
}

fn bfs_levels(
    start: u32,
    set: &[u32],
    adj: &[Vec<u32>],
    in_set: &[bool],
    dist: &mut [i32],
    ops: &mut u64,
) -> Vec<Vec<u32>> {
    for &v in set {
        dist[v as usize] = -1;
    }
    let mut levels: Vec<Vec<u32>> = vec![vec![start]];
    dist[start as usize] = 0;
    loop {
        let mut nxt = Vec::new();
        for &u in levels.last().unwrap() {
            for &v in &adj[u as usize] {
                *ops += 1;
                if in_set[v as usize] && dist[v as usize] < 0 {
                    dist[v as usize] = levels.len() as i32;
                    nxt.push(v);
                }
            }
        }
        if nxt.is_empty() {
            break;
        }
        levels.push(nxt);
    }
    levels
}

fn nd_rec(
    set: &[u32],
    adj: &[Vec<u32>],
    in_set: &mut Vec<bool>,
    order: &mut Vec<u32>,
    ops: &mut u64,
    depth: u32,
    stats: &mut (u64, u64),
) {
    if set.is_empty() {
        return;
    }
    const LEAF: usize = 12;
    for &v in set {
        in_set[v as usize] = true;
    }
    let mut dist = vec![-1i32; in_set.len()];
    // components
    let mut comps: Vec<Vec<u32>> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for &s in set {
        if seen.contains(&s) {
            continue;
        }
        let lv = bfs_levels(s, set, adj, in_set, &mut dist, ops);
        let comp: Vec<u32> = lv.into_iter().flatten().collect();
        for &v in &comp {
            seen.insert(v);
        }
        comps.push(comp);
    }
    if comps.len() > 1 {
        for &v in set {
            in_set[v as usize] = false;
        }
        for c in comps {
            nd_rec(&c, adj, in_set, order, ops, depth, stats);
        }
        return;
    }
    let comp = &comps[0];
    let finish_leaf =
        |comp: &Vec<u32>, in_set: &mut Vec<bool>, order: &mut Vec<u32>, ops: &mut u64| {
            let mut v: Vec<(usize, u32)> = comp
                .iter()
                .map(|&x| {
                    *ops += adj[x as usize].len() as u64;
                    (
                        adj[x as usize]
                            .iter()
                            .filter(|&&y| in_set[y as usize])
                            .count(),
                        x,
                    )
                })
                .collect();
            v.sort();
            for (_, x) in v {
                order.push(x);
            }
            for &x in comp {
                in_set[x as usize] = false;
            }
        };
    if comp.len() <= LEAF {
        finish_leaf(comp, in_set, order, ops);
        return;
    }
    // pseudo-peripheral start: two sweeps
    let l0 = bfs_levels(comp[0], comp, adj, in_set, &mut dist, ops);
    let far = *l0
        .last()
        .unwrap()
        .iter()
        .min_by_key(|&&v| adj[v as usize].len())
        .unwrap();
    let lv = bfs_levels(far, comp, adj, in_set, &mut dist, ops);
    let k = lv.len();
    if k < 3 {
        finish_leaf(comp, in_set, order, ops);
        return;
    }
    let total = comp.len();
    let mut before = 0usize;
    let mut best: Option<(usize, usize)> = None; // (score, level)
    for s in 1..k - 1 {
        before += lv[s - 1].len();
        let after = total - before - lv[s].len();
        let score = lv[s].len() * 2 + before.abs_diff(after);
        if best.is_none_or(|(b, _)| score < b) {
            best = Some((score, s));
        }
    }
    let s = best.unwrap().1;
    let sep = lv[s].clone();
    let a: Vec<u32> = lv[..s].iter().flatten().copied().collect();
    let b: Vec<u32> = lv[s + 1..].iter().flatten().copied().collect();
    if depth == 0 {
        stats.0 = sep.len() as u64;
        stats.1 = total as u64;
    }
    for &v in comp {
        in_set[v as usize] = false;
    }
    nd_rec(&a, adj, in_set, order, ops, depth + 1, stats);
    nd_rec(&b, adj, in_set, order, ops, depth + 1, stats);
    // separator last, ordered by degree inside it
    for &v in &sep {
        in_set[v as usize] = true;
    }
    finish_leaf(&sep, in_set, order, ops);
}

// ───────────────────────── solver ─────────────────────────

fn solve(
    rows: &[Row],
    matched: &[usize],
    n: usize,
    kind: Kind,
    extra: &mut (u64, u64),
) -> Result<(Vec<u64>, Rep), Rep> {
    let mut rep = Rep::default();
    let mut w: Vec<BTreeMap<u32, u64>> = matched
        .iter()
        .map(|&r| rows[r].0.iter().copied().collect())
        .collect();
    let mut rhs: Vec<u64> = matched.iter().map(|&r| rows[r].1).collect();
    let mut col_rows: Vec<Vec<u32>> = vec![Vec::new(); n];
    for (i, row) in w.iter().enumerate() {
        for &c in row.keys() {
            col_rows[c as usize].push(i as u32);
        }
    }
    let mut row_alive = vec![true; n];
    let mut col_alive = vec![true; n];
    let mut col_w: Vec<usize> = col_rows.iter().map(|v| v.len()).collect();
    let mut x = vec![0u64; n];
    let mut known = vec![false; n];
    let mut stack: Vec<(u32, u32)> = Vec::new();
    let mut frozen: Vec<Option<(BTreeMap<u32, u64>, u64)>> = vec![None; n];

    let live_rows = |c: usize,
                     col_rows: &Vec<Vec<u32>>,
                     w: &Vec<BTreeMap<u32, u64>>,
                     row_alive: &Vec<bool>|
     -> Vec<u32> {
        let mut v: Vec<u32> = col_rows[c]
            .iter()
            .copied()
            .filter(|&i| row_alive[i as usize] && w[i as usize].contains_key(&(c as u32)))
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };

    // peeling, identical to solve_square; ops classified arith / book
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..n {
            if !row_alive[i] || w[i].len() != 1 {
                continue;
            }
            let (&c, &a) = w[i].iter().next().unwrap();
            let inv = invmod(a).ok_or(rep.clone())?;
            rep.inversions += 1;
            rep.arith += 1;
            let v = mulmod(rhs[i], inv);
            let c = c as usize;
            x[c] = v;
            known[c] = true;
            row_alive[i] = false;
            col_alive[c] = false;
            rep.forward += 1;
            for j in live_rows(c, &col_rows, &w, &row_alive) {
                let j = j as usize;
                let a_jc = w[j].remove(&(c as u32)).unwrap();
                rep.arith += 1;
                rhs[j] = submod(rhs[j], mulmod(a_jc, v));
            }
            col_w[c] = 0;
            changed = true;
        }
        for c in 0..n {
            if !col_alive[c] {
                continue;
            }
            let live = live_rows(c, &col_rows, &w, &row_alive);
            rep.book += 1;
            col_w[c] = live.len();
            if live.len() != 1 {
                continue;
            }
            let i = live[0] as usize;
            row_alive[i] = false;
            col_alive[c] = false;
            rep.backward += 1;
            for &c2 in w[i].keys() {
                if (c2 as usize) != c {
                    rep.book += 1;
                    col_w[c2 as usize] = col_w[c2 as usize].saturating_sub(1);
                }
            }
            frozen[i] = Some((std::mem::take(&mut w[i]), rhs[i]));
            stack.push((i as u32, c as u32));
            changed = true;
        }
    }

    let core: Vec<usize> = (0..n).filter(|&c| col_alive[c]).collect();
    rep.core_dim = core.len() as u64;
    // static order for AMD / ND over the core column-interaction graph
    let mut static_order: Vec<usize> = Vec::new();
    if kind != Kind::Markowitz && !core.is_empty() {
        let idx: BTreeMap<usize, u32> = core
            .iter()
            .enumerate()
            .map(|(k, &c)| (c, k as u32))
            .collect();
        let mut sets: Vec<BTreeSet<u32>> = vec![BTreeSet::new(); core.len()];
        let mut ops = 0u64;
        for i in 0..n {
            if !row_alive[i] {
                continue;
            }
            let cs: Vec<u32> = w[i].keys().map(|c| idx[&(*c as usize)]).collect();
            for &a in &cs {
                for &b in &cs {
                    ops += 1;
                    if a != b {
                        sets[a as usize].insert(b);
                    }
                }
            }
        }
        let adj: Vec<Vec<u32>> = sets.iter().map(|s| s.iter().copied().collect()).collect();
        rep.core_edges = adj.iter().map(|v| v.len() as u64).sum::<u64>() / 2;
        let ord = match kind {
            Kind::Amd => amd_order(core.len(), &adj, &mut ops),
            Kind::Nd => nd_order(core.len(), &adj, &mut ops, extra),
            Kind::Markowitz => unreachable!(),
        };
        assert_eq!(ord.len(), core.len());
        static_order = ord.into_iter().map(|k| core[k as usize]).collect();
        rep.order_ops = ops;
        rep.book += ops;
    }
    let mut remaining = core.len();
    let mut step = 0usize;
    while remaining > 0 {
        let c = if kind == Kind::Markowitz {
            let mut best: Option<(usize, usize)> = None;
            for &c in &core {
                if !col_alive[c] {
                    continue;
                }
                rep.book += 1;
                let wc = col_w[c];
                if best.is_none_or(|(_, bw)| wc < bw) {
                    best = Some((c, wc));
                }
            }
            best.unwrap().0
        } else {
            let c = static_order[step];
            step += 1;
            c
        };
        let live = live_rows(c, &col_rows, &w, &row_alive);
        if live.is_empty() {
            return Err(rep);
        }
        let p = *live.iter().min_by_key(|&&i| w[i as usize].len()).unwrap() as usize;
        let a_pc = w[p][&(c as u32)];
        let inv = invmod(a_pc).ok_or(rep.clone())?;
        rep.inversions += 1;
        let prow: Vec<(u32, u64)> = w[p].iter().map(|(&k, &v)| (k, v)).collect();
        rep.l_nnz += prow.len() as u64;
        rep.max_row = rep.max_row.max(prow.len() as u64);
        for &j in &live {
            let j = j as usize;
            if j == p {
                continue;
            }
            let a_jc = w[j][&(c as u32)];
            let f = mulmod(a_jc, inv);
            rep.arith += 1;
            for &(k, v) in &prow {
                rep.arith += 1;
                let e = w[j].entry(k).or_insert_with(|| {
                    rep.fill += 1;
                    col_rows[k as usize].push(j as u32);
                    col_w[k as usize] += 1;
                    0
                });
                *e = submod(*e, mulmod(f, v));
                if *e == 0 {
                    w[j].remove(&k);
                    col_w[k as usize] = col_w[k as usize].saturating_sub(1);
                }
            }
            rhs[j] = submod(rhs[j], mulmod(f, rhs[p]));
            rep.arith += 1;
        }
        if rep.total() > CAP {
            rep.aborted = true;
            return Err(rep);
        }
        row_alive[p] = false;
        col_alive[c] = false;
        for &(k, _) in &prow {
            col_w[k as usize] = col_w[k as usize].saturating_sub(1);
        }
        frozen[p] = Some((std::mem::take(&mut w[p]), rhs[p]));
        stack.push((p as u32, c as u32));
        remaining -= 1;
    }
    for &(i, c) in stack.iter().rev() {
        let (row, b) = frozen[i as usize].as_ref().unwrap();
        let mut acc = *b;
        let mut a_c = 0;
        for (&k, &v) in row {
            if k == c {
                a_c = v;
                continue;
            }
            rep.arith += 1;
            acc = submod(acc, mulmod(v, x[k as usize]));
        }
        let inv = invmod(a_c).ok_or(rep.clone())?;
        rep.inversions += 1;
        rep.arith += 1;
        x[c as usize] = mulmod(acc, inv);
        known[c as usize] = true;
    }
    if known.iter().any(|k| !k) {
        return Err(rep);
    }
    Ok((x, rep))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "nd.jsonl".into());
    let seeds: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(16);
    let only_fb: Option<usize> = args.get(3).and_then(|s| s.parse().ok());
    let mut f = std::fs::File::create(&out).expect("create");
    for &fb in &[128usize, 256, 512, 1024] {
        if only_fb.is_some_and(|o| o != fb) {
            continue;
        }
        for &m in &[2usize, 3, 4] {
            for seed in 0..seeds {
                let cols = fb + 1;
                let mut st = Stream::new(seed, fb, m);
                let mut gate = MatchGate::new(cols);
                let mut rows: Vec<Row> = Vec::new();
                while gate.size() < cols {
                    let r = st.next_row();
                    let cs: Vec<u32> = r.0.iter().map(|e| e.0).collect();
                    gate.add_row(&cs);
                    rows.push(r);
                    if rows.len() > 60 * cols {
                        panic!("no full structural rank");
                    }
                }
                let matched: Vec<usize> = (0..cols).map(|c| gate.row_of(c).unwrap()).collect();
                let mut is_m = vec![false; rows.len()];
                for &r in &matched {
                    is_m[r] = true;
                }
                let conv: u64 = rows.iter().map(|r| r.0.len() as u64).sum();
                let surplus: u64 = rows
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !is_m[*i])
                    .map(|(_, r)| r.0.len() as u64)
                    .sum();
                // library mixed-peel on the same rows (cross-check)
                let mut lib = matrix_by_name("mixed-peel", cols, MODULUS).unwrap();
                for (e, rhs) in &rows {
                    let mut d = vec![0u64; cols];
                    for &(c, v) in e {
                        d[c as usize] = v;
                    }
                    let s = lib.add_row(d, *rhs);
                    assert!(s != RowStatus::Inconsistent);
                    let _ = lib.pinned(fb);
                }
                let lib_total = lib.work().0;
                let lib_pin = lib.pinned(fb);
                let lib_solve = lib_total - conv - gate.visits - surplus;
                let wrap = conv + gate.visits + surplus; // non-solve part of mixed-peel row_ops
                for (name, kind) in [
                    ("markowitz", Kind::Markowitz),
                    ("amd", Kind::Amd),
                    ("nd", Kind::Nd),
                ] {
                    let mut extra = (0u64, 0u64);
                    let t0 = std::time::Instant::now();
                    let res = solve(&rows, &matched, cols, kind, &mut extra);
                    let ms = t0.elapsed().as_secs_f64() * 1e3;
                    let (ok, correct, rep) = match res {
                        Ok((x, rep)) => (true, x == st.x, rep),
                        Err(rep) => (false, false, rep),
                    };
                    let rec = serde_json::json!({
                        "fb": fb, "m": m, "seed": seed, "order": name, "rows_read": rows.len(),
                        "solved": ok, "correct": correct, "aborted": rep.aborted,
                        "core_dim": rep.core_dim, "core_edges": rep.core_edges,
                        "forward": rep.forward, "backward": rep.backward,
                        "fill": rep.fill, "pivot_nnz": rep.l_nnz, "max_pivot_row": rep.max_row,
                        "arith": rep.arith, "book": rep.book, "order_ops": rep.order_ops,
                        "solve_row_ops": rep.total(), "wrap_row_ops": wrap,
                        "mixedpeel_equiv_row_ops": rep.total() + wrap,
                        "lib_mixed_peel_row_ops": lib_total, "lib_solve_part": lib_solve,
                        "lib_correct": lib_pin == Some(st.x[fb]),
                        "lib_matches_reimpl": if kind == Kind::Markowitz { Some(lib_solve == rep.total()) } else { None },
                        "nd_top_sep": extra.0, "nd_top_comp": extra.1, "ms": ms,
                    });
                    writeln!(f, "{rec}").unwrap();
                }
            }
            eprintln!("fb={fb} m={m} done");
        }
    }
}
