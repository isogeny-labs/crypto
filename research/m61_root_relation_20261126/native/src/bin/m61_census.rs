//! Exact M61 b=40 curve and root-factor-base census; no relation search.

use ecdlp_pieces::{Curve, Point, P, R, SELECTED};

const N: u64 = 2_305_843_009_662_956_113;
const ROOT_ORDER: u64 = 4_950;
const ROOT_GENERATOR: u64 = 636_328_216_386_749_903;
const FULL_GENERATOR: Point = Point::Affine {
    x: 0,
    y: 313_662_643_971_843_112,
};

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    base %= modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base, modulus);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul_mod(base, base, modulus);
        }
    }
    result
}

/// Deterministic Miller--Rabin on all u64 values, using the established
/// seven-base set {2,325,9375,28178,450775,9780504,1795265022}.
fn is_prime_u64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for prime in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == prime {
            return true;
        }
        if n.is_multiple_of(prime) {
            return false;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    for base in [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
        let base = base % n;
        if base == 0 {
            continue;
        }
        let mut x = pow_mod(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut witness = true;
        for _ in 1..s {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                witness = false;
                break;
            }
        }
        if witness {
            return false;
        }
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut low = 0_u64;
    let mut high = 1_u64 << 32;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if (middle as u128) * (middle as u128) <= n as u128 {
            low = middle;
        } else {
            high = middle;
        }
    }
    low
}

fn binomial(n: usize, k: usize) -> u128 {
    let mut value = 1_u128;
    for i in 1..=k {
        value = value.checked_mul((n + 1 - i) as u128).unwrap() / i as u128;
    }
    value
}

fn curve_rhs(x: u64) -> u64 {
    let x2 = mul_mod(x, x, P);
    let x3 = mul_mod(x2, x, P);
    let three_x = mul_mod(3, x, P);
    let difference = if x3 >= three_x {
        x3 - three_x
    } else {
        P - (three_x - x3)
    };
    let sum = difference + 40;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}

struct Census {
    class_counts: [u64; 7],
    signed_counts: [[u128; 7]; 8],
    base_size: usize,
    hasse_half_width: u64,
}

fn census(curve: Curve) -> Census {
    assert!(is_prime_u64(P));
    assert!(is_prime_u64(R));
    assert_eq!(N, 7 * R);
    assert!(curve.is_on_curve(FULL_GENERATOR));
    assert_eq!(curve.mul(N, FULL_GENERATOR).unwrap(), Point::Infinity);
    assert_ne!(curve.mul(R, FULL_GENERATOR).unwrap(), Point::Infinity);
    assert_ne!(curve.mul(7, FULL_GENERATOR).unwrap(), Point::Infinity);

    // Hasse plus a point of exact order N proves #E(F_p)=N: the Hasse
    // interval contains no other multiple of N.
    let half_width = isqrt(4 * P);
    let trace = P as i128 + 1 - N as i128;
    assert!(trace.unsigned_abs() <= half_width as u128);
    assert!(2 * half_width < N);

    // 4950=2*3^2*5^2*11. A root of x^4950=1 has exact order 4950 if
    // no quotient by one of its distinct prime factors still annihilates it.
    assert_eq!((P - 1) % ROOT_ORDER, 0);
    assert_eq!(pow_mod(ROOT_GENERATOR, ROOT_ORDER, P), 1);
    for prime in [2, 3, 5, 11] {
        assert_ne!(pow_mod(ROOT_GENERATOR, ROOT_ORDER / prime, P), 1);
    }

    let torsion_generator = curve.mul(R, FULL_GENERATOR).unwrap();
    let mut torsion = [Point::Infinity; 7];
    for (j, slot) in torsion.iter_mut().enumerate() {
        *slot = curve.mul(j as u64, torsion_generator).unwrap();
    }
    for i in 0..7 {
        for j in 0..i {
            assert_ne!(torsion[i], torsion[j]);
        }
    }

    let mut classes = Vec::new();
    let mut x = 1_u64;
    for _ in 0..ROOT_ORDER {
        let rhs = curve_rhs(x);
        if rhs == 0 || pow_mod(rhs, (P - 1) / 2, P) == 1 {
            let root = pow_mod(rhs, (P + 1) / 4, P); // P = 3 (mod 4)
            assert_eq!(mul_mod(root, root, P), rhs);
            let y = root.min(P - root);
            let point = Point::Affine { x, y };
            assert!(curve.is_on_curve(point));
            let component = curve.mul(R, point).unwrap();
            let class = torsion.iter().position(|&item| item == component).unwrap();
            classes.push(class);
        }
        x = mul_mod(x, ROOT_GENERATOR, P);
    }
    assert_eq!(x, 1);
    assert_eq!(classes.len(), 2_447);

    let mut class_counts = [0_u64; 7];
    let mut signed_counts = [[0_u128; 7]; 8];
    signed_counts[0][0] = 1;
    for (i, &class) in classes.iter().enumerate() {
        class_counts[class] += 1;
        // Descend in k: each x-column is selected at most once.
        for k in (1..=usize::min(i + 1, 7)).rev() {
            for h in 0..7 {
                let extra = signed_counts[k - 1][(h + 7 - class) % 7]
                    .checked_add(signed_counts[k - 1][(h + class) % 7])
                    .expect("u128 tuple count overflow");
                signed_counts[k][h] = signed_counts[k][h]
                    .checked_add(extra)
                    .expect("u128 tuple count overflow");
            }
        }
    }
    for k in [6, 7] {
        let total: u128 = signed_counts[k].iter().sum();
        assert_eq!(total, (1_u128 << k) * binomial(classes.len(), k));
    }
    Census {
        class_counts,
        signed_counts,
        base_size: classes.len(),
        hasse_half_width: half_width,
    }
}

fn main() {
    let result = census(SELECTED);
    println!("p={P}; p prime (deterministic u64 Miller--Rabin)");
    println!(
        "N={N}=7*r; r={R} prime; point order=N; Hasse half-width={}",
        result.hasse_half_width
    );
    println!(
        "mu_4950 generator={ROOT_GENERATOR}; valid x={}",
        result.base_size
    );
    println!(
        "chosen-lift cofactor classes 0..6={:?}",
        result.class_counts
    );
    for k in [6, 7] {
        let counts = result.signed_counts[k];
        let total: u128 = counts.iter().sum();
        let class_zero = counts[0];
        let sign_quotient = if k == 7 { 2 } else { 1 };
        println!("k={k}; exact signed class counts={counts:?}");
        println!(
            "k={k}; exact total={total}; class0={class_zero}; ratio={:.12}",
            total as f64 / class_zero as f64
        );
        println!(
            "k={k}; heuristic projected={:.9}; full-equality={:.9}",
            total as f64 / (sign_quotient as f64 * R as f64),
            class_zero as f64 / (sign_quotient as f64 * R as f64)
        );
    }
    println!("Heuristic line assumes r-component uniformity within each cofactor class;");
    println!("exact class counts alone do not establish relation rank or search cost.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_primality_controls() {
        assert!(is_prime_u64(P));
        assert!(is_prime_u64(R));
        assert!(!is_prime_u64(N));
        assert!(!is_prime_u64(341_550_071_728_321));
    }

    #[test]
    fn exact_census_matches_fixed_values() {
        let result = census(SELECTED);
        assert_eq!(result.class_counts, [324, 369, 343, 339, 352, 365, 355]);
        assert_eq!(result.signed_counts[6][0], 2_709_499_351_595_025_010);
        assert_eq!(result.signed_counts[7][0], 1_889_682_261_129_892_671_314);
        assert_eq!(
            result.signed_counts[6],
            [
                2_709_499_351_595_025_010,
                2_709_499_348_773_312_186,
                2_709_499_350_084_845_738,
                2_709_499_351_223_713_027,
                2_709_499_351_223_713_027,
                2_709_499_350_084_845_738,
                2_709_499_348_773_312_186,
            ]
        );
        assert_eq!(
            result.signed_counts[7],
            [
                1_889_682_261_129_892_671_314,
                1_889_682_261_138_435_140_895,
                1_889_682_261_137_844_283_656,
                1_889_682_261_122_081_387_248,
                1_889_682_261_122_081_387_248,
                1_889_682_261_137_844_283_656,
                1_889_682_261_138_435_140_895,
            ]
        );
        assert_eq!(
            result.signed_counts[6].iter().sum::<u128>(),
            18_966_495_451_758_766_912
        );
        assert_eq!(
            result.signed_counts[7].iter().sum::<u128>(),
            13_227_775_827_926_614_294_912
        );
    }
}
