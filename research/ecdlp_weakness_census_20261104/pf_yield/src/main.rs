use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;

type Pt = Option<(u64, u64)>;
const SEED: u64 = 0x9E3779B97F4A7C15;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 { return false; }
        d += 1;
    }
    true
}
fn mulm(a: u64, b: u64, p: u64) -> u64 { ((a as u128 * b as u128) % p as u128) as u64 }
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    a %= p;
    while e > 0 {
        if e & 1 == 1 { r = mulm(r, a, p); }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 { powm(a, p - 2, p) }

#[derive(Clone, Copy)]
struct Curve { p: u64, a: u64, b: u64 }
impl Curve {
    fn add(&self, q1: Pt, q2: Pt) -> Pt {
        let p = self.p;
        let (a, b) = match (q1, q2) {
            (None, _) => return q2,
            (_, None) => return q1,
            (Some(a), Some(b)) => (a, b),
        };
        let lam;
        if a.0 == b.0 {
            if (a.1 + b.1) % p == 0 { return None; }
            let num = (3 * mulm(a.0, a.0, p) + self.a) % p;
            lam = mulm(num, inv(2 * a.1 % p, p), p);
        } else {
            let num = (b.1 + p - a.1) % p;
            let den = (b.0 + p - a.0) % p;
            lam = mulm(num, inv(den, p), p);
        }
        let x3 = (mulm(lam, lam, p) + 2 * p - a.0 - b.0) % p;
        let y3 = (mulm(lam, (a.0 + p - x3) % p, p) + p - a.1) % p;
        Some((x3, y3))
    }
    fn mul(&self, mut k: u64, q: Pt) -> Pt {
        let mut r: Pt = None;
        let mut base = q;
        while k > 0 {
            if k & 1 == 1 { r = self.add(r, base); }
            base = self.add(base, base);
            k >>= 1;
        }
        r
    }
}

/// All affine points sorted by (x, y), plus order = len+1.
fn enumerate(c: &Curve, sqrt: &[u64]) -> Vec<(u64, u64)> {
    let p = c.p;
    let mut v = Vec::new();
    for x in 0..p {
        let rhs = (mulm(mulm(x, x, p), x, p) + mulm(c.a, x, p) + c.b) % p;
        let y = sqrt[rhs as usize];
        if y == u64::MAX { continue; }
        v.push((x, y));
        if y != 0 { v.push((x, p - y)); }
    }
    v.sort();
    v
}

fn fact(n: usize) -> f64 { (1..=n).fold(1.0, |a, i| a * i as f64) }

struct Stats {
    n_targets: u64,
    mean: f64,
    var: f64,
    decomp: f64,
    hist: Vec<u64>,
}
fn stats(counts: &[u64]) -> Stats {
    let n = counts.len() as f64;
    let s: u64 = counts.iter().sum();
    let s2: f64 = counts.iter().map(|&c| (c * c) as f64).sum();
    let mean = s as f64 / n;
    let var = s2 / n - mean * mean;
    let dec = counts.iter().filter(|&&c| c >= 1).count() as f64 / n;
    let mx = *counts.iter().max().unwrap() as usize;
    let mut hist = vec![0u64; mx + 1];
    for &c in counts { hist[c as usize] += 1; }
    Stats { n_targets: counts.len() as u64, mean, var, decomp: dec, hist }
}
fn jarr_u(v: &[u64]) -> String { format!("[{}]", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")) }
fn jarr_f(v: &[f64]) -> String { format!("[{}]", v.iter().map(|x| format!("{:.8e}", x)).collect::<Vec<_>>().join(",")) }
fn jstats(s: &Stats, mu: f64) -> String {
    let pois: Vec<f64> = (0..s.hist.len()).map(|n| (-mu).exp() * mu.powi(n as i32) / fact(n)).collect();
    format!(
        "{{\"n_targets\":{},\"mean\":{:.10},\"variance\":{:.10},\"decomposable_fraction\":{:.10},\"hist\":{},\"hist_fraction\":{},\"poisson_mu\":{}}}",
        s.n_targets, s.mean, s.var, s.decomp, jarr_u(&s.hist),
        jarr_f(&s.hist.iter().map(|&h| h as f64 / s.n_targets as f64).collect::<Vec<_>>()),
        jarr_f(&pois)
    )
}

fn main() {
    println!("seed = {:#x}", SEED);
    let mut rng = Rng(SEED);
    let primes = [2003u64, 5003, 10007, 20011, 40009, 80021];
    let mut json = String::from("{\"seed\":\"0x9E3779B97F4A7C15\",\"rows\":[\n");
    let mut csv = String::from("p,curve,a,b,r,B,X,mu,mean,variance,decomposable_fraction,predicted_fraction,ratio,var_over_mean,decomp_frac_outside,n_outside,base,U_mean,U_P_ge1,U_predicted,U_ratio,U_var_over_mean\n");
    let mut table = String::from("| base | p | a | b | r | B | mu | N decomp frac | 1-exp(-mu) | N ratio | N var/mean | N decomp frac (R not in F, not O) | mean U | U var/mean | P(U>=1) | 1-exp(-mean U) | U ratio |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
    let mut first_row = true;
    for &p in &primes {
        if !is_prime(p) { println!("skip non-prime {}", p); continue; }
        let mut sqrt = vec![u64::MAX; p as usize];
        for y in 0..p { let s = (y * y % p) as usize; if sqrt[s] == u64::MAX { sqrt[s] = y; } }
        let mut found = 0;
        let mut seen = Vec::new();
        while found < 3 {
            let a = rng.next() % p;
            let b = rng.next() % p;
            if (4 * mulm(mulm(a, a, p), a, p) + 27 * mulm(b, b, p)) % p == 0 { continue; }
            if seen.contains(&(a, b)) { continue; }
            let c = Curve { p, a, b };
            let pts = enumerate(&c, &sqrt);
            let r = pts.len() as u64 + 1;
            if !is_prime(r) || r == p { continue; }
            seen.push((a, b));
            found += 1;
            // self-tests
            for _ in 0..20 {
                let pick = |rng: &mut Rng| { let q = pts[(rng.next() % pts.len() as u64) as usize]; Some(q) };
                let (p1, q1, s1) = (pick(&mut rng), pick(&mut rng), pick(&mut rng));
                assert!(c.mul(r, p1).is_none(), "r*P != O");
                assert_eq!(c.add(c.add(p1, q1), s1), c.add(p1, c.add(q1, s1)), "assoc");
            }
            let mut index: HashMap<(u64, u64), usize> = HashMap::with_capacity(pts.len() * 2);
            for (i, &q) in pts.iter().enumerate() { index.insert(q, i); }
            let inf_idx = pts.len();
            // distinct x values with cumulative counts
            let mut cum: Vec<(u64, usize)> = Vec::new();
            for (i, q) in pts.iter().enumerate() {
                if i + 1 == pts.len() || pts[i + 1].0 != q.0 { cum.push((q.0, i + 1)); }
            }
            let rf = r as f64;
            for (tag, target) in [("0.5", (rf / 2.0).sqrt()), ("1", rf.sqrt()), ("2", (2.0 * rf).sqrt())] {
                let _ = tag;
                let &(xx, bb) = cum.iter().min_by(|u, v| {
                    (u.1 as f64 - target).abs().partial_cmp(&(v.1 as f64 - target).abs()).unwrap()
                }).unwrap();
                let mut brng = Rng(SEED ^ (p << 20) ^ (a << 5) ^ b ^ ((bb as u64) << 40) | 1);
                for _ in 0..8 { brng.next(); }
                for base in ["small_x", "random", "random_negclosed"] {
                let fb: Vec<(u64, u64)> = match base {
                    "small_x" => pts[..bb].to_vec(),
                    "random" => {
                        let mut set = std::collections::BTreeSet::new();
                        while set.len() < bb { set.insert(pts[(brng.next() % pts.len() as u64) as usize]); }
                        set.into_iter().collect()
                    }
                    _ => {
                        let mut set = std::collections::BTreeSet::new();
                        while set.len() < bb {
                            let q = pts[(brng.next() % pts.len() as u64) as usize];
                            set.insert(q);
                            set.insert((q.0, if q.1 == 0 { 0 } else { p - q.1 }));
                        }
                        set.into_iter().collect()
                    }
                };
                let bb = fb.len();
                let in_f: std::collections::HashSet<(u64, u64)> = fb.iter().cloned().collect();
                let mut cnt = vec![0u64; r as usize];
                let mut ucnt = vec![0u64; r as usize];
                for (i1, &p1) in fb.iter().enumerate() {
                    for (i2, &p2) in fb.iter().enumerate() {
                        let s = c.add(Some(p1), Some(p2));
                        let i = match s { None => inf_idx, Some(q) => index[&q] };
                        cnt[i] += 1;
                        if i1 <= i2 { ucnt[i] += 1; }
                    }
                }
                let b2 = (bb * bb) as u64;
                assert_eq!(cnt.iter().sum::<u64>(), b2, "sum N(R) != B^2");
                assert_eq!(ucnt.iter().sum::<u64>(), (bb * (bb + 1) / 2) as u64, "sum U != B(B+1)/2");
                if base != "random" { assert_eq!(cnt[inf_idx], bb as u64, "N(O) != B"); }
                let mu = b2 as f64 / rf;
                let all = stats(&cnt);
                assert!((all.mean * rf - b2 as f64).abs() < 1e-6);
                let outside: Vec<u64> = (0..pts.len()).filter(|&i| !in_f.contains(&pts[i])).map(|i| cnt[i]).collect();
                let out = stats(&outside);
                let ust = stats(&ucnt);
                let upred = 1.0 - (-ust.mean).exp();
                let uratio = ust.decomp / upred;
                let uvm = ust.var / ust.mean;
                let pred = 1.0 - (-mu).exp();
                let ratio = all.decomp / pred;
                let vm = all.var / all.mean;
                println!("base={} p={} a={} b={} r={} B={} X={} mu={:.4} dec={:.5} pred={:.5} ratio={:.4} var/mean={:.4} dec_out={:.5} | U mean={:.4} var/mean={:.4} P(U>=1)={:.5} pred={:.5} ratio={:.4}", base, p, a, b, r, bb, xx, mu, all.decomp, pred, ratio, vm, out.decomp, ust.mean, uvm, ust.decomp, upred, uratio);
                if !first_row { json.push_str(",\n"); }
                first_row = false;
                let _ = write!(json, "{{\"base\":\"{}\",\"p\":{},\"a\":{},\"b\":{},\"r\":{},\"B\":{},\"X\":{},\"mu\":{:.10},\"sum_N\":{},\"predicted_fraction\":{:.10},\"ratio\":{:.10},\"all_targets\":{},\"outside_F_and_infinity\":{},\"unordered_U\":{},\"U_predicted_fraction\":{:.10},\"U_ratio\":{:.10},\"U_var_over_mean\":{:.10}}}",
                    base, p, a, b, r, bb, xx, mu, b2, pred, ratio, jstats(&all, mu), jstats(&out, mu), jstats(&ust, ust.mean), upred, uratio, uvm);
                let _ = writeln!(csv, "{},{},{},{},{},{},{},{:.6},{:.8},{:.8},{:.8},{:.8},{:.6},{:.6},{:.8},{},{},{:.8},{:.8},{:.8},{:.6},{:.6}", p, found, a, b, r, bb, xx, mu, all.mean, all.var, all.decomp, pred, ratio, vm, out.decomp, out.n_targets, base, ust.mean, ust.decomp, upred, uratio, uvm);
                let _ = writeln!(table, "| {} | {} | {} | {} | {} | {} | {:.3} | {:.4} | {:.4} | {:.3} | {:.3} | {:.4} | {:.3} | {:.3} | {:.4} | {:.4} | {:.3} |", base, p, a, b, r, bb, mu, all.decomp, pred, ratio, vm, out.decomp, ust.mean, uvm, ust.decomp, upred, uratio);
                }
            }
        }
    }
    json.push_str("\n]}\n");
    fs::create_dir_all("results").unwrap();
    fs::write("results/pf_yield.json", json).unwrap();
    fs::write("results/pf_yield.csv", csv).unwrap();
    fs::write("results/table.md", table).unwrap();
}
