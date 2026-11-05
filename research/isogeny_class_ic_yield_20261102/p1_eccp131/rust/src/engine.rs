//! Incremental Macaulay-matrix (XL-style) elimination over the 131-bit field.
//!
//! The repository's `f4_fp` is limited to p < 2^63, so ECCp-131 (p ~ 2^131) needs its own
//! instrument.  This engine builds, degree by degree D, the rows `mu * f_i` with
//! deg(mu * f_i) = D (lower multiples were added at earlier degrees), reduces each against the
//! echelon rows already held (columns in degrevlex order, leading column = largest monomial),
//! and keeps the echelon rows.  It records, per degree, the matrix size, rank, number of new
//! pivots, new pivots whose leading degree is below D (a degree fall), zero reductions, the
//! multiplication count and the staircase of leading monomials, from which it decides
//!   * refutation (the constant 1 is a leading monomial: the system has no solution), and
//!   * solving (the number of standard monomials equals the exact solution count K and every
//!     variable has a pure power as a leading monomial).

use crate::fp::{muls, Fp};
use crate::poly::{cmp_degrevlex, mono_deg, mono_exp, Mono, Poly};
use std::collections::HashMap;
use std::time::Instant;

pub struct Limits {
    pub dcap: u32,
    pub max_cols: usize,
    pub budget_s: f64,
}

#[derive(Clone, Debug)]
pub struct DegStat {
    pub d: u32,
    pub rows_added: usize,
    pub rows_cum: usize,
    pub cols: usize,
    pub rank: usize,
    pub new_pivots: usize,
    pub new_pivots_below_d: usize,
    pub zero_red: usize,
    pub nnz_pivots: usize,
    pub nstd: usize,
    pub pure_powers: usize,
    pub const_pivot: bool,
    pub mults: u64,
    pub row_ops: u64,
    pub cpu_s: f64,
}

pub struct Outcome {
    pub status: &'static str, // solved | cap_degree | cap_time | cap_cols
    pub d_solve: Option<u32>,
    pub d_max: u32,
    pub first_fall: Option<u32>,
    pub stats: Vec<DegStat>,
    pub pivots_by_lead_degree: Vec<usize>,
    pub final_rows: usize,
    pub final_cols: usize,
    pub final_rank: usize,
    pub nstd_final: usize,
    pub mults: u64,
    pub row_ops: u64,
    pub inconsistent: bool,
}

pub fn thread_cpu_s() -> f64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    unsafe {
        libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts);
    }
    ts.tv_sec as f64 + ts.tv_nsec as f64 * 1e-9
}

fn binom(n: usize, k: usize) -> u128 {
    let k = k.min(n - k);
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (n - i) as u128 / (i + 1) as u128;
    }
    r
}

fn gen_monos(n: usize, d: u32, out: &mut Vec<Mono>) {
    fn rec(pos: usize, n: usize, left: u32, cur: Mono, out: &mut Vec<Mono>) {
        if pos == n - 1 {
            out.push(cur | ((left as Mono) << (8 * pos)));
            return;
        }
        for e in 0..=left {
            rec(pos + 1, n, left - e, cur | ((e as Mono) << (8 * pos)), out);
        }
    }
    rec(0, n, d, 0, out);
}

pub struct MonoIds {
    pub list: Vec<Mono>,
    pub off: Vec<usize>, // off[k] = number of monomials of degree < k
    pub id: HashMap<Mono, u32>,
}

pub fn build_monos(n: usize, dmax: u32) -> MonoIds {
    let mut list = Vec::new();
    let mut off = vec![0usize];
    for d in 0..=dmax {
        let mut v = Vec::new();
        gen_monos(n, d, &mut v);
        v.sort_by(|a, b| cmp_degrevlex(*a, *b, n));
        list.extend(v);
        off.push(list.len());
    }
    let id = list.iter().enumerate().map(|(i, m)| (*m, i as u32)).collect();
    MonoIds { list, off, id }
}

/// Largest D with C(n + D, D) <= max_cols.
pub fn cap_by_cols(n: usize, dcap: u32, max_cols: usize) -> u32 {
    let mut d = 0;
    while d < dcap && binom(n + (d as usize) + 1, (d as usize) + 1) <= max_cols as u128 {
        d += 1;
    }
    d
}

pub fn semiregular_degree(degs: &[u32], n: usize) -> Option<u32> {
    // first k >= 1 with a non-positive coefficient of prod(1 - z^d_i) / (1 - z)^n
    let kmax = 80usize;
    let mut s = vec![0i128; kmax + 1];
    s[0] = 1;
    for &d in degs {
        for k in (d as usize..=kmax).rev() {
            s[k] -= s[k - d as usize];
        }
    }
    for _ in 0..n {
        for k in 1..=kmax {
            s[k] += s[k - 1];
        }
    }
    (1..=kmax).find(|&k| s[k] <= 0).map(|k| k as u32)
}

pub fn run(polys: &[Poly], n: usize, k_sol: usize, lim: &Limits) -> Outcome {
    let degs: Vec<u32> = polys.iter().map(|p| p.degree()).collect();
    let dcap = cap_by_cols(n, lim.dcap, lim.max_cols);
    let monos = build_monos(n, dcap);
    let ncols_all = monos.list.len();
    let terms: Vec<Vec<(Mono, Fp)>> = polys.iter().map(|p| p.sorted_terms(n)).collect();
    let mut pivots: Vec<Option<Vec<(u32, Fp)>>> = vec![None; ncols_all];
    let mut acc: Vec<Fp> = vec![Fp::ZERO; ncols_all];
    let start = Instant::now();
    let cpu0 = thread_cpu_s();
    let mul0 = muls();
    let mut row_ops: u64 = 0;
    let mut rows_cum = 0usize;
    let mut rank = 0usize;
    let mut stats: Vec<DegStat> = Vec::new();
    let mut first_fall = None;
    let mut status = "cap_degree";
    let mut d_solve = None;
    let mut d_max = 0;
    let mut nnz = 0usize;
    let mut aborted = false;
    let dmin = *degs.iter().min().unwrap();
    let mut deg_of: Vec<u8> = Vec::with_capacity(ncols_all);
    for d in 0..=dcap {
        for _ in monos.off[d as usize]..monos.off[d as usize + 1] {
            deg_of.push(d as u8);
        }
    }
    for d in dmin..=dcap {
        let t_d = thread_cpu_s();
        let m_d = muls();
        let ops_d = row_ops;
        let mut rows_added = 0usize;
        let mut new_p = 0usize;
        let mut new_below = 0usize;
        let mut zero_red = 0usize;
        'eqs: for (i, f) in terms.iter().enumerate() {
            if degs[i] > d {
                continue;
            }
            let md = d - degs[i];
            for mu_id in monos.off[md as usize]..monos.off[md as usize + 1] {
                if rows_added % 64 == 0 && start.elapsed().as_secs_f64() > lim.budget_s {
                    aborted = true;
                    break 'eqs;
                }
                let mu = monos.list[mu_id];
                let mut top = 0usize;
                for (m, c) in f {
                    let id = monos.id[&(m + mu)] as usize;
                    acc[id] = *c;
                    if id > top {
                        top = id;
                    }
                }
                rows_added += 1;
                // reduce
                let mut c = top as isize;
                loop {
                    while c >= 0 && acc[c as usize].is_zero() {
                        c -= 1;
                    }
                    if c < 0 {
                        zero_red += 1;
                        break;
                    }
                    let cu = c as usize;
                    if let Some(piv) = &pivots[cu] {
                        let fct = acc[cu];
                        for (id, v) in piv.iter() {
                            let a = &mut acc[*id as usize];
                            *a = a.sub(fct.mul(*v));
                        }
                        acc[cu] = Fp::ZERO;
                        row_ops += 1;
                        c -= 1;
                    } else {
                        let inv = acc[cu].inv();
                        let mut row: Vec<(u32, Fp)> = Vec::new();
                        for id in (0..=cu).rev() {
                            if !acc[id].is_zero() {
                                row.push((id as u32, acc[id].mul(inv)));
                                acc[id] = Fp::ZERO;
                            }
                        }
                        nnz += row.len();
                        pivots[cu] = Some(row);
                        rank += 1;
                        new_p += 1;
                        if (deg_of[cu] as u32) < d {
                            new_below += 1;
                        }
                        break;
                    }
                }
            }
        }
        if aborted {
            // clear scratch of the aborted row (only reachable between rows, acc is clean)
            status = "cap_time";
        }
        rows_cum += rows_added;
        let cols = monos.off[d as usize + 1];
        d_max = d;
        if new_below > 0 && first_fall.is_none() {
            first_fall = Some(d);
        }
        // staircase of leading monomials of degree <= d
        let mut in_ideal = vec![false; cols];
        let mut nstd = 0usize;
        for id in 0..cols {
            let m = monos.list[id];
            let mut inside = pivots[id].is_some();
            if !inside {
                for v in 0..n {
                    if mono_exp(m, v) > 0 {
                        let q = m - (1u128 << (8 * v));
                        if in_ideal[monos.id[&q] as usize] {
                            inside = true;
                            break;
                        }
                    }
                }
            }
            in_ideal[id] = inside;
            if !inside {
                nstd += 1;
            }
        }
        let mut pure = 0usize;
        for v in 0..n {
            if (1..=d).any(|e| pivots[monos.id[&((e as Mono) << (8 * v))] as usize].is_some()) {
                pure += 1;
            }
        }
        let const_pivot = pivots[0].is_some();
        stats.push(DegStat {
            d,
            rows_added,
            rows_cum,
            cols,
            rank,
            new_pivots: new_p,
            new_pivots_below_d: new_below,
            zero_red,
            nnz_pivots: nnz,
            nstd,
            pure_powers: pure,
            const_pivot,
            mults: muls() - m_d,
            row_ops: row_ops - ops_d,
            cpu_s: thread_cpu_s() - t_d,
        });
        if aborted {
            break;
        }
        let solved = if k_sol == 0 { const_pivot } else { pure == n && nstd == k_sol };
        if solved {
            status = "solved";
            d_solve = Some(d);
            break;
        }
        if const_pivot && k_sol > 0 {
            // inconsistent although solutions were counted: report, never hide
            status = "contradiction_with_K";
            d_solve = Some(d);
            break;
        }
    }
    if status == "cap_degree" && dcap < lim.dcap {
        status = "cap_cols";
    }
    let last = stats.last().unwrap();
    let mut by_lead = vec![0usize; d_max as usize + 1];
    for (id, p) in pivots.iter().enumerate() {
        if p.is_some() {
            by_lead[deg_of[id] as usize] += 1;
        }
    }
    Outcome {
        status,
        d_solve,
        d_max,
        first_fall,
        pivots_by_lead_degree: by_lead,
        final_rows: rows_cum,
        final_cols: last.cols,
        final_rank: last.rank,
        nstd_final: last.nstd,
        inconsistent: last.const_pivot,
        stats: stats.clone(),
        mults: muls() - mul0,
        row_ops,
    }
    .with_cpu(cpu0)
}

impl Outcome {
    fn with_cpu(self, _c: f64) -> Outcome {
        self
    }
}

#[allow(dead_code)]
pub fn total_deg(m: Mono) -> u32 {
    mono_deg(m)
}
