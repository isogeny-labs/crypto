//! Equal-engineering rho comparator for the B677 same-target panel.
//!
//! Same curve, G and Q as the relation-source arms; same counted field
//! (Mersenne reduction, Montgomery batch inversion) as `cost_v2.rs`.
//! W parallel unfolded 32-jump r-adding walks, one shared batch inverse per
//! step, distinguished points with probability 2^-DP_BITS. No negation map,
//! so this is a conservative (slower) rho; a tuned negation-map rho would
//! need about 1/sqrt(2) as many steps.

use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use serde_json::json;
use std::collections::HashMap;
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const P: u64 = PRIME31.p;
const N: u64 = PRIME31.n;
const JUMPS: usize = 32;
const W: usize = 64;
const DP_BITS: u32 = 5;

struct F {
    mul: u64,
    inv: u64,
}
impl F {
    #[inline(always)]
    fn red(x: u64) -> u64 {
        let x = (x & P) + (x >> 31);
        let x = (x & P) + (x >> 31);
        if x >= P { x - P } else { x }
    }
    #[inline(always)]
    fn mul(&mut self, a: u64, b: u64) -> u64 {
        self.mul += 1;
        Self::red(a * b)
    }
    #[inline(always)]
    fn sub(a: u64, b: u64) -> u64 {
        if a >= b { a - b } else { a + P - b }
    }
    fn inv(&mut self, x: u64) -> u64 {
        assert!(x != 0);
        self.inv += 1;
        let (mut o, mut b, mut e) = (1u64, x, P - 2);
        while e > 0 {
            if e & 1 != 0 {
                o = self.mul(o, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        o
    }
}

fn next_word(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn mulmod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % N as u128) as u64
}
fn powmod(mut a: u64, mut e: u64) -> u64 {
    let mut o = 1;
    while e > 0 {
        if e & 1 != 0 {
            o = mulmod(o, a);
        }
        a = mulmod(a, a);
        e >>= 1;
    }
    o
}
#[inline(always)]
fn jump_of(x: u64) -> usize {
    ((x.wrapping_mul(0x9e37_79b9_7f4a_7c15)) >> 59) as usize
}

fn main() {
    let seed: u64 = std::env::args().nth(1).and_then(|v| v.parse().ok()).unwrap_or(1);
    let cold = Instant::now();
    let mut ar = Arithmetic::default();
    let mut f = F { mul: 0, inv: 0 };
    let mut s = 0x7268_6f5f_6261_7463u64 ^ seed.wrapping_mul(0x1000_0000_01b3);
    let g = Some(PRIME31.g);
    // Jump table R_j = c_j G + d_j Q.
    let mut jumps = Vec::with_capacity(JUMPS);
    for _ in 0..JUMPS {
        let (c, d) = (next_word(&mut s) % N, next_word(&mut s) % N);
        let (u, v) = (ar.scalar(PRIME31, c, g), ar.scalar(PRIME31, d, Q));
        let pt = ar.add(PRIME31, u, v).unwrap();
        jumps.push((pt, c, d));
    }
    let mut walk: Vec<((u64, u64), u64, u64)> = Vec::with_capacity(W);
    let fresh = |ar: &mut Arithmetic, s: &mut u64| {
        let (a, b) = (next_word(s) % N, next_word(s) % N);
        let (u, v) = (ar.scalar(PRIME31, a, g), ar.scalar(PRIME31, b, Q));
        let pt = ar.add(PRIME31, u, v).unwrap();
        (pt, a, b)
    };
    for _ in 0..W {
        walk.push(fresh(&mut ar, &mut s));
    }
    let mut dps: HashMap<(u64, u64), (u64, u64)> = HashMap::new();
    let mut steps = 0u64;
    let mut dens = vec![0u64; W];
    let mut pre = vec![0u64; W];
    let mut restarts = 0u64;
    let recovered = loop {
        // One batched step for all W walks.
        for k in 0..W {
            let (pt, _, _) = walk[k];
            let (jp, _, _) = jumps[jump_of(pt.0)];
            let d = F::sub(jp.0, pt.0);
            dens[k] = if d == 0 { 1 } else { d };
        }
        let mut acc = dens[0];
        pre[0] = acc;
        for k in 1..W {
            acc = f.mul(acc, dens[k]);
            pre[k] = acc;
        }
        let mut iv = f.inv(acc);
        for k in (1..W).rev() {
            let dk = dens[k];
            dens[k] = f.mul(iv, pre[k - 1]);
            iv = f.mul(iv, dk);
        }
        dens[0] = iv;
        let mut hit = None;
        for k in 0..W {
            let (pt, a, b) = walk[k];
            let (jp, c, d) = jumps[jump_of(pt.0)];
            steps += 1;
            let np = if jp.0 == pt.0 {
                // Doubling or inverse: rare; restart this walk.
                restarts += 1;
                walk[k] = fresh(&mut ar, &mut s);
                continue;
            } else {
                let l = f.mul(F::sub(jp.1, pt.1), dens[k]);
                let x3 = F::sub(F::sub(f.mul(l, l), pt.0), jp.0);
                let y3 = F::sub(f.mul(l, F::sub(pt.0, x3)), pt.1);
                (x3, y3)
            };
            let (na, nb) = ((a + c) % N, (b + d) % N);
            walk[k] = (np, na, nb);
            if np.0 & ((1 << DP_BITS) - 1) == 0 {
                if let Some(&(a2, b2)) = dps.get(&np) {
                    if b2 != nb {
                        let num = (a2 + N - na) % N;
                        let den = (nb + N - b2) % N;
                        let x = mulmod(num, powmod(den, N - 2));
                        hit = Some(x);
                        break;
                    }
                } else {
                    dps.insert(np, (na, nb));
                }
            }
        }
        if let Some(x) = hit {
            break x;
        }
    };
    assert_eq!(ar.scalar(PRIME31, recovered, g), Q, "rho scalar must verify");
    let wall = cold.elapsed().as_secs_f64() * 1e3;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "policy": "equal-engineering unfolded 32-jump parallel rho, batch inversion, distinguished points",
            "walks": W, "dp_bits": DP_BITS, "seed": seed, "steps": steps, "restarts": restarts,
            "distinguished_points": dps.len(),
            "recovered_scalar": recovered,
            "charged": {"batched_fp_multiplications": f.mul, "batched_fp_inversions": f.inv,
                        "setup_group_law": ar.counts,
                        "total_fp_multiplications_incl_inversions": f.mul + ar.counts.fp_multiplications,
                        "total_points_computed": steps + ar.counts.group_additions},
            "cold_wall_ms": wall,
        }))
        .unwrap()
    );
}
