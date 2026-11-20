//! LANE xor-ribbon-build: static XOR / ribbon filters over negation-folded
//! pair-sum abscissae x(P_i +- P_j) of a prime-abscissa factor base.
//! Self-contained (no crate imports). All claims [pcc:unverified].
use std::collections::HashMap;
use std::time::Instant;

fn sm(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        sm(self.0)
    }
}

// ---------- prime-field curve y^2 = x^3 + a x + b ----------
#[derive(Clone, Copy)]
struct Cv {
    p: u64,
    a: u64,
    b: u64,
}
impl Cv {
    fn mul(&self, x: u64, y: u64) -> u64 {
        x * y % self.p
    }
    fn pow(&self, mut x: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, x);
            }
            x = self.mul(x, x);
            e >>= 1;
        }
        r
    }
    fn inv(&self, x: u64) -> u64 {
        self.pow(x, self.p - 2)
    }
    fn rhs(&self, x: u64) -> u64 {
        (self.mul(self.mul(x, x), x) + self.mul(self.a, x) + self.b) % self.p
    }
    fn is_qr(&self, v: u64) -> bool {
        v == 0 || self.pow(v, (self.p - 1) / 2) == 1
    }
    fn sqrt(&self, v: u64) -> u64 {
        self.pow(v, (self.p + 1) / 4) // p = 3 mod 4
    }
    // affine add; None = infinity
    #[allow(non_snake_case)]
    fn add(&self, P: (u64, u64), Q: (u64, u64)) -> Option<(u64, u64)> {
        let p = self.p;
        let l = if P.0 == Q.0 {
            if (P.1 + Q.1).is_multiple_of(p) {
                return None;
            }
            let num = (3 * self.mul(P.0, P.0) + self.a) % p;
            self.mul(num, self.inv(2 * P.1 % p))
        } else {
            let num = (Q.1 + p - P.1) % p;
            let den = (Q.0 + p - P.0) % p;
            self.mul(num, self.inv(den))
        };
        let x3 = (self.mul(l, l) + 2 * p - P.0 - Q.0) % p;
        let y3 = (self.mul(l, (P.0 + p - x3) % p) + p - P.1) % p;
        Some((x3, y3))
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

/// smallest `size` on-curve prime abscissae; returns (points, scanned)
fn factor_base(c: &Cv, size: usize) -> (Vec<(u64, u64)>, usize) {
    let mut v = vec![];
    let mut scanned = 0;
    let mut x = 1;
    while v.len() < size {
        x += 1;
        if !is_prime(x) {
            continue;
        }
        scanned += 1;
        let r = c.rhs(x);
        if c.is_qr(r) {
            let y = c.sqrt(r);
            assert_eq!(c.mul(y, y), r);
            v.push((x, y));
        }
    }
    (v, scanned)
}

/// negation-folded pair-sum keys: x(Pi+Pj), x(Pi-Pj), i<j. Returns (distinct keys, total emitted)
fn pair_keys(c: &Cv, fb: &[(u64, u64)]) -> (Vec<u32>, usize, usize) {
    let mut ks = Vec::with_capacity(fb.len() * fb.len());
    let mut adds = 0;
    for i in 0..fb.len() {
        for j in i + 1..fb.len() {
            let q = fb[j];
            let nq = (q.0, (c.p - q.1) % c.p);
            for r in [c.add(fb[i], q), c.add(fb[i], nq)] {
                adds += 1;
                if let Some((x, _)) = r {
                    ks.push(x as u32);
                }
            }
        }
    }
    let emitted = ks.len();
    ks.sort_unstable();
    ks.dedup();
    (ks, emitted, adds)
}

// ---------- hashing ----------
#[inline]
fn red(h: u32, n: usize) -> usize {
    ((h as u64 * n as u64) >> 32) as usize
}
#[inline]
fn hk(k: u32, seed: u64) -> u64 {
    sm(k as u64 ^ seed)
}

// ---------- XOR filter (3 segments, peeling) ----------
struct Xor {
    seed: u64,
    seg: usize,
    fp: Vec<u16>,
    mask: u16,
    fbits: u32,
}
impl Xor {
    #[inline]
    fn pos(&self, h: u64) -> [usize; 3] {
        let s = self.seg;
        [
            red(h as u32, s),
            s + red(h.rotate_left(21) as u32, s),
            2 * s + red(h.rotate_left(42) as u32, s),
        ]
    }
    #[inline]
    fn fpr(h: u64, mask: u16) -> u16 {
        ((h ^ (h >> 32)) as u16) & mask
    }
    fn build(keys: &[u32], fbits: u32, seed: u64) -> Option<Xor> {
        let n = keys.len();
        let seg = ((1.23 * n as f64) as usize + 32) / 3 + 1;
        let cap = 3 * seg;
        let mask: u16 = if fbits >= 16 {
            0xFFFF
        } else {
            (1u16 << fbits) - 1
        };
        let mut f = Xor {
            seed,
            seg,
            fp: vec![0; cap],
            mask,
            fbits,
        };
        let hs: Vec<u64> = keys.iter().map(|&k| hk(k, seed)).collect();
        let mut cnt = vec![0u32; cap];
        let mut xm = vec![0u64; cap];
        for &h in &hs {
            for p in f.pos(h) {
                cnt[p] += 1;
                xm[p] ^= h;
            }
        }
        let mut q: Vec<usize> = (0..cap).filter(|&i| cnt[i] == 1).collect();
        let mut stack: Vec<(u64, usize)> = Vec::with_capacity(n);
        while let Some(i) = q.pop() {
            if cnt[i] != 1 {
                continue;
            }
            let h = xm[i];
            stack.push((h, i));
            for p in f.pos(h) {
                cnt[p] -= 1;
                xm[p] ^= h;
                if cnt[p] == 1 {
                    q.push(p);
                }
            }
        }
        if stack.len() != n {
            return None;
        }
        while let Some((h, i)) = stack.pop() {
            let [a, b, c] = f.pos(h);
            let v = Xor::fpr(h, mask) ^ f.fp[a] ^ f.fp[b] ^ f.fp[c] ^ f.fp[i];
            f.fp[i] = v; // i is one of a,b,c and was still 0 in the xor above
        }
        Some(f)
    }
    #[inline]
    fn contains(&self, k: u32) -> bool {
        let h = hk(k, self.seed);
        let [a, b, c] = self.pos(h);
        Xor::fpr(h, self.mask) == (self.fp[a] ^ self.fp[b] ^ self.fp[c])
    }
    fn bits(&self) -> usize {
        self.fp.len() * self.fbits as usize
    }
}

// ---------- Ribbon filter (w = 64, sorted insertion, standard Gaussian) ----------
struct Ribbon {
    seed: u64,
    m: usize,
    z: Vec<u16>,
    mask: u16,
    rbits: u32,
}
const W: usize = 64;
impl Ribbon {
    #[inline]
    fn hash3(k: u32, seed: u64, m: usize, mask: u16) -> (usize, u64, u16) {
        let h = hk(k, seed);
        let s = (((h & 0xFFFF_FFFF) * (m - W + 1) as u64) >> 32) as usize;
        let c = sm(h ^ 0xA5A5_5A5A_1234_5678) | 1;
        let f = ((sm(h ^ 0x0F0F_F0F0_8765_4321) >> 40) as u16) & mask;
        (s, c, f)
    }
    fn build(keys: &[u32], rbits: u32, eps: f64, seed: u64) -> Option<Ribbon> {
        let n = keys.len();
        let m = ((n as f64 * (1.0 + eps)) as usize).max(W + 1);
        let mask: u16 = if rbits >= 16 {
            0xFFFF
        } else {
            (1u16 << rbits) - 1
        };
        let mut e: Vec<(usize, u64, u16)> = keys
            .iter()
            .map(|&k| Ribbon::hash3(k, seed, m, mask))
            .collect();
        e.sort_unstable_by_key(|t| t.0);
        let mut cs = vec![0u64; m];
        let mut rs = vec![0u16; m];
        for (mut s, mut c, mut r) in e {
            loop {
                if c == 0 {
                    if r != 0 {
                        return None;
                    }
                    break;
                }
                let t = c.trailing_zeros() as usize;
                s += t;
                c >>= t;
                if cs[s] == 0 {
                    cs[s] = c;
                    rs[s] = r;
                    break;
                }
                c ^= cs[s];
                r ^= rs[s];
            }
        }
        let mut z = vec![0u16; m + W];
        for i in (0..m).rev() {
            if cs[i] == 0 {
                continue;
            }
            let mut v = rs[i];
            let mut c = cs[i] & !1;
            while c != 0 {
                v ^= z[i + c.trailing_zeros() as usize];
                c &= c - 1;
            }
            z[i] = v;
        }
        Some(Ribbon {
            seed,
            m,
            z,
            mask,
            rbits,
        })
    }
    #[inline]
    fn contains(&self, k: u32) -> bool {
        let (s, mut c, f) = Ribbon::hash3(k, self.seed, self.m, self.mask);
        let mut v = 0u16;
        while c != 0 {
            v ^= self.z[s + c.trailing_zeros() as usize];
            c &= c - 1;
        }
        v == f
    }
    fn bits(&self) -> usize {
        self.m * self.rbits as usize
    }
}

// ---------- Bloom ----------
struct Bloom {
    seed: u64,
    m: usize,
    k: u32,
    bits: Vec<u64>,
}
impl Bloom {
    fn build(keys: &[u32], bpk: f64, k: u32, seed: u64) -> Bloom {
        let m = (keys.len() as f64 * bpk).ceil() as usize;
        let mut b = Bloom {
            seed,
            m,
            k,
            bits: vec![0; m.div_ceil(64)],
        };
        for &x in keys {
            let h = hk(x, seed);
            let (h1, h2) = (h as u32 as u64, (h >> 32) | 1);
            for i in 0..k as u64 {
                let p = ((((h1 + i * h2) as u32 as u64) * m as u64) >> 32) as usize;
                b.bits[p >> 6] |= 1 << (p & 63);
            }
        }
        b
    }
    /// returns (member?, reads)
    #[inline]
    fn contains(&self, x: u32) -> (bool, u32) {
        let h = hk(x, self.seed);
        let (h1, h2) = (h as u32 as u64, (h >> 32) | 1);
        for i in 0..self.k as u64 {
            let p = ((((h1 + i * h2) as u32 as u64) * self.m as u64) >> 32) as usize;
            if (self.bits[p >> 6] >> (p & 63)) & 1 == 0 {
                return (false, i as u32 + 1);
            }
        }
        (true, self.k)
    }
}

fn hashmap_bytes(n: usize) -> usize {
    // std HashMap<u32,u32> with_capacity(n): buckets = next_pow2(n*8/7), 8 B/bucket + 1 ctrl B + 16 ctrl tail
    let b = (n * 8).div_ceil(7).next_power_of_two();
    b * 9 + 16
}

fn qlist(c: &Cv, nq: usize, seed: u64) -> Vec<u32> {
    let mut r = Rng(seed);
    let mut v = Vec::with_capacity(nq);
    while v.len() < nq {
        let x = r.next() % c.p;
        if c.is_qr(c.rhs(x)) {
            v.push(x as u32);
        }
    }
    v
}

fn ns_per(t: Instant, n: usize) -> f64 {
    t.elapsed().as_nanos() as f64 / n as f64
}

fn run(name: &str, c: Cv, fsize: usize, nq: usize, nbuilds: usize) {
    println!(
        "\n==================== {name}: p={} |F|={fsize} ====================",
        c.p
    );
    let t0 = Instant::now();
    let (fb, scanned) = factor_base(&c, fsize);
    println!(
        "factor base: {} abscissae from {} prime candidates scanned (largest x={})",
        fb.len(),
        scanned,
        fb.last().unwrap().0
    );
    let (keys, emitted, adds) = pair_keys(&c, &fb);
    println!(
        "pair keys: group adds={} emitted={} distinct x={} (dup/collisions={}) build_s={:.2}",
        adds,
        emitted,
        keys.len(),
        emitted - keys.len(),
        t0.elapsed().as_secs_f64()
    );
    let n = keys.len();
    let exact: HashMap<u32, u32> = keys
        .iter()
        .enumerate()
        .map(|(i, &k)| (k, i as u32))
        .collect();
    let qs = qlist(&c, nq, 0xC0FFEE ^ c.p);
    let hits = qs.iter().filter(|q| exact.contains_key(q)).count();
    let neg = nq - hits;
    println!(
        "queries (random on-curve x): {nq}, exact hits={hits} => measured hit_rate={:.6e}; model keys/((p+1)/2)={:.6e}",
        hits as f64 / nq as f64,
        n as f64 / ((c.p + 1) as f64 / 2.0)
    );
    let _ = neg;
    println!("exact std HashMap<u32,u32>: est bytes={} bits/key={:.2}; packed-ideal bits/key={} (ceil log2 p + value bits {})",
        hashmap_bytes(n), hashmap_bytes(n) as f64 * 8.0 / n as f64,
        64 - (c.p).leading_zeros(), 2 * (64 - ((fsize - 1) as u64).leading_zeros()) + 1);
    {
        let t = Instant::now();
        let mut h = 0usize;
        for q in &qs {
            h += exact.contains_key(q) as usize;
        }
        println!("exact HashMap lookup: {:.1} ns/lookup (hits {h}); bucket probes/lookup ~1 (SwissTable group)", ns_per(t, nq));
    }

    // ---- construction success statistics ----
    println!("-- construction success (seeds 0..{nbuilds}) --");
    for fb_ in [8u32, 16] {
        let mut ok = 0;
        let mut tries_sum = 0;
        for s in 0..nbuilds {
            if Xor::build(&keys, fb_, 1000 + s as u64).is_some() {
                ok += 1;
            }
        }
        // retries until success from seed 7
        for s in 0..nbuilds {
            let mut t = 0;
            loop {
                t += 1;
                if Xor::build(&keys, fb_, 77_000 + (s * 64 + t) as u64).is_some() {
                    break;
                }
            }
            tries_sum += t;
        }
        println!(
            "xor{fb_}: first-attempt success {ok}/{nbuilds}; mean attempts to success {:.3}",
            tries_sum as f64 / nbuilds as f64
        );
    }
    for eps in [0.02f64, 0.05, 0.08, 0.10, 0.15] {
        let mut ok = 0;
        for s in 0..nbuilds {
            if Ribbon::build(&keys, 8, eps, 5000 + s as u64).is_some() {
                ok += 1;
            }
        }
        println!(
            "ribbon w=64 eps={eps:.2}: first-attempt success {ok}/{nbuilds}; mean attempts ~{:.3}",
            nbuilds as f64 / ok.max(1) as f64
        );
    }

    // ---- FPR, probes, bits/key per structure ----
    println!("-- structures at f=8 and f=16 (3 independent builds each; FPR over {} non-member queries) --", neg);
    let nb = 3;
    for f in [8u32, 16] {
        let theo = 2f64.powi(-(f as i32));
        // xor
        {
            let (mut fps, mut bits) = (0usize, 0usize);
            let mut tns = 0.0;
            let mut got = 0;
            for s in 0..nb {
                let mut sd = 9000 + s as u64 * 131;
                let x = loop {
                    if let Some(x) = Xor::build(&keys, f, sd) {
                        break x;
                    }
                    sd += 1;
                };
                // no false negatives
                assert!(keys.iter().all(|&k| x.contains(k)));
                let t = Instant::now();
                let mut fpc = 0;
                for q in &qs {
                    if x.contains(*q) && !exact.contains_key(q) {
                        fpc += 1;
                    }
                }
                tns += ns_per(t, nq);
                fps += fpc;
                bits = x.bits();
                got += 1;
            }
            let fpr = fps as f64 / (got * neg) as f64;
            println!("xor{f}: bits/key={:.3} (1.23*f={:.3}) FPR={:.4e} (theory 2^-{f}={:.4e}, FP count {fps}) probes/lookup=3 (+3 hash positions, 1 hash) {:.1} ns/lookup",
                bits as f64 / n as f64, 1.23 * f as f64, fpr, theo, tns / got as f64);
        }
        // ribbon
        for eps in [0.10f64, 0.15] {
            let (mut fps, mut bits) = (0usize, 0usize);
            let mut got = 0;
            let mut tns = 0.0;
            let mut pc = 0.0;
            for s in 0..nb {
                let mut sd = 12000 + s as u64 * 977;
                let x = loop {
                    if let Some(x) = Ribbon::build(&keys, f, eps, sd) {
                        break x;
                    }
                    sd += 1;
                };
                assert!(keys.iter().all(|&k| x.contains(k)));
                let t = Instant::now();
                let mut fpc = 0;
                for q in &qs {
                    if x.contains(*q) && !exact.contains_key(q) {
                        fpc += 1;
                    }
                }
                tns += ns_per(t, nq);
                fps += fpc;
                bits = x.bits();
                got += 1;
                let mut tot = 0u64;
                for q in qs.iter().take(100000) {
                    tot += Ribbon::hash3(*q, x.seed, x.m, x.mask).1.count_ones() as u64;
                }
                pc += tot as f64 / 100000.0;
            }
            let fpr = fps as f64 / (got * neg) as f64;
            println!("ribbon{f} eps={eps:.2}: bits/key={:.3} (=(1+eps)f={:.3}) FPR={:.4e} (theory {:.4e}, FP count {fps}) slot reads/lookup(avg popcount)={:.1} within <=64-slot window (<=2 cache lines) {:.1} ns/lookup",
                bits as f64 / n as f64, (1.0 + eps) * f as f64, fpr, theo, pc / got as f64, tns / got as f64);
        }
        // bloom at equal FPR (theoretical optimum sizing)
        {
            let bpk = f as f64 * std::f64::consts::LOG2_E;
            let mut fps = 0usize;
            let mut reads = 0u64;
            let mut tns = 0.0;
            for s in 0..nb {
                let b = Bloom::build(&keys, bpk, f, 31 + s as u64);
                assert!(keys.iter().all(|&k| b.contains(k).0));
                let t = Instant::now();
                let mut fpc = 0;
                for q in &qs {
                    let (r, rd) = b.contains(*q);
                    if r && !exact.contains_key(q) {
                        fpc += 1;
                    }
                    if !exact.contains_key(q) {
                        reads += rd as u64;
                    }
                }
                tns += ns_per(t, nq);
                fps += fpc;
            }
            let fpr = fps as f64 / (nb * neg) as f64;
            println!("bloom (k={f}, m=1.4427 f n): bits/key={:.3} FPR={:.4e} (theory ~{:.4e}) mean reads/negative lookup={:.2} {:.1} ns/lookup",
                bpk, fpr, theo, reads as f64 / (nb * neg) as f64, tns / nb as f64);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nq: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(16_000_000);
    let nbuilds: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
    let c24 = Cv {
        p: 16777199,
        a: 16777196,
        b: 31,
    };
    let c28 = Cv {
        p: 268435399,
        a: 268435396,
        b: 3,
    };
    // on-curve sanity for generators
    assert_eq!(c24.mul(5412647, 5412647), c24.rhs(2));
    assert_eq!(c28.mul(1, 1), c28.rhs(1));
    println!("generators on curve: p24 G=(2,5412647) ok, p28 G=(1,1) ok");
    run("p24", c24, 128, nq, nbuilds);
    run("p24", c24, 512, nq, nbuilds / 4);
    run("p28", c28, 323, nq, nbuilds);
    run("p28", c28, 1024, nq, nbuilds / 4);
}
