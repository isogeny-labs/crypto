//! Deterministic search for small **a = −3 prime-order bench curves**
//! that model the P-256 / GOST CryptoPro-B curve shape
//! (`y² = x³ − 3x + b`, prime order, h = 1).
//!
//! P-256 and GOST CryptoPro-B both use `a = p − 3`; neither has CM or a
//! non-trivial automorphism, so they are the *generic-j* prime-field IC
//! case.  This helper finds the smallest-field members of that family:
//!
//! - `p` is the largest prime below `2^bits` in the requested residue
//!   class mod 4 (`p ≡ 3 (mod 4)` matches P-256's Solinas-style shape;
//!   `p ≡ 1 (mod 4)` matches CryptoPro-B),
//! - `a = p − 3`,
//! - `b` is the smallest positive integer making `#E(F_p)` prime,
//! - `G` is the smallest-x point on the curve.
//!
//! ```bash
//! cargo run --release --example find_a3_bench_curves
//! ```

use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;

/// Exact group order for small fields by the textbook quadratic-character
/// sum.  Cost is Θ(p) modular exponentiations, so it is only used for the
/// toy rungs.
fn count_points(p: u64, a: u64, b: u64) -> u64 {
    crypto_lib::cryptanalysis::ai_schoof::count_points_small(p, a, b)
}

/// Exact group order by Hasse-interval search (Mestre's method): take a
/// point `P`, compute `(p+1)P`, and scan `k` with `kP = ±(p+1)P` for
/// `|k| ≤ 2√p`.  Then `#E` is a multiple of `ord(P)` inside the Hasse
/// interval.  The caller only accepts a *prime* candidate `m`, and that
/// makes the result exact: with `E(F_p) ≅ Z/n₁ × Z/n₂`, `n₁ | n₂`, a prime
/// `m ∈ [p+1−2√p, p+1+2√p]` divisible by `ord(P)` forces `n₂ = m` and
/// `n₁ = 1` (otherwise `#E = m² > p+1+2√p`).
///
/// Cost is Θ(√p) group operations, so 36–44-bit rungs are seconds, where
/// the Θ(p) counter needs billions.
fn count_points_hasse(p: u64, a: u64, b: u64) -> Option<u64> {
    let curve = CurveParams {
        name: "probe",
        p: BigUint::from(p),
        a: BigUint::from(a),
        b: BigUint::from(b),
        gx: BigUint::from(0u32),
        gy: BigUint::from(0u32),
        n: BigUint::from(1u32),
        h: 1,
    };
    let a_fe = curve.a_fe();

    // Smallest-x point on the curve; it generates a large subgroup with
    // overwhelming probability, and a degenerate one simply yields no
    // candidate below.
    let mut base = None;
    for x in 1..64u64 {
        let rhs = (((x as u128 * x as u128 % p as u128) * x as u128 % p as u128
            + (a as u128 * x as u128 % p as u128)
            + b as u128)
            % p as u128) as u64;
        if let Some(y) = legendre_or_sqrt(rhs, p) {
            base = Some(Point::Affine {
                x: curve.fe(BigUint::from(x)),
                y: curve.fe(BigUint::from(y)),
            });
            break;
        }
    }
    let base = base?;

    // k with k·P = (p+1)P  ⇒  #E = p+1−k ; the negative branch gives p+1+k.
    let bound = 2 * (p as f64).sqrt().ceil() as u64 + 2;
    let r = base.scalar_mul(&BigUint::from(p + 1), &a_fe);
    if let Some(k) = base.linear_dlog_vartime(&r, bound, &a_fe) {
        return (k <= p).then(|| p + 1 - k);
    }
    let neg_r = r.neg();
    if let Some(k) = base.linear_dlog_vartime(&neg_r, bound, &a_fe) {
        return Some(p + 1 + k);
    }
    None
}

fn legendre_or_sqrt(value: u64, p: u64) -> Option<u64> {
    // Tonelli–Shanks is unnecessary at these sizes; Euler + brute search
    // for the root would be slow, so use the p % 4 == 3 shortcut when
    // available and a tiny table walk otherwise.
    if p % 4 == 3 {
        let candidate = mod_pow(value, (p + 1) / 4, p);
        return if (candidate * candidate) % p == value % p && candidate != 0 {
            Some(candidate)
        } else {
            None
        };
    }
    // p % 4 == 1: brute-force the square root only for QRs (small p).
    if mod_pow(value, (p - 1) / 2, p) != 1 {
        return None;
    }
    let target = value % p;
    let mut y = 1u64;
    while y < p {
        if (y * y) % p == target {
            return Some(y);
        }
        y += 1;
    }
    None
}

fn mod_pow(base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result: u128 = 1;
    let mut b = base as u128 % m as u128;
    let m_u = m as u128;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * b) % m_u;
        }
        b = (b * b) % m_u;
        exp >>= 1;
    }
    result as u64
}

fn find_curve(bits: u32, residue_mod_4: u64) -> Option<(u64, u64, u64, u64, u64, u64)> {
    // Largest prime < 2^bits in the residue class.
    let mut p = (1u64 << bits) - 1;
    loop {
        if p % 4 == residue_mod_4 && is_prime_u64(p) {
            break;
        }
        p -= 1;
        if p < 16 {
            return None;
        }
    }
    let a = p - 3;
    let mut b = 1u64;
    loop {
        // Skip singular curves (4a^3 + 27b^2 == 0 mod p).
        if !(4 * (a as u128) * (a as u128) * (a as u128) + 27 * (b as u128) * (b as u128))
            .is_multiple_of(p as u128)
        {
            // Θ(p) counting is only affordable on the toy rungs.
            let order = if bits <= 24 {
                Some(count_points(p, a, b))
            } else {
                count_points_hasse(p, a, b)
            };
            if let Some(order) = order {
                if is_prime_u64(order) {
                    // Generator: smallest x >= 1 with a QR rhs.
                    for x in 1..p {
                        let rhs = (((x as u128 * x as u128 % p as u128) * x as u128 % p as u128
                            + (a as u128 * x as u128 % p as u128)
                            + b as u128)
                            % p as u128) as u64;
                        if let Some(y) = legendre_or_sqrt(rhs, p) {
                            // (x, y) is a generator: the group has prime order
                            // and (x, y) is not the identity.
                            return Some((p, a, b, x, y, order));
                        }
                    }
                }
            }
        }
        b += 1;
        if b > 2000 {
            return None;
        }
    }
}

fn main() {
    // Optional `bits` arguments extend the default ladder; a bare run
    // reproduces the published rungs.
    let requested: Vec<u32> = std::env::args()
        .skip(1)
        .filter_map(|a| a.parse().ok())
        .collect();
    let mut rungs: Vec<(u32, u64, &str)> = Vec::new();
    if requested.is_empty() {
        rungs.extend([
            (16u32, 3u64, "p256class"),
            (20, 3, "p256class"),
            (20, 1, "cryptoproclass"),
            (24, 3, "p256class"),
            (24, 1, "cryptoproclass"),
            (28, 3, "p256class"),
            (28, 1, "cryptoproclass"),
        ]);
    } else {
        // `p ≡ 3 (mod 4)` uses the fast Euler square root; the `p ≡ 1`
        // branch brute-forces the root and is toy-rung only.
        for bits in requested {
            let residue = if bits <= 24 { 1 } else { 3 };
            rungs.push((
                bits,
                residue,
                if residue == 3 {
                    "p256class"
                } else {
                    "cryptoproclass"
                },
            ));
            if bits <= 24 {
                rungs.push((bits, 3, "p256class"));
            }
        }
    }
    for (bits, residue, label) in rungs {
        match find_curve(bits, residue) {
            Some((p, a, b, gx, gy, order)) => {
                println!(
                    "{}",
                    serde_json::json!({
                        "bits": bits,
                        "class": label,
                        "p": p.to_string(),
                        "p_mod_4": p % 4,
                        "a": a.to_string(),
                        "a_is_p_minus_3": a == p - 3,
                        "b": b.to_string(),
                        "gx": gx.to_string(),
                        "gy": gy.to_string(),
                        "n": order.to_string(),
                        "h": 1,
                    })
                );
            }
            None => eprintln!("{bits}-bit residue {residue}: no curve found"),
        }
    }
}
