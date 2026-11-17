//! # Prefix-cached roots: the target-free links of a chain, reduced once.
//!
//! A chained decomposition system for `m ≥ 3` summands is
//! `S₃(x₁,x₂,e₁), S₃(e₁,x₃,e₂), …, S₃(e_{m−2},x_m,x_R)`: only the last
//! link reads the target.  Its first `(m − 2)·n` generators, and so every
//! Macaulay row built from them, are the same for every target of a
//! factor base.  The from-scratch root reduces those rows again for each
//! target.  This module reduces them **once**: the prefix rows are brought
//! to reduced row echelon form and cut into blocks of `K` pivots, each with
//! a table of all `2^K` sums of its rows (Four Russians).  A target then
//! pays only for its own link: each of the link's rows is reduced against
//! the cached prefix by one table lookup and one row XOR per block, and the
//! residuals are brought to echelon form among themselves.
//!
//! Exact: the Macaulay rows of a system are the union of the rows of its
//! generators (multipliers are per generator, under either multiplier
//! policy, with the whole system's occurring variables as the multiplier
//! mask), and the observed columns of the union are the union of the
//! observed columns.  So the cached RREF rows and the residual echelon rows
//! span exactly the from-scratch root's row space on exactly its columns:
//! every tail, refutation, forced variable, split and verdict is the same.
//! The basis rows differ (the prefix part is fully reduced), so the work
//! below the root may differ, and is measured, not assumed.
//!
//! ## Accounting
//!
//! The stage unit is charged per target for everything a target does:
//! the copy of the cached prefix rows into its basis (one word operation
//! per word written), the read of each link row's pivot bits (one per word
//! read), one table XOR per block (the words from the block's first
//! leading word to the end of the row), and the residual echelon on the
//! repository's elimination kernel.  The one-time work — the prefix
//! elimination, its back-substitution and the tables — is charged to the
//! first root that builds the cache *and* reported separately by
//! [`prefix_root_counts`], so a comparison can amortise it over any number
//! of targets or charge it in full.

use super::{leading_column, InheritCost, ReducedBasis};
use crate::cryptanalysis::bitop_tally;
use crate::cryptanalysis::koblitz_groebner::{
    all_variable_mask, build_inherited_macaulay, build_inherited_macaulay_support_local,
    echelon_f2_counted, rref_f2_counted,
};
use crate::cryptanalysis::pq_groebner_f2::{mono_key, F2BoolMono, F2BoolPoly, Mask};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Pivots per Four Russians block.
const K: usize = 8;

/// Most prefix caches kept per thread (one per factor base and degree).
const MAX_ENTRIES: usize = 4;

/// Word budget of one entry's tables; above it the block width is halved.
const TABLE_WORD_CAP: usize = 1 << 26;

thread_local! {
    static OVERRIDE: Cell<Option<bool>> = const { Cell::new(None) };
    static HINT: Cell<usize> = const { Cell::new(0) };
    static CACHE: RefCell<Vec<Rc<PrefixEntry>>> = const { RefCell::new(Vec::new()) };
    static COUNTS: Cell<(u64, u64)> = const { Cell::new((0, 0)) };
}

/// Whether roots use the prefix cache on this thread: [`set_prefix_root`]
/// if called, else `KIC_F4_PREFIX_ROOT=1` (off by default).
pub fn prefix_root_enabled() -> bool {
    static ENV: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    OVERRIDE.with(Cell::get).unwrap_or_else(|| {
        *ENV.get_or_init(|| std::env::var("KIC_F4_PREFIX_ROOT").as_deref() == Ok("1"))
    })
}

/// Turn the prefix cache on or off on this thread (a same-binary control).
pub fn set_prefix_root(on: bool) {
    OVERRIDE.with(|c| c.set(Some(on)));
}

/// The number of leading target-free generators of the system the solver
/// on this thread is about to be handed (`0`: none known).  Set by the
/// chain decomposers; read by the root builder.
pub fn set_prefix_hint(generators: usize) {
    HINT.with(|c| c.set(generators));
}

/// See [`set_prefix_hint`].
pub fn prefix_hint() -> usize {
    HINT.with(Cell::get)
}

/// `(caches built, one-time word operations)` on this thread since start.
pub fn prefix_root_counts() -> (u64, u64) {
    COUNTS.with(Cell::get)
}

/// Drop this thread's prefix caches.
pub fn clear_prefix_cache() {
    CACHE.with(|c| c.borrow_mut().clear());
}

/// One cached prefix: its generators (the key), the root parameters, the
/// observed columns of its rows and their reduced row echelon form.
struct PrefixEntry {
    prefix: Vec<F2BoolPoly>,
    n_vars: usize,
    degree: u32,
    support_local: bool,
    occurring: Mask,
    quadratic: bool,
    columns: Vec<Mask>,
    /// RREF rows on `columns`, in pivot order.
    rows: Vec<Vec<u64>>,
    /// The tables, keyed by the layout they were remapped to.
    layout: RefCell<Option<Rc<Layout>>>,
}

/// The cached prefix on a target's full column layout.
struct Layout {
    columns: Vec<Mask>,
    /// Prefix RREF rows remapped to `columns`.
    rows: Vec<Vec<u64>>,
    pivots: Vec<usize>,
    block: usize,
    /// Per block: first leading word, and `2^block` sums of its rows (from
    /// that word on).
    tables: Vec<(usize, Vec<Vec<u64>>)>,
}

fn rows_of(
    polys: &[F2BoolPoly],
    n_vars: usize,
    degree: u32,
    support_local: bool,
    occurring: Mask,
    quadratic: bool,
) -> Option<(Vec<Mask>, Vec<Vec<u64>>)> {
    if support_local {
        build_inherited_macaulay_support_local(polys, n_vars, degree, occurring, quadratic)
    } else {
        build_inherited_macaulay(polys, n_vars, degree, occurring, quadratic)
    }
}

fn sort_key(m: Mask) -> std::cmp::Reverse<crate::cryptanalysis::pq_groebner_f2::MonoKey> {
    std::cmp::Reverse(mono_key(F2BoolMono::from_mask(m)))
}

/// Remap `row` (on `from`) to the layout `to`, given each source column's
/// target position.
fn remap(row: &[u64], pos: &[usize], words: usize) -> Vec<u64> {
    let mut out = vec![0u64; words];
    for (w, &x) in row.iter().enumerate() {
        let mut x = x;
        while x != 0 {
            let j = w * 64 + x.trailing_zeros() as usize;
            let p = pos[j];
            out[p / 64] |= 1u64 << (p % 64);
            x &= x - 1;
        }
    }
    out
}

fn positions(from: &[Mask], to: &[Mask]) -> Option<Vec<usize>> {
    let index: std::collections::HashMap<Mask, usize> =
        to.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    from.iter().map(|m| index.get(m).copied()).collect()
}

impl PrefixEntry {
    fn build(
        prefix: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
        occurring: Mask,
        quadratic: bool,
        once: &mut u64,
    ) -> Option<Self> {
        let (columns, mut rows) =
            rows_of(prefix, n_vars, degree, support_local, occurring, quadratic)?;
        if !rows.is_empty() {
            let rank = rref_f2_counted(&mut rows, columns.len(), once);
            rows.truncate(rank);
        }
        Some(Self {
            prefix: prefix.to_vec(),
            n_vars,
            degree,
            support_local,
            occurring,
            quadratic,
            columns,
            rows,
            layout: RefCell::new(None),
        })
    }

    fn matches(
        &self,
        prefix: &[F2BoolPoly],
        n_vars: usize,
        degree: u32,
        support_local: bool,
        occurring: Mask,
        quadratic: bool,
    ) -> bool {
        self.n_vars == n_vars
            && self.degree == degree
            && self.support_local == support_local
            && self.occurring == occurring
            && self.quadratic == quadratic
            && self.prefix.as_slice() == prefix
    }

    /// The tables on layout `columns`, built (and charged to `once`) on
    /// first use of that layout.
    fn layout(&self, columns: &[Mask], once: &mut u64) -> Option<Rc<Layout>> {
        if let Some(l) = self.layout.borrow().as_ref() {
            if l.columns == columns {
                return Some(l.clone());
            }
        }
        let words = columns.len().div_ceil(64).max(1);
        let pos = positions(&self.columns, columns)?;
        let rows: Vec<Vec<u64>> = self.rows.iter().map(|r| remap(r, &pos, words)).collect();
        *once += (rows.len() * words) as u64;
        let pivots: Vec<usize> = rows
            .iter()
            .map(|r| leading_column(r).expect("rank rows are nonzero"))
            .collect();
        // Column order is preserved by the remap, so the rows stay in
        // pivot order and stay reduced.
        debug_assert!(pivots.windows(2).all(|w| w[0] < w[1]));
        let mut block = K;
        while block > 1 && rows.len().div_ceil(block) * (1 << block) * words > TABLE_WORD_CAP {
            block /= 2;
        }
        let mut tables = Vec::with_capacity(rows.len().div_ceil(block));
        for chunk in rows.chunks(block) {
            let lead = pivots[tables.len() * block] / 64;
            let mut t: Vec<Vec<u64>> = Vec::with_capacity(1 << chunk.len());
            t.push(vec![0u64; words - lead]);
            for (b, row) in chunk.iter().enumerate() {
                for i in 0..(1usize << b) {
                    let mut s = t[i].clone();
                    for (d, x) in s.iter_mut().zip(&row[lead..]) {
                        *d ^= x;
                    }
                    t.push(s);
                }
                *once += (1u64 << b) * (words - lead) as u64;
            }
            tables.push((lead, t));
        }
        let l = Rc::new(Layout {
            columns: columns.to_vec(),
            rows,
            pivots,
            block,
            tables,
        });
        *self.layout.borrow_mut() = Some(l.clone());
        Some(l)
    }
}

impl ReducedBasis {
    /// The root of `system`, whose first `prefix` generators are reduced
    /// from this thread's cache (built on first use) and only the rest per
    /// call.  Spans exactly the row space of
    /// [`ReducedBasis::from_system_with`] on the same columns; see the
    /// module notes.  The returned cost holds the per-target work, plus the
    /// one-time work when this call built the cache.  `None` past the F4
    /// size caps, or where the prefix does not apply (no prefix, an empty
    /// link, or a column of the prefix outside the system's layout), in
    /// which case the caller builds from scratch.
    pub fn from_system_with_prefix(
        system: &[F2BoolPoly],
        prefix: usize,
        n_vars: usize,
        degree: u32,
        support_local: bool,
    ) -> Option<(Self, InheritCost)> {
        if prefix == 0 || prefix >= system.len() || system.iter().any(F2BoolPoly::is_zero) {
            return None;
        }
        // `bitop/v1`: the copy, the degree scan and the support fold, as
        // the from-scratch root charges them
        bitop_tally::word_ops(3 * system.iter().map(|p| 1 + p.terms.len() as u64).sum::<u64>());
        let degrees: Vec<u32> = system.iter().map(super::poly_degree).collect();
        let quadratic = degrees.iter().all(|&d| d <= 2);
        let occurring = system
            .iter()
            .flat_map(|p| p.terms.iter())
            .fold(0 as Mask, |acc, t| acc | t.mask)
            & all_variable_mask(n_vars);
        let (head, link) = system.split_at(prefix);
        let mut once = 0u64;
        let entry = CACHE.with(|c| {
            c.borrow()
                .iter()
                .find(|e| e.matches(head, n_vars, degree, support_local, occurring, quadratic))
                .cloned()
        });
        let entry = match entry {
            Some(e) => e,
            None => {
                let e = Rc::new(PrefixEntry::build(
                    head,
                    n_vars,
                    degree,
                    support_local,
                    occurring,
                    quadratic,
                    &mut once,
                )?);
                CACHE.with(|c| {
                    let mut c = c.borrow_mut();
                    if c.len() >= MAX_ENTRIES {
                        c.remove(0);
                    }
                    c.push(e.clone());
                });
                COUNTS.with(|c| c.set((c.get().0 + 1, c.get().1)));
                e
            }
        };
        let (link_columns, link_rows) =
            rows_of(link, n_vars, degree, support_local, occurring, quadratic)?;
        super::note_rows_generated(
            link_rows.len() as u64,
            (link_rows.len() * link_columns.len().div_ceil(64)) as u64,
        );
        // The from-scratch layout: the union of the observed columns, in
        // the builder's order.
        let mut columns: Vec<Mask> = entry.columns.clone();
        columns.extend_from_slice(&link_columns);
        columns.sort_unstable_by_key(|&m| sort_key(m));
        columns.dedup();
        bitop_tally::word_ops(columns.len() as u64 + bitop_tally::sort_cost(columns.len()));
        let words = columns.len().div_ceil(64).max(1);
        let layout = entry.layout(&columns, &mut once)?;
        let link_pos = positions(&link_columns, &columns)?;

        let mut cost = InheritCost::default();
        // Per target: copy the prefix rows into this basis.
        cost.reduce_word_ops += (layout.rows.len() * words) as u64;
        // Reduce each link row against the cached RREF prefix.
        let mut residual: Vec<Vec<u64>> = Vec::with_capacity(link_rows.len());
        for r in &link_rows {
            let mut row = remap(r, &link_pos, words);
            cost.reduce_word_ops += words as u64;
            for (b, (lead, table)) in layout.tables.iter().enumerate() {
                let base = b * layout.block;
                let end = (base + layout.block).min(layout.pivots.len());
                let mut idx = 0usize;
                for (i, &p) in layout.pivots[base..end].iter().enumerate() {
                    idx |= (((row[p / 64] >> (p % 64)) & 1) as usize) << i;
                }
                if idx != 0 {
                    for (d, x) in row[*lead..].iter_mut().zip(&table[idx]) {
                        *d ^= x;
                    }
                    cost.reduce_word_ops += (words - lead) as u64;
                }
            }
            if row.iter().any(|&w| w != 0) {
                residual.push(row);
            }
        }
        if !residual.is_empty() {
            let rank = echelon_f2_counted(&mut residual, columns.len(), &mut cost.reduce_word_ops);
            residual.truncate(rank);
        }
        if once != 0 {
            COUNTS.with(|c| c.set((c.get().0, c.get().1 + once)));
            cost.reduce_word_ops += once;
        }
        super::note_root_elimination(cost.reduce_word_ops);
        let mut rows: Vec<Vec<u64>> = layout.rows.clone();
        rows.extend(residual);
        let system: Rc<Vec<F2BoolPoly>> = Rc::new(system.to_vec());
        let out = Self::assemble(
            system,
            Rc::new(degrees),
            n_vars,
            degree,
            support_local,
            columns,
            rows,
        );
        Some((out, cost))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pq_groebner_f2::F2BoolMono;

    /// A pseudo-random Boolean polynomial in `n` variables of degree ≤ `deg`.
    fn poly(seed: &mut u64, n: usize, deg: u32, terms: usize) -> F2BoolPoly {
        let mut monos = Vec::new();
        for _ in 0..terms {
            let mut mask: Mask = 0;
            for _ in 0..deg {
                *seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                if !(*seed >> 40).is_multiple_of(3) {
                    mask |= (1 as Mask) << ((*seed >> 33) as usize % n);
                }
            }
            monos.push(F2BoolMono::from_mask(mask));
        }
        F2BoolPoly::from_monos(monos, n)
    }

    #[test]
    fn prefix_root_spans_the_from_scratch_root() {
        let n = 12;
        for (seed0, support_local) in [(1u64, true), (2, false), (3, true)] {
            let mut seed = seed0;
            let prefix: Vec<F2BoolPoly> = (0..6).map(|_| poly(&mut seed, n, 3, 6)).collect();
            clear_prefix_cache();
            for _target in 0..3 {
                let mut system = prefix.clone();
                system.extend((0..8).map(|_| poly(&mut seed, n, 2, 5)));
                if system.iter().any(F2BoolPoly::is_zero) {
                    continue;
                }
                let (scratch, _) =
                    ReducedBasis::from_system_with(&system, n, 3, support_local).unwrap();
                let (cached, _) =
                    ReducedBasis::from_system_with_prefix(&system, 6, n, 3, support_local).unwrap();
                assert!(cached.same_row_space(&scratch));
            }
            assert_eq!(
                CACHE.with(|c| c.borrow().len()),
                1,
                "one cache entry per prefix"
            );
        }
    }
}
