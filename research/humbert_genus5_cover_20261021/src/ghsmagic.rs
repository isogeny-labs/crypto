//! `humbert ghs-magic <N> <modulus-hex> <b-hex>`: the GHS magic number of
//! E: y^2 + xy = x^3 + a x^2 + b over F_{2^N} for the descent to F_2 (n = N),
//!   m(b) = dim_{F_2} span{ (1, sigma^i(sqrt b)) : 0 <= i < N },  sigma(y) = y^2,
//! (Menezes-Qu / Hess). The descended cover has genus 2^{m-1} or 2^{m-1} - 1.
//! Independent of the crypto crate's `ghs_screen`; small, for prime N only.

/// Elements of F_2[z]/(f) as little-endian u64 limbs.
struct F2n {
    n: usize,
    modulus: Vec<u64>, // includes z^n
}

impl F2n {
    fn limbs(&self) -> usize {
        (self.n + 63) / 64 + 1
    }
    fn bit(v: &[u64], i: usize) -> bool {
        (v[i / 64] >> (i % 64)) & 1 == 1
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        let l = self.limbs();
        let mut prod = vec![0u64; 2 * l + 1];
        for i in 0..self.n {
            if Self::bit(a, i) {
                for (j, &w) in b.iter().enumerate() {
                    if w == 0 {
                        continue;
                    }
                    let (q, r) = ((i / 64) + j, i % 64);
                    prod[q] ^= w << r;
                    if r > 0 {
                        prod[q + 1] ^= w >> (64 - r);
                    }
                }
            }
        }
        // reduce bits >= n by the modulus
        for i in (self.n..(2 * l + 1) * 64).rev() {
            if Self::bit(&prod, i) {
                let s = i - self.n;
                for k in 0..=self.n {
                    if Self::bit(&self.modulus, k) {
                        prod[(s + k) / 64] ^= 1u64 << ((s + k) % 64);
                    }
                }
            }
        }
        prod.truncate(l);
        prod
    }
}

fn parse_hex(s: &str, limbs: usize) -> Vec<u64> {
    let s = s.trim_start_matches("0x");
    let mut v = vec![0u64; limbs];
    for (i, c) in s.chars().rev().enumerate() {
        let d = c.to_digit(16).expect("hex digit") as u64;
        v[(4 * i) / 64] |= d << ((4 * i) % 64);
    }
    v
}

/// Rank over F_2 of bit vectors (each a Vec<u64>), by elimination on the leading bit.
fn rank(mut rows: Vec<Vec<u64>>, bits: usize) -> usize {
    let mut r = 0;
    for col in (0..bits).rev() {
        let Some(p) = (r..rows.len()).find(|&i| F2n::bit(&rows[i], col)) else { continue };
        rows.swap(r, p);
        let piv = rows[r].clone();
        for i in 0..rows.len() {
            if i != r && F2n::bit(&rows[i], col) {
                for k in 0..piv.len() {
                    rows[i][k] ^= piv[k];
                }
            }
        }
        r += 1;
    }
    r
}

pub fn magic(n: usize, modulus_hex: &str, b_hex: &str) -> (usize, bool) {
    let f = F2n { n, modulus: parse_hex(modulus_hex, (n + 64) / 64 + 1) };
    let l = f.limbs();
    let b = parse_hex(b_hex, l);
    // sqrt(b) = b^(2^(n-1))
    let mut s = b.clone();
    for _ in 0..n - 1 {
        s = f.mul(&s, &s);
    }
    let check = f.mul(&s, &s) == b;
    // vectors (1, sigma^i(sqrt b)): bit n carries the leading 1
    let mut rows = Vec::with_capacity(n);
    let mut y = s;
    for _ in 0..n {
        let mut v = y.clone();
        v.resize(l, 0);
        v[n / 64] |= 1u64 << (n % 64);
        rows.push(v);
        y = f.mul(&y, &y);
    }
    (rank(rows, n + 1), check)
}

pub fn run(args: &[String]) {
    let n: usize = args[0].parse().expect("N");
    let (m, ok) = magic(n, &args[1], &args[2]);
    let genus = if m >= 1 { format!("2^{} or 2^{} - 1", m - 1, m - 1) } else { "-".into() };
    println!("{{\"N\":{n},\"b\":\"{}\",\"sqrt_check\":{ok},\"magic_number\":{m},\"cover_genus\":\"{genus}\"}}", args[2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subfield_b_has_magic_one_and_generic_b_has_large_magic() {
        // N = 7, f = z^7 + z + 1 (0x83); b = 1 is in F_2.
        // x^7 - 1 = (x+1)(x^3+x+1)(x^3+x^2+1) over F_2, so special b
        // (e.g. b = z) give intermediate m; the maximum over all b must be 7
        assert_eq!(magic(7, "0x83", "0x1"), (1, true));
        let ms: Vec<usize> = (1u64..128).map(|b| magic(7, "0x83", &format!("{b:x}"))).map(|(m, ok)| { assert!(ok); m }).collect();
        assert_eq!(*ms.iter().max().unwrap(), 7);
        assert!(ms.iter().all(|&m| [1, 4, 7].contains(&m)), "{ms:?}");
    }
}
