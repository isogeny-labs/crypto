//! The arithmetic of the isogeny class: `D = t² − 4q = f²·D_K`, the class
//! numbers `h(d²D_K)` of every order between `Z[π]` and `O_K`, and hence
//! the exact number of `F_q`-isomorphism classes in the isogeny class,
//! `Σ_{d | f} h(d²D_K)`.  `h(D_K)` is counted from the reduced primitive
//! forms; the other class numbers follow from the conductor formula.

use serde_json::json;

use super::field::{factor, gcd};

/// `(D_K, f)` with `−d_abs = f²·D_K`, `D_K` fundamental.
pub fn fundamental(d_abs: u64) -> (i64, u64) {
    assert!(
        d_abs <= i64::MAX as u64,
        "class-number pipeline requires |D| <= i64::MAX"
    );
    let (mut core, mut f) = (1u64, 1u64);
    for (q, e) in factor(d_abs) {
        core *= q.pow(e % 2);
        f *= q.pow(e / 2);
    }
    let d0 = -(core as i64);
    if d0.rem_euclid(4) == 1 {
        (d0, f)
    } else {
        (4 * d0, f / 2)
    }
}

/// Kronecker symbol `(D/ℓ)` for a prime `ℓ`.
pub fn kronecker(d: i64, ell: u64) -> i32 {
    if ell == 2 {
        if d % 2 == 0 {
            return 0;
        }
        return match d.rem_euclid(8) {
            1 | 7 => 1,
            _ => -1,
        };
    }
    let r = d.rem_euclid(ell as i64) as u64;
    if r == 0 {
        return 0;
    }
    if super::field::powmod(r, (ell - 1) / 2, ell) == 1 {
        1
    } else {
        -1
    }
}

/// Exact class number `h(D)` of the discriminant `D < 0` by counting the
/// reduced primitive forms `(a, b, c)`: `|b| ≤ a ≤ c`, `b ≥ 0` when
/// `|b| = a` or `a = c`, `gcd(a, b, c) = 1`, `b² − 4ac = D`.
pub fn class_number(d: i64) -> u64 {
    assert!(d < 0 && (d.rem_euclid(4) == 0 || d.rem_euclid(4) == 1));
    let d_abs = (-d) as u64;
    let bmax = ((d_abs as f64 / 3.0).sqrt() as u64) + 1;
    let mut h = 0u64;
    let mut b = if d_abs.is_multiple_of(2) { 0u64 } else { 1u64 };
    while b <= bmax {
        let n = (b * b + d_abs) / 4; // = ac
        if n == 0 {
            b += 2;
            continue;
        }
        for a in divisors(n) {
            if a < b || a * a > n {
                continue;
            }
            let c = n / a;
            if a > c {
                continue;
            }
            if gcd(gcd(a, b), c) != 1 {
                continue;
            }
            h += if b == 0 || b == a || a == c { 1 } else { 2 };
        }
        b += 2;
    }
    h
}

fn divisors(n: u64) -> Vec<u64> {
    let mut ds = vec![1u64];
    for (q, e) in factor(n) {
        let cur = ds.clone();
        let mut pw = 1u64;
        for _ in 0..e {
            pw *= q;
            ds.extend(cur.iter().map(|d| d * pw));
        }
    }
    ds
}

/// Unit index `[O_K^* : O^*]` for the order of conductor `d > 1`.
fn unit_index(dk: i64, d: u64) -> u64 {
    if d == 1 {
        1
    } else if dk == -3 {
        3
    } else if dk == -4 {
        2
    } else {
        1
    }
}

/// `h(d²D_K)` from `h(D_K)` by the conductor formula.
pub fn class_number_order(h_k: u64, dk: i64, d: u64) -> u64 {
    // h(d²D_K) = h(D_K)·d/[O_K^*:O^*]·∏_{ℓ|d} (1 − (D_K/ℓ)/ℓ)
    let mut num = h_k as u128 * d as u128;
    let mut den = unit_index(dk, d) as u128;
    for (q, _) in factor(d) {
        let chi = kronecker(dk, q);
        num *= (q as i128 - chi as i128) as u128;
        den *= q as u128;
    }
    assert!(
        num.is_multiple_of(den),
        "class number formula is not integral"
    );
    (num / den) as u64
}

pub fn divisors_sorted(n: u64) -> Vec<u64> {
    let mut d = divisors(n);
    d.sort();
    d
}

/// The isogeny class: `D`, `D_K`, `f`, `h(D_K)`, every stratum's class
/// number, their sum, and the per-prime volcano depths and crater kinds.
pub fn class_summary(t: i64, q: u64, d_abs: u64, h_full_check: bool) -> serde_json::Value {
    assert!(
        d_abs <= i64::MAX as u64,
        "class-number pipeline requires |D| <= i64::MAX"
    );
    let (dk, f) = fundamental(d_abs);
    let h_k = class_number(dk);
    let strata: Vec<serde_json::Value> = divisors_sorted(f)
        .iter()
        .map(|&d| {
            json!({
                "conductor": d,
                "disc": dk * (d as i64) * (d as i64),
                "class_number": class_number_order(h_k, dk, d),
            })
        })
        .collect();
    let total: u64 = strata
        .iter()
        .map(|s| s["class_number"].as_u64().unwrap())
        .sum();
    let h_full = if h_full_check && d_abs < 1 << 44 {
        Some(class_number(-(d_abs as i64)))
    } else {
        None
    };
    let primes: Vec<serde_json::Value> = factor(f)
        .iter()
        .map(|&(ell, e)| json!({"ell": ell, "depth": e, "crater": crater_kind(kronecker(dk, ell))}))
        .collect();
    json!({
        "trace": t, "q": q, "disc": -(d_abs as i64), "fundamental_disc": dk,
        "conductor": f, "conductor_factored": factor(f),
        "h_fundamental": h_k, "strata": strata, "class_size": total,
        "h_full_order_counted": h_full,
        "h_full_order_formula": class_number_order(h_k, dk, f),
        "volcano_primes": primes,
    })
}

pub fn crater_kind(r: i32) -> &'static str {
    match r {
        1 => "split",
        -1 => "inert",
        _ => "ramified",
    }
}

/// Trace of Frobenius of the Koblitz curve `y² + xy = x³ + ax² + 1` over
/// `F_{2^m}`: `t_m` with `t_0 = 2`, `t_1 = μ`, `t_{k+1} = μt_k − 2t_{k−1}`,
/// `μ = (−1)^{1−a}`.
pub fn koblitz_trace(a: u64, m: u32) -> i128 {
    let mu: i128 = if a == 1 { 1 } else { -1 };
    let (mut t0, mut t1) = (2i128, mu);
    for _ in 1..m {
        (t0, t1) = (t1, mu * t1 - 2 * t0);
    }
    if m == 0 {
        2
    } else {
        t1
    }
}
