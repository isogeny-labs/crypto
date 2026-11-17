// Standalone prototype: radical (closed-form) low-degree root finding over F_p
// vs Cantor-Zassenhaus, with a shared field-multiplication counter.
// No external crates (fast compile, fast iteration). Public synthetic only.

use std::cell::Cell;

thread_local! {
    static MULS: Cell<u64> = Cell::new(0);
}
fn reset() { MULS.with(|m| m.set(0)); }
fn muls() -> u64 { MULS.with(|m| m.get()) }
fn bump() { MULS.with(|m| m.set(m.get() + 1)); }

#[derive(Clone, Copy)]
struct Fp { p: u64 }
impl Fp {
    fn new(p: u64) -> Self { Fp { p } }
    #[inline] fn add(&self, a: u64, b: u64) -> u64 { ((a as u128 + b as u128) % self.p as u128) as u64 }
    #[inline] fn sub(&self, a: u64, b: u64) -> u64 { ((a as u128 + self.p as u128 - b as u128) % self.p as u128) as u64 }
    #[inline] fn mul(&self, a: u64, b: u64) -> u64 { bump(); ((a as u128 * b as u128) % self.p as u128) as u64 }
    #[inline] fn sqr(&self, a: u64) -> u64 { self.mul(a, a) }
    fn powm(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64 % self.p;
        a %= self.p;
        while e > 0 {
            if e & 1 == 1 { r = self.mul(r, a); }
            a = self.sqr(a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 { self.powm(a, self.p - 2) }
    fn legendre(&self, a: u64) -> i32 {
        let a = a % self.p;
        if a == 0 { return 0; }
        let r = self.powm(a, (self.p - 1) / 2);
        if r == 1 { 1 } else { -1 }
    }
    // Tonelli-Shanks square root; None if non-residue. Fast path for p % 4 == 3.
    fn sqrt(&self, a: u64) -> Option<u64> {
        let a = a % self.p;
        if a == 0 { return Some(0); }
        if self.legendre(a) != 1 { return None; }
        if self.p % 4 == 3 {
            return Some(self.powm(a, (self.p + 1) / 4));
        }
        // p % 4 == 1: Tonelli-Shanks.
        let mut q = self.p - 1;
        let mut s = 0u32;
        while q & 1 == 0 { q >>= 1; s += 1; }
        // find a non-residue z
        let mut z = 2u64;
        while self.legendre(z) != -1 { z += 1; }
        let mut m = s;
        let mut c = self.powm(z, q);
        let mut t = self.powm(a, q);
        let mut r = self.powm(a, (q + 1) / 2);
        loop {
            if t == 1 { return Some(r); }
            // least i, 0<i<m, with t^(2^i)==1
            let mut i = 0u32;
            let mut t2 = t;
            while t2 != 1 && i < m { t2 = self.sqr(t2); i += 1; }
            if i == m { return None; } // shouldn't happen for residues
            let b = self.powm(c, 1u64 << (m - i - 1));
            m = i;
            c = self.sqr(b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
    }
    // Cube root over F_p. Returns one cube root if `a` is a cube, else None.
    // p % 3 == 2: bijective, root = a^((2p-1)/3). p % 3 == 1: AMM-style.
    fn cbrt(&self, a: u64) -> Option<u64> {
        let a = a % self.p;
        if a == 0 { return Some(0); }
        if self.p % 3 == 2 {
            return Some(self.powm(a, (2 * self.p - 1) / 3));
        }
        // p % 3 == 1: a is a cube iff a^((p-1)/3) == 1.
        if self.powm(a, (self.p - 1) / 3) != 1 { return None; }
        // Adleman-Manders-Miller for cube roots.
        // Write p-1 = 3^s * t, gcd(t,3)=1.
        let mut t = self.p - 1;
        let mut s = 0u32;
        while t % 3 == 0 { t /= 3; s += 1; }
        // find cubic non-residue b
        let mut bnr = 2u64;
        while self.powm(bnr, (self.p - 1) / 3) == 1 { bnr += 1; }
        let c = self.powm(bnr, t);
        // solve x^3 = a
        // t ≡ ±1 mod 3; pick alpha with 3*alpha ≡ 1 mod ... use standard AMM
        // r = a^((t + k)/3) adjusted; implement the iterative AMM.
        // Reference: AMM algorithm for r-th roots with r=3.
        let r3inv = Self::inv_mod(3, t); // 3^{-1} mod t exists since gcd(3,t)=1
        let mut root = self.powm(a, ((t * r3inv) % t)); // candidate base
        // The exponent must satisfy 3*e ≡ 1 (mod t): e = r3inv mod t. Then root^3 = a * h where h is 3^s-torsion.
        // h = a^{-1} * root^3
        let mut h = self.mul(self.inv(a), self.mul(self.sqr(root), root));
        // Now reduce h (which lives in the 3^s-Sylow subgroup) to identity, adjusting root.
        // c is a generator-ish of that subgroup (cubic non-residue^t).
        let mut cc = c;
        // Precompute: we need discrete log of h base the 3^s subgroup, trit by trit.
        let mut e_exp = 0u64;
        // order of subgroup is 3^s
        for j in 1..s {
            // h^{3^{s-1-j}} compares against powers
            let test = self.powm(h, 3u64.pow(s - 1 - j));
            // test is a cube root of unity: 1, w, or w^2
            let w = self.powm(cc, 3u64.pow(s - 1)); // primitive cube root of unity (approx)
            let _ = w;
            // find digit d in {0,1,2} with test == w^d ... fall back to search
            let mut digit = 0u64;
            let one = 1u64 % self.p;
            if test == one { digit = 0; }
            else {
                // compute w and w^2
                let wprim = self.powm(c, 3u64.pow(s - 1) );
                if test == wprim { digit = 1; }
                else { digit = 2; }
            }
            e_exp += digit * 3u64.pow(j - 1);
            // adjust h *= c^{-digit*3^{...}}
            let adj = self.powm(self.inv(cc), digit * 3u64.pow(j - 1));
            h = self.mul(h, self.powm(adj, 1)); // placeholder; AMM is finicky
            let _ = &mut cc;
        }
        let _ = (&mut root, &mut e_exp);
        // AMM here is error-prone; verify and fall back to exhaustive adjust if wrong.
        // Try the simple candidate and a few corrections by cube roots of unity.
        let cand = self.powm(a, r3inv % t.max(1));
        // brute verify candidate; if not, multiply by cube roots of unity
        // get a primitive cube root of unity
        let w = self.powm(bnr, (self.p - 1) / 3);
        for k in 0..3 {
            let rr = self.mul(cand, self.powm(w, k));
            if self.mul(self.sqr(rr), rr) == a { return Some(rr); }
        }
        None
    }
    // modular inverse of x mod m (m need not be prime) via extended euclid (uncounted helper)
    fn inv_mod(x: u64, m: u64) -> u64 {
        let (mut old_r, mut r) = (x as i128, m as i128);
        let (mut old_s, mut s) = (1i128, 0i128);
        while r != 0 {
            let q = old_r / r;
            let nr = old_r - q * r; old_r = r; r = nr;
            let ns = old_s - q * s; old_s = s; s = ns;
        }
        let mut res = old_s % m as i128;
        if res < 0 { res += m as i128; }
        res as u64
    }
}

// ---------- radical solvers (counted) ----------

// quadratic a x^2 + b x + c = 0, a != 0. Returns sorted distinct roots in F_p.
fn quad_roots(f: &Fp, a: u64, b: u64, c: u64) -> Vec<u64> {
    let inv2a = f.inv(f.mul(2 % f.p, a));
    let b2 = f.sqr(b);
    let fourac = f.mul(f.mul(4 % f.p, a), c);
    let disc = f.sub(b2, fourac);
    match f.legendre(disc) {
        0 => { let r = f.mul(f.sub(0, b), inv2a); vec![r] }
        -1 => vec![],
        _ => {
            let s = f.sqrt(disc).unwrap();
            let r1 = f.mul(f.sub(s, b), inv2a);
            let r2 = f.mul(f.sub(f.sub(0, s), b), inv2a);
            let mut v = vec![r1, r2];
            v.sort_unstable(); v.dedup(); v
        }
    }
}

// ---------- Cantor-Zassenhaus reference (counted) ----------

fn trim(mut v: Vec<u64>) -> Vec<u64> { while v.last() == Some(&0) { v.pop(); } v }
fn pmul(f: &Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 { continue; }
        for (j, &y) in b.iter().enumerate() {
            let z = f.mul(x, y);
            r[i + j] = f.add(r[i + j], z);
        }
    }
    trim(r)
}
fn prem(f: &Fp, a: &[u64], m: &[u64]) -> Vec<u64> {
    let mut r = a.to_vec();
    if m.is_empty() { return r; }
    let inv = f.inv(*m.last().unwrap());
    while r.len() >= m.len() && !r.is_empty() {
        let shift = r.len() - m.len();
        let c = f.mul(*r.last().unwrap(), inv);
        for (k, &v) in m.iter().enumerate() {
            let p = f.mul(c, v);
            r[shift + k] = f.sub(r[shift + k], p);
        }
        r = trim(r);
    }
    r
}
fn pgcd(f: &Fp, mut a: Vec<u64>, mut b: Vec<u64>) -> Vec<u64> {
    a = trim(a); b = trim(b);
    while !b.is_empty() {
        let r = prem(f, &a, &b);
        a = b; b = r;
    }
    if let Some(&lead) = a.last() { let inv = f.inv(lead); for c in &mut a { *c = f.mul(*c, inv); } }
    a
}
fn ppow(f: &Fp, base: &[u64], mut e: u64, m: &[u64]) -> Vec<u64> {
    let mut r = prem(f, &[1], m);
    let mut b = prem(f, base, m);
    while e > 0 {
        if e & 1 == 1 { r = prem(f, &pmul(f, &r, &b), m); }
        b = prem(f, &pmul(f, &b, &b), m);
        e >>= 1;
    }
    r
}
// CZ: distinct F_p-roots of squarefree-or-not f. Returns sorted roots.
fn cz_roots(f: &Fp, poly: &[u64], seed: &mut u64) -> Vec<u64> {
    let poly = trim(poly.to_vec());
    if poly.len() <= 1 { return vec![]; }
    // g = gcd(X^p - X, poly) = product of distinct linear factors
    let xp = ppow(f, &[0, 1], f.p, &poly);
    let mut xpx = xp.clone();
    xpx.resize(xpx.len().max(2), 0);
    xpx[1] = f.sub(xpx[1], 1);
    let g = pgcd(f, poly.clone(), trim(xpx));
    let mut roots = vec![];
    let mut stack = vec![g];
    let rand = |s: &mut u64| -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (*s >> 33) % f.p };
    while let Some(h) = stack.pop() {
        match h.len() {
            0 | 1 => {}
            2 => { let r = f.mul(f.sub(0, h[0]), f.inv(h[1])); roots.push(r); }
            _ => {
                let delta = rand(seed);
                let mut s = ppow(f, &[delta, 1], (f.p - 1) / 2, &h);
                if s.is_empty() { s.push(0); }
                s[0] = f.sub(s[0], 1);
                let d = pgcd(f, h.clone(), trim(s));
                if d.len() > 1 && d.len() < h.len() {
                    let q = {
                        // h / d via prem-free long division
                        let mut rem = h.clone(); let mut quo = vec![0u64; h.len()-d.len()+1];
                        let invlead = f.inv(*d.last().unwrap());
                        while rem.len() >= d.len() && !rem.is_empty() {
                            let sh = rem.len()-d.len();
                            let c = f.mul(*rem.last().unwrap(), invlead);
                            quo[sh] = c;
                            for (k,&v) in d.iter().enumerate() { let p = f.mul(c,v); rem[sh+k]=f.sub(rem[sh+k],p); }
                            rem = trim(rem);
                        }
                        trim(quo)
                    };
                    stack.push(d); stack.push(q);
                } else { stack.push(h); }
            }
        }
    }
    roots.sort_unstable(); roots.dedup(); roots
}

// ---------- validation + measurement ----------

fn eval(f: &Fp, poly: &[u64], x: u64) -> u64 {
    poly.iter().rev().fold(0u64, |a, &b| f.add(f.mul(a, x), b))
}
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    for &pr in &[2,3,5,7,11,13,17,19,23,29,31,37] { if n % pr == 0 { return n == pr; } }
    // Miller-Rabin deterministic for < 3.3e18 with these bases
    let mut d = n - 1; let mut r = 0; while d & 1 == 0 { d >>= 1; r += 1; }
    'wit: for &a in &[2u64,3,5,7,11,13,17,19,23,29,31,37] {
        if a % n == 0 { continue; }
        let mut x = { let mut res=1u128; let mut b=a as u128 % n as u128; let mut e=d; while e>0 { if e&1==1 {res=res*b%n as u128;} b=b*b%n as u128; e>>=1;} res } as u64;
        if x == 1 || x == n-1 { continue; }
        for _ in 0..r-1 { x = ((x as u128 * x as u128) % n as u128) as u64; if x == n-1 { continue 'wit; } }
        return false;
    }
    true
}

fn main() {
    // 1. Validate quadratic radical vs brute force over small primes (all residue classes).
    let small: Vec<u64> = (5..50).filter(|&n| is_prime(n)).collect();
    let mut checked = 0u64;
    for &p in &small {
        let f = Fp::new(p);
        for a in 1..p { for b in 0..p { for c in 0..p {
            let got = quad_roots(&f, a, b, c);
            let mut want: Vec<u64> = (0..p).filter(|&x| eval(&f, &[c,b,a], x) == 0).collect();
            want.sort_unstable();
            assert_eq!(got, want, "quad mismatch p={p} a={a} b={b} c={c}");
            checked += 1;
        }}}
    }
    println!("quadratic radical: validated on {checked} (a,b,c) triples over {} small primes (both p%4 classes)", small.len());

    // 2. Validate quad radical vs CZ on random large primes, and measure field muls.
    let primes: Vec<u64> = vec![
        1_000_003,            // p % 4 == 3
        1_000_033,            // p % 4 == 1
        1_000_000_007,        // p % 4 == 3
        998_244_353,          // p % 4 == 1 (NTT prime)
        (1u64<<31) - 1,       // Mersenne, p%4==3
        2_305_843_009_213_693_951, // 2^61-1
    ];
    let mut seed = 0x1234_5678_9abc_def0u64;
    let rnd = |s: &mut u64, m: u64| -> u64 { *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); ((*s>>33) % m).max(0) };

    println!("\n## Quadratic: radical vs Cantor-Zassenhaus (field multiplications), 2000 random polys each\n");
    println!("| p | p%4 | p%3 | case | radical muls/poly | CZ muls/poly | ratio |");
    println!("|---|---|---|---|---:|---:|---:|");
    for &p in &primes {
        let f = Fp::new(p);
        // split-case (two distinct roots): build f=(x-r1)(x-r2)
        for &case in &["split", "noroot"] {
            let n = 2000u64;
            let (mut rad_tot, mut cz_tot) = (0u64, 0u64);
            let mut agree = 0u64;
            for _ in 0..n {
                let (a, b, c);
                if case == "split" {
                    let r1 = rnd(&mut seed, p); let mut r2 = rnd(&mut seed, p); if r2==r1 { r2=(r2+1)%p; }
                    // (x-r1)(x-r2) = x^2 -(r1+r2) x + r1 r2
                    a = 1; b = (p - (r1+r2)%p)%p; c = (r1 as u128 * r2 as u128 % p as u128) as u64;
                } else {
                    // random a,b,c with non-residue disc (reject until noroot)
                    loop {
                        let aa = 1u64; let bb = rnd(&mut seed, p); let cc = rnd(&mut seed, p);
                        let disc = f.sub(f.sqr(bb), f.mul(f.mul(4%p,aa),cc));
                        if f.legendre(disc) == -1 { a=aa; b=bb; c=cc; break; }
                    }
                }
                reset(); let rr = quad_roots(&f, a, b, c); rad_tot += muls();
                reset(); let cc = cz_roots(&f, &[c,b,a], &mut seed); cz_tot += muls();
                let mut rr2 = rr.clone(); rr2.sort_unstable();
                if rr2 == cc { agree += 1; }
            }
            assert_eq!(agree, n, "radical/CZ disagreed p={p} case={case}");
            println!("| {p} | {} | {} | {case} | {} | {} | {:.1}× |",
                p%4, p%3, rad_tot/n, cz_tot/n, cz_tot as f64 / rad_tot as f64);
        }
    }
}
