//! pcdeg: exact polynomial-calculus solving degree of digit-presented Semaev systems over F_p.
//! L3-digit lab, 2026-11-13. Std only. Build: rustc -O pcdeg.rs -o pcdeg
//!
//! Ring A = tensor_j F_p[v_j]/(prod_{c in X_j}(v_j - c)), monomial basis prod v_j^{e_j}, e_j < |X_j|,
//! degree = sum e_j.  f = S_{m+1}(l_1..l_m, r) with l_k = sum_{j in block k} w_j v_j, obtained exactly by
//! evaluating on the grid and interpolating axis by axis.  V_c = PC closure of {f} in A_{<=c}:
//! every basis row of degree < c is multiplied by every variable (reduced in A) until a fixed point.
//! Semi-echelon over columns sorted by (degree, index); leading column = highest index, so rows with a
//! low leading column span V_c ∩ A_{<c}.  Closure algorithm follows digit_fall.rs (repo study
//! research/prime_digit_presentation_20261029), generalised to arbitrary digit sets.
use std::collections::HashMap;
use std::time::Instant;

const NONE: u32 = u32::MAX;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
fn powmod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 {
    powmod(a % p, p - 2, p)
}

// ------------------------------------------------------------------ curve
struct Curve {
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    issq: Vec<bool>,
}
impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (x * x % p * x + self.a * x + self.b) % p
    }
    fn is_x(&self, x: u64) -> bool {
        self.issq[self.rhs(x) as usize]
    }
    fn sqrt(&self, v: u64) -> Option<u64> {
        // tonelli-shanks-free: brute only for small p is too slow; use generic Tonelli-Shanks
        let p = self.p;
        if v == 0 {
            return Some(0);
        }
        if !self.issq[v as usize] {
            return None;
        }
        let mut q = p - 1;
        let mut s = 0;
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = 2;
        while self.issq[z as usize] {
            z += 1;
        }
        let mut mm = s;
        let mut c = powmod(z, q, p);
        let mut t = powmod(v, q, p);
        let mut r = powmod(v, (q + 1) / 2, p);
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = tt * tt % p;
                i += 1;
            }
            let bb = powmod(c, 1 << (mm - i - 1), p);
            mm = i;
            c = bb * bb % p;
            t = t * c % p;
            r = r * bb % p;
        }
        Some(r)
    }
    fn add(&self, p1: Option<(u64, u64)>, p2: Option<(u64, u64)>) -> Option<(u64, u64)> {
        let p = self.p;
        let (x1, y1) = match p1 {
            None => return p2,
            Some(v) => v,
        };
        let (x2, y2) = match p2 {
            None => return p1,
            Some(v) => v,
        };
        let lam;
        if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            lam = (3 * x1 % p * x1 + self.a) % p * inv(2 * y1 % p, p) % p;
        } else {
            lam = (y2 + p - y1) % p * inv((x2 + p - x1) % p, p) % p;
        }
        let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
        let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
        Some((x3, y3))
    }
}
fn new_curve(rng: &mut Rng, bits: u32) -> Curve {
    loop {
        let lo = 1u64 << (bits - 1);
        let p = loop {
            let c = lo + rng.below(lo);
            if is_prime(c) && c % 4 == 3 || (is_prime(c) && c > 3) {
                break c;
            }
        };
        let mut issq = vec![false; p as usize];
        for x in 1..p {
            issq[(x * x % p) as usize] = true;
        }
        for _ in 0..200 {
            let a = rng.below(p);
            let b = 1 + rng.below(p - 1);
            if (4 * a % p * a % p * a + 27 * b % p * b) % p == 0 {
                continue;
            }
            let mut cnt = 1u64;
            for x in 0..p {
                let v = (x * x % p * x + a * x + b) % p;
                cnt += if v == 0 { 1 } else if issq[v as usize] { 2 } else { 0 };
            }
            if is_prime(cnt) {
                return Curve { p, a, b, order: cnt, issq };
            }
        }
    }
}

// ------------------------------------------------------------------ Semaev numerics
fn s3(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> u64 {
    let (c2, c1, c0) = s3coef(x1, x2, a, b, p);
    (c2 * x3 % p * x3 + c1 * x3 + c0) % p
}
/// S_3(x1,x2,u) = c2 u^2 + c1 u + c0
fn s3coef(x1: u64, x2: u64, a: u64, b: u64, p: u64) -> (u64, u64, u64) {
    let d = (x1 + p - x2) % p;
    let c2 = d * d % p;
    let s = (x1 + x2) % p;
    let pr = x1 * x2 % p;
    let c1 = (p - 2 * ((s * ((pr + a) % p) % p + 2 * b) % p) % p) % p;
    let t = (pr + p - a) % p;
    let c0 = (t * t % p + p - 4 * b % p * s % p) % p;
    (c2, c1, c0)
}
fn res22(f: (u64, u64, u64), g: (u64, u64, u64), p: u64) -> u64 {
    // Res of a2 u^2+a1 u+a0 and b2 u^2 + b1 u + b0
    let (a2, a1, a0) = f;
    let (b2, b1, b0) = g;
    let m = |x: u64, y: u64| x * y % p;
    let sub = |x: u64, y: u64| (x + p - y) % p;
    let t1 = sub(m(a2, b0), m(a0, b2));
    let t2 = sub(m(a2, b1), m(a1, b2));
    let t3 = sub(m(a1, b0), m(a0, b1));
    sub(m(t1, t1), m(t2, t3))
}
fn s4(x1: u64, x2: u64, x3: u64, r: u64, a: u64, b: u64, p: u64) -> u64 {
    res22(s3coef(x1, x2, a, b, p), s3coef(x3, r, a, b, p), p)
}
fn det(mut mtx: Vec<Vec<u64>>, p: u64) -> u64 {
    let n = mtx.len();
    let mut d = 1u64;
    for c in 0..n {
        let mut piv = None;
        for r in c..n {
            if mtx[r][c] != 0 {
                piv = Some(r);
                break;
            }
        }
        let r0 = match piv {
            None => return 0,
            Some(r) => r,
        };
        if r0 != c {
            mtx.swap(r0, c);
            d = (p - d) % p;
        }
        d = d * mtx[c][c] % p;
        let iv = inv(mtx[c][c], p);
        for r in c + 1..n {
            let f = mtx[r][c] * iv % p;
            if f == 0 {
                continue;
            }
            for k in c..n {
                mtx[r][k] = (mtx[r][k] + p - f * mtx[c][k] % p) % p;
            }
        }
    }
    d
}
fn s5(x: &[u64], r: u64, a: u64, b: u64, p: u64) -> u64 {
    // Res_u(S3(x1,x2,u), S4(u,x3,x4,r)); S4 in u has degree <= 4: interpolate on u = 0..4
    let ys: Vec<u64> = (0..5u64).map(|u| s4(u, x[2], x[3], r, a, b, p)).collect();
    let g = interp(&(0..5u64).collect::<Vec<_>>(), &ys, p); // g[0..5] coefficients
    let (c2, c1, c0) = s3coef(x[0], x[1], a, b, p);
    // Sylvester 6x6: f deg 2 (c2,c1,c0), g deg 4
    let mut m = vec![vec![0u64; 6]; 6];
    for i in 0..4 {
        m[i][i] = c2;
        m[i][i + 1] = c1;
        m[i][i + 2] = c0;
    }
    for i in 0..2 {
        for j in 0..5 {
            m[4 + i][i + j] = g[4 - j];
        }
    }
    det(m, p)
}
fn semaev(ls: &[u64], r: u64, cv: &Curve) -> u64 {
    let (a, b, p) = (cv.a, cv.b, cv.p);
    match ls.len() {
        1 => (ls[0] + p - r) % p, // "lin" family uses this path directly
        2 => s3(ls[0], ls[1], r, a, b, p),
        3 => s4(ls[0], ls[1], ls[2], r, a, b, p),
        4 => s5(ls, r, a, b, p),
        _ => panic!("m"),
    }
}
/// coefficients c_0..c_{n-1} of the polynomial through (xs[i], ys[i])
fn interp(xs: &[u64], ys: &[u64], p: u64) -> Vec<u64> {
    let vi = vand_inv(xs, p);
    let n = xs.len();
    (0..n).map(|e| (0..n).fold(0u64, |acc, i| (acc + vi[e][i] * ys[i]) % p)).collect()
}
/// inverse of V[i][e] = xs[i]^e, returned as W[e][i]
fn vand_inv(xs: &[u64], p: u64) -> Vec<Vec<u64>> {
    let n = xs.len();
    let mut m: Vec<Vec<u64>> = (0..n)
        .map(|i| {
            let mut row: Vec<u64> = (0..n).map(|e| powmod(xs[i], e as u64, p)).collect();
            row.extend((0..n).map(|j| (i == j) as u64));
            row
        })
        .collect();
    for c in 0..n {
        let r0 = (c..n).find(|&r| m[r][c] != 0).expect("singular vandermonde");
        m.swap(r0, c);
        let iv = inv(m[c][c], p);
        for k in 0..2 * n {
            m[c][k] = m[c][k] * iv % p;
        }
        for r in 0..n {
            if r != c && m[r][c] != 0 {
                let f = m[r][c];
                for k in 0..2 * n {
                    m[r][k] = (m[r][k] + p - f * m[c][k] % p) % p;
                }
            }
        }
    }
    // m = [I | V^{-1}], V^{-1}[e][i]: V^{-1} maps values (indexed i) to coefficients (indexed e)
    // rows of V are i, so V^{-1} rows are e.
    (0..n).map(|e| (0..n).map(|i| m[e][n + i]).collect()).collect()
}

// ------------------------------------------------------------------ presentation
struct Pres {
    p: u64,
    sets: Vec<Vec<u64>>, // X_j
    w: Vec<u64>,         // weights
    block: Vec<usize>,   // block of variable j
    m: usize,
    stride: Vec<usize>,
    g: usize, // grid size
}
impl Pres {
    fn new(p: u64, sets: Vec<Vec<u64>>, w: Vec<u64>, block: Vec<usize>, m: usize) -> Pres {
        let mut stride = vec![1usize; sets.len()];
        let mut g = 1usize;
        for j in 0..sets.len() {
            stride[j] = g;
            g *= sets[j].len();
        }
        Pres { p, sets, w, block, m, stride, g }
    }
    fn nv(&self) -> usize {
        self.sets.len()
    }
    fn digit(&self, idx: usize, j: usize) -> usize {
        (idx / self.stride[j]) % self.sets[j].len()
    }
    fn ls(&self, idx: usize) -> Vec<u64> {
        let mut l = vec![0u64; self.m];
        for j in 0..self.nv() {
            l[self.block[j]] = (l[self.block[j]] + self.w[j] * self.sets[j][self.digit(idx, j)]) % self.p;
        }
        l
    }
    fn deg(&self, idx: usize) -> u32 {
        (0..self.nv()).map(|j| self.digit(idx, j) as u32).sum()
    }
    /// values (indexed by grid idx, digit = position in set) -> coefficients (indexed by exponent idx)
    fn to_coef(&self, vals: &[u64]) -> Vec<u64> {
        let p = self.p;
        let mut cur = vals.to_vec();
        for j in 0..self.nv() {
            let vi = vand_inv(&self.sets[j], p);
            let b = self.sets[j].len();
            let st = self.stride[j];
            let mut nxt = vec![0u64; self.g];
            for idx in 0..self.g {
                if (idx / st) % b != 0 {
                    continue;
                }
                for e in 0..b {
                    let mut acc = 0u64;
                    for i in 0..b {
                        acc = (acc + vi[e][i] * cur[idx + i * st]) % p;
                    }
                    nxt[idx + e * st] = acc;
                }
            }
            cur = nxt;
        }
        cur
    }
    fn to_vals(&self, coef: &[u64]) -> Vec<u64> {
        let p = self.p;
        let mut cur = coef.to_vec();
        for j in 0..self.nv() {
            let b = self.sets[j].len();
            let st = self.stride[j];
            let mut nxt = vec![0u64; self.g];
            for idx in 0..self.g {
                if (idx / st) % b != 0 {
                    continue;
                }
                for i in 0..b {
                    let x = self.sets[j][i];
                    let mut acc = 0u64;
                    let mut xp = 1u64;
                    for e in 0..b {
                        acc = (acc + xp * cur[idx + e * st]) % p;
                        xp = xp * x % p;
                    }
                    nxt[idx + i * st] = acc;
                }
            }
            cur = nxt;
        }
        cur
    }
}

// ------------------------------------------------------------------ ring + closure
struct Ring {
    cols: Vec<usize>, // col -> grid exponent idx
    deg: Vec<u32>,
    col_of: Vec<u32>, // exponent idx -> col or NONE
    red: Vec<Vec<u64>>, // red[j][t]: v^{b} = sum_t red[j][t] v^t
}
impl Ring {
    fn new(pr: &Pres, c: u32) -> Ring {
        let mut v: Vec<(u32, usize)> = (0..pr.g).map(|i| (pr.deg(i), i)).filter(|x| x.0 <= c).collect();
        v.sort_unstable();
        let mut col_of = vec![NONE; pr.g];
        for (k, &(_, i)) in v.iter().enumerate() {
            col_of[i] = k as u32;
        }
        let p = pr.p;
        let red = pr
            .sets
            .iter()
            .map(|xs| {
                // prod (v - c) = v^b + sum pi_t v^t  -> v^b = sum (-pi_t) v^t
                let mut poly = vec![1u64];
                for &c in xs {
                    let mut np = vec![0u64; poly.len() + 1];
                    for (t, &co) in poly.iter().enumerate() {
                        np[t + 1] = (np[t + 1] + co) % p;
                        np[t] = (np[t] + p - co * c % p) % p;
                    }
                    poly = np;
                }
                let b = xs.len();
                (0..b).map(|t| (p - poly[t]) % p).collect()
            })
            .collect();
        Ring { deg: v.iter().map(|x| x.0).collect(), cols: v.iter().map(|x| x.1).collect(), col_of, red }
    }
}
fn times_var(pr: &Pres, ring: &Ring, row: &[(u32, u64)], j: usize, acc: &mut HashMap<u32, u64>) -> Vec<(u32, u64)> {
    let p = pr.p;
    acc.clear();
    let b = pr.sets[j].len();
    let st = pr.stride[j];
    for &(col, co) in row {
        let idx = ring.cols[col as usize];
        let e = (idx / st) % b;
        if e + 1 < b {
            let t = ring.col_of[idx + st];
            assert!(t != NONE, "degree overflow");
            *acc.entry(t).or_insert(0) += co;
        } else {
            let base = idx - e * st;
            for t in 0..b {
                let rc = ring.red[j][t];
                if rc == 0 {
                    continue;
                }
                let tc = ring.col_of[base + t * st];
                assert!(tc != NONE);
                *acc.entry(tc).or_insert(0) += co * rc % p;
            }
        }
    }
    let mut v: Vec<(u32, u64)> = acc.iter().map(|(&k, &c)| (k, c % p)).filter(|x| x.1 != 0).collect();
    v.sort_unstable();
    v
}
struct Ech {
    p: u64,
    piv: Vec<u32>,
    rows: Vec<Vec<u32>>,
    acc: Vec<u64>,
}
impl Ech {
    fn new(cols: usize, p: u64) -> Ech {
        Ech { p, piv: vec![NONE; cols], rows: vec![], acc: vec![0; cols] }
    }
    fn reduce_top(&mut self, sparse: &[(u32, u64)]) -> Option<usize> {
        // returns leading column after full top-down reduction (acc holds the reduced vector) or None
        let p = self.p;
        for x in self.acc.iter_mut() {
            *x = 0;
        }
        let mut top = 0usize;
        for &(c, v) in sparse {
            self.acc[c as usize] = v;
            top = top.max(c as usize + 1);
        }
        let mut ops = 0u32;
        loop {
            let mut t = None;
            for j in (0..top).rev() {
                if self.acc[j] % p != 0 {
                    t = Some(j);
                    break;
                }
            }
            let t = t?;
            let c = self.acc[t] % p;
            let ri = self.piv[t];
            if ri == NONE {
                return Some(t);
            }
            let k = p - c;
            let row = &self.rows[ri as usize];
            for (x, &y) in self.acc[..row.len()].iter_mut().zip(row.iter()) {
                *x += k * y as u64;
            }
            ops += 1;
            if ops % 4096 == 0 {
                for x in self.acc[..=t].iter_mut() {
                    *x %= p;
                }
            }
            top = t;
        }
    }
    fn insert(&mut self, sparse: &[(u32, u64)]) -> Option<usize> {
        let p = self.p;
        let t = self.reduce_top(sparse)?;
        let ic = inv(self.acc[t] % p, p);
        let row: Vec<u32> = self.acc[..=t].iter().map(|&x| ((x % p) * ic % p) as u32).collect();
        self.piv[t] = self.rows.len() as u32;
        self.rows.push(row);
        Some(t)
    }
    fn sparse_row(&self, t: usize) -> Vec<(u32, u64)> {
        self.rows[self.piv[t] as usize].iter().enumerate().filter(|(_, &v)| v != 0).map(|(j, &v)| (j as u32, v as u64)).collect()
    }
}
struct CRes {
    cols: usize,
    rank: usize,
    hit: bool,
    censored: bool,
    secs: f64,
}
/// closure of {f} at degree c; targets: if empty -> test 1; else test that all targets lie in V_c
fn closure(pr: &Pres, fco: &[u64], c: u32, targets: &[Vec<(usize, u64)>], tlimit: f64, t0: &Instant) -> CRes {
    let st = Instant::now();
    let ring = Ring::new(pr, c);
    let ncols = ring.cols.len();
    let fdeg = (0..pr.g).filter(|&i| fco[i] != 0).map(|i| pr.deg(i)).max().unwrap_or(0);
    let mut ech = Ech::new(ncols, pr.p);
    if fdeg > c {
        return CRes { cols: ncols, rank: 0, hit: false, censored: false, secs: 0.0 };
    }
    let frow: Vec<(u32, u64)> = {
        let mut v: Vec<(u32, u64)> = (0..pr.g).filter(|&i| fco[i] != 0).map(|i| (ring.col_of[i], fco[i])).collect();
        v.sort_unstable();
        v
    };
    let tg: Vec<Vec<(u32, u64)>> = targets
        .iter()
        .map(|t| {
            let mut v: Vec<(u32, u64)> = t.iter().map(|&(i, co)| (ring.col_of[i], co)).collect();
            v.sort_unstable();
            v
        })
        .collect();
    let mut queue = vec![];
    if let Some(t) = ech.insert(&frow) {
        if ring.deg[t] < c {
            queue.push(t);
        }
    }
    let mut hm = HashMap::new();
    let mut since = 0usize;
    let check = |ech: &mut Ech| -> bool {
        if tg.is_empty() {
            ech.piv[0] != NONE
        } else {
            tg.iter().all(|t| ech.reduce_top(t).is_none())
        }
    };
    while let Some(t) = queue.pop() {
        if tg.is_empty() && ech.piv[0] != NONE {
            break;
        }
        if !tg.is_empty() && since >= 512 {
            since = 0;
            if check(&mut ech) {
                break;
            }
        }
        if t0.elapsed().as_secs_f64() > tlimit {
            return CRes { cols: ncols, rank: ech.rows.len(), hit: false, censored: true, secs: st.elapsed().as_secs_f64() };
        }
        let row = ech.sparse_row(t);
        for j in 0..pr.nv() {
            let pr_ = times_var(pr, &ring, &row, j, &mut hm);
            if let Some(t2) = ech.insert(&pr_) {
                since += 1;
                if ring.deg[t2] < c {
                    queue.push(t2);
                }
            }
        }
    }
    let hit = check(&mut ech);
    CRes { cols: ncols, rank: ech.rows.len(), hit, censored: false, secs: st.elapsed().as_secs_f64() }
}


/// Red-team variant: closure of several axioms; order 0 = LIFO (as pcdeg), 1 = FIFO, 2 = pseudo-random pick.
/// Returns (cols, rank at stop, hit, rank_when_1_first_seen)
fn closure_multi(pr: &Pres, fcos: &[Vec<u64>], c: u32, order: u32) -> (usize, usize, bool) {
    let ring = Ring::new(pr, c);
    let ncols = ring.cols.len();
    let mut ech = Ech::new(ncols, pr.p);
    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for fco in fcos {
        let fdeg = (0..pr.g).filter(|&i| fco[i] != 0).map(|i| pr.deg(i)).max().unwrap_or(0);
        if fdeg > c { continue; }
        let mut v: Vec<(u32, u64)> = (0..pr.g).filter(|&i| fco[i] != 0).map(|i| (ring.col_of[i], fco[i])).collect();
        v.sort_unstable();
        if let Some(t) = ech.insert(&v) { if ring.deg[t] < c { queue.push_back(t); } }
    }
    let mut hm = HashMap::new();
    let mut rng = Rng(12345);
    loop {
        if ech.piv[0] != NONE { break; }
        let t = match order {
            0 => queue.pop_back(),
            1 => queue.pop_front(),
            _ => { if queue.is_empty() { None } else { let i = (rng.next() as usize) % queue.len(); queue.swap(i, 0); queue.pop_front() } }
        };
        let t = match t { None => break, Some(t) => t };
        let row = ech.sparse_row(t);
        for j in 0..pr.nv() {
            let pr_ = times_var(pr, &ring, &row, j, &mut hm);
            if let Some(t2) = ech.insert(&pr_) {
                if ring.deg[t2] < c { queue.push_back(t2); }
            }
        }
    }
    (ncols, ech.rows.len(), ech.piv[0] != NONE)
}

// ------------------------------------------------------------------ main
fn arg<T: std::str::FromStr>(args: &[String], name: &str, d: T) -> T {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(d)
}
fn args_s(args: &[String], name: &str, d: &str) -> String {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned().unwrap_or(d.to_string())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fam = args_s(&args, "--fam", "bool");
    let m: usize = arg(&args, "--m", 2);
    let k: usize = arg(&args, "--k", 3);
    let bsize: usize = arg(&args, "--B", 8);
    let bits: u32 = arg(&args, "--bits", 20);
    let seed: u64 = arg(&args, "--seed", 1);
    let draws: usize = arg(&args, "--draws", 1);
    let target = args_s(&args, "--target", "unsat");
    let cmax: u32 = arg(&args, "--cmax", 40);
    let tlimit: f64 = arg(&args, "--tlimit", 1100.0);
    let t00 = Instant::now();
    let census: u32 = arg(&args, "--census", 0);
    let draws = if census == 1 { usize::MAX } else { draws };
    for dr in 0..draws {
        let dseed = if census == 1 { seed.wrapping_mul(1_000_003).wrapping_add(17) } else { seed.wrapping_mul(1_000_003).wrapping_add(dr as u64 * 7919 + 17) };
        let mut rng = Rng(dseed);
        let cv = new_curve(&mut rng, bits);
        let p = cv.p;
        // build presentation
        let (mut sets, mut w, mut block) = (vec![], vec![], vec![]);
        let islin = fam.starts_with("lin") || fam == "sym";
        let mm = if islin { 1 } else { m };
        let mut bsz = 1u64;
        for blk in 0..mm {
            match fam.as_str() {
                "bool" | "b3" | "b4" | "mix" | "supp" | "lin" | "linb3" | "linrand" | "sym" => {
                    let mut wt = 1u64;
                    for i in 0..k {
                        let b = match fam.as_str() {
                            "b3" | "linb3" => 3,
                            "b4" => 4,
                            "mix" => if i % 2 == 0 { 2 } else { 3 },
                            _ => 2,
                        };
                        sets.push((0..b as u64).collect::<Vec<_>>());
                        w.push(if fam == "sym" { 1 } else if fam == "linrand" { 1 + rng.below(p - 1) } else { wt % p });
                        block.push(blk);
                        wt *= b;
                    }
                    bsz = if fam == "sym" { k as u64 + 1 } else { wt };
                }
                "uniint" => {
                    sets.push((0..bsize as u64).collect());
                    w.push(1);
                    block.push(blk);
                    bsz = bsize as u64;
                }
                "unirand" => {
                    let mut s: Vec<u64> = vec![];
                    while s.len() < bsize {
                        let x = rng.below(p);
                        if cv.is_x(x) && !s.contains(&x) {
                            s.push(x);
                        }
                    }
                    sets.push(s);
                    w.push(1);
                    block.push(blk);
                    bsz = bsize as u64;
                }
                _ => panic!("fam"),
            }
        }
        let pr = Pres::new(p, sets, w, block, mm);
        // choose target
        let mut planted: Option<usize> = None;
        let mut supp_coef: Option<Vec<u64>> = None;
        let rfix: i64 = if census == 1 { if dr as u64 >= p { break; } dr as i64 } else { arg(&args, "--r", -1) };
        let (r, vals) = loop {
            if rfix >= 0 {
                let r = rfix as u64 % p;
                let vals: Vec<u64> = (0..pr.g).map(|i| semaev(&pr.ls(i), r, &cv)).collect();
                break (r, vals);
            }
            if target == "plant" {
                let idx = rng.below(pr.g as u64) as usize;
                let ls = pr.ls(idx);
                if !ls.iter().all(|&x| cv.is_x(x)) {
                    continue;
                }
                let mut acc = None;
                for &x in &ls {
                    let y = cv.sqrt(cv.rhs(x)).unwrap();
                    let y = if rng.below(2) == 0 { y } else { (p - y) % p };
                    acc = cv.add(acc, Some((x, y)));
                }
                let r = match acc {
                    None => continue,
                    Some((x, _)) => x,
                };
                let vals: Vec<u64> = (0..pr.g).map(|i| semaev(&pr.ls(i), r, &cv)).collect();
                if vals[idx] != 0 {
                    eprintln!("V3 FAIL: planted not a zero");
                    std::process::exit(3);
                }
                planted = Some(idx);
                break (r, vals);
            } else {
                let r = if islin {
                    // unsat: rho outside range of l over the grid
                    let mut rr;
                    loop {
                        rr = rng.below(p);
                        if fam == "linrand" || rr >= bsz.max(k as u64 + 1) {
                            break;
                        }
                    }
                    rr
                } else {
                    rng.below(p)
                };
                let mut vals: Vec<u64> = (0..pr.g).map(|i| semaev(&pr.ls(i), r, &cv)).collect();
                if fam == "supp" {
                    let co = pr.to_coef(&vals);
                    let rc: Vec<u64> = co.iter().map(|&c| if c == 0 { 0 } else { 1 + rng.below(p - 1) }).collect();
                    vals = pr.to_vals(&rc);
                    supp_coef = Some(rc);
                }
                if vals.iter().all(|&v| v != 0) {
                    break (r, vals);
                }
            }
        };
        let nsol = vals.iter().filter(|&&v| v == 0).count();
        // V2 (m = 2): grid zeros with abscissa l's == decompositions from curve arithmetic
        let mut v2 = String::from("na");
        if mm == 2 && fam != "supp" && pr.g <= 70000 {
            let mut z_ab = 0;
            let mut dec = 0;
            for i in 0..pr.g {
                let ls = pr.ls(i);
                if !ls.iter().all(|&x| cv.is_x(x)) {
                    continue;
                }
                if vals[i] == 0 {
                    z_ab += 1;
                }
                let p1 = Some((ls[0], cv.sqrt(cv.rhs(ls[0])).unwrap()));
                let y2 = cv.sqrt(cv.rhs(ls[1])).unwrap();
                let mut ok = false;
                for y in [y2, (p - y2) % p] {
                    if let Some((x, _)) = cv.add(p1, Some((ls[1], y))) {
                        if x == r {
                            ok = true;
                        }
                    }
                }
                if ok {
                    dec += 1;
                }
            }
            v2 = if z_ab == dec { format!("ok({})", dec) } else { format!("FAIL({} vs {})", z_ab, dec) };
        }
        let fco = match supp_coef {
            Some(c) => c,
            None => pr.to_coef(&vals),
        };
        let fdeg = (0..pr.g).filter(|&i| fco[i] != 0).map(|i| pr.deg(i)).max().unwrap_or(0);
        let fsupp = fco.iter().filter(|&&c| c != 0).count();
        // targets for planted: basis of degree<=1 functions vanishing on the zero set
        let mut targets: Vec<Vec<(usize, u64)>> = vec![];
        if nsol > 0 {
            let zs: Vec<usize> = (0..pr.g).filter(|&i| vals[i] == 0).collect();
            // unknown alpha_0..alpha_n: alpha_0 + sum alpha_j x_j(z) = 0 for z in zs (x_j = set value)
            let n1 = pr.nv() + 1;
            let mut mat: Vec<Vec<u64>> = zs
                .iter()
                .map(|&z| {
                    let mut row = vec![1u64];
                    row.extend((0..pr.nv()).map(|j| pr.sets[j][pr.digit(z, j)]));
                    row
                })
                .collect();
            // nullspace via RREF
            let mut pivc = vec![];
            let mut rr = 0;
            for c in 0..n1 {
                if let Some(r0) = (rr..mat.len()).find(|&r| mat[r][c] != 0) {
                    mat.swap(r0, rr);
                    let iv = inv(mat[rr][c], p);
                    for x in mat[rr].iter_mut() {
                        *x = *x * iv % p;
                    }
                    for r2 in 0..mat.len() {
                        if r2 != rr && mat[r2][c] != 0 {
                            let f = mat[r2][c];
                            for cc in 0..n1 {
                                mat[r2][cc] = (mat[r2][cc] + p - f * mat[rr][cc] % p) % p;
                            }
                        }
                    }
                    pivc.push(c);
                    rr += 1;
                }
            }
            for free in 0..n1 {
                if pivc.contains(&free) {
                    continue;
                }
                let mut vecv = vec![0u64; n1];
                vecv[free] = 1;
                for (ri, &pc) in pivc.iter().enumerate() {
                    vecv[pc] = (p - mat[ri][free]) % p;
                }
                // to monomial basis: x_j value = sets[j][e] ; degree-1 monomial is v_j itself (exponent 1)
                let mut t = vec![];
                if vecv[0] != 0 {
                    t.push((0usize, vecv[0]));
                }
                for j in 0..pr.nv() {
                    if vecv[j + 1] != 0 {
                        t.push((pr.stride[j], vecv[j + 1]));
                    }
                }
                targets.push(t);
            }
        }

        let rt_order: u32 = arg(&args, "--order", 99);
        if rt_order < 99 {
            // order-dependence test on the unsplit system at given c range
            for c in fdeg.max(1)..=cmax {
                let (cols, rk, hit) = closure_multi(&pr, &[fco.clone()], c, rt_order);
                if hit { println!("{{\"test\":\"order\",\"order\":{},\"fam\":\"{}\",\"m\":{},\"k\":{},\"p\":{},\"r\":{},\"D\":{},\"cols\":{},\"rank_at_first_1\":{}}}", rt_order, fam, mm, k, p, r, c, cols, rk); break; }
            }
            continue;
        }
        let split: u32 = arg(&args, "--split", 0);
        if split == 1 && mm == 3 {
            let cu = cmax as usize;
            let mut sets2 = pr.sets.clone(); sets2.push((0..=cu as u64).collect());
            let mut w2 = pr.w.clone(); w2.push(1);
            let mut b2 = pr.block.clone(); b2.push(3);
            let pr2 = Pres::new(p, sets2, w2, b2, 4);
            let g1: Vec<u64> = (0..pr2.g).map(|i| { let l = pr2.ls(i); s3(l[0], l[1], l[3], cv.a, cv.b, p) }).collect();
            let g2: Vec<u64> = (0..pr2.g).map(|i| { let l = pr2.ls(i); s3(l[3], l[2], r, cv.a, cv.b, p) }).collect();
            let c1 = pr2.to_coef(&g1); let c2 = pr2.to_coef(&g2);
            let d1 = (0..pr2.g).filter(|&i| c1[i] != 0).map(|i| pr2.deg(i)).max().unwrap_or(0);
            let d2 = (0..pr2.g).filter(|&i| c2[i] != 0).map(|i| pr2.deg(i)).max().unwrap_or(0);
            let mut tr = vec![];
            let mut dsplit = String::from("null");
            for c in d1.min(d2).max(1)..=cmax {
                let st = Instant::now();
                let (cols, rk, hit) = closure_multi(&pr2, &[c1.clone(), c2.clone()], c, 0);
                tr.push(format!("[{},{},{},{},{:.2}]", c, cols, rk, hit as u8, st.elapsed().as_secs_f64()));
                if hit { dsplit = format!("{}", c); break; }
            }
            println!("{{\"test\":\"split\",\"nsol\":{},\"fam\":\"{}\",\"m\":3,\"k\":{},\"p\":{},\"r\":{},\"unsplit_fdeg\":{},\"g_degs\":[{},{}],\"D_split\":{},\"trace\":[{}]}}", nsol, fam, k, p, r, fdeg, d1, d2, dsplit, tr.join(","));
            continue;
        }
        let dump = args_s(&args, "--dump", "");
        if !dump.is_empty() {
            let names: Vec<String> = (0..pr.nv()).map(|j| format!("v{}", j)).collect();
            let mut out = format!("{}\n{}\n", names.join(","), p);
            let mono = |idx: usize| -> String {
                let mut t = vec![];
                for j in 0..pr.nv() {
                    let e = pr.digit(idx, j);
                    if e == 1 { t.push(names[j].clone()); } else if e > 1 { t.push(format!("{}^{}", names[j], e)); }
                }
                t.join("*")
            };
            let mut polys = vec![];
            let mut terms = vec![];
            for i in 0..pr.g { if fco[i] != 0 { let mo = mono(i); terms.push(if mo.is_empty() { format!("{}", fco[i]) } else { format!("{}*{}", fco[i], mo) }); } }
            polys.push(terms.join("+"));
            for j in 0..pr.nv() {
                let mut poly = vec![1u64];
                for &c in &pr.sets[j] {
                    let mut np = vec![0u64; poly.len() + 1];
                    for (t, &co) in poly.iter().enumerate() { np[t + 1] = (np[t + 1] + co) % p; np[t] = (np[t] + p - co * c % p) % p; }
                    poly = np;
                }
                let tt: Vec<String> = poly.iter().enumerate().filter(|x| *x.1 != 0).map(|(e, &co)| if e == 0 { format!("{}", co) } else if e == 1 { format!("{}*{}", co, names[j]) } else { format!("{}*{}^{}", co, names[j], e) }).collect();
                polys.push(tt.join("+"));
            }
            out.push_str(&polys.join(",\n"));
            out.push('\n');
            std::fs::write(format!("{}_{}.ms", dump, dr), out).unwrap();
        }
        let t0 = Instant::now();
        let mut trace = vec![];
        let mut dres: Option<(u32, CRes)> = None;
        let mut prev: Option<CRes> = None;
        let mut censored = false;
        let mut c = fdeg.max(1);
        while c <= cmax {
            let cr = closure(&pr, &fco, c, &targets, tlimit, &t0);
            trace.push(format!("[{},{},{},{},{:.2}]", c, cr.cols, cr.rank, cr.hit as u8, cr.secs));
            if cr.censored {
                censored = true;
                break;
            }
            if cr.hit {
                dres = Some((c, cr));
                break;
            }
            prev = Some(cr);
            c += 1;
        }
        let nv = pr.nv();
        let full: usize = pr.g;
        let (dstr, cols, rank) = match &dres {
            Some((d, cr)) => (format!("{}", d), cr.cols, cr.rank),
            None => (format!("\">{}\"", c - 1), 0, 0),
        };
        let cert = dres.is_some() && (prev.is_some() || dres.as_ref().unwrap().0 == fdeg.max(1));
        println!(
            "{{\"fam\":\"{}\",\"m\":{},\"k\":{},\"B\":{},\"nv\":{},\"grid\":{},\"p\":{},\"a\":{},\"b\":{},\"order\":{},\"r\":{},\"target\":\"{}\",\"planted\":{},\"nsol\":{},\"ntargets\":{},\"V2\":\"{}\",\"fdeg\":{},\"fsupp\":{},\"D\":{},\"certified\":{},\"censored\":{},\"cols_D\":{},\"rank_D\":{},\"rank_Dm1\":{},\"trace\":[{}],\"secs\":{:.2},\"seed\":{}}}",
            fam, mm, k, bsz, nv, full, p, cv.a, cv.b, cv.order, r, target,
            planted.map(|x| x as i64).unwrap_or(-1), nsol, targets.len(), v2, fdeg, fsupp, dstr, cert, censored, cols, rank,
            prev.as_ref().map(|x| x.rank).unwrap_or(0), trace.join(","), t0.elapsed().as_secs_f64(), dseed
        );
        let _ = &t00;
    }
}
