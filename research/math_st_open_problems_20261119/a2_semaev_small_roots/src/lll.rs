//! LLL in the L^2 style (Nguyen-Stehle): exact integer basis and exact integer
//! Gram matrix (num-bigint); Gram-Schmidt coefficients in a floating type
//! (fl::D = f64 mantissa + wide exponent, or dd::F = double-double). Lazy size
//! reduction with eta = 0.51, Lovasz condition with delta = 0.99.
//! `lll` tries D first and restarts from the input basis with F on failure.
use crate::dd::F;
use crate::fl::{Fl, D};
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

pub struct LllStats { pub swaps: u64, pub iters: u64, pub precision: &'static str }

fn dot_big(a: &[BigInt], b: &[BigInt]) -> BigInt {
    let mut s = BigInt::zero();
    for i in 0..a.len() {
        if a[i].is_zero() || b[i].is_zero() { continue; }
        s += &a[i] * &b[i];
    }
    s
}

pub fn lll(b: &mut Vec<Vec<BigInt>>, delta: f64) -> Result<LllStats, String> {
    let orig = b.clone();
    match lll_t::<D>(b, delta) {
        Ok(mut s) => { s.precision = "f64"; Ok(s) }
        Err(_) => {
            *b = orig;
            let mut s = lll_t::<F>(b, delta)?;
            s.precision = "dd";
            Ok(s)
        }
    }
}

pub fn lll_t<T: Fl>(b: &mut Vec<Vec<BigInt>>, delta: f64) -> Result<LllStats, String> {
    let n = b.len();
    let mut stats = LllStats { swaps: 0, iters: 0, precision: "" };
    if n <= 1 { return Ok(stats); }
    let d = b[0].len();
    // exact Gram matrix (full, symmetric)
    // lower triangle only: g[i][j] for j <= i
    let mut g: Vec<Vec<BigInt>> = (0..n).map(|i| (0..=i).map(|j| dot_big(&b[i], &b[j])).collect()).collect();
    let mut r = vec![vec![T::zero(); n]; n];
    let mut mu = vec![vec![T::zero(); n]; n];
    let fdelta = T::from_f64(delta);
    r[0][0] = T::from_big(&g[0][0]);
    let mut k = 1usize;
    let max_iters: u64 = 50_000_000;
    let mut xs: Vec<BigInt> = vec![BigInt::zero(); n];
    while k < n {
        stats.iters += 1;
        if stats.iters > max_iters { return Err("LLL iteration cap".into()); }
        let mut rounds = 0;
        let mut last_maxlog = f64::INFINITY;
        loop {
            rounds += 1;
            if rounds > 100 { return Err(format!("size reduction did not converge at k={k}")); }
            for j in 0..k {
                let mut rkj = T::from_big(&g[k][j]);
                for i in 0..j {
                    if mu[j][i].is_zero() || r[k][i].is_zero() { continue; }
                    rkj = rkj.sub(mu[j][i].mul(r[k][i]));
                }
                r[k][j] = rkj;
                mu[k][j] = rkj.div(r[j][j]);
            }
            if (0..k).all(|j| mu[k][j].abs_le_half()) { break; }
            // lazy size reduction: compute all X_j with the float mu, then apply exactly
            let maxlog = (0..k).map(|j| mu[k][j].log2()).fold(f64::NEG_INFINITY, f64::max);
            if rounds > 10 && maxlog >= last_maxlog { return Err("size reduction stalled".into()); }
            last_maxlog = maxlog;
            for j in (0..k).rev() {
                if mu[k][j].abs_le_half() { xs[j] = BigInt::zero(); continue; }
                let x = mu[k][j].round_big();
                let xf = T::from_big(&x);
                for i in 0..j { if !mu[j][i].is_zero() { mu[k][i] = mu[k][i].sub(xf.mul(mu[j][i])); } }
                mu[k][j] = mu[k][j].sub(xf);
                xs[j] = x;
            }
            // apply: b_k -= sum_j X_j b_j ; update Gram
            let (lo, hi) = b.split_at_mut(k);
            let bk = &mut hi[0];
            for j in (0..k).rev() {
                if xs[j].is_zero() { continue; }
                let x = std::mem::take(&mut xs[j]);
                // G_kk <- G_kk - 2 X G_kj + X^2 G_jj   (G_kj current)
                let t = &x * &g[k][j];
                let gkk = &g[k][k] - (&t + &t) + &x * &x * &g[j][j];
                // G_ki -= X G_ji for all i != k, using the lower-triangle layout
                match x.to_i64() {
                    Some(xi) => {
                        for t in 0..d { if !lo[j][t].is_zero() { bk[t] -= &lo[j][t] * xi; } }
                        for i in 0..n { if i == k { continue; } let v = { let w = if i <= j { &g[j][i] } else { &g[i][j] }; if w.is_zero() { continue; } w * xi }; if i < k { g[k][i] -= v; } else { g[i][k] -= v; } }
                    }
                    None => {
                        for t in 0..d { if !lo[j][t].is_zero() { bk[t] -= &lo[j][t] * &x; } }
                        for i in 0..n { if i == k { continue; } let v = { let w = if i <= j { &g[j][i] } else { &g[i][j] }; if w.is_zero() { continue; } w * &x }; if i < k { g[k][i] -= v; } else { g[i][k] -= v; } }
                    }
                }
                g[k][k] = gkk;
            }
        }
        let mut rkk = T::from_big(&g[k][k]);
        for j in 0..k { rkk = rkk.sub(mu[k][j].mul(r[k][j])); }
        r[k][k] = rkk;
        let lhs = fdelta.mul(r[k - 1][k - 1]);
        let rhs = rkk.add(mu[k][k - 1].mul(mu[k][k - 1]).mul(r[k - 1][k - 1]));
        if lhs.gt(rhs) {
            b.swap(k - 1, k);
            swap_lower(&mut g, k - 1, k);
            stats.swaps += 1;
            if k == 1 { r[0][0] = T::from_big(&g[0][0]); } else { k -= 1; }
        } else {
            k += 1;
        }
    }
    Ok(stats)
}

/// Swap indices a < b = a + 1 of a symmetric matrix stored as its lower triangle.
fn swap_lower(g: &mut Vec<Vec<BigInt>>, a: usize, b: usize) {
    debug_assert!(b == a + 1);
    let n = g.len();
    // diagonal
    let (lo, hi) = g.split_at_mut(b);
    std::mem::swap(&mut lo[a][a], &mut hi[0][b]);
    // g[b][a] stays (symmetric pair a,b)
    // columns j < a: g[a][j] <-> g[b][j]
    for j in 0..a { let (lo, hi) = g.split_at_mut(b); std::mem::swap(&mut lo[a][j], &mut hi[0][j]); }
    // rows i > b: g[i][a] <-> g[i][b]
    for i in b + 1..n { g[i].swap(a, b); }
}
