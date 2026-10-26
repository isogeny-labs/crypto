//! Parallel mutant-closure engine for the Huang–Kosters–Yeo last fall degree,
//! for the large cells of `research/fall_degree_bounds_20261008` (E7).
//!
//! Same closure as `examples/hky_last_fall_degree.rs` (validated against
//! Sage): the degree-`c` Macaulay rows `m·f`, closed under "multiply `g` by a
//! variable when `deg g < c`", in the Boolean ring. Different mechanics:
//!
//! - **Columns** are monomials of degree `≤ c` ordered by degree, then by
//!   colexicographic rank, so a monomial's column is computed from its bits
//!   (combinatorial number system) instead of looked up in a hash map. Any
//!   degree-compatible order gives the same closure.
//! - **Rounds.** Each round reduces its candidate rows against the current
//!   echelon basis in parallel (read-only), chunk by chunk, then eliminates
//!   among the nonzero residues in parallel and inserts them. The next round's candidates are the variable
//!   multiples of the rows inserted with degree `< c`. The fixpoint is the
//!   same as the engine's work queue.
//! - **Rows** are stored only up to their pivot word: a row's leading term is
//!   its highest column, so nothing above it is ever set.
//! - Stops early when `1 ∈ V_{F,c}` (unsatisfiable systems) or when the
//!   closure reaches a given target dimension.
//!
//! Subcommands:
//! - `ladder <records.jsonl> --m M --cmax C`: committed `dreg_ladder` draws
//!   (as `hky_last_fall_degree ladder`), for validation;
//! - `newcells --cells m:n:ell,... --seed S --keep K --cmax C`: fresh draws
//!   exactly as `dreg_ladder` makes them (as `hky_grid newcells`).
use rayon::prelude::*;
use std::time::Instant;

struct Space {
    c: u32,
    /// binom[n][k] for n ≤ nv, k ≤ c + 1
    binom: Vec<Vec<u64>>,
    /// first column of each degree
    off: Vec<usize>,
    monos: Vec<u128>,
    words: usize,
}

impl Space {
    fn new(nv: u32, c: u32) -> Space {
        let mut binom = vec![vec![0u64; c as usize + 2]; nv as usize + 1];
        for n in 0..=nv as usize {
            binom[n][0] = 1;
            for k in 1..=c as usize + 1 {
                binom[n][k] = if n == 0 {
                    0
                } else {
                    binom[n - 1][k - 1] + binom[n - 1][k]
                };
            }
        }
        let mut off = vec![0usize; c as usize + 2];
        for d in 0..=c as usize {
            off[d + 1] = off[d] + binom[nv as usize][d] as usize;
        }
        let total = off[c as usize + 1];
        let mut monos = vec![0u128; total];
        // enumerate each degree in colex order (Gosper), which is rank order
        for d in 0..=c {
            let mut m: u128 = if d == 0 { 0 } else { (1u128 << d) - 1 };
            let base = off[d as usize];
            for slot in monos[base..off[d as usize + 1]].iter_mut() {
                *slot = m;
                if d == 0 {
                    break;
                }
                let t = m | (m - 1);
                m = (t + 1) | (((!t & (t + 1)) - 1) >> (m.trailing_zeros() + 1));
            }
        }
        let words = total.div_ceil(64);
        Space {
            c,
            binom,
            off,
            monos,
            words,
        }
    }
    #[inline]
    fn col(&self, m: u128) -> usize {
        let d = m.count_ones() as usize;
        let (mut r, mut x, mut j) = (0u64, m, 1usize);
        while x != 0 {
            let b = x.trailing_zeros() as usize;
            r += self.binom[b][j];
            x &= x - 1;
            j += 1;
        }
        self.off[d] + r as usize
    }
    fn deg_of_col(&self, col: usize) -> u32 {
        self.monos[col].count_ones()
    }
    fn row(&self, poly: &[u128]) -> Vec<u64> {
        let mut r = vec![0u64; self.words];
        for &m in poly {
            let i = self.col(m);
            r[i / 64] ^= 1 << (i % 64);
        }
        r
    }
    /// `x_i · row`, or `None` if a product leaves degree `≤ c`.
    fn times_var(&self, row: &[u64], i: u32) -> Option<Vec<u64>> {
        let mut out = vec![0u64; self.words];
        let bit = 1u128 << i;
        for (w, &word) in row.iter().enumerate() {
            let mut b = word;
            while b != 0 {
                let m = self.monos[w * 64 + b.trailing_zeros() as usize] | bit;
                b &= b - 1;
                if m.count_ones() > self.c {
                    return None;
                }
                let j = self.col(m);
                out[j / 64] ^= 1 << (j % 64);
            }
        }
        Some(out)
    }
}

fn top(v: &[u64]) -> Option<usize> {
    v.iter()
        .enumerate()
        .rev()
        .find(|(_, &w)| w != 0)
        .map(|(i, &w)| i * 64 + 63 - w.leading_zeros() as usize)
}

/// Echelon basis: `piv[col]` is the row whose leading column is `col`,
/// stored truncated to `col / 64 + 1` words.
struct Basis {
    piv: Vec<Option<Box<[u64]>>>,
    /// bit set at every pivot column
    mask: Vec<u64>,
    dim: usize,
}

impl Basis {
    /// Clear every pivot column of `v` (high to low); `None` if `v` becomes 0.
    fn reduce_full(&self, mut v: Vec<u64>) -> Option<Vec<u64>> {
        let mut w = v.len();
        while w > 0 {
            w -= 1;
            loop {
                let hit = v[w] & self.mask[w];
                if hit == 0 {
                    break;
                }
                let col = w * 64 + 63 - hit.leading_zeros() as usize;
                let row = self.piv[col].as_ref().unwrap();
                for (x, &y) in v.iter_mut().zip(row.iter()) {
                    *x ^= y;
                }
            }
        }
        v.iter().any(|&x| x != 0).then_some(v)
    }
    fn insert(&mut self, t: usize, v: Vec<u64>) {
        let mut v = v;
        v.truncate(t / 64 + 1);
        self.piv[t] = Some(v.into_boxed_slice());
        self.mask[t / 64] |= 1 << (t % 64);
        self.dim += 1;
    }

    /// Insert a batch of rows that are fully reduced against the basis:
    /// parallel Gaussian elimination among them, each new pivot cleared from
    /// every remaining row. Returns the new pivot columns.
    fn absorb(&mut self, mut rows: Vec<Vec<u64>>) -> Vec<usize> {
        let mut new = vec![];
        let mut tops: Vec<usize> = rows.par_iter().map(|r| top(r).unwrap()).collect();
        while !rows.is_empty() {
            let (i, &t) = tops.iter().enumerate().max_by_key(|(_, &t)| t).unwrap();
            let p = rows.swap_remove(i);
            tops.swap_remove(i);
            let (w, bit) = (t / 64, 1u64 << (t % 64));
            let pr = &p[..=w];
            rows.par_iter_mut()
                .zip(tops.par_iter_mut())
                .for_each(|(r, tp)| {
                    if r[w] & bit != 0 {
                        for (x, &y) in r.iter_mut().zip(pr) {
                            *x ^= y;
                        }
                        *tp = top(r).unwrap_or(usize::MAX);
                    }
                });
            // drop rows that became zero
            let mut k = 0;
            while k < rows.len() {
                if tops[k] == usize::MAX {
                    rows.swap_remove(k);
                    tops.swap_remove(k);
                } else {
                    k += 1;
                }
            }
            self.insert(t, p);
            new.push(t);
        }
        new
    }
}

/// A candidate row, materialised only when its chunk is processed: a
/// Macaulay row `m·f`, or a pivot row times a variable.
#[derive(Clone, Copy)]
enum Src {
    Mac(usize, u128),
    Mul(usize, u32),
}

/// `(plain dim, closure dim)`; the closure stops at `target` or at `1 ∈ V`
/// (then reports `min(#monomials, target)`), as the validated engine does.
///
/// Each round's candidates are processed in chunks (`HKY_CHUNK` rows, default
/// about 1 GiB of full-width rows): reduce the chunk against the basis in
/// parallel, absorb the residues, go on. A later chunk is reduced against a
/// larger basis, which changes nothing about the span or the fixpoint.
pub(super) fn closure_par(
    eqs: &[Vec<u128>],
    nv: u32,
    c: u32,
    target: usize,
) -> (usize, usize, u32) {
    let sp = Space::new(nv, c);
    let total = sp.monos.len();
    let mut basis = Basis {
        piv: vec![None; total],
        mask: vec![0; sp.words],
        dim: 0,
    };
    let chunk: usize = std::env::var("HKY_CHUNK")
        .ok()
        .map_or((1usize << 30) / (8 * sp.words), |v| v.parse().unwrap())
        .max(256);
    let bases: Vec<Option<Vec<u64>>> = eqs
        .iter()
        .map(|f| (f.iter().map(|m| m.count_ones()).max().unwrap_or(0) <= c).then(|| sp.row(f)))
        .collect();
    let mut srcs: Vec<Src> = vec![];
    for (k, f) in eqs.iter().enumerate() {
        if bases[k].is_none() {
            continue;
        }
        let df = f.iter().map(|m| m.count_ones()).max().unwrap_or(0);
        srcs.extend(
            sp.monos
                .iter()
                .filter(|m| m.count_ones() <= c - df)
                .map(|&m| Src::Mac(k, m)),
        );
    }
    let mut plain = None;
    let mut rounds = 0u32;
    let prof = std::env::var("HKY_PROF").is_ok();
    loop {
        rounds += 1;
        let t0 = Instant::now();
        let (ncand, mut nres, mut tred) = (srcs.len(), 0usize, 0f64);
        let mut new_low: Vec<usize> = vec![];
        for part in srcs.chunks(chunk) {
            let t1 = Instant::now();
            let (spr, basr, bsr) = (&sp, &basis, &bases);
            let residues: Vec<Vec<u64>> = part
                .par_iter()
                .filter_map(|&s| {
                    let v = match s {
                        Src::Mac(k, m) => {
                            let mut r = bsr[k].clone().unwrap();
                            for i in 0..nv {
                                if m >> i & 1 == 1 {
                                    r = spr.times_var(&r, i)?;
                                }
                            }
                            r
                        }
                        Src::Mul(t, i) => {
                            let mut full = basr.piv[t].as_ref().unwrap().to_vec();
                            full.resize(spr.words, 0);
                            spr.times_var(&full, i)?
                        }
                    };
                    basr.reduce_full(v)
                })
                .collect();
            tred += t1.elapsed().as_secs_f64();
            nres += residues.len();
            new_low.extend(
                basis
                    .absorb(residues)
                    .into_iter()
                    .filter(|&t| sp.deg_of_col(t) < c),
            );
            if basis.piv[0].is_some() {
                break;
            }
        }
        if prof {
            eprintln!("c={c} round {rounds}: {ncand} candidates, {nres} residues, parallel reduce {tred:.2}s, absorb {:.2}s, dim {}", t0.elapsed().as_secs_f64() - tred, basis.dim);
        }
        if plain.is_none() {
            plain = Some(basis.dim);
        }
        if basis.piv[0].is_some() {
            return (plain.unwrap(), total.min(target), rounds);
        }
        if basis.dim >= target || new_low.is_empty() {
            return (plain.unwrap(), basis.dim, rounds);
        }
        // next candidates: variable multiples of the new low-degree rows
        srcs = new_low
            .iter()
            .flat_map(|&t| (0..nv).map(move |i| Src::Mul(t, i)))
            .collect();
    }
}
