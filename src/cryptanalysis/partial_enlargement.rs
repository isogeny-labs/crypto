//! MutantXL's fixed-degree closure reached by **partial enlargement**
//! (lead L2 of `research/koblitz_ic_new_leads_20261018/`; the
//! measurement is `research/koblitz_ic_leads_build_20261024/`).
//!
//! The question the degree-`D` closure answers is whether `1` lies in the
//! smallest space `W` of polynomials of degree `≤ D` that contains the
//! generators and is closed under multiplying any element of degree `< D`
//! by a variable.  That `W` is the saturated row space
//! [`ReducedBasis::from_system_closed_full`] builds: every Macaulay row
//! `t·f` with `deg t + deg f ≤ D` is reached by multiplying `f` by the
//! variables of `t` one at a time, and every intermediate has degree
//! `< D`.  The dense engine builds the whole degree-`D` Macaulay matrix
//! first and then saturates it.  Its peak is that matrix plus the closure
//! products, 0.4–1 million rows at 40–44 unknowns
//! (`research/dreg_closure_driver_20261016/`).
//!
//! Partial enlargement (Buchmann, Cabarcas, Ding, Mohamed, AFRICACRYPT
//! 2010, the MXL3 and MGB solvers) explores the same `W` in a different
//! order and stops at `1`:
//!
//! 1. seed with the Macaulay rows of degree `≤ D − 1`;
//! 2. keep every stored row of leading degree `< D` on a queue; expand
//!    them a batch at a time (each row times every occurring variable),
//!    lowest leading degree first (mutants before the enlargement), then
//!    by leading variable, smallest variable first;
//! 3. absorb each batch into a reduced row echelon basis, queue the new
//!    low rows, and stop as soon as the basis holds `1`.
//!
//! Every inserted row lies in the ideal, so a refutation is sound.  If the
//! queue empties, every low row has been multiplied by every variable, so
//! the basis spans `W` exactly and refutes exactly when the dense closure
//! does; the unit tests check both modes against
//! [`ReducedBasis::from_system_closed_full`].  What partial enlargement can
//! change is only how many rows are stored before `1` appears.
//!
//! [`PeMode::Full`] runs the dense engine's order (whole degree-`D`
//! Macaulay matrix, then every fall each round) through the same code, so
//! the two modes are one instrument and their peaks are comparable.
//!
//! [`ReducedBasis::from_system_closed_full`]: crate::cryptanalysis::inherited_f4::ReducedBasis::from_system_closed_full

use crate::cryptanalysis::fx_hash::MaskMap;
use crate::cryptanalysis::koblitz_groebner::{
    macaulay_rows_monos_with_mask, monomials_up_to_mask, rref_f2_counted,
};
use crate::cryptanalysis::pq_groebner_f2::{
    bit, sort_masks_descending, F2BoolPoly, Mask, MASK_BITS,
};
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::time::Instant;

/// Which order explores the closure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeMode {
    /// The dense engine's order: the full degree-`D` Macaulay matrix, then
    /// every pending fall times every variable, one round at a time.
    Full,
    /// Partial enlargement: seed at degree `D − 1`, expand low rows in
    /// batches of at most `batch_rows`, lowest leading degree first, and
    /// stop at `1`.
    Partial { batch_rows: usize },
}

/// Queue order of [`PeMode::Partial`] (the order sweep of
/// `research/koblitz_ic_leads_build_20261024/`, Amendment 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeOrder {
    /// Lowest leading degree first, then the smallest leading variable.
    LowFirst,
    /// Highest leading degree first (below `D`), then the smallest leading
    /// variable.
    HighFirst,
    /// A fixed pseudo-random permutation of the pivot columns.
    Random(u64),
}

/// Options of [`pe_closure_with`]; [`Default`] is [`pe_closure`]'s order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeOptions {
    pub order: PeOrder,
    /// Seed degree of [`PeMode::Partial`]; `None` is `D − 1` (at least the
    /// system's degree).
    pub seed_degree: Option<u32>,
    /// MXL's restriction: multiply a row only by the variables not larger
    /// than its leading variable; when the queue empties, a completion pass
    /// re-expands every low row by every variable, so the closure computed
    /// is the same.
    pub restrict_vars: bool,
}

impl Default for PeOptions {
    fn default() -> Self {
        Self {
            order: PeOrder::LowFirst,
            seed_degree: None,
            restrict_vars: false,
        }
    }
}

/// One closure run at a fixed degree.
#[derive(Clone, Debug, PartialEq)]
pub struct PeReport {
    pub degree: u32,
    pub mode: &'static str,
    /// `1` entered the basis.
    pub refuted: bool,
    /// The queue emptied: the basis spans the whole closure.
    pub saturated: bool,
    /// Columns of the layout: every monomial of degree `≤ D` over the
    /// occurring variables.
    pub columns: usize,
    /// Most rows the basis held after an absorb.
    pub peak_basis_rows: usize,
    /// Most rows one elimination held (basis plus batch): the memory peak
    /// of the run, at `columns / 8` bytes per row.
    pub peak_matrix_rows: usize,
    /// Basis rows when the run stopped (before collapsing to `1`).
    pub final_rows: usize,
    /// Absorbs (eliminations) run.
    pub absorbs: usize,
    /// Rows expanded (multiplied by every variable).
    pub expanded: u64,
    /// Product rows built.
    pub products: u64,
    /// Word XORs charged by the eliminations.
    pub word_ops: u64,
    /// The run stopped at the row cap before refuting or saturating:
    /// unmeasured, never evidence about the degree.
    pub capped: bool,
    /// Basis rows when the restricted phase ended and the completion pass
    /// began (`restrict_vars` only).
    pub completion_at_rows: Option<usize>,
    /// `1` entered during the completion pass, not the restricted phase.
    pub refuted_in_completion: bool,
    pub secs: f64,
}

impl PeReport {
    /// Peak bytes of the dense matrix, `peak_matrix_rows · ⌈columns/64⌉ · 8`.
    pub fn peak_matrix_bytes(&self) -> u64 {
        self.peak_matrix_rows as u64 * self.columns.div_ceil(64) as u64 * 8
    }
}

/// Dense layout and basis.
struct Engine {
    degree: u32,
    columns: Vec<Mask>,
    index: MaskMap<u32>,
    words: usize,
    vars: Vec<Mask>,
    /// `times[v][c]`: column of `columns[c] · vars[v]`, or `u32::MAX` when
    /// that monomial has degree `> D` (never needed: only rows of leading
    /// degree `< D` are multiplied, and a row's every monomial has degree
    /// at most its leading degree).
    times: Vec<Vec<u32>>,
    /// Reduced row echelon basis, sorted by pivot column.
    basis: Vec<Vec<u64>>,
    /// `piv[i]`: pivot column of `basis[i]`, strictly increasing.
    piv: Vec<usize>,
    report: PeReport,
}

fn leading_column(row: &[u64]) -> Option<usize> {
    row.iter()
        .enumerate()
        .find(|(_, w)| **w != 0)
        .map(|(i, w)| i * 64 + w.trailing_zeros() as usize)
}

impl Engine {
    fn new(
        system: &[F2BoolPoly],
        degree: u32,
        max_cols: usize,
        mode: &'static str,
    ) -> Option<Self> {
        let occurring = system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0 as Mask, |acc, t| acc | t.mask);
        let vars: Vec<Mask> = (0..MASK_BITS as u32)
            .filter(|&i| occurring >> i & 1 == 1)
            .map(bit)
            .collect();
        // Count before materialising.
        let n = vars.len() as u64;
        let mut count = 1u64;
        let mut binom = 1u64;
        for k in 1..=degree as u64 {
            binom = binom * (n + 1).saturating_sub(k) / k;
            count += binom;
        }
        if count as usize > max_cols {
            return None;
        }
        let mut columns = monomials_up_to_mask(occurring, degree);
        sort_masks_descending(&mut columns);
        let mut index: MaskMap<u32> = MaskMap::default();
        for (c, &m) in columns.iter().enumerate() {
            index.insert(m, c as u32);
        }
        let times = vars
            .iter()
            .map(|&v| {
                columns
                    .iter()
                    .map(|&m| index.get(&(m | v)).copied().unwrap_or(u32::MAX))
                    .collect()
            })
            .collect();
        let words = columns.len().div_ceil(64).max(1);
        let report = PeReport {
            degree,
            mode,
            refuted: false,
            saturated: false,
            columns: columns.len(),
            peak_basis_rows: 0,
            peak_matrix_rows: 0,
            final_rows: 0,
            absorbs: 0,
            expanded: 0,
            products: 0,
            word_ops: 0,
            capped: false,
            completion_at_rows: None,
            refuted_in_completion: false,
            secs: 0.0,
        };
        Some(Self {
            degree,
            columns,
            index,
            words,
            vars,
            times,
            basis: Vec::new(),
            piv: Vec::new(),
            report,
        })
    }

    fn pack(&self, monos: &[Mask]) -> Vec<u64> {
        let mut row = vec![0u64; self.words];
        for m in monos {
            let c = self.index[m] as usize;
            row[c / 64] ^= 1u64 << (c % 64);
        }
        row
    }

    fn lead_degree(&self, c: usize) -> u32 {
        self.columns[c].count_ones()
    }

    /// Leading variable of column `c`'s monomial: its largest variable,
    /// the lowest set bit (`v_0 > v_1 > …`).  The constant has none.
    fn lead_var(&self, c: usize) -> u32 {
        let m = self.columns[c];
        if m == 0 {
            64
        } else {
            m.trailing_zeros()
        }
    }

    /// Absorb `batch` into the reduced basis incrementally; return the
    /// pivot columns that are new.  Sets `refuted` if `1` is in the span.
    ///
    /// 1. Reduce every batch row against the basis, eight pivots at a time
    ///    through a 256-entry table of their combinations (Four Russians).
    ///    The basis is in reduced form, so the eight rows have zeros at one
    ///    another's pivots and one pass clears every basis pivot column.
    /// 2. Bring the reduced batch to reduced row echelon form on its own.
    /// 3. Clear the new pivot columns from the old rows the same way, and
    ///    merge.  The result is the reduced row echelon form of the union.
    fn absorb(&mut self, mut batch: Vec<Vec<u64>>) -> Vec<usize> {
        let words = self.words;
        self.report.peak_matrix_rows = self
            .report
            .peak_matrix_rows
            .max(self.basis.len() + batch.len());
        let ops = reduce_by_groups(&mut batch, &self.basis, &self.piv, words);
        self.report.word_ops += ops;
        batch.retain(|r| r.iter().any(|&w| w != 0));
        let rank = if batch.is_empty() {
            0
        } else {
            rref_f2_counted(&mut batch, self.columns.len(), &mut self.report.word_ops)
        };
        batch.truncate(rank);
        let new_piv: Vec<usize> = batch
            .iter()
            .map(|r| leading_column(r).expect("rank rows are nonzero"))
            .collect();
        if !batch.is_empty() {
            let ops = reduce_by_groups(&mut self.basis, &batch, &new_piv, words);
            self.report.word_ops += ops;
        }
        // Merge two pivot-sorted lists.
        let old_rows = std::mem::take(&mut self.basis);
        let old_piv = std::mem::take(&mut self.piv);
        let mut rows = Vec::with_capacity(old_rows.len() + batch.len());
        let mut piv = Vec::with_capacity(rows.capacity());
        let mut a = old_rows.into_iter().zip(old_piv).peekable();
        let mut b = batch.into_iter().zip(new_piv.iter().copied()).peekable();
        loop {
            let take_a = match (a.peek(), b.peek()) {
                (Some((_, pa)), Some((_, pb))) => pa < pb,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => break,
            };
            let (r, p) = if take_a {
                a.next().unwrap()
            } else {
                b.next().unwrap()
            };
            rows.push(r);
            piv.push(p);
        }
        self.basis = rows;
        self.piv = piv;
        self.report.absorbs += 1;
        self.report.peak_basis_rows = self.report.peak_basis_rows.max(self.basis.len());
        if self.piv.last() == Some(&(self.columns.len() - 1)) {
            self.report.refuted = true;
        }
        new_piv
    }

    /// The row leading at column `c`, if any.
    fn row_at(&self, c: usize) -> Option<&Vec<u64>> {
        self.piv.binary_search(&c).ok().map(|i| &self.basis[i])
    }

    /// `row` times every variable (or, with `below`, only the variables of
    /// bit index at least `below`), as dense rows; zero products dropped.
    fn expand(&mut self, row: &[u64], below: Option<u32>, out: &mut Vec<Vec<u64>>) {
        let mut set: Vec<usize> = Vec::new();
        for (i, &w) in row.iter().enumerate() {
            let mut bits = w;
            while bits != 0 {
                set.push(i * 64 + bits.trailing_zeros() as usize);
                bits &= bits - 1;
            }
        }
        for v in 0..self.vars.len() {
            if below.is_some_and(|b| self.vars[v].trailing_zeros() < b) {
                continue;
            }
            let mut p = vec![0u64; self.words];
            for &c in &set {
                let t = self.times[v][c];
                debug_assert!(
                    t != u32::MAX,
                    "a low row times a variable stays within degree D"
                );
                let t = t as usize;
                p[t / 64] ^= 1u64 << (t % 64);
            }
            if p.iter().any(|&w| w != 0) {
                out.push(p);
            }
        }
        self.report.expanded += 1;
    }
}

/// Clear the pivot columns `piv` of the reduced rows `pivots` (sorted by
/// pivot, mutually reduced) from every row of `targets`, eight pivots per
/// table.  Returns the word XORs charged.
fn reduce_by_groups(
    targets: &mut [Vec<u64>],
    pivots: &[Vec<u64>],
    piv: &[usize],
    words: usize,
) -> u64 {
    if targets.is_empty() || pivots.is_empty() {
        return 0;
    }
    let mut ops = 0u64;
    let mut table: Vec<u64> = Vec::new();
    for g in (0..pivots.len()).step_by(8) {
        let k = (pivots.len() - g).min(8);
        let start = piv[g] / 64;
        let span = words - start;
        table.clear();
        table.resize((1usize << k) * span, 0);
        for mask in 1usize..(1 << k) {
            let low = mask.trailing_zeros() as usize;
            let prev = mask & (mask - 1);
            let (dst, src) = (mask * span, prev * span);
            let row = &pivots[g + low][start..];
            for w in 0..span {
                table[dst + w] = table[src + w] ^ row[w];
            }
        }
        ops += ((1u64 << k) - 1) * span as u64;
        let cols: Vec<usize> = piv[g..g + k].to_vec();
        let hits: u64 = targets
            .par_iter_mut()
            .map(|t| {
                let mut idx = 0usize;
                for (i, &c) in cols.iter().enumerate() {
                    idx |= ((t[c / 64] >> (c % 64)) as usize & 1) << i;
                }
                if idx == 0 {
                    return 0;
                }
                let src = &table[idx * span..(idx + 1) * span];
                for (d, s) in t[start..].iter_mut().zip(src) {
                    *d ^= *s;
                }
                1
            })
            .sum();
        ops += hits * span as u64;
    }
    ops
}

/// The degree-`degree` MutantXL closure of `system`, explored in `mode`.
/// `None` when the layout would exceed `max_cols` columns.
pub fn pe_closure(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    mode: PeMode,
    max_cols: usize,
    max_rows: usize,
) -> Option<PeReport> {
    pe_closure_with(
        system,
        n_vars,
        degree,
        mode,
        max_cols,
        max_rows,
        PeOptions::default(),
    )
}

fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// [`pe_closure`] with a queue order, seed degree and variable restriction
/// ([`PeOptions`]).  Every option computes the same closure; only the
/// order in which `1` can appear changes.
pub fn pe_closure_with(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    mode: PeMode,
    max_cols: usize,
    max_rows: usize,
    opts: PeOptions,
) -> Option<PeReport> {
    pe_closure_checkpointed(system, n_vars, degree, mode, max_cols, max_rows, opts, None)
}

/// Where and how often [`pe_closure_checkpointed`] saves its state.
#[derive(Clone, Debug)]
pub struct PeCheckpoint {
    /// The checkpoint file.  It is written to `path` + `.tmp`, synced, and
    /// renamed over `path`, so a crash never leaves a torn file.
    pub path: std::path::PathBuf,
    /// Minimum time between saves.  A save happens only at a consistent
    /// point: after a seed chunk or a whole expansion batch is absorbed.
    pub every: std::time::Duration,
}

/// Saved loop state beside the engine's basis and report.
struct Saved {
    /// 0: still absorbing the seed; 1: in the expansion loop.
    phase: u8,
    /// Next seed chunk to absorb (phase 0).
    seed_next: u64,
    full_vars: bool,
    queue: Vec<(u32, u32, usize)>,
    /// Pivots the seed has produced so far (phase 0).
    new: Vec<usize>,
    /// Wall seconds spent before this process started.
    secs_before: f64,
}

const CKPT_MAGIC: &[u8; 8] = b"PECKPT01";

/// Identity of a run: the system, the layout and every option.  A
/// checkpoint is resumed only under the same fingerprint.
fn fingerprint(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    mode: PeMode,
    max_cols: usize,
    opts: PeOptions,
) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(b"pe_closure checkpoint v1");
    h.update(&(n_vars as u64).to_le_bytes());
    h.update(&degree.to_le_bytes());
    h.update(&(max_cols as u64).to_le_bytes());
    match mode {
        PeMode::Full => h.update(&[0u8]),
        PeMode::Partial { batch_rows } => {
            h.update(&[1u8]);
            h.update(&(batch_rows as u64).to_le_bytes())
        }
    };
    match opts.order {
        PeOrder::LowFirst => h.update(&[0u8]),
        PeOrder::HighFirst => h.update(&[1u8]),
        PeOrder::Random(s) => {
            h.update(&[2u8]);
            h.update(&s.to_le_bytes())
        }
    };
    h.update(&opts.seed_degree.unwrap_or(u32::MAX).to_le_bytes());
    h.update(&[opts.restrict_vars as u8]);
    h.update(&(system.len() as u64).to_le_bytes());
    for p in system {
        h.update(&(p.terms.len() as u64).to_le_bytes());
        for t in &p.terms {
            h.update(&(t.mask as u128).to_le_bytes());
        }
    }
    *h.finalize().as_bytes()
}

fn put_u64(w: &mut impl std::io::Write, v: u64) -> std::io::Result<()> {
    w.write_all(&v.to_le_bytes())
}

fn get_u64(r: &mut impl std::io::Read) -> std::io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}

fn save_checkpoint(
    e: &Engine,
    ck: &PeCheckpoint,
    fp: &[u8; 32],
    st: &Saved,
) -> std::io::Result<()> {
    use std::io::Write;
    let mut tmp = ck.path.clone().into_os_string();
    tmp.push(".tmp");
    let tmp = std::path::PathBuf::from(tmp);
    let file = std::fs::File::create(&tmp)?;
    let mut w = std::io::BufWriter::with_capacity(1 << 22, file);
    w.write_all(CKPT_MAGIC)?;
    w.write_all(fp)?;
    w.write_all(&[st.phase, st.full_vars as u8])?;
    put_u64(&mut w, st.seed_next)?;
    put_u64(&mut w, st.secs_before.to_bits())?;
    let r = &e.report;
    for v in [
        r.peak_basis_rows as u64,
        r.peak_matrix_rows as u64,
        r.absorbs as u64,
        r.expanded,
        r.products,
        r.word_ops,
        r.completion_at_rows.map_or(u64::MAX, |x| x as u64),
    ] {
        put_u64(&mut w, v)?;
    }
    put_u64(&mut w, e.words as u64)?;
    put_u64(&mut w, e.basis.len() as u64)?;
    for &c in &e.piv {
        put_u64(&mut w, c as u64)?;
    }
    let mut buf = Vec::with_capacity(e.words * 8);
    for row in &e.basis {
        buf.clear();
        for x in row {
            buf.extend_from_slice(&x.to_le_bytes());
        }
        w.write_all(&buf)?;
    }
    put_u64(&mut w, st.queue.len() as u64)?;
    for &(a, b, c) in &st.queue {
        w.write_all(&a.to_le_bytes())?;
        w.write_all(&b.to_le_bytes())?;
        put_u64(&mut w, c as u64)?;
    }
    put_u64(&mut w, st.new.len() as u64)?;
    for &c in &st.new {
        put_u64(&mut w, c as u64)?;
    }
    w.write_all(b"END!")?;
    let file = w.into_inner().map_err(|e| e.into_error())?;
    file.sync_all()?;
    std::fs::rename(&tmp, &ck.path)?;
    if let Some(dir) = ck.path.parent() {
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
    }
    Ok(())
}

/// Load a checkpoint into `e`.  `Ok(None)` when there is none; an error
/// when it is torn or belongs to a different run (never silently ignored).
fn load_checkpoint(
    e: &mut Engine,
    ck: &PeCheckpoint,
    fp: &[u8; 32],
) -> std::io::Result<Option<Saved>> {
    use std::io::{Error, ErrorKind, Read};
    let file = match std::fs::File::open(&ck.path) {
        Ok(f) => f,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    let mut r = std::io::BufReader::with_capacity(1 << 22, file);
    let bad = |m: &str| Error::new(ErrorKind::InvalidData, m.to_string());
    let mut magic = [0u8; 8];
    r.read_exact(&mut magic)?;
    if &magic != CKPT_MAGIC {
        return Err(bad("not a pe_closure checkpoint"));
    }
    let mut got = [0u8; 32];
    r.read_exact(&mut got)?;
    if &got != fp {
        return Err(bad("checkpoint belongs to a different system or options"));
    }
    let mut flags = [0u8; 2];
    r.read_exact(&mut flags)?;
    let seed_next = get_u64(&mut r)?;
    let secs_before = f64::from_bits(get_u64(&mut r)?);
    let mut counters = [0u64; 7];
    for v in counters.iter_mut() {
        *v = get_u64(&mut r)?;
    }
    let words = get_u64(&mut r)? as usize;
    if words != e.words {
        return Err(bad("checkpoint row width differs from the layout"));
    }
    let nrows = get_u64(&mut r)? as usize;
    let mut piv = Vec::with_capacity(nrows);
    for _ in 0..nrows {
        piv.push(get_u64(&mut r)? as usize);
    }
    let mut basis = Vec::with_capacity(nrows);
    let mut buf = vec![0u8; words * 8];
    for _ in 0..nrows {
        r.read_exact(&mut buf)?;
        basis.push(
            buf.chunks_exact(8)
                .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
                .collect::<Vec<u64>>(),
        );
    }
    let nq = get_u64(&mut r)? as usize;
    let mut queue = Vec::with_capacity(nq);
    for _ in 0..nq {
        let mut a = [0u8; 4];
        let mut b = [0u8; 4];
        r.read_exact(&mut a)?;
        r.read_exact(&mut b)?;
        queue.push((
            u32::from_le_bytes(a),
            u32::from_le_bytes(b),
            get_u64(&mut r)? as usize,
        ));
    }
    let nn = get_u64(&mut r)? as usize;
    let mut new = Vec::with_capacity(nn);
    for _ in 0..nn {
        new.push(get_u64(&mut r)? as usize);
    }
    let mut end = [0u8; 4];
    r.read_exact(&mut end)?;
    if &end != b"END!" {
        return Err(bad("checkpoint is torn"));
    }
    e.basis = basis;
    e.piv = piv;
    let rep = &mut e.report;
    rep.peak_basis_rows = counters[0] as usize;
    rep.peak_matrix_rows = counters[1] as usize;
    rep.absorbs = counters[2] as usize;
    rep.expanded = counters[3];
    rep.products = counters[4];
    rep.word_ops = counters[5];
    rep.completion_at_rows = (counters[6] != u64::MAX).then_some(counters[6] as usize);
    Ok(Some(Saved {
        phase: flags[0],
        seed_next,
        full_vars: flags[1] != 0,
        queue,
        new,
        secs_before,
    }))
}

/// [`pe_closure_with`] that saves its state to `ckpt` at consistent points
/// and resumes from it when the file exists, so a killed process (or a
/// lost host whose disk survives) continues where it stopped.  The resumed
/// run computes exactly what an uninterrupted one would: the same basis,
/// queue order and counters.
#[allow(clippy::too_many_arguments)]
pub fn pe_closure_checkpointed(
    system: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    mode: PeMode,
    max_cols: usize,
    max_rows: usize,
    opts: PeOptions,
    ckpt: Option<&PeCheckpoint>,
) -> Option<PeReport> {
    let started = Instant::now();
    let label = match mode {
        PeMode::Full => "full",
        PeMode::Partial { .. } => "partial",
    };
    let mut e = Engine::new(system, degree, max_cols, label)?;
    let fp = fingerprint(system, n_vars, degree, mode, max_cols, opts);
    let resumed = ckpt.and_then(|c| {
        load_checkpoint(&mut e, c, &fp)
            .unwrap_or_else(|err| panic!("checkpoint {}: {err}", c.path.display()))
    });
    let secs_before = resumed.as_ref().map_or(0.0, |s| s.secs_before);
    let mut last_save = Instant::now();
    let mut maybe_save = |e: &Engine, st: &dyn Fn() -> Saved| {
        if let Some(c) = ckpt {
            if last_save.elapsed() >= c.every {
                // A failed save (a full disk, say) must not cost the run:
                // the previous checkpoint is still intact, because the new
                // one only replaces it by rename after a complete write.
                if let Err(err) = save_checkpoint(e, c, &fp, &st()) {
                    eprintln!(
                        "checkpoint {}: save failed, continuing: {err}",
                        c.path.display()
                    );
                    let mut tmp = c.path.clone().into_os_string();
                    tmp.push(".tmp");
                    let _ = std::fs::remove_file(std::path::PathBuf::from(tmp));
                }
                last_save = Instant::now();
            }
        }
    };
    let occurring = e.vars.iter().fold(0 as Mask, |a, v| a | v);
    let seed_degree = match mode {
        PeMode::Full => degree,
        PeMode::Partial { .. } => opts.seed_degree.unwrap_or(degree.saturating_sub(1)).max(
            system
                .iter()
                .filter_map(|p| p.terms.first())
                .map(|t| t.mask.count_ones())
                .max()
                .unwrap_or(0),
        ),
    };
    let in_loop = resumed.as_ref().is_some_and(|s| s.phase == 1);
    let mut new = resumed.as_ref().map_or(Vec::new(), |s| s.new.clone());
    if !in_loop {
        let seed = macaulay_rows_monos_with_mask(system, n_vars, seed_degree, occurring, None)?;
        // Absorb the seed in chunks so one elimination never holds it all.
        let first = resumed.as_ref().map_or(0, |s| s.seed_next as usize);
        for (i, chunk) in seed.chunks(8192).enumerate().skip(first) {
            let rows: Vec<Vec<u64>> = chunk.iter().map(|m| e.pack(m)).collect();
            new.extend(e.absorb(rows));
            maybe_save(&e, &|| Saved {
                phase: 0,
                seed_next: i as u64 + 1,
                full_vars: !opts.restrict_vars,
                queue: Vec::new(),
                new: new.clone(),
                secs_before: secs_before + started.elapsed().as_secs_f64(),
            });
            if e.report.refuted || e.basis.len() > max_rows {
                break;
            }
        }
    }
    // Queue keyed (leading degree, leading-variable rank, column): the
    // lowest degree first; within a degree the smallest leading variable
    // (highest index) first.  The other orders change the first component.
    let key = |e: &Engine, c: usize| match opts.order {
        PeOrder::LowFirst => (e.lead_degree(c), 64 - e.lead_var(c), c),
        PeOrder::HighFirst => (degree - e.lead_degree(c), 64 - e.lead_var(c), c),
        PeOrder::Random(s) => ((splitmix(c as u64 ^ s) >> 32) as u32, 0, c),
    };
    let mut full_vars = !opts.restrict_vars;
    let mut queue: BTreeSet<(u32, u32, usize)> = BTreeSet::new();
    if let Some(s) = resumed.as_ref().filter(|s| s.phase == 1) {
        full_vars = s.full_vars;
        queue.extend(s.queue.iter().copied());
    } else {
        for c in new {
            if e.lead_degree(c) < degree {
                queue.insert(key(&e, c));
            }
        }
    }
    loop {
        if queue.is_empty() && !full_vars && !e.report.refuted {
            // Completion pass: every low row by every variable.
            full_vars = true;
            e.report.completion_at_rows = Some(e.basis.len());
            for i in 0..e.piv.len() {
                let c = e.piv[i];
                if e.lead_degree(c) < degree {
                    queue.insert(key(&e, c));
                }
            }
        }
        if e.report.refuted || queue.is_empty() {
            break;
        }
        if e.basis.len() > max_rows {
            e.report.capped = true;
            break;
        }
        let take: Vec<(u32, u32, usize)> = match mode {
            PeMode::Full => std::mem::take(&mut queue).into_iter().collect(),
            PeMode::Partial { batch_rows } => {
                let mut v = Vec::new();
                while v.len() < batch_rows.max(1) {
                    match queue.pop_first() {
                        Some(k) => v.push(k),
                        None => break,
                    }
                }
                v
            }
        };
        let mut batch = Vec::new();
        for (_, _, c) in take {
            // The row now leading at `c`; a lead can vanish only if the
            // basis was reduced around it, which an RREF never does.
            let Some(row) = e.row_at(c).cloned() else {
                continue;
            };
            let below = (!full_vars).then(|| e.lead_var(c));
            e.expand(&row, below, &mut batch);
        }
        e.report.products += batch.len() as u64;
        while !batch.is_empty() && !e.report.refuted {
            let rest = batch.split_off(batch.len().min(8192));
            let chunk = std::mem::replace(&mut batch, rest);
            for c in e.absorb(chunk) {
                if e.lead_degree(c) < degree {
                    queue.insert(key(&e, c));
                }
            }
        }
        if !e.report.refuted {
            maybe_save(&e, &|| Saved {
                phase: 1,
                seed_next: 0,
                full_vars,
                queue: queue.iter().copied().collect(),
                new: Vec::new(),
                secs_before: secs_before + started.elapsed().as_secs_f64(),
            });
        }
    }
    e.report.refuted_in_completion = e.report.refuted && e.report.completion_at_rows.is_some();
    e.report.saturated = e.report.refuted || (queue.is_empty() && !e.report.capped);
    e.report.final_rows = e.basis.len();
    e.report.secs = secs_before + started.elapsed().as_secs_f64();
    let _ = e.degree;
    Some(e.report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::F2mElement;
    use crate::cryptanalysis::inherited_f4::ReducedBasis;
    use crate::cryptanalysis::koblitz_bench::ladder_sample;
    use crate::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn draws(m: usize, n: u32, ell: usize, k: usize, seed: u64) -> Vec<(Vec<F2BoolPoly>, usize)> {
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let b = F2mElement::one(n);
        let mut rng = StdRng::seed_from_u64(seed);
        (0..k)
            .filter_map(|_| {
                let (basis, x_r) = ladder_sample(n, ell, &mut rng);
                build_decomposition_system(&basis, &x_r, &b, m, &st)
                    .map(|s| (s.equations, s.n_vars))
            })
            .collect()
    }

    /// Both orders reach the dense closure: the same refutation verdict at
    /// every degree, and, where nothing refutes, the same saturated rank.
    #[test]
    fn both_modes_agree_with_the_dense_closure() {
        let mut refuted_seen = 0;
        for (m, n, ell) in [(3usize, 7u32, 2usize), (3, 9, 3), (4, 5, 2)] {
            for (sys, n_vars) in draws(m, n, ell, 6, 20261024 + n as u64) {
                for d in 3..=4u32 {
                    let Some((dense, _)) =
                        ReducedBasis::from_system_closed_full(&sys, n_vars, d, 1_000_000)
                    else {
                        continue;
                    };
                    let full =
                        pe_closure(&sys, n_vars, d, PeMode::Full, 1 << 20, usize::MAX).unwrap();
                    let part = pe_closure(
                        &sys,
                        n_vars,
                        d,
                        PeMode::Partial { batch_rows: 4 },
                        1 << 20,
                        usize::MAX,
                    )
                    .unwrap();
                    assert_eq!(full.refuted, dense.is_refuted(), "full m={m} n={n} d={d}");
                    assert_eq!(
                        part.refuted,
                        dense.is_refuted(),
                        "partial m={m} n={n} d={d}"
                    );
                    assert!(full.saturated && part.saturated);
                    if dense.is_refuted() {
                        refuted_seen += 1;
                    } else {
                        assert_eq!(full.final_rows, dense.rank(), "full rank m={m} n={n} d={d}");
                        assert_eq!(
                            part.final_rows,
                            dense.rank(),
                            "partial rank m={m} n={n} d={d}"
                        );
                    }
                }
            }
        }
        assert!(refuted_seen > 0, "the panel exercises refutation");
    }

    /// A planted solution is never refuted, in either mode.
    /// A run interrupted at any point and resumed from its checkpoint ends
    /// exactly where an uninterrupted run does: same verdict, rank and
    /// counters.  A checkpoint for another system is refused.
    #[test]
    fn resumed_run_equals_uninterrupted_run() {
        let dir = std::env::temp_dir().join(format!("pe_ckpt_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut checked = 0;
        for (m, n, ell) in [(3usize, 9u32, 3usize), (4, 5, 2)] {
            for (k, (sys, n_vars)) in draws(m, n, ell, 2, 20261103 + n as u64)
                .into_iter()
                .enumerate()
            {
                for restrict_vars in [false, true] {
                    let opts = PeOptions {
                        restrict_vars,
                        ..PeOptions::default()
                    };
                    let mode = PeMode::Partial { batch_rows: 8 };
                    let whole =
                        pe_closure_with(&sys, n_vars, 4, mode, 1 << 20, usize::MAX, opts).unwrap();
                    for cut in [
                        whole.final_rows / 4,
                        whole.final_rows / 2,
                        whole.final_rows * 9 / 10,
                    ] {
                        let ck = PeCheckpoint {
                            path: dir.join(format!("m{m}n{n}k{k}r{restrict_vars}c{cut}.ckpt")),
                            every: std::time::Duration::ZERO,
                        };
                        let _ = std::fs::remove_file(&ck.path);
                        let first = pe_closure_checkpointed(
                            &sys,
                            n_vars,
                            4,
                            mode,
                            1 << 20,
                            cut.max(1),
                            opts,
                            Some(&ck),
                        )
                        .unwrap();
                        if first.refuted || !first.capped {
                            continue;
                        }
                        let resumed = pe_closure_checkpointed(
                            &sys,
                            n_vars,
                            4,
                            mode,
                            1 << 20,
                            usize::MAX,
                            opts,
                            Some(&ck),
                        )
                        .unwrap();
                        assert_eq!(
                            (
                                resumed.refuted,
                                resumed.saturated,
                                resumed.final_rows,
                                resumed.products,
                                resumed.expanded,
                                resumed.absorbs,
                                resumed.word_ops,
                                resumed.peak_basis_rows,
                                resumed.completion_at_rows
                            ),
                            (
                                whole.refuted,
                                whole.saturated,
                                whole.final_rows,
                                whole.products,
                                whole.expanded,
                                whole.absorbs,
                                whole.word_ops,
                                whole.peak_basis_rows,
                                whole.completion_at_rows
                            ),
                            "m={m} n={n} k={k} restrict={restrict_vars} cut={cut}"
                        );
                        checked += 1;
                    }
                }
            }
        }
        // A checkpoint is refused for a different system.
        let mut refused = false;
        if let Some(path) = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .find(|p| p.extension().is_some_and(|x| x == "ckpt"))
        {
            let (sys, n_vars) = draws(3, 7, 2, 1, 1).remove(0);
            let ck = PeCheckpoint {
                path,
                every: std::time::Duration::ZERO,
            };
            refused = std::panic::catch_unwind(|| {
                pe_closure_checkpointed(
                    &sys,
                    n_vars,
                    4,
                    PeMode::Partial { batch_rows: 8 },
                    1 << 20,
                    usize::MAX,
                    PeOptions::default(),
                    Some(&ck),
                )
            })
            .is_err();
        }
        let _ = std::fs::remove_dir_all(&dir);
        assert!(
            checked >= 4,
            "the panel exercised resumption ({checked} cases)"
        );
        assert!(refused, "a foreign checkpoint must be refused");
    }

    #[test]
    fn every_order_computes_the_same_closure() {
        let orders = [
            PeOrder::LowFirst,
            PeOrder::HighFirst,
            PeOrder::Random(1),
            PeOrder::Random(2),
        ];
        let mut refuted_seen = 0;
        for (m, n, ell) in [(3usize, 7u32, 2usize), (3, 9, 3), (4, 5, 2)] {
            for (sys, n_vars) in draws(m, n, ell, 2, 20261025 + n as u64) {
                for d in 3..=4u32 {
                    let Some((dense, _)) =
                        ReducedBasis::from_system_closed_full(&sys, n_vars, d, 1_000_000)
                    else {
                        continue;
                    };
                    for order in orders {
                        for restrict_vars in [false, true] {
                            for seed_degree in [None, Some(0)] {
                                let opts = PeOptions {
                                    order,
                                    seed_degree,
                                    restrict_vars,
                                };
                                let r = pe_closure_with(
                                    &sys,
                                    n_vars,
                                    d,
                                    PeMode::Partial { batch_rows: 16 },
                                    1 << 20,
                                    usize::MAX,
                                    opts,
                                )
                                .unwrap();
                                assert_eq!(r.refuted, dense.is_refuted(), "{opts:?} m={m} d={d}");
                                assert!(r.saturated);
                                if !dense.is_refuted() {
                                    assert_eq!(r.final_rows, dense.rank(), "{opts:?} m={m} d={d}");
                                } else {
                                    refuted_seen += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(refuted_seen > 0, "the panel exercises refutation");
    }

    #[test]
    fn satisfiable_systems_are_never_refuted() {
        use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;
        // x0·x1 + x2 = 0, x1 + x2 + x3 = 0, x0·x3 + 1 = 0 has the solution
        // x0 = x3 = 1, x1 = x2 = 0 … check: 0 + 0 = 0; 0+0+1 ≠ 0.  Use
        // x0 = 1, x3 = 1, x1 = 1, x2 = 0: x0x1 + x2 = 1 ≠ 0.  Build the
        // system from a chosen point instead.
        let point = [true, false, true, true, false];
        let mono = |m: Mask| F2BoolMono::from_mask(m);
        let eval = |m: Mask| (0..5).all(|i| m >> i & 1 == 0 || point[i]);
        let mut sys = Vec::new();
        for masks in [
            vec![0b00011 as Mask, 0b00100],
            vec![0b00110, 0b01001, 0b10000],
            vec![0b11000, 0b00101],
        ] {
            let mut monos: Vec<F2BoolMono> = masks.iter().map(|&m| mono(m)).collect();
            // fix the constant so the point satisfies it
            if masks.iter().filter(|&&m| eval(m)).count() % 2 == 1 {
                monos.push(mono(0));
            }
            sys.push(F2BoolPoly::from_monos(monos, 5));
        }
        for d in 2..=5 {
            for mode in [PeMode::Full, PeMode::Partial { batch_rows: 1 }] {
                let r = pe_closure(&sys, 5, d, mode, 1 << 16, usize::MAX).unwrap();
                assert!(!r.refuted, "{mode:?} at {d}");
                assert!(r.saturated);
            }
        }
    }
}
