//! Independent recount of the split / ramified / inert pattern of small primes
//! in the Frobenius field of the ten published NIST / X9.62 prime curves.
//!
//! Dependency-free (plain `rustc -O`). Everything is computed modulo each odd
//! prime ℓ ≤ 200000 from the hex parameters p and n (OpenSSL `ecparam`), so no
//! big-integer library is needed: t ≡ p + 1 − n and Δ ≡ t² − 4p (mod ℓ).
//!
//! Baseline (the uploaded report's model, re-derived here): Frobenius mod ℓ is
//! uniform among matrices in GL2(F_ℓ) of determinant p. The number of such
//! matrices with trace τ is ℓ² + ℓ·χ(τ² − 4p) out of ℓ(ℓ² − 1), so
//!   P(split)    = (ℓ − 2 − χ(p)) / (2(ℓ − 1)),
//!   P(ramified) = (1 + χ(p)) ℓ / (ℓ² − 1),
//!   P(inert)    = 1 − P(split) − P(ramified).
//! Conditioning on ℓ ∤ n removes the trace class τ ≡ p + 1 (discriminant
//! (p − 1)², split when p ≢ 1, ramified when p ≡ 1 mod ℓ) and renormalises.
//!
//! Output: JSON on stdout with, per curve and cutoff, the observed split count,
//! the conditioned expectation, its standard deviation and z, plus the partial
//! Euler product for L(1, χ) over odd ℓ ≤ cutoff and its ratio to the model's
//! expected value exp(E[log ∏]).

use std::fmt::Write as _;

struct Curve {
    name: &'static str,
    p_hex: &'static str,
    n_hex: &'static str,
}

// Parameters printed by `openssl ecparam -name <curve> -param_enc explicit -text -noout`
// (OpenSSL 3.0.13), colons removed. Cofactor 1 for every curve.
const CURVES: &[Curve] = &[
    Curve { name: "prime192v1 (P-192)", p_hex: "fffffffffffffffffffffffffffffffeffffffffffffffff", n_hex: "ffffffffffffffffffffffff99def836146bc9b1b4d22831" },
    Curve { name: "prime192v2", p_hex: "fffffffffffffffffffffffffffffffeffffffffffffffff", n_hex: "fffffffffffffffffffffffe5fb1a724dc80418648d8dd31" },
    Curve { name: "prime192v3", p_hex: "fffffffffffffffffffffffffffffffeffffffffffffffff", n_hex: "ffffffffffffffffffffffff7a62d031c83f4294f640ec13" },
    Curve { name: "secp224r1 (P-224)", p_hex: "ffffffffffffffffffffffffffffffff000000000000000000000001", n_hex: "ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d" },
    Curve { name: "prime239v1", p_hex: "7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff", n_hex: "7fffffffffffffffffffffff7fffff9e5e9a9f5d9071fbd1522688909d0b" },
    Curve { name: "prime239v2", p_hex: "7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff", n_hex: "7fffffffffffffffffffffff800000cfa7e8594377d414c03821bc582063" },
    Curve { name: "prime239v3", p_hex: "7fffffffffffffffffffffff7fffffffffff8000000000007fffffffffff", n_hex: "7fffffffffffffffffffffff7fffff975deb41b3a6057c3c432146526551" },
    Curve { name: "prime256v1 (P-256)", p_hex: "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff", n_hex: "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551" },
    Curve { name: "secp384r1 (P-384)", p_hex: "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff", n_hex: "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973" },
    Curve { name: "secp521r1 (P-521)", p_hex: "1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", n_hex: "1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebb6fb71e91386409" },
];

const CUTOFFS: &[u64] = &[100, 200, 500, 691, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000];

fn hex_mod(hex: &str, m: u64) -> u64 {
    let mut acc: u128 = 0;
    for c in hex.chars() {
        let d = c.to_digit(16).expect("hex digit") as u128;
        acc = (acc * 16 + d) % (m as u128);
    }
    acc as u64
}

fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r: u128 = 1;
    let mut bb = (b % m) as u128;
    b = 0; let _ = b;
    while e > 0 {
        if e & 1 == 1 { r = r * bb % (m as u128); }
        bb = bb * bb % (m as u128);
        e >>= 1;
    }
    r as u64
}

/// Legendre symbol (a / ℓ) for an odd prime ℓ, returned as −1, 0, +1.
fn legendre(a: u64, l: u64) -> i32 {
    let a = a % l;
    if a == 0 { return 0; }
    let e = pow_mod(a, (l - 1) / 2, l);
    if e == 1 { 1 } else if e == l - 1 { -1 } else { panic!("ℓ = {l} is not prime") }
}

fn primes_up_to(n: usize) -> Vec<u64> {
    let mut s = vec![true; n + 1];
    s[0] = false; if n >= 1 { s[1] = false; }
    let mut i = 2;
    while i * i <= n { if s[i] { let mut j = i * i; while j <= n { s[j] = false; j += i; } } i += 1; }
    (2..=n).filter(|&k| s[k]).map(|k| k as u64).collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind { Split, Ramified, Inert }

struct PerPrime {
    l: u64,
    kind: Kind,
    p_split: f64,    // conditioned on ℓ ∤ n
    p_ram: f64,
    p_inert: f64,
    divides_n: bool,
}

fn classify(c: &Curve, l: u64) -> PerPrime {
    let lf = l as f64;
    let p = hex_mod(c.p_hex, l);
    let n = hex_mod(c.n_hex, l);
    let t = (p + 1 + l - n) % l;                 // t = p + 1 − n
    let disc = ((t * t) % l + 4 * l - (4 * p) % l) % l; // Δ = t² − 4p
    let chi_d = legendre(disc, l);
    let kind = match chi_d { 1 => Kind::Split, 0 => Kind::Ramified, _ => Kind::Inert };
    let chi_p = legendre(p, l) as f64;
    // unconditioned
    let ps = (lf - 2.0 - chi_p) / (2.0 * (lf - 1.0));
    let pr = (1.0 + chi_p) * lf / (lf * lf - 1.0);
    // the removed trace class τ ≡ p + 1: discriminant (p − 1)²
    let p_is_one = p % l == 1;
    let m = (lf + if p_is_one { 0.0 } else { 1.0 }) / (lf * lf - 1.0);
    let (ps_c, pr_c) = if p_is_one {
        (ps / (1.0 - m), (pr - m) / (1.0 - m))
    } else {
        ((ps - m) / (1.0 - m), pr / (1.0 - m))
    };
    let pi_c = 1.0 - ps_c - pr_c;
    PerPrime { l, kind, p_split: ps_c, p_ram: pr_c, p_inert: pi_c, divides_n: n == 0 }
}

fn main() {
    let primes = primes_up_to(200000);
    let odd: Vec<u64> = primes.iter().copied().filter(|&l| l > 2).collect();
    let mut out = String::new();
    out.push_str("{\n  \"schema\": \"split-prime-recount/v1\",\n");
    out.push_str("  \"baseline\": \"GL2(F_l) uniform with det p, conditioned on l not dividing n; see tools/splitcount.rs\",\n");
    out.push_str("  \"cutoffs\": [");
    for (i, c) in CUTOFFS.iter().enumerate() { if i > 0 { out.push_str(", "); } let _ = write!(out, "{c}"); }
    out.push_str("],\n  \"curves\": [\n");

    // pooled accumulators per cutoff
    let mut pooled_obs = vec![0.0f64; CUTOFFS.len()];
    let mut pooled_exp = vec![0.0f64; CUTOFFS.len()];
    let mut pooled_var = vec![0.0f64; CUTOFFS.len()];
    let mut pooled_zsum = vec![0.0f64; CUTOFFS.len()];

    for (ci, c) in CURVES.iter().enumerate() {
        let rows: Vec<PerPrime> = odd.iter().map(|&l| classify(c, l)).collect();
        let any_div = rows.iter().any(|r| r.divides_n);
        let _ = write!(out, "    {{\"name\": \"{}\", \"some_l_divides_n\": {}, \"cutoffs\": [\n", c.name, any_div);
        for (k, &cut) in CUTOFFS.iter().enumerate() {
            let (mut obs_s, mut obs_r, mut obs_i, mut exp_s, mut exp_r, mut var_s, mut log_l, mut exp_log_l, mut count) =
                (0u64, 0u64, 0u64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0u64);
            for r in rows.iter().filter(|r| r.l <= cut && !r.divides_n) {
                count += 1;
                match r.kind { Kind::Split => obs_s += 1, Kind::Ramified => obs_r += 1, Kind::Inert => obs_i += 1 }
                exp_s += r.p_split;
                exp_r += r.p_ram;
                var_s += r.p_split * (1.0 - r.p_split);
                let lf = r.l as f64;
                let chi = match r.kind { Kind::Split => 1.0, Kind::Ramified => 0.0, Kind::Inert => -1.0 };
                log_l += -(1.0 - chi / lf).ln();
                exp_log_l += r.p_split * (-(1.0 - 1.0 / lf).ln()) + r.p_inert * (-(1.0 + 1.0 / lf).ln());
                // ramified contributes log 1 = 0
            }
            let sd = var_s.sqrt();
            let z = (obs_s as f64 - exp_s) / sd;
            pooled_obs[k] += obs_s as f64; pooled_exp[k] += exp_s; pooled_var[k] += var_s; pooled_zsum[k] += z;
            let _ = write!(out,
                "      {{\"cutoff\": {cut}, \"primes\": {count}, \"split\": {obs_s}, \"ramified\": {obs_r}, \"inert\": {obs_i}, \"expected_split\": {exp_s:.4}, \"expected_ramified\": {exp_r:.4}, \"sd\": {sd:.4}, \"z\": {z:.3}, \"partial_L1\": {:.5}, \"model_expected_L1\": {:.5}, \"L1_ratio\": {:.4}}}{}\n",
                log_l.exp(), exp_log_l.exp(), (log_l - exp_log_l).exp(), if k + 1 < CUTOFFS.len() { "," } else { "" });
        }
        let _ = write!(out, "    ]}}{}\n", if ci + 1 < CURVES.len() { "," } else { "" });
    }
    out.push_str("  ],\n  \"pooled\": [\n");
    for (k, &cut) in CUTOFFS.iter().enumerate() {
        let z_sum = (pooled_obs[k] - pooled_exp[k]) / pooled_var[k].sqrt();
        let z_mean = pooled_zsum[k] / (CURVES.len() as f64).sqrt();
        let _ = write!(out, "    {{\"cutoff\": {cut}, \"split\": {}, \"expected_split\": {:.3}, \"sd\": {:.3}, \"z_pooled_counts\": {z_sum:.3}, \"z_stouffer\": {z_mean:.3}}}{}\n",
            pooled_obs[k] as u64, pooled_exp[k], pooled_var[k].sqrt(), if k + 1 < CUTOFFS.len() { "," } else { "" });
    }
    out.push_str("  ]\n}\n");
    print!("{out}");
}
