//! Frozen sparse arithmetic extraction; see PRIME_TERMINAL_DESIGN.md.
#![allow(dead_code, unused_imports)]
use super::donor;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    path::Path,
};
pub type Poly = BTreeMap<u64, u128>;
pub fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
pub fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
pub fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap();
}
pub fn add(a: u128, b: u128, p: u128) -> u128 {
    assert!(a < p && b < p && p < (1u128 << 127));
    (a + b) % p
}
pub fn neg(a: u128, p: u128) -> u128 {
    if a == 0 {
        0
    } else {
        p - a
    }
}
pub fn mul(a: u128, b: u128, p: u128) -> u128 {
    donor::modular_mul(a, b, p)
}
pub fn accum(a: &mut Poly, m: u64, c: u128, p: u128) {
    let v = add(*a.get(&m).unwrap_or(&0), c, p);
    if v == 0 {
        a.remove(&m);
    } else {
        a.insert(m, v);
    }
}
pub fn degree(a: &Poly) -> usize {
    a.keys().map(|m| m.count_ones() as usize).max().unwrap_or(0)
}
pub fn scaled_add(a: &mut Poly, b: &Poly, c: u128, p: u128) {
    for (&m, &v) in b {
        accum(a, m, mul(v, c, p), p);
    }
}
pub fn constant(c: u128) -> Poly {
    if c == 0 {
        Poly::new()
    } else {
        [(0, c)].into()
    }
}
pub fn linear(n: usize) -> Poly {
    (0..n).map(|i| (1u64 << i, 1)).collect()
}
pub fn multiply_x(a: &Poly, w: &[u128], p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (j, &weight) in w.iter().enumerate() {
            let bit = 1u64 << j;
            accum(
                &mut out,
                m | bit,
                if m & bit != 0 { mul(c, weight, p) } else { c },
                p,
            );
        }
    }
    out
}
// Independent general polynomial multiplication for controls and small B factors.
pub fn product(a: &Poly, b: &Poly, w: &[u128], p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (&n, &d) in b {
            let mut coefficient = mul(c, d, p);
            let mut repeats = m & n;
            while repeats != 0 {
                let j = repeats.trailing_zeros() as usize;
                coefficient = mul(coefficient, w[j], p);
                repeats &= repeats - 1;
            }
            accum(&mut out, m | n, coefficient, p);
        }
    }
    out
}
pub fn tensor(a: &Poly, b: &Poly, n: usize, p: u128) -> Poly {
    let mut out = Poly::new();
    for (&m, &c) in a {
        for (&k, &d) in b {
            accum(&mut out, m | (k << n), mul(c, d, p), p);
        }
    }
    out
}
pub fn h_poly(n: usize, p: u128) -> Poly {
    assert!(n >= 3 && n % 2 == 1 && n <= 27 && p > n as u128 && p < (1u128 << 127));
    let mut h = constant(1);
    for j in 0..n / 2 {
        let mut next = Poly::new();
        for (&m, &c) in &h {
            accum(&mut next, m | (1 << (2 * j)), c, p);
            accum(&mut next, m | (1 << (2 * j + 1)), neg(c, p), p);
        }
        h = next;
    }
    h
}
pub fn pairing(h: &Poly, a: &Poly, n: usize, p: u128) -> u128 {
    let all = (1u64 << n) - 1;
    h.iter().fold(0, |sum, (&m, &c)| {
        add(sum, mul(c, *a.get(&(all ^ m)).unwrap_or(&0), p), p)
    })
}
pub fn bytes(a: &Poly, n: usize, p: u128) -> Vec<u8> {
    let mut b = b"N83PFF1\n".to_vec();
    b.extend((n as u32).to_le_bytes());
    b.extend((degree(a) as u32).to_le_bytes());
    b.extend((a.len() as u64).to_le_bytes());
    b.extend(p.to_le_bytes());
    for (&m, &c) in a {
        assert!(m < (1 << n) && c > 0 && c < p);
        b.extend(m.to_le_bytes());
        b.extend(c.to_le_bytes());
    }
    b
}
pub fn parse(b: &[u8]) -> (usize, u128, Poly) {
    assert!(b.len() >= 40);
    assert_eq!(&b[..8], b"N83PFF1\n");
    let n = u32::from_le_bytes(b[8..12].try_into().unwrap()) as usize;
    let deg = u32::from_le_bytes(b[12..16].try_into().unwrap()) as usize;
    let count = u64::from_le_bytes(b[16..24].try_into().unwrap()) as usize;
    let p = u128::from_le_bytes(b[24..40].try_into().unwrap());
    assert!(n <= 27 && p > n as u128 && p < (1 << 127));
    assert_eq!(b.len(), 40 + 24 * count);
    let mut a = Poly::new();
    let mut prior = None;
    for row in b[40..].chunks_exact(24) {
        let m = u64::from_le_bytes(row[..8].try_into().unwrap());
        let c = u128::from_le_bytes(row[8..].try_into().unwrap());
        assert!(m < (1 << n) && c > 0 && c < p && prior.map_or(true, |v| m > v));
        prior = Some(m);
        a.insert(m, c);
    }
    assert_eq!(degree(&a), deg);
    (n, p, a)
}
