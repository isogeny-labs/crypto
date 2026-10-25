// Solinas-shape primes vs residue-matched and generic primes: distribution of
// #E(F_p) over random short-Weierstrass curves, against the exact
// matrix-model prediction (Gekeler; Castryck-Hubrechts). Protocol:
// research/solinas_distribution_20261023/PROTOCOL.md.
//
//   cargo run --release --example solinas_order_census -- \
//       --bits-min 16 --bits-max 22 --curves 4000 --nulls 2 --seed 20261023 \
//       --out research/solinas_distribution_20261023/runs/census.jsonl
//
// Output: one JSON line per prime (raw counts and p-values), then a summary
// on stdout. Nothing is tuned after the data: the decision rule is in the
// protocol.
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

const J: u32 = 5; // 2-adic precision: matrices mod 2^J
const MOD2: u64 = 1 << J;
const NB: usize = (J as usize) + 1; // v2 bins 0..=J (J = "2^J divides")

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

// ---- exact model: Frobenius uniform on {M in M2(Z/m) : det M = d} ----

// P(min(v2(det(I - M) mod 2^J), J) = j) for each odd d mod 2^J.
fn model2() -> Vec<[f64; NB]> {
    let mut counts = vec![[0u64; NB]; MOD2 as usize];
    let mut tot = vec![0u64; MOD2 as usize];
    for a in 0..MOD2 {
        for b in 0..MOD2 {
            for c in 0..MOD2 {
                for d in 0..MOD2 {
                    let det = (a * d + MOD2 * MOD2 - b * c % MOD2) % MOD2;
                    if det % 2 == 0 {
                        continue;
                    }
                    let tr = (a + d) % MOD2;
                    let e = (1 + 2 * MOD2 - tr + det) % MOD2; // det(I - M)
                    let v = if e == 0 { J } else { e.trailing_zeros() };
                    counts[det as usize][v as usize] += 1;
                    tot[det as usize] += 1;
                }
            }
        }
    }
    counts
        .iter()
        .zip(tot.iter())
        .map(|(c, &t)| {
            let mut r = [0f64; NB];
            if t > 0 {
                for j in 0..NB {
                    r[j] = c[j] as f64 / t as f64;
                }
            }
            r
        })
        .collect()
}

// P(l | det(I - M)) for M uniform with det M = d mod l, for each d in 1..l.
fn model_l(l: u64) -> Vec<f64> {
    let mut hit = vec![0u64; l as usize];
    let mut tot = vec![0u64; l as usize];
    for a in 0..l {
        for b in 0..l {
            for c in 0..l {
                for d in 0..l {
                    let det = (a * d + l * l - b * c % l) % l;
                    if det == 0 {
                        continue;
                    }
                    let tr = (a + d) % l;
                    let e = (1 + 2 * l - tr + det) % l;
                    tot[det as usize] += 1;
                    if e == 0 {
                        hit[det as usize] += 1;
                    }
                }
            }
        }
    }
    (0..l as usize)
        .map(|d| if tot[d] > 0 { hit[d] as f64 / tot[d] as f64 } else { 0.0 })
        .collect()
}

// ---- chi-square survival: Q(k/2, x/2) ----
fn ln_gamma(x: f64) -> f64 {
    let c = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5,
    ];
    let mut y = x;
    let tmp = x + 5.5 - (x + 0.5) * (x + 5.5).ln();
    let mut ser = 1.000000000190015;
    for cj in c {
        y += 1.0;
        ser += cj / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

fn gamma_q(a: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    if x < a + 1.0 {
        let (mut ap, mut sum, mut del) = (a, 1.0 / a, 1.0 / a);
        for _ in 0..1000 {
            ap += 1.0;
            del *= x / ap;
            sum += del;
            if del.abs() < sum.abs() * 1e-16 {
                break;
            }
        }
        1.0 - sum * (-x + a * x.ln() - ln_gamma(a)).exp()
    } else {
        let tiny = 1e-300;
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / tiny;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..1000 {
            let an = -(i as f64) * (i as f64 - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < tiny {
                d = tiny;
            }
            c = b + an / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-16 {
                break;
            }
        }
        (-x + a * x.ln() - ln_gamma(a)).exp() * h
    }
}

fn chi2_sf(x: f64, df: usize) -> f64 {
    gamma_q(df as f64 / 2.0, x / 2.0)
}

// chi-square of observed counts against probabilities, merging bins whose
// expected count is below 5 into their neighbour. Returns (stat, df).
fn chi2(obs: &[u64; NB], prob: &[f64; NB], n: u64) -> (f64, usize) {
    let mut o = Vec::new();
    let mut e = Vec::new();
    let (mut ao, mut ae) = (0f64, 0f64);
    for j in 0..NB {
        ao += obs[j] as f64;
        ae += prob[j] * n as f64;
        if ae >= 5.0 {
            o.push(ao);
            e.push(ae);
            ao = 0.0;
            ae = 0.0;
        }
    }
    if ae > 0.0 || ao > 0.0 {
        if let (Some(lo), Some(le)) = (o.last_mut(), e.last_mut()) {
            *lo += ao;
            *le += ae;
        }
    }
    let stat: f64 = o.iter().zip(e.iter()).map(|(o, e)| (o - e) * (o - e) / e).sum();
    (stat, o.len().saturating_sub(1))
}

fn z2_pvalue(hits: u64, n: u64, p0: f64) -> (f64, f64) {
    let z2 = {
        let mu = n as f64 * p0;
        let var = n as f64 * p0 * (1.0 - p0);
        let d = hits as f64 - mu;
        d * d / var
    };
    (z2, chi2_sf(z2, 1))
}

// ---- census of one prime ----
struct PrimeRow {
    role: &'static str,
    shape: &'static str,
    n: u32,
    k: u32,
    p: u64,
    obs2: [u64; NB],
    h3: u64,
    h5: u64,
    curves: u64,
    p_f1: f64,
    p_f2: f64,
    p_f3: f64,
    p_f4: f64,
    ncp: f64, // N * sum (q - r)^2 / r : noncentrality of F2 (wrong-class model)
    tv_generic: f64,
}

fn census(p: u64, curves: u64, seed: u64) -> ([u64; NB], u64, u64, u64) {
    let pu = p as usize;
    // Legendre symbol table: chi[0] = 0, nonzero squares +1, non-squares -1.
    let mut chi = vec![0i8; pu];
    let mut isq = vec![false; pu];
    for x in 1..p {
        isq[(x * x % p) as usize] = true;
    }
    for t in 1..pu {
        chi[t] = if isq[t] { 1 } else { -1 };
    }
    chi[0] = 0;
    drop(isq);
    let cube: Vec<u32> = (0..p).map(|x| (x * x % p * x % p) as u32).collect();
    let mut st = seed;
    let mut obs = [0u64; NB];
    let (mut h3, mut h5, mut done) = (0u64, 0u64, 0u64);
    while done < curves {
        let a = splitmix(&mut st) % p;
        let b = splitmix(&mut st) % p;
        // discriminant: 4a^3 + 27 b^2 != 0
        let disc = (4 * (a * a % p * a % p) + 27 * (b * b % p)) % p;
        if disc == 0 {
            continue;
        }
        let mut s: i64 = 0;
        let mut ax = 0u64;
        for x in 0..pu {
            let mut t = cube[x] as u64 + ax + b;
            if t >= p {
                t -= p;
            }
            if t >= p {
                t -= p;
            }
            s += chi[t as usize] as i64;
            ax += a;
            if ax >= p {
                ax -= p;
            }
        }
        let order = (p as i64 + 1 + s) as u64;
        let v = if order == 0 { J } else { order.trailing_zeros().min(J) };
        obs[v as usize] += 1;
        if order % 3 == 0 {
            h3 += 1;
        }
        if order % 5 == 0 {
            h5 += 1;
        }
        done += 1;
    }
    (obs, h3, h5, done)
}

fn analyse(
    role: &'static str,
    shape: &'static str,
    n: u32,
    k: u32,
    p: u64,
    curves: u64,
    seed: u64,
    m2: &[[f64; NB]],
    m3: &[f64],
    m5: &[f64],
) -> PrimeRow {
    let (obs2, h3, h5, done) = census(p, curves, seed);
    let right = m2[(p % MOD2) as usize];
    // generic model: mean over odd d with the same residue mod 4
    let mut gen = [0f64; NB];
    let mut cnt = 0.0;
    for d in (1..MOD2).step_by(2) {
        if d % 4 == p % 4 {
            for j in 0..NB {
                gen[j] += m2[d as usize][j];
            }
            cnt += 1.0;
        }
    }
    for j in 0..NB {
        gen[j] /= cnt;
    }
    let (s1, d1) = chi2(&obs2, &right, done);
    // F2 (positive control): the model for the wrong class mod 4, d' = -p.
    let wrong = m2[((MOD2 - p % MOD2) % MOD2) as usize];
    let (s2, d2) = chi2(&obs2, &wrong, done);
    let (_, p3) = z2_pvalue(h3, done, m3[(p % 3) as usize]);
    let (_, p5) = z2_pvalue(h5, done, m5[(p % 5) as usize]);
    let ncp: f64 = (0..NB)
        .filter(|&j| right[j] > 0.0)
        .map(|j| (wrong[j] - right[j]).powi(2) / right[j])
        .sum::<f64>()
        * done as f64;
    // Exact total-variation distance between the prime's own-residue model and
    // the generic model (mean over odd d = p mod 4).
    let tv_generic: f64 = 0.5 * (0..NB).map(|j| (gen[j] - right[j]).abs()).sum::<f64>();
    PrimeRow {
        role,
        shape,
        n,
        k,
        p,
        obs2,
        h3,
        h5,
        curves: done,
        p_f1: chi2_sf(s1, d1),
        p_f2: chi2_sf(s2, d2),
        p_f3: p3,
        p_f4: p5,
        ncp,
        tv_generic,
    }
}

fn arg<T: std::str::FromStr>(args: &[String], name: &str, default: T) -> T {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bmin: u32 = arg(&args, "--bits-min", 16);
    let bmax: u32 = arg(&args, "--bits-max", 22);
    let curves: u64 = arg(&args, "--curves", 4000);
    let nulls: usize = arg(&args, "--nulls", 2);
    let seed: u64 = arg(&args, "--seed", 20261023);
    let kmin: u32 = arg(&args, "--kmin", 6);
    let threads: usize = arg(&args, "--threads", 4);
    let out: String = arg(&args, "--out", String::new());

    let m2 = Arc::new(model2());
    let m3 = Arc::new(model_l(3));
    let m5 = Arc::new(model_l(5));

    // Jobs: (role, shape, n, k, p, seed)
    let mut jobs: Vec<(&'static str, &'static str, u32, u32, u64, u64)> = Vec::new();
    let mut rs = seed;
    let mut seen = std::collections::HashSet::new();
    for n in bmin..=bmax {
        for k in kmin..n.saturating_sub(2) {
            for (shape, sign) in [("p224-shape", 1i64), ("p192-shape", -1i64)] {
                let p = ((1i64 << n) - (1i64 << k) + sign) as u64;
                if is_prime(p) {
                    seen.insert(p);
                    jobs.push(("target", shape, n, k, p, splitmix(&mut rs)));
                }
            }
        }
    }
    let targets: Vec<_> = jobs.clone();
    for &(_, shape, n, k, p, _) in &targets {
        // residue-matched: p' = p mod 480, random in [2^(n-1), 2^n)
        let r = p % 480;
        let mut got = 0;
        while got < nulls {
            let lo = 1u64 << (n - 1);
            let m = splitmix(&mut rs) % (lo / 480);
            let cand = lo + (r + 480 - lo % 480) % 480 + 480 * m;
            if cand >> n != 0 || cand < lo {
                continue;
            }
            if is_prime(cand) && !seen.contains(&cand) {
                seen.insert(cand);
                jobs.push(("residue-matched", shape, n, k, cand, splitmix(&mut rs)));
                got += 1;
            }
        }
        let mut got = 0;
        while got < nulls {
            let lo = 1u64 << (n - 1);
            let cand = lo + splitmix(&mut rs) % lo | 1;
            if is_prime(cand) && !seen.contains(&cand) {
                seen.insert(cand);
                jobs.push(("generic", shape, n, k, cand, splitmix(&mut rs)));
                got += 1;
            }
        }
    }
    // Largest first for load balance.
    jobs.sort_by_key(|j| std::cmp::Reverse(j.4));
    eprintln!("{} primes ({} targets)", jobs.len(), targets.len());
    let queue = Arc::new(Mutex::new(jobs));
    let results = Arc::new(Mutex::new(Vec::<PrimeRow>::new()));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let (queue, results) = (queue.clone(), results.clone());
        let (m2, m3, m5) = (m2.clone(), m3.clone(), m5.clone());
        hs.push(thread::spawn(move || loop {
            let job = queue.lock().unwrap().pop();
            let Some((role, shape, n, k, p, s)) = job else { break };
            let row = analyse(role, shape, n, k, p, curves, s, &m2, &m3, &m5);
            eprintln!("done {} p={} ({} bits)", role, p, n);
            results.lock().unwrap().push(row);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    let mut rows = results.lock().unwrap();
    rows.sort_by_key(|r| (r.n, r.k, r.shape, r.role, r.p));

    if !out.is_empty() {
        let mut f = File::create(&out).unwrap();
        for r in rows.iter() {
            writeln!(
                f,
                "{{\"role\":\"{}\",\"shape\":\"{}\",\"n\":{},\"k\":{},\"p\":{},\"curves\":{},\"v2_bins\":{:?},\"h3\":{},\"h5\":{},\"p_f1\":{:e},\"p_f2\":{:e},\"p_f3\":{:e},\"p_f4\":{:e},\"ncp_f2\":{:.3},\"tv_generic\":{:.5},\"seed\":{}}}",
                r.role, r.shape, r.n, r.k, r.p, r.curves, r.obs2, r.h3, r.h5,
                r.p_f1, r.p_f2, r.p_f3, r.p_f4, r.ncp, r.tv_generic, seed
            )
            .unwrap();
        }
    }

    // Summary under the protocol's decision rule.
    let alpha = 0.01;
    let tests_f1 = rows.len() * 3; // F1, F3, F4 per prime
    let thr = alpha / tests_f1 as f64;
    println!("alpha={} Bonferroni over {} F1/F3/F4 tests: threshold {:.3e}", alpha, tests_f1, thr);
    for role in ["target", "residue-matched", "generic"] {
        let sel: Vec<&PrimeRow> = rows.iter().filter(|r| r.role == role).collect();
        let rej = |f: fn(&PrimeRow) -> f64| sel.iter().filter(|r| f(r) < thr).count();
        println!(
            "{:16} primes={:3}  F1 reject {:2}  F3 reject {:2}  F4 reject {:2}",
            role,
            sel.len(),
            rej(|r| r.p_f1),
            rej(|r| r.p_f3),
            rej(|r| r.p_f4)
        );
    }
    // Detector power (F2: generic model, should reject for targets / R primes)
    let thr2 = alpha / rows.len() as f64;
    for role in ["target", "residue-matched", "generic"] {
        let sel: Vec<&PrimeRow> = rows.iter().filter(|r| r.role == role).collect();
        let rej = sel.iter().filter(|r| r.p_f2 < thr2).count();
        let med_ncp = {
            let mut v: Vec<f64> = sel.iter().map(|r| r.ncp).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v.get(v.len() / 2).copied().unwrap_or(0.0)
        };
        println!(
            "{:16} F2 (wrong-class model) reject {:2}/{:2}, median noncentrality {:.1}",
            role,
            rej,
            sel.len(),
            med_ncp
        );
    }
}
