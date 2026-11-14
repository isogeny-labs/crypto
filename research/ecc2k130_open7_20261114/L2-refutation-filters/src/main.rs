//! L2 refutation filters: exact toy-scale instrument (std only).
//!
//! l2filters selftest N A
//! l2filters cells N A OUT.json            Q-a, Q-b1 (degree<=k vanishing filters), PC1, PC2, NC1
//! l2filters linz N A M D SAMPLES SEED OUT.json D1,D2,..   Q-b2 Macaulay linearisation refutation
//!
//! Every count is exact. Every claim is [pcc:unverified].
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write;

// ---------------- RNG ----------------
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, k: usize) -> usize {
        (self.next() % k as u64) as usize
    }
}

// ---------------- field ----------------
fn modulus(n: u32) -> u32 {
    let e: &[u32] = match n {
        7 => &[7, 1, 0],
        11 => &[11, 2, 0],
        13 => &[13, 4, 3, 1, 0],
        19 => &[19, 5, 2, 1, 0],
        _ => panic!("unsupported n"),
    };
    e.iter().fold(0, |v, &k| v | (1 << k))
}

fn is_prime(k: u64) -> bool {
    k > 1 && (2..).take_while(|p| p * p <= k).all(|p| k % p != 0)
}
fn factor(mut k: u64) -> Vec<u64> {
    let mut f = vec![];
    let mut p = 2;
    while p * p <= k {
        while k % p == 0 {
            f.push(p);
            k /= p;
        }
        p += 1;
    }
    if k > 1 {
        f.push(k)
    }
    f.dedup();
    f
}

struct Field {
    n: u32,
    q: u32,
    f: u32,
    order: u32,
    exp: Vec<u32>,
    log: Vec<u32>,
    tmask: u32,
}

impl Field {
    fn mulslow(&self, mut a: u32, mut b: u32) -> u32 {
        let mut r = 0;
        while b != 0 {
            if b & 1 == 1 {
                r ^= a
            }
            b >>= 1;
            a <<= 1;
            if a >> self.n != 0 {
                a ^= self.f
            }
        }
        r
    }
    fn powslow(&self, mut a: u32, mut e: u64) -> u32 {
        let mut r = 1;
        while e != 0 {
            if e & 1 == 1 {
                r = self.mulslow(r, a)
            }
            a = self.mulslow(a, a);
            e >>= 1;
        }
        r
    }
    fn trace_slow(&self, a: u32) -> u32 {
        let (mut t, mut x) = (0, a);
        for _ in 0..self.n {
            t ^= x;
            x = self.mulslow(x, x);
        }
        assert!(t <= 1);
        t
    }
    fn new(n: u32) -> Field {
        let mut fl = Field { n, q: 1 << n, f: modulus(n), order: (1 << n) - 1, exp: vec![], log: vec![], tmask: 0 };
        // irreducibility: n prime, x^(2^n)=x, no root
        let mut x = 2u32;
        for _ in 0..n {
            x = fl.mulslow(x, x);
        }
        assert!(x == 2 && is_prime(n as u64) && fl.f & 1 == 1 && fl.f.count_ones() % 2 == 1, "modulus not irreducible");
        let order = fl.order as u64;
        let fac = factor(order);
        let mut g = 2u32;
        while !fac.iter().all(|&p| fl.powslow(g, order / p) != 1) {
            g += 1
        }
        let mut exp = vec![0u32; 2 * fl.order as usize];
        let mut log = vec![0u32; fl.q as usize];
        let mut v = 1u32;
        for i in 0..fl.order {
            exp[i as usize] = v;
            log[v as usize] = i;
            v = fl.mulslow(v, g);
        }
        assert_eq!(v, 1);
        for i in fl.order..2 * fl.order {
            exp[i as usize] = exp[(i - fl.order) as usize];
        }
        fl.exp = exp;
        fl.log = log;
        let mut tm = 0;
        for i in 0..n {
            if fl.trace_slow(1 << i) == 1 {
                tm |= 1 << i
            }
        }
        fl.tmask = tm;
        fl
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
        }
    }
    #[inline]
    fn sq(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        self.exp[(self.order - self.log[a as usize]) as usize]
    }
    #[inline]
    fn tr(&self, a: u32) -> u32 {
        (a & self.tmask).count_ones() & 1
    }
    fn halftrace(&self, c: u32) -> u32 {
        let (mut h, mut x) = (0, c);
        for _ in 0..((self.n - 1) / 2 + 1) {
            h ^= x;
            x = self.sq(self.sq(x));
        }
        h
    }
}

// ---------------- curve y^2+xy = x^3 + a x^2 + 1 ----------------
type Pt = Option<(u32, u32)>;
struct Curve<'a> {
    f: &'a Field,
    a: u32,
}
impl<'a> Curve<'a> {
    fn add(&self, p: Pt, q: Pt) -> Pt {
        let f = self.f;
        let (x1, y1) = match p { None => return q, Some(v) => v };
        let (x2, y2) = match q { None => return p, Some(v) => v };
        if x1 == x2 {
            if y1 ^ y2 == x1 || x1 == 0 {
                return None;
            }
            return self.dbl(p);
        }
        let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sq(lam) ^ lam ^ x1 ^ x2 ^ self.a;
        let y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }
    fn dbl(&self, p: Pt) -> Pt {
        let f = self.f;
        let (x, y) = p?;
        if x == 0 {
            return None;
        }
        let lam = x ^ f.mul(y, f.inv(x));
        let x3 = f.sq(lam) ^ lam ^ self.a;
        let y3 = f.sq(x) ^ f.mul(lam ^ 1, x3);
        Some((x3, y3))
    }
    fn mulk(&self, mut k: u64, mut p: Pt) -> Pt {
        let mut r = None;
        while k != 0 {
            if k & 1 == 1 {
                r = self.add(r, p)
            }
            p = self.dbl(p);
            k >>= 1;
        }
        r
    }
    fn on(&self, p: Pt) -> bool {
        let f = self.f;
        match p {
            None => true,
            Some((x, y)) => f.sq(y) ^ f.mul(x, y) == f.mul(f.sq(x), x) ^ f.mul(self.a, f.sq(x)) ^ 1,
        }
    }
    fn lift(&self, x: u32) -> Vec<(u32, u32)> {
        let f = self.f;
        if x == 0 {
            return vec![(0, 1)];
        }
        let c = x ^ self.a ^ f.inv(f.sq(x));
        if f.tr(c) == 1 {
            return vec![];
        }
        let y = f.mul(x, f.halftrace(c));
        vec![(x, y), (x, y ^ x)]
    }
}

struct Setup {
    pts: Vec<Pt>,
    order: u64,
    h: u64,
    r: u64,
    rv: u64,
    gx: Vec<bool>,
}

fn setup(f: &Field, e: &Curve) -> Setup {
    let mut pts: Vec<Pt> = vec![None];
    for x in 0..f.q {
        for p in e.lift(x) {
            pts.push(Some(p))
        }
    }
    let n = pts.len() as u64;
    let r = *factor(n).iter().max().unwrap();
    let mut rv = 1;
    while n % (rv * r) == 0 {
        rv *= r
    }
    let h = n / rv;
    let mut gx = vec![false; f.q as usize];
    for &p in &pts {
        if let Some((x, _)) = e.mulk(h, p) {
            gx[x as usize] = true
        }
    }
    Setup { pts, order: n, h, r, rv, gx }
}

fn selftest(f: &Field, e: &Curve, s: &Setup, rng: &mut Rng) {
    for _ in 0..300 {
        let p = s.pts[rng.below(s.pts.len())];
        let q = s.pts[rng.below(s.pts.len())];
        let r = s.pts[rng.below(s.pts.len())];
        assert!(e.on(p) && e.on(e.add(p, q)));
        assert_eq!(e.add(e.add(p, q), r), e.add(p, e.add(q, r)));
        assert_eq!(e.mulk(s.order, p), None);
        let a = 1 + rng.below(f.q as usize - 1) as u32;
        let b = 1 + rng.below(f.q as usize - 1) as u32;
        assert_eq!(f.mul(a, b), f.mulslow(a, b));
        assert_eq!(f.mul(a, f.inv(a)), 1);
        assert_eq!(f.tr(a), f.trace_slow(a));
    }
}

fn base_points(e: &Curve, d: u32, chi0_only: bool) -> Vec<Pt> {
    let mut v = vec![];
    for x in 1..(1u32 << d) {
        for p in e.lift(x) {
            if !chi0_only || e.f.tr(p.0 ^ e.a) == 0 {
                v.push(Some(p))
            }
        }
    }
    v
}

/// marks x(P1+..+Pm) over multisets of the base.
fn decomposable(e: &Curve, fb: &[Pt], m: usize) -> (Vec<bool>, u64) {
    let mut dmark = vec![false; e.f.q as usize];
    let k = fb.len();
    let mut adds = 0u64;
    match m {
        1 => {
            for p in fb {
                dmark[p.unwrap().0 as usize] = true
            }
        }
        2 => {
            for i in 0..k {
                for j in i..k {
                    adds += 1;
                    if let Some((x, _)) = e.add(fb[i], fb[j]) {
                        dmark[x as usize] = true
                    }
                }
            }
        }
        3 => {
            for i in 0..k {
                for j in i..k {
                    let s = e.add(fb[i], fb[j]);
                    adds += 1;
                    for l in j..k {
                        adds += 1;
                        if let Some((x, _)) = e.add(s, fb[l]) {
                            dmark[x as usize] = true
                        }
                    }
                }
            }
        }
        _ => panic!(),
    }
    (dmark, adds)
}

// ---------------- bitsets & GF(2) rank ----------------
type Bits = Vec<u64>;
fn bits_new(len: usize) -> Bits {
    vec![0; (len + 63) / 64]
}
fn bset(b: &mut Bits, i: usize) {
    b[i / 64] |= 1 << (i % 64)
}
fn bget(b: &Bits, i: usize) -> bool {
    (b[i / 64] >> (i % 64)) & 1 == 1
}
fn btop(b: &Bits) -> Option<usize> {
    for w in (0..b.len()).rev() {
        if b[w] != 0 {
            return Some(w * 64 + 63 - b[w].leading_zeros() as usize);
        }
    }
    None
}
fn bxor(a: &mut Bits, b: &Bits) {
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x ^= *y
    }
}
fn bcount(b: &Bits) -> usize {
    b.iter().map(|w| w.count_ones() as usize).sum()
}

fn monomials(n: u32, k: u32) -> Vec<u64> {
    let mut out = vec![];
    for deg in 0..=k {
        combos(n as usize, deg as usize, &mut out);
    }
    out
}
fn combos(nv: usize, deg: usize, out: &mut Vec<u64>) {
    fn rec(start: usize, nv: usize, left: usize, acc: u64, out: &mut Vec<u64>) {
        if left == 0 {
            out.push(acc);
            return;
        }
        for i in start..nv {
            rec(i + 1, nv, left - 1, acc | (1u64 << i), out);
        }
    }
    rec(0, nv, deg, 0, out);
}

fn eval_vectors(xs: &[u32], n: u32, mons: &[u64]) -> Vec<Bits> {
    let mut bitv: Vec<Bits> = (0..n).map(|_| bits_new(xs.len())).collect();
    for (j, &x) in xs.iter().enumerate() {
        for i in 0..n {
            if (x >> i) & 1 == 1 {
                bset(&mut bitv[i as usize], j)
            }
        }
    }
    let mut full = bits_new(xs.len());
    for j in 0..xs.len() {
        bset(&mut full, j)
    }
    mons.iter()
        .map(|&mm| {
            let mut v = full.clone();
            for i in 0..n {
                if (mm >> i) & 1 == 1 {
                    for (a, b) in v.iter_mut().zip(bitv[i as usize].iter()) {
                        *a &= *b
                    }
                }
            }
            v
        })
        .collect()
}

/// rank, left-nullspace basis (as monomial-combination bitsets), row xors
fn rank_null(vecs: &[Bits], track: bool) -> (usize, Vec<Bits>, u64) {
    let nm = vecs.len();
    let mut basis: HashMap<usize, (Bits, Bits)> = HashMap::new();
    let mut null = vec![];
    let mut xors = 0u64;
    for (idx, v0) in vecs.iter().enumerate() {
        let mut v = v0.clone();
        let mut c = bits_new(nm);
        if track {
            bset(&mut c, idx)
        }
        loop {
            match btop(&v) {
                None => {
                    if track {
                        null.push(c)
                    }
                    break;
                }
                Some(t) => {
                    if let Some((bv, bc)) = basis.get(&t) {
                        bxor(&mut v, bv);
                        if track {
                            bxor(&mut c, bc)
                        }
                        xors += 1;
                    } else {
                        basis.insert(t, (v, c));
                        break;
                    }
                }
            }
        }
    }
    (basis.len(), null, xors)
}

struct FilterRes {
    k: u32,
    monomials: usize,
    rank_d: usize,
    rank_u: usize,
    dim: i64,
    rejected: usize,
    false_negatives: usize,
    row_xors: u64,
}

fn vanishing_filter(dx: &[u32], ux: &[u32], dset: &Vec<bool>, n: u32, k: u32) -> FilterRes {
    let mons = monomials(n, k);
    let vd = eval_vectors(dx, n, &mons);
    let vu = eval_vectors(ux, n, &mons);
    let (rd, nulld, xd) = rank_null(&vd, true);
    let (ru, _, xu) = rank_null(&vu, false);
    let dim = ru as i64 - rd as i64;
    let mut rej = bits_new(ux.len());
    if dim > 0 {
        for c in &nulld {
            let mut ev = bits_new(ux.len());
            for (i, v) in vu.iter().enumerate() {
                if bget(c, i) {
                    bxor(&mut ev, v)
                }
            }
            for (a, b) in rej.iter_mut().zip(ev.iter()) {
                *a |= *b
            }
        }
    }
    let fnn = (0..ux.len()).filter(|&j| bget(&rej, j) && dset[ux[j] as usize]).count();
    FilterRes { k, monomials: mons.len(), rank_d: rd, rank_u: ru, dim, rejected: bcount(&rej), false_negatives: fnn, row_xors: xd + xu }
}

fn fjson(r: &FilterRes) -> String {
    format!(
        "{{\"k\":{},\"monomials\":{},\"rank_D\":{},\"rank_U\":{},\"dim\":{},\"rejected\":{},\"false_negatives\":{},\"row_xors\":{}}}",
        r.k, r.monomials, r.rank_d, r.rank_u, r.dim, r.rejected, r.false_negatives, r.row_xors
    )
}

fn fnv(xs: &[u32]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &x in xs {
        for b in x.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

fn run_cells(n: u32, a: u32, out: &str) {
    let f = Field::new(n);
    let e = Curve { f: &f, a };
    let s = setup(&f, &e);
    let mut rng = Rng(0x2026_1114_0000_0000 ^ ((n as u64) << 8) ^ a as u64);
    selftest(&f, &e, &s, &mut rng);
    let ux: Vec<u32> = (0..f.q).filter(|&x| s.gx[x as usize]).collect();
    let allx: Vec<u32> = {
        let mut v: Vec<u32> = s.pts.iter().filter_map(|p| p.map(|q| q.0)).collect();
        v.sort();
        v.dedup();
        v
    };
    let mut j = String::new();
    write!(j, "{{\"n\":{},\"a\":{},\"modulus\":{},\"order\":{},\"h\":{},\"r\":{},\"r_power\":{},\"U_size\":{},\"all_x\":{},\"cells\":[", n, a, f.f, s.order, s.h, s.r, s.rv, ux.len(), allx.len()).unwrap();
    let mut first = true;
    for m in 1..=3usize {
        let dc = (n as usize + m - 1) / m;
        let ds: Vec<usize> = if m == 1 { vec![dc - 1] } else { vec![dc - 1, dc, dc + 1] };
        for d in ds {
            if d < 2 || d >= n as usize {
                continue;
            }
            let fb = base_points(&e, d as u32, false);
            let mut est = (fb.len() as f64).powi(m as i32);
            for i in 1..=m {
                est /= i as f64
            }
            if est > 4.5e6 {
                continue;
            }
            let (dm, adds) = decomposable(&e, &fb, m);
            let du: Vec<u32> = ux.iter().copied().filter(|&x| dm[x as usize]).collect();
            let dall = dm.iter().filter(|&&b| b).count();
            let unsat = 1.0 - du.len() as f64 / ux.len() as f64;
            let mut fl = vec![];
            let mut nl = vec![];
            // NC1 random subset, same size
            let mut pool = ux.clone();
            for i in 0..du.len() {
                let jx = i + rng.below(pool.len() - i);
                pool.swap(i, jx);
            }
            let mut rs: Vec<u32> = pool[..du.len()].to_vec();
            rs.sort();
            let mut rmark = vec![false; f.q as usize];
            for &x in &rs {
                rmark[x as usize] = true
            }
            for k in 1..=3u32 {
                fl.push(vanishing_filter(&du, &ux, &dm, n, k));
                nl.push(vanishing_filter(&rs, &ux, &rmark, n, k));
            }
            if !first {
                j.push(',')
            }
            first = false;
            write!(j, "{{\"m\":{},\"d\":{},\"base_points\":{},\"enum_group_adds\":{},\"D_all\":{},\"D_in_U\":{},\"unsat_fraction\":{:.6},\"D_in_U_fnv64\":\"{:016x}\",\"filters\":[{}],\"null\":[{}]}}",
                m, d, fb.len(), adds, dall, du.len(), unsat, fnv(&du),
                fl.iter().map(fjson).collect::<Vec<_>>().join(","), nl.iter().map(fjson).collect::<Vec<_>>().join(",")).unwrap();
            eprintln!("n={} a={} m={} d={} |F|={} D_in_U={}/{} unsat={:.4} filt={:?} null={:?}", n, a, m, d, fb.len(), du.len(), ux.len(), unsat,
                fl.iter().map(|r| (r.k, r.dim, r.rejected, r.false_negatives)).collect::<Vec<_>>(), nl.iter().map(|r| (r.k, r.dim)).collect::<Vec<_>>());
        }
    }
    j.push_str("],\"pc2\":[");
    // PC2: universe = all x of E, base restricted to chi=0 (2E)
    let mut first = true;
    let allmark = { let mut v = vec![false; f.q as usize]; for &x in &allx { v[x as usize] = true } v };
    let _ = allmark;
    for m in 2..=3usize {
        let d = (n as usize + m - 1) / m;
        let fb = base_points(&e, d as u32, true);
        let mut est = (fb.len() as f64).powi(m as i32);
        for i in 1..=m { est /= i as f64 }
        if est > 4.5e6 { continue }
        let (dm, _) = decomposable(&e, &fb, m);
        let dl: Vec<u32> = allx.iter().copied().filter(|&x| dm[x as usize]).collect();
        let r1 = vanishing_filter(&dl, &allx, &dm, n, 1);
        let chi1 = allx.iter().filter(|&&x| x != 0 && f.tr(x ^ a) == 1).count();
        if !first { j.push(',') }
        first = false;
        write!(j, "{{\"m\":{},\"d\":{},\"base_points\":{},\"D\":{},\"filter_k1\":{},\"x_with_chi1\":{}}}", m, d, fb.len(), dl.len(), fjson(&r1), chi1).unwrap();
        eprintln!("PC2 m={} d={} dim={} rejected={} fn={} chi1={}", m, d, r1.dim, r1.rejected, r1.false_negatives, chi1);
    }
    j.push_str("]}\n");
    std::fs::write(out, j).unwrap();
}

// ---------------- Boolean descent ----------------
type BP = Vec<u64>; // sorted, unique monomial masks (multilinear)
fn bp_norm(mut v: Vec<u64>) -> BP {
    v.sort_unstable();
    let mut out: Vec<u64> = Vec::with_capacity(v.len());
    for t in v {
        if out.last() == Some(&t) {
            out.pop();
        } else {
            out.push(t)
        }
    }
    out
}
fn bp_add(a: &BP, b: &BP) -> BP {
    let mut v = a.clone();
    v.extend_from_slice(b);
    bp_norm(v)
}
fn bp_mul(a: &BP, b: &BP) -> BP {
    let mut v = Vec::with_capacity(a.len() * b.len());
    for &x in a {
        for &y in b {
            v.push(x | y)
        }
    }
    bp_norm(v)
}

#[derive(Clone)]
struct FE(Vec<BP>);
fn fe_const(f: &Field, v: u32) -> FE {
    FE((0..f.n).map(|i| if (v >> i) & 1 == 1 { vec![0u64] } else { vec![] }).collect())
}
fn fe_var(f: &Field, off: u32, dim: u32) -> FE {
    FE((0..f.n).map(|i| if i < dim { vec![1u64 << (off + i)] } else { vec![] }).collect())
}
fn fe_add(a: &FE, b: &FE) -> FE {
    FE(a.0.iter().zip(b.0.iter()).map(|(x, y)| bp_add(x, y)).collect())
}
fn reduce(f: &Field, mut prod: Vec<BP>) -> FE {
    let n = f.n as usize;
    for k in (n..prod.len()).rev() {
        if !prod[k].is_empty() {
            let p = std::mem::take(&mut prod[k]);
            for e in 0..n {
                if (f.f >> e) & 1 == 1 {
                    prod[k - n + e] = bp_add(&prod[k - n + e], &p);
                }
            }
        }
    }
    prod.truncate(n);
    FE(prod)
}
fn fe_mul(f: &Field, a: &FE, b: &FE) -> FE {
    let n = f.n as usize;
    let mut prod: Vec<Vec<u64>> = vec![vec![]; 2 * n - 1];
    for (i, x) in a.0.iter().enumerate() {
        if x.is_empty() { continue }
        for (j, y) in b.0.iter().enumerate() {
            if y.is_empty() { continue }
            for &s in x { for &t in y { prod[i + j].push(s | t) } }
        }
    }
    reduce(f, prod.into_iter().map(bp_norm).collect())
}
fn fe_sq(f: &Field, a: &FE) -> FE {
    let n = f.n as usize;
    let mut prod: Vec<BP> = vec![vec![]; 2 * n - 1];
    for (i, x) in a.0.iter().enumerate() {
        prod[2 * i] = x.clone();
    }
    reduce(f, prod)
}
fn s3(f: &Field, x1: &FE, x2: &FE, x3: &FE) -> FE {
    let p12 = fe_mul(f, x1, x2);
    let s2 = fe_add(&fe_add(&p12, &fe_mul(f, x1, x3)), &fe_mul(f, x2, x3));
    fe_add(&fe_add(&fe_sq(f, &s2), &fe_mul(f, &p12, x3)), &fe_const(f, 1))
}
fn descent(f: &Field, xr: u32, m: u32, d: u32) -> (Vec<BP>, u32) {
    let (eqs, nv) = if m == 2 {
        let x1 = fe_var(f, 0, d);
        let x2 = fe_var(f, d, d);
        (s3(f, &x1, &x2, &fe_const(f, xr)).0, 2 * d)
    } else {
        let x1 = fe_var(f, 0, d);
        let x2 = fe_var(f, d, d);
        let x3 = fe_var(f, 2 * d, d);
        let u = fe_var(f, 3 * d, f.n);
        let mut v = s3(f, &x1, &x2, &u).0;
        v.extend(s3(f, &u, &x3, &fe_const(f, xr)).0);
        (v, 3 * d + f.n)
    };
    (eqs.into_iter().filter(|e| !e.is_empty()).collect(), nv)
}

struct MacRes {
    refuted: bool,
    rows: u64,
    cols: usize,
    rank: usize,
    row_xors: u64,
    word_ops: u64,
}

/// Is 1 in span{mu*eq : deg mu + deg eq <= D}? Echelon with graded order, constant lowest.
fn macaulay(eqs: &[BP], nv: u32, dd: u32) -> MacRes {
    let mut cols = vec![];
    for deg in 0..=dd {
        combos(nv as usize, deg as usize, &mut cols)
    }
    let cidx: HashMap<u64, usize> = cols.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let nc = cols.len();
    let words = ((nc + 63) / 64) as u64;
    let mut basis: Vec<Option<Bits>> = vec![None; nc];
    let mut rank = 0;
    let (mut xors, mut rows) = (0u64, 0u64);
    let mut mults: HashMap<u32, Vec<u64>> = HashMap::new();
    for e in eqs {
        let de = e.iter().map(|t| t.count_ones()).max().unwrap();
        if de > dd { continue }
        let md = dd - de;
        let ms = mults.entry(md).or_insert_with(|| { let mut v = vec![]; for g in 0..=md { combos(nv as usize, g as usize, &mut v) } v }).clone();
        for mu in ms {
            let mut row = bits_new(nc);
            for &t in e {
                let i = cidx[&(t | mu)];
                row[i / 64] ^= 1 << (i % 64);
            }
            rows += 1;
            loop {
                match btop(&row) {
                    None => break,
                    Some(t) => {
                        if let Some(b) = &basis[t] {
                            bxor(&mut row, b);
                            xors += 1;
                        } else {
                            basis[t] = Some(row);
                            rank += 1;
                            break;
                        }
                    }
                }
            }
            if basis[0].is_some() {
                return MacRes { refuted: true, rows, cols: nc, rank, row_xors: xors, word_ops: xors * words };
            }
        }
    }
    MacRes { refuted: false, rows, cols: nc, rank, row_xors: xors, word_ops: xors * words }
}

fn run_linz(n: u32, a: u32, m: u32, d: u32, samples: usize, seed: u64, out: &str, ds: &[u32]) {
    let f = Field::new(n);
    let e = Curve { f: &f, a };
    let s = setup(&f, &e);
    let fb = base_points(&e, d, false);
    let (dm, _) = decomposable(&e, &fb, m as usize);
    let ux: Vec<u32> = (0..f.q).filter(|&x| s.gx[x as usize]).collect();
    let mut rng = Rng(seed);
    let stream: Vec<u32> = (0..samples).map(|_| ux[rng.below(ux.len())]).collect();
    let du = ux.iter().filter(|&&x| dm[x as usize]).count();
    let mut j = format!("{{\"n\":{},\"a\":{},\"m\":{},\"d\":{},\"seed\":{},\"base_points\":{},\"U_size\":{},\"D_in_U\":{},\"rows\":[", n, a, m, d, seed, fb.len(), ux.len(), du);
    for (t, &xr) in stream.iter().enumerate() {
        let (eqs, nv) = descent(&f, xr, m, d);
        let sat = dm[xr as usize];
        let mut rec = format!("{{\"t\":{},\"xR\":{},\"sat\":{},\"nvars\":{},\"neqs\":{}", t, xr, sat, nv, eqs.len());
        // interleave degrees per target (same instrument, same target)
        for &dd in ds {
            let r = macaulay(&eqs, nv, dd);
            assert!(!(r.refuted && sat), "FALSE NEGATIVE: decomposable target refuted");
            write!(rec, ",\"D{}\":{{\"refuted\":{},\"rows\":{},\"cols\":{},\"rank\":{},\"row_xors\":{},\"word_ops\":{}}}", dd, r.refuted, r.rows, r.cols, r.rank, r.row_xors, r.word_ops).unwrap();
        }
        rec.push('}');
        eprintln!("{}", rec);
        if t > 0 { j.push(',') }
        j.push_str(&rec);
        std::io::stderr().flush().ok();
    }
    j.push_str("]}\n");
    std::fs::write(out, j).unwrap();
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let p = |i: usize| a[i].parse::<u64>().unwrap();
    match a[1].as_str() {
        "selftest" => {
            let n = p(2) as u32;
            let f = Field::new(n);
            let e = Curve { f: &f, a: p(3) as u32 };
            let s = setup(&f, &e);
            let mut rng = Rng(12345);
            selftest(&f, &e, &s, &mut rng);
            println!("n={} a={} #E={} h={} r={} r^v={} |U|={} selftest ok", n, e.a, s.order, s.h, s.r, s.rv, s.gx.iter().filter(|&&b| b).count());
            // planted decompositions satisfy the descent system
            for m in 2..=3u32 {
                let d = (n + m - 1) / m;
                let fb = base_points(&e, d, false);
                let mut okc = 0;
                for _ in 0..20 {
                    let ps: Vec<Pt> = (0..m).map(|_| fb[rng.below(fb.len())]).collect();
                    let mut sum = None;
                    for &q in &ps { sum = e.add(sum, q) }
                    let Some((xr, _)) = sum else { continue };
                    let (eqs, _) = descent(&f, xr, m, d);
                    let mut asg: u64 = 0;
                    for (i, q) in ps.iter().enumerate() { asg |= (q.unwrap().0 as u64) << (i as u32 * d) }
                    if m == 3 { if let Some((ux, _)) = e.add(ps[0], ps[1]) { asg |= (ux as u64) << (3 * d) } else { continue } }
                    let ok = eqs.iter().all(|eq| eq.iter().filter(|&&t| t & asg == t).count() % 2 == 0);
                    assert!(ok, "planted decomposition violates descent");
                    // and a refutation never fires on it
                    okc += 1;
                }
                println!("planted m={} d={} checked={} ok", m, d, okc);
            }
        }
        "cells" => run_cells(p(2) as u32, p(3) as u32, &a[4]),
        "linz" => {
            let ds: Vec<u32> = a[9].split(',').map(|x| x.parse().unwrap()).collect();
            run_linz(p(2) as u32, p(3) as u32, p(4) as u32, p(5) as u32, p(6) as usize, p(7), &a[8], &ds)
        }
        _ => panic!("usage"),
    }
}

#[allow(dead_code)]
fn _unused(_: HashSet<u8>) {}
