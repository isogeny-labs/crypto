//! LANE cuckoo: cuckoo filter / cuckoo table (bucket 4, partial-key hashing) over the
//! negation-folded pair-sum abscissae of a prime-abscissa base on p24. Self-contained.
//! Harness helpers (curve, factor base, Bloom) follow ic_alt_xor_ribbon_build.rs. All claims [pcc:unverified].
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
    fn add(&self, P: (u64, u64), Q: (u64, u64)) -> Option<(u64, u64)> {
        let p = self.p;
        let l;
        if P.0 == Q.0 {
            if (P.1 + Q.1) % p == 0 {
                return None;
            }
            let num = (3 * self.mul(P.0, P.0) + self.a) % p;
            l = self.mul(num, self.inv(2 * P.1 % p));
        } else {
            let num = (Q.1 + p - P.1) % p;
            let den = (Q.0 + p - P.0) % p;
            l = self.mul(num, self.inv(den));
        }
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
        if n % d == 0 {
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
            bits: vec![0; (m + 63) / 64],
        };
        for &x in keys {
            let h = hk(x, seed);
            let (h1, h2) = (h as u32 as u64, (h >> 32) | 1);
            for i in 0..k as u64 {
                let p = ((h1 + i * h2) as u32 as u64 * m as u64 >> 32) as usize;
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
            let p = ((h1 + i * h2) as u32 as u64 * self.m as u64 >> 32) as usize;
            if self.bits[p >> 6] >> (p & 63) & 1 == 0 {
                return (false, i as u32 + 1);
            }
        }
        (true, self.k)
    }
}

fn hashmap_bytes(n: usize) -> usize {
    // std HashMap<u32,u32> with_capacity(n): buckets = next_pow2(n*8/7), 8 B/bucket + 1 ctrl B + 16 ctrl tail
    let b = ((n * 8 + 6) / 7).next_power_of_two();
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

// ---------- cuckoo filter / table: 4-slot buckets, partial-key cuckoo hashing ----------
const BS: usize = 4;
struct Cuckoo {
    seed: u64,
    nb: usize,
    f: u32,
    fp: Vec<u16>,  // 0 = empty
    val: Vec<u32>, // packed pair index (table mode)
    n: usize,
    rng: Rng,
    kicks_total: u64,
}
impl Cuckoo {
    fn new(nb: usize, f: u32, seed: u64) -> Cuckoo {
        Cuckoo {
            seed,
            nb,
            f,
            fp: vec![0; nb * BS],
            val: vec![0; nb * BS],
            n: 0,
            rng: Rng(seed ^ 0xABCDEF),
            kicks_total: 0,
        }
    }
    #[inline]
    fn idx(&self, k: u32) -> (usize, u16) {
        let h = hk(k, self.seed);
        let m = (1u64 << self.f) - 1; // fingerprints in 1..=m
        (red(h as u32, self.nb), 1 + ((h >> 32) % m) as u16)
    }
    #[inline]
    fn alt(&self, i: usize, fp: u16) -> usize {
        let g = red(sm(fp as u64 ^ self.seed ^ 0x5555_AAAA) as u32, self.nb);
        (g + self.nb - i) % self.nb // involution: alt(alt(i)) = i
    }
    #[inline]
    fn put(&mut self, b: usize, fp: u16, v: u32) -> bool {
        for s in 0..BS {
            if self.fp[b * BS + s] == 0 {
                self.fp[b * BS + s] = fp;
                self.val[b * BS + s] = v;
                self.n += 1;
                return true;
            }
        }
        false
    }
    /// Some(kick-outs) on success; None on failure (one displaced entry is then lost)
    fn insert(&mut self, k: u32, v: u32, maxk: u32) -> Option<u32> {
        let (i1, fp) = self.idx(k);
        let i2 = self.alt(i1, fp);
        if self.put(i1, fp, v) || self.put(i2, fp, v) {
            return Some(0);
        }
        let mut i = if self.rng.next() & 1 == 0 { i1 } else { i2 };
        let (mut cf, mut cv) = (fp, v);
        for kk in 1..=maxk {
            let s = (self.rng.next() % BS as u64) as usize;
            std::mem::swap(&mut cf, &mut self.fp[i * BS + s]);
            std::mem::swap(&mut cv, &mut self.val[i * BS + s]);
            i = self.alt(i, cf);
            self.kicks_total += 1;
            if self.put(i, cf, cv) {
                return Some(kk);
            }
        }
        None
    }
    /// (found, buckets read, slot fingerprint compares)
    #[inline]
    fn lookup(&self, k: u32) -> (bool, u32, u32) {
        let (i1, fp) = self.idx(k);
        let mut cmp = 0;
        for s in 0..BS {
            cmp += 1;
            if self.fp[i1 * BS + s] == fp {
                return (true, 1, cmp);
            }
        }
        let i2 = self.alt(i1, fp);
        for s in 0..BS {
            cmp += 1;
            if self.fp[i2 * BS + s] == fp {
                return (true, 2, cmp);
            }
        }
        (false, 2, cmp)
    }
    /// all candidate values in both buckets (table mode)
    fn candidates(&self, k: u32, out: &mut Vec<u32>) {
        out.clear();
        let (i1, fp) = self.idx(k);
        let i2 = self.alt(i1, fp);
        for b in [i1, i2] {
            for s in 0..BS {
                if self.fp[b * BS + s] == fp {
                    out.push(self.val[b * BS + s]);
                }
            }
            if i1 == i2 {
                break;
            }
        }
    }
    fn delete(&mut self, k: u32) -> bool {
        let (i1, fp) = self.idx(k);
        let i2 = self.alt(i1, fp);
        for b in [i1, i2] {
            for s in 0..BS {
                if self.fp[b * BS + s] == fp {
                    self.fp[b * BS + s] = 0;
                    self.n -= 1;
                    return true;
                }
            }
        }
        false
    }
    /// delete the entry matching both fingerprint and stored value (table mode)
    fn delete_kv(&mut self, k: u32, v: u32) -> bool {
        let (i1, fp) = self.idx(k);
        let i2 = self.alt(i1, fp);
        for b in [i1, i2] {
            for s in 0..BS {
                if self.fp[b * BS + s] == fp && self.val[b * BS + s] == v {
                    self.fp[b * BS + s] = 0;
                    self.n -= 1;
                    return true;
                }
            }
        }
        false
    }
    fn slots(&self) -> usize {
        self.nb * BS
    }
}

fn vbits(f: usize) -> u32 {
    2 * (64 - ((f - 1) as u64).leading_zeros()) + 1
}

/// pair table: (x, packed (i,j,sign)) for every emitted pair sum, i<j (duplicates kept as multiset)
fn pair_table(c: &Cv, fb: &[(u64, u64)]) -> Vec<(u32, u32)> {
    let f = fb.len() as u32;
    let mut v = Vec::with_capacity(fb.len() * fb.len());
    for i in 0..fb.len() {
        for j in i + 1..fb.len() {
            let q = fb[j];
            let nq = (q.0, (c.p - q.1) % c.p);
            for (sg, r) in [c.add(fb[i], q), c.add(fb[i], nq)].into_iter().enumerate() {
                if let Some((x, _)) = r {
                    v.push((x as u32, ((i as u32 * f + j as u32) << 1) | sg as u32));
                }
            }
        }
    }
    v
}
/// recompute x for a packed value (1 group add)
fn resolve(c: &Cv, fb: &[(u64, u64)], v: u32) -> Option<u32> {
    let f = fb.len() as u32;
    let sg = v & 1;
    let ij = v >> 1;
    let (i, j) = ((ij / f) as usize, (ij % f) as usize);
    let q = fb[j];
    let q = if sg == 1 { (q.0, (c.p - q.1) % c.p) } else { q };
    c.add(fb[i], q).map(|r| r.0 as u32)
}
/// keys of one factor-base element e against all others in fb (excluding e itself)
fn elem_keys(c: &Cv, fb: &[(u64, u64)], e: usize, adds: &mut u64) -> Vec<(u32, u32)> {
    let f = fb.len() as u32;
    let mut v = vec![];
    for j in 0..fb.len() {
        if j == e {
            continue;
        }
        let (a, b) = if e < j { (e, j) } else { (j, e) };
        let q = fb[b];
        let nq = (q.0, (c.p - q.1) % c.p);
        for (sg, r) in [c.add(fb[a], q), c.add(fb[a], nq)].into_iter().enumerate() {
            *adds += 1;
            if let Some((x, _)) = r {
                v.push((x as u32, ((a as u32 * f + b as u32) << 1) | sg as u32));
            }
        }
    }
    v
}

fn nbf(n: usize, load: f64) -> usize {
    ((n as f64 / (BS as f64 * load)).ceil() as usize).max(2)
}

fn band(load: f64) -> usize {
    if load < 0.5 {
        0
    } else if load < 0.8 {
        1
    } else if load < 0.9 {
        2
    } else if load < 0.95 {
        3
    } else {
        4
    }
}
const BANDS: [&str; 5] = ["<0.50", "0.50-0.80", "0.80-0.90", "0.90-0.95", ">=0.95"];

/// E1: fill to first failure with distinct pseudo-random keys
fn e1_fill(nb: usize, f: u32, seeds: usize, maxk: u32) {
    let mut loads = vec![];
    let mut bk = [0u64; 5];
    let mut bn = [0u64; 5];
    let mut bmax = [0u32; 5];
    for s in 0..seeds {
        let mut t = Cuckoo::new(nb, f, 777 + s as u64 * 7919);
        let cap = t.slots();
        let mut ctr = 0u64;
        loop {
            let k =
                ((ctr.wrapping_mul(0x9E3779B1).wrapping_add(s as u64 * 12345)) & 0xFF_FFFF) as u32;
            let load = t.n as f64 / cap as f64;
            match t.insert(k, 0, maxk) {
                Some(kk) => {
                    let b = band(load);
                    bk[b] += kk as u64;
                    bn[b] += 1;
                    bmax[b] = bmax[b].max(kk);
                }
                None => break,
            }
            ctr += 1;
            if t.n == cap {
                break;
            }
        }
        loads.push(t.n as f64 / cap as f64);
    }
    loads.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = loads.iter().sum::<f64>() / loads.len() as f64;
    println!(
        "fill-to-failure nb={nb} ({} slots) f={f} maxkicks={maxk} seeds={seeds}: load at first failure mean={:.4} min={:.4} median={:.4} max={:.4}",
        nb * BS, mean, loads[0], loads[loads.len() / 2], loads[loads.len() - 1]
    );
    let mut line = String::from("   kick-outs/insert by load band:");
    for b in 0..5 {
        if bn[b] > 0 {
            line += &format!(
                " [{}] mean={:.3} max={} (n={})",
                BANDS[b],
                bk[b] as f64 / bn[b] as f64,
                bmax[b],
                bn[b]
            );
        }
    }
    println!("{line}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nq: usize = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8_000_000);
    let seeds: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(200);
    let c = Cv {
        p: 16777199,
        a: 16777196,
        b: 31,
    };
    assert_eq!(c.mul(5412647, 5412647), c.rhs(2));
    println!("generator G=(2,5412647) on curve p24 ok");
    let fsize = 128;
    let (fb, scanned) = factor_base(&c, fsize);
    let (keys, emitted, adds) = pair_keys(&c, &fb);
    let n = keys.len();
    println!("factor base |F|={} (scanned {} primes, max x={}); pair keys: adds={} emitted={} distinct={} dup={}", fb.len(), scanned, fb.last().unwrap().0, adds, emitted, n, emitted - n);
    let tab = pair_table(&c, &fb);
    let ntab = tab.len();
    let v = vbits(fsize);
    println!("pair table entries (multiset)={ntab}; value bits v={v}");
    let exact: HashMap<u32, u32> = keys
        .iter()
        .enumerate()
        .map(|(i, &k)| (k, i as u32))
        .collect();
    let qs = qlist(&c, nq, 0xC0FFEE ^ c.p);
    let neg: Vec<u32> = qs
        .iter()
        .copied()
        .filter(|q| !exact.contains_key(q))
        .collect();
    println!("queries {nq} random on-curve x, non-members {}", neg.len());

    println!("\n== E1: load factor reached and kick-outs (random distinct keys) ==");
    for f in [8u32, 12, 16] {
        e1_fill(4096, f, seeds, 500);
    }
    e1_fill(4096, 12, seeds, 100);
    e1_fill(4096, 12, seeds, 2000);
    e1_fill(65536, 12, seeds / 4, 500);
    e1_fill(512, 12, seeds, 500);

    println!("\n== E1b: build the actual {n} pair keys (f=12), success rate vs target load, {seeds} hash seeds ==");
    for load in [0.80, 0.85, 0.90, 0.92, 0.94, 0.95, 0.96, 0.97, 0.98] {
        let nb = nbf(n, load);
        let (mut ok, mut kk, mut ins) = (0, 0u64, 0u64);
        for s in 0..seeds {
            let mut t = Cuckoo::new(nb, 12, 31337 + s as u64);
            let mut good = true;
            for &k in &keys {
                match t.insert(k, 0, 500) {
                    Some(x) => {
                        kk += x as u64;
                        ins += 1;
                    }
                    None => {
                        good = false;
                        break;
                    }
                }
            }
            ok += good as usize;
        }
        println!("target load {load:.2}: nb={nb} actual load={:.4} success {ok}/{seeds}; kick-outs/insert={:.3}", n as f64 / (nb * BS) as f64, kk as f64 / ins as f64);
    }

    println!("\n== E3: filter / table comparison on the real key set (n={n}) ==");
    println!(
        "exact std HashMap<u32,u32>: est {} bytes = {:.2} bits/key (FPR 0)",
        hashmap_bytes(n),
        hashmap_bytes(n) as f64 * 8.0 / n as f64
    );
    {
        let t = Instant::now();
        let mut h = 0usize;
        for q in &qs {
            h += exact.contains_key(q) as usize;
        }
        println!(
            "  lookup {:.1} ns (hits {h}); packed-ideal bits/key = 24 + v = {}",
            ns_per(t, nq),
            24 + v
        );
    }
    let mut cfg = vec![];
    for f in [8u32, 12, 16] {
        for load in [0.50, 0.90, 0.95] {
            cfg.push((f, load));
        }
    }
    for &(f, load) in &cfg {
        let nb = nbf(n, load);
        let mut sd = 4242u64;
        let t = loop {
            let mut t = Cuckoo::new(nb, f, sd);
            if keys.iter().all(|&k| t.insert(k, 0, 500).is_some()) {
                break t;
            }
            sd += 1;
        };
        let attempts = sd - 4242 + 1;
        assert!(keys.iter().all(|&k| t.lookup(k).0), "false negative");
        let al = n as f64 / t.slots() as f64;
        // negatives
        let ts = Instant::now();
        let (mut fp, mut bk, mut cm) = (0usize, 0u64, 0u64);
        for q in &neg {
            let (r, b, c2) = t.lookup(*q);
            fp += r as usize;
            bk += b as u64;
            cm += c2 as u64;
        }
        let tns = ns_per(ts, neg.len());
        let (mut pb, mut pc) = (0u64, 0u64);
        for &k in &keys {
            let (_, b, c2) = t.lookup(k);
            pb += b as u64;
            pc += c2 as u64;
        }
        let fpr = fp as f64 / neg.len() as f64;
        let theo = 1.0 - (1.0 - 1.0 / ((1u64 << f) - 1) as f64).powf(2.0 * BS as f64 * al);
        println!(
            "cuckoo f={f:2} nb={nb:5} load={al:.4} (attempts {attempts}): filter bits/key={:.2}, table bits/key={:.2} (f+v={}); FPR={:.4e} (theory {:.4e}, FP {fp}); neg: {:.3} buckets {:.2} cmps {:.1} ns; pos: {:.3} buckets {:.2} cmps; kicks/insert={:.3}",
            t.slots() as f64 * f as f64 / n as f64,
            t.slots() as f64 * (f + v) as f64 / n as f64,
            f + v,
            fpr, theo,
            bk as f64 / neg.len() as f64, cm as f64 / neg.len() as f64, tns,
            pb as f64 / n as f64, pc as f64 / n as f64,
            t.kicks_total as f64 / n as f64
        );
        // equal-FPR Bloom
        if load == 0.95 {
            let k = (1.0 / fpr).log2().round().max(1.0) as u32;
            let bpk = k as f64 * std::f64::consts::LOG2_E;
            let b = Bloom::build(&keys, bpk, k, 99);
            let (mut bfp, mut rd) = (0usize, 0u64);
            let tb = Instant::now();
            for q in &neg {
                let (r, x) = b.contains(*q);
                bfp += r as usize;
                rd += x as u64;
            }
            println!(
                "    Bloom k={k}: bits/key={:.2} FPR={:.4e} reads/neg={:.2} {:.1} ns   [cuckoo/Bloom bits ratio {:.3}]",
                bpk, bfp as f64 / neg.len() as f64, rd as f64 / neg.len() as f64, ns_per(tb, neg.len()),
                (t.slots() as f64 * f as f64 / n as f64) / bpk
            );
        }
    }

    // ---- E3b: exact (zero-FP) variant via bijective quotienting, pow2 buckets ----
    println!("\n-- exact quotient cuckoo (key = bucket+stored remainder, FPR=0 by construction), 24-bit domain --");
    for b in [12u32, 13] {
        let nb = 1usize << b;
        let load = ntab.min(n) as f64 / (nb * BS) as f64;
        println!("  nb=2^{b}: load={load:.3} ({}); stored remainder bits={} => table bits/key={:.2} (+v={v}) vs packed-exact {}", if load > 0.96 { "infeasible: above ~0.95-0.96 b=4 limit" } else { "feasible" }, 24 - b, (nb * BS) as f64 * (24 - b + v) as f64 / n as f64, 24 + v);
    }

    // ---- E4: deletion ----
    println!("\n== E4: deletion (f=16 table, load~0.90, {ntab} entries with values) ==");
    {
        let nb = nbf(ntab, 0.90);
        let mut sd = 1u64;
        let mut t = loop {
            let mut t = Cuckoo::new(nb, 16, sd);
            if tab.iter().all(|&(k, v)| t.insert(k, v, 500).is_some()) {
                break t;
            }
            sd += 1;
        };
        let e = 17usize;
        let mut ad = 0u64;
        let ek = elem_keys(&c, &fb, e, &mut ad);
        let before = t.n;
        let del = ek.iter().filter(|&&(k, v)| t.delete_kv(k, v)).count();
        println!("evict element #{e} (x={}): recomputed {} keys with {ad} group adds; deleted {del}/{}; entries {before}->{}", fb[e].0, ek.len(), ek.len(), t.n);
        // surviving keys all still present with value
        let mut v2 = Vec::new();
        let mut lost = 0;
        let evicted: std::collections::HashSet<(u32, u32)> = ek.iter().copied().collect();
        for &(k, v) in &tab {
            if evicted.contains(&(k, v)) {
                continue;
            }
            t.candidates(k, &mut v2);
            if !v2.contains(&v) {
                lost += 1;
            }
        }
        println!(
            "surviving entries lost after deletions (false negatives): {lost}/{}",
            ntab - ek.len()
        );
        // reinsert new element keys
        let (fb2, _) = factor_base(&c, 130);
        let mut fbn = fb.clone();
        fbn[e] = fb2[128];
        let mut ad2 = 0u64;
        let nk = elem_keys(&c, &fbn, e, &mut ad2);
        let okc = nk
            .iter()
            .filter(|&&(k, v)| t.insert(k, v, 500).is_some())
            .count();
        println!("insert replacement element (x={}): {} keys, {ad2} adds, inserted {okc}/{}; load now {:.4}", fbn[e].0, nk.len(), nk.len(), t.n as f64 / t.slots() as f64);
        // false deletion: delete never-inserted keys (filter-only semantics, no value check)
        let mut tf = Cuckoo::new(nb, 12, 55);
        for &(k, _) in &tab {
            assert!(tf.insert(k, 0, 500).is_some());
        }
        let nbefore = tf.n;
        let trials = 2_000_000.min(neg.len());
        let mut falsedel = 0usize;
        for q in neg.iter().take(trials) {
            if tf.delete(*q) {
                falsedel += 1;
            }
        }
        let lostk = keys.iter().filter(|&&k| !tf.lookup(k).0).count();
        println!("filter-only f=12: deleting {trials} NEVER-inserted keys removed {falsedel} entries (rate {:.3e}, theory ~{:.3e}); entries {nbefore}->{}; members now false-negative: {lostk}/{n}", falsedel as f64 / trials as f64, 2.0 * BS as f64 * (ntab as f64 / tf.slots() as f64) / 4095.0, tf.n);
    }

    // ---- E5: dynamic factor-base policies ----
    println!("\n== E5: dynamic factor base: hits per trial and GAE to R relations (random on-curve R, cuckoo table f=16) ==");
    let reps = 20usize;
    let (fbbig, _) = factor_base(&c, 400);
    let ct = 1.0479f64;
    let policies: Vec<(&str, usize, usize, usize)> = vec![
        // name, F0, growth every G trials (0=static), swap every K (0 none)
        ("static F=64", 64, 0, 0),
        ("static F=96", 96, 0, 0),
        ("static F=128", 128, 0, 0),
        ("static F=160", 160, 0, 0),
        ("static F=192", 192, 0, 0),
        ("grow 32->, +1 elem / 100 trials", 32, 100, 0),
        ("grow 64->, +1 elem / 100 trials", 64, 100, 0),
        ("grow 64->, +1 elem / 500 trials", 64, 500, 0),
        (
            "swap F=128: replace random elem / 2000 trials",
            128,
            0,
            2000,
        ),
        (
            "swap F=128: replace least-hit elem / 2000 trials",
            128,
            0,
            -2000i64 as usize,
        ),
    ];
    for &(name, f0, g, k) in &policies {
        let (mut gae_fixed87, mut gae_prop, mut trials87, mut hitsum) =
            (vec![], vec![], vec![], 0usize);
        let mut fpcand = 0u64;
        let mut totswap = 0u64;
        let mut corr_num = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0usize);
        for r in 0..reps {
            let mut fbc: Vec<(u64, u64)> = fbbig[..f0].to_vec();
            let mut next = f0.max(128); // next unused abscissa
            if g == 0 && k == 0 {
                next = f0;
            }
            let mut adds: u64 = 0;
            let _ = &mut adds;
            let pack = |i: usize, j: usize, sg: u32| (((i * 512 + j) as u32) << 1) | sg;
            let mut t = Cuckoo::new(nbf(2 * 220 * 219 / 2 * 2, 0.85), 16, 99 + r as u64);
            let add_pair =
                |t: &mut Cuckoo, fbc: &Vec<(u64, u64)>, i: usize, j: usize, adds: &mut u64| {
                    let (a, b) = if i < j { (i, j) } else { (j, i) };
                    let q = fbc[b];
                    let nq = (q.0, (c.p - q.1) % c.p);
                    for (sg, rr) in [c.add(fbc[a], q), c.add(fbc[a], nq)]
                        .into_iter()
                        .enumerate()
                    {
                        *adds += 1;
                        if let Some((x, _)) = rr {
                            assert!(t.insert(x as u32, pack(a, b, sg as u32), 500).is_some());
                        }
                    }
                };
            let del_pair =
                |t: &mut Cuckoo, fbc: &Vec<(u64, u64)>, i: usize, j: usize, adds: &mut u64| {
                    let (a, b) = if i < j { (i, j) } else { (j, i) };
                    let q = fbc[b];
                    let nq = (q.0, (c.p - q.1) % c.p);
                    for (sg, rr) in [c.add(fbc[a], q), c.add(fbc[a], nq)]
                        .into_iter()
                        .enumerate()
                    {
                        *adds += 1;
                        if let Some((x, _)) = rr {
                            assert!(t.delete_kv(x as u32, pack(a, b, sg as u32)));
                        }
                    }
                };
            let mut adds_setup: u64 = 0;
            for i in 0..fbc.len() {
                for j in i + 1..fbc.len() {
                    add_pair(&mut t, &fbc, i, j, &mut adds_setup);
                }
            }
            let mut rng = Rng(0xFEED + r as u64 * 17 + g as u64);
            let mut hits = 0usize;
            let mut elemhits = vec![0u32; 512];
            let mut elemhits_h1 = vec![0u32; 512];
            let mut cand = Vec::new();
            let mut trials = 0usize;
            let mut done87: Option<(f64, usize)> = None;
            let mut done_prop: Option<f64> = None;
            let mut swap_adds = 0u64;
            let tmax = 400_000usize;
            let mut cur_x;
            while trials < tmax {
                // random on-curve x
                loop {
                    cur_x = (rng.next() % c.p) as u32;
                    if c.is_qr(c.rhs(cur_x as u64)) {
                        break;
                    }
                }
                trials += 1;
                t.candidates(cur_x, &mut cand);
                for &v in &cand {
                    let ij = (v >> 1) as usize;
                    let (a, b) = (ij / 512, ij % 512);
                    // verify by recomputing one add
                    let q = fbc[b];
                    let q = if v & 1 == 1 {
                        (q.0, (c.p - q.1) % c.p)
                    } else {
                        q
                    };
                    swap_adds += 1; // verification adds counted as GAE below
                    if c.add(fbc[a], q).map(|r| r.0 as u32) == Some(cur_x) {
                        hits += 1;
                        elemhits[a] += 1;
                        elemhits[b] += 1;
                        if trials <= tmax / 2 {
                            elemhits_h1[a] += 1;
                            elemhits_h1[b] += 1;
                        }
                    } else {
                        fpcand += 1;
                    }
                }
                let fcur = fbc.len();
                if done87.is_none() && hits >= 87 {
                    done87 = Some((trials as f64, 0));
                    gae_fixed87.push(adds_setup as f64 + swap_adds as f64 + ct * trials as f64);
                    trials87.push(trials as f64);
                }
                if done_prop.is_none() && hits as f64 >= 0.68 * fcur as f64 {
                    done_prop = Some(0.0);
                    gae_prop.push(adds_setup as f64 + swap_adds as f64 + ct * trials as f64);
                }
                if done87.is_some() && done_prop.is_some() && !(g == 0 && k == 0 && f0 == 128) {
                    break;
                }
                // dynamic actions
                if g > 0 && trials % g == 0 && fbc.len() < 200 && done_prop.is_none() {
                    let e = fbc.len();
                    fbc.push(fbbig[e]);
                    for j in 0..e {
                        add_pair(&mut t, &fbc, j, e, &mut adds_setup);
                    }
                }
                if k != 0
                    && (trials
                        % (if k > usize::MAX / 2 {
                            (-(k as i64)) as usize
                        } else {
                            k
                        })
                        == 0)
                {
                    let least = k > usize::MAX / 2;
                    let e = if least {
                        (0..fbc.len()).min_by_key(|&i| elemhits[i]).unwrap()
                    } else {
                        (rng.next() % fbc.len() as u64) as usize
                    };
                    for j in 0..fbc.len() {
                        if j != e {
                            del_pair(&mut t, &fbc, e, j, &mut adds_setup);
                        }
                    }
                    fbc[e] = fbbig[next];
                    next += 1;
                    elemhits[e] = 0;
                    for j in 0..fbc.len() {
                        if j != e {
                            add_pair(&mut t, &fbc, e, j, &mut adds_setup);
                        }
                    }
                    totswap += 1;
                }
            }
            hitsum += hits;
            // split-half correlation of per-element hits (static case only)
            if g == 0 && k == 0 {
                // second-half counts = total - first half
                for i in 0..fbc.len() {
                    let a = elemhits_h1[i] as f64;
                    let b = (elemhits[i] - elemhits_h1[i]) as f64;
                    corr_num.0 += a;
                    corr_num.1 += b;
                    corr_num.2 += a * a;
                    corr_num.3 += b * b;
                    corr_num.4 += a * b;
                    corr_num.5 += 1;
                }
            }
            let _ = totswap;
        }
        let mean = |v: &Vec<f64>| {
            if v.is_empty() {
                f64::NAN
            } else {
                v.iter().sum::<f64>() / v.len() as f64
            }
        };
        let mut s = format!("{name:52}: reps {reps}; GAE-to-87-relations mean={:.0} (n={}) trials-to-87 mean={:.0}; GAE-to-0.68|F|-relations mean={:.0} (n={}); FP cands {}", mean(&gae_fixed87), gae_fixed87.len(), mean(&trials87), mean(&gae_prop), gae_prop.len(), fpcand);
        if corr_num.5 > 0 {
            let n_ = corr_num.5 as f64;
            let cov = corr_num.4 / n_ - (corr_num.0 / n_) * (corr_num.1 / n_);
            let va = corr_num.2 / n_ - (corr_num.0 / n_).powi(2);
            let vb = corr_num.3 / n_ - (corr_num.1 / n_).powi(2);
            s += &format!(
                "; split-half per-element hit correlation r={:.4} (n={})",
                cov / (va * vb).sqrt(),
                corr_num.5
            );
        }
        println!("{s}");
        println!("    total hits over reps={hitsum}");
    }
}
