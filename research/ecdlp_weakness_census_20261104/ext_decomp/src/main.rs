use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::time::Instant;

type E = [u64; 4];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

#[derive(Clone)]
struct Fld {
    q: u64,
    n: usize,
    f: E, // t^n = -sum f[j] t^j
    p: u64,
}

impl Fld {
    fn zero(&self) -> E { [0; 4] }
    fn one(&self) -> E { [1, 0, 0, 0] }
    fn is_zero(&self, a: &E) -> bool { a.iter().all(|&c| c == 0) }
    fn add(&self, a: &E, b: &E) -> E {
        let mut r = [0; 4];
        for i in 0..self.n { r[i] = (a[i] + b[i]) % self.q; }
        r
    }
    fn sub(&self, a: &E, b: &E) -> E {
        let mut r = [0; 4];
        for i in 0..self.n { r[i] = (a[i] + self.q - b[i]) % self.q; }
        r
    }
    fn neg(&self, a: &E) -> E { self.sub(&self.zero(), a) }
    fn mul(&self, a: &E, b: &E) -> E {
        let n = self.n;
        let q = self.q;
        let mut pr = [0u64; 8];
        for i in 0..n { for j in 0..n { pr[i + j] = (pr[i + j] + a[i] * b[j]) % q; } }
        if n >= 1 {
            let mut k = 2 * n - 2;
            while k >= n {
                let c = pr[k];
                if c != 0 {
                    for j in 0..n {
                        pr[k - n + j] = (pr[k - n + j] + (q - self.f[j]) % q * c) % q;
                    }
                }
                pr[k] = 0;
                if k == 0 { break; }
                k -= 1;
            }
        }
        let mut r = [0; 4];
        r[..n].copy_from_slice(&pr[..n]);
        r
    }
    fn pow(&self, a: &E, mut e: u64) -> E {
        let mut r = self.one();
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 { r = self.mul(&r, &b); }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: &E) -> E { assert!(!self.is_zero(a)); self.pow(a, self.p - 2) }
    fn enc(&self, a: &E) -> u64 {
        let mut r = 0;
        for i in (0..self.n).rev() { r = r * self.q + a[i]; }
        r
    }
    fn dec(&self, mut v: u64) -> E {
        let mut r = [0; 4];
        for i in 0..self.n { r[i] = v % self.q; v /= self.q; }
        r
    }
    fn scalar(&self, c: u64) -> E { let mut r = [0; 4]; r[0] = c % self.q; r }
    fn is_square(&self, a: &E) -> bool {
        self.is_zero(a) || self.pow(a, (self.p - 1) / 2) == self.one()
    }
    // Tonelli-Shanks over F_{q^n}
    fn sqrt(&self, a: &E) -> Option<E> {
        if self.is_zero(a) { return Some(self.zero()); }
        if !self.is_square(a) { return None; }
        let mut qq = self.p - 1;
        let mut s = 0;
        while qq % 2 == 0 { qq /= 2; s += 1; }
        let mut z = self.dec(2);
        let mut v = 2;
        while self.pow(&z, (self.p - 1) / 2) == self.one() { v += 1; z = self.dec(v); }
        let mut m = s;
        let mut c = self.pow(&z, qq);
        let mut t = self.pow(a, qq);
        let mut r = self.pow(a, (qq + 1) / 2);
        while t != self.one() {
            let mut i = 0;
            let mut tt = t;
            while tt != self.one() { tt = self.mul(&tt, &tt); i += 1; }
            let mut b = c;
            for _ in 0..(m - i - 1) { b = self.mul(&b, &b); }
            m = i;
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            r = self.mul(&r, &b);
        }
        Some(r)
    }
}

// polynomials over F_q as Vec<u64>
fn ptrim(a: &mut Vec<u64>) { while a.last() == Some(&0) { a.pop(); } }
fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1;
    b %= m;
    while e > 0 { if e & 1 == 1 { r = r * b % m; } b = b * b % m; e >>= 1; }
    r
}
fn pgcd(q: u64, mut a: Vec<u64>, mut b: Vec<u64>) -> Vec<u64> {
    ptrim(&mut a); ptrim(&mut b);
    while !b.is_empty() {
        // a mod b
        let inv = powmod(*b.last().unwrap(), q - 2, q);
        while a.len() >= b.len() {
            let c = a.last().unwrap() * inv % q;
            let sh = a.len() - b.len();
            for i in 0..b.len() { a[sh + i] = (a[sh + i] + q - c * b[i] % q) % q; }
            ptrim(&mut a);
            if a.is_empty() { break; }
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn find_modulus(q: u64, n: usize, rng: &mut Rng) -> Fld {
    loop {
        let mut f = [0u64; 4];
        for j in 0..n { f[j] = rng.next() % q; }
        if f[0] == 0 { continue; }
        let p = q.pow(n as u32);
        let fl = Fld { q, n, f, p };
        let mut t = [0u64; 4];
        t[1 % n.max(2)] = 1; // n>=2
        // t^{q^k} by repeated q-th powers
        let mut cur = t;
        let mut pw: Vec<E> = vec![t]; // pw[k] = t^{q^k}
        for _ in 0..n { cur = fl.pow(&cur, q); pw.push(cur); }
        if pw[n] != t { continue; }
        let mut ok = true;
        for pr in [2usize, 3] {
            if n % pr == 0 {
                let k = n / pr;
                let g = fl.sub(&pw[k], &t);
                let mut fv: Vec<u64> = (0..n).map(|j| f[j]).collect();
                fv.push(1);
                let gv: Vec<u64> = g[..n].to_vec();
                let d = pgcd(q, fv, gv);
                if d.len() != 1 { ok = false; }
            }
        }
        if ok { return fl; }
    }
}

fn selftest(fl: &Fld, rng: &mut Rng) {
    for _ in 0..50 {
        let x = fl.dec(rng.next() % fl.p);
        let y = fl.dec(rng.next() % fl.p);
        let z = fl.dec(rng.next() % fl.p);
        assert_eq!(fl.pow(&x, fl.p), x, "x^{{q^n}} != x");
        assert_eq!(fl.mul(&fl.mul(&x, &y), &z), fl.mul(&x, &fl.mul(&y, &z)));
        if !fl.is_zero(&x) { assert_eq!(fl.mul(&x, &fl.inv(&x)), fl.one()); }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Pt { Inf, A(E, E) }

struct Curve<'a> { fl: &'a Fld, a: E, b: E }
impl<'a> Curve<'a> {
    fn rhs(&self, x: &E) -> E {
        let fl = self.fl;
        let x2 = fl.mul(x, x);
        let x3 = fl.mul(&x2, x);
        fl.add(&fl.add(&x3, &fl.mul(&self.a, x)), &self.b)
    }
    fn add(&self, p: &Pt, q: &Pt) -> Pt {
        let fl = self.fl;
        match (p, q) {
            (Pt::Inf, _) => *q,
            (_, Pt::Inf) => *p,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                let lam;
                if x1 == x2 {
                    if y1 != y2 || fl.is_zero(y1) { return Pt::Inf; }
                    let x12 = fl.mul(x1, x1);
                    let num = fl.add(&fl.add(&fl.add(&x12, &x12), &x12), &self.a);
                    let den = fl.add(y1, y1);
                    lam = fl.mul(&num, &fl.inv(&den));
                } else {
                    lam = fl.mul(&fl.sub(y2, y1), &fl.inv(&fl.sub(x2, x1)));
                }
                let x3 = fl.sub(&fl.sub(&fl.mul(&lam, &lam), x1), x2);
                let y3 = fl.sub(&fl.mul(&lam, &fl.sub(x1, &x3)), y1);
                Pt::A(x3, y3)
            }
        }
    }
    fn key(&self, p: &Pt) -> u64 {
        match p { Pt::Inf => u64::MAX, Pt::A(x, y) => self.fl.enc(x) << 20 | self.fl.enc(y) }
    }
    fn count(&self) -> u64 {
        let fl = self.fl;
        let mut c = 1;
        for v in 0..fl.p {
            let r = self.rhs(&fl.dec(v));
            if fl.is_zero(&r) { c += 1; } else if fl.is_square(&r) { c += 2; }
        }
        c
    }
}

fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut v = vec![];
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 { let mut e = 0; while n % d == 0 { n /= d; e += 1; } v.push((d, e)); }
        d += 1;
    }
    if n > 1 { v.push((n, 1)); }
    v
}

fn rec(c: &Curve, fb: &[Pt], depth: usize, n: usize, start: usize, part: Pt,
       map: &mut HashMap<u64, u32>, cnt: &mut u64, ordered: bool) {
    if depth == n {
        *map.entry(c.key(&part)).or_insert(0) += 1;
        *cnt += 1;
        return;
    }
    for i in (if ordered { 0 } else { start })..fb.len() {
        let np = c.add(&part, &fb[i]);
        rec(c, fb, depth + 1, n, i, np, map, cnt, ordered);
    }
}

fn hist(map: &HashMap<u64, u32>, nE: u64) -> BTreeMap<u32, u64> {
    let mut h = BTreeMap::new();
    for &v in map.values() { *h.entry(v).or_insert(0u64) += 1; }
    h.insert(0, nE - map.len() as u64);
    h.retain(|_, v| *v > 0);
    h
}

fn main() {
    let t0 = Instant::now();
    let seed: u64 = 20261104;
    println!("seed {}", seed);
    let cases = [(101u64, 2usize), (211, 2), (401, 2), (23, 3), (41, 3), (61, 3), (11, 4), (13, 4)];
    let mut rng = Rng(seed);
    let mut json = String::from("{\n \"seed\": 20261104,\n \"rows\": [\n");
    let mut csv = String::from("curve_over_subfield,q,n,a_enc,b_enc,modulus,nE,nE_factorisation,fb_size,fb_over_q,multisets,decomposable_fraction,pred_1_over_nfact,pred_1_minus_exp,ratio_to_1_over_nfact,mean_M,ordered_tuples,ordered_decomposable_fraction,ordered_mean_N,ordered_support_equals_multiset_support,hist_M\n");
    let hdr = "| q | n | curve (a_enc,b_enc) | #E | factorisation | \\|F\\| | \\|F\\|/q | decomposable | 1/n! | 1-exp(-1/n!) | ratio |\n|---|---|---|---|---|---|---|---|---|---|---|\n";
    let mut table = String::from(hdr);
    let mut table_c = String::from(hdr);
    let mut first = true;
    for &(q, n) in &cases {
        let fl = find_modulus(q, n, &mut rng);
        selftest(&fl, &mut rng);
        let nfact: f64 = (1..=n).map(|k| k as f64).product();
        let pred = 1.0 / nfact;
        let pred2 = 1.0 - (-pred).exp();
        // candidate curves
        let mut cands: Vec<(f64, u64, u64, u64)> = vec![];
        let mut tried = vec![];
        while tried.len() < 8 {
            let a = rng.next() % q;
            let b = rng.next() % q;
            if (4 * a * a % q * a + 27 * b % q * b) % q == 0 { continue; }
            if tried.contains(&(a, b)) { continue; }
            tried.push((a, b));
            let c = Curve { fl: &fl, a: fl.scalar(a), b: fl.scalar(b) };
            let ne = c.count();
            let lpf = factor(ne).last().unwrap().0;
            cands.push((lpf as f64 / ne as f64, a, b, ne));
        }
        cands.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap());
        let mut todo: Vec<(u64, u64, u64, bool)> = cands.iter().take(2).map(|x| (x.1, x.2, x.3, true)).collect();
        let mut ngen = 0;
        while ngen < 2 {
            let ae = fl.dec(rng.next() % fl.p);
            let be = fl.dec(rng.next() % fl.p);
            if ae[1..].iter().all(|&v| v == 0) && be[1..].iter().all(|&v| v == 0) { continue; }
            let a3 = fl.mul(&fl.mul(&ae, &ae), &ae);
            let sing = fl.add(&fl.mul(&fl.scalar(4), &a3), &fl.mul(&fl.scalar(27), &fl.mul(&be, &be)));
            if fl.is_zero(&sing) { continue; }
            let c = Curve { fl: &fl, a: ae, b: be };
            todo.push((fl.enc(&ae), fl.enc(&be), c.count(), false));
            ngen += 1;
        }
        for &(a, b, ne, sub) in todo.iter() {
            let c = Curve { fl: &fl, a: fl.dec(a), b: fl.dec(b) };
            let fac = factor(ne);
            let facs: Vec<String> = fac.iter().map(|&(p, e)| if e > 1 { format!("{}^{}", p, e) } else { p.to_string() }).collect();
            let facs = facs.join("*");
            // factor base
            let mut fb: Vec<Pt> = vec![];
            for x in 0..q {
                let xe = fl.scalar(x);
                let r = c.rhs(&xe);
                let y = fl.sqrt(&r);
                if let Some(y) = y {
                    assert_eq!(fl.mul(&y, &y), r, "sqrt self-test");
                    fb.push(Pt::A(xe, y));
                    if !fl.is_zero(&y) { fb.push(Pt::A(xe, fl.neg(&y))); }
                }
            }
            let m = fb.len();
            let mut map = HashMap::new();
            let mut cnt = 0u64;
            rec(&c, &fb, 0, n, 0, Pt::Inf, &mut map, &mut cnt, false);
            // expected multiset count C(m+n-1,n)
            let mut binom = 1u128;
            for k in 0..n { binom = binom * (m + n - 1 - k) as u128 / (k as u128 + 1); }
            assert_eq!(cnt as u128, binom);
            let frac = map.len() as f64 / ne as f64;
            let mean = cnt as f64 / ne as f64;
            let h = hist(&map, ne);
            let hs: Vec<String> = h.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
            let hs = hs.join(";");
            let (mut ocnt, mut ofrac, mut omean, mut oeq) = (0u64, f64::NAN, f64::NAN, None);
            if n <= 3 {
                let mut om = HashMap::new();
                rec(&c, &fb, 0, n, 0, Pt::Inf, &mut om, &mut ocnt, true);
                assert_eq!(ocnt, (m as u64).pow(n as u32));
                ofrac = om.len() as f64 / ne as f64;
                omean = ocnt as f64 / ne as f64;
                oeq = Some(om.len() == map.len() && om.keys().all(|k| map.contains_key(k)));
            }
            let ratio = frac / pred;
            println!("q={} n={} mod={:?} a={} b={} #E={} ({}) |F|={} multisets={} frac={:.5} pred={:.5} ratio={:.4} meanM={:.4} ord={:?}/{:?}/{:?}",
                q, n, &fl.f[..n], a, b, ne, facs, m, cnt, frac, pred, ratio, mean, ofrac, omean, oeq);
            writeln!(csv, "{},{},{},{},{},\"{:?}\",{},{},{},{:.6},{},{:.6},{:.6},{:.6},{:.6},{:.6},{},{},{},{},{}",
                sub, q, n, a, b, &fl.f[..n], ne, facs, m, m as f64 / q as f64, cnt, frac, pred, pred2, ratio, mean,
                ocnt, if n <= 3 { format!("{:.6}", ofrac) } else { "NA".into() },
                if n <= 3 { format!("{:.6}", omean) } else { "NA".into() },
                oeq.map(|b| b.to_string()).unwrap_or("NA".into()), hs).unwrap();
            if !first { json.push_str(",\n"); }
            first = false;
            let hj: Vec<String> = h.iter().map(|(k, v)| format!("\"{}\": {}", k, v)).collect();
            write!(json, "  {{\"curve_over_subfield\": {}, \"q\": {}, \"n\": {}, \"modulus_low_coeffs\": {:?}, \"a_enc\": {}, \"b_enc\": {}, \"nE\": {}, \"nE_factorisation\": \"{}\", \"fb_size\": {}, \"fb_over_q\": {:.6}, \"multisets\": {}, \"decomposable_fraction\": {:.6}, \"pred_1_over_nfact\": {:.6}, \"pred_1_minus_exp\": {:.6}, \"ratio\": {:.6}, \"mean_M\": {:.6}, \"ordered_tuples\": {}, \"ordered_decomposable_fraction\": {}, \"ordered_mean_N\": {}, \"ordered_support_equals_multiset_support\": {}, \"hist_M\": {{{}}}}}",
                sub, q, n, &fl.f[..n], a, b, ne, facs, m, m as f64 / q as f64, cnt, frac, pred, pred2, ratio, mean, ocnt,
                if n <= 3 { format!("{:.6}", ofrac) } else { "null".into() },
                if n <= 3 { format!("{:.6}", omean) } else { "null".into() },
                oeq.map(|b| b.to_string()).unwrap_or("null".into()), hj.join(", ")).unwrap();
            writeln!(if sub { &mut table_c } else { &mut table }, "| {} | {} | ({},{}) | {} | {} | {} | {:.4} | {:.5} | {:.5} | {:.5} | {:.4} |", q, n, a, b, ne, facs, m, m as f64 / q as f64, frac, pred, pred2, ratio).unwrap();
        }
    }
    json.push_str("\n ]\n}\n");
    std::fs::create_dir_all("results").unwrap();
    std::fs::write("results/ext_decomp.json", json).unwrap();
    std::fs::write("results/ext_decomp.csv", csv).unwrap();
    std::fs::write("results/table_primary.md", &table).unwrap();
    std::fs::write("results/table_control.md", &table_c).unwrap();
    println!("{}\n{}\nruntime {:.1}s", table, table_c, t0.elapsed().as_secs_f64());
}
