//! Which weighting of the split primes separates the published from the skipped
//! curves? Each of the three attack rationales for a split-prime skip rule
//! implies a different weight w(ℓ) on a split prime ℓ:
//!
//!   uniform      w = 1                 class-group relation collection (one
//!                                      generator / one column per split ℓ);
//!   log          w = ln ℓ              CRT-style per-ℓ information (bits of
//!                                      Frobenius data, as in SEA);
//!   inv          w = 1/ℓ               smoothness yield of a split ℓ in a
//!                                      random norm (divides with prob ≈ 2/ℓ);
//!   invsqrt      w = 1/√ℓ              the per-ℓ cost of an Elkies step
//!                                      scaled away (middle ground);
//!   mazur        w = [ℓ ∈ M]           rational ℓ-isogenies over Q exist only
//!                                      for ℓ ∈ M = {2,3,5,7,11,13,17,19,37,43,67,163}
//!                                      (Mazur 1978, Kenku 1982): lifting
//!                                      rationales can only use these;
//!   le163        w = [ℓ ≤ 163]         the same bound without the gaps;
//!   gt163        w = [ℓ > 163]         the complement: what the lifting
//!                                      rationale cannot see;
//!   inclusive    w = 1 on χ(ℓ) ≥ 0     ramified primes counted too (the
//!                                      earlier study's sensitivity family).
//!
//! For every weight and every odd-prime cutoff B ≤ 2000 the tool forms
//! W_B(E) = Σ_{odd ℓ ≤ B, χ(ℓ) = +1} w(ℓ) (χ ≥ 0 for `inclusive`) and asks
//! whether, in every field that has both published and skipped curves
//! (P-192, P-224, P-239, P-256), max over published < min over skipped
//! ("lower" direction) or the reverse ("upper"). It reports the cutoffs at
//! which a weight separates, the margins at B = 691, and a per-band profile of
//! split fractions pooled over published and over skipped curves.
//!
//! Dependency-free Rust (`rustc -O`). Everything is reduced modulo ℓ, so no
//! big-integer arithmetic is needed. Input: the lab's census
//! `research/nist_skip_criterion_20261025/inputs.tsv` (21 rows: label, bits,
//! role, run, index, p, b, n, seed; hex with 0x prefix). Output: JSON on stdout.
//!
//! Usage: weightings <inputs.tsv>

use std::fmt::Write as _;

const MAZUR_ODD: &[u64] = &[3, 5, 7, 11, 13, 17, 19, 37, 43, 67, 163];
const LMAX: u64 = 2000;
const BANDS: &[(u64, u64)] = &[(3, 50), (51, 163), (164, 400), (401, 691), (692, 2000)];

#[derive(Clone)]
struct Curve {
    label: String,
    bits: u32,
    role: String,
    p_hex: String,
    n_hex: String,
}

fn hex_mod(hex: &str, m: u64) -> u64 {
    let h = hex.trim_start_matches("0x").trim_start_matches("0X");
    let mut acc: u128 = 0;
    for c in h.chars() {
        let d = c.to_digit(16).expect("hex digit") as u128;
        acc = (acc * 16 + d) % (m as u128);
    }
    acc as u64
}

fn pow_mod(b: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u128 = 1;
    let mut bb = (b % m) as u128;
    let mm = m as u128;
    while e > 0 {
        if e & 1 == 1 {
            r = r * bb % mm;
        }
        bb = bb * bb % mm;
        e >>= 1;
    }
    r as u64
}

/// Legendre symbol (a/ℓ) for odd prime ℓ: +1, 0, −1.
fn legendre(a: u64, l: u64) -> i8 {
    let a = a % l;
    if a == 0 {
        return 0;
    }
    let e = pow_mod(a, (l - 1) / 2, l);
    if e == 1 {
        1
    } else {
        -1
    }
}

fn odd_primes_upto(n: u64) -> Vec<u64> {
    let n = n as usize;
    let mut sieve = vec![true; n + 1];
    let mut out = Vec::new();
    for i in 2..=n {
        if sieve[i] {
            if i > 2 {
                out.push(i as u64);
            }
            let mut j = i * i;
            while j <= n {
                sieve[j] = false;
                j += i;
            }
        }
    }
    out
}

/// χ(ℓ) = ((t² − 4p) / ℓ) with t ≡ p + 1 − n (mod ℓ). Also flags ℓ | n.
fn chi(c: &Curve, l: u64) -> (i8, bool) {
    let p = hex_mod(&c.p_hex, l);
    let n = hex_mod(&c.n_hex, l);
    let t = (p + 1 + l - n) % l;
    let disc = ((t * t) % l + l - (4 * p) % l) % l;
    (legendre(disc, l), n == 0)
}

fn parse_tsv(path: &str) -> Vec<Curve> {
    let text = std::fs::read_to_string(path).expect("read inputs.tsv");
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        assert!(f.len() >= 8, "row needs 8 columns: {line}");
        out.push(Curve {
            label: f[0].to_string(),
            bits: f[1].parse().expect("bits"),
            role: f[2].to_string(),
            p_hex: f[5].to_string(),
            n_hex: f[7].to_string(),
        });
    }
    out
}

struct Weight {
    name: &'static str,
    rationale: &'static str,
    inclusive: bool,
    f: fn(u64) -> f64,
}

fn weights() -> Vec<Weight> {
    vec![
        Weight { name: "uniform", rationale: "class-group relations: one generator per split prime", inclusive: false, f: |_| 1.0 },
        Weight { name: "log", rationale: "CRT information: ln(l) bits of Frobenius data per split prime", inclusive: false, f: |l| (l as f64).ln() },
        Weight { name: "inv", rationale: "smoothness yield: a split l divides a random norm with probability about 2/l", inclusive: false, f: |l| 1.0 / l as f64 },
        Weight { name: "invsqrt", rationale: "intermediate decay 1/sqrt(l)", inclusive: false, f: |l| 1.0 / (l as f64).sqrt() },
        Weight { name: "mazur", rationale: "lifting: rational l-isogenies over Q exist only for l in {2,3,5,7,11,13,17,19,37,43,67,163}", inclusive: false, f: |l| if MAZUR_ODD.contains(&l) { 1.0 } else { 0.0 } },
        Weight { name: "le163", rationale: "lifting bound without the gaps: l <= 163", inclusive: false, f: |l| if l <= 163 { 1.0 } else { 0.0 } },
        Weight { name: "gt163", rationale: "complement of the lifting range: 163 < l", inclusive: false, f: |l| if l > 163 { 1.0 } else { 0.0 } },
        Weight { name: "inclusive", rationale: "uniform on chi(l) >= 0: ramified primes counted too", inclusive: true, f: |_| 1.0 },
    ]
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn fmt_f(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e12 {
        format!("{}", x as i64)
    } else {
        format!("{:.6}", x)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: weightings <inputs.tsv>");
        std::process::exit(2);
    }
    let curves = parse_tsv(&args[1]);
    let primes = odd_primes_upto(LMAX);

    // chi table: curve × prime
    let mut table: Vec<Vec<i8>> = Vec::new();
    let mut divides_n = Vec::new();
    for c in &curves {
        let mut row = Vec::with_capacity(primes.len());
        for &l in &primes {
            let (x, dn) = chi(c, l);
            if dn {
                divides_n.push((c.label.clone(), l));
            }
            row.push(x);
        }
        table.push(row);
    }

    let fields: Vec<u32> = {
        let mut v: Vec<u32> = curves.iter().map(|c| c.bits).collect();
        v.sort();
        v.dedup();
        v
    };
    // fields with both roles
    let paired: Vec<u32> = fields
        .iter()
        .cloned()
        .filter(|b| {
            curves.iter().any(|c| c.bits == *b && c.role == "published")
                && curves.iter().any(|c| c.bits == *b && c.role == "skipped")
        })
        .collect();

    let ws = weights();
    let mut out = String::new();
    out.push_str("{\n");
    let _ = writeln!(out, "  \"lmax\": {LMAX},");
    let _ = writeln!(out, "  \"odd_prime_cutoffs\": {},", primes.len());
    let _ = writeln!(out, "  \"mazur_odd\": {:?},", MAZUR_ODD);
    let _ = writeln!(out, "  \"paired_fields\": {:?},", paired);
    let _ = writeln!(
        out,
        "  \"primes_dividing_n\": [{}],",
        divides_n.iter().map(|(l, p)| format!("[{}, {}]", json_str(l), p)).collect::<Vec<_>>().join(", ")
    );

    // per-curve: split set among Mazur primes, counts at 691 (control against at691.tsv), band counts
    out.push_str("  \"curves\": [\n");
    for (ci, c) in curves.iter().enumerate() {
        let row = &table[ci];
        let mut strict691 = 0;
        let mut ram691 = 0;
        let mut mazur_split = Vec::new();
        let mut mazur_ram = Vec::new();
        let mut band_split = Vec::new();
        for &(lo, hi) in BANDS {
            let mut s = 0;
            let mut tot = 0;
            for (i, &l) in primes.iter().enumerate() {
                if l >= lo && l <= hi {
                    tot += 1;
                    if row[i] == 1 {
                        s += 1;
                    }
                }
            }
            band_split.push((s, tot));
        }
        for (i, &l) in primes.iter().enumerate() {
            if l <= 691 {
                if row[i] == 1 {
                    strict691 += 1;
                }
                if row[i] == 0 {
                    ram691 += 1;
                }
            }
            if MAZUR_ODD.contains(&l) {
                if row[i] == 1 {
                    mazur_split.push(l);
                }
                if row[i] == 0 {
                    mazur_ram.push(l);
                }
            }
        }
        let mut wsum = Vec::new();
        for w in &ws {
            let mut acc = 0.0;
            for (i, &l) in primes.iter().enumerate() {
                if l <= 691 && (row[i] == 1 || (w.inclusive && row[i] == 0)) {
                    acc += (w.f)(l);
                }
            }
            wsum.push(format!("{}: {}", json_str(w.name), fmt_f(acc)));
        }
        let _ = writeln!(
            out,
            "    {{\"label\": {}, \"bits\": {}, \"role\": {}, \"strict691\": {}, \"ramified691\": {}, \"mazur_split\": {:?}, \"mazur_ramified\": {:?}, \"band_split\": [{}], \"w691\": {{{}}}}}{}",
            json_str(&c.label),
            c.bits,
            json_str(&c.role),
            strict691,
            ram691,
            mazur_split,
            mazur_ram,
            band_split.iter().map(|(s, t)| format!("[{s}, {t}]")).collect::<Vec<_>>().join(", "),
            wsum.join(", "),
            if ci + 1 < curves.len() { "," } else { "" }
        );
    }
    out.push_str("  ],\n");
    let _ = writeln!(
        out,
        "  \"bands\": [{}],",
        BANDS.iter().map(|(lo, hi)| format!("[{lo}, {hi}]")).collect::<Vec<_>>().join(", ")
    );

    // pooled band profile: split fraction over published vs skipped curves (paired fields only)
    out.push_str("  \"band_profile_paired_fields\": [\n");
    for (bi, &(lo, hi)) in BANDS.iter().enumerate() {
        let mut acc = [(0usize, 0usize), (0usize, 0usize)]; // published, skipped
        for (ci, c) in curves.iter().enumerate() {
            if !paired.contains(&c.bits) {
                continue;
            }
            let k = if c.role == "published" { 0 } else { 1 };
            for (i, &l) in primes.iter().enumerate() {
                if l >= lo && l <= hi {
                    acc[k].1 += 1;
                    if table[ci][i] == 1 {
                        acc[k].0 += 1;
                    }
                }
            }
        }
        let fp = acc[0].0 as f64 / acc[0].1.max(1) as f64;
        let fs = acc[1].0 as f64 / acc[1].1.max(1) as f64;
        // two-proportion z with the pooled fraction (cells treated as independent
        // Bernoulli trials; a descriptive yardstick, not a calibrated test, since
        // the cells of one curve share its discriminant)
        let pooled = (acc[0].0 + acc[1].0) as f64 / (acc[0].1 + acc[1].1).max(1) as f64;
        let se = (pooled * (1.0 - pooled) * (1.0 / acc[0].1.max(1) as f64 + 1.0 / acc[1].1.max(1) as f64)).sqrt();
        let z = if se > 0.0 { (fp - fs) / se } else { 0.0 };
        let _ = writeln!(
            out,
            "    {{\"band\": [{lo}, {hi}], \"published_split\": {}, \"published_cells\": {}, \"skipped_split\": {}, \"skipped_cells\": {}, \"published_fraction\": {:.4}, \"skipped_fraction\": {:.4}, \"difference\": {:.4}, \"z_two_proportion\": {:.2}}}{}",
            acc[0].0, acc[0].1, acc[1].0, acc[1].1, fp, fs, fp - fs, z,
            if bi + 1 < BANDS.len() { "," } else { "" }
        );
    }
    out.push_str("  ],\n");

    // separation sweep per weight
    out.push_str("  \"weights\": [\n");
    for (wi, w) in ws.iter().enumerate() {
        // W_B per curve, cumulative over cutoffs
        let mut cum: Vec<f64> = vec![0.0; curves.len()];
        let mut lower_cutoffs = Vec::new();
        let mut upper_cutoffs = Vec::new();
        let mut margins691 = Vec::new();
        for (i, &l) in primes.iter().enumerate() {
            for (ci, _) in curves.iter().enumerate() {
                let x = table[ci][i];
                if x == 1 || (w.inclusive && x == 0) {
                    cum[ci] += (w.f)(l);
                }
            }
            let mut lower_all = true;
            let mut upper_all = true;
            let mut margins = Vec::new();
            for &b in &paired {
                let pub_max = curves.iter().enumerate().filter(|(_, c)| c.bits == b && c.role == "published").map(|(ci, _)| cum[ci]).fold(f64::MIN, f64::max);
                let pub_min = curves.iter().enumerate().filter(|(_, c)| c.bits == b && c.role == "published").map(|(ci, _)| cum[ci]).fold(f64::MAX, f64::min);
                let skp_min = curves.iter().enumerate().filter(|(_, c)| c.bits == b && c.role == "skipped").map(|(ci, _)| cum[ci]).fold(f64::MAX, f64::min);
                let skp_max = curves.iter().enumerate().filter(|(_, c)| c.bits == b && c.role == "skipped").map(|(ci, _)| cum[ci]).fold(f64::MIN, f64::max);
                if !(pub_max < skp_min) {
                    lower_all = false;
                }
                if !(pub_min > skp_max) {
                    upper_all = false;
                }
                margins.push((b, pub_max, skp_min, skp_min - pub_max));
            }
            if lower_all {
                lower_cutoffs.push(l);
            }
            if upper_all {
                upper_cutoffs.push(l);
            }
            if l == 691 {
                margins691 = margins;
            }
        }
        let _ = writeln!(
            out,
            "    {{\"name\": {}, \"rationale\": {}, \"lower_separating_cutoffs\": {:?}, \"lower_count\": {}, \"upper_separating_cutoffs\": {:?}, \"upper_count\": {}, \"margins_at_691\": [{}]}}{}",
            json_str(w.name),
            json_str(w.rationale),
            lower_cutoffs,
            lower_cutoffs.len(),
            upper_cutoffs,
            upper_cutoffs.len(),
            margins691
                .iter()
                .map(|(b, pm, sm, d)| format!("{{\"field\": {b}, \"published_max\": {}, \"skipped_min\": {}, \"margin\": {}}}", fmt_f(*pm), fmt_f(*sm), fmt_f(*d)))
                .collect::<Vec<_>>()
                .join(", "),
            if wi + 1 < ws.len() { "," } else { "" }
        );
    }
    out.push_str("  ]\n}\n");
    print!("{out}");
}
