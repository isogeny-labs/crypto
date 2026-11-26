//! Toy F_p* relation collector for `examples/lp_retention_eval.rs` and
//! `tests/lp_retention.rs` (included with `#[path]`; not part of the library).
//!
//! Draw `k` uniformly, factor `g^k mod p` (`p < 2^32`) over primes `≤ B`, and
//! keep the draw as a complete relation (cofactor 1) or a partial with one or
//! two large primes `< L`. Logs live modulo `ℓ`, the largest prime factor of
//! `p − 1`: row `k ≡ Σ e_j log q_j + Σ log L_i (mod ℓ)`. Every emitted row can
//! be replayed by exponentiation ([`replay`]).

use crypto_lib::cryptanalysis::large_prime_filter::PartialRelation;
use crypto_lib::cryptanalysis::large_prime_graph::retention::StreamRow;
use serde::{Deserialize, Serialize};

pub struct SplitMix(pub u64);
impl SplitMix {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

pub fn pow_mod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

pub fn primes_up_to(n: u64) -> Vec<u64> {
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    let mut out = Vec::new();
    for i in 2..=n {
        if sieve[i] {
            out.push(i as u64);
            let mut j = i * i;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    out
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += if d == 2 { 1 } else { 2 };
    }
    true
}

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut f = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            f.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        f.push(n);
    }
    f
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    pub p: u64,
    pub g: u64,
    pub ell: u64,
}

impl Instance {
    /// Prime `p < 2^32`, least primitive root `≥ g_min`, `ℓ` = largest prime factor of `p − 1`.
    pub fn new(p: u64, g_min: u64) -> Self {
        assert!(p < (1 << 32) && is_prime(p), "p must be a prime below 2^32");
        let qs = prime_factors(p - 1);
        let ell = *qs.last().expect("p > 2");
        let g = (g_min..p)
            .find(|&g| qs.iter().all(|&q| pow_mod(g, (p - 1) / q, p) != 1))
            .expect("primitive root");
        Self { p, g, ell }
    }

    pub fn next_prime(start: u64) -> u64 {
        (start..).find(|&n| is_prime(n)).expect("prime")
    }
}

/// The collected stream plus what replay needs.
pub struct Collection {
    pub instance: Instance,
    pub factor_base: Vec<u64>,
    pub lp_bound: u64,
    pub stream: Vec<StreamRow>,
    pub exponents: Vec<u64>,
}

pub fn collect(
    inst: &Instance,
    smooth_bound: u64,
    lp_bound: u64,
    draws: u64,
    seed: u64,
) -> Collection {
    let Instance { p, g, ell } = *inst;
    let fb = primes_up_to(smooth_bound);
    let mid: Vec<u64> = primes_up_to(lp_bound)
        .into_iter()
        .filter(|&q| q > smooth_bound)
        .collect();
    let mut rng = SplitMix(seed);
    let (mut stream, mut exponents) = (Vec::new(), Vec::new());
    for draw in 1..=draws {
        let k = 1 + rng.next_u64() % (p - 2);
        let mut c = pow_mod(g, k, p);
        let mut small = Vec::new();
        for (j, &q) in fb.iter().enumerate() {
            let mut e = 0u64;
            while c.is_multiple_of(q) {
                c /= q;
                e += 1;
            }
            if e > 0 {
                small.push((j as u32, e % ell));
            }
        }
        let large: Vec<(u64, u64)> = if c == 1 {
            vec![]
        } else if c < lp_bound && is_prime(c) {
            vec![(c, 1)]
        } else if !is_prime(c) {
            match mid
                .iter()
                .take_while(|&&q| q * q <= c)
                .find(|&&q| c.is_multiple_of(q))
            {
                Some(&a) if c / a < lp_bound && is_prime(c / a) => vec![(a, 1), (c / a, 1)],
                _ => continue,
            }
        } else {
            continue;
        };
        let id = stream.len() as u64;
        stream.push(StreamRow {
            draw,
            relation: PartialRelation {
                id,
                small,
                large,
                rhs: k % ell,
            },
        });
        exponents.push(k);
    }
    Collection {
        instance: inst.clone(),
        factor_base: fb,
        lp_bound,
        stream,
        exponents,
    }
}

/// `g^k ≡ ∏ q_j^{e_j} · ∏ L` (mod p) for every row; small exponents are below
/// `ℓ`, so their stored residues are the exponents. Returns rows checked or
/// the first failing position.
pub fn replay(col: &Collection) -> Result<usize, usize> {
    let Instance { p, g, .. } = col.instance;
    let b = *col.factor_base.last().unwrap_or(&1);
    for (i, (row, &k)) in col.stream.iter().zip(&col.exponents).enumerate() {
        let mut prod = 1u64;
        for &(j, e) in &row.relation.small {
            prod = prod * pow_mod(col.factor_base[j as usize], e, p) % p;
        }
        for &(q, m) in &row.relation.large {
            if !(q > b && q < col.lp_bound && is_prime(q)) {
                return Err(i);
            }
            prod = prod * pow_mod(q, m, p) % p;
        }
        if prod != pow_mod(g, k, p) {
            return Err(i);
        }
    }
    Ok(col.stream.len())
}
