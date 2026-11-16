//! P3 fallback of `research/prime_s5_small_root_leads_20261028/`: an exact
//! search over triangular Coppersmith shift lattices for a polynomial whose
//! Newton polytope is the box `[0, d]^m` (`S_{m+1}` in the `x_i`, premise P2)
//! or the simplex `{|β| ≤ D}` with weights `w_i = i` (its symmetrisation in
//! the `e_i`).  This is **our** search, not the linear program of Ding et
//! al. (ePrint 2026/1027), which could not be retrieved.
//!
//! A lattice is a monomial set `M = {β ≥ 0 : ⟨c_j, β⟩ ≤ s_j}` and a leading
//! vertex `L` of the Newton polytope `N`.  The row for `β ∈ M` is
//! `p^{ℓ−k}·x^{β−kL}·f^k` with `k(β)` the largest `k ≤ ℓ` such that
//! `γ = β − kL ≥ 0` and `γ + k·N ⊂ M`, so every row stays inside `M` and the
//! rows are triangular in any order refining a functional maximised at `L`.
//! The determinant condition with root bounds `X_i = p^{δ w_i}` gives
//!
//! ```text
//! δ(M, L, ℓ) = Σ_β k(β) / Σ_β ⟨w, β⟩,
//! ```
//!
//! dimension factors ignored, as in README §3.2.  The search enumerates
//! rectangles, rectangles cut by `|β| ≤ s`, and rectangles cut by one
//! half-space with coefficients in `{0, 1, 2}`, for every nonzero vertex
//! `L`, and prints the best `δ` per (shape, ℓ) next to the Jochemsz–May
//! value of §3.2.  One JSON line per (shape, m, ℓ).
//!
//! ```text
//! cargo run --release --example semaev_shift_search -- --m 3 --levels 1,2,3,4,6,8
//! ```

use std::cell::RefCell;
use std::time::Instant;

thread_local! {
    /// Number of per-coordinate cap extensions tried (`--extras`).
    static EXTRAS: RefCell<i64> = const { RefCell::new(3) };
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Box,
    Simplex,
}

struct Problem {
    m: usize,
    shape: Shape,
    /// The degree `d = D = 2^{m−1}`.
    d: i64,
    vertices: Vec<Vec<i64>>,
    w: Vec<i64>,
}

impl Problem {
    fn new(m: usize, shape: Shape) -> Self {
        let d = 1i64 << (m - 1);
        let vertices = match shape {
            Shape::Box => (0..1usize << m)
                .map(|mask| {
                    (0..m)
                        .map(|i| if mask >> i & 1 == 1 { d } else { 0 })
                        .collect()
                })
                .collect(),
            Shape::Simplex => std::iter::once(vec![0; m])
                .chain((0..m).map(|i| (0..m).map(|j| if i == j { d } else { 0 }).collect()))
                .collect(),
        };
        let w = match shape {
            Shape::Box => vec![1; m],
            Shape::Simplex => (1..=m as i64).collect(),
        };
        Self {
            m,
            shape,
            d,
            vertices,
            w,
        }
    }
}

/// `M = {β ≥ 0 : ⟨c_j, β⟩ ≤ s_j}`, bounded (it always contains a box cap).
struct MonoSet {
    halfspaces: Vec<(Vec<i64>, i64)>,
    caps: Vec<i64>,
}

fn dot(a: &[i64], b: &[i64]) -> i64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

/// `δ(M, L, ℓ)` and `|M|`; `None` if `M` is empty or the ratio has no rows
/// with `k > 0`.
fn evaluate(pr: &Problem, set: &MonoSet, lead: &[i64], level: i64) -> Option<(f64, usize)> {
    // max over N of ⟨c, ·⟩, per half-space.
    let hmax: Vec<i64> = set
        .halfspaces
        .iter()
        .map(|(c, _)| pr.vertices.iter().map(|v| dot(c, v)).max().unwrap())
        .collect();
    let mut beta = vec![0i64; pr.m];
    let (mut num, mut den, mut size) = (0i64, 0i64, 0usize);
    loop {
        if set.halfspaces.iter().all(|(c, s)| dot(c, &beta) <= *s) {
            size += 1;
            den += dot(&pr.w, &beta);
            // Largest feasible k ≤ ℓ; feasibility is monotone in k.
            let mut k = 0;
            for kk in (1..=level).rev() {
                let gamma: Vec<i64> = beta.iter().zip(lead).map(|(b, l)| b - kk * l).collect();
                if gamma.iter().any(|&g| g < 0) {
                    continue;
                }
                if set
                    .halfspaces
                    .iter()
                    .zip(&hmax)
                    .all(|((c, s), &h)| dot(c, &gamma) + kk * h <= *s)
                {
                    k = kk;
                    break;
                }
            }
            num += k;
        }
        // Next β in the cap box.
        let mut i = 0;
        loop {
            if i == pr.m {
                return (num > 0 && den > 0).then(|| (num as f64 / den as f64, size));
            }
            beta[i] += 1;
            if beta[i] <= set.caps[i] {
                break;
            }
            beta[i] = 0;
            i += 1;
        }
    }
}

/// Jochemsz–May level-`ℓ` values of README §3.2.
fn jm(pr: &Problem, level: i64) -> f64 {
    let m = pr.m as i64;
    let d = pr.d;
    match pr.shape {
        Shape::Box => {
            let num: f64 = (0..level)
                .map(|i| ((d * i + 1) as f64).powi(m as i32))
                .sum();
            num / (m as f64 * ((d * level + 1) as f64).powi(m as i32) * (d * level) as f64 / 2.0)
        }
        Shape::Simplex => {
            let c = |n: i64, k: i64| (0..k).fold(1f64, |a, i| a * (n - i) as f64 / (i + 1) as f64);
            let num: f64 = (0..level).map(|i| c(d * i + m, m)).sum();
            num / ((m * (m + 1) / 2) as f64 * c(d * level + m, m + 1))
        }
    }
}

fn candidates(pr: &Problem, level: i64) -> Vec<MonoSet> {
    let m = pr.m;
    let d = pr.d;
    let base = d * level;
    let mut out = Vec::new();
    let unit = |i: usize| (0..m).map(|j| (i == j) as i64).collect::<Vec<i64>>();
    // Extra room per coordinate, shared pattern: 0..=d in steps of d/2.
    let step = (d / 2).max(1);
    let extras: Vec<i64> = (0..EXTRAS.with(|e| *e.borrow()))
        .map(|t| t * step)
        .collect();
    let mut ext = vec![0usize; m];
    loop {
        let caps: Vec<i64> = ext.iter().map(|&e| base + extras[e]).collect();
        let rect: Vec<(Vec<i64>, i64)> = (0..m).map(|i| (unit(i), caps[i])).collect();
        out.push(MonoSet {
            halfspaces: rect.clone(),
            caps: caps.clone(),
        });
        // Cut by |β| ≤ s.
        let total: i64 = caps.iter().sum();
        for frac in [0, 1, 2, 3, 4, 5, 6] {
            let s = base + (total - base) * frac / 8;
            let mut hs = rect.clone();
            hs.push((vec![1; m], s));
            out.push(MonoSet {
                halfspaces: hs,
                caps: caps.clone(),
            });
        }
        // Cut by one half-space with coefficients in {0, 1, 2}.
        let mut c = vec![0i64; m];
        loop {
            let mut i = 0;
            while i < m {
                c[i] += 1;
                if c[i] <= 2 {
                    break;
                }
                c[i] = 0;
                i += 1;
            }
            if i == m {
                break;
            }
            if c.iter().filter(|&&x| x > 0).count() < 2 {
                continue;
            }
            let full = dot(&c, &caps);
            for frac in [4, 6] {
                let mut hs = rect.clone();
                hs.push((c.clone(), full * frac / 8 + base / 2));
                out.push(MonoSet {
                    halfspaces: hs,
                    caps: caps.clone(),
                });
            }
        }
        // Next extra pattern.
        let mut i = 0;
        loop {
            if i == m {
                return out;
            }
            ext[i] += 1;
            if ext[i] < extras.len() {
                break;
            }
            ext[i] = 0;
            i += 1;
        }
    }
}

/// The (dimension, δ) points no smaller set beats, δ to 5 places.
fn frontier(all: &[(f64, usize)]) -> Vec<(usize, f64)> {
    let mut v: Vec<(usize, f64)> = all.iter().map(|&(d, s)| (s, d)).collect();
    v.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.total_cmp(&a.1)));
    let mut out: Vec<(usize, f64)> = Vec::new();
    let mut top = 0f64;
    for (s, d) in v {
        if d > top + 1e-12 {
            top = d;
            out.push((s, (d * 1e5).round() / 1e5));
        }
    }
    out
}

fn arg<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let m: usize = arg(&args, "--m", "3").parse().unwrap();
    EXTRAS.with(|e| *e.borrow_mut() = arg(&args, "--extras", "3").parse().unwrap());
    let levels: Vec<i64> = arg(&args, "--levels", "1,2,3,4")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    for shape in [Shape::Simplex, Shape::Box] {
        let pr = Problem::new(m, shape);
        let leads: Vec<Vec<i64>> = pr
            .vertices
            .iter()
            .filter(|v| v.iter().any(|&x| x != 0))
            .cloned()
            .collect();
        for &level in &levels {
            let t = Instant::now();
            let sets = candidates(&pr, level);
            let mut best = (0f64, 0usize, String::new(), Vec::new());
            let mut jm_shape = 0f64;
            let mut all: Vec<(f64, usize)> = Vec::new();
            let mut jm_dim = 0usize;
            for set in &sets {
                for lead in &leads {
                    if let Some((d, size)) = evaluate(&pr, set, lead, level) {
                        all.push((d, size));
                        if d > best.0 {
                            let desc = set
                                .halfspaces
                                .iter()
                                .map(|(c, s)| format!("{c:?}<={s}"))
                                .collect::<Vec<_>>()
                                .join(" & ");
                            best = (d, size, desc, lead.clone());
                        }
                    }
                }
            }
            // The JM monomial set itself, evaluated by the same code.
            let jm_set = match shape {
                Shape::Box => MonoSet {
                    halfspaces: (0..m)
                        .map(|i| ((0..m).map(|j| (i == j) as i64).collect(), pr.d * level))
                        .collect(),
                    caps: vec![pr.d * level; m],
                },
                Shape::Simplex => MonoSet {
                    halfspaces: vec![(vec![1; m], pr.d * level)],
                    caps: vec![pr.d * level; m],
                },
            };
            let jm_lead = match shape {
                Shape::Box => vec![pr.d; m],
                Shape::Simplex => (0..m).map(|j| if j == m - 1 { pr.d } else { 0 }).collect(),
            };
            if let Some((d, size)) = evaluate(&pr, &jm_set, &jm_lead, level) {
                jm_shape = d;
                jm_dim = size;
            }
            println!(
                "{{\"shape\":\"{}\",\"m\":{m},\"level\":{level},\"candidates\":{},\"best_delta\":{:.6},\"best_dim\":{},\"best_set\":\"{}\",\"best_lead\":\"{:?}\",\"jm_set_delta\":{:.6},\"jm_dim\":{},\"best_at_jm_dim\":{:.6},\"jm_formula\":{:.6},\"asymptote\":{:.6},\"frontier\":{:?},\"ms\":{}}}",
                if shape == Shape::Box { "box" } else { "simplex" },
                sets.len() * leads.len(),
                best.0,
                best.1,
                best.2,
                best.3,
                jm_shape,
                jm_dim,
                all.iter().filter(|x| x.1 <= jm_dim).map(|x| x.0).fold(0f64, f64::max),
                jm(&pr, level),
                2.0 / ((m * (m + 1)) as f64 * (1u64 << (m - 1)) as f64),
                frontier(&all),
                t.elapsed().as_millis()
            );
        }
    }
}
