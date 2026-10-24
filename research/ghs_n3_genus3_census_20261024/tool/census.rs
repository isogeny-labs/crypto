//! Standalone study tool (protocol: ../PROTOCOL.md). Links the built
//! `crypto_lib` rlib; reimplements only exp/log tables, point counting by
//! the Artin-Schreier trace formula, factoring, Wilson intervals and SVG.
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::ec_trapdoor::*;
use num_bigint::BigUint;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::process::Command;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

struct Field {
    n: u32,
    poly: u32, // includes z^n
    m: u64,    // 2^n - 1
    exp: Vec<u32>,
    log: Vec<u32>,
    tl: Vec<u8>, // trace bit indexed by log
}
impl Field {
    fn new(n: u32) -> Field {
        let m = (1u64 << n) - 1;
        let mut poly = (1u32 << n) | 1;
        loop {
            let mut x = 1u32;
            let mut ord = 0u64;
            loop {
                x <<= 1;
                if (x >> n) & 1 == 1 { x ^= poly; }
                ord += 1;
                if x == 1 || ord > m { break; }
            }
            if x == 1 && ord == m { break; }
            poly += 2;
        }
        let mut exp = vec![0u32; m as usize];
        let mut log = vec![0u32; 1 << n];
        let mut x = 1u32;
        for i in 0..m {
            exp[i as usize] = x;
            log[x as usize] = i as u32;
            x <<= 1;
            if (x >> n) & 1 == 1 { x ^= poly; }
        }
        // trace mask: bit i = Tr(z^i)
        let tr = |v: u32| -> u32 {
            if v == 0 { return 0; }
            let lv = log[v as usize] as u64;
            let mut acc = 0u32;
            for j in 0..n {
                acc ^= exp[((lv << j) % m) as usize];
            }
            acc
        };
        let mut mask = 0u32;
        for i in 0..n {
            let t = tr(1 << i);
            assert!(t <= 1);
            mask |= t << i;
        }
        let tl: Vec<u8> = (0..m as usize).map(|i| ((exp[i] & mask).count_ones() & 1) as u8).collect();
        Field { n, poly, m, exp, log, tl }
    }
    fn irr(&self) -> IrreduciblePoly {
        let low: Vec<u32> = (0..self.n).filter(|i| (self.poly >> i) & 1 == 1).collect();
        IrreduciblePoly { degree: self.n, low_terms: low }
    }
    fn elem(&self, v: u32) -> F2mElement {
        F2mElement::from_biguint(&BigUint::from(v), self.n)
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 { return 0; }
        self.exp[((self.log[a as usize] as u64 + self.log[b as usize] as u64) % self.m) as usize]
    }
    /// c = #{x in K*: Tr(x)+Tr(b x^-2) = 0}; #E(a, Tr a = 0) = 2+2c, (Tr a = 1) = 2+2(M-c).
    fn c0(&self, lb: u64) -> u64 {
        let m = self.m as i64;
        let mut idx = lb as i64;
        let mut c = 0u64;
        for lx in 0..m {
            c += (self.tl[lx as usize] ^ self.tl[idx as usize] ^ 1) as u64;
            idx -= 2;
            if idx < 0 { idx += m; }
        }
        c
    }
}

fn largest_prime_and_v2(mut n: u64) -> (u64, u32, Vec<u64>) {
    let mut v2 = 0;
    let mut best = 1;
    let mut primes = vec![];
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; if p == 2 { v2 += 1; } }
            best = p;
            primes.push(p);
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if n > 1 { best = n; primes.push(n); }
    (best, v2, primes)
}
fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n { r -= 1; }
    while (r + 1) * (r + 1) <= n { r += 1; }
    r
}
fn wilson(k: u64, n: u64) -> (f64, f64) {
    if n == 0 { return (0.0, 1.0); }
    let z = 1.96f64;
    let nf = n as f64;
    let p = k as f64 / nf;
    let den = 1.0 + z * z / nf;
    let c = (p + z * z / (2.0 * nf)) / den;
    let h = z * (p * (1.0 - p) / nf + z * z / (4.0 * nf * nf)).sqrt() / den;
    (c - h, c + h)
}

#[derive(Default, Clone)]
struct Arm { n: u64, s1: u64, s2: u64 }
impl Arm {
    fn add(&mut self, e: u64) {
        let (r, _, _) = largest_prime_and_v2(e);
        self.n += 1;
        if r * r > e { self.s1 += 1; }
        if e % r == 0 && e / r <= 4 { self.s2 += 1; }
    }
    fn f1(&self) -> f64 { self.s1 as f64 / self.n.max(1) as f64 }
}

fn cls(g: &BigUint, ty: bool, m: u32) -> String {
    format!("m={} type={} genus={}", m, if ty { "I" } else { "II" }, g)
}

fn main() {
    let out = std::env::args().nth(1).expect("outdir");
    let ghs = std::env::args().nth(2).expect("path to ghs_screen");
    let mut log = String::new();
    macro_rules! say { ($($t:tt)*) => {{ let s = format!($($t)*); println!("{}", s); log.push_str(&s); log.push('\n'); }} }
    let mut q1_csv = String::from("l,N,moduli_hex,class,m,type,genus,observed,derived,match\n");
    let mut q2_csv = String::from("l,N,a_kind,class,count,cond8_true,fraction\n");
    let mut q3_csv = String::from("l,N,arm,a_kind,sample_n,s1_shaped,s1_frac,s1_lo,s1_hi,s2_cof_le4,s2_frac\n");
    let mut curves_csv = String::from("l,b,a_kind,order\n");
    let mut crosscheck = String::from("l,N,modulus,a,b,class,lib_m,lib_genus,lib_type,lib_c8,scr_m,scr_genus,scr_type,scr_c8,match\n");
    let mut graph_q1: Vec<(u32, [f64; 3])> = vec![];
    let mut graph_q3: BTreeMap<u32, Vec<(String, f64, f64, f64)>> = BTreeMap::new();
    let mut total_dev = 0u64;
    let mut cross_n = 0u32;
    let mut cross_bad = 0u32;

    for l in 2u32..=7 {
        let n = 3 * l;
        let f = Field::new(n);
        let m = f.m;
        let s = m / ((1u64 << l) - 1);
        let tower = FieldTower::new(n, 3, l, f.irr());
        say!("== l={} N={} modulus=0x{:x} s={} ==", l, n, f.poly, s);
        // a1: smallest element with absolute trace 1 (library), cross-checked with table
        let mut a1 = 1u32;
        while !absolute_trace(&tower, &f.elem(a1)) { a1 += 1; }
        assert_eq!(f.tl[f.log[a1 as usize] as usize], 1);
        let e_a0 = f.elem(0);
        let e_a1 = f.elem(a1);
        // ---- Q1/Q2 exhaustive
        let mut tally: BTreeMap<String, u64> = BTreeMap::new();
        let mut c8: BTreeMap<(String, u8), (u64, u64)> = BTreeMap::new();
        let mut dev = 0u64;
        let mut cls_of = vec![0u8; (m + 1) as usize]; // 0=A 1=B 2=C by value
        for b in 1..=m as u32 {
            let be = f.elem(b);
            let (g, ty) = ghs_genus_with_type(&tower, &be);
            let mm = magic_number(&tower, &e_a0, &be);
            let key = cls(&g, ty, mm);
            *tally.entry(key.clone()).or_default() += 1;
            let in_a = (f.log[b as usize] as u64) % s == 0;
            let in_b = relative_trace(&tower, &be).is_zero();
            let (pred, code) = if in_a { (cls(&BigUint::from(1u32), false, 1), 0u8) }
                else if in_b { (cls(&BigUint::from(3u32), true, 3), 1u8) }
                else { (cls(&BigUint::from(4u32), false, 3), 2u8) };
            if pred != key { dev += 1; }
            cls_of[b as usize] = code;
            let name = ["A","B","C"][code as usize].to_string();
            for (ak, ae) in [(0u8, &e_a0), (1u8, &e_a1)] {
                let e = c8.entry((name.clone(), ak)).or_default();
                e.0 += 1;
                if mmt_condition_8(&tower, ae, &be) { e.1 += 1; }
            }
        }
        total_dev += dev;
        let q = 1u64 << l;
        let derived = [("A", q - 1, "m=1 type=II genus=1"), ("B", q * q - 1, "m=3 type=I genus=3"), ("C", (q - 1) * (q * q - 1), "m=3 type=II genus=4")];
        let mut fr = [0.0; 3];
        for (i, (nm, cnt, key)) in derived.iter().enumerate() {
            let obs = *tally.get(*key).unwrap_or(&0);
            fr[i] = obs as f64 / m as f64;
            let _ = writeln!(q1_csv, "{},{},0x{:x},{},{},{},{},{},{},{}", l, n, f.poly, nm,
                key.split(' ').next().unwrap(), key.split(' ').nth(1).unwrap(), key.split(' ').nth(2).unwrap(), obs, cnt, obs == *cnt);
        }
        let other: u64 = tally.iter().filter(|(k, _)| !derived.iter().any(|d| d.2 == k.as_str())).map(|(_, v)| *v).sum();
        say!("Q1 l={} observed tally {:?}; other-class b = {}; per-b deviations from derived class = {}", l, tally, other, dev);
        for ((nm, ak), (cnt, t)) in &c8 {
            let _ = writeln!(q2_csv, "{},{},{},{},{},{},{:.6}", l, n, if *ak == 0 { "a=0" } else { "Tr(a)=1" }, nm, cnt, t, *t as f64 / *cnt as f64);
        }
        graph_q1.push((l, fr));

        // ---- independent brute-force check of the point-count formula (small N)
        if n <= 9 {
            let mut bad = 0;
            for b in 1..=m as u32 {
                for (ak, av) in [(0u8, 0u32), (1u8, a1)] {
                    // enumerate affine points
                    let mut cnt = 2u64; // infinity + (0, sqrt b) counted below
                    cnt = 1; // infinity
                    for x in 0..=m as u32 {
                        for y in 0..=m as u32 {
                            let lhs = f.mul(y, y) ^ f.mul(x, y);
                            let x2 = f.mul(x, x);
                            let rhs = f.mul(x2, x) ^ f.mul(av, x2) ^ b;
                            if lhs == rhs { cnt += 1; }
                        }
                    }
                    let c = f.c0(f.log[b as usize] as u64);
                    let formula = if ak == 0 { 2 + 2 * c } else { 2 + 2 * (m - c) };
                    if cnt != formula { bad += 1; }
                }
            }
            say!("BRUTE-FORCE affine-point check of count formula, l={} N={}: all b, both a: {} mismatches", l, n, bad);
            assert_eq!(bad, 0);
        }

        // ---- Q3
        let qq = 1u64 << n;
        let w = isqrt(4 * qq);
        let mut checks = (0u64, 0u64, 0u64, 0u64, 0u64); // hasse_bad, even_bad, mod4_bad, twist_bad, frob_bad
        let mut arms: BTreeMap<(&str, u8), Arm> = BTreeMap::new();
        let mut counts_b: BTreeMap<u64, (u64, u64)> = BTreeMap::new(); // lb -> (E0, E1)
        for b in 1..=m as u32 {
            if cls_of[b as usize] != 1 { continue; }
            let lb = f.log[b as usize] as u64;
            let c = f.c0(lb);
            let e0 = 2 + 2 * c;
            let e1 = 2 + 2 * (m - c);
            counts_b.insert(lb, (e0, e1));
        }
        for (&lb, &(e0, e1)) in &counts_b {
            // orbit representative
            let mut rep = lb;
            let mut x = lb;
            let mut osz = 0u64;
            let mut first = true;
            while first || x != lb {
                first = false;
                osz += 1;
                x = (x * 2) % m;
                if x < rep { rep = x; }
            }
            let _ = osz;
            for (ak, e) in [(0u8, e0), (1u8, e1)] {
                let d = if e > qq + 1 { e - qq - 1 } else { qq + 1 - e };
                if d * d > 4 * qq { checks.0 += 1; }
                if e % 2 != 0 { checks.1 += 1; }
                if (ak == 0 && e % 4 != 0) || (ak == 1 && e % 4 != 2) { checks.2 += 1; }
                arms.entry(("B", ak)).or_default().add(e);
                if rep == lb { arms.entry(("Brep", ak)).or_default().add(e); }
            }
            if e0 + e1 != 2 * qq + 2 { checks.3 += 1; }
            let (r0, r1) = counts_b[&rep];
            if r0 != e0 || r1 != e1 { checks.4 += 1; }
            for (ak, e) in [(0u8, e0), (1u8, e1)] { let _ = writeln!(curves_csv, "{},{},{},{}", l, lb, ak, e); }
        }
        say!("Q3 checks l={} (B arm, {} curves x 2 a): Hasse violations {}, odd #E {}, wrong #E mod 4 {}, twist-sum failures {}, Frobenius-orbit count mismatches {}",
            l, counts_b.len(), checks.0, checks.1, checks.2, checks.3, checks.4);
        // generic and null arms
        let mut rng = Rng(0x9E3779B97F4A7C15 ^ ((l as u64) << 32));
        let ns = 20000u64;
        for _ in 0..ns {
            let lb = rng.below(m);
            let c = f.c0(lb);
            arms.entry(("G", 0)).or_default().add(2 + 2 * c);
            arms.entry(("G", 1)).or_default().add(2 + 2 * (m - c));
        }
        for ak in 0..2u8 {
            let target = if ak == 0 { 0 } else { 2 };
            for _ in 0..ns {
                loop {
                    let d = rng.below(2 * w + 1) as i64 - w as i64;
                    let e = (qq as i64 + 1 + d) as u64;
                    if e % 4 == target { arms.entry(("I", ak)).or_default().add(e); break; }
                }
            }
        }
        // arm D: b in k*, literal criterion
        let (mut dn, mut dlit) = ([0u64; 2], [0u64; 2]);
        let mut dsh = [0u64; 2];
        for jb in 0..(q - 1) {
            let lb = jb * s;
            let c = f.c0(lb);
            // count over k
            let mut ck = 0u64;
            for jx in 0..(q - 1) {
                let lx = jx * s;
                let idx = ((lb as i64 - 2 * lx as i64).rem_euclid(m as i64)) as usize;
                ck += (f.tl[lx as usize] ^ f.tl[idx] ^ 1) as u64;
            }
            for ak in 0..2usize {
                let ek = if ak == 0 { 2 + 2 * c } else { 2 + 2 * (m - c) };
                let ekk = if ak == 0 { 2 + 2 * ck } else { 2 + 2 * ((q - 1) - ck) };
                let (_, _, ps) = largest_prime_and_v2(ek);
                dn[ak] += 1;
                if ps.iter().any(|&p| p * p > ek) { dsh[ak] += 1; }
                if ps.iter().any(|&p| p * p > ek && ekk % p != 0) { dlit[ak] += 1; }
                assert_eq!(ek % ekk, 0, "#E(k) must divide #E(K)");
            }
        }
        say!("Arm D (b in k*, l={}): n={}/{}; r_max>sqrt(#E(K)): {}/{}; literal (r|#E(K), r nmid #E(k), r>sqrt): {}/{} (a=0 / Tr(a)=1)",
            l, dn[0], dn[1], dsh[0], dsh[1], dlit[0], dlit[1]);
        for ak in 0..2usize {
            let _ = writeln!(q3_csv, "{},{},Dliteral,{},{},{},{:.4},,,,", l, n, if ak == 0 { "a=0" } else { "Tr(a)=1" }, dn[ak], dlit[ak], dlit[ak] as f64 / dn[ak] as f64);
        }
        let mut gl = vec![];
        for ((arm, ak), a) in &arms {
            let (lo, hi) = wilson(a.s1, a.n);
            let akn = if *ak == 0 { "a=0" } else { "Tr(a)=1" };
            let _ = writeln!(q3_csv, "{},{},{},{},{},{},{:.4},{:.4},{:.4},{},{:.4}", l, n, arm, akn, a.n, a.s1, a.f1(), lo, hi, a.s2, a.s2 as f64 / a.n as f64);
            say!("Q3 l={} arm={:<4} {:<8} n={:<6} S1 shaped={:<6} frac={:.4} [{:.4},{:.4}]  S2 (cofactor<=4) frac={:.4}", l, arm, akn, a.n, a.s1, a.f1(), lo, hi, a.s2 as f64 / a.n as f64);
            if *ak == 0 { gl.push((arm.to_string(), a.f1(), lo, hi)); }
        }
        graph_q3.insert(l, gl);
        // ---- cross-check against ghs_screen binary (3 <= l)
        if l >= 3 {
            let mut crng = Rng(0xC0FFEE ^ (l as u64));
            for i in 0..12u32 {
                let want = i % 4; // 0 random,1 B,2 A,3 random
                let b = loop {
                    let b = (1 + crng.below(m)) as u32;
                    let c = cls_of[b as usize];
                    if want == 1 && c != 1 { continue; }
                    if want == 2 && c != 0 { continue; }
                    break b;
                };
                let (ak, av) = if i % 2 == 0 { (0, 0u32) } else { (1, a1) };
                let _ = ak;
                let be = f.elem(b);
                let ae = f.elem(av);
                let (g, ty) = ghs_genus_with_type(&tower, &be);
                let mm = magic_number(&tower, &ae, &be);
                let c8v = mmt_condition_8(&tower, &ae, &be);
                let o = Command::new(&ghs)
                    .args(["--degree", &n.to_string(), "--modulus", &format!("0x{:x}", f.poly), "--a", &format!("0x{:x}", av), "--b", &format!("0x{:x}", b), "--genus-bound", "4"])
                    .output().expect("run ghs_screen");
                let txt = String::from_utf8_lossy(&o.stdout).to_string();
                let pos = txt.find("\"relative_degree\": 3,").expect("n=3 row");
                let blk = &txt[pos..(pos + 900).min(txt.len())];
                let get = |k: &str| -> String {
                    let p = blk.find(&format!("\"{}\": ", k)).unwrap() + k.len() + 4;
                    blk[p..].split(|c| c == ',' || c == '\n').next().unwrap().trim().trim_matches('"').to_string()
                };
                let (sm, sg, st, sc) = (get("magic_number"), get("genus"), get("ghs_type"), get("mmt_condition_8"));
                let ok = sm == mm.to_string() && sg == g.to_string() && (st == "I") == ty && (sc == "true") == c8v;
                cross_n += 1;
                if !ok { cross_bad += 1; }
                let _ = writeln!(crosscheck, "{},{},0x{:x},0x{:x},0x{:x},{},{},{},{},{},{},{},{},{},{}", l, n, f.poly, av, b,
                    ["A","B","C"][cls_of[b as usize] as usize], mm, g, if ty { "I" } else { "II" }, c8v, sm, sg, st, sc, ok);
            }
        }
    }

    // ---- Q2c: n = 2 detector positive control
    for l in 3u32..=5 {
        let n = 2 * l;
        let f = Field::new(n);
        let tower = FieldTower::new(n, 2, l, f.irr());
        let mut a1 = 1u32;
        while !absolute_trace(&tower, &f.elem(a1)) { a1 += 1; }
        let (mut t0, mut t1, mut tot) = (0u64, 0u64, 0u64);
        for b in 1..=f.m as u32 {
            let be = f.elem(b);
            tot += 1;
            if mmt_condition_8(&tower, &f.elem(0), &be) { t0 += 1; }
            if mmt_condition_8(&tower, &f.elem(a1), &be) { t1 += 1; }
        }
        let q = 1u64 << l;
        let exp_t1 = q * q - q; // derived: b with Tr_{K/k} b != 0
        say!("Q2c n=2 l={} N={}: cond8 true for a=0: {}/{}; for Tr(a)=1: {}/{} (derived expectation {}/{})", l, n, t0, tot, t1, tot, exp_t1, tot);
        let _ = writeln!(q2_csv, "{},{},n=2 a=0,all,{},{},{:.6}", l, n, tot, t0, t0 as f64 / tot as f64);
        let _ = writeln!(q2_csv, "{},{},n=2 Tr(a)=1,all,{},{},{:.6}", l, n, tot, t1, t1 as f64 / tot as f64);
        assert_eq!(t0, tot);
        assert_eq!(t1, exp_t1);
    }
    say!("TOTAL per-b class deviations from derived table (Q1): {}", total_dev);
    say!("Cross-check vs ghs_screen: rows={} mismatches={}", cross_n, cross_bad);
    fs::write(format!("{}/q1_census.csv", out), q1_csv).unwrap();
    fs::write(format!("{}/q2_cond8.csv", out), q2_csv).unwrap();
    fs::write(format!("{}/q3_shaped.csv", out), q3_csv).unwrap();
    fs::write(format!("{}/classB_orders.csv", out), curves_csv).unwrap();
    fs::write(format!("{}/crosscheck_ghs_screen.csv", out), crosscheck).unwrap();
    fs::write(format!("{}/run.log", out), &log).unwrap();
    fs::write(format!("{}/graph.svg", out), svg(&graph_q1, &graph_q3)).unwrap();
}

fn svg(q1: &[(u32, [f64; 3])], q3: &BTreeMap<u32, Vec<(String, f64, f64, f64)>>) -> String {
    let (w, h) = (1100.0, 480.0);
    let mut s = String::new();
    let _ = write!(s, "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {} {}' width='{}' height='{}' font-family='DejaVu Sans, Arial, sans-serif' font-size='13'>", w, h, w, h);
    let _ = write!(s, "<rect width='{}' height='{}' fill='white'/>", w, h);
    let _ = write!(s, "<text x='20' y='24' font-size='16' font-weight='bold'>Binary curves over F_(2^3l): class fractions and large-prime-subgroup rate</text>");
    let _ = write!(s, "<text x='20' y='42' fill='#444'>Left: fraction of b by class (measured, exhaustive; equals derived). Right: P(r_max &gt; sqrt #E), measured. ghs_n3_genus3_census_20261024</text>");
    let cols = ["#0072B2", "#D55E00", "#009E73", "#CC79A7", "#E69F00"];
    // left panel: x l in [1.7,7.3], y log10 in [-5,0]
    let (x0, x1, y0, y1) = (70.0, 500.0, 70.0, 400.0);
    let px = |l: f64| x0 + (l - 1.7) / 5.6 * (x1 - x0);
    let py = |v: f64| y1 - (v.log10() + 5.0) / 5.0 * (y1 - y0);
    let _ = write!(s, "<rect x='{}' y='{}' width='{}' height='{}' fill='none' stroke='#888'/>", x0, y0, x1 - x0, y1 - y0);
    for e in 0..=5 {
        let v = 10f64.powi(-e);
        let _ = write!(s, "<line x1='{}' x2='{}' y1='{:.1}' y2='{:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end' fill='#333'>1e-{}</text>", x0, x1, py(v), py(v), x0 - 6.0, py(v) + 4.0, e);
    }
    for l in 2..=7 {
        let _ = write!(s, "<text x='{:.1}' y='{}' text-anchor='middle' fill='#333'>{}</text>", px(l as f64), y1 + 18.0, l);
    }
    let _ = write!(s, "<text x='{}' y='{}' text-anchor='middle'>l  (N = 3l)</text>", (x0 + x1) / 2.0, y1 + 38.0);
    let _ = write!(s, "<text x='20' y='{}' transform='rotate(-90 20 {})' text-anchor='middle'>fraction of b in K*</text>", (y0 + y1) / 2.0, (y0 + y1) / 2.0);
    let names = ["A: b in k (m=1, g=1)", "B: Tr(b)=0 (m=3, g=3)", "C: rest (m=3, g=4)"];
    for c in 0..3 {
        let pts: Vec<String> = q1.iter().map(|(l, f)| format!("{:.1},{:.1}", px(*l as f64), py(f[c]))).collect();
        let _ = write!(s, "<polyline points='{}' fill='none' stroke='{}' stroke-width='2'/>", pts.join(" "), cols[c]);
        for (l, f) in q1 {
            let _ = write!(s, "<circle cx='{:.1}' cy='{:.1}' r='4' fill='{}'/>", px(*l as f64), py(f[c]), cols[c]);
        }
        let _ = write!(s, "<rect x='{}' y='{}' width='12' height='12' fill='{}'/><text x='{}' y='{}'>{}</text>", x0 + 12.0, y1 - 62.0 + 18.0 * c as f64, cols[c], x0 + 30.0, y1 - 51.0 + 18.0 * c as f64, names[c]);
    }
    let l4 = q1.iter().find(|(l, _)| *l == 4).unwrap().1[1];
    let _ = write!(s, "<text x='{:.1}' y='{:.1}' fill='#D55E00'>l=4: {:.2}%</text>", px(4.0) + 8.0, py(l4) - 8.0, l4 * 100.0);
    // right panel
    let (rx0, rx1) = (600.0, 1060.0);
    let rpx = |l: f64| rx0 + (l - 1.7) / 5.6 * (rx1 - rx0);
    let rpy = |v: f64| y1 - v / 1.0 * (y1 - y0);
    let _ = write!(s, "<rect x='{}' y='{}' width='{}' height='{}' fill='none' stroke='#888'/>", rx0, y0, rx1 - rx0, y1 - y0);
    for t in 0..=5 {
        let v = t as f64 / 5.0;
        let _ = write!(s, "<line x1='{}' x2='{}' y1='{:.1}' y2='{:.1}' stroke='#ddd'/><text x='{}' y='{:.1}' text-anchor='end' fill='#333'>{:.1}</text>", rx0, rx1, rpy(v), rpy(v), rx0 - 6.0, rpy(v) + 4.0, v);
    }
    for l in 2..=7 {
        let _ = write!(s, "<text x='{:.1}' y='{}' text-anchor='middle' fill='#333'>{}</text>", rpx(l as f64), y1 + 18.0, l);
    }
    let _ = write!(s, "<text x='{}' y='{}' text-anchor='middle'>l  (N = 3l), a = 0</text>", (rx0 + rx1) / 2.0, y1 + 38.0);
    let arms = [("B", "class B (all b)", 1usize), ("G", "generic b", 2), ("I", "null integers", 3)];
    for (ai, (key, label, ci)) in arms.iter().enumerate() {
        let off = (ai as f64 - 1.0) * 7.0;
        let mut pts = vec![];
        for (l, v) in q3 {
            if let Some((_, f, lo, hi)) = v.iter().find(|(k, ..)| k == key) {
                let x = rpx(*l as f64) + off;
                pts.push(format!("{:.1},{:.1}", x, rpy(*f)));
                let _ = write!(s, "<line x1='{:.1}' x2='{:.1}' y1='{:.1}' y2='{:.1}' stroke='{}' stroke-width='1.5'/><circle cx='{:.1}' cy='{:.1}' r='3.5' fill='{}'/>", x, x, rpy(*lo), rpy(*hi), cols[*ci], x, rpy(*f), cols[*ci]);
            }
        }
        let _ = write!(s, "<polyline points='{}' fill='none' stroke='{}' stroke-width='1' stroke-dasharray='3 3'/>", pts.join(" "), cols[*ci]);
        let _ = write!(s, "<rect x='{}' y='{}' width='12' height='12' fill='{}'/><text x='{}' y='{}'>{}</text>", rx0 + 12.0, y0 + 10.0 + 18.0 * ai as f64, cols[*ci], rx0 + 30.0, y0 + 21.0 + 18.0 * ai as f64, label);
    }
    let _ = write!(s, "<text x='{}' y='{}' transform='rotate(-90 {} {})' text-anchor='middle'>P(r_max &gt; sqrt #E)</text>", rx0 - 40.0, (y0 + y1) / 2.0, rx0 - 40.0, (y0 + y1) / 2.0);
    let _ = write!(s, "<text x='20' y='{}' fill='#444'>Bars: Wilson 95% (B: all curves, so correlated by Frobenius orbits; G and I: n = 20000). Toy fields; no claim about any deployed curve.</text>", h - 14.0);
    s.push_str("</svg>");
    s
}
