//! LANE dulmage-mendelsohn of `research/ic_sketch_mixed_solver_20261115/alt`.
//!
//! Planted IC-shaped streams (generator copied from `ic_mixed_solver_stream`:
//! `m` non-zeros on uniform factor-base columns plus one on the target column
//! in EVERY row).  For each (|F|, m, seed):
//!
//!  * the library arms `incremental-gauss`, `incremental-gauss-echelon`,
//!    `mixed-peel` run exactly as in the stream example (rows read, row_ops);
//!  * the DM arm keeps a maximum matching of the factor-base part B of the rows
//!    (Kuhn augmenting path per arriving row, every adjacency probe counted).
//!    The target is pinned iff some combination of rows cancels B and keeps the
//!    target coefficient, i.e. iff the rows of B are dependent (target column
//!    is in every row).  The first row whose augmentation fails closes the
//!    Hall violator: the rows reached by the failed alternating search are the
//!    unique matroid circuit = the horizontal DM block of B.  Only that block
//!    is handed to a library solver (rows restricted to the block's columns
//!    plus the target);
//!  * verification: slack vs the library pin, circuit == coarse DM block from a
//!    from-scratch maximum matching, circuit minimality (S-i matchable for all
//!    i), block solve equals the planted target, S-i never pins, and the fine
//!    DM block (Tarjan SCC) of the full matrix A containing the target column.
//!
//! ```text
//! cargo run --release --example ic_alt_dulmage_mendelsohn -- OUT.jsonl [SEEDS]
//! ```
use std::collections::BTreeSet;
use std::io::Write;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crypto_lib::cryptanalysis::ic_framework::linalg::{is_deferred_sparse, matrix_by_name};

const MODULUS: u64 = (1u64 << 61) - 1;

fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % MODULUS as u128) as u64
}
fn addmod(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % MODULUS as u128) as u64
}

type Stream = (Vec<u64>, Vec<(Vec<u64>, u64)>);

/// Same generator as ic_mixed_solver_stream (identical prefix for any `len`).
fn stream(seed: u64, fb: usize, m: usize, len: usize) -> Stream {
    let cols = fb + 1;
    let mut rng = StdRng::seed_from_u64(
        0x1C_5EED ^ seed.wrapping_mul(0x9E37_79B9) ^ (fb as u64) << 40 ^ (m as u64) << 56,
    );
    let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..MODULUS)).collect();
    let rows = (0..len)
        .map(|_| {
            let mut row = vec![0u64; cols];
            for _ in 0..m {
                row[rng.gen_range(0..fb)] = rng.gen_range(1..MODULUS);
            }
            row[fb] = rng.gen_range(1..MODULUS);
            let rhs = row
                .iter()
                .zip(&x)
                .fold(0, |a, (&p, &q)| addmod(a, mulmod(p, q)));
            (row, rhs)
        })
        .collect();
    (x, rows)
}

/// Incremental maximum matching (rows to columns) with exact probe counts.
struct Matching {
    adj: Vec<Vec<usize>>,         // row -> columns
    mate_col: Vec<Option<usize>>, // column -> row
    mate_row: Vec<Option<usize>>,
    vis_col: Vec<bool>,
    probes: u64,
    // set on a failed augmentation
    reach_rows: Vec<usize>,
    reach_cols: Vec<usize>,
}

impl Matching {
    fn new(ncols: usize) -> Self {
        Self {
            adj: vec![],
            mate_col: vec![None; ncols],
            mate_row: vec![],
            vis_col: vec![false; ncols],
            probes: 0,
            reach_rows: vec![],
            reach_cols: vec![],
        }
    }
    fn dfs(&mut self, r: usize) -> bool {
        self.reach_rows.push(r);
        for k in 0..self.adj[r].len() {
            let c = self.adj[r][k];
            self.probes += 1;
            if self.vis_col[c] {
                continue;
            }
            self.vis_col[c] = true;
            self.reach_cols.push(c);
            let ok = match self.mate_col[c] {
                None => true,
                Some(r2) => self.dfs(r2),
            };
            if ok {
                self.mate_col[c] = Some(r);
                self.mate_row[r] = Some(c);
                return true;
            }
        }
        false
    }
    /// Add a row; true if the matching grew (row independent in the
    /// transversal matroid).  On false, reach_rows/reach_cols hold the Hall
    /// violator (rows = circuit).
    fn add(&mut self, cols: Vec<usize>) -> bool {
        let r = self.adj.len();
        self.adj.push(cols);
        self.mate_row.push(None);
        for v in self.vis_col.iter_mut() {
            *v = false;
        }
        self.reach_rows.clear();
        self.reach_cols.clear();
        self.dfs(r)
    }
}

/// Run a library solver on rows `sel` restricted to `cols_sel` + target.
/// Returns (pinned value, row_ops).  Rows are fed in the stream order.
fn block_solve(
    name: &str,
    rows: &[(Vec<u64>, u64)],
    sel: &[usize],
    cols_sel: &[usize],
    fb: usize,
    skip: Option<usize>,
) -> (Option<u64>, u64) {
    let nc = cols_sel.len() + 1;
    let mut mat = matrix_by_name(name, nc, MODULUS).expect("arm");
    let mut ans = None;
    for &ri in sel {
        if Some(ri) == skip {
            continue;
        }
        let (full, rhs) = &rows[ri];
        let mut d = vec![0u64; nc];
        for (k, &c) in cols_sel.iter().enumerate() {
            d[k] = full[c];
        }
        d[nc - 1] = full[fb];
        mat.add_row(d, *rhs);
        if let Some(p) = mat.pinned(nc - 1) {
            ans = Some(p);
            break;
        }
    }
    (ans, mat.work().0)
}

/// Tarjan SCC over columns of the matched digraph of A; returns the SCC
/// (columns) containing `t` and the number of rows matched into it.
fn fine_block_of_target(
    rows: &[(Vec<u64>, u64)],
    n: usize,
    ncols: usize,
    t: usize,
) -> (usize, usize, usize, u64) {
    let mut m = Matching::new(ncols);
    for r in 0..n {
        let cols: Vec<usize> = (0..ncols).filter(|&c| rows[r].0[c] != 0).collect();
        m.add(cols);
    }
    let matched_rows = m.mate_row.iter().filter(|x| x.is_some()).count();
    // digraph on columns: for matched row r with mate column mc, for each
    // adjacent column c != mc: edge c -> mc.
    let mut g: Vec<Vec<usize>> = vec![vec![]; ncols];
    let mut edges = 0u64;
    for r in 0..n {
        if let Some(mc) = m.mate_row[r] {
            for &c in &m.adj[r] {
                if c != mc {
                    g[c].push(mc);
                    edges += 1;
                }
            }
        }
    }
    // iterative Tarjan
    let mut index = vec![usize::MAX; ncols];
    let mut low = vec![0usize; ncols];
    let mut on = vec![false; ncols];
    let mut st = vec![];
    let mut comp = vec![usize::MAX; ncols];
    let mut idx = 0;
    let mut ncomp = 0;
    for s in 0..ncols {
        if index[s] != usize::MAX {
            continue;
        }
        let mut call: Vec<(usize, usize)> = vec![(s, 0)];
        index[s] = idx;
        low[s] = idx;
        idx += 1;
        st.push(s);
        on[s] = true;
        while let Some(&mut (v, ref mut i)) = call.last_mut() {
            if *i < g[v].len() {
                let w = g[v][*i];
                *i += 1;
                if index[w] == usize::MAX {
                    index[w] = idx;
                    low[w] = idx;
                    idx += 1;
                    st.push(w);
                    on[w] = true;
                    call.push((w, 0));
                } else if on[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                if low[v] == index[v] {
                    loop {
                        let w = st.pop().unwrap();
                        on[w] = false;
                        comp[w] = ncomp;
                        if w == v {
                            break;
                        }
                    }
                    ncomp += 1;
                }
                call.pop();
                if let Some(&(u, _)) = call.last() {
                    low[u] = low[u].min(low[v]);
                }
            }
        }
    }
    let ct = comp[t];
    let cols_in = (0..ncols).filter(|&c| comp[c] == ct).count();
    let rows_in = (0..n)
        .filter(|&r| m.mate_row[r].is_some_and(|mc| comp[mc] == ct))
        .count();
    (cols_in, rows_in, matched_rows, m.probes + edges)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out = args.get(1).cloned().unwrap_or_else(|| "dm.jsonl".into());
    let seeds: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(16);
    let mut f = std::fs::File::create(&out).expect("create output");
    for &fb in &[64usize, 128, 256, 512] {
        for &m in &[2usize, 3] {
            for seed in 0..seeds {
                let cols = fb + 1;
                let (x, rows) = stream(seed, fb, m, 12 * cols);
                let mut rec = serde_json::json!({"fb": fb, "m": m, "seed": seed});

                // library arms, stream-example loop
                for arm in [
                    "incremental-gauss",
                    "incremental-gauss-echelon",
                    "mixed-peel",
                ] {
                    let full_rank = is_deferred_sparse(arm);
                    let mut mat = matrix_by_name(arm, cols, MODULUS).unwrap();
                    let (mut read, mut ans) = (0usize, None);
                    for (row, rhs) in &rows {
                        read += 1;
                        mat.add_row(row.clone(), *rhs);
                        let pin = mat.pinned(fb);
                        if pin.is_some() && (!full_rank || mat.rank() == cols) {
                            ans = pin;
                            break;
                        }
                    }
                    rec[arm] = serde_json::json!({
                        "rows_read": read, "row_ops": mat.work().0,
                        "correct": ans == Some(x[fb]), "solved": ans.is_some(),
                    });
                }

                // DM arm: incremental matching of B until the first failure
                let mut mt = Matching::new(fb);
                let mut oracle_rows = None;
                for (i, (row, _)) in rows.iter().enumerate() {
                    let bc: Vec<usize> = (0..fb).filter(|&c| row[c] != 0).collect();
                    if !mt.add(bc) {
                        oracle_rows = Some(i + 1);
                        break;
                    }
                }
                let n = oracle_rows.expect("B dependent by row fb+1");
                let mut s: Vec<usize> = mt.reach_rows.clone();
                s.sort();
                let mut cs: Vec<usize> = mt.reach_cols.clone();
                cs.sort();
                let probes = mt.probes;
                assert_eq!(cs.len() + 1, s.len(), "Hall violator |N|=|S|-1");
                let lib_n = rec["incremental-gauss"]["rows_read"].as_u64().unwrap() as usize;
                let lib_n_ech = rec["incremental-gauss-echelon"]["rows_read"]
                    .as_u64()
                    .unwrap() as usize;

                // block solves
                let (a_ig, ops_ig) = block_solve("incremental-gauss", &rows, &s, &cs, fb, None);
                let (a_ech, ops_ech) =
                    block_solve("incremental-gauss-echelon", &rows, &s, &cs, fb, None);

                // coarse DM of B at the pin prefix, from scratch: rows reachable
                // from the unmatched row(s) by alternating paths
                let mut m2 = Matching::new(fb);
                let mut unmatched = vec![];
                for r in 0..n {
                    let bc: Vec<usize> = (0..fb).filter(|&c| rows[r].0[c] != 0).collect();
                    if !m2.add(bc) {
                        unmatched.push(r);
                    }
                }
                // rows reachable from unmatched rows (matching is partial: the
                // failed rows stay unmatched)
                let mut seen_r: BTreeSet<usize> = unmatched.iter().copied().collect();
                let mut seen_c: BTreeSet<usize> = BTreeSet::new();
                let mut stack: Vec<usize> = unmatched.clone();
                while let Some(r) = stack.pop() {
                    for &c in &m2.adj[r] {
                        if seen_c.insert(c) {
                            if let Some(r2) = m2.mate_col[c] {
                                if seen_r.insert(r2) {
                                    stack.push(r2);
                                }
                            }
                        }
                    }
                }
                let coarse_rows: Vec<usize> = seen_r.into_iter().collect();

                // minimality: S-i is matchable (independent) for each i
                let mut minimal = true;
                for &skip in &s {
                    let mut mm = Matching::new(fb);
                    for &r in &s {
                        if r == skip {
                            continue;
                        }
                        let bc: Vec<usize> = (0..fb).filter(|&c| rows[r].0[c] != 0).collect();
                        if !mm.add(bc) {
                            minimal = false;
                        }
                    }
                }
                // S-i never pins (library solver), all i if small else 24 sampled
                let step = (s.len() / 24).max(1);
                let mut no_pin_without = true;
                let mut tested = 0;
                for (k, &skip) in s.iter().enumerate() {
                    if k % step != 0 {
                        continue;
                    }
                    tested += 1;
                    let (a, _) = block_solve("incremental-gauss", &rows, &s, &cs, fb, Some(skip));
                    if a.is_some() {
                        no_pin_without = false;
                    }
                }

                // fine DM block of the full matrix A containing the target
                let (fcols, frows, matched, fvisits) = fine_block_of_target(&rows, n, cols, fb);

                // rows that never touch the circuit
                rec["dm"] = serde_json::json!({
                    "oracle_rows": n,
                    "slack_vs_incremental_gauss": lib_n as i64 - n as i64,
                    "slack_vs_echelon": lib_n_ech as i64 - n as i64,
                    "circuit_rows": s.len(), "circuit_cols": cs.len(),
                    "circuit_equals_coarse_dm_block": coarse_rows == s,
                    "circuit_minimal": minimal,
                    "no_pin_without_any_row": no_pin_without, "no_pin_tested": tested,
                    "block_pin_correct_ig": a_ig == Some(x[fb]),
                    "block_pin_correct_echelon": a_ech == Some(x[fb]),
                    "matching_probes": probes,
                    "block_row_ops_ig": ops_ig, "block_row_ops_echelon": ops_ech,
                    "dm_total_ig": ops_ig + probes, "dm_total_echelon": ops_ech + probes,
                    "fineA_target_block_cols": fcols, "fineA_target_block_rows": frows,
                    "fineA_rows_matched": matched, "fineA_bookkeeping": fvisits,
                });
                writeln!(f, "{rec}").unwrap();
            }
            eprintln!("fb={fb} m={m} done");
        }
    }
}
