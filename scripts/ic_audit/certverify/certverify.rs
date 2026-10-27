// Independent certificate verifier for the slice-A audit. std only; shares no code
// with the repository under audit.
//
// Input (stdin): one JSON-ish case per line, tab-separated fields (written by jq -r):
//   id  kind  field_param  a  b  gx  gy  qx  qy  d  r
//   kind = "gf2m"  : field_param = "n:low1,low2,..."  (x^n + x^low1 + ... + 1 ; low terms exclude 0 unless listed)
//          "gf2m?" : field_param = "n"  (polynomial unknown: search trinomials/pentanomials for one that
//                     puts G and Q on the curve, then require [r]G = O and [d]G = Q)
//          "fp"    : field_param = p
//   a, b, gx.. qy, d, r are decimal or 0x-hex integers (u128).
// Output: one line per case: id <TAB> status <TAB> detail
use std::io::{self, BufRead};

fn parse(s: &str) -> u128 {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x") {
        u128::from_str_radix(h, 16).unwrap()
    } else {
        s.parse::<u128>().unwrap()
    }
}

// ---- GF(2^n) ----
#[derive(Clone, Copy)]
struct F2 {
    n: u32,
    red: u128, // x^n = red (mod poly), low part of the poly (without x^n)
}
impl F2 {
    fn mul(&self, a: u128, b: u128) -> u128 {
        let mut r = 0u128;
        let mut a = a;
        let mut b = b;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            let carry = (a >> (self.n - 1)) & 1;
            a <<= 1;
            if carry == 1 {
                a ^= self.red;
            }
            a &= (1u128 << self.n) - 1;
        }
        r
    }
    fn sq(&self, a: u128) -> u128 { self.mul(a, a) }
    fn inv(&self, a: u128) -> u128 {
        // a^(2^n - 2)
        let mut r = 1u128;
        let mut e = a;
        for _ in 1..self.n {
            e = self.sq(e);
            r = self.mul(r, e);
        }
        r
    }
}
type P2 = Option<(u128, u128)>;
fn add2(f: &F2, a: u128, p: P2, q: P2) -> P2 {
    match (p, q) {
        (None, x) | (x, None) => x,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 {
                if y1 != y2 || x1 == 0 { return None; } // q = -p, or p of order 2
                let l = x1 ^ f.mul(y1, f.inv(x1));
                let x3 = f.sq(l) ^ l ^ a;
                let y3 = f.sq(x1) ^ f.mul(l ^ 1, x3);
                Some((x3, y3))
            } else {
                let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
                let x3 = f.sq(l) ^ l ^ x1 ^ x2 ^ a;
                let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
                Some((x3, y3))
            }
        }
    }
}
fn on2(f: &F2, a: u128, b: u128, p: (u128, u128)) -> bool {
    let (x, y) = p;
    f.sq(y) ^ f.mul(x, y) == f.mul(f.sq(x), x) ^ f.mul(a, f.sq(x)) ^ b
}
fn mul2(f: &F2, a: u128, k: u128, p: P2) -> P2 {
    let mut r: P2 = None;
    let mut q = p;
    let mut k = k;
    while k != 0 {
        if k & 1 == 1 { r = add2(f, a, r, q); }
        q = add2(f, a, q, q);
        k >>= 1;
    }
    r
}

// ---- GF(p) ----
fn mulmod(a: u128, b: u128, p: u128) -> u128 {
    // p < 2^63 is assumed, so a*b < 2^126 fits
    (a % p) * (b % p) % p
}
fn powmod(mut a: u128, mut e: u128, p: u128) -> u128 {
    let mut r = 1u128;
    a %= p;
    while e > 0 { if e & 1 == 1 { r = mulmod(r, a, p); } a = mulmod(a, a, p); e >>= 1; }
    r
}
type PP = Option<(u128, u128)>;
fn addp(p: u128, a: u128, x: PP, y: PP) -> PP {
    match (x, y) {
        (None, z) | (z, None) => z,
        (Some((x1, y1)), Some((x2, y2))) => {
            let l;
            if x1 == x2 {
                if (y1 + y2) % p == 0 { return None; }
                l = mulmod((3 * mulmod(x1, x1, p) + a) % p, powmod(2 * y1 % p, p - 2, p), p);
            } else {
                l = mulmod((y2 + p - y1) % p, powmod((x2 + p - x1) % p, p - 2, p), p);
            }
            let x3 = (mulmod(l, l, p) + 2 * p - x1 - x2) % p;
            let y3 = (mulmod(l, (x1 + p - x3) % p, p) + p - y1) % p;
            Some((x3, y3))
        }
    }
}
fn mulp(p: u128, a: u128, mut k: u128, pt: PP) -> PP {
    let mut r: PP = None; let mut q = pt;
    while k != 0 { if k & 1 == 1 { r = addp(p, a, r, q); } q = addp(p, a, q, q); k >>= 1; }
    r
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() { continue; }
        let t: Vec<&str> = line.split('\t').collect();
        let (id, kind, fp) = (t[0], t[1], t[2]);
        let (a, b) = (parse(t[3]), parse(t[4]));
        let g = (parse(t[5]), parse(t[6]));
        let q = (parse(t[7]), parse(t[8]));
        let d = parse(t[9]);
        let r = parse(t[10]);
        match kind {
            "fp" => {
                let p = parse(fp);
                let onc = |pt: (u128, u128)| mulmod(pt.1, pt.1, p) == (powmod(pt.0, 3, p) + mulmod(a, pt.0, p) + b) % p;
                let ok_on = onc(g) && onc(q);
                let rg = mulp(p, a, r, Some(g));
                let dg = mulp(p, a, d, Some(g));
                println!("{}\t{}\ton_curve={} rG_is_O={} dG_eq_Q={}", id, if ok_on && rg.is_none() && dg == Some(q) { "VERIFIED" } else { "FAILED" }, ok_on, rg.is_none(), dg == Some(q));
            }
            "gf2m" | "gf2m?" | "gf2mpoly" => {
                let (n, polys): (u32, Vec<u128>) = if kind == "gf2mpoly" { let mut it = fp.split(':'); let n: u32 = it.next().unwrap().parse().unwrap(); let poly = parse(it.next().unwrap()); (n, vec![poly ^ (1u128 << n)]) } else if kind == "gf2m" {
                    let mut it = fp.split(':');
                    let n: u32 = it.next().unwrap().parse().unwrap();
                    let mut red = 1u128;
                    for l in it.next().unwrap_or("").split(',').filter(|s| !s.is_empty()) {
                        red |= 1u128 << l.parse::<u32>().unwrap();
                    }
                    (n, vec![red])
                } else {
                    let n: u32 = fp.parse().unwrap();
                    let mut v = vec![];
                    for k in 1..n { v.push(1u128 | (1u128 << k)); }
                    for i in 1..n { for j in (i + 1)..n { for k in (j + 1)..n { v.push(1u128 | (1u128 << i) | (1u128 << j) | (1u128 << k)); } } }
                    (n, v)
                };
                let mut verdicts = vec![];
                for red in polys {
                    let f = F2 { n, red };
                    // quick on-curve filter for the unknown-polynomial search
                    if !on2(&f, a, b, g) || !on2(&f, a, b, q) { continue; }
                    let rg = mul2(&f, a, r, Some(g));
                    let dg = mul2(&f, a, d, Some(g));
                    let ok = rg.is_none() && dg == Some(q);
                    verdicts.push((red, ok));
                }
                if verdicts.is_empty() {
                    println!("{}\tFAILED\tno field polynomial puts G and Q on the curve", id);
                } else {
                    let good: Vec<_> = verdicts.iter().filter(|v| v.1).collect();
                    if good.len() == 1 && verdicts.len() == 1 || (kind == "gf2m?" && good.len() >= 1) {
                        println!("{}\tVERIFIED\tpoly_low_bits={:#x} candidates_on_curve={}", id, good[0].0, verdicts.len());
                    } else {
                        println!("{}\tFAILED\tcandidates_on_curve={} verified={}", id, verdicts.len(), good.len());
                    }
                }
            }
            _ => println!("{}\tFAILED\tunknown kind {}", id, kind),
        }
    }
}
