//! Protocol A1 (research/math_st_open_problems_20261119/PROTOCOLS.md).
//!
//! R(n,d) = max over (a,b,c) != 0 of #{x in F_n : a + b x + c x^d = 0}.
//! For c = 1 the scaling x -> u x sends b to b u^{1-d}, so b ranges over 0 and
//! one representative per coset of (F_n^*)^{d-1}; R is then the largest value
//! multiplicity of x -> x^d + b x. Trinomials with c = 0 have at most 1 root.
//!
//! Usage:
//!   a1_trinomial_roots control            brute-force agreement for primes < 200
//!   a1_trinomial_roots sweep LO HI OUT    all primes LO <= n < HI, all d; writes OUT
//!   a1_trinomial_roots prime N OUT        one prime, all d; writes OUT
//!   a1_trinomial_roots analyze JSON BITS_CSV IN.csv...   summary statistics
use rayon::prelude::*;
use std::io::Write;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
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

fn pow_mod(mut b: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1 % n;
    b %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % n;
        }
        b = b * b % n;
        e >>= 1;
    }
    r
}

fn prime_factors(mut m: u64) -> Vec<u64> {
    let mut f = Vec::new();
    let mut p = 2;
    while p * p <= m {
        if m % p == 0 {
            f.push(p);
            while m % p == 0 {
                m /= p;
            }
        }
        p += 1;
    }
    if m > 1 {
        f.push(m);
    }
    f
}

fn generator(n: u64) -> u64 {
    let fs = prime_factors(n - 1);
    (2..n)
        .find(|&g| fs.iter().all(|&q| pow_mod(g, (n - 1) / q, n) != 1))
        .unwrap()
}

/// Per-prime tables: pw[k] = g^k for k in 0..n-1.
struct Field {
    n: u64,
    pw: Vec<u32>,
}

impl Field {
    fn new(n: u64) -> Self {
        let g = generator(n);
        let mut pw = vec![0u32; (n - 1) as usize];
        let mut x = 1u64;
        for k in 0..(n - 1) as usize {
            pw[k] = x as u32;
            x = x * g % n;
        }
        Field { n, pw }
    }
}

struct Row {
    d: u64,
    r: u32,
    l: u64,
    b_star: u64,
}

/// Exact R(n,d) by the reduced method; `counts` is scratch of length n.
fn r_reduced(f: &Field, d: u64, counts: &mut [u32]) -> (u32, u64) {
    let n = f.n;
    let nm1 = n - 1;
    let e = gcd(d - 1, nm1);
    let mut best = 1u32; // c = 0 gives at most one root; any c = 1 trinomial has >= 0
    let mut best_b = 0u64;
    // b = 0, then b = g^j for j < e (coset representatives of the (d-1)-th powers).
    for j in 0..=e {
        let b: u64 = if j == 0 { 0 } else { f.pw[(j - 1) as usize] as u64 };
        counts.iter_mut().for_each(|c| *c = 0);
        counts[0] += 1; // x = 0 gives value 0
        let mut local = counts[0];
        let mut kd: u64 = 0; // k*d mod (n-1)
        let dm = d % nm1;
        for k in 0..nm1 as usize {
            let x = f.pw[k] as u64;
            let xd = f.pw[kd as usize] as u64;
            let v = ((xd + b * x) % n) as usize;
            counts[v] += 1;
            if counts[v] > local {
                local = counts[v];
            }
            kd += dm;
            if kd >= nm1 {
                kd -= nm1;
            }
        }
        if local > best {
            best = local;
            best_b = b;
        }
    }
    (best, best_b)
}

fn lower_envelope(n: u64, d: u64) -> u64 {
    gcd(d, n - 1).max(1 + gcd(d - 1, n - 1))
}

fn run_prime(n: u64) -> Vec<Row> {
    let f = Field::new(n);
    let mut rows: Vec<Row> = (2..=n - 2)
        .into_par_iter()
        .map_init(
            || vec![0u32; n as usize],
            |counts, d| {
                let (r, b) = r_reduced(&f, d, counts);
                Row { d, r, l: lower_envelope(n, d), b_star: b }
            },
        )
        .collect();
    rows.sort_by_key(|r| r.d);
    rows
}

/// Control 1: brute force over all (a, b) with c = 1 for small primes.
fn brute(n: u64, d: u64) -> u32 {
    let mut best = 1u32;
    let xd: Vec<u64> = (0..n).map(|x| pow_mod(x, d, n)).collect();
    for b in 0..n {
        let mut counts = vec![0u32; n as usize];
        for x in 0..n {
            counts[((xd[x as usize] + b * x) % n) as usize] += 1;
        }
        best = best.max(*counts.iter().max().unwrap());
    }
    best
}

fn inverse_mod(a: u64, m: u64) -> Option<u64> {
    let (mut t, mut nt, mut r, mut nr) = (0i128, 1i128, m as i128, a as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    if r != 1 {
        return None;
    }
    Some(t.rem_euclid(m as i128) as u64)
}

/// Controls 2 and 3 on computed rows; returns the number of failures.
fn check_rows(n: u64, rows: &[Row]) -> usize {
    let mut fail = 0;
    let by_d: std::collections::HashMap<u64, u32> = rows.iter().map(|r| (r.d, r.r)).collect();
    for row in rows {
        if (row.r as u64) < row.l {
            eprintln!("CONTROL3 FAIL n={} d={} R={} L={}", n, row.d, row.r, row.l);
            fail += 1;
        }
        if gcd(row.d, n - 1) == 1 {
            let di = inverse_mod(row.d, n - 1).unwrap();
            if let Some(&ri) = by_d.get(&di) {
                if ri != row.r {
                    eprintln!("CONTROL2 FAIL n={} d={} R={} d^-1={} R={}", n, row.d, row.r, di, ri);
                    fail += 1;
                }
            }
        }
    }
    fail
}

fn write_rows(out: &mut impl Write, n: u64, rows: &[Row]) {
    for r in rows {
        writeln!(
            out,
            "{},{},{},{},{},{},{},{}",
            n,
            r.d,
            r.r,
            r.l,
            gcd(r.d, n - 1),
            gcd(r.d - 1, n - 1),
            gcd(r.d, n + 1),
            r.b_star
        )
        .unwrap();
    }
}


/// Summary of one or more CSV files written by `sweep`/`prime`.
fn analyze(paths: &[String], out_json: &str, out_bits_csv: &str) {
    use std::collections::BTreeMap;
    // bits -> (rows, sum excess, max excess, argmax line, lead rows, histogram)
    #[derive(Default)]
    struct B { per_prime_max: std::collections::BTreeMap<u64, i64>, rows: u64, sum_ex: i64, max_ex: i64, arg: String, lead: u64, max_r_small_l: u32, primes: std::collections::BTreeSet<u64> }
    let mut by_bits: BTreeMap<u32, B> = BTreeMap::new();
    let mut hist: BTreeMap<i64, u64> = BTreeMap::new();
    // gcd(d, n+1) bucket -> (rows, max excess)
    let mut by_np1: BTreeMap<u64, (u64, i64)> = BTreeMap::new();
    let mut lead_lines: Vec<String> = Vec::new();
    for path in paths {
        let text = std::fs::read_to_string(path).unwrap();
        for line in text.lines().skip(1) {
            let v: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
            let (n, r, l, gp1) = (v[0], v[2] as i64, v[3] as i64, v[6]);
            let bits = 64 - n.leading_zeros();
            let ex = r - l;
            let log2n = (n as f64).log2();
            let b = by_bits.entry(bits).or_default();
            b.rows += 1;
            b.sum_ex += ex;
            b.primes.insert(n);
            let pm = b.per_prime_max.entry(n).or_insert(i64::MIN);
            if ex > *pm { *pm = ex; }
            if ex > b.max_ex || b.arg.is_empty() {
                b.max_ex = ex;
                b.arg = line.to_string();
            }
            if l <= 3 {
                b.max_r_small_l = b.max_r_small_l.max(r as u32);
            }
            let is_lead = r >= 4 * l && (r as f64) >= 4.0 * log2n;
            if is_lead {
                b.lead += 1;
                if lead_lines.len() < 1000 { lead_lines.push(line.to_string()); }
            }
            *hist.entry(ex).or_default() += 1;
            let bucket = if gp1 >= 64 { 64 } else if gp1 >= 16 { 16 } else if gp1 >= 4 { 4 } else { gp1 };
            let e = by_np1.entry(bucket).or_default();
            e.0 += 1;
            e.1 = e.1.max(ex);
        }
    }
    let mut csv = String::from("bits,primes,rows,mean_excess,mean_per_prime_max_excess,max_excess,max_R_when_L_le_3,lead_rows,argmax_row\n");
    for (bits, b) in &by_bits {
        let ppm = b.per_prime_max.values().sum::<i64>() as f64 / b.per_prime_max.len() as f64;
        csv += &format!("{},{},{},{:.4},{:.3},{},{},{},\"{}\"\n", bits, b.primes.len(), b.rows,
            b.sum_ex as f64 / b.rows as f64, ppm, b.max_ex, b.max_r_small_l, b.lead, b.arg);
    }
    std::fs::write(out_bits_csv, csv).unwrap();
    let mut j = String::from("{\n  \"schema\": \"a1-summary/v1\",\n  \"inputs\": [");
    j += &paths.iter().map(|p| format!("\"{}\"", p)).collect::<Vec<_>>().join(", ");
    j += "],\n  \"excess_histogram\": {";
    j += &hist.iter().map(|(k, v)| format!("\"{}\": {}", k, v)).collect::<Vec<_>>().join(", ");
    j += "},\n  \"max_excess_by_gcd_d_np1_bucket\": {";
    j += &by_np1.iter().map(|(k, v)| format!("\"{}\": {{\"rows\": {}, \"max_excess\": {}}}", k, v.0, v.1)).collect::<Vec<_>>().join(", ");
    j += &format!("}},\n  \"lead_rows_total\": {},\n  \"lead_rows_sample\": [", lead_lines.len());
    j += &lead_lines.iter().map(|l| format!("\"{}\"", l)).collect::<Vec<_>>().join(", ");
    j += "]\n}\n";
    std::fs::write(out_json, j).unwrap();
}

const HEADER: &str = "n,d,R,L,gcd_d_nm1,gcd_dm1_nm1,gcd_d_np1,b_star";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("control") => {
            let mut checked = 0;
            let mut fail = 0;
            for n in (5..200).filter(|&n| is_prime(n)) {
                let rows = run_prime(n);
                for r in &rows {
                    let b = brute(n, r.d);
                    checked += 1;
                    if b != r.r {
                        eprintln!("CONTROL1 FAIL n={} d={} reduced={} brute={}", n, r.d, r.r, b);
                        fail += 1;
                    }
                }
                fail += check_rows(n, &rows);
            }
            println!("control: {} (n,d) pairs checked against brute force, {} failures", checked, fail);
            if fail > 0 {
                std::process::exit(1);
            }
        }
        Some("sweep") | Some("prime") => {
            let (primes, out_path): (Vec<u64>, &str) = if args[1] == "sweep" {
                let lo: u64 = args[2].parse().unwrap();
                let hi: u64 = args[3].parse().unwrap();
                ((lo..hi).filter(|&n| is_prime(n)).collect(), &args[4])
            } else {
                (vec![args[2].parse().unwrap()], &args[3])
            };
            let mut out = std::io::BufWriter::new(std::fs::File::create(out_path).unwrap());
            writeln!(out, "{}", HEADER).unwrap();
            let mut fail = 0;
            for n in primes {
                let t = std::time::Instant::now();
                let rows = run_prime(n);
                fail += check_rows(n, &rows);
                write_rows(&mut out, n, &rows);
                let worst = rows.iter().max_by_key(|r| (r.r as i64 - r.l as i64, r.r)).unwrap();
                eprintln!(
                    "n={} done in {:.2}s; max R-L = {} at d={} (R={}, L={})",
                    n,
                    t.elapsed().as_secs_f64(),
                    worst.r as i64 - worst.l as i64,
                    worst.d,
                    worst.r,
                    worst.l
                );
            }
            if fail > 0 {
                eprintln!("{} control failures", fail);
                std::process::exit(1);
            }
        }
        Some("analyze") => {
            // analyze OUT_JSON OUT_BITS_CSV IN.csv...
            analyze(&args[4..], &args[2], &args[3]);
        }
        _ => {
            eprintln!("usage: control | sweep LO HI OUT | prime N OUT");
            std::process::exit(2);
        }
    }
}
