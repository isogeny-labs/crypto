//! L4-A of `research/prime_s5_small_root_leads_20261028/`: the measured
//! small-root reach of symmetric Jochemsz–May lattices for `S₄` (`m = 3`)
//! and `S₅` (`m = 4`) over a prime field.
//!
//! For a target abscissa `z`, `S_{m+1}(x₁, …, x_m; z)` is symmetric in the
//! `x_i`, so it is a polynomial `g(e₁, …, e_m)` in the elementary symmetric
//! functions of e-degree at most `D = 2^{m−1}`.  `g` is found by a linear
//! solve: its coefficients on the `C(D + m, m)` monomials `e^β` are fitted to
//! values of `S_{m+1}` at random points, and extra points check the fit.
//!
//! The level-`ℓ` lattice has one row per `e^β` with `|β| ≤ Dℓ`:
//! `p^{ℓ−k}·e^γ·g^k` with `k = ⌊β_m / D⌋` and `γ = β − kD·ε_m`, `g` made
//! monic in `e_m^D`.  Rows are triangular in the order (β_m, rest), and
//! column `β` is scaled by `Π W_i^{β_i}`, `W_i = C(m, i)·B^i`, the bound on
//! `e_i` for `x_i ∈ [0, B]`.  After LLL a row is read back as a polynomial
//! `h` and evaluated over ℤ at the planted `e*`; `hg_holds` records whether
//! the first row is below Howgrave-Graham's `p^ℓ/√dim`.
//!
//! Planted targets take random `x_i ∈ [1, B]` (all nonzero, so `e* ≠ 0`) and a
//! root `z ∈ F_p` of `S_{m+1}(x; Z)`; unplanted ones take a random `z`.  Unless
//! `--allow-global` is given, a planted `z` with `S_{m+1}(x; z) = 0` over ℤ
//! (tested modulo `2^61 − 1`) is redrawn: such global relations, e.g. the
//! trivial `P − P + Q = Q`, are found by any lattice that contains `S_{m+1}`.
//! A cell with no non-global root in 500 draws prints a `skipped` line.  One JSON line per
//! trial.
//!
//! ```text
//! cargo run --release --example semaev_lattice_reach -- --p 1099511627563 --a -3 --b 56 \
//!     --m 3 --levels 1,2 --bounds 1,2,3,4 --trials 8 --seed 20261028
//! ```

use crypto_lib::cryptanalysis::lattice::lll_reduce;
use crypto_lib::cryptanalysis::semaev_prime_poly::{
    interpolate, poly_div_exact, poly_gcd, poly_powmod, trim, Curve, Fp,
};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

/// Exponent vectors `β ∈ ℕ^m` with `|β| ≤ deg`, sorted by `(β_m, β)`.
fn monomials(m: usize, deg: usize) -> Vec<Vec<usize>> {
    fn rec(m: usize, left: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == m {
            out.push(cur.clone());
            return;
        }
        for e in 0..=left {
            cur.push(e);
            rec(m, left - e, cur, out);
            cur.pop();
        }
    }
    let mut out = Vec::new();
    rec(m, deg, &mut Vec::new(), &mut out);
    out.sort_by(|a, b| (a[m - 1], a).cmp(&(b[m - 1], b)));
    out
}

fn elementary(f: Fp, xs: &[u64]) -> Vec<u64> {
    // e[i] for i = 1..m, by the product Π (1 + x_j t).
    let mut e = vec![1u64];
    for &x in xs {
        let mut next = vec![0u64; e.len() + 1];
        for (i, &c) in e.iter().enumerate() {
            next[i] = f.add(next[i], c);
            next[i + 1] = f.add(next[i + 1], f.mul(c, x));
        }
        e = next;
    }
    e[1..].to_vec()
}

fn mono_eval(f: Fp, e: &[u64], beta: &[usize]) -> u64 {
    beta.iter()
        .zip(e)
        .fold(1, |acc, (&b, &v)| f.mul(acc, f.pow(v, b as u64)))
}

/// Solve `A x = y` mod p for square nonsingular `A`; `None` if singular.
fn solve(f: Fp, mut a: Vec<Vec<u64>>, mut y: Vec<u64>) -> Option<Vec<u64>> {
    let n = a.len();
    for c in 0..n {
        let piv = (c..n).find(|&r| a[r][c] != 0)?;
        a.swap(piv, c);
        y.swap(piv, c);
        let inv = f.inv(a[c][c]);
        for k in c..n {
            a[c][k] = f.mul(a[c][k], inv);
        }
        y[c] = f.mul(y[c], inv);
        for r in 0..n {
            if r != c && a[r][c] != 0 {
                let t = a[r][c];
                for k in c..n {
                    let s = f.mul(t, a[c][k]);
                    a[r][k] = f.sub(a[r][k], s);
                }
                y[r] = f.sub(y[r], f.mul(t, y[c]));
            }
        }
    }
    Some(y)
}

/// `S_{m+1}(·; z)` as `g(e)` on `monomials(m, D)`; `None` if the fit fails
/// its check points (the symmetric support assumption would be wrong).
fn symmetrise(c: &Curve, m: usize, z: u64, rng: &mut StdRng) -> Option<Vec<u64>> {
    let f = c.f;
    let d = 1usize << (m - 1);
    let mons = monomials(m, d);
    let n = mons.len();
    let check = 6;
    let pts: Vec<Vec<u64>> = (0..n + check)
        .map(|_| (0..m).map(|_| rng.gen_range(0..f.p)).collect())
        .collect();
    let rows: Vec<Vec<u64>> = pts
        .iter()
        .map(|x| {
            let e = elementary(f, x);
            mons.iter().map(|b| mono_eval(f, &e, b)).collect()
        })
        .collect();
    let vals: Vec<u64> = pts.iter().map(|x| c.summation(x, z)).collect();
    let g = solve(f, rows[..n].to_vec(), vals[..n].to_vec())?;
    for i in n..n + check {
        let v = rows[i]
            .iter()
            .zip(&g)
            .fold(0, |acc, (&r, &gc)| f.add(acc, f.mul(r, gc)));
        if v != vals[i] {
            return None;
        }
    }
    Some(g)
}

/// Some root of the univariate `q` in `F_p` (Cantor–Zassenhaus).
fn some_root(f: Fp, q: &[u64], rng: &mut StdRng) -> Option<u64> {
    let q = trim(q.to_vec());
    if q.len() < 2 {
        return None;
    }
    let mut xp = poly_powmod(f, &[0, 1], f.p, &q);
    if xp.len() < 2 {
        xp.resize(2, 0);
    }
    xp[1] = f.sub(xp[1], 1);
    let mut h = poly_gcd(f, &trim(xp), &q);
    while h.len() > 2 {
        let t = rng.gen_range(0..f.p);
        let mut w = poly_powmod(f, &[t, 1], (f.p - 1) / 2, &h);
        w[0] = f.sub(w[0], 1);
        let s = poly_gcd(f, &trim(w), &h);
        if s.len() > 1 && s.len() < h.len() {
            h = if s.len() <= h.len() - s.len() + 1 {
                s
            } else {
                poly_div_exact(f, &h, &s)
            };
        }
    }
    if h.len() == 2 {
        Some(f.mul(f.neg(h[0]), f.inv(h[1])))
    } else {
        None
    }
}

fn binom(n: usize, k: usize) -> u64 {
    (0..k).fold(1u64, |acc, i| acc * (n - i) as u64 / (i + 1) as u64)
}

struct Lattice {
    mons: Vec<Vec<usize>>,
    weights: Vec<BigInt>,
    rows: Vec<Vec<BigInt>>,
    modulus: BigInt,
}

fn mono_index(mons: &[Vec<usize>], beta: &[usize]) -> usize {
    mons.binary_search_by(|a| (a[a.len() - 1], a.as_slice()).cmp(&(beta[beta.len() - 1], beta)))
        .expect("monomial in set")
}

fn build(m: usize, level: usize, g: &[u64], p: u64, bound: u64) -> Lattice {
    let f = Fp { p };
    let d = 1usize << (m - 1);
    let gm = monomials(m, d);
    // Monic in e_m^D.
    let mut lead = vec![0usize; m];
    lead[m - 1] = d;
    let li = mono_index(&gm, &lead);
    let inv = f.inv(g[li]);
    let gmon: Vec<u64> = g.iter().map(|&c| f.mul(c, inv)).collect();
    let mons = monomials(m, d * level);
    let modulus = BigInt::from(p).pow(level as u32);
    let pb = BigInt::from(p);
    // Powers g^k, k = 0..=level, on `mons`, reduced mod p^level.
    let mut pow: Vec<Vec<BigInt>> = Vec::new();
    let mut one = vec![BigInt::zero(); mons.len()];
    one[mono_index(&mons, &vec![0; m])] = BigInt::one();
    pow.push(one);
    for _ in 1..=level {
        let prev = pow.last().unwrap();
        let mut next = vec![BigInt::zero(); mons.len()];
        for (i, a) in prev.iter().enumerate() {
            if a.is_zero() {
                continue;
            }
            for (j, &b) in gmon.iter().enumerate() {
                if b == 0 {
                    continue;
                }
                let s: Vec<usize> = mons[i].iter().zip(&gm[j]).map(|(x, y)| x + y).collect();
                if s.iter().sum::<usize>() > d * level {
                    continue;
                }
                let k = mono_index(&mons, &s);
                next[k] = (&next[k] + a * BigInt::from(b)).mod_floor(&modulus);
            }
        }
        pow.push(next);
    }
    let w: Vec<BigInt> = (1..=m)
        .map(|i| BigInt::from(binom(m, i)) * BigInt::from(bound).pow(i as u32))
        .collect();
    let weights: Vec<BigInt> = mons
        .iter()
        .map(|b| {
            b.iter()
                .zip(&w)
                .fold(BigInt::one(), |acc, (&e, wi)| acc * wi.pow(e as u32))
        })
        .collect();
    let rows = mons
        .iter()
        .map(|beta| {
            let k = beta[m - 1] / d;
            let mut gamma = beta.clone();
            gamma[m - 1] -= k * d;
            let scale = pb.pow((level - k) as u32);
            let mut row = vec![BigInt::zero(); mons.len()];
            for (i, c) in pow[k].iter().enumerate() {
                if c.is_zero() {
                    continue;
                }
                let s: Vec<usize> = mons[i].iter().zip(&gamma).map(|(x, y)| x + y).collect();
                let j = mono_index(&mons, &s);
                // k = 0 rows are p^ℓ·e^β; reducing those mod p^ℓ would zero them.
                let v = c * &scale;
                let v = if k == 0 { v } else { v.mod_floor(&modulus) };
                row[j] = v * &weights[j];
            }
            row
        })
        .collect();
    Lattice {
        mons,
        weights,
        rows,
        modulus,
    }
}

fn vanishes(l: &Lattice, row: &[BigInt], e: &[u64]) -> bool {
    let mut acc = BigInt::zero();
    for (j, c) in row.iter().enumerate() {
        if c.is_zero() {
            continue;
        }
        let (q, r) = c.div_rem(&l.weights[j]);
        assert!(
            r.is_zero(),
            "reduced row not divisible by its column weight"
        );
        let mono = l.mons[j].iter().zip(e).fold(BigInt::one(), |a, (&b, &v)| {
            a * BigInt::from(v).pow(b as u32)
        });
        acc += q * mono;
    }
    acc.is_zero() && row.iter().any(|c| !c.is_zero())
}

fn norm_sq(row: &[BigInt]) -> BigInt {
    row.iter().map(|c| c * c).sum()
}

fn arg<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or(default)
}

fn list(s: &str) -> Vec<u64> {
    s.split(',').map(|v| v.parse().unwrap()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p: u64 = arg(&args, "--p", "1099511627563").parse().unwrap();
    let a: i64 = arg(&args, "--a", "-3").parse().unwrap();
    let b: u64 = arg(&args, "--b", "56").parse().unwrap();
    let m: usize = arg(&args, "--m", "3").parse().unwrap();
    let levels = list(arg(&args, "--levels", "1,2"));
    let bounds = list(arg(&args, "--bounds", "1,2,3,4"));
    let trials: usize = arg(&args, "--trials", "8").parse().unwrap();
    // Reject planted roots with S₄(x; z) = 0 over ℤ (global relations, e.g. the
    // trivial P − P + Q = Q), detected as vanishing modulo a second prime.
    let exclude_global = !args.iter().any(|a| a == "--allow-global");
    let p2 = (1u64 << 61) - 1;
    let c2 = Curve::new(p2, (a.rem_euclid(p2 as i64)) as u64, b % p2);
    let seed: u64 = arg(&args, "--seed", "20261028").parse().unwrap();
    let f = Fp::new(p);
    let c = Curve::new(p, f.reduce_i64(a), b);
    let d = 1usize << (m - 1);
    for &level in &levels {
        for &bound in &bounds {
            let mut rng = StdRng::seed_from_u64(seed ^ (level << 40) ^ (bound << 20) ^ m as u64);
            for t in 0..2 * trials {
                let planted = t < trials;
                let (z, xs) = if planted {
                    let mut attempts = 0;
                    let found = loop {
                        attempts += 1;
                        if attempts > 500 {
                            break None;
                        }
                        let xs: Vec<u64> = (0..m).map(|_| rng.gen_range(1..=bound)).collect();
                        let nodes: Vec<u64> = (0..=d as u64).collect();
                        let vals: Vec<u64> = nodes.iter().map(|&zz| c.summation(&xs, zz)).collect();
                        let q = interpolate(f, &nodes, &vals);
                        if let Some(z) = some_root(f, &q, &mut rng) {
                            if exclude_global && c2.summation(&xs, z) == 0 {
                                continue;
                            }
                            break Some((z, xs));
                        }
                    };
                    let Some(found) = found else {
                        println!(
                            "{{\"p\":{p},\"m\":{m},\"level\":{level},\"bound\":{bound},\"planted\":true,\"skipped\":\"no non-global planted root in 500 draws\"}}"
                        );
                        continue;
                    };
                    found
                } else {
                    (rng.gen_range(0..p), Vec::new())
                };
                let Some(g) = symmetrise(&c, m, z, &mut rng) else {
                    println!("{{\"error\":\"symmetric fit failed\",\"z\":{z}}}");
                    continue;
                };
                let t0 = Instant::now();
                let lat = build(m, level as usize, &g, p, bound);
                let dim = lat.rows.len();
                let build_ms = t0.elapsed().as_millis();
                let mut basis = lat.rows.clone();
                let t1 = Instant::now();
                let ok = lll_reduce(&mut basis, 0.75).is_ok();
                let lll_ms = t1.elapsed().as_millis();
                // Howgrave-Graham: ‖h(eW)‖² < p^{2ℓ}/dim.
                let hg =
                    &norm_sq(&basis[0]) * BigInt::from(dim as u64) < &lat.modulus * &lat.modulus;
                let (vanish, estar) = if planted {
                    let e: Vec<u64> = {
                        let mut e = vec![1u128];
                        for &x in &xs {
                            let mut next = vec![0u128; e.len() + 1];
                            for (i, &cc) in e.iter().enumerate() {
                                next[i] += cc;
                                next[i + 1] += cc * x as u128;
                            }
                            e = next;
                        }
                        e[1..].iter().map(|&v| v as u64).collect()
                    };
                    (
                        basis.iter().filter(|r| vanishes(&lat, r, &e)).count(),
                        format!("{e:?}"),
                    )
                } else {
                    (0, "null".into())
                };
                let first_bits = norm_sq(&basis[0]).abs().bits() / 2;
                println!(
                    "{{\"p\":{p},\"m\":{m},\"level\":{level},\"dim\":{dim},\"bound\":{bound},\"planted\":{planted},\"z\":{z},\"estar\":\"{estar}\",\"lll_ok\":{ok},\"hg_holds\":{hg},\"vanishing_rows\":{vanish},\"first_row_bits\":{first_bits},\"modulus_bits\":{},\"build_ms\":{build_ms},\"lll_ms\":{lll_ms},\"enum_cost\":{}}}",
                    lat.modulus.bits(),
                    (bound + 1).pow(m as u32 - 1)
                );
            }
        }
    }
}
