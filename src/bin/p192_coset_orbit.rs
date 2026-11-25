//! Bounded P-192 prefix evaluation for the oriented y-coset proposal.
//! This does not establish exactness of the full coset character.
//!
//! Run: cargo run --release --no-default-features --bin p192_coset_orbit -- 1000 [Qx_hex Qy_hex]

use crypto_lib::ecc::endo_generic_point::{
    scalar_mul_wnaf, textbook_mul, wnaf_digits, Affine, Curve, Jacobian, TableMode,
};
use num_bigint::BigUint;
use num_traits::One;
use std::hint::black_box;
use std::time::{Duration, Instant};

const WIDTH: u32 = 5;
const STEP: &str = "6716193708113405591269066791027723138604200469306871";
const ORDER: &str = "6277101735386680763835789423176059013767194773182842284081";
const COSET_SIZE: &str = "76517458511310880473565352";
const ODD_FACTOR: &str = "9564682313913860059195669";
const PRIME: &str = "6277101735386680763835789423207666416083908700390324961279";
const B: &str = "64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1";
const GX: &str = "188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012";
const GY: &str = "07192B95FFC8DA78631011ED6B24CDD573F977A11E794811";

fn decimal(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).expect("decimal constant")
}

fn hex(s: &str) -> BigUint {
    BigUint::parse_bytes(s.strip_prefix("0x").unwrap_or(s).as_bytes(), 16).expect("hex integer")
}

fn cpu_now() -> Duration {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts) },
        0
    );
    Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

fn step(curve: &Curve<4>, point: &Jacobian<4>, digits: &[i8]) -> Jacobian<4> {
    // The backend's width-5 table construction: 1 double and 7 full adds.
    let twice = curve.double(point);
    let mut table = Vec::with_capacity(8);
    table.push(*point);
    for _ in 1..8 {
        table.push(curve.add(table.last().unwrap(), &twice));
    }
    let mut result = curve.identity();
    for &digit in digits.iter().rev() {
        if !result.is_identity() {
            result = curve.double(&result);
        }
        if digit != 0 {
            let mut entry = table[(digit.unsigned_abs() as usize - 1) / 2];
            if digit < 0 {
                entry = curve.neg(&entry);
            }
            result = curve.add(&result, &entry);
        }
    }
    result
}

fn prefix(
    curve: &Curve<4>,
    base: &Affine<4>,
    digits: &[i8],
    count: usize,
) -> (crypto_lib::ecc::endo_generic_field::Fe<4>, Jacobian<4>) {
    let mut point = curve.from_affine(base);
    let mut product = curve.f.one();
    for index in 0..count {
        assert!(!point.is_identity());
        product = curve.f.mul(&product, &curve.f.mul(&point.y, &point.z));
        if index + 1 < count {
            point = step(curve, &point, digits);
        }
    }
    (product, point)
}

fn sign(value: &BigUint, prime: &BigUint) -> i8 {
    let result = value.modpow(&((prime - BigUint::one()) >> 1), prime);
    if result == BigUint::one() {
        1
    } else {
        -1
    }
}

fn verify(
    curve: &Curve<4>,
    base: &Affine<4>,
    digits: &[i8],
    step_scalar: &BigUint,
    order: &BigUint,
    prime: &BigUint,
    count: usize,
    actual: &Jacobian<4>,
) {
    let index = step_scalar.modpow(&BigUint::from(count - 1), order);
    let expected = textbook_mul(curve, base, &index).expect("nonidentity reference");
    assert_eq!(
        curve.affine_to_biguint(&curve.to_affine(actual).unwrap()),
        expected
    );
    let short = count.min(16);
    let (projective_product, _) = prefix(curve, base, digits, short);
    let mut affine_product = BigUint::one();
    let mut scalar = BigUint::one();
    for _ in 0..short {
        let (_, y) = textbook_mul(curve, base, &scalar).expect("nonidentity reference");
        affine_product = (affine_product * y) % prime;
        scalar = (scalar * step_scalar) % order;
    }
    assert_eq!(
        sign(&curve.f.to_biguint(&projective_product), prime),
        sign(&affine_product, prime)
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 1 || args.len() == 2 || args.len() == 4,
        "usage: p192_coset_orbit [count [Qx_hex Qy_hex]]"
    );
    let count = args
        .get(1)
        .map_or(1000, |s| s.parse().expect("positive count"));
    assert!((1..=100_000).contains(&count));
    let setup_wall = Instant::now();
    let setup_cpu = cpu_now();
    let prime = decimal(PRIME);
    let order = decimal(ORDER);
    let scalar = decimal(STEP);
    let coset_size = decimal(COSET_SIZE);
    let odd_factor = decimal(ODD_FACTOR);
    assert_eq!(scalar.modpow(&coset_size, &order), &order - 1u8);
    assert_ne!(scalar.modpow(&(&coset_size / 2u8), &order), BigUint::one());
    assert_ne!(
        scalar.modpow(&(&coset_size / &odd_factor), &order),
        BigUint::one()
    );
    let curve = Curve::<4>::new(&prime, &(&prime - BigUint::from(3u8)), &hex(B)).unwrap();
    let generator = curve.affine(&hex(GX), &hex(GY)).unwrap();
    assert!(curve.is_on_curve(&generator));
    let (q, target_scalar) = if args.len() == 4 {
        let q = curve.affine(&hex(&args[2]), &hex(&args[3])).unwrap();
        assert!(curve.is_on_curve(&q), "target is not on P-192");
        assert!(
            textbook_mul(&curve, &q, &order).is_none(),
            "target is not in the order-r subgroup"
        );
        (q, "unknown")
    } else {
        let q = curve
            .to_affine(&scalar_mul_wnaf(
                &curve,
                &generator,
                &BigUint::from(7u8),
                WIDTH,
                TableMode::Jacobian,
            ))
            .unwrap();
        let expected_q = textbook_mul(&curve, &generator, &BigUint::from(7u8)).unwrap();
        assert_eq!(curve.affine_to_biguint(&q), expected_q);
        (q, "7")
    };
    let digits = wnaf_digits(&scalar, WIDTH);
    let setup_wall_elapsed = setup_wall.elapsed();
    let setup_cpu_elapsed = cpu_now() - setup_cpu;

    let pair_wall = Instant::now();
    let pair_cpu = cpu_now();
    let (base_product, base_last) = prefix(&curve, &generator, &digits, count);
    let (target_product, target_last) = prefix(&curve, &q, &digits, count);
    let ratio_product = curve.f.mul(&base_product, &target_product);
    let prefix_ratio = sign(&curve.f.to_biguint(&ratio_product), &prime);
    let pair_wall_elapsed = pair_wall.elapsed();
    let pair_cpu_elapsed = cpu_now() - pair_cpu;

    verify(
        &curve, &generator, &digits, &scalar, &order, &prime, count, &base_last,
    );
    verify(
        &curve,
        &q,
        &digits,
        &scalar,
        &order,
        &prime,
        count,
        &target_last,
    );

    let add_count = 2 * count * 203;
    let rhs = curve.from_affine(&q);
    let mut walk = curve.from_affine(&generator);
    let add_wall = Instant::now();
    let add_cpu = cpu_now();
    for _ in 0..add_count {
        walk = curve.add(&walk, &rhs);
    }
    black_box(walk);
    let add_wall_elapsed = add_wall.elapsed();
    let add_cpu_elapsed = cpu_now() - add_cpu;

    println!(
        "count={} target_scalar={} width={} step_scalar={} prefix_ratio={}",
        count, target_scalar, WIDTH, STEP, prefix_ratio
    );
    let (qx, qy) = curve.affine_to_biguint(&q);
    println!("target_x_hex={qx:x} target_y_hex={qy:x}");
    println!(
        "setup_wall_ns={} setup_cpu_ns={}",
        setup_wall_elapsed.as_nanos(),
        setup_cpu_elapsed.as_nanos()
    );
    println!(
        "pair_wall_ns={} pair_cpu_ns={}",
        pair_wall_elapsed.as_nanos(),
        pair_cpu_elapsed.as_nanos()
    );
    println!(
        "full_add_count={} full_add_wall_ns={} full_add_cpu_ns={}",
        add_count,
        add_wall_elapsed.as_nanos(),
        add_cpu_elapsed.as_nanos()
    );
    println!(
        "orbit_to_full_add_wall_ratio={:.6} orbit_to_full_add_cpu_ratio={:.6}",
        pair_wall_elapsed.as_secs_f64() / (add_wall_elapsed.as_secs_f64() / 203.0),
        pair_cpu_elapsed.as_secs_f64() / (add_cpu_elapsed.as_secs_f64() / 203.0)
    );
    println!("validation=point_and_projective_sign_prefix_passed");
}
