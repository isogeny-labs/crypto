//! P3.3 native rerun: medians of upstream math-st's recorded pure-power and
//! unconstrained norms, grouped by bit size of p. Std only; no CSV crate.
//!
//! Usage: p33_native RAW_CSV
//! Rule (same as the earlier Python read): a row is used only if
//! power2_norm, power3_norm and unconstrained_norm are all non-empty.
use std::collections::BTreeMap;

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    assert!(n > 0);
    if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 }
}

/// RFC 4180 field split for one line: double-quoted fields may hold commas,
/// and "" inside quotes is a literal quote. Panics on an unterminated quote.
fn split_csv(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut q) = (Vec::new(), String::new(), false);
    let mut it = line.chars().peekable();
    while let Some(ch) = it.next() {
        match (q, ch) {
            (true, '"') if it.peek() == Some(&'"') => { cur.push('"'); it.next(); }
            (true, '"') => q = false,
            (false, '"') if cur.is_empty() => q = true,
            (false, ',') => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    assert!(!q, "unterminated quote");
    out.push(cur);
    out
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: p33_native RAW_CSV");
    let text = std::fs::read_to_string(&path).expect("read csv");
    let mut lines = text.lines();
    let header = split_csv(lines.next().expect("header"));
    let col = |name: &str| header.iter().position(|h| h == name).unwrap_or_else(|| panic!("no column {name}"));
    let (cp, cb) = (col("p"), col("p_bits"));
    let (c2, c3, cu) = (col("power2_norm"), col("power3_norm"), col("unconstrained_norm"));
    let (c2c, c3c) = (col("power2_censored"), col("power3_censored"));

    // bits -> (n2/p, n3/p, nu/sqrt p, censored2, censored3)
    let mut g: BTreeMap<u32, (Vec<f64>, Vec<f64>, Vec<f64>, u32, u32)> = BTreeMap::new();
    // Sensitivity: bits -> N2/p over every row with power2_norm present.
    let mut g2all: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut skipped_rows: Vec<String> = Vec::new();
    let (mut rows, mut skipped) = (0u32, 0u32);
    for (i, line) in lines.enumerate() {
        if line.is_empty() { continue; }
        let f = split_csv(line);
        assert_eq!(f.len(), header.len(), "row {} has {} fields, header {}", i + 2, f.len(), header.len());
        rows += 1;
        if !f[c2].is_empty() {
            let p: u64 = f[cp].parse().expect("p");
            let bits: u32 = f[cb].parse().expect("p_bits");
            g2all.entry(bits).or_default().push(f[c2].parse::<u128>().expect("power2_norm") as f64 / p as f64);
        }
        if f[c2].is_empty() || f[c3].is_empty() || f[cu].is_empty() {
            skipped += 1;
            skipped_rows.push(format!("row {} bits {} power2 '{}' power3 '{}' censored2 {} censored3 {}", i + 2, f[cb], f[c2], f[c3], f[c2c], f[c3c]));
            continue;
        }
        let p: u64 = f[cp].parse().expect("p");
        let bits: u32 = f[cb].parse().expect("p_bits");
        assert_eq!(64 - p.leading_zeros(), bits, "p_bits disagrees with p on row {}", i + 2);
        let n2: u128 = f[c2].parse().expect("power2_norm");
        let n3: u128 = f[c3].parse().expect("power3_norm");
        let nu: u128 = f[cu].parse().expect("unconstrained_norm");
        assert!(n2.is_power_of_two(), "power2_norm not a power of 2, row {}", i + 2);
        let mut t = n3; while t % 3 == 0 && t > 1 { t /= 3; } assert_eq!(t, 1, "power3_norm not a power of 3, row {}", i + 2);
        let e = g.entry(bits).or_default();
        e.0.push(n2 as f64 / p as f64);
        e.1.push(n3 as f64 / p as f64);
        e.2.push(nu as f64 / (p as f64).sqrt());
        e.3 += (f[c2c] == "1") as u32;
        e.4 += (f[c3c] == "1") as u32;
    }
    println!("rows {rows} used {} skipped {skipped}", rows - skipped);
    println!("bits,instances,median_N2_over_p,median_N3_over_p,median_Nunc_over_sqrtp,min_N2_over_p,max_N2_over_p,censored2,censored3");
    for (bits, (mut a, mut b, mut c, k2, k3)) in g {
        let n = a.len();
        let (lo, hi) = a.iter().fold((f64::MAX, f64::MIN), |(l, h), &x| (l.min(x), h.max(x)));
        println!("{bits},{n},{:.3},{:.3},{:.3},{:.3},{:.3},{k2},{k3}", median(&mut a), median(&mut b), median(&mut c), lo, hi);
    }
    println!("skipped rows:");
    for r in &skipped_rows { println!("  {r}"); }
    println!("sensitivity: median N2/p over all rows with power2_norm present");
    println!("bits,instances,median_N2_over_p");
    for (bits, mut v) in g2all { let n = v.len(); println!("{bits},{n},{:.3}", median(&mut v)); }
}
