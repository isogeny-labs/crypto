//! Yield and cost of the conjugate-pair factor base in a complete toy
//! index-calculus solve on y^2 = (x-e1)(x-e2)(x-e3) over K = F_{p^4}
//! (PROTOCOL.md, frozen at 578d0db3b0). Fixed-size F_{p^4} arithmetic,
//! written fresh for this study.
//!
//!   cpcost run <out_dir>
//!   cpcost summarize <out_dir>
//!   cpcost validate <out_dir> <claimed.json>

mod sha;

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::io::Write as _;

const N: usize = 4;
type E = [u64; N];

// ------------------------------------------------------------- F_p[z] ----

fn inv_p(a: u64, p: u64) -> u64 {
    let (mut r, mut b, mut e) = (1u64, a % p, p - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}
fn trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}
fn pmod(a: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    let mut r = a.to_vec();
    trim(&mut r);
    let dm = m.len() - 1;
    let li = inv_p(*m.last().unwrap(), p);
    while r.len() > dm {
        let c = r.last().unwrap() * li % p;
        let sh = r.len() - 1 - dm;
        for i in 0..=dm {
            r[sh + i] = (r[sh + i] + p * p - c * m[i] % p) % p;
        }
        trim(&mut r);
    }
    r
}
fn pmulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % p;
        }
    }
    pmod(&r, m, p)
}
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    trim(&mut a);
    trim(&mut b);
    while !b.is_empty() {
        let r = pmod(&a, &b, p);
        a = b;
        b = r;
    }
    a
}
fn xpow_pk(m: &[u64], p: u64, k: usize) -> Vec<u64> {
    let mut r = pmod(&[0, 1], m, p);
    for _ in 0..k {
        let (mut acc, mut b, mut e) = (vec![1u64], r.clone(), p);
        while e > 0 {
            if e & 1 == 1 {
                acc = pmulmod(&acc, &b, m, p);
            }
            b = pmulmod(&b, &b, m, p);
            e >>= 1;
        }
        r = acc;
    }
    r
}

// ----------------------------------------------------------- field K ----

struct K {
    p: u64,
    m: [u64; N], // z^4 = -(m0 + m1 z + m2 z^2 + m3 z^3)
    q: u64,      // p^4
}

impl K {
    fn new(p: u64, rng: &mut Rng) -> K {
        loop {
            let mut mm: Vec<u64> = (0..N).map(|_| rng.below(p)).collect();
            mm.push(1);
            // Rabin: x^(p^4) = x and gcd(x^(p^2) - x, m) = 1
            if xpow_pk(&mm, p, 4) != pmod(&[0, 1], &mm, p) {
                continue;
            }
            let mut h = xpow_pk(&mm, p, 2);
            while h.len() < 2 {
                h.push(0);
            }
            h[1] = (h[1] + p - 1) % p;
            if pgcd(&h, &mm, p).len() != 1 {
                continue;
            }
            let mut m = [0u64; N];
            m.copy_from_slice(&mm[..N]);
            return K { p, m, q: p.pow(4) };
        }
    }
    fn c(&self, v: u64) -> E {
        [v % self.p, 0, 0, 0]
    }
    fn add(&self, a: &E, b: &E) -> E {
        let p = self.p;
        [
            (a[0] + b[0]) % p,
            (a[1] + b[1]) % p,
            (a[2] + b[2]) % p,
            (a[3] + b[3]) % p,
        ]
    }
    fn sub(&self, a: &E, b: &E) -> E {
        let p = self.p;
        [
            (a[0] + p - b[0]) % p,
            (a[1] + p - b[1]) % p,
            (a[2] + p - b[2]) % p,
            (a[3] + p - b[3]) % p,
        ]
    }
    fn neg(&self, a: &E) -> E {
        self.sub(&[0; N], a)
    }
    fn mul(&self, a: &E, b: &E) -> E {
        let p = self.p;
        let mut r = [0u64; 2 * N - 1];
        for i in 0..N {
            for j in 0..N {
                r[i + j] += a[i] * b[j];
            }
        }
        for v in r.iter_mut() {
            *v %= p;
        }
        for k in (N..2 * N - 1).rev() {
            let c = r[k];
            if c != 0 {
                for i in 0..N {
                    r[k - N + i] = (r[k - N + i] + p * p - c * self.m[i]) % p;
                }
            }
        }
        [r[0], r[1], r[2], r[3]]
    }
    fn sq(&self, a: &E) -> E {
        self.mul(a, a)
    }
    fn pow(&self, a: &E, mut e: u64) -> E {
        let (mut r, mut b) = (self.c(1), *a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.sq(&b);
            e >>= 1;
        }
        r
    }
    fn is_zero(a: &E) -> bool {
        a.iter().all(|&x| x == 0)
    }
    fn inv(&self, a: &E) -> E {
        assert!(!Self::is_zero(a));
        self.pow(a, self.q - 2)
    }
    fn frob(&self, a: &E, k: u32) -> E {
        self.pow(a, self.p.pow(k))
    }
    #[allow(dead_code)]
    fn in_k0(&self, a: &E) -> bool {
        self.frob(a, 2) == *a
    }
    fn chi(&self, a: &E) -> i32 {
        if Self::is_zero(a) {
            return 0;
        }
        let v = self.pow(a, (self.q - 1) / 2);
        if v == self.c(1) {
            1
        } else {
            assert_eq!(v, self.c(self.p - 1));
            -1
        }
    }
    fn rand(&self, rng: &mut Rng) -> E {
        [
            rng.below(self.p),
            rng.below(self.p),
            rng.below(self.p),
            rng.below(self.p),
        ]
    }
    fn rand_k0(&self, rng: &mut Rng) -> E {
        let a = self.rand(rng);
        self.add(&a, &self.frob(&a, 2))
    }
    fn sqrt(&self, a: &E, nonres: &E) -> E {
        let mut s = 0;
        let mut qo = self.q - 1;
        while qo % 2 == 0 {
            qo /= 2;
            s += 1;
        }
        let (mut mm, mut c, mut t, mut r) = (
            s,
            self.pow(nonres, qo),
            self.pow(a, qo),
            self.pow(a, (qo + 1) / 2),
        );
        let one = self.c(1);
        while t != one {
            let (mut i, mut tt) = (0, t);
            while tt != one {
                tt = self.sq(&tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(mm - i - 1) {
                b = self.sq(&b);
            }
            mm = i;
            c = self.sq(&b);
            t = self.mul(&t, &c);
            r = self.mul(&r, &b);
        }
        debug_assert_eq!(self.sq(&r), *a);
        r
    }
}

// --------------------------------------------------------------- rng ----

struct Rng(u64);
impl Rng {
    fn new(tag: &str) -> Rng {
        let mut h: u64 = 0xcbf29ce484222325;
        for c in tag.bytes() {
            h ^= c as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        Rng(h)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        let zone = u64::MAX - (u64::MAX % m);
        loop {
            let v = self.next();
            if v < zone {
                return v % m;
            }
        }
    }
}

// ------------------------------------------------------------- curve ----

struct Curve {
    a2: E,
    a4: E,
    a6: E,
}
type Pt = Option<(E, E)>;

fn fval(k: &K, c: &Curve, x: &E) -> E {
    let t = k.add(x, &c.a2);
    let t = k.add(&k.mul(&t, x), &c.a4);
    k.add(&k.mul(&t, x), &c.a6)
}
fn padd(k: &K, c: &Curve, p: &Pt, q: &Pt) -> Pt {
    let (x1, y1) = match p {
        None => return *q,
        Some(v) => v,
    };
    let (x2, y2) = match q {
        None => return *p,
        Some(v) => v,
    };
    let lam = if x1 == x2 {
        if K::is_zero(&k.add(y1, y2)) {
            return None;
        }
        let num = k.add(
            &k.add(
                &k.mul(&k.c(3), &k.sq(x1)),
                &k.mul(&k.mul(&k.c(2), &c.a2), x1),
            ),
            &c.a4,
        );
        k.mul(&num, &k.inv(&k.mul(&k.c(2), y1)))
    } else {
        k.mul(&k.sub(y2, y1), &k.inv(&k.sub(x2, x1)))
    };
    let x3 = k.sub(&k.sub(&k.sub(&k.sq(&lam), &c.a2), x1), x2);
    let y3 = k.sub(&k.mul(&lam, &k.sub(x1, &x3)), y1);
    Some((x3, y3))
}
fn pneg(k: &K, p: &Pt) -> Pt {
    p.map(|(x, y)| (x, k.neg(&y)))
}
fn pmul(k: &K, c: &Curve, p: &Pt, mut n: u64) -> Pt {
    let (mut r, mut s) = (None, *p);
    while n > 0 {
        if n & 1 == 1 {
            r = padd(k, c, &r, &s);
        }
        s = padd(k, c, &s, &s);
        n >>= 1;
    }
    r
}
fn key(p: &Pt) -> [u64; 2 * N] {
    match p {
        None => [u64::MAX; 2 * N],
        Some((x, y)) => [x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]],
    }
}
fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut v = vec![];
    let mut d = 2;
    while d * d <= n {
        let mut e = 0;
        while n % d == 0 {
            n /= d;
            e += 1;
        }
        if e > 0 {
            v.push((d, e));
        }
        d += 1;
    }
    if n > 1 {
        v.push((n, 1));
    }
    v
}
fn rand_point(k: &K, c: &Curve, nonres: &E, rng: &mut Rng) -> Pt {
    loop {
        let x = k.rand(rng);
        let fx = fval(k, c, &x);
        if k.chi(&fx) == 1 {
            return Some((x, k.sqrt(&fx, nonres)));
        }
    }
}

fn order_enum(k: &K, c: &Curve) -> u64 {
    let p = k.p;
    let mut cnt = 1u64;
    for i in 0..k.q {
        let x = [i % p, (i / p) % p, (i / (p * p)) % p, i / (p * p * p)];
        cnt += (1 + k.chi(&fval(k, c, &x))) as u64;
    }
    cnt
}

/// BSGS in the Hasse interval with exact point orders, intersected over 8 points.
fn order_bsgs(k: &K, c: &Curve, nonres: &E, rng: &mut Rng) -> Option<u64> {
    let s = (2.0 * (k.q as f64).sqrt()).ceil() as u64 + 1;
    let (lo, width) = (k.q + 1 - s, 2 * s);
    let m = ((width + 1) as f64).sqrt().ceil() as u64;
    let mut cands: Option<HashSet<u64>> = None;
    for _ in 0..8 {
        let pt = rand_point(k, c, nonres, rng);
        let mut table: HashMap<[u64; 2 * N], u64> = HashMap::new();
        let mut jp: Pt = None;
        for j in 0..m {
            table.entry(key(&jp)).or_insert(j);
            jp = padd(k, c, &jp, &pt);
        }
        let mp = pmul(k, c, &pt, m);
        let mut r = pmul(k, c, &pt, lo);
        let mut n0 = None;
        let mut i = 0;
        while i * m <= width && n0.is_none() {
            if let Some(&j) = table.get(&key(&pneg(k, &r))) {
                n0 = Some(lo + i * m + j);
            }
            r = padd(k, c, &r, &mp);
            i += 1;
        }
        let n0 = n0?;
        let mut o = n0;
        for (f, _) in factor(n0) {
            while o % f == 0 && pmul(k, c, &pt, o / f).is_none() {
                o /= f;
            }
        }
        let first = lo.div_ceil(o) * o;
        let fs: HashSet<u64> = (0..)
            .map(|t| first + t * o)
            .take_while(|&x| x <= lo + width)
            .collect();
        cands = Some(match cands {
            None => fs,
            Some(cs) => cs.intersection(&fs).copied().collect(),
        });
    }
    let cs = cands.unwrap();
    (cs.len() == 1).then(|| *cs.iter().next().unwrap())
}

// ---------------------------------------------- conjugate-pair lines ----

fn nullspace(cols: &[E], p: u64) -> Vec<Vec<u64>> {
    let m = cols.len();
    let mut a: Vec<Vec<u64>> = (0..N)
        .map(|i| (0..m).map(|j| cols[j][i]).collect())
        .collect();
    let mut piv: Vec<usize> = vec![];
    let mut r = 0;
    for c in 0..m {
        let Some(pr) = (r..N).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, pr);
        let iv = inv_p(a[r][c], p);
        for x in a[r].iter_mut() {
            *x = *x * iv % p;
        }
        for i in 0..N {
            if i != r && a[i][c] != 0 {
                let f = a[i][c];
                for j in 0..m {
                    a[i][j] = (a[i][j] + p * p - f * a[r][j]) % p;
                }
            }
        }
        piv.push(c);
        r += 1;
        if r == N {
            break;
        }
    }
    (0..m)
        .filter(|c| !piv.contains(c))
        .map(|f| {
            let mut v = vec![0u64; m];
            v[f] = 1;
            for (ri, &pc) in piv.iter().enumerate() {
                v[pc] = (p - a[ri][f]) % p;
            }
            v
        })
        .collect()
}

/// Conjugate-pair lines (x0, u), deduplicated, in a deterministic order.
fn conj_lines(k: &K, es: &[E; 3], rng: &mut Rng) -> Vec<(E, E)> {
    let p = k.p;
    let zb: Vec<E> = (0..N)
        .map(|i| {
            let mut e = [0u64; N];
            e[i] = 1;
            e
        })
        .collect();
    // basis of K0
    let mut k0b: Vec<E> = vec![];
    while k0b.len() < 2 {
        let g = k.rand_k0(rng);
        if K::is_zero(&g) {
            continue;
        }
        if k0b.len() == 1 {
            // independent over F_p?
            let a = k0b[0];
            let dep = (0..p).any(|t| k.mul(&a, &k.c(t)) == g);
            if dep {
                continue;
            }
        }
        k0b.push(g);
    }
    let mut out: Vec<(Vec<E>, (E, E))> = vec![];
    for lone in 0..3 {
        let oth: Vec<usize> = (0..3).filter(|&i| i != lone).collect();
        for &(ia, ib) in &[(oth[0], oth[1]), (oth[1], oth[0])] {
            let da = k.sub(&es[ia], &es[lone]);
            let db = k.sub(&es[ib], &es[lone]);
            for kk in [1u32, 3] {
                let sda = k.frob(&da, kk);
                let mut cols: Vec<E> = zb
                    .iter()
                    .map(|v| k.sub(&k.mul(&db, v), &k.mul(&sda, &k.frob(v, kk))))
                    .collect();
                cols.extend(k0b.iter().map(|g| k.sub(g, &k.frob(g, kk))));
                let ns = nullspace(&cols, p);
                for v in &ns {
                    let mut w = [0u64; N];
                    for (j, b) in zb.iter().enumerate() {
                        w = k.add(&w, &k.mul(b, &k.c(v[j])));
                    }
                    if K::is_zero(&w) {
                        continue;
                    }
                    let mut g = [0u64; N];
                    for (j, b) in k0b.iter().enumerate() {
                        g = k.add(&g, &k.mul(b, &k.c(v[N + j])));
                    }
                    let u = k.inv(&w);
                    let x0 = k.sub(&es[lone], &k.mul(&g, &u));
                    let mut xs: Vec<E> = (0..p).map(|t| k.add(&x0, &k.mul(&u, &k.c(t)))).collect();
                    xs.sort();
                    if !out.iter().any(|e| e.0 == xs) {
                        out.push((xs, (x0, u)));
                    }
                }
            }
        }
    }
    out.into_iter().map(|e| e.1).collect()
}

// ------------------------------------------------------- index calculus ----

struct Arm {
    k_lift: u64,
    cols: usize,
    trials: u64,
    rels: u64,
    rank: usize,
    solved: bool,
    adds: u64,
    capped: bool,
}

#[allow(clippy::too_many_arguments)]
fn solve_arm(
    k: &K,
    c: &Curve,
    nonres: &E,
    x0: &E,
    u: &E,
    ell: u64,
    h: u64,
    p0: &Pt,
    q0: &Pt,
    s_true: u64,
    targets: &[(u64, u64)],
    cap: u64,
) -> Arm {
    let p = k.p;
    // factor base
    let mut fb: Vec<Pt> = vec![];
    let mut k_lift = 0;
    for t in 0..p {
        let x = k.add(x0, &k.mul(u, &k.c(t)));
        let fx = fval(k, c, &x);
        match k.chi(&fx) {
            -1 => {}
            0 => {
                k_lift += 1;
                fb.push(Some((x, [0; N])));
            }
            _ => {
                k_lift += 1;
                let y = k.sqrt(&fx, nonres);
                fb.push(Some((x, y)));
                fb.push(Some((x, k.neg(&y))));
            }
        }
    }
    let mut adds = 0u64;
    // columns: [h] P_i up to sign
    let mut colid: HashMap<E, usize> = HashMap::new();
    let mut fbcol: Vec<Option<(usize, i64)>> = vec![];
    for pt in &fb {
        let pr = pmul(k, c, pt, h);
        fbcol.push(pr.map(|(x, y)| {
            let n = colid.len();
            let id = *colid.entry(x).or_insert(n);
            let ny = k.neg(&y);
            (id, if y <= ny { 1 } else { -1 })
        }));
    }
    let d = colid.len();
    // pair-sum table (i <= j)
    let mut table: HashMap<[u64; 2 * N], Vec<(u16, u16)>> = HashMap::new();
    for i in 0..fb.len() {
        for j in i..fb.len() {
            let s = padd(k, c, &fb[i], &fb[j]);
            adds += 1;
            table.entry(key(&s)).or_default().push((i as u16, j as u16));
        }
    }
    let entries: Vec<([u64; 2 * N], Vec<(u16, u16)>)> =
        table.iter().map(|(a, b)| (*a, b.clone())).collect();
    let unkey = |kk: &[u64; 2 * N]| -> Pt {
        if kk[0] == u64::MAX {
            None
        } else {
            Some(([kk[0], kk[1], kk[2], kk[3]], [kk[4], kk[5], kk[6], kk[7]]))
        }
    };
    // incremental elimination mod ell over unknowns (L_0..L_{d-1}, s)
    let w = d + 1;
    let mut ech: Vec<(usize, Vec<u64>, u64)> = vec![]; // (pivot, row, rhs)
    let (mut trials, mut rels) = (0u64, 0u64);
    let mut solved = false;
    let mut capped = true;
    for &(a, b) in targets.iter().take(cap as usize) {
        trials += 1;
        let r = padd(k, c, &pmul(k, c, p0, a), &pmul(k, c, q0, b));
        let mut found: HashSet<[u16; 4]> = HashSet::new();
        for (sk, pairs) in &entries {
            let need = padd(k, c, &r, &pneg(k, &unkey(sk)));
            adds += 1;
            if let Some(other) = table.get(&key(&need)) {
                for &(i, j) in pairs {
                    for &(x, y) in other {
                        let mut m4 = [i, j, x, y];
                        m4.sort();
                        found.insert(m4);
                    }
                }
            }
        }
        rels += found.len() as u64;
        let mut fl: Vec<[u16; 4]> = found.into_iter().collect();
        fl.sort();
        for m4 in fl {
            // row: sum sign*L_col - h b s = h a  (mod ell)
            let mut row = vec![0u64; w];
            for &i in &m4 {
                if let Some((col, sg)) = fbcol[i as usize] {
                    row[col] = if sg > 0 {
                        (row[col] + 1) % ell
                    } else {
                        (row[col] + ell - 1) % ell
                    };
                }
            }
            row[d] = (ell - (h % ell) * (b % ell) % ell) % ell;
            let mut rhs = (h % ell) * (a % ell) % ell;
            for (pc, er, erhs) in &ech {
                let f = row[*pc];
                if f != 0 {
                    for j in 0..w {
                        row[j] = (row[j] + ell - f * er[j] % ell) % ell;
                    }
                    rhs = (rhs + ell - f * erhs % ell) % ell;
                }
            }
            if let Some(pc) = row.iter().position(|&x| x != 0) {
                let iv = modinv(row[pc], ell);
                for x in row.iter_mut() {
                    *x = *x * iv % ell;
                }
                rhs = rhs * iv % ell;
                // keep fully reduced
                for (_, er, erhs) in ech.iter_mut() {
                    let f = er[pc];
                    if f != 0 {
                        for j in 0..w {
                            er[j] = (er[j] + ell - f * row[j] % ell) % ell;
                        }
                        *erhs = (*erhs + ell - f * rhs % ell) % ell;
                    }
                }
                ech.push((pc, row, rhs));
            }
        }
        if ech.len() == w {
            capped = false;
            let s = ech.iter().find(|e| e.0 == d).map(|e| e.2).unwrap();
            solved = s == s_true % ell && pmul(k, c, p0, s) == *q0;
            break;
        }
    }
    Arm {
        k_lift,
        cols: d,
        trials,
        rels,
        rank: ech.len(),
        solved,
        adds,
        capped,
    }
}

fn modinv(a: u64, m: u64) -> u64 {
    let (mut r, mut b, mut e) = (1u64, a % m, m - 2);
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    n >= 2 && factor(n).len() == 1 && factor(n)[0].1 == 1
}

// --------------------------------------------------------------- run ----

fn run_curve(k: &K, p: u64, idx: usize, cap: u64) -> Vec<String> {
    let cell = format!("\"p\":{p},\"n\":4");
    let mut rng = Rng::new(&format!("cpcost-20261106/{p}/{idx}"));
    let nonres = loop {
        let z = k.rand(&mut rng);
        if k.chi(&z) == -1 {
            break z;
        }
    };
    let es: [E; 3] = loop {
        let e = [k.rand(&mut rng), k.rand(&mut rng), k.rand(&mut rng)];
        if e[0] != e[1] && e[0] != e[2] && e[1] != e[2] {
            break e;
        }
    };
    let s1 = k.add(&k.add(&es[0], &es[1]), &es[2]);
    let s2 = k.add(
        &k.add(&k.mul(&es[0], &es[1]), &k.mul(&es[0], &es[2])),
        &k.mul(&es[1], &es[2]),
    );
    let s3 = k.mul(&k.mul(&es[0], &es[1]), &es[2]);
    let c = Curve {
        a2: k.neg(&s1),
        a4: s2,
        a6: k.neg(&s3),
    };
    let order = if p == 13 {
        Some(order_enum(k, &c))
    } else {
        order_bsgs(k, &c, &nonres, &mut rng)
    };
    let lines = conj_lines(k, &es, &mut rng);
    let mut head = format!(
        "{{\"kind\":\"curve\",{cell},\"curve\":{idx},\"conj_lines\":{}",
        lines.len()
    );
    let Some(order) = order else {
        head.push_str(",\"kept\":false,\"reason\":\"order\"}");
        return vec![head];
    };
    let mut ok = true;
    for _ in 0..8 {
        if pmul(k, &c, &rand_point(k, &c, &nonres, &mut rng), order).is_some() {
            ok = false;
        }
    }
    let f = factor(order);
    let (ell, v) = *f.last().unwrap();
    write!(
        head,
        ",\"order\":{order},\"order_ok\":{ok},\"ell\":{ell},\"v\":{v}"
    )
    .unwrap();
    if !ok || v != 1 || !is_prime(ell) {
        head.push_str(",\"kept\":false,\"reason\":\"order_or_v\"}");
        return vec![head];
    }
    // first all-lift conjugate-pair line
    let conj = lines
        .iter()
        .find(|(x0, u)| (0..p).all(|t| k.chi(&fval(k, &c, &k.add(x0, &k.mul(u, &k.c(t))))) != -1));
    let all_lift_lines = lines
        .iter()
        .filter(|(x0, u)| (0..p).all(|t| k.chi(&fval(k, &c, &k.add(x0, &k.mul(u, &k.c(t))))) != -1))
        .count();
    write!(head, ",\"all_lift_conj_lines\":{all_lift_lines}").unwrap();
    let Some(&(cx0, cu)) = conj else {
        head.push_str(",\"kept\":false,\"reason\":\"no_all_lift_conj\"}");
        return vec![head];
    };
    head.push_str(",\"kept\":true,\"reason\":\"\"}");
    // target
    let h = order / ell;
    let p0 = loop {
        let pr = pmul(k, &c, &rand_point(k, &c, &nonres, &mut rng), h);
        if pr.is_some() {
            break pr;
        }
    };
    let s_true = 1 + rng.below(ell - 1);
    let q0 = pmul(k, &c, &p0, s_true);
    let targets: Vec<(u64, u64)> = (0..cap)
        .map(|_| loop {
            let (a, b) = (rng.below(ell), rng.below(ell));
            if a != 0 || b != 0 {
                break (a, b);
            }
        })
        .collect();
    let rx0 = k.rand(&mut rng);
    let ru = loop {
        let u = k.rand(&mut rng);
        if !K::is_zero(&u) {
            break u;
        }
    };
    let mut out = vec![head];
    for (arm, x0, u) in [
        ("conj", cx0, cu),
        ("std", [0; N], k.c(1)),
        ("rand", rx0, ru),
    ] {
        let a = solve_arm(
            k, &c, &nonres, &x0, &u, ell, h, &p0, &q0, s_true, &targets, cap,
        );
        out.push(format!(
            "{{\"kind\":\"arm\",{cell},\"curve\":{idx},\"arm\":\"{arm}\",\"k\":{},\"cols\":{},\"trials\":{},\"rels\":{},\"rank\":{},\"solved\":{},\"capped\":{},\"adds\":{},\"ell\":{ell}}}",
            a.k_lift, a.cols, a.trials, a.rels, a.rank, a.solved, a.capped, a.adds
        ));
    }
    out
}

const CELLS: &[u64] = &[13, 101];
const KEEP: usize = 16;
const CAP: u64 = 20_000;

fn run(dir: &str) {
    std::fs::create_dir_all(dir).unwrap();
    let mut fh = std::fs::File::create(format!("{dir}/rows.jsonl")).unwrap();
    for &p in CELLS {
        let t0 = std::time::Instant::now();
        let mut rng = Rng::new(&format!("cpcost-20261106/field/{p}"));
        let k = K::new(p, &mut rng);
        writeln!(fh, "{{\"kind\":\"field\",\"p\":{p},\"modulus\":{:?}}}", k.m).unwrap();
        // draw curves in batches of 4 (one thread each) until KEEP are kept
        let mut kept = 0;
        let mut idx = 0;
        while kept < KEEP {
            let batch: Vec<usize> = (idx..idx + 4).collect();
            idx += 4;
            let mut res: Vec<Vec<String>> = vec![vec![]; batch.len()];
            std::thread::scope(|s| {
                let mut hs = vec![];
                for (slot, &ci) in res.iter_mut().zip(batch.iter()) {
                    let k = &k;
                    hs.push(s.spawn(move || *slot = run_curve(k, p, ci, CAP)));
                }
                for h in hs {
                    h.join().unwrap();
                }
            });
            for r in res {
                let is_kept = r[0].contains("\"kept\":true");
                if is_kept && kept >= KEEP {
                    continue; // batch overshoot: drop, deterministic
                }
                kept += is_kept as usize;
                for line in r {
                    writeln!(fh, "{line}").unwrap();
                }
            }
            fh.flush().unwrap();
        }
        eprintln!(
            "cell p={p}: {kept} kept of {idx} drawn, {:.1} s",
            t0.elapsed().as_secs_f64()
        );
    }
}

// ---------------------------------------------------------- summarize ----

fn jget<'a>(l: &'a str, key: &str) -> Option<&'a str> {
    let pat = format!("\"{key}\":");
    let i = l.find(&pat)? + pat.len();
    let rest = &l[i..];
    if let Some(s) = rest.strip_prefix('"') {
        return s.find('"').map(|j| &s[..j]);
    }
    let end = rest.find([',', '}']).unwrap_or(rest.len());
    Some(rest[..end].trim())
}
fn jnum(l: &str, k: &str) -> u64 {
    jget(l, k)
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("{k} in {l}"))
}
fn jbool(l: &str, k: &str) -> bool {
    jget(l, k) == Some("true")
}

fn median(mut v: Vec<f64>) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn compute(dir: &str) -> (String, String, Vec<(String, f64)>) {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let rows: Vec<&str> = text.lines().collect();
    let curves: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("curve"))
        .collect();
    let arms: Vec<&str> = rows
        .iter()
        .copied()
        .filter(|l| jget(l, "kind") == Some("arm"))
        .collect();
    let mut vals: Vec<(String, f64)> = vec![];
    let mut md = String::new();
    let v = |b: bool| if b { "PASS" } else { "FAIL" };
    let (mut y1, mut y2, mut y3, mut y4) = (true, true, true, true);
    let mut tab = String::new();
    writeln!(tab, "| p | arm | curves | mean k | mean D | relations / trial Y | ρ = Y/D | solved | median trials to rank | mean relations used | brute-force adds / trial |\n|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    let mut ratios = String::new();
    writeln!(ratios, "| p | Y(conj)/Y(std) | (k_conj/k_std)^4 | ratio / prediction | ρ(conj)/ρ(std) | Y(rand)/Y(std) | median T(std)/T(conj) | median T(rand)/T(conj) | mean D(conj)/D(std) |\n|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for &p in CELLS {
        let drawn = curves.iter().filter(|l| jnum(l, "p") == p).count();
        let kept = curves
            .iter()
            .filter(|l| jnum(l, "p") == p && jbool(l, "kept"))
            .count();
        let no_conj = curves
            .iter()
            .filter(|l| jnum(l, "p") == p && jget(l, "reason") == Some("no_all_lift_conj"))
            .count();
        let bad_ord = curves
            .iter()
            .filter(|l| {
                jnum(l, "p") == p
                    && (jget(l, "reason") == Some("order")
                        || jget(l, "reason") == Some("order_or_v"))
            })
            .count();
        vals.push((format!("drawn_{p}"), drawn as f64));
        vals.push((format!("kept_{p}"), kept as f64));
        vals.push((format!("no_all_lift_conj_{p}"), no_conj as f64));
        vals.push((format!("order_or_v_skipped_{p}"), bad_ord as f64));
        let mut ystat: HashMap<&str, (f64, f64, f64)> = HashMap::new(); // Y, rho, k
        for arm in ["conj", "std", "rand"] {
            let g: Vec<&&str> = arms
                .iter()
                .filter(|l| jnum(l, "p") == p && jget(l, "arm") == Some(arm))
                .collect();
            let tr: u64 = g.iter().map(|l| jnum(l, "trials")).sum();
            let rl: u64 = g.iter().map(|l| jnum(l, "rels")).sum();
            let y = rl as f64 / tr as f64;
            let dmean = g.iter().map(|l| jnum(l, "cols") as f64).sum::<f64>() / g.len() as f64;
            let kmean = g.iter().map(|l| jnum(l, "k") as f64).sum::<f64>() / g.len() as f64;
            let rho = y / dmean;
            let solved = g.iter().filter(|l| jbool(l, "solved")).count();
            let mt = median(
                g.iter()
                    .filter(|l| !jbool(l, "capped"))
                    .map(|l| jnum(l, "trials") as f64)
                    .collect(),
            );
            let mr = g.iter().map(|l| jnum(l, "rels") as f64).sum::<f64>() / g.len() as f64;
            let addpt = g.iter().map(|l| jnum(l, "adds") as f64).sum::<f64>() / tr as f64;
            ystat.insert(arm, (y, rho, kmean));
            vals.push((format!("Y_{arm}_{p}_x1e6"), (y * 1e6).round()));
            vals.push((format!("solved_{arm}_{p}"), solved as f64));
            vals.push((format!("trials_{arm}_{p}"), tr as f64));
            vals.push((format!("rels_{arm}_{p}"), rl as f64));
            if arm == "conj" && solved != g.len() {
                y3 = false;
            }
            writeln!(tab, "| {p} | {arm} | {} | {kmean:.1} | {dmean:.1} | {y:.4} | {rho:.5} | {solved}/{} | {mt:.0} | {mr:.0} | {addpt:.0} |", g.len(), g.len()).unwrap();
        }
        let (yc, rc, kc) = ystat["conj"];
        let (ys, rs, ks) = ystat["std"];
        let (yr, _, _) = ystat["rand"];
        let pred = (kc / ks).powi(4);
        let r1 = yc / ys;
        if !(r1 >= 0.6 * pred && r1 <= 1.6 * pred) || !(yr / ys >= 0.5 && yr / ys <= 2.0) {
            y1 = false;
        }
        let r2 = rc / rs;
        if !(4.8..=12.8).contains(&r2) {
            y2 = false;
        }
        // trial ratios per curve
        let tr_ratio = |other: &str| -> f64 {
            let mut v = vec![];
            for ci in curves
                .iter()
                .filter(|l| jnum(l, "p") == p && jbool(l, "kept"))
                .map(|l| jnum(l, "curve"))
            {
                let get = |arm: &str| {
                    arms.iter()
                        .find(|l| {
                            jnum(l, "p") == p
                                && jnum(l, "curve") == ci
                                && jget(l, "arm") == Some(arm)
                        })
                        .copied()
                };
                if let (Some(a), Some(b)) = (get("conj"), get(other)) {
                    if jbool(a, "solved") && jbool(b, "solved") {
                        v.push(jnum(b, "trials") as f64 / jnum(a, "trials") as f64);
                    }
                }
            }
            median(v)
        };
        let ts = tr_ratio("std");
        let trr = tr_ratio("rand");
        if !(4.0..=16.0).contains(&ts) {
            y4 = false;
        }
        let dc = arms
            .iter()
            .filter(|l| jnum(l, "p") == p && jget(l, "arm") == Some("conj"))
            .map(|l| jnum(l, "cols") as f64)
            .sum::<f64>();
        let ds = arms
            .iter()
            .filter(|l| jnum(l, "p") == p && jget(l, "arm") == Some("std"))
            .map(|l| jnum(l, "cols") as f64)
            .sum::<f64>();
        vals.push((format!("Yratio_conj_std_{p}_x1000"), (r1 * 1000.0).round()));
        vals.push((
            format!("rhoratio_conj_std_{p}_x1000"),
            (r2 * 1000.0).round(),
        ));
        vals.push((format!("Tratio_std_conj_{p}_x1000"), (ts * 1000.0).round()));
        writeln!(
            ratios,
            "| {p} | {r1:.2} | {pred:.2} | {:.2} | {r2:.2} | {:.2} | {ts:.2} | {trr:.2} | {:.2} |",
            r1 / pred,
            yr / ys,
            dc / ds
        )
        .unwrap();
    }
    writeln!(md, "# Summary (coded rules, PROTOCOL.md)\n").unwrap();
    writeln!(md, "- **Y1** (Y(conj)/Y(std) within [0.6, 1.6] × (k_conj/k_std)^4; Y(rand)/Y(std) in [0.5, 2]): **{}**", v(y1)).unwrap();
    writeln!(
        md,
        "- **Y2** (ρ(conj)/ρ(std) in [4.8, 12.8]): **{}**",
        v(y2)
    )
    .unwrap();
    writeln!(
        md,
        "- **Y3** (every kept curve solved by conj, s verified): **{}**",
        v(y3)
    )
    .unwrap();
    writeln!(
        md,
        "- **Y4** (median T(std)/T(conj) in [4, 16]): **{}**\n",
        v(y4)
    )
    .unwrap();
    writeln!(md, "## Ratios\n\n{ratios}\n## Per arm\n\n{tab}").unwrap();
    let mut cur = String::new();
    for &p in CELLS {
        let get = |k: &str| vals.iter().find(|x| x.0 == format!("{k}_{p}")).unwrap().1;
        writeln!(cur, "- p = {p}: {} drawn, {} kept; skipped for no all-lift conjugate-pair line: {}; skipped for order/v: {}", get("drawn"), get("kept"), get("no_all_lift_conj"), get("order_or_v_skipped")).unwrap();
    }
    writeln!(md, "## Curves (Y5)\n\n{cur}").unwrap();
    let js = format!(
        "{{\"Y1\":{y1},\"Y2\":{y2},\"Y3\":{y3},\"Y4\":{y4},\"values\":{{{}}}}}\n",
        vals.iter()
            .map(|(k, v)| format!("\"{k}\":{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
    (md, js, vals)
}

fn summarize(dir: &str) {
    let (md, js, _) = compute(dir);
    std::fs::write(format!("{dir}/SUMMARY.md"), &md).unwrap();
    std::fs::write(format!("{dir}/summary.json"), js).unwrap();
    std::fs::write(format!("{dir}/fig1_cost.svg"), svg(dir)).unwrap();
    print!("{md}");
}

fn svg(dir: &str) -> String {
    let text = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let arms: Vec<&str> = text
        .lines()
        .filter(|l| jget(l, "kind") == Some("arm"))
        .collect();
    let (w, h) = (820.0, 330.0);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" font-family=\"sans-serif\" font-size=\"11\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"20\" y=\"22\" font-size=\"13\" font-weight=\"bold\">Trials to a full-rank system, per curve (log scale)</text>\n");
    let (x0, top, ph) = (70.0, 40.0, 230.0);
    let ymax = 5.0f64; // log10
    let y = |t: f64| top + ph - (t.max(1.0).log10() / ymax) * ph;
    for e in 0..=5 {
        let t = 10f64.powi(e);
        writeln!(s, "<line x1=\"{x0}\" y1=\"{}\" x2=\"800\" y2=\"{}\" stroke=\"#e5e5e5\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\">{t}</text>", y(t), y(t), x0 - 4.0, y(t) + 4.0).unwrap();
    }
    let groups: Vec<(u64, &str, &str)> = vec![
        (13, "conj", "#2e86c1"),
        (13, "std", "#95a5a6"),
        (13, "rand", "#e67e22"),
        (101, "conj", "#2e86c1"),
        (101, "std", "#95a5a6"),
        (101, "rand", "#e67e22"),
    ];
    let gw = 720.0 / groups.len() as f64;
    for (gi, (p, arm, col)) in groups.iter().enumerate() {
        let cx = x0 + gw * gi as f64 + gw / 2.0;
        let pts: Vec<(f64, bool)> = arms
            .iter()
            .filter(|l| jnum(l, "p") == *p && jget(l, "arm") == Some(arm))
            .map(|l| (jnum(l, "trials") as f64, jbool(l, "solved")))
            .collect();
        for (i, (t, ok)) in pts.iter().enumerate() {
            let jx = cx + ((i as f64 * 7.3) % 40.0) - 20.0;
            if *ok {
                writeln!(s, "<circle cx=\"{jx:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{col}\" fill-opacity=\"0.8\"/>", y(*t)).unwrap();
            } else {
                writeln!(s, "<circle cx=\"{jx:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"none\" stroke=\"{col}\"/>", y(*t)).unwrap();
            }
        }
        let med = median(pts.iter().map(|x| x.0).collect());
        writeln!(s, "<line x1=\"{}\" y1=\"{:.1}\" x2=\"{}\" y2=\"{:.1}\" stroke=\"black\" stroke-width=\"2\"/>", cx - 28.0, y(med), cx + 28.0, y(med)).unwrap();
        writeln!(s, "<text x=\"{cx}\" y=\"{}\" text-anchor=\"middle\">{arm}</text><text x=\"{cx}\" y=\"{}\" text-anchor=\"middle\" font-size=\"10\">q = {p}</text>", top + ph + 16.0, top + ph + 29.0).unwrap();
    }
    writeln!(s, "<text x=\"70\" y=\"322\" font-size=\"10\">filled = solved (s verified); open = cap reached; bar = median. conj = all-lift conjugate-pair line, std = F_q, rand = random line.</text>").unwrap();
    s.push_str("</svg>\n");
    s
}

fn validate(dir: &str, claimed: &str) {
    let (_, _, vals) = compute(dir);
    let text = std::fs::read_to_string(claimed).unwrap();
    let mut heads = vec![];
    let mut ok = true;
    for line in text.lines() {
        let t = line.trim().trim_end_matches(',');
        let Some((k, v)) = t.split_once(':') else {
            continue;
        };
        let k = k.trim().trim_matches('"');
        let Ok(v) = v.trim().parse::<f64>() else {
            continue;
        };
        let got = vals
            .iter()
            .find(|x| x.0 == k)
            .unwrap_or_else(|| panic!("no value {k}"))
            .1;
        if got != v {
            ok = false;
        }
        heads.push(format!(
            "    {{\"name\": \"{k}\", \"claimed\": {v}, \"recomputed\": {got}, \"rel_tol\": 0}}"
        ));
    }
    let data = std::fs::read(format!("{dir}/rows.jsonl")).unwrap();
    let rows_sha = sha::sha256_hex(&data);
    let raw = match std::fs::read(format!("{dir}/.artifacts.json")) {
        Ok(m) => {
            if !String::from_utf8_lossy(&m).contains(&rows_sha) {
                eprintln!("rows.jsonl is not the manifest's pin");
                ok = false;
            }
            format!("{{\"path\": \".artifacts.json\", \"sha256\": \"{}\", \"pins\": {{\"rows.jsonl\": \"{rows_sha}\"}}}}", sha::sha256_hex(&m))
        }
        Err(_) => format!("{{\"path\": \"rows.jsonl\", \"sha256\": \"{rows_sha}\"}}"),
    };
    let rep = format!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/lift_dense_conjugate_pair_cost_20261106\",\n  \"recomputed_by\": {{\"tool\": \"research/lift_dense_conjugate_pair_cost_20261106/cpcost (cpcost validate)\", \"command\": \"cpcost validate <run> claimed.json\"}},\n  \"note\": \"Producer-run recompute of README headline numbers from the pinned raw rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n    {raw}\n  ],\n  \"headline\": [\n{}\n  ]\n}}\n",
        if ok { "pass" } else { "fail" },
        heads.join(",\n")
    );
    std::fs::write(format!("{dir}/validator-report.json"), rep).unwrap();
    eprintln!(
        "validator-report.json: {}",
        if ok { "pass" } else { "FAIL" }
    );
    if !ok {
        std::process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("run") => run(&args[2]),
        Some("summarize") => summarize(&args[2]),
        Some("validate") => validate(&args[2], &args[3]),
        _ => eprintln!("usage: cpcost run|summarize|validate <dir> [claimed.json]"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup(p: u64, tag: &str) -> (K, Rng) {
        let mut rng = Rng::new(tag);
        let k = K::new(p, &mut rng);
        (k, rng)
    }
    #[test]
    fn field_basics() {
        let (k, mut rng) = setup(13, "f");
        let a = k.rand(&mut rng);
        assert_eq!(k.frob(&a, 4), a);
        if !K::is_zero(&a) {
            assert_eq!(k.mul(&a, &k.inv(&a)), k.c(1));
        }
        let g = k.rand_k0(&mut rng);
        assert!(k.in_k0(&g));
    }
    #[test]
    fn six_conjugate_pair_lines() {
        let (k, mut rng) = setup(101, "six");
        let es = [k.rand(&mut rng), k.rand(&mut rng), k.rand(&mut rng)];
        assert_eq!(conj_lines(&k, &es, &mut rng).len(), 6);
    }
    #[test]
    fn bsgs_equals_enumeration() {
        let (k, mut rng) = setup(13, "ord");
        let nonres = loop {
            let z = k.rand(&mut rng);
            if k.chi(&z) == -1 {
                break z;
            }
        };
        let es = [k.rand(&mut rng), k.rand(&mut rng), k.rand(&mut rng)];
        let s1 = k.add(&k.add(&es[0], &es[1]), &es[2]);
        let s2 = k.add(
            &k.add(&k.mul(&es[0], &es[1]), &k.mul(&es[0], &es[2])),
            &k.mul(&es[1], &es[2]),
        );
        let s3 = k.mul(&k.mul(&es[0], &es[1]), &es[2]);
        let c = Curve {
            a2: k.neg(&s1),
            a4: s2,
            a6: k.neg(&s3),
        };
        let n = order_enum(&k, &c);
        let b = order_bsgs(&k, &c, &nonres, &mut rng);
        assert!(b.is_none() || b == Some(n));
    }
    #[test]
    fn sha() {
        assert_eq!(
            sha::sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
