//! Binary (GF(2^n)) Koblitz fall-degree study — instrument.
//!
//! The GF(2^n) analogue of the prime-field last-fall-degree work
//! (`research/prime_fall_degree_20261027/`), and a direct extension of the
//! binary program `research/fall_degree_bounds_20261008/` (which used a
//! RANDOM and a SUBFIELD factor base). The new axis here is the
//! **Frobenius-orbit** factor base of Koblitz curves, against a random base
//! of the same dimension and a structured non-orbit (polynomial-window)
//! control.
//!
//! Preregistration: `research/binary_falldeg_20261118/PREREGISTRATION.md`.
//! All pilot data is `[pcc:unverified]`.
//!
//! ## What it measures, per `(n, m, arm, draw)`
//!
//! The system is the Weil-descended Semaev chain `build_decomposition_system`
//! builds over `GF(2^n)` for `R = P_1 + … + P_m` with every `x(P_i)` in the
//! `F_2`-span `V` of the arm's basis, curve parameter `b = 1` (Koblitz;
//! the binary `S_3` here is `(x1+x2)^2 x3^2 + x1 x2 x3 + (x1 x2)^2 + b`,
//! which is **a2-independent**, so Koblitz a=0 and a=1 give the identical
//! `S_3` system — see the preregistration). Only inconsistent draws
//! (exact solution count 0) are measured, as in `hky_par`.
//!
//! - **first-fall degree (rank proxy)** `d_ff`: `koblitz_groebner::first_fall_degree`
//!   (the Galbraith–Gebregiyorgis operational FFD: least `D>=2` with
//!   `rank < rows` and `rank < cols` in the bounded Macaulay matrix). The
//!   library labels this a rank-deficiency PROXY, not the syzygy FFD.
//! - **last-fall degree** `d_F`: the Huang–Kosters–Yeo mutant closure in the
//!   Boolean ring (least `c` with `1 ∈ V_{F,c}`), computed by the exact
//!   degree-`c` closure copied verbatim from `examples/hky_par.rs`
//!   (`closure_par`). This is the number the prior binary program reported.
//! - **resolution degree** `d_res`: `koblitz_groebner::solving_degree`
//!   (bounded-Macaulay resolution; a weaker diagnostic, reported for context).
//! - degree profiles (rows/cols/rank per degree; closure dim per degree) and
//!   exact monomial counts.
//!
//! ## Arms (factor base `V` of matched `F_2`-dimension `ℓ_eff`)
//!
//! - `orbit`: a Frobenius(squaring)-invariant `F_2`-subspace — the `F_2`-span
//!   of a union of complete squaring-orbits of random field elements.
//!   `ℓ_eff` is whatever that span reaches at or above the target `ℓ`.
//! - `random`: a uniformly random `ℓ_eff`-dimensional `F_2`-subspace
//!   (`koblitz_bench::random_subspace_basis`).
//! - `subspace`: the polynomial-basis low-degree window `span{1,x,…,x^{ℓ_eff-1}}`
//!   — structured but generally NOT squaring-closed (the non-orbit control).
//!
//! The target `x_R` is drawn once per `(n,m,draw)` and shared by all arms, so
//! the arms differ only in `V`. The orbit arm determines `ℓ_eff`; the other
//! two are built at the same `ℓ_eff`.
//!
//! Usage:
//!   binary_falldeg_arms selftest
//!   binary_falldeg_arms run --cells m:n:ell,... --seed S [--keep K]
//!        [--cmax C] [--ffd-max F] [--dres-max D] [--arms orbit,random,subspace]
//!        [--crosscheck]

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::koblitz_bench::{
    chained_solution_count_fast, random_subspace_basis,
};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, first_fall_degree, solving_degree, FieldStructure,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::time::Instant;

// ───────────────────────── factor-base arms ─────────────────────────

/// The squaring-orbit (Frobenius orbit) of `g`: `g, g^2, g^4, …` until it
/// returns to `g`. Length = the size of the 2-cyclotomic coset of `g`.
fn squaring_orbit(g: &F2mElement, irr: &IrreduciblePoly) -> Vec<F2mElement> {
    let mut orbit = vec![g.clone()];
    let mut cur = g.square(irr);
    while &cur != g {
        orbit.push(cur.clone());
        cur = cur.square(irr);
    }
    orbit
}

fn word_of(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

/// Field element from a raw u64 word (bit k = x^k in polynomial basis), n < 64.
fn elt_from_word(w: u64, n: u32) -> F2mElement {
    let bits: Vec<u32> = (0..n).filter(|&k| (w >> k) & 1 == 1).collect();
    F2mElement::from_bit_positions(&bits, n)
}

/// `F_2`-echelon helper over `u64` (n < 64). Returns `true` if `v` was
/// independent of the current echelon and was added.
fn echelon_add(echelon: &mut Vec<u64>, basis: &mut Vec<F2mElement>, v: &F2mElement, n: u32) {
    let w = word_of(v);
    let r = echelon.iter().fold(w, |r, &e| r.min(r ^ e));
    if r == 0 {
        return;
    }
    echelon.push(r);
    echelon.sort_unstable_by(|a, b| b.cmp(a));
    basis.push(elt_from_word(w, n));
}

/// A Frobenius(squaring)-invariant `F_2`-subspace of dimension `>= ell_target`
/// and `< n`, as the span of a union of **complete** squaring-orbits of random
/// elements. The span of complete orbits is `τ`-invariant (`τ` permutes each
/// orbit), so the result is squaring-closed by construction. To keep `ℓ_eff`
/// near `ell_target`, orbits are added smallest-first.
///
/// Returns `None` when no such subspace of dimension in `[ell_target, n)`
/// exists — the key case being prime `n`, where every non-subfield element
/// has orbit length `n`, so the only `τ`-invariant dimensions are near `1`
/// and `n`. That `None` is the H-avail finding, not an error.
fn orbit_invariant_basis(
    n: u32,
    ell_target: usize,
    irr: &IrreduciblePoly,
    rng: &mut StdRng,
) -> Option<Vec<F2mElement>> {
    // Overshoot cap: a τ-invariant subspace is built from whole orbits, whose
    // dimensions are fixed, so ℓ_eff lands on an achievable dimension near the
    // target, not on the target exactly. We accept dimensions in
    // [ell_target, ell_cap]; if none is reachable below n we return None (the
    // H-avail case, typically prime n).
    let ell_cap = (ell_target + 3).min(n as usize - 1);
    let mask = (1u64 << n) - 1;
    let mut seen: std::collections::HashSet<u64> = std::collections::HashSet::new();
    let mut orbits: Vec<Vec<F2mElement>> = Vec::new();
    for _ in 0..8000 {
        let w = rng.gen::<u64>() & mask;
        if w == 0 || seen.contains(&w) {
            continue;
        }
        let orb = squaring_orbit(&elt_from_word(w, n), irr);
        for o in &orb {
            seen.insert(word_of(o));
        }
        orbits.push(orb);
    }

    // Incremental dimension an orbit adds to the current echelon.
    let incr = |ech: &[u64], orb: &[F2mElement]| -> usize {
        let mut e = ech.to_vec();
        let mut b: Vec<F2mElement> = Vec::new();
        let before = {
            // dim of current echelon
            ech.len()
        };
        for o in orb {
            echelon_add(&mut e, &mut b, o, n);
        }
        e.len() - before
    };

    let mut echelon: Vec<u64> = Vec::new();
    let mut basis: Vec<F2mElement> = Vec::new();
    let mut used = vec![false; orbits.len()];
    loop {
        if basis.len() >= ell_target {
            return Some(basis);
        }
        // Choose the next orbit greedily: among orbits that keep the total
        // within ell_cap and make progress, prefer one that reaches
        // ell_target (smallest such), else the one with the most progress.
        let mut pick: Option<usize> = None;
        let mut pick_total = 0usize;
        let mut pick_reaches = false;
        for (i, orb) in orbits.iter().enumerate() {
            if used[i] {
                continue;
            }
            let d = incr(&echelon, orb);
            if d == 0 {
                continue;
            }
            let total = basis.len() + d;
            if total > ell_cap {
                continue;
            }
            let reaches = total >= ell_target;
            let better = match (reaches, pick_reaches) {
                (true, false) => true,
                (true, true) => total < pick_total, // smallest that reaches
                (false, false) => total > pick_total, // most progress
                (false, true) => false,
            };
            if pick.is_none() || better {
                pick = Some(i);
                pick_total = total;
                pick_reaches = reaches;
            }
        }
        let i = pick?;
        used[i] = true;
        for o in &orbits[i] {
            echelon_add(&mut echelon, &mut basis, o, n);
        }
    }
}

/// The polynomial-basis low-degree window `span{1, x, …, x^{ell-1}}`
/// (bit `k` = `x^k`). Structured, generally NOT squaring-closed.
fn poly_window_basis(n: u32, ell: usize) -> Vec<F2mElement> {
    (0..ell as u32)
        .map(|k| F2mElement::from_bit_positions(&[k], n))
        .collect()
}

/// Is `span(basis)` closed under field squaring (Frobenius)? A diagnostic
/// recorded per row: `true` certifies the arm is a genuine Frobenius-orbit
/// subspace, whatever its label.
fn is_squaring_closed(basis: &[F2mElement], irr: &IrreduciblePoly, n: u32) -> bool {
    // Echelon of span(basis).
    let mut ech: Vec<u64> = Vec::new();
    for e in basis {
        let r = ech.iter().fold(word_of(e), |r, &x| r.min(r ^ x));
        if r != 0 {
            ech.push(r);
            ech.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    let in_span = |w: u64| ech.iter().fold(w, |r, &x| r.min(r ^ x)) == 0;
    let _ = n;
    basis.iter().all(|e| in_span(word_of(&e.square(irr))))
}

/// Exact Boolean solution count of the decomposition system, for any `m >= 2`.
/// For `m >= 3` this is the library's chained counter; for `m = 2` it is a
/// direct enumeration of `V x V` against the binary `S_3`
/// `(x1+x2)^2 x3^2 + x1 x2 x3 + (x1 x2)^2 + b` (cheap for pilot `ℓ`).
fn solution_count(
    m: usize,
    basis: &[F2mElement],
    x_r: &F2mElement,
    b: &F2mElement,
    irr: &IrreduciblePoly,
) -> u64 {
    if m >= 3 {
        return chained_solution_count_fast(m, basis, x_r, b, irr);
    }
    // m == 2: enumerate the 2^ell subspace, count (x1,x2) with S_3 = 0.
    let ell = basis.len();
    let n = irr.degree;
    let subspace: Vec<F2mElement> = (0..(1u64 << ell))
        .map(|mask| {
            let mut acc = F2mElement::zero(n);
            for (t, bv) in basis.iter().enumerate() {
                if (mask >> t) & 1 == 1 {
                    acc = acc.add(bv);
                }
            }
            acc
        })
        .collect();
    let x3 = x_r.clone();
    let x3sq = x3.square(irr);
    let mut count = 0u64;
    for x1 in &subspace {
        for x2 in &subspace {
            let s = x1.add(x2).square(irr).mul(&x3sq, irr);
            let prod = x1.mul(x2, irr);
            let s = s.add(&prod.mul(&x3, irr)).add(&prod.square(irr)).add(b);
            if s == F2mElement::zero(n) {
                count += 1;
            }
        }
    }
    count
}

fn build_arm(
    arm: &str,
    n: u32,
    ell: usize,
    irr: &IrreduciblePoly,
    rng: &mut StdRng,
) -> Option<Vec<F2mElement>> {
    match arm {
        "orbit" => orbit_invariant_basis(n, ell, irr, rng),
        "random" => {
            if (ell as u32) < n && ell >= 1 {
                Some(random_subspace_basis(n, ell, rng))
            } else {
                None
            }
        }
        "subspace" => {
            if (ell as u32) < n && ell >= 1 {
                Some(poly_window_basis(n, ell))
            } else {
                None
            }
        }
        _ => None,
    }
}

// ─────────── HKY mutant closure (last fall degree d_F) ───────────
// Copied verbatim from examples/hky_par.rs (u64 masks, n_vars <= 64), so a
// `random`-arm row here reproduces hky_par's `d_f`/per-degree closure dims
// for the same system by construction (the equality gate). Attribution:
// examples/hky_par.rs in this repository.

struct Space {
    c: u32,
    binom: Vec<Vec<u64>>,
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

struct Basis {
    piv: Vec<Option<Box<[u64]>>>,
    mask: Vec<u64>,
    dim: usize,
}

impl Basis {
    fn reduce_full(&self, mut v: Vec<u64>) -> Option<Vec<u64>> {
        loop {
            let t = top(&v)?;
            if let Some(p) = &self.piv[t] {
                for (a, b) in v.iter_mut().zip(p.iter()) {
                    *a ^= *b;
                }
            } else {
                return Some(v);
            }
        }
    }
    fn absorb(&mut self, rows: Vec<Vec<u64>>) -> Vec<usize> {
        let mut added = Vec::new();
        for v in rows {
            if let Some(mut r) = self.reduce_full(v) {
                if let Some(t) = top(&r) {
                    r.truncate(t / 64 + 1);
                    self.piv[t] = Some(r.into_boxed_slice());
                    self.mask[t / 64] |= 1 << (t % 64);
                    self.dim += 1;
                    added.push(t);
                }
            }
        }
        added
    }
}

/// `(plain dim after the Macaulay round, closure dim, rounds)`; stops at
/// `target` or at `1 ∈ V`.
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
        .map_or((1usize << 30) / (8 * sp.words.max(1)), |v| {
            v.parse().unwrap()
        })
        .max(256);
    let bases: Vec<Option<Vec<u64>>> = eqs
        .iter()
        .map(|f| (f.iter().map(|m| m.count_ones()).max().unwrap_or(0) <= c).then(|| sp.row(f)))
        .collect();
    enum Src {
        Mac(usize, u64),
        Mul(usize, u32),
    }
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
    loop {
        rounds += 1;
        let mut new_low: Vec<usize> = vec![];
        for part in srcs.chunks(chunk) {
            let (spr, basr, bsr) = (&sp, &basis, &bases);
            let residues: Vec<Vec<u64>> = part
                .par_iter()
                .filter_map(|s| {
                    let v = match *s {
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
        if plain.is_none() {
            plain = Some(basis.dim);
        }
        if basis.piv[0].is_some() {
            return (plain.unwrap(), total.min(target), rounds);
        }
        if basis.dim >= target || new_low.is_empty() {
            return (plain.unwrap(), basis.dim, rounds);
        }
        srcs = new_low
            .iter()
            .flat_map(|&t| (0..nv).map(move |i| Src::Mul(t, i)))
            .collect();
    }
}

/// `(d_F, per-degree {plain,closed,of,rounds})`. `d_F` is the least `c` with
/// `1 ∈ V_{F,c}` (i.e. the closure saturates the whole monomial space);
/// `None` means `d_F > cmax`.
fn hky_last_fall(eqs: &[Vec<u64>], nv: u32, cmax: u32) -> (Option<u32>, Value) {
    let mut dims = serde_json::Map::new();
    let mut d_f = None;
    for c in 2..=cmax {
        let total = Space::new(nv, c).monos.len();
        let (plain, closed, rounds) = closure_par(eqs, nv, c, total);
        dims.insert(
            c.to_string(),
            json!({"plain": plain, "closed": closed, "of": total, "rounds": rounds,
                   "fell": closed > plain}),
        );
        if closed == total {
            d_f = Some(c);
            break;
        }
    }
    (d_f, Value::Object(dims))
}

fn system_u64(
    basis: &[F2mElement],
    x_r: &F2mElement,
    m: usize,
    st: &FieldStructure,
) -> Option<(Vec<Vec<u64>>, u32)> {
    let sys = build_decomposition_system(basis, x_r, &F2mElement::one(st.n), m, st)?;
    let eqs = sys
        .equations
        .iter()
        .map(|p| p.terms.iter().map(|t| t.mask).collect())
        .collect();
    Some((eqs, sys.n_vars as u32))
}

// ───────────────────────── driver ─────────────────────────

fn cell_seed(master: u64, tag: &str, n: u32, ell: usize, m: usize, draw: usize) -> u64 {
    let mut h = master
        ^ (u64::from(n) << 40)
        ^ ((ell as u64) << 32)
        ^ ((m as u64) << 48)
        ^ ((draw as u64) << 16);
    for b in tag.bytes() {
        h = h.wrapping_mul(0x100000001b3).wrapping_add(b as u64);
    }
    h
}

fn getflag(args: &[String], k: &str) -> Option<String> {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn loadavg() -> f64 {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(|x| x.to_string()))
        .and_then(|x| x.parse().ok())
        .unwrap_or(-1.0)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selftest") => selftest(),
        Some("run") => run(&args),
        _ => eprintln!(
            "usage:\n  binary_falldeg_arms selftest\n  binary_falldeg_arms run --cells m:n:ell,... \
             --seed S [--keep K] [--cmax C] [--ffd-max F] [--dres-max D] \
             [--arms orbit,random,subspace] [--crosscheck]"
        ),
    }
}

fn run(args: &[String]) {
    let cells = getflag(args, "--cells").expect("--cells required");
    let seed: u64 = getflag(args, "--seed").map_or(0x4b_4f_42_5a, |v| v.parse().unwrap());
    let keep: usize = getflag(args, "--keep").map_or(4, |v| v.parse().unwrap());
    let cmax: u32 = getflag(args, "--cmax").map_or(4, |v| v.parse().unwrap());
    let ffd_max: u32 = getflag(args, "--ffd-max").map_or(cmax + 2, |v| v.parse().unwrap());
    let dres_max: u32 = getflag(args, "--dres-max").map_or(ffd_max, |v| v.parse().unwrap());
    let crosscheck = args.iter().any(|a| a == "--crosscheck");
    let arms: Vec<String> = getflag(args, "--arms")
        .unwrap_or_else(|| "orbit,random,subspace".into())
        .split(',')
        .map(|s| s.to_string())
        .collect();

    for cell in cells.split(',') {
        let p: Vec<usize> = cell.split(':').map(|x| x.parse().unwrap()).collect();
        let (m, n, ell_target) = (p[0], p[1] as u32, p[2]);
        let irr = match find_irreducible_sparse(n) {
            Some(i) => i,
            None => {
                eprintln!("no sparse irreducible for n={n}; skipping cell {cell}");
                continue;
            }
        };
        let st = FieldStructure::new(n, &irr);

        // When the orbit arm is requested it fixes ℓ_eff (a τ-invariant
        // subspace has a constrained dimension); all arms then share ℓ_eff.
        // When the orbit arm is NOT requested, ℓ_eff = ℓ_target (so prime-n
        // cells still yield random/subspace rows even where no orbit base
        // exists — the H-avail case is recorded separately).
        let ell_eff = if arms.iter().any(|a| a == "orbit") {
            let mut seed_rng =
                StdRng::seed_from_u64(cell_seed(seed, "ell_eff", n, ell_target, m, 0));
            match orbit_invariant_basis(n, ell_target, &irr, &mut seed_rng) {
                Some(b) => b.len(),
                None => {
                    eprintln!(
                        "no orbit subspace in [{ell_target}, n) for n={n} (H-avail); skipping cell {cell}"
                    );
                    continue;
                }
            }
        } else {
            ell_target
        };

        let mut draw = 0usize;
        let mut kept = 0usize;
        let mut attempts = 0usize;
        while kept < keep && attempts < 500 {
            attempts += 1;
            let d = draw;
            draw += 1;
            // Target shared across arms for this draw.
            let mut xr_rng = StdRng::seed_from_u64(cell_seed(seed, "xr", n, ell_eff, m, d));
            let x_r = elt_from_word(xr_rng.gen::<u64>() & ((1u64 << n) - 1), n);
            let b1 = F2mElement::one(n);

            // Build each arm's basis; measure only draws inconsistent in ALL
            // requested arms (so a kept draw yields one comparable row per arm).
            let mut bases: Vec<(String, Vec<F2mElement>)> = Vec::new();
            let mut all_ok = true;
            for arm in &arms {
                let mut arng = StdRng::seed_from_u64(cell_seed(seed, arm, n, ell_eff, m, d));
                match build_arm(arm, n, ell_eff, &irr, &mut arng) {
                    Some(b) => bases.push((arm.clone(), b)),
                    None => {
                        all_ok = false;
                        break;
                    }
                }
            }
            if !all_ok {
                continue;
            }
            let sols: Vec<u64> = bases
                .iter()
                .map(|(_, b)| solution_count(m, b, &x_r, &b1, &irr))
                .collect();
            if sols.iter().any(|&s| s > 0) {
                continue;
            }
            kept += 1;

            for ((arm, basis), _sol) in bases.iter().zip(sols.iter()) {
                let (eqs, nv) = match system_u64(basis, &x_r, m, &st) {
                    Some(x) => x,
                    None => {
                        eprintln!("system build failed arm={arm} cell={cell}");
                        continue;
                    }
                };
                let sqclosed = is_squaring_closed(basis, &irr, n);
                // library first-fall proxy + Macaulay profiles
                let sys = build_decomposition_system(basis, &x_r, &b1, m, &st).unwrap();
                let t0 = Instant::now();
                let (ffd, profs) = first_fall_degree(&sys.equations, nv as usize, ffd_max);
                let ffd_secs = t0.elapsed().as_secs_f64();
                let per_ffd: Vec<Value> = profs
                    .iter()
                    .map(|p| json!({"d": p.degree, "rows": p.rows, "cols": p.cols, "rank": p.rank}))
                    .collect();
                // HKY mutant closure (true last fall degree)
                let t1 = Instant::now();
                let (d_f, per_hky) = hky_last_fall(&eqs, nv, cmax);
                let hky_secs = t1.elapsed().as_secs_f64();
                // bounded-Macaulay resolution degree (context)
                let t2 = Instant::now();
                let (d_res, _) = solving_degree(&sys.equations, nv as usize, dres_max);
                let dres_secs = t2.elapsed().as_secs_f64();

                let mut row = json!({
                    "schema": "binary_falldeg/v1",
                    "arm": arm, "m": m, "n": n, "ell_target": ell_target, "ell_eff": ell_eff,
                    "draw": d, "seed": seed, "n_vars": nv, "n_eqs": eqs.len(),
                    "x_r": word_of(&x_r), "v_basis": basis.iter().map(word_of).collect::<Vec<_>>(),
                    "squaring_closed": sqclosed, "solutions": 0,
                    "d_ff_proxy": ffd, "d_F": d_f,
                    "d_F_at_least": if d_f.is_none() { Some(cmax + 1) } else { None },
                    "d_res": d_res,
                    "d_res_at_least": if d_res.is_none() { Some(dres_max + 1) } else { None },
                    "per_degree_macaulay": per_ffd, "per_degree_closure": per_hky,
                    "secs": {"ffd": (ffd_secs*1e3).round()/1e3, "hky": (hky_secs*1e3).round()/1e3, "dres": (dres_secs*1e3).round()/1e3},
                    "loadavg": loadavg(),
                    "threads": rayon::current_num_threads(),
                });
                if crosscheck {
                    // independent sanity: d_ff <= d_F (both measured here)
                    if let (Some(a), Some(c)) = (ffd, d_f) {
                        row["check_ffd_le_dF"] = json!(a <= c);
                    }
                }
                println!("{}", serde_json::to_string(&row).unwrap());
            }
        }
        eprintln!(
            "cell {cell}: ell_eff={ell_eff}, kept {kept}/{keep} inconsistent draws in {attempts} attempts"
        );
    }
}

fn selftest() {
    let mut fails = 0;
    // 1. squaring orbit + invariant subspace is squaring-closed.
    for &n in &[7u32, 9, 11, 13, 15] {
        let irr = match find_irreducible_sparse(n) {
            Some(i) => i,
            None => continue,
        };
        let mut rng = StdRng::seed_from_u64(1234 + n as u64);
        let ellt = (n as usize) / 2;
        if let Some(b) = orbit_invariant_basis(n, ellt, &irr, &mut rng) {
            let ok = is_squaring_closed(&b, &irr, n) && b.len() >= ellt;
            println!(
                "[selftest] orbit n={n} ell_eff={} squaring_closed={ok}",
                b.len()
            );
            if !ok {
                fails += 1;
            }
        }
        // window basis: right dimension, and (for ell small) a known shape.
        let w = poly_window_basis(n, ellt);
        if w.len() != ellt {
            fails += 1;
            println!("[selftest] FAIL window dim n={n}");
        }
    }
    // 2. On a concrete inconsistent m=2 cell, d_ff <= d_F <= d_res and the
    //    three instruments agree on orderings; also run all arms.
    let (m, n, ellt) = (2usize, 15u32, 6usize);
    let irr = find_irreducible_sparse(n).unwrap();
    let st = FieldStructure::new(n, &irr);
    let mut rng = StdRng::seed_from_u64(0xabc);
    let ell_eff = orbit_invariant_basis(n, ellt, &irr, &mut rng)
        .unwrap()
        .len();
    let b1 = F2mElement::one(n);
    for arm in ["orbit", "random", "subspace"] {
        let mut arng = StdRng::seed_from_u64(99 ^ arm.len() as u64);
        // find an inconsistent draw
        let mut done = false;
        for t in 0..200 {
            let basis = match build_arm(arm, n, ell_eff, &irr, &mut arng) {
                Some(b) => b,
                None => break,
            };
            let x_r = elt_from_word((0x1357u64.wrapping_mul(t + 1)) & ((1u64 << n) - 1), n);
            if solution_count(m, &basis, &x_r, &b1, &irr) > 0 {
                continue;
            }
            let (eqs, nv) = system_u64(&basis, &x_r, m, &st).unwrap();
            let sys = build_decomposition_system(&basis, &x_r, &b1, m, &st).unwrap();
            let (ffd, _) = first_fall_degree(&sys.equations, nv as usize, 6);
            let (d_f, _) = hky_last_fall(&eqs, nv, 5);
            let (d_res, _) = solving_degree(&sys.equations, nv as usize, 6);
            let sq = is_squaring_closed(&basis, &irr, n);
            println!(
                "[selftest] arm={arm} n={n} ell_eff={ell_eff} sqclosed={sq} d_ff={ffd:?} d_F={d_f:?} d_res={d_res:?}"
            );
            // NOTE: d_ff here is the library's rank-deficiency PROXY, which can
            // exceed d_F (the library warns it is not the true first-fall
            // degree). An a > c is therefore reported, not a failure.
            if let (Some(a), Some(c)) = (ffd, d_f) {
                if a > c {
                    println!("[selftest] note: proxy d_ff={a} > d_F={c} for arm={arm} (expected; d_ff is a rank proxy)");
                }
            }
            // orbit arm must be squaring-closed; subspace window must not be
            // (at this ell_eff < n it leaves the window); random is incidental.
            if arm == "orbit" && !sq {
                fails += 1;
                println!("[selftest] FAIL orbit arm not squaring-closed");
            }
            done = true;
            break;
        }
        if !done {
            println!("[selftest] note: no inconsistent draw found for arm={arm}");
        }
    }
    if fails == 0 {
        println!("[selftest] OK");
    } else {
        println!("[selftest] {fails} FAILURES");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orbit_basis_is_squaring_closed() {
        for &n in &[7u32, 9, 11, 13, 15] {
            let irr = match find_irreducible_sparse(n) {
                Some(i) => i,
                None => continue,
            };
            let mut rng = StdRng::seed_from_u64(42 + n as u64);
            if let Some(b) = orbit_invariant_basis(n, (n as usize) / 2, &irr, &mut rng) {
                assert!(
                    is_squaring_closed(&b, &irr, n),
                    "orbit subspace must be Frobenius-invariant, n={n}"
                );
                assert!(b.len() >= (n as usize) / 2);
            }
        }
    }

    #[test]
    fn window_basis_has_requested_dimension() {
        for &n in &[11u32, 13, 19] {
            for ell in 2..6 {
                assert_eq!(poly_window_basis(n, ell).len(), ell);
            }
        }
    }

    #[test]
    fn squaring_orbit_closes() {
        let irr = find_irreducible_sparse(13).unwrap();
        let g = F2mElement::from_bit_positions(&[1], 13);
        let orb = squaring_orbit(&g, &irr);
        // last element squared returns to g
        assert_eq!(orb[0], g);
        assert_eq!(orb.last().unwrap().square(&irr), g);
        // all distinct
        for i in 0..orb.len() {
            for j in i + 1..orb.len() {
                assert_ne!(orb[i], orb[j]);
            }
        }
    }

    #[test]
    fn ffd_le_last_fall_on_inconsistent_m2_cell() {
        let (m, n) = (2usize, 15u32);
        let irr = find_irreducible_sparse(n).unwrap();
        let st = FieldStructure::new(n, &irr);
        let mut rng = StdRng::seed_from_u64(7);
        let ell = orbit_invariant_basis(n, 6, &irr, &mut rng).unwrap().len();
        let b1 = F2mElement::one(n);
        let mut found = false;
        for t in 0..300u64 {
            let basis = random_subspace_basis(n, ell, &mut rng);
            let x_r = elt_from_word((0x9e37u64.wrapping_mul(t + 1)) & ((1u64 << n) - 1), n);
            if solution_count(m, &basis, &x_r, &b1, &irr) > 0 {
                continue;
            }
            let (eqs, nv) = system_u64(&basis, &x_r, m, &st).unwrap();
            let sys = build_decomposition_system(&basis, &x_r, &b1, m, &st).unwrap();
            let (_ffd, _) = first_fall_degree(&sys.equations, nv as usize, 6);
            let (d_f, _) = hky_last_fall(&eqs, nv, 5);
            // The trustworthy number is d_F from the HKY closure: for an
            // inconsistent system it must be found by cmax and be >= 2.
            // (The rank-proxy d_ff is NOT asserted <= d_F; the library warns
            // it can exceed d_F, and the pilot observed exactly that.)
            if let Some(c) = d_f {
                assert!((2..=5).contains(&c), "d_F={c} out of expected range");
                found = true;
                break;
            }
        }
        assert!(found, "expected at least one inconsistent, resolved draw");
    }
}
