//! **Closed-form vs `gcd(f, L_V mod f)` on binary Semaev `S₄` quartics.**
//!
//! The per-pair step of `semaev_decomp::decompose`, on identical quartics:
//! real `S₄(X₁, X₂, X₃, x_R)` rows (random target, random row `X₁`, every
//! `X₂ ≥ X₁` of the low-order subspace `V`), made monic by one batched
//! inversion a row as `decompose` does (not timed, shared by both arms).
//!
//! * **base** — `QuarticSubspaceRoots::gcd`, the production step.
//! * **radical** — `RadicalQuartic::row`: affine reduction, Dickson cube
//!   root, two half-traces, four batched inversion stages a row, and the
//!   gcd as fallback for quartics the closed form cannot take.
//!
//! Every quartic's subspace-root count is compared between the arms.
//! Both arms run in `pclmulqdq` builds.  Times are the minimum of
//! `REPS` runs.
//!
//! ```bash
//! cargo run --release --example binary_radical_quartic_bench
//! ```
use crypto_lib::cryptanalysis::binary_radical_root::{
    RadicalQuartic, RowCounts, RowRoots, RowScratch,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible;
use crypto_lib::cryptanalysis::semaev_decomp::{quartic_in_x3, Gf2, QuarticSubspaceRoots};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

const BUDGET: usize = 1 << 18;
const REPS: usize = 5;

#[inline(always)]
fn base_body(rows: &[Vec<[u64; 4]>], base: &QuarticSubspaceRoots, gf: &Gf2, degs: &mut Vec<u8>) {
    degs.clear();
    for row in rows {
        for &f in row {
            degs.push(base.gcd(f, gf).deg().unwrap_or(0) as u8);
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn base_clmul(
    rows: &[Vec<[u64; 4]>],
    base: &QuarticSubspaceRoots,
    gf: &Gf2,
    degs: &mut Vec<u8>,
) {
    base_body(rows, base, gf, degs)
}

fn run_base(rows: &[Vec<[u64; 4]>], base: &QuarticSubspaceRoots, gf: &Gf2, degs: &mut Vec<u8>) {
    #[cfg(target_arch = "x86_64")]
    if gf.folds() {
        return unsafe { base_clmul(rows, base, gf, degs) };
    }
    base_body(rows, base, gf, degs)
}

fn run_radical(
    rows: &[Vec<[u64; 4]>],
    r: &RadicalQuartic,
    base: &QuarticSubspaceRoots,
    gf: &Gf2,
    sc: &mut RowScratch,
    all: &mut Vec<RowRoots>,
    counts: &mut RowCounts,
) {
    let mut out = Vec::new();
    all.clear();
    for row in rows {
        r.row(row, base, gf, sc, &mut out, counts);
        all.extend_from_slice(&out);
    }
}

struct Meta {
    xr: u64,
    x1: u64,
    x2s: Vec<u64>,
}

fn rows_for(n: u32, l: u32, gf: &Gf2, seed: u64) -> (Vec<Vec<[u64; 4]>>, Vec<Meta>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let span = 1u64 << l;
    let mut rows = Vec::new();
    let mut metas = Vec::new();
    let mut total = 0;
    let mut scratch = Vec::new();
    while total < BUDGET {
        let xr = rng.gen::<u64>() & ((1u64 << n) - 1);
        let x1 = rng.gen_range(0..span);
        let mut qs = Vec::new();
        let mut leads = Vec::new();
        let mut x2s = Vec::new();
        for x2 in x1..span {
            let q = quartic_in_x3(x1, x2, xr, gf);
            if q.deg() == Some(4) {
                leads.push(q.c[4]);
                qs.push(q);
                x2s.push(x2);
            }
        }
        metas.push(Meta { xr, x1, x2s });
        gf.batch_inv(&mut leads, &mut scratch);
        let row: Vec<[u64; 4]> = qs
            .iter()
            .zip(&leads)
            .map(|(q, &li)| [0, 1, 2, 3].map(|i| gf.mul(q.c[i], li)))
            .collect();
        total += row.len();
        rows.push(row);
    }
    (rows, metas)
}

/// Roots in `V` of `A·X² + B·X + C` (Koblitz `S₃` shape), appended to
/// `out`; `ab_inv = 1/(A·B)` when `A·B ≠ 0`.
#[inline(always)]
fn quad_roots(
    a: u64,
    b: u64,
    c: u64,
    ab_inv: u64,
    r: &RadicalQuartic,
    gf: &Gf2,
    mut keep: impl FnMut(u64),
) {
    // X = (B/A)·t, t² + t = A·C/B².
    let inv_a = gf.mul(b, ab_inv);
    let inv_b = gf.mul(a, ab_inv);
    let w = gf.mul(gf.mul(a, c), gf.sqr(inv_b));
    if r.trace(w) == 0 {
        let k = gf.mul(b, inv_a);
        let x0 = gf.mul(k, r.half_trace(w));
        keep(x0);
        keep(x0 ^ k);
    }
}

#[derive(Default)]
struct FacScratch {
    inv: Vec<u64>,
    tmp: Vec<u64>,
    us: Vec<u64>,
    uowner: Vec<usize>,
}

#[derive(Default, Clone, Copy)]
struct FacCounts {
    pairs: u64,
    irrational_u: u64,
    irrational_u_with_roots: u64,
    degenerate: u64,
}

/// `S₄ = S₃(u, x_R, X)·S₃(u', x_R, X)` with `u, u'` the roots of
/// `S₃(x₁, x₂, ·)`: two batched Artin–Schreier stages per row.  Pairs
/// whose `u` is irrational, or degenerate, go to the gcd.
#[inline(always)]
fn factored_body(
    rows: &[Vec<[u64; 4]>],
    metas: &[Meta],
    l: u32,
    r: &RadicalQuartic,
    base: &QuarticSubspaceRoots,
    gf: &Gf2,
    sc: &mut FacScratch,
    counts_out: &mut Vec<u8>,
    fc: &mut FacCounts,
) {
    counts_out.clear();
    let mut sets: Vec<[u64; 4]> = Vec::new();
    let mut lens: Vec<u8> = Vec::new();
    for (row, m) in rows.iter().zip(metas) {
        let k = row.len();
        fc.pairs += k as u64;
        sets.clear();
        sets.resize(k, [0; 4]);
        lens.clear();
        lens.resize(k, 0);
        // Stage A: u from S₃(x₁, x₂, u) = (x₁+x₂)²u² + x₁x₂·u + x₁²x₂² + 1.
        sc.inv.clear();
        for &x2 in &m.x2s {
            let a1 = gf.sqr(m.x1 ^ x2);
            let b1 = gf.mul(m.x1, x2);
            sc.inv.push(gf.mul(a1, b1));
        }
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        sc.us.clear();
        sc.uowner.clear();
        for (i, &x2) in m.x2s.iter().enumerate() {
            let a1 = gf.sqr(m.x1 ^ x2);
            let b1 = gf.mul(m.x1, x2);
            if a1 == 0 || b1 == 0 {
                fc.degenerate += 1;
                lens[i] = 0xff; // gcd fallback
                continue;
            }
            let c1 = gf.sqr(b1) ^ 1;
            let mut got = 0;
            quad_roots(a1, b1, c1, sc.inv[i], r, gf, |u| {
                sc.us.push(u);
                sc.uowner.push(i);
                got += 1;
            });
            if got == 0 {
                fc.irrational_u += 1;
                lens[i] = 0xfe; // irrational u: gcd, and record roots
            }
        }
        // Stage B: X from S₃(u, x_R, X) for every rational u.
        sc.inv.clear();
        for &u in &sc.us {
            sc.inv.push(gf.mul(gf.sqr(u ^ m.xr), gf.mul(u, m.xr)));
        }
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        for (j, &u) in sc.us.iter().enumerate() {
            let i = sc.uowner[j];
            let a2 = gf.sqr(u ^ m.xr);
            let b2 = gf.mul(u, m.xr);
            if a2 == 0 || b2 == 0 {
                fc.degenerate += 1;
                lens[i] = 0xff;
                continue;
            }
            let c2 = gf.sqr(b2) ^ 1;
            let (set, len) = (&mut sets[i], &mut lens[i]);
            if *len >= 0xfe {
                continue;
            }
            quad_roots(a2, b2, c2, sc.inv[j], r, gf, |x| {
                if x >> l == 0 && !set[..*len as usize].contains(&x) {
                    set[*len as usize] = x;
                    *len += 1;
                }
            });
        }
        for i in 0..k {
            let n = match lens[i] {
                0xff | 0xfe => {
                    let d = base.gcd(row[i], gf).deg().unwrap_or(0) as u8;
                    if lens[i] == 0xfe && d > 0 {
                        fc.irrational_u_with_roots += 1;
                    }
                    d
                }
                n => n,
            };
            counts_out.push(n);
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "pclmulqdq")]
unsafe fn factored_clmul(
    rows: &[Vec<[u64; 4]>],
    metas: &[Meta],
    l: u32,
    r: &RadicalQuartic,
    base: &QuarticSubspaceRoots,
    gf: &Gf2,
    sc: &mut FacScratch,
    counts_out: &mut Vec<u8>,
    fc: &mut FacCounts,
) {
    factored_body(rows, metas, l, r, base, gf, sc, counts_out, fc)
}

fn main() {
    let ns: Vec<u32> = vec![23, 29, 31, 37, 41, 47, 53, 59, 61];
    let ls: Vec<u32> = vec![6, 8, 10, 12, 14];
    println!(
        "{:>3} {:>3} {:>8} {:>9} {:>9} {:>7} | {:>6} {:>6} {:>6} {:>6} | agree",
        "n", "l", "quartics", "base ns", "rad ns", "speedup", "closed", "biject", "f3=0", "other"
    );
    let mut fac_lines: Vec<String> = Vec::new();
    for &n in &ns {
        let gf = Gf2::new(&find_irreducible(n).expect("irreducible"));
        for &l in &ls {
            if l >= n {
                continue;
            }
            let Some(r) = RadicalQuartic::new(l, &gf) else {
                println!("{n:>3} {l:>3}  (closed form needs odd n)");
                continue;
            };
            let base = QuarticSubspaceRoots::new(l, &gf);
            let (rows, metas) = rows_for(n, l, &gf, 0x5eed ^ ((n as u64) << 8) ^ l as u64);
            let quartics: usize = rows.iter().map(Vec::len).sum();

            let mut degs = Vec::with_capacity(quartics);
            let mut best_base = f64::MAX;
            for _ in 0..REPS {
                let t = Instant::now();
                run_base(&rows, &base, &gf, &mut degs);
                best_base = best_base.min(t.elapsed().as_secs_f64());
            }
            let mut sc = RowScratch::default();
            let mut all = Vec::with_capacity(quartics);
            let mut best_rad = f64::MAX;
            let mut counts = RowCounts::default();
            for rep in 0..REPS {
                let mut c = RowCounts::default();
                let t = Instant::now();
                run_radical(&rows, &r, &base, &gf, &mut sc, &mut all, &mut c);
                best_rad = best_rad.min(t.elapsed().as_secs_f64());
                if rep == 0 {
                    counts = c;
                }
            }

            let mut fsc = FacScratch::default();
            let mut fcounts_out = Vec::with_capacity(quartics);
            let mut best_fac = f64::MAX;
            let mut fc = FacCounts::default();
            for rep in 0..REPS {
                let mut c = FacCounts::default();
                let t = Instant::now();
                #[cfg(target_arch = "x86_64")]
                unsafe {
                    factored_clmul(
                        &rows,
                        &metas,
                        l,
                        &r,
                        &base,
                        &gf,
                        &mut fsc,
                        &mut fcounts_out,
                        &mut c,
                    )
                };
                #[cfg(not(target_arch = "x86_64"))]
                factored_body(
                    &rows,
                    &metas,
                    l,
                    &r,
                    &base,
                    &gf,
                    &mut fsc,
                    &mut fcounts_out,
                    &mut c,
                );
                best_fac = best_fac.min(t.elapsed().as_secs_f64());
                if rep == 0 {
                    fc = c;
                }
            }
            let fac_agree = fcounts_out == degs;
            fac_lines.push(format!(
                "{n:>3} {l:>3} {:>9.1} {:>9.1} {:>6.2}x | irr-u {:>5.1}% (with roots: {}) degen {:>5.2}% | {}",
                1e9 * best_base / quartics as f64,
                1e9 * best_fac / quartics as f64,
                best_base / best_fac,
                100.0 * fc.irrational_u as f64 / fc.pairs as f64,
                fc.irrational_u_with_roots,
                100.0 * fc.degenerate as f64 / fc.pairs as f64,
                if fac_agree { "agree" } else { "MISMATCH" }
            ));

            // Agreement: subspace-root counts, and every listed root is one.
            let mut agree = true;
            let flat: Vec<&[u64; 4]> = rows.iter().flatten().collect();
            for (k, (rr, &d)) in all.iter().zip(&degs).enumerate() {
                let ok = if rr.multi_gcd {
                    d >= 2
                } else {
                    rr.count() == d as usize
                };
                let roots_ok = rr.roots[..rr.count()].iter().all(|&x| {
                    x == u64::MAX || {
                        let f = flat[k];
                        let mut v = 1u64;
                        for c in f.iter().rev() {
                            v = gf.mul(v, x) ^ c;
                        }
                        v == 0
                    }
                });
                if !(ok && roots_ok) {
                    agree = false;
                    eprintln!("mismatch n={n} l={l} k={k} gcd_deg={d} radical={rr:?}");
                    break;
                }
            }
            let q = counts.quartics as f64;
            let pct = |x: u64| 100.0 * x as f64 / q;
            let other = counts.shift_root + counts.alpha_zero + counts.no_exponent;
            println!(
                "{n:>3} {l:>3} {quartics:>8} {:>9.1} {:>9.1} {:>6.2}x | {:>5.1}% {:>5.1}% {:>5.2}% {:>5.2}% | {}",
                1e9 * best_base / quartics as f64,
                1e9 * best_rad / quartics as f64,
                best_base / best_rad,
                pct(counts.closed_form),
                pct(counts.bijective),
                pct(counts.f3_zero),
                pct(other),
                if agree { "yes" } else { "NO" }
            );
        }
    }
    println!("\nfactored S₄ = S₃(u)·S₃(u'):");
    println!(
        "{:>3} {:>3} {:>9} {:>9} {:>7} |",
        "n", "l", "base ns", "fac ns", "speedup"
    );
    for line in fac_lines {
        println!("{line}");
    }
}
