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
use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::koblitz_bench::{
    chained_s3_solution_count, chained_solution_count, ladder_sample,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rayon::prelude::*;
use serde_json::{json, Value};
use std::time::Instant;

struct Space {
    #[allow(dead_code)]
    nv: u32,
    c: u32,
    /// binom[n][k] for n ≤ nv, k ≤ c + 1
    binom: Vec<Vec<u64>>,
    /// first column of each degree
    off: Vec<usize>,
    monos: Vec<u64>,
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
        let mut monos = vec![0u64; total];
        // enumerate each degree in colex order (Gosper), which is rank order
        for d in 0..=c {
            let mut m: u64 = if d == 0 { 0 } else { (1u64 << d) - 1 };
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
            nv,
            c,
            binom,
            off,
            monos,
            words,
        }
    }
    #[inline]
    fn col(&self, m: u64) -> usize {
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
    fn row(&self, poly: &[u64]) -> Vec<u64> {
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
        let bit = 1u64 << i;
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
    Mac(usize, u64),
    Mul(usize, u32),
}

/// `(plain dim, closure dim)`; the closure stops at `target` or at `1 ∈ V`
/// (then reports `min(#monomials, target)`), as the validated engine does.
///
/// Each round's candidates are processed in chunks (`HKY_CHUNK` rows, default
/// about 1 GiB of full-width rows): reduce the chunk against the basis in
/// parallel, absorb the residues, go on. A later chunk is reduced against a
/// larger basis, which changes nothing about the span or the fixpoint.
fn closure_par(eqs: &[Vec<u64>], nv: u32, c: u32, target: usize) -> (usize, usize, u32) {
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

fn system(m: usize, n: u32, basis: &[F2mElement], x_r: &F2mElement) -> (Vec<Vec<u64>>, u32) {
    let irr = find_irreducible_sparse(n).unwrap();
    let st = FieldStructure::new(n, &irr);
    let sys =
        build_decomposition_system(basis, x_r, &F2mElement::one(n), m, &st).expect("system builds");
    let eqs = sys
        .equations
        .iter()
        .map(|p| p.terms.iter().map(|t| t.mask).collect())
        .collect();
    (eqs, sys.n_vars as u32)
}

fn refute(eqs: &[Vec<u64>], nv: u32, cmax: u32) -> (Option<u32>, serde_json::Map<String, Value>) {
    let mut dims = serde_json::Map::new();
    for c in 2..=cmax {
        let total = Space::new(nv, c).monos.len();
        let t0 = Instant::now();
        let (_, closed, rounds) = closure_par(eqs, nv, c, total);
        dims.insert(c.to_string(), json!({"dim": closed, "of": total, "rounds": rounds, "secs": (t0.elapsed().as_secs_f64() * 10.0).round() / 10.0}));
        if closed == total {
            return (Some(c), dims);
        }
    }
    (None, dims)
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
    let cmax: u32 = get("--cmax").map_or(4, |v| v.parse().unwrap());
    match args.get(1).map(String::as_str) {
        Some("ladder") => {
            let m: usize = get("--m").map_or(3, |v| v.parse().unwrap());
            for line in std::fs::read_to_string(&args[2]).unwrap().lines().filter(|l| l.starts_with('{')) {
                let rec: Value = serde_json::from_str(line).unwrap();
                if rec.get("draw").is_none() || rec["solutions"].as_u64() != Some(0) {
                    continue;
                }
                let n = rec["n"].as_u64().unwrap() as u32;
                let el = |w: u64| F2mElement::from_biguint(&num_bigint::BigUint::from(w), n);
                let basis: Vec<F2mElement> =
                    rec["v_basis"].as_array().unwrap().iter().map(|w| el(w.as_u64().unwrap())).collect();
                let (eqs, nv) = system(m, n, &basis, &el(rec["x_r"].as_u64().unwrap()));
                let t0 = Instant::now();
                let (d_f, dims) = refute(&eqs, nv, cmax);
                println!("{}", json!({"schema": "hky_par_ladder/v1", "m": m, "cell": rec["cell"], "draw": rec["draw"], "n_vars": nv,
                    "d_f": d_f, "d_f_at_least": if d_f.is_none() { Some(cmax + 1) } else { None }, "per_degree": dims,
                    "secs": (t0.elapsed().as_secs_f64() * 10.0).round() / 10.0, "threads": rayon::current_num_threads()}));
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
                    let sols = if m == 3 {
                        chained_s3_solution_count(&basis, &x_r, &b1, &irr)
                    } else {
                        chained_solution_count(m, &basis, &x_r, &b1, &irr)
                    };
                    if sols > 0 {
                        continue;
                    }
                    kept += 1;
                    let (eqs, nv) = system(m, n, &basis, &x_r);
                    let t0 = Instant::now();
                    let (d_f, dims) = refute(&eqs, nv, cmax);
                    println!("{}", json!({"schema": "hky_par_newcell/v1", "m": m, "n": n, "ell": ell, "draw": draw, "seed": seed,
                        "v_basis": basis.iter().map(word).collect::<Vec<_>>(), "x_r": word(&x_r), "n_vars": nv, "n_eqs": eqs.len(),
                        "solutions": 0, "d_f": d_f, "d_f_at_least": if d_f.is_none() { Some(cmax + 1) } else { None },
                        "per_degree": dims, "secs": (t0.elapsed().as_secs_f64() * 10.0).round() / 10.0,
                        "threads": rayon::current_num_threads()}));
                }
            }
        }
        _ => eprintln!("usage: hky_par ladder RECORDS --m M [--cmax C] | newcells --cells m:n:ell,... --seed S [--keep K] [--cmax C]"),
    }
}
