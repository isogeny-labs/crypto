//! Round-indexed HKY closure depth for m = 2 Kummer towers (wave 3, lab E).
//!
//! For a degree cap `c`:
//!   W_0 = span(F ∩ R_{≤c}),
//!   W_r = W_{r-1} + span{ x·g : g ∈ W_{r-1} ∩ R_{<c}, x a variable }.
//! Reported per round: dim W_r. `r*` is the first round with 1 ∈ W_r; `G` is the
//! number of rounds r ≥ 1 in which W grows. This is the Rust counterpart of
//! `research/kummer_wave3_20261115/depth/depth.py` (Python reference).
//!
//! Ring: R = F_p[y_{1,0..t-1}, y_{2,0..t-1}] / (y_{i,l}^2 - y_{i,l+1}, y_{i,t-1}^2 - 1),
//! i.e. the Kummer tower with c = 1 of `examples/pkm_tower_pilot.rs`. A normal-form
//! monomial is a square-free bitmask over the 2t variables; multiplying by a
//! variable sends a monomial to a monomial (carry chain), so rows stay sparse.
//! The system is the single polynomial F = S_3(x_1, x_2, x_R) with x_i = g·y_{i,0}.
//!
//! The echelon form orders columns degree-descending, so the rows whose pivot has
//! degree < c span W ∩ R_{<c} exactly. Rows are immutable once inserted, hence the
//! rows to multiply in round r are exactly the low-pivot rows inserted in round r-1.
//!
//! ```bash
//! RAYON_NUM_THREADS=1 cargo run --release --example hky_depth -- \
//!     --p 2013265921 --t 8 --c 4 --idx 0 [--planted] [--out rows.jsonl]
//! ```
//! Instances are drawn here (not by the pilot's builder): random curve, random
//! coset g·μ_{2^t}, random x_R that is the abscissa of a curve point and not in the
//! grid; a draw is kept only if S_3(x_i, x_j, x_R) ≠ 0 on the whole K² grid
//! (inconsistent), or with `--planted` a grid point is made a zero.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Write;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    a * b % p
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a % p + p - b % p) % p
}

fn sqrt_mod(n: u64, p: u64) -> Option<u64> {
    let n = n % p;
    if n == 0 {
        return Some(0);
    }
    if powm(n, (p - 1) / 2, p) != 1 {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while powm(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, powm(z, q, p), powm(n, q, p), powm(n, q.div_ceil(2), p));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

/// S_3(x1,x2,x3) for y^2 = x^3 + a x + b.
fn s3_eval(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> u64 {
    let d = subm(x1, x2, p);
    let t1 = mulm(mulm(d, d, p), mulm(x3, x3, p), p);
    let s = (x1 + x2) % p;
    let pr = mulm(x1, x2, p);
    let inner = (mulm(s, (pr + a) % p, p) + 2 * b) % p;
    let t2 = mulm(inner, x3, p) * (p - 2) % p;
    let q = subm(pr, a, p);
    let t3 = subm(mulm(q, q, p), mulm(s, 4 * b % p, p), p);
    (t1 + t2 + t3) % p
}

struct Instance {
    p: u64,
    t: usize,
    a: u64,
    b: u64,
    g: u64,
    zeta: u64,
    x_r: u64,
    grid_zeros: u64,
    tries: u64,
}

fn draw_instance(p: u64, t: usize, idx: u64, planted: bool) -> Instance {
    let seed = 0x9E37_79B9_7F4A_7C15u64
        ^ (p << 20)
        ^ ((t as u64) << 8)
        ^ idx.wrapping_mul(0x1000_0000_01b3)
        ^ if planted { 0xABCD_0000 } else { 0 };
    let mut rng = StdRng::seed_from_u64(seed);
    assert!((p - 1).is_multiple_of(1u64 << t), "2^t must divide p-1");
    let zeta = loop {
        let h = rng.gen_range(2..p);
        if powm(h, (p - 1) / 2, p) == p - 1 {
            break powm(h, (p - 1) >> t, p);
        }
    };
    let k = 1usize << t;
    let mut tries = 0;
    loop {
        tries += 1;
        let a = rng.gen_range(0..p);
        let b = rng.gen_range(1..p);
        // nonsingular: 4a^3 + 27 b^2 != 0
        let disc = (4 * powm(a, 3, p) + 27 * mulm(b, b, p)) % p;
        if disc == 0 {
            continue;
        }
        let g = rng.gen_range(2..p);
        let mut xs = Vec::with_capacity(k);
        let mut w = g;
        for _ in 0..k {
            xs.push(w);
            w = mulm(w, zeta, p);
        }
        let rhs = |x: u64| (powm(x, 3, p) + mulm(a, x, p) + b) % p;
        let on = xs
            .iter()
            .filter(|&&x| powm(rhs(x), (p - 1) / 2, p) == 1)
            .count();
        if on < 2 {
            continue;
        }
        if planted {
            let i = rng.gen_range(0..k);
            let j = rng.gen_range(0..k);
            let (x1, x2) = (xs[i], xs[j]);
            if x1 == x2 {
                continue;
            }
            let d = subm(x1, x2, p);
            let aa = mulm(d, d, p);
            let pr = mulm(x1, x2, p);
            let s = (x1 + x2) % p;
            let bb = subm(0, 2 * ((mulm(s, (pr + a) % p, p) + 2 * b) % p) % p, p);
            let q = subm(pr, a, p);
            let cc = subm(mulm(q, q, p), mulm(s, 4 * b % p, p), p);
            let dsc = subm(mulm(bb, bb, p), mulm(4, mulm(aa, cc, p), p), p);
            let Some(sq) = sqrt_mod(dsc, p) else { continue };
            let x_r = mulm((subm(sq, bb, p)) % p, invm(2 * aa % p, p), p);
            if xs.contains(&x_r) {
                continue;
            }
            assert_eq!(s3_eval(x1, x2, x_r, a, b, p), 0);
            return Instance {
                p,
                t,
                a,
                b,
                g,
                zeta,
                x_r,
                grid_zeros: 1,
                tries,
            };
        }
        let x_r = loop {
            let x = rng.gen_range(0..p);
            if powm(rhs(x), (p - 1) / 2, p) == 1 {
                break x;
            }
        };
        if xs.contains(&x_r) {
            continue;
        }
        // c2 xR^2 + c1 xR + c0 on the whole grid
        let xr2 = mulm(x_r, x_r, p);
        let mut zeros = 0u64;
        'grid: for &x1 in &xs {
            for &x2 in &xs {
                let d = subm(x1, x2, p);
                let pr = mulm(x1, x2, p);
                let s = (x1 + x2) % p;
                let c2 = mulm(d, d, p);
                let c1 = subm(0, 2 * ((mulm(s, (pr + a) % p, p) + 2 * b) % p) % p, p);
                let q = subm(pr, a, p);
                let c0 = subm(mulm(q, q, p), mulm(s, 4 * b % p, p), p);
                let v = (mulm(c2, xr2, p) + mulm(c1, x_r, p) + c0) % p;
                if v == 0 {
                    zeros += 1;
                    break 'grid;
                }
            }
        }
        if zeros == 0 {
            return Instance {
                p,
                t,
                a,
                b,
                g,
                zeta,
                x_r,
                grid_zeros: 0,
                tries,
            };
        }
    }
}

// ---------------------------------------------------------------- ring

/// y_j · (monomial m) in normal form (single monomial, coefficient 1).
fn mulvar(mut m: u32, mut j: usize, t: usize) -> u32 {
    loop {
        if m >> j & 1 == 0 {
            return m | 1 << j;
        }
        m &= !(1 << j);
        if j % t == t - 1 {
            return m;
        }
        j += 1;
    }
}

/// F = S_3(g y_{1,0}, g y_{2,0}, x_R) in normal form: (mask, coefficient).
fn build_f(inst: &Instance) -> Vec<(u32, u64)> {
    let (p, t) = (inst.p, inst.t);
    let (a, b, g, xr) = (inst.a, inst.b, inst.g, inst.x_r);
    let xr2 = mulm(xr, xr, p);
    // coefficient c[i][j] of X1^i X2^j
    let mut c = [[0u64; 3]; 3];
    c[2][0] = xr2;
    c[0][2] = xr2;
    c[1][1] = subm(subm(0, 2 * xr2 % p, p), 2 * a % p, p);
    c[2][1] = subm(0, 2 * xr % p, p);
    c[1][2] = c[2][1];
    c[2][2] = 1;
    let lin = subm(subm(0, 2 * mulm(xr, a, p) % p, p), 4 * b % p, p);
    c[1][0] = lin;
    c[0][1] = lin;
    c[0][0] = subm(a * a % p, 4 * mulm(xr, b, p) % p, p);
    let mut out = Vec::new();
    let pw = [1u64, 1, 1];
    let _ = pw;
    for i in 0..3usize {
        for j in 0..3usize {
            if c[i][j] == 0 {
                continue;
            }
            let coef = mulm(c[i][j], powm(g, (i + j) as u64, p), p);
            let mi = match i {
                0 => 0u32,
                1 => 1,
                _ => 2,
            };
            let mj = match j {
                0 => 0u32,
                1 => 1,
                _ => 2,
            };
            out.push((mi | mj << t, coef));
        }
    }
    out
}

/// Check F's normal form against S_3 at random grid points.
fn check_f(inst: &Instance, f: &[(u32, u64)]) {
    let (p, t) = (inst.p, inst.t);
    let k = 1u64 << t;
    let mut rng = StdRng::seed_from_u64(7);
    for _ in 0..64 {
        let (i, j) = (rng.gen_range(0..k), rng.gen_range(0..k));
        let yv = |e: u64| -> Vec<u64> {
            let z = powm(inst.zeta, e, p);
            (0..t).map(|l| powm(z, 1u64 << l, p)).collect()
        };
        let (y1, y2) = (yv(i), yv(j));
        let var = |v: usize| if v < t { y1[v] } else { y2[v - t] };
        let mut acc = 0;
        for &(m, c) in f {
            let mut term = c;
            for v in 0..2 * t {
                if m >> v & 1 == 1 {
                    term = mulm(term, var(v), p);
                }
            }
            acc = (acc + term) % p;
        }
        let x1 = mulm(inst.g, powm(inst.zeta, i, p), p);
        let x2 = mulm(inst.g, powm(inst.zeta, j, p), p);
        assert_eq!(
            acc,
            s3_eval(x1, x2, inst.x_r, inst.a, inst.b, p),
            "NF of F is wrong"
        );
    }
}

// ------------------------------------------------------------- closure

const NONE: u32 = u32::MAX;

struct Cols {
    mask: Vec<u32>,
    idx: Vec<u32>,
    mulmap: Vec<u32>,
    first_low: u32,
}

fn build_cols(nv: usize, t: usize, c: u32) -> Cols {
    assert!(nv <= 26);
    let mut masks: Vec<u32> = Vec::new();
    fn rec(start: usize, nv: usize, left: u32, cur: u32, out: &mut Vec<u32>) {
        out.push(cur);
        if left == 0 {
            return;
        }
        for j in start..nv {
            rec(j + 1, nv, left - 1, cur | 1 << j, out);
        }
    }
    rec(0, nv, c, 0, &mut masks);
    masks.sort_unstable_by_key(|&m| (Reverse(m.count_ones()), m));
    let deg: Vec<u8> = masks.iter().map(|m| m.count_ones() as u8).collect();
    let mut idx = vec![NONE; 1 << nv];
    for (i, &m) in masks.iter().enumerate() {
        idx[m as usize] = i as u32;
    }
    let first_low = deg.iter().position(|&d| (d as u32) < c).unwrap() as u32;
    let mut mulmap = vec![NONE; masks.len() * nv];
    for (i, &m) in masks.iter().enumerate() {
        if (deg[i] as u32) < c {
            for v in 0..nv {
                let m2 = mulvar(m, v, t);
                debug_assert!(idx[m2 as usize] != NONE);
                mulmap[i * nv + v] = idx[m2 as usize];
            }
        }
    }
    Cols {
        mask: masks,
        idx,
        mulmap,
        first_low,
    }
}

type Row = Vec<(u32, u32)>;

struct Ech {
    p: u64,
    pivot: Vec<u32>,
    rows: Vec<Row>,
    entries: u64,
}

impl Ech {
    /// Fully reduce `r` (sorted by column) against all pivots; monic result.
    fn reduce(&self, r: &[(u32, u32)], acc: &mut [u64], touched: &mut Vec<u32>) -> Row {
        let p = self.p;
        let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(r.len() * 2);
        for &(col, c) in r {
            if acc[col as usize] == 0 {
                touched.push(col);
                heap.push(Reverse(col));
            }
            acc[col as usize] = (acc[col as usize] + u64::from(c)) % p;
        }
        let mut out: Row = Vec::new();
        while let Some(Reverse(col)) = heap.pop() {
            let v = acc[col as usize];
            if v == 0 {
                continue;
            }
            let piv = self.pivot[col as usize];
            if piv == NONE {
                out.push((col, v as u32));
                acc[col as usize] = 0;
                continue;
            }
            let f = p - v;
            for &(c2, x) in &self.rows[piv as usize] {
                let i = c2 as usize;
                if acc[i] == 0 && c2 != col {
                    touched.push(c2);
                    heap.push(Reverse(c2));
                }
                acc[i] = (acc[i] + f * u64::from(x)) % p;
            }
        }
        for &tt in touched.iter() {
            acc[tt as usize] = 0;
        }
        touched.clear();
        if let Some(&(_, lead)) = out.first() {
            let s = invm(u64::from(lead), p);
            for e in out.iter_mut() {
                e.1 = mulm(u64::from(e.1), s, p) as u32;
            }
        }
        out
    }
}

struct RoundRec {
    r: u32,
    dim: u64,
    added: u64,
    low_added: u64,
    ms: f64,
}

struct Outcome {
    r_star: Option<u32>,
    g_rounds: u32,
    hist: Vec<RoundRec>,
    censored: Option<String>,
    cols: usize,
}

fn run_rounds(inst: &Instance, c: u32, budget_s: f64, max_entries: u64) -> Outcome {
    let (p, t) = (inst.p, inst.t);
    let nv = 2 * t;
    let started = Instant::now();
    let cols = build_cols(nv, t, c);
    let n = cols.mask.len();
    let one_col = (n - 1) as u32;
    let f = build_f(inst);
    check_f(inst, &f);
    let mut ech = Ech {
        p,
        pivot: vec![NONE; n],
        rows: Vec::new(),
        entries: 0,
    };
    let mut acc = vec![0u64; n];
    let mut touched: Vec<u32> = Vec::new();
    let mut hist = Vec::new();
    let mut r_star = None;
    let mut censored = None;
    let mut prev_low: Vec<usize> = Vec::new();

    // W_0
    let deg_f = f.iter().map(|(m, _)| m.count_ones()).max().unwrap();
    if deg_f <= c {
        let mut row: Row = f
            .iter()
            .map(|&(m, co)| (cols.idx[m as usize], co as u32))
            .collect();
        row.sort_unstable_by_key(|x| x.0);
        let red = ech.reduce(&row, &mut acc, &mut touched);
        if let Some(&(lead, _)) = red.first() {
            ech.entries += red.len() as u64;
            ech.pivot[lead as usize] = 0;
            if lead >= cols.first_low {
                prev_low.push(0);
            }
            if lead == one_col {
                r_star = Some(0);
            }
            ech.rows.push(red);
        }
    }
    hist.push(RoundRec {
        r: 0,
        dim: ech.rows.len() as u64,
        added: ech.rows.len() as u64,
        low_added: prev_low.len() as u64,
        ms: 0.0,
    });
    let mut g_rounds = 0u32;
    let mut r = 0u32;
    while r_star.is_none() {
        r += 1;
        let t0 = Instant::now();
        let dim_before = ech.rows.len();
        let mut new_low: Vec<usize> = Vec::new();
        let mut buf: Vec<(u32, u32)> = Vec::new();
        'round: for &ri in &prev_low {
            for v in 0..nv {
                if started.elapsed().as_secs_f64() > budget_s {
                    censored = Some(format!("budget at round {r}"));
                    break 'round;
                }
                buf.clear();
                for &(col, co) in &ech.rows[ri] {
                    let tgt = cols.mulmap[col as usize * nv + v];
                    debug_assert!(tgt != NONE, "row of degree c in the low set");
                    buf.push((tgt, co));
                }
                buf.sort_unstable_by_key(|x| x.0);
                // merge duplicates
                let mut w = 0;
                let mut i = 0;
                while i < buf.len() {
                    let col = buf[i].0;
                    let mut s = 0u64;
                    while i < buf.len() && buf[i].0 == col {
                        s += u64::from(buf[i].1);
                        i += 1;
                    }
                    let s = s % p;
                    if s != 0 {
                        buf[w] = (col, s as u32);
                        w += 1;
                    }
                }
                buf.truncate(w);
                let red = ech.reduce(&buf, &mut acc, &mut touched);
                let Some(&(lead, _)) = red.first() else {
                    continue;
                };
                ech.entries += red.len() as u64;
                ech.pivot[lead as usize] = ech.rows.len() as u32;
                if lead >= cols.first_low {
                    new_low.push(ech.rows.len());
                }
                ech.rows.push(red);
                if lead == one_col {
                    r_star = Some(r);
                    break 'round;
                }
                if ech.entries * 8 > max_entries {
                    censored = Some(format!("max_entries at round {r}"));
                    break 'round;
                }
            }
        }
        let added = (ech.rows.len() - dim_before) as u64;
        hist.push(RoundRec {
            r,
            dim: ech.rows.len() as u64,
            added,
            low_added: new_low.len() as u64,
            ms: t0.elapsed().as_secs_f64() * 1e3,
        });
        eprintln!(
            "  round {r:3} dim {:7} added {:6} low_added {:6} entries {:10} {:.0} ms",
            ech.rows.len(),
            added,
            new_low.len(),
            ech.entries,
            t0.elapsed().as_secs_f64() * 1e3
        );
        if added > 0 {
            g_rounds += 1;
        }
        if censored.is_some() || added == 0 {
            break;
        }
        prev_low = new_low;
    }
    Outcome {
        r_star,
        g_rounds,
        hist,
        censored,
        cols: n,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .map(|i| args[i + 1].clone())
    };
    let p: u64 = get("--p").map(|s| s.parse().unwrap()).unwrap_or(2013265921);
    let t: usize = get("--t").expect("--t").parse().unwrap();
    let c: u32 = get("--c").expect("--c").parse().unwrap();
    let idx: u64 = get("--idx").map(|s| s.parse().unwrap()).unwrap_or(0);
    let planted = args.iter().any(|a| a == "--planted");
    let budget: f64 = get("--budget")
        .map(|s| s.parse().unwrap())
        .unwrap_or(6000.0);
    let max_mb: u64 = get("--max-mb").map(|s| s.parse().unwrap()).unwrap_or(2500);
    let t0 = Instant::now();
    let inst = draw_instance(p, t, idx, planted);
    let t_inst = t0.elapsed().as_secs_f64();
    eprintln!(
        "instance p={p} t={t} idx={idx} planted={planted} tries={} (drawn in {t_inst:.1}s)",
        inst.tries
    );
    let t1 = Instant::now();
    let o = run_rounds(&inst, c, budget, max_mb * 1_000_000);
    let secs = t1.elapsed().as_secs_f64();
    let hist: Vec<String> = o
        .hist
        .iter()
        .map(|h| {
            format!(
                "{{\"r\":{},\"dim\":{},\"added\":{},\"low_added\":{},\"ms\":{:.0}}}",
                h.r, h.dim, h.added, h.low_added, h.ms
            )
        })
        .collect();
    let line = format!(
        "{{\"p\":{},\"t\":{},\"c\":{},\"idx\":{},\"kind\":\"{}\",\"a\":{},\"b\":{},\"g\":{},\"zeta\":{},\"x_r\":{},\"grid_zeros\":{},\"tries\":{},\"cols\":{},\"r_star\":{},\"G\":{},\"rounds\":{},\"final_dim\":{},\"censored\":{},\"inst_secs\":{:.1},\"secs\":{:.1},\"hist\":[{}]}}",
        p, t, c, idx,
        if planted { "planted" } else { "random" },
        inst.a, inst.b, inst.g, inst.zeta, inst.x_r, inst.grid_zeros, inst.tries, o.cols,
        o.r_star.map_or("null".to_string(), |r| r.to_string()),
        o.g_rounds,
        o.hist.len() - 1,
        o.hist.last().unwrap().dim,
        o.censored.as_ref().map_or("null".to_string(), |s| format!("\"{s}\"")),
        t_inst, secs, hist.join(",")
    );
    println!("{line}");
    if let Some(path) = get("--out") {
        let mut fh = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        writeln!(fh, "{line}").unwrap();
    }
}
