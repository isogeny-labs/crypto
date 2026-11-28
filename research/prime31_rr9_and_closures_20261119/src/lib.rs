//! Workstream 3: Non-enumerative `Prime31` `RR9` 4-variable Sparse Wiedemann &
//! Shared-Coefficient Remainder Tower extraction (Lead P2-6) + Native verification &
//! scoped closure of Prime-Field Auxiliary Covers & Deuring Vertical-Isogeny Rarity (Lead P1-4).

pub mod fx_hash {
    use std::collections::{HashMap, HashSet};
    use std::hash::{BuildHasherDefault, Hasher};

    #[derive(Default, Clone)]
    pub struct FxHasher {
        hash: u64,
    }

    impl Hasher for FxHasher {
        #[inline]
        fn finish(&self) -> u64 {
            self.hash
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            const K: u64 = 0x517c_c1b7_2722_0a95;
            for &b in bytes {
                self.hash = (self.hash.rotate_left(5) ^ (b as u64)).wrapping_mul(K);
            }
        }
    }

    pub type FxBuildHasher = BuildHasherDefault<FxHasher>;
    pub type FxMap<K, V> = HashMap<K, V, FxBuildHasher>;
    pub type FxSet<K> = HashSet<K, FxBuildHasher>;
}

pub mod groebner_f4 {
    use std::cmp::Ordering as CmpOrdering;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub enum Ordering {
        Lex,
        Grlex,
        Grevlex,
    }

    #[inline]
    pub fn cmp_monomial(a: &[u32], b: &[u32], ord: Ordering) -> CmpOrdering {
        debug_assert_eq!(a.len(), b.len());
        match ord {
            Ordering::Lex => {
                for (x, y) in a.iter().zip(b.iter()) {
                    match x.cmp(y) {
                        CmpOrdering::Equal => continue,
                        non_eq => return non_eq,
                    }
                }
                CmpOrdering::Equal
            }
            Ordering::Grlex => {
                let da: u32 = a.iter().sum();
                let db: u32 = b.iter().sum();
                match da.cmp(&db) {
                    CmpOrdering::Equal => cmp_monomial(a, b, Ordering::Lex),
                    non_eq => non_eq,
                }
            }
            Ordering::Grevlex => {
                let da: u32 = a.iter().sum();
                let db: u32 = b.iter().sum();
                match da.cmp(&db) {
                    CmpOrdering::Equal => {
                        for (x, y) in a.iter().zip(b.iter()).rev() {
                            match y.cmp(x) {
                                CmpOrdering::Equal => continue,
                                non_eq => return non_eq,
                            }
                        }
                        CmpOrdering::Equal
                    }
                    non_eq => non_eq,
                }
            }
        }
    }
}

pub mod jv_bitops {
    #[derive(Clone, Copy, Debug)]
    pub enum Kind {
        FieldMul,
        FieldAdd,
        FieldInv,
    }

    #[inline]
    pub fn enabled() -> bool {
        false
    }

    #[inline]
    pub fn charge(_kind: Kind, _n: u64) {}

    #[inline]
    pub fn row_ops(_n: u64) {}

    #[inline]
    pub fn words(_n: u64) {}

    #[inline]
    pub fn probes(_probes: u64, _words_per_key: u64) {}

    #[inline]
    pub fn sort_cost(n: usize) -> u64 {
        if n <= 1 {
            0
        } else {
            (n as u64) * ((usize::BITS - (n - 1).leading_zeros()) as u64)
        }
    }
}

pub mod residual_walk {
    use crate::prime31_field::fp_pow;

    /// Deterministic Miller–Rabin primality test exact for all `u64`.
    pub fn is_prime_u64(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        for small in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
            if n == small {
                return true;
            }
            if n % small == 0 {
                return false;
            }
        }
        let d_unshifted = n - 1;
        let s = d_unshifted.trailing_zeros();
        let d = d_unshifted >> s;
        for a in [2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
            let base = a % n;
            if base == 0 {
                continue;
            }
            let mut x = fp_pow(base, d, n);
            if x == 1 || x == n - 1 {
                continue;
            }
            let mut composite = true;
            for _ in 1..s {
                x = ((x as u128 * x as u128) % (n as u128)) as u64;
                if x == n - 1 {
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
}

pub mod glv_invariant_base {
    use crate::prime31_field::{fp_add, fp_inv, fp_mul, fp_neg, fp_sub, splitmix64};
    use std::time::Instant;

    fn poly_trim(mut f: Vec<u64>) -> Vec<u64> {
        while f.last().copied() == Some(0) {
            f.pop();
        }
        f
    }

    fn poly_rem_counted(a: &[u64], b: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
        let b = poly_trim(b.to_vec());
        if b.is_empty() {
            return poly_trim(a.to_vec());
        }
        let mut r = poly_trim(a.to_vec());
        let db = b.len() - 1;
        let inv_lead = fp_inv(b[db], p);
        *muls += 1;
        while r.len() >= b.len() && !r.is_empty() {
            let dr = r.len() - 1;
            let lead = r[dr];
            if lead != 0 {
                let c = fp_mul(lead, inv_lead, p);
                *muls += 1;
                let shift = dr - db;
                for k in 0..db {
                    let prod = fp_mul(c, b[k], p);
                    *muls += 1;
                    r[shift + k] = fp_sub(r[shift + k], prod, p);
                }
            }
            r.pop();
            r = poly_trim(r);
        }
        r
    }

    fn poly_mul_mod_counted(a: &[u64], b: &[u64], m: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut prod = vec![0u64; a.len() + b.len() - 1];
        for (i, &ai) in a.iter().enumerate() {
            if ai == 0 {
                continue;
            }
            for (j, &bj) in b.iter().enumerate() {
                if bj == 0 {
                    continue;
                }
                prod[i + j] = fp_add(prod[i + j], fp_mul(ai, bj, p), p);
                *muls += 1;
            }
        }
        poly_rem_counted(&prod, m, p, muls)
    }

    fn poly_powmod_counted(
        base: &[u64],
        mut exp: u64,
        modulus: &[u64],
        p: u64,
        muls: &mut u64,
    ) -> Vec<u64> {
        let mut acc = poly_rem_counted(&[1u64], modulus, p, muls);
        let mut cur = poly_rem_counted(base, modulus, p, muls);
        while exp > 0 {
            if (exp & 1) == 1 {
                acc = poly_mul_mod_counted(&acc, &cur, modulus, p, muls);
            }
            exp >>= 1;
            if exp > 0 {
                cur = poly_mul_mod_counted(&cur, &cur, modulus, p, muls);
            }
        }
        acc
    }

    fn poly_gcd_counted(a: &[u64], b: &[u64], p: u64, muls: &mut u64) -> Vec<u64> {
        let mut x = poly_trim(a.to_vec());
        let mut y = poly_trim(b.to_vec());
        while !y.is_empty() {
            let r = poly_rem_counted(&x, &y, p, muls);
            x = y;
            y = r;
        }
        if let Some(&lead) = x.last() {
            if lead != 1 {
                let inv = fp_inv(lead, p);
                *muls += 1;
                for c in &mut x {
                    *c = fp_mul(*c, inv, p);
                    *muls += 1;
                }
            }
        }
        x
    }

    /// Counted Cantor–Zassenhaus rational root finder over `F_p` matching `f4_fp::solve_visit`.
    pub fn poly_roots_fp_counted_until(
        f: &[u64],
        p: u64,
        seed: u64,
        muls: &mut u64,
        deadline: Option<Instant>,
    ) -> Option<Vec<u64>> {
        let expired = || deadline.is_some_and(|d| Instant::now() >= d);
        if expired() {
            return None;
        }
        let f = poly_trim(f.to_vec());
        if f.len() <= 1 {
            return Some(Vec::new());
        }
        if p == 2 {
            return Some(
                (0..2)
                    .filter(|&x| f.iter().rev().fold(0, |v, &c| (v * x + c) % 2) == 0)
                    .collect(),
            );
        }
        if f.len() == 2 {
            *muls += 2;
            return Some(vec![fp_neg(fp_mul(f[0], fp_inv(f[1], p), p), p)]);
        }
        let mut xp = poly_powmod_counted(&[0, 1], p, &f, p, muls);
        if xp.len() < 2 {
            xp.resize(2, 0);
        }
        xp[1] = fp_sub(xp[1], 1, p);
        let g = poly_gcd_counted(&f, &poly_trim(xp), p, muls);
        let mut rng_state = seed ^ 0x524F_4F54;
        let mut out = Vec::new();
        let mut stack = vec![g];
        while let Some(h) = stack.pop() {
            if expired() {
                return None;
            }
            match h.len() {
                0 | 1 => {}
                2 => {
                    *muls += 2;
                    out.push(fp_neg(fp_mul(h[0], fp_inv(h[1], p), p), p));
                }
                _ => {
                    let delta = splitmix64(&mut rng_state) % p;
                    let mut s = poly_powmod_counted(&[delta, 1], (p - 1) / 2, &h, p, muls);
                    if s.is_empty() {
                        s.push(0);
                    }
                    s[0] = fp_sub(s[0], 1, p);
                    let d = poly_gcd_counted(&h, &poly_trim(s), p, muls);
                    if d.len() <= 1 || d.len() == h.len() {
                        stack.push(h);
                        continue;
                    }
                    let mut q = h.clone();
                    let mut quotient = vec![0u64; h.len() - d.len() + 1];
                    let inv_lead = fp_inv(*d.last().unwrap(), p);
                    *muls += 1;
                    while q.len() >= d.len() && !q.is_empty() {
                        let dq = q.len() - 1;
                        let c = fp_mul(q[dq], inv_lead, p);
                        quotient[dq - (d.len() - 1)] = c;
                        for k in 0..d.len() {
                            q[dq - (d.len() - 1) + k] = fp_sub(
                                q[dq - (d.len() - 1) + k],
                                fp_mul(c, d[k], p),
                                p,
                            );
                        }
                        *muls += d.len() as u64 + 1;
                        q.pop();
                        q = poly_trim(q);
                    }
                    stack.push(d);
                    stack.push(poly_trim(quotient));
                }
            }
        }
        out.sort_unstable();
        out.dedup();
        if expired() {
            None
        } else {
            Some(out)
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/f4_fp_bridge.rs"));
pub use f4_fp_host::f4_fp;

#[allow(dead_code, clippy::all)]
#[path = "../../../src/cryptanalysis/plane_quartic_pic.rs"]
pub mod plane_quartic_pic;

pub mod cover_and_deuring;
pub mod p2_6_suite;
pub mod prime31_field;
pub mod rr9_remainder_tower;
pub mod rr9_wiedemann;
