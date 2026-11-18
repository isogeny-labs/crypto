//! Lane sort-merge-join: relation-phase lookup of the m=2 prime-abscissa pipeline on p24,
//! per-trial hash probe vs batched radix-sort + merge-join. Std only. All figures [pcc:unverified].
use std::time::Instant;

const P: u64 = 16777199;
const A: u64 = 16777196;
const B: u64 = 31;
const R: u64 = 16782299;
const GX: u64 = 2;
const GY: u64 = 5412647;
const FB: usize = 128;
type Pt = Option<(u64, u64)>;

struct Ec {
    ops: u64,
}
fn pw(mut b: u64, mut e: u64) -> u64 {
    let mut r = 1;
    b %= P;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e >>= 1;
    }
    r
}
fn inv(a: u64) -> u64 {
    let (mut t, mut nt, mut r, mut nr) = (0i64, 1i64, P as i64, a as i64);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    if t < 0 {
        t += P as i64;
    }
    t as u64
}
impl Ec {
    fn add(&mut self, p: Pt, q: Pt) -> Pt {
        let (p, q) = match (p, q) {
            (None, q) => return q,
            (p, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        self.ops += 1;
        let l;
        if p.0 == q.0 {
            if (p.1 + q.1) % P == 0 {
                return None;
            }
            l = (3 * p.0 % P * p.0 + A) % P * inv(2 * p.1 % P) % P;
        } else {
            l = (q.1 + P - p.1) % P * inv((q.0 + P - p.0) % P) % P;
        }
        let x = (l * l % P + 2 * P - p.0 - q.0) % P;
        let y = (l * ((p.0 + P - x) % P) % P + P - p.1) % P;
        Some((x, y))
    }
    fn neg(p: Pt) -> Pt {
        p.map(|(x, y)| (x, (P - y) % P))
    }
    fn mul(&mut self, mut k: u64, p: Pt) -> Pt {
        let mut acc: Pt = None;
        let mut base = p;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.add(base, base);
            }
        }
        acc
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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Table: sorted unique keys, entry ranges, entries (i,j,s), hash index.
struct Table {
    keys: Vec<u32>,
    start: Vec<u32>,
    ent: Vec<(u8, u8, u8)>,
    slots: Vec<u64>,
    mask: usize,
    build_ops: u64,
    raw_entries: usize,
}
const EMPTY: u64 = u64::MAX;
fn hslot(k: u32, mask: usize) -> usize {
    ((k as u64).wrapping_mul(0x9E3779B97F4A7C15) >> 40) as usize & mask
}

fn build_table() -> (Table, Vec<Pt>) {
    let mut ec = Ec { ops: 0 };
    let mut f: Vec<Pt> = vec![];
    let mut x = 2u64;
    while f.len() < FB {
        if is_prime(x) {
            let rhs = (x * x % P * x + A * x + B) % P;
            let y = pw(rhs, (P + 1) / 4);
            if y * y % P == rhs && rhs != 0 {
                f.push(Some((x, y)));
            }
        }
        x += 1;
    }
    let mut e: Vec<(u32, (u8, u8, u8))> = vec![];
    for i in 0..FB {
        let d = ec.add(f[i], f[i]);
        if let Some((x, _)) = d {
            e.push((x as u32, (i as u8, i as u8, 0)));
        }
        for j in i + 1..FB {
            if let Some((x, _)) = ec.add(f[i], f[j]) {
                e.push((x as u32, (i as u8, j as u8, 0)));
            }
            if let Some((x, _)) = ec.add(f[i], Ec::neg(f[j])) {
                e.push((x as u32, (i as u8, j as u8, 1)));
            }
        }
    }
    let raw = e.len();
    e.sort();
    let (mut keys, mut start, mut ent) = (vec![], vec![], vec![]);
    for (n, (k, v)) in e.iter().enumerate() {
        if keys.last() != Some(k) {
            keys.push(*k);
            start.push(n as u32);
        }
        ent.push(*v);
    }
    start.push(e.len() as u32);
    let sz = (keys.len() * 2).next_power_of_two();
    let mask = sz - 1;
    let mut slots = vec![EMPTY; sz];
    for (u, &k) in keys.iter().enumerate() {
        let mut s = hslot(k, mask);
        while slots[s] != EMPTY {
            s = (s + 1) & mask;
        }
        slots[s] = ((k as u64) << 32) | u as u64;
    }
    (
        Table {
            keys,
            start,
            ent,
            slots,
            mask,
            build_ops: ec.ops,
            raw_entries: raw,
        },
        f,
    )
}

#[derive(Default, Clone, Copy)]
struct Cnt {
    cmp: u64,
    words: u64,
    lines: u64,
    probes: u64,
}

struct Walk {
    ec: Ec,
    r: Pt,
    u: u64,
    v: u64,
    rng: Rng,
    steps: Vec<(u64, u64, Pt)>,
    setup_ops: u64,
}
impl Walk {
    fn new(seed: u64, kq: u64) -> Walk {
        let mut ec = Ec { ops: 0 };
        let g = Some((GX, GY));
        let q = ec.mul(kq, g);
        let mut rng = Rng(seed | 1);
        let mut steps = vec![];
        for _ in 0..32 {
            let a = rng.next() % R;
            let b = rng.next() % R;
            let ga = ec.mul(a, g);
            let qb = ec.mul(b, q);
            let s = ec.add(ga, qb);
            steps.push((a, b, s));
        }
        let (u, v) = (rng.next() % R, rng.next() % R);
        let ug = ec.mul(u, g);
        let vq = ec.mul(v, q);
        let r = ec.add(ug, vq);
        let setup_ops = ec.ops;
        ec.ops = 0;
        Walk {
            ec,
            r,
            u,
            v,
            rng,
            steps,
            setup_ops,
        }
    }
    /// next trial: one group addition; returns (key, u, v)
    fn step(&mut self) -> (u32, u64, u64) {
        let s = (self.rng.next() >> 20) as usize % self.steps.len();
        let (a, b, sp) = self.steps[s];
        self.r = self.ec.add(self.r, sp);
        self.u = (self.u + a) % R;
        self.v = (self.v + b) % R;
        (
            self.r.map(|(x, _)| x as u32).unwrap_or(u32::MAX),
            self.u,
            self.v,
        )
    }
}

struct Out {
    hits: Vec<(u32, u32)>,
    c: Cnt,
    ops: u64,
    setup_ops: u64,
    t_total: f64,
    t_lookup: f64,
    uv: Vec<(u64, u64)>,
}

fn hash_arm(t: &Table, n: usize, seed: u64, kq: u64) -> Out {
    let mut w = Walk::new(seed, kq);
    let mut c = Cnt::default();
    let mut hits = vec![];
    let mut uv = vec![];
    let mut tl = 0.0;
    let t0 = Instant::now();
    for idx in 0..n {
        let (k, u, v) = w.step();
        c.words += 1; // hash computation
        let mut s = hslot(k, t.mask);
        let mut last = usize::MAX;
        loop {
            let line = s >> 3;
            if line != last {
                c.lines += 1;
                last = line;
            }
            c.probes += 1;
            c.words += 1;
            let e = t.slots[s];
            if e == EMPTY {
                break;
            }
            c.cmp += 1;
            if (e >> 32) as u32 == k {
                hits.push((idx as u32, e as u32));
                uv.push((u, v));
                break;
            }
            s = (s + 1) & t.mask;
        }
    }
    let _ = &mut tl;
    Out {
        hits,
        c,
        ops: w.ec.ops,
        setup_ops: w.setup_ops,
        t_total: t0.elapsed().as_secs_f64(),
        t_lookup: tl,
        uv,
    }
}

fn radix_sort(v: &mut Vec<u64>, tmp: &mut Vec<u64>, c: &mut Cnt) {
    // sort by bits 32..56 of packed (key<<32 | idx); 3 passes of 8 bits
    let n = v.len();
    tmp.resize(n, 0);
    for pass in 0..3 {
        let sh = 32 + 8 * pass;
        let mut h = [0usize; 256];
        for &x in v.iter() {
            h[((x >> sh) & 255) as usize] += 1;
        }
        let mut s = 0;
        for b in h.iter_mut() {
            let t = *b;
            *b = s;
            s += t;
        }
        for &x in v.iter() {
            let d = ((x >> sh) & 255) as usize;
            tmp[h[d]] = x;
            h[d] += 1;
        }
        std::mem::swap(v, tmp);
        c.words += 5 * n as u64 + 256;
        c.lines += 3 * ((n as u64 * 8 + 63) / 64); // hist read, scatter read, scatter write (est.)
    }
}

fn batch_arm(t: &Table, n: usize, seed: u64, kq: u64, bs: usize, gallop: bool) -> Out {
    let mut w = Walk::new(seed, kq);
    let mut c = Cnt::default();
    let mut hits = vec![];
    let mut tl = 0.0;
    let mut uvall: Vec<(u64, u64)> = Vec::with_capacity(n);
    let (mut buf, mut tmp): (Vec<u64>, Vec<u64>) = (Vec::with_capacity(bs), Vec::new());
    let t0 = Instant::now();
    let mut done = 0;
    while done < n {
        buf.clear();
        let m = bs.min(n - done);
        for i in 0..m {
            let (k, u, v) = w.step();
            uvall.push((u, v));
            buf.push(((k as u64) << 32) | (done + i) as u64);
        }
        let l0 = Instant::now();
        radix_sort(&mut buf, &mut tmp, &mut c);
        let keys = &t.keys;
        let mut j = 0usize;
        let mut last = usize::MAX;
        for &x in buf.iter() {
            let k = (x >> 32) as u32;
            if gallop {
                // exponential then binary search for first keys[j'] >= k
                let mut step = 1;
                let mut hi = j;
                c.words += 1;
                loop {
                    if hi >= keys.len() {
                        break;
                    }
                    let line = hi >> 4;
                    if line != last {
                        c.lines += 1;
                        last = line;
                    }
                    c.cmp += 1;
                    c.words += 1;
                    if keys[hi] >= k {
                        break;
                    }
                    j = hi + 1;
                    hi = j + step - 1;
                    step <<= 1;
                }
                let mut hi = hi.min(keys.len());
                let mut lo = j;
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    let line = mid >> 4;
                    if line != last {
                        c.lines += 1;
                        last = line;
                    }
                    c.cmp += 1;
                    c.words += 1;
                    if keys[mid] < k {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                j = lo;
                if j < keys.len() {
                    c.cmp += 1;
                    c.words += 1;
                    if keys[j] == k {
                        hits.push(((x & 0xFFFF_FFFF) as u32, j as u32));
                    }
                }
            } else {
                while j < keys.len() && keys[j] < k {
                    let line = j >> 4;
                    if line != last {
                        c.lines += 1;
                        last = line;
                    }
                    c.cmp += 1;
                    c.words += 2;
                    j += 1;
                }
                if j < keys.len() {
                    c.cmp += 1;
                    c.words += 2;
                    if keys[j] == k {
                        hits.push(((x & 0xFFFF_FFFF) as u32, j as u32));
                    }
                }
            }
        }
        c.lines += (m as u64 * 8 + 63) / 64; // batch array read in merge
        tl += l0.elapsed().as_secs_f64();
        done += m;
    }
    hits.sort();
    let uv = hits.iter().map(|h| uvall[h.0 as usize]).collect();
    Out {
        hits,
        c,
        ops: w.ec.ops,
        setup_ops: w.setup_ops,
        t_total: t0.elapsed().as_secs_f64(),
        t_lookup: tl,
        uv,
    }
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: usize = args.get(1).map(|s| s.parse().unwrap()).unwrap_or(48550);
    let reps: usize = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(15);
    let kq = 1234567u64;
    let g = Some((GX, GY));
    assert!(
        (GY * GY) % P == (GX * GX % P * GX + A * GX + B) % P,
        "G on curve"
    );
    let mut ec = Ec { ops: 0 };
    assert!(ec.mul(R, g).is_none(), "r*G = O");
    let (t, f) = build_table();
    println!("N_trials={} reps={} |F|={} table_raw_entries={} unique_keys={} build_group_ops={} (|F|^2={}) hash_slots={}", n, reps, f.len(), t.raw_entries, t.keys.len(), t.build_ops, FB * FB, t.slots.len());
    let arms: Vec<(String, usize, bool)> = vec![
        ("hash".into(), 0, false),
        ("merge256".into(), 256, false),
        ("merge4096".into(), 4096, false),
        ("merge65536".into(), 65536, false),
        ("gallop256".into(), 256, true),
        ("gallop4096".into(), 4096, true),
        ("gallop65536".into(), 65536, true),
    ];
    // exact counts and identity at rep 0 seed
    let seed0 = 0xC0FFEE;
    let base = hash_arm(&t, n, seed0, kq);
    let mut all_ident = true;
    println!(
        "{:<12} {:>9} {:>9} {:>10} {:>12} {:>12} {:>11} {:>8} {:>6}",
        "arm",
        "walk_ops",
        "setup_ops",
        "hits",
        "key_cmps",
        "word_ops",
        "line_touch",
        "ratio_w",
        "ident"
    );
    println!(
        "{:<12} {:>9} {:>9} {:>10} {:>12} {:>12} {:>11} {:>8.3} {:>6}",
        "hash",
        base.ops,
        base.setup_ops,
        base.hits.len(),
        base.c.cmp,
        base.c.words,
        base.c.lines,
        1.0,
        "base"
    );
    let mut relations = 0usize;
    for h in &base.hits {
        relations += (t.start[h.1 as usize + 1] - t.start[h.1 as usize]) as usize;
    }
    // verify relations: (u+v*kq)G has x == key, uncounted
    let mut ver = 0;
    for (h, (u, v)) in base.hits.iter().zip(base.uv.iter()) {
        let mut e2 = Ec { ops: 0 };
        let s = (u + (*v as u128 * kq as u128 % R as u128) as u64) % R;
        let p = e2.mul(s, g);
        if p.map(|(x, _)| x as u32) == Some(t.keys[h.1 as usize]) {
            ver += 1;
        }
    }
    println!(
        "hash arm: trial-hits={} table-entry relations={} verified_by_scalar_mult={}",
        base.hits.len(),
        relations,
        ver
    );
    let mut counts: Vec<(String, Cnt, u64)> = vec![("hash".into(), base.c, base.ops)];
    for (name, bs, gal) in arms.iter().skip(1) {
        let o = batch_arm(&t, n, seed0, kq, *bs, *gal);
        let ident = o.hits == base.hits
            && o.ops == base.ops
            && o.setup_ops == base.setup_ops
            && o.uv == base.uv;
        all_ident &= ident;
        println!(
            "{:<12} {:>9} {:>9} {:>10} {:>12} {:>12} {:>11} {:>8.3} {:>6}",
            name,
            o.ops,
            o.setup_ops,
            o.hits.len(),
            o.c.cmp,
            o.c.words,
            o.c.lines,
            o.c.words as f64 / base.c.words as f64,
            ident
        );
        counts.push((name.clone(), o.c, o.ops));
    }
    println!("ALL_IDENTICAL_HITS_AND_GROUP_OPS={}", all_ident);
    // timing
    println!(
        "wall L0 (uncertified), median of {} reps, ns/trial: total (walk+lookup) | lookup-only",
        reps
    );
    for (name, bs, gal) in arms.iter() {
        let (mut tt, mut tlk) = (vec![], vec![]);
        for r in 0..reps {
            let seed = 0xABCD00 + r as u64 * 7919;
            let o = if *bs == 0 {
                hash_arm(&t, n, seed, kq)
            } else {
                batch_arm(&t, n, seed, kq, *bs, *gal)
            };
            tt.push(o.t_total * 1e9 / n as f64);
            tlk.push(o.t_lookup * 1e9 / n as f64);
        }
        println!(
            "{:<12} total {:>9.1}  lookup {:>9.1}",
            name,
            median(&mut tt),
            median(&mut tlk)
        );
    }
}
