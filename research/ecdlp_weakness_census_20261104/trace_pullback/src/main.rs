// Trace-pullback census for GAP-PF-TRACE-PULLBACK-FACTOR-BASE. Pure Rust, no crates.
use std::fmt::Write as _;
use std::time::Instant;

type F2 = (u64, u64);
type Pt = Option<(F2, F2)>;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d * d <= n { if n % d == 0 { return false; } d += 1; }
    true
}
fn powm(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    b %= p;
    while e > 0 { if e & 1 == 1 { r = r * b % p; } b = b * b % p; e >>= 1; }
    r
}

#[derive(Clone, Copy)]
struct Ctx { p: u64, a: u64, b: u64 } // curve y^2 = x^3 + a x + b, a,b in F_p

impl Ctx {
    fn add2(&self, x: F2, y: F2) -> F2 { ((x.0 + y.0) % self.p, (x.1 + y.1) % self.p) }
    fn sub2(&self, x: F2, y: F2) -> F2 { ((x.0 + self.p - y.0) % self.p, (x.1 + self.p - y.1) % self.p) }
    fn neg2(&self, x: F2) -> F2 { ((self.p - x.0) % self.p, (self.p - x.1) % self.p) }
    fn mul2(&self, x: F2, y: F2) -> F2 {
        let p = self.p;
        ((x.0 * y.0 + p * p - x.1 * y.1) % p, (x.0 * y.1 + x.1 * y.0) % p)
    }
    fn inv2(&self, x: F2) -> F2 {
        let p = self.p;
        let n = (x.0 * x.0 + x.1 * x.1) % p;
        let ni = powm(n, p - 2, p);
        (x.0 * ni % p, (p - x.1) % p * ni % p)
    }
    fn pow2(&self, mut b: F2, mut e: u64) -> F2 {
        let mut r = (1, 0);
        while e > 0 { if e & 1 == 1 { r = self.mul2(r, b); } b = self.mul2(b, b); e >>= 1; }
        r
    }
    fn frob(&self, x: F2) -> F2 { (x.0, (self.p - x.1) % self.p) } // (a+bi)^p = a-bi
    fn rhs(&self, x: F2) -> F2 {
        let x2 = self.mul2(x, x);
        let x3 = self.mul2(x2, x);
        self.add2(self.add2(x3, (self.a * x.0 % self.p, self.a * x.1 % self.p)), (self.b, 0))
    }
    fn sqrt_p(&self, n: u64) -> Option<u64> {
        let s = powm(n, (self.p + 1) / 4, self.p);
        if s * s % self.p == n % self.p { Some(s) } else { None }
    }
    // square root in F_{p^2}; None if non-square.
    fn sqrt2(&self, z: F2) -> Option<F2> {
        let p = self.p;
        let (a, b) = z;
        if b == 0 {
            if let Some(x) = self.sqrt_p(a) { return Some((x, 0)); }
            return self.sqrt_p((p - a) % p).map(|y| (0, y));
        }
        let n = (a * a + b * b) % p;
        let s0 = self.sqrt_p(n)?;
        let inv2 = (p + 1) / 2;
        for s in [s0, (p - s0) % p] {
            let t = (a + s) % p * inv2 % p;
            if let Some(x) = self.sqrt_p(t) {
                if x != 0 {
                    let y = b * powm(2 * x % p, p - 2, p) % p;
                    let r = (x, y);
                    if self.mul2(r, r) == z { return Some(r); }
                }
            }
        }
        None
    }
    fn padd(&self, p1: Pt, p2: Pt) -> Pt {
        let (a, b) = match (p1, p2) { (None, q) => return q, (q, None) => return q, (Some(a), Some(b)) => (a, b) };
        let lam;
        if a.0 == b.0 {
            if self.add2(a.1, b.1) == (0, 0) { return None; }
            let x2 = self.mul2(a.0, a.0);
            let num = self.add2(self.add2(self.add2(x2, x2), x2), (self.a, 0));
            let den = self.add2(a.1, a.1);
            lam = self.mul2(num, self.inv2(den));
        } else {
            lam = self.mul2(self.sub2(b.1, a.1), self.inv2(self.sub2(b.0, a.0)));
        }
        let x3 = self.sub2(self.sub2(self.mul2(lam, lam), a.0), b.0);
        let y3 = self.sub2(self.mul2(lam, self.sub2(a.0, x3)), a.1);
        Some((x3, y3))
    }
    fn smul(&self, mut k: u64, mut q: Pt) -> Pt {
        let mut r = None;
        while k > 0 { if k & 1 == 1 { r = self.padd(r, q); } q = self.padd(q, q); k >>= 1; }
        r
    }
    fn trace(&self, q: (F2, F2)) -> Pt {
        let fq = (self.frob(q.0), self.frob(q.1));
        self.padd(Some(q), Some(fq))
    }
}

fn order_fp(p: u64, a: u64, b: u64) -> u64 {
    let mut n = 1u64;
    for x in 0..p {
        let r = (x * x % p * x + a * x + b) % p;
        if r == 0 { n += 1; } else if powm(r, (p - 1) / 2, p) == 1 { n += 2; }
    }
    n
}

struct VRow { label: String, f_size: usize, t_size: usize, ratio: f64, decomp: f64, pred: f64 }

fn main() {
    let t0 = Instant::now();
    let seed: u64 = 0x5EED_2026_1104_0001;
    println!("seed = {:#x}", seed);
    let mut rng = Rng(seed);
    let primes = [103u64, 211, 503, 1019, 2003];
    let mut json = String::from("{\n  \"seed\": \"0x5eed202611040001\",\n  \"curves\": [\n");
    let mut csv = String::from("p,A,B,n_Fp,n_Fp2,V,F_size,T_size,T_over_n,decomp_frac,pred\n");
    let mut summary: Vec<String> = Vec::new();
    let mut first_curve = true;

    for &p in &primes {
        assert!(is_prime(p) && p % 4 == 3);
        let mut chosen: Vec<(u64, u64, u64)> = Vec::new();
        while chosen.len() < 2 {
            let a = rng.next() % p;
            let b = rng.next() % p;
            if (4 * a % p * a % p * a % p + 27 * b % p * b) % p == 0 { continue; }
            if chosen.iter().any(|c| c.0 == a && c.1 == b) { continue; }
            let n = order_fp(p, a, b);
            if is_prime(n) && n != p { chosen.push((a, b, n)); }
        }
        for &(a, b, n) in &chosen {
            let c = Ctx { p, a, b };
            let t = p as i64 + 1 - n as i64;
            let n2 = (p + 1) * (p + 1) - (t * t) as u64;
            // --- self-tests ---
            for _ in 0..50 {
                let x = (rng.next() % p, rng.next() % p);
                assert_eq!(c.pow2(x, p * p), x, "x^(p^2)=x");
                assert_eq!(c.pow2(x, p), c.frob(x), "frobenius");
                if x != (0, 0) { assert_eq!(c.mul2(x, c.inv2(x)), (1, 0)); }
            }
            // generator of E(F_p) and log table
            let mut gen: Pt = None;
            for x in 0..p {
                let r = (x * x % p * x + a * x + b) % p;
                if let Some(y) = c.sqrt_p(r) { if y != 0 || true { gen = Some(((x, 0), (y, 0))); break; } }
            }
            let g = gen.unwrap();
            let mut logtab = vec![u32::MAX; (p * p) as usize];
            let mut cur: Pt = None;
            let nn = n as usize;
            for k in 0..nn {
                match cur { None => assert_eq!(k, 0), Some((x, y)) => {
                    assert!(x.1 == 0 && y.1 == 0);
                    let idx = (x.0 * p + y.0) as usize;
                    assert_eq!(logtab[idx], u32::MAX);
                    logtab[idx] = k as u32;
                } }
                cur = c.padd(cur, Some(g));
            }
            assert!(cur.is_none(), "n*G = O over F_p");
            // group-law vs log-addition consistency
            let pts: Vec<(F2, F2)> = (0..nn).filter_map(|k| { let q = c.smul(k as u64, Some(g)); q }).collect();
            for _ in 0..200 {
                let i = (rng.next() % (nn as u64 - 1)) as usize;
                let j = (rng.next() % (nn as u64 - 1)) as usize;
                let s = c.padd(Some(pts[i]), Some(pts[j]));
                let lg = match s { None => 0, Some((x, y)) => logtab[(x.0 * p + y.0) as usize] as usize };
                assert_eq!(lg, (i + 1 + j + 1) % nn);
            }
            let lookup = |q: Pt| -> u32 { match q { None => 0, Some((x, y)) => { assert!(x.1 == 0 && y.1 == 0, "trace not in F_p"); let l = logtab[(x.0 * p + y.0) as usize]; assert!(l != u32::MAX); l } } };

            // random points of E(F_{p^2}): trace in F_p and N*P = O
            let mut tested = 0;
            while tested < 20 {
                let x = (rng.next() % p, rng.next() % p);
                if let Some(y) = c.sqrt2(c.rhs(x)) {
                    assert_eq!(c.mul2(y, y), c.rhs(x));
                    let q = (x, y);
                    let tr = c.trace(q);
                    if let Some((tx, ty)) = tr { assert!(tx.1 == 0 && ty.1 == 0); }
                    assert!(c.smul(n2, Some(q)).is_none(), "N*P=O");
                    tested += 1;
                }
            }

            // --- census over subspaces ---
            let mut rows: Vec<VRow> = Vec::new();
            let mut total_pts: u64 = 0; // sum |F'_V|
            let mut zero_cnt = 0u64;
            let mut mark = vec![false; nn];
            let mut min_t = usize::MAX;
            let mut min_row = 0usize;
            for vi in 0..=p {
                let (label, cv) = if vi < p { (format!("{}", vi), vi) } else { ("inf".to_string(), 0) };
                let mut logs: Vec<u32> = Vec::new();
                let mut fsize = 0usize;
                for tt in 0..p {
                    let x: F2 = if vi < p { (tt, tt * cv % p) } else { (0, tt) };
                    if let Some(y) = c.sqrt2(c.rhs(x)) {
                        let ys: Vec<F2> = if y == (0, 0) { vec![y] } else { vec![y, c.neg2(y)] };
                        for yy in ys {
                            fsize += 1;
                            if x == (0, 0) { zero_cnt += 1; }
                            logs.push(lookup(c.trace((x, yy))));
                        }
                    }
                }
                total_pts += fsize as u64;
                logs.sort_unstable();
                logs.dedup();
                let tsize = logs.len();
                for m in mark.iter_mut() { *m = false; }
                let n32 = nn as u32;
                for &u in &logs { for &w in &logs { let mut s = u + w; if s >= n32 { s -= n32; } mark[s as usize] = true; } }
                let dec = mark.iter().filter(|&&m| m).count() as f64 / nn as f64;
                let pred = 1.0 - (-((tsize * tsize) as f64) / nn as f64).exp();
                if tsize < min_t { min_t = tsize; min_row = rows.len(); }
                rows.push(VRow { label, f_size: fsize, t_size: tsize, ratio: tsize as f64 / nn as f64, decomp: dec, pred });
            }
            // x=0 counted in every one of p+1 subspaces; remaining x sets are disjoint; add O.
            let n2_enum = total_pts - zero_cnt / (p + 1) * p + 1;
            assert_eq!(n2_enum, n2, "#E(F_p^2) enumeration vs (p+1)^2-t^2");
            let mut sizes: Vec<usize> = rows.iter().map(|r| r.t_size).collect();
            sizes.sort_unstable();
            let k = sizes.len();
            let median = if k % 2 == 1 { sizes[k / 2] as f64 } else { (sizes[k / 2 - 1] + sizes[k / 2]) as f64 / 2.0 };
            let mn = rows[min_row].t_size;
            let mx = *sizes.last().unwrap();
            let mr = &rows[min_row];
            println!("p={} A={} B={} n={} n2={} |T| min/med/max = {}/{}/{} argmin c={} ratio={:.4} dec={:.4} pred={:.4}",
                p, a, b, n, n2, mn, median, mx, mr.label, mr.ratio, mr.decomp, mr.pred);
            summary.push(format!("| {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.4} | {:.4} | {:.4} |",
                p, a, b, n, n2, mn, median, mx, mr.label, mn as f64 / nn as f64, mr.decomp, mr.pred));

            if !first_curve { json.push_str(",\n"); }
            first_curve = false;
            write!(json, "    {{\"p\": {}, \"A\": {}, \"B\": {}, \"n_Fp\": {}, \"n_Fp2\": {}, \"T_min\": {}, \"T_median\": {}, \"T_max\": {}, \"argmin_c\": \"{}\", \"min_ratio\": {:.6}, \"decomp_at_min\": {:.6}, \"pred_at_min\": {:.6},\n      \"V\": [", p, a, b, n, n2, mn, median, mx, mr.label, mn as f64 / nn as f64, mr.decomp, mr.pred).unwrap();
            for (i, r) in rows.iter().enumerate() {
                if i > 0 { json.push_str(", "); }
                write!(json, "{{\"c\": \"{}\", \"F_size\": {}, \"T_size\": {}, \"T_over_n\": {:.6}, \"decomp\": {:.6}, \"pred\": {:.6}}}", r.label, r.f_size, r.t_size, r.ratio, r.decomp, r.pred).unwrap();
                writeln!(csv, "{},{},{},{},{},{},{},{},{:.6},{:.6},{:.6}", p, a, b, n, n2, r.label, r.f_size, r.t_size, r.ratio, r.decomp, r.pred).unwrap();
            }
            json.push_str("]}");
        }
    }
    json.push_str("\n  ]\n}\n");
    std::fs::write("results/trace_pullback.json", json).unwrap();
    std::fs::write("results/trace_pullback.csv", csv).unwrap();
    let mut s = String::from("| p | A | B | #E(F_p) | #E(F_p^2) | min T | median T | max T | argmin c | min T / #E(F_p) | decomp at min | pred at min |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    for r in &summary { s.push_str(r); s.push('\n'); }
    std::fs::write("results/summary_table.md", &s).unwrap();
    println!("{}", s);
    println!("runtime {:.1}s", t0.elapsed().as_secs_f64());
}
