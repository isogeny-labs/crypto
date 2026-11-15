//! Native, independent re-check of every arm of a cell file.
//!
//! cmsem verify <cell-file> [source_of_iso=ARM:SRC,...]
//! Checks per arm: nonsingular; group order equals the cell's group order
//! (prime-order cell: n prime, n > width of the Hasse interval, n*P = O for
//! 8 random points; side cell: n' = N/4 prime, N*P = O and 4P != O); the
//! Montgomery data (rhs(alpha) = 0, s^2 = 3 alpha^2 + a) for MONT/EDY arms;
//! j-invariant equality for the listed isomorphic pairs.
use crate::nt::{is_prime, isqrt, mulmod, powmod};
use crate::{Curve, Fp, Rng};

fn jinv(a: u64, b: u64, p: u64) -> u64 {
    let a3 = mulmod(mulmod(a, a, p), a, p);
    let num = mulmod(1728 * 4 % p, a3, p);
    let den = (mulmod(4, a3, p) + mulmod(27, mulmod(b, b, p), p)) % p;
    mulmod(num, powmod(den, p - 2, p), p)
}

pub fn cmd_verify(args: &[String]) {
    let text = std::fs::read_to_string(&args[0]).unwrap();
    let pairs: Vec<(String, String)> = args
        .get(1)
        .map(|s| s.split(',').filter(|x| !x.is_empty()).map(|x| {
            let mut it = x.split(':');
            (it.next().unwrap().to_string(), it.next().unwrap().to_string())
        }).collect())
        .unwrap_or_default();
    let mut p = 0u64;
    let mut ng = 0u64;
    let mut out = vec![];
    let mut all_ok = true;
    let mut ab: Vec<(String, u64, u64)> = vec![];
    for l in text.lines() {
        let w: Vec<&str> = l.split_whitespace().collect();
        if w.is_empty() {
            continue;
        }
        if w[0] == "cell" {
            p = w[2].parse().unwrap();
            ng = w[3].parse().unwrap();
            continue;
        }
        let name = w[1];
        let a: u64 = w[2].parse().unwrap();
        let b: u64 = w[3].parse().unwrap();
        ab.push((name.into(), a, b));
        let f = Fp::new(p);
        let c = Curve { f, a: f.to_m(a), b: f.to_m(b) };
        let mut rng = Rng(crate::fnv(&format!("verify/{}", name)));
        let disc = (mulmod(4, mulmod(mulmod(a, a, p), a, p), p) + mulmod(27, mulmod(b, b, p), p)) % p;
        let width = 4 * isqrt(p as u128) as u64 + 4;
        let hasse_ok = ng + 2 * (isqrt(p as u128) as u64) + 2 >= p + 1 && ng <= p + 1 + 2 * (isqrt(p as u128) as u64) + 2;
        let (order_ok, kind) = if is_prime(ng) {
            let mut ok = ng > width && disc != 0 && hasse_ok;
            for _ in 0..8 {
                let pt = c.random_point(&mut rng);
                ok &= c.mul_is_inf(ng, pt);
            }
            (ok, "prime order n: n > Hasse width and n*P = O for 8 random P")
        } else {
            let np = ng / 4;
            let mut ok = ng % 4 == 0 && is_prime(np) && np > width && disc != 0 && hasse_ok;
            let mut seen_big = false;
            for _ in 0..8 {
                let pt = c.random_point(&mut rng);
                ok &= c.mul_is_inf(ng, pt);
                if !c.mul_is_inf(4, pt) {
                    seen_big = true;
                }
            }
            (ok && seen_big, "N = 4n', n' prime > Hasse width, N*P = O and 4P != O")
        };
        let mut mont_ok = true;
        if w[4] != "X" {
            let alpha: u64 = w[5].parse().unwrap();
            let s: u64 = w[6].parse().unwrap();
            let rhs = (mulmod(mulmod(alpha, alpha, p), alpha, p) + mulmod(a, alpha, p) + b) % p;
            let s2 = mulmod(s, s, p);
            let want = (mulmod(3, mulmod(alpha, alpha, p), p) + a) % p;
            mont_ok = rhs == 0 && s2 == want;
        }
        let ok = order_ok && mont_ok;
        all_ok &= ok;
        out.push(format!("{{\"arm\":\"{}\",\"order_ok\":{},\"order_check\":\"{}\",\"model_ok\":{},\"j\":{}}}", name, order_ok, kind, mont_ok, jinv(a, b, p)));
    }
    let mut iso_out = vec![];
    for (x, y) in pairs.iter() {
        let fx = ab.iter().find(|r| &r.0 == x).unwrap();
        let fy = ab.iter().find(|r| &r.0 == y).unwrap();
        let eq = jinv(fx.1, fx.2, p) == jinv(fy.1, fy.2, p);
        all_ok &= eq;
        iso_out.push(format!("{{\"arm\":\"{}\",\"source\":\"{}\",\"same_j\":{}}}", x, y, eq));
    }
    println!("{{\"cell_file\":\"{}\",\"p\":{},\"group_order\":{},\"all_ok\":{},\"arms\":[{}],\"iso_pairs\":[{}]}}", args[0], p, ng, all_ok, out.join(","), iso_out.join(","));
}
