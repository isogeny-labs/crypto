//! pc_closure: degree-D polynomial-calculus closure W_D of a set of generator
//! polynomials over F_p, and whether 1 is in W_D.
//!
//! Port of `closure_degree` (with `monomials`, `rref_small`, `matmul_mod`) from
//! `../scripts/raw_closure.py`. Same column order (degree descending,
//! combinations_with_replacement order within a degree, constant last), same
//! "expand every basis row whose pivot has degree <= D-1 and whose content
//! changed since it was last expanded" loop, same per-round snapshot of the rows
//! to expand. The basis is kept in fully reduced row echelon form, which is
//! unique for a given space, so rank, one_in_W and the expansion count match the
//! Python reference exactly.
//!
//! Input JSON: {"p": int, "n": int, "polys": [[[[e_0,...,e_{n-1}], coef], ...], ...]}
//! Usage: pc_closure --system FILE [--dmin 1] [--dmax 8] [--out FILE] [--quiet]

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

#[derive(Deserialize)]
struct System {
    p: u64,
    n: usize,
    polys: Vec<Vec<(Vec<u64>, i64)>>,
}

const NONE: u32 = u32::MAX;

fn pow_mod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}

fn inv_mod(a: u64, p: u64) -> u64 {
    pow_mod(a, p - 2, p)
}

/// Monomials of total degree <= D in n variables, degree descending, constant last;
/// within a degree, itertools.combinations_with_replacement(range(n), d) order.
fn monomials(n: usize, d_max: usize) -> Vec<Vec<u16>> {
    fn rec(n: usize, start: usize, left: usize, cur: &mut Vec<u16>, out: &mut Vec<Vec<u16>>) {
        if left == 0 {
            out.push(cur.clone());
            return;
        }
        for i in start..n {
            cur[i] += 1;
            rec(n, i, left - 1, cur, out);
            cur[i] -= 1;
        }
    }
    let mut out = Vec::new();
    for d in (0..=d_max).rev() {
        let mut cur = vec![0u16; n];
        rec(n, 0, d, &mut cur, &mut out);
    }
    out
}

/// Two independent 64-bit hashes of a row's content (expansion bookkeeping).
fn row_hash(row: &[u32]) -> (u64, u64) {
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0x9e37_79b9_7f4a_7c15;
    for (j, &x) in row.iter().enumerate() {
        if x == 0 {
            continue;
        }
        let w = ((j as u64) << 32) | x as u64;
        h1 = (h1 ^ w).wrapping_mul(0x0000_0100_0000_01b3);
        let mut z = w.wrapping_add(h2).wrapping_add(0x9e37_79b9_7f4a_7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        h2 = z ^ (z >> 31);
    }
    (h1, h2)
}

/// dst[j] = (dst[j] + coef * src[j]) mod p for j >= start.
#[inline]
fn axpy(dst: &mut [u32], src: &[u32], coef: u64, start: usize, p: u64) {
    for (d, &s) in dst[start..].iter_mut().zip(src[start..].iter()) {
        if s != 0 {
            *d = ((*d as u64 + coef * s as u64) % p) as u32;
        }
    }
}

struct Basis {
    p: u64,
    cols: usize,
    rows: Vec<Vec<u32>>,  // fully reduced; row i has pivot piv[i] (value 1)
    piv: Vec<u32>,        // pivot column of each row
    row_of_col: Vec<u32>, // col -> row index, NONE if not a pivot column
}

impl Basis {
    fn new(p: u64, cols: usize) -> Self {
        Basis { p, cols, rows: Vec::new(), piv: Vec::new(), row_of_col: vec![NONE; cols] }
    }

    /// Reduce r against the basis: walk r's entries at pivot columns in increasing
    /// order. The basis is fully reduced (pivot columns are unit vectors), so each
    /// subtraction leaves r unchanged at every other pivot column.
    fn reduce(&self, r: &mut [u32]) {
        let p = self.p;
        for c in 0..self.cols {
            let x = r[c];
            if x == 0 {
                continue;
            }
            let ri = self.row_of_col[c];
            if ri == NONE {
                continue;
            }
            axpy(r, &self.rows[ri as usize], p - x as u64, c, p);
        }
    }

    /// Insert a reduced, nonzero row (zero at every existing pivot column):
    /// normalise, eliminate its pivot from the existing rows, append.
    fn insert(&mut self, mut r: Vec<u32>) {
        let p = self.p;
        let lead = r.iter().position(|&x| x != 0).expect("insert of zero row");
        let iv = inv_mod(r[lead] as u64, p);
        for x in r[lead..].iter_mut() {
            if *x != 0 {
                *x = (*x as u64 * iv % p) as u32;
            }
        }
        for row in self.rows.iter_mut() {
            let y = row[lead];
            if y != 0 {
                axpy(row, &r, p - y as u64, lead, p);
            }
        }
        self.row_of_col[lead] = self.rows.len() as u32;
        self.piv.push(lead as u32);
        self.rows.push(r);
    }

    /// Reduce and insert if a nonzero residual remains; returns true if rank grew.
    fn add(&mut self, mut r: Vec<u32>) -> bool {
        self.reduce(&mut r);
        if r.iter().any(|&x| x != 0) {
            self.insert(r);
            true
        } else {
            false
        }
    }
}

struct DResult {
    one: bool,
    rank: usize,
    cols: usize,
    expansions: u64,
}

fn closure_degree(sys: &System, d: usize, quiet: bool) -> DResult {
    let n = sys.n;
    let p = sys.p;
    let monos = monomials(n, d);
    let cols = monos.len();
    let mut col: HashMap<Vec<u16>, u32> = HashMap::with_capacity(cols);
    for (i, mo) in monos.iter().enumerate() {
        col.insert(mo.clone(), i as u32);
    }
    let deg_of_col: Vec<usize> = monos.iter().map(|m| m.iter().map(|&e| e as usize).sum()).collect();
    // shift[v*cols + c] = column of monos[c] * x_v (only for deg(c) <= D-1)
    let mut shift = vec![NONE; n * cols];
    for (c, mo) in monos.iter().enumerate() {
        if deg_of_col[c] + 1 > d {
            continue;
        }
        let mut e = mo.clone();
        for v in 0..n {
            e[v] += 1;
            shift[v * cols + c] = col[&e];
            e[v] -= 1;
        }
    }
    let const_col = cols - 1;

    // generators with total degree <= D (degree as Poly.deg(): max over listed terms)
    let mut basis = Basis::new(p, cols);
    let mut n_gens = 0usize;
    for poly in &sys.polys {
        let pdeg: i64 = poly.iter().map(|(e, _)| e.iter().sum::<u64>() as i64).max().unwrap_or(-1);
        if pdeg > d as i64 {
            continue;
        }
        n_gens += 1;
        let mut r = vec![0u32; cols];
        for (e, c) in poly {
            let key: Vec<u16> = e.iter().map(|&x| x as u16).collect();
            let ci = col[&key] as usize;
            let cv = c.rem_euclid(p as i64) as u64;
            r[ci] = ((r[ci] as u64 + cv) % p) as u32;
        }
        basis.add(r);
    }
    if n_gens == 0 {
        return DResult { one: false, rank: 0, cols, expansions: 0 };
    }

    let mut expanded: Vec<Option<(u64, u64)>> = vec![None; cols];
    let mut n_exp: u64 = 0;
    let mut round = 0usize;
    loop {
        if basis.row_of_col[const_col] != NONE {
            return DResult { one: true, rank: basis.rows.len(), cols, expansions: n_exp };
        }
        // rows with pivot of degree <= D-1 whose content changed since last expansion;
        // snapshot them sparsely (as Python builds all shifted rows before merging)
        let mut todo: Vec<Vec<(u32, u32)>> = Vec::new();
        for (ri, &pc) in basis.piv.iter().enumerate() {
            let pc = pc as usize;
            if deg_of_col[pc] + 1 <= d {
                let row = &basis.rows[ri];
                let h = row_hash(row);
                if expanded[pc] != Some(h) {
                    expanded[pc] = Some(h);
                    todo.push(
                        row.iter()
                            .enumerate()
                            .filter(|(_, &x)| x != 0)
                            .map(|(j, &x)| (j as u32, x))
                            .collect(),
                    );
                }
            }
        }
        if todo.is_empty() {
            return DResult { one: false, rank: basis.rows.len(), cols, expansions: n_exp };
        }
        round += 1;
        let rank0 = basis.rows.len();
        n_exp += (todo.len() * n) as u64;
        let t0 = Instant::now();
        for snap in todo.into_iter() {
            for v in 0..n {
                let mut r = vec![0u32; cols];
                let sh = &shift[v * cols..(v + 1) * cols];
                for &(c0, x) in &snap {
                    let c1 = sh[c0 as usize];
                    debug_assert!(c1 != NONE);
                    r[c1 as usize] = x;
                }
                basis.add(r);
            }
        }
        if !quiet {
            eprintln!(
                "    D={} round {}: expanded rows -> +{} pivots (rank {}/{}), {:.2}s",
                d,
                round,
                basis.rows.len() - rank0,
                basis.rows.len(),
                cols,
                t0.elapsed().as_secs_f64()
            );
        }
    }
}

fn usage() -> ! {
    eprintln!("usage: pc_closure --system FILE [--dmin N] [--dmax N] [--out FILE] [--quiet]");
    std::process::exit(2)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut system: Option<String> = None;
    let mut dmin: usize = 1;
    let mut dmax: usize = 8;
    let mut out: Option<String> = None;
    let mut quiet = false;
    let mut i = 1;
    while i < args.len() {
        let a = args[i].as_str();
        let val = |i: usize| -> String { args.get(i + 1).cloned().unwrap_or_else(|| usage()) };
        match a {
            "--system" => { system = Some(val(i)); i += 1; }
            "--dmin" => { dmin = val(i).parse().unwrap_or_else(|_| usage()); i += 1; }
            "--dmax" => { dmax = val(i).parse().unwrap_or_else(|_| usage()); i += 1; }
            "--out" => { out = Some(val(i)); i += 1; }
            "--quiet" => quiet = true,
            "-h" | "--help" => usage(),
            _ => usage(),
        }
        i += 1;
    }
    let path = system.unwrap_or_else(|| usage());
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {}", path, e);
        std::process::exit(1)
    });
    let sys: System = serde_json::from_str(&text).unwrap_or_else(|e| {
        eprintln!("bad system JSON {}: {}", path, e);
        std::process::exit(1)
    });
    assert!(sys.p > 2 && sys.p < (1u64 << 31), "p must be an odd prime < 2^31");
    for poly in &sys.polys {
        for (e, _) in poly {
            assert_eq!(e.len(), sys.n, "exponent vector length != n");
        }
    }
    if !quiet {
        eprintln!("system {}: p={} n={} polys={}", path, sys.p, sys.n, sys.polys.len());
    }
    let mut outf = out.map(|f| {
        OpenOptions::new().create(true).append(true).open(&f).unwrap_or_else(|e| {
            eprintln!("cannot open {}: {}", f, e);
            std::process::exit(1)
        })
    });
    let mut emit = |s: String| {
        println!("{}", s);
        std::io::stdout().flush().ok();
        if let Some(f) = outf.as_mut() {
            writeln!(f, "{}", s).ok();
            f.flush().ok();
        }
    };
    for d in dmin..=dmax {
        let t0 = Instant::now();
        let r = closure_degree(&sys, d, quiet);
        let secs = (t0.elapsed().as_secs_f64() * 100.0).round() / 100.0;
        emit(format!(
            "{{\"D\": {}, \"rank\": {}, \"cols\": {}, \"one_in_W\": {}, \"expansions\": {}, \"seconds\": {:.2}}}",
            d, r.rank, r.cols, r.one, r.expansions, secs
        ));
        if r.one {
            emit(format!("{{\"certified_D\": {}}}", d));
            return;
        }
    }
    emit(format!("{{\"censored_D_ge\": {}}}", dmax + 1));
}
