//! Exact census of every F_p-isogeny class of elliptic curves (Tate: one per
//! Frobenius trace t with |t| <= 2 sqrt(p)).
//!
//!   census     --p P --out ROWS.jsonl [--threads T]   one row per trace + check file
//!   bruteforce --p P                                  naive point count of every
//!              y^2 = x^3 + a x + b over a small F_p, compared with (p-1)/2 * H(4p - t^2)
//!   summary    --rows ROWS.jsonl --out SUMMARY.json   statistics over the rows
//!
//! H(4p - t^2) = sum_{d | f} h(d^2 D_K) * 2 / w(d^2 D_K) is the number of
//! F_p-isomorphism classes with trace t, each weighted by 2/|Aut|; summed over
//! t it equals 2p exactly, which `census` checks (stored as 6H, an integer).

mod classno;
mod nt;
mod phase2;
mod phase3;
mod phase4;
mod phase5;
mod stats;

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

use nt::{factor, frob_disc, is_prime, isqrt, kronecker, order_class_ratio, divisors, powmod};

struct Row {
    t: i64,
    n: u64,
    factors: Vec<(u64, u32)>,
    r: u64,
    twist_r: u64,
    d_k: i64,
    f: u64,
    f_factors: Vec<(u64, u32)>,
    h_dk: u64,
    h6: u64, // 6 * H(4p - t^2)
    n_orders: u64,
    k: Option<u64>,
    anomalous: bool,
    supersingular: bool,
    rho_cost_ph: f64,
}

fn mult_order(a: u64, r: u64) -> u64 {
    // order of a in (Z/r)^*, r prime, a != 0 mod r
    let mut k = r - 1;
    for (q, _) in factor((r - 1) as u128) {
        let q = q as u64;
        while k % q == 0 && powmod(a as u128, (k / q) as u128, r as u128) == 1 {
            k /= q;
        }
    }
    k
}

fn row(p: u64, t: i64) -> Row {
    let n = (p as i64 + 1 - t) as u64;
    let nt = (p as i64 + 1 + t) as u64;
    let factors: Vec<(u64, u32)> = factor(n as u128).into_iter().map(|(q, e)| (q as u64, e)).collect();
    let r = factors.iter().map(|x| x.0).max().unwrap();
    let twist_r = factor(nt as u128).into_iter().map(|x| x.0 as u64).max().unwrap();
    let fd = frob_disc(t as i128, p as u128);
    let d_k = fd.d_k as i64;
    let h_dk = classno::class_number(fd.d_k);
    // 6H = sum_{c | f} h(c^2 D_K) * 12 / w(c^2 D_K)
    let mut h6: u128 = 0;
    let divs = divisors(&fd.f_factors);
    for (c, cf) in &divs {
        let (num, den) = order_class_ratio(fd.d_k, cf);
        let hc = h_dk as u128 * num;
        assert!(hc % den == 0, "class number formula not integral");
        let hc = hc / den;
        let w: u128 = if *c == 1 && d_k == -3 { 6 } else if *c == 1 && d_k == -4 { 4 } else { 2 };
        h6 += hc * 12 / w;
    }
    let k = if r != p { Some(mult_order(p % r, r)) } else { None };
    let rho_cost_ph = factors
        .iter()
        .map(|&(q, e)| e as f64 * (std::f64::consts::PI * q as f64 / 2.0).sqrt())
        .sum();
    Row {
        t,
        n,
        factors,
        r,
        twist_r,
        d_k,
        f: fd.f as u64,
        f_factors: fd.f_factors.iter().map(|&(q, e)| (q as u64, e)).collect(),
        h_dk,
        h6: h6 as u64,
        n_orders: divs.len() as u64,
        k,
        anomalous: n == p,
        supersingular: t == 0,
        rho_cost_ph,
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn row_json(x: &Row) -> String {
    let fac: Vec<String> = x.factors.iter().map(|(q, e)| format!("[{q},{e}]")).collect();
    let vol: Vec<String> = x.f_factors.iter().map(|(q, e)| format!("\"{q}\":{e}")).collect();
    let g = gcd(x.h6, 6);
    format!(
        "{{\"t\":{},\"N\":{},\"factors\":[{}],\"r\":{},\"log2_r\":{:.6},\"cofactor\":{},\"twist_r\":{},\"fp2_r\":{},\"D_K\":{},\"f\":{},\"h_DK\":{},\"H_num\":{},\"H_den\":{},\"n_orders\":{},\"volcano\":{{{}}},\"embedding_degree\":{},\"anomalous\":{},\"supersingular\":{},\"rho_cost_ph\":{:.3}}}",
        x.t, x.n, fac.join(","), x.r, (x.r as f64).log2(), x.n / x.r, x.twist_r, x.r.max(x.twist_r),
        x.d_k, x.f, x.h_dk, x.h6 / g, 6 / g, x.n_orders, vol.join(","),
        x.k.map(|k| k.to_string()).unwrap_or("null".into()), x.anomalous, x.supersingular, x.rho_cost_ph
    )
}

fn trace_bound(p: u64) -> i64 {
    let mut b = isqrt(4 * p as u128) as i64;
    if (b as u128) * (b as u128) == 4 * p as u128 {
        b -= 1;
    }
    b
}

fn compute_all(p: u64, threads: usize) -> Vec<Row> {
    assert!(p > 3 && is_prime(p as u128));
    let b = trace_bound(p);
    let ts: Vec<i64> = (-b..=b).collect();
    let chunk = ts.len().div_ceil(threads);
    let mut out: Vec<Row> = Vec::with_capacity(ts.len());
    std::thread::scope(|s| {
        let hs: Vec<_> = ts.chunks(chunk).map(|c| s.spawn(move || c.iter().map(|&t| row(p, t)).collect::<Vec<_>>())).collect();
        for h in hs {
            out.extend(h.join().unwrap());
        }
    });
    out
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).map(|i| args[i + 1].clone())
}

fn cmd_census(args: &[String]) {
    let p: u64 = arg(args, "--p").map(|s| s.parse().unwrap()).unwrap_or((1 << 31) - 1);
    let out = arg(args, "--out").expect("--out");
    let threads: usize = arg(args, "--threads").map(|s| s.parse().unwrap()).unwrap_or(4);
    let t0 = std::time::Instant::now();
    let rows = compute_all(p, threads);
    let mut w = BufWriter::new(OpenOptions::new().write(true).create_new(true).open(&out).expect("output exists"));
    let mut sum6: u128 = 0;
    for x in &rows {
        sum6 += x.h6 as u128;
        writeln!(w, "{}", row_json(x)).unwrap();
    }
    w.flush().unwrap();
    let ok = sum6 == 12 * p as u128;
    let check = format!(
        "{{\"p\":{p},\"traces\":{},\"t_range\":[{},{}],\"sum_6H\":{sum6},\"twelve_p\":{},\"sum_H_equals_2p\":{ok},\"seconds\":{:.1},\"tool\":\"fp31-class-census census\"}}",
        rows.len(), rows[0].t, rows[rows.len() - 1].t, 12 * p as u128, t0.elapsed().as_secs_f64()
    );
    println!("{check}");
    let mut f = OpenOptions::new().write(true).create_new(true).open(format!("{out}.check.json")).unwrap();
    writeln!(f, "{check}").unwrap();
    if !ok {
        std::process::exit(1);
    }
}

fn cmd_bruteforce(args: &[String]) {
    let p: u64 = arg(args, "--p").expect("--p").parse().unwrap();
    assert!(p < 2000, "naive check is O(p^3)");
    let mut sq = vec![0u64; p as usize];
    for y in 0..p {
        sq[(y * y % p) as usize] += 1;
    }
    let mut cnt: BTreeMap<i64, u64> = BTreeMap::new();
    for a in 0..p {
        for b in 0..p {
            if (4 * a * a % p * a + 27 * b % p * b) % p == 0 {
                continue;
            }
            let mut n = 1u64;
            for x in 0..p {
                n += sq[((x * x % p * x + a * x + b) % p) as usize];
            }
            *cnt.entry(p as i64 + 1 - n as i64).or_default() += 1;
        }
    }
    let rows = compute_all(p, 1);
    let mut bad = 0;
    for x in &rows {
        // #{(a,b)} = (p-1)/2 * H  <=>  12 * #{(a,b)} = (p-1) * 6H
        let got = *cnt.get(&x.t).unwrap_or(&0);
        if 12 * got != (p - 1) * x.h6 {
            bad += 1;
            println!("MISMATCH t={} pairs={} 6H={}", x.t, got, x.h6);
        }
    }
    let extra = cnt.keys().filter(|t| t.abs() > trace_bound(p)).count();
    println!(
        "{{\"p\":{p},\"p_mod_12\":{},\"curves\":{},\"traces\":{},\"mismatches\":{bad},\"traces_outside_bound\":{extra}}}",
        p % 12, cnt.values().sum::<u64>(), rows.len()
    );
    if bad > 0 || extra > 0 {
        std::process::exit(1);
    }
}

// ---- summary --------------------------------------------------------------

fn jnum(line: &str, key: &str) -> Option<String> {
    let pat = format!("\"{key}\":");
    let i = line.find(&pat)? + pat.len();
    let rest = &line[i..];
    let end = rest.find(|c: char| c == ',' || c == '}').unwrap();
    Some(rest[..end].to_string())
}

struct S {
    t: i64,
    r: u64,
    twist_r: u64,
    h: f64,
    f: u64,
    d_k: i64,
    n_orders: u64,
    k: Option<u64>,
    cost: f64,
    n: u64,
    line: String,
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            r[idx[k]] = avg;
        }
        i = j + 1;
    }
    r
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let (ma, mb) = (a.iter().sum::<f64>() / n, b.iter().sum::<f64>() / n);
    let (mut sab, mut saa, mut sbb) = (0.0, 0.0, 0.0);
    for i in 0..a.len() {
        sab += (a[i] - ma) * (b[i] - mb);
        saa += (a[i] - ma).powi(2);
        sbb += (b[i] - mb).powi(2);
    }
    sab / (saa * sbb).sqrt()
}

fn spearman(a: &[f64], b: &[f64]) -> f64 {
    pearson(&ranks(a), &ranks(b))
}

fn wquantile(v: &mut Vec<(f64, f64)>, q: f64) -> f64 {
    v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let tot: f64 = v.iter().map(|x| x.1).sum();
    let mut acc = 0.0;
    for &(x, w) in v.iter() {
        acc += w;
        if acc >= q * tot {
            return x;
        }
    }
    v.last().unwrap().0
}

fn cmd_summary(args: &[String]) {
    let rows_path = arg(args, "--rows").expect("--rows");
    let out = arg(args, "--out").expect("--out");
    let mut v: Vec<S> = Vec::new();
    for line in BufReader::new(File::open(&rows_path).unwrap()).lines() {
        let line = line.unwrap();
        let hn: f64 = jnum(&line, "H_num").unwrap().parse().unwrap();
        let hd: f64 = jnum(&line, "H_den").unwrap().parse().unwrap();
        let k = jnum(&line, "embedding_degree").unwrap();
        v.push(S {
            t: jnum(&line, "t").unwrap().parse().unwrap(),
            r: jnum(&line, "r").unwrap().parse().unwrap(),
            twist_r: jnum(&line, "twist_r").unwrap().parse().unwrap(),
            h: hn / hd,
            f: jnum(&line, "f").unwrap().parse().unwrap(),
            d_k: jnum(&line, "D_K").unwrap().parse().unwrap(),
            n_orders: jnum(&line, "n_orders").unwrap().parse().unwrap(),
            k: if k == "null" { None } else { Some(k.parse().unwrap()) },
            cost: jnum(&line, "rho_cost_ph").unwrap().parse().unwrap(),
            n: jnum(&line, "N").unwrap().parse().unwrap(),
            line,
        });
    }
    let p = v[0].n as i64 + v[0].t - 1;
    let curves: f64 = v.iter().map(|x| x.h).sum();
    let ordinary: Vec<&S> = v.iter().filter(|x| x.t != 0).collect();
    let mut o = String::from("{\n");
    o += &format!("\"p\":{p},\"classes\":{},\"weighted_curves\":{curves},\n", v.len());

    // 1. largest prime r: per class and per curve
    let prime_order_classes = v.iter().filter(|x| x.r == x.n).count();
    let prime_order_curves: f64 = v.iter().filter(|x| x.r == x.n).map(|x| x.h).sum();
    o += &format!("\"prime_order\":{{\"classes\":{prime_order_classes},\"curve_fraction\":{:.6}}},\n", prime_order_curves / curves);
    let mut bins: BTreeMap<i64, (u64, f64)> = BTreeMap::new();
    for x in &v {
        let b = ((x.r as f64).log2() * 4.0).floor() as i64;
        let e = bins.entry(b).or_default();
        e.0 += 1;
        e.1 += x.h;
    }
    let bs: Vec<String> = bins.iter().map(|(b, (c, w))| format!("[{:.2},{c},{:.6}]", *b as f64 / 4.0, w / curves)).collect();
    o += &format!("\"log2_r_hist_quarter_bits\":{{\"columns\":[\"bin_lo\",\"classes\",\"curve_fraction\"],\"rows\":[{}]}},\n", bs.join(","));
    for (name, f) in [("log2_rho_cost_ph", 0usize), ("log2_r", 1usize)] {
        let mut pc: Vec<(f64, f64)> = v.iter().map(|x| (if f == 0 { x.cost.log2() } else { (x.r as f64).log2() }, x.h)).collect();
        let mut pk: Vec<(f64, f64)> = v.iter().map(|x| (if f == 0 { x.cost.log2() } else { (x.r as f64).log2() }, 1.0)).collect();
        let qs = [0.0001, 0.001, 0.01, 0.1, 0.5, 0.9];
        let a: Vec<String> = qs.iter().map(|&q| format!("\"{q}\":{:.4}", wquantile(&mut pc, q))).collect();
        let b: Vec<String> = qs.iter().map(|&q| format!("\"{q}\":{:.4}", wquantile(&mut pk, q))).collect();
        o += &format!("\"{name}_quantiles\":{{\"curve_weighted\":{{{}}},\"class_weighted\":{{{}}}}},\n", a.join(","), b.join(","));
    }
    let full = (std::f64::consts::PI * p as f64 / 2.0).sqrt().log2();
    o += &format!("\"log2_rho_cost_prime_order_reference\":{full:.4},\n");

    // 2. weakest classes by expected generic cost
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].cost.partial_cmp(&v[b].cost).unwrap());
    let weak: Vec<String> = idx[..25].iter().map(|&i| v[i].line.clone()).collect();
    o += &format!("\"weakest_25_by_rho_cost_ph\":[\n{}\n],\n", weak.join(",\n"));
    let anom: Vec<String> = v.iter().filter(|x| x.n as i64 == p).map(|x| x.line.clone()).collect();
    o += &format!("\"anomalous\":[{}],\n", anom.join(","));
    let ss: Vec<String> = v.iter().filter(|x| x.t == 0).map(|x| x.line.clone()).collect();
    o += &format!("\"supersingular\":[{}],\n", ss.join(","));

    // 3. embedding degree
    let mut small_k: Vec<&S> = v.iter().filter(|x| x.k.map_or(false, |k| k <= 1000)).collect();
    small_k.sort_by_key(|x| x.k.unwrap());
    let sk: Vec<String> = small_k.iter().map(|x| format!("{{\"t\":{},\"r\":{},\"k\":{},\"H\":{}}}", x.t, x.r, x.k.unwrap(), x.h)).collect();
    o += &format!("\"embedding_degree_le_1000\":[{}],\n", sk.join(","));
    let min_log2_k = v.iter().filter_map(|x| x.k).map(|k| (k as f64).log2()).fold(f64::INFINITY, f64::min);
    o += &format!("\"min_log2_embedding_degree\":{min_log2_k:.4},\n");

    // 3b. embedding degree among classes whose largest prime is large
    for lo in [20u32, 24, 28] {
        let m = v.iter().filter(|x| x.r >= 1u64 << lo).filter_map(|x| x.k.map(|k| (k, x.t, x.r))).min();
        if let Some((k, t, r)) = m {
            o += &format!("\"min_embedding_degree_r_ge_2^{lo}\":{{\"k\":{k},\"log2_k\":{:.3},\"t\":{t},\"r\":{r}}},\n", (k as f64).log2());
        }
    }
    // 3c. Lenstra: share of curves (H-weighted) and of classes with l | N, small primes l
    let mut lr_rows = Vec::new();
    for l in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47] {
        let cw: f64 = v.iter().filter(|x| x.n % l == 0).map(|x| x.h).sum::<f64>() / curves;
        let kw = v.iter().filter(|x| x.n % l == 0).count() as f64 / v.len() as f64;
        let lf = l as f64;
        // Frobenius mod l is equidistributed in {g in GL2(F_l): det g = p mod l}; it has
        // eigenvalue 1 with probability l/(l^2-1) when p = 1 mod l (double root) and
        // 1/(l-1) otherwise (Lenstra 1987; Howe 1993).
        let pred = if (p as u64) % l == 1 { lf / (lf * lf - 1.0) } else { 1.0 / (lf - 1.0) };
        lr_rows.push(format!("[{l},{},{cw:.6},{kw:.6},{pred:.6},{:.6}]", (p as u64) % l == 1, 1.0 / lf));
    }
    o += &format!("\"lenstra_divisibility\":{{\"columns\":[\"l\",\"p_equiv_1_mod_l\",\"curve_fraction_l_divides_N\",\"class_fraction_l_divides_N\",\"lenstra_prediction\",\"one_over_l\"],\"rows\":[{}]}},\n", lr_rows.join(","));
    // 4. twist and F_{p^2}
    let tw_bigger = v.iter().filter(|x| x.twist_r > x.r).count();
    let fp2_smaller = v.iter().filter(|x| x.r.max(x.twist_r) < x.r).count();
    o += &format!("\"twist\":{{\"classes_twist_r_gt_r\":{tw_bigger},\"classes_fp2_r_lt_r\":{fp2_smaller}}},\n");

    // 5. class structure vs cost (ordinary classes)
    let lr: Vec<f64> = ordinary.iter().map(|x| (x.r as f64).log2()).collect();
    let lh: Vec<f64> = ordinary.iter().map(|x| x.h.log2()).collect();
    let lf: Vec<f64> = ordinary.iter().map(|x| (x.f as f64).log2()).collect();
    let lk: Vec<f64> = ordinary.iter().map(|x| (x.d_k.unsigned_abs() as f64).log2()).collect();
    let no: Vec<f64> = ordinary.iter().map(|x| x.n_orders as f64).collect();
    let at: Vec<f64> = ordinary.iter().map(|x| x.t.unsigned_abs() as f64).collect();
    o += &format!(
        "\"spearman_log2r_vs\":{{\"n\":{},\"log2_H\":{:.5},\"log2_f\":{:.5},\"log2_abs_DK\":{:.5},\"n_orders\":{:.5},\"abs_t\":{:.5}}},\n",
        lr.len(), spearman(&lr, &lh), spearman(&lr, &lf), spearman(&lr, &lk), spearman(&lr, &no), spearman(&lr, &at)
    );
    // curve-count-weighted mean log2 r by number of endomorphism-ring strata
    let mut by_no: BTreeMap<u64, (u64, f64, f64)> = BTreeMap::new();
    for x in &ordinary {
        let e = by_no.entry(x.n_orders.min(8)).or_default();
        e.0 += 1;
        e.1 += x.h;
        e.2 += x.h * (x.r as f64).log2();
    }
    let bn: Vec<String> = by_no.iter().map(|(k, (c, w, s))| format!("[{k},{c},{:.6},{:.4}]", w / curves, s / w)).collect();
    o += &format!("\"by_n_orders_capped8\":{{\"columns\":[\"n_orders\",\"classes\",\"curve_fraction\",\"mean_log2_r_curve_weighted\"],\"rows\":[{}]}},\n", bn.join(","));
    let largest = v.iter().max_by(|a, b| a.h.partial_cmp(&b.h).unwrap()).unwrap();
    let smallest = ordinary.iter().min_by(|a, b| a.h.partial_cmp(&b.h).unwrap()).unwrap();
    let max_f = ordinary.iter().max_by_key(|x| x.f).unwrap();
    let max_no = ordinary.iter().max_by_key(|x| x.n_orders).unwrap();
    o += &format!("\"largest_class\":{},\n\"smallest_ordinary_class\":{},\n\"max_conductor_class\":{},\n\"max_strata_class\":{}\n}}\n", largest.line, smallest.line, max_f.line, max_no.line);
    let mut f = OpenOptions::new().write(true).create_new(true).open(&out).unwrap();
    f.write_all(o.as_bytes()).unwrap();
    let _ = kronecker; // re-exported helper used by classno
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("census") => cmd_census(&args),
        Some("bruteforce") => cmd_bruteforce(&args),
        Some("summary") => cmd_summary(&args),
        Some("roots") => phase2::cmd_roots(&args),
        Some("select") => phase2::cmd_select(&args),
        Some("analyze") => phase2::cmd_analyze(&args),
        Some("ic-classes") => phase3::cmd_ic_classes(&args),
        Some("ic-analyze") => phase3::cmd_ic_analyze(&args),
        Some("timing") => phase4::cmd_timing(&args),
        Some("rho-spec") => phase4::cmd_rho_spec(&args),
        Some("rho-timing") => phase4::cmd_rho_timing(&args),
        Some("ic-timing") => phase5::cmd_ic_timing(&args),
        Some("rho-timing-diag") => phase5::cmd_rho_timing_diag(&args),
        Some("rho-timing-diag2") => phase5::cmd_rho_timing_diag2(&args),
        Some("rho-timing-diag3") => phase5::cmd_rho_timing_diag3(&args),
        _ => {
            eprintln!("usage: fp31-class-census census|bruteforce|summary|roots|select|analyze|ic-classes|ic-analyze|timing|rho-spec|rho-timing|ic-timing ...");
            std::process::exit(2);
        }
    }
}
