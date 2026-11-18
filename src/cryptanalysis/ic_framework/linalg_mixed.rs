//! # A structural-rank gate and a mixed peel/Markowitz relation solver.
//!
//! Two pieces for the full-rank relation matrices of `linalg.rs`.
//!
//! **[`MatchGate`]** keeps a maximum bipartite matching between the rows
//! received so far and the columns they touch, one augmenting-path search
//! per arriving row (Kuhn).  Its size is the *structural* rank: no
//! assignment of values to the non-zeros can give a larger rank, so while
//! it is below `cols` a full-rank solve is certain to fail.  Over `Z/rZ`
//! with `r` prime, once it reaches `cols` the matched square subsystem is
//! non-singular for all but a fraction of at most `cols / r` of coefficient
//! choices (Schwartz–Zippel on its determinant), so the first solve after
//! the gate opens almost always succeeds.  Every edge the search looks at
//! is counted (`gate_visits`), and the solvers here charge those visits in
//! their `row_ops`, one per visit, so the gate is never free.
//!
//! **[`MixedPeel`]** (`mixed-peel`) is a deferred solver that combines the
//! cheap pieces of the solvers beside it:
//!
//! 1. the gate decides *when* to solve (as soon as structural full rank
//!    holds, rather than on a fixed cadence);
//! 2. the matching decides *which* rows to solve: the `cols` matched rows
//!    form a square system whose non-zero pivots certify the rank, so no
//!    second independently seeded solve is needed (the deferred Krylov
//!    solvers need one because they cannot certify rank);
//! 3. that square system is peeled from both sides, as an invertible
//!    Bloom lookup table is decoded: a row with one live column fixes that
//!    column at once (forward substitution), and a column with one live
//!    row puts that row on the back-substitution stack;
//! 4. what is left (the 2-core) is eliminated by sparse Gaussian
//!    elimination with the Markowitz choice (lightest column, then its
//!    lightest row);
//! 5. the unmatched surplus rows are then checked against the solution,
//!    one multiply-add per non-zero, so a wrong relation shows up as
//!    `Inconsistent` exactly as a contradiction does in the other solvers.
//!
//! Optionally (`MixedPeel::with_bloom`) arriving rows are screened for
//! exact duplicates by a Bloom filter over a canonical row hash in front of
//! an exact check, the "sketch" component the study measures separately.
//!
//! Work unit: `row_ops`, one per non-zero multiply-subtract, one per
//! non-zero for converting each arriving row and for checking a row against
//! a solution, as [`SparseCado`](super::linalg::SparseCado) counts them;
//! plus one per gate visit, one per peeling weight update and one per
//! Bloom hash evaluation.  Inversions are counted beside it
//! (`work_counters`).
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::cryptanalysis::ic_boundary::{RelationSolver, RowStatus};

const NONE: u32 = u32::MAX;

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn addmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}
fn submod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + m as u128 - b as u128) % m as u128) as u64
}
/// `a^(m-2) mod m` (the modulus is prime); `None` for `a ≡ 0`.
fn invmod(a: u64, m: u64) -> Option<u64> {
    let a = a % m;
    if a == 0 {
        return None;
    }
    let (mut base, mut e, mut acc) = (a, m - 2, 1u64);
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, base, m);
        }
        base = mulmod(base, base, m);
        e >>= 1;
    }
    Some(acc)
}

/// Incremental maximum bipartite matching, rows against columns.
#[derive(Clone, Debug)]
pub struct MatchGate {
    adj: Vec<Vec<u32>>,
    match_col: Vec<u32>,
    match_row: Vec<u32>,
    stamp: Vec<u32>,
    via: Vec<u32>,
    epoch: u32,
    size: usize,
    /// Edges looked at by every search so far.
    pub visits: u64,
}

impl MatchGate {
    pub fn new(cols: usize) -> Self {
        Self {
            adj: Vec::new(),
            match_col: vec![NONE; cols],
            match_row: Vec::new(),
            stamp: vec![0; cols],
            via: vec![NONE; cols],
            epoch: 0,
            size: 0,
            visits: 0,
        }
    }

    /// The structural rank of the rows added so far.
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn cols(&self) -> usize {
        self.match_col.len()
    }

    /// The row matched to column `c`.
    pub fn row_of(&self, c: usize) -> Option<usize> {
        let r = self.match_col[c];
        (r != NONE).then_some(r as usize)
    }

    /// Add a row (its distinct non-zero columns) and try to grow the
    /// matching by one.  Returns whether it grew.  The matching stays
    /// maximum: a new row can raise the maximum by at most one, and only
    /// along an augmenting path that starts at it.
    pub fn add_row(&mut self, cols: &[u32]) -> bool {
        let r0 = self.adj.len() as u32;
        self.adj.push(cols.to_vec());
        self.match_row.push(NONE);
        if self.size == self.match_col.len() {
            return false;
        }
        // Greedy: a free column of the new row.
        for &c in cols {
            self.visits += 1;
            if self.match_col[c as usize] == NONE {
                self.match_col[c as usize] = r0;
                self.match_row[r0 as usize] = c;
                self.size += 1;
                return true;
            }
        }
        // Columns stamped with the current epoch are either visited by this
        // search or proven dead by an earlier failed one: from a dead column
        // no alternating path reaches a free column, and that stays true
        // until the matching changes, because a new (unmatched) row adds
        // edges only at a vertex no alternating path enters.  So the epoch
        // moves only after an augmentation, and the failed searches between
        // two augmentations cost O(edges) together, not each.
        if self.epoch == 0 {
            self.epoch = 1;
        }
        let mut stack: Vec<(u32, usize)> = vec![(r0, 0)];
        while let Some(top) = stack.last_mut() {
            let (r, i) = (top.0, top.1);
            let adj = &self.adj[r as usize];
            if i == adj.len() {
                stack.pop();
                continue;
            }
            top.1 += 1;
            let c = adj[i] as usize;
            self.visits += 1;
            if self.stamp[c] == self.epoch {
                continue;
            }
            self.stamp[c] = self.epoch;
            self.via[c] = r;
            let owner = self.match_col[c];
            if owner == NONE {
                // Augment back along the path.
                let mut cur = c;
                loop {
                    let rr = self.via[cur];
                    let prev = self.match_row[rr as usize];
                    self.match_col[cur] = rr;
                    self.match_row[rr as usize] = cur as u32;
                    if rr == r0 {
                        break;
                    }
                    cur = prev as usize;
                }
                self.size += 1;
                self.epoch = self.epoch.wrapping_add(1);
                if self.epoch == 0 {
                    self.stamp.iter_mut().for_each(|s| *s = 0);
                    self.epoch = 1;
                }
                return true;
            }
            stack.push((owner, 0));
        }
        false
    }
}

/// A Bloom filter over 64-bit keys, `k` probes from one 64-bit hash by
/// double hashing.  Used only to screen arriving rows for duplicates.
#[derive(Clone, Debug)]
pub struct Bloom {
    bits: Vec<u64>,
    mask: u64,
    k: u32,
}

impl Bloom {
    /// About `bits_per_key · expected` bits, rounded up to a power of two.
    pub fn new(expected: usize, bits_per_key: usize, k: u32) -> Self {
        let n = (expected.max(1) * bits_per_key.max(1)).next_power_of_two().max(64);
        Self {
            bits: vec![0; n / 64],
            mask: n as u64 - 1,
            k,
        }
    }
    fn probes(&self, h: u64) -> impl Iterator<Item = u64> + '_ {
        let h2 = (h >> 32) | 1;
        (0..self.k as u64).map(move |i| h.wrapping_add(i.wrapping_mul(h2)) & self.mask)
    }
    pub fn contains(&self, h: u64) -> bool {
        self.probes(h)
            .all(|b| self.bits[(b >> 6) as usize] >> (b & 63) & 1 == 1)
    }
    pub fn insert(&mut self, h: u64) {
        let p: Vec<u64> = self.probes(h).collect();
        for b in p {
            self.bits[(b >> 6) as usize] |= 1 << (b & 63);
        }
    }
    pub fn k(&self) -> u32 {
        self.k
    }
}

fn mix(v: u64) -> u64 {
    let mut z = v.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash of a row up to a non-zero scalar (normalised so its first
/// coefficient is 1), with its right-hand side.
fn row_hash(entries: &[(u32, u64)], rhs: u64, m: u64, ops: &mut u64) -> u64 {
    let inv = invmod(entries[0].1, m).unwrap_or(1);
    let mut h = 0x5ee7_u64;
    for &(c, v) in entries {
        *ops += 1;
        h = mix(h ^ (c as u64).rotate_left(17) ^ mulmod(v, inv, m));
    }
    mix(h ^ mulmod(rhs, inv, m))
}

#[derive(Default)]
struct MixedState {
    rows: Vec<(Vec<(u32, u64)>, u64)>,
    gate: Option<MatchGate>,
    covered_gate_visits: u64,
    solution: Option<Vec<u64>>,
    inconsistent: bool,
    row_ops: u64,
    inversions: u64,
    dependent: u64,
    attempts: u64,
    failed_attempts: u64,
    next_attempt: usize,
    peeled_forward: u64,
    peeled_backward: u64,
    core_dim: u64,
    core_fill: u64,
    bloom: Option<Bloom>,
    bloom_positive: u64,
    bloom_false_positive: u64,
    duplicates: u64,
    hashes: BTreeMap<u64, Vec<usize>>,
    /// `mixed-peel-lazy`: no matching until every column is covered; then
    /// one batch matching over the stored rows, incremental afterwards;
    /// the square solve uses worklist peeling and a bucket-queue pivot.
    lazy: bool,
    covered: Vec<bool>,
    uncovered: usize,
    bookkeeping: u64,
    arithmetic: u64,
    /// Amendment A3: the matched square block was singular once; solve
    /// over all stored rows from then on.
    fallback: bool,
    fallbacks: u64,
}

/// `mixed-peel`: see the module documentation.
pub struct MixedPeel {
    modulus: u64,
    cols: usize,
    state: RefCell<MixedState>,
}

impl MixedPeel {
    pub fn new(cols: usize, modulus: u64) -> Self {
        Self {
            modulus,
            cols,
            state: RefCell::new(MixedState {
                gate: Some(MatchGate::new(cols)),
                next_attempt: cols,
                ..MixedState::default()
            }),
        }
    }

    /// The same solver with a Bloom duplicate screen on arriving rows
    /// (`bits_per_key` bits a row, `k` probes).
    pub fn with_bloom(cols: usize, modulus: u64, bits_per_key: usize, k: u32) -> Self {
        let s = Self::new(cols, modulus);
        s.state.borrow_mut().bloom = Some(Bloom::new(2 * cols + 16, bits_per_key, k));
        s
    }

    pub fn attempts(&self) -> u64 {
        self.state.borrow().attempts
    }

    /// `mixed-peel-lazy` (amendment A2 of the study): the same solver with
    /// the gate built only once every column occurs in some row (a
    /// necessary condition, checked for one count per non-zero), worklist
    /// peeling and a bucket-queue Markowitz pivot.
    pub fn lazy(cols: usize, modulus: u64) -> Self {
        Self {
            modulus,
            cols,
            state: RefCell::new(MixedState {
                gate: None,
                next_attempt: cols,
                lazy: true,
                covered: vec![false; cols],
                uncovered: cols,
                ..MixedState::default()
            }),
        }
    }

    fn try_solve(&self, st: &mut MixedState) {
        let Some(gate) = st.gate.as_ref() else {
            return;
        };
        if st.solution.is_some() || gate.size() < self.cols || st.rows.len() < st.next_attempt {
            return;
        }
        st.attempts += 1;
        let square: Vec<usize> = (0..self.cols)
            .map(|c| gate.row_of(c).expect("full matching"))
            .collect();
        let all: Vec<usize> = (0..st.rows.len()).collect();
        let solve = |rows: &[usize], st: &MixedState| {
            if st.lazy {
                solve_square_fast(&st.rows, rows, self.cols, self.modulus)
            } else {
                solve_square(&st.rows, rows, self.cols, self.modulus)
            }
        };
        // Amendment A3: index-calculus rows carry ±1 on the factor-base
        // columns, so a full matching does not make the matched square
        // block non-singular (an even cycle of ±1 rows is singular whatever
        // the matching).  When the square block fails, eliminate over every
        // stored row instead (pivots chosen among all of them), and from
        // then on go straight to that.
        let mut matched = if st.fallback { all.clone() } else { square };
        let mut solved = solve(&matched, st);
        if solved.is_err() && !st.fallback {
            if let Err(rep) = solved {
                st.bookkeeping += rep.book;
                st.arithmetic += rep.row_ops - rep.book;
                st.row_ops += rep.row_ops;
                st.inversions += rep.inversions;
                st.failed_attempts += 1;
            }
            st.fallback = true;
            st.fallbacks += 1;
            matched = all;
            solved = solve(&matched, st);
        }
        if st.fallback {
            // Every row is checked below: none is "matched".
            matched.clear();
        }
        match solved {
            Ok((x, rep)) => {
                st.bookkeeping += rep.book;
                st.arithmetic += rep.row_ops - rep.book;
                st.row_ops += rep.row_ops;
                st.inversions += rep.inversions;
                st.peeled_forward += rep.forward;
                st.peeled_backward += rep.backward;
                st.core_dim += rep.core_dim;
                st.core_fill += rep.fill;
                // Check every surplus row against the solution.
                let mut is_matched = vec![false; st.rows.len()];
                for &r in &matched {
                    is_matched[r] = true;
                }
                let m = self.modulus;
                for (i, (e, rhs)) in st.rows.iter().enumerate() {
                    if is_matched[i] {
                        continue;
                    }
                    st.row_ops += e.len() as u64;
                    st.arithmetic += e.len() as u64;
                    let lhs = e
                        .iter()
                        .fold(0u64, |acc, &(c, v)| addmod(acc, mulmod(v, x[c as usize], m), m));
                    if lhs != *rhs {
                        st.inconsistent = true;
                        return;
                    }
                }
                st.dependent += st.rows.len().saturating_sub(self.cols) as u64;
                st.solution = Some(x);
            }
            Err(rep) => {
                st.bookkeeping += rep.book;
                st.arithmetic += rep.row_ops - rep.book;
                st.row_ops += rep.row_ops;
                st.inversions += rep.inversions;
                st.failed_attempts += 1;
                // Rank-deficient over all stored rows: retry on a cadence.
                st.next_attempt = st.rows.len() + (self.cols / 32).max(1);
            }
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct SquareReport {
    /// Everything charged, bookkeeping included.
    row_ops: u64,
    /// The bookkeeping part of `row_ops` (weight updates, queue operations,
    /// incidence scans); `solve_square_fast` only.
    book: u64,
    inversions: u64,
    forward: u64,
    backward: u64,
    core_dim: u64,
    fill: u64,
}

/// Solve the square system of rows `matched[0..n]` (any order) over the
/// `n` columns: two-sided peeling, then Markowitz elimination of the core,
/// then back-substitution.  `Err` when a pivot vanishes (singular).
fn solve_square(
    rows: &[(Vec<(u32, u64)>, u64)],
    matched: &[usize],
    n: usize,
    m: u64,
) -> Result<(Vec<u64>, SquareReport), SquareReport> {
    let mut rep = SquareReport::default();
    // Working copies: row-major sparse maps and column → rows incidence.
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
    let nr = matched.len();
    let mut row_alive = vec![true; nr];
    let mut col_alive = vec![true; n];
    let mut col_w: Vec<usize> = col_rows.iter().map(|v| v.len()).collect();
    let mut x = vec![0u64; n];
    let mut known = vec![false; n];
    // Back-substitution stack: (row index, its pivot column, its frozen entries, rhs).
    let mut stack: Vec<(u32, u32)> = Vec::new();
    let mut frozen: Vec<Option<(BTreeMap<u32, u64>, u64)>> = vec![None; nr];

    // Live rows of a column (lazy: incidence lists keep stale entries).
    // Lists can hold a row twice (a cancelled entry refilled later), so
    // the live set is de-duplicated.
    let live_rows = |c: usize, col_rows: &Vec<Vec<u32>>, w: &Vec<BTreeMap<u32, u64>>, row_alive: &Vec<bool>| -> Vec<u32> {
        let mut v: Vec<u32> = col_rows[c]
            .iter()
            .copied()
            .filter(|&i| row_alive[i as usize] && w[i as usize].contains_key(&(c as u32)))
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };

    // ── Peeling ──
    let mut changed = true;
    while changed {
        changed = false;
        // Forward: rows with one live column.
        for i in 0..nr {
            if !row_alive[i] || w[i].len() != 1 {
                continue;
            }
            let (&c, &a) = w[i].iter().next().expect("one entry");
            let inv = invmod(a, m).ok_or(rep)?;
            rep.inversions += 1;
            rep.row_ops += 1;
            let v = mulmod(rhs[i], inv, m);
            let c = c as usize;
            x[c] = v;
            known[c] = true;
            row_alive[i] = false;
            col_alive[c] = false;
            rep.forward += 1;
            for j in live_rows(c, &col_rows, &w, &row_alive) {
                let j = j as usize;
                let a_jc = w[j].remove(&(c as u32)).expect("live");
                rep.row_ops += 1;
                rhs[j] = submod(rhs[j], mulmod(a_jc, v, m), m);
            }
            col_w[c] = 0;
            changed = true;
        }
        // Backward: columns with one live row.
        for c in 0..n {
            if !col_alive[c] {
                continue;
            }
            let live = live_rows(c, &col_rows, &w, &row_alive);
            rep.row_ops += 1; // the weight read
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
                    rep.row_ops += 1;
                    col_w[c2 as usize] = col_w[c2 as usize].saturating_sub(1);
                }
            }
            frozen[i] = Some((std::mem::take(&mut w[i]), rhs[i]));
            stack.push((i as u32, c as u32));
            changed = true;
        }
    }

    // ── Markowitz elimination of the core ──
    let core: Vec<usize> = (0..n).filter(|&c| col_alive[c]).collect();
    rep.core_dim = core.len() as u64;
    let mut remaining = core.len();
    while remaining > 0 {
        // Lightest live column.
        let mut best: Option<(usize, usize)> = None;
        for &c in &core {
            if !col_alive[c] {
                continue;
            }
            rep.row_ops += 1;
            let wc = col_w[c];
            if best.is_none_or(|(_, bw)| wc < bw) {
                best = Some((c, wc));
            }
        }
        let (c, _) = best.expect("a live column");
        let live = live_rows(c, &col_rows, &w, &row_alive);
        if live.is_empty() {
            return Err(rep);
        }
        let p = *live
            .iter()
            .min_by_key(|&&i| w[i as usize].len())
            .expect("non-empty") as usize;
        let a_pc = w[p][&(c as u32)];
        let inv = invmod(a_pc, m).ok_or(rep)?;
        rep.inversions += 1;
        let prow: Vec<(u32, u64)> = w[p].iter().map(|(&k, &v)| (k, v)).collect();
        for &j in &live {
            let j = j as usize;
            if j == p {
                continue;
            }
            let a_jc = w[j][&(c as u32)];
            let f = mulmod(a_jc, inv, m);
            rep.row_ops += 1;
            for &(k, v) in &prow {
                rep.row_ops += 1;
                let e = w[j].entry(k).or_insert_with(|| {
                    rep.fill += 1;
                    col_rows[k as usize].push(j as u32);
                    col_w[k as usize] += 1;
                    0
                });
                *e = submod(*e, mulmod(f, v, m), m);
                if *e == 0 {
                    w[j].remove(&k);
                    col_w[k as usize] = col_w[k as usize].saturating_sub(1);
                }
            }
            rhs[j] = submod(rhs[j], mulmod(f, rhs[p], m), m);
            rep.row_ops += 1;
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

    // ── Back-substitution, last pivot first ──
    for &(i, c) in stack.iter().rev() {
        let (row, b) = frozen[i as usize].as_ref().expect("frozen");
        let mut acc = *b;
        let mut a_c = 0;
        for (&k, &v) in row {
            if k == c {
                a_c = v;
                continue;
            }
            debug_assert!(known[k as usize]);
            rep.row_ops += 1;
            acc = submod(acc, mulmod(v, x[k as usize], m), m);
        }
        let inv = invmod(a_c, m).ok_or(rep)?;
        rep.inversions += 1;
        rep.row_ops += 1;
        x[c as usize] = mulmod(acc, inv, m);
        known[c as usize] = true;
    }
    if known.iter().any(|k| !k) {
        return Err(rep);
    }
    Ok((x, rep))
}

/// [`solve_square`] with worklists: peeling driven by queues of weight-one
/// rows and columns (as an invertible Bloom lookup table is decoded) and
/// Markowitz pivots taken from a bucket queue keyed by live column weight,
/// so no step scans every column.  Exact column weights are maintained;
/// every queue push or pop, weight update and incidence entry read is
/// charged as bookkeeping (in `row_ops` and in `book`).
fn solve_square_fast(
    rows: &[(Vec<(u32, u64)>, u64)],
    matched: &[usize],
    n: usize,
    m: u64,
) -> Result<(Vec<u64>, SquareReport), SquareReport> {
    let mut rep = SquareReport::default();
    macro_rules! book {
        ($k:expr) => {{
            rep.book += $k as u64;
            rep.row_ops += $k as u64;
        }};
    }
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
        book!(row.len());
    }
    let nr = matched.len();
    let mut row_alive = vec![true; nr];
    let mut col_alive = vec![true; n];
    let mut col_w: Vec<usize> = col_rows.iter().map(|v| v.len()).collect();
    let mut x = vec![0u64; n];
    let mut stack: Vec<(u32, u32)> = Vec::new();
    let mut frozen: Vec<Option<(BTreeMap<u32, u64>, u64)>> = vec![None; nr];
    let live_rows = |c: usize,
                     col_rows: &Vec<Vec<u32>>,
                     w: &Vec<BTreeMap<u32, u64>>,
                     row_alive: &Vec<bool>,
                     rep: &mut SquareReport|
     -> Vec<u32> {
        rep.book += col_rows[c].len() as u64;
        rep.row_ops += col_rows[c].len() as u64;
        let mut v: Vec<u32> = col_rows[c]
            .iter()
            .copied()
            .filter(|&i| row_alive[i as usize] && w[i as usize].contains_key(&(c as u32)))
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let mut row_q: Vec<u32> = (0..nr as u32).filter(|&i| w[i as usize].len() == 1).collect();
    let mut col_q: Vec<u32> = (0..n as u32).filter(|&c| col_w[c as usize] == 1).collect();
    book!(2 * n);
    loop {
        if let Some(i) = row_q.pop() {
            book!(1);
            let i = i as usize;
            if !row_alive[i] || w[i].len() != 1 {
                continue;
            }
            let (&c, &a) = w[i].iter().next().expect("one entry");
            let c = c as usize;
            let inv = invmod(a, m).ok_or(rep)?;
            rep.inversions += 1;
            rep.row_ops += 1;
            let v = mulmod(rhs[i], inv, m);
            x[c] = v;
            row_alive[i] = false;
            col_alive[c] = false;
            rep.forward += 1;
            for j in live_rows(c, &col_rows, &w, &row_alive, &mut rep) {
                let j = j as usize;
                let a_jc = w[j].remove(&(c as u32)).expect("live");
                rep.row_ops += 1;
                rhs[j] = submod(rhs[j], mulmod(a_jc, v, m), m);
                if w[j].len() == 1 {
                    row_q.push(j as u32);
                    book!(1);
                }
            }
            continue;
        }
        if let Some(c) = col_q.pop() {
            book!(1);
            let c = c as usize;
            if !col_alive[c] || col_w[c] != 1 {
                continue;
            }
            let live = live_rows(c, &col_rows, &w, &row_alive, &mut rep);
            let Some(&i) = live.first() else {
                return Err(rep);
            };
            let i = i as usize;
            row_alive[i] = false;
            col_alive[c] = false;
            rep.backward += 1;
            for &k in w[i].keys() {
                let k = k as usize;
                if k != c {
                    book!(1);
                    col_w[k] -= 1;
                    if col_w[k] == 1 {
                        col_q.push(k as u32);
                        book!(1);
                    }
                }
            }
            frozen[i] = Some((std::mem::take(&mut w[i]), rhs[i]));
            stack.push((i as u32, c as u32));
            continue;
        }
        break;
    }

    // ── Markowitz on the core, pivots from a bucket queue ──
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); n + 2];
    let mut remaining = 0usize;
    let mut lo = usize::MAX;
    for c in 0..n {
        if col_alive[c] {
            let wc = col_w[c].min(n + 1);
            buckets[wc].push(c as u32);
            lo = lo.min(wc);
            remaining += 1;
            book!(1);
        }
    }
    rep.core_dim = remaining as u64;
    while remaining > 0 {
        // Lowest non-empty bucket holding a fresh entry.
        let c = loop {
            while lo < buckets.len() && buckets[lo].is_empty() {
                lo += 1;
            }
            if lo >= buckets.len() {
                return Err(rep);
            }
            let c = buckets[lo].pop().expect("non-empty") as usize;
            book!(1);
            if col_alive[c] && col_w[c].min(n + 1) == lo {
                break c;
            }
        };
        if col_w[c] == 0 {
            return Err(rep);
        }
        let live = live_rows(c, &col_rows, &w, &row_alive, &mut rep);
        if live.is_empty() {
            return Err(rep);
        }
        let p = *live
            .iter()
            .min_by_key(|&&i| w[i as usize].len())
            .expect("non-empty") as usize;
        book!(live.len());
        let a_pc = w[p][&(c as u32)];
        let inv = invmod(a_pc, m).ok_or(rep)?;
        rep.inversions += 1;
        let prow: Vec<(u32, u64)> = w[p].iter().map(|(&k, &v)| (k, v)).collect();
        let mut touched: Vec<u32> = Vec::new();
        for &j in &live {
            let j = j as usize;
            if j == p {
                continue;
            }
            let a_jc = w[j][&(c as u32)];
            let f = mulmod(a_jc, inv, m);
            rep.row_ops += 1;
            for &(k, v) in &prow {
                if k as usize == c {
                    w[j].remove(&k);
                    continue;
                }
                rep.row_ops += 1;
                let mut filled = false;
                let e = w[j].entry(k).or_insert_with(|| {
                    filled = true;
                    0
                });
                *e = submod(*e, mulmod(f, v, m), m);
                let zero = *e == 0;
                if zero {
                    w[j].remove(&k);
                }
                if filled && !zero {
                    rep.fill += 1;
                    col_rows[k as usize].push(j as u32);
                    col_w[k as usize] += 1;
                    touched.push(k);
                    book!(2);
                } else if !filled && zero {
                    col_w[k as usize] -= 1;
                    touched.push(k);
                    book!(1);
                }
            }
            rhs[j] = submod(rhs[j], mulmod(f, rhs[p], m), m);
            rep.row_ops += 1;
        }
        row_alive[p] = false;
        col_alive[c] = false;
        for &(k, _) in &prow {
            if k as usize != c {
                col_w[k as usize] -= 1;
                touched.push(k);
                book!(1);
            }
        }
        touched.sort_unstable();
        touched.dedup();
        for k in touched {
            let k = k as usize;
            if col_alive[k] {
                let wk = col_w[k].min(n + 1);
                buckets[wk].push(k as u32);
                lo = lo.min(wk);
                book!(1);
            }
        }
        frozen[p] = Some((std::mem::take(&mut w[p]), rhs[p]));
        stack.push((p as u32, c as u32));
        remaining -= 1;
    }

    for &(i, c) in stack.iter().rev() {
        let (row, b) = frozen[i as usize].as_ref().expect("frozen");
        let mut acc = *b;
        let mut a_c = 0;
        for (&k, &v) in row {
            if k == c {
                a_c = v;
                continue;
            }
            rep.row_ops += 1;
            acc = submod(acc, mulmod(v, x[k as usize], m), m);
        }
        let inv = invmod(a_c, m).ok_or(rep)?;
        rep.inversions += 1;
        rep.row_ops += 1;
        x[c as usize] = mulmod(acc, inv, m);
    }
    Ok((x, rep))
}

impl RelationSolver for MixedPeel {
    fn add_row(&mut self, dense: Vec<u64>, rhs: u64) -> RowStatus {
        let m = self.modulus;
        let st = self.state.get_mut();
        if std::mem::take(&mut st.inconsistent) {
            return RowStatus::Inconsistent;
        }
        let entries: Vec<(u32, u64)> = dense
            .iter()
            .enumerate()
            .filter(|(_, &v)| v % m != 0)
            .map(|(c, &v)| (c as u32, v % m))
            .collect();
        let rhs = rhs % m;
        st.row_ops += entries.len() as u64;
        if entries.is_empty() {
            if rhs == 0 {
                st.dependent += 1;
                return RowStatus::Dependent;
            }
            return RowStatus::Inconsistent;
        }
        if let Some(x) = &st.solution {
            let lhs = entries
                .iter()
                .fold(0u64, |acc, &(c, v)| addmod(acc, mulmod(v, x[c as usize], m), m));
            st.row_ops += entries.len() as u64;
            if lhs == rhs {
                st.dependent += 1;
                return RowStatus::Dependent;
            }
            return RowStatus::Inconsistent;
        }
        if st.bloom.is_some() {
            let mut ops = 0u64;
            let h = row_hash(&entries, rhs, m, &mut ops);
            st.row_ops += ops + st.bloom.as_ref().map_or(0, |b| b.k() as u64);
            st.inversions += 1;
            let positive = st.bloom.as_ref().is_some_and(|b| b.contains(h));
            if positive {
                st.bloom_positive += 1;
                // Exact check against the stored rows with this hash.
                let same = st.hashes.get(&h).cloned().unwrap_or_default();
                let mut dup = false;
                for i in same {
                    st.row_ops += entries.len() as u64;
                    let (e2, r2) = &st.rows[i];
                    if e2.len() == entries.len() && e2.iter().zip(&entries).all(|(a, b)| a.0 == b.0) {
                        // Proportional rows with proportional rhs.
                        let f = mulmod(entries[0].1, invmod(e2[0].1, m).unwrap_or(0), m);
                        if e2.iter().zip(&entries).all(|(a, b)| mulmod(a.1, f, m) == b.1)
                            && mulmod(*r2, f, m) == rhs
                        {
                            dup = true;
                            break;
                        }
                    }
                }
                if dup {
                    st.duplicates += 1;
                    st.dependent += 1;
                    return RowStatus::Dependent;
                }
                st.bloom_false_positive += 1;
            }
            st.bloom.as_mut().expect("bloom").insert(h);
            st.hashes.entry(h).or_default().push(st.rows.len());
        }
        let cols: Vec<u32> = entries.iter().map(|e| e.0).collect();
        st.rows.push((entries, rhs));
        if st.lazy {
            for &c in &cols {
                st.row_ops += 1;
                st.bookkeeping += 1;
                if !st.covered[c as usize] {
                    st.covered[c as usize] = true;
                    st.uncovered -= 1;
                }
            }
            if st.gate.is_none() {
                if st.uncovered > 0 || st.rows.len() < self.cols {
                    return RowStatus::Independent;
                }
                // Build the matching once, over every stored row.
                let mut gate = MatchGate::new(self.cols);
                for (e, _) in &st.rows {
                    let cs: Vec<u32> = e.iter().map(|x| x.0).collect();
                    gate.add_row(&cs);
                }
                st.covered_gate_visits += gate.visits;
                st.row_ops += gate.visits;
                st.bookkeeping += gate.visits;
                st.gate = Some(gate);
                return RowStatus::Independent;
            }
        }
        let gate = st.gate.as_mut().expect("gate");
        let before = gate.visits;
        gate.add_row(&cols);
        let visits = gate.visits - before;
        st.covered_gate_visits += visits;
        st.row_ops += visits;
        st.bookkeeping += visits;
        RowStatus::Independent
    }

    fn pinned(&self, col: usize) -> Option<u64> {
        let mut st = self.state.borrow_mut();
        self.try_solve(&mut st);
        st.solution.as_ref().map(|x| x[col])
    }

    fn rank(&self) -> usize {
        let st = self.state.borrow();
        if st.solution.is_some() {
            self.cols
        } else {
            st.gate
                .as_ref()
                .map_or(0, |g| g.size())
                .min(self.cols.saturating_sub(1))
        }
    }

    fn dependent(&self) -> u64 {
        self.state.borrow().dependent
    }

    fn work(&self) -> (u64, &'static str) {
        (self.state.borrow().row_ops, "row_ops")
    }

    fn work_counters(&self) -> BTreeMap<String, u64> {
        let st = self.state.borrow();
        let mut out = BTreeMap::new();
        out.insert("mixed_gate_visits".into(), st.covered_gate_visits);
        out.insert("mixed_attempts".into(), st.attempts);
        out.insert("mixed_failed_attempts".into(), st.failed_attempts);
        out.insert("mixed_inversions".into(), st.inversions);
        out.insert("mixed_peeled_forward".into(), st.peeled_forward);
        out.insert("mixed_peeled_backward".into(), st.peeled_backward);
        out.insert("mixed_core_dim".into(), st.core_dim);
        out.insert("mixed_core_fill".into(), st.core_fill);
        if st.fallbacks > 0 {
            out.insert("mixed_fallbacks".into(), st.fallbacks);
        }
        if st.lazy {
            out.insert("mixed_bookkeeping".into(), st.bookkeeping);
            out.insert("mixed_arithmetic".into(), st.arithmetic);
        }
        if st.bloom.is_some() {
            out.insert("mixed_bloom_positive".into(), st.bloom_positive);
            out.insert("mixed_bloom_false_positive".into(), st.bloom_false_positive);
            out.insert("mixed_duplicates".into(), st.duplicates);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    /// The incremental matching agrees with a from-scratch maximum
    /// matching (Kuhn over all rows) after every row, on random sparse
    /// bipartite graphs.
    #[test]
    fn incremental_matching_is_maximum() {
        for seed in 0..300u64 {
            let mut rng = StdRng::seed_from_u64(seed);
            let cols = rng.gen_range(2..40);
            let mut g = MatchGate::new(cols);
            let mut rows: Vec<Vec<u32>> = Vec::new();
            for _ in 0..2 * cols {
                let mut r: Vec<u32> = (0..rng.gen_range(1..4))
                    .map(|_| rng.gen_range(0..cols) as u32)
                    .collect();
                r.sort();
                r.dedup();
                g.add_row(&r);
                rows.push(r);
                for c in 0..cols {
                    if let Some(r) = g.row_of(c) {
                        assert!(rows[r].contains(&(c as u32)));
                        assert_eq!(g.match_row[r], c as u32);
                    }
                }
            }
            // The structural rank equals the rank of the same pattern with
            // random coefficients mod a large prime (with high probability).
            {
                let p = 1_000_000_007u64;
                let mut mat: Vec<Vec<u64>> = rows
                    .iter()
                    .map(|r| {
                        let mut d = vec![0u64; cols];
                        for &c in r {
                            d[c as usize] = rng.gen_range(1..p);
                        }
                        d
                    })
                    .collect();
                let mut rank = 0;
                for c in 0..cols {
                    if let Some(piv) = (rank..mat.len()).find(|&i| mat[i][c] != 0) {
                        mat.swap(rank, piv);
                        let inv = invmod(mat[rank][c], p).unwrap();
                        for i in 0..mat.len() {
                            if i != rank && mat[i][c] != 0 {
                                let f = mulmod(mat[i][c], inv, p);
                                for k in 0..cols {
                                    let s = mulmod(f, mat[rank][k], p);
                                    mat[i][k] = submod(mat[i][k], s, p);
                                }
                            }
                        }
                        rank += 1;
                    }
                }
                assert_eq!(g.size(), rank, "seed {seed}: structural vs numeric rank");
            }
        }
    }

    fn planted(
        seed: u64,
        cols: usize,
        m: u64,
        weight: (usize, usize),
    ) -> (Vec<u64>, impl FnMut() -> (Vec<u64>, u64)) {
        let mut rng = StdRng::seed_from_u64(seed);
        let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..m)).collect();
        let x2 = x.clone();
        let gen = move || {
            let mut row = vec![0u64; cols];
            for _ in 0..rng.gen_range(weight.0..=weight.1) {
                row[rng.gen_range(0..cols)] = rng.gen_range(1..m);
            }
            let rhs = row
                .iter()
                .zip(&x2)
                .fold(0u64, |a, (&p, &q)| addmod(a, mulmod(p, q, m), m));
            (row, rhs)
        };
        (x, gen)
    }

    #[test]
    fn mixed_peel_recovers_planted_solutions() {
        for seed in 0..200u64 {
            let m = [1_000_003u64, 4_294_967_291, (1u64 << 61) - 1][(seed % 3) as usize];
            let cols = [5usize, 17, 60, 150][(seed / 3 % 4) as usize];
            let (x, mut gen) = planted(seed, cols, m, (2, 4));
            for variant in 0..3 {
                let mut s = match variant {
                    0 => MixedPeel::new(cols, m),
                    1 => MixedPeel::with_bloom(cols, m, 10, 4),
                    _ => MixedPeel::lazy(cols, m),
                };
                let mut done = false;
                for _ in 0..20 * cols {
                    let (row, rhs) = gen();
                    let st = s.add_row(row, rhs);
                    assert_ne!(st, RowStatus::Inconsistent, "seed {seed}");
                    if s.pinned(0).is_some() {
                        done = true;
                        break;
                    }
                }
                assert!(done, "seed {seed} variant {variant}: not solved");
                assert_eq!(s.rank(), cols);
                for c in 0..cols {
                    assert_eq!(s.pinned(c), Some(x[c]), "seed {seed} col {c}");
                }
            }
        }
    }

    /// Index-calculus rows `±x_i ± x_j + b·t = a` (±1 on the base): the
    /// matched square block can be singular (even ±1 cycles); every
    /// variant must still solve, through the all-rows fallback.
    #[test]
    fn mixed_peel_solves_signed_two_term_rows() {
        let m = 16_782_299u64;
        for seed in 0..120u64 {
            let mut rng = StdRng::seed_from_u64(0xA3 ^ seed);
            let fb = [8usize, 32, 128][(seed % 3) as usize];
            let cols = fb + 1;
            let x: Vec<u64> = (0..cols).map(|_| rng.gen_range(0..m)).collect();
            for variant in 0..3 {
                let mut s = match variant {
                    0 => MixedPeel::new(cols, m),
                    1 => MixedPeel::with_bloom(cols, m, 10, 4),
                    _ => MixedPeel::lazy(cols, m),
                };
                let mut r2 = StdRng::seed_from_u64(seed);
                let mut done = false;
                for _ in 0..60 * cols {
                    let mut row = vec![0u64; cols];
                    let i = r2.gen_range(0..fb);
                    let j = r2.gen_range(0..fb);
                    let si = if r2.gen_bool(0.5) { 1 } else { m - 1 };
                    let sj = if r2.gen_bool(0.5) { 1 } else { m - 1 };
                    row[i] = addmod(row[i], si, m);
                    row[j] = addmod(row[j], sj, m);
                    row[fb] = r2.gen_range(1..m);
                    let rhs = row.iter().zip(&x).fold(0u64, |a, (&p, &q)| addmod(a, mulmod(p, q, m), m));
                    assert_ne!(s.add_row(row, rhs), RowStatus::Inconsistent);
                    if s.pinned(fb).is_some() && s.rank() == cols {
                        done = true;
                        break;
                    }
                }
                assert!(done, "seed {seed} variant {variant}: not solved");
                for c in 0..cols {
                    assert_eq!(s.pinned(c), Some(x[c]), "seed {seed} variant {variant} col {c}");
                }
            }
        }
    }

    #[test]
    fn mixed_peel_reports_a_wrong_relation() {
        let m = 1_000_003u64;
        let cols = 30;
        let (_, mut gen) = planted(7, cols, m, (2, 3));
        let mut s = MixedPeel::new(cols, m);
        let mut saw = false;
        for i in 0..40 * cols {
            let (row, mut rhs) = gen();
            if i == 3 {
                rhs = addmod(rhs, 1, m);
            }
            if s.add_row(row, rhs) == RowStatus::Inconsistent {
                saw = true;
                break;
            }
            let _ = s.pinned(0);
        }
        assert!(saw, "a corrupted row must surface as Inconsistent");
    }

    #[test]
    fn bloom_screen_drops_exact_duplicates() {
        let m = 1_000_003u64;
        let cols = 20;
        let (x, mut gen) = planted(11, cols, m, (2, 3));
        let mut s = MixedPeel::with_bloom(cols, m, 10, 4);
        let (row, rhs) = gen();
        assert_eq!(s.add_row(row.clone(), rhs), RowStatus::Independent);
        let scaled: Vec<u64> = row.iter().map(|&v| mulmod(v, 5, m)).collect();
        assert_eq!(s.add_row(scaled, mulmod(rhs, 5, m)), RowStatus::Dependent);
        for _ in 0..40 * cols {
            let (row, rhs) = gen();
            s.add_row(row, rhs);
            if s.pinned(0).is_some() {
                break;
            }
        }
        assert_eq!(s.pinned(3), Some(x[3]));
        assert_eq!(s.work_counters()["mixed_duplicates"], 1);
    }
}
