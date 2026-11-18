//! EXP-DSPEC-001: Fourier flatness of the m=2 decomposability indicator on
//! binary curves y^2+xy=x^3+ax^2+b over F_{2^n}, n <= 31. See ../PROTOCOL.md.
use rayon::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use std::io::Write;

#[derive(Clone, Copy)]
struct F { n: u32, f: u64 } // f includes z^n

impl F {
    fn mul(&self, a: u64, b: u64) -> u64 {
        let mut r: u64 = 0; let mut x = a; let mut y = b;
        while y != 0 { if y & 1 == 1 { r ^= x; } y >>= 1; x <<= 1; }
        self.red(r)
    }
    fn red(&self, mut r: u64) -> u64 {
        let n = self.n;
        while r >> n != 0 { let d = 63 - r.leading_zeros(); r ^= self.f << (d - n); }
        r
    }
    fn sq(&self, a: u64) -> u64 { self.mul(a, a) }
    fn inv(&self, a: u64) -> u64 {
        // extended Euclid over F2[z]
        assert!(a != 0);
        let (mut u, mut v, mut g1, mut g2) = (a, self.f, 1u64, 0u64);
        while u != 1 {
            let mut j = (63 - u.leading_zeros()) as i32 - (63 - v.leading_zeros()) as i32;
            if j < 0 { std::mem::swap(&mut u, &mut v); std::mem::swap(&mut g1, &mut g2); j = -j; }
            u ^= v << j; g1 ^= g2 << j;
        }
        self.red(g1)
    }
    fn pow2k(&self, a: u64, k: u32) -> u64 { let mut x = a; for _ in 0..k { x = self.sq(x); } x }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum P { O, A(u64, u64) }

struct Curve { k: F, a: u64, b: u64 }
impl Curve {
    fn neg(&self, p: P) -> P { match p { P::O => P::O, P::A(x, y) => P::A(x, x ^ y) } }
    fn add(&self, p: P, q: P) -> P {
        let k = &self.k;
        match (p, q) {
            (P::O, _) => q, (_, P::O) => p,
            (P::A(x1, y1), P::A(x2, y2)) => {
                if x1 == x2 {
                    if y1 ^ y2 == x2 || x1 == 0 { return P::O; } // q = -p (or 2-torsion doubling)
                    // doubling
                    let l = x1 ^ k.mul(y1, k.inv(x1));
                    let x3 = k.sq(l) ^ l ^ self.a;
                    let y3 = k.sq(x1) ^ k.mul(l ^ 1, x3);
                    return P::A(x3, y3);
                }
                let l = k.mul(y1 ^ y2, k.inv(x1 ^ x2));
                let x3 = k.sq(l) ^ l ^ self.a ^ x1 ^ x2;
                let y3 = k.mul(l, x1 ^ x3) ^ x3 ^ y1;
                P::A(x3, y3)
            }
        }
    }
    fn on(&self, p: P) -> bool {
        match p { P::O => true, P::A(x, y) => { let k = &self.k;
            k.sq(y) ^ k.mul(x, y) == k.mul(k.sq(x), x) ^ k.mul(self.a, k.sq(x)) ^ self.b } }
    }
    fn smul(&self, mut e: u64, p: P) -> P {
        let mut r = P::O; let mut q = p;
        while e > 0 { if e & 1 == 1 { r = self.add(r, q); } q = self.add(q, q); e >>= 1; }
        r
    }
}

fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *s; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31)
}
fn is_prime(n: u64) -> bool { if n < 2 { return false; } let mut d = 2; while d * d <= n { if n % d == 0 { return false; } d += 1; } true }

fn trace_mask(k: &F) -> u64 {
    let mut m = 0;
    for i in 0..k.n { let z = 1u64 << i; let mut t = 0; let mut x = z; for _ in 0..k.n { t ^= x; x = k.sq(x); } assert!(t <= 1); if t == 1 { m |= z; } }
    m
}
fn tr(m: u64, x: u64) -> u64 { ((m & x).count_ones() & 1) as u64 }
fn half_trace(k: &F, c: u64) -> u64 { let mut h = 0; let mut x = c; for _ in 0..=(k.n - 1) / 2 { h ^= x; x = k.sq(k.sq(x)); } h }

fn count(c: &Curve, tm: u64) -> u64 {
    let k = &c.k; let q = 1u64 << k.n;
    let s: u64 = (1..q).into_par_iter().map(|x| { let t = x ^ c.a ^ k.mul(c.b, k.sq(k.inv(x))); if tr(tm, t) == 0 { 2 } else { 0 } }).sum();
    2 + s
}

// F2-linear algebra on u64 row vectors
fn span(basis: &[u64]) -> Vec<u64> {
    let mut v = vec![0u64];
    for &b in basis { let l = v.len(); for i in 0..l { v.push(v[i] ^ b); } }
    v
}
fn rank(vs: &[u64]) -> usize {
    let mut rows: Vec<u64> = vs.to_vec(); let mut r = 0;
    for bit in (0..64).rev() {
        if let Some(p) = (r..rows.len()).find(|&i| rows[i] >> bit & 1 == 1) {
            rows.swap(r, p); for i in 0..rows.len() { if i != r && rows[i] >> bit & 1 == 1 { rows[i] ^= rows[r]; } } r += 1;
        }
    }
    r
}
/// kernel of linear map L given by images L(z^i) = cols[i]
fn kernel(n: u32, cols: &[u64]) -> Vec<u64> {
    // Gaussian elimination on augmented [image | identity]
    let mut rows: Vec<(u64, u64)> = (0..n as usize).map(|i| (cols[i], 1u64 << i)).collect();
    let mut r = 0;
    for bit in (0..n).rev() {
        if let Some(p) = (r..rows.len()).find(|&i| rows[i].0 >> bit & 1 == 1) {
            rows.swap(r, p); let pr = rows[r];
            for i in 0..rows.len() { if i != r && rows[i].0 >> bit & 1 == 1 { rows[i].0 ^= pr.0; rows[i].1 ^= pr.1; } } r += 1;
        }
    }
    rows[r..].iter().map(|x| { assert!(x.0 == 0); x.1 }).collect()
}
fn polymod(mut a: u64, b: u64) -> u64 { let db = 63 - b.leading_zeros(); while a != 0 && 63 - a.leading_zeros() >= db { a ^= b << (63 - a.leading_zeros() - db); } a }
fn polymul(a: u64, b: u64) -> u64 { let mut r = 0; let mut x = a; let mut y = b; while y != 0 { if y & 1 == 1 { r ^= x; } y >>= 1; x <<= 1; } r }

fn frob_subspace(k: &F) -> Option<(u64, Vec<u64>)> {
    let n = k.n as u64;
    let mut e = 1; let mut t = 2 % n; while t != 1 { t = t * 2 % n; e += 1; }
    if e == n - 1 { return None; }
    let phi = (1u64 << n) - 1 >> 0; // z^{n-1}+...+1 : bits 0..n-1
    let phi = phi & ((1u64 << n) - 1);
    let g = ((1u64 << e)..(1u64 << (e + 1))).find(|&g| polymod(phi, g) == 0).unwrap();
    let h = polymul(g, 0b11); // (s+1) g(s)
    let cols: Vec<u64> = (0..k.n).map(|i| { let z = 1u64 << i; let mut acc = 0; let mut x = z;
        for j in 0..64 { if h >> j & 1 == 1 { acc ^= x; } x = k.sq(x); } acc }).collect();
    Some((g, kernel(k.n, &cols)))
}

struct Ctx { n: u32, nn: u64, half: u64, xidx: Vec<u32> }

fn fold(nn: u64, k: u64) -> u64 { let k = k % nn; if k > nn / 2 { nn - k } else { k } }

fn fb_from_x(ctx: &Ctx, xs: &[u64]) -> Vec<u64> {
    let mut v: Vec<u64> = xs.iter().filter_map(|&x| { let i = ctx.xidx[x as usize]; if i > 0 { Some(i as u64) } else { None } }).collect();
    v.sort(); v.dedup(); v
}
fn decomp(ctx: &Ctx, fb: &[u64]) -> Vec<bool> {
    let nn = ctx.nn; let mut d = vec![false; (ctx.half + 1) as usize];
    for i in 0..fb.len() { for j in i..fb.len() {
        d[fold(nn, fb[i] + fb[j]) as usize] = true; d[fold(nn, nn + fb[i] - fb[j]) as usize] = true; } }
    d[0] = false; d
}
fn zstat(ctx: &Ctx, d: &[bool]) -> (u64, f64, u64) {
    let nn = ctx.nn as usize;
    let mut buf: Vec<Complex<f64>> = (0..nn).map(|k| { let f = fold(ctx.nn, k as u64) as usize; Complex::new(if k > 0 && d[f] { 1.0 } else { 0.0 }, 0.0) }).collect();
    let card = d.iter().filter(|&&b| b).count() as u64;
    let mut pl = FftPlanner::new(); let fft = pl.plan_fft_forward(nn); fft.process(&mut buf);
    let (mut best, mut arg) = (0.0f64, 0u64);
    for a in 1..nn { let m = buf[a].norm(); if m > best { best = m; arg = a as u64; } }
    let full = 2.0 * card as f64; // indicator on Z/N has 2|D| ones
    let var = full * (1.0 - full / nn as f64);
    (card, best / var.sqrt(), arg)
}
fn rand_idx(ctx: &Ctx, size: usize, seed: u64, exclude: &[u64]) -> Vec<u64> {
    let mut s = seed; let mut set = std::collections::BTreeSet::new();
    for &e in exclude { set.insert(e); }
    while set.len() < size + exclude.len() { let k = 1 + splitmix(&mut s) % ctx.half; set.insert(k); }
    set.into_iter().filter(|k| !exclude.contains(k)).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args[1].parse().unwrap();
    let out = &args[2];
    let env = |k: &str, d: u64| std::env::var(k).ok().map(|v| v.parse::<u64>().unwrap()).unwrap_or(d);
    let nulls: u64 = env("DSPEC_NULLS", 20);
    let salt: u64 = env("DSPEC_SALT", 0);
    let (rs0, rs1) = (env("DSPEC_RSUB_FROM", 1), env("DSPEC_RSUB_TO", 5));
    let rep = std::env::var("DSPEC_ONLY_CURVE").ok();
    let fpoly: u64 = match n { 17 => (1 << 17) | 0b1001, 19 => (1 << 19) | 0b100111, 23 => (1 << 23) | 0b100001,
        _ => panic!("n not in protocol") };
    let k = F { n, f: fpoly };
    // irreducibility: z^(2^n) == z and n prime
    assert!(is_prime(n as u64) && k.pow2k(2, n) == 2, "modulus not irreducible");
    let tm = trace_mask(&k);
    let mut curves: Vec<(String, u64, u64)> = vec![("K0".into(), 0, 1), ("K1".into(), 1, 1)];
    let mut s = 0u64; let mut i = 0u64; let mut found = 0;
    let mut cand = vec![];
    while found < 3 {
        let mut st = n as u64 * 1000 + i; let b = splitmix(&mut st) % (1u64 << n); i += 1;
        if b == 0 { continue; }
        let c = Curve { k, a: 1, b }; let ne = count(&c, tm);
        if ne % 2 == 0 && is_prime(ne / 2) { found += 1; cand.push((format!("R{}", found), 1u64, b)); }
        s += 1; let _ = s;
    }
    curves.extend(cand);
    let mut fo = std::fs::File::create(out).unwrap();
    for (name, a, b) in curves {
        if let Some(oc) = &rep { if &name != oc { continue; } }
        let c = Curve { k, a, b }; let ne = count(&c, tm);
        let (h, nn) = if ne % 2 == 0 && is_prime(ne / 2) { (2, ne / 2) } else if ne % 4 == 0 && is_prime(ne / 4) { (4, ne / 4) } else {
            writeln!(fo, "{{\"n\":{},\"curve\":\"{}\",\"a\":{},\"b\":{},\"order\":{},\"skipped\":\"order not h*prime\"}}", n, name, a, b, ne).unwrap(); continue; };
        // base point
        let mut g = P::O;
        for x in 1..(1u64 << n) { let t = x ^ a ^ k.mul(b, k.sq(k.inv(x))); if tr(tm, t) != 0 { continue; }
            let z = half_trace(&k, t); let p = P::A(x, k.mul(x, z)); assert!(c.on(p));
            let q = c.smul(h, p); if q != P::O { g = q; break; } }
        assert!(c.smul(nn, g) == P::O && g != P::O);
        let half = (nn - 1) / 2;
        let mut xidx = vec![0u32; 1usize << n];
        let mut p = g;
        for kk in 1..=half { if let P::A(x, _) = p { assert!(xidx[x as usize] == 0, "x collision"); xidx[x as usize] = kk as u32; } else { panic!() } p = c.add(p, g); }
        let ctx = Ctx { n, nn, half, xidx };
        let gx = if let P::A(x, _) = g { x } else { 0 };
        let lam = ctx.xidx[k.sq(gx) as usize] as u64; // fold(lambda) if Frobenius is an endomorphism (K0/K1)
        let koblitz = b == 1 && a <= 1;
        // arms
        let d = (n + 1) / 2;
        let mut arms: Vec<(String, u64, Vec<u64>, String)> = vec![];
        let poly: Vec<u64> = span(&(0..d).map(|i| 1u64 << i).collect::<Vec<_>>());
        let fbp = fb_from_x(&ctx, &poly); let bsz = fbp.len();
        arms.push(("poly".into(), 0, fbp.clone(), format!("dim={}", d)));
        if let Some((gpoly, ker)) = frob_subspace(&k) {
            let fbf = fb_from_x(&ctx, &span(&ker));
            let mut note = format!("dim={} g={:#b}", ker.len(), gpoly);
            if koblitz { let bad = fbf.iter().filter(|&&q| fbf.binary_search(&fold(nn, (q as u128 * lam as u128 % nn as u128) as u64)).is_err()).count();
                note += &format!(" lambda={} invariance_violations={}", lam, bad); }
            arms.push(("frob".into(), 0, fbf, note));
        }
        for sd in rs0..=rs1 {
            let mut st = 0xD5EC_0000 + n as u64 * 100 + sd; let mut basis = vec![];
            while basis.len() < d as usize { let v = splitmix(&mut st) % (1u64 << n); let mut t = basis.clone(); t.push(v); if rank(&t) == t.len() { basis = t; } }
            arms.push(("rsub".into(), sd, fb_from_x(&ctx, &span(&basis)), format!("dim={}", d)));
        }
        arms.push(("pint".into(), 0, (1..=bsz as u64).collect(), "interval".into()));
        let ni = bsz / 10; let intv: Vec<u64> = (1..=ni as u64).collect();
        let mut mix = rand_idx(&ctx, bsz - ni, 0xA11CE, &intv); mix.extend(intv.iter()); mix.sort();
        arms.push(("pmix".into(), 1, mix, format!("interval_part={}", ni)));
        // jobs: each arm + its matched nulls
        let mut jobs: Vec<(String, u64, u64, Vec<u64>, String)> = vec![];
        if rep.is_some() { arms.retain(|x| x.0 == "rsub"); }
        for (ai, (an, sd, fb, note)) in arms.iter().enumerate() {
            jobs.push((an.clone(), *sd, u64::MAX, fb.clone(), note.clone()));
            if an == "poly" || an == "frob" || an == "rsub" {
                for ns in 1..=nulls { let seed = 0x0000_5EED_0000 + (n as u64) * 1_000_000 + (ai as u64) * 1000 + ns + a * 100_000 + (b % 997) * 7 + salt * 0x1_0000_0000 + sd * 7919 * (salt > 0) as u64;
                    jobs.push((format!("null:{}:{}", an, sd), ns, ai as u64, rand_idx(&ctx, fb.len(), seed, &[]), String::new())); }
            }
        }
        let rows: Vec<String> = jobs.par_iter().map(|(an, sd, _ai, fb, note)| {
            let d = decomp(&ctx, fb); let (card, z, arg) = zstat(&ctx, &d);
            format!("{{\"n\":{},\"curve\":\"{}\",\"a\":{},\"b\":{},\"h\":{},\"N\":{},\"arm\":\"{}\",\"seed\":{},\"fb\":{},\"D\":{},\"Z\":{:.6},\"argmax\":{},\"note\":\"{}\"}}",
                ctx.n, name, a, b, h, nn, an, sd, fb.len(), card, z, arg, note)
        }).collect();
        for r in rows { writeln!(fo, "{}", r).unwrap(); }
        eprintln!("done n={} {} N={} h={} fb={}", n, name, nn, h, bsz);
    }
}
