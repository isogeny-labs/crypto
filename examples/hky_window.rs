//! Restricted ("windowed") mutant closure for the Huang–Kosters–Yeo last
//! fall degree, for chained Weil-descended Semaev systems too large for the
//! dense degree-4 closure of `examples/hky_par.rs` (E11).
//!
//! **What it computes.** The same closure as `hky_par` — the degree-`c`
//! Macaulay rows `m·f`, closed under "multiply `g` by a variable when
//! `deg g < c`", in the Boolean ring — but at `c = 4` only over the columns
//! of a chosen monomial set `M = (all monomials of degree ≤ 3) ∪ M4`:
//! a generator `m·f` is admitted only if its support lies in `M`, and a
//! variable multiple `x·g` of a basis row `g` with `deg g < 4` only if its
//! support lies in `M`.
//!
//! **Soundness.** Every row the restricted closure holds is a row of the
//! dense closure `V_{F,4}` (by induction: generators are in `V_{F,4}`, and
//! `V_{F,4}` contains `x·g` for each of its elements `g` of degree `< 4`).
//! So `1` in the restricted closure certifies `1 ∈ V_{F,4}`, that is
//! `d_F ≤ 4`. Together with a full degree-3 closure that does not hold `1`
//! (computed here exactly as `hky_par` does) it certifies `d_F = 4`. If the
//! restricted closure stops without `1`, the result is **inconclusive**
//! (`d_F ≥ 4`), never `d_F ≥ 5`.
//!
//! **Policies** for `M4` (`--policies`, comma separated):
//! - `full`: every degree-4 monomial (the dense closure; a validation arm);
//! - `wK`: monomials inside the variable blocks of `K` consecutive chain
//!   links (blocks: one of `ℓ` bits per summand, one of `n` bits per
//!   intermediate point, as `build_decomposition_system` lays them out);
//! - `supp`: the degree-4 monomials of the degree-4 Macaulay generators
//!   `x·f`;
//! - `a&b`: the intersection of two policies.
//!
//! **Mechanics.** As `hky_par` (rounds, chunked parallel reduction, rows
//! truncated at their pivot word, early stop at `1`), with monomials as
//! `u128` (up to 128 variables; build with `--features wide-vars` beyond 64,
//! since `build_decomposition_system` uses the crate's `Mask`). Degree ≤ 3
//! columns use the combinatorial number system; the degree-4 columns of a
//! restricted set are its members in increasing mask order, which is colex
//! order, so `full` reproduces `hky_par`'s column order exactly.
//!
//! Subcommands:
//! - `ladder <records.jsonl> [--m M] --policies P`: committed draws (records
//!   with `v_basis`, `x_r`, `n`, `solutions == 0`);
//! - `newcells --cells m:n:ell,... --seed S --keep K --policies P`: fresh
//!   draws exactly as `hky_par newcells` makes them (same `cell_seed` and
//!   `ladder_sample` stream; solutions are counted with
//!   `chained_solution_count_fast`, which returns the same counts as the
//!   scans it replaced).
//!
//! Options: `--budget C` skips a policy whose column count exceeds `C`;
//! `--draw D` (ladder) runs one draw only.
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::{
    chained_s3_solution_count_fast, chained_solution_count, chained_solution_count_fast,
    ladder_sample,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::time::Instant;

#[derive(Default)]
struct MaskHasher(u64);
impl Hasher for MaskHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ u64::from(b)).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }
    fn write_u128(&mut self, v: u128) {
        let x = (v as u64) ^ ((v >> 64) as u64).rotate_left(29);
        let h = x.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        self.0 = h ^ (h >> 31);
    }
}
type Fx = BuildHasherDefault<MaskHasher>;

struct Space {
    c: u32,
    /// binom[n][k] for n ≤ nv, k ≤ 5
    binom: Vec<Vec<u64>>,
    /// first column of each degree; off[c + 1] = total
    off: Vec<usize>,
    monos: Vec<u128>,
    /// restricted degree-4 columns (index within the degree-4 block);
    /// `None` means every degree-4 monomial (combinatorial rank)
    m4: Option<HashMap<u128, u32, Fx>>,
    words: usize,
}

fn binom_table(nv: u32) -> Vec<Vec<u64>> {
    let mut binom = vec![vec![0u64; 6]; nv as usize + 1];
    for n in 0..=nv as usize {
        binom[n][0] = 1;
        for k in 1..6 {
            binom[n][k] = if n == 0 {
                0
            } else {
                binom[n - 1][k - 1] + binom[n - 1][k]
            };
        }
    }
    binom
}

/// Every degree-`d` monomial in `nv` variables in colex (= increasing) order.
fn all_of_degree(nv: u32, d: u32, count: usize) -> Vec<u128> {
    let mut out = Vec::with_capacity(count);
    if d == 0 {
        out.push(0);
        return out;
    }
    let mut m: u128 = (1u128 << d) - 1;
    for _ in 0..count {
        out.push(m);
        let t = m | (m - 1);
        m = (t + 1) | (((!t & (t + 1)) - 1) >> (m.trailing_zeros() + 1));
    }
    let _ = nv;
    out
}

impl Space {
    /// Columns of degree `≤ c`; at `c = 4`, `m4 = Some(sorted set)` restricts
    /// the degree-4 block.
    fn new(nv: u32, c: u32, m4: Option<Vec<u128>>) -> Space {
        assert!(c <= 4 && nv <= 128);
        let binom = binom_table(nv);
        let mut off = vec![0usize; c as usize + 2];
        for d in 0..=c as usize {
            let cnt = if d == 4 {
                m4.as_ref()
                    .map_or(binom[nv as usize][4] as usize, |s| s.len())
            } else {
                binom[nv as usize][d] as usize
            };
            off[d + 1] = off[d] + cnt;
        }
        let total = off[c as usize + 1];
        let mut monos = Vec::with_capacity(total);
        for d in 0..=c {
            if d == 4 {
                if let Some(s) = &m4 {
                    monos.extend_from_slice(s);
                    continue;
                }
            }
            monos.extend(all_of_degree(nv, d, off[d as usize + 1] - off[d as usize]));
        }
        debug_assert_eq!(monos.len(), total);
        let map = m4.map(|s| {
            let mut h: HashMap<u128, u32, Fx> =
                HashMap::with_capacity_and_hasher(s.len(), Fx::default());
            for (i, &m) in s.iter().enumerate() {
                h.insert(m, i as u32);
            }
            h
        });
        Space {
            c,
            binom,
            off,
            monos,
            m4: map,
            words: total.div_ceil(64),
        }
    }
    fn total(&self) -> usize {
        self.monos.len()
    }
    #[inline]
    fn col(&self, m: u128) -> Option<usize> {
        let d = m.count_ones();
        if d > self.c {
            return None;
        }
        if d == 4 {
            if let Some(h) = &self.m4 {
                return h.get(&m).map(|&i| self.off[4] + i as usize);
            }
        }
        let (mut r, mut x, mut j) = (0u64, m, 1usize);
        while x != 0 {
            let b = x.trailing_zeros() as usize;
            r += self.binom[b][j];
            x &= x - 1;
            j += 1;
        }
        Some(self.off[d as usize] + r as usize)
    }
    fn deg_of_col(&self, col: usize) -> u32 {
        self.monos[col].count_ones()
    }
    fn row(&self, poly: &[u128]) -> Option<Vec<u64>> {
        let mut r = vec![0u64; self.words];
        for &m in poly {
            let i = self.col(m)?;
            r[i / 64] ^= 1 << (i % 64);
        }
        Some(r)
    }
    /// `x_i · row`, or `None` if a product leaves the column set (it cannot
    /// cancel: see the module notes in ENGINE.md).
    fn times_var(&self, row: &[u64], i: u32) -> Option<Vec<u64>> {
        let mut out = vec![0u64; self.words];
        let bit = 1u128 << i;
        for (w, &word) in row.iter().enumerate() {
            let mut b = word;
            while b != 0 {
                let m = self.monos[w * 64 + b.trailing_zeros() as usize] | bit;
                b &= b - 1;
                let j = self.col(m)?;
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

/// Echelon basis, as in `hky_par`.
struct Basis {
    piv: Vec<Option<Box<[u64]>>>,
    mask: Vec<u64>,
    dim: usize,
}

impl Basis {
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
    /// (stored words, set bits) of the basis: its memory and its fill-in.
    fn footprint(&self) -> (usize, u64) {
        self.piv
            .par_iter()
            .flatten()
            .map(|r| {
                (
                    r.len(),
                    r.iter().map(|w| u64::from(w.count_ones())).sum::<u64>(),
                )
            })
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
    }
}

#[derive(Clone, Copy)]
enum Src {
    Mac(usize, u128),
    Mul(usize, u32),
}

struct Closed {
    dim: usize,
    refuted: bool,
    rounds: u32,
    /// generators admitted / skipped (support outside the column set)
    gens: (usize, usize),
    /// variable multiples tried / inadmissible
    muls: (u64, u64),
    /// exact window passes run, products tried in them
    exact: (u32, u64),
    words: usize,
    ones: u64,
}

/// Restricted closure over `sp`'s columns; stops at `1`.
fn closure(eqs: &[Vec<u128>], nv: u32, sp: &Space, windows: Option<&[u128]>) -> Closed {
    let c = sp.c;
    let total = sp.total();
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
        .map(|f| {
            (f.iter().map(|m| m.count_ones()).max().unwrap_or(0) <= c)
                .then(|| sp.row(f))
                .flatten()
        })
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
    let mut rounds = 0u32;
    let mut gens = (0usize, 0usize);
    let mut muls = (0u64, 0u64);
    let prof = std::env::var("HKY_PROF").is_ok();
    let mut first = true;
    let mut exact = (0u32, 0u64);
    let c3 = sp.off[3];
    let mut states: Vec<WinState> = match windows {
        None => vec![],
        // per-variable exact pass: for `x`, the low columns `t` with
        // `x·t` inside the column set (degree ≤ 2, `x ∈ t`, or `t ∪ x ∈ M4`)
        Some([]) => (0..nv)
            .map(|x| {
                let bit = 1u128 << x;
                win_state(sp, bit, |col| {
                    col < c3 || sp.monos[col] & bit != 0 || sp.col(sp.monos[col] | bit).is_some()
                })
            })
            .collect(),
        // per-window exact pass: the cubics inside the window
        Some(ws) => ws
            .iter()
            .map(|&u| win_state(sp, u, |col| col < c3 || sp.monos[col] & !u == 0))
            .collect(),
    };
    loop {
        rounds += 1;
        let t0 = Instant::now();
        let (ncand, mut nres) = (srcs.len(), 0usize);
        let mut inadmissible = 0usize;
        let mut new_low: Vec<usize> = vec![];
        for part in srcs.chunks(chunk) {
            let (spr, basr, bsr) = (sp, &basis, &bases);
            let built: Vec<Option<Vec<u64>>> = part
                .par_iter()
                .map(|&s| {
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
                    Some(v)
                })
                .collect();
            inadmissible += built.iter().filter(|b| b.is_none()).count();
            let residues: Vec<Vec<u64>> = built
                .into_par_iter()
                .flatten()
                .filter_map(|v| basr.reduce_full(v))
                .collect();
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
        if first {
            gens = (ncand - inadmissible, inadmissible);
            first = false;
        } else {
            muls.0 += ncand as u64;
            muls.1 += inadmissible as u64;
        }
        if prof {
            eprintln!(
                "c={c} round {rounds}: {ncand} candidates, {inadmissible} inadmissible, {nres} residues, {:.2}s, dim {}",
                t0.elapsed().as_secs_f64(),
                basis.dim
            );
        }
        let mut refuted = basis.piv[0].is_some();
        if !refuted && new_low.is_empty() && windows.is_some() {
            // Exact pass: per window, the low rows whose cubic part is
            // local, multiplied by the window's variables.
            let t0 = Instant::now();
            let (got, tried) = exact_pass(&mut basis, sp, &mut states, chunk, nv);
            exact.0 += 1;
            exact.1 += tried;
            new_low = got.into_iter().filter(|&t| sp.deg_of_col(t) < c).collect();
            refuted = basis.piv[0].is_some();
            if prof {
                eprintln!(
                    "exact pass: {tried} products, {} new low pivots, {:.2}s, dim {}",
                    new_low.len(),
                    t0.elapsed().as_secs_f64(),
                    basis.dim
                );
            }
        }
        if refuted || new_low.is_empty() {
            let (words, ones) = basis.footprint();
            return Closed {
                dim: if refuted { total } else { basis.dim },
                refuted,
                rounds,
                gens,
                muls,
                exact,
                words,
                ones,
            };
        }
        srcs = new_low
            .iter()
            .flat_map(|&t| (0..nv).map(move |i| Src::Mul(t, i)))
            .collect();
    }
}

/// Per-window state of the exact pass: the low-column permutation that puts
/// the window's local cubics below the non-local ones, and `done`, an echelon
/// basis (in permuted coordinates) of local low rows whose products by the
/// window's variables are already in the closure.
struct WinState {
    u: u128,
    perm: Vec<usize>,
    inv: Vec<usize>,
    boundary: usize,
    done: Basis,
}

/// `u` is the mask of variables the pass multiplies by; `local(col)` says
/// whether a low column may appear in a row multiplied by them.
fn win_state(sp: &Space, u: u128, local: impl Fn(usize) -> bool) -> WinState {
    let lowc = sp.off[4];
    let mut perm = vec![0usize; lowc];
    let mut k = 0;
    for (col, p) in perm.iter_mut().enumerate() {
        if local(col) {
            *p = k;
            k += 1;
        }
    }
    let boundary = k;
    for (col, p) in perm.iter_mut().enumerate() {
        if !local(col) {
            *p = k;
            k += 1;
        }
    }
    let mut inv = vec![0usize; lowc];
    for (col, &p) in perm.iter().enumerate() {
        inv[p] = col;
    }
    let lw = lowc.div_ceil(64);
    WinState {
        u,
        perm,
        inv,
        boundary,
        done: Basis {
            piv: vec![None; lowc],
            mask: vec![0; lw],
            dim: 0,
        },
    }
}

fn remap(r: &[u64], map: &[usize], words: usize) -> Vec<u64> {
    let mut v = vec![0u64; words];
    for (w, &word) in r.iter().enumerate() {
        let mut b = word;
        while b != 0 {
            let p = map[w * 64 + b.trailing_zeros() as usize];
            b &= b - 1;
            v[p / 64] |= 1 << (p % 64);
        }
    }
    v
}

/// Exact window pass. For each window `U`: a basis of the low rows (degree
/// `< 4`) of the current closure whose degree-3 part lies inside `U` (the
/// intersection of the closure's low part with a coordinate subspace),
/// found by echelonising the low rows with the non-local cubic columns
/// ordered highest. Its products `x·g`, `x ∈ U`, have support in `M`
/// whenever `M4 ⊇ C(U, 4)`, so adding them is sound. Only a complement of
/// `done` is multiplied: rows of `done` (local basis rows, which the round
/// loop already multiplied by every variable, and earlier passes' rows) have
/// their products in the closure already. Returns (new pivots, products).
fn exact_pass(
    basis: &mut Basis,
    sp: &Space,
    states: &mut [WinState],
    chunk: usize,
    nv: u32,
) -> (Vec<usize>, u64) {
    let lowc = sp.off[4];
    let lw = lowc.div_ceil(64);
    let c3 = sp.off[3];
    let cubic_rows: Vec<usize> = (c3..lowc).filter(|&t| basis.piv[t].is_some()).collect();
    let mut new = vec![];
    let mut tried = 0u64;
    let prof = std::env::var("HKY_PROF").is_ok();
    for st in states.iter_mut() {
        let t0 = Instant::now();
        let u = st.u;
        let permuted: Vec<Vec<u64>> = cubic_rows
            .par_iter()
            .map(|&t| remap(basis.piv[t].as_ref().unwrap(), &st.perm, lw))
            .collect();
        // rows already local are done (the round loop multiplied them)
        let (already, mixed): (Vec<Vec<u64>>, Vec<Vec<u64>>) = permuted
            .into_iter()
            .partition(|r| top(r).unwrap() < st.boundary);
        let fresh: Vec<Vec<u64>> = already
            .into_par_iter()
            .filter_map(|r| st.done.reduce_full(r))
            .collect();
        st.done.absorb(fresh);
        // the local part of the span of the mixed rows, modulo `done`
        let mut ech = Basis {
            piv: vec![None; lowc],
            mask: vec![0; lw],
            dim: 0,
        };
        ech.absorb(mixed);
        let local: Vec<Vec<u64>> = (0..st.boundary)
            .filter_map(|p| {
                ech.piv[p].as_ref().map(|r| {
                    let mut v = r.to_vec();
                    v.resize(lw, 0);
                    v
                })
            })
            .collect();
        drop(ech);
        let local: Vec<Vec<u64>> = local
            .into_par_iter()
            .filter_map(|r| st.done.reduce_full(r))
            .collect();
        let pivs = st.done.absorb(local);
        let comp: Vec<Vec<u64>> = pivs
            .iter()
            .map(|&p| {
                let mut r = st.done.piv[p].as_ref().unwrap().to_vec();
                r.resize(lw, 0);
                let mut v = remap(&r, &st.inv, lw);
                v.resize(sp.words, 0);
                v
            })
            .collect();
        let t1 = t0.elapsed().as_secs_f64();
        let vars: Vec<u32> = (0..nv).filter(|&i| u >> i & 1 == 1).collect();
        let pairs: Vec<(usize, u32)> = (0..comp.len())
            .flat_map(|r| vars.iter().map(move |&x| (r, x)))
            .collect();
        let mut stop = false;
        for part in pairs.chunks(chunk) {
            tried += part.len() as u64;
            let basr = &*basis;
            let residues: Vec<Vec<u64>> = part
                .par_iter()
                .filter_map(|&(r, x)| sp.times_var(&comp[r], x))
                .filter_map(|v| basr.reduce_full(v))
                .collect();
            new.extend(basis.absorb(residues));
            if basis.piv[0].is_some() {
                stop = true;
                break;
            }
        }
        if prof {
            eprintln!(
                "  window {u:#x}: {} cubic rows, complement {}, echelon {t1:.2}s, products {:.2}s",
                cubic_rows.len(),
                comp.len(),
                t0.elapsed().as_secs_f64() - t1
            );
        }
        if stop {
            break;
        }
    }
    (new, tried)
}

fn system(m: usize, n: u32, basis: &[F2mElement], x_r: &F2mElement) -> (Vec<Vec<u128>>, u32) {
    let irr = find_irreducible_sparse(n).unwrap();
    let st = FieldStructure::new(n, &irr);
    let sys =
        build_decomposition_system(basis, x_r, &F2mElement::one(n), m, &st).expect("system builds");
    #[allow(clippy::useless_conversion)]
    let eqs = sys
        .equations
        .iter()
        .map(|p| p.terms.iter().map(|t| u128::from(t.mask)).collect())
        .collect();
    (eqs, sys.n_vars as u32)
}

/// Variable masks of the chain's links (blocks as `build_decomposition_system`).
fn span_mask(lo: usize, len: usize) -> u128 {
    if len == 0 {
        0
    } else {
        (((1u128 << (len - 1)) << 1).wrapping_sub(1)) << lo
    }
}

/// Variable masks of the blocks: `m` summand blocks, then `m − 2`
/// intermediate blocks.
fn block_masks(m: usize, n: u32, ell: usize) -> Vec<u128> {
    let n = n as usize;
    (0..m)
        .map(|i| span_mask(i * ell, ell))
        .chain((0..m.saturating_sub(2)).map(|j| span_mask(m * ell + j * n, n)))
        .collect()
}

fn link_masks(m: usize, n: u32, ell: usize) -> Vec<u128> {
    let span = |lo: usize, len: usize| -> u128 {
        if len == 0 {
            0
        } else {
            (((1u128 << (len - 1)) << 1).wrapping_sub(1)) << lo
        }
    };
    let n = n as usize;
    let x = |i: usize| span(i * ell, ell);
    let e = |j: usize| span(m * ell + j * n, n);
    if m == 2 {
        return vec![x(0) | x(1)];
    }
    let mut links = vec![x(0) | x(1) | e(0)];
    for i in 1..m - 2 {
        links.push(e(i - 1) | x(i + 1) | e(i));
    }
    links.push(e(m - 3) | x(m - 1));
    links
}

/// Variable masks of the windows of policy `wK`.
fn window_masks(policy: &str, m: usize, n: u32, ell: usize) -> Vec<u128> {
    if policy == "wp" {
        // every pair of blocks inside one link
        let blocks = block_masks(m, n, ell);
        let mut out = vec![];
        for l in link_masks(m, n, ell) {
            let inside: Vec<u128> = blocks.iter().copied().filter(|&b| b & l == b).collect();
            for i in 0..inside.len() {
                for j in i + 1..inside.len() {
                    let u = inside[i] | inside[j];
                    if !out.contains(&u) {
                        out.push(u);
                    }
                }
            }
        }
        return out;
    }
    let k: usize = policy[1..].parse().expect("wK");
    let links = link_masks(m, n, ell);
    let k = k.min(links.len());
    (0..=links.len() - k)
        .map(|j| links[j..j + k].iter().fold(0u128, |a, &b| a | b))
        .collect()
}

fn subsets4(mask: u128, out: &mut HashSet<u128, Fx>) {
    let pos: Vec<u32> = (0..128).filter(|&i| mask >> i & 1 == 1).collect();
    let k = pos.len();
    for a in 0..k {
        for b in a + 1..k {
            for c in b + 1..k {
                for d in c + 1..k {
                    out.insert(
                        (1u128 << pos[a])
                            | (1u128 << pos[b])
                            | (1u128 << pos[c])
                            | (1u128 << pos[d]),
                    );
                }
            }
        }
    }
}

/// The degree-4 column set of a policy; `None` = every degree-4 monomial.
fn policy_set(
    policy: &str,
    eqs: &[Vec<u128>],
    m: usize,
    n: u32,
    ell: usize,
) -> Option<HashSet<u128, Fx>> {
    if let Some((a, b)) = policy.split_once('&') {
        if let Some(k) = b.strip_prefix('b') {
            // block-degree filter: at most k variables in any one block
            let k: u32 = k.parse().expect("bK");
            let blocks = block_masks(m, n, ell);
            let ok = |t: &u128| blocks.iter().all(|&b| (t & b).count_ones() <= k);
            let nv = (m * ell) as u32 + (m as u32 - 2) * n;
            return Some(match policy_set(a, eqs, m, n, ell) {
                Some(s) => s.into_iter().filter(ok).collect(),
                None => {
                    let cnt = binom_table(nv)[nv as usize][4] as usize;
                    all_of_degree(nv, 4, cnt).into_iter().filter(ok).collect()
                }
            });
        }
        let sa = policy_set(a, eqs, m, n, ell);
        let sb = policy_set(b, eqs, m, n, ell);
        return match (sa, sb) {
            (None, s) | (s, None) => s,
            (Some(x), Some(y)) => Some(x.intersection(&y).copied().collect()),
        };
    }
    let mut out: HashSet<u128, Fx> = HashSet::default();
    match policy {
        "full" => return None,
        "supp" => {
            let nv = (m * ell) as u32 + (m as u32 - 2) * n;
            for f in eqs {
                for &t in f.iter().filter(|t| t.count_ones() == 3) {
                    for i in 0..nv {
                        if t >> i & 1 == 0 {
                            out.insert(t | 1u128 << i);
                        }
                    }
                }
            }
        }
        p if p.starts_with('w') => {
            for u in window_masks(p, m, n, ell) {
                subsets4(u, &mut out);
            }
        }
        _ => panic!("unknown policy {policy}"),
    }
    Some(out)
}

fn vmhwm_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("VmHWM:"))
                .and_then(|l| l.split_whitespace().nth(1).map(|v| v.parse().unwrap()))
        })
        .unwrap_or(0)
}

fn r1(x: f64) -> f64 {
    (x * 10.0).round() / 10.0
}

/// Full closures at `c = 2, 3` (as `hky_par`), then the restricted degree-4
/// closure for each policy unless degree 3 already refutes.
fn run_draw(
    eqs: &[Vec<u128>],
    nv: u32,
    m: usize,
    n: u32,
    ell: usize,
    policies: &[String],
    budget: usize,
) -> Value {
    let mut dims = serde_json::Map::new();
    let mut d_le3 = None;
    for c in 2..=3 {
        let sp = Space::new(nv, c, None);
        let t0 = Instant::now();
        let r = closure(eqs, nv, &sp, None);
        dims.insert(c.to_string(), json!({"dim": r.dim, "of": sp.total(), "rounds": r.rounds, "secs": r1(t0.elapsed().as_secs_f64())}));
        if r.refuted {
            d_le3 = Some(c);
            break;
        }
    }
    let mut win = serde_json::Map::new();
    if d_le3.is_none() {
        let low = Space::new(nv, 3, None).total();
        for p in policies {
            let t0 = Instant::now();
            let set = policy_set(p.trim_end_matches(['x', 'v']), eqs, m, n, ell);
            let deg4 = set
                .as_ref()
                .map_or(binom_table(nv)[nv as usize][4] as usize, |s| s.len());
            let cols = low + deg4;
            if cols > budget {
                win.insert(
                    p.clone(),
                    json!({"cols": cols, "deg4_cols": deg4, "over_budget": true}),
                );
                continue;
            }
            let sorted = set.map(|s| {
                let mut v: Vec<u128> = s.into_iter().collect();
                v.sort_unstable();
                v
            });
            let sp = Space::new(nv, 4, sorted);
            // exact window passes for a policy ending in `x`, over the
            // windows of its `wK` component; per-variable passes for `v`
            let wins = if p.ends_with('v') {
                Some(vec![])
            } else {
                p.strip_suffix('x').and_then(|q| {
                    q.split('&')
                        .find(|c| c.starts_with('w'))
                        .map(|w| window_masks(w, m, n, ell))
                })
            };
            let r = closure(eqs, nv, &sp, wins.as_deref());
            win.insert(
                p.clone(),
                json!({"cols": cols, "deg4_cols": deg4, "certified": r.refuted, "dim": r.dim,
                "rounds": r.rounds, "gens_admitted": r.gens.0, "gens_skipped": r.gens.1,
                "muls_tried": r.muls.0, "muls_inadmissible": r.muls.1,
                "exact_passes": r.exact.0, "exact_products": r.exact.1,
                "basis_bytes": r.words * 8, "basis_ones": r.ones,
                "secs": r1(t0.elapsed().as_secs_f64()), "vmhwm_kb": vmhwm_kb()}),
            );
        }
    }
    json!({"per_degree": dims, "d_f_le3": d_le3, "window": win,
           "d_f_window": if d_le3.is_some() { json!(d_le3) } else if win.values().any(|w| w["certified"] == json!(true)) { json!(4) } else { Value::Null },
           "vmhwm_kb": vmhwm_kb()})
}

fn cell_seed(seed: u64, n: u32, ell: usize, m: usize) -> u64 {
    let base = seed ^ (u64::from(n) << 40) ^ ((ell as u64) << 32);
    if m == 3 {
        base
    } else {
        base ^ ((m as u64) << 48)
    }
}

fn word(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
    };
    let policies: Vec<String> = get("--policies")
        .unwrap_or_else(|| "w1".into())
        .split(',')
        .map(String::from)
        .collect();
    let budget: usize = get("--budget").map_or(usize::MAX, |v| v.parse().unwrap());
    let only_draw: Option<u64> = get("--draw").map(|v| v.parse().unwrap());
    match args.get(1).map(String::as_str) {
        Some("ladder") => {
            let mdef: usize = get("--m").map_or(3, |v| v.parse().unwrap());
            for line in std::fs::read_to_string(&args[2]).unwrap().lines().filter(|l| l.starts_with('{')) {
                let rec: Value = serde_json::from_str(line).unwrap();
                if rec.get("draw").is_none() || rec["solutions"].as_u64() != Some(0) || rec.get("v_basis").is_none() {
                    continue;
                }
                if only_draw.is_some() && rec["draw"].as_u64() != only_draw {
                    continue;
                }
                let m = rec["m"].as_u64().map_or(mdef, |v| v as usize);
                let n = rec["n"].as_u64().unwrap() as u32;
                let el = |w: u64| F2mElement::from_biguint(&num_bigint::BigUint::from(w), n);
                let basis: Vec<F2mElement> =
                    rec["v_basis"].as_array().unwrap().iter().map(|w| el(w.as_u64().unwrap())).collect();
                let (eqs, nv) = system(m, n, &basis, &el(rec["x_r"].as_u64().unwrap()));
                let t0 = Instant::now();
                let mut out = run_draw(&eqs, nv, m, n, basis.len(), &policies, budget);
                let o = out.as_object_mut().unwrap();
                o.insert("schema".into(), json!("hky_window_ladder/v1"));
                for k in ["cell", "draw", "seed", "v_basis", "x_r"] {
                    o.insert(k.into(), rec.get(k).cloned().unwrap_or(Value::Null));
                }
                o.insert("m".into(), json!(m));
                o.insert("n".into(), json!(n));
                o.insert("ell".into(), json!(basis.len()));
                o.insert("n_vars".into(), json!(nv));
                o.insert("n_eqs".into(), json!(eqs.len()));
                o.insert("source".into(), json!(args[2]));
                o.insert("secs".into(), json!(r1(t0.elapsed().as_secs_f64())));
                o.insert("threads".into(), json!(rayon::current_num_threads()));
                println!("{out}");
            }
        }
        Some("count") => {
            // Independent recount of a record's solutions with the
            // root-walking counter `chained_solution_count` (no
            // Artin–Schreier shortcut), against the fast counter.
            for line in std::fs::read_to_string(&args[2]).unwrap().lines().filter(|l| l.starts_with('{')) {
                let rec: Value = serde_json::from_str(line).unwrap();
                let m = rec["m"].as_u64().unwrap() as usize;
                let n = rec["n"].as_u64().unwrap() as u32;
                let el = |w: u64| F2mElement::from_biguint(&num_bigint::BigUint::from(w), n);
                let basis: Vec<F2mElement> =
                    rec["v_basis"].as_array().unwrap().iter().map(|w| el(w.as_u64().unwrap())).collect();
                let x_r = el(rec["x_r"].as_u64().unwrap());
                let irr = find_irreducible_sparse(n).unwrap();
                let b1 = F2mElement::one(n);
                let t0 = Instant::now();
                let slow = chained_solution_count(m, &basis, &x_r, &b1, &irr);
                let ts = t0.elapsed().as_secs_f64();
                let fast = (m == 3).then(|| chained_s3_solution_count_fast(&basis, &x_r, &b1, &irr));
                println!("{}", json!({"schema": "hky_window_count/v1", "m": m, "n": n, "ell": basis.len(), "draw": rec["draw"],
                    "chained_solution_count": slow, "chained_s3_solution_count_fast": fast, "slow_secs": r1(ts)}));
            }
        }
        Some("newcells") => {
            let seed: u64 = get("--seed").unwrap().parse().unwrap();
            let keep: usize = get("--keep").map_or(4, |v| v.parse().unwrap());
            for cell in get("--cells").unwrap().split(',') {
                let p: Vec<usize> = cell.split(':').map(|x| x.parse().unwrap()).collect();
                let (m, n, ell) = (p[0], p[1] as u32, p[2]);
                let mut rng = StdRng::seed_from_u64(cell_seed(seed, n, ell, m));
                let irr = find_irreducible_sparse(n).unwrap();
                let (mut kept, mut drawn) = (0, 0);
                while kept < keep && drawn < 1000 {
                    let (basis, x_r) = ladder_sample(n, ell, &mut rng);
                    let draw = drawn;
                    drawn += 1;
                    let b1 = F2mElement::one(n);
                    let tc = Instant::now();
                    // Exact (`fast_chained_count_matches_the_reference`); the
                    // scan behind `chained_solution_count` is infeasible at
                    // n ≥ 20 (Amendment 11).
                    let sols = chained_solution_count_fast(m, &basis, &x_r, &b1, &irr);
                    let count_secs = tc.elapsed().as_secs_f64();
                    if sols > 0 {
                        eprintln!("{cell} draw {draw}: {sols} solutions ({count_secs:.1}s), skipped");
                        continue;
                    }
                    kept += 1;
                    let (eqs, nv) = system(m, n, &basis, &x_r);
                    let t0 = Instant::now();
                    let mut out = run_draw(&eqs, nv, m, n, ell, &policies, budget);
                    let o = out.as_object_mut().unwrap();
                    o.insert("schema".into(), json!("hky_window_newcell/v1"));
                    o.insert("m".into(), json!(m));
                    o.insert("n".into(), json!(n));
                    o.insert("ell".into(), json!(ell));
                    o.insert("draw".into(), json!(draw));
                    o.insert("seed".into(), json!(seed));
                    o.insert("v_basis".into(), json!(basis.iter().map(word).collect::<Vec<_>>()));
                    o.insert("x_r".into(), json!(word(&x_r)));
                    o.insert("n_vars".into(), json!(nv));
                    o.insert("n_eqs".into(), json!(eqs.len()));
                    o.insert("solutions".into(), json!(0));
                    o.insert("counter".into(), json!("chained_solution_count_fast"));
                    o.insert("count_secs".into(), json!(r1(count_secs)));
                    o.insert("secs".into(), json!(r1(t0.elapsed().as_secs_f64())));
                    o.insert("threads".into(), json!(rayon::current_num_threads()));
                    println!("{out}");
                }
            }
        }
        _ => eprintln!("usage: hky_window ladder RECORDS [--m M] --policies P [--budget C] [--draw D] | newcells --cells m:n:ell,... --seed S [--keep K] --policies P"),
    }
}
