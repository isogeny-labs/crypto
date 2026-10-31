//! # m = 4 two-plus-two MITM beyond the native `u64` cap
//!
//! The same planted-solve scaling study as `m4_mitm_scaling`, on
//! arbitrary-precision primes (`num-bigint::BigUint`), so the ladder can
//! pass the `Fp64` ceiling `p < 2^62`.
//!
//! The table is keyed by the full canonical point `(x, y)` packed into
//! `u128`s, so it is exact (no sign collisions).  With factor-base size
//! `B = round(p^{1/5})` the pair table is `B^2 = p^{2/5}` entries; this is
//! the memory wall of the 2 + 2 split and the reason the ladder stops
//! where it does.  Each group operation is one affine addition and is
//! charged one addition.
//!
//! ```bash
//! cargo run --release --example m4_mitm_scaling_big
//! cargo run --release --example m4_mitm_scaling_big -- --bits 64,68,72 --targets 5
//! M4BIG_OUT=research/ic_m4_mitm_scaling_20261031/run_big.txt \
//!   cargo run --release --example m4_mitm_scaling_big
//! ```
//!
//! **Not a security claim.** Synthetic known-answer instances only.

use std::collections::HashMap;
use std::time::Instant;

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

fn is_probable_prime(n: &BigUint) -> bool {
    let two = BigUint::from(2u32);
    if n < &two {
        return false;
    }
    if n == &two {
        return true;
    }
    if n % &two == BigUint::zero() {
        return false;
    }
    let one = BigUint::one();
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0u32;
    while (&d % &two) == BigUint::zero() {
        d /= &two;
        s += 1;
    }
    for a in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let a = BigUint::from(a);
        if &a >= n {
            continue;
        }
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        let mut composite = true;
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n_minus_one {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn largest_prime_below_pow2(bits: u32, res_mod4: u32) -> BigUint {
    let mut p = (BigUint::one() << bits) - BigUint::one();
    // Keep p odd and reduce by 2 until it is 3 mod 4.
    while (&p % 4u32) != BigUint::from(res_mod4) {
        p -= BigUint::from(2u32);
    }
    while !is_probable_prime(&p) {
        p -= BigUint::from(4u32);
    }
    p
}

fn modinv(a: &BigUint, p: &BigUint) -> BigUint {
    let (mut t, mut newt) = (BigInt::zero(), BigInt::one());
    let (mut r, mut newr) = (BigInt::from(p.clone()), BigInt::from((a % p).clone()));
    while !newr.is_zero() {
        let q = &r / &newr;
        let next_t = &t - &q * &newt;
        t = std::mem::replace(&mut newt, next_t);
        let next_r = &r - &q * &newr;
        r = std::mem::replace(&mut newr, next_r);
    }
    let m = BigInt::from(p.clone());
    (((t % &m) + &m) % &m).to_biguint().unwrap()
}

#[derive(Clone, PartialEq, Eq)]
struct PtBig {
    x: BigUint,
    y: BigUint,
}

struct CurveBig {
    p: BigUint,
    a: BigUint,
    b: BigUint,
}

impl CurveBig {
    fn add(&self, a: Option<&PtBig>, b: Option<&PtBig>) -> Option<PtBig> {
        let p = &self.p;
        let (a, b) = match (a, b) {
            (None, x) => return x.cloned(),
            (x, None) => return x.cloned(),
            (Some(a), Some(b)) => (a, b),
        };
        if a.x == b.x {
            if (&a.y + &b.y) % p == BigUint::zero() {
                return None;
            }
            // doubling: lambda = (3x^2 + a) / (2y)
            let num = (BigUint::from(3u32) * &a.x * &a.x + &self.a) % p;
            let den = (BigUint::from(2u32) * &a.y) % p;
            let lam = (num * modinv(&den, p)) % p;
            let x3 = (&lam * &lam + p - &a.x + p - &b.x) % p;
            let y3 = (&lam * ((&a.x + p - &x3) % p) + p - &a.y) % p;
            Some(PtBig { x: x3, y: y3 })
        } else {
            let num = (&b.y + p - &a.y) % p;
            let den = (&b.x + p - &a.x) % p;
            let lam = (num * modinv(&den, p)) % p;
            let x3 = (&lam * &lam + p - &a.x + p - &b.x) % p;
            let y3 = (&lam * ((&a.x + p - &x3) % p) + p - &a.y) % p;
            Some(PtBig { x: x3, y: y3 })
        }
    }

    fn lift_x(&self, x: &BigUint) -> Option<PtBig> {
        let p = &self.p;
        let x2 = (x * x) % p;
        let rhs = (&x2 * x + &self.a * x + &self.b) % p;
        let e = (p + BigUint::one()) / BigUint::from(4u32);
        let y = rhs.modpow(&e, p);
        if (&y * &y) % p == rhs {
            Some(PtBig { x: x.clone(), y })
        } else {
            None
        }
    }
}

fn key_of(pt: &PtBig) -> (u128, u128) {
    (pt.x.to_u128_digit(), pt.y.to_u128_digit())
}

trait ToU128 {
    fn to_u128_digit(&self) -> u128;
}
impl ToU128 for BigUint {
    fn to_u128_digit(&self) -> u128 {
        let mut v = 0u128;
        for d in self.to_u64_digits().iter().rev() {
            v = (v << 64) | u128::from(*d);
        }
        v
    }
}

fn batch_inv(v: &[BigUint], p: &BigUint) -> Vec<BigUint> {
    let n = v.len();
    let mut pre = Vec::with_capacity(n);
    let mut acc = BigUint::one();
    for x in v {
        pre.push(acc.clone());
        acc = (&acc * x) % p;
    }
    let mut inv = modinv(&acc, p);
    let mut out = vec![BigUint::zero(); n];
    for i in (0..n).rev() {
        out[i] = (&inv * &pre[i]) % p;
        inv = (&inv * &v[i]) % p;
    }
    out
}

fn run(bits: u32, targets: usize, seed: u64) -> (f64, f64, usize, usize, f64) {
    let p = largest_prime_below_pow2(bits, 3);
    let curve = CurveBig {
        a: (&p - BigUint::from(3u32)) % &p,
        b: BigUint::one(),
        p: p.clone(),
    };
    let pf = p.to_string().parse::<f64>().unwrap();
    let bsz = pf.powf(0.2).round().max(4.0) as usize;
    let mut rng = StdRng::seed_from_u64(seed ^ u64::from(bits));

    let mut base: Vec<PtBig> = Vec::with_capacity(bsz);
    while base.len() < bsz {
        let x = BigUint::from(rng.gen::<u128>()) % &p;
        if x.is_zero() {
            continue;
        }
        if let Some(pt) = curve.lift_x(&x) {
            base.push(pt);
        }
    }

    let t0 = Instant::now();
    let b = base.len();
    let cap = (b * (b + 1) / 2).min(40_000_000);
    let mut table: HashMap<(u128, u128), (u32, u32)> = HashMap::with_capacity(cap);
    let mut sums: Vec<Option<PtBig>> = vec![None; b * (b + 1) / 2];
    let mut table_adds = 0u64;

    // Pair table, one batch inversion per row instead of one per addition.
    let three = BigUint::from(3u32);
    let two = BigUint::from(2u32);
    for i in 0..b {
        let mut ds: Vec<BigUint> = Vec::with_capacity(b - i);
        let mut js: Vec<usize> = Vec::with_capacity(b - i);
        for j in i..b {
            let den = if j == i {
                (&two * &base[i].y) % &p
            } else {
                (&base[j].x + &p - &base[i].x) % &p
            };
            if den.is_zero() {
                continue;
            }
            ds.push(den);
            js.push(j);
        }
        let invs = batch_inv(&ds, &p);
        for (t, &j) in js.iter().enumerate() {
            let invl = &invs[t];
            let lam = if j == i {
                (((&three * &base[i].x * &base[i].x) + &curve.a) % &p * invl) % &p
            } else {
                ((&base[j].y + &p - &base[i].y) % &p * invl) % &p
            };
            let x3 = (&lam * &lam + &p - &base[i].x + &p - &base[j].x) % &p;
            let y3 = (&lam * ((&base[i].x + &p - &x3) % &p) + &p - &base[i].y) % &p;
            let sp = PtBig { x: x3, y: y3 };
            table.entry(key_of(&sp)).or_insert((i as u32, j as u32));
            sums[i * b - i * (i - 1) / 2 + (j - i)] = Some(sp);
            table_adds += 1;
        }
    }

    let mut q_total = 0u64;
    let mut verified = 0usize;
    for _ in 0..targets {
        let mut idx = [0usize; 4];
        for k in 0..4 {
            loop {
                let c = rng.gen_range(0..b);
                if !idx[..k].contains(&c) {
                    idx[k] = c;
                    break;
                }
            }
        }
        let mut r = base[idx[0]].clone();
        for &k in &idx[1..] {
            r = curve.add(Some(&r), Some(&base[k])).unwrap();
        }
        let mut q = 0u64;
        let mut ok = false;
        'outer: for i in 0..b {
            // Denominators x_S - x_R for every pair (i, j); one batch inverse.
            let mut ds: Vec<BigUint> = Vec::with_capacity(b - i);
            let mut jl: Vec<usize> = Vec::with_capacity(b - i);
            let mut nums: Vec<BigUint> = Vec::with_capacity(b - i);
            for j in i..b {
                let Some(s) = sums[i * b - i * (i - 1) / 2 + (j - i)].as_ref() else {
                    continue;
                };
                let den = (&s.x + &p - &r.x) % &p;
                if den.is_zero() {
                    continue;
                }
                let num = (&p + &p - &s.y % &p - &r.y % &p) % &p;
                ds.push(den);
                nums.push(num);
                jl.push(j);
            }
            let invs = batch_inv(&ds, &p);
            for (t, &j) in jl.iter().enumerate() {
                q += 1;
                let s = sums[i * b - i * (i - 1) / 2 + (j - i)].as_ref().unwrap();
                let lam = (&nums[t] * &invs[t]) % &p;
                let x3 = (&lam * &lam + &p - &r.x + &p - &s.x) % &p;
                let y3 = (&lam * ((&r.x + &p - &x3) % &p) + &p - &r.y) % &p;
                let d = PtBig { x: x3, y: y3 };
                if let Some(&(k, l)) = table.get(&key_of(&d)) {
                    let s4 = curve
                        .add(Some(&base[i]), Some(&base[j]))
                        .and_then(|s| curve.add(Some(&s), Some(&base[k as usize])))
                        .and_then(|s| curve.add(Some(&s), Some(&base[l as usize])));
                    ok = s4.map(|s| s == r).unwrap_or(false);
                    break 'outer;
                }
            }
        }
        q_total += q;
        verified += ok as usize;
    }
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    let per = table_adds as f64 / targets as f64 + q_total as f64 / targets as f64;
    (per, table_adds as f64, b, verified, ms)
}

fn fit(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64;
    let mx = xs.iter().sum::<f64>() / n;
    let my = ys.iter().sum::<f64>() / n;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    for i in 0..xs.len() {
        sxx += (xs[i] - mx) * (xs[i] - mx);
        sxy += (xs[i] - mx) * (ys[i] - my);
    }
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;
    for i in 0..xs.len() {
        let pred = intercept + slope * xs[i];
        ss_res += (ys[i] - pred) * (ys[i] - pred);
        ss_tot += (ys[i] - my) * (ys[i] - my);
    }
    (slope, 1.0 - ss_res / ss_tot)
}

fn main() {
    let mut bits: Vec<u32> = vec![64, 66, 68];
    let mut targets = 4usize;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bits" => {
                bits = args[i + 1].split(',').map(|s| s.parse().unwrap()).collect();
                i += 2;
            }
            "--targets" => {
                targets = args[i + 1].parse().unwrap();
                i += 2;
            }
            other => panic!("unknown flag {other}"),
        }
    }

    println!("=== m=4 2+2 MITM, arbitrary precision (planted solve) ===");
    println!(
        "{:>4}  {:>7}  {:>10}  {:>14}  {:>9}  {:>8}",
        "bits", "|F|", "pairs", "adds/target", "verified", "ms"
    );
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for &b in &bits {
        let (per, table, bsz, ok, ms) = run(b, targets, 0x4D34_B160);
        println!(
            "{b:>4}  {bsz:>7}  {table:>10.0}  {per:>14.1}  {ok:>3}/{:<3}  {ms:>8.1}",
            targets
        );
        assert_eq!(ok, targets, "every planted witness must verify");
        xs.push((2f64).powi(b as i32).log2());
        ys.push(per.log2());
    }
    let (slope, r2) = fit(&xs, &ys);
    println!(
        "\nplanted solve exponent = {slope:.3} (R^2 {r2:.4}, {} sizes, arbitrary precision)",
        xs.len()
    );
    assert!(
        (0.30..0.50).contains(&slope),
        "planted solve exponent {slope:.3} outside 0.30..0.50"
    );
    println!(
        "note: the table is B^2 = p^0.4 entries; at 72 bits that is ~2.4e8 entries, \
         the practical memory wall of the 2+2 split"
    );

    if let Ok(path) = std::env::var("M4BIG_OUT") {
        if let Some(dir) = std::path::Path::new(&path).parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let _ = std::fs::write(
            path,
            format!(
                "planted_solve_exponent {slope:.4}\nplanted_r2 {r2:.4}\nsizes {}\n",
                xs.len()
            ),
        );
    }
}
