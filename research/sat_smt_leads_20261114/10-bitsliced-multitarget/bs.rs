// Idea 10 pilot, part (a): scalar vs bit-sliced decide of the m=2 PDP leaf over F_{2^31}.
// Standalone: rustc -O -C target-cpu=native bs.rs
// Usage: bs <l_min> <l_max> <instance_dir>   -> JSON lines on stdout
use std::arch::x86_64::*;
use std::collections::HashMap;
use std::time::Instant;

const NB: usize = 31;
const MASK: u64 = (1u64 << 31) - 1;
const CURVE_B: u32 = 0x2f1a3c5b & 0x7fff_ffff;

#[inline(always)]
fn red(mut r: u64) -> u32 {
    let h = r >> 31;
    r = (r & MASK) ^ h ^ (h << 3);
    let h = r >> 31;
    r = (r & MASK) ^ h ^ (h << 3);
    let h = r >> 31;
    r = (r & MASK) ^ h ^ (h << 3);
    r as u32
}
#[inline(always)]
fn mul(a: u32, b: u32) -> u32 {
    unsafe {
        let r = _mm_cvtsi128_si64(_mm_clmulepi64_si128(
            _mm_set_epi64x(0, a as i64),
            _mm_set_epi64x(0, b as i64),
            0,
        )) as u64;
        red(r)
    }
}
#[inline(always)]
fn sq(a: u32) -> u32 {
    mul(a, a)
}
fn inv(a: u32) -> u32 {
    let mut r = a;
    for _ in 0..29 {
        r = mul(sq(r), a);
    }
    sq(r)
}
fn sqrt(a: u32) -> u32 {
    let mut r = a;
    for _ in 0..30 {
        r = sq(r);
    }
    r
}
#[inline(always)]
fn mulx(a: u32) -> u32 {
    red((a as u64) << 1)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn fe(&mut self) -> u32 {
        (self.next() & MASK) as u32
    }
}

struct Ctx {
    l: usize,
    v: Vec<u32>,
    elems: Vec<u32>,
    x1sq: Vec<u32>,
    invx1sq: Vec<u32>,
    trmask: u32,
    ht: [[u32; 256]; 4],
    rt: [[u32; 256]; 4],
}

fn lin_table(f: &dyn Fn(u32) -> u32) -> [[u32; 256]; 4] {
    let mut t = [[0u32; 256]; 4];
    for byte in 0..4 {
        for val in 0..256u32 {
            let mut acc = 0;
            for bit in 0..8 {
                if val >> bit & 1 == 1 && byte * 8 + bit < 31 {
                    acc ^= f(1u32 << (byte * 8 + bit));
                }
            }
            t[byte][val as usize] = acc;
        }
    }
    t
}
#[inline(always)]
fn app(t: &[[u32; 256]; 4], x: u32) -> u32 {
    t[0][(x & 255) as usize]
        ^ t[1][(x >> 8 & 255) as usize]
        ^ t[2][(x >> 16 & 255) as usize]
        ^ t[3][(x >> 24) as usize]
}

impl Ctx {
    fn new(l: usize, rng: &mut Rng) -> Ctx {
        // random rank-l basis
        let mut v: Vec<u32> = Vec::new();
        let mut ech: Vec<u32> = Vec::new(); // echelon, distinct top bits
        while v.len() < l {
            let c = rng.fe();
            let mut r = c;
            for &e in &ech {
                let top = 31 - e.leading_zeros();
                if r >> top & 1 == 1 {
                    r ^= e;
                }
            }
            if r != 0 {
                v.push(c);
                ech.push(r);
                ech.sort_by(|a, b| b.cmp(a));
                // make reduced: clear each pivot from the others
                let k = ech.len();
                for i in 0..k {
                    let top = 31 - ech[i].leading_zeros();
                    for j in 0..k {
                        if j != i && ech[j] >> top & 1 == 1 {
                            ech[j] ^= ech[i];
                        }
                    }
                }
                ech.sort_by(|a, b| b.cmp(a));
            }
        }
        let ech2 = ech.clone();
        let redf = move |mut x: u32| -> u32 {
            for &e in &ech2 {
                let top = 31 - e.leading_zeros();
                if x >> top & 1 == 1 {
                    x ^= e;
                }
            }
            x
        };
        let rt = lin_table(&redf);
        let htf = |a: u32| -> u32 {
            let mut acc = a;
            let mut t = a;
            for _ in 0..15 {
                t = sq(sq(t));
                acc ^= t;
            }
            acc
        };
        let ht = lin_table(&htf);
        let mut trmask = 0u32;
        for i in 0..31 {
            let a = 1u32 << i;
            let mut t = a;
            let mut acc = a;
            for _ in 0..30 {
                t = sq(t);
                acc ^= t;
            }
            assert!(acc <= 1);
            trmask |= acc << i;
        }
        let mut elems = vec![0u32; 1 << l];
        for idx in 1..(1usize << l) {
            let b = idx.trailing_zeros() as usize;
            elems[idx] = elems[idx & (idx - 1)] ^ v[b];
        }
        let x1sq: Vec<u32> = elems.iter().map(|&e| sq(e)).collect();
        let invx1sq: Vec<u32> = elems.iter().map(|&e| if e == 0 { 0 } else { sq(inv(e)) }).collect();
        Ctx { l, v, elems, x1sq, invx1sq, trmask, ht, rt }
    }
    #[inline(always)]
    fn tr(&self, a: u32) -> u32 {
        (a & self.trmask).count_ones() & 1
    }
    #[inline(always)]
    fn in_v(&self, a: u32) -> bool {
        app(&self.rt, a) == 0
    }
    // roots of al*x^2 + be*x + ga (not all zero)
    fn solve_quad(&self, al: u32, be: u32, ga: u32) -> (usize, [u32; 2]) {
        if al == 0 {
            if be == 0 {
                assert!(ga != 0, "degenerate quadratic");
                return (0, [0, 0]);
            }
            return (1, [mul(ga, inv(be)), 0]);
        }
        if be == 0 {
            return (1, [sqrt(mul(ga, inv(al))), 0]);
        }
        let ib = inv(be);
        let d = mul(mul(al, ga), sq(ib));
        if self.tr(d) == 1 {
            return (0, [0, 0]);
        }
        let z = app(&self.ht, d);
        let boa = mul(be, inv(al));
        let r1 = mul(z, boa);
        (2, [r1, r1 ^ boa])
    }
    // number of x2 in V, x2 != x1, with S3(x1,x2,X)=0 (slow generic path)
    fn slow_leaf(&self, x1: u32, x: u32) -> u32 {
        let al = sq(x1 ^ x);
        let be = mul(x1, x);
        let ga = mul(sq(x1), sq(x)) ^ CURVE_B;
        let (k, r) = self.solve_quad(al, be, ga);
        let mut c = 0;
        for i in 0..k {
            if self.in_v(r[i]) && r[i] != x1 {
                c += 1;
            }
        }
        c
    }
}

#[inline(always)]
fn s3(x1: u32, x2: u32, x: u32) -> u32 {
    let p = mul(x1, x2);
    let t = p ^ mul(x1 ^ x2, x);
    sq(t) ^ mul(p, x) ^ CURVE_B
}

// ---------- arm 1: scalar enumeration ----------
fn enum_scalar(c: &Ctx, xs: &[u32], counts: &mut [u32]) -> u64 {
    let n = 1usize << c.l;
    let mut units = 0u64;
    for i in 0..n {
        let x1 = c.elems[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let x2 = c.elems[j];
            let p = mul(x1, x2);
            let s = x1 ^ x2;
            let pb = sq(p) ^ CURVE_B;
            let s2 = sq(s);
            for (t, &x) in xs.iter().enumerate() {
                // (p + sX)^2 + pX + b = p^2 + s^2 X^2 + pX + b
                let r = pb ^ mul(s2, sq(x)) ^ mul(p, x);
                if r == 0 {
                    counts[t] += 1;
                }
            }
            units += xs.len() as u64;
        }
    }
    units
}

// ---------- arm 3: scalar linear scan ----------
fn scan_scalar(c: &Ctx, xs: &[u32], counts: &mut [u32], scratch: &mut Vec<u32>) -> u64 {
    let n = 1usize << c.l;
    let mut units = 0u64;
    scratch.resize(2 * n, 0);
    for (t, &x) in xs.iter().enumerate() {
        let x2 = sq(x);
        let invx2 = sq(inv(x));
        // batch inversion of d_i = elems[i]^x
        let (pre, dinv) = scratch.split_at_mut(n);
        let mut acc = 1u32;
        for i in 0..n {
            let d = c.elems[i] ^ x;
            pre[i] = acc;
            if d != 0 {
                acc = mul(acc, d);
            }
        }
        let mut ia = inv(acc);
        for i in (0..n).rev() {
            let d = c.elems[i] ^ x;
            if d != 0 {
                dinv[i] = mul(ia, pre[i]);
                ia = mul(ia, d);
            } else {
                dinv[i] = 0;
            }
        }
        let mut cnt = 0u32;
        for i in 0..n {
            let x1 = c.elems[i];
            if x1 == 0 || x1 == x {
                cnt += c.slow_leaf(x1, x);
                continue;
            }
            let ainv = sq(dinv[i]);
            let alpha = sq(x1 ^ x);
            let beta = mul(x1, x);
            let gamma = mul(c.x1sq[i], x2) ^ CURVE_B;
            let delta = mul(mul(alpha, gamma), mul(c.invx1sq[i], invx2));
            if c.tr(delta) == 1 {
                continue;
            }
            let z = app(&c.ht, delta);
            let boa = mul(beta, ainv);
            let r1 = mul(z, boa);
            let r2 = r1 ^ boa;
            if c.in_v(r1) && r1 != x1 {
                cnt += 1;
            }
            if c.in_v(r2) && r2 != x1 {
                cnt += 1;
            }
        }
        counts[t] += cnt;
        units += n as u64;
    }
    units
}

// ---------- bit-sliced planes shared by arms 2 and 4 ----------
struct Planes {
    w: usize,
    q: Vec<u64>,  // q[j*w+k]   = plane k of X^2 v_j^2
    pm: Vec<u64>, // pm[(i*l+j)*w+k] = plane k of X v_i v_j + v_i^2 v_j^2
    bconst: Vec<u64>,
    ops: u64,
}
fn lin_planes(cst: u32, planes: &[u64; NB], w: usize, out: &mut [u64]) -> u64 {
    let mut cs = cst;
    let mut ops = 0u64;
    for s in 0..NB {
        let mut m = cs & ((1u32 << w) - 1);
        ops += m.count_ones() as u64;
        while m != 0 {
            let k = m.trailing_zeros() as usize;
            out[k] ^= planes[s];
            m &= m - 1;
        }
        cs = mulx(cs);
    }
    ops
}
fn build_planes(c: &Ctx, xs: &[u32], w: usize) -> Planes {
    let l = c.l;
    let mut xp = [0u64; NB];
    let mut x2p = [0u64; NB];
    for (t, &x) in xs.iter().enumerate() {
        let x2 = sq(x);
        for s in 0..NB {
            xp[s] |= ((x >> s & 1) as u64) << t;
            x2p[s] |= ((x2 >> s & 1) as u64) << t;
        }
    }
    let mut ops = 0u64;
    let mut q = vec![0u64; l * w];
    for j in 0..l {
        ops += lin_planes(sq(c.v[j]), &x2p, w, &mut q[j * w..(j + 1) * w]);
    }
    let mut pm = vec![0u64; l * l * w];
    for i in 0..l {
        for j in 0..l {
            let o = (i * l + j) * w;
            let vv = mul(c.v[i], c.v[j]);
            ops += lin_planes(vv, &xp, w, &mut pm[o..o + w]);
            let k2 = sq(vv);
            for k in 0..w {
                if k2 >> k & 1 == 1 {
                    pm[o + k] ^= !0u64;
                }
            }
        }
    }
    let mut bconst = vec![0u64; w];
    for k in 0..w {
        if CURVE_B >> k & 1 == 1 {
            bconst[k] = !0u64;
        }
    }
    Planes { w, q, pm, bconst, ops }
}
// S[k*(l+1)+j]: column j<l of the affine system in the bits of x2, column l = rhs
fn sys_init(c: &Ctx, p: &Planes, s: &mut [u64]) {
    let l = c.l;
    for k in 0..p.w {
        for j in 0..l {
            s[k * (l + 1) + j] = p.q[j * p.w + k];
        }
        s[k * (l + 1) + l] = p.bconst[k];
    }
}
#[inline(always)]
fn sys_toggle(c: &Ctx, p: &Planes, s: &mut [u64], i: usize) -> u64 {
    let l = c.l;
    let w = p.w;
    for k in 0..w {
        let row = &mut s[k * (l + 1)..(k + 1) * (l + 1)];
        for j in 0..l {
            row[j] ^= p.pm[(i * l + j) * w + k];
        }
        row[l] ^= p.q[i * w + k];
    }
    (w * (l + 1)) as u64
}

// ---------- arm 2: bit-sliced enumeration ----------
fn enum_bs(c: &Ctx, xs: &[u32], counts: &mut [u32], fallbacks: &mut u64) -> u64 {
    let l = c.l;
    let w = 16usize;
    let p = build_planes(c, xs, w);
    let mut ops = p.ops;
    let mut s = vec![0u64; w * (l + 1)];
    sys_init(c, &p, &mut s);
    let n = 1usize << l;
    let mut gi = 0usize;
    let mut r = [0u64; 16];
    for step in 0..n {
        if step > 0 {
            let bit = step.trailing_zeros() as usize;
            gi ^= 1 << bit;
            ops += sys_toggle(c, &p, &mut s, bit);
        }
        let x1 = c.elems[gi];
        for k in 0..w {
            r[k] = s[k * (l + 1) + l];
        }
        let mut gj = 0usize;
        for st2 in 0..n {
            if st2 > 0 {
                let bit = st2.trailing_zeros() as usize;
                gj ^= 1 << bit;
                for k in 0..w {
                    r[k] ^= s[k * (l + 1) + bit];
                }
                ops += w as u64;
            }
            let mut orr = 0u64;
            let mut k = 0;
            while k < w && orr != !0u64 {
                orr |= r[k];
                k += 1;
            }
            ops += k as u64;
            let mut cand = !orr;
            if xs.len() < 64 {
                cand &= (1u64 << xs.len()) - 1;
            }
            while cand != 0 {
                let t = cand.trailing_zeros() as usize;
                cand &= cand - 1;
                *fallbacks += 1;
                if gi != gj && s3(x1, c.elems[gj], xs[t]) == 0 {
                    counts[t] += 1;
                }
            }
        }
    }
    ops
}

// ---------- arm 4: bit-sliced linear scan (treatment) ----------
fn scan_bs(c: &Ctx, xs: &[u32], counts: &mut [u32], fallbacks: &mut u64) -> u64 {
    let l = c.l;
    let w = l + 16;
    let lc = l + 1;
    let p = build_planes(c, xs, w);
    let mut ops = p.ops;
    let mut s = vec![0u64; w * lc];
    let mut wk = vec![0u64; w * lc];
    sys_init(c, &p, &mut s);
    let n = 1usize << l;
    let mut gi = 0usize;
    let lanes = if xs.len() < 64 { (1u64 << xs.len()) - 1 } else { !0u64 };
    for step in 0..n {
        if step > 0 {
            let bit = step.trailing_zeros() as usize;
            gi ^= 1 << bit;
            ops += sys_toggle(c, &p, &mut s, bit);
        }
        wk.copy_from_slice(&s);
        ops += (w * lc) as u64;
        let mut irregular = 0u64;
        for j in 0..l {
            // bring a pivot into row j, lane by lane
            let mut r = j + 1;
            while r < w && wk[j * lc + j] != !0u64 {
                let m = !wk[j * lc + j] & wk[r * lc + j];
                ops += 2;
                if m != 0 {
                    for cc in j..lc {
                        let add = wk[r * lc + cc] & m;
                        wk[j * lc + cc] ^= add;
                    }
                    ops += 2 * (lc - j) as u64;
                }
                r += 1;
            }
            irregular |= !wk[j * lc + j];
            // eliminate below
            let (top, bot) = wk.split_at_mut((j + 1) * lc);
            let prow = &top[j * lc..(j + 1) * lc];
            for r in (j + 1)..w {
                let row = &mut bot[(r - j - 1) * lc..(r - j) * lc];
                let m = row[j];
                for cc in (j + 1)..lc {
                    row[cc] ^= prow[cc] & m;
                }
            }
            ops += 2 * ((w - j - 1) * (lc - j - 1)) as u64;
        }
        let mut orr = 0u64;
        let mut r = l;
        while r < w && orr != !0u64 {
            orr |= wk[r * lc + l];
            r += 1;
        }
        ops += (r - l) as u64;
        let mut cand = (irregular | !orr) & lanes;
        let x1 = c.elems[gi];
        while cand != 0 {
            let t = cand.trailing_zeros() as usize;
            cand &= cand - 1;
            *fallbacks += 1;
            counts[t] += c.slow_leaf(x1, xs[t]);
        }
    }
    ops
}

// ---------- arm 5: hash join over all targets ----------
fn join(c: &Ctx, xs: &[u32], counts: &mut [u32]) -> u64 {
    let mut h: HashMap<u32, Vec<usize>> = HashMap::new();
    for (t, &x) in xs.iter().enumerate() {
        h.entry(x).or_default().push(t);
    }
    let n = 1usize << c.l;
    let mut units = 0u64;
    for i in 0..n {
        for j in (i + 1)..n {
            let x1 = c.elems[i];
            let x2 = c.elems[j];
            let s = x1 ^ x2;
            let p = mul(x1, x2);
            let (k, r) = c.solve_quad(sq(s), p, sq(p) ^ CURVE_B);
            units += 1;
            for q in 0..k {
                if let Some(ts) = h.get(&r[q]) {
                    for &t in ts {
                        counts[t] += 2;
                    }
                }
            }
        }
    }
    units
}

fn timed<F: FnMut() -> u64>(mut f: F) -> (u64, f64) {
    // repeat until >= 30 ms; return units of one call and ns per call
    let t0 = Instant::now();
    let mut it = 0u64;
    let mut u = 0;
    loop {
        u = std::hint::black_box(f());
        it += 1;
        if t0.elapsed().as_millis() >= 30 {
            break;
        }
    }
    let _ = u;
    (u, t0.elapsed().as_nanos() as f64 / it as f64)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let lmin: usize = a[1].parse().unwrap();
    let lmax: usize = a[2].parse().unwrap();
    let dir = a[3].clone();
    let reps = 3;
    for l in lmin..=lmax {
        let mut rng = Rng(20261114 + 1000 * l as u64);
        let c = Ctx::new(l, &mut rng);
        let mut rng = Rng(20261114 + 1000 * l as u64 + 1);
        let n = 1u64 << l;
        // targets
        let mut planted: Vec<u32> = Vec::new();
        while planted.len() < 256 {
            let i = (rng.next() % n) as usize;
            let j = (rng.next() % n) as usize;
            if i == j {
                continue;
            }
            let (x1, x2) = (c.elems[i], c.elems[j]);
            let p = mul(x1, x2);
            let (k, r) = c.solve_quad(sq(x1 ^ x2), p, sq(p) ^ CURVE_B);
            if k == 0 {
                continue;
            }
            let x = r[(rng.next() as usize) % k];
            if x == 0 || planted.contains(&x) {
                continue;
            }
            assert_eq!(s3(x1, x2, x), 0);
            planted.push(x);
        }
        let mut random: Vec<u32> = Vec::new();
        while random.len() < 256 {
            let x = rng.fe();
            if x != 0 && !planted.contains(&x) && !random.contains(&x) {
                random.push(x);
            }
        }
        let classes: [(&str, &Vec<u32>); 2] = [("planted", &planted), ("random", &random)];
        // reference counts + cross-check of all arms
        let mut all: Vec<u32> = planted.clone();
        all.extend(random.iter());
        let mut jc = vec![0u32; all.len()];
        let ju = join(&c, &all, &mut jc);
        let mut scratch = Vec::new();
        let mut refc: Vec<Vec<u32>> = Vec::new();
        for (ci, (cname, xs)) in classes.iter().enumerate() {
            let mut c3 = vec![0u32; 256];
            scan_scalar(&c, xs, &mut c3, &mut scratch);
            let mut c1 = vec![0u32; 256];
            enum_scalar(&c, xs, &mut c1);
            let mut c2 = vec![0u32; 256];
            let mut c4 = vec![0u32; 256];
            let (mut f2, mut f4) = (0u64, 0u64);
            let (mut o2, mut o4) = (0u64, 0u64);
            for b in 0..4 {
                o2 += enum_bs(&c, &xs[b * 64..(b + 1) * 64], &mut c2[b * 64..(b + 1) * 64], &mut f2);
                o4 += scan_bs(&c, &xs[b * 64..(b + 1) * 64], &mut c4[b * 64..(b + 1) * 64], &mut f4);
            }
            let ok = c1 == c3 && c2 == c3 && c4 == c3 && jc[ci * 256..(ci + 1) * 256] == c3[..];
            assert!(ok, "arm count mismatch l={} class={}", l, cname);
            let nz = c3.iter().filter(|&&x| x > 0).count();
            println!(
                "{{\"kind\":\"check\",\"l\":{},\"class\":\"{}\",\"arms_agree\":true,\"targets\":256,\"decomposable\":{},\"enum_bs_ops\":{},\"scan_bs_ops\":{},\"enum_bs_fallbacks\":{},\"scan_bs_fallbacks\":{}}}",
                l, cname, nz, o2, o4, f2, f4
            );
            refc.push(c3);
        }
        // instance dump
        let js = |v: &Vec<u32>| v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
        let inst = format!(
            "{{\"n\":31,\"poly\":\"x^31+x^3+1\",\"b\":{},\"l\":{},\"basis\":[{}],\"planted\":[{}],\"random\":[{}],\"count_planted\":[{}],\"count_random\":[{}]}}\n",
            CURVE_B, l, js(&c.v), js(&planted), js(&random), js(&refc[0]), js(&refc[1])
        );
        std::fs::write(format!("{}/instance_l{}.json", dir, l), inst).unwrap();
        // timed, interleaved
        for rep in 0..reps {
            for (cname, xs) in classes.iter() {
                let mut dummy = vec![0u32; 256];
                let mut fb = 0u64;
                let (u, ns) = timed(|| enum_scalar(&c, xs, &mut dummy));
                println!("{{\"kind\":\"time\",\"l\":{},\"class\":\"{}\",\"rep\":{},\"arm\":\"enum_scalar\",\"unit\":\"tuple_evals\",\"units_per_target\":{},\"ns_per_target\":{:.3}}}", l, cname, rep, u as f64 / 256.0, ns / 256.0);
                let (u, ns) = timed(|| {
                    let mut o = 0;
                    for b in 0..4 {
                        o += enum_bs(&c, &xs[b * 64..(b + 1) * 64], &mut dummy[b * 64..(b + 1) * 64], &mut fb);
                    }
                    o
                });
                println!("{{\"kind\":\"time\",\"l\":{},\"class\":\"{}\",\"rep\":{},\"arm\":\"enum_bitsliced\",\"unit\":\"u64_word_ops\",\"units_per_target\":{},\"ns_per_target\":{:.3}}}", l, cname, rep, u as f64 / 256.0, ns / 256.0);
                let (u, ns) = timed(|| scan_scalar(&c, xs, &mut dummy, &mut scratch));
                println!("{{\"kind\":\"time\",\"l\":{},\"class\":\"{}\",\"rep\":{},\"arm\":\"scan_scalar\",\"unit\":\"leaf_solves\",\"units_per_target\":{},\"ns_per_target\":{:.3}}}", l, cname, rep, u as f64 / 256.0, ns / 256.0);
                let (u, ns) = timed(|| {
                    let mut o = 0;
                    for b in 0..4 {
                        o += scan_bs(&c, &xs[b * 64..(b + 1) * 64], &mut dummy[b * 64..(b + 1) * 64], &mut fb);
                    }
                    o
                });
                println!("{{\"kind\":\"time\",\"l\":{},\"class\":\"{}\",\"rep\":{},\"arm\":\"scan_bitsliced\",\"unit\":\"u64_word_ops\",\"units_per_target\":{},\"ns_per_target\":{:.3}}}", l, cname, rep, u as f64 / 256.0, ns / 256.0);
            }
            let mut dj = vec![0u32; all.len()];
            let (u, ns) = timed(|| join(&c, &all, &mut dj));
            println!("{{\"kind\":\"time\",\"l\":{},\"class\":\"all512\",\"rep\":{},\"arm\":\"join\",\"unit\":\"quadratic_solves\",\"units_per_target\":{},\"ns_per_target\":{:.3}}}", l, rep, u as f64 / 512.0, ns / 512.0);
        }
        let _ = ju;
    }
}
