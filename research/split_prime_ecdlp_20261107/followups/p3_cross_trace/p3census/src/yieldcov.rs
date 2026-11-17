//! `yield`: exact m = 2 decomposition coverage of a prime-abscissa factor base.
//!
//! B = first N points whose abscissa is a prime x = 2, 3, 5, ... lying on E (one y
//! each). S = { +-B_i +- B_j : i <= j } (as a set of group elements, O included
//! via B_i - B_i). coverage = |S| / n. |S| is counted by x-class: S is closed
//! under negation, n is odd so no element has y = 0, hence
//! |S| = 2 * #(distinct x among the non-identity elements) + 1.
//! lambda = 2 N^2 / n is the no-collision-loss expectation used to normalise.
use crate::curves::{load, CurveRec};
use crate::ec::*;
use crate::util::*;

pub struct Cov {
    pub distinct_x: u64,
    pub size: u64,
    pub coverage: f64,
    pub lambda: f64,
    pub cov_norm: f64,
    pub secs: f64,
}

pub fn base_points(c: &Curve, nbase: usize) -> Result<Vec<(u64, u64)>, String> {
    let mut lim = (nbase * 24 + 1000).min(c.p as usize - 1);
    loop {
        let pr = primes_up_to(lim);
        let mut v = vec![];
        for &x in &pr {
            if x >= c.p {
                break;
            }
            if let Some(y) = sqrt_mod(c.rhs(x), c.p) {
                if y != 0 {
                    v.push((x, y));
                    if v.len() == nbase {
                        return Ok(v);
                    }
                }
            }
        }
        if lim >= c.p as usize - 1 {
            return Err(format!("only {} prime abscissae on the curve below p, need {}", v.len(), nbase));
        }
        lim = (lim * 2).min(c.p as usize - 1);
    }
}

pub fn coverage(c: &Curve, n: u64, nbase: usize) -> Result<Cov, String> {
    let t0 = std::time::Instant::now();
    let p = c.p;
    let bp = base_points(c, nbase)?;
    let mut xs: Vec<u32> = Vec::with_capacity(nbase * nbase);
    let mut dens: Vec<u64> = vec![];
    let mut pre: Vec<u64> = vec![];
    let mut o = 0u64;
    for i in 0..nbase {
        let (xi, yi) = bp[i];
        // 2 B_i  (B_i - B_i = O is accounted by the +1)
        if let Some((x3, _)) = c.dbl(&mut o, Some((xi, yi))) {
            xs.push(x3 as u32);
        }
        let rest = &bp[i + 1..];
        if rest.is_empty() {
            continue;
        }
        dens.clear();
        pre.clear();
        let mut acc = 1u64;
        for &(xj, _) in rest {
            let d = (xj + p - xi) % p;
            dens.push(d);
            acc = acc * d % p;
            pre.push(acc);
        }
        let mut inv_acc = inv_p(acc, p);
        for k in (0..rest.len()).rev() {
            let prev = if k == 0 { 1 } else { pre[k - 1] };
            let inv = inv_acc * prev % p;
            inv_acc = inv_acc * dens[k] % p;
            let (xj, yj) = rest[k];
            // B_i + B_j
            let l1 = (yj + p - yi) % p * inv % p;
            xs.push(((l1 * l1 % p + 2 * p - xi - xj) % p) as u32);
            // B_i - B_j = B_i + (xj, -yj)
            let l2 = (2 * p - yj - yi) % p * inv % p;
            xs.push(((l2 * l2 % p + 2 * p - xi - xj) % p) as u32);
        }
    }
    xs.sort_unstable();
    xs.dedup();
    let distinct = xs.len() as u64;
    let size = 2 * distinct + 1;
    let coverage = size as f64 / n as f64;
    let lambda = 2.0 * (nbase as f64) * (nbase as f64) / n as f64;
    Ok(Cov { distinct_x: distinct, size, coverage, lambda, cov_norm: coverage / lambda, secs: t0.elapsed().as_secs_f64() })
}

pub fn cmd(args: &[String]) {
    let curves = load(&opt(args, "--curves").expect("--curves"));
    let nb = opt_u64(args, "--N", None) as usize;
    let mut sink = Sink::open(opt(args, "--out"));
    let t0 = std::time::Instant::now();
    for cr in &curves {
        let CurveRec { idx, c, n, .. } = *cr;
        let cv = coverage(&c, n, nb).unwrap_or_else(|e| panic!("curve {idx}: {e}"));
        sink.line(&format!(
            "{{\"kind\":\"yield\",\"idx\":{},\"n\":{},\"N\":{},\"distinct_x\":{},\"size\":{},\"coverage\":{:.8},\"lambda\":{:.8},\"cov_norm\":{:.8},\"secs\":{:.4}}}",
            idx, n, nb, cv.distinct_x, cv.size, cv.coverage, cv.lambda, cv.cov_norm, cv.secs
        ));
    }
    sink.flush();
    eprintln!("yield: {} curves, N={} in {:.2}s", curves.len(), nb, t0.elapsed().as_secs_f64());
}
